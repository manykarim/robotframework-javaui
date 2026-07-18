## ADDED Requirements

### Requirement: `type:` locators resolve for SWT/RCP widgets
The library SHALL recognize the `type:<value>` (and `type=<value>`) locator prefix for SWT/RCP
widget finding and actions, routing it to the agent's type matcher (which matches on the widget's
SWT class simple name), rather than mis-parsing it as a class name with the prefix attached.

#### Scenario: type locator finds widgets by SWT class
- **WHEN** `Find Widgets type:Shell` (or `type:Text`, `type:Button`, ...) runs against a connected SWT/RCP app
- **THEN** it returns the widgets whose SWT class simple name matches (e.g. all live shells), instead of 0 results

#### Scenario: type locator drives actions
- **WHEN** an action keyword (`Click Widget`, `Input Text`, `Check Button`) is given a `type:` locator
- **THEN** the target widget is resolved by type and the action is performed, rather than failing to resolve

#### Scenario: other locator prefixes are unchanged
- **WHEN** `text:`, `name:`, `class:`, `id:`, or `index:` locators are used
- **THEN** they behave exactly as before (regression-free)

### Requirement: main workbench window widgets are reachable
With `type:` locators fixed, generic widget finding SHALL reach widgets rendered in the main
Eclipse-4 workbench window, not only widgets inside modal dialogs.

#### Scenario: main-window reachability
- **WHEN** the DBeaver reachability suite enumerates `Find Widgets type:<T>` for common SWT types after the workbench renders
- **THEN** at least one type returns a non-zero count (the main window's widgets are found), where before the fix all returned 0
