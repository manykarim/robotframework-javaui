## ADDED Requirements

### Requirement: RCP keywords validated against a real Eclipse workbench
The RCP toolkit SHALL be validated end-to-end against a real Eclipse Rich Client Platform application, not only against the in-memory `MockRcpApplication` simulation. A buildable Eclipse RCP test application SHALL exist, be produced by the build, and be launched by the RCP robot suites.

#### Scenario: Real RCP test app builds and launches
- **WHEN** the project build runs for the RCP test application under `tests/apps/rcp`
- **THEN** it produces a runnable Eclipse RCP artifact and the RCP robot suites can launch it with the Java agent and connect via `Connect To SWT Application`

#### Scenario: RCP suites run against real Eclipse in CI
- **WHEN** the RCP robot suites execute in CI against the real Eclipse RCP app on a headless display
- **THEN** the perspective/view/editor keywords (`View Should Be Open`, `Get View Title`, `Get Active Perspective Id`, `Is Editor Dirty`, etc.) pass against the live workbench, not the mock

### Requirement: RCP introspection keywords return live workbench data
`Get All Rcp Views`, `Get All Rcp Editors`, and `Get Rcp Component Tree` SHALL return real data from a live Eclipse workbench when connected to a real RCP application, instead of `{"error":"Eclipse RCP not available"}`.

#### Scenario: Introspection against real Eclipse
- **WHEN** connected to the real Eclipse RCP test app and `Get All Rcp Views` is called
- **THEN** it returns the actual open view identifiers/titles from the Eclipse workbench, and `Get Rcp Component Tree` returns a non-error tree rooted at the workbench

#### Scenario: Clear error only when Eclipse genuinely absent
- **WHEN** an introspection keyword is called against a non-Eclipse (mock or plain SWT) target
- **THEN** the keyword fails with a clear, documented error explaining a real Eclipse workbench is required, rather than silently returning empty data

### Requirement: Mock RCP app retained only as a fast fixture
The `MockRcpApplication` MAY remain as a fast smoke-test fixture, but SHALL NOT be the sole validation path for any RCP keyword that a real Eclipse workbench can exercise.

#### Scenario: Every real-Eclipse-capable keyword has a real-Eclipse test
- **WHEN** the RCP capability matrix is reviewed before release
- **THEN** each RCP keyword that depends on the Eclipse workbench has at least one test against the real Eclipse app, and keywords only exercised by the mock are explicitly documented as mock-only
