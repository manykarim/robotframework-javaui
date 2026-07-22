# AGENTS.md — Java test applications (`tests/apps/`)

Nested guide for the Swing/SWT/RCP-mock apps the live Robot suites drive against. Repo-wide build,
verify-loop, and gotchas live in the root [`/AGENTS.md`](../../AGENTS.md) — read it first; this file
only covers building and launching the test apps. The agent JAR they load is built per `agent/AGENTS.md`.

## The apps
| Dir | Toolkit | Build → jar | Main-Class |
|-----|---------|-------------|-----------|
| `swing/` | Swing | `swing-test-app-1.0.0.jar` (shaded) | `testapp.SwingTestApp` |
| `swt/` | SWT | `swt-test-app-1.0.0-all.jar` (fat/`-all`) | `testapp.SwtTestApp` |
| `rcp-mock/` | SWT+JFace mock | `rcp-mock-test-app-1.0.0-all.jar` (fat) | `testapp.rcp.MockRcpApplication` |

`rcp/` is the real-Eclipse bundle (see `rcp/build-and-run-real-eclipse.sh`, not Maven-packaged here);
`jnlp/` holds a Web Start descriptor (`app.jnlp`) for `Launch Web Start Application`.

## Build (Java 17 + Maven 3.9+)
```bash
mvn -f tests/apps/swing/pom.xml package        # → swing/target/swing-test-app-1.0.0.jar
mvn -f tests/apps/swt/pom.xml package          # → swt/target/swt-test-app-1.0.0-all.jar
mvn -f tests/apps/rcp-mock/pom.xml package     # → rcp-mock/target/rcp-mock-test-app-1.0.0-all.jar
```
Built jars land in each app's `target/`. Use the shaded/`-all` jar (has all deps); `original-*.jar`
is pre-shade — do not use it. `mvn` at `/tmp/apache-maven-3.9.9` is a sandbox artifact, not real;
use a system `mvn 3.9+` or download from archive.apache.org.

## Launch one with the agent (for a live test)
Load the bundled agent and pick a port, then `Connect To Application  port=NNNN` from Robot/Python:
```bash
# Swing (toolkit auto-detects at premain):
java -javaagent:python/JavaGui/jars/javagui-agent.jar=port=18080 \
     -jar tests/apps/swing/target/swing-test-app-1.0.0.jar
# SWT / RCP-mock — you MUST force toolkit=swt at premain (auto fails before SWT loads):
java -javaagent:python/JavaGui/jars/javagui-agent.jar=port=18081,toolkit=swt \
     -jar tests/apps/swt/target/swt-test-app-1.0.0-all.jar
```
- **Headless/CI**: wrap in `xvfb-run -a` (needs a display) —
  `xvfb-run -a java -javaagent:...=port=NNNN,toolkit=swt -jar <app>`.
- **SWT VMARG**: none on Linux; on macOS add `-XstartOnFirstThread` (SWT UI-thread rule).
- **Apps do NOT persist across tool calls** — launch the app AND drive it in ONE command
  (`subprocess.Popen` inside `xvfb-run uv run python`, or a Robot `Suite Setup`), never launch in one
  call and connect in the next. Don't `pkill -f` a pattern that appears in your own command line.

The full RF integration run (`xvfb-run -a uv run robot -d results tests/robot/<toolkit>`) expects the
matching jar already built; suites self-skip when their app or `DISPLAY` is absent.
