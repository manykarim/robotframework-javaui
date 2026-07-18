## Why

The July 2026 work proved the RCP keywords against a **real Eclipse platform runtime** — but running a **bespoke `com.testapp.rcp` bundle we built to be introspectable**. A fair skeptic still asks: does the library drive a *real, third-party, packaged* Eclipse RCP product that nobody designed for testability?

To answer with evidence, we built a reproducible **Docker harness** and drove **DBeaver Community Edition 26.1.2** (a widely-used, Eclipse-4/e4 RCP product) headless under Xvfb, with the agent attached and **framebuffer screenshots** taken after every step. The experiment worked — introspection returned live DBeaver data and a real workbench rendered — but it surfaced **one hard blocker to even attaching**, **one timing race**, and a cluster of **state-changing-action defects that are invisible to assertions but visible on screen**. This change captures the harness as a durable capability and proposes the fixes the evidence demands.

### What the experiment found (all reproduced live against DBeaver CE 26.1.2)

| # | Finding | Evidence |
|---|---------|----------|
| F1 | **Packaged RCP products ship a trimmed JRE without `java.instrument`** → `-javaagent` cannot load (`libinstrument.so: cannot open shared object file`). | Bundled JRE 21.0.10 has no `java.instrument` module; agent only attaches after repointing `-vm` at a full JDK 21. |
| F2 | **The agent's SWT `Display` bootstrap is not lifecycle-aware.** It resolves once, early (premain, before SWT loads), then its retry path scans thread context classloaders (`ContextFinder`) that cannot load `org.eclipse.swt.widgets.Display` → permanent `SWT not initialized`. | `[SwtBridge] Display class not loaded yet` → repeated `No Display found`. Works **only** once the driver waits ~10 s for SWT to load, then `getAllLoadedClasses()` finds it (`EquinoxClassLoader[org.eclipse.swt:3.134…]`). |
| F3 | **State-changing RCP operations run on the RPC thread, not the SWT UI thread** → `org.eclipse.swt.SWTException: Invalid thread access`. `Execute Command` returned `status=PASS` at the RPC layer while DBeaver popped an **"Invalid thread access"** error dialog. | `evidence/invalid-thread-access.png` + `evidence/invalid-thread-access.stack.txt`. In `EclipseWorkbenchHelper`, `syncExec` wraps only `getActiveWindow`/`getActivePage` (:158/:174); `showView`/`showPerspective`/`hideView`/`executeCommand` invoke on the calling thread. |
| F4 | **`Show View` / `Open Perspective` fail with "not found" for valid, registered ids** returned moments earlier by the introspection keywords. | `Show View org.jkiss.dbeaver.core.databaseNavigator` → `RcpError: View not found`; `Open Perspective org.eclipse.ui.resourcePerspective` → `Perspective not found`, active perspective unchanged. |
| F5 | **`Close View` is a false-success no-op** — returns `status=PASS` but the view stays open (read-back id list and `viewCount=4` unchanged). | `close_view status=PASS` then `after_close_ids` still contains the target. |
| F6 | **The library's own visual-confirmation is broken.** `Capture Screenshot` isn't even exposed on `JavaGui.Rcp`, and the Rust binding is a stub that writes no file; the Java agent's working PNG capture is never invoked. | `capture_screenshot … No keyword … found`; `capture_screenshot_produced_file = False`. All visual proof came from an external X-framebuffer grab. |
| F7 | **API inconsistency:** `Get Rcp Component Tree` / `Get All Rcp Views` / `Get All Rcp Editors` are absent from `JavaGui.Rcp` (they live on the Swing library). | `No keyword with name 'Get All Rcp Views' found` while connected as an RCP app. |
| F8 | **First-run friction:** DBeaver's modal **"Data share" consent dialog** sits over the workbench for the whole session; the harness should pre-dismiss it for clean, deterministic runs. | `evidence/workbench-live.png`. |

### What worked (positive evidence)

Once F1+F2 were worked around in the harness, **introspection is fully real** on DBeaver: `Get Workbench Info` → `{info: 'Eclipse RCP Workbench', windowCount: 1, viewCount: 4, activePerspective: 'org.jkiss.dbeaver.core.perspective'}`; `Get Available Perspectives` → the 3 real registry ids; `Get Open Views` → the 4 real views with titles (Connections/Projects/Files/Chat). A genuine DBeaver workbench renders headless (`evidence/workbench-live.png`).

## What Changes

- **Add** the reproducible Docker harness (`tests/docker/rcp/`) and an evidence-first Robot proof suite (`tests/robot/rcp/real_dbeaver/`) that automates DBeaver CE headless, validates introspection + actions by read-back, and captures a framebuffer screenshot after every step. Opt-in / self-skipping so default CI is unaffected.
- **Fix F1 (harness):** document + script the "full JDK via `-vm`" requirement for packaged RCP products whose bundled JRE lacks `java.instrument`.
- **Fix F2 (agent):** make SWT `Display` discovery lifecycle-aware — retry `getAllLoadedClasses()` on each RPC until SWT is loaded; add a `Wait Until SWT Ready` / readiness signal so drivers don't have to guess timing.
- **Fix F3 (agent):** wrap **every** state-changing workbench reflection call in `SwtReflectionBridge.syncExec` so it runs on the SWT UI thread — eliminating `Invalid thread access`.
- **Fix F4/F5 (agent):** resolve views/perspectives against the live workbench registry and report **honest** success/failure — no "not found" for registered ids, no PASS for a no-op.
- **Fix F6 (rust+python):** close the `capture_screenshot` Rust stub, add the SWT/RCP binding, expose `Capture Screenshot` on the RCP/SWT libraries, and embed the PNG into the Robot log for true in-report visual confirmation.
- **Fix F7 (python):** expose the RCP inspector keywords on the RCP library (or clearly deprecate/alias), removing the cross-library inconsistency.

## Capabilities

### New Capabilities
- `rcp-real-app-automation`: a reproducible Docker harness + proof suite that automates a real, third-party packaged Eclipse RCP product (DBeaver CE) headless, validating introspection and actions by read-back **and** framebuffer screenshot, with the assert-and-see-after-every-action contract.
- `rcp-workbench-thread-safety`: state-changing RCP/SWT operations execute on the SWT UI thread and never raise `Invalid thread access` against a real workbench.
- `agent-swt-readiness`: SWT `Display` discovery is lifecycle-aware and exposes a readiness handshake, so an agent attached at premain becomes usable once (and only once) the workbench's Display exists.
- `agent-visual-confirmation`: the `Capture Screenshot` keyword captures a real image end-to-end (agent→rust→python) and embeds it in the Robot log.

### Modified Capabilities
- `rcp-real-eclipse-validation`: strengthened so that view/perspective **action** keywords are validated against a real workbench by read-back (not only introspection), and report honest success/failure.

## Widget-level follow-up findings (second experiment: `tests/robot/rcp/dbeaver_widgets/`)

A second experiment drove DBeaver's **generic SWT controls** (modal, checkbox, button, text
field, combo, menu) with fill/validate/click keywords (`Find Widget`, `Check Button`,
`Click Widget`, `Input Text`, `Widget Text Should Be`, `Select Combo Item`, `Select Main Menu`,
`Get Widget Property`). Suite is 6/6 green (documents outcomes as evidence). New findings:

- **F9 (FIXED — see change `fix-swt-type-locator`):** every `type:<SwtClass>` locator returned
  **0** for the main workbench window (Text/Button/Combo/Composite/Tree/ToolItem all 0), which
  looked like "the main e4 window is unreachable". True root cause: the Rust `parse_locator`
  (`swt_library.rs`, `base_library.rs`) recognized `class|name|text|index|id` but **omitted
  `type`**, so `type:Shell` was mangled into `class="type:Shell"` and matched nothing — while
  `text:`/`name:` locators (the modal checkbox/button) worked. Fix: add `type` to the recognized
  prefixes. Verified live: reachability flipped from all-0 to Text=4, Button=4, Composite=41,
  Label=27, Tree=5. Evidence: `evidence/widget-experiment/`.
- **F10:** `Check Button` on the modal checkbox returns success but does **not** tick it (visual:
  `checkbox-not-ticked-F10.png`) — a false-success no-op on this control (while `Click Widget`
  on the Confirm button genuinely dismissed the modal).
- **Menu:** confirmed `Select Main Menu` is a **no-op stub** on real Eclipse (returns success, no
  UI change) — matches the static analysis; real menu/command actions should go through
  `Execute Command <id>`.
- **First-run modal chain:** DBeaver shows sequential first-run modals (Data share → "create
  sample database?"), which block `Execute Command …preferences` — reinforcing F8 (harness should
  pre-dismiss first-run modals for deterministic widget automation).
- **API fix applied:** added `Get Widget Property` to the RCP library (was SWT-only) so widget
  property read-back works when connected as an RCP app (same API-consistency family as F7).

## Impact

- **Tests:** new `tests/docker/rcp/` (Dockerfile + entrypoint) and `tests/robot/rcp/real_dbeaver/`; opt-in, self-skips without the harness. Evidence preserved under `openspec/changes/rcp-real-app-automation/evidence/`.
- **Agent (Java):** `SwtReflectionBridge` (Display lifecycle + readiness), `EclipseWorkbenchHelper` / `SwtReflectionRpcServer` (UI-thread wrapping + honest view/perspective resolution). Rebuild `agent/target/javagui-agent.jar`.
- **Rust:** `src/python/swing_library.rs` `capture_screenshot` stub → real RPC; add binding to `swt_library.rs`/`rcp_library.rs`.
- **Python:** expose `Capture Screenshot` + the RCP inspector keywords on `JavaGui.Rcp`; embed screenshots in the RF log.
- **CI:** optional dedicated opt-in job that builds the harness image and runs the DBeaver suite (network download of DBeaver + a full JDK in the image).
- **Docs:** how to run the harness; the "packaged-product JRE needs `java.instrument`" gotcha.
- **Non-goal:** no change to the default fast CI gate; the mock RCP app and the bespoke `com.testapp.rcp` real-Eclipse suite remain.
