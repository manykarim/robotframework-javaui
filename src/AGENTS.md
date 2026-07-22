# AGENTS.md — `src/` (Rust core)

Nested guide for the Rust core. Parent contract: [`../AGENTS.md`](../AGENTS.md) — build/test/lint/PR
rules live there; this file covers only `src/`-local specifics. Read the code before editing.

## What this crate is
PyO3 extension compiled to `JavaGui._core` (see `lib.rs` `#[pymodule] fn _core`). It parses/matches
locators and hosts the RF library classes; the Java agent (`../agent/`) does the in-JVM work over
JSON-RPC. `_core` exports: `JavaGuiElement`, `JavaGuiLibrary`, `SwingLibrary`/`SwingElement`,
`SwtLibrary`/`SwtElement`, `RcpLibrary`, the `suggest_locators`/`explain_locator` functions (javagui-spy),
and the exception hierarchy. Feature flags: `swing`, `swt`, `rcp`, `all-toolkits`.

## Module map
- `lib.rs` — pymodule entry; `add_class`/`add_function` registration. New Python-visible type ⇒ register here.
- `python/` — PyO3 bindings = the RF keywords' Rust side. `base_library.rs` (unified `JavaGuiLibrary`),
  `swing_library.rs`, `swt_library.rs`, `rcp_library.rs` (extends SWT), `element.rs`/`swt_element.rs`,
  `exceptions.rs` (`register_exceptions`), `tests.rs`. These are large (>500 line guideline waived).
- `locator/` — the locator engine:
  - `grammar.pest` + `parser.rs` → `parse_locator` (pest grammar; edit `.pest` AND parser together).
  - `ast.rs` — `Locator`/selectors/`Combinator`; `expression.rs` — CSS/XPath/simple `LocatorExpression`.
  - `matcher.rs` — Swing matching (`Evaluator`, `find_matching_components`). `swt_matcher.rs` — SWT
    (`parse_swt_locator`, `SwtMatcher`). `unified.rs` — cross-toolkit `UnifiedLocator`/`LocatorFactory`.
  - `generator.rs` — offline locator suggestion + parse-error explainer (backs javagui-spy `suggest`/`describe`).
  - `cache.rs` — locator cache (built + tested; verify it is actually wired before relying on it in hot paths).
- `core/` — `Backend` trait, `Config`, unified `element`, `type_mapping`, `element_cache`.
- `model/` — wire data models (`component`/`widget`/`tree`/`rcp`/`element`).
- `protocol/` — JSON-RPC shapes. `connection/` — TCP connection mgmt. `error.rs` — error types.

## Locator features (where they live)
Combinators `>` (child) and `>>` (cascaded/capture, `*` = capture target); `:has(...)`, `:nth-of-type(n)`;
geometry attrs `[x][y][width][height]`; forms `JButton[name='ok']`, `#id`, `text:Login`, `//JButton[@text='OK']`.
Grammar in `grammar.pest`; Swing match semantics in `matcher.rs` (`match_combinator_chain` advances the
ancestor cursor; `find_cascaded_with_capture` filters captures by subtree containment); SWT in `swt_matcher.rs`.
Change grammar ⇒ touch `.pest`, `parser.rs`, `ast.rs`, and the relevant matcher, plus tests, together.

## Rebuild / test / lint (src-local)
```bash
uv run maturin develop --release     # rebuild ONLY the Rust ext after a src/ change (fastest loop)
cargo test                           # 245 passed — unit tests live in *tests.rs + #[cfg(test)] mods
cargo fmt && cargo clippy -D warnings # must stay clean (invoke lint runs both)
```
`cargo test`/`clippy` run without Python or a JVM. `maturin develop` mutates the venv's installed
`JavaGui._core` — a stale ext is the usual cause of "Python change didn't take": rebuild after editing `src/`.
Pure-Rust edits don't need the agent JAR; only re-run `invoke build-dev` if you also touched `../agent/`.

## Gotchas (src-specific)
- Version string comes from `env!("CARGO_PKG_VERSION")` — bump `Cargo.toml` (and `Cargo.lock`) with `pyproject.toml`.
- PyO3 signatures are the RF keyword surface: renaming/reordering args here changes public keywords —
  update `python/JavaGui/` + `docs/keywords/*.html` in the same change (parent Anti-drift rule).
- `ast::PseudoSelector` and `expression::PseudoSelector` are distinct types (aliased `AstPseudoSelector` in
  `mod.rs`); `swt_matcher::LocatorError` ≠ `unified::LocatorParseError`. Import the right one.
