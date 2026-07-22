# Design — agent-facing documentation

Docs-only change. Decisions below are grounded in July-2026 research (Fable agent, citations at
the end) and the repo's actual state (verified: `invoke` task names, the `javagui-spy schema`
surface, the 33-file `docs/` sprawl, the Claude-Flow `CLAUDE.md`).

## 1. Two audiences, two artifacts, two voices

```
  CONTRIBUTOR-AGENT (develops the lib)          AUTOMATION-AGENT (uses the lib)
  ────────────────────────────────────          ───────────────────────────────
  home: AGENTS.md (+ nested)                     home: docs/llms.txt + cheatsheet
  voice: terse imperative, command-first,        voice: example-first (RF voice),
    "run X → expect Y", done-gate                  lead with the runnable snippet
  answers: how do I build/test/not-break?        answers: how do I connect/attach,
                                                   write a locator, verify it?
```

They are **separate** on purpose — mixing them wastes context and breeds contradictions
(single-source-of-truth is the top anti-drift guard).

## 2. Tool-agnostic wiring — `AGENTS.md` canonical, thin pointers

`AGENTS.md` is the convergence point (natively read by Codex, Cursor, Copilot, Gemini/Jules,
Windsurf, Zed, Aider, opencode, VS Code, …). Everything else points to it.

```
  AGENTS.md  ◄── See @AGENTS.md ── CLAUDE.md
       ▲       ◄──────────────────  .github/copilot-instructions.md
       │       ◄──────────────────  .cursor/rules/agents.mdc
       │       ◄──────────────────  GEMINI.md
       └── nested (nearest wins): src/ · agent/ · python/JavaGui/ · tests/apps/
```

**Decision: stub-pointer, not symlink.** A Java-GUI library attracts Windows contributors, where
symlinks break on checkout / some CI; a stub (`See @AGENTS.md` + optional tool-only lines) is
portable and can carry tool-specific extras. Trade-off accepted: a one-line pointer is trivial
duplication vs. a symlink's zero-duplication.

**Decision: nested `AGENTS.md` per subproject.** The polyglot build (Rust + Java + Python) means a
rule usually applies to one component; the nearest-file precedence keeps each root/leaf minimal.

## 3. `AGENTS.md` content model (dev-agent)

Keep under ~200 lines (every line spends each session's context budget); deep detail behind links.
Sections:

1. **Overview** — 2–3 lines: Rust core + Java agent + Python RF keywords for Swing/SWT/RCP.
2. **Setup** — exact commands (`source ~/.cargo/env && uv sync --all-groups`; Maven download+path).
3. **Build** — per component: `uv run invoke build-dev` (agent+Rust, no wheel), `invoke build`
   (full), agent `mvn -f agent/pom.xml package`, test apps `cd tests/apps && mvn package`.
4. **Test / verify-loop** — the ORDERED pipeline with expected results, e.g.
   `uv run robot --dryrun -d results/dryrun tests/robot/` → *expect 1159 passed / 0 failed /
   30 skipped*; pytest / cargo / clippy / ruff / robocop baselines. Full RF run needs `xvfb` + live
   Java apps.
5. **Definition of done** — build ok · pipeline at baseline · lint deltas explained · no secrets.
6. **The rebuild+sync loop** — after Java changes: rebuild the agent jar AND copy it into
   `python/JavaGui/jars/` (or `invoke build-dev`), then re-run tests. After Rust changes:
   `uv run maturin develop --release`.
7. **Gotchas / failure modes** — `pkill -f <pattern-in-your-own-command>` self-kills the shell;
   apps don't persist across tool calls (launch+drive in ONE command / `xvfb-run`); `Broken pipe`
   flakiness worsens under load/large responses; `toolkit=swt` must be forced at premain (auto works
   at attach); scratchpad files can vanish mid-session.
8. **Style & structure** — 100-char lines, `ruff`/`cargo fmt`, ≤500-line files, typed public APIs.
9. **PR rules** — branch (not `main`), commit trailer, describe tests run.
10. **Links** — nested `AGENTS.md`, `docs/runtime-attach.md`, `docs/spy.md`, this openspec dir.

Nested files carry only the local specifics (e.g. `agent/AGENTS.md`: `toolkit=swt` + the OSGi
bundle-classloader lesson; `src/AGENTS.md`: PyO3/maturin, `cargo test` layout; `tests/apps/AGENTS.md`:
Maven-at-`/tmp/apache-maven-3.9.9`, per-toolkit run recipes).

## 4. Usage channel (automation-agent)

**Source of truth stays the code** — prose points to the self-describing surfaces rather than
duplicating them (least drift):

- `javagui-spy schema` → verbs + locator grammar cheatsheet + candidate contract (agent bootstrap).
- `javagui-spy mcp` → MCP stdio server exposing the verbs as tools.
- Generated **libdoc** (`invoke docs` → `docs/keywords/{Swing,Swt,Rcp}.html`) → keyword reference;
  add a machine-readable **libdoc JSON/XML** emit so agents enumerate keywords/args without scraping
  HTML.

Prose artifacts (thin, example-first RF voice):

- **`docs/llms.txt`** (llmstxt.org format): `# Title` + one-line summary blockquote, then
  `## Getting started`, `## Attach vs Connect`, `## Locators`, `## Keywords by toolkit` (links to
  libdoc), `## Spy workflow`, `## Gotchas`, and a `## Optional` section (advanced/internals) that
  may be dropped for a shorter context.
- **`docs/llms-full.txt`** (optional): concatenated context for direct ingestion.
- **`docs/agent-usage-cheatsheet.md`**: the ~20 highest-value keywords, the locator grammar, the
  **verify-loop** front-and-centre (`dump-tree → find → validate` exit codes `0/2/3/4`; run
  `javagui-spy suggest` when stuck), the connect-vs-attach decision, and the
  `Element Text Should Be` vs `Get Element Text` divergence on `JGSearchField`.

## 5. Curate `docs/`

Keep: `runtime-attach.md`, `spy.md`, `keywords/*.html`, the new agent docs, the current
`RELEASE_NOTES_*`. Consolidate each duplicate cluster to one canonical page
(COMPONENT_TREE ×6→1, SWT ×4→1, OUTPUT_FORMAT ×3→1, QUICK_REF ×3→1); FEATURE_* comparison charts →
`docs/archive/`. Move phase/mission/status/`*_v2` throwaways → `docs/archive/` via `git mv`
(history preserved). Leave a short `docs/README.md` index so the folder is navigable.

## 6. Anti-drift (docs as engineering artifacts)

- **Single source + pointers** — one file to edit; pointers carry no facts.
- **Generated sections** — libdoc index + `llms.txt` emitted from source in the docs build; they
  cannot drift.
- **CI doc-lint** (new job/step): every pointer file resolves to `AGENTS.md`; no dead relative
  links in `AGENTS.md`/`llms.txt`/cheatsheet; the baseline numbers quoted in `AGENTS.md` match a
  small machine-checkable source (or are marked `~approx`).
- **Doc-tested commands** — the cheatsheet's runnable blocks are exercised by the existing
  `robot --dryrun` / pytest run so a renamed keyword breaks CI, not silently the docs.
- **Migration rule** — instruction files are on the change checklist: any change that renames a
  command/keyword updates them in the same PR.

## 7. Alternatives considered

- **Symlink pointers** — zero duplication but break on Windows/CI; rejected for a cross-platform lib.
- **One combined agent doc** — mixes audiences, wastes context, invites contradiction; rejected.
- **Hand-written keyword cheatsheet as source of truth** — drifts from code; rejected in favour of
  generated libdoc + `schema`, with the cheatsheet limited to curated high-value examples.
- **Leave `CLAUDE.md` as-is** — it actively misleads and contradicts `AGENTS.md`; rejected.

## Sources (research, July 2026)
agents.md · llmstxt.org · Anthropic Claude Code best-practices (≤~200-line, verify-loop, iterate) ·
kau.sh & Towards-AI (one source of truth, stub-pointer) · Codacy (instructions as engineering
artifacts, CI drift checks) · morphllm & llms-txt.io (AGENTS.md structure, nested convention,
AGENTS.md-vs-llms.txt) · Mintlify (llms-full.txt) · langchain-ai/mcpdoc (MCP usage channel) ·
tairov/awesome-agents.md, KbWen/agentic-os (exemplars, definition-of-done gates).
