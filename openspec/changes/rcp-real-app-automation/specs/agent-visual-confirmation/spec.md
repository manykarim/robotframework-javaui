## ADDED Requirements

### Requirement: Capture Screenshot works end-to-end and is available on every toolkit library
The `Capture Screenshot` keyword SHALL produce a real image of the running application (or a widget)
by invoking the agent's existing capture, transporting it through the Rust layer, and writing it to
the screenshot directory. It SHALL be available on the SWT and RCP libraries, not only Swing.

#### Scenario: Screenshot writes a real image (no stub)
- **WHEN** `Capture Screenshot <path>` is called against a connected application
- **THEN** the Rust layer invokes the agent `captureScreenshot`, decodes the returned base64 PNG, and
  writes a non-empty PNG to `<path>` — rather than returning a fabricated path with no file

#### Scenario: Available on the RCP/SWT libraries
- **WHEN** a suite loaded with `JavaGui.Rcp` (or `JavaGui.Swt`) calls `Capture Screenshot`
- **THEN** the keyword exists and captures the app, rather than failing with "No keyword with name
  'Capture Screenshot' found"

### Requirement: Screenshots are embedded in the Robot log
When a screenshot is captured, its image SHALL be embedded into the Robot Framework log so the run
report provides in-context visual confirmation.

#### Scenario: Image appears in log.html
- **WHEN** a screenshot is captured during a suite
- **THEN** the saved image is embedded (as `<img>`) in the Robot log at that step
