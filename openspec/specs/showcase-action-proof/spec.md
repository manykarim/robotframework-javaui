# showcase-action-proof Specification

## Purpose
TBD - created by archiving change prove-actions-on-showcase. Update Purpose after archive.
## Requirements
### Requirement: Proof suite drives the real Smart Client Showcase app
The proof suite SHALL launch the third-party Smart Client Showcase application with the JavaGui agent attached, connect to it, and run real user interactions against it (not a purpose-built test app).

#### Scenario: Suite connects to the running showcase
- **WHEN** the showcase jar is available and the proof suite runs
- **THEN** it starts the app with the agent, connects successfully, and reports a connected session before any action test executes

#### Scenario: Suite is opt-in and self-skips without the jar
- **WHEN** the showcase jar (`example-apps/smart-client-showcase-24.09.0.jar`, gitignored) is not present
- **THEN** the suite skips its tests with a clear message rather than failing, so the default CI run is unaffected

### Requirement: Every performed action is independently validated by reading app state
Each test SHALL follow every state-changing action with an assertion that reads the application's state back and confirms the action took effect. A test SHALL NOT treat "the keyword returned without error" as proof.

#### Scenario: Text entry is verified
- **WHEN** the suite enters text into a field with `Input Text`
- **THEN** it reads the field's value back and asserts it equals the entered text; and after `Clear Text` it asserts the field is empty

#### Scenario: Navigation is verified
- **WHEN** the suite navigates to a different section/page of the showcase
- **THEN** it asserts a page-specific element or section title of the new section is showing/visible, confirming the navigation actually changed the view

#### Scenario: Radio-button selection is verified
- **WHEN** the suite selects a radio button in a group
- **THEN** it asserts that radio is selected AND the other radios in the group are not selected

#### Scenario: Checkbox toggling is verified
- **WHEN** the suite checks then unchecks a checkbox
- **THEN** it asserts the checkbox is selected after checking and not selected after unchecking

#### Scenario: Combo/list selection is verified
- **WHEN** the suite selects an item in a combo box or list
- **THEN** it asserts the selected value or index matches the chosen item

#### Scenario: Table/tree interaction is verified
- **WHEN** the suite selects a table row / tree node or expands a tree node
- **THEN** it asserts the resulting selection or expansion state via the corresponding getter

#### Scenario: Button action effect is verified
- **WHEN** the suite clicks a button that produces a visible change
- **THEN** it asserts the resulting change (e.g. a status label text, a dialog appearing, or an enabled/disabled transition), not merely that the click returned

### Requirement: Proof coverage spans the distinct action categories
The suite SHALL cover, at minimum, the categories: text entry, navigation, radio buttons, checkboxes, combo/list selection, table or tree interaction, and button-triggered visible change — each with its validating assertion.

#### Scenario: Category coverage is demonstrable
- **WHEN** the proof suite is reviewed
- **THEN** at least one validated test exists for each listed action category, and a run summary shows them passing against the live showcase app

### Requirement: Findings are reported, not hidden
If proving an action reveals that a keyword does not actually affect the app (a genuine defect), the suite SHALL surface it rather than relaxing the assertion to force green.

#### Scenario: A non-working action is surfaced
- **WHEN** an action cannot be validated because the app state does not change as expected
- **THEN** the test fails or is explicitly marked with a documented reason referencing the defect, instead of asserting a weaker condition that hides it

