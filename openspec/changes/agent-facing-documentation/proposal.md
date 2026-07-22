## Why

AI coding agents are now first-class users of this repository — both **developing** it (build the
Rust/Java/Python stack, run the test pipeline, avoid the traps) and **using** it (write Robot
Framework automation against Swing/SWT/RCP apps). The repo does not serve them well today:

- **The primary agent-instruction file actively misleads.** `CLAUDE.md` is ~230 lines of
  Claude-Flow / ruflo *swarm* boilerplate (topologies, "15 agents", hive-mind consensus) that has
  nothing to do with building this library. A Claude-based agent following it is steered away from
  the real toolstack. It also **contradicts** `AGENTS.md`, and single-source-of-truth is the top
  guard against agents getting lost.
- **`AGENTS.md` is a thin, dev-only skeleton.** Its commands are accurate but it omits the spy
  tool, runtime attach, MCP, the `xvfb` requirement, the ordered test pipeline with baselines, and
  the hard-won gotchas.
- **The real operational knowledge is not in the repo.** The build/test-pipeline order, the
  `pkill -f <self>` self-kill trap, `Broken pipe` flakiness under load, the agent-rebuild+sync
  loop, "apps don't persist across shell calls", `toolkit=swt` must be forced at premain — all live
  only in a maintainer's notes, not where an agent can read them.
- **`docs/` is 33 top-level files of sprawl** (COMPONENT_TREE ×6, SWT ×4, OUTPUT_FORMAT ×3,
  FEATURE ×3, QUICK_REF ×3, plus phase/mission/status throwaways). Only 3 are referenced from the
  README. An agent landing here cannot tell canonical from dead.
- **No usage channel built for agents.** The self-describing surfaces already exist —
  `javagui-spy schema`, the `javagui-spy mcp` server, and generated libdoc — but nothing points
  agents at them, and there is no `llms.txt` (the tool-agnostic convention that Cursor / Windsurf /
  Claude Code / Copilot / Cline all fetch).

Research (July 2026, cited in `design.md`) converges on a clear answer: **`AGENTS.md` is the
tool-agnostic convergence point** (20+ tools read it natively; Linux-Foundation-governed; 60k+
repos), tool-specific files should be **thin pointers** to it, `llms.txt` is the usage-side
channel, and instruction files must be treated as **engineering artifacts** with CI anti-drift.

## What Changes

- **Canonical dev-agent instructions in `AGENTS.md`** (terse, imperative, command-first, verify-loops
  "run X → expect Y", a definition-of-done gate, and the gotchas) — kept under ~200 lines with deep
  detail behind links (progressive disclosure).
- **Nested `AGENTS.md`** in `src/` (Rust/PyO3/maturin), `agent/` (Java/mvn/`toolkit=swt`),
  `python/JavaGui/` (keyword layer), and `tests/apps/` (Maven-at-`/tmp`, `xvfb`, app run) — agents
  read the nearest file.
- **Tool-agnostic pointers:** replace the swarm boilerplate in `CLAUDE.md` with a one-line
  `See @AGENTS.md` stub (+ any Claude-only lines), and add `.github/copilot-instructions.md`,
  `.cursor/rules/agents.mdc`, `GEMINI.md` stubs. **Stub-pointers, not symlinks** (Windows-safe).
- **Usage-agent channel:** `docs/llms.txt` (+ optional `docs/llms-full.txt`) and a curated,
  example-first `docs/agent-usage-cheatsheet.md`. Prose stays thin and **points to the
  self-describing surfaces** (`javagui-spy schema`, MCP, generated libdoc) as the source of truth —
  keep the friendly Robot Framework example-first voice here.
- **Curate `docs/`:** consolidate each duplicate cluster to one canonical page, move
  phase/mission/status throwaways to `docs/archive/`, and leave a navigable index.
- **CI anti-drift:** a doc-lint (pointers resolve, no dead links, documented baselines current) plus
  wiring the cheatsheet's runnable blocks into the existing `robot --dryrun` / pytest run.

## Impact

- **Docs-only, additive and non-breaking.** No source, keyword, or API change; no behavior change.
- Touches root instruction files, adds nested `AGENTS.md`, adds `docs/llms.txt` + cheatsheet, moves
  throwaway docs under `docs/archive/` (history preserved via `git mv`), and adds one CI job/step.
- **Two coordinated audiences, two voices:** terse imperative for contributor-agents; example-first
  RF voice for automation-author agents.
- **Reduces, not increases, doc surface** — the net effect is fewer live files, one source of truth,
  and machine-generated indexes that cannot drift.
