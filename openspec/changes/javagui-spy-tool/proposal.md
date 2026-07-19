## Why

Writing locators for off-the-shelf Java desktop apps is the painful part of using
robotframework-javaui. Real Swing/SWT/RCP software has **deeply nested widget trees where most
widgets have no `name`** and `text`/`type` repeat many times (identical rows, cells, buttons).
Today a user hand-crafts a locator, runs the test, sees 0 or N matches, and iterates blind.

There is **no spy/inspector tool** in the ecosystem: `[project.scripts]` is empty, `scripts/`
has no scanner, and nothing *synthesizes* a locator — the Rust matcher can only *evaluate* one.
Meanwhile the substrate to build a great spy already exists: a rich locator grammar (CSS + XPath
+ `>>` cascade + capture `*` + geometry + `:nth-of-type`/`:has()`), a structured tree with
geometry (`getComponentTree`/`Get UI Tree`), screenshot-as-data-URL RPCs, and a dependency-light
TCP JSON-RPC channel any external tool can reuse.

This change captures **deep research (via Fable agents) + a validated design** for `javagui-spy`:
a tool that scans a running Java app and generates unique, *pre-verified* Robot Framework
locators — with a graphical UI and a CLI optimized for AI agents. It is explore-mode output
(research + design + evidence), not an implementation.

### Validated crux — round-trip identity experiment (evidence/)

A throwaway generator was round-tripped through the **production** `find_elements` matcher against
the live Swing test app (137 nodes, depth 11), in two modes:

| Mode | Working unique locator | Notes |
|------|------------------------|-------|
| Full attributes | **106/136 (77.9%)** | single `Type[name\|text]` = **100%** (87/87); anchored `>>` adds 19 |
| **Names stripped** (off-the-shelf sim) | **73/136 (53.7%)** | single `Type[text]` = **100%** (63/63); anchored `>>` covers deep no-name nodes |

**Every actionable widget** (button/field/label/combo/menu) got a working, concise locator
(median 26 chars). Every *uncovered* node is a structural/anonymous container. The 22% shortfall
is entirely two **prototype-only** divergences that the design forbids — approximate
`nth-of-type` (vs the production `get_type_index`) and tree-space geometry (vs matcher-space) —
which is itself the empirical case for the design's core rule: **parity by construction**.

## What Changes

- **Add** `javagui-spy` — an external client of the existing JSON-RPC agent (Appium-Inspector
  model), shipped inside the `robotframework-javagui` wheel as a `[project.scripts]` console
  entry point. One SpyCore serves a graphical UI, an agentic CLI, and (later) an MCP façade.
- **Add** a Rust **locator generator** (`src/locator/generator.rs`) that reuses the production
  parser/matcher (`parse`, `find_matching_components`, `find_cascaded_with_capture`,
  `get_type_index`) as its **uniqueness oracle** — a candidate is emitted only if it resolves to
  exactly one node whose id equals the target. Tiered ladder (name > accessiblename > text >
  tooltip), then nearest-stable-ancestor `>>` anchored chains, then `:has()`/capture container
  pinning, with `nth`/geometry as explicitly brittle-flagged last resorts.
- **Add** three small public-API-only agent RPCs (later phase): `hitTest(x,y)` (click-to-inspect
  ancestor path), `highlight` (hollow overlay), `getUiGeneration` (change polling).
- **Add** a round-trip validation harness + CI gates on the Swing/SWT/showcase/DBeaver matrix.

## Capabilities

### New Capabilities
- `spy-locator-generation`: synthesize a unique, stable, human-readable RF locator for any node —
  verified against the live tree, with anchored-chain and flagged fallbacks for deep no-name
  non-unique widgets.
- `spy-scan-and-pick`: scan a live instrumented app over the existing agent RPC (per-window,
  lazy, cached), pick a widget (screenshot-click now; `hitTest` later), and highlight matches.
- `spy-cli-agentic`: a stateless, `--json`, exit-code-driven CLI (`dump-tree`/`find`/`validate`/
  `suggest`/`describe`/`screenshot`/`schema`) an LLM can chain deterministically.

## Impact

- **Rust:** new `src/locator/generator.rs` (reuses matcher; PyO3 `suggest_locators`).
- **Python:** new `python/JavaGui/spy/` (core, cli, server, static) + `[project.scripts]`
  `javagui-spy`; maturin `include` extended for spy static assets.
- **Agent (Java):** ~50–100 LOC/toolkit for `hitTest`/`highlight`/`getUiGeneration` (public
  AWT/SWT APIs; SWT via `syncExec` with timeouts) — a later phase; the CLI MVP needs **zero**
  agent changes.
- **Tests/CI:** `tests/spy/` round-trip harness; PR tier (swing/swt/showcase under xvfb) +
  nightly DBeaver gate on `spy_validation_metrics.json` with a permanent failed-locator corpus.
- **Non-goal:** not a recorder. `Copy as RF` is the boundary. No JS/Java-side locator parsing
  (parity break). Full research + design detail: `evidence/fable-research-report.md`.
