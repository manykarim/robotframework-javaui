## ADDED Requirements

### Requirement: RCP action keywords are validated against a real workbench by read-back
Beyond introspection, the RCP **action** keywords (show/close/activate view, open/reset
perspective, execute command) SHALL be validated against a real Eclipse workbench by performing the
action and then reading the workbench state back to confirm it changed — not only by the keyword
returning without error.

#### Scenario: View action round-trips against real Eclipse
- **WHEN** a real-Eclipse suite closes then re-shows a registered view
- **THEN** the open-view read-back reflects each change (the view is absent after close, present
  after show), proving the action drove the live workbench

#### Scenario: Perspective switch is confirmed by read-back
- **WHEN** a real-Eclipse suite opens a registered perspective different from the active one
- **THEN** `Get Active Perspective Id` returns the requested id afterwards

#### Scenario: A keyword that raises on the UI is not counted as passing
- **WHEN** an action produces an on-screen error (e.g. `Invalid thread access`) while the RPC returns success
- **THEN** the validation treats this as a failure — surfaced via read-back and/or screenshot — rather
  than a pass
