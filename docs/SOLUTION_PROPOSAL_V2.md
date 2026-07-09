# Solution Proposal V2: Findings from Swing Automation Report v2

**Date:** 2026-02-25
**Status:** DRAFT — Awaiting approval before implementation
**Based on:** `docs/JAVAGUI_SWING_AUTOMATION_REPORT_v2.md`

---

## 1. Research Methodology

Five parallel research agents investigated the v2 report findings:

1. **force=True RPC passthrough** — traced parameter from Python through Rust to Java agent
2. **Visibility keyword semantics** — verified v1 fix landed in compiled code
3. **Connection stability** — analyzed TCP socket timeouts and keepalive gaps
4. **Component tree scoping** — traced why locator param is ignored
5. **Live experiments** — ran 6 experiments against Smart Client Showcase 24.09.0

All findings below are backed by code analysis with exact file paths/line numbers AND live experimental validation.

---

## 2. Issues Summary

| # | Issue | Severity | Root Cause | Fix Complexity |
|---|-------|----------|-----------|----------------|
| 1 | `__version__` reports 0.1.0, package is 0.4.0 | **P0** | Hardcoded string in `__init__.py:149` | Trivial |
| 2 | Connection drops after ~30s idle (Broken pipe) | **P0** | 30s socket timeout on both sides, no keepalive | Medium |
| 3 | `Get Component Tree` ignores locator | **P1** | Rust discards component ID, never sends to Java | Medium |
| 4 | Default port 5678 vs agent port 18080 | **P1** | Agent default differs from library default | Trivial |
| 5 | `component_should_be_visible` is misleading | **P1** | Deprecated alias that doesn't assert visibility | Low |
| 6 | Missing `Element Should Not Be Showing` keyword | **P2** | Asymmetric API — showing pair incomplete | Low |
| 7 | Duplicate keywords (8+ pairs) | **P2** | No deprecation strategy | Low |
| 8 | Agent JAR has no version info | **P2** | No manifest version | Low |
| 9 | Missing feature keywords (6 requested) | **P3** | Not implemented | Varies |

### v2 Report Corrections

Two issues from the v2 report were **resolved by investigation**:

| Reported Issue | Finding |
|---------------|---------|
| **`force=True` ineffective** | The parameter chain is COMPLETE and CORRECT in source code (Python→Rust→Java). Live experiment confirmed clicks work on non-showing elements BOTH with and without `force=True` (Tier 2 fallback handles it). The v2 tester likely used a PyPI wheel built before the fix. |
| **`Element Should Be Visible` uses isShowing()** | INCORRECT. The v1 fix DID land: `element_should_be_visible` checks `visible` only (line 1361 in swing_library.rs). Live experiment confirmed: `Element Should Be Visible` PASSES for visible-but-not-showing elements. |
| **`Component Should Be Visible` exists** | DOES NOT EXIST as a keyword. No function in Rust, no method in Python. The v2 report likely tested a different keyword or version. |

---

## 3. Issue Details & Proposed Fixes

### 3.1 Version Mismatch (P0)

**Root Cause:** `__version__` is hardcoded at `python/JavaGui/__init__.py:149`:
```python
__version__ = "0.1.0"
```

`ROBOT_LIBRARY_VERSION` at line 173 references `__version__`, so both are wrong.

**Proposed Fix:**
```python
# Replace hardcoded version with dynamic lookup
try:
    from importlib.metadata import version as _pkg_version
    __version__ = _pkg_version("robotframework-javagui")
except Exception:
    __version__ = "0.0.0"  # fallback for editable installs
```

**Impact:** Robot Framework logs, libdoc, and `Get Library Instance` will report correct version.

**Files:** `python/JavaGui/__init__.py` (1 line changed)

---

### 3.2 Connection Drops After 30s Idle (P0)

**Root Cause (confirmed by code analysis):**

| Component | Setting | File:Line |
|-----------|---------|-----------|
| Java agent | `socket.setSoTimeout(30000)` | `RpcServer.java:65` |
| Rust client | `set_read_timeout(Duration::from_secs(30))` | `connection/mod.rs:202` |
| Rust client | `set_read_timeout(Duration::from_secs(30))` | `base_library.rs:216,959` |

Both sides timeout after 30s of inactivity. The Java side's `readLine()` throws `SocketTimeoutException` → `IOException` → connection closed. No TCP_KEEPALIVE, no heartbeat, no auto-reconnect.

**Proposed Fix (3 parts):**

**A. Increase socket timeout + add TCP keepalive:**

Java agent (`RpcServer.java`):
```java
socket.setSoTimeout(300000);  // 5 minutes (was 30s)
socket.setKeepAlive(true);    // Enable TCP keepalive
```

Rust client (`connection/mod.rs` and `base_library.rs`):
```rust
stream.set_read_timeout(Some(Duration::from_secs(300))).ok();  // 5 minutes
stream.set_write_timeout(Some(Duration::from_secs(300))).ok();
// Enable TCP keepalive via socket2 crate
```

**B. Add periodic ping keepalive in Rust:**

The `Ping` RPC method already exists (`protocol/mod.rs:50`, `RpcServer.java:119`) but is never automatically invoked.

```rust
// In connection module: spawn background keepalive thread
fn start_keepalive(stream: &TcpStream, interval: Duration) {
    // Send "ping" RPC every `interval` seconds when idle
    // Reset timer on any successful RPC call
}
```

**C. Add auto-reconnect on broken pipe:**

```rust
fn send_rpc_request(&self, method: &str, params: Value) -> PyResult<Value> {
    match self.send_rpc_request_inner(method, params.clone()) {
        Err(e) if is_broken_pipe(&e) => {
            // Auto-reconnect once, then retry
            self.reconnect()?;
            self.send_rpc_request_inner(method, params)
        }
        result => result
    }
}
```

**Files:**
- `agent/src/main/java/com/robotframework/swing/RpcServer.java` (~5 lines)
- `agent/src/main/java/com/robotframework/swt/SwtRpcServer.java` (~5 lines)
- `src/connection/mod.rs` (~30 lines)
- `src/python/base_library.rs` (~40 lines)
- `Cargo.toml` (add `socket2` dependency for keepalive)

---

### 3.3 Get Component Tree Locator Scoping (P1)

**Root Cause (confirmed by code tracing across all 3 layers):**

| Layer | File:Lines | Issue |
|-------|-----------|-------|
| Python | `__init__.py:1591-1610` | Warns locator unsupported, but passes it anyway |
| Rust | `swing_library.rs:1606-1616` | Calls `find_element(loc)` but DISCARDS component ID; both if/else branches call same full-tree function |
| Rust RPC | `swing_library.rs:2886-2894` | `fetch_tree_from_agent()` only sends `maxDepth`, never `componentId` |
| Java | `RpcServer.java:129-140` | HAS scoped variant `getComponentTree(componentId, maxDepth)` — but never receives componentId |

The Java agent already supports scoped tree retrieval. The fix is entirely in the Rust layer.

**Proposed Fix:**

```rust
// swing_library.rs - get_component_tree()
let tree = if let Some(loc) = locator {
    let element = self.find_element(loc)?;
    let component_id = element.component_id;  // Extract ID instead of discarding
    self.fetch_scoped_tree_from_agent(component_id, max_depth)?
} else {
    self.get_or_refresh_tree_with_depth(max_depth)?
};

// New helper
fn fetch_scoped_tree_from_agent(&self, component_id: i64, max_depth: Option<u32>) -> PyResult<UITree> {
    let params = serde_json::json!({
        "componentId": component_id,
        "maxDepth": max_depth.unwrap_or(10)
    });
    let result = self.send_rpc_request("getComponentTree", params)?;
    // Parse into UITree rooted at the matched component
}
```

Also: remove the misleading `DeprecationWarning` from `__init__.py:1594-1598`.

**Files:**
- `src/python/swing_library.rs` (~20 lines changed)
- `python/JavaGui/__init__.py` (~5 lines removed)

---

### 3.4 Default Port Mismatch (P1)

**Root Cause:** Library defaults to port 5678 (`__init__.py:349`). The bundled agent defaults to 18080 (`RpcServer.java` startup).

**Proposed Fix:** Align defaults. Two options:

**Option A (Preferred):** Change library default to match agent:
```python
# __init__.py line 349
def connect_to_application(self, ..., port: int = 18080, ...):
```

**Option B:** Change agent to match library:
```java
// RpcServer.java
private static final int DEFAULT_PORT = 5678;
```

Option A is preferred because the agent port is configurable via JVM args, but changing the library default is less disruptive.

**Files:** `python/JavaGui/__init__.py` (1 line)

---

### 3.5 `component_should_be_visible` Misleading (P1)

**Root Cause:** The keyword exists as a deprecated alias that calls `get_element_states()` and returns states list without asserting anything about visibility.

**Proposed Fix:** Make it actually assert, then deprecate properly:
```python
@keyword("Component Should Be Visible")
def component_should_be_visible(self, locator: str) -> None:
    """Deprecated: Use 'Element Should Be Visible' instead."""
    import warnings
    warnings.warn(
        "Component Should Be Visible is deprecated. Use Element Should Be Visible.",
        DeprecationWarning, stacklevel=2
    )
    self.element_should_be_visible(locator)
```

**Files:** `python/JavaGui/__init__.py` (~10 lines)

---

### 3.6 Missing `Element Should Not Be Showing` (P2)

**Root Cause:** Asymmetric API — `element_should_be_showing` exists but its negative counterpart does not.

**Proposed Fix:**

Rust (`swing_library.rs`):
```rust
pub fn element_should_not_be_showing(&self, locator: &str) -> PyResult<()> {
    self.ensure_connected()?;
    match self.find_elements_internal(locator) {
        Ok(elements) if elements.is_empty() => Ok(()),
        Ok(elements) => {
            let element = &elements[0];
            if element.visible && element.showing {
                Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                    "Element '{}' is showing (visible={}, showing={})",
                    locator, element.visible, element.showing
                )))
            } else {
                Ok(())
            }
        }
        Err(_) => Ok(()),
    }
}
```

Python (`__init__.py`):
```python
def element_should_not_be_showing(self, locator: str) -> None:
    """Verify element is NOT showing (isShowing=false or does not exist)."""
    self._lib.element_should_not_be_showing(locator)
```

**Files:**
- `src/python/swing_library.rs` (~15 lines)
- `python/JavaGui/__init__.py` (~5 lines)

---

### 3.7 Duplicate Keywords (P2)

**8+ duplicate pairs identified:**

| Keep (preferred) | Deprecate | Reason |
|-----------------|-----------|--------|
| `Wait Until Element Is Visible` | `Wait Until Element Visible` | RF convention uses "Is" |
| `Wait Until Element Is Enabled` | `Wait Until Element Enabled` | Same |
| `Get Element Text` | `Get Text`, `Get Component Text` | "Element" prefix consistent |
| `Get Element Property` | `Get Property` | Same |
| `Get Element Properties` | `Get Properties` | Same |
| `Get List Item Count` | `Get Number Of List Items` | Shorter name |
| `Get Table Row Count` | `Get Number Of Table Rows` | Same |
| `Get Table Column Count` | `Get Number Of Table Columns` | Same |

**Proposed Fix:** Add `@deprecated` decorator (or warning) to the deprecated variants. Keep them functional for backward compatibility through v0.5.x, remove in v1.0.0.

**Files:** `python/JavaGui/__init__.py` (~20 lines of deprecation warnings)

---

### 3.8 Agent JAR Versioning (P2)

**Proposed Fix:** Add version to JAR manifest:
```xml
<!-- agent/pom.xml -->
<manifestEntries>
    <Agent-Version>${project.version}</Agent-Version>
</manifestEntries>
```

And print on startup:
```java
System.out.println("[JavaGui Agent] v" + version + " listening on port " + port);
```

**Files:**
- `agent/pom.xml` (~5 lines)
- `agent/src/main/java/.../RpcServer.java` (~3 lines)

---

### 3.9 Missing Feature Keywords (P3)

| Keyword | Complexity | Implementation Notes |
|---------|-----------|---------------------|
| `Scroll Element Into View` | Medium | Java agent: find parent JScrollPane, call `scrollRectToVisible()` |
| `Get Window Title` | Low | Convenience: `Get Element Property JFrame title` |
| `Maximize Window` / `Minimize Window` | Low | Java agent: `frame.setExtendedState(JFrame.MAXIMIZED_BOTH)` |
| `Wait Until Page Contains` | Medium | Search UI tree text content with polling |
| `Get All Element Texts` | Low | `find_elements()` + map to `.text` |
| `Element Should Be Focused` | Low | Check `focused` property from component state |

**Deferred to future release.** These are enhancements, not bug fixes.

---

## 4. Implementation Plan

### Phase 1: Quick Wins (P0 + trivial P1)

| Step | File | Change | Est. LOC |
|------|------|--------|----------|
| 1.1 | `python/JavaGui/__init__.py` | Dynamic `__version__` via `importlib.metadata` | 5 |
| 1.2 | `python/JavaGui/__init__.py` | Change default port to 18080 | 1 |
| 1.3 | Agent rebuild | `mvn package` | — |

### Phase 2: Connection Stability (P0)

| Step | File | Change | Est. LOC |
|------|------|--------|----------|
| 2.1 | `RpcServer.java` | Increase timeout to 300s, add keepalive | 5 |
| 2.2 | `SwtRpcServer.java` | Same | 5 |
| 2.3 | `src/connection/mod.rs` | Increase timeout, add TCP keepalive | 15 |
| 2.4 | `src/python/base_library.rs` | Increase timeout, add auto-reconnect | 40 |
| 2.5 | `Cargo.toml` | Add `socket2` dependency | 1 |

### Phase 3: Component Tree Scoping (P1)

| Step | File | Change | Est. LOC |
|------|------|--------|----------|
| 3.1 | `src/python/swing_library.rs` | Pass componentId to `fetch_tree_from_agent` | 20 |
| 3.2 | `python/JavaGui/__init__.py` | Remove misleading DeprecationWarning | 5 |

### Phase 4: API Cleanup (P1-P2)

| Step | File | Change | Est. LOC |
|------|------|--------|----------|
| 4.1 | `src/python/swing_library.rs` | Add `element_should_not_be_showing` | 15 |
| 4.2 | `python/JavaGui/__init__.py` | Add `Element Should Not Be Showing` keyword | 5 |
| 4.3 | `python/JavaGui/__init__.py` | Fix `component_should_be_visible` to actually assert | 10 |
| 4.4 | `python/JavaGui/__init__.py` | Add deprecation warnings to 8 duplicate keywords | 20 |
| 4.5 | `agent/pom.xml` + `RpcServer.java` | Agent JAR versioning + startup logging | 10 |

### Phase 5: Testing & Validation

| Step | Scope | Method |
|------|-------|--------|
| 5.1 | Unit tests | Rust tests for new/changed functions |
| 5.2 | RF dryrun | `robot --dryrun` to verify keyword signatures |
| 5.3 | Python tests | `pytest tests/python/ tests/unit/` |
| 5.4 | Rust tests | `cargo test` |
| 5.5 | Integration | Robot tests against Smart Client Showcase |

---

## 5. Estimated Totals

| Phase | Files | Est. LOC | Risk |
|-------|-------|----------|------|
| Phase 1: Quick Wins | 1 | 6 | LOW |
| Phase 2: Connection Stability | 5 | 66 | MEDIUM |
| Phase 3: Tree Scoping | 2 | 25 | LOW |
| Phase 4: API Cleanup | 4 | 60 | LOW |
| Phase 5: Testing | test files | ~80 | — |
| **Total** | **~8 files** | **~237 lines** | |

---

## 6. Risk Assessment

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| Auto-reconnect loses state (e.g., cached component IDs) | MEDIUM | Invalidate component cache on reconnect; document behavior |
| Changing default port breaks existing users | LOW | Document in changelog; old port still works if agent configured |
| Deprecation warnings annoying in test output | LOW | Use `DeprecationWarning` (filtered by default in Python 3.2+) |
| Keepalive thread interference with EDT dispatch | LOW | Ping is read-only, no UI interaction |
| Component tree scoping returns stale cached tree | LOW | Always fetch fresh tree when locator is specified |

---

## 7. Out of Scope (Deferred)

- Feature request keywords (Scroll Into View, Get Window Title, etc.) — Phase 2 roadmap
- Full keyword deprecation removal — v1.0.0 release
- Agent port auto-detection (scan 5678/18080/8080) — nice-to-have, not critical
- UI Tree diff comparison — enhancement, not bug fix
- Element-scoped screenshots — enhancement
