## ADDED Requirements

### Requirement: SWT Display discovery is lifecycle-aware
The agent SHALL resolve the SWT `Display` in a lifecycle-aware way: because the agent's RPC port
opens at premain — before the SWT bundle is loaded and the workbench `Display` exists — Display
resolution SHALL be retried on each Display-dependent RPC (via `Instrumentation.getAllLoadedClasses()`)
until the `Display` class is loaded and an instance is available, rather than resolving once early
and caching failure.

#### Scenario: Agent attached at premain becomes usable when the workbench appears
- **WHEN** the agent attaches at premain and a client connects before the workbench `Display` exists
- **THEN** once the product creates its `Display`, the next Display-dependent RPC resolves it via the
  SWT bundle classloader and succeeds — without restarting the agent or the client

#### Scenario: Not-ready is a distinct, typed status
- **WHEN** a Display-dependent RPC is called before SWT is initialized
- **THEN** the agent returns a distinct `SWT_NOT_READY` status (not a generic error), so callers can
  retry rather than treat it as a hard failure

### Requirement: Readiness handshake for drivers
The agent SHALL expose a readiness wait, and `Connect To Swt Application` SHALL use it so callers do
not have to hand-roll a timing wait for the workbench.

#### Scenario: Wait Until SWT Ready blocks until the Display exists
- **WHEN** `Wait Until SWT Ready <timeout>` is called against an agent whose product is still starting
- **THEN** it blocks until the `Display` exists (up to the timeout) and then returns, so subsequent
  keywords run against a live workbench

#### Scenario: Connect retries while SWT is not ready
- **WHEN** `Connect To Swt Application` connects while the agent reports `SWT_NOT_READY`
- **THEN** it retries the readiness check up to the connect timeout instead of failing the first ping
