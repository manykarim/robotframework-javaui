# release-documentation-accuracy Specification

## Purpose
TBD - created by archiving change release-ready-0-5-0. Update Purpose after archive.
## Requirements
### Requirement: README reflects verified per-toolkit maturity
The README SHALL present the three toolkits with their actual, verified maturity rather than implying parity. It SHALL state that Swing and SWT are stable and end-to-end tested, and that RCP requires a real Eclipse workbench and document its supported scope.

#### Scenario: Maturity table present
- **WHEN** a reader opens the README
- **THEN** a per-toolkit maturity/support table is shown (Swing: stable; SWT: stable; RCP: real-Eclipse required with documented scope), with no claim of equal keyword coverage across toolkits

### Requirement: All README claims are verifiable
The README SHALL only assert facts that are true at release time. Installation, platform, and capability claims SHALL be verified or softened.

#### Scenario: PyPI claim verified
- **WHEN** the README instructs `pip install robotframework-javagui`
- **THEN** the package is actually published at the release version on PyPI, or the instruction is adjusted to the real install path

#### Scenario: Platform and locator claims accurate
- **WHEN** cross-platform and "full XPath-style locator" claims are reviewed
- **THEN** they are backed by CI evidence / documented locator support, or reworded to match what is actually verified

### Requirement: Examples are runnable against bundled test apps
The `examples/` directory SHALL contain clear, runnable examples that drive the bundled Swing/SWT (and, where applicable, RCP) test applications, with no duplicated or dead example directories.

#### Scenario: Example runs end to end
- **WHEN** a new user follows an example's instructions
- **THEN** it launches the referenced bundled test app, connects, and executes real keywords successfully, demonstrating the library without external dependencies

#### Scenario: No duplicate example trees
- **WHEN** `examples/` and `example-apps/` are reviewed
- **THEN** their roles are distinct and documented (curated examples vs. test-app binaries), with no redundant duplication

