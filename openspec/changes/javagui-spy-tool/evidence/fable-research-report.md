# Java-UI Spy — Design Recommendation

**Codename:** `javagui-spy` · **Scope:** Swing / SWT / Eclipse RCP · **Surfaces:** local web GUI + agentic CLI (+ optional MCP façade)

---

## 1. Executive Summary & Product Vision

Build the Spy as an **external client of the existing JSON-RPC-over-TCP agent** (the Appium Inspector architecture), shipped **inside the existing `robotframework-javagui` wheel** as a `javagui-spy` console entry point. It is three thin layers over one shared core: a new Rust **locator generator** (`src/locator/generator.rs`) that reuses the production parser/matcher as its uniqueness oracle; a Python **SpyCore** that owns one serialized agent connection, a tree cache, and screenshots; and two presentation surfaces — a zero-build, self-contained local web page (screenshot + bounds-overlay picker, Chrome-DevTools-style breadcrumbs and live match counts) and a stateless JSON CLI modeled on Playwright MCP's ref-tagged snapshot pattern. The hard problem — unique, stable, readable locators for deeply nested, nameless widgets — is solved by a **tiered generation algorithm with ancestor anchoring** (Playwright codegen's refinement loop ported onto this library's `>`/`>>` grammar), verified against the live tree before anything is ever shown, and validated continuously by a **round-trip identity harness** on real apps (JGoodies showcase, DBeaver).

**Product vision (one paragraph):** A user or an AI agent points `javagui-spy` at any running instrumented Java app — Swing, SWT, or RCP, local or headless — clicks a widget on a live screenshot (or asks for a compact ref-tagged tree snapshot), and receives a ranked, *pre-verified* list of locators, each badged "1 of 1" with a stability score and copyable as a ready-to-paste Robot Framework keyword line; the same five read-only CLI verbs give an LLM everything it needs to author a working test in a few hundred tokens, with zero extra installation because the spy, the agent jar, and the exact locator engine RF will execute all ship in the one wheel the user already has.

**Core trust guarantee — parity by construction:** every locator the spy emits was parsed and matched by the *same* Rust engine (`parse_locator` + `find_matching_components` / `find_cascaded_with_capture`) that Robot Framework runs. No reimplementation in JS or Java, ever. A locator that passes in the spy cannot fail to parse in RF.

---

## 2. Architecture

### 2.1 Topology (CDP-bridge model)

```
                       ┌───────────── target JVM ─────────────┐
Browser GUI ──HTTP/SSE─┐                                       │
                       ├─ SpyCore (Python daemon)              │
CLI / LLM ────verbs────┤   · ONE serialized TCP connection ──► agent (existing JSON-RPC 2.0)
                       │   · tree cache (agent node ids)       │  + 3 small new RPCs:
MCP façade ───tools────┘   · PyO3 Rust matcher + generator     │    hitTest, highlight, armPick
                                                               └───────────────────────────────┘
```

- **Attach:** two paths, both zero-instrumentation for already-agented apps. `javagui-spy connect host:port` opens the same TCP RPC that RF uses — the agent's `RpcServer` already supports multiple concurrent clients (cachedThreadPool), so the spy attaches *alongside* a running RF session. `javagui-spy launch --jar app.jar [--toolkit swt]` self-injects the **wheel-bundled** `javagui-agent.jar` (`-javaagent:...=port=...,toolkit=...`); RCP is documented as `--toolkit swt` forced (per the real-Eclipse findings).
- **One spy, three toolkits:** because all UI data comes over the wire in a uniform node shape (id, type, name, text, bounds, screenX/Y, visible, enabled, child_count), there is no per-toolkit UI code — the anti-pattern SWTBot Spy/SwingExplorer fell into.

### 2.2 Scan

- **Per-window, cached, lazy.** Fetch `listWindows`/`listShells` first, then per-window trees; huge subtrees expand lazily via the existing `getComponentTree(componentId, maxDepth)` overload. Never repeatedly pull the full tree — full fetches build JSON synchronously on the EDT/UI-thread and are exactly what triggers the documented "Broken pipe (os error 32)" flakiness.
- **Cache keyed by agent ids.** `ComponentInspector.getOrCreateId` gives session-stable ids (reverseCache), making bridge-side caching, diffing, and subtree refresh *correct*. Ids are session handles only — never persisted into generated locators.
- **Change detection is pull-based:** a cheap new `getUiGeneration` RPC (hash of window count + component count + focused id) polled at ~300 ms; refetch only windows that changed. True event push (AWTEventListener / `Display.addFilter` streaming JSON-RPC notifications) is protocol-compatible and deferred to a later phase.
- **Connection discipline:** SpyCore serializes all agent I/O behind one queue (CLI + GUI + verify traffic share it), retries once on broken pipe, and reports transport failures separately.

### 2.3 Pick (three modes, all needed — prior-art consensus)

1. **Screenshot click (default, MVP, headless-first).** `captureScreenshot(-1)` + cached tree bounds → client-side deepest-node hit-test. Zero agent changes; this is also what LLM agents consume (Appium Inspector model).
2. **`hitTest(x, y)` agent RPC (v1.1).** In-JVM ground truth: Swing = EDT + windows-topmost-first + `SwingUtilities.getDeepestComponentAt`; SWT = `syncExec` + shell recursion + `Table/Tree.getItem(Point)` item special-casing; RCP enriches the result with the workbench part id (best stable scope anchor in Eclipse apps). Returns the **full root→leaf ancestor id path** — the raw material for both the breadcrumb UX and anchored-chain generation. Client geometry hit-testing is wrong in exactly the hard cases (glass panes, JLayeredPane, z-order) — the same lesson as the already-fixed click-retargeting bug.
3. **`armPick` in-app pick (v2).** One-shot `Toolkit.addAWTEventListener` / `Display.addFilter` that captures-and-consumes the next **Ctrl+Shift+click** (SWTBot's freeze gesture — a plain click would activate the widget). Strictly opt-in, time-boxed, documented as incompatible with a concurrently running test.

### 2.4 Highlight

- **In-JVM overlay** (`highlight(id, durationMs, color)`): non-focusable, always-on-top `JWindow` with `setShape` / SWT `Shell` with `setRegion`, cut to a **hollow border rectangle** so the interior stays clickable; auto-dispose; tagged with a reserved name (`__javagui_spy_overlay__`) and **filtered out of every inspection RPC** so the spy never corrupts its own scans.
- **Screenshot annotation** is a first-class *equal* mode (not a fallback): bridge-drawn rectangles from cached bounds, screenshot and bounds fetched back-to-back and stamped with the generation counter. If SWT overlay proves flaky on a platform, the UX says so and flashes on the screenshot instead — never a silent no-op.

---

## 3. The Locator-Generation Algorithm (the crux)

**Placement:** `src/locator/generator.rs` (~500 LOC) beside `matcher.rs`, sharing `parse()`, `find_matching_components()`, `find_cascaded_with_capture()`, `get_type_index()`, `get_attribute_value()`. Exposed via PyO3 as `suggest_locators(tree_json, node_id, toolkit) -> Vec<Candidate>`. The grammar already supports everything the generator emits (attribute operators, `:nth-of-type`, `:has()`, `:showing`, `:contains()`, `>`/`>>`/descendant, `*` capture) — **no new syntax**.

**Uniqueness oracle (hard gate):** `unique(cand) := matches(parse(cand), snapshot).len() == 1 && matches[0].id == target.id`. Matching the right *count* but the wrong *node* is a fail. `>>`-with-capture candidates must run through `find_cascaded_with_capture` (uniqueness is defined on the post-capture-filter set). All candidates for a target are batch-evaluated in one in-memory tree pass over the cached snapshot (zero RPCs per candidate); the **winner is re-verified once against a fresh tree** before being handed out.

### Attribute-priority ladder (per-node qualifier, best rung first)

| Rung | Qualifier | Stability weight |
|---|---|---|
| R1 | `#name` / `Type[name='v']` | 1.00 |
| R2 | `Type[accessiblename='v']` (off-the-shelf apps set these far more often than `name`) | 0.90 |
| R3 | `Type[text='v']` — normalized: strip `&` mnemonics, collapse whitespace; HTML/long text → `[text^='…']` or `:contains()` | 0.75 (static) / 0.45 (dynamic-looking) |
| R4 | `Type[tooltip='v']` | 0.65 |
| R5 | `Type:nth-of-type(k)` — computed by the **same** `get_type_index` code as the matcher (no off-by-one drift) | 0.40 |
| R6 | `Type[index='k']` raw sibling index | 0.25 |
| R7 | Geometry — `[width][height]` before `[x][y]` (size survives window moves) | 0.20 / 0.10 |

The ladder is **per-toolkit data, not code** (SWT's attribute surface differs and lacks AccessibleContext — audit `swt_matcher.rs` before enabling R2/R4 there), and is extended by a **per-app recognition-rules config** (Ranorex weights / Marathon NamingStrategy): e.g. "JGoodies `FormsLabel` → prefer text", "vendor row class → prefer client-property". This escape hatch is what makes the tool work on apps with garbage default metadata.

### Four-phase search

1. **Global single segment.** Try R1–R4 on the target alone; collect every unique candidate. If any, rank and return.
2. **Nearest-stable-ancestor anchored chain** (the workhorse — Playwright refinement + Squish container-relative names). Walk ancestors bottom-up; take the first ~3 with an R1–R4 qualifier whose global match count is 1 (ideally, ≤3 acceptable). For each anchor × target rung (now including R5), try in order: `Anchor >> Target` (**preferred — survives wrapper-panel insertion**), `Anchor Target` (descendant), and `Anchor > hops > Target` only when depth gap ≤ 3. Nearest anchor wins; stop climbing once candidates exist. **Never jump from ambiguity straight to index.**
3. **Duplicate-container pinning** for identical rows/cards/tiles: find the nearest ancestor container whose subtree holds a distinguishing R1–R3 descendant and pin it — `Container:has(Label[text='…']) >> target`, or the capture form `*Container >> Label[text='…']` when the container itself is the target (proven on the JGoodies showcase). Cap `:has()` search to containers within k ancestors (it's a subtree scan; unbounded it goes O(N²)).
4. **Structural/geometry fallback — always emitted, always flagged brittle:** `best_anchor >> Type:nth-of-type(k)`, then `[index]`, then geometry last. Rendered in red with "resizes/relayouts will break this".

### Scoring & output

Uniqueness gates; then `score = 0.45·stability + 0.25·readability + 0.20·brevity + 0.10·anchor-locality`. **Stability of a chain = MIN over segments** (weakest link), minus 0.05 per `>` hop through an anonymous intermediate (`>>` takes no penalty — exactly why it's preferred). Deterministic tie-break: stability, then string length. **Return the top-3 ranked candidates** with `{locator, score, stability, brittle_flags, preconditions}` — this one choice serves the GUI dropdown, LLM risk-tolerance selection, and Selenium-IDE-style fallback storage / future self-healing without rework.

### Special cases (correctness traps)

- **JTable/JList/JTree cells are stamped renderers, not components.** Detect these types and emit a **data-locator keyword suggestion** instead (`Select Table Cell    JTable[name='data']    row=3    col=Name`, tree path `Project Root/src`) — never a component locator.
- **Tabs/CardLayout:** hidden cards keep their children in the tree; when duplicates differ only by showing-state, append `:showing` and record a precondition (`"requires tab Settings active"`) in candidate metadata.
- **Volatile containers:** take two snapshots moments apart; containers whose child_count changed are marked volatile — never emit nth/index inside them; emit content-based `[text*='…']` or a parameterized template instead.
- **Dynamic text heuristic:** text containing counters, dates, durations, or differing between snapshots is downgraded to 0.45, not banned — a downgraded-but-unique text locator still beats an index.

---

## 4. UX

### 4.1 GUI (local web page, `javagui-spy ui --port 8123`)

Layout proven by Appium Inspector / uiautomatorviewer, affordances from Chrome DevTools + Playwright Inspector + SelectorsHub:

- **Left — virtualized tree** (lazy render, 10k-node capable) with substring search across type/name/text that auto-expands ancestors of hits; filters = the existing `Get UI Tree` filters (types, visible_only, max_depth).
- **Center — screenshot** with hover-tracking rectangle overlays (from screenX/screenY bounds); click = pick. Explicit **Refresh** button + tree timestamp (never auto-poll).
- **Right — inspector:** full property table, **clickable ancestor breadcrumb**, and the ranked candidate list — each candidate with match-count badge, stability score, brittle flag, [Copy], and [Flash matches] (highlight RPC on every match).
- **Top — live locator bar** (the single most important affordance, per SelectorsHub/Playwright): free-text editable, 250 ms debounce, re-verified on every edit through the Rust matcher; badge **green = 1 of 1, amber = 1 of N, red = 0 / parse error with position + hint**. This converts the deep-non-unique-tree problem into an interactive narrowing game: add one attribute or one ancestor level, watch the count drop to 1.
- **[Copy as RF ▾]** dropdown emits ready keyword lines (`Click    JButton[name='ok']`, Input Text, Wait Until Element Is Visible…). **[Try Click]** exists but is disabled behind an explicit "live actions" toggle — the deliberate boundary against recorder scope creep.
- **Selection sync contract:** selecting a node *anywhere* (tree, screenshot, pick, candidate hover) updates all panes at once.
- Optional **export to RF resource file** (`${LOGIN_BUTTON}=    JButton[text='Login']`) — Squish/Marathon's alias layer as an opt-in export, never Jubula's mandatory mapping database.

### 4.2 Agentic CLI (Playwright-MCP pattern)

Stateless, idempotent, read-only verbs; `--json` always; uniform envelope; **exit codes as control flow** (`validate`: 0 = unique, 3 = zero matches, 4 = ambiguous, 2 = parse error). Verbs:

`connect` / `launch` · `dump-tree` (JSONL, one node/line, pre-order, `--visible-only` default, field allowlist `id,type,name,text,bounds,depth` ≈ 20 tokens/node) · `find LOC` · `validate LOC` · `suggest --node-id N | --at X,Y` · `pick --at X,Y` (later `--arm`) · `describe --node-id N` (props + ancestor chain, each ancestor annotated with its own best discriminator) · `screenshot [--annotate LOC] -o out.png` · `schema` (JSON Schema of every verb + a locator-grammar cheatsheet with 6 canonical examples — the agent bootstrap).

Every node carries **both** `node_id` (ephemeral session ref for command chaining — the Playwright-MCP/CDP-MCP uid pattern) and the durable artifact, a resolved tier-1..4 locator. Refs are validated before use; stale → explicit `NODE_GONE`, "re-snapshot".

**Example — `suggest` I/O:**

```json
$ javagui-spy suggest --node-id 312 --json
{"ok": true, "command": "suggest",
 "data": {
   "target": {"node_id": 312, "path": "0/2/1/3", "type": "JButton", "name": null,
              "text": "Save", "bounds": {"x": 232, "y": 38, "w": 80, "h": 24}, "depth": 7},
   "candidates": [
     {"locator": "ToolBar[name='mainToolbar'] >> JButton[text='Save']",
      "strategy": "anchored", "match_count": 1, "unique": true,
      "stability": 0.75, "brittle_flags": [], "preconditions": []},
     {"locator": "JButton[text='Save']", "strategy": "text",
      "match_count": 3, "unique": false, "stability": 0.75,
      "brittle_flags": ["ambiguous"], "preconditions": []},
     {"locator": "ToolBar[name='mainToolbar'] >> JButton:nth-of-type(2)",
      "strategy": "index-chain", "match_count": 1, "unique": true,
      "stability": 0.40, "brittle_flags": ["sibling-index"], "preconditions": []}],
   "rf_snippets": {"click": "Click    ToolBar[name='mainToolbar'] >> JButton[text='Save']"}},
 "meta": {"toolkit": "swing", "agent_version": "0.7.0", "tree_timestamp": 1752900000000}}
```

**Canonical five-call agent workflow** (documented verbatim in `--help` and `schema`): ① `dump-tree --visible-only` to orient → ② `find "text:Save"` to shortlist → ③ `suggest --node-id 312` → ④ `validate` the top candidate (exit 0 = done) → ⑤ `screenshot --annotate LOC -o proof.png` for visual confirmation. The **MCP server façade** exposes these same verbs 1:1 as tools — a fourth client of the same SpyCore, no new logic.

Error shape LLMs self-correct on: `{"code":"PARSE_ERROR","position":17,"snippet":"JButton[naem='ok']","hint":"unknown attribute 'naem'; valid: name,text,tooltip,accessiblename,x,y,width,height,..."}`.

---

## 5. Tech Stack, Rationale, Packaging

| Layer | Choice | Rationale |
|---|---|---|
| Locator generation + verification | **Rust**, `src/locator/generator.rs`, shared with matcher via PyO3 | Parity by construction; offline batch uniqueness testing (zero RPC per candidate); sub-second whole-tree generation at DBeaver scale |
| Bridge/daemon | **Python** (`python/JavaGui/spy/`: `core.py`, `cli.py`, `server.py`, `static/spy.html` — each < 500 lines per project rule) | Reuses existing connection code + PyO3 `send_rpc_request`/`find_elements`/screenshot; Python is forced anyway by the parity requirement |
| GUI | **One self-contained HTML file**, vanilla JS (or Preact-via-htm), served by **stdlib `ThreadingHTTPServer` + SSE**, bound to **127.0.0.1 only** (the agent RPC has no auth) | No Electron/Tauri/npm — contradicts the single-wheel distribution; works over SSH port-forward (matches the project's headless/xvfb reality). `starlette+uvicorn` behind an optional `[spy]` extra only if ergonomics demand it |
| CLI | argparse subcommands, same dicts as HTTP bodies | One SpyCore contract → CLI, GUI, and MCP cannot diverge; tested once |
| Agent additions | ~50–100 LOC/toolkit each: `hitTest`, `highlight`, `armPick`, `getUiGeneration` — **public AWT/SWT APIs only, no bytecode changes**; all SWT via `syncExec` with bridge-side timeouts | The class of threading bug already fixed twice in `EclipseWorkbenchHelper` must not recur |

**Rejected:** Electron/Tauri (per-platform binaries vs. the existing abi3 wheel), TUI (can't do visual picking; Textual breaks the ≥3.8.1 floor), any JS-side locator parsing (parity break — the worst possible trust failure).

**Packaging:** add `[project.scripts] javagui-spy = "JavaGui.spy.cli:main"`; extend maturin `include` to `["JavaGui/jars/*.jar", "JavaGui/spy/static/*"]`. The wheel already bundles the 450 KB agent jar, so `pip install robotframework-javagui` is the *entire* install story — the `rfbrowser init` precedent shows the RF ecosystem accepts console scripts in the library wheel.

---

## 6. Validation Plan & Success Criteria

**Core protocol — round-trip identity** (`tests/spy/validation/roundtrip_harness.py`, pytest-parametrized): fetch tree once → for every sampled node, `generate_locator(node)` → resolve through the **production path** (`find_elements` → Rust matcher; *never* agent `findWidgets`, which can't exercise combinators/geometry/index) → PASS iff exactly 1 match AND `match.id == node.id`. Requires exposing the agent node id on `_SwingElement` (also powers `spy validate --expect-id N`).

**App matrix (all in-repo):** `tests/apps/swing` + `tests/apps/swt` (controlled, named — expect ~100%); JGoodies showcase jar (depth 14+, name=null, custom classes — *the* hard case); DBeaver Docker harness (real RCP; stratified ~500-node sample by depth bucket × named/unnamed × duplicated-type — ±1.9 pp CI at 95%).

**Gates:**
- Controlled apps: ≥ 99% round-trip identity.
- Showcase + DBeaver: ≥ 95% overall; ≥ 90% for name-null nodes at depth ≥ 10; **100% of visible actionable widgets get *some* working unique locator** via anchored/indexed fallback.
- **Geometry fallback share < 10%** (the headline quality KPI — target < 5% on the showcase).
- Median locator ≤ 60 chars, ≤ 3 chain segments; determinism ≥ 99.9% (generate twice, string-equal); generation < 100 ms/node p95 on cached tree.

**Phase 2 — perturbation robustness:** baseline-resolve all locators, apply one perturbation (resize ±20% via xdotool, tab away/back, add 5 rows, dialog reopen, showcase nav away/back), re-resolve. Component-recreating perturbations score by **structural identity** (className + text + path + bounds tolerance), since agent ids are reborn. Gate: ≥ 90% of non-geometry locators survive resize/tab/rows; geometry locators are *expected* to break on resize — a separate reporting bucket, with high geometry *share* treated as the real defect.

**Harness hygiene (hard-won project lessons):** execution-based metrics only (the 96.2% string-presence coverage trap); retry transport errors once and account `transport_failures` outside the denominators (broken-pipe pollution); launch app + harness in one command under xvfb; CI self-skip must be *loud* when the showcase jar/Docker is absent. **Regression corpus:** every locator that ever fails round-trip is stored and replayed forever. CI tiers: PR = swing+swt+showcase under xvfb; nightly = DBeaver harness gating on `spy_validation_metrics.json`, archiving failing locators + widget screenshots. Publish the first DBeaver baseline and *ratchet* — don't tune gates in the dark.

---

## 7. Risks & Unknowns

1. **Uniqueness ≠ stability.** Verified against the current tree only; text tiers are i18n/skin-fragile, geometry is DPI/resize-fragile. Mitigation: honest scoring labeled heuristic, brittle flags, perturbation KPIs, top-3 fallback lists. Never claim permanence.
2. **Transport fragility.** The spy could amplify the known broken-pipe flakiness. Mitigation is architectural (serialized single connection, per-window/lazy fetches, generation-counter polling, debounced verify, retry-once) — non-negotiable, not optional polish.
3. **In-JVM footprint.** Overlay windows and armPick listeners run inside the customer's app; SWT `syncExec` can hang on modal loops. Mitigation: hollow non-focusable tagged-and-filtered overlays, bridge-side timeouts, armPick strictly opt-in, screenshot-annotation as an equal mode. An armed pick could swallow an RF-synthesized click — document as incompatible with concurrent test runs.
4. **SWT/RCP attribute surface unknowns.** `swt_matcher.rs` is a separate implementation; SWT lacks AccessibleContext, and SWT has no public deep hit-test (per-widget `getItem(Point)` polish needed). **Unknown until audited** — do the ladder audit before enabling R2/R4 for SWT.
5. **Id lifetime + agent memory.** Ids die on component recreation; the strong-ref `componentCache` grows during long sessions. Mitigation: ids as session handles only, structural-identity fallback, a prune-on-detach path. **Probe needed:** confirm cache reference semantics against the showcase (10-line script).
6. **Generator/matcher drift** on `:nth-of-type` and `>>` capture semantics. Mitigation: shared functions only + round-trip property test explicitly covering capture-form candidates.
7. **Coordinate drift** (HiDPI, multi-monitor, SWT vs AWT spaces). Mitigation: per-window screenshots keyed to window origin, generation-stamped bounds+pixels, `hitTest` as ground truth over client geometry; test HiDPI early.
8. **Scope creep toward a recorder.** A recorder is a different product. The [Copy as RF] dropdown is the boundary; if recording ever comes, follow Abbot's SemanticRecorder design (semantic actions) and harvest locators into the resource-file export (Squish).
9. **DBeaver gate thresholds are a guess** until the first baseline run — publish and ratchet.

---

## 8. Phased Roadmap

**Phase 0 — Generator core (Rust) — ~1–2 wks.** `generator.rs` + PyO3 export; per-toolkit ladder tables; round-trip property tests against *recorded* tree fixtures (showcase, DBeaver, test apps). Ships the crux with zero agent/UI risk.

**Phase 1 — Agentic CLI (MVP, zero agent changes) — ~1 wk.** SpyCore + `connect`/`launch`/`dump-tree`/`find`/`validate`/`describe`/`screenshot`/`suggest`/`schema`, all `--json`; screenshot-click picking via client-side hit-test. Immediately useful to LLM agents (this session is the persona) and testable headless under xvfb. Entry point + packaging land here.

**Phase 2 — Validation harness + CI gates — ~1 wk, overlaps P1.** Round-trip harness on the 4-app matrix, metrics JSON, PR + nightly tiers, regression corpus, first DBeaver baseline.

**Phase 3 — Agent RPCs v1.1 — ~1–2 wks.** `hitTest` (ancestor path), `highlight` (hollow overlay), `getUiGeneration`; Swing first, then SWT (budget the `getItem(Point)` polish), RCP part-id enrichment. CLI `pick --at` upgraded to in-JVM ground truth.

**Phase 4 — Web GUI — ~2 wks.** Single-file page over SpyCore's HTTP/SSE: tree / screenshot / inspector panes, live locator bar with match-count badge, breadcrumbs, Copy-as-RF, Flash-matches, resource-file export. The GUI API *is* the CLI verb schemas.

**Phase 5 — Full — as demanded.** `armPick` Ctrl+Shift in-app picking; MCP server façade; per-app recognition-rules config file; data-locator suggestions for tables/trees; perturbation suite in nightly CI; event-push refresh; (only if pulled by users) semantic recording per Abbot.

Each phase ships standalone value; Phases 0–2 alone already give AI agents a complete, validated locator-authoring workflow.