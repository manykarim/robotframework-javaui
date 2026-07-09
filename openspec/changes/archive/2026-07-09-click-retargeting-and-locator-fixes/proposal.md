## Why

A UI-automation library must honor one guarantee: **if a human can steer the app by clicking, the library can too.** Today it can't in a common case. The Swing agent's `Click` dispatches a synthetic mouse event **directly to the located component**. Real OS clicks don't work that way — AWT's `LightweightDispatcher` delivers a pixel event to the deepest component *that has mouse listeners*, retargeting past listener-less children. So when a widget's behavior lives on an ancestor (e.g. a card panel with the `MouseListener`, showing a listener-less label), clicking the label does nothing via the library but works for a user.

This was proven against the JGoodies Smart Client Showcase: its hub tiles are `com.jgoodies.fluent.tiles.AbstractTileView` cards whose `MouseHandler` sits on the card `JPanel`, not the visible `FormsLabel`. `Click FormsLabel[text='Input']` is a no-op; a real click navigates. The workaround (click the card by geometry `JPanel[x][y][width][height]`) works but is brittle. The right fix is to make the synthetic click behave like a real one. The same investigation surfaced two locator-engine bugs that undermine the alternative (targeting the ancestor precisely).

## What Changes

- **Click retargeting (primary):** in the Swing agent's `performMouseClick(Component, int)`, replicate `LightweightDispatcher` behavior — if the target component has no `MouseListener`s (and no `MouseMotionListener`s where relevant), walk up its ancestor chain to the nearest component that has mouse listeners (stop at the `Window`), translate the click point with `SwingUtilities.convertPoint`, and dispatch the `MOUSE_PRESSED`/`MOUSE_RELEASED`/`MOUSE_CLICKED` sequence to that ancestor. Result: `Click FormsLabel[text='Input']` (and any label-on-a-clickable-card pattern) just works. The existing `AbstractButton.doClick()` fast-path is unchanged.
- **Locator bug A — capture semantics:** the capture marker (`*`) is ignored on CSS `>` child chains, and on `>>` cascaded chains it returns the captured segment's matches **unfiltered** by the final segment. Make capture apply on both, returning only captured elements whose subtree matches the remaining chain.
- **Locator bug B — deep CSS `>` chains:** CSS child chains of 3+ levels return 0 matches in cases that should match (e.g. `JViewport > JPanel > JPanel`). Fix the evaluator so multi-level `>` chains match correctly.
- **Tests:** unit/integration coverage for retargeting (a synthetic component tree with the listener on an ancestor) and for the two locator behaviors; verify `Click FormsLabel[text='Input']` navigates against the showcase (opt-in).
- **Non-goal (this change):** a `java.awt.Robot` real-pixel-click keyword — noted as an alternative but not required once retargeting works. Extending the showcase proof to radio/table/tree is a follow-up (see Impact).

## Capabilities

### New Capabilities
- `click-retargeting`: `Click` (and click variants) must activate the handler a real user would trigger — dispatching to the nearest ancestor with mouse listeners when the located component has none — so any clickable pixel is reachable by locating a component at or within it.
- `locator-chain-matching`: CSS `>` child chains of arbitrary depth and the capture (`*`) marker must match correctly, so callers can precisely target an ancestor/among-siblings element when needed.

### Modified Capabilities
<!-- None — additive behavior + bug fixes; no existing spec's requirements are being rewritten. -->

## Impact

- **Code:** `agent/src/main/java/com/robotframework/swing/ActionExecutor.java` (`performMouseClick`); locator evaluator/matcher in `src/locator/` (`matcher.rs` capture + `>` chain handling, and its evaluator). Agent rebuild (`mvn`) + Rust rebuild (`maturin`).
- **Tests:** new Rust/agent unit tests + a live showcase check (opt-in, self-skips without the jar).
- **Follow-up (separate/after):** extend `tests/robot/showcase/` to navigate via `Click FormsLabel[text='...']` and drive the radio/table/tree demo pages with validated read-back — closing the categories the first showcase proof could not reach.
- **Risk:** retargeting changes click delivery for listener-less targets; guarded by "only when the target has no listeners" so normal clicks are unaffected (verified by the full Swing regression on any change).
