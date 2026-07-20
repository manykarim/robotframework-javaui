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
- [ ] 0.2 OpenWebStart (vs IcedTea-Web): process topology + whether `deployment.properties` JVM-args accept the `-javaagent:jar=args` colon form (CI vector)
- [ ] 0.3 Forked-child topology: a JNLP requesting heap args → confirm distinct child PID + a reliable discriminating marker
- [ ] 0.4 `jattach` against a JRE-only host (no JDK on PATH) → confirm the fallback loads the agent
- [ ] 0.5 AppContext empty-tree repro on a genuine multi-AppContext app (design §7) before building readiness/EDT-scoping

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
- [ ] 3.4 Keyword docs with runnable examples (RF voice); libdoc regenerated

## 4. Java Web Start layer
- [x] 4.1 `Launch Web Start Application  app.jnlp  [launcher=javaws|ows]` → spawn launcher, poll discovery until the app JVM (in-process OR forked) is classifiable, then `Attach To Application`
- [x] 4.2 SecurityManager reality: detect the `JNLPSecurityManager`-denied case and report it clearly (unsupported/degraded); all-permissions + modern OWS/JDK are the supported paths
- [ ] 4.3 Deployment matrix doc: dynamic-attach default; OWS `deployment.properties` opt-in CI vector; `JAVA_TOOL_OPTIONS` rejected (launcher double-load, ITW #949)

## 5. Introspection hardening for WebStart (agent-side, additive; only if spikes show needed)
- [ ] 5.1 AppContext-scoped root discovery (run on app EDT / enumerate `AppContext.getAppContexts()`) — gated on spike 0.5
- [ ] 5.2 Readiness gate: `getUiTree` waits for a real "first app window" signal (WINDOW_OPENED latch)
- [ ] 5.3 JNLPClassLoader resolution by reflection (only if reflecting into app classes / SWT-under-JNLP) — mirrors the RCP OSGi bundle-classloader fix

## 6. Fix found bug: SpyCore SWT/RCP tree path
- [x] 6.1 `SpyCore.refresh()` sources the tree per toolkit — raw `getComponentTree` RPC for swt/rcp (Swing keeps `get_ui_tree`); fixes broken `dump-tree`/`find`/`suggest` for SWT/RCP
- [x] 6.2 Add `--attach pid` (and `--attach-main-class`/`--attach-title`) to the spy CLI/SpyCore so attach is a first-class spy entry point
- [x] 6.3 Un-skip/extend the SWT spy tests to actually exercise the SpyCore SWT tree path

## 7. Test harness + CI (validated recipe)
- [x] 7.1 `tests/apps/jnlp/`: minimal Swing app (reuse existing swing jar) + `app.jnlp` (sandbox) + signed all-permissions variant
- [x] 7.2 `tests/robot/jnlp/`: launch→discover→attach→introspect suite, self-skipping when no launcher (showcase convention)
- [x] 7.3 Non-WebStart runtime-attach tests: attach to a running Swing AND SWT app (no `-javaagent`), assert introspection (ports of `evidence/` experiments)
- [ ] 7.4 Docker harness: JDK + portable ITW/OWS + Xvfb; wire an opt-in CI job (mirror `real-rcp-dbeaver`)
- [x] 7.5 Regression: cargo + python unit + full RF suites green; new keywords covered by `scripts/keyword_coverage.py`

## 8. Docs
- [x] 8.1 README + docs: "Attach to a running app" and "Java Web Start" sections; the injection/topology/SecurityManager/JDK-version matrix; sample-app + CI recipe
