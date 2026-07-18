## ADDED Requirements

### Requirement: State-changing workbench operations run on the SWT UI thread
Every state-changing RCP/SWT operation (show/hide/activate view, open/reset perspective, execute
command, editor open/save, widget interaction) SHALL execute its full reflection sequence on the
SWT UI thread via `syncExec`, so it never raises `org.eclipse.swt.SWTException: Invalid thread
access` against a real workbench.

#### Scenario: Execute Command does not raise Invalid thread access
- **WHEN** `Execute Command org.eclipse.ui.window.preferences` runs against a real Eclipse/DBeaver workbench
- **THEN** the command executes on the UI thread and the Preferences dialog opens, with no
  `Invalid thread access` error dialog and no `SWTException: Invalid thread access` in the product log

#### Scenario: Exceptions on the UI thread are surfaced, not hidden
- **WHEN** a state-changing operation throws while running inside `syncExec`
- **THEN** the exception is propagated to the caller as the RPC error, rather than the RPC returning
  a success that the UI contradicts

### Requirement: Action keywords report honest success or failure
An action keyword SHALL report success only when the workbench state actually changed, confirmed by
read-back, and SHALL report a clear failure otherwise. It SHALL NOT return success for a no-op, and
SHALL NOT report "not found" for an id present in the live registry.

#### Scenario: Close View is not a false-success no-op
- **WHEN** `Close View <open view id>` runs and the view remains open
- **THEN** the keyword reports failure (the read-back view list still contains the id), rather than success

#### Scenario: Show View / Open Perspective resolve registered ids
- **WHEN** `Show View <id>` or `Open Perspective <id>` is called with an id that the introspection
  keywords returned for the running product
- **THEN** the operation resolves the id against the live registry and performs it, rather than
  failing with "View not found" / "Perspective not found"
