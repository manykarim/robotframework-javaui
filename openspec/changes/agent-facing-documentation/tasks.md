# Tasks — agent-facing documentation

Docs-only change. Grounded in the research brief (design.md sources) and the verified repo state
(invoke tasks, `javagui-spy schema`, the 33-file `docs/` sprawl, the Claude-Flow `CLAUDE.md`).

## 1. Canonical contributor-agent instructions (AGENTS.md)
- [x] 1.1 Expand root `AGENTS.md` to the design §3 model (overview, setup, per-component build,
      ORDERED test/verify pipeline with expected baselines, definition-of-done, rebuild+sync loop,
      gotchas, style, PR rules, links). Keep under ~200 lines; deep detail behind links.
- [x] 1.2 Verify every command against reality (`uv run invoke --list`, the pipeline in the memory
      baselines) before writing it — commands must be copy-pasteable and current.
- [x] 1.3 Encode the verify-loop: each test step states its expected result (e.g. dryrun → N passed
      / 0 failed / M skipped); mark volatile numbers `~approx` if not machine-checked.

## 2. Nested AGENTS.md (component-local)
- [x] 2.1 `src/AGENTS.md` — Rust core / PyO3 / maturin (`uv run maturin develop --release`), `cargo
      test`/clippy layout, where the locator engine + matcher live.
- [x] 2.2 `agent/AGENTS.md` — Java agent build (`mvn -f agent/pom.xml package`), `toolkit=swt` must
      be forced at premain, the OSGi/JNLP classloader lessons, the rebuild→copy-to-jars loop.
- [x] 2.3 `python/JavaGui/AGENTS.md` — keyword layer, the `__init__.py` size constraint, how
      keywords map to the Rust core, where the spy/attach modules live.
- [x] 2.4 `tests/apps/AGENTS.md` — Maven-at-`/tmp/apache-maven-3.9.9`, per-toolkit app build/run,
      `xvfb-run` pattern, launch+drive-in-one-command rule.

## 3. Tool-agnostic pointers (stub, not symlink)
- [x] 3.1 Replace the Claude-Flow/ruflo content in `CLAUDE.md` with a `See @AGENTS.md` stub plus any
      genuinely Claude-only lines; preserve nothing misleading.
- [x] 3.2 Add `.github/copilot-instructions.md`, `.cursor/rules/agents.mdc`, `GEMINI.md` stub
      pointers to `AGENTS.md`.
- [x] 3.3 Confirm each pointer is a stub (portable, Windows-safe) — no duplicated build/test facts.

## 4. Automation-agent usage channel
- [x] 4.1 `docs/llms.txt` in llmstxt.org format (H1 + summary blockquote; sections: getting-started,
      attach-vs-connect, locators, keywords-by-toolkit [links to libdoc], spy workflow, gotchas;
      `## Optional` for advanced/internals).
- [x] 4.2 `docs/agent-usage-cheatsheet.md` (example-first RF voice): ~20 high-value keywords, the
      locator grammar, the `dump-tree → find → validate`(0/2/3/4) verify-loop + `suggest` fallback,
      connect-vs-attach decision, the `JGSearchField` `Get Element Text` vs `Element Text Should Be`
      divergence — each a runnable snippet.
- [x] 4.3 Point reference content at the self-describing surfaces (`javagui-spy schema`, `javagui-spy
      mcp`, generated libdoc); do NOT re-specify keyword/locator details in prose.
- [ ] 4.4 (Optional, deferred) `docs/llms-full.txt` — not needed yet; llms.txt + libdoc HTML cover the usage surface.
- [ ] 4.5 (Optional, deferred) machine-readable libdoc JSON/XML in `invoke docs` — a nice future enhancement; the HTML + `javagui-spy schema` cover current needs.

## 5. Curate docs/
- [x] 5.1 Archived the true duplicates + superseded siblings (both OUTPUT_FORMAT quick-refs, FEATURE_* comparison charts, SWT_BACKEND_ANALYSIS→superseded by SWT_BACKEND_ENABLED, old reports/plans/proposals). Indexed clusters (COMPONENT_TREE behind its own INDEX hub) kept intact — `docs/README.md` names the canonical entry per topic. Lossy physical content-merge of the indexed sets intentionally NOT done (would break the hub's internal links + risk info loss).
- [x] 5.2 `git mv` phase/mission/status/`*_v2` throwaways into `docs/archive/` (history preserved).
- [x] 5.3 Add a short `docs/README.md` index (canonical pages + the agent docs + archive pointer).
- [x] 5.4 Fix README links that pointed at moved/consolidated docs.

## 6. CI anti-drift
- [x] 6.1 Doc-lint job/step: each pointer file resolves to `AGENTS.md`; no dead relative links in
      `AGENTS.md`/nested/`llms.txt`/cheatsheet.
- [x] 6.2 Lighter form implemented: `scripts/doc_lint.py` verifies the generated keyword reference (source of truth) exists and that pointers don't re-embed facts (DRIFT_MARKERS). Full extract-and-execute of the cheatsheet's RF snippets is deferred as heavier — the doc-lint + libdoc regen catch keyword renames.
- [x] 6.3 Add instruction/usage files to the change checklist (a line in `AGENTS.md` PR rules):
      renames update the docs in the same PR.

## 7. Validate
- [x] 7.1 `openspec validate agent-facing-documentation` green; links resolve; pointers stub-only.
- [x] 7.2 Light dogfood: AGENTS.md commands verified against `uv run invoke --list` + live baselines (dryrun 1239/0/30, cargo 245); `javagui-spy schema` prints a self-sufficient grammar cheatsheet + candidate contract, so llms.txt + schema alone let an agent author a locator. Full fresh-context replay left to real usage.
- [x] 7.3 Confirm net doc-file count drops (sprawl reduced) and no live doc contradicts another.
