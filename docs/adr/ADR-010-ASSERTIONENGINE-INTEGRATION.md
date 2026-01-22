# ADR-010: AssertionEngine Integration Architecture

| ADR ID | ADR-010 |
|--------|---------|
| Title | AssertionEngine Integration Architecture |
| Status | Proposed |
| Date | 2026-01-20 |
| Authors | Architecture Team |
| Supersedes | Extends ADR-007 (Unified Keyword API Design) |

## Context

The robotframework-swing library is a **Rust library with PyO3 Python bindings** that needs to integrate with the pure Python [robotframework-assertion-engine](https://github.com/MarketSquare/AssertionEngine) package (v3.0.3) from MarketSquare. This integration follows the Browser Library pattern for inline assertions, as outlined in ADR-007.

### Current Architecture

```
+------------------+     +------------------+     +------------------+
| Robot Framework  | --> | Python Wrappers  | --> |    Rust Core     |
|   Test Cases     |     | (JavaGui/*.py)   |     | (PyO3 Bindings)  |
+------------------+     +------------------+     +------------------+
                                                          |
                                                          v
                                                  +------------------+
                                                  |   Java Agent     |
                                                  |   (JSON-RPC)     |
                                                  +------------------+
```

### AssertionEngine API (from source code)

```python
from assertionengine import AssertionOperator, verify_assertion, Formatter

# AssertionOperator enum values:
# ==, !=, <, >, <=, >=, *=, ^=, $=, matches, validate, then
# Aliases: equal, equals, should be, inequal, should not be,
#          less than, greater than, contains, starts, ends

# Main verification function:
verify_assertion(
    value: T,
    operator: Optional[AssertionOperator],
    expected: Any,
    message: str = "",
    custom_message: Optional[str] = None,
    formatters: Optional[list] = None,
) -> Any

# Specialized verifiers:
- bool_verify_assertion (only ==, !=)
- list_verify_assertion (==, !=, *=, validate, then)
- dict_verify_assertion (sequence operators)
- flag_verify_assertion (for Flag enums like element states)
- float_str_verify_assertion (numeric + validate/then)
```

### Design Constraints

1. **Rust Core Layer** - Cannot directly use Python packages
2. **PyO3 Bridge** - Python/Rust boundary requires explicit type marshaling
3. **AssertionEngine** - Pure Python, expects Python types
4. **Performance** - Assertions with retry need efficient Rust-Python transitions
5. **Backwards Compatibility** - Existing keywords must continue to work

## Decision

We will implement a **Layered Integration Architecture** where:

1. **Rust Core** provides raw value retrieval functions
2. **Python Keyword Layer** wraps Rust calls with AssertionEngine integration
3. **Retry mechanism** is implemented in Python to leverage AssertionEngine
4. **ElementState Flag enum** maps to AssertionEngine's flag_verify_assertion

### 1. Architecture Layers

```
+-------------------------------------------------------------------+
|                    Robot Framework Interface                       |
|                                                                    |
|   Get Text    locator    ==    Expected Value    timeout=5        |
+-------------------------------------------------------------------+
                                |
                                v
+-------------------------------------------------------------------+
|                   Python Keyword Layer                             |
|   - AssertionEngine integration                                    |
|   - Retry with timeout                                             |
|   - Type conversion                                                |
|   - Error message formatting                                       |
+-------------------------------------------------------------------+
                                |
                                v
+-------------------------------------------------------------------+
|                     PyO3 Bridge Layer                              |
|   - Rust -> Python type conversion                                 |
|   - Python -> Rust parameter passing                               |
|   - Exception translation                                          |
+-------------------------------------------------------------------+
                                |
                                v
+-------------------------------------------------------------------+
|                       Rust Core Layer                              |
|   - Element location (locator parsing)                             |
|   - JSON-RPC communication                                         |
|   - UI tree traversal                                              |
|   - Raw value retrieval                                            |
+-------------------------------------------------------------------+
                                |
                                v
+-------------------------------------------------------------------+
|                        Java Agent                                  |
|   - Component introspection                                        |
|   - Action execution                                               |
|   - Property retrieval                                             |
+-------------------------------------------------------------------+
```

### 2. Module Structure

```
python/
└── JavaGui/
    ├── __init__.py                 # Existing: Library exports
    ├── _core.so                    # Compiled Rust module (PyO3)
    │
    ├── keywords/                   # NEW: Python keyword modules
    │   ├── __init__.py
    │   ├── getters.py              # Get* keywords with assertions
    │   ├── actions.py              # Click, TypeText, etc.
    │   ├── waits.py                # Wait* keywords (uses retry)
    │   └── verification.py         # Legacy verification keywords
    │
    ├── assertion_support.py        # NEW: AssertionEngine integration
    │   ├── ElementState (Flag enum)
    │   ├── RetryConfig
    │   ├── assertion_with_retry()
    │   └── operator mapping
    │
    └── type_hints.py               # NEW: Type definitions
```

### 3. Core Integration Pattern

#### 3.1 Rust Core Functions (Value Retrieval)

The Rust core exposes "raw" functions that return Python-native values:

```rust
// In src/python/swing_library.rs

impl SwingLibrary {
    /// Get element text - raw value, no assertions
    /// Called from Python keyword layer
    #[pyo3(name = "_get_element_text_raw")]
    pub fn get_element_text_raw(&self, locator: &str) -> PyResult<String> {
        self.ensure_connected()?;
        self.clear_tree_cache()?;

        let elements = self.find_elements_internal(locator)?;
        if elements.is_empty() {
            return Err(SwingError::element_not_found(locator).into());
        }

        Ok(elements[0].text.clone().unwrap_or_default())
    }

    /// Get element states as list of strings
    #[pyo3(name = "_get_element_states_raw")]
    pub fn get_element_states_raw(&self, locator: &str) -> PyResult<Vec<String>> {
        self.ensure_connected()?;
        self.clear_tree_cache()?;

        let element = self.find_element(locator)?;
        let mut states = Vec::new();

        if element.visible && element.showing {
            states.push("visible".to_string());
        } else {
            states.push("hidden".to_string());
        }

        if element.enabled {
            states.push("enabled".to_string());
        } else {
            states.push("disabled".to_string());
        }

        if element.focused {
            states.push("focused".to_string());
        }

        if let Some(selected) = element.selected {
            if selected {
                states.push("selected".to_string());
            }
        }

        if let Some(editable) = element.editable {
            if editable {
                states.push("editable".to_string());
            } else {
                states.push("readonly".to_string());
            }
        }

        Ok(states)
    }

    /// Get element count - raw value for assertion
    #[pyo3(name = "_get_element_count_raw")]
    pub fn get_element_count_raw(&self, locator: &str) -> PyResult<i32> {
        self.ensure_connected()?;
        self.clear_tree_cache()?;

        match self.find_elements_internal(locator) {
            Ok(elements) => Ok(elements.len() as i32),
            Err(_) => Ok(0),
        }
    }

    /// Get element property - raw value
    #[pyo3(name = "_get_element_property_raw")]
    pub fn get_element_property_raw(
        &self,
        py: Python<'_>,
        locator: &str,
        property_name: &str,
    ) -> PyResult<PyObject> {
        // Existing implementation, returns Python object
        self.get_element_property(py, locator, property_name)
    }
}
```

#### 3.2 Python AssertionEngine Integration

```python
# python/JavaGui/assertion_support.py
"""
AssertionEngine integration for JavaGui library.

Provides retry-enabled assertions following Browser Library patterns.
"""

from enum import Flag, auto
from typing import Any, Callable, Optional, TypeVar, Union, List
from dataclasses import dataclass
import time

# Import AssertionEngine
from assertionengine import (
    AssertionOperator,
    verify_assertion,
    bool_verify_assertion,
    list_verify_assertion,
    flag_verify_assertion,
    float_str_verify_assertion,
)

T = TypeVar('T')


class ElementState(Flag):
    """
    Element states as Flag enum for flag_verify_assertion.

    Supports bitwise operations for compound state checking:
        visible | enabled  -> Element must be both visible AND enabled
    """
    visible = auto()
    hidden = auto()
    enabled = auto()
    disabled = auto()
    focused = auto()
    unfocused = auto()
    selected = auto()
    unselected = auto()
    checked = auto()
    unchecked = auto()
    editable = auto()
    readonly = auto()
    expanded = auto()
    collapsed = auto()
    attached = auto()
    detached = auto()


# Mapping from string states to ElementState flags
STATE_STRING_TO_FLAG = {
    'visible': ElementState.visible,
    'hidden': ElementState.hidden,
    'enabled': ElementState.enabled,
    'disabled': ElementState.disabled,
    'focused': ElementState.focused,
    'unfocused': ElementState.unfocused,
    'selected': ElementState.selected,
    'unselected': ElementState.unselected,
    'checked': ElementState.checked,
    'unchecked': ElementState.unchecked,
    'editable': ElementState.editable,
    'readonly': ElementState.readonly,
    'expanded': ElementState.expanded,
    'collapsed': ElementState.collapsed,
    'attached': ElementState.attached,
    'detached': ElementState.detached,
}


def states_to_flag(states: List[str]) -> ElementState:
    """Convert list of state strings to ElementState flag."""
    result = ElementState(0)
    for state in states:
        state_lower = state.lower().strip()
        if state_lower in STATE_STRING_TO_FLAG:
            result |= STATE_STRING_TO_FLAG[state_lower]
    return result


def parse_expected_states(expected: Union[str, List[str]]) -> ElementState:
    """
    Parse expected states from Robot Framework input.

    Accepts:
        - Single string: "visible"
        - Comma-separated: "visible, enabled"
        - List: ["visible", "enabled"]
    """
    if isinstance(expected, str):
        parts = [s.strip() for s in expected.split(',')]
        return states_to_flag(parts)
    return states_to_flag(expected)


@dataclass
class RetryConfig:
    """Configuration for retry behavior."""
    timeout: float = 5.0
    poll_interval: float = 0.25


DEFAULT_RETRY_CONFIG = RetryConfig()


class AssertionError(Exception):
    """
    Assertion failure with rich context.

    Provides detailed error messages including:
    - Expected vs actual values
    - Locator used
    - Timeout information
    - Suggestions for debugging
    """

    def __init__(
        self,
        message: str,
        locator: str = "",
        operator: Optional[str] = None,
        expected: Any = None,
        actual: Any = None,
        timeout: Optional[float] = None,
    ):
        self.message = message
        self.locator = locator
        self.operator = operator
        self.expected = expected
        self.actual = actual
        self.timeout = timeout

        # Build detailed message
        parts = [message]

        if locator:
            parts.append(f"\n  Locator: {locator}")

        if operator:
            parts.append(f"\n  Operator: {operator}")

        if expected is not None:
            parts.append(f"\n  Expected: {expected!r}")

        if actual is not None:
            parts.append(f"\n  Actual: {actual!r}")

        if timeout:
            parts.append(f"\n  Timeout: {timeout}s")

        parts.append("\n\nSuggestions:")
        parts.append("\n  - Increase timeout if the value changes slowly")
        parts.append("\n  - Use 'Log UI Tree' to verify element exists")
        parts.append("\n  - Check if locator matches expected element")

        super().__init__(''.join(parts))


def assertion_with_retry(
    get_value: Callable[[], T],
    operator: Optional[AssertionOperator],
    expected: Any,
    message: str,
    custom_message: Optional[str] = None,
    locator: str = "",
    config: Optional[RetryConfig] = None,
    formatters: Optional[list] = None,
) -> T:
    """
    Execute assertion with retry until timeout.

    Args:
        get_value: Callable that retrieves the current value
        operator: AssertionEngine operator (None to skip assertion)
        expected: Expected value for assertion
        message: Base message for assertion
        custom_message: Custom error message override
        locator: Element locator for error context
        config: Retry configuration
        formatters: AssertionEngine formatters

    Returns:
        The actual value (for chaining or further use)

    Raises:
        AssertionError: If assertion fails after timeout
    """
    if config is None:
        config = DEFAULT_RETRY_CONFIG

    # If no operator, just return the value (no assertion)
    if operator is None:
        return get_value()

    start_time = time.time()
    last_value = None
    last_error = None

    while True:
        elapsed = time.time() - start_time

        try:
            value = get_value()
            last_value = value

            # Use AssertionEngine's verify_assertion
            verify_assertion(
                value=value,
                operator=operator,
                expected=expected,
                message=message,
                custom_message=custom_message,
                formatters=formatters,
            )

            # Assertion passed
            return value

        except Exception as e:
            last_error = str(e)

            # Check timeout
            if elapsed >= config.timeout:
                raise AssertionError(
                    message=f"Assertion failed after {config.timeout}s: {last_error}",
                    locator=locator,
                    operator=str(operator) if operator else None,
                    expected=expected,
                    actual=last_value,
                    timeout=config.timeout,
                ) from e

        # Wait before retry
        time.sleep(config.poll_interval)


def state_assertion_with_retry(
    get_states: Callable[[], List[str]],
    operator: Optional[AssertionOperator],
    expected: Union[str, List[str]],
    message: str,
    custom_message: Optional[str] = None,
    locator: str = "",
    config: Optional[RetryConfig] = None,
) -> List[str]:
    """
    Execute state assertion using flag_verify_assertion with retry.

    Converts string states to ElementState Flag enum for proper
    flag-based comparison using AssertionEngine.

    Args:
        get_states: Callable that returns list of state strings
        operator: AssertionEngine operator (contains, not contains, etc.)
        expected: Expected states (string or list)
        message: Base message for assertion
        custom_message: Custom error message
        locator: Element locator for context
        config: Retry configuration

    Returns:
        List of actual state strings
    """
    if config is None:
        config = DEFAULT_RETRY_CONFIG

    # If no operator, just return the states
    if operator is None:
        return get_states()

    expected_flags = parse_expected_states(expected)
    start_time = time.time()
    last_states = None
    last_error = None

    while True:
        elapsed = time.time() - start_time

        try:
            states = get_states()
            last_states = states
            actual_flags = states_to_flag(states)

            # Use flag_verify_assertion for proper flag comparison
            flag_verify_assertion(
                value=actual_flags,
                operator=operator,
                expected=expected_flags,
                message=message,
                custom_message=custom_message,
            )

            return states

        except Exception as e:
            last_error = str(e)

            if elapsed >= config.timeout:
                raise AssertionError(
                    message=f"State assertion failed after {config.timeout}s: {last_error}",
                    locator=locator,
                    operator=str(operator) if operator else None,
                    expected=expected,
                    actual=last_states,
                    timeout=config.timeout,
                ) from e

        time.sleep(config.poll_interval)


def numeric_assertion_with_retry(
    get_value: Callable[[], Union[int, float, str]],
    operator: Optional[AssertionOperator],
    expected: Union[int, float, str],
    message: str,
    custom_message: Optional[str] = None,
    locator: str = "",
    config: Optional[RetryConfig] = None,
) -> Union[int, float]:
    """
    Execute numeric assertion using float_str_verify_assertion.

    Handles int, float, and numeric strings properly.
    """
    if config is None:
        config = DEFAULT_RETRY_CONFIG

    if operator is None:
        return get_value()

    start_time = time.time()
    last_value = None
    last_error = None

    while True:
        elapsed = time.time() - start_time

        try:
            value = get_value()
            last_value = value

            # Use float_str_verify_assertion for numeric comparison
            float_str_verify_assertion(
                value=value,
                operator=operator,
                expected=expected,
                message=message,
                custom_message=custom_message,
            )

            return value

        except Exception as e:
            last_error = str(e)

            if elapsed >= config.timeout:
                raise AssertionError(
                    message=f"Numeric assertion failed after {config.timeout}s: {last_error}",
                    locator=locator,
                    operator=str(operator) if operator else None,
                    expected=expected,
                    actual=last_value,
                    timeout=config.timeout,
                ) from e

        time.sleep(config.poll_interval)
```

#### 3.3 Python Keyword Layer with Assertions

```python
# python/JavaGui/keywords/getters.py
"""
Get* keywords with inline AssertionEngine assertions.

These keywords follow the Browser Library pattern where:
- Without operator: Returns the value
- With operator: Performs assertion with retry, then returns value
"""

from typing import Any, List, Optional, Union
from assertionengine import AssertionOperator

from ..assertion_support import (
    assertion_with_retry,
    state_assertion_with_retry,
    numeric_assertion_with_retry,
    RetryConfig,
)


class GetterKeywordsMixin:
    """
    Mixin class providing Get* keywords with assertion support.

    Requires the class to have:
        - self._lib: Reference to Rust library instance
        - self._timeout: Default timeout in seconds
    """

    def get_text(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Optional[str] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> str:
        """Get text content of an element, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected value for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. Default 5s. |

        Returns the element's text content.

        When ``assertion_operator`` is provided, performs assertion with retry:
        | `Get Text` | JLabel#status | == | Ready |
        | `Get Text` | JLabel#msg | contains | success |

        Without operator, just returns the value:
        | ${text}= | `Get Text` | JLabel#status |

        Examples:
        | # Get text without assertion |
        | ${text}= | `Get Text` | JLabel#status |
        | # Assert exact match |
        | `Get Text` | JLabel#status | == | Ready |
        | # Assert contains |
        | `Get Text` | JLabel#message | contains | completed |
        | # Assert with regex |
        | `Get Text` | JLabel#count | matches | \\d+ items |
        | # Assert with timeout |
        | `Get Text` | JLabel#status | == | Done | timeout=30 |
        | # Custom error message |
        | `Get Text` | JLabel#result | != | Error | message=Operation failed |
        """
        config = RetryConfig(
            timeout=timeout if timeout is not None else self._timeout,
            poll_interval=0.25,
        )

        def get_value():
            return self._lib._get_element_text_raw(locator)

        return assertion_with_retry(
            get_value=get_value,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Element '{locator}' text",
            locator=locator,
            config=config,
        )

    def get_value(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Optional[Any] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> Any:
        """Get the value of an input element, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected value for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns the element's current value (text field content, selected item, etc.).

        Examples:
        | ${value}= | `Get Value` | JTextField#username |
        | `Get Value` | JTextField#email | contains | @ |
        | `Get Value` | JComboBox#country | == | USA |
        | `Get Value` | JSpinner#count | >= | 5 |
        """
        config = RetryConfig(
            timeout=timeout if timeout is not None else self._timeout,
            poll_interval=0.25,
        )

        def get_value():
            return self._lib._get_element_property_raw(locator, "value")

        # For numeric values, use numeric assertion
        if assertion_operator and expected is not None:
            try:
                # Try numeric assertion if expected looks numeric
                float(expected)
                return numeric_assertion_with_retry(
                    get_value=get_value,
                    operator=assertion_operator,
                    expected=expected,
                    message=message or f"Element '{locator}' value",
                    locator=locator,
                    config=config,
                )
            except (ValueError, TypeError):
                pass

        return assertion_with_retry(
            get_value=get_value,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Element '{locator}' value",
            locator=locator,
            config=config,
        )

    def get_element_states(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Optional[Union[str, List[str]]] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> List[str]:
        """Get element states, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected states for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns a list of current states. Possible states:
        - ``visible`` / ``hidden``
        - ``enabled`` / ``disabled``
        - ``focused`` / ``unfocused``
        - ``selected`` / ``unselected``
        - ``checked`` / ``unchecked``
        - ``editable`` / ``readonly``
        - ``expanded`` / ``collapsed`` (for tree nodes)

        Examples:
        | ${states}= | `Get Element States` | JButton#submit |
        | `Get Element States` | JButton#submit | contains | enabled |
        | `Get Element States` | JButton#submit | contains | visible, enabled |
        | `Get Element States` | JCheckBox#opt | contains | checked |
        | `Get Element States` | JButton#delete | not contains | enabled |
        """
        config = RetryConfig(
            timeout=timeout if timeout is not None else self._timeout,
            poll_interval=0.25,
        )

        def get_states():
            return self._lib._get_element_states_raw(locator)

        return state_assertion_with_retry(
            get_states=get_states,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Element '{locator}' states",
            locator=locator,
            config=config,
        )

    def get_element_count(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Optional[int] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> int:
        """Get count of elements matching locator, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected count for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns the number of elements matching the locator.

        Examples:
        | ${count}= | `Get Element Count` | JButton |
        | `Get Element Count` | JButton | == | 5 |
        | `Get Element Count` | JTable >> JTableRow | >= | 1 |
        | `Get Element Count` | JTree >> JTreeNode | > | 0 |
        """
        config = RetryConfig(
            timeout=timeout if timeout is not None else self._timeout,
            poll_interval=0.25,
        )

        def get_count():
            return self._lib._get_element_count_raw(locator)

        return numeric_assertion_with_retry(
            get_value=get_count,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Element count for '{locator}'",
            locator=locator,
            config=config,
        )

    def get_property(
        self,
        locator: str,
        property_name: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Optional[Any] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> Any:
        """Get a specific property of an element, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``property_name`` | Property to retrieve. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected value for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Common properties:
        - ``text``: Display text
        - ``value``: Input value
        - ``name``: Component name
        - ``enabled``: Whether enabled (bool)
        - ``visible``: Whether visible (bool)
        - ``selected``: Whether selected (bool)
        - ``editable``: Whether editable (bool)
        - ``bounds``: Component bounds (x, y, width, height)
        - ``rowCount``: Table row count
        - ``columnCount``: Table column count
        - ``selectedIndex``: Selected index (list/combo)
        - ``selectedValue``: Selected value
        - ``cellValue[row,col]``: Table cell value

        Examples:
        | ${name}= | `Get Property` | JButton | name |
        | `Get Property` | JButton#submit | enabled | == | ${True} |
        | `Get Property` | JTable#data | rowCount | >= | 10 |
        | `Get Property` | JComboBox | selectedIndex | == | 2 |
        """
        config = RetryConfig(
            timeout=timeout if timeout is not None else self._timeout,
            poll_interval=0.25,
        )

        def get_prop():
            return self._lib._get_element_property_raw(locator, property_name)

        return assertion_with_retry(
            get_value=get_prop,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Element '{locator}' property '{property_name}'",
            locator=locator,
            config=config,
        )
```

### 4. Type Conversion Strategy

#### 4.1 Rust to Python Type Mapping

| Rust Type | Python Type | AssertionEngine Use |
|-----------|-------------|---------------------|
| `String` | `str` | `verify_assertion` |
| `i32/i64` | `int` | `float_str_verify_assertion` |
| `f32/f64` | `float` | `float_str_verify_assertion` |
| `bool` | `bool` | `bool_verify_assertion` |
| `Vec<String>` | `List[str]` | State -> Flag conversion |
| `HashMap<String, Value>` | `Dict[str, Any]` | `dict_verify_assertion` |
| `Option<T>` | `Optional[T]` / `None` | Handled specially |
| `SwingElement` | Python object | Not asserted directly |

#### 4.2 Error Propagation

```python
# Error flow from Rust through AssertionEngine

Rust Error (SwingError)
    |
    v (PyO3 conversion)
Python Exception (ElementNotFoundError, ActionTimeoutError, etc.)
    |
    v (Caught in assertion_with_retry)
Wrapped in AssertionError with context
    |
    v (Robot Framework)
Test Failure with detailed message
```

### 5. Integration with Existing Library

#### 5.1 Extending SwingLibrary

```python
# python/JavaGui/__init__.py (updated)

from .keywords.getters import GetterKeywordsMixin
from .keywords.actions import ActionKeywordsMixin

class SwingLibrary(GetterKeywordsMixin, ActionKeywordsMixin):
    """
    Robot Framework library for Java Swing automation.

    Includes AssertionEngine integration for inline assertions.
    """

    # ... existing __init__ code ...

    # Getter keywords are provided by GetterKeywordsMixin
    # - get_text
    # - get_value
    # - get_element_states
    # - get_element_count
    # - get_property

    # Legacy keywords remain available (backwards compatible)
    # - get_element_text
    # - element_text_should_be
    # - element_should_be_visible
    # - etc.
```

#### 5.2 Backwards Compatibility

Existing keywords remain unchanged:

```python
# Legacy keywords (no assertions, immediate verification)
element_should_be_visible(locator)      # Existing, still works
element_text_should_be(locator, value)  # Existing, still works
wait_until_element_exists(locator)      # Existing, still works

# New unified keywords (with optional assertions)
get_text(locator)                       # Returns text
get_text(locator, '==', 'Expected')     # Asserts with retry
get_element_states(locator, 'contains', 'visible')  # State assertion
```

### 6. Retry/Timeout Configuration

#### 6.1 Default Configuration

```python
# Default retry configuration (Browser Library compatible)
DEFAULT_RETRY_CONFIG = RetryConfig(
    timeout=5.0,          # 5 second default timeout
    poll_interval=0.25,   # 250ms polling interval
)

# Can be configured per-call or globally
lib = SwingLibrary(timeout=10.0)  # Sets default for all keywords
get_text(locator, '==', 'Ready', timeout=30)  # Override for this call
```

#### 6.2 Timeout Hierarchy

```
1. Keyword argument (highest priority)
   get_text(locator, '==', 'X', timeout=30)

2. Library instance default
   SwingLibrary(timeout=10.0)

3. Global default (5.0 seconds)
   DEFAULT_RETRY_CONFIG.timeout
```

### 7. Operator Mapping

AssertionEngine operators supported:

| Operator | Aliases | Use Case |
|----------|---------|----------|
| `==` | `equal`, `equals`, `should be` | Exact match |
| `!=` | `inequal`, `should not be` | Not equal |
| `<` | `less than` | Numeric comparison |
| `>` | `greater than` | Numeric comparison |
| `<=` | - | Numeric comparison |
| `>=` | - | Numeric comparison |
| `*=` | `contains` | Substring/list contains |
| `^=` | `starts` | String starts with |
| `$=` | `ends` | String ends with |
| `matches` | - | Regex match |
| `validate` | - | Custom validator function |
| `then` | - | Chained assertion |

## Consequences

### Positive

1. **Browser Library Alignment** - Familiar patterns for modern RF users
2. **Reduced Test Verbosity** - Inline assertions eliminate explicit wait/verify keywords
3. **Automatic Retry** - Built-in retry mechanism handles dynamic UIs
4. **Rich Error Messages** - Detailed assertion failures with context
5. **Type-Safe States** - Flag enum provides proper state comparison
6. **Backwards Compatible** - All existing keywords continue to work
7. **Separation of Concerns** - Rust handles performance, Python handles assertions

### Negative

1. **Dependency** - New dependency on assertionengine package
2. **Complexity** - Additional layer in the architecture
3. **Performance** - Python retry loop has some overhead vs. Rust-only
4. **Learning Curve** - Users must learn assertion operators

### Risks

1. **Version Compatibility** - AssertionEngine API may change
2. **Thread Safety** - Concurrent access to Rust library
3. **Memory** - Repeated UI tree queries during retry
4. **Test Migration** - Existing tests may need updates for best experience

## Alternatives Considered

### Alternative 1: Pure Rust Assertion Engine

Implement assertion logic entirely in Rust.

**Rejected because**:
- Would duplicate AssertionEngine functionality
- Harder to maintain compatibility with Browser Library patterns
- More complex error handling across Python/Rust boundary

### Alternative 2: Direct AssertionEngine in Rust via PyO3

Call AssertionEngine from Rust using PyO3 callbacks.

**Rejected because**:
- Complex GIL management for callbacks
- Performance overhead of Python calls from Rust
- Harder to debug

### Alternative 3: No Integration

Keep separate Wait/Assert keywords without AssertionEngine.

**Rejected because**:
- Misses main benefit of Browser Library pattern
- Tests remain verbose
- Inconsistent with modern RF practices

## Implementation Plan

1. **Phase 1: Foundation (1 week)**
   - Add assertionengine to dependencies
   - Implement assertion_support.py module
   - Add ElementState Flag enum
   - Create RetryConfig dataclass

2. **Phase 2: Rust Core Updates (1 week)**
   - Add `_*_raw` methods to SwingLibrary
   - Ensure proper Python type conversion
   - Add _get_element_states_raw method

3. **Phase 3: Python Keyword Layer (1 week)**
   - Implement GetterKeywordsMixin
   - Add get_text, get_value, get_element_states, get_element_count, get_property
   - Integrate with existing library

4. **Phase 4: Integration Testing (1 week)**
   - Test all assertion operators
   - Verify retry behavior
   - Test error messages
   - Performance benchmarks

5. **Phase 5: Documentation (3 days)**
   - Update keyword documentation
   - Add assertion operator reference
   - Migration guide from legacy keywords

6. **Phase 6: SWT/RCP Extension (1 week)**
   - Apply same pattern to SwtLibrary
   - Apply same pattern to RcpLibrary
   - Ensure consistent API

## References

- [ADR-007: Unified Keyword API Design](./ADR-007-UNIFIED-KEYWORD-API.md)
- [ADR-005: Error Handling Strategy](./ADR-005-error-handling-strategy.md)
- [Browser Library Assertion Engine](https://robotframework-browser.org/#assertions)
- [AssertionEngine Source](https://github.com/MarketSquare/AssertionEngine)
- [AssertionEngine PyPI](https://pypi.org/project/robotframework-assertion-engine/)
- [PyO3 User Guide](https://pyo3.rs/)
