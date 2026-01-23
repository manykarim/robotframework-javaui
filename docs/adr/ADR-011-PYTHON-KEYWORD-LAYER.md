# ADR-011: Python Keyword Layer Design with AssertionEngine

| ADR ID | ADR-011 |
|--------|---------|
| Title | Python Keyword Layer Design with AssertionEngine |
| Status | Proposed |
| Date | 2026-01-20 |
| Authors | Architecture Team |
| Extends | ADR-007 (Unified Keyword API Design) |

## Context

The robotframework-swing library uses a Rust core with PyO3 bindings to Python. The Rust layer handles raw value retrieval and low-level GUI operations, while the Python layer is responsible for Robot Framework integration, keyword exposure, and assertion handling.

Following the Browser Library pattern (as outlined in ADR-007), we need a Python keyword layer that:
1. Wraps Rust core functions for Robot Framework consumption
2. Integrates with `robotframework-assertion-engine` v3.0.3 from MarketSquare
3. Provides the same assertion patterns used by Browser Library
4. Supports both simple value retrieval and inline assertions

### Decision Drivers

- Browser Library has proven the assertion engine pattern reduces test verbosity by 30-40%
- MarketSquare's AssertionEngine is battle-tested in Browser Library
- Rust core should remain focused on GUI operations, not assertion logic
- Python layer provides better extensibility and Robot Framework integration
- Same API patterns as Browser Library reduces learning curve

## Decision

We will implement a **Python Keyword Layer** that sits between the Rust PyO3 bindings and Robot Framework, providing:
1. AssertionEngine integration for all Get keywords
2. ElementState flag enum for state assertions
3. Configurable retry mechanism for assertions
4. Text formatters for flexible comparisons
5. LibDoc-compatible documentation

### 1. Architecture Overview

```
Robot Framework Test
        |
        v
+-------------------+
| Python Keywords   |  <-- ADR-011 (This ADR)
| (assertion layer) |
+-------------------+
        |
        v
+-------------------+
| Rust PyO3 Bindings|  <-- ADR-001 (Base Architecture)
| (raw value ops)   |
+-------------------+
        |
        v
+-------------------+
| Java Agent        |
| (GUI automation)  |
+-------------------+
```

### 2. AssertionEngine Integration

#### 2.1 Supported Operators

The AssertionEngine package provides these operators (from source):

| Operator | Aliases | Description | Value Types |
|----------|---------|-------------|-------------|
| `==` | `equal`, `equals`, `should be` | Exact equality | any |
| `!=` | `inequal`, `should not be` | Not equal | any |
| `>` | `greater than` | Greater than | numeric |
| `>=` | | Greater or equal | numeric |
| `<` | `less than` | Less than | numeric |
| `<=` | | Less or equal | numeric |
| `*=` | `contains` | Contains substring/item | str, list |
| `not contains` | | Does not contain | str, list |
| `^=` | `starts`, `should start with` | String starts with | str |
| `$=` | `ends`, `should end with` | String ends with | str |
| `matches` | | Regex match (at least once) | str |
| `validate` | | Python expression evaluates to True | any |
| `then` / `evaluate` | | Returns evaluated Python expression | any |

#### 2.2 Core Python Module Structure

```python
# src/python/swing_keywords/__init__.py
"""
robotframework-swing Python Keyword Layer

This module provides Robot Framework keywords with AssertionEngine integration.
"""

from .getter_keywords import GetterKeywords
from .action_keywords import ActionKeywords
from .session_keywords import SessionKeywords
from .introspection_keywords import IntrospectionKeywords
from .menu_keywords import MenuKeywords
from .element_state import ElementState
from .formatters import Formatter, normalize_spaces, strip_text, case_insensitive
from .retry import with_retry, RetryConfig

__all__ = [
    'GetterKeywords',
    'ActionKeywords',
    'SessionKeywords',
    'IntrospectionKeywords',
    'MenuKeywords',
    'ElementState',
    'Formatter',
    'with_retry',
    'RetryConfig',
]
```

### 3. ElementState Flag Enum

```python
# src/python/swing_keywords/element_state.py
"""Element state flags for assertion operations."""

from enum import Flag, auto
from typing import List


class ElementState(Flag):
    """Flag enum representing possible element states.

    States are represented as flags allowing multiple states to be
    combined and checked simultaneously using the AssertionEngine's
    flag_verify_assertion function.

    State pairs are mutually exclusive:
    - visible / hidden
    - enabled / disabled
    - focused / unfocused
    - selected / unselected
    - checked / unchecked
    - editable / readonly
    - expanded / collapsed
    - attached / detached

    Examples (Robot Framework):
    | ${states}= | Get Element States | JButton#submit |
    | Get Element States | JButton#submit | contains | visible |
    | Get Element States | JButton#submit | contains | enabled & visible |
    """

    # Visibility states
    visible = auto()
    hidden = auto()

    # Enablement states
    enabled = auto()
    disabled = auto()

    # Focus states
    focused = auto()
    unfocused = auto()

    # Selection states (for lists, tabs, etc.)
    selected = auto()
    unselected = auto()

    # Check states (for checkboxes, toggles)
    checked = auto()
    unchecked = auto()

    # Edit states (for text fields)
    editable = auto()
    readonly = auto()

    # Tree node states
    expanded = auto()
    collapsed = auto()

    # DOM attachment states
    attached = auto()
    detached = auto()

    @classmethod
    def from_string(cls, state_str: str) -> 'ElementState':
        """Convert string to ElementState.

        Args:
            state_str: State name (case-insensitive).

        Returns:
            ElementState flag.

        Raises:
            ValueError: If state name is not recognized.
        """
        state_str = state_str.lower().strip()
        try:
            return cls[state_str]
        except KeyError:
            valid_states = [s.name for s in cls]
            raise ValueError(
                f"Unknown state '{state_str}'. Valid states: {', '.join(valid_states)}"
            )

    @classmethod
    def from_strings(cls, state_strs: List[str]) -> 'ElementState':
        """Convert list of strings to combined ElementState flags.

        Args:
            state_strs: List of state names.

        Returns:
            Combined ElementState flags.
        """
        result = cls(0)
        for state_str in state_strs:
            result |= cls.from_string(state_str)
        return result

    def to_list(self) -> List[str]:
        """Convert ElementState flags to list of state names.

        Returns:
            List of state names that are set.
        """
        return [state.name for state in ElementState if state in self]


def element_states_to_flag(
    visible: bool = True,
    enabled: bool = True,
    focused: bool = False,
    selected: bool = False,
    checked: bool = False,
    editable: bool = True,
    expanded: bool = False,
    attached: bool = True
) -> ElementState:
    """Create ElementState flags from individual boolean properties.

    Args:
        visible: Whether element is visible.
        enabled: Whether element is enabled.
        focused: Whether element has focus.
        selected: Whether element is selected.
        checked: Whether element is checked.
        editable: Whether element is editable.
        expanded: Whether element is expanded (for trees).
        attached: Whether element is attached to DOM.

    Returns:
        Combined ElementState flags.
    """
    state = ElementState(0)

    state |= ElementState.visible if visible else ElementState.hidden
    state |= ElementState.enabled if enabled else ElementState.disabled
    state |= ElementState.focused if focused else ElementState.unfocused
    state |= ElementState.selected if selected else ElementState.unselected
    state |= ElementState.checked if checked else ElementState.unchecked
    state |= ElementState.editable if editable else ElementState.readonly
    state |= ElementState.expanded if expanded else ElementState.collapsed
    state |= ElementState.attached if attached else ElementState.detached

    return state
```

### 4. Retry Mechanism

```python
# src/python/swing_keywords/retry.py
"""Retry mechanism for assertion operations."""

import time
from dataclasses import dataclass
from typing import Any, Callable, Optional, TypeVar, Union
from robot.api import logger

T = TypeVar('T')


@dataclass
class RetryConfig:
    """Configuration for retry behavior.

    Attributes:
        timeout: Maximum time to retry in seconds.
        interval: Time between retries in seconds.
        message: Custom error message on failure.
    """
    timeout: float = 5.0
    interval: float = 0.1
    message: Optional[str] = None


class RetryError(Exception):
    """Raised when retry operation fails after timeout."""

    def __init__(
        self,
        message: str,
        last_value: Any = None,
        last_error: Optional[Exception] = None,
        timeout: float = 0,
        attempts: int = 0
    ):
        self.message = message
        self.last_value = last_value
        self.last_error = last_error
        self.timeout = timeout
        self.attempts = attempts
        super().__init__(self._build_message())

    def _build_message(self) -> str:
        parts = [self.message]
        if self.timeout > 0:
            parts.append(f"Timeout: {self.timeout}s")
        if self.attempts > 0:
            parts.append(f"Attempts: {self.attempts}")
        if self.last_value is not None:
            parts.append(f"Last value: {self.last_value!r}")
        if self.last_error is not None:
            parts.append(f"Last error: {self.last_error}")
        return " | ".join(parts)


def with_retry(
    func: Callable[[], T],
    timeout: float = 5.0,
    interval: float = 0.1,
    message: Optional[str] = None,
    on_retry: Optional[Callable[[int, Any, Optional[Exception]], None]] = None
) -> T:
    """Retry a function until success or timeout.

    Args:
        func: Function to call that returns a value.
        timeout: Maximum time to retry in seconds.
        interval: Time between retries in seconds.
        message: Custom error message on failure.
        on_retry: Optional callback called on each retry with
                  (attempt_number, last_value, last_error).

    Returns:
        The successful return value from func.

    Raises:
        RetryError: If timeout is reached without success.

    Examples:
        >>> value = with_retry(lambda: get_element_text(locator), timeout=10.0)
    """
    start_time = time.time()
    attempts = 0
    last_value = None
    last_error = None

    while True:
        attempts += 1
        try:
            value = func()
            return value
        except Exception as e:
            last_error = e
            elapsed = time.time() - start_time

            if elapsed >= timeout:
                raise RetryError(
                    message=message or f"Retry failed after {timeout}s",
                    last_value=last_value,
                    last_error=last_error,
                    timeout=timeout,
                    attempts=attempts
                )

            if on_retry:
                on_retry(attempts, last_value, last_error)

            logger.debug(
                f"Retry attempt {attempts}: {e}. "
                f"Elapsed: {elapsed:.2f}s, remaining: {timeout - elapsed:.2f}s"
            )
            time.sleep(interval)


def with_retry_assertion(
    get_value: Callable[[], T],
    assert_func: Callable[[T], bool],
    timeout: float = 5.0,
    interval: float = 0.1,
    message: Optional[str] = None
) -> T:
    """Retry getting a value until assertion passes or timeout.

    This is useful for polling-based assertions where the value
    may change over time (e.g., waiting for UI to update).

    Args:
        get_value: Function to get the current value.
        assert_func: Function that returns True if assertion passes.
        timeout: Maximum time to retry in seconds.
        interval: Time between retries in seconds.
        message: Custom error message on failure.

    Returns:
        The value that passed the assertion.

    Raises:
        AssertionError: If timeout is reached without passing.
    """
    start_time = time.time()
    attempts = 0
    last_value = None
    last_error = None

    while True:
        attempts += 1
        try:
            value = get_value()
            last_value = value

            if assert_func(value):
                return value

        except Exception as e:
            last_error = e

        elapsed = time.time() - start_time
        if elapsed >= timeout:
            error_parts = [message or "Assertion failed"]
            error_parts.append(f"after {timeout}s ({attempts} attempts)")
            if last_value is not None:
                error_parts.append(f"Last value: {last_value!r}")
            if last_error:
                error_parts.append(f"Last error: {last_error}")
            raise AssertionError(" | ".join(error_parts))

        time.sleep(interval)
```

### 5. Text Formatters

```python
# src/python/swing_keywords/formatters.py
"""Text formatters for assertion value preprocessing."""

import re
from enum import Enum, auto
from typing import Any, Callable, List, Optional, Union


class Formatter(Enum):
    """Built-in formatters for text preprocessing before assertions.

    Formatters can be applied to values before assertion comparison
    to normalize text, handle whitespace, or perform case-insensitive
    comparisons.

    Examples (Robot Framework):
    | Get Text | JLabel#status | == | Hello World | formatters=[normalize_spaces] |
    | Get Text | JLabel#status | == | hello | formatters=[strip, lower] |
    """

    normalize_spaces = auto()
    strip = auto()
    lower = auto()
    upper = auto()
    strip_tags = auto()

    def apply(self, value: Any) -> Any:
        """Apply this formatter to a value.

        Args:
            value: Value to format.

        Returns:
            Formatted value.
        """
        if not isinstance(value, str):
            return value

        if self == Formatter.normalize_spaces:
            return normalize_spaces(value)
        elif self == Formatter.strip:
            return strip_text(value)
        elif self == Formatter.lower:
            return value.lower()
        elif self == Formatter.upper:
            return value.upper()
        elif self == Formatter.strip_tags:
            return strip_html_tags(value)
        return value


def normalize_spaces(text: str) -> str:
    """Normalize whitespace in text.

    Replaces all sequences of whitespace (spaces, tabs, newlines)
    with a single space and strips leading/trailing whitespace.

    Args:
        text: Input text.

    Returns:
        Text with normalized whitespace.

    Examples:
        >>> normalize_spaces("  hello   world  ")
        'hello world'
        >>> normalize_spaces("line1\\nline2\\tline3")
        'line1 line2 line3'
    """
    return ' '.join(text.split())


def strip_text(text: str) -> str:
    """Strip leading and trailing whitespace.

    Args:
        text: Input text.

    Returns:
        Stripped text.
    """
    return text.strip()


def strip_html_tags(text: str) -> str:
    """Remove HTML tags from text.

    Args:
        text: Input text with potential HTML tags.

    Returns:
        Text with HTML tags removed.
    """
    return re.sub(r'<[^>]+>', '', text)


def case_insensitive(text: str) -> str:
    """Convert text to lowercase for case-insensitive comparison.

    Args:
        text: Input text.

    Returns:
        Lowercase text.
    """
    return text.lower()


def apply_formatters(value: Any, formatters: Optional[List[Formatter]] = None) -> Any:
    """Apply a list of formatters to a value.

    Args:
        value: Value to format.
        formatters: List of formatters to apply in order.

    Returns:
        Formatted value.
    """
    if formatters is None:
        return value

    result = value
    for formatter in formatters:
        result = formatter.apply(result)
    return result
```

### 6. Core Getter Keywords with AssertionEngine

```python
# src/python/swing_keywords/getter_keywords.py
"""Getter keywords with AssertionEngine integration."""

from typing import Any, Dict, List, Optional, Union
from robot.api.deco import keyword, library
from robot.api import logger

from assertionengine import (
    AssertionOperator,
    verify_assertion,
    flag_verify_assertion,
    Formatter as AEFormatter,
)

from .element_state import ElementState, element_states_to_flag
from .formatters import apply_formatters, Formatter
from .retry import with_retry_assertion

# Import Rust core bindings
from robotframework_swing_core import SwingCore


@library(scope='GLOBAL', auto_keywords=False)
class GetterKeywords:
    """Keywords for retrieving element values with optional assertions.

    This class provides getter keywords that follow the Browser Library
    assertion pattern. Each getter can optionally perform an inline
    assertion using the AssertionEngine operators.

    == Assertion Pattern ==

    All getter keywords support optional assertion parameters:
    - ``assertion_operator``: The comparison operator (e.g., ``==``, ``contains``)
    - ``expected``: The expected value to compare against
    - ``message``: Custom error message on assertion failure
    - ``timeout``: Retry timeout for assertion (polls until success)

    When no operator is provided, the keyword simply returns the value.
    When an operator is provided, the keyword asserts and returns the value.

    == Supported Operators ==

    | =Operator= | =Aliases= | =Description= |
    | ``==`` | ``equal``, ``equals``, ``should be`` | Exact equality |
    | ``!=`` | ``inequal``, ``should not be`` | Not equal |
    | ``>`` | ``greater than`` | Greater than (numeric) |
    | ``>=`` | | Greater or equal |
    | ``<`` | ``less than`` | Less than (numeric) |
    | ``<=`` | | Less or equal |
    | ``*=`` | ``contains`` | Contains substring/item |
    | ``not contains`` | | Does not contain |
    | ``^=`` | ``starts``, ``should start with`` | Starts with |
    | ``$=`` | ``ends``, ``should end with`` | Ends with |
    | ``matches`` | | Regex match |
    | ``validate`` | | Python expression evaluates to True |
    | ``then`` / ``evaluate`` | | Returns evaluated expression |

    == Examples ==

    | # Get text without assertion |
    | ${text}= | `Get Text` | JLabel#status |
    |
    | # Assert exact match |
    | `Get Text` | JLabel#status | == | Ready |
    |
    | # Assert contains with timeout |
    | `Get Text` | JLabel#message | contains | Success | timeout=10 |
    |
    | # Assert with regex |
    | `Get Text` | JLabel#count | matches | \\\\d+ items |
    """

    def __init__(self, core: SwingCore):
        """Initialize with Rust core bindings.

        Args:
            core: SwingCore instance for low-level operations.
        """
        self._core = core
        self._default_timeout = 5.0
        self._default_interval = 0.1

    @keyword(tags=['getter', 'assertion'])
    def get_text(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Any = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
        formatters: Optional[List[Formatter]] = None
    ) -> str:
        """Get element text content, optionally with assertion.

        Returns the text content of the element matching the locator.
        When an assertion operator is provided, asserts the text matches
        the expected value before returning.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected value for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. Default ``5.0``. |
        | ``formatters`` | Text formatters to apply before comparison. |

        Returns the element's text content (after formatting if specified).

        == Examples ==

        | # Get text without assertion |
        | ${text}= | `Get Text` | JLabel#status |
        |
        | # Assert exact match |
        | `Get Text` | JLabel#status | == | Ready |
        |
        | # Assert contains |
        | `Get Text` | JLabel#message | contains | success |
        |
        | # Assert with regex pattern |
        | `Get Text` | JLabel#count | matches | \\\\d+ items found |
        |
        | # Assert with timeout (polling) |
        | `Get Text` | JLabel#status | == | Complete | timeout=30 |
        |
        | # Assert with normalized whitespace |
        | `Get Text` | JLabel#msg | == | Hello World | formatters=[normalize_spaces] |
        |
        | # Custom error message |
        | `Get Text` | JLabel#result | != | Error | message=Operation should succeed |
        """
        effective_timeout = timeout if timeout is not None else self._default_timeout

        def get_value() -> str:
            raw_text = self._core.get_element_text(locator)
            return apply_formatters(raw_text, formatters)

        if assertion_operator is None:
            return get_value()

        return self._assert_with_retry(
            get_value=get_value,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Get Text '{locator}'",
            timeout=effective_timeout
        )

    @keyword(tags=['getter', 'assertion'])
    def get_value(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Any = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None
    ) -> Any:
        """Get the value of an input element, optionally with assertion.

        Returns the current value of an input element (text field content,
        selected item in combo box, spinner value, etc.).

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected value for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns the element's current value.

        == Examples ==

        | # Get text field value |
        | ${value}= | `Get Value` | JTextField#username |
        |
        | # Assert text field contains @ symbol |
        | `Get Value` | JTextField#email | contains | @ |
        |
        | # Assert combo box selection |
        | `Get Value` | JComboBox#country | == | United States |
        |
        | # Assert spinner value is at least 5 |
        | `Get Value` | JSpinner#quantity | >= | 5 |
        """
        effective_timeout = timeout if timeout is not None else self._default_timeout

        def get_value_fn() -> Any:
            return self._core.get_element_value(locator)

        if assertion_operator is None:
            return get_value_fn()

        return self._assert_with_retry(
            get_value=get_value_fn,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Get Value '{locator}'",
            timeout=effective_timeout
        )

    @keyword(tags=['getter', 'assertion', 'state'])
    def get_element_states(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Union[str, List[str], ElementState, None] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None
    ) -> List[str]:
        """Get element states, optionally with assertion.

        Returns a list of current element states. Uses flag_verify_assertion
        for state checking with the AssertionEngine.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected states (string, list, or ElementState). |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns a list of state names that are currently set.

        == Possible States ==

        | =State= | =Description= |
        | ``visible`` / ``hidden`` | Element visibility |
        | ``enabled`` / ``disabled`` | Element enabled state |
        | ``focused`` / ``unfocused`` | Element focus state |
        | ``selected`` / ``unselected`` | Selection state (lists, tabs) |
        | ``checked`` / ``unchecked`` | Check state (checkboxes) |
        | ``editable`` / ``readonly`` | Edit state (text fields) |
        | ``expanded`` / ``collapsed`` | Expansion state (trees) |
        | ``attached`` / ``detached`` | DOM attachment state |

        == Examples ==

        | # Get states without assertion |
        | ${states}= | `Get Element States` | JButton#submit |
        | Log | States: @{states} |
        |
        | # Assert element is visible and enabled |
        | `Get Element States` | JButton#submit | contains | visible |
        | `Get Element States` | JButton#submit | contains | enabled |
        |
        | # Assert multiple states at once |
        | `Get Element States` | JButton#submit | contains | ['visible', 'enabled'] |
        |
        | # Assert element is NOT disabled |
        | `Get Element States` | JTextField#input | not contains | disabled |
        |
        | # Assert checkbox is checked |
        | `Get Element States` | JCheckBox#accept | contains | checked |
        |
        | # Wait for element to become enabled |
        | `Get Element States` | JButton#next | contains | enabled | timeout=10 |
        """
        effective_timeout = timeout if timeout is not None else self._default_timeout

        def get_states() -> ElementState:
            props = self._core.get_element_properties(locator)
            return element_states_to_flag(
                visible=props.get('visible', True),
                enabled=props.get('enabled', True),
                focused=props.get('focused', False),
                selected=props.get('selected', False),
                checked=props.get('checked', False),
                editable=props.get('editable', True),
                expanded=props.get('expanded', False),
                attached=props.get('attached', True)
            )

        if assertion_operator is None:
            states = get_states()
            return states.to_list()

        # Convert expected to ElementState if needed
        if isinstance(expected, str):
            expected_state = ElementState.from_string(expected)
        elif isinstance(expected, list):
            expected_state = ElementState.from_strings(expected)
        elif isinstance(expected, ElementState):
            expected_state = expected
        else:
            expected_state = expected

        # Use flag_verify_assertion for state checking
        result = self._flag_assert_with_retry(
            get_value=get_states,
            operator=assertion_operator,
            expected=expected_state,
            message=message or f"Get Element States '{locator}'",
            timeout=effective_timeout
        )

        return result.to_list()

    @keyword(tags=['getter', 'assertion', 'count'])
    def get_element_count(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Optional[int] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None
    ) -> int:
        """Get count of elements matching locator, optionally with assertion.

        Returns the number of elements matching the locator.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected count for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns the count of matching elements.

        == Examples ==

        | # Get count without assertion |
        | ${count}= | `Get Element Count` | JButton |
        |
        | # Assert exact count |
        | `Get Element Count` | JButton | == | 5 |
        |
        | # Assert at least one element exists |
        | `Get Element Count` | JTable >> row | >= | 1 |
        |
        | # Assert no elements exist |
        | `Get Element Count` | JDialog | == | 0 |
        |
        | # Wait for elements to appear |
        | `Get Element Count` | JListItem | > | 0 | timeout=10 |
        """
        effective_timeout = timeout if timeout is not None else self._default_timeout

        def get_count() -> int:
            return self._core.get_element_count(locator)

        if assertion_operator is None:
            return get_count()

        return self._assert_with_retry(
            get_value=get_count,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Get Element Count '{locator}'",
            timeout=effective_timeout
        )

    @keyword(tags=['getter', 'assertion', 'property'])
    def get_property(
        self,
        locator: str,
        property_name: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Any = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None
    ) -> Any:
        """Get a specific property of an element, optionally with assertion.

        Returns the value of the specified property for the element.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``property_name`` | Name of the property to retrieve. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected value for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns the property value.

        == Common Properties ==

        | =Property= | =Description= | =Return Type= |
        | ``text`` | Display text | str |
        | ``value`` | Input value | str |
        | ``name`` | Component name | str |
        | ``enabled`` | Is enabled | bool |
        | ``visible`` | Is visible | bool |
        | ``selected`` | Is selected | bool |
        | ``editable`` | Is editable | bool |
        | ``bounds`` | Component bounds | dict(x, y, width, height) |
        | ``rowCount`` | Table row count | int |
        | ``columnCount`` | Table column count | int |
        | ``selectedIndex`` | Selected index | int |
        | ``selectedValue`` | Selected value | str |
        | ``cellValue[row,col]`` | Table cell value | str |

        == Examples ==

        | # Get property without assertion |
        | ${name}= | `Get Property` | JButton#submit | name |
        |
        | # Assert element is enabled |
        | `Get Property` | JButton#submit | enabled | == | ${True} |
        |
        | # Assert table has rows |
        | `Get Property` | JTable#data | rowCount | >= | 10 |
        |
        | # Assert specific table cell value |
        | `Get Property` | JTable#data | cellValue[0,1] | == | John |
        |
        | # Assert combo box selection index |
        | `Get Property` | JComboBox#type | selectedIndex | == | 2 |
        """
        effective_timeout = timeout if timeout is not None else self._default_timeout

        def get_prop() -> Any:
            # Handle cellValue[row,col] syntax
            if property_name.startswith('cellValue['):
                match = re.match(r'cellValue\[(\d+),(\d+)\]', property_name)
                if match:
                    row, col = int(match.group(1)), int(match.group(2))
                    return self._core.get_table_cell_value(locator, row, col)

            return self._core.get_element_property(locator, property_name)

        if assertion_operator is None:
            return get_prop()

        return self._assert_with_retry(
            get_value=get_prop,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Get Property '{locator}' -> '{property_name}'",
            timeout=effective_timeout
        )

    @keyword(tags=['getter'])
    def get_properties(
        self,
        locator: str
    ) -> Dict[str, Any]:
        """Get all common properties of an element.

        Returns a dictionary containing all retrievable properties
        of the element. No assertion support for this keyword.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |

        Returns a dictionary with property names as keys.

        == Examples ==

        | ${props}= | `Get Properties` | JButton#submit |
        | Log | Text: ${props}[text] |
        | Log | Enabled: ${props}[enabled] |
        | Log | Visible: ${props}[visible] |
        | Should Be True | ${props}[enabled] |
        """
        return self._core.get_element_properties(locator)

    @keyword(tags=['getter', 'assertion', 'table'])
    def get_table_cell_value(
        self,
        locator: str,
        row: int,
        column: Union[int, str],
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Any = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None
    ) -> str:
        """Get table cell value, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Table locator. See `Locator Syntax`. |
        | ``row`` | Row index (0-based). |
        | ``column`` | Column index (0-based) or column header name. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected value for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns the cell value as string.

        == Examples ==

        | # Get cell by indices |
        | ${value}= | `Get Table Cell Value` | JTable#data | 0 | 1 |
        |
        | # Get cell by column header |
        | ${name}= | `Get Table Cell Value` | JTable#data | 0 | Name |
        |
        | # Assert cell value |
        | `Get Table Cell Value` | JTable#data | 0 | Name | == | John |
        |
        | # Assert cell contains substring |
        | `Get Table Cell Value` | JTable#data | 1 | Email | contains | @example.com |
        """
        effective_timeout = timeout if timeout is not None else self._default_timeout

        def get_cell() -> str:
            if isinstance(column, str):
                col_idx = self._core.get_table_column_index(locator, column)
            else:
                col_idx = column
            return self._core.get_table_cell_value(locator, row, col_idx)

        if assertion_operator is None:
            return get_cell()

        return self._assert_with_retry(
            get_value=get_cell,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Get Table Cell Value '{locator}' [{row},{column}]",
            timeout=effective_timeout
        )

    @keyword(tags=['getter', 'assertion', 'table'])
    def get_table_row_count(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Optional[int] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None
    ) -> int:
        """Get table row count, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Table locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected count for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns the number of rows in the table.

        == Examples ==

        | # Get row count |
        | ${count}= | `Get Table Row Count` | JTable#results |
        |
        | # Assert exact row count |
        | `Get Table Row Count` | JTable#results | == | 10 |
        |
        | # Assert at least one row |
        | `Get Table Row Count` | JTable#results | >= | 1 |
        |
        | # Wait for rows to load |
        | `Get Table Row Count` | JTable#results | > | 0 | timeout=30 |
        """
        effective_timeout = timeout if timeout is not None else self._default_timeout

        def get_count() -> int:
            return self._core.get_table_row_count(locator)

        if assertion_operator is None:
            return get_count()

        return self._assert_with_retry(
            get_value=get_count,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Get Table Row Count '{locator}'",
            timeout=effective_timeout
        )

    @keyword(tags=['getter', 'assertion', 'table'])
    def get_table_column_count(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Optional[int] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None
    ) -> int:
        """Get table column count, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Table locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected count for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns the number of columns in the table.

        == Examples ==

        | ${cols}= | `Get Table Column Count` | JTable#data |
        | `Get Table Column Count` | JTable#data | == | 5 |
        """
        effective_timeout = timeout if timeout is not None else self._default_timeout

        def get_count() -> int:
            return self._core.get_table_column_count(locator)

        if assertion_operator is None:
            return get_count()

        return self._assert_with_retry(
            get_value=get_count,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Get Table Column Count '{locator}'",
            timeout=effective_timeout
        )

    @keyword(tags=['getter', 'assertion', 'selection'])
    def get_selected_item(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Any = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None
    ) -> str:
        """Get selected item from list/combo/tree, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Component locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected value for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns the text of the selected item.

        == Supported Components ==

        - JComboBox
        - JList
        - JTree
        - JTabbedPane

        == Examples ==

        | # Get selected combo item |
        | ${selected}= | `Get Selected Item` | JComboBox#country |
        |
        | # Assert selection |
        | `Get Selected Item` | JComboBox#type | == | Java Project |
        |
        | # Assert tree selection |
        | `Get Selected Item` | JTree#files | contains | Document |
        """
        effective_timeout = timeout if timeout is not None else self._default_timeout

        def get_selected() -> str:
            return self._core.get_selected_item(locator)

        if assertion_operator is None:
            return get_selected()

        return self._assert_with_retry(
            get_value=get_selected,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Get Selected Item '{locator}'",
            timeout=effective_timeout
        )

    @keyword(tags=['getter', 'assertion', 'selection'])
    def get_selected_items(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        expected: Any = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None
    ) -> List[str]:
        """Get all selected items (for multi-select), optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Component locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``expected`` | Expected values for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns a list of selected item texts.

        == Examples ==

        | # Get all selected list items |
        | ${items}= | `Get Selected Items` | JList#multiSelect |
        |
        | # Assert selection contains specific item |
        | `Get Selected Items` | JList#files | contains | README.md |
        |
        | # Assert selection count |
        | ${items}= | `Get Selected Items` | JList#files |
        | Length Should Be | ${items} | 3 |
        """
        effective_timeout = timeout if timeout is not None else self._default_timeout

        def get_selections() -> List[str]:
            return self._core.get_selected_items(locator)

        if assertion_operator is None:
            return get_selections()

        return self._assert_with_retry(
            get_value=get_selections,
            operator=assertion_operator,
            expected=expected,
            message=message or f"Get Selected Items '{locator}'",
            timeout=effective_timeout
        )

    # ======================
    # Private Helper Methods
    # ======================

    def _assert_with_retry(
        self,
        get_value: callable,
        operator: AssertionOperator,
        expected: Any,
        message: str,
        timeout: float
    ) -> Any:
        """Perform assertion with retry using AssertionEngine.

        Args:
            get_value: Function to get current value.
            operator: AssertionEngine operator.
            expected: Expected value.
            message: Error message prefix.
            timeout: Retry timeout in seconds.

        Returns:
            The value that passed assertion.

        Raises:
            AssertionError: If assertion fails after timeout.
        """
        import time
        start_time = time.time()
        last_value = None
        last_error = None

        while True:
            try:
                value = get_value()
                last_value = value

                # Use AssertionEngine's verify_assertion
                result = verify_assertion(
                    value=value,
                    operator=operator,
                    expected=expected,
                    message=message,
                    custom_message=None
                )
                return result

            except AssertionError as e:
                last_error = e
                elapsed = time.time() - start_time

                if elapsed >= timeout:
                    # Build detailed error message
                    error_msg = self._build_assertion_error(
                        message=message,
                        operator=operator,
                        expected=expected,
                        actual=last_value,
                        timeout=timeout,
                        error=last_error
                    )
                    raise AssertionError(error_msg) from None

                time.sleep(self._default_interval)

            except Exception as e:
                last_error = e
                elapsed = time.time() - start_time

                if elapsed >= timeout:
                    raise

                time.sleep(self._default_interval)

    def _flag_assert_with_retry(
        self,
        get_value: callable,
        operator: AssertionOperator,
        expected: ElementState,
        message: str,
        timeout: float
    ) -> ElementState:
        """Perform flag assertion with retry.

        Uses AssertionEngine's flag_verify_assertion for ElementState flags.
        """
        import time
        start_time = time.time()
        last_value = None
        last_error = None

        while True:
            try:
                value = get_value()
                last_value = value

                result = flag_verify_assertion(
                    value=value,
                    operator=operator,
                    expected=expected,
                    message=message,
                    custom_message=None
                )
                return result

            except AssertionError as e:
                last_error = e
                elapsed = time.time() - start_time

                if elapsed >= timeout:
                    error_msg = self._build_assertion_error(
                        message=message,
                        operator=operator,
                        expected=expected.to_list() if hasattr(expected, 'to_list') else expected,
                        actual=last_value.to_list() if hasattr(last_value, 'to_list') else last_value,
                        timeout=timeout,
                        error=last_error
                    )
                    raise AssertionError(error_msg) from None

                time.sleep(self._default_interval)

    def _build_assertion_error(
        self,
        message: str,
        operator: AssertionOperator,
        expected: Any,
        actual: Any,
        timeout: float,
        error: Optional[Exception]
    ) -> str:
        """Build detailed assertion error message."""
        lines = [
            f"AssertionError: {message}",
            "",
            f"  Operator: {operator.name if hasattr(operator, 'name') else operator}",
            f"  Expected: {expected!r}",
            f"  Actual:   {actual!r}",
        ]

        if timeout > 0:
            lines.append(f"  Timeout:  {timeout}s")

        if error and str(error):
            lines.append(f"")
            lines.append(f"  Original error: {error}")

        lines.extend([
            "",
            "Suggestions:",
            "  - Increase timeout if the value changes slowly",
            "  - Verify the expected value is correct",
            "  - Check if the locator matches the intended element"
        ])

        return "\n".join(lines)
```

### 7. Complete Keyword Class Integration

```python
# src/python/swing_keywords/library.py
"""Main library class integrating all keyword modules."""

from typing import Optional
from robot.api.deco import library

from .getter_keywords import GetterKeywords
from .action_keywords import ActionKeywords
from .session_keywords import SessionKeywords
from .introspection_keywords import IntrospectionKeywords
from .menu_keywords import MenuKeywords

# Import Rust core
from robotframework_swing_core import SwingCore


@library(scope='GLOBAL', doc_format='ROBOT')
class SwingLibrary(
    GetterKeywords,
    ActionKeywords,
    SessionKeywords,
    IntrospectionKeywords,
    MenuKeywords
):
    """Robot Framework library for Java Swing GUI automation.

    SwingLibrary provides keywords for automating Java Swing applications.
    It uses a Rust core for high-performance GUI operations with Python
    bindings for Robot Framework integration.

    == Getting Started ==

    | *** Settings ***
    | Library    SwingLibrary
    |
    | *** Test Cases ***
    | Login Test
    |     Connect To Application    myapp.jar
    |     Type Text    JTextField#username    testuser
    |     Type Text    JTextField#password    secret
    |     Click    JButton#login
    |     Get Text    JLabel#status    ==    Welcome
    |     [Teardown]    Disconnect

    == Assertion Pattern ==

    All Get keywords support inline assertions using the AssertionEngine:

    | # Without assertion - just get the value
    | ${text}=    Get Text    JLabel#status
    |
    | # With assertion - assert and return value
    | Get Text    JLabel#status    ==    Ready
    | Get Text    JLabel#msg    contains    Success
    | Get Element Count    JButton    >=    1

    See `Assertion Operators` for the full list of operators.

    == Locator Syntax ==

    See `Locator Syntax` for details on finding elements.

    == Timeout and Retry ==

    Assertions with operators automatically retry until success or timeout.
    Default timeout is 5 seconds. Override per-call with ``timeout`` argument.

    | Get Text    JLabel#status    ==    Complete    timeout=30
    """

    ROBOT_LIBRARY_SCOPE = 'GLOBAL'
    ROBOT_LIBRARY_DOC_FORMAT = 'ROBOT'

    def __init__(
        self,
        timeout: float = 5.0,
        poll_interval: float = 0.1,
        screenshot_directory: str = '.'
    ):
        """Initialize SwingLibrary.

        | =Argument= | =Description= |
        | ``timeout`` | Default assertion timeout in seconds. Default ``5.0``. |
        | ``poll_interval`` | Polling interval for retry. Default ``0.1``. |
        | ``screenshot_directory`` | Directory for screenshots. Default ``.``. |

        Examples:
        | =Setting= | =Value= | =Value= |
        | Library | SwingLibrary | |
        | Library | SwingLibrary | timeout=30 |
        | Library | SwingLibrary | timeout=10 | poll_interval=0.5 |
        """
        # Initialize Rust core
        self._core = SwingCore(
            timeout=timeout,
            poll_interval=poll_interval,
            screenshot_directory=screenshot_directory
        )

        # Initialize all keyword modules with core
        GetterKeywords.__init__(self, self._core)
        ActionKeywords.__init__(self, self._core)
        SessionKeywords.__init__(self, self._core)
        IntrospectionKeywords.__init__(self, self._core)
        MenuKeywords.__init__(self, self._core)

        self._default_timeout = timeout
        self._default_interval = poll_interval
```

### 8. Robot Framework Documentation Generation

The library is designed to be fully LibDoc-compatible with:

1. **Type hints** for all parameters supporting Robot Framework's type conversion
2. **Docstrings** in Robot Framework documentation format
3. **Examples** in every keyword documentation
4. **Tags** for keyword categorization
5. **Argument tables** documenting each parameter

Generate documentation with:

```bash
python -m robot.libdoc SwingLibrary SwingLibrary.html
```

### 9. Keyword Summary Table

| Category | Keyword | Assertion Support | Description |
|----------|---------|-------------------|-------------|
| **Session** | `Connect To Application` | No | Connect to application |
| | `Disconnect` | No | Close connection |
| | `Is Connected` | No | Check connection status |
| **Get/Assert** | `Get Text` | Yes | Get/assert element text |
| | `Get Value` | Yes | Get/assert input value |
| | `Get Element States` | Yes (flag) | Get/assert element states |
| | `Get Element Count` | Yes | Get/assert element count |
| | `Get Property` | Yes | Get/assert element property |
| | `Get Properties` | No | Get all properties |
| | `Get Table Cell Value` | Yes | Get/assert table cell |
| | `Get Table Row Count` | Yes | Get/assert row count |
| | `Get Table Column Count` | Yes | Get/assert column count |
| | `Get Selected Item` | Yes | Get/assert selection |
| | `Get Selected Items` | Yes | Get/assert multi-selection |
| **Action** | `Click` | No | Click element |
| | `Type Text` | No | Type into element |
| | `Clear Text` | No | Clear text field |
| | `Select Item` | No | Select from list/combo/tree |
| | `Set Checkbox` | No | Set checkbox state |
| **Menu** | `Select Menu` | No | Select menu item |
| | `Select Context Menu` | No | Select context menu |
| **Introspection** | `Get UI Tree` | No | Get component tree |
| | `Log UI Tree` | No | Log component tree |
| | `Refresh UI Tree` | No | Refresh cache |
| | `Find Elements` | No | Find matching elements |

## Consequences

### Positive

1. **Browser Library Parity**: Same assertion patterns as Browser Library
2. **Reduced Verbosity**: Inline assertions reduce test code by 30-40%
3. **Built-in Retry**: Automatic retry eliminates explicit wait keywords
4. **Type Safety**: Full type hints for IDE support and validation
5. **LibDoc Support**: Proper documentation generation
6. **Separation of Concerns**: Rust handles GUI, Python handles assertions
7. **Extensibility**: Easy to add new formatters and operators

### Negative

1. **Dependency**: Requires robotframework-assertion-engine package
2. **Python Overhead**: Additional Python layer between RF and Rust
3. **Learning Curve**: Users must learn AssertionEngine operators
4. **Complexity**: Multiple modules to maintain

### Risks

1. **AssertionEngine Compatibility**: Must track upstream changes
2. **Performance**: Python layer adds some overhead
3. **Documentation Sync**: Must keep docs synchronized with implementation
4. **Type Conversion**: Complex types may not convert cleanly

## Alternatives Considered

### Alternative 1: Pure Rust Assertions

Implement assertion logic entirely in Rust.

**Rejected because**:
- Would require reimplementing AssertionEngine in Rust
- Loses compatibility with Browser Library patterns
- Harder to extend and customize

### Alternative 2: No Assertion Integration

Keep simple get/verify keywords separate.

**Rejected because**:
- More verbose tests
- Not aligned with modern Browser Library patterns
- Users expect inline assertions

### Alternative 3: Custom Assertion Engine

Build our own assertion engine.

**Rejected because**:
- Reinventing the wheel
- AssertionEngine is battle-tested
- Harder to maintain

## Implementation Plan

1. **Phase 1**: Core Python structure (2 days)
   - Create module structure
   - Implement ElementState enum
   - Implement formatters and retry

2. **Phase 2**: Getter keywords (3 days)
   - Implement all Get keywords with AssertionEngine
   - Add comprehensive docstrings
   - Unit tests for assertions

3. **Phase 3**: Integration (2 days)
   - Integrate with Rust core bindings
   - Wire up all modules in SwingLibrary

4. **Phase 4**: Documentation (2 days)
   - Generate LibDoc documentation
   - Write user guide for assertion patterns
   - Create migration guide from old keywords

5. **Phase 5**: Testing (3 days)
   - Acceptance tests for all keywords
   - Integration tests with real Swing apps
   - Performance benchmarking

## References

- [ADR-007: Unified Keyword API Design](./ADR-007-UNIFIED-KEYWORD-API.md)
- [ADR-001: Unified Base Class Architecture](./ADR-001-unified-base-class-architecture.md)
- [robotframework-assertion-engine v3.0.3](https://pypi.org/project/robotframework-assertion-engine/)
- [Browser Library Assertions](https://robotframework-browser.org/#assertions)
- [Robot Framework LibDoc](https://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html#libdoc)
- [Python Flag Enum](https://docs.python.org/3/library/enum.html#flag)
