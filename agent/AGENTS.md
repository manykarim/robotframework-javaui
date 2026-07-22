# agent/AGENTS.md — the Java agent

Nested guide for the in-JVM introspection agent. Read the root **[/AGENTS.md](../AGENTS.md)** first —
it is canonical for build/test/style/PR. This file covers only agent-local specifics; do not restate
the root.

## What this is
A Java agent (`com.robotframework.*`, Maven, Java 17) that runs *inside* the target JVM and speaks
JSON-RPC over TCP to the Rust core. Shipped in the wheel at `python/JavaGui/jars/javagui-agent.jar`.
- `UnifiedAgent` — the entry point (`premain`/`agentmain`); parses `port=,host=,toolkit=` and starts a server.
- `swing/ComponentInspector.java` — Swing/AWT introspection.
- `swt/SwtReflectionBridge.java` + `swt/SwtReflectionRpcServer.java` — SWT/RCP over pure reflection.
- `attach/AttachMain.java` — standalone runtime-attach launcher (JDK Attach API).

## Build — and the MANDATORY sync step
```bash
mvn -f agent/pom.xml package -DskipTests           # rebuild ONLY the agent jar
cp agent/target/javagui-agent.jar python/JavaGui/jars/    # <-- REQUIRED after every agent change
```
A rebuilt jar that is not copied into `python/JavaGui/jars/` is **not used** — the library loads the
bundled one. `uv run invoke build-dev` does the compile + copy for you; prefer it. `mvn ... -DskipTests`
skips the JUnit suite. The pom's manifest declares `Premain-Class`/`Agent-Class` = `UnifiedAgent` and
`Can-Redefine-Classes`/`Can-Retransform-Classes` = true (needed by the module-open trick below).

## premain vs agentmain (`UnifiedAgent`)
- **`premain`** — launch time: `java -javaagent:javagui-agent.jar=port=NNNN,toolkit=swt Main`. The
  socket opens before the GUI toolkit loads; SWT clients that connect early are covered by
  `SwtReflectionBridge.waitForSwtReady`.
- **`agentmain`** — runtime attach into an already-running JVM (via `AttachMain`/`jattach`).
- Idempotent: `initialize()` runs once (guarded by `initialized`); a second load is a no-op.

## CRITICAL: force `toolkit=swt` at premain for SWT/RCP
`toolkit=auto` (`detectToolkit()`) inspects loaded classes/threads for `org.eclipse.swt.*`. At
**premain** the agent runs before SWT loads, so auto mis-detects and starts the *Swing* server — the
wrong server, permanently. **Always pass `toolkit=swt` explicitly when launching an SWT or Eclipse RCP
app.** Auto-detection is only reliable at runtime **attach** (`agentmain`), where SWT is already loaded.
`swt` and `rcp` both route to `SwtReflectionRpcServer`.

## SWT server is thread-per-client, ops serialized on the UI thread
`SwtReflectionRpcServer` spawns a daemon `SwtRpc-client` thread per accepted socket (concurrent
connections — a spy can hold one while another drives the app). All actual widget access funnels
through `SwtReflectionBridge.syncExec` onto the SWT UI thread, so concurrency only overlaps request
framing, never widget state. Any new SWT bridge method that touches a widget **must** run inside
`syncExec` — off-thread SWT access throws `SWTException: Invalid thread access`.

## Classloader lessons (why everything is reflection)
- **OSGi bundle isolation:** a bare `Class.forName("org.eclipse.ui.PlatformUI")` / `"…swt.widgets.Display"`
  uses the *agent's* classloader, which is blind to OSGi bundles — it fails inside Eclipse/RCP. Instead
  discover the owning bundle classloader by scanning `UnifiedAgent.getInstrumentation().getAllLoadedClasses()`
  for the class, then `getClassLoader()` (see `EclipseWorkbenchHelper` for `org.eclipse.ui`, and
  `SwtReflectionBridge.findDisplayViaInstrumentation` for the Display). Load all further SWT classes
  from that same classloader. The SWT bridge holds **no** static SWT imports for exactly this reason.
- **AppContext + module opens (Swing):** `Window.getWindows()` is AppContext-scoped, so the agent misses
  windows from other AppContexts (Java Web Start / applets). `ComponentInspector.allWindows()` opens
  `sun.awt` + `java.awt` internals of the `java.desktop` module via `Instrumentation.redefineModule`
  (requires the `Can-Redefine`/`Can-Retransform` manifest flags), then enumerates every AppContext.
  Falls back to plain `getWindows()` when the opens are denied — never regresses.

## AttachMain (runtime attach)
`java --add-modules jdk.attach -cp javagui-agent.jar com.robotframework.attach.AttachMain <pid> <agent-jar> "port=NNNN,toolkit=auto"`.
Uses `com.sun.tools.attach.VirtualMachine` — needs a **JDK** (`jdk.attach` module) on the *attaching*
host (the pom compiles it with `--add-modules jdk.attach`); JRE-only hosts fall back to the `jattach`
binary. Exit codes: 0 loaded · 2 usage · 3 attach/target error · 4 agent loaded but `agentmain` failed
(usually a restrictive SecurityManager in a sandboxed JNLP target). Python side: `python/JavaGui/_attach.py`.

## Verify after an agent change
Rebuild + copy (above), then run the root pipeline. Live SWT/RCP suites need a display and self-skip
without one: `xvfb-run -a uv run robot -d results tests/robot/swt`. Also remember: launched apps do not
persist across tool calls — launch and drive in one command (see root Gotchas).
