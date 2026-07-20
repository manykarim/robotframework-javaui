# Java Web Start (JNLP) automation harness

Reproducible Docker harness that launches a **real Java Web Start (JNLP) application** with a
real Web Start launcher (**portable IcedTea-Web 1.8.8**), headless under Xvfb, and drives the
robotframework-javaui **runtime dynamic-attach** flow (`Launch Web Start Application`).

It reuses the repo's Swing test app (`tests/apps/swing/target/swing-test-app-1.0.0.jar`,
`Main-Class testapp.SwingTestApp`) via the sandbox JNLP in `tests/apps/jnlp/`.

## What it proves (honest scope)

- **Launch + discover** — the JNLP is served over `127.0.0.1:8099`, launched with IcedTea-Web,
  and the running JVM is discovered for injection.
- **The documented SecurityManager block** — under IcedTea-Web a `JNLPSecurityManager`
  **structurally blocks runtime dynamic attach**, *independent of the app's permission level*
  (a signed all-permissions JNLP was still blocked). So on this image the proven outcome is a
  clear, expected `SecurityManager` `AttachError` — **not** a successful attach.

A **fully-green attach** requires a launcher/JDK **without the legacy SecurityManager**
(modern **OpenWebStart**, or **JDK 24+**). Point `JAVAGUI_JAVAWS` at such an image to get a
green connect; on IcedTea-Web the suite instead proves the failure is the documented block.

The Robot suite (`tests/robot/jnlp/01_webstart.robot`) asserts exactly that **either/or** and
never asserts success unconditionally, so the harness is honest by construction.

## Run locally

```bash
# 1. Build the host artifacts the harness bind-mounts (agent jar + in-source _core + swing app)
uv run invoke build
uv run maturin develop --release
(cd tests/apps/swing && mvn package)

# 2. Build the harness image (downloads portable IcedTea-Web 1.8.8 + a full JDK 17)
docker build -t jnlp-harness tests/docker/jnlp

# 3. Run: serves the JNLP, launches it with IcedTea-Web, drives the attach, writes evidence
docker run --rm -v "$PWD":/work jnlp-harness

# 4. Inspect
xdg-open results/jnlp/log.html          # Robot report (the either/or assertion)
less    results/jnlp/logs/robot.stdout.log
less    results/jnlp/logs/httpd.log      # the local JNLP web server
```

The Robot suite self-skips (does not fail) when a precondition is missing — no Web Start
launcher, no `DISPLAY`, or the swing jar not built — so it is safe to run outside the harness
too. Inside this image the launcher (`JAVAGUI_JAVAWS=/opt/icedtea-web-image`), `DISPLAY`
(Xvfb `:99`) and the bind-mounted swing jar are all present, so the suite actually runs.

## Why the harness does what it does (findings baked in)

- **Full JDK 17, not `-headless`** — the attach injection runs the bundled agent jar's
  `AttachMain` under a `java` that has the `jdk.attach` module; the full JDK also carries the
  AWT/Swing native libs the launched app needs under Xvfb.
- **Portable IcedTea-Web (no root)** — unzipped to `/opt/icedtea-web-image`; a directory is
  treated as an ITW image and launched via `bin/itw-modularjdk.args` +
  `share/icedtea-web/javaws.jar` + `net.sourceforge.jnlp.runtime.Boot`.
- **No trust/grant dialogs** — `~/.config/icedtea-web/deployment.properties` sets
  `deployment.security.level=ALLOW_UNSIGNED` and `deployment.security.askgrantdialog.show=false`
  so the launch runs unattended.
- **Ubuntu 24.04 base** — matches the host that builds `_core.abi3.so`, so the bind-mounted
  abi3 extension is glibc-compatible (same rationale as `tests/docker/rcp`).

## Parameters

- `--build-arg ITW_URL=...` — pin a different IcedTea-Web portable build.
- `SERVE_PORT` env — local JNLP web server port (default 8099; must match the JNLP codebase).
- `SUITE_DIR` env — override the Robot suite directory (default `/work/tests/robot/jnlp`).
