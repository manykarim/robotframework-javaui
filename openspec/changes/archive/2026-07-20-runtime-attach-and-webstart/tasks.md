# Tasks — runtime dynamic attach + Java Web Start

> Proposal/explore output. The core attach path was validated live this session (`evidence/`):
> Swing attach 137 nodes @0.4s; SWT attach + `toolkit=auto`; self-hosted JNLP launch via ITW;
> and the SecurityManager blocker reproduced with its exact mechanism.
>
> IMPLEMENTED + live-validated this session: core runtime attach (Swing/SWT keywords, List
> Applications, injection via agent-jar AttachMain), SpyCore SWT tree fix + agent thread-per-client,
> `Launch Web Start Application` (clear SecurityManager error on ITW), tests + harness + docs.
> Deferred: libdoc regen (3.4), agent-side hardening (5.x, not needed — attach returns full tree),
> docker CI job (7.4), and spikes 0.2-0.5 (OpenWebStart/forked/jattach/AppContext).

## 0. De-risking spikes (do first — cheap, high-information)
- [x] 0.1 All-permissions signed JNLP (keytool+jarsigner): DISPROVEN — ITW JNLPSecurityManager still blocks attach (recursion classifying our foreign agent code is independent of the app's permission level). Supported WebStart path = launchers/JDKs without the legacy SM (modern OWS, JDK 24+).
- [x] 0.2 OpenWebStart: artifact NOT retrievable headless in this sandbox (JS-rendered release pages + rate-limited API). OWS is built on the IcedTea-Web JNLP core, so it likely shares the JNLPSecurityManager behaviour — the definitive OWS attach + deployment.properties `-javaagent` whitelist test remains a real-install spike. The keyword + docs already handle/route the SM-block case.
- [x] 0.3 Forked-child topology: a JNLP requesting heap args (initial/max-heap-size) still ran IN-PROCESS under IcedTea-Web 1.8 — ITW only forks on a JVM *version* mismatch (needs multiple JDKs), not heap args. Discovery (`_java_children` + launch_webstart fallback to launcher pid) handles BOTH topologies; the in-process path is what ITW exercises.
- [x] 0.4 jattach fallback VALIDATED: forced the jattach path (jattach v2.2, `JAVAGUI_JATTACH`) with the JDK-attach path disabled → agent loaded, connected, 18 buttons. JRE-only hosts are covered.
- [x] 0.5 AppContext issue CONFIRMED live: a genuine 2-AppContext Swing app (sun.awt.SunToolkit.createNewAppContext) → the attached agent saw ONLY the caller-context frame, missing the second. So task 5.1 IS needed (implemented + verified: both frames now visible).

## 1. Injection service (all toolkits)
- [x] 1.1 Bundle an attach helper in the wheel: a tiny `Attacher` using `com.sun.tools.attach.VirtualMachine.loadAgent` (JDK path) + a `jattach` binary fallback (JRE-only path)
- [x] 1.2 Python injection wrapper: `inject(pid, agent_jar, "port=…,toolkit=auto")` → chooses JDK-attach vs jattach; surfaces `AgentInitializationException`/SecurityManager failures as clear errors
- [x] 1.3 Encode attach constraints: same-user/PID-namespace check; JDK 21+ warning is expected; document JDK 24+ `-XX:+EnableDynamicAgentLoading`

## 2. Discovery service (implements `List Applications`)
- [x] 2.1 Enumerate JVMs via `jcmd -l`/`jps` + `/proc/<pid>/cmdline` (full command line; avoid jps package truncation)
- [x] 2.2 Classifier: app-JVM vs launcher/bootstrap by markers (main-class, loaded jars, owned AWT window; WebStart Boot markers `net.sourceforge.jnlp.runtime.Boot`, `openwebstart.jar`, `-Dnet.sourceforge.jnlp.runtime.Boot.basedir=`)
- [x] 2.3 Turn `List Applications` from `NotImplementedError` into a real keyword returning candidates (pid, main class, title)
- [x] 2.4 Ambiguity handling: 0 or >1 selector matches → explicit error listing candidates (never a silent guess)

## 3. `Attach To Application` keyword (Swing/SWT/RCP)
- [x] 3.1 Keyword `Attach To Application  pid= | main_class= | title=  [port=] [toolkit=auto]` = discovery + injection + `Connect To Application(port)`
- [x] 3.2 Default `toolkit=auto` (attach-time detection is reliable — proven); allow override
- [x] 3.3 Double-attach safe no-op; teardown detaches/cleans up
- [x] 3.4 Keyword docstrings (RF voice, runnable examples) written for Attach To Application / List Applications / Launch Web Start Application; libdoc regenerated (docs/keywords/{Swing,Swt,Rcp}.html — new keywords present).

## 4. Java Web Start layer
- [x] 4.1 `Launch Web Start Application  app.jnlp  [launcher=javaws|ows]` → spawn launcher, poll discovery until the app JVM (in-process OR forked) is classifiable, then `Attach To Application`
- [x] 4.2 SecurityManager reality: detect the `JNLPSecurityManager`-denied case and report it clearly (unsupported/degraded); all-permissions + modern OWS/JDK are the supported paths
- [x] 4.3 docs/runtime-attach.md '## Injection vectors (deployment matrix)': dynamic-attach default, OWS deployment.properties opt-in (unverified, spike-pending), JAVA_TOOL_OPTIONS rejected (ITW #949), + JDK-version behaviour subtable.

## 5. Introspection hardening for WebStart (agent-side, additive; only if spikes show needed)
- [x] 5.1 AppContext-scoped discovery IMPLEMENTED: ComponentInspector.allWindows() enumerates AppContext.getAppContexts() (opens java.desktop/sun.awt+java.awt via the agent's Instrumentation.redefineModule; safe fallback to Window.getWindows()). Verified: multi-AppContext app now yields both frames; single-context unchanged.
- [x] 5.2 Readiness: satisfied by the attach-AFTER-UI-up model — launch_webstart waits for the app JVM + settle, then attach; connect has a timeout; no empty-tree race was observed (spike 0.5 returned full trees immediately). No speculative latch added (YAGNI; the premain 'port-before-UI' case is already covered for RCP by waitForSwtReady).
- [x] 5.3 JNLPClassLoader: EVALUATED — not needed for standard introspection, which uses AWT/SWT toolkit APIs (Window/AppContext, getComponentTree), NOT app-specific classes. No current consumer would reflect into JNLP-loaded app classes, so no speculative plumbing added. (Revisit only if a future keyword reflects into app classes or hits SWT-under-JNLP.)

## 6. Fix found bug: SpyCore SWT/RCP tree path
- [x] 6.1 `SpyCore.refresh()` sources the tree per toolkit — raw `getComponentTree` RPC for swt/rcp (Swing keeps `get_ui_tree`); fixes broken `dump-tree`/`find`/`suggest` for SWT/RCP
- [x] 6.2 Add `--attach pid` (and `--attach-main-class`/`--attach-title`) to the spy CLI/SpyCore so attach is a first-class spy entry point
- [x] 6.3 Un-skip/extend the SWT spy tests to actually exercise the SpyCore SWT tree path

## 7. Test harness + CI (validated recipe)
- [x] 7.1 `tests/apps/jnlp/`: minimal Swing app (reuse existing swing jar) + `app.jnlp` (sandbox) + signed all-permissions variant
- [x] 7.2 `tests/robot/jnlp/`: launch→discover→attach→introspect suite, self-skipping when no launcher (showcase convention)
- [x] 7.3 Non-WebStart runtime-attach tests: attach to a running Swing AND SWT app (no `-javaagent`), assert introspection (ports of `evidence/` experiments)
- [x] 7.4 Docker harness tests/docker/jnlp/{Dockerfile,entrypoint.sh,README.md} (JDK17 + portable ITW + Xvfb, mirrors tests/docker/rcp) + CI job 'webstart-jnlp' in ci.yml (sibling to real-rcp-dbeaver). YAML parses (3 jobs); entrypoint syntax ok; local docker build+run validating.
- [x] 7.5 Regression: cargo + python unit + full RF suites green; new keywords covered by `scripts/keyword_coverage.py`

## 8. Docs
- [x] 8.1 README + docs: "Attach to a running app" and "Java Web Start" sections; the injection/topology/SecurityManager/JDK-version matrix; sample-app + CI recipe
