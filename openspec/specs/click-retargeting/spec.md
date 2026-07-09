# click-retargeting Specification

## Purpose
TBD - created by archiving change click-retargeting-and-locator-fixes. Update Purpose after archive.
## Requirements
### Requirement: Click activates the handler a real user would trigger
When a click targets a component that has no mouse listeners of its own but is visually inside a clickable ancestor (a common Swing pattern: a listener-less label on a card panel whose `MouseListener` performs the action), the library's `Click` SHALL activate that ancestor's handler — matching what a real pixel click does via AWT's `LightweightDispatcher`. The guiding invariant: **if a user can steer the app by clicking a location, the library can steer it by locating a component at or within that location.**

#### Scenario: Clicking a listener-less child activates the ancestor's handler
- **WHEN** `Click` targets a component that has no `MouseListener`s and whose nearest listener-bearing ancestor (below the `Window`) carries the click behavior
- **THEN** the synthetic press/release/click sequence is dispatched to that ancestor (with the point translated via `SwingUtilities.convertPoint`), and the ancestor's handler fires — e.g. `Click FormsLabel[text='Input']` on the Smart Client Showcase navigates to the Input page

#### Scenario: Normal targets are unaffected
- **WHEN** `Click` targets an `AbstractButton` or any component that has its own mouse listeners
- **THEN** behavior is unchanged (buttons still use the `doClick()` fast-path; components with listeners receive the event directly)

#### Scenario: No ancestor handler exists
- **WHEN** the target and all its ancestors up to the `Window` have no mouse listeners
- **THEN** the click is delivered to the located component as before (no spurious retargeting, no error)

### Requirement: Retargeting is verifiable without a specific third-party app
The retargeting behavior SHALL be covered by a deterministic test that does not depend on the showcase jar.

#### Scenario: Synthetic ancestor-listener tree
- **WHEN** a test builds a panel with a `MouseListener` containing a listener-less child, and clicks the child through the agent
- **THEN** the panel's listener records the click

