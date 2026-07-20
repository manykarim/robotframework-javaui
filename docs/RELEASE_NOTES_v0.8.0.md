# Release Notes — v0.8.0

Adds **runtime agent attach** (drive apps that are already running, including **Java Web Start**)
and completes the **javagui-spy** tool. Builds on v0.7.0; fully backward compatible.

## Highlights

### Attach to an already-running JVM — no `-javaagent` required
Until now the only way to automate an app was to control its launch and add
`-javaagent:javagui-agent.jar`. New keywords attach the agent to a **JVM that is already
running** via the JDK Attach API, for Swing/SWT/RCP:

- **`Attach To Application`** — select the target by `pid`, `main_class`, or window `title`;
  the agent is injected at runtime and the session connects. `toolkit=auto` detects the toolkit
  from the loaded classes (more reliable than launch-time detection).
- **`List Applications`** — real JVM discovery (`jps`/`jcmd` + `/proc`), classifying application
  vs. launcher processes. (Previously a `NotImplementedError` stub.)

Injection uses the JDK Attach API by default and falls back to **`jattach`** for JRE-only hosts.
Ambiguous selectors error with the candidate list rather than guessing.

### Java Web Start (JNLP) support
- **`Launch Web Start Application`** launches a `.jnlp` with `javaws`/OpenWebStart, discovers the
  application JVM (in-process *or* forked), attaches, and connects. `-javaagent` cannot ride along
  in a JNLP (it is not on the JSR-56 secure vm-args whitelist), so runtime attach is the mechanism.
- **Honest limitation:** an app under IcedTea-Web's legacy `SecurityManager` structurally blocks
  dynamic attach — *independent of the app's permission level* (an all-permissions signed JNLP is
  still blocked). Supported Web Start attach targets run under a launcher/JDK without the legacy
  SecurityManager (modern OpenWebStart, JDK 24+); the keyword reports a clear error otherwise.
- A self-hosted, self-skipping JNLP test suite + a Dockerized headless harness (`webstart-jnlp`
  CI job) prove launch → discover → attach end to end.

### Multi-AppContext apps are now fully visible
Java's `Window.getWindows()` is AppContext-scoped, so an attached agent used to miss windows
created in a separate `AppContext` — exactly what Web Start and applets do. The Swing agent now
enumerates **every AppContext** (opening the required `java.desktop` internals via the agent's
own `Instrumentation`, with a safe fallback), so those windows are found.

### javagui-spy tool — completed
- **Offline Rust locator generator** (`suggest_locators`): candidate generation verified against
  the tree with the production matcher, with **no per-candidate RPC** (the Python live oracle
  remains as a fallback).
- **Machine-correctable parse errors** (`explain_locator`): the CLI `validate` verb now reports a
  byte `position` and an expected-token `hint` for malformed locators.
- **Full SWT `armPick`** via reflection (Ctrl+Shift in-app pick), and a dedicated spy CI tier.
- **SpyCore SWT/RCP fix:** the spy's `dump-tree`/`find`/`suggest` now work for SWT/RCP (they
  previously errored on a missing tree call); the agent's SWT server is now thread-per-client.
  New `--attach-pid` / `--attach-main-class` / `--attach-title` flags make attach a first-class
  spy entry point.

## Documentation
- New README section **"Attach to a Running Application"** + the Java Web Start subsection.
- `docs/runtime-attach.md`: the injection/discovery model and the deployment matrix (dynamic-attach
  default, OpenWebStart JVM-args opt-in, `JAVA_TOOL_OPTIONS` rejected, JDK-version behavior).
- Keyword docs (`docs/keywords/*.html`) regenerated with the new keywords.

## Packaging
- ABI3 wheels (Python 3.8+) for Linux (manylinux), Windows, and macOS — one per platform, each
  bundling `javagui-agent.jar`, the `_core` native extension, the spy web UI (`spy/static/spy.html`),
  and the `javagui-spy` console entry point.
- No breaking changes: existing `-javaagent` launch + `Connect To Application(port)` are unchanged.

## Install
```bash
pip install robotframework-javagui==0.8.0
```
