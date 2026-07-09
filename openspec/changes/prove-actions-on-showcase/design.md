## Context

The Smart Client Showcase is a JGoodies demo app (Plastic look-and-feel) with a three-panel `SplitView`: a left navigation panel of `NavigationToggleButton`s, a middle sub-nav, and a content `PageFrame`/`PivotFrame`. A prior exploration (the v2 Swing report) documented version 22.04; the bundled jar is **24.09.0**, so exact component names/locators must be **re-confirmed at apply time** rather than assumed. Known behaviors from that exploration that shape the design:

- The library connects to the agent on a specific port; the agent's default differs from older docs — confirm the actual port at apply (`Connect To Application port=...`).
- The left nav panel opens with a ~2s animation; toggle buttons are `showing=false` until the panel is expanded.
- The UI tree is cached — `Refresh UI Tree` is required after navigation to see new content.
- Custom components (`NavigationToggleButton`, `ReadOnlyTextField`, `JGSearchField`, `PageFrame`, `PivotFrame`) are discoverable by type; standard widgets (`JTextField`, `JRadioButton`, `JCheckBox`, `JComboBox`, `JTable`, `JList`, `JTree`, `JButton`) appear on the demo pages (Input, Forms, Selection, Components, Validation, Basics, Master-Details).

## Goals / Non-Goals

**Goals:**
- Prove real interaction with a third-party app: every action validated by reading state back.
- Cover all action categories (text, navigation, radio, checkbox, combo/list, table/tree, button-effect).
- Keep it opt-in: self-skip when the (gitignored) jar is absent so default CI is unaffected.

**Non-Goals:**
- No library/agent/Rust code changes (findings raised separately if a real defect surfaces).
- Not exhaustive coverage of every showcase page — one solid validated test per category is enough to constitute proof.
- Not committing the third-party jar.

## Decisions

### D1: Discovery-first authoring, then assert-after-every-action
At apply time, first launch the app headless (Xvfb) and dump the UI tree per target page (`Get UI Tree` / `Save UI Tree`) to capture the **real** 24.09.0 locators, then write tests. Every state-changing keyword is immediately followed by a read-back assertion.
- **Why:** the app layout must be ground-truthed; guessed locators would produce brittle or false tests. Assert-after-every-action is the whole point of the proof.
- **Alternative:** author purely from the v2 report (rejected — it's an older version and may drift).

### D2: A validated-action helper vocabulary
Provide resource keywords that bundle action+assertion, e.g. `Enter And Verify Text`, `Navigate And Verify Section`, `Select Radio And Verify`, `Toggle Checkbox And Verify`, `Select Combo And Verify`, so each test reads as a sequence of self-validating steps and the assertion can never be forgotten.
- **Why:** encodes the contract structurally; keeps tests readable and consistent.

### D3: Robust navigation + connection handling
The showcase resource wraps: launch with agent → connect on the confirmed port → expand the left nav panel (with animation wait) → click the target `NavigationToggleButton` → `Refresh UI Tree` → verify the section is showing. Long flows guard against the known broken-pipe flakiness with an `Ensure Connected` reconnect helper.
- **Why:** matches documented app behavior (animation, tree staleness) and the known connection-stability caveat.

### D4: Opt-in self-skip
`Suite Setup` checks the jar exists (and/or the agent port becomes reachable); if not, `Skip` the suite with a clear message. Mirrors the pattern already used by `tests/robot/rcp/real_eclipse/`.
- **Why:** the jar is gitignored; default CI must not fail.

### D5: Verification uses independent reads, not the action's own return
Validation keywords read state through a *different* path than the action where possible (e.g. verify a radio via `Element Should Be Selected` / `Get Element Property selected`, not by trusting `Select Radio Button`). Navigation is verified by a page-specific element becoming showing, not just by the nav click succeeding.
- **Why:** independent confirmation is what makes this a proof rather than a smoke test.

## Risks / Trade-offs

- **[24.09.0 layout differs from the documented 22.04]** → Discovery-first (D1); dump trees before authoring; use stable type/text locators over indexes where possible.
- **[Left-nav animation / tree staleness causes flakiness]** → explicit panel-expansion wait + `Refresh UI Tree` after navigation (D3).
- **[Broken-pipe under long runs]** → `Ensure Connected` reconnect guard; keep each test focused.
- **[Some pages may lack a given widget type]** → pick the page that actually contains it during discovery; the spec requires one validated test per category, not per page.
- **[Headless rendering differences]** → assert on model state (selected/text/showing), not pixels.
- **[Jar unavailable in CI]** → opt-in self-skip (D4); document how to supply it for a dedicated job.

## Migration Plan

1. Launch showcase headless; confirm connect port; dump per-page UI trees to capture real locators.
2. Build `resources/showcase.resource` (launch/connect/navigate/ensure-connected + validated-action helpers + self-skip guard).
3. Author one validated suite per action category (or a grouped suite) using the helpers.
4. Run headless (Xvfb); iterate until green against the live app; capture a run summary as evidence.
5. Add a short README describing how to run it and what each test proves.
- **Rollback:** additive test-only; delete the suite/resource to revert. No product code touched.

## Open Questions

- **Q1:** Actual agent/connect port for the 24.09.0 jar (confirm at apply via startup logs / `ss -tlnp`).
- **Q2:** Which showcase pages host the cleanest examples of each widget (Input vs Forms vs Components) — decided during discovery.
- **Q3:** Should a dedicated opt-in CI job supply the jar (e.g. from a release URL or secret), or does this stay a local/manual proof? Default: local/manual + self-skip in CI.
