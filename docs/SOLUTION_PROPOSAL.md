# Solution Proposal: isShowing() vs isVisible() Fix

**Date:** 2026-02-25
**Status:** DRAFT — Awaiting approval before implementation

---

## 1. Problem Summary

Java Swing's `isShowing()` returns `false` for components inside custom containers (e.g., JGoodies `SplitView`) even though they are `isVisible()=true`, painted on screen, and fully functional. This causes:

1. **All interactive keywords fail** — `Click`, `Double Click`, `Right Click`, `Input Text` throw `IllegalStateException` because the Java agent gates on `isShowing()` before dispatch
2. **Assertion inconsistency** — `Element Should Be Visible` fails (checks `visible && showing`) while `Component Should Be Visible` passes (checks `visible` only)
3. **State contradiction** — `Get Element States` reports `'visible'` but `Element Should Be Visible` fails for the same component

**Reproduced on:** JGoodies Smart Client Showcase 24.09.0 — `JLabel` and `JProgressBar` inside SplitView right content panel. Same root cause as original report (22.04.2 version affected NavigationToggleButtons).

---

## 2. Root Cause

```
SplitView (showing=True)
  └─ JPanel (child)         ← isShowing()=False breaks here
       └─ JLabel            ← inherits isShowing()=False from parent
       └─ JProgressBar      ← same
```

`isShowing()` is recursive — it walks up the ancestor chain. If any ancestor returns `false`, all descendants are `not showing`. The JGoodies `SplitView` has child JPanels where the `isShowing()` chain breaks, even though the components are visually rendered.

**Key insight:** The agent already uses `component.dispatchEvent(new MouseEvent(...))` for non-button clicks (not `java.awt.Robot`), so screen coordinates and `getLocationOnScreen()` are NOT needed. The `isShowing()` gate is the **only** barrier — removing it for synthetic dispatch is safe.

---

## 3. Affected Code Locations

### Java Agent (ActionExecutor.java)

| Method | Line | Gate | Behavior |
|--------|------|------|----------|
| `click()` | 53 | `isShowing()` | Polls 20×100ms after window activation, throws if still false |
| `doubleClick()` | 133 | `isShowing()` | Same retry+throw pattern |
| `rightClick()` | 209 | `ensureVisible()` | Immediate throw, no retry |
| `typeText()` | 264 | `ensureVisible()` | Immediate throw, no retry |
| `ensureVisible()` | 1062-1065 | `isShowing()` | Helper — throws `IllegalStateException` |

### Rust Layer (swing_library.rs)

| Function | Line | Check | Impact |
|----------|------|-------|--------|
| `element_should_be_visible()` | 1322-1333 | `visible && showing` | Fails for affected components |
| `wait_until_element_is_visible()` | 481-487 | `visible && showing` | Times out for affected components |

### Python Layer (__init__.py)

| Keyword | Line | Delegates to |
|---------|------|-------------|
| `element_should_be_visible` | 835 | Rust `element_should_be_visible` |
| `wait_until_element_is_visible` | 787 | Rust `wait_until_element_is_visible` |
| Timeout parsing | 806 | Float seconds only, no RF time strings |

---

## 4. Proposed Solution: Tiered Visibility Strategy

### 4.1 Architecture

Three tiers of click dispatch, tried in order:

```
Tier 1: Standard Path (isShowing=true)
  → Window activation + retry (existing behavior, unchanged)
  → AbstractButton.doClick() or performMouseClick()

Tier 2: Lenient Fallback (isVisible=true, isShowing=false)
  → Log warning
  → AbstractButton.doClick() (already works, no isShowing check)
  → Synthetic dispatchEvent(MouseEvent) with component-local coordinates

Tier 3: Strict Failure (isVisible=false)
  → Throw IllegalStateException (component truly not visible)
```

### 4.2 Changes by Layer

#### A. Java Agent — ActionExecutor.java (PRIMARY FIX)

**Change 1: Replace hard fail with tiered fallback in `click()`**

Current (line 53-95):
```java
if (!component.isShowing()) {
    // ... window activation + 20 retries ...
    throw new IllegalStateException("Component not visible for click...");
}
```

Proposed:
```java
if (!component.isShowing()) {
    // ... window activation + 20 retries (keep existing) ...
    if (!component.isShowing() && component.isVisible()) {
        // Tier 2: Component visible but not showing (custom container issue)
        logger.warning("Component " + componentId + " is visible but not showing. "
            + "Using synthetic event dispatch (JGoodies SplitView workaround).");
        // Fall through to existing dispatch — doClick() or performMouseClick()
    } else if (!component.isVisible()) {
        // Tier 3: truly not visible
        throw new IllegalStateException("Component is not visible");
    }
}
// Existing dispatch code continues unchanged
```

**Change 2: Apply same pattern to `doubleClick()`, `rightClick()`, `typeText()`**

Update `ensureVisible()` to implement the tiered logic:
```java
private static void ensureVisible(Component component) {
    if (!component.isShowing()) {
        if (component.isVisible()) {
            logger.warning("Component visible but not showing — using synthetic dispatch");
            return; // Allow fallthrough to synthetic dispatch
        }
        throw new IllegalStateException("Component is not visible");
    }
}
```

**Change 3: Add `forceInteract` RPC parameter (optional)**

Add an optional boolean parameter to click/type RPC methods:
```json
{
    "method": "click",
    "params": {
        "componentId": 42,
        "forceInteract": true
    }
}
```

When `forceInteract=true`, skip `isShowing()` check entirely (user explicitly requested it).

#### B. Rust Layer — swing_library.rs

**Change 4: Split visibility assertion into two keywords**

```rust
// Existing: checks both (KEEP, but rename semantics)
pub fn element_should_be_visible(&self, locator: &str) -> PyResult<()> {
    // Check visible only (align with Component Should Be Visible)
    let element = self.find_element(locator)?;
    if !element.visible {
        return Err(PyAssertionError::new_err(...));
    }
    Ok(())
}

// NEW: strict check for isShowing
pub fn element_should_be_showing(&self, locator: &str) -> PyResult<()> {
    let element = self.find_element(locator)?;
    if !element.visible || !element.showing {
        return Err(PyAssertionError::new_err(...));
    }
    Ok(())
}
```

**Change 5: Fix `wait_until_element_is_visible` to check `visible` only**

```rust
pub fn wait_until_element_is_visible(...) -> PyResult<SwingElement> {
    self.wait_for_element_condition(locator, timeout, |e| e.visible, "visible")
}

// NEW
pub fn wait_until_element_is_showing(...) -> PyResult<SwingElement> {
    self.wait_for_element_condition(locator, timeout, |e| e.visible && e.showing, "showing")
}
```

**Change 6: Add `force_interact` parameter to click keywords**

```rust
#[pyo3(signature = (locator, click_count=1, force_interact=false))]
pub fn click_element(&self, locator: &str, click_count: u32, force_interact: bool) -> PyResult<()> {
    // Pass force_interact through RPC
}
```

#### C. Python Layer — __init__.py

**Change 7: Expose new keywords and parameters**

```python
def element_should_be_visible(self, locator: str) -> None:
    """Verify element is visible (isVisible). See `Element Should Be Showing` for strict check."""
    self._lib.element_should_be_visible(locator)

def element_should_be_showing(self, locator: str) -> None:
    """Verify element is showing (isVisible AND isShowing). Strict visibility check."""
    self._lib.element_should_be_showing(locator)

def click(self, locator: str, force: bool = False) -> None:
    """Click element. Set force=True to bypass isShowing() check."""
    self._lib.click_element(locator, click_count=1, force_interact=force)
```

**Change 8: Add RF time string support to timeout parameters**

```python
from robot.utils import timestr_to_secs

def wait_until_element_is_visible(self, locator: str, timeout=None) -> None:
    timeout_val = timestr_to_secs(timeout) if timeout is not None else self._timeout
    self._lib.wait_until_element_is_visible(locator, timeout_val)
```

---

## 5. Implementation Plan

### Phase 1: Java Agent Fix (Critical — Unblocks All Interactions)

| Step | File | Change | Risk |
|------|------|--------|------|
| 1.1 | `ActionExecutor.java` | Modify `ensureVisible()` to allow visible-but-not-showing | LOW — fallthrough to existing dispatch code |
| 1.2 | `ActionExecutor.java` | Update `click()` retry block to log warning instead of throw | LOW |
| 1.3 | `ActionExecutor.java` | Apply same to `doubleClick()`, `rightClick()`, `typeText()` | LOW |
| 1.4 | `RpcServer.java` | Add optional `forceInteract` param to RPC methods | LOW |
| 1.5 | Agent rebuild | `mvn package` for agent JAR | — |

### Phase 2: Rust Layer Alignment

| Step | File | Change | Risk |
|------|------|--------|------|
| 2.1 | `swing_library.rs` | Change `element_should_be_visible` to check `visible` only | MED — changes existing behavior |
| 2.2 | `swing_library.rs` | Add `element_should_be_showing` (strict check) | LOW — new keyword |
| 2.3 | `swing_library.rs` | Fix `wait_until_element_is_visible` to check `visible` only | MED |
| 2.4 | `swing_library.rs` | Add `wait_until_element_is_showing` | LOW |
| 2.5 | `swing_library.rs` | Add `force_interact` param to click/input RPC calls | LOW |
| 2.6 | `protocol/mod.rs` | Update RPC message construction for new params | LOW |

### Phase 3: Python Layer & UX

| Step | File | Change | Risk |
|------|------|--------|------|
| 3.1 | `__init__.py` | Expose `Element Should Be Showing` keyword | LOW |
| 3.2 | `__init__.py` | Expose `Wait Until Element Is Showing` keyword | LOW |
| 3.3 | `__init__.py` | Add `force` param to Click/Double Click/Right Click/Input Text | LOW |
| 3.4 | `__init__.py` | Add RF time string parsing via `robot.utils.timestr_to_secs` | LOW |

### Phase 4: Testing & Validation

| Step | Scope | Method |
|------|-------|--------|
| 4.1 | Unit tests | Rust tests for new visibility logic |
| 4.2 | Integration | Robot tests against Smart Client Showcase 24.09.0 |
| 4.3 | Regression | Run existing test suite (`robot --dryrun`, pytest, cargo test) |
| 4.4 | Manual | Verify Click works on JLabel/JProgressBar in SplitView |

---

## 6. Behavioral Changes Summary

| Keyword | Current Behavior | New Behavior |
|---------|-----------------|-------------|
| `Element Should Be Visible` | Fails if `!showing` | Passes if `visible` (regardless of `showing`) |
| `Element Should Be Showing` | N/A (new) | Fails if `!showing` (old strict behavior) |
| `Wait Until Element Is Visible` | Times out if `!showing` | Succeeds when `visible` |
| `Wait Until Element Is Showing` | N/A (new) | Times out if `!showing` (old behavior) |
| `Click` / `Click Element` | Throws if `!showing` | Warns + uses synthetic dispatch if `visible && !showing` |
| `Double Click` | Throws if `!showing` | Same fallback |
| `Right Click` | Throws if `!showing` | Same fallback |
| `Input Text` | Throws if `!showing` | Same fallback |
| `Click ... force=True` | N/A (new param) | Skips `isShowing()` check entirely |

---

## 7. Risk Assessment

| Risk | Likelihood | Mitigation |
|------|-----------|-----------|
| Synthetic dispatch doesn't fire all listeners | LOW — already used for non-button clicks | `doClick()` remains primary for AbstractButton |
| Changing `Element Should Be Visible` breaks existing tests | MED | Tests checking `isShowing` should migrate to `Element Should Be Showing` |
| `forceInteract` misused on truly invisible components | LOW | Clear documentation, warning logs |
| L&F-specific MouseEvent handling issues | LOW | Component-local coordinates don't depend on L&F |

---

## 8. Files to Modify

| File | LOC est. | Phase |
|------|---------|-------|
| `agent/.../ActionExecutor.java` | ~40 lines changed | 1 |
| `agent/.../RpcServer.java` | ~15 lines changed | 1 |
| `src/python/swing_library.rs` | ~60 lines changed | 2 |
| `src/protocol/mod.rs` | ~10 lines changed | 2 |
| `python/JavaGui/__init__.py` | ~50 lines changed | 3 |
| Test files (new) | ~100 lines | 4 |
| **Total** | **~275 lines** | |

---

## 9. Alternative Considered: Ancestor Walking

Walking up the component hierarchy to fix `isShowing()` (e.g., selecting tabs, adjusting split panes) was considered but **rejected as primary approach** because:

1. Requires per-container-type code (JTabbedPane, JSplitPane, CardLayout, JGoodies SplitView...)
2. Custom/proprietary containers can't be handled generically
3. Changes application state as side effect
4. The synthetic dispatch path already exists and works — it's the simpler fix

Ancestor walking remains viable as a **future enhancement** for specific container types.
