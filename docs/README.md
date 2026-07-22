# docs/ — index

Navigation for humans and agents. **Agents:** start with the two files under "For AI agents".
Canonical build/test/contributor instructions live in the repo-root [`AGENTS.md`](../AGENTS.md).

## For AI agents
- [`llms.txt`](llms.txt) — orientation index for agents *using* the library (llmstxt.org format).
- [`agent-usage-cheatsheet.md`](agent-usage-cheatsheet.md) — example-first: connect/attach, locator
  grammar, the spy verify-loop, top keywords, gotchas.
- [`keywords/`](keywords/) — generated **libdoc** keyword reference (`Swing.html`, `Swt.html`,
  `Rcp.html`); the machine-readable source of truth. Regenerate with `uv run invoke docs`.
- Self-describing surfaces (authoritative): `javagui-spy schema` (verbs + grammar + candidate
  contract) and `javagui-spy mcp` (MCP server).

## Using the library
- [`runtime-attach.md`](runtime-attach.md) — attach to a running JVM + Java Web Start (JNLP).
- [`spy.md`](spy.md) — the `javagui-spy` tool (scan a UI, generate verified locators).
- [`COMPONENT_TREE_DOCUMENTATION_INDEX.md`](COMPONENT_TREE_DOCUMENTATION_INDEX.md) — component-tree
  guides hub (quick-start / filtering / RCP / API changes).
- [`OUTPUT_FORMATS_GUIDE.md`](OUTPUT_FORMATS_GUIDE.md) — tree/output formats.
- [`SWT_QUICK_START.md`](SWT_QUICK_START.md), [`SWT_BACKEND_ENABLED.md`](SWT_BACKEND_ENABLED.md) — SWT.
- [`USER_PERFORMANCE_GUIDE.md`](USER_PERFORMANCE_GUIDE.md),
  [`PERFORMANCE_OPTIMIZATION_GUIDE.md`](PERFORMANCE_OPTIMIZATION_GUIDE.md),
  [`RUNNING_BENCHMARKS.md`](RUNNING_BENCHMARKS.md) — performance.
- [`MIGRATION_GUIDE.md`](MIGRATION_GUIDE.md), [`DEPLOYMENT_CHECKLIST.md`](DEPLOYMENT_CHECKLIST.md).
- `RELEASE_NOTES_v*.md` — per-release notes (latest: `RELEASE_NOTES_v0.8.0.md`).

## Design & internals
- [`ddd-design-model.md`](ddd-design-model.md), [`DDD_VISIBILITY_DESIGN.md`](DDD_VISIBILITY_DESIGN.md)
  — domain design.
- Subdirectories: `adr/` (decision records), `architecture/`, `api-reference/`, `user-guide/`,
  `performance/`, `research/`, `specs/`, `test-plans/`, `examples/`.
- Living specs & change history are in [`../openspec/`](../openspec/) (not here).

## Archive
- [`archive/`](archive/) — historical/superseded docs (old reports, completed implementation plans,
  superseded proposals, comparison charts). Kept for history; not current guidance.
