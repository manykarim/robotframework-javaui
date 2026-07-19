## ADDED Requirements

### Requirement: Generate a unique, verified locator for any node
The tool SHALL synthesize a Robot Framework locator for a given widget node and SHALL emit it only
after verifying, against the live tree, that it resolves to EXACTLY ONE node whose identity equals
the target node (right match-count AND right node). Generation and verification SHALL use the same
production Rust parser/matcher that Robot Framework executes (parity by construction) — never a
separate reimplementation.

#### Scenario: Emitted locator round-trips to the target
- **WHEN** the generator produces a locator for node N and it is resolved via the production `find_elements`
- **THEN** the result is exactly one element whose agent id equals N's id; candidates that match a
  different node or a different count are rejected before being shown

#### Scenario: Actionable widgets are covered
- **WHEN** locators are generated for every visible actionable widget (button, text field, label, combo, menu item, list/table/tree)
- **THEN** each receives at least one working unique locator (validated ≥95% overall, ≥90% for name-null nodes at depth ≥10 on the showcase/DBeaver matrix)

### Requirement: Tiered ladder with ancestor anchoring for deep, no-name, non-unique nodes
The generator SHALL prefer stable attributes in order (name > accessiblename > text > tooltip) and,
when no single segment is unique, SHALL search nearest-stable-ancestor `>>` anchored chains before
resorting to `nth-of-type`/index, and geometry only as a last resort. `nth-of-type` SHALL be
computed by the matcher's own type-index routine (no off-by-one drift).

#### Scenario: No-name deep node resolved by anchoring
- **WHEN** a target has no `name` and non-unique `text`/`type`
- **THEN** the generator anchors on the nearest ancestor carrying a unique qualifier and emits
  `Anchor >> Target…`, verified unique — rather than failing or jumping straight to a brittle index

#### Scenario: Fallbacks are flagged, geometry share stays low
- **WHEN** only an index or geometry locator can make a node unique
- **THEN** the candidate is emitted but flagged brittle with the reason; geometry-fallback share is
  reported as a quality KPI (target < 10%, < 5% on the showcase)

### Requirement: Ranked candidates with stability metadata
The generator SHALL return the top-3 ranked candidates as a uniform contract
`{locator, strategy, match_count, unique, stability, brittle_flags, preconditions}`, serving the GUI,
the CLI, and future self-healing without divergence.

#### Scenario: Correctness traps handled
- **WHEN** the target is a stamped table/list/tree cell renderer, or a hidden tab/card child, or inside a volatile (changing child_count) container
- **THEN** the generator suggests a data-locator keyword instead of a component locator (cells),
  appends `:showing` + a precondition (hidden cards), or avoids nth/index in favor of content-based
  locators (volatile containers)
