# v0.4.1 Release Notes

## Connection Stability
- Increased TCP socket timeout from 30s to 5 minutes on both Java agents (Swing/SWT) and Rust client
- Enabled TCP keepalive to prevent idle connection drops
- Added auto-reconnect on broken pipe — transparently re-establishes the TCP connection and retries the RPC request

## Component Tree Scoping
- `Get Component Tree` now supports the `locator` parameter to return a scoped subtree instead of the full tree
- `Save UI Tree` with a locator saves only the matching subtree
- Passes `componentId` to the Java agent's `getComponentTree` RPC for server-side filtering

## New Keywords
- **Element Should Not Be Showing** — strict negative assertion using `isShowing()` + `isVisible()`

## API Cleanup
- `Wait Until Element Visible` and `Wait Until Element Enabled` now emit `DeprecationWarning` directing users to the `Is` variants
- Dynamic `__version__` via `importlib.metadata` (no more hardcoded version string)
- `Agent-Version` added to Java agent JAR manifest

## Bug Fixes
- Fixed default port regression (was accidentally changed from 5678 to 18080)

## Test Results
- **692 Robot Framework tests**: 662 passed, 0 failed, 30 skipped
- **22 integration visibility tests**: 22 passed, 0 failed
