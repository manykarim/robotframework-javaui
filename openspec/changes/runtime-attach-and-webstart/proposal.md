## Why

Today the only way to automate a Java app with this library is to control its launch and add
`-javaagent:javagui-agent.jar` to the command line. That excludes two important cases:

1. **Found-in-the-wild apps** — a Swing/SWT/RCP app that is *already running* (started by a user,
   a shortcut, a service) that you cannot relaunch with an agent flag.
2. **Java Web Start (JNLP) apps** — launched by `javaws`/OpenWebStart/IcedTea-Web, where you do
   **not** own the command line and, decisively, `-javaagent` is **not** on the JSR-56 secure
   `vm-args` whitelist (signing does not unlock it). The library's normal front door is closed.

The capability that unlocks both is the same one: **attach the agent to an already-running JVM by
PID** (the JDK Attach API / `agentmain`), instead of loading it at launch (`premain`). The agent
jar already ships `agentmain` + `Agent-Class` in its manifest — the lever exists but nothing on
the library side pulls it, and the `List Applications` keyword that would discover target JVMs is
a `NotImplementedError` stub.

This change adds a **runtime dynamic-attach capability for all toolkits**, exposes it as first-class
keywords (`Attach To Application`, a real `List Applications`), and builds **Java Web Start support
as a thin layer on top** (launch `javaws` → discover the app JVM → attach → connect).

### Validated crux — live experiments (evidence/)

Run this session against real JVMs (see `evidence/`):

| Experiment | Setup | Result |
|---|---|---|
| Swing runtime attach | `java -jar app.jar` (**no** `-javaagent`) → `VirtualMachine.attach(pid).loadAgent(...)` | Agent booted in **0.4 s**; connected; `get_ui_tree` → **137 nodes**, `find_elements` → 18 buttons |
| SWT runtime attach, `toolkit=auto` | plain SWT app, attach with `toolkit=auto` | Agent **auto-detected SWT** (`Detected SWT via loaded class …gtk3.GdkEventMotion`); `getComponentTree` 16.7 KB |
| **JNLP end-to-end** | self-hosted minimal Swing JNLP via IcedTea-Web `javaws` under xvfb | Launched + rendered the app; sample-app recipe validated |
| JNLP attach under sandbox | attach agent to the ITW JNLP JVM | **Blocked**: ITW `JNLPSecurityManager` recurses on the attach-loaded agent's code → `AgentInitializationException` |

Two facts these experiments settle:

- **Runtime attach is not just viable — it is *better* than launch-time for toolkit detection.**
  `premain` runs before Eclipse/SWT loads, so it must be forced `toolkit=swt`; at attach-time the
  toolkit classes are already loaded and `toolkit=auto` just works. `agentmain` and `premain`
  funnel into the identical `initialize()`, so **zero agent changes** are required for the core.
- **The SecurityManager is the one real blocker for *sandboxed legacy* JNLP** — and it is a
  receding, legacy problem: `SecurityManager` is deprecated (JDK 18) and cannot be enabled at all
  from JDK 24 (JEP 486). It does not apply to all-permissions enterprise JNLPs, nor to modern
  OpenWebStart-on-modern-JDK.

## What Changes

- **Add a Discovery service** — enumerate candidate JVMs (`jps`/`jcmd` + `/proc/<pid>/cmdline`)
  and classify the app JVM by markers (main-class, loaded jars, window, WebStart Boot markers).
  This turns `List Applications` from a stub into a real keyword.
- **Add an Injection service** — load the agent into a live PID without `tools.jar`, using the
  JDK Attach API (bundled tiny attacher) with a `jattach` fallback for JRE-only environments.
- **Add `Attach To Application`** keyword — `pid=… | main_class=… | title=…` → discovery + attach
  + existing `Connect To Application(port)`. Works for Swing/SWT/RCP; `toolkit=auto` by default.
- **Add Java Web Start (JNLP) support** — a documented flow that launches a `.jnlp` with
  `javaws`/OpenWebStart, discovers the (in-process **or** forked) app JVM, attaches, and connects;
  plus a self-hosted minimal-Swing-JNLP test harness (`tests/apps/jnlp/`, `tests/robot/jnlp/`,
  self-skipping when no launcher is present).
- **Fix a found bug** — `SpyCore("swt").refresh()` calls a nonexistent `get_ui_tree` on the Rust
  `SwtLibrary`; the spy tool's SWT/RCP tree path is broken. Route it through the raw
  `getComponentTree` RPC (confirmed working this session), and make the spy `--attach pid` capable.
- **Document the deployment matrix** — dynamic attach as the default; JDK-21+ dynamic-agent
  warning + JDK-24 `-XX:+EnableDynamicAgentLoading`; OpenWebStart `deployment.properties` as the
  opt-in CI vector; `JAVA_TOOL_OPTIONS` explicitly rejected (double-loads the launcher JVM).

## Impact

- **Non-breaking, additive.** Existing `-javaagent` launch + `Connect To Application(port)` are
  unchanged; runtime attach is a new, parallel path.
- New Python surface (discovery + injection + keyword), a bundled attach helper in the wheel, and
  a new opt-in test suite. No agent-jar changes required for the core attach; WebStart hardening
  (AppContext-scoped discovery, `doPrivileged` wrapping) is agent-side but additive.
- Scope explicitly excludes making *sandboxed legacy IcedTea-Web* apps work under an active
  `SecurityManager` — documented as degraded/unsupported, with all-permissions and modern
  OWS/JDK as the supported WebStart paths.
