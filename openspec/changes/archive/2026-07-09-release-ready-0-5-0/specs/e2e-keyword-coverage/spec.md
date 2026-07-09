## ADDED Requirements

### Requirement: Every non-deprecated keyword has a live-app E2E test
Each public, non-deprecated Robot Framework keyword exposed by `SwingLibrary`, `SwtLibrary`, and `RcpLibrary` SHALL be exercised by at least one robot test that drives a real running Java application (Swing/SWT/RCP), asserting an observable effect or returned value.

#### Scenario: Coverage threshold met
- **WHEN** E2E keyword coverage is measured across `tests/robot/`
- **THEN** at least 90% of non-deprecated public keywords have a live-app test, and the remaining exceptions are explicitly listed with a justification

#### Scenario: Previously uncovered keywords now tested
- **WHEN** the SWT/RCP getters and assertions previously reported as uncovered (SWT table/tree/widget getters, SWT table/tree assertions, list assertions, RCP editor/view/perspective queries) are checked
- **THEN** each has a robot test that invokes it against a live app and verifies its result

### Requirement: No test suite appears green without executing
Robot suites SHALL NOT present as passing while their cases are skipped. Suites whose cases are tagged `robot:skip` SHALL either be enabled to run against a live app or removed.

#### Scenario: Cascaded Swing suites resolved
- **WHEN** the cascaded Swing suites (`16_cascaded_basic.robot`, `17_cascaded_engines.robot`, `18_cascaded_capture.robot`) are reviewed
- **THEN** their `robot:skip`-tagged cases are either un-skipped and passing against the live Swing app, or the dead cases are deleted, so no suite reports success without running its assertions

### Requirement: Coverage is measurable and enforced
The project SHALL provide a repeatable way to compute keyword E2E coverage so regressions are detectable.

#### Scenario: Coverage report reproducible
- **WHEN** a maintainer runs the documented coverage check
- **THEN** it reports total keywords, covered count, uncovered names, and the coverage percentage, matching the release threshold gate
