## ADDED Requirements

### Requirement: CSS child chains match at any depth
A CSS-style child chain (`A > B > C > ...`) SHALL match components whose ancestor path satisfies each `>` step, regardless of chain length. Chains of three or more levels MUST NOT return zero matches when a matching path exists.

#### Scenario: Three-level child chain matches
- **WHEN** the locator `JViewport > JPanel > JPanel` is evaluated against a tree containing a `JViewport` with a child `JPanel` that has a child `JPanel`
- **THEN** the inner `JPanel`(s) are returned (a non-empty result), not zero matches

#### Scenario: Two-level chains keep working
- **WHEN** an existing two-level chain such as `JPanel > JButton` is evaluated
- **THEN** it continues to return the same matches as before (no regression)

### Requirement: Capture marker returns the captured element filtered by the rest of the chain
The capture marker (`*`) SHALL select the marked segment's element as the result, filtered to those whose subtree satisfies the remaining chain. This SHALL apply on both CSS `>` child chains and `>>` cascaded chains.

#### Scenario: Capture on a cascaded chain filters by the final segment
- **WHEN** `*JPanel >> FormsLabel[text='Input']` is evaluated
- **THEN** only the `JPanel`(s) that contain a matching `FormsLabel[text='Input']` in their subtree are returned — not every `JPanel`

#### Scenario: Capture on a child chain is honored
- **WHEN** a capture marker is used within a `>` child chain
- **THEN** the captured segment's elements are returned (filtered by the remaining chain), rather than the marker being ignored

#### Scenario: Enables precise ancestor targeting
- **WHEN** a caller needs the clickable card ancestor of a labelled tile
- **THEN** a capture expression can return that ancestor directly (so it can be clicked without hardcoded geometry), complementing the click-retargeting behavior
