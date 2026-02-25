# ADR-015: Tiered Visibility Strategy for Interactive Keywords

| ADR ID | ADR-015 |
|--------|---------|
| Title | Tiered Visibility Strategy for Interactive Keywords |
| Status | Proposed |
| Date | 2026-02-25 |
| Authors | Architecture Team |
| Related | ADR-001 (DDD Architecture), ADR-005 (Error Handling), ADR-007 (Unified Keyword API) |

---

## Context

### The isShowing() vs isVisible() Problem

Java Swing provides two distinct visibility predicates on `java.awt.Component`:

- **`isVisible()`** -- Returns whether the component's `visible` flag is set. This is a local property: a component can be "visible" even if its parent container is not displayed.
- **`isShowing()`** -- Returns whether the component is actually painted on screen. This is recursive: it walks the entire ancestor chain and returns `false` if **any** ancestor returns `isShowing()=false`.

The robotframework-javaui library currently gates all interactive keywords (`Click`, `Double Click`, `Right Click`, `Input Text`) on `isShowing()` in the Java agent's `ActionExecutor.java`. This causes a **complete failure** for components inside certain custom containers where `isShowing()` returns `false` despite the component being visually rendered and fully functional.

### Reproduction Environment

The problem is reproducible on JGoodies Smart Client Showcase (versions 22.04.2, 24.09.0) with `JLabel` and `JProgressBar` components inside `SplitView` right-content panels:

```
SplitView (showing=True)
  +-- JPanel (child)           <-- isShowing()=False breaks here
       +-- JLabel              <-- inherits isShowing()=False from parent
       +-- JProgressBar        <-- inherits isShowing()=False from parent
```

The JGoodies `SplitView` uses child JPanels where the `isShowing()` chain breaks, even though the components are visually rendered, painted, and receive events normally.

### Current Failure Modes

Three distinct categories of failure exist in the current codebase:

**1. All interactive keywords throw `IllegalStateException`**

The Java agent's `ActionExecutor` methods gate on `isShowing()`:

| Method | File:Line | Gate | Behavior |
|--------|-----------|------|----------|
| `click()` | `ActionExecutor.java:53` | `isShowing()` | Polls 20x100ms after window activation, throws if still false |
| `doubleClick()` | `ActionExecutor.java:133` | `isShowing()` | Same retry+throw pattern |
| `rightClick()` | `ActionExecutor.java:209` | `ensureVisible()` | Immediate throw, no retry |
| `typeText()` | `ActionExecutor.java:264` | `ensureVisible()` | Immediate throw, no retry |
| `ensureVisible()` | `ActionExecutor.java:1062-1065` | `isShowing()` | Helper -- throws `IllegalStateException` |

**2. Assertion inconsistency between keywords**

The Rust layer (`swing_library.rs`) checks both `visible && showing` for `Element Should Be Visible`, while the legacy `Component Should Be Visible` checks `visible` only. This produces contradictory results for affected components:

| Keyword | Check | Result for Affected Component |
|---------|-------|-------------------------------|
| `Element Should Be Visible` | `visible && showing` | FAIL |
| `Component Should Be Visible` | `visible` only | PASS |
| `Get Element States` | returns `'visible'` | Reports "visible" |

**3. Wait keywords time out incorrectly**

`Wait Until Element Is Visible` at `swing_library.rs:486` checks `e.visible && e.showing`, causing indefinite timeout for components that are visible but not showing.

### Why isShowing() Is Not Needed for Synthetic Dispatch

The agent already uses `component.dispatchEvent(new MouseEvent(...))` for non-button clicks and `AbstractButton.doClick()` for buttons. Neither mechanism requires `getLocationOnScreen()` or screen coordinates. The `isShowing()` check is the **only** barrier preventing interaction with these components. Removing it for synthetic dispatch is safe because:

1. `dispatchEvent()` operates on component-local coordinates, not screen coordinates
2. `AbstractButton.doClick()` is purely programmatic, no coordinate dependency
3. The component's event listeners fire regardless of `isShowing()` state
4. The component is genuinely in the component tree (`isDisplayable()=true`)

### Decision Drivers

- Interactive keywords must work with components inside custom containers (JGoodies SplitView, similar)
- Visibility assertions must be consistent and predictable
- Backwards compatibility must be preserved for existing test suites
- Users need an explicit escape hatch for edge cases (`force` parameter)
- The solution must fit within the DDD bounded contexts defined in ADR-001

---

## Decision

We will implement a **Tiered Visibility Strategy** with three tiers of interaction dispatch and split visibility assertions into semantic categories aligned with the Java Swing API.

### Tiered Dispatch Architecture

```
+-----------------------------------------------------------------------+
|                     TIERED VISIBILITY STRATEGY                         |
+-----------------------------------------------------------------------+
|                                                                         |
|  Keyword invocation: Click, Double Click, Right Click, Input Text       |
|         |                                                               |
|         v                                                               |
|  +-- Tier 1: Standard Path (isShowing=true) ------+                    |
|  |   - Window activation + retry (existing logic)  |                    |
|  |   - AbstractButton.doClick() or dispatchEvent() |                    |
|  |   - No warnings, no fallback needed             |                    |
|  +-- SUCCESS? ---> RETURN -------------------------+                    |
|         |                                                               |
|         | isShowing=false after retries                                  |
|         v                                                               |
|  +-- Tier 2: Lenient Fallback (isVisible=true) ---+                    |
|  |   - Log WARNING: "visible but not showing"     |                    |
|  |   - Emit VisibilityFallbackUsed event          |                    |
|  |   - AbstractButton.doClick() (works directly)  |                    |
|  |   - Synthetic dispatchEvent(MouseEvent)         |                    |
|  |   - Component-local coordinates (0-based)       |                    |
|  +-- SUCCESS? ---> RETURN -------------------------+                    |
|         |                                                               |
|         | isVisible=false                                                |
|         v                                                               |
|  +-- Tier 3: Strict Failure ----------------------+                    |
|  |   - Component is truly not visible             |                    |
|  |   - Throw IllegalStateException                |                    |
|  |   - Clear error message with diagnostics       |                    |
|  +------------------------------------------------+                    |
|                                                                         |
|  Force Override: force=True parameter bypasses Tier 1/2/3 entirely     |
|  - Skip isShowing() check                                              |
|  - Skip isVisible() check                                              |
|  - Proceed directly to dispatch                                        |
|  - Emit InteractionForced event                                        |
|                                                                         |
+-----------------------------------------------------------------------+
```

### Split Visibility Assertions

```
+-----------------------------------------------------------------------+
|              VISIBILITY ASSERTION SEMANTICS                             |
+-----------------------------------------------------------------------+
|                                                                         |
|  Element Should Be Visible (CHANGED)                                    |
|  - Checks: isVisible() only                                            |
|  - Meaning: "Component has its visible flag set"                        |
|  - Aligns with: Component Should Be Visible (legacy)                   |
|  - Use case: Standard visibility check                                  |
|                                                                         |
|  Element Should Be Showing (NEW)                                        |
|  - Checks: isVisible() AND isShowing()                                  |
|  - Meaning: "Component is painted on screen"                            |
|  - Aligns with: Old Element Should Be Visible behavior                  |
|  - Use case: Strict screen-presence verification                        |
|                                                                         |
|  Wait Until Element Is Visible (CHANGED)                                |
|  - Polls: isVisible() only                                              |
|  - Succeeds when component's visible flag becomes true                  |
|                                                                         |
|  Wait Until Element Is Showing (NEW)                                    |
|  - Polls: isVisible() AND isShowing()                                   |
|  - Succeeds when component is painted on screen                         |
|                                                                         |
+-----------------------------------------------------------------------+
```

---

## DDD Integration

The changes map to three bounded contexts from ADR-001:

### 1. Keyword Execution Context (Core)

The Keyword Execution Context gains a new `force` parameter on all interactive keywords. This parameter is an explicit user override that bypasses visibility gates entirely.

**Affected Keywords:**

| Keyword | Current Signature | New Signature |
|---------|-------------------|---------------|
| `Click` | `Click locator` | `Click locator [force=False]` |
| `Click Element` | `Click Element locator [click_count=1]` | `Click Element locator [click_count=1] [force=False]` |
| `Double Click` | `Double Click locator` | `Double Click locator [force=False]` |
| `Right Click` | `Right Click locator` | `Right Click locator [force=False]` |
| `Input Text` | `Input Text locator text` | `Input Text locator text [force=False]` |

The `force` parameter flows through the keyword dispatch pipeline:

```
Robot Framework Keyword
    |
    v
Python Layer (__init__.py)
    |  force=True/False
    v
Rust Layer (swing_library.rs)
    |  force_interact=True/False
    v
RPC Protocol (protocol/mod.rs)
    |  "forceInteract": true/false
    v
Java Agent (ActionExecutor.java)
    |  Skip isShowing() when forceInteract=true
    v
Synthetic Event Dispatch
```

### 2. Assertion Engine Context (Core)

The Assertion Engine Context splits the single `Element Should Be Visible` semantic into two distinct assertions aligned with the Java AWT API:

| DDD Concept | Current | Proposed |
|-------------|---------|----------|
| Visibility Assertion | `visible && showing` (conflated) | `visible` only (standard) |
| Showing Assertion | N/A | `visible && showing` (strict) |
| Wait-for-Visible | `visible && showing` (conflated) | `visible` only (standard) |
| Wait-for-Showing | N/A | `visible && showing` (strict) |

This split aligns assertion semantics with the Java Swing API terminology, reducing user confusion.

### 3. Infrastructure Layer (Toolkit Adapters)

The Infrastructure Layer (Swing Adapter) modifies the Java agent's `ActionExecutor` to implement tiered fallback logic. The `ensureVisible()` helper becomes the central decision point:

```
ensureVisible(component, forceInteract)
    |
    +-- forceInteract=true --> return (skip all checks)
    |
    +-- isShowing()=true --> return (standard path)
    |
    +-- isVisible()=true, isShowing()=false
    |       |
    |       +-- Log warning
    |       +-- return (allow synthetic dispatch)
    |
    +-- isVisible()=false
            |
            +-- throw IllegalStateException
```

---

## Detailed Changes by Layer

### A. Java Agent -- ActionExecutor.java

**Change 1: Tiered `ensureVisible()` with `forceInteract` parameter**

Current implementation (`ActionExecutor.java:1062-1065`):
```java
private static void ensureVisible(Component component) {
    if (!component.isShowing()) {
        throw new IllegalStateException("Component is not visible");
    }
}
```

Proposed implementation:
```java
/**
 * Ensure component is interactable using tiered visibility strategy.
 *
 * Tier 1: isShowing()=true  -> proceed (standard path)
 * Tier 2: isVisible()=true  -> warn + proceed (lenient fallback)
 * Tier 3: isVisible()=false -> throw (strict failure)
 *
 * @param component    The target component
 * @param forceInteract If true, skip all visibility checks
 */
private static void ensureVisible(Component component, boolean forceInteract) {
    if (forceInteract) {
        System.err.println("[SwingAgent] Force interact: skipping visibility checks for "
            + component.getClass().getSimpleName());
        return;
    }

    if (component.isShowing()) {
        return; // Tier 1: standard path
    }

    if (component.isVisible()) {
        // Tier 2: visible but not showing (custom container workaround)
        System.err.println("[SwingAgent] WARNING: Component "
            + component.getClass().getSimpleName()
            + " is visible but not showing. "
            + "Using synthetic event dispatch (custom container workaround). "
            + "Parent chain: " + getAncestorSummary(component));
        return; // Allow fallthrough to synthetic dispatch
    }

    // Tier 3: truly not visible
    throw new IllegalStateException(
        "Component is not visible (isVisible=false, isShowing=false). "
        + "Component: " + component.getClass().getSimpleName()
        + ", Parent: " + (component.getParent() != null
            ? component.getParent().getClass().getSimpleName()
            : "none"));
}

/**
 * Build a summary of the ancestor chain for diagnostic logging.
 */
private static String getAncestorSummary(Component component) {
    StringBuilder sb = new StringBuilder();
    Component current = component;
    int depth = 0;
    while (current != null && depth < 5) {
        if (depth > 0) sb.append(" -> ");
        sb.append(current.getClass().getSimpleName());
        sb.append("(showing=").append(current.isShowing());
        sb.append(",visible=").append(current.isVisible()).append(")");
        current = current.getParent();
        depth++;
    }
    return sb.toString();
}
```

**Change 2: Update `click()` retry block**

The `click()` method at line 53-95 currently throws after exhausting retries when `isShowing()` is false. The change replaces the hard failure with the tiered fallback:

```java
public static void click(int componentId, boolean forceInteract) {
    Component component = ComponentInspector.getComponentById(componentId);
    if (component == null) {
        throw new IllegalArgumentException("Component not found: " + componentId);
    }

    // Check visibility SYNCHRONOUSLY before async click
    EdtHelper.runOnEdt(() -> {
        if (forceInteract) {
            // Skip all visibility checks
            return;
        }

        if (!component.isShowing()) {
            // ... existing window activation + retry logic (unchanged) ...

            if (!showing) {
                // NEW: Tiered fallback instead of hard throw
                if (component.isVisible()) {
                    System.err.println("[SwingAgent] WARNING: Component " + componentId
                        + " visible but not showing after " + (retries * 100)
                        + "ms. Proceeding with synthetic dispatch.");
                    // Fall through -- do NOT throw
                } else {
                    throw new IllegalStateException(
                        "Component is not visible for click: " + componentId);
                }
            }
        }
    });

    // Existing async dispatch (unchanged)
    EdtHelper.runOnEdtLater(() -> {
        if (component instanceof AbstractButton) {
            ((AbstractButton) component).doClick();
        } else {
            performMouseClick(component, 1);
        }
    });

    EdtHelper.sleep(150);
}
```

**Change 3: Apply same pattern to `doubleClick()`, `rightClick()`, `typeText()`**

All four interactive methods receive the `forceInteract` boolean parameter and delegate to the tiered `ensureVisible(component, forceInteract)`:

```java
public static void rightClick(int componentId, boolean forceInteract) {
    Component component = EdtHelper.runOnEdtAndReturn(() -> {
        Component c = getComponent(componentId);
        ensureVisible(c, forceInteract);  // Tiered check
        return c;
    });
    // ... existing synthetic dispatch unchanged ...
}

public static void typeText(int componentId, String text, boolean forceInteract) {
    EdtHelper.runOnEdt(() -> {
        Component component = getComponent(componentId);
        ensureVisible(component, forceInteract);  // Tiered check
        // ... existing text input logic unchanged ...
    });
}
```

### B. Rust Layer -- swing_library.rs

**Change 4: Split `element_should_be_visible` to check `visible` only**

Current (`swing_library.rs:1322-1333`):
```rust
pub fn element_should_be_visible(&self, locator: &str) -> PyResult<()> {
    self.ensure_connected()?;
    let element = self.find_element(locator)?;
    if !element.visible || !element.showing {
        return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
            "Element '{}' is not visible", locator
        )));
    }
    Ok(())
}
```

Proposed:
```rust
/// Check if element is visible (isVisible property).
/// This checks the component's local visible flag only.
/// For strict screen-presence check, use element_should_be_showing.
#[pyo3(signature = (locator))]
pub fn element_should_be_visible(&self, locator: &str) -> PyResult<()> {
    self.ensure_connected()?;
    let element = self.find_element(locator)?;
    if !element.visible {
        return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
            "Element '{}' is not visible (isVisible=false)", locator
        )));
    }
    Ok(())
}
```

**Change 5: Add `element_should_be_showing` (strict check)**

```rust
/// Check if element is showing on screen (isVisible AND isShowing).
/// This verifies the component is actually painted in a visible window.
/// Use this when you need to confirm the component is on screen,
/// not just that its visible flag is set.
///
/// Example:
///     | Element Should Be Showing | name:statusPanel |
#[pyo3(signature = (locator))]
pub fn element_should_be_showing(&self, locator: &str) -> PyResult<()> {
    self.ensure_connected()?;
    let element = self.find_element(locator)?;
    if !element.visible || !element.showing {
        return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
            "Element '{}' is not showing on screen \
             (visible={}, showing={})",
            locator, element.visible, element.showing
        )));
    }
    Ok(())
}
```

**Change 6: Fix `wait_until_element_is_visible` to check `visible` only**

Current (`swing_library.rs:481-487`):
```rust
pub fn wait_until_element_is_visible(
    &self, locator: &str, timeout: Option<f64>,
) -> PyResult<SwingElement> {
    self.wait_for_element_condition(locator, timeout, |e| e.visible && e.showing, "visible")
}
```

Proposed:
```rust
#[pyo3(signature = (locator, timeout=None))]
pub fn wait_until_element_is_visible(
    &self, locator: &str, timeout: Option<f64>,
) -> PyResult<SwingElement> {
    self.wait_for_element_condition(locator, timeout, |e| e.visible, "visible")
}

/// Wait until element is showing on screen (isVisible AND isShowing).
///
/// Args:
///     locator: Element locator
///     timeout: Maximum wait time in seconds
///
/// Example:
///     | Wait Until Element Is Showing | name:resultPanel |
///     | Wait Until Element Is Showing | name:resultPanel | timeout=15 |
#[pyo3(signature = (locator, timeout=None))]
pub fn wait_until_element_is_showing(
    &self, locator: &str, timeout: Option<f64>,
) -> PyResult<SwingElement> {
    self.wait_for_element_condition(
        locator, timeout, |e| e.visible && e.showing, "showing"
    )
}
```

**Change 7: Add `force_interact` parameter to click keywords**

Current (`swing_library.rs:502-503`):
```rust
#[pyo3(signature = (locator, click_count=1))]
pub fn click_element(&self, locator: &str, click_count: u32) -> PyResult<()> {
```

Proposed:
```rust
#[pyo3(signature = (locator, click_count=1, force_interact=false))]
pub fn click_element(
    &self,
    locator: &str,
    click_count: u32,
    force_interact: bool,
) -> PyResult<()> {
    self.ensure_connected()?;
    let element = self.find_element(locator)?;

    // Pre-interaction checks (skip if force)
    if !force_interact {
        self.check_element_interactable(&element, "click")?;
    }

    // Build RPC params with forceInteract flag
    let params = serde_json::json!({
        "componentId": element.component_id,
        "forceInteract": force_interact
    });

    // Dispatch via RPC
    match click_count {
        1 => self.send_rpc(RpcMethod::Click, params)?,
        2 => self.send_rpc(RpcMethod::DoubleClick, params)?,
        _ => {
            for _ in 0..click_count {
                self.send_rpc(RpcMethod::Click, params.clone())?;
            }
        }
    }

    Ok(())
}
```

**Change 8: Update `check_element_interactable` for tiered logic**

Current (`swing_library.rs:2745`):
```rust
if !element.visible || !element.showing {
    return Err(SwingError::action_failed(
        action,
        format!("Element '{}' is not visible", element.simple_name),
    ).into());
}
```

Proposed:
```rust
if !element.visible {
    return Err(SwingError::action_failed(
        action,
        format!(
            "Element '{}' is not visible (isVisible=false). \
             The component is truly hidden.",
            element.simple_name
        ),
    ).into());
}

if !element.showing {
    // Tier 2: warn but allow -- the Java agent will handle dispatch
    log::warn!(
        "Element '{}' is visible but not showing (custom container). \
         Proceeding with synthetic dispatch.",
        element.simple_name
    );
}
```

### C. RPC Protocol -- protocol/mod.rs

**Change 9: Add `forceInteract` parameter to RPC message construction**

The RPC request builder adds the optional `forceInteract` boolean to click/type method params:

```rust
/// Build RPC params for click with optional force flag
pub fn click_params(component_id: i64, force_interact: bool) -> serde_json::Value {
    let mut params = serde_json::json!({
        "componentId": component_id
    });
    if force_interact {
        params["forceInteract"] = serde_json::Value::Bool(true);
    }
    params
}

/// Build RPC params for typeText with optional force flag
pub fn type_text_params(
    component_id: i64,
    text: &str,
    force_interact: bool,
) -> serde_json::Value {
    let mut params = serde_json::json!({
        "componentId": component_id,
        "text": text
    });
    if force_interact {
        params["forceInteract"] = serde_json::Value::Bool(true);
    }
    params
}
```

### D. Python Layer -- __init__.py

**Change 10: Expose new keywords and `force` parameter**

```python
def element_should_be_visible(self, locator: str) -> None:
    """Verify that an element is visible (isVisible property).

    Checks the component's local visible flag. This does NOT require
    the component to be "showing" (painted on screen). For a strict
    screen-presence check, use ``Element Should Be Showing``.

    | **Argument** | **Description** |
    | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

    Example:
    | Element Should Be Visible    JPanel#main
    | Element Should Be Visible    #loginForm

    See also: `Element Should Be Showing`, `Get Element States`
    """
    self._lib.element_should_be_visible(locator)

def element_should_be_showing(self, locator: str) -> None:
    """Verify that an element is showing on screen (isVisible AND isShowing).

    This is a strict check that verifies the component is actually painted
    in a visible window. Use this when you need to confirm screen presence,
    not just that the component's visible flag is set.

    Note: Components inside certain custom containers (e.g., JGoodies
    SplitView) may be visible but not "showing". In those cases, use
    ``Element Should Be Visible`` instead.

    | **Argument** | **Description** |
    | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

    Example:
    | Element Should Be Showing    JPanel#main
    | Element Should Be Showing    #statusBar

    See also: `Element Should Be Visible`, `Get Element States`
    """
    self._lib.element_should_be_showing(locator)
```

**Change 11: Add `force` parameter to Click and related keywords**

```python
def click(self, locator: str, force: bool = False) -> None:
    """Click on an element.

    | **Argument** | **Description** |
    | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |
    | ``force``   | If True, bypass isShowing() check. Default: False. |

    Set ``force=True`` when clicking components inside custom containers
    (e.g., JGoodies SplitView) where the component is visually present
    but isShowing() returns False.

    Example:
    | Click    JButton#ok
    | Click    JButton#ok    force=True

    """
    self._lib.click_element(locator, click_count=1, force_interact=force)
```

**Change 12: Add RF time string support to timeout parameters**

```python
from robot.utils import timestr_to_secs

def wait_until_element_is_visible(
    self, locator: str, timeout=None,
) -> None:
    """Wait until an element becomes visible.

    | **Argument** | **Description** |
    | ``locator``  | CSS or XPath-like locator string. See `Locator Syntax`. |
    | ``timeout``  | Maximum wait time. Accepts seconds (float) or RF time strings (e.g., ``10s``, ``1 min``). |

    Example:
    | Wait Until Element Is Visible    JLabel#status
    | Wait Until Element Is Visible    JLabel#status    timeout=15
    | Wait Until Element Is Visible    JLabel#status    timeout=1 min 30s

    """
    if timeout is not None:
        timeout_val = timestr_to_secs(timeout) if isinstance(timeout, str) else float(timeout)
    else:
        timeout_val = self._timeout
    self._lib.wait_until_element_is_visible(locator, timeout_val)
```

---

## New Value Objects

### VisibilityStrategy Enum

Represents the three tiers of visibility checking, used as a configuration value in the keyword dispatch pipeline:

```rust
/// Strategy for handling visibility checks during interaction.
///
/// This value object determines how the library gates interactive
/// keywords (Click, TypeText, etc.) based on component visibility.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VisibilityStrategy {
    /// Standard: require isShowing()=true.
    /// Falls through to Lenient if isShowing()=false but isVisible()=true.
    /// This is the default for all interactive keywords.
    Standard,

    /// Lenient: require isVisible()=true only.
    /// Used automatically as fallback when Standard fails on visible
    /// components inside custom containers. Logs a warning.
    Lenient,

    /// ForceInteract: skip all visibility checks.
    /// Used when the user explicitly passes force=True.
    /// Logs a notice and emits InteractionForced event.
    ForceInteract,
}

impl VisibilityStrategy {
    /// Determine the effective strategy based on component state.
    ///
    /// This implements the tiered fallback logic:
    ///   1. If force_interact -> ForceInteract
    ///   2. If isShowing -> Standard
    ///   3. If isVisible but !isShowing -> Lenient
    ///   4. If !isVisible -> error (not represented as strategy)
    pub fn resolve(
        visible: bool,
        showing: bool,
        force_interact: bool,
    ) -> Result<Self, VisibilityError> {
        if force_interact {
            return Ok(Self::ForceInteract);
        }
        if showing {
            return Ok(Self::Standard);
        }
        if visible {
            return Ok(Self::Lenient);
        }
        Err(VisibilityError::NotVisible)
    }

    /// Whether this strategy requires logging a warning.
    pub fn should_warn(&self) -> bool {
        matches!(self, Self::Lenient | Self::ForceInteract)
    }
}

/// Error when no valid visibility strategy can be resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VisibilityError {
    /// Component is truly not visible (isVisible=false).
    NotVisible,
}
```

### VisibilityState Value Object

An immutable snapshot of a component's visibility state at a point in time:

```rust
/// Immutable snapshot of a component's visibility state.
///
/// Captures the three orthogonal visibility predicates from
/// java.awt.Component to enable precise diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VisibilityState {
    /// java.awt.Component.isVisible() -- local visible flag.
    pub visible: bool,

    /// java.awt.Component.isShowing() -- recursive ancestor check.
    pub showing: bool,

    /// java.awt.Component.isDisplayable() -- in a native peer tree.
    pub in_tree: bool,
}

impl VisibilityState {
    pub fn new(visible: bool, showing: bool, in_tree: bool) -> Self {
        Self { visible, showing, in_tree }
    }

    /// Whether this state allows standard interaction (Tier 1).
    pub fn allows_standard_interaction(&self) -> bool {
        self.visible && self.showing
    }

    /// Whether this state allows lenient interaction (Tier 2).
    pub fn allows_lenient_interaction(&self) -> bool {
        self.visible && !self.showing && self.in_tree
    }

    /// Whether the component is truly not interactable.
    pub fn is_not_interactable(&self) -> bool {
        !self.visible
    }

    /// Human-readable diagnostic string for error messages.
    pub fn diagnostic_summary(&self) -> String {
        format!(
            "visible={}, showing={}, in_tree={}",
            self.visible, self.showing, self.in_tree
        )
    }
}

impl std::fmt::Display for VisibilityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.showing {
            write!(f, "showing")
        } else if self.visible {
            write!(f, "visible (not showing)")
        } else {
            write!(f, "not visible")
        }
    }
}
```

---

## New Domain Events

These events integrate with the domain event system defined in ADR-001:

```rust
/// Domain events for the Tiered Visibility Strategy.
///
/// These events enable observability into the visibility fallback
/// behavior, allowing monitoring, logging, and test diagnostics.

/// Emitted when the Lenient fallback path (Tier 2) is taken.
/// Indicates a component is visible but not showing -- typically
/// caused by custom containers like JGoodies SplitView.
#[derive(Clone, Debug)]
pub struct VisibilityFallbackUsed {
    /// The locator used to find the element
    pub locator: String,
    /// Component class name (e.g., "JLabel", "JProgressBar")
    pub component_class: String,
    /// The visibility state at time of fallback
    pub visibility_state: VisibilityState,
    /// The action being performed (e.g., "click", "typeText")
    pub action: String,
    /// Ancestor chain summary for diagnostics
    pub ancestor_summary: String,
    /// Timestamp of the event
    pub timestamp: std::time::Instant,
}

/// Emitted when force=True is used to bypass all visibility checks.
/// This is an explicit user override and should be logged for
/// audit/debugging purposes.
#[derive(Clone, Debug)]
pub struct InteractionForced {
    /// The locator used to find the element
    pub locator: String,
    /// Component class name
    pub component_class: String,
    /// The visibility state at time of forced interaction
    pub visibility_state: VisibilityState,
    /// The action being performed
    pub action: String,
    /// Timestamp of the event
    pub timestamp: std::time::Instant,
}

/// Integration with the existing DomainEvent enum from ADR-001:
pub enum DomainEvent {
    // ... existing variants ...

    /// Visibility fallback was used (Tier 2 lenient path)
    VisibilityFallbackUsed(VisibilityFallbackUsed),

    /// Interaction was forced by user (force=True)
    InteractionForced(InteractionForced),
}
```

---

## Migration Guide

### Impact on Existing Tests

**Low-risk changes (no action needed):**

All interactive keywords (`Click`, `Double Click`, `Right Click`, `Input Text`) remain fully backwards compatible. The tiered fallback is transparent -- tests that currently pass will continue to pass. Tests that currently fail due to `isShowing()=false` on visible components will start passing automatically.

**Medium-risk changes (may need review):**

| Change | Impact | Action Required |
|--------|--------|-----------------|
| `Element Should Be Visible` now checks `visible` only | Tests that relied on `visible && showing` semantics may pass where they previously failed | Review tests that use `Element Should Be Visible` on components expected to NOT be showing. Migrate to `Element Should Be Showing` if strict semantics are needed. |
| `Wait Until Element Is Visible` now checks `visible` only | Tests may succeed earlier than before | Review timeout-dependent test logic. Migrate to `Wait Until Element Is Showing` if strict semantics are needed. |

### Before / After Examples

**Example 1: Click on component inside JGoodies SplitView**

Before (FAILS with current code):
```robot
*** Test Cases ***
Click Label In SplitView
    [Documentation]    Fails: IllegalStateException - Component not visible
    Connect To Application    smartclient
    Click    JLabel#statusLabel
    # --> THROWS: Component not visible for click after window activation
```

After (PASSES with tiered strategy):
```robot
*** Test Cases ***
Click Label In SplitView
    [Documentation]    Works: Tier 2 lenient fallback used automatically
    Connect To Application    smartclient
    Click    JLabel#statusLabel
    # --> WARNING logged, but click succeeds via synthetic dispatch
```

**Example 2: Explicit force for edge cases**

```robot
*** Test Cases ***
Click With Force Override
    [Documentation]    Force bypass for extreme edge cases
    Connect To Application    smartclient
    Click    JLabel#deeplyNestedLabel    force=True
    # --> No visibility check at all, direct dispatch
```

**Example 3: Migrating visibility assertion**

Before:
```robot
*** Test Cases ***
Verify Panel Is On Screen
    [Documentation]    Checks both visible AND showing
    Element Should Be Visible    JPanel#dashboard
```

After (if strict semantics needed):
```robot
*** Test Cases ***
Verify Panel Is On Screen
    [Documentation]    Strict check: visible AND showing
    Element Should Be Showing    JPanel#dashboard

Verify Panel Is Visible
    [Documentation]    Standard check: visible flag only
    Element Should Be Visible    JPanel#dashboard
```

**Example 4: Using RF time strings in waits**

Before:
```robot
*** Test Cases ***
Wait For Element
    Wait Until Element Is Visible    JLabel#status    timeout=15
    # Only accepts float seconds
```

After:
```robot
*** Test Cases ***
Wait For Element
    Wait Until Element Is Visible    JLabel#status    timeout=15
    Wait Until Element Is Visible    JLabel#status    timeout=1 min 30s
    Wait Until Element Is Showing    JLabel#status    timeout=30s
```

---

## Backwards Compatibility

### Breaking Changes

| Change | Breaking? | Mitigation |
|--------|-----------|------------|
| `Element Should Be Visible` semantics loosened | **YES** -- tests relying on `showing` check will no longer fail for not-showing components | `Element Should Be Showing` provides the old strict behavior. Deprecation warning in release notes. |
| `Wait Until Element Is Visible` semantics loosened | **YES** -- tests may succeed earlier | `Wait Until Element Is Showing` provides old behavior. |
| New `force` parameter on Click/TypeText | No -- default is `False`, preserving existing behavior | N/A |
| New `Element Should Be Showing` keyword | No -- additive | N/A |
| New `Wait Until Element Is Showing` keyword | No -- additive | N/A |
| Java agent method signatures | No -- `forceInteract` is optional JSON parameter | Agent parses it as optional; missing = false |

### Compatibility Matrix

| Keyword | v0.3.x (current) | v0.4.x (proposed) | Migration Path |
|---------|-------------------|--------------------|----------------|
| `Element Should Be Visible` | Checks `visible && showing` | Checks `visible` only | Use `Element Should Be Showing` for old behavior |
| `Wait Until Element Is Visible` | Waits for `visible && showing` | Waits for `visible` only | Use `Wait Until Element Is Showing` for old behavior |
| `Click` / `Click Element` | Throws if `!showing` | Warns + fallback if `visible && !showing` | No change needed |
| `Element Should Be Showing` | N/A | Checks `visible && showing` | New keyword |
| `Wait Until Element Is Showing` | N/A | Waits for `visible && showing` | New keyword |

### Deprecation Policy

No keywords are deprecated in this ADR. The semantic change to `Element Should Be Visible` is intentional and aligns the keyword with the dominant user expectation (checking `visible`, not `showing`). The `Element Should Be Showing` keyword provides an explicit opt-in for the stricter semantics.

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Synthetic dispatch does not fire all expected listeners | LOW | MED | `dispatchEvent()` is already used for non-button clicks in the existing codebase. `AbstractButton.doClick()` remains primary for buttons. Both paths fire all registered listeners. |
| Changing `Element Should Be Visible` breaks existing tests | MED | MED | Tests that specifically need `isShowing` semantics migrate to `Element Should Be Showing`. Release notes will highlight this as a behavioral change. |
| `force=True` misused on truly invisible components | LOW | LOW | Clear documentation states `force` skips ALL checks. Warning logs when `force` is used. Users explicitly opt in. |
| Look-and-Feel-specific MouseEvent handling differences | LOW | LOW | Component-local coordinates are L&F-independent. Synthetic events use the same `dispatchEvent` path regardless of L&F. |
| Performance impact of additional isVisible/isShowing checks | VERY LOW | VERY LOW | Both are O(1) field reads (isVisible) or O(depth) ancestor walks (isShowing). Negligible compared to RPC round-trip latency. |
| Ancestor walking adds complexity to diagnostics | LOW | LOW | `getAncestorSummary()` is logging-only, capped at 5 levels, no functional impact on interaction path. |
| `forceInteract` parameter rejected by older Java agents | MED | LOW | The RPC handler ignores unknown JSON properties by default. Older agents will simply not read the parameter, falling back to existing behavior. |
| Tiered fallback masks legitimate visibility bugs in SUT | LOW | MED | The WARNING log message clearly identifies when Tier 2 is used. `Element Should Be Showing` provides a strict assertion for tests that need it. The `VisibilityFallbackUsed` domain event enables monitoring. |

---

## Alternatives Considered

### Alternative 1: Ancestor Walking

Walk up the component hierarchy to programmatically fix `isShowing()` (e.g., selecting tabs, adjusting split panes to reveal the component).

**Rejected because:**
- Requires per-container-type code (JTabbedPane, JSplitPane, CardLayout, JGoodies SplitView, etc.)
- Custom/proprietary containers cannot be handled generically
- Changes application state as a side effect (opening tabs, moving splitters)
- The synthetic dispatch path already exists and works -- it is the simpler fix

Ancestor walking remains viable as a **future enhancement** for specific container types.

### Alternative 2: Replace isShowing() with isDisplayable()

Check `isDisplayable()` instead of `isShowing()` as the visibility gate.

**Rejected because:**
- `isDisplayable()` only checks native peer existence, not visibility
- Would allow interaction with truly hidden components (e.g., in a non-selected tab)
- Too permissive for the default case

### Alternative 3: Remove isShowing() check entirely

Remove all `isShowing()` gates from the agent.

**Rejected because:**
- Loses the protection against interacting with truly non-visible components
- Makes error messages less helpful (failures would occur at dispatch time with less context)
- The tiered approach preserves the safety of the `isShowing()` check while providing a fallback

### Alternative 4: Configuration-only (no code change)

Add a library-level configuration option like `visibility_mode=lenient` instead of per-keyword `force` parameter.

**Rejected because:**
- Global configuration cannot distinguish between components that need lenient handling and those that should fail strictly
- Per-keyword `force` parameter gives precise control
- The automatic Tier 2 fallback handles most cases without any configuration

---

## Implementation Phases

### Phase 1: Java Agent Fix (Estimated: 2 days)

**Goal:** Unblock all interactive keywords for affected components.

| Step | File | Change | Effort | Risk |
|------|------|--------|--------|------|
| 1.1 | `ActionExecutor.java` | Modify `ensureVisible()` to implement tiered logic | 2h | LOW |
| 1.2 | `ActionExecutor.java` | Update `click()` retry block fallback | 2h | LOW |
| 1.3 | `ActionExecutor.java` | Apply pattern to `doubleClick()`, `rightClick()`, `typeText()` | 3h | LOW |
| 1.4 | `ActionExecutor.java` | Add `getAncestorSummary()` diagnostic helper | 1h | LOW |
| 1.5 | `RpcServer.java` | Parse optional `forceInteract` param from RPC JSON | 2h | LOW |
| 1.6 | Agent rebuild | `mvn package` for agent JAR | 0.5h | -- |
| | | **Phase 1 Total** | **~1.5 days** | |

### Phase 2: Rust Layer Alignment (Estimated: 3 days)

**Goal:** Align Rust visibility checks, add new keywords, add `force_interact` parameter.

| Step | File | Change | Effort | Risk |
|------|------|--------|--------|------|
| 2.1 | `swing_library.rs` | Change `element_should_be_visible` to check `visible` only | 1h | MED |
| 2.2 | `swing_library.rs` | Add `element_should_be_showing` keyword | 2h | LOW |
| 2.3 | `swing_library.rs` | Change `wait_until_element_is_visible` to check `visible` only | 1h | MED |
| 2.4 | `swing_library.rs` | Add `wait_until_element_is_showing` keyword | 2h | LOW |
| 2.5 | `swing_library.rs` | Add `force_interact` param to `click_element`, `right_click`, `type_text` | 3h | LOW |
| 2.6 | `swing_library.rs` | Update `check_element_interactable` for tiered logic | 2h | LOW |
| 2.7 | `protocol/mod.rs` | Add `forceInteract` to RPC request builders | 1h | LOW |
| 2.8 | `element.rs` | (No structural changes -- `visible` and `showing` fields already exist) | 0h | -- |
| 2.9 | Value objects | Implement `VisibilityStrategy`, `VisibilityState` | 3h | LOW |
| 2.10 | Domain events | Implement `VisibilityFallbackUsed`, `InteractionForced` | 2h | LOW |
| | | **Phase 2 Total** | **~2.5 days** | |

### Phase 3: Python Layer and UX (Estimated: 1.5 days)

**Goal:** Expose new keywords and parameters to Robot Framework users.

| Step | File | Change | Effort | Risk |
|------|------|--------|--------|------|
| 3.1 | `__init__.py` | Expose `Element Should Be Showing` keyword | 1h | LOW |
| 3.2 | `__init__.py` | Expose `Wait Until Element Is Showing` keyword | 1h | LOW |
| 3.3 | `__init__.py` | Add `force` param to `Click`, `Double Click`, `Right Click`, `Input Text` | 2h | LOW |
| 3.4 | `__init__.py` | Add RF time string parsing via `robot.utils.timestr_to_secs` | 1h | LOW |
| 3.5 | `__init__.py` | Update keyword docstrings with cross-references | 2h | LOW |
| | | **Phase 3 Total** | **~1 day** | |

### Phase 4: Testing and Validation (Estimated: 2.5 days)

**Goal:** Comprehensive test coverage for all changes.

| Step | Scope | Method | Effort |
|------|-------|--------|--------|
| 4.1 | Value objects | Rust unit tests for `VisibilityStrategy`, `VisibilityState` | 2h |
| 4.2 | Assertion keywords | Rust tests for new `element_should_be_showing` | 2h |
| 4.3 | Wait keywords | Rust tests for `wait_until_element_is_showing` | 2h |
| 4.4 | Force parameter | Rust tests for `force_interact` flow | 2h |
| 4.5 | Domain events | Rust tests for event emission | 1h |
| 4.6 | Python layer | pytest for new keyword exposure and RF time parsing | 2h |
| 4.7 | Robot Framework | RF tests for `Element Should Be Showing`, `force=True` | 3h |
| 4.8 | Regression | Full test suite: `robot --dryrun`, `pytest`, `cargo test` | 2h |
| 4.9 | Integration | Manual validation against JGoodies Smart Client Showcase 24.09.0 | 3h |
| | | **Phase 4 Total** | **~2.5 days** | |

### Total Estimated Effort

| Phase | Effort | Dependencies |
|-------|--------|--------------|
| Phase 1: Java Agent Fix | 1.5 days | None |
| Phase 2: Rust Layer | 2.5 days | Phase 1 (agent JAR) |
| Phase 3: Python Layer | 1 day | Phase 2 (Rust bindings) |
| Phase 4: Testing | 2.5 days | Phases 1-3 |
| **Total** | **~7.5 days** | |

---

## Files Modified Summary

| File | LOC (est.) | Phase |
|------|-----------|-------|
| `agent/.../ActionExecutor.java` | ~60 lines changed/added | 1 |
| `agent/.../RpcServer.java` | ~15 lines changed | 1 |
| `src/python/swing_library.rs` | ~80 lines changed/added | 2 |
| `src/protocol/mod.rs` | ~15 lines changed | 2 |
| `src/domain/value_objects/visibility.rs` (new) | ~120 lines | 2 |
| `src/domain/events/visibility_events.rs` (new) | ~50 lines | 2 |
| `python/JavaGui/__init__.py` | ~70 lines changed/added | 3 |
| Test files (new/modified) | ~200 lines | 4 |
| **Total** | **~610 lines** | |

---

## References

- [Java AWT Component.isShowing() Javadoc](https://docs.oracle.com/en/java/javase/17/docs/api/java.desktop/java/awt/Component.html#isShowing())
- [Java AWT Component.isVisible() Javadoc](https://docs.oracle.com/en/java/javase/17/docs/api/java.desktop/java/awt/Component.html#isVisible())
- [JGoodies Smart Client Framework](https://www.jgoodies.com/)
- [Solution Proposal: isShowing vs isVisible Fix](../SOLUTION_PROPOSAL.md)
- [ADR-001: DDD Architecture](ADR-001-DDD-ARCHITECTURE.md)
- [ADR-005: Error Handling Strategy](ADR-005-error-handling-strategy.md)
- [ADR-007: Unified Keyword API](ADR-007-UNIFIED-KEYWORD-API.md)

---

*Document Version: 1.0*
*Status: Proposed*
*Last Updated: 2026-02-25*
