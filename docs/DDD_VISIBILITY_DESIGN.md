# DDD Implementation Design: Visibility Fix and Timeout Changes

**Date:** 2026-02-25
**Status:** DRAFT
**Author:** System Architecture Designer
**Related:** ADR-001-DDD-ARCHITECTURE.md, SOLUTION_PROPOSAL.md

---

## 1. Bounded Context Impact Analysis

### 1.1 Contexts Affected by the Visibility Fix

The visibility fix (isShowing vs isVisible) touches three of the five bounded contexts
defined in ADR-001. The Introspection Context is indirectly affected through data
model changes. The Locator Resolution Context is not affected.

| Bounded Context | Impact Level | Reason |
|-----------------|-------------|--------|
| **Keyword Execution** (Core) | HIGH | Click, TypeText, RightClick all gate on visibility before dispatch. The `force_interact` option is a new `CommandOptions` field. |
| **Assertion Engine** (Core) | HIGH | `Element Should Be Visible` changes from `visible && showing` to `visible` only. New `Element Should Be Showing` keyword. Auto-retry `wait_until_*` semantics change. |
| **Session Management** (Supporting) | MEDIUM | `ApplicationSession` gains a `visibility_strategy` configuration. RPC protocol changes for `forceInteract` parameter. |
| **Introspection** (Supporting) | LOW | `VisibilityState` value object used in element metadata. No keyword changes. |
| **Locator Resolution** (Generic) | NONE | Locator parsing and evaluation are unaffected. |

### 1.2 Contexts Affected by Timeout Parsing

Timeout parsing changes are confined to the anti-corruption layer between Python and
Rust, plus the Assertion Engine where timeout values are consumed.

| Bounded Context | Impact Level | Reason |
|-----------------|-------------|--------|
| **Assertion Engine** (Core) | MEDIUM | Timeout values flow through assertion specs. `TimeoutValue` replaces raw `f64`. |
| **Keyword Execution** (Core) | MEDIUM | `CommandOptions.timeout` changes from `Duration` to `TimeoutValue`. |
| **Session Management** (Supporting) | LOW | Connection timeout already accepts `f64`; no RF string parsing needed there. |
| **Locator Resolution** (Generic) | NONE | No timeout involvement. |
| **Introspection** (Supporting) | NONE | No timeout involvement. |

### 1.3 Context Dependency Diagram

```
                    Python Anti-Corruption Layer
                    (RF time string -> TimeoutValue)
                    (force_interact -> ForceInteract)
                              |
                              | translates at boundary
                              v
+------------------------------------------------------------------------+
|                          CORE SUBDOMAIN                                 |
|                                                                         |
|  +----------------------------+    +-------------------------------+    |
|  | KEYWORD EXECUTION CONTEXT  |    | ASSERTION ENGINE CONTEXT      |    |
|  |                            |    |                               |    |
|  | KeywordCommand aggregate   |    | AssertionSpec aggregate       |    |
|  |   +force_interact option   |    |   +TimeoutValue (not f64)    |    |
|  |   +visibility_strategy     |    |                               |    |
|  |                            |    | New keywords:                 |    |
|  | KeywordDispatcherService   |    |   Element Should Be Showing   |    |
|  |   +tiered click dispatch   |    |   Wait Until Element Is       |    |
|  |                            |    |     Showing                   |    |
|  +-------------|-----+--------+    +-------------------------------+    |
|                |     |                                                  |
|   [Shared Kernel: VisibilityState, TimeoutValue, InteractionMode]       |
|                |     |                                                  |
+----------------|-----|-------------------------------------------------+
                 |     |
    +------------+     +--------------------+
    |                                       |
    v                                       v
+------------------------+     +------------------------+
| SESSION MANAGEMENT     |     | INTROSPECTION          |
| CONTEXT                |     | CONTEXT                |
|                        |     |                        |
| ApplicationSession     |     | VisibilityState in     |
|   +visibility_strategy |     | element metadata       |
|                        |     | (read-only)            |
| SwingAdapter           |     |                        |
|   +forceInteract param |     |                        |
+----------+-------------+     +------------------------+
           |
           | RPC (JSON-RPC 2.0)
           v
+------------------------------------------------------------------------+
|                        INFRASTRUCTURE LAYER                             |
|                                                                         |
|  +------------------------------------------------------------------+  |
|  | Java Agent                                                        |  |
|  |   ActionExecutor: tiered ensureVisible()                          |  |
|  |   RpcServer: forceInteract parameter in click/doubleClick/        |  |
|  |             rightClick/typeText                                   |  |
|  +------------------------------------------------------------------+  |
+------------------------------------------------------------------------+
```

---

## 2. Aggregate Root Changes

### 2.1 KeywordCommand Aggregate: New `force_interact` in CommandOptions

**Before** (from ADR-001, `CommandOptions` value object):

```rust
/// Current CommandOptions in ADR-001 design
pub struct CommandOptions {
    pub timeout: Duration,
    pub retry_policy: RetryPolicy,
    pub soft_assert: bool,
    pub screenshot_on_fail: bool,
}
```

**After** (with visibility fix additions):

```rust
/// Updated CommandOptions with visibility strategy and force_interact
pub struct CommandOptions {
    pub timeout: TimeoutValue,                      // Changed: was Duration
    pub retry_policy: RetryPolicy,
    pub soft_assert: bool,
    pub screenshot_on_fail: bool,
    pub visibility_strategy: VisibilityStrategy,    // NEW
    pub interaction_mode: InteractionMode,           // NEW
}

impl Default for CommandOptions {
    fn default() -> Self {
        Self {
            timeout: TimeoutValue::default(),            // 10 seconds
            retry_policy: RetryPolicy::default(),
            soft_assert: false,
            screenshot_on_fail: true,
            visibility_strategy: VisibilityStrategy::Standard,
            interaction_mode: InteractionMode::Normal,
        }
    }
}
```

**New invariant**: When `interaction_mode` is `Forced`, `visibility_strategy` is
implicitly `Lenient` (the dispatcher does not check any visibility before sending
the RPC call with `forceInteract=true`).

### 2.2 ApplicationSession Aggregate: New `visibility_strategy` Configuration

**Before** (current `LibraryConfig` in `swing_library.rs` line 33):

```rust
struct LibraryConfig {
    timeout: f64,
    poll_interval: f64,
    log_actions: bool,
    screenshot_directory: String,
    screenshot_format: String,
}
```

**After** (with session-level visibility strategy):

```rust
struct LibraryConfig {
    timeout: f64,
    poll_interval: f64,
    log_actions: bool,
    screenshot_directory: String,
    screenshot_format: String,
    visibility_strategy: VisibilityStrategy,     // NEW: session-level default
}

impl Default for LibraryConfig {
    fn default() -> Self {
        Self {
            timeout: 10.0,
            poll_interval: 0.5,
            log_actions: true,
            screenshot_directory: ".".to_string(),
            screenshot_format: "png".to_string(),
            visibility_strategy: VisibilityStrategy::Standard,  // NEW
        }
    }
}
```

The session-level strategy serves as the default for all keyword commands. Individual
keywords can override it per-invocation through their `force_interact` parameter.

**Precedence chain**: keyword parameter > session config > hardcoded default

---

## 3. New Value Objects

### 3.1 VisibilityStrategy

```rust
/// Determines how visibility is checked before interaction.
///
/// The three tiers correspond to the solution proposal's tiered dispatch:
/// - Standard: requires isShowing()=true (existing strict behavior)
/// - Lenient: accepts isVisible()=true even if isShowing()=false
///            (logs warning, uses synthetic dispatch)
/// - ForceInteract: skips all visibility checks (user explicitly requested)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VisibilityStrategy {
    /// Tier 1: Standard path. Requires component.isShowing() == true.
    /// Window activation + retry (existing behavior, unchanged).
    Standard,

    /// Tier 2: Lenient fallback. Accepts component.isVisible() == true
    /// even when isShowing() == false (JGoodies SplitView workaround).
    /// Logs a warning and uses synthetic event dispatch.
    Lenient,

    /// Tier 3: Force interact. Skips all visibility checks.
    /// User explicitly requested interaction on a potentially invisible component.
    /// Maps to forceInteract=true in the RPC call.
    ForceInteract,
}

impl Default for VisibilityStrategy {
    fn default() -> Self {
        Self::Standard
    }
}
```

**Location**: `src/domain/value_objects/visibility.rs` (new file) or initially
embedded in `src/python/swing_library.rs` pending the full DDD module restructure.

### 3.2 VisibilityState

```rust
/// Immutable snapshot of a component's visibility flags.
///
/// Captures the distinction between Java's isVisible() (component-level flag)
/// and isShowing() (recursive ancestor-chain check). This is the value object
/// that replaces the separate `visible: bool` + `showing: bool` fields in
/// SwingElement and ComponentState.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VisibilityState {
    /// Component's own visibility flag (java.awt.Component.isVisible).
    /// True if setVisible(true) was called on this component.
    pub visible: bool,

    /// Whether the component is actually displayable on screen
    /// (java.awt.Component.isShowing). Recursive: returns false if
    /// any ancestor returns false.
    pub showing: bool,

    /// Whether the component is part of the AWT component hierarchy.
    /// A component may be visible but not yet added to a container.
    pub in_tree: bool,
}

impl VisibilityState {
    /// Standard visibility check: both visible and showing.
    pub fn is_fully_visible(&self) -> bool {
        self.visible && self.showing && self.in_tree
    }

    /// Lenient visibility check: visible, but showing may be false
    /// due to custom container issues (e.g., JGoodies SplitView).
    pub fn is_visible_lenient(&self) -> bool {
        self.visible && self.in_tree
    }

    /// Truly invisible: component's own visible flag is false.
    pub fn is_hidden(&self) -> bool {
        !self.visible
    }

    /// Determine which tier applies for interaction dispatch.
    pub fn interaction_tier(&self) -> VisibilityStrategy {
        if self.is_fully_visible() {
            VisibilityStrategy::Standard
        } else if self.is_visible_lenient() {
            VisibilityStrategy::Lenient
        } else {
            // Truly not visible -- no tier applies, will error
            VisibilityStrategy::Standard // caller must check is_hidden()
        }
    }
}
```

**Relationship to existing structs**:
- `ComponentState` (in `src/model/component.rs` line 212) currently has `visible: bool`
  and `showing: bool` as flat fields. These remain for backwards compatibility.
- `VisibilityState` is the domain value object that encapsulates the semantics.
- `SwingElement` (in `src/python/element.rs` line 77-79) exposes `visible` and
  `showing` as separate `#[pyo3(get)]` fields. This also remains for Python API
  compatibility.

### 3.3 TimeoutValue

```rust
/// Strongly-typed timeout duration that handles parsing at the system boundary.
///
/// In Robot Framework, timeouts can be expressed as:
/// - Float seconds: 10.0, 5, 0.5
/// - RF time strings: "10s", "1 minute", "2min 30s", "1h", "500ms"
///
/// TimeoutValue is parsed from RF time strings at the Python anti-corruption
/// layer and crosses into Rust as a validated Duration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeoutValue {
    duration: Duration,
}

impl TimeoutValue {
    /// Create from a pre-validated duration (already parsed by Python layer).
    pub fn from_seconds(seconds: f64) -> Result<Self, TimeoutError> {
        if seconds < 0.0 {
            return Err(TimeoutError::Negative(seconds));
        }
        if seconds > 86400.0 {
            return Err(TimeoutError::TooLarge(seconds));
        }
        Ok(Self {
            duration: Duration::from_secs_f64(seconds),
        })
    }

    /// Get the underlying Duration.
    pub fn as_duration(&self) -> Duration {
        self.duration
    }

    /// Get as seconds (f64) for backwards compatibility.
    pub fn as_secs_f64(&self) -> f64 {
        self.duration.as_secs_f64()
    }
}

impl Default for TimeoutValue {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(10),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TimeoutError {
    Negative(f64),
    TooLarge(f64),
}
```

### 3.4 InteractionMode

```rust
/// Determines how the Rust layer dispatches interactions to the Java agent.
///
/// This controls the RPC parameters and pre-checks:
/// - Normal: standard dispatch, visibility checks apply
/// - Synthetic: explicitly request synthetic event dispatch
/// - Forced: set forceInteract=true, skip all visibility checks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractionMode {
    /// Standard dispatch. Visibility strategy determines checks.
    /// For AbstractButton: doClick(). For others: performMouseClick().
    Normal,

    /// Explicit synthetic dispatch. Always use dispatchEvent(MouseEvent)
    /// regardless of component type. Useful when doClick() has side effects.
    Synthetic,

    /// Forced dispatch. Pass forceInteract=true to Java agent.
    /// Skip all isShowing() checks. Use when the test author knows the
    /// component is interactable despite isShowing()=false.
    Forced,
}

impl Default for InteractionMode {
    fn default() -> Self {
        Self::Normal
    }
}
```

---

## 4. Domain Service Changes

### 4.1 KeywordDispatcherService: Click Dispatch with Tiered Visibility

The `KeywordDispatcherService` routes keyword commands to the appropriate handler.
For click operations, it now incorporates the visibility strategy to determine
whether to proceed, warn, or fail before sending the RPC request.

```
Keyword Call: Click Element    locator    force_interact=False
         |
         v
+-------------------+
| KeywordCommand    |  1. Build command with CommandOptions
| (Aggregate)       |     - visibility_strategy from session config
+-------------------+     - interaction_mode from force_interact param
         |
         v
+-------------------+
| Dispatcher        |  2. Resolve element via LocatorResolverService
| Service           |     -> SwingElement with VisibilityState
+-------------------+
         |
         v
+-------------------+
| Visibility        |  3. Evaluate visibility tier:
| Decision          |
+-------------------+
    |       |       |
    v       v       v
 [Tier1] [Tier2] [Tier3]
 showing  visible  hidden
 =true    =true    =true
          show=    vis=
          false    false
    |       |       |
    v       v       v
 Standard  Warn+   ERROR:
 dispatch  Lenient  "Component
           dispatch  is not
                    visible"
    |       |
    +---+---+
        |
        v
+-------------------+
| RPC Call          |  4. Send to Java Agent
| Construction      |     - Standard: {"method":"click","params":{"componentId":42}}
+-------------------+     - Forced:   {"method":"click","params":{"componentId":42,"forceInteract":true}}
         |
         v
+-------------------+
| Java Agent        |  5. ActionExecutor.click(componentId, forceInteract)
| (Infrastructure)  |     - Tier 1: existing path (isShowing=true)
+-------------------+     - Tier 2: warning + synthetic dispatch
                          - forceInteract: skip isShowing check entirely
```

### 4.2 Dispatch Flow Pseudocode

```rust
impl KeywordDispatcherService {
    fn dispatch_click(
        &self,
        locator: &str,
        click_count: u32,
        force_interact: bool,
    ) -> Result<(), SwingError> {
        // Step 1: Resolve element
        let element = self.locator_service.resolve(locator)?;

        // Step 2: Determine effective interaction mode
        let mode = if force_interact {
            InteractionMode::Forced
        } else {
            self.session.config().interaction_mode
        };

        // Step 3: Evaluate visibility tier
        let vis = VisibilityState {
            visible: element.visible,
            showing: element.showing,
            in_tree: true, // element was found in tree
        };

        match mode {
            InteractionMode::Forced => {
                // Skip all checks, pass forceInteract=true
            }
            InteractionMode::Normal | InteractionMode::Synthetic => {
                if vis.is_hidden() {
                    // Tier 3: truly not visible -- hard error
                    return Err(SwingError::not_visible(locator));
                }
                if !vis.is_fully_visible() && vis.is_visible_lenient() {
                    // Tier 2: visible but not showing -- warn + proceed
                    self.emit_event(DomainEvent::VisibilityFallback {
                        locator: locator.to_string(),
                        element_id: element.hash_code,
                        strategy_used: VisibilityStrategy::Lenient,
                    });
                    log::warn!(
                        "Component '{}' is visible but not showing. \
                         Using lenient dispatch (JGoodies SplitView workaround).",
                        locator
                    );
                }
                // Tier 1: fully visible -- normal path
            }
        }

        // Step 4: Build RPC parameters
        let mut params = serde_json::json!({
            "componentId": element.hash_code as i32,
        });
        if mode == InteractionMode::Forced {
            params["forceInteract"] = serde_json::Value::Bool(true);
        }

        // Step 5: Send RPC
        let method = match click_count {
            2 => "doubleClick",
            _ => "click",
        };
        self.session.send_rpc_request(method, params)?;

        // Step 6: Emit success event
        self.emit_event(DomainEvent::KeywordExecuted { /* ... */ });

        Ok(())
    }
}
```

---

## 5. Infrastructure Layer Changes

### 5.1 SwingAdapter: New RPC Parameters

The `send_rpc_request` method in `swing_library.rs` (line 1999) already constructs
JSON-RPC requests with arbitrary params. The change adds an optional `forceInteract`
field to click/doubleClick/rightClick/typeText calls.

**Current RPC message** (click, `swing_library.rs` line 515):

```json
{
    "jsonrpc": "2.0",
    "method": "click",
    "params": {
        "componentId": 42
    },
    "id": 7
}
```

**Updated RPC message** (click with forceInteract):

```json
{
    "jsonrpc": "2.0",
    "method": "click",
    "params": {
        "componentId": 42,
        "forceInteract": true
    },
    "id": 7
}
```

**Affected RPC methods**:

| Method | Current Params | New Optional Param |
|--------|---------------|-------------------|
| `click` | `componentId: int` | `forceInteract: bool` (default: false) |
| `doubleClick` | `componentId: int` | `forceInteract: bool` (default: false) |
| `rightClick` | `componentId: int` | `forceInteract: bool` (default: false) |
| `typeText` | `componentId: int, text: string` | `forceInteract: bool` (default: false) |
| `waitUntilVisible` | `componentId: int, timeout: long` | `lenient: bool` (default: false) |

The `forceInteract` parameter is optional and backward-compatible. If absent, the
Java agent uses existing behavior. The wire protocol (line-delimited JSON-RPC 2.0
over TCP, `RpcServer.java` line 74) does not change.

### 5.2 RPC Protocol Changes

No protocol-level changes are needed. The existing JSON-RPC 2.0 implementation
supports arbitrary params objects. The `dispatchMethod` in `RpcServer.java`
(line 114) already uses `paramsObj.has("key")` pattern for optional parameters
(see `selectMenu` at line 202 which checks `paramsObj.has("timeout")`).

### 5.3 Java Agent: Tiered ensureVisible() Logic

**Current `ensureVisible` (ActionExecutor.java line 1062-1065)**:

```java
private static void ensureVisible(Component component) {
    if (!component.isShowing()) {
        throw new IllegalStateException("Component is not visible");
    }
}
```

**Proposed `ensureVisible` with tiered logic**:

```java
/**
 * Tiered visibility check.
 *
 * Tier 1: isShowing()=true -> proceed normally
 * Tier 2: isVisible()=true, isShowing()=false -> warn + allow synthetic dispatch
 * Tier 3: isVisible()=false -> throw (truly not visible)
 *
 * @param component    The target component
 * @param forceInteract If true, skip all checks (Tier 0)
 */
private static void ensureVisible(Component component, boolean forceInteract) {
    if (forceInteract) {
        // Tier 0: user explicitly requested -- skip all checks
        return;
    }

    if (component.isShowing()) {
        // Tier 1: standard path -- fully visible
        return;
    }

    if (component.isVisible()) {
        // Tier 2: visible but not showing (custom container issue)
        System.err.println(
            "[SwingAgent] WARNING: Component is visible but not showing. "
            + "Using synthetic event dispatch. "
            + "Class: " + component.getClass().getName()
            + ", Bounds: " + component.getBounds()
        );
        return; // Allow fallthrough to synthetic dispatch
    }

    // Tier 3: truly not visible
    throw new IllegalStateException(
        "Component is not visible (isVisible=false). "
        + "Class: " + component.getClass().getName()
    );
}

// Backward-compatible overload
private static void ensureVisible(Component component) {
    ensureVisible(component, false);
}
```

**Changes to `click()` method (ActionExecutor.java line 43-116)**:

The `click()` method has inline visibility logic (not delegated to `ensureVisible`).
It needs the same tiered modification in its synchronous visibility check block
(lines 52-103). The key change is replacing the hard `throw` at line 90 with the
tiered fallback:

```java
// Line 89-95: Replace hard throw with tiered fallback
if (!showing) {
    if (component.isVisible()) {
        // Tier 2: visible but not showing -- warn and proceed
        System.err.println("[SwingAgent] Component " + componentId
            + " is visible but not showing. Using synthetic dispatch.");
        // Fall through to async click below
    } else {
        // Tier 3: truly not visible
        throw new IllegalStateException(
            "Component not visible for click: " + componentId);
    }
}
```

**RpcServer dispatch changes** (RpcServer.java line 170-173):

```java
case "click":
    boolean forceClick = paramsObj.has("forceInteract")
        && paramsObj.get("forceInteract").getAsBoolean();
    ActionExecutor.click(
        paramsObj.get("componentId").getAsInt(),
        forceClick
    );
    return JsonNull.INSTANCE;
```

Equivalent changes for `doubleClick`, `rightClick`, and `typeText`.

---

## 6. Anti-Corruption Layer

### 6.1 Python Layer: RF Time String Translation

The Python layer serves as the anti-corruption layer (ACL) between Robot Framework's
string-based API and the Rust domain model. Currently, timeout parameters accept only
`float` seconds (see `__init__.py` line 806).

**Current** (`python/JavaGui/__init__.py` line 806):

```python
def wait_until_element_is_visible(self, locator: str, timeout: Optional[float] = None):
    timeout_val = timeout if timeout is not None else self._timeout
    self._lib.wait_until_element_is_visible(locator, timeout_val)
```

**Proposed** (with RF time string support):

```python
from robot.utils import timestr_to_secs

def _parse_timeout(self, timeout) -> float:
    """Anti-corruption layer: translate RF time strings to float seconds.

    Accepts:
      - None -> use session default
      - float/int -> seconds directly
      - str -> RF time string ("10s", "1 minute", "500ms")
    """
    if timeout is None:
        return self._timeout
    if isinstance(timeout, (int, float)):
        return float(timeout)
    if isinstance(timeout, str):
        return timestr_to_secs(timeout)
    raise TypeError(f"Invalid timeout type: {type(timeout)}")

def wait_until_element_is_visible(self, locator: str, timeout=None):
    timeout_val = self._parse_timeout(timeout)
    self._lib.wait_until_element_is_visible(locator, timeout_val)
```

The `_parse_timeout` helper is the ACL function. It ensures that by the time the
timeout crosses from Python into Rust (via PyO3), it is always a validated `f64`.
The Rust side wraps this in `TimeoutValue::from_seconds()`.

**All keywords accepting timeout parameters must use `_parse_timeout`**:

| Python Keyword | Current Type | New Type |
|---------------|-------------|----------|
| `wait_until_element_is_visible` | `Optional[float]` | `Optional[Union[float, str]]` |
| `wait_until_element_is_enabled` | `Optional[float]` | `Optional[Union[float, str]]` |
| `wait_until_element_exists` | `Optional[float]` | `Optional[Union[float, str]]` |
| `wait_until_element_does_not_exist` | `Optional[float]` | `Optional[Union[float, str]]` |
| `connect_to_application` (timeout) | `float` | `Union[float, str]` |

### 6.2 Python Layer: force_interact Propagation

The `force_interact` parameter propagates through three layers.

```
Robot Framework Test:
  Click Element    locator    force=True
       |
       v
Python ACL (__init__.py):
  def click_element(self, locator, click_count=1, force=False):
      self._lib.click_element(locator, click_count, force)
       |
       | PyO3 bridge (force: bool -> bool)
       v
Rust Domain (swing_library.rs):
  #[pyo3(signature = (locator, click_count=1, force_interact=false))]
  pub fn click_element(&self, locator: &str, click_count: u32,
                        force_interact: bool) -> PyResult<()> {
      let component_id = self.get_component_id(locator)?;
      let mut params = serde_json::json!({
          "componentId": component_id
      });
      if force_interact {
          params["forceInteract"] = serde_json::Value::Bool(true);
      }
      self.send_rpc_request("click", params)?;
      Ok(())
  }
       |
       | JSON-RPC over TCP
       v
Java Agent (ActionExecutor.java):
  public static void click(int componentId, boolean forceInteract) {
      Component component = getComponent(componentId);
      ensureVisible(component, forceInteract);
      // ... dispatch click
  }
```

**Propagation path summary**:

```
RF keyword arg (force=True)
  -> Python ACL: translates kwarg name (force -> force_interact)
  -> PyO3 bridge: bool -> bool (zero-cost)
  -> Rust domain: builds RPC params with forceInteract field
  -> JSON-RPC wire: {"forceInteract": true}
  -> Java RpcServer: paramsObj.get("forceInteract").getAsBoolean()
  -> Java ActionExecutor: ensureVisible(component, forceInteract)
```

---

## 7. Event Sourcing

### 7.1 New Domain Events

```rust
/// New domain events for visibility fallback tracking
pub enum DomainEvent {
    // ... existing events from ADR-001 ...

    /// Emitted when the dispatcher falls back from Tier 1 to Tier 2 visibility.
    /// This tracks cases where isShowing()=false but the interaction proceeds
    /// because isVisible()=true. Useful for identifying problematic containers.
    VisibilityFallback {
        /// Locator used to find the element
        locator: String,
        /// Component hash code
        element_id: i64,
        /// Component class name (e.g., "com.jgoodies.uif2.SplitView")
        class_name: String,
        /// Which strategy was ultimately used
        strategy_used: VisibilityStrategy,
        /// Timestamp of the fallback
        timestamp: Instant,
    },

    /// Emitted when a force_interact override is used.
    /// For audit/debugging: tracks explicit visibility bypasses.
    ForceInteractUsed {
        /// Locator used
        locator: String,
        /// Component hash code
        element_id: i64,
        /// The keyword that was force-interacted
        keyword: String,
        /// Timestamp
        timestamp: Instant,
    },

    /// Emitted when visibility assertion semantics differ.
    /// Tracks the behavioral change from isShowing to isVisible.
    VisibilityAssertionChanged {
        /// Locator checked
        locator: String,
        /// Whether the old strict check would have passed
        strict_would_pass: bool,
        /// Whether the new lenient check passed
        lenient_passed: bool,
        /// Component's VisibilityState
        visibility_state: VisibilityState,
        /// Timestamp
        timestamp: Instant,
    },
}
```

### 7.2 Event Flow Diagram

```
Click Element    locator    force=False
         |
         v
+-------------------+
| Locate Element    |  -> ElementResolved event
+-------------------+
         |
         v
+-------------------+
| Check Visibility  |
+-------------------+
    |         |
    v         v
 [showing]  [visible,
 =true      !showing]
    |         |
    |    +----+----+
    |    | Emit    |
    |    | Visibi- |  -> VisibilityFallback event
    |    | lityFall|     (logged, counted in metrics)
    |    | back    |
    |    +---------+
    |         |
    +----+----+
         |
         v
+-------------------+
| Send RPC Click    |
+-------------------+
         |
         v
+-------------------+
| Result            |  -> KeywordExecuted event
+-------------------+
         |
    +----+------+------+
    |           |      |
    v           v      v
+--------+ +--------+ +--------+
| Logger | | Metric | | Screen |
| (RF    | | Collec | | shot   |
|  log)  | | tor    | | Handler|
+--------+ +--------+ +--------+
              |
              v
       VisibilityFallback
       counter incremented
       (observable in test
        report summary)
```

**Event subscribers**:

| Subscriber | Consumes | Action |
|-----------|----------|--------|
| RF Logger | VisibilityFallback | Writes WARN to Robot Framework log |
| Metrics Collector | VisibilityFallback | Increments `visibility_fallback_count` counter |
| Metrics Collector | ForceInteractUsed | Increments `force_interact_count` counter |
| Debug Logger | VisibilityAssertionChanged | Writes DEBUG entry showing old vs new behavior |

---

## 8. File-Level Change Matrix

### 8.1 Java Agent (Phase 1)

| File | Bounded Context | Change Type | Est. LOC | Description |
|------|----------------|-------------|----------|-------------|
| `agent/src/main/java/com/robotframework/swing/ActionExecutor.java` | Infrastructure | Modify | ~50 | Tiered `ensureVisible()` with forceInteract overload; update `click()`, `doubleClick()`, `rightClick()`, `typeText()` to accept forceInteract |
| `agent/src/main/java/com/robotframework/swing/RpcServer.java` | Infrastructure | Modify | ~25 | Extract `forceInteract` from params in `click`, `doubleClick`, `rightClick`, `typeText` dispatch cases; update `waitUntilVisible` for lenient mode |

### 8.2 Rust Layer (Phase 2)

| File | Bounded Context | Change Type | Est. LOC | Description |
|------|----------------|-------------|----------|-------------|
| `src/python/swing_library.rs` | Keyword Execution / Assertion Engine | Modify | ~80 | Add `force_interact` param to `click_element`, `right_click_element`; change `element_should_be_visible` to `visible` only; add `element_should_be_showing`; add `wait_until_element_is_showing`; fix `wait_until_element_is_visible` condition |
| `src/python/element.rs` | Introspection | None | 0 | No changes needed. `SwingElement` already exposes both `visible` and `showing` fields. |
| `src/model/component.rs` | Shared Kernel | None | 0 | No changes needed. `ComponentState` already has both `visible` and `showing` fields. |

### 8.3 Python Layer (Phase 3)

| File | Bounded Context | Change Type | Est. LOC | Description |
|------|----------------|-------------|----------|-------------|
| `python/JavaGui/__init__.py` | ACL / Keyword Execution | Modify | ~60 | Add `_parse_timeout` helper; add `force` param to click/input keywords; expose `Element Should Be Showing` and `Wait Until Element Is Showing`; update all timeout params to accept RF time strings |

### 8.4 New Files (if full DDD restructure is active)

| File | Bounded Context | Change Type | Est. LOC | Description |
|------|----------------|-------------|----------|-------------|
| `src/domain/value_objects/visibility.rs` | Shared Kernel | New | ~80 | `VisibilityStrategy`, `VisibilityState`, `InteractionMode` enums |
| `src/domain/value_objects/timeout.rs` | Shared Kernel | New | ~50 | `TimeoutValue` wrapper |
| `src/domain/events/visibility_events.rs` | Domain Events | New | ~40 | `VisibilityFallback`, `ForceInteractUsed`, `VisibilityAssertionChanged` events |

**Note**: If the full DDD module restructure (ADR-001 Phase 1) has not yet been
executed, these value objects should be defined inline in the existing files where
they are consumed. The module restructure can extract them later.

### 8.5 Test Files (Phase 4)

| File | Bounded Context | Change Type | Est. LOC | Description |
|------|----------------|-------------|----------|-------------|
| `tests/unit/test_visibility_strategy.rs` | Domain | New | ~60 | Unit tests for `VisibilityState.interaction_tier()`, `VisibilityStrategy` defaults |
| `tests/unit/test_timeout_value.rs` | Domain | New | ~40 | Unit tests for `TimeoutValue::from_seconds()` edge cases |
| `tests/python/test_timeout_parsing.py` | ACL | New | ~40 | Tests for `_parse_timeout` with RF time strings |
| `tests/robot/visibility/tiered_click.robot` | Integration | New | ~50 | RF tests for click on visible-but-not-showing components |

### 8.6 Summary

| Category | Files Modified | Files New | Total Est. LOC |
|----------|---------------|-----------|---------------|
| Java Agent | 2 | 0 | ~75 |
| Rust Layer | 1 | 0-3 | ~80-210 |
| Python Layer | 1 | 0 | ~60 |
| Tests | 0 | 3-4 | ~150-190 |
| **Total** | **4** | **3-7** | **~365-535** |

---

## Appendix A: Behavioral Change Summary

| Keyword | Before | After | Breaking? |
|---------|--------|-------|-----------|
| `Element Should Be Visible` | Fails if `!showing` | Passes if `visible` (regardless of `showing`) | YES -- loosens check |
| `Element Should Not Be Visible` | Passes if `!visible OR !showing` | Passes only if `!visible` | YES -- tightens check |
| `Wait Until Element Is Visible` | Requires `visible && showing` | Requires `visible` only | YES -- resolves faster |
| `Element Should Be Showing` | N/A (new) | Fails if `!showing` | NO -- new keyword |
| `Wait Until Element Is Showing` | N/A (new) | Requires `visible && showing` | NO -- new keyword |
| `Click Element` | Throws if `!showing` | Warns + synthetic dispatch if `visible && !showing` | YES -- loosens gate |
| `Click Element ... force=True` | N/A (new param) | Skips `isShowing()` entirely | NO -- opt-in |
| All wait keywords | Accept `float` only | Accept `float` or RF time string | NO -- superset |

## Appendix B: Migration Notes

For existing tests that relied on the strict `isShowing()` behavior of
`Element Should Be Visible`, the migration path is:

```robotframework
*** Old (strict) ***
Element Should Be Visible    locator

*** New (equivalent strict) ***
Element Should Be Showing    locator
```

For tests that were broken by `isShowing()=false` on JGoodies SplitView components,
no migration is needed -- the fix makes their existing tests pass.

---

*Document Version: 1.0*
*Status: DRAFT*
*Last Updated: 2026-02-25*
