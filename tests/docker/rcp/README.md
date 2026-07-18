# Real-world Eclipse RCP automation harness (DBeaver CE)

Reproducible Docker harness that automates a **real, public, packaged Eclipse RCP product**
— [DBeaver Community Edition](https://dbeaver.io) — headless with the JavaGui agent, and
captures framebuffer screenshots as visual confirmation. This is the evidence base for the
OpenSpec change `rcp-real-app-automation`.

## Run

```bash
# 1. Build the agent jar if needed
mvn -f agent/pom.xml package

# 2. Build the harness image (downloads DBeaver CE + a full JDK 21; ~2–4 min first time)
docker build -t rcp-dbeaver-harness tests/docker/rcp

# 3. Run: launches DBeaver headless, drives it, writes results + screenshots
docker run --rm --shm-size=512m -v "$PWD":/work rcp-dbeaver-harness

# 4. Inspect
xdg-open results/dbeaver/log.html          # Robot report (screenshots embedded)
ls      results/dbeaver/shots/*.png        # framebuffer screenshots per step
less    results/dbeaver/logs/dbeaver.log   # product + agent log
```

The Robot suite (`tests/robot/rcp/real_dbeaver/`) self-skips if the agent port is unreachable,
so it is safe to run outside the harness too.

## Why the harness does what it does (findings baked in)

- **Full JDK via `-vm`** — DBeaver's bundled `jlink` JRE omits the `java.instrument` module, so a
  `-javaagent` cannot load (`libinstrument.so: cannot open shared object file`). The entrypoint
  installs a full JDK 21 and repoints `dbeaver.ini` `-vm` at it (before `-vmargs`).
- **Wait for the rendered workbench** — the agent RPC port opens at premain, ~6 s *before* the SWT
  `Display` and workbench window exist. The entrypoint waits for the framebuffer to render before
  driving; otherwise the first `ping` fails with `SWT not initialized`.
- **X-framebuffer screenshots** — the library's own `Capture Screenshot` is currently a no-op
  (Rust stub), so visual confirmation uses `import -window root` on the Xvfb display. See the
  OpenSpec change for the fix that makes the library keyword work end-to-end.

## Parameters

- `--build-arg DBEAVER_URL=...` — pin a different DBeaver build.
- `AGENT_PORT` env — agent RPC port (default 5682).
