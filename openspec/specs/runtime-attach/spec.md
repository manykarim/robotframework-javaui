# runtime-attach Specification

## Purpose

Attach the javagui agent to an already-running JVM (Swing/SWT/RCP) by PID, without a launch-time
`-javaagent`, and discover candidate JVMs — enabling automation of found-in-the-wild apps and
Java Web Start (JNLP) applications.

## Requirements

### Requirement: Attach the agent to a running JVM by PID

The library SHALL load the agent into an already-running target JVM using the JDK Attach API
(`agentmain`), without requiring `-javaagent` at the target's launch. The attach SHALL boot the
same RPC server that `premain` boots, and the caller SHALL then connect via the existing
`Connect To Application(port)` path.

#### Scenario: Attach to a plain Swing app started without an agent
- **WHEN** a Swing app is running that was launched as `java -jar app.jar` (no `-javaagent`) and the library attaches the agent to its PID with `port=P`
- **THEN** the agent's RPC server binds `P`, and `get_ui_tree`/`find_elements` return the real component tree (non-empty), identical to a launch-time attach

#### Scenario: Attach to a plain SWT app auto-detects the toolkit
- **WHEN** the library attaches with `toolkit=auto` to a running SWT app
- **THEN** the agent detects SWT from the already-loaded classes and starts the SWT RPC server (it does NOT default to Swing), and `getComponentTree` returns the SWT widget tree

#### Scenario: JRE-only host uses the jattach fallback
- **WHEN** no JDK is present on the automation host (only a JRE) and attach is requested
- **THEN** the library uses the bundled `jattach` fallback to load the agent, rather than failing for lack of `tools.jar`/JDK Attach

### Requirement: Discover candidate target JVMs

The library SHALL enumerate running JVMs and classify them so a caller can select a target by
`pid`, `main_class`, or window `title`. `List Applications` SHALL return discovered JVMs rather
than raising `NotImplementedError`. Selection ambiguity SHALL be an explicit error listing the
candidates — never a silent guess.

#### Scenario: List Applications returns running JVMs
- **WHEN** one or more instrumentable JVMs are running and `List Applications` is called
- **THEN** it returns a list of candidates with at least pid, main class / command line, and any window title — not an empty stub or an error

#### Scenario: Select the app JVM, not the launcher
- **WHEN** a target is launched by a wrapper/launcher JVM and both are running
- **THEN** discovery classifies and selects the application JVM (by main-class / loaded jars / owned window markers) and excludes the launcher/bootstrap JVM

#### Scenario: Ambiguous selector errors with candidates
- **WHEN** a selector (`main_class=`/`title=`) matches zero or more than one JVM
- **THEN** the attach keyword raises an explicit error enumerating the candidate PIDs, and does not attach to an arbitrary one

### Requirement: Runtime attach is additive and non-breaking

The runtime-attach path SHALL NOT change the existing launch-time (`-javaagent`) flow or the
`Connect To Application(port)` keyword. Attaching an already-attached JVM SHALL be a safe no-op.

#### Scenario: Existing launch-time flow unchanged
- **WHEN** a user launches an app with `-javaagent` and connects by port as before
- **THEN** behavior is identical to the prior release (no regression)

#### Scenario: Double attach is a safe no-op
- **WHEN** the agent is attached to a JVM that already has the agent initialized
- **THEN** the second attach detects the existing initialization and does not start a second server or crash the target
