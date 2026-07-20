# Runtime Attach

Automate a Java GUI app that is **already running** — no `-javaagent` on its command line — by loading the agent into the live JVM through the JDK Attach API. This is the path for apps you cannot relaunch and for Java Web Start (JNLP) apps, whose launcher will not let you add JVM arguments.

Works for Swing, SWT, and Eclipse RCP.

---

## Why runtime attach

The normal flow (`Connect To Application`) needs the target started with the agent already loaded:

```bash
java -javaagent:javagui-agent.jar=port=5678 -jar your-app.jar
```

That is the lowest-overhead path — use it whenever you launch the app yourself. But sometimes you can't:

- The app is **already up** and relaunching it would lose state.
- **Someone else** (a CI harness, an installer, an IDE) launched it.
- It starts through **Java Web Start**, which builds the JVM command line itself and strips `-javaagent` — that flag is not on the JNLP secure vm-args whitelist.

In all three cases you attach the agent to the *running* JVM instead. From your test it is one keyword:

```robotframework
Attach To Application    main_class=testapp.SwingTestApp
```

---

## The model: discover → inject → connect

`Attach To Application` runs three steps.

1. **Discover.** Enumerate the JVMs owned by your user and match the one you named (`pid`, `main_class`, or `title`). If 0 or >1 match, it raises with the candidate list instead of guessing.
2. **Inject.** Pick a free RPC port, then load `javagui-agent.jar` into the target JVM via the JDK Attach API. The jar's `com.robotframework.attach.AttachMain` performs the load; on a JRE-only host a `jattach` binary is used instead. The agent starts its RPC server on the chosen port inside the target.
3. **Connect.** Once the port accepts connections, the library connects exactly as `Connect To Application` would. From here every keyword behaves identically.

`toolkit=auto` detects Swing vs SWT from the classes loaded in the target, so the agent hooks the right toolkit even when you don't specify it.

### Discovery

`List Applications` exposes step 1 on its own so you can inspect what is attachable:

```robotframework
${apps}=    List Applications
FOR    ${app}    IN    @{apps}
    Log    ${app}[pid] — ${app}[display_name] (${app}[main_class])
END
Attach To Application    pid=${apps}[0][pid]
```

Each entry is a dict:

| Field | Meaning |
|-------|---------|
| `pid` | Process ID of the JVM. |
| `main_class` | Main class or entry jar, parsed from the command line. |
| `command_line` | Full launch command line. |
| `display_name` | Human-friendly label. |
| `is_launcher` | `True` for `javaws`/IDE-bootstrap-style processes. |
| `markers` | Detected hints (toolkit, launcher type, …). |

Launcher processes are filtered out by default. Pass `include_launchers=True` to include them — useful when debugging a Web Start launch.

### Selecting the target

Pick exactly one process:

| Argument | Matches on | Notes |
|----------|-----------|-------|
| `pid` | Exact process ID | Most explicit; use it when you already know the PID. |
| `main_class` | Regex over main class / entry jar / command line | Convenient and stable across runs. |
| `title` | Window-title pattern (`*` wildcards) | Needs `wmctrl` on the test host. |

Ambiguity is an error, by design. If your `main_class` regex matches two JVMs, narrow it or fall back to `pid` (often from `List Applications`).

---

## Requirements & the version / launcher matrix

Runtime attach depends on the **test host's** JVM and, for Web Start, the launcher running the target.

### Test host

| Need | Detail |
|------|--------|
| **JDK with `jdk.attach`** | Attach uses the JDK Attach API. A JDK 17+ works out of the box. Set `JAVAGUI_JAVA` to point at a specific `java`. |
| **or `jattach`** | JRE-only host? Provide a `jattach` binary via `JAVAGUI_JATTACH` (or on `PATH`). The library falls back to it automatically. |
| **Same-user access** | The OS only permits attaching to JVMs owned by the same user. |
| **`wmctrl`** | Only if you select by `title=`. |

### Target JVM

| Target JDK / launcher | Attach behavior |
|-----------------------|-----------------|
| **JDK 8 – 20** | Attaches cleanly. |
| **JDK 21 – 23** | Attaches; prints a one-line *"a dynamic agent has been loaded"* warning on the target's console. Harmless. |
| **JDK 24+** | The target must have been launched with `-XX:+EnableDynamicAgentLoading`. That flag is set at the *target's* launch (not by the test); without it the JVM refuses dynamic agent loads. |
| **Plain `java -jar app.jar`** (any of the above) | ✅ Attaches — this is the common case. |
| **JNLP under modern OpenWebStart** | ✅ Attaches. |
| **JNLP under IcedTea-Web** (legacy `SecurityManager`) | ❌ Blocked — see below. |

---

## Injection vectors (deployment matrix)

There is more than one way to get `javagui-agent.jar` into a target JVM. The library uses exactly one by default — dynamic attach — and it is the right choice almost everywhere. The others exist only for constrained CI/launcher scenarios and carry real caveats. This matrix states, per vector, when the agent code actually lands in the **application** JVM (not the launcher), and what it costs.

| Vector | How the agent loads | Status | Use it when |
|--------|--------------------|--------|-------------|
| **Dynamic attach** (default) | `agentmain` via the JDK Attach API — injected into the *live* target JVM after it starts. | ✅ Supported, the default. | Always, unless a launcher structurally blocks it. Covers plain running apps and Web Start on a no-`SecurityManager` launcher. |
| **OpenWebStart JVM args** | `-javaagent:…` set as OWS per-JNLP / default JVM args in `deployment.properties`, so the flag rides along at the **app JVM's** launch. | ⚠️ Opt-in, **unverified** (spike pending). | CI only, when dynamic attach is unavailable and you control the OWS install. |
| **`JAVA_TOOL_OPTIONS` / `_JAVA_OPTIONS`** | Environment variable carrying `-javaagent:…`, read by *every* JVM the process tree spawns. | ❌ Rejected — last-resort scripted fallback only. | Effectively never; see the hard constraints below. |

### 1. Dynamic attach — the default vector

This is the discover → inject → connect flow described above: the target starts on its own, then the agent is loaded into the running JVM through `agentmain`. It needs no cooperation from the launcher and no environment plumbing, which is why it is the default for plain apps **and** for Web Start on launchers without the legacy `SecurityManager` (modern OpenWebStart, JDK 24+ `javaws`). Its only limits are the JDK-version and `SecurityManager` rules already tabled above.

### 2. OpenWebStart JVM args — opt-in CI vector

`-javaagent` is **not** on the JNLP secure vm-args whitelist, so you cannot put it in the `.jnlp` and you cannot pass it to `javaws` — the launcher strips it (see [Java Web Start](#java-web-start-jnlp) below). The **only** way to make `-javaagent` ride along to the *application* JVM is to set it as a default/per-JNLP JVM argument in OpenWebStart's own `deployment.properties`, which OWS applies when it builds the app JVM's command line. That gets the agent loaded at launch (via `premain`) rather than by attach — sidestepping any dynamic-agent-load restriction entirely.

This vector is **opt-in and currently unverified** against a specific OpenWebStart release — a validation spike is pending. Treat it as a documented possibility for a CI harness you fully control, not a proven path. It applies only to OpenWebStart; IcedTea-Web's `JNLPSecurityManager` block is orthogonal and not solved by this.

### 3. `JAVA_TOOL_OPTIONS` / `_JAVA_OPTIONS` — rejected

Setting `JAVA_TOOL_OPTIONS='-javaagent:javagui-agent.jar=…'` (or `_JAVA_OPTIONS`) looks like an easy universal switch. It is not, and the library does not use it. Two failures:

- **It double-loads into the wrong JVM.** The variable is read by the **launcher** JVM as well as the app JVM. With Web Start the launcher runs first — so the agent tries to load there before the application JVM even exists. IcedTea-Web bug [#949](https://github.com/AdoptOpenJDK/IcedTea-Web/issues/949) is exactly this: a Java 8 `javaws` launcher tried to load a Java 11 agent and died with `UnsupportedClassVersionError` before the app ever started.
- **It leaks into every Java process.** The variable is inherited by *every* JVM the shell and its children spawn, agent and all, for as long as it is exported. That is a broad, hard-to-scope side effect on any shared CI host.

If you truly have no other option, this is a **scripted last resort only**, and then the agent jar must be compiled to **Java 8 bytecode** and carry a **`premain` self-guard** that no-ops when it finds itself loaded into a launcher / wrong JVM. Do not reach for it before dynamic attach (vector 1) or the OWS JVM-args vector (vector 2).

### JDK-version behavior (applies to attach vectors)

The dynamic-attach vector's behavior tracks the **target** JDK version:

| Target JDK | Dynamic attach behavior |
|------------|-------------------------|
| **≤ 20** | Clean — no warning, no flag needed. |
| **21 – 23** | Attaches; prints a one-line *"a dynamic agent has been loaded"* warning on the target console. Harmless. |
| **24+** | Refuses dynamic agent loading **unless** the target was launched with `-XX:+EnableDynamicAgentLoading`. That flag must be set **at the target's launch** — which you cannot do for an app you found running in the wild. On such targets, use a launch-time vector (a `-javaagent` you control, or the OWS JVM-args vector). |

---

## Java Web Start (JNLP)

```robotframework
*** Settings ***
Library    JavaGui.Swing

*** Test Cases ***
Automate A Web Start App
    Launch Web Start Application    /path/to/app.jnlp    launcher=/opt/openwebstart
    Click    JButton[text='Start']
    [Teardown]    Disconnect
```

`Launch Web Start Application` starts the `.jnlp` with the launcher, waits `settle` seconds for the **application** JVM to appear (it may run in-process in the launcher or in a forked child), attaches the agent, and connects.

| Argument | Detail |
|----------|--------|
| `jnlp` | Path or URL to the `.jnlp`. |
| `launcher` | A `javaws` binary or an IcedTea-Web / OpenWebStart image directory. Default: `JAVAGUI_JAVAWS` env, else `javaws` on `PATH`. |
| `host` / `port` | Agent RPC interface / port (a free port is chosen when omitted). |
| `toolkit` | Toolkit hint; default `auto`. |
| `settle` | Seconds to wait for the app JVM before attaching. Default `8`. |
| `timeout` | Overall launch + attach + connect timeout. Default `60`. |

### Why `-javaagent` cannot be used

Web Start does not run your command line — `javaws` reads the `.jnlp`, applies the JNLP **secure vm-args whitelist**, and builds the JVM invocation itself. `-javaagent` is not on that whitelist, so any attempt to add it is dropped. That is why Web Start automation goes through runtime attach rather than `Connect To Application`.

### The IcedTea-Web block is structural

IcedTea-Web installs a `JNLPSecurityManager`. When you attach an agent, the JVM loads foreign code that ITW's security manager cannot classify against the JNLP's code source, so it **denies the agent's initialization**. The attach itself lands, but the agent never finishes starting.

This is **not** a permissions dial you can turn up. It is independent of the app's permission level — an `all-permissions`, signed JNLP is blocked in exactly the same way. (This was tested directly: a signed all-permissions app under IcedTea-Web still failed to initialize the agent.)

When it happens, `Launch Web Start Application` raises a clear `AttachError` that names the `SecurityManager` — it does not hang.

**Supported Web Start paths** are launchers without the legacy security manager:

- **Modern OpenWebStart** (no `JNLPSecurityManager`).
- A **JDK 24+** `javaws`.

You can run a **portable** IcedTea-Web / OpenWebStart image without root — unzip it and point `launcher=` (or `JAVAGUI_JAVAWS`) at the image directory.

---

## Troubleshooting

| Symptom | Cause | Fix |
|---------|-------|-----|
| `AttachError: agent loaded but failed to initialize … SecurityManager` (mentions `AgentInitializationException`) | The target runs under a restrictive `SecurityManager` — almost always an IcedTea-Web JNLP app. | Use a launcher without the legacy security manager (OpenWebStart / JDK 24+ `javaws`). Raising the app to all-permissions does **not** help. |
| `AttachError` listing several candidate JVMs | Your selector matched more than one process. | Narrow the `main_class` regex, or select by `pid` (from `List Applications`). |
| `AttachError` with an empty candidate list | Nothing matched. | Check the app is running as **your** user; widen the selector; add `include_launchers=True` if it is a Web Start launcher. |
| `no JDK 'java' (with jdk.attach) and no 'jattach' found` | The test host has only a JRE. | Install a JDK, set `JAVAGUI_JAVA`, or supply `jattach` via `JAVAGUI_JATTACH`. |
| Target console prints *"a dynamic agent has been loaded"* | Informational JDK 21+ warning. | Ignore it. |
| Attach fails on a **JDK 24+** target | Dynamic agent loading is disabled by default. | Launch the target with `-XX:+EnableDynamicAgentLoading` (set at the target's launch). |
| `RPC port … never opened` | Agent loaded but its server did not come up in time. | Increase `timeout`; confirm the toolkit hint is right (try `toolkit=auto`). |

---

## Attach from `javagui-spy`

The spy CLI can attach to a running JVM too — the same discover-and-inject path, with no `-javaagent` on the target. Use it to explore locators against an app you didn't launch:

```bash
# By PID
javagui-spy dump-tree --attach-pid 48213 --toolkit auto

# By main-class / jar regex
javagui-spy suggest --attach-main-class 'testapp.SwingTestApp' --node-id 7

# By window title (needs wmctrl)
javagui-spy find "text:Save" --attach-title '*Editor*'
```

| Flag | Meaning |
|------|---------|
| `--attach-pid PID` | Attach the agent to a running JVM by PID. |
| `--attach-main-class REGEX` | Attach by main-class / jar regex. |
| `--attach-title PATTERN` | Attach by window title (needs `wmctrl`). |

These compose with the usual connection flags (`--toolkit`, `--host`, `--port`, `--timeout`). See [docs/spy.md](spy.md) for the rest of the spy workflow.

---

## See also

- **[README → Attach to a Running Application](../README.md#attach-to-a-running-application)** — the quick reference and keyword table.
- **[docs/spy.md](spy.md)** — the `javagui-spy` locator tool.
