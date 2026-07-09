# Release Notes — v0.5.0

This release makes `robotframework-javaui` honest and verified across all three toolkits,
and — for the first time — proves RCP automation against a **real Eclipse workbench**.

## Highlights

### RCP now works against real Eclipse (not just a mock)
Previously every RCP suite ran only against an in-memory simulation (`MockRcpApplication`),
and the workbench-introspection keywords returned `{"error":"Eclipse RCP not available"}` in
every tested configuration. Two agent bugs were fixed:

- **OSGi classloader resolution** — `EclipseWorkbenchHelper` resolved Eclipse classes with the
  agent's own classloader, which cannot see OSGi bundle classes. It now discovers the
  `org.eclipse.ui` bundle classloader via the agent Instrumentation.
- **SWT UI-thread affinity** — active-window/page queries returned `null` off the SWT UI thread;
  they now run on the UI thread via `SwtReflectionBridge.syncExec`.

`Get Open Views`, `Get Available Perspectives`, `Get Active Perspective Id`, and
`Get Workbench Info` now return live data from a real Eclipse RCP application. See
`tests/apps/rcp/build-and-run-real-eclipse.sh` and `tests/robot/rcp/real_eclipse/`.

### Keyword honesty
- **BREAKING:** `List Applications` no longer returns a misleading empty list; it raises a clear
  error directing you to `Connect To Application` with an explicit port.
- `Type Text` documentation corrected — it appends via the `Input Text` path and does not simulate
  per-character key events (the previous docstring overpromised).
- `Log UI Tree` now honors its `locator` argument instead of ignoring it.

### Fixed keywords (were silently broken)
`Get Widget Property`, `Get Widget Properties`, `Get Widget States`, and `Get Swt Tree Node Count`
delegated to a Rust method that does not exist and either crashed or returned empty/degraded data.
They now read the real widget property map via `find_widget().to_dict()`.

### Verified end-to-end coverage
Keyword E2E coverage rose from ~66% to **96.2% (230/239)** against live Swing/SWT/RCP apps:
Swing 93.5%, SWT 95.8%, RCP 100%. A reproducible checker (`scripts/keyword_coverage.py`,
`--min 90` gate) enforces this. Live results: Swing 171+ pass, SWT 238 pass, RCP-mock 248 pass,
real-Eclipse RCP 4/4.

### Documentation & repository
- README now carries an honest per-toolkit **maturity table** (Swing/SWT stable; RCP requires a
  real Eclipse workbench) instead of implying parity.
- `docs/` reduced from 146 to ~82 files (throwaway process/mission/phase reports removed);
  generated keyword libdoc regenerated from current source.
- Tracked `.claude-flow/` runtime files untracked; `example-apps/*.jar` gitignored.

## Known issues (tracked for a follow-up release)
- **Connection stability under heavy load:** rapid keyword sequences can still hit
  `Broken pipe (os error 32)`; wrap long flows with a reconnect guard (`Is Connected`).
- A few keywords remain backed by missing/incorrect Rust methods and are not yet functional:
  `Swt Tree Node Should Exist` / `tree_node_exists` (always false), `Get Swt Tree Node Count`
  (returns 0 — reads child count, not item count), `List Selection Should Be` /
  `Get Selected List Item` (empty/None), `Log Component Tree` (forwards `format=None`).
  These are documented and their tests are intentionally not asserted green.
- `get_all_rcp_views/editors` and `get_rcp_component_tree` are exposed on `SwingLibrary` rather
  than `RcpLibrary` (API inconsistency) and `RcpComponentInspector` still needs the same
  classloader fix that `EclipseWorkbenchHelper` received.
