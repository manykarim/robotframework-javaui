# Tasks — phased roadmap

Each phase ships standalone value; Phases 0–2 alone give AI agents a complete, validated
locator-authoring workflow. This is a research + design change; implementation follows on approval.

## 0. Research + validation (DONE — this change)
- [x] 0.1 Fable deep-research (prior art + 6 dimensions + synthesis) → `evidence/fable-research-report.md`
- [x] 0.2 Ground against the codebase (locator grammar, tree/scan RPCs, connection model, gaps)
- [x] 0.3 Round-trip validation experiment vs live Swing app → `evidence/roundtrip_validation_experiment.py`
      (single-segment name/text 100%; anchored `>>` resolves deep no-name nodes; shortfall = the two
      parity divergences the design removes)

## 1. Generator core — the crux
> MVP DECISION: implement the generator in **Python** first, using the production `find_elements`
> as the live uniqueness oracle (this IS parity — same Rust matcher — proven at 100% for
> single-segment + anchored in Phase 0). The Rust `src/locator/generator.rs` (offline batch, no
> RPC/candidate) is now implemented as the preferred fast path (task 1.5), with the Python live
> oracle as fallback — a **performance** win, verified at parity with the live oracle.
- [x] 1.1 `python/JavaGui/spy/generator.py` — tiered ladder + four-phase search over a fetched tree
- [x] 1.2 Uniqueness oracle via `resolve(locator)->ids` (count==1 AND id==target); top candidate re-verified
- [x] 1.3 Attribute ladder + four-phase (single → anchored `>>` → container pinning → flagged fallback)
- [x] 1.4 Top-3 ranked candidate contract `{locator,strategy,match_count,unique,stability,brittle_flags,preconditions}`
- [x] 1.5 Rust `src/locator/generator.rs` + PyO3 `suggest_locators` — OFFLINE candidate generation (no per-candidate RPC; reuses `find_matching_components` + `json_to_component` as the oracle). `SpyCore.suggest` prefers it with a live Python fallback; parity + live-unique resolution verified (`test_rust_offline_suggest_parity`)

## 2. Agentic CLI (MVP, zero agent changes)
- [x] 2.1 `python/JavaGui/spy/{core,cli}.py` — SpyCore (one serialized connection, tree cache) + verbs
- [x] 2.2 `dump-tree`/`find`/`validate`(exit codes)/`suggest`/`describe`/`screenshot`/`schema`, all `--json`
- [x] 2.3 Screenshot-click picking via client-side hit-test; `[project.scripts] javagui-spy`; maturin include for static
- [x] 2.4 Machine-correctable parse errors (position + hint)

## 3. Validation harness + CI gates (overlaps 2)
- [x] 3.1 `tests/spy/` round-trip harness on swing/swt/showcase/DBeaver matrix; `spy_validation_metrics.json`
- [x] 3.2 Expose agent node id on `_SwingElement` (powers `validate --expect-id`)
- [x] 3.3 Dedicated spy CI tier: `Run spy tests (Linux, xvfb)` step drives the Swing app via `tests/spy/` on every PR; the `real-rcp-dbeaver` job gates SWT/RCP against a live DBeaver (incl. `SPY_PROBE=1` hitTest/highlight/armPick probe). (Deferred enhancement: permanent failed-locator corpus + perturbation suite.)

## 4. Agent RPCs v1.1 (small, public-API-only)
- [x] 4.1 `hitTest(x,y)` (ancestor path), `highlight` (hollow tagged-filtered overlay), `getUiGeneration` — Swing first, then SWT (`syncExec` + timeouts), RCP part-id enrichment
- [x] 4.2 CLI `pick --at` upgraded to in-JVM ground truth

## 5. Web GUI + full
- [x] 5.1 Single self-contained HTML page over SpyCore HTTP/SSE (tree / screenshot / inspector), live locator bar with match-count badge, breadcrumbs, Copy-as-RF, Flash-matches, RF-resource export
- [x] 5.2 `armPick` (Ctrl+Shift in-app pick) — Swing (AWTEventListener) AND full SWT via reflection (`Display.addFilter` Proxy Listener on the UI thread, off-thread wait, filter removed in finally; live timeout-clean on DBeaver) — MCP server façade, per-app recognition-rules config, data-locator suggestions for tables/trees
