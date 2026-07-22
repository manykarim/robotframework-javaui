# AGENTS.md — robotframework-javagui

Canonical instructions for AI coding agents working ON this repository. This is the single source
of truth; tool files (`CLAUDE.md`, `.github/copilot-instructions.md`, `.cursor/rules/`, `GEMINI.md`)
are pointers here. Using the library to write automation? See `docs/llms.txt` +
`docs/agent-usage-cheatsheet.md`.

## Overview
A Robot Framework library for automating Java **Swing / SWT / Eclipse RCP** GUIs. Three layers:
- **Rust core** (`src/`) — locator parsing/matching, RPC/connection (PyO3 → `JavaGui._core`).
- **Java agent** (`agent/`) — in-JVM introspection over JSON-RPC/TCP; ships in the wheel at
  `python/JavaGui/jars/javagui-agent.jar`.
- **Python keywords** (`python/JavaGui/`) — the RF library (`SwingLibrary`/`SwtLibrary`/`RcpLibrary`),
  the `_attach` runtime-attach helper, and the `javagui-spy` tool.

Each subproject has its own nested `AGENTS.md` — read the nearest one.

## Setup
```bash
source ~/.cargo/env                 # Rust toolchain on PATH
uv sync --all-groups                # Python deps (uv is required; do NOT use pip directly)
uv tool install maturin             # build backend for the Rust extension
# Java 17 + Maven 3.9+ required to (re)build the agent and test apps.
```

## Build
```bash
uv run invoke build-dev             # agent JAR + Rust ext for local dev (no wheel) — the usual one
uv run invoke build                 # full: agent + copy JAR into package + wheel (release)
uv run maturin develop --release    # rebuild ONLY the Rust extension after src/ changes
mvn -f agent/pom.xml package -DskipTests   # rebuild ONLY the Java agent (then re-sync, see below)
```
After a Java-agent change, the JAR must land in the package: `invoke build-dev` copies it, or do it
by hand: `cp agent/target/javagui-agent.jar python/JavaGui/jars/`. See `agent/AGENTS.md`.

## Test / verify-loop  (run the step, check the expected result)
Ordered fastest→slowest. Numbers are current baselines (~approx — re-run and compare, don't regress).
```bash
uv run robot --dryrun -d results/dryrun tests/robot/     # syntax, no Java   → ~1239 tests, 0 failed, 30 skipped
uv run pytest tests/python/ tests/unit/                  # Python unit       → ~636 passed, 0 failed
cargo test                                               # Rust              → 245 passed, 0 failed
uv run invoke lint                                       # ruff + cargo clippy -D warnings
# Full RF integration (needs live Java apps + a display):
xvfb-run -a uv run robot -d results tests/robot/swing    # (build the test app first — see tests/apps/AGENTS.md)
```
`robot --dryrun` = fast syntax precheck (no Java). The full `robot` run needs the test apps built
AND a display (`xvfb-run` in headless/CI). Robot/pytest live tests self-skip when their app or
`DISPLAY` is absent.

## Definition of done
- [ ] Build succeeds (`invoke build-dev`).
- [ ] The pipeline above is at (or better than) its baselines — no regressions.
- [ ] Lint deltas explained; new keywords covered by both an RF suite and a pytest.
- [ ] Docs updated in the SAME change if a command/keyword was renamed (see Anti-drift).
- [ ] No secrets/credentials committed.

## Gotchas (documented because agents cannot infer them)
- **`pkill -f <pattern>` can self-kill your shell** if the pattern appears in your own command line
  (e.g. `pkill -f smart-client` while your command contains "smart-client"). Kill by a pattern that
  is NOT in your invocation.
- **Launched apps do NOT persist across tool calls.** Launch the Java app AND drive it in ONE
  command (`subprocess.Popen` inside `xvfb-run uv run python`, or a Robot `Suite Setup`).
- **`Broken pipe (os error 32)` is flaky under load / large responses**, not a real regression —
  re-run; reduce concurrency.
- **The agent must be forced `toolkit=swt`** at launch-time (`premain`) for SWT/RCP — auto-detection
  fails before SWT loads. At runtime *attach* (`agentmain`), `toolkit=auto` works. See `agent/AGENTS.md`.
- **Scratchpad files can vanish mid-session** — write probe scripts to `/tmp`, not the scratchpad.
- **Version lives in three files** — bump `pyproject.toml`, `Cargo.toml`, and `Cargo.lock`/`uv.lock`
  together.

## Style & structure
- Python: 4-space indent, 100-char lines, `ruff format` + `ruff` + `mypy`.
- Rust: `cargo fmt`; keep `cargo clippy -D warnings` clean.
- Robot: numeric-prefixed suites (`01_connection.robot`), readable test names.
- Keep files ≤~500 lines; typed public APIs; input validation at boundaries.
- Layout: `src/` (Rust) · `agent/` (Java) · `python/JavaGui/` (keywords) · `tests/{python,unit,robot,apps}`
  · `openspec/` (specs + changes) · `docs/` (see `docs/README.md`).

## PR & workflow
- Branch off `main` (never commit to `main` directly). Conventional commit prefix (`fix:`, `feat:`,
  `docs:`, `release:`). Describe behavior changes + list the test commands you ran.
- This repo uses OpenSpec for non-trivial work: `openspec/changes/`. `/opsx:explore|propose|apply|archive`.

## Anti-drift
Instruction/usage docs are engineering artifacts. If you rename a command or keyword, update
`AGENTS.md`, the nested files, and `docs/llms.txt`/`docs/agent-usage-cheatsheet.md` in the SAME
change. Tool-specific files must stay thin pointers to this file — never copy facts into them.

## Deeper docs
- Nested: `src/AGENTS.md`, `agent/AGENTS.md`, `python/JavaGui/AGENTS.md`, `tests/apps/AGENTS.md`.
- `docs/runtime-attach.md` (attach + Web Start), `docs/spy.md` (the spy tool), `docs/keywords/*.html`
  (libdoc keyword reference), `docs/README.md` (docs index).
- Using the library as an agent: `docs/llms.txt` + `docs/agent-usage-cheatsheet.md`.
