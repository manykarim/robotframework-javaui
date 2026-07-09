## Context

Root cause, confirmed by decompiling the Smart Client Showcase and reading the agent:

- **Agent click** (`agent/src/main/java/com/robotframework/swing/ActionExecutor.java`): `click()` (~line 43) uses `AbstractButton.doClick()` for buttons, else `performMouseClick(Component, int)` (~lines 1103-1136), which builds `MOUSE_PRESSED`/`MOUSE_RELEASED`/`MOUSE_CLICKED` at the component center and calls `component.dispatchEvent(...)` **directly on the target**. `force` only skips the visibility check. A static `java.awt.Robot` exists but is used only for screenshots/menu hover — never for clicks.
- **AWT reality**: a real OS click goes through `LightweightDispatcher`, which delivers to the deepest component *that has mouse listeners*, retargeting past listener-less children. Synthetic direct dispatch skips this.
- **Showcase tiles**: `com.jgoodies.fluent.tiles.AbstractTileView` registers its `MouseHandler` on the card `JPanel`; the visible `com.jgoodies…$FormsLabel` has no listener. So `Click FormsLabel[text='Input']` (synthetic → label) is a no-op; a real click (→ card) navigates.
- **Locator engine** (`src/locator/matcher.rs`): geometry attributes `x/y/width/height` are supported (~:532-535) — the current working (brittle) workaround. Capture (`*`) only participates in `>>` cascaded chains (~:866) and returns the captured segment **unfiltered**; CSS `>` chains of 3+ levels return 0 in cases that should match.

## Goals / Non-Goals

**Goals:**
- Make `Click` activate the handler a real user would trigger (ancestor retargeting), so listener-less-child patterns work.
- Fix the two locator bugs so precise ancestor targeting is a reliable alternative.
- Cover both with tests that don't require the showcase jar.

**Non-Goals:**
- A `java.awt.Robot` real-pixel-click keyword (viable alternative; unnecessary once retargeting works — may be a future option for pixel-exact cases).
- Extending the showcase proof to radio/table/tree (follow-up in `tasks.md`, gated on this landing).
- Reworking the whole locator grammar — only the capture + deep-`>` defects.

## Decisions

### D1: Retarget in `performMouseClick`, mirroring LightweightDispatcher, only when the target has no listeners
If `target.getMouseListeners().length == 0`, walk `getParent()` upward to the first ancestor with `getMouseListeners().length > 0`, stopping at (and not past) the `Window`. Dispatch the press/release/click to that ancestor, with the point translated by `SwingUtilities.convertPoint(target, center, ancestor)`. If no such ancestor exists, dispatch to the original target (today's behavior).
- **Why:** faithfully reproduces what AWT does for a real click; scoped by "target has no listeners" so buttons and normal listener-bearing components are untouched (no regression risk to the 697-test Swing suite).
- **Alternatives considered:** (a) always use `java.awt.Robot` pixel clicks — most faithful but needs the window frontmost/on-screen and is flakier headless; rejected as the default. (b) Dispatch to the label AND bubble manually to every ancestor — over-fires listeners; rejected. (c) Client-side (Rust) walk-up before sending the click — the agent has the live component graph and listener info, the Rust side does not; must be agent-side.

### D2: Fix locator capture + deep `>` in the Rust matcher/evaluator
- **Capture:** make `*` mark the returned element on both `>` and `>>` chains, and filter captured elements to those whose subtree matches the remaining chain (so `*JPanel >> FormsLabel[text='Input']` returns only cards containing that label).
- **Deep `>`:** fix multi-level child-chain evaluation so `A > B > C (> …)` matches whenever a satisfying ancestor path exists.
- **Why:** these make "target the clickable ancestor precisely" a robust option, complementing D1 and replacing the brittle geometry workaround.
- **Guard:** add matcher/evaluator unit tests for both, plus regression assertions that existing 2-level chains and non-capture selectors are unchanged.

### D3: Deterministic retargeting test independent of the showcase
Add an agent-level test that builds a `JPanel` with a recording `MouseListener` containing a listener-less child, clicks the child through the same path `performMouseClick` uses, and asserts the panel's listener fired. Keep an opt-in live showcase check (`Click FormsLabel[text='Input']` navigates) that self-skips without the jar.
- **Why:** proves the behavior in CI without a third-party binary; the live check guards the real-world case.

## Risks / Trade-offs

- **[Retargeting changes click delivery for listener-less targets]** → scoped strictly to "target has no listeners"; buttons/normal components unchanged. Validate with the full Swing regression (697 tests, 0 failed baseline) + the new unit tests.
- **[Over-retargeting to an unintended ancestor]** → stop at the first listener-bearing ancestor and never past `Window`; document that behavior. Point translation via `convertPoint` keeps coordinates correct.
- **[Locator evaluator change regresses existing selectors]** → add before/after tests for 2-level `>`, simple types, attributes, and `>>`; run the RF dryrun + Swing suite.
- **[Headless focus concerns]** → retargeting uses synthetic dispatch (no window focus needed), avoiding the Robot approach's flakiness.

## Migration Plan

1. Implement D1 in `ActionExecutor.performMouseClick`; add the agent unit test (D3). Rebuild agent.
2. Implement D2 in `src/locator/`; add matcher/evaluator unit tests. Rebuild the extension.
3. Verify: `Click FormsLabel[text='Input']` navigates on the live showcase; `*JPanel >> FormsLabel[text='Input']` returns only the card; `JViewport > JPanel > JPanel` matches.
4. Regression: full Swing suite (expect 0 failures), RF dryrun, `cargo test`, `pytest`.
5. Follow-up change/PR: extend `tests/robot/showcase/` to navigate via label click and drive radio/table/tree pages with validated read-back.
- **Rollback:** both changes are localized (one Java method, one matcher module); revert independently.

## Open Questions

- **Q1:** Should retargeting also cover `Double Click` / `Right Click` / `Click Element` variants in the same change? (Likely yes — they share `performMouseClick`; confirm they route through it.)
- **Q2:** Should a `Robot`-based pixel-click keyword be offered as an explicit opt-in for pixel-exact cases (e.g. clicking a specific spot in a canvas)? Deferred.
- **Q3:** For capture filtering, is the desired result the captured ancestor only, or all captured-segment matches with a boolean "has descendant"? Spec says the filtered ancestor; confirm no existing `>>` users depend on the current unfiltered behavior.
