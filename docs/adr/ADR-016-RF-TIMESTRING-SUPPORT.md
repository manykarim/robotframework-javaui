# ADR-016: Robot Framework Time String Support

| ADR ID | ADR-016 |
|--------|---------|
| Title | Robot Framework Time String Support |
| Status | Proposed |
| Date | 2026-02-25 |
| Authors | Architecture Team |
| Related | ADR-005 (Error Handling), ADR-007 (Unified Keyword API), ADR-010 (AssertionEngine Integration) |

## Context

All timeout parameters in the library currently accept only numeric float values representing seconds. This is enforced at two levels:

1. **Python layer** (`python/JavaGui/__init__.py`) -- signatures typed as `Optional[float]`:
   ```python
   def wait_until_element_exists(
       self,
       locator: str,
       timeout: Optional[float] = None,
   ) -> None:
       timeout_val = timeout if timeout is not None else self._timeout
       self._lib.wait_until_element_exists(locator, timeout_val)
   ```

2. **Rust layer** (`src/python/base_library.rs`) -- the `py_to_f64` helper extracts an `i64` or `f64`:
   ```rust
   pub fn py_to_f64(py: Python<'_>, obj: Option<PyObject>) -> Option<f64> {
       obj.and_then(|o| {
           if let Ok(i) = o.extract::<i64>(py) {
               Some(i as f64)
           } else {
               o.extract::<f64>(py).ok()
           }
       })
   }
   ```

Robot Framework's convention -- followed by SeleniumLibrary, Browser Library, AppiumLibrary, and the built-in library -- is to accept **time strings** in addition to plain numeric seconds. The standard utility `robot.utils.timestr_to_secs()` converts various human-readable formats into float seconds:

| Input | Result (seconds) |
|-------|-----------------|
| `10` | 10.0 |
| `10.5` | 10.5 |
| `3s` | 3.0 |
| `500ms` | 0.5 |
| `500 milliseconds` | 0.5 |
| `1 min` | 60.0 |
| `1 minute 30 seconds` | 90.0 |
| `1:30` | 90.0 |
| `1h` | 3600.0 |
| `${TIMEOUT}` | (resolved by RF before reaching keyword) |

Users migrating from SeleniumLibrary or Browser Library expect this to work:

```robot
*** Settings ***
Library    JavaGui.Swing    timeout=30s

*** Test Cases ***
Example With Time Strings
    Set Timeout    1 min
    Wait Until Element Exists    JButton#submit    timeout=500ms
    Set Assertion Timeout    3s
```

Currently these calls raise `TypeError` or silently fail because `"30s"` cannot be extracted as `f64`.

### Decision Drivers

- Conform to Robot Framework ecosystem conventions
- Minimize changes to the Rust core (no RF dependency in Rust)
- Maintain full backwards compatibility with existing numeric values
- Provide clear error messages for invalid time strings

## Decision

We will add `robot.utils.timestr_to_secs()` parsing at the **Python layer** for every parameter that represents a duration. The Rust layer will continue to accept `f64` values unchanged. A shared helper function `_parse_timeout` will normalize any time string input into float seconds before passing it down to Rust.

### Affected Keywords

The following keywords and parameters accept timeout/interval/duration values and must be updated:

| # | Keyword / Parameter | Class(es) | Current Type | Location |
|---|---------------------|-----------|-------------|----------|
| 1 | `Wait For Element` (`timeout`) | `SwingLibrary` | `Optional[float]` | `__init__.py:1318` |
| 2 | `Wait Until Element Exists` (`timeout`) | `SwingLibrary` | `Optional[float]` | `__init__.py:433` |
| 3 | `Wait Until Element Does Not Exist` (`timeout`) | `SwingLibrary` | `Optional[float]` | `__init__.py:454` |
| 4 | `Wait Until Element Is Visible` (`timeout`) | `SwingLibrary` | `Optional[float]` | `__init__.py:787` |
| 5 | `Wait Until Element Is Enabled` (`timeout`) | `SwingLibrary` | `Optional[float]` | `__init__.py:809` |
| 6 | `Set Timeout` (`timeout`) | `SwingLibrary`, `SwtLibrary`, `RcpLibrary` | `float` | `__init__.py:1129,2572,3312` |
| 7 | `Set Assertion Timeout` (`timeout`) | `GetterKeywords` (mixin) | `float` | `keywords/getters.py:408` |
| 8 | `Set Assertion Interval` (`interval`) | `GetterKeywords` (mixin) | `float` | `keywords/getters.py:425` |
| 9 | Library `__init__` (`timeout`) | `SwingLibrary`, `SwtLibrary`, `RcpLibrary` | `float` | `__init__.py:259,2323,3125` |
| 10 | Alias: `Wait Until Element Visible` | `SwingLibrary` | `Optional[float]` | `__init__.py:1302` (delegates to #4) |
| 11 | Alias: `Wait Until Element Enabled` | `SwingLibrary` | `Optional[float]` | `__init__.py:1310` (delegates to #5) |
| 12 | Alias: `Wait Until Element Contains` | `SwingLibrary` | `Optional[float]` | `__init__.py:1341` |

Alias keywords (#10, #11) delegate to their canonical counterparts, so they inherit the conversion automatically once the canonical keyword is updated. They are listed here for completeness.

### Implementation Approach

#### 1. Shared Conversion Helper

Add a module-level helper in `python/JavaGui/__init__.py` (or a dedicated `python/JavaGui/timeutil.py` if preferred for testability):

```python
from typing import Optional, Union

from robot.utils import timestr_to_secs


def _parse_timeout(
    value: Union[str, int, float, None],
    default: Optional[float] = None,
) -> Optional[float]:
    """Convert a Robot Framework time string to float seconds.

    Accepts any format supported by ``robot.utils.timestr_to_secs``:
    plain numbers, ``3s``, ``500ms``, ``1 min``, ``1:30``, etc.

    Args:
        value: Time string, numeric value, or None.
        default: Value to return when ``value`` is None.

    Returns:
        Float seconds, or ``default`` if ``value`` is None.

    Raises:
        ValueError: If ``value`` is a string that cannot be parsed
            as a valid time expression.
    """
    if value is None:
        return default
    if isinstance(value, (int, float)):
        return float(value)
    try:
        return timestr_to_secs(value)
    except ValueError as err:
        raise ValueError(
            f"Invalid time string '{value}'. "
            f"Expected a number (seconds) or Robot Framework time string "
            f"like '3s', '500ms', '1 min', '1:30'. "
            f"Original error: {err}"
        ) from err
```

#### 2. Type Annotation Changes

All affected parameters change their type annotation from `float` or `Optional[float]` to `Union[str, float]` or `Optional[Union[str, float]]` so that Robot Framework's argument inspection correctly reports the accepted types:

```python
# Before
def __init__(self, timeout: float = 10.0, ...) -> None:

# After
def __init__(self, timeout: Union[str, float] = 10.0, ...) -> None:
```

```python
# Before
def wait_until_element_exists(
    self,
    locator: str,
    timeout: Optional[float] = None,
) -> None:

# After
def wait_until_element_exists(
    self,
    locator: str,
    timeout: Optional[Union[str, float]] = None,
) -> None:
```

#### 3. Code Changes -- Before/After

**`SwingLibrary.__init__`** (`python/JavaGui/__init__.py`):

```python
# BEFORE
def __init__(
    self,
    timeout: float = 10.0,
    poll_interval: float = 0.5,
    screenshot_directory: str = ".",
) -> None:
    ...
    self._lib = _SwingLibrary(
        timeout=timeout,
        poll_interval=poll_interval,
        screenshot_directory=screenshot_directory,
    )
    self._timeout = timeout

# AFTER
def __init__(
    self,
    timeout: Union[str, float] = 10.0,
    poll_interval: Union[str, float] = 0.5,
    screenshot_directory: str = ".",
) -> None:
    ...
    timeout_secs = _parse_timeout(timeout, default=10.0)
    poll_secs = _parse_timeout(poll_interval, default=0.5)
    self._lib = _SwingLibrary(
        timeout=timeout_secs,
        poll_interval=poll_secs,
        screenshot_directory=screenshot_directory,
    )
    self._timeout = timeout_secs
```

**`wait_until_element_exists`** (`python/JavaGui/__init__.py`):

```python
# BEFORE
def wait_until_element_exists(
    self,
    locator: str,
    timeout: Optional[float] = None,
) -> None:
    timeout_val = timeout if timeout is not None else self._timeout
    self._lib.wait_until_element_exists(locator, timeout_val)

# AFTER
def wait_until_element_exists(
    self,
    locator: str,
    timeout: Optional[Union[str, float]] = None,
) -> None:
    timeout_val = _parse_timeout(timeout, default=self._timeout)
    self._lib.wait_until_element_exists(locator, timeout_val)
```

The same pattern applies to `wait_until_element_does_not_exist`, `wait_until_element_is_visible`, `wait_until_element_is_enabled`, `wait_for_element`, and `wait_until_element_contains`.

**`set_timeout`** (`python/JavaGui/__init__.py`):

```python
# BEFORE
def set_timeout(self, timeout: float) -> None:
    self._timeout = timeout
    self._lib.set_timeout(timeout)

# AFTER
def set_timeout(self, timeout: Union[str, float]) -> None:
    timeout_secs = _parse_timeout(timeout)
    self._timeout = timeout_secs
    self._lib.set_timeout(timeout_secs)
```

**`set_assertion_timeout`** (`python/JavaGui/keywords/getters.py`):

```python
# BEFORE
def set_assertion_timeout(self, timeout: float) -> float:
    old = self._assertion_timeout
    self._assertion_timeout = timeout
    return old

# AFTER
def set_assertion_timeout(self, timeout: Union[str, float]) -> float:
    from JavaGui import _parse_timeout
    timeout_secs = _parse_timeout(timeout)
    old = self._assertion_timeout
    self._assertion_timeout = timeout_secs
    return old
```

**`set_assertion_interval`** (`python/JavaGui/keywords/getters.py`):

```python
# BEFORE
def set_assertion_interval(self, interval: float) -> float:
    old = self._assertion_interval
    self._assertion_interval = interval
    return old

# AFTER
def set_assertion_interval(self, interval: Union[str, float]) -> float:
    from JavaGui import _parse_timeout
    interval_secs = _parse_timeout(interval)
    old = self._assertion_interval
    self._assertion_interval = interval_secs
    return old
```

#### 4. No Rust Changes Required

The Rust layer already accepts `f64` (or `Option<f64>`) for all timeout parameters. The `py_to_f64` function in `src/python/base_library.rs` handles `i64` and `f64` extraction. Since the Python layer will always pass a converted float, no Rust modifications are needed.

The `set_timeout` in the base Rust library (`src/python/base_library.rs:895`) accepts a `PyObject` and uses `py_to_f64`, which already handles numeric types. The `SwingLibrary` Rust implementation (`src/python/swing_library.rs:1709`) accepts `f64` directly. Both will continue to work because they only receive pre-converted floats from Python.

### Robot Framework Usage After Change

```robot
*** Settings ***
Library    JavaGui.Swing    timeout=30s

*** Variables ***
${TIMEOUT}    1 min

*** Test Cases ***
Time String In Library Import
    [Documentation]    Library-level timeout accepts time strings
    Log    Library initialized with 30s timeout

Wait With Time String
    Wait Until Element Exists    JButton#submit    timeout=500ms
    Wait Until Element Is Visible    JLabel#status    timeout=3s
    Wait Until Element Is Enabled    JButton#next    timeout=1 min

Set Timeout With Time String
    Set Timeout    1 min
    Set Timeout    30s
    Set Timeout    ${TIMEOUT}

Assertion Timeouts With Time Strings
    Set Assertion Timeout    3s
    Set Assertion Interval    200ms
    Get Text    JLabel#status    ==    Ready    timeout=5s

Plain Numbers Still Work
    Wait Until Element Exists    JButton#submit    timeout=10
    Set Timeout    30
    Set Assertion Timeout    5.0
```

## Testing Plan

### Unit Tests

Add to `tests/python/test_timestring.py`:

```python
import pytest
from JavaGui import _parse_timeout


class TestParseTimeout:
    """Tests for RF time string parsing."""

    def test_none_returns_default(self):
        assert _parse_timeout(None, default=10.0) == 10.0

    def test_none_returns_none_without_default(self):
        assert _parse_timeout(None) is None

    def test_int_value(self):
        assert _parse_timeout(10) == 10.0

    def test_float_value(self):
        assert _parse_timeout(10.5) == 10.5

    def test_seconds_string(self):
        assert _parse_timeout("3s") == 3.0

    def test_milliseconds_string(self):
        assert _parse_timeout("500ms") == 0.5

    def test_minutes_string(self):
        assert _parse_timeout("1 min") == 60.0

    def test_compound_string(self):
        assert _parse_timeout("1 minute 30 seconds") == 90.0

    def test_timer_format(self):
        assert _parse_timeout("1:30") == 90.0

    def test_hours_string(self):
        assert _parse_timeout("1h") == 3600.0

    def test_plain_number_string(self):
        assert _parse_timeout("10") == 10.0

    def test_invalid_string_raises_value_error(self):
        with pytest.raises(ValueError, match="Invalid time string"):
            _parse_timeout("not_a_time")

    def test_empty_string_raises_value_error(self):
        with pytest.raises(ValueError):
            _parse_timeout("")

    def test_negative_value(self):
        # Robot Framework's timestr_to_secs accepts negative, passes through
        assert _parse_timeout(-5) == -5.0

    def test_zero(self):
        assert _parse_timeout(0) == 0.0
        assert _parse_timeout("0") == 0.0
```

### Robot Framework Integration Tests

Add to `tests/robot/swing/time_string_support.robot`:

```robot
*** Settings ***
Library    JavaGui.Swing    timeout=5s
Suite Setup    Connect To Application    ${APP}

*** Test Cases ***
Set Timeout Accepts Time String
    [Documentation]    Verify Set Timeout accepts RF time strings
    Set Timeout    3s
    Set Timeout    500ms
    Set Timeout    1 min
    Set Timeout    10

Set Assertion Timeout Accepts Time String
    Set Assertion Timeout    3s
    Set Assertion Timeout    200ms

Set Assertion Interval Accepts Time String
    Set Assertion Interval    100ms
    Set Assertion Interval    0.5

Wait Keywords Accept Time Strings
    Wait Until Element Exists    JButton    timeout=2s
    Wait Until Element Is Visible    JButton    timeout=1s
```

## Backwards Compatibility

This change is **fully backwards compatible**:

1. **Numeric float values** -- `timeout=10.0` -- continue to work identically. The `_parse_timeout` helper passes `int` and `float` through without calling `timestr_to_secs`.

2. **Numeric string values** -- `timeout="10"` -- are handled by `timestr_to_secs`, which parses plain numeric strings as seconds. This is the same result as before (Robot Framework converts string `"10"` to the string `"10"` before passing it to the keyword).

3. **None/default values** -- `timeout=None` -- are handled explicitly, returning the provided default. Behavior is identical to the existing `timeout if timeout is not None else self._timeout` pattern.

4. **Existing test suites** require zero changes. All existing numeric timeout arguments produce the same float values they did before.

5. **Return values** from `Set Timeout`, `Set Assertion Timeout`, and `Set Assertion Interval` continue to return float seconds, so code that captures and restores previous values is unaffected:
   ```robot
   ${old}=    Set Timeout    30s
   # ... operations ...
   Set Timeout    ${old}    # ${old} is a float, still works
   ```

## DDD Mapping

This change fits within the **Keyword Execution Context** bounded context. The time string parsing is a concern of the `CommandOptions` value object -- the set of normalized parameters that the Python keyword layer assembles before dispatching to the Rust core.

```
Keyword Execution Context
  +-- CommandOptions (value object)
  |     +-- timeout: float        # always float after parsing
  |     +-- poll_interval: float
  |     +-- assertion_timeout: float
  |     +-- assertion_interval: float
  +-- TimeParser (domain service)
        +-- parse(input: str | float | None) -> float
```

The `_parse_timeout` function acts as a **domain service** at the boundary between Robot Framework's string-typed world and the library's typed internal model. It enforces the invariant that all durations are non-negative floats by the time they reach the Rust core.

## Consequences

### Positive

1. **Ecosystem Conformance**: Aligns with SeleniumLibrary, Browser Library, and all major RF libraries
2. **Better Readability**: `timeout=30s` is clearer than `timeout=30` in test cases
3. **Zero Rust Changes**: All parsing stays in Python; Rust core is unmodified
4. **Full Backwards Compatibility**: Existing tests and library imports work without changes
5. **Single Conversion Point**: The `_parse_timeout` helper centralizes all time parsing logic
6. **Better Error Messages**: Invalid time strings produce a clear `ValueError` with format examples

### Negative

1. **Additional Dependency**: Relies on `robot.utils.timestr_to_secs`, though Robot Framework is already a required dependency
2. **Slight Overhead**: One function call per timeout parameter; negligible compared to network I/O

### Risks

1. **Type Checker Friction**: Changing `float` to `Union[str, float]` may require `# type: ignore` in some call sites or updated stubs. Mitigation: use a `TimeString` type alias for clarity.
2. **RF Version Compatibility**: `timestr_to_secs` has been stable since Robot Framework 2.x and is present in all supported versions (3.x, 4.x, 5.x, 6.x, 7.x). Risk is minimal.

## Alternatives Considered

### Alternative 1: Parse in Rust via PyO3

Add `timestr_to_secs` calls inside the Rust `py_to_f64` helper by calling back into Python.

**Rejected because**:
- Introduces a Robot Framework dependency into the Rust layer
- Complicates the Rust build and testing (would need Python + RF available for Rust tests)
- Violates the architecture principle that RF-specific concerns belong in the Python layer

### Alternative 2: Custom Time Parser in Rust

Implement a Rust-native time string parser that understands RF formats.

**Rejected because**:
- Duplicates logic already provided by `robot.utils.timestr_to_secs`
- Risk of subtle format incompatibilities with Robot Framework
- Additional code to maintain with no real benefit

### Alternative 3: Accept Only Strings, Always Parse

Change all timeout parameters to `str` type and always run through `timestr_to_secs`.

**Rejected because**:
- Breaks backwards compatibility for programmatic Python callers using numeric values
- Loses type safety for the common case
- `timestr_to_secs` handles numeric strings, but callers passing `float` would get a `TypeError`

## Implementation Plan

1. **Phase 1**: Add `_parse_timeout` helper and unit tests (0.5 day)
2. **Phase 2**: Update `SwingLibrary.__init__` and all `SwingLibrary` wait keywords (0.5 day)
3. **Phase 3**: Update `SwtLibrary` and `RcpLibrary` `__init__` and `set_timeout` (0.5 day)
4. **Phase 4**: Update `GetterKeywords.set_assertion_timeout` and `set_assertion_interval` (0.5 day)
5. **Phase 5**: Add Robot Framework integration tests (0.5 day)
6. **Phase 6**: Update keyword documentation strings to mention time string support (0.5 day)

Total estimated effort: **3 days**

## References

- [Robot Framework User Guide -- Time Format](https://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html#time-format)
- [robot.utils.timestr_to_secs API](https://robot-framework.readthedocs.io/en/stable/autodoc/robot.utils.html#robot.utils.robottypes.timestr_to_secs)
- [SeleniumLibrary timeout handling](https://robotframework.org/SeleniumLibrary/SeleniumLibrary.html#Timeouts) -- uses `timestr_to_secs`
- [Browser Library timeout handling](https://marketsquare.github.io/robotframework-browser/Browser.html#Implicit%20waiting) -- uses `timestr_to_secs`
- [ADR-005: Error Handling Strategy](./ADR-005-error-handling-strategy.md)
- [ADR-007: Unified Keyword API](./ADR-007-UNIFIED-KEYWORD-API.md)
- [ADR-010: AssertionEngine Integration](./ADR-010-ASSERTIONENGINE-INTEGRATION.md)
