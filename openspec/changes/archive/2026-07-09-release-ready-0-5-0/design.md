## Context

The library is functionally strong (Swing/SWT keywords truly drive real Java UIs via the Rust core → Java agent) but not release-honest. The dominant technical risk in this change is **proving RCP on real Eclipse**: the real RCP test app `tests/apps/rcp/plugins/com.testapp.rcp` exists as standard Eclipse plugin sources (`plugin.xml`, `META-INF/MANIFEST.MF`, `Application.java`, perspectives, views, editors) but has **no build** — no `pom.xml`, no Tycho parent, no target platform. It is never compiled, never launched, and never referenced by any test. All 10 RCP robot suites launch the plain-SWT `MockRcpApplication` (`tests/apps/rcp-mock`, connect on port 5680), and the RCP introspection keywords (`Get All Rcp Views/Editors`, `Get Rcp Component Tree`) return `{"error":"Eclipse RCP not available"}` in every tested configuration.

The Java agent side is already real: `EclipseWorkbenchHelper.java` (~548 lines of `org.eclipse.ui` reflection) and `RcpComponentInspector.java` implement genuine workbench introspection. The gap is purely: (1) a buildable+launchable real RCP app, (2) agent attachment into Equinox, (3) headless CI, and (4) tests that exercise the real path.

The other four workstreams (E2E gaps, keyword honesty, repo hygiene, doc accuracy) are lower-risk and mostly mechanical, but must land together for an honest release.

## Goals / Non-Goals

**Goals:**
- Build and launch the real Eclipse RCP test app in CI on a headless display, and run the RCP suites against it (not only the mock).
- Make `Get All Rcp Views/Editors` and `Get Rcp Component Tree` return live workbench data against real Eclipse.
- Raise non-deprecated keyword E2E coverage to ≥90%; eliminate skip-only "green" suites.
- Remove all stubs/overpromises (`List Applications`, `Type Text`, `Log UI Tree`).
- Cut `docs/` to ~25–40 curated files, untrack tooling artifacts, single-source and regenerate the keyword reference.
- README + examples reflect verified per-toolkit maturity; PyPI/platform/locator claims verified or softened.

**Non-Goals:**
- Splitting the 3622-line `__init__.py` god file (tracked separately under ADR-001; out of scope here).
- Supporting arbitrary third-party Eclipse products (DBeaver, etc.) — we validate against our own RCP app; real-product support remains documented-but-unverified.
- New feature keywords from the v2 report backlog (Scroll Into View, Maximize Window, etc.) — deferred.
- Rewriting the mock; it stays as a fast smoke fixture.

## Decisions

### D1: Build the real RCP app with Tycho, pinned to a fixed Eclipse release
Use Maven **Tycho** (`tycho-maven-plugin` + `target-platform-configuration`) to compile `com.testapp.rcp` and export a runnable product, pinned to a specific Eclipse release p2 repository (e.g. `2023-x`) via a `.target` file committed in-repo.
- **Why:** Tycho is the only CI-friendly, reproducible way to build OSGi/RCP from Maven; matches the existing Maven-based test-app build flow.
- **Alternatives:** PDE "Export Product" (requires the Eclipse IDE, not headless/CI-reproducible — rejected). Hand-assembling an Equinox launch from raw jars (brittle, rejected). Bnd/plain-OSGi (reinvents Tycho — rejected).
- **Trade-off:** Tycho pulls a large p2 target platform from the network on first build; mitigated by pinning versions and caching.

### D2: Attach the Java agent to Equinox via product `vmargs`, connect on a distinct port
Inject `-javaagent:javagui-agent.jar` through the product config / `eclipse.ini`-style `vmargs` (or `-vmargs` on the launcher command line the robot suite builds), and have the real RCP app listen on a **separate port** (e.g. 5681) from the mock (5680) so both can be tested without collision.
- **Why:** Equinox launches its own JVM; the agent must be a `-javaagent` on that JVM, not the launcher. A distinct port keeps mock and real suites independent.
- **Alternatives:** Dynamic attach API (`com.sun.tools.attach`) — more moving parts, rejected for now.

### D3: Headless CI via Xvfb; parametrize the RCP suites over app target
Run RCP (and SWT) suites under `xvfb-run` in CI. Refactor `tests/robot/rcp/resources/common.resource` so the app jar/product path, port, and app-name are **variables**, letting the same suites run against either `mock` or `real-eclipse` via a variable set / suite variant.
- **Why:** SWT/Eclipse need a display; parametrizing avoids duplicating ~10 suites. Keeps mock as the fast local default and adds a real-Eclipse variant (CI + opt-in local).
- **Alternatives:** Duplicate the suites for real Eclipse (maintenance burden — rejected). Mock-in-a-container-with-display only (doesn't prove real Eclipse — rejected).

### D4: Keyword honesty — implement where cheap, else remove or re-document
- `List Applications`: implement JVM discovery via the Java Attach API / `jps`-style enumeration **if** cheap and reliable; otherwise **remove** it from the public surface (BREAKING, documented in migration). Do not keep the `[]` placeholder.
- `Type Text`: prefer implementing real per-char `KeyEvent` dispatch in the agent (the honest reading of its name); fallback is to correct the docstring to state it delegates to `input_text`. Decision recorded per Open Question Q3.
- `Log UI Tree`: forward the `locator` to `get_ui_tree`/`log_ui_tree` (the plumbing exists — `Log Component Tree` already does this) rather than removing the arg.

### D5: Doc scrub is deletion + single-source libdoc, not rewriting-in-place
Delete throwaway docs wholesale (git rm), regenerate `docs/keywords/*` from current source with `libdoc`, and designate the generated libdoc as the single canonical keyword reference; reduce the hand-written markdown reference to a thin pointer to avoid drift.
- **Why:** 146 → ~25–40; stale HTML predates the v0.4.1 API cleanup. Regeneration is deterministic; hand-editing invites re-drift.

### D6: Coverage gate is a reproducible script
Add a documented coverage checker (keyword surface vs. robot usage, excluding `@property` accessors and deprecated aliases) that emits total/covered/uncovered/percent, so the ≥90% threshold is enforceable and regressions visible.

## Risks / Trade-offs

- **[Tycho target-platform network fetch is slow/flaky in CI]** → Pin exact Eclipse release + a committed `.target`; cache `~/.m2` and the p2 bundle pool; allow an offline mirror.
- **[Agent fails to attach to Equinox / SWT-thread conflicts]** → Validate attach early (spike before wiring suites); the agent's SWT reflection already runs on the SWT display thread for the mock, reducing novelty.
- **[Headless SWT rendering differences vs real display]** → Xvfb with a sane resolution/color depth; keep assertions on model state (views/editors/perspectives) not pixels.
- **[Removing `List Applications` / renaming deprecated keywords breaks users]** → Treat as BREAKING in a 0.5.0 minor with a migration note; keep deprecated aliases emitting `DeprecationWarning` through 0.5.x per existing deprecation policy.
- **[Mass doc deletion loses something still referenced]** → Grep links before deleting; keep ADRs, guides, migration, release notes.
- **[Un-skipping cascaded suites surfaces real failures]** → If cascaded selectors are genuinely unimplemented, delete the dead cases rather than shipping skips; record the decision.
- **[Scope is large for one change]** → Sequence in phases (RCP-real first as the risk spike, then honesty fixes, then coverage, then hygiene, then docs); each phase independently verifiable.

## Migration Plan

1. **Spike (de-risk):** Add Tycho build for `com.testapp.rcp`, produce a product, launch headless with the agent attached, and confirm one introspection keyword returns live data. Gate the rest of the RCP work on this.
2. **Wire suites:** Parametrize `common.resource`; add a real-Eclipse suite variant; keep mock as default fast path.
3. **Honesty fixes:** `List Applications`, `Type Text`, `Log UI Tree` (may touch Python, Rust core, and Java agent).
4. **Coverage:** Add tests for the 68 uncovered keywords; resolve cascaded suites; add the coverage script + CI gate.
5. **Hygiene:** git rm throwaway docs + `.claude-flow/` files; gitignore `example-apps/`; regenerate libdoc.
6. **Docs:** README maturity table + claim verification; curate `examples/`.
7. **Release:** bump to 0.5.0, changelog with BREAKING notes, verify PyPI publish.
- **Rollback:** each phase is a separate commit/PR; RCP build is additive (mock path unchanged) so it can be reverted without affecting Swing/SWT.

## Open Questions

- **Q1 (RESOLVED):** Pinned to **Eclipse 4.30** (`R-4.30-202312010110`). Bundle requires `JavaSE-11`; runs on the available JDK 17. Reflection surface in `EclipseWorkbenchHelper` works against it.
- **Q2 (RESOLVED for 0.5.0):** `List Applications` kept but made honest — raises `NotImplementedError` (documented not-supported result) instead of returning `[]`. Full removal deferred (BREAKING) pending maintainer sign-off.
- **Q3 (RESOLVED for 0.5.0):** `Type Text` re-documented as an `input_text`-path append (no false key-event claim). Real `KeyEvent` dispatch remains a follow-up.
- **Q4 (RESOLVED):** Environment provides `Xvfb`/`xvfb-run` + JDK 17; Maven Central and download.eclipse.org are reachable. The real-Eclipse validation runs headless via `xvfb-run`.
- **DESIGN DEVIATION (from D1):** Tycho was NOT used. Instead the bundle is compiled directly against a downloaded Eclipse platform's plugin jars and installed via `dropins/` (see `tests/apps/rcp/build-and-run-real-eclipse.sh`). Simpler, fully reproducible, no Tycho target-platform resolution. Two agent bugs had to be fixed to make real-Eclipse introspection work: OSGi classloader discovery + SWT-UI-thread execution in `EclipseWorkbenchHelper`.
- **Q5 (open):** Should the real RCP app also ship as a runnable example, or remain test-only? Currently test-only.
- **Q6 (open):** `get_all_rcp_views/editors/get_rcp_component_tree` live on `SwingLibrary` (via `RcpComponentInspector`, a separate agent path) rather than `RcpLibrary` — an API inconsistency, and `RcpComponentInspector` still needs the same classloader fix that `EclipseWorkbenchHelper` received.
