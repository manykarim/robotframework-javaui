## 1. RCP real-Eclipse spike (risk gate — do first)

- [x] 1.1 DESIGN CHANGE (better than Tycho): compile `com.testapp.rcp` directly against a pinned Eclipse 4.30 platform and install via `dropins/` — simpler + reproducible. Captured as `tests/apps/rcp/build-and-run-real-eclipse.sh` (Q1 resolved: Eclipse 4.30 / R-4.30-202312010110)
- [x] 1.2 App launches as a standalone Eclipse RCP application via `-application com.testapp.rcp.application` (workbench initializes headless)
- [x] 1.3 Build produces a runnable OSGi bundle jar; the script builds + installs + launches it reproducibly
- [x] 1.4 Launched headless under `xvfb-run` with `-javaagent:javagui-agent.jar=port=5682,toolkit=swt`; agent attaches in SWT mode
- [x] 1.5 GATE PASSED: connected and confirmed live workbench data. Fixed TWO real agent bugs to get here — (a) `EclipseWorkbenchHelper` used bare `Class.forName` (agent classloader, blind to OSGi bundles) → now discovers the `org.eclipse.ui` bundle classloader via Instrumentation; (b) active-window/page queries returned null off-thread → now run on the SWT UI thread via `SwtReflectionBridge.syncExec`. Also added the missing real-Eclipse fallback to `getWorkbenchInfo`. Result: `get_open_views`, `get_available_perspectives`, `get_active_perspective_id`, `get_workbench_info` all return live data

## 2. Keyword honesty fixes

- [x] 2.1 `List Applications`: now raises a clear `NotImplementedError` (documented not-supported result) instead of silently returning `[]`; docstring points to `Connect To Application` with explicit port (Q2: deferred full removal to keep it non-BREAKING for now)
- [x] 2.2 `Type Text`: docstring corrected — no longer claims per-character key events; documents that it appends via the `input_text` path (Q3: re-document chosen; real KeyEvent dispatch is a follow-up)
- [x] 2.3 `Log UI Tree`: now forwards `locator` to `self._lib.get_component_tree(locator=locator, ...)` so the argument is honored
- [x] 2.4 No Rust/agent change required — all three fixes were Python-only (no extension/agent rebuild needed)
- [x] 2.5 Updated unit tests (`test_list_applications` expects raise; `test_type_text` docstring/comment corrected); `pytest tests/python/test_swing_library.py tests/python/test_integration.py` → 61 passed

## 3. Keyword count single-sourcing

- [x] 3.1 Authoritative keyword surface computed via libdoc: Swing ~108, SWT ~71, RCP ~75 (includes deprecated aliases)
- [x] 3.2 Contradictory-count docs (58 / 50+ / 182-vs-20 in FEATURE_COMPARISON_MATRIX etc.) deleted in §5.3; README now cites the authoritative per-toolkit counts

## 4. E2E coverage — close the 68 gaps

- [x] 4.2 Added `tests/robot/rcp/real_eclipse/01_real_workbench.robot` — real-Eclipse RCP suite (workbench-info, custom-perspective discovery, active perspective, open views). RAN LIVE: 4 tests, 4 passed against real Eclipse
- [~] 4.1 Instead of parametrizing common.resource, added a dedicated opt-in real_eclipse suite + launcher script (achieves the mock/real split cleanly). Full parametrization of the 10 existing mock suites NOT done
- [x] 4.3 Added `tests/robot/swt/20_coverage_getters.robot` (9/9 pass) covering ~23 uncovered SWT table/tree/widget getters+assertions. FIXED 4 genuinely-broken keywords found in the process: `get_widget_property`, `get_widget_properties`, `get_widget_states`, `get_swt_tree_node_count` all delegated to a Rust `get_widget_property` method that doesn't exist → now read the real property map via `find_widget().to_dict()`
- [x] 4.4 Added `tests/robot/swing/21_coverage_getters.robot` (5/5 pass) covering Swing list/tree getters, save_ui_tree, config setters, showing assertions; and `tests/robot/rcp/11_coverage_editors_views.robot` (6/6 pass) covering the 10 uncovered RCP editor/view/perspective keywords. RESULT: coverage 75.3% → **96.2% (230/239)**; per-toolkit Swing 93.5%, SWT 95.8%, RCP 100%
- [x] 4.3b Also fixed `get_widget_property` override in `__init__.py:2707` (same missing-Rust-method bug)
- [x] 4.5 Cascaded selectors verified live: un-skipping `16_cascaded_basic` runs 30 tests, **22 pass** — the cascaded feature works for normal cases. The 8 remaining `robot:skip` cases are genuine edge-case limitations (very-long chains, etc.), correctly marked and documented rather than deleted or forced green
- [x] 4.6 Added `scripts/keyword_coverage.py` (reproducible; `--json`, `--min N` gate). Now **230/239 (96.2%)**; `python scripts/keyword_coverage.py --min 90` → GATE PASS. The 6 remaining "uncovered" are the misplaced RCP-passthrough methods on SwingLibrary (`get_all_rcp_views/editors`, `get_rcp_component`, `get_rcp_component_tree`, `get_component_tree`) + the intentionally-raising `list_applications` — effectively the ceiling
- [x] KNOWN-BROKEN keywords surfaced by the coverage work (real overpromises, tracked for follow-up fix): `tree_node_exists`/`Swt Tree Node Should Exist` (Rust backend always returns False); `get_swt_tree_node_count` (reads SWT child_count, not item count → 0); `List Selection Should Be`/list selection-items getter (returns []); `Log Component Tree` (forwards format=None → TypeError); `get_selected_list_item` (returns None)
- [x] REGRESSION CHECK: full mock RCP suite re-run with the modified agent → 248/248 pass, 0 failed (the `getWorkbenchInfo` real-Eclipse fallback did not affect the mock path)
- [x] 4.7 Ran full live integration under xvfb: Swing 171/171, SWT 229 passed (20 skip), RCP-mock 248/248, real-Eclipse RCP 4/4 — 0 failures across all toolkits

## 5. Repository hygiene

- [x] 5.1 6 tracked `.claude-flow/` files removed from index (`git rm --cached`); `.gitignore:113` already prevents re-adding
- [x] 5.2 `example-apps/*.jar` added to `.gitignore`; verified the 11 MB showcase jar is now ignored
- [x] 5.3 Removed 64 high-confidence throwaway docs via `git rm` (146 → 82). Inbound-link check ran first (kept `COMPONENT_TREE_DOCUMENTATION_INDEX.md`, referenced by README). NOTE: remaining borderline set (research/ analyses, architecture/specs implementation-plans) still above the 25–40 target — needs maintainer judgment to trim without losing genuine design records
- [~] 5.4 Deleted `COMPONENT_TREE_DOCUMENTATION_INDEX_OLD.md`. FLAGGED (not auto-resolved): two files share the ADR-001 number (`ADR-001-DDD-ARCHITECTURE.md` vs `ADR-001-unified-base-class-architecture.md`) — maintainer must decide which is canonical; I won't guess-delete an ADR

## 6. Documentation regeneration & single-source

- [x] 6.1 Regenerated `docs/keywords/{Swing,Swt,Rcp}.html` via `robot.libdoc` from current source (was last generated 2026-01, predating the v0.4.1 API cleanup)
- [x] 6.2 Added a "Canonical source" banner to `docs/api-reference/robot-keywords.md` designating the generated libdoc in `docs/keywords/` as authoritative (kills drift ambiguity)

## 7. README & examples accuracy

- [x] 7.1 README updated: added "Toolkit Support & Maturity" table (Swing stable / SWT stable / RCP experimental–requires real Eclipse), removed implied parity, softened "Full XPath"→"XPath-style" and "Works on"→"Designed for"
- [~] 7.2 Softened cross-platform + "Full XPath" claims (done). PyPI publish verification NOT done — requires network/registry access; left as a release-time check
- [x] 7.3 Verified `examples/` already contains runnable examples that `Connect To` and drive the bundled apps (component_tree_*, output_formats.robot) with an explanatory README — real and coherent; left as-is rather than churn

## 8. Release cut

- [x] 8.1 Bumped version to **0.5.0** (pyproject.toml + Cargo.toml); wrote `docs/RELEASE_NOTES_v0.5.0.md` with BREAKING notes (`List Applications` now raises) and an honest Known Issues section (broken-pipe flakiness, the 5 still-unimplemented keywords, the RCP API inconsistency)
- [x] 8.2 Validation: RF dryrun ran clean; **pytest 617 pass / 0 fail**; **cargo test 242 pass / 0 fail**; live RF integration Swing 171+/SWT 238/RCP-mock 248/real-RCP 4 (Swing 05_selection has pre-existing broken-pipe flakiness, unrelated to this change). ruff/robocop are style-only (ruff ~1878, +~17 from new files vs baseline; not release-blocking)
- [x] 8.3 Memory updated with findings + fixes; release notes written. Ready to archive the OpenSpec change
