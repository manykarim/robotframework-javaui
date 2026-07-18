# Tasks

Status: implemented + verified against real DBeaver CE 26.1.2 (strict oracle suite
`tests/robot/rcp/real_dbeaver/experiment.robot` → **8/8 pass**; `Invalid thread access`
count in the product log → **0**). Unit regression clean: Rust **245 pass**, Python **617 pass**.

## 1. Docker harness + proof suite
- [x] 1.1 `tests/docker/rcp/Dockerfile` — ubuntu:24.04 + Xvfb + GTK3 + ImageMagick + full JDK 21 + DBeaver CE 26.1.2
- [x] 1.2 `tests/docker/rcp/entrypoint.sh` — Xvfb, repoint `-vm` to full JDK, attach agent, wait for rendered workbench, run suite, screenshots, chown results
- [x] 1.3 `tests/robot/rcp/real_dbeaver/` — strict evidence-first oracle (introspection + actions, read-back + screenshot per step), self-skips without the harness
- [x] 1.4 Baseline + after-fix evidence under `evidence/` (workbench-live, invalid-thread-access + stack, after-fix-no-thread-error, library-capture-screenshot-works)
- [ ] 1.5 Pre-dismiss the DBeaver "Data share" modal for deterministic runs (deferred — non-blocking; introspection/actions work behind it)
- [ ] 1.6 Optional opt-in CI job that builds the image and runs the suite

## 2. F1 — Agent attach to packaged products (harness/doc)
- [x] 2.1 Documented the "bundled JRE lacks `java.instrument`" gotcha + the `-vm <full-jdk>` fix (`tests/docker/rcp/README.md`, entrypoint)
- [ ] 2.2 Generalize the `-vm` insertion to any product `.ini` (parameterize product path + required Java version) — currently DBeaver-specific

## 3. F2 — Lifecycle-aware SWT readiness (agent, P0) — VERIFIED
- [x] 3.1 `SwtReflectionRpcServer` returns typed `SWT_NOT_READY` before the Display exists; Display re-resolved per-RPC via `getAllLoadedClasses()`
- [x] 3.2 Discovery prefers `getDisplays()`/`findDisplay(uiThread)` over `getDefault()` (no rogue off-thread Display)
- [x] 3.3 `waitForSwtReady(timeoutMs)` RPC + `Wait Until Swt Ready` keyword (oracle: PASS → ready=True)
- [x] 3.4 Rcp `connect_to_swt_application` retries while error contains `SWT_NOT_READY` up to the connect timeout
- [ ] 3.5 Dedicated unit/integration test for premain-connect race (covered end-to-end by the harness; no isolated unit test yet)

## 4. F3 — UI-thread execution for all state-changing ops (agent, P0) — VERIFIED
- [x] 4.1 `EclipseWorkbenchHelper`: showView/hideView/openPerspective/resetPerspective/activateView/executeCommand/close-all/save-all wrapped in one `syncExec` (lookup+invoke+readback)
- [x] 4.2 Exceptions propagate out of `syncExec` (no false-success)
- [x] 4.3 Audited handlers; getters left off-thread per prior verification (documented)
- [x] 4.4 Real DBeaver: `Execute Command …preferences` runs clean, **0** `Invalid thread access` (before-fix screenshot vs after-fix screenshot in `evidence/`)

## 5. F4/F5 — Honest view/perspective actions (agent, P1) — VERIFIED
- [x] 5.1 show/open delegate to the live-registry real-Eclipse path; "not found" only when genuinely unregistered
- [x] 5.2 `Show View <valid id>` restores the view (read-back contains it) on real DBeaver
- [~] 5.3 `Open Perspective` invokes the real API on the UI thread without error; DBeaver is effectively **single-perspective** and keeps its own — documented as an app characteristic, not a library defect (perspective *switch* read-back to be re-verified on the `com.testapp.rcp` app, which defines switchable perspectives)
- [x] 5.4 `Close View` confirms the view is gone by read-back; no false-success no-op (verified: after-close list no longer contains the view)

## 6. F6 — Visual confirmation end-to-end (rust + python + agent) — VERIFIED
- [x] 6.1 `swing_library.rs::capture_screenshot`: real RPC → decode base64 PNG → write file (stub removed)
- [x] 6.2 `capture_screenshot` added to `swt_library.rs`/`rcp_library.rs`; **`SwtReflectionBridge.captureScreenshotDataUrl` + `SwtReflectionRpcServer` handler added** (the RCP reflection server had no capture handler)
- [x] 6.3 `Capture Screenshot` exposed on `JavaGui.Rcp` and `JavaGui.Swt`
- [x] 6.4 Saved PNG embedded in the Robot log (`logger.info(<img>, html=True)`)
- [x] 6.5 Verified: `Capture Screenshot` produced a real 1600×1000 PNG of the DBeaver workbench (`evidence/library-capture-screenshot-works.png`)

## 7. F7 — RCP inspector API consistency (python, P2)
- [x] 7.1 `Get Rcp Component Tree` / `Get All Rcp Views` / `Get All Rcp Editors` exposed on `JavaGui.Rcp` (oracle: keywords present, no "No keyword" error)
- [ ] 7.2 Apply OSGi-bundle-classloader discovery to `RcpComponentInspector` (not required for DBeaver; deferred)

## 8. Validation
- [x] 8.1 Strict harness run: 8/8 pass with read-back AND real screenshots; 0 `Invalid thread access`; no false-success
- [~] 8.2 Regression: Rust 245 pass, Python 617 pass (no regressions). Full live RF mock-RCP / `com.testapp.rcp` suites not re-run here (need live Java apps) — recommended before archive
- [x] 8.3 `openspec validate rcp-real-app-automation --strict` → valid
