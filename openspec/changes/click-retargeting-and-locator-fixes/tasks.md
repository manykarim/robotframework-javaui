## 1. Click retargeting (agent)

- [ ] 1.1 In `agent/.../swing/ActionExecutor.java` `performMouseClick(Component, int)`: if the target has no `MouseListener`s, walk `getParent()` up to the nearest ancestor with mouse listeners (stop at `Window`); translate the point with `SwingUtilities.convertPoint` and dispatch press/release/click to that ancestor. Fall back to the target when no such ancestor exists.
- [ ] 1.2 Confirm `Double Click` / `Right Click` / `Click Element` route through `performMouseClick` so they inherit retargeting (resolves Q1); adjust if any bypasses it.
- [ ] 1.3 Add an agent unit test (no showcase jar): a `JPanel` with a recording `MouseListener` containing a listener-less child; click the child; assert the panel's listener fired.
- [ ] 1.4 Rebuild the agent (`mvn -f agent/pom.xml package`).

## 2. Locator engine fixes (Rust)

- [ ] 2.1 Fix capture (`*`): apply on both `>` child chains and `>>` cascaded chains; return captured-segment elements filtered to those whose subtree matches the remaining chain (`*JPanel >> FormsLabel[text='Input']` → only cards containing that label).
- [ ] 2.2 Fix deep CSS `>` chains: `A > B > C (> …)` must match when a satisfying ancestor path exists (`JViewport > JPanel > JPanel` → non-empty).
- [ ] 2.3 Add matcher/evaluator unit tests for 2.1 and 2.2, plus regression tests that 2-level `>`, simple type, attribute, geometry, and `>>` selectors are unchanged.
- [ ] 2.4 Rebuild the extension (`maturin`); run `cargo test`.

## 3. Verify

- [ ] 3.1 Live showcase (opt-in, self-skip): `Click FormsLabel[text='Input']` navigates to the Input page; `*JPanel >> FormsLabel[text='Input']` returns only the card `JPanel`; `JViewport > JPanel > JPanel` matches.
- [ ] 3.2 Regression: full Swing suite (baseline 697 tests, 0 failed), RF dryrun, `cargo test`, `pytest` — confirm no regressions from retargeting or the locator changes.

## 4. Follow-up — extend the showcase proof to radio/table/tree

- [ ] 4.1 Update `tests/robot/showcase/resources/showcase.resource`: navigate to demo pages via `Click FormsLabel[text='<Section>']` (label click now works) instead of geometry locators.
- [ ] 4.2 Add validated-action tests for the categories the first proof couldn't reach: **radio buttons** (select → `Element Should Be Selected` + sibling not), **table** (select row/cell → verify selection via getter), **tree** (expand/select node → verify) — each with independent read-back, on the demo pages now reachable (e.g. Selection page's `PivotBar` + `JList`, and pages hosting radios/tables/trees).
- [ ] 4.3 Update `tests/robot/showcase/README.md` finding #2 to "resolved by click-retargeting" and record which categories are now proven end-to-end.
- [ ] 4.4 Run the extended showcase suite headless (Xvfb) to green; capture the run summary.
