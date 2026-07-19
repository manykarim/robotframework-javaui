# Design — javagui-spy

Full research (prior art + six dimensions + synthesis, produced by Fable agents) is in
`evidence/fable-research-report.md`. This is the condensed decision record + the validation
evidence that grounds it.

## Architecture (external client of the existing agent — Appium-Inspector model)

```
Browser GUI ─HTTP/SSE─┐
                      ├─ SpyCore (Python daemon, ONE serialized TCP connection) ─► agent (existing JSON-RPC 2.0)
CLI / LLM ──verbs─────┤     · tree cache keyed by agent node ids                    + 3 later RPCs:
MCP façade ─tools─────┘     · PyO3 Rust matcher + NEW generator                       hitTest, highlight, getUiGeneration
```

- **Attach** two ways, both zero-instrumentation for agented apps: `connect host:port` (the agent
  RpcServer already accepts concurrent clients — the spy sits *alongside* a running RF session)
  or `launch --jar app.jar [--toolkit swt]` (self-inject the wheel-bundled agent jar).
- **One tool, three toolkits:** all UI data arrives in a uniform node shape (id, type, name,
  text, geometry, state, child_count), so there is no per-toolkit UI code.
- **Transport discipline (non-negotiable):** single serialized connection, per-window + lazy tree
  fetches, `getUiGeneration` polling instead of auto-refresh, debounced verify, retry-once — the
  architectural defense against the known `Broken pipe (os error 32)` flakiness under load.
- **Pick:** screenshot-click + client-side rect hit-test for the MVP (zero agent change,
  headless-first, LLM-friendly); an in-JVM `hitTest(x,y)` RPC later for z-order ground truth
  (glass panes / layered panes — the same class the click-retargeting fix addressed).

## The locator-generation algorithm (the crux)

Placement `src/locator/generator.rs`, sharing the matcher's `parse`/`find_*`/`get_type_index` —
**no new grammar**. Uniqueness oracle (hard gate): `unique(c) := matches(c).len()==1 &&
matches[0].id == target.id` (right count, wrong node = FAIL). Batch-evaluated in one in-memory
pass over the cached tree; winner re-verified against a fresh tree before hand-out.

- **Attribute ladder** (per-node, best rung): `#name`(1.0) > `[accessiblename]`(0.9) >
  `[text]`(0.75) > `[tooltip]`(0.65) > `:nth-of-type(k)`(0.4) > `[index]`(0.25) > geometry(0.2/0.1).
  Ladder is per-toolkit data + a per-app recognition-rules config (e.g. JGoodies `FormsLabel` →
  prefer text) — the escape hatch for apps with garbage default metadata.
- **Four-phase search:** (1) global single segment; (2) **nearest-stable-ancestor anchored `>>`
  chain** — the workhorse, walk ancestors bottom-up, prefer `Anchor >> Target` (survives wrapper
  insertion), never jump straight to index; (3) **duplicate-container pinning**
  (`Container:has(Label[text='…']) >> target` / capture `*Container >> Label[text]`); (4)
  structural/geometry fallback, always emitted, always flagged brittle.
- **Scoring:** `0.45·stability + 0.25·readability + 0.20·brevity + 0.10·anchor-locality`;
  chain stability = MIN over segments, −0.05 per `>` hop through an anonymous intermediate (`>>`
  takes no penalty — why it is preferred). Return the **top-3** with `{locator, score, stability,
  brittle_flags, preconditions}` — one contract for GUI dropdown, LLM risk selection, self-heal.
- **Correctness traps:** table/list/tree cells are stamped renderers → suggest a data-locator
  keyword, never a component locator; hidden tab/card children → append `:showing` + a
  precondition; volatile containers (child_count changes between two snapshots) → never emit
  nth/index inside them.

## Validation evidence (round-trip identity, `evidence/roundtrip_validation_experiment.py`)

Live Swing test app, 137 nodes, depth 11; each generated locator resolved through the **production
`find_elements`** and checked for exactly-one-match with `element.hash_code == node.id`.

| Mode | Working unique locator | Winning tiers |
|------|------------------------|---------------|
| Full | 106/136 (77.9%) | single `Type[name\|text]` 100% (87/87); anchored `>>` +19 |
| Names stripped (off-the-shelf sim) | 73/136 (53.7%) | single `Type[text]` 100% (63/63); anchored `>>` +10 |

Read: **single-segment name/text is airtight; anchoring genuinely resolves deep no-name nodes; all
uncovered nodes are anonymous structural containers, not actionable widgets.** The shortfall is two
**prototype-only** divergences the production design removes:
1. the prototype computes `nth-of-type` itself (drift → live-verify rejects it) — the real
   generator calls the matcher's `get_type_index` (parity, no drift);
2. the prototype's geometry candidate uses tree coordinates in a different space than the
   matcher's `[x][y]` (live-verify rejects it) — the real generator reads matcher-space geometry.
Both fail *at live verification*, which is the empirical justification for the design's rule:
**generate in the same Rust engine RF executes; verify every candidate live before emitting.**

## Decisions
- **D1 Parity by construction** — generation + verification in the production Rust matcher; never a
  JS/Java reimplementation. (The experiment shows divergence = exactly where it breaks.)
- **D2 CLI-first, agent-change-free MVP** — Phases 0–2 (Rust generator + agentic CLI + harness)
  ship a complete, validated locator-authoring workflow for AI agents with no agent/UI risk.
- **D3 Single-wheel distribution** — a `javagui-spy` console script + one self-contained HTML page
  over stdlib HTTP+SSE bound to 127.0.0.1. No Electron/Tauri/npm (contradicts the abi3 wheel).
- **D4 Read-only by default** — a spy, not a recorder; `[Try Click]` behind an explicit toggle.

## Risks (see report §7)
Uniqueness ≠ stability (heuristic scoring + brittle flags + perturbation KPIs, never claim
permanence); transport fragility (architectural mitigations above); in-JVM overlay/armPick
footprint (hollow tagged-filtered overlays, `syncExec` timeouts, opt-in); SWT attribute-surface
unknowns (audit `swt_matcher.rs` before enabling accessiblename/tooltip rungs); agent-id lifetime
(session handles only + structural-identity fallback).
