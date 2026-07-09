## 1. Click retargeting (agent)

- [x] 1.1 Added `resolveClickTarget(Component)` (walks to the nearest listener-bearing ancestor, stops at `Window`) and applied it in `performMouseClick` with `SwingUtilities.convertPoint`; falls back to the target when no such ancestor exists
- [x] 1.2 `Click` (L108) and `Double Click` (L190) route through `performMouseClick` → inherit retargeting; `rightClick` had its own dispatch → now also uses `resolveClickTarget` (Q1 resolved)
- [x] 1.3 Added `agent/src/test/java/.../ClickRetargetingTest.java` (4 tests, no showcase jar): resolve→card, resolve keeps own-listener/orphan, and full `performMouseClick` fires the card's mouseReleased/mouseClicked. **4 run, 0 failures**
- [x] 1.4 Agent compiles + tests pass; final `mvn package` done after §2

## 2. Locator engine fixes (Rust)

- [x] 2.1 Fixed capture in `find_cascaded_with_capture`: captured elements are now filtered to those whose subtree contains a final match (added `subtree_contains` helper) — `*JPanel >> FormsLabel[text='Input']` returns only cards containing that label, not every JPanel.
- [x] 2.2 Fixed `match_combinator_chain`: added an ancestor `cursor` that advances as Child/Descendant combinators are consumed, so `A > B > C` (3+ levels) matches. Root cause was `current_context` never advancing up the tree (every compound checked the target's immediate parent).
- [x] 2.3 Added 3 unit tests: `test_deep_child_chain_matches_three_levels`, `test_two_level_child_chain_still_matches` (regression), `test_capture_on_cascaded_filters_by_final_segment`. **245 passed, 0 failed** (was 242 → no regression).
- [x] 2.4 `cargo build` + `cargo test` green; extension rebuilt via `uv run` for the live checks.

## 3. Verify

- [x] 3.1 Live showcase verified (rebuilt agent + `maturin develop --release`): [A] `Click FormsLabel[text='Input']` navigates to "Input Dialogs" (ReadOnlyTextField 0→3); [B] `*JPanel >> FormsLabel[text='Input']` returns **10 of 109** JPanels (filtered, was all); [C] `JViewport > JPanel > JPanel` returns **1** (was 0). All PASS.
- [x] 3.2 Regression clean: `cargo test` 245 passed/0 failed; full Swing suite 0 failures; `pytest` 617 passed/0 failed. No regressions from retargeting or the locator changes.

## 4. Follow-up — extend the showcase proof to radio/table/tree

- [x] 4.1 Added `Open Demo Tile` to `showcase.resource` — navigates from the hub to a demo page via `Click FormsLabel[text='<Section>']` (label click now works thanks to the retargeting fix).
- [x] 4.2 Added `tests/robot/showcase/03_tile_navigation.robot` (3/3 pass): tile click navigates to the "Input Dialogs" demo page (ReadOnlyTextField appears — the hub has none); different tiles reach different pages (JList appears); and the capture/deep-`>` locator fixes resolve against the live app in an RF test. NOTE: the specific radio/table/tree *widgets* sit behind deeper pivot sub-navigation (`NavigationToggleButton` tabs with variable-suffix, non-readable labels) — the enabling navigation is now proven, but driving those individual widgets end-to-end is deferred as further showcase work (not a library limitation).
- [x] 4.3 Updated `tests/robot/showcase/README.md` finding #2 to "resolved by the click-retargeting fix" with the proven results.
- [x] 4.4 Ran headless (Xvfb): `03_tile_navigation.robot` 3/3, `02_validated_actions.robot` 5/5, `01_smoke.robot` 3/3.
