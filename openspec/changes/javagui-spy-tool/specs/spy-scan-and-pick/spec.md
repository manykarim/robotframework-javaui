## ADDED Requirements

### Requirement: Scan a live instrumented app over the existing agent
The spy SHALL attach to a running instrumented Java app (Swing/SWT/RCP) by reusing the existing
JSON-RPC-over-TCP agent — either connecting alongside a running session (`connect host:port`) or
launching the app with the wheel-bundled agent (`launch --jar …`). It SHALL NOT require new
instrumentation for the scan MVP.

#### Scenario: Attach without disrupting a running test
- **WHEN** the spy connects to an app that already has a Robot Framework session attached
- **THEN** it scans concurrently (the agent accepts multiple clients) without interfering

#### Scenario: Scans are transport-safe
- **WHEN** trees are fetched
- **THEN** the spy uses a single serialized connection, per-window and lazy/depth-bounded fetches,
  and change-polling rather than repeated full-tree pulls — avoiding the known broken-pipe flakiness

### Requirement: Pick a widget and highlight matches
The spy SHALL let a user pick a widget from a screenshot (client-side hit-test against cached
geometry in the MVP; an in-JVM `hitTest(x,y)` RPC returning the root→leaf ancestor path in a later
phase) and SHALL highlight a locator's matches.

#### Scenario: Click-to-inspect resolves the picked widget
- **WHEN** the user clicks a point on the app screenshot
- **THEN** the deepest widget at that point is selected and its properties + ancestor breadcrumb shown

#### Scenario: Highlight never corrupts the scan
- **WHEN** the spy highlights matches via an in-JVM overlay
- **THEN** the overlay is non-focusable, hollow (interior stays interactive), tagged, auto-disposed,
  and filtered out of every inspection RPC so it never appears in its own scans
