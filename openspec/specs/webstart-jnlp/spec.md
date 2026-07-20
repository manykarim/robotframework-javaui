# webstart-jnlp Specification

## Purpose

Automate Java Web Start (JNLP) applications launched by `javaws`/OpenWebStart/IcedTea-Web, by
launching the `.jnlp`, discovering the (in-process or forked) application JVM, and attaching the
agent — building on the `runtime-attach` capability.

## Requirements

### Requirement: Launch and attach to a Web Start application

The library SHALL provide a flow that launches a `.jnlp` with an available launcher, discovers the
application JVM (whether it runs in-process in the launcher or in a forked child), attaches the
agent, and connects — yielding a working introspection session.

#### Scenario: In-process JNLP app (no special JVM args)
- **WHEN** a JNLP whose `<j2se>` requests no launcher-mismatched vm-args is launched, so the app runs in the launcher JVM
- **THEN** discovery identifies that JVM as the application JVM and the attach + connect succeeds against it

#### Scenario: Forked JNLP app (requests JVM args)
- **WHEN** a JNLP requests JVM args (e.g. heap) that force the launcher to fork a child app JVM
- **THEN** discovery selects the forked child (not the launcher) via its markers and attaches to it

### Requirement: Web Start injection strategy and limits

Dynamic attach SHALL be the default injection vector for Web Start (because `-javaagent` is not on
the JSR-56 secure vm-args whitelist). `JAVA_TOOL_OPTIONS` SHALL NOT be used as the default (it
double-loads the launcher JVM). Applications running under an active `SecurityManager` that denies
agent initialization SHALL be reported clearly as unsupported/degraded rather than hanging.

#### Scenario: -javaagent-in-JNLP is not relied upon
- **WHEN** WebStart support is designed
- **THEN** it does not depend on placing `-javaagent` in the JNLP `<j2se java-vm-args>` (which the secure whitelist rejects and signing does not unlock); it uses dynamic attach instead

#### Scenario: Sandboxed legacy launcher is reported, not hung
- **WHEN** attach fails because a legacy launcher's `SecurityManager` denies agent initialization (e.g. IcedTea-Web `JNLPSecurityManager`)
- **THEN** the library surfaces a clear, actionable error identifying the SecurityManager as the cause, and does not silently hang or report a false success

#### Scenario: Modern launcher / JDK without a legacy SecurityManager works
- **WHEN** the JNLP app runs under a launcher/JDK that does not install a restrictive SecurityManager (modern OpenWebStart, or JDK 24+ where the SecurityManager cannot be enabled)
- **THEN** dynamic attach + connect succeeds and introspection returns the real UI tree
- **NOTE** an all-permissions grant does NOT by itself unlock attach under IcedTea-Web: its `JNLPSecurityManager` denies the attach-loaded agent's foreign code regardless of the app's permission level (verified by experiment)

### Requirement: Deterministic Web Start test harness

The change SHALL ship a self-hosted minimal-Swing JNLP test harness that runs headless under xvfb
and self-skips when no JNLP launcher is present (following the existing showcase/opt-in convention).

#### Scenario: Self-hosted JNLP suite runs or self-skips
- **WHEN** the JNLP test suite runs on a host with a launcher (`javaws`) available
- **THEN** it serves a minimal Swing app as a JNLP, launches it, attaches, and asserts introspection; **and WHEN** no launcher is present, the suite skips rather than failing
