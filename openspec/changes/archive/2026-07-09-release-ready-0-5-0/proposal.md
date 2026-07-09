## Why

`robotframework-javaui` is functionally strong — Swing and SWT keywords genuinely drive real Java UIs via the Rust core → Java agent, and the v0.4.1 fixes closed the prior report's critical bugs (`force=True`, tree scoping, version mismatch). But it is **not release-honest**: RCP is presented as a peer of Swing/SWT yet is only ever tested against a self-built mock, several keywords overpromise or stub, one third of the keyword surface has no live-app test, and the repository is buried under ~71 throwaway process docs with stale generated keyword references. A public release must match the marketing to the verified reality.

## What Changes

- **Prove RCP on a real Eclipse workbench.** Build and wire the currently dead `tests/apps/rcp/plugins/com.testapp.rcp` Eclipse plugin app, run the RCP robot suites against it (not only `MockRcpApplication`), and make the flagship introspection keywords (`Get All Rcp Views`/`Get All Rcp Editors`/`Get Rcp Component Tree`) return real data instead of `{"error":"Eclipse RCP not available"}`. Keep the mock as a fast-path fixture but no longer as the *only* validation.
- **Close E2E coverage gaps.** Add live-app robot tests for the 68 uncovered keywords (dominated by SWT/RCP getters & assertions), and either un-skip or delete the cascaded Swing suites (16–18) that are currently `robot:skip` and only appear green. Target: raise E2E coverage from 66.5% toward ≥90% of non-deprecated keywords.
- **Fix stubs & overpromises.** Implement or remove `List Applications` (currently `return []`); make `Type Text` actually type char-by-char via key events or correct its docstring; honor the ignored `locator` argument of `Log UI Tree`. No keyword may claim behavior it does not perform.
- **Scrub the repository.** Delete/relocate ~71 throwaway `PHASE_*`/`MISSION_*`/`*_DELIVERY`/`*_REPORT` docs (146 → ~25–40), `git rm --cached` the 6 tracked `.claude-flow/` runtime files, gitignore `example-apps/` (11 MB third-party jar), and remove the duplicate/`_OLD` docs.
- **Consolidate & refresh keyword documentation.** Pick one canonical keyword reference, regenerate the stale libdoc (`docs/keywords/*.html` predate the v0.4.1 API cleanup), and remove the drift between the HTML libdoc and the hand-written markdown reference.
- **Rewrite README + examples for honesty.** **BREAKING (docs):** replace the implied Swing/SWT/RCP parity with a per-toolkit maturity table (Swing: stable, SWT: stable, RCP: real-Eclipse required); verify the PyPI publish claim at 0.4.1; and replace/curate `examples/` with runnable examples that drive the bundled test apps.

## Capabilities

### New Capabilities
- `rcp-real-eclipse-validation`: RCP keywords must be validated against a real Eclipse workbench app, not only a mock; defines the buildable test app, the CI wiring, and the required behavior of the introspection keywords.
- `e2e-keyword-coverage`: Every non-deprecated public keyword must have at least one live-app robot test; defines the coverage contract and the treatment of skipped suites.
- `keyword-honesty`: No keyword may stub or overpromise; defines the required behavior of `List Applications`, `Type Text`, and `Log UI Tree` and a general no-fake-return rule.
- `release-repository-hygiene`: Defines what may ship in the public repo — doc set bounds, untracked-artifact rules, gitignore requirements, and single-source keyword documentation.
- `release-documentation-accuracy`: README and examples must reflect verified per-toolkit maturity and only claim published/verified facts (PyPI, platforms, locator coverage).

### Modified Capabilities
<!-- None — openspec/specs/ contains no existing capability specs (only an empty archive). -->

## Impact

- **Code:** `python/JavaGui/__init__.py` (stub/overpromise fixes, docstrings), `src/python/swing_library.rs` & `rcp_library.rs` (if `Log UI Tree` locator / `List Applications` need core support), Java agent RCP routing (`SwtRpcServer.java`, `RcpComponentInspector.java`, `EclipseWorkbenchHelper.java`).
- **Tests:** new suites under `tests/robot/{swt,rcp}/`, buildable `tests/apps/rcp` Eclipse app + Maven/Tycho build, un-skip/removal of `tests/robot/swing/16-18_cascaded_*.robot`.
- **Docs:** mass deletion under `docs/`, regenerated `docs/keywords/*`, rewritten `README.md`, curated `examples/`.
- **Build/CI:** RCP-on-real-Eclipse job (headless Eclipse/SWT display), regenerate-libdoc step, PyPI publish verification.
- **Dependencies:** Eclipse RCP/Tycho build tooling for the real RCP test app; no new runtime dependencies for library users.
