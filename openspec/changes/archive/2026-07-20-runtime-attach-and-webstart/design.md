# Design — runtime dynamic attach + Java Web Start

Explore-mode output: architecture + decisions grounded in this session's live experiments
(`evidence/`) and Fable research (`evidence/jnlp-research-brief.md`). Not an implementation.

## 1. The crux: injection + discovery, not introspection

Once the agent is inside the target JVM, introspection is unchanged — the Swing/SWT engines read
the AWT tree via `Window.getWindows()`/`Frame.getFrames()` (boot-loader APIs a custom classloader
cannot hide). Proven live: attach to a plain Swing JVM → 137 nodes; attach to a plain SWT JVM →
`getComponentTree` 16.7 KB. So ~80% of the novelty and risk is in two services:

```
  ┌─────────────────────────────────────────────────────────────────────────┐
  │  A. INJECTION            B. DISCOVERY            C. ATTACH KEYWORD        │
  │  load agent into pid     find the app JVM        A + B + connect(port)    │
  │  (JDK Attach / jattach)  (jps/jcmd + cmdline)    Swing/SWT/RCP, auto tk   │
  └─────────────────────────────────────────────────────────────────────────┘
                                     ▲
                        D. WEBSTART LAYER: launch javaws → B (topology-aware) → C
```

## 2. Injection — how the agent gets into a live PID

`agentmain` == `premain` → same `initialize()` (verified in `UnifiedAgent.java`). No agent change
needed to *be* attachable. The open question is how the *library* triggers the attach portably:

| Option | Needs | Verdict |
|---|---|---|
| `com.sun.tools.attach.VirtualMachine.loadAgent` from a **JDK** | a JDK on the automation host | **Default.** Proven this session (0.4 s). Ship a tiny `Attacher` run via the JDK. |
| **`jattach`** (standalone C tool) | a small native binary, no JDK | **Fallback** for JRE-only hosts (e.g. only the OWS-bundled JRE present). |
| `tools.jar` reflection (pre-9) | legacy | Rejected — dead on JDK 9+. |
| `JAVA_TOOL_OPTIONS=-javaagent:…` | env control | **Rejected** except last-resort: double-loads the *launcher* JVM (ITW #949 crash) and leaks to every JVM. |

Decision: **JDK Attach API by default, `jattach` fallback, both shipped in the wheel.** The
attacher takes `(pid, agentJar, "port=…,toolkit=auto")`.

### Attach constraints to encode
- Same user + same PID namespace (containers: automation and app must share the namespace).
- JDK 21+ prints "a Java agent has been loaded dynamically" (works, warning only).
- JDK 24+ (future) may require `-XX:+EnableDynamicAgentLoading` **at target launch** — which we
  cannot set for a found-in-the-wild app. Document as the forward-compat opt-in; track JEP 451.

## 3. Discovery — find the *right* JVM (implements `List Applications`)

`jps` truncates the package on the short main-class form → prefer `jcmd -l` / `/proc/<pid>/cmdline`
for the full command line. Classify candidates by markers:

```
  plain app JVM       : main-class / app jar on classpath / owns a visible AWT window
  WebStart launcher   : net.sourceforge.jnlp.runtime.Boot ; openwebstart.jar on bootclasspath ;
                        -Dnet.sourceforge.jnlp.runtime.Boot.basedir= ; ICEDTEA_WEB_SPLASH
  WebStart app JVM    : forked child with -Xnofork / the requested JVM args ; OR (see §5) the
                        SAME pid as the launcher when the app runs in-process
```

`Attach To Application` selectors: `pid=` (explicit), `main_class=` (regex over cmdline),
`title=` (window title → owning JVM). Ambiguity (0 or >1 match) is an explicit error listing
candidates, never a silent guess.

## 4. The attach keyword + toolkit detection

`Attach To Application  pid=1234` → inject `toolkit=auto` → agent's `detectToolkit()` reads
`instrumentation.getAllLoadedClasses()` → correct toolkit (SWT proven auto-detected at attach).
This *removes* the premain "must force toolkit=swt" footgun for the attach path. Then the existing
`Connect To Application(host, port)` runs unchanged.

## 5. WebStart layer — topology is conditional (live finding)

The desk research said "javaws always forks a child JVM." **Experiment contradicts it:** ITW 1.8
ran a no-special-args JNLP **in-process** (one PID; log `Starting application [...]`). It forks a
child **only when the JNLP requests JVM args the launcher lacks** (heap, `--add-opens`, etc.).

```
  JNLP with no special <j2se> vm-args   →  app runs IN-PROCESS  (app pid == launcher pid)
  JNLP requesting heap / vm-args        →  launcher FORKS a child app JVM (distinct pid)
```

→ Discovery must handle **both**. In-process: the launcher pid *is* the app pid (attach there,
but see §6). Forked: pick the child by the requested-args / `-Xnofork` marker.

WebStart flow: `Launch Web Start Application  app.jnlp` → spawn `javaws`/OWS → poll discovery
until an app JVM (in-process or child) is classifiable → `Attach To Application` → connect.

## 6. The SecurityManager blocker (live-confirmed, and receding)

Attaching to the **sandboxed ITW JNLP JVM failed**: `JNLPSecurityManager.checkPermission →
getApplication → JNLPPolicy.getPermissions → …` recurses on the attach-loaded agent's code
(which belongs to no JNLP "application") → `AgentInitializationException`. So `doPrivileged`
wrapping *by us* does not help — it is ITW classifying **our** code that blows up.

Corrected premise (from research, matched by the trace): an attach-loaded agent is **not**
auto-granted `AllPermission` under an active SM. Where the SM applies:

| Case | Attach works? | Note |
|---|---|---|
| All-permissions (signed) JNLP under **IcedTea-Web** | **No** (spike 0.1, live) | the recursion is in ITW classifying OUR agent's code source — independent of the *app's* permission level; all-permissions does NOT help |
| Sandboxed JNLP under **IcedTea-Web** | **No** (live) | same structural cause |
| Modern OpenWebStart on JDK 17+ | Likely yes | trends toward no legacy SM — **verify (spike 0.2)** |
| **JDK 24+ (JEP 486)** | Yes | SM cannot be enabled — blocker gone |

Decision (corrected by spike 0.1): **the blocker is any active IcedTea-Web `JNLPSecurityManager`,
NOT the app's permission level.** All-permissions does *not* unlock attach on ITW. Supported
WebStart paths are therefore **launchers/JDKs without the legacy SM** — modern OpenWebStart and
JDK 24+. The `Launch Web Start Application` keyword detects the denied case and raises a clear,
actionable error rather than hanging.

## 7. Introspection hardening for WebStart (agent-side, additive)

Ports of fixes this project already shipped for RCP:
- **AppContext scoping** — `getWindows()` from the agent's socket thread may see only its own
  AppContext (empty tree despite visible frames). Run root discovery on the app EDT via
  `invokeAndWait`, or enumerate `AppContext.getAppContexts()`. *Not reproduced this session on the
  directly-attached Swing case (137 nodes came back) — verify specifically for multi-AppContext
  WebStart/applet apps before building on it.*
- **Readiness gating** — WebStart startup is slow; gate `getUiTree` on a real "first app window"
  signal (WINDOW_OPENED latch). Largely self-mitigated by attaching *after* the UI is up.
- **JNLPClassLoader** — only if reflecting into app classes or SWT-under-JNLP; obtain the
  JNLPClassLoader by reflection (same shape as the Eclipse OSGi bundle-classloader fix).

## 8. Found bug — SpyCore SWT tree path

`SpyCore.refresh()` (`python/JavaGui/spy/core.py:121`) calls `self.lib.get_ui_tree(format="json")`,
but the Rust `SwtLibrary` has **no** `get_ui_tree` (only JTree/Tree *content* getters) →
`AttributeError`. The spy's `dump-tree`/`find`/`suggest` are broken for SWT/RCP through SpyCore
(masked: SWT spy tests self-skip; the DBeaver probe used raw `getComponentTree`). Fix: source the
tree per toolkit — raw `getComponentTree` RPC for swt/rcp — and add `--attach pid` to the spy CLI
so attach becomes a first-class spy entry point.

## 9. Test + sample-app strategy (validated)

- **Primary CI:** self-hosted minimal Swing JNLP over `http.server` + `javaws` under xvfb —
  *proven this session with IcedTea-Web 1.8.8 on JDK 17*. Reuse the existing swing test jar.
  `tests/apps/jnlp/` + `tests/robot/jnlp/`, self-skipping when no launcher (showcase convention).
- **Signed/all-permissions variant** (keytool + jarsigner) to exercise the SM-present-but-granted
  path and confirm §6's "expected yes".
- **Opportunistic real-world smoke (never a gate):** JClic `reportServer.jnlp` (research-verified
  HTTP 200), self-skipping.
- **Docker:** JDK + portable ITW/OWS + Xvfb (portable ITW zip needs no root — used this session).

## 10. Priority spikes before/while implementing

1. All-permissions-signed JNLP → confirm attach succeeds under SM-granted (§6).
2. OpenWebStart (vs ITW) topology + `deployment.properties` `-javaagent` whitelist form (colon-arg).
3. Forked-child topology: a JNLP requesting heap args → confirm distinct child pid + marker.
4. `jattach` against a JRE-only host (no JDK on PATH).
5. AppContext empty-tree repro on a genuine multi-AppContext app (§7).
