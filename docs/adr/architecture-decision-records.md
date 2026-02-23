# Architecture Decision Records - robotframework-javaui

---

# ADR-001: Module Restructuring - Split __init__.py God File

## Status: Proposed

## Context

The file `python/JavaGui/__init__.py` is currently 3523 lines long and contains four complete Robot Framework library classes:

- **SwingLibrary** (lines 200-1807): ~1600 lines including connection, element finding, clicking, text input, selection, table, tree, menu, wait, verification, screenshot, and configuration keywords, plus the full SwingElement wrapper class.
- **SwtLibrary** (lines 1809-3175): ~1400 lines with a massive docstring (~1400 lines of documentation alone), connection, shell, widget finding, click, text, selection, table, tree, wait, verification, and configuration keywords.
- **RcpLibrary** (lines 3176-3503): ~330 lines delegating to SwtLibrary and adding RCP-specific keywords for workbench, perspectives, views, editors, commands, toolbar, preferences, and menus.
- **SwingElement** (lines ~1700-1807): Element wrapper class.

Additionally, the file contains module-level imports, exception re-exports, agent JAR path utilities, and Robot Framework metadata. This violates the single-responsibility principle and makes the codebase difficult to navigate, review, and test in isolation.

The existing `python/JavaGui/keywords/` package already demonstrates the intended pattern -- it contains separate modules for `getters.py`, `tables.py`, `swt_tables.py`, `swt_trees.py`, `swt_getters.py`, and `rcp_keywords.py` as mixin classes. However, the main library classes and their core keywords remain monolithic in `__init__.py`.

A secondary issue: `__version__` in `__init__.py` is hardcoded to `"0.1.0"` (line 145) while `pyproject.toml` declares version `"0.3.0"` and `Cargo.toml` declares `"0.1.0"`.

## Decision

Split `python/JavaGui/__init__.py` into focused modules while preserving all public API imports for backward compatibility:

```
python/JavaGui/
  __init__.py          # ~100 lines: imports, __all__, aliases, metadata
  _version.py          # Single source of truth for version
  _agent.py            # Agent JAR path utilities
  swing.py             # SwingLibrary class (connection, core keywords)
  swt.py               # SwtLibrary class (connection, shell, core keywords)
  rcp.py               # RcpLibrary class (delegates to SwtLibrary + RCP keywords)
  element.py           # SwingElement wrapper class
  keywords/            # (existing) Mixin keyword classes
    __init__.py
    getters.py
    tables.py
    trees.py           # NEW: Extract tree keywords from SwingLibrary
    lists.py           # NEW: Extract list keywords from SwingLibrary
    swing_connection.py # NEW: Extract connection keywords
    swing_clicks.py    # NEW: Extract click/interaction keywords
    swing_waits.py     # NEW: Extract wait keywords
    swing_verification.py # NEW: Extract verification keywords
    swt_getters.py
    swt_tables.py
    swt_trees.py
    rcp_keywords.py
  assertions/          # (existing) Assertion engine integration
  deprecation.py       # (existing) Deprecation utilities
```

The restructured `__init__.py` will re-export all public classes and functions at the package level so that `from JavaGui import SwingLibrary` and `Library    JavaGui.Swing` continue to work unchanged.

## Alternatives Considered

1. **Leave as-is**: Rejected because 3523 lines violates maintainability standards and the project's own CLAUDE.md rule ("Keep files under 500 lines").
2. **Move classes to separate packages** (`JavaGui.swing.SwingLibrary`): Rejected because it would break the Robot Framework import pattern `Library    JavaGui.Swing`.
3. **Split only SwingLibrary, leave SWT/RCP**: Rejected because SwtLibrary has the same problem at ~1400 lines of code plus documentation.

## Consequences

**Positive:**
- Each module stays well under 500 lines
- Easier to navigate, review, and test individual keyword groups
- Clear separation of concerns between toolkits
- Enables parallel development on different toolkit modules
- Docstrings for SwtLibrary and RcpLibrary (which are huge) move to their own files

**Negative:**
- One-time migration effort with risk of circular imports
- Must maintain backward-compatible re-exports in `__init__.py`
- IDE auto-imports may reference internal module paths instead of the package root

## Implementation Notes

- **Key files affected**: `python/JavaGui/__init__.py` (split into ~10 files)
- **Estimated complexity**: Medium -- mostly mechanical extraction, but requires careful import management
- **Risks**: Circular imports between `swing.py` and `element.py`; Robot Framework keyword discovery depends on class hierarchy, which must be preserved exactly
- **Testing**: All existing `tests/python/` tests and `tests/robot/` tests must pass unchanged
- **Migration**: Single atomic commit; no API changes visible to users

---

# ADR-002: SWT/RCP Base Class Extraction

## Status: Proposed

## Context

There is significant code duplication between `SwtLibrary` (in `__init__.py` lines 1809-3175) and `RcpLibrary` (lines 3176-3503). Specifically:

1. **RcpLibrary duplicates ~40 methods from SwtLibrary** including: `connect_to_swt_application`, `connect_to_application`, `disconnect`, `is_connected`, `get_shells`, `get_all_shells`, `activate_shell`, `close_shell`, `find_widget`, `find_widgets`, `click_widget`, `double_click_widget`, `input_text`, `clear_text`, `select_combo_item`, `select_list_item`, `check_button`, `uncheck_button`, `get_table_row_count`, `get_table_cell`, `select_table_row`, `expand_tree_item`, `collapse_tree_item`, `select_tree_item`, `wait_until_widget_exists`, `wait_until_widget_enabled`, `widget_should_be_visible`, `widget_should_be_enabled`, `widget_text_should_be`, `set_timeout`.

2. **`_validate_locator` is triplicated** -- it appears identically in `SwingLibrary` (line 296), `SwtLibrary` (line ~3195 equivalent), and `RcpLibrary` (line 3196).

3. **The Rust layer already models this correctly**: `RcpLibrary` in `src/python/rcp_library.rs` wraps `SwtLibrary` via composition (`swt_lib: SwtLibrary` at line 37). The Python wrappers should mirror this design.

4. RcpLibrary's `__getattr__` fallback (line 3500) attempts to delegate unknown attributes to `self._lib`, but this only works for methods not already defined on RcpLibrary itself, creating a fragile implicit API.

## Decision

Extract a common base class `SwtBaseLibrary` that provides shared SWT widget interaction keywords:

```python
# python/JavaGui/keywords/swt_base.py

class SwtBaseKeywords:
    """Mixin providing common SWT widget interaction keywords.

    Used by both SwtLibrary and RcpLibrary to avoid duplication.
    Expects self._lib to be a Rust SwtLibrary or RcpLibrary instance.
    """

    _timeout: float
    _assertion_timeout: float
    _assertion_interval: float

    @staticmethod
    def _validate_locator(locator) -> None:
        if not isinstance(locator, str):
            return
        if not locator or not locator.strip():
            raise ValueError("Locator cannot be empty or whitespace")

    # Connection keywords
    def connect_to_swt_application(self, app, host="localhost", port=5679, timeout=None):
        ...

    def disconnect(self):
        ...

    # ~35 more shared widget keywords
```

Then `SwtLibrary` and `RcpLibrary` both inherit from `SwtBaseKeywords` plus their respective assertion mixins:

```python
class SwtLibrary(SwtBaseKeywords, SwtGetterKeywords, SwtTableKeywords, SwtTreeKeywords):
    ...

class RcpLibrary(SwtBaseKeywords, SwtGetterKeywords, SwtTableKeywords, SwtTreeKeywords, RcpKeywords):
    ...
```

Remove the `__getattr__` delegation from RcpLibrary entirely -- all keywords should be explicit.

## Alternatives Considered

1. **Keep RcpLibrary as thin delegation layer**: Rejected because it duplicates ~40 methods verbatim and the `__getattr__` fallback hides the actual API surface.
2. **Have RcpLibrary subclass SwtLibrary directly**: Rejected because Robot Framework keyword discovery from multiple inheritance can be fragile with class hierarchies, and we want explicit keyword registration.
3. **Use composition in Python too** (like Rust does): Rejected because Robot Framework discovers keywords by inspecting class methods directly; delegation requires explicit wrapper methods, which is exactly the current duplication problem.

## Consequences

**Positive:**
- Eliminates ~40 duplicated method definitions (~600 lines removed)
- Single `_validate_locator` implementation
- Adding a new SWT widget keyword automatically available in both SwtLibrary and RcpLibrary
- Bug fixes apply to both libraries simultaneously

**Negative:**
- Must carefully manage MRO (Method Resolution Order) with multiple inheritance
- Robot Framework keyword discovery may need validation -- mixins must expose methods with correct signatures

## Implementation Notes

- **Key files affected**: `python/JavaGui/__init__.py` (SwtLibrary, RcpLibrary classes), new `python/JavaGui/keywords/swt_base.py`
- **Estimated complexity**: Medium
- **Risks**: MRO conflicts if mixin classes define overlapping method names
- **Dependencies**: ADR-001 (module restructuring) should be done first or concurrently
- **Testing**: `tests/unit/test_empty_locator_validation.py` tests both SwtLibrary and RcpLibrary and must continue passing

---

# ADR-003: Validation Standardization

## Status: Proposed

## Context

Input validation ordering is inconsistent across the three toolkit libraries, creating different failure modes for the same type of bad input:

1. **SwingLibrary validates locators correctly** in the Python wrapper before calling Rust. For example, `click_element` (line 515) calls `self._validate_locator(locator)` before `self._lib.click_element()`. However, the bare `click` method (line 488-501) does NOT call `_validate_locator` -- it goes directly to `self._lib.click_element()`.

2. **SwtLibrary in Rust checks connection before validating input**: The Rust `SwtLibrary` methods (e.g., in `src/python/swt_library.rs`) check if connected before validating the locator. An empty locator sent while disconnected produces a confusing "Not connected" error instead of "Locator cannot be empty". The Python wrapper's `_validate_locator` calls mitigate this for methods that have the check, but several SWT methods lack it.

3. **Missing `_validate_locator` calls in SwtLibrary/RcpLibrary methods**: Several methods bypass validation entirely:
   - `select_list_item` (line 3299) -- no `_validate_locator` call
   - `get_table_row_count` (line 3312-3314) -- no validation
   - `get_table_cell` (line 3316-3318) -- no validation
   - `select_table_row` (line 3320-3322) -- no validation
   - `expand_tree_item`, `collapse_tree_item`, `select_tree_item` (lines 3325-3335) -- no validation

4. **Validation ordering bug**: The correct order should be: (1) validate input, (2) check connection, (3) execute operation. Currently some paths do (2) then fail, never reaching (1).

## Decision

Standardize validation across all libraries with a consistent ordering contract:

### Validation Order (enforced by convention and testing)

```
1. Input validation (empty locator, invalid type, out-of-range index)
2. Connection state check (connected to application)
3. Element/widget finding (locator resolution)
4. Action execution (click, type, etc.)
```

### Implementation approach

1. Move `_validate_locator` to `SwtBaseKeywords` mixin (per ADR-002) so it is shared.
2. Add `_validate_locator` calls to ALL public keyword methods that accept a locator parameter, in both SwingLibrary and SwtBaseKeywords.
3. Fix SwingLibrary's `click` method (line 488) to call `_validate_locator`.
4. Add validation to all SWT/RCP methods currently missing it (6+ methods identified above).
5. Optionally add Rust-level locator validation as a defense-in-depth measure that produces clear error messages even if the Python layer is bypassed.

### Validation functions to standardize

```python
@staticmethod
def _validate_locator(locator: Union[str, Any]) -> None:
    """Validate locator is not empty/whitespace. Non-string types pass through."""
    if isinstance(locator, str) and (not locator or not locator.strip()):
        raise ValueError("Locator cannot be empty or whitespace")

@staticmethod
def _validate_index(value: int, name: str = "index", minimum: int = 0) -> None:
    """Validate an index parameter."""
    if not isinstance(value, int):
        raise TypeError(f"{name} must be an integer, got {type(value).__name__}")
    if value < minimum:
        raise ValueError(f"{name} must be >= {minimum}, got {value}")
```

## Alternatives Considered

1. **Validate only in Rust**: Rejected because Python callers would get cryptic Rust panic messages instead of clean Python exceptions.
2. **Validate only in Python**: Acceptable as primary layer, but Rust should still validate as defense-in-depth for direct `_core` users.
3. **Use a decorator for validation**: Considered, but the validation parameters differ per method (some take locator, some take path, some take both), making a generic decorator complex.

## Consequences

**Positive:**
- Consistent, predictable error messages regardless of toolkit or method
- Input errors caught immediately with clear messages before any network/RPC calls
- Empty locators can never reach the Java agent, preventing agent-side crashes
- Existing `tests/unit/test_empty_locator_validation.py` tests will pass (they currently may fail for SWT methods)

**Negative:**
- Small additional overhead per keyword call (negligible -- measured at <0.01ms in test_empty_locator_validation.py)
- Must audit every public keyword method to ensure validation is present

## Implementation Notes

- **Key files affected**: `python/JavaGui/__init__.py` (all three library classes), potentially `src/python/swt_library.rs`
- **Estimated complexity**: Low -- mechanical addition of `_validate_locator` calls
- **Risks**: None significant; validation is additive and non-breaking
- **Testing**: Extend `tests/unit/test_empty_locator_validation.py` to cover all currently-unprotected methods
- **Dependencies**: Best done alongside or after ADR-002 (base class extraction)

---

# ADR-004: Error Handling Strategy

## Status: Proposed

## Context

The codebase has multiple error handling anti-patterns:

1. **29 swallowed exceptions** (try-except-pass patterns): Found across the Python codebase, particularly in:
   - `python/JavaGui/keywords/swt_trees.py`: Multiple bare `except Exception: pass` blocks (lines 144, 153-154, 216-218, 265-266, 296-297). For example, `get_swt_tree_node_count` at line 136 calls `self._lib.expand_tree_item()` wrapped in try/except that silently swallows all errors including connection failures and invalid locators.
   - `python/JavaGui/keywords/rcp_keywords.py`: `perspective_should_be_active` (line 620-621) catches all exceptions during the poll loop, masking real errors.

2. **`get_swt_tree_item_text` returns fabricated data instead of actual text**: At `swt_trees.py` line 213-223, the `get_text()` inner function splits the path by `/` or `|` and returns the last component -- it never actually queries the widget for its text. This is a functional bug masked by the error handling pattern.

3. **`get_swt_tree_node_count` counts the wrong thing**: At `swt_trees.py` lines 136-165, when `parent_path` is provided, it calls `get_selected_tree_nodes(locator)` which returns selected nodes, not children of the given path. The count has no relation to child node count.

4. **114 `print()` statements** used instead of Robot Framework's `robot.api.logger`: Print statements are invisible in Robot Framework log files and pollute stdout during test execution.

5. **Mixed exception types**: The Rust layer defines a unified exception hierarchy (see `src/lib.rs` lines 33-56) with `JavaGuiError` as base, but the Python wrappers still reference legacy names like `SwingConnectionError`, `SwingError`, etc.

## Decision

### Phase 1: Fix Critical Bugs (P0/P1)

1. Fix `get_swt_tree_item_text` to actually retrieve widget text:
   ```python
   def get_text():
       return self._lib.get_tree_item_text(locator, path)
   ```

2. Fix `get_swt_tree_node_count` to count children, not selected nodes:
   ```python
   def get_count():
       if parent_path:
           return self._lib.get_tree_node_child_count(locator, parent_path)
       else:
           return self._lib.get_tree_root_count(locator)
   ```

### Phase 2: Replace Swallowed Exceptions

Replace all try-except-pass blocks with one of three patterns:

**Pattern A: Log and continue** (for best-effort operations like pre-expanding trees):
```python
try:
    self._lib.expand_tree_item(locator, parent_path)
except Exception as e:
    logger.debug(f"Could not expand tree item '{parent_path}': {e}")
```

**Pattern B: Re-raise with context** (for operations that must succeed):
```python
try:
    self._lib.select_tree_item(locator, path)
except Exception as e:
    raise RuntimeError(f"Failed to select tree item '{path}' in '{locator}': {e}") from e
```

**Pattern C: Catch specific exceptions** (for expected failure modes):
```python
try:
    exists = self._lib.tree_node_exists(locator, path)
except ElementNotFoundError:
    exists = False
except Exception as e:
    raise RuntimeError(f"Unexpected error checking tree node: {e}") from e
```

### Phase 3: Replace print() with logger

Replace all 114 `print()` statements with `robot.api.logger`:
```python
from robot.api import logger

# Instead of: print(f"Connected to {host}:{port}")
logger.info(f"Connected to {host}:{port}")

# Instead of: print(f"DEBUG: tree structure = {tree}")
logger.debug(f"Tree structure: {tree}")
```

## Alternatives Considered

1. **Fix only the functional bugs, leave error handling**: Rejected because swallowed exceptions mask real failures in CI, making tests appear to pass when they should fail.
2. **Use Python warnings module for suppressed errors**: Rejected because Robot Framework has its own logging that integrates with test reports.
3. **Add a global exception handler decorator**: Rejected as over-engineering; explicit handling per method is clearer and more maintainable.

## Consequences

**Positive:**
- `get_swt_tree_item_text` and `get_swt_tree_node_count` return correct data
- All error conditions produce visible diagnostic output in Robot Framework logs
- No more silent failures that corrupt test results
- Consistent logging through Robot Framework's logger

**Negative:**
- Tests that currently "pass" due to swallowed exceptions may start failing (exposing real bugs)
- Log output volume increases (mitigated by using debug level for verbose output)

## Implementation Notes

- **Key files affected**: `python/JavaGui/keywords/swt_trees.py` (most critical), `python/JavaGui/keywords/rcp_keywords.py`, scattered `print()` across all Python files
- **Estimated complexity**: Medium for bug fixes, Low for print->logger replacement
- **Risks**: Fixing `get_swt_tree_item_text` and `get_swt_tree_node_count` changes observable behavior -- existing Robot Framework tests may need updating if they relied on the buggy behavior
- **Testing**: Need Robot Framework tests with a real SWT app to validate tree keyword fixes

---

# ADR-005: Rust Code Hygiene

## Status: Proposed

## Context

The Rust codebase (~25K LOC in `src/`) has accumulated several categories of code quality issues:

### P0: never_loop Clippy Errors

Two `for` loops in `src/locator/parser.rs` that immediately return on the first iteration, which is a clippy `never_loop` error:

1. **`parse_match_operator`** (line 462): `for inner in pair.into_inner() { return match ... }` -- this iterates exactly once and returns.
2. **`parse_combinator`** (line 754): Same pattern -- `for inner in pair.into_inner() { return match ... }`.

Both should use `pair.into_inner().next()` instead of a for loop.

### P1: Dead Code

12+ unused functions and fields identified:

- `next_request_id` field/counter in `ConnectionState` (swing_library.rs line 72) -- `request_id` is declared but incremented inconsistently
- `parse_css` and related functions in locator module -- superseded by the unified Pest parser
- 9 methods in `swing_library.rs` that are defined but never called from Python wrappers

### P2: Clippy Warnings (81 total)

Categories include:
- `needless_return` -- explicit `return` where implicit return suffices
- `redundant_closure` -- closures that just call a function: `.map(|x| f(x))` vs `.map(f)`
- `clone_on_ref_ptr` -- `.clone()` on `Arc` should use `Arc::clone(&x)`
- `single_match` -- `match` with one arm + wildcard should be `if let`
- `manual_map` -- manual `match` on Option that should use `.map()`

### P2: Integer Truncation Casts (29 instances)

Casts like `value as i32` from `i64` or `usize` that silently truncate on overflow. These occur in:
- Component ID handling
- Table row/column indexing
- Element property conversions

### Derive Implementation Opportunities

Several structs manually implement `Clone`, `Default`, or `Debug` where `#[derive(...)]` would suffice and be more maintainable.

## Decision

### Phase 1: Fix P0 Clippy Errors

Replace the `never_loop` patterns in `src/locator/parser.rs`:

```rust
// Before (line 462):
fn parse_match_operator(pair: pest::iterators::Pair<Rule>) -> Result<MatchOperator, ParseError> {
    for inner in pair.into_inner() {
        return match inner.as_rule() {
            Rule::equals => Ok(MatchOperator::Equals),
            // ...
        };
    }
    Err(...)
}

// After:
fn parse_match_operator(pair: pest::iterators::Pair<Rule>) -> Result<MatchOperator, ParseError> {
    let inner = pair.into_inner().next().ok_or_else(|| ParseError::new(
        "Missing operator".to_string(),
        ParseErrorKind::InvalidAttribute,
        0,
    ))?;
    match inner.as_rule() {
        Rule::equals => Ok(MatchOperator::Equals),
        // ...
    }
}
```

Apply the same fix to `parse_combinator` at line 754.

### Phase 2: Remove Dead Code

1. Remove unused functions identified by `cargo clippy -- -W dead_code`
2. Remove unused fields (e.g., `next_request_id` if truly unused)
3. Add `#[allow(dead_code)]` only where code is intentionally kept for future use, with a comment explaining why

### Phase 3: Fix Clippy Warnings

Run `cargo clippy --all-targets -- -D warnings` and fix all 81 warnings, prioritizing:
1. Correctness warnings (integer truncation)
2. Performance warnings (unnecessary clones)
3. Style warnings (needless_return, etc.)

### Phase 4: Integer Safety

Replace truncating casts with checked conversions:

```rust
// Before:
let row_index = row as i32;

// After:
let row_index: i32 = row.try_into().map_err(|_| {
    SwingError::new("Row index too large for i32")
})?;
```

## Alternatives Considered

1. **Suppress clippy warnings with allow attributes**: Rejected because the warnings indicate real issues (especially `never_loop` and integer truncation).
2. **Fix only P0, defer the rest**: Acceptable as a phased approach, but the clippy warnings should be addressed before they accumulate further.
3. **Enable clippy as CI gate immediately**: Premature -- fix existing violations first, then add the gate (see ADR-008).

## Consequences

**Positive:**
- Zero clippy warnings enables CI clippy gate (ADR-008)
- Integer truncation bugs caught at compile time
- Reduced binary size from dead code removal
- Cleaner codebase for contributors

**Negative:**
- Risk of accidentally removing code that IS used but not detected by static analysis (e.g., code called via PyO3 dynamic dispatch)
- Integer safety conversions add error handling paths

## Implementation Notes

- **Key files affected**: `src/locator/parser.rs` (P0), `src/python/swing_library.rs` (dead code), all `.rs` files for clippy fixes
- **Estimated complexity**: Low for P0 (2 functions), Medium for full clippy compliance
- **Risks**: PyO3 `#[pymethods]` are called dynamically from Python -- `dead_code` analysis may incorrectly flag them. Use `#[allow(dead_code)]` selectively for PyO3 entry points.
- **Testing**: `cargo test` must pass; Python integration tests must pass after dead code removal

---

# ADR-006: Test Infrastructure Modernization

## Status: Proposed

## Context

The test infrastructure has multiple issues spanning three categories:

### Failing Tests (4 identified)

1. **SWT empty locator tests** (`tests/unit/test_empty_locator_validation.py`): These tests import `SwtLibrary`, `SwingLibrary`, and `RcpLibrary` directly from `JavaGui` (line 9) and attempt to instantiate them. If the Rust core is not built/available, these imports fail. The test is also placed in `tests/unit/` instead of `tests/python/`, outside the configured `testpaths`.

2. **`test_empty_locator.py`** (`tests/test_empty_locator.py`): Located at the project root's `tests/` directory (line 27 in test inventory), this is NOT a pytest file -- it appears to be a standalone script, but its location suggests it was meant to be a test.

3. **`test_list_shells_and_retry`** (`tests/unit/test_list_shells_and_retry.py`): Located in `tests/unit/`, outside the configured pytest testpaths of `tests/python/`.

### Flaky Benchmark Tests (3 identified)

4. **`tests/python/test_benchmark.py`**: Uses hardcoded microsecond thresholds for performance assertions. On slower CI machines or under load, these timing-based assertions fail intermittently. Example: assertion retry benchmarks expect sub-microsecond performance.

### Fragmented Test Configuration

- **pytest config** in `pyproject.toml` (lines 76-80): `testpaths = ["tests/python"]`, `addopts = "-v --tb=short"`.
- Tests in `tests/unit/` are never discovered by default `pytest` invocation.
- Tests in `tests/` root are never discovered.
- No pytest markers defined for categorizing tests (unit, integration, benchmark, smoke).
- **Robot Framework tests**: 37 `.robot` files across `tests/robot/swing/`, `tests/robot/swt/`, and `tests/robot/rcp/`. These use `Force Tags` (104 instances) which is deprecated in RF 7.0 -- should be `Test Tags`.
- **140 instances of `Set Variable`** in Robot Framework tests -- deprecated in favor of `VAR` syntax.

### SWT Test File Numbering Collisions

The `tests/robot/swt/` directory has duplicate numbers:
- `02_shells.robot` AND `02_widgets.robot` (both numbered 02)
- `03_tables.robot` AND `03_widget_finding.robot` (both numbered 03)
- `04_trees.robot` AND `04_clicks.robot` (both numbered 04)

## Decision

### Phase 1: Unify Test Layout and Configuration

Update `pyproject.toml` to discover all test directories:

```toml
[tool.pytest.ini_options]
testpaths = ["tests/python", "tests/unit"]
python_files = ["test_*.py"]
python_functions = ["test_*"]
addopts = "-v --tb=short"
markers = [
    "unit: Unit tests (no external dependencies)",
    "integration: Integration tests (require built library)",
    "benchmark: Performance benchmark tests",
    "smoke: Quick smoke tests",
    "slow: Slow tests (>10 seconds)",
]
```

### Phase 2: Fix Failing Tests

1. Move `tests/test_empty_locator.py` to `tests/python/test_empty_locator.py` or delete if redundant with `tests/unit/test_empty_locator_validation.py`.
2. Guard `tests/unit/test_empty_locator_validation.py` with `pytest.importorskip("JavaGui._core")` so it skips gracefully when the Rust core is not built.
3. Apply the same guard to `tests/unit/test_list_shells_and_retry.py`.

### Phase 3: Fix Flaky Benchmarks

Replace absolute timing thresholds with relative comparisons or statistical bounds:

```python
# Before:
assert mean_time_us < 10.0, f"Too slow: {mean_time_us}us"

# After:
# Use a generous multiplier over the warmup baseline
assert mean_time_us < baseline_time_us * 20, (
    f"Performance regression: {mean_time_us}us vs baseline {baseline_time_us}us"
)
```

Mark benchmarks with `@pytest.mark.benchmark` so they can be excluded in CI with `-m "not benchmark"`.

### Phase 4: Fix Robot Framework Deprecations

- Replace `Force Tags` with `Test Tags` (104 instances)
- Replace `Set Variable` with `VAR` syntax (140 instances)

### Phase 5: Fix SWT Test Numbering

Renumber `tests/robot/swt/` files to avoid collisions:
```
01_connection.robot
02_shells.robot
03_widget_finding.robot
04_widgets.robot
05_text_input.robot
06_clicks.robot
07_tables.robot
08_trees.robot
09_selection.robot
```

## Alternatives Considered

1. **Keep tests/unit/ as a separate pytest run**: Rejected because it creates split test runs and makes CI more complex.
2. **Remove benchmark tests entirely**: Rejected because they catch performance regressions; they just need better threshold handling.
3. **Ignore Robot Framework deprecations**: Rejected because RF 8.0 may remove these, and warnings clutter output.

## Consequences

**Positive:**
- Single `pytest` command runs all Python tests
- No more failing tests due to import errors or wrong directories
- Benchmark tests stable across different hardware
- Robot Framework tests compatible with RF 7.0+ without deprecation warnings

**Negative:**
- Robot Framework deprecation fixes (244 instances) are tedious but mechanical
- SWT test renumbering may cause merge conflicts with in-flight PRs

## Implementation Notes

- **Key files affected**: `pyproject.toml`, `tests/unit/*.py`, `tests/test_empty_locator.py`, `tests/python/test_benchmark.py`, 37 `.robot` files
- **Estimated complexity**: Low per item, Medium in aggregate (244 RF deprecation fixes)
- **Risks**: Renumbering SWT tests could break CI if test paths are hardcoded anywhere
- **Testing**: Run full `pytest` and `robot` suites after changes

---

# ADR-007: Git and Build Hygiene

## Status: Proposed

## Context

Several build and version management issues need attention:

### Version Mismatch

- `pyproject.toml` declares version `"0.3.0"` (line 6)
- `Cargo.toml` declares version `"0.1.0"` (line 3)
- `python/JavaGui/__init__.py` hardcodes `__version__ = "0.1.0"` (line 145)

Three different versions for the same package create confusion about the actual release state.

### Wrong GitHub URLs

- `pyproject.toml` lines 50-53 reference `https://github.com/robotframework/robotframework-javagui` -- this is the `robotframework` org, not the actual `manykarim` repository.
- `Cargo.toml` line 8 has the same wrong URL.

### Build Artifacts in Git

The `.gitignore` (line 48) covers `agent/target/` and `demo/target/` but NOT `tests/apps/*/target/`. The CI workflow builds Java test apps with Maven (`cd swing && mvn clean package`), and these `target/` directories may have been committed.

Additionally, `.claude-flow/` directories (line 111 in .gitignore) suggests these were committed at some point and then ignored.

### Duplicate dev Dependencies

`pyproject.toml` has two sets of dev dependencies:
1. `[project.optional-dependencies] dev` (lines 39-47): `pytest`, `pytest-cov`, `black`, `mypy`, `ruff`, `pyyaml`
2. `[dependency-groups] dev` (lines 82-89): `invoke`, `mypy`, `maturin`, `robocop`, `ruff`

`mypy` and `ruff` appear in both groups. The `[dependency-groups]` section is PEP 735 (newer standard), while `[project.optional-dependencies]` is PEP 621 (established standard). Having both is confusing.

### Java Version Inconsistency

CI uses Java 17 (ci.yml line 49), but test app `pom.xml` files may target Java 11. The SWT dependency version also varies between test apps (3.125 vs 3.127).

## Decision

### 1. Sync Versions

Establish a single source of truth for the version:

- Set `pyproject.toml` version to `"0.3.0"` (already correct)
- Update `Cargo.toml` to `version = "0.3.0"`
- Replace `__version__ = "0.1.0"` in `__init__.py` with:
  ```python
  from importlib.metadata import version as _get_version
  try:
      __version__ = _get_version("robotframework-javagui")
  except Exception:
      __version__ = "0.3.0"
  ```

### 2. Fix GitHub URLs

Update both `pyproject.toml` and `Cargo.toml` to reference the actual repository:
```
https://github.com/manykarim/robotframework-javaui
```

### 3. Fix .gitignore

Add test app build artifacts:
```gitignore
# Java test app build artifacts
tests/apps/*/target/
```

Remove any committed `target/` directories:
```bash
git rm -r --cached tests/apps/swing/target/ tests/apps/swt/target/ tests/apps/rcp-mock/target/
```

### 4. Consolidate dev Dependencies

Keep `[dependency-groups]` (PEP 735) as the primary dev dependency specification and remove the duplicate `[project.optional-dependencies] dev` section. PEP 735 is the modern standard supported by `uv`.

```toml
[dependency-groups]
dev = [
    "invoke>=2.2.1",
    "maturin>=1.4,<2.0",
    "mypy>=1.14.1",
    "pytest>=7.0",
    "pytest-cov>=4.0",
    "pyyaml>=6.0",
    "robotframework-robocop>=5.5.0",
    "ruff>=0.14.11",
]
```

### 5. Standardize Java Versions

Ensure all test app `pom.xml` files target Java 17 to match CI.

## Alternatives Considered

1. **Use Cargo.toml as version source**: Rejected because `pyproject.toml` is the Python packaging standard and is more discoverable for Python users.
2. **Keep both optional-dependencies and dependency-groups**: Rejected because having the same tools in two places causes version drift (ruff 0.1 vs 0.14.11).
3. **Keep Java 11 compatibility**: Rejected because Java 11 is past end of public updates and CI already uses 17.

## Consequences

**Positive:**
- Single authoritative version number
- Correct GitHub URLs for issue tracking, documentation links
- Clean git history without binary artifacts
- No dependency confusion between two dev groups

**Negative:**
- Version bump in Cargo.toml triggers a Rust recompile
- Removing committed build artifacts creates a large git diff in the cleanup commit
- Consolidating to PEP 735 `[dependency-groups]` may not work with older pip (but `uv` supports it)

## Implementation Notes

- **Key files affected**: `pyproject.toml`, `Cargo.toml`, `python/JavaGui/__init__.py`, `.gitignore`, `tests/apps/*/pom.xml`
- **Estimated complexity**: Low -- all changes are configuration/metadata
- **Risks**: Changing Cargo.toml version may affect crate publishing; confirm no published crate exists first
- **Testing**: `uv sync --group dev` must resolve correctly; `maturin develop` must build; CI must pass

---

# ADR-008: CI/CD Pipeline Improvements

## Status: Proposed

## Context

The CI pipeline (`.github/workflows/ci.yml`) has several bugs and missing capabilities:

### Bug: Duplicate Swing Test Path on macOS

Line 170:
```yaml
- name: Run Robot Framework tests (macOS only, no RCP)
  if: runner.os == 'macOS'
  run: uv run robot --outputdir tests/robot/output tests/robot/swing tests/robot/swing tests/robot/swt
```

`tests/robot/swing` appears TWICE, causing Swing tests to run twice on macOS while SWT tests run once. This wastes CI time and may cause file-locking conflicts with test output.

### Missing: Lint Gates

The CI pipeline has no linting steps. It builds, tests, but never runs:
- `cargo clippy` (Rust linting)
- `ruff check` (Python linting)
- `mypy` (Python type checking)
- `robocop` (Robot Framework linting)

This allows lint violations to accumulate (currently: 81 clippy warnings, 70 unused Python imports, 552 robocop violations).

### Missing: Rust Test Execution

CI never runs `cargo test`. The Rust crate has test modules in `src/core/tests.rs`, `src/python/tests.rs`, and `src/locator/unified_tests.rs` that are never executed in CI.

### Limited Test Matrix

The current matrix tests Python 3.10, 3.12, and 3.14 on three OSes. However:
- Python 3.14 frequently fails and falls back to 3.13 (lines 34-38)
- Python 3.8/3.9 are declared as supported in `pyproject.toml` classifiers but never tested
- No Rust version testing (only uses `stable`)

### Missing: Artifact Caching

Cargo caching exists (lines 55-64) but Python package caching does not. `uv sync` downloads and installs packages on every run.

### Missing: Separate Lint and Test Jobs

All steps are in a single monolithic job (`build-test`). This means a lint failure (if added) would prevent test results, and vice versa. Separating lint, build, and test into distinct jobs enables:
- Faster feedback (lint fails in seconds, not after a 5-minute build)
- Parallel execution
- Clearer failure attribution

## Decision

### 1. Fix macOS Duplicate Path Bug

```yaml
- name: Run Robot Framework tests (macOS only, no RCP)
  if: runner.os == 'macOS'
  run: uv run robot --outputdir tests/robot/output tests/robot/swing tests/robot/swt
```

### 2. Add Lint Job (Fast, Parallel)

```yaml
lint:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
      with:
        components: clippy
    - uses: astral-sh/setup-uv@v2
    - run: uv python install 3.12
    - run: uv sync --group dev

    # Rust linting
    - run: cargo clippy --all-targets -- -D warnings

    # Python linting
    - run: uv run ruff check python/ tests/

    # Python type checking
    - run: uv run mypy python/JavaGui/ --ignore-missing-imports

    # Robot Framework linting (warnings only initially)
    - run: uv run robocop --configure return_status:quality_gate:W=-1 tests/robot/
```

### 3. Add Rust Test Step

Add to the build-test job:
```yaml
- name: Run Rust tests
  run: cargo test --all-features
```

### 4. Trim Test Matrix

```yaml
matrix:
  os: [ubuntu-latest, windows-latest, macos-14]
  python: ["3.10", "3.12"]
include:
  - os: ubuntu-latest
    python: "3.13"
```

Drop Python 3.14 (not released) and 3.8/3.9 (if we're willing to drop support), or add them back explicitly. Remove the 3.14->3.13 fallback hack.

### 5. Add UV Cache

```yaml
- uses: astral-sh/setup-uv@v2
  with:
    version: latest
    enable-cache: true
```

### 6. Separate Jobs with Dependencies

```yaml
jobs:
  lint:
    ...  # Fast, runs first

  build:
    needs: lint
    ...  # Build wheels and test apps

  test-python:
    needs: build
    ...  # Python tests

  test-robot:
    needs: build
    ...  # Robot Framework tests

  test-rust:
    needs: lint
    ...  # cargo test (independent of Python build)
```

## Alternatives Considered

1. **Add linting as pre-commit hooks only**: Rejected because developers can skip hooks with `--no-verify`, and CI must be the authoritative gate.
2. **Keep single monolithic job**: Rejected because it wastes CI time -- lint failures should be caught in seconds, not after building Java test apps.
3. **Use nightly Rust**: Rejected because stable is sufficient and nightly introduces instability.

## Consequences

**Positive:**
- macOS CI no longer runs Swing tests twice
- Lint violations caught before merge
- Rust tests run in CI for the first time
- Faster CI feedback (lint job completes in ~1 minute)
- Better cache utilization reduces dependency installation time

**Negative:**
- Existing 81 clippy warnings, 70 unused imports, and 552 robocop violations must be fixed before enabling `-D warnings` gates (or initially use warning-level thresholds)
- Separated jobs add slight overhead from repeated checkout/setup steps (mitigated by caching)
- More complex workflow file

## Implementation Notes

- **Key files affected**: `.github/workflows/ci.yml`
- **Estimated complexity**: Medium -- workflow restructuring with new jobs
- **Risks**: New lint gates may initially block all PRs until existing violations are fixed. Recommendation: add lint job as `continue-on-error: true` initially, then fix violations, then make it blocking.
- **Dependencies**: ADR-005 (Rust code hygiene) should be done before enabling `cargo clippy -D warnings` gate
- **Testing**: Validate the new workflow on a feature branch before merging to main
