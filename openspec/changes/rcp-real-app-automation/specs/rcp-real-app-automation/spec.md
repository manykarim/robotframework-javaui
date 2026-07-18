## ADDED Requirements

### Requirement: Reproducible harness drives a real packaged Eclipse RCP product
A reproducible Docker harness SHALL automate a real, third-party, packaged Eclipse RCP
application (DBeaver Community Edition) headless, with the JavaGui agent attached — not a
purpose-built test bundle. The harness SHALL be self-contained and produce evidence
(logs + screenshots) on a single `docker run`.

#### Scenario: Harness launches the product with the agent and connects
- **WHEN** the harness image is built and run
- **THEN** it starts DBeaver headless under Xvfb with the agent attached, waits for the workbench
  to render, and the proof suite connects to the agent before any scenario executes

#### Scenario: Packaged product with a trimmed JRE is still automatable
- **WHEN** the product ships a `jlink` runtime that omits the `java.instrument` module
- **THEN** the harness attaches a full JDK via `-vm` in the product `.ini` so the `-javaagent`
  loads, rather than failing with `libinstrument.so: cannot open shared object file`

#### Scenario: Suite is opt-in and self-skips
- **WHEN** the agent port is not reachable (harness not running)
- **THEN** the suite skips with a clear message rather than failing, so default CI is unaffected

### Requirement: Introspection returns live data from the real product
The RCP introspection keywords SHALL return live workbench data from the running product, not
mock/error placeholders.

#### Scenario: Live workbench, perspectives, and views
- **WHEN** connected to the running DBeaver and the introspection keywords run
- **THEN** `Get Workbench Info` returns a real window/view count and active perspective,
  `Get Available Perspectives` returns the product's registry ids, and `Get Open Views` returns
  the product's open views with titles

### Requirement: Every action is validated by read-back AND visual confirmation
Each scenario SHALL follow a state-changing action with (a) an assertion that reads the workbench
state back and (b) a framebuffer screenshot embedded in the run output. A scenario SHALL NOT treat
"the keyword returned without error" as proof.

#### Scenario: Action outcome is seen, not just returned
- **WHEN** the suite performs a workbench action (show/close view, switch perspective, execute a command)
- **THEN** it reads the resulting workbench state back to confirm the change AND captures a screenshot,
  so an RPC-level "success" that did not change the UI (or raised an on-screen error) is caught
