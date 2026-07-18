# Design — Automating real packaged Eclipse RCP apps

## Context

Experiment: DBeaver CE 26.1.2 (Eclipse 4.x/e4 RCP, SWT 3.134) driven headless in Docker
(`ubuntu:24.04` + Xvfb + GTK3 + ImageMagick), agent attached via `dbeaver.ini`
`-vmargs -javaagent:javagui-agent.jar=port=5682,toolkit=swt`, Robot driver co-located,
framebuffer screenshots via `import -window root`. Everything below is grounded in that run;
raw evidence is under `evidence/` and `results/dbeaver/` (gitignored).

## The startup timeline (why order matters)

```
t=0    JVM premain → UnifiedAgent starts → SWT RPC server LISTENS on :5682   ← port "up" here
       SwtReflectionBridge.initialize(): getAllLoadedClasses() → "Display class not loaded yet"
       (SWT bundle not resolved yet) → falls back to ContextFinder scan → FAILS
t≈4s   driver can TCP-connect, but ping → "SWT not initialized. Display not found."
t≈10s  Equinox resolves org.eclipse.swt; DBeaver UI thread creates Display; workbench renders
t>10s  getAllLoadedClasses() now contains Display (EquinoxClassLoader[org.eclipse.swt]);
       Display.getDefault()/findDisplay() resolves the instance → bridge usable
```

The agent's TCP port opening is **not** a readiness signal — it opens ~6 s before the Display
exists. Two consequences: the harness must wait for the workbench to render before driving
(F2 workaround), and the agent should offer a real readiness handshake (F2 fix).

## Decisions

### D1 — Harness attaches a FULL JDK, not the product's bundled JRE (F1)
Packaged Eclipse products (`jlink`-trimmed runtimes) commonly omit `java.instrument`, so
`-javaagent` cannot load. **Decision:** the harness installs a full JDK matching the product's
`-Dosgi.requiredJavaVersion` and inserts `-vm <jdk>/bin/java` before `-vmargs` in the product
`.ini` (single `-vm`, before `-vmargs`). This is a harness/deployment concern, not a library
change, but it MUST be documented as the standard way to automate a shipped RCP product.
Alternative rejected: adding `java.instrument` to the trimmed runtime (not portable; we don't
own the product's JRE).

### D2 — Lifecycle-aware Display discovery + readiness handshake (F2)
`SwtReflectionBridge.initialize()` already re-runs `findDisplayViaInstrumentation()` while
`displayInstance == null`, and the instrumentation path is correct once SWT is loaded. The gap
is (a) no explicit "not ready yet, retry" contract, and (b) the thread-classloader fallback is
noise that can mask the real state. **Decision:**
- On each RPC that needs the Display, attempt `getAllLoadedClasses()` resolution; if the SWT
  Display class is absent OR `Display.getDefault()` returns null, return a **distinct, typed
  "SWT_NOT_READY"** status (not a generic error).
- Add `waitForSwtReady(timeoutMs)` on the agent and a `Wait Until SWT Ready` keyword that polls
  it, so a driver connecting at premain blocks correctly instead of failing the first ping.
- `Connect To Swt Application` SHOULD internally retry ping while the status is `SWT_NOT_READY`
  up to the connect timeout, so callers don't hand-roll the wait.

### D3 — Run ALL state-changing workbench ops on the SWT UI thread (F3) — the core fix
Proven root cause (`evidence/invalid-thread-access.stack.txt`):
```
executeCommand (RPC thread)
  → IHandlerService.executeCommand → WorkbenchSourceProvider.getActiveWindow
  → Display.getActiveShell → Display.checkDevice → SWTException: Invalid thread access
```
In `EclipseWorkbenchHelper`, only `getActiveWindow`/`getActivePage` are inside `syncExec`
(:158,:174); the mutations (`showPerspective.invoke` :280, `showView.invoke` :332/:335,
`hideView.invoke` :365, `executeCommand.invoke` :570) run on the calling RPC thread.
**Decision:** wrap each state-changing operation's **entire** reflection sequence (lookup +
invoke + read-back) in a single `SwtReflectionBridge.syncExec(...)` so it executes atomically on
the UI thread, and propagate any exception thrown inside as the RPC error. This is the umbrella
fix for F3 and is a prerequisite for F4/F5 being meaningful.

### D4 — Honest view/perspective resolution (F4/F5)
`SwtReflectionRpcServer` returns `"View not found"` / `"Perspective not found"`
(:1289/:1302/:1171/:1183) for ids that `Get Open View Ids` / `Get Available Perspectives`
returned. Likely causes to verify under the fix: the dual-mode path tries `MockRcpApplication`
first and the real-Eclipse resolution either isn't reached or throws off-thread (D3) and is
mis-reported as "not found". **Decision:**
- Resolve view ids via `IViewRegistry` / open `IViewReference`s and perspective ids via
  `IPerspectiveRegistry` on the UI thread; only report "not found" when genuinely absent from the
  registry.
- `Close View` (`hideView`) MUST verify the view is gone via read-back and return failure if it
  is not — never a false-success no-op. Contract: an action keyword's success means the workbench
  state changed, confirmed by read-back.

### D5 — Close the visual-confirmation pipe (F6)
The Java agent already renders widgets/full-display to base64 PNG (Swing + SWT, both wired into
their RPC servers). The break is the Rust `capture_screenshot` stub (`swing_library.rs:1759`,
returns a fake path, never calls the agent) and the missing SWT/RCP bindings, plus no exposure
on `JavaGui.Rcp` and no RF-log embedding. **Decision:** Rust sends `"captureScreenshot"`, decodes
`data:image/png;base64,…`, writes the file; add the binding to `swt_library.rs`/`rcp_library.rs`;
expose `Capture Screenshot` on the RCP/SWT libraries; embed `<img>` in the RF log. Keep the
X-framebuffer grab as a toolkit-agnostic fallback the harness uses regardless.

### D6 — Deterministic first-run (F8)
Pre-seed the DBeaver workspace / preferences (or check "Do not share data" then Confirm) so the
modal "Data share" dialog does not sit over the workbench. Non-blocking for introspection but it
can steal focus / block modal-sensitive actions and makes screenshots noisy.

## Fix priority

```
P0  D3 UI-thread execution      ← without it every "real" action is unreliable (Invalid thread access)
P0  D2 Display readiness         ← without it the agent is unusable on any packaged product
P1  D4 honest view/persp resolution + no false-success no-op
P1  D5 screenshot pipe end-to-end + RF-log embedding
P2  D1 harness JDK/-vm doc+script (already implemented in this change's harness)
P2  F7 RCP inspector-keyword API consistency
P2  D6 first-run determinism
```

## Risks / open questions
- e4 vs 3.x compat: DBeaver's `org.eclipse.ui` comes via the compatibility layer; the 3.x
  `IWorkbench` reflection works for introspection (proven), but some e4-native model operations
  may need the `EModelService`/`EPartService` path — to confirm while fixing D4.
- `Display.getDefault()` from a non-UI thread can *create* a display; prefer `findDisplay(uiThread)`
  / `getDisplays()` for discovery to avoid a rogue display.
- Running the product as root in-container is fine for DBeaver; other products may need a non-root
  user + writable `HOME`/`XDG_RUNTIME_DIR`.
