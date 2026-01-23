# ADR-014: AssertionEngine Integration Implementation Plan

| ADR ID | ADR-014 |
|--------|---------|
| Title | AssertionEngine Integration with Claude-Flow Self-Learning |
| Status | Proposed |
| Date | 2026-01-20 |
| Authors | Architecture Team |
| Depends On | ADR-007 (Unified Keyword API), ADR-009 (Implementation Plan) |
| References | [MarketSquare/AssertionEngine](https://github.com/MarketSquare/AssertionEngine), [Browser Library](https://github.com/MarketSquare/robotframework-browser) |

## Context

This ADR details the implementation plan for integrating [robotframework-assertion-engine](https://pypi.org/project/robotframework-assertion-engine/) (v3.0.3) from MarketSquare into robotframework-javagui. The AssertionEngine provides the same assertion capabilities used by Browser Library, enabling inline assertions with retry mechanisms.

### Current State

The library currently has:
- Rust/PyO3 core with Python keyword wrappers (`python/JavaGui/__init__.py`)
- Separate verification keywords (`element_text_should_be`, `element_should_be_visible`, etc.)
- Separate wait keywords (`wait_until_element_exists`, `wait_until_element_is_visible`, etc.)
- Manual polling loops in Python layer for waits

### Target State

- Unified Get keywords with optional inline assertions
- AssertionEngine-powered retry mechanism
- Browser Library-compatible assertion operators
- Claude-flow integration for self-learning optimization
- Reduced keyword count (~60% fewer verification keywords)

### Decision Drivers

- Browser Library pattern adoption (industry standard)
- MarketSquare's proven AssertionEngine implementation
- Reduced test verbosity through inline assertions
- Claude-flow hooks for continuous improvement
- Full backwards compatibility via deprecation aliases

## Decision

We will integrate AssertionEngine in a **5-phase approach** spanning 8 weeks, with Claude-flow hooks at each phase for self-learning and pattern optimization.

---

## Phase 1: Foundation (Weeks 1-2)

### Objective
Establish AssertionEngine dependency and core wrapper infrastructure.

### 1.1 Add AssertionEngine Dependency

**File: `pyproject.toml`**

```toml
[project]
dependencies = [
    "docutils>=0.20.1",
    "robotframework>=4.0",
    "robotframework-assertion-engine>=3.0.0",  # NEW
]
```

**Acceptance Criteria:**
- [ ] `pip install -e .` successfully installs AssertionEngine
- [ ] `from AssertionEngine import verify_assertion` works in Python
- [ ] No version conflicts with existing dependencies

### 1.2 Create Python Keyword Module Structure

**Directory Structure:**
```
python/
+-- JavaGui/
    +-- __init__.py          # Existing
    +-- assertions/          # NEW
    |   +-- __init__.py
    |   +-- engine.py        # AssertionEngine wrapper
    |   +-- operators.py     # Operator aliases
    |   +-- formatters.py    # Custom formatters
    |   +-- states.py        # ElementState enum
    +-- keywords/            # NEW
    |   +-- __init__.py
    |   +-- getters.py       # Get keywords with assertions
    |   +-- actions.py       # Action keywords
    |   +-- configuration.py # Timeout/interval keywords
```

### 1.3 Design ElementState Flag Enum

**File: `python/JavaGui/assertions/states.py`**

```python
"""Element state flags for Get Element States keyword."""

from enum import Flag, auto
from typing import List, Set


class ElementState(Flag):
    """
    Flags representing element states.

    Multiple states can be combined using bitwise OR:
        states = ElementState.VISIBLE | ElementState.ENABLED

    States come in pairs (positive/negative):
    - visible/hidden
    - enabled/disabled
    - focused/unfocused
    - selected/unselected
    - checked/unchecked
    - editable/readonly
    - expanded/collapsed
    - attached/detached
    """
    # Visibility
    VISIBLE = auto()
    HIDDEN = auto()

    # Interactivity
    ENABLED = auto()
    DISABLED = auto()

    # Focus
    FOCUSED = auto()
    UNFOCUSED = auto()

    # Selection (lists, tables, etc.)
    SELECTED = auto()
    UNSELECTED = auto()

    # Check state (checkboxes, toggles)
    CHECKED = auto()
    UNCHECKED = auto()

    # Editability
    EDITABLE = auto()
    READONLY = auto()

    # Tree node expansion
    EXPANDED = auto()
    COLLAPSED = auto()

    # DOM attachment
    ATTACHED = auto()
    DETACHED = auto()

    @classmethod
    def from_string(cls, state_str: str) -> "ElementState":
        """Convert string to ElementState flag."""
        mapping = {
            "visible": cls.VISIBLE,
            "hidden": cls.HIDDEN,
            "enabled": cls.ENABLED,
            "disabled": cls.DISABLED,
            "focused": cls.FOCUSED,
            "unfocused": cls.UNFOCUSED,
            "selected": cls.SELECTED,
            "unselected": cls.UNSELECTED,
            "checked": cls.CHECKED,
            "unchecked": cls.UNCHECKED,
            "editable": cls.EDITABLE,
            "readonly": cls.READONLY,
            "expanded": cls.EXPANDED,
            "collapsed": cls.COLLAPSED,
            "attached": cls.ATTACHED,
            "detached": cls.DETACHED,
        }
        return mapping.get(state_str.lower(), cls(0))

    @classmethod
    def from_strings(cls, states: List[str]) -> "ElementState":
        """Convert list of strings to combined ElementState flags."""
        result = cls(0)
        for state in states:
            result |= cls.from_string(state)
        return result

    def to_strings(self) -> List[str]:
        """Convert flags to list of state strings."""
        result = []
        for state in ElementState:
            if state in self and state.name:
                result.append(state.name.lower())
        return result


def get_element_states_from_properties(
    visible: bool,
    enabled: bool,
    focused: bool = False,
    selected: bool = False,
    checked: bool = None,
    editable: bool = None,
    expanded: bool = None,
) -> ElementState:
    """Convert element properties to ElementState flags."""
    states = ElementState(0)

    states |= ElementState.VISIBLE if visible else ElementState.HIDDEN
    states |= ElementState.ENABLED if enabled else ElementState.DISABLED
    states |= ElementState.FOCUSED if focused else ElementState.UNFOCUSED
    states |= ElementState.SELECTED if selected else ElementState.UNSELECTED

    if checked is not None:
        states |= ElementState.CHECKED if checked else ElementState.UNCHECKED

    if editable is not None:
        states |= ElementState.EDITABLE if editable else ElementState.READONLY

    if expanded is not None:
        states |= ElementState.EXPANDED if expanded else ElementState.COLLAPSED

    states |= ElementState.ATTACHED  # Assume attached if we can query it

    return states
```

### 1.4 Implement Retry Wrapper

**File: `python/JavaGui/assertions/engine.py`**

```python
"""
AssertionEngine wrapper with retry mechanism for robotframework-javagui.

This module provides a thin wrapper around AssertionEngine's verify_assertion
with automatic retry capabilities and Java GUI-specific error handling.
"""

import time
from typing import Any, Callable, Optional, TypeVar, Union
from functools import wraps

from AssertionEngine import (
    verify_assertion,
    AssertionOperator,
)
from AssertionEngine.assertion_engine import float_str_verify_assertion

from .states import ElementState

T = TypeVar("T")


class AssertionConfig:
    """Configuration for assertion behavior."""

    def __init__(
        self,
        timeout: float = 10.0,
        interval: float = 0.25,
        strict_mode: bool = False,
    ):
        """
        Initialize assertion configuration.

        Args:
            timeout: Default timeout for retrying assertions (seconds).
            interval: Polling interval between retry attempts (seconds).
            strict_mode: If True, validate operators are not potentially dangerous.
        """
        self.timeout = timeout
        self.interval = interval
        self.strict_mode = strict_mode

    # Security: List of operators that should be validated in strict mode
    RESTRICTED_OPERATORS = {"validate"}


class JavaGuiAssertionEngine:
    """
    AssertionEngine wrapper tailored for Java GUI automation.

    Provides retry mechanism, custom formatters, and integration with
    Rust/PyO3 element retrieval.
    """

    def __init__(self, config: Optional[AssertionConfig] = None):
        """Initialize with optional configuration."""
        self.config = config or AssertionConfig()
        self._formatters = {}

    def verify_with_retry(
        self,
        getter: Callable[[], T],
        operator: Optional[AssertionOperator],
        expected: Any,
        timeout: Optional[float] = None,
        message: Optional[str] = None,
        prefix_message: str = "",
    ) -> T:
        """
        Execute assertion with retry mechanism.

        Args:
            getter: Callable that retrieves the current value.
            operator: AssertionEngine operator (None to skip assertion).
            expected: Expected value for comparison.
            timeout: Override default timeout (seconds).
            message: Custom failure message.
            prefix_message: Prefix for error messages.

        Returns:
            The retrieved value (last successful retrieval).

        Raises:
            AssertionError: If assertion fails after all retries.
            ElementNotFoundError: If element cannot be found.
        """
        if operator is None:
            # No assertion, just return the value
            return getter()

        # Security check in strict mode
        if self.config.strict_mode:
            self._validate_operator(operator)

        effective_timeout = timeout if timeout is not None else self.config.timeout
        interval = self.config.interval
        end_time = time.time() + effective_timeout
        last_value = None
        last_error = None

        while time.time() < end_time:
            try:
                last_value = getter()

                # Apply formatters
                formatted_value = self._apply_formatters(last_value)

                # Attempt assertion (returns value on success, raises on failure)
                verify_assertion(
                    formatted_value,
                    operator,
                    expected,
                    prefix_message,
                    message,
                )
                return last_value  # Success

            except AssertionError as e:
                last_error = e
                # Continue retrying
            except Exception as e:
                # Element not found or other retrieval error
                last_error = e

            time.sleep(interval)

        # All retries exhausted
        self._raise_timeout_error(
            operator, expected, last_value, last_error, effective_timeout, message
        )

    def verify_states_with_retry(
        self,
        getter: Callable[[], ElementState],
        operator: Optional[AssertionOperator],
        expected: Union[str, list, ElementState],
        timeout: Optional[float] = None,
        message: Optional[str] = None,
    ) -> ElementState:
        """
        Verify element states with retry mechanism.

        Specialized for ElementState flag assertions.
        """
        if operator is None:
            return getter()

        # Convert expected to ElementState if needed
        if isinstance(expected, str):
            expected_states = ElementState.from_string(expected)
        elif isinstance(expected, list):
            expected_states = ElementState.from_strings(expected)
        else:
            expected_states = expected

        def state_getter():
            states = getter()
            return states.to_strings()  # Convert to list for assertion

        expected_list = expected_states.to_strings()

        return self.verify_with_retry(
            state_getter,
            operator,
            expected_list,
            timeout,
            message,
            "Element states",
        )

    def verify_count_with_retry(
        self,
        getter: Callable[[], int],
        operator: Optional[AssertionOperator],
        expected: Union[int, str],
        timeout: Optional[float] = None,
        message: Optional[str] = None,
    ) -> int:
        """
        Verify element count with retry mechanism.

        Uses float_str_verify_assertion for numeric comparisons.
        """
        if operator is None:
            return getter()

        effective_timeout = timeout if timeout is not None else self.config.timeout
        interval = self.config.interval
        end_time = time.time() + effective_timeout
        last_value = None
        last_error = None

        while time.time() < end_time:
            try:
                last_value = getter()

                # Use numeric assertion
                float_str_verify_assertion(
                    last_value,
                    operator,
                    expected,
                    "Element count",
                    message,
                )
                return last_value

            except AssertionError as e:
                last_error = e
            except Exception as e:
                last_error = e

            time.sleep(interval)

        self._raise_timeout_error(
            operator, expected, last_value, last_error, effective_timeout, message
        )

    def _validate_operator(self, operator: AssertionOperator) -> None:
        """Validate operator is allowed in strict mode."""
        if operator.name.lower() in self.config.RESTRICTED_OPERATORS:
            raise ValueError(
                f"Operator '{operator.name}' is restricted in strict mode. "
                f"Set strict_mode=False to enable or use a different operator."
            )

    def _apply_formatters(self, value: Any) -> Any:
        """Apply registered formatters to value."""
        result = value
        for formatter in self._formatters.values():
            result = formatter(result)
        return result

    def _raise_timeout_error(
        self,
        operator: AssertionOperator,
        expected: Any,
        last_value: Any,
        last_error: Exception,
        timeout: float,
        message: Optional[str],
    ) -> None:
        """Raise detailed timeout error."""
        error_parts = []

        if message:
            error_parts.append(message)

        error_parts.append(f"Assertion failed after {timeout}s timeout")
        error_parts.append(f"  Operator: {operator.name if operator else 'None'}")
        error_parts.append(f"  Expected: {expected!r}")
        error_parts.append(f"  Actual:   {last_value!r}")

        if last_error and not isinstance(last_error, AssertionError):
            error_parts.append(f"  Last error: {last_error}")

        raise AssertionError("\n".join(error_parts))

    def set_formatter(self, name: str, formatter: Callable[[Any], Any]) -> None:
        """Register a custom formatter."""
        self._formatters[name] = formatter

    def remove_formatter(self, name: str) -> None:
        """Remove a registered formatter."""
        self._formatters.pop(name, None)

    def clear_formatters(self) -> None:
        """Remove all formatters."""
        self._formatters.clear()


# Global instance for library use
_assertion_engine: Optional[JavaGuiAssertionEngine] = None


def get_assertion_engine() -> JavaGuiAssertionEngine:
    """Get or create the global assertion engine instance."""
    global _assertion_engine
    if _assertion_engine is None:
        _assertion_engine = JavaGuiAssertionEngine()
    return _assertion_engine


def configure_assertion_engine(
    timeout: Optional[float] = None,
    interval: Optional[float] = None,
    strict_mode: Optional[bool] = None,
) -> None:
    """Configure the global assertion engine."""
    engine = get_assertion_engine()
    if timeout is not None:
        engine.config.timeout = timeout
    if interval is not None:
        engine.config.interval = interval
    if strict_mode is not None:
        engine.config.strict_mode = strict_mode
```

### 1.5 Claude-Flow Integration: Phase 1

```bash
# Before starting Phase 1
npx @claude-flow/cli@latest hooks pre-task \
    --task-id "adr014-phase1" \
    --description "Implement AssertionEngine foundation - dependency, module structure, retry wrapper"

# Search for existing patterns
npx @claude-flow/cli@latest memory search \
    --query "assertionengine verify_assertion retry wrapper patterns" \
    --namespace patterns

# After completing Phase 1
npx @claude-flow/cli@latest hooks post-task \
    --task-id "adr014-phase1" \
    --success true \
    --quality 0.9

# Store successful patterns
npx @claude-flow/cli@latest memory store \
    --namespace patterns \
    --key "assertion-retry-wrapper" \
    --value "Use time.time() + timeout for deadline, getter callable pattern, separate verify functions for states/counts"

npx @claude-flow/cli@latest hooks post-edit \
    --file "python/JavaGui/assertions/engine.py" \
    --success true \
    --train-neural true
```

### Phase 1 Quality Gate

| Metric | Target | Measurement |
|--------|--------|-------------|
| AssertionEngine imports | Working | Unit test |
| ElementState enum coverage | 100% | pytest-cov |
| Retry wrapper tests | 95%+ | pytest-cov |
| No security vulnerabilities | 0 critical | `safety check` |

---

## Phase 2: Core Get Keywords (Weeks 3-4)

### Objective
Implement the 6 core Get keywords with AssertionEngine integration.

### 2.1 Implement Get Text with verify_assertion

**File: `python/JavaGui/keywords/getters.py`**

```python
"""
Get keywords with AssertionEngine support.

These keywords follow Browser Library patterns for inline assertions.
"""

from typing import Any, Dict, List, Optional, Union

from AssertionEngine import AssertionOperator

from ..assertions.engine import get_assertion_engine, JavaGuiAssertionEngine
from ..assertions.states import ElementState, get_element_states_from_properties


class GetterKeywords:
    """
    Keywords for retrieving element values with optional assertions.

    All Get keywords support inline assertions via the operator parameter.
    When operator is provided, the keyword performs assertion with retry.
    When operator is None, the keyword simply returns the value.
    """

    def __init__(self, library):
        """
        Initialize with reference to main library.

        Args:
            library: SwingLibrary, SwtLibrary, or RcpLibrary instance.
        """
        self._lib = library
        self._engine = get_assertion_engine()

    def get_text(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        assertion_expected: Optional[str] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> str:
        """Get text content of an element, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``assertion_expected`` | Expected value for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns the element's text content.

        When ``assertion_operator`` is provided, performs assertion with retry:
        | `Get Text` | JLabel#status | == | Ready |
        | `Get Text` | JLabel#msg | contains | success |

        Without operator, just returns the value:
        | ${text}= | `Get Text` | JLabel#status |

        == Assertion Operators ==

        | *Operator* | *Description* | *Example* |
        | == | Exact equality | `Get Text` | loc | == | value |
        | != | Not equal | `Get Text` | loc | != | error |
        | contains | Contains substring | `Get Text` | loc | contains | success |
        | *not contains | Does not contain | `Get Text` | loc | *not contains | error |
        | matches | Regex match | `Get Text` | loc | matches | ^Ready.* |
        | starts | Starts with | `Get Text` | loc | starts | Loading |
        | ends | Ends with | `Get Text` | loc | ends | complete |
        | validate | Custom validator | Advanced use only |

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
        def getter():
            return self._lib._lib.get_element_text(locator)

        return self._engine.verify_with_retry(
            getter=getter,
            operator=assertion_operator,
            expected=assertion_expected,
            timeout=timeout,
            message=message,
            prefix_message=f"Text of '{locator}'",
        )

    def get_value(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        assertion_expected: Optional[Any] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> Any:
        """Get the value of an input element, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``assertion_expected`` | Expected value for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns the element's current value (text field content, selected item, etc.).

        Examples:
        | ${value}= | `Get Value` | JTextField#username |
        | `Get Value` | JTextField#email | contains | @ |
        | `Get Value` | JComboBox#country | == | USA |
        | `Get Value` | JSpinner#count | >= | 5 |
        """
        def getter():
            # Try text property first, fall back to other value sources
            try:
                return self._lib._lib.get_element_property(locator, "text")
            except Exception:
                try:
                    return self._lib._lib.get_element_property(locator, "selectedItem")
                except Exception:
                    return self._lib._lib.get_element_property(locator, "value")

        return self._engine.verify_with_retry(
            getter=getter,
            operator=assertion_operator,
            expected=assertion_expected,
            timeout=timeout,
            message=message,
            prefix_message=f"Value of '{locator}'",
        )

    def get_element_count(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        assertion_expected: Optional[Union[int, str]] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> int:
        """Get count of elements matching locator, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``assertion_expected`` | Expected count for assertion. |
        | ``message`` | Custom assertion failure message. |
        | ``timeout`` | Assertion retry timeout in seconds. |

        Returns the number of elements matching the locator.

        Supports numeric comparison operators:
        | *Operator* | *Description* |
        | == | Equals |
        | != | Not equals |
        | > | Greater than |
        | >= | Greater than or equal |
        | < | Less than |
        | <= | Less than or equal |

        Examples:
        | ${count}= | `Get Element Count` | JButton |
        | `Get Element Count` | JButton | == | 5 |
        | `Get Element Count` | JTable >> JTableRow | >= | 1 |
        | `Get Element Count` | JTree >> JTreeNode | > | 0 |
        """
        def getter():
            elements = self._lib._lib.find_elements(locator)
            return len(elements) if elements else 0

        return self._engine.verify_count_with_retry(
            getter=getter,
            operator=assertion_operator,
            expected=assertion_expected,
            timeout=timeout,
            message=message,
        )

    def get_element_states(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        assertion_expected: Optional[Union[str, List[str]]] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> List[str]:
        """Get element states, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``assertion_expected`` | Expected states for assertion. |
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
        - ``attached`` / ``detached``

        State assertions use ``contains`` semantics:
        | `Get Element States` | JButton#submit | contains | enabled |
        | `Get Element States` | JButton#submit | contains | ['enabled', 'visible'] |
        | `Get Element States` | JButton#delete | *not contains | enabled |

        Examples:
        | ${states}= | `Get Element States` | JButton#submit |
        | `Get Element States` | JButton#submit | contains | enabled |
        | `Get Element States` | JCheckBox#opt | contains | checked |
        """
        def getter():
            props = {}
            for prop in ["visible", "enabled", "focused", "selected", "editable"]:
                try:
                    props[prop] = self._lib._lib.get_element_property(locator, prop)
                except Exception:
                    pass

            # Get checked state for checkboxes
            try:
                props["checked"] = self._lib._lib.get_element_property(locator, "selected")
            except Exception:
                pass

            states = get_element_states_from_properties(
                visible=props.get("visible", True),
                enabled=props.get("enabled", True),
                focused=props.get("focused", False),
                selected=props.get("selected", False),
                checked=props.get("checked"),
                editable=props.get("editable"),
            )
            return states

        states = self._engine.verify_states_with_retry(
            getter=getter,
            operator=assertion_operator,
            expected=assertion_expected,
            timeout=timeout,
            message=message,
        )

        if isinstance(states, ElementState):
            return states.to_strings()
        return states

    def get_property(
        self,
        locator: str,
        property_name: str,
        assertion_operator: Optional[AssertionOperator] = None,
        assertion_expected: Optional[Any] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> Any:
        """Get a specific property of an element, optionally with assertion.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |
        | ``property_name`` | Property to retrieve. |
        | ``assertion_operator`` | Assertion operator. See `Assertion Operators`. |
        | ``assertion_expected`` | Expected value for assertion. |
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
        | `Get Property` | JButton#submit | enabled | == | True |
        | `Get Property` | JTable#data | rowCount | >= | 10 |
        | `Get Property` | JTable#data | cellValue[0,1] | == | John |
        | `Get Property` | JComboBox | selectedIndex | == | 2 |
        """
        # Handle special cellValue[row,col] syntax
        if property_name.startswith("cellValue["):
            # Parse cellValue[row,col]
            import re
            match = re.match(r"cellValue\[(\d+),(\d+)\]", property_name)
            if match:
                row, col = int(match.group(1)), int(match.group(2))
                def getter():
                    return self._lib._lib.get_table_cell_value(locator, row, str(col))
            else:
                raise ValueError(f"Invalid cellValue syntax: {property_name}")
        else:
            def getter():
                return self._lib._lib.get_element_property(locator, property_name)

        return self._engine.verify_with_retry(
            getter=getter,
            operator=assertion_operator,
            expected=assertion_expected,
            timeout=timeout,
            message=message,
            prefix_message=f"Property '{property_name}' of '{locator}'",
        )

    def get_properties(
        self,
        locator: str,
    ) -> Dict[str, Any]:
        """Get all common properties of an element.

        | =Argument= | =Description= |
        | ``locator`` | Element locator. See `Locator Syntax`. |

        Returns a dictionary containing all retrievable properties.

        Examples:
        | ${props}= | `Get Properties` | JButton#submit |
        | Log | Text: ${props}[text], Enabled: ${props}[enabled] |
        """
        properties = {}
        standard_props = [
            "name", "text", "enabled", "visible", "selected",
            "editable", "focused", "bounds", "className",
        ]

        for prop in standard_props:
            try:
                properties[prop] = self._lib._lib.get_element_property(locator, prop)
            except Exception:
                pass

        return properties
```

### 2.2 Claude-Flow Integration: Phase 2

```bash
# Before starting Phase 2
npx @claude-flow/cli@latest hooks pre-task \
    --task-id "adr014-phase2" \
    --description "Implement core Get keywords with verify_assertion integration"

# Route task to optimal agent
npx @claude-flow/cli@latest hooks route \
    --task "Implement Get Text, Get Value, Get Element Count with AssertionEngine"

# After each keyword implementation
npx @claude-flow/cli@latest hooks post-edit \
    --file "python/JavaGui/keywords/getters.py" \
    --success true \
    --train-neural true

# After completing Phase 2
npx @claude-flow/cli@latest hooks post-task \
    --task-id "adr014-phase2" \
    --success true \
    --quality 0.92

# Store patterns for future reference
npx @claude-flow/cli@latest memory store \
    --namespace patterns \
    --key "get-keyword-pattern" \
    --value "getter function captures locator in closure, engine.verify_with_retry handles assertion, prefix_message provides context"

# Trigger optimization worker
npx @claude-flow/cli@latest hooks worker dispatch --trigger optimize
```

### Phase 2 Quality Gate

| Metric | Target | Measurement |
|--------|--------|-------------|
| Get Text coverage | 95%+ | pytest-cov |
| Get Value coverage | 95%+ | pytest-cov |
| Get Element Count coverage | 95%+ | pytest-cov |
| Get Element States coverage | 95%+ | pytest-cov |
| Get Property coverage | 95%+ | pytest-cov |
| All operators tested | 100% | Test matrix |

---

## Phase 3: Advanced Keywords (Weeks 5-6)

### Objective
Implement table, tree, and list keywords with assertions and formatters.

### 3.1 Table Keywords

**File: `python/JavaGui/keywords/tables.py`**

```python
"""Table keywords with AssertionEngine support."""

from typing import Any, List, Optional, Union
from AssertionEngine import AssertionOperator
from ..assertions.engine import get_assertion_engine


class TableKeywords:
    """Keywords for table operations with assertions."""

    def __init__(self, library):
        self._lib = library
        self._engine = get_assertion_engine()

    def get_table_cell(
        self,
        locator: str,
        row: int,
        column: Union[int, str],
        assertion_operator: Optional[AssertionOperator] = None,
        assertion_expected: Optional[str] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> str:
        """Get table cell value with optional assertion.

        | =Argument= | =Description= |
        | ``locator`` | Table locator. |
        | ``row`` | Row index (0-based). |
        | ``column`` | Column index (0-based) or column name. |
        | ``assertion_operator`` | Assertion operator. |
        | ``assertion_expected`` | Expected value. |
        | ``message`` | Custom error message. |
        | ``timeout`` | Assertion timeout. |

        Examples:
        | ${value}= | `Get Table Cell` | JTable | 0 | 1 |
        | `Get Table Cell` | JTable | 0 | Name | == | John |
        """
        def getter():
            return self._lib._lib.get_table_cell_value(locator, row, str(column))

        return self._engine.verify_with_retry(
            getter=getter,
            operator=assertion_operator,
            expected=assertion_expected,
            timeout=timeout,
            message=message,
            prefix_message=f"Cell [{row},{column}] of '{locator}'",
        )

    def get_table_row_count(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        assertion_expected: Optional[Union[int, str]] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> int:
        """Get table row count with optional assertion.

        Examples:
        | ${count}= | `Get Table Row Count` | JTable |
        | `Get Table Row Count` | JTable | >= | 10 |
        """
        def getter():
            return self._lib._lib.get_table_row_count(locator)

        return self._engine.verify_count_with_retry(
            getter=getter,
            operator=assertion_operator,
            expected=assertion_expected,
            timeout=timeout,
            message=message,
        )

    def get_table_column_count(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        assertion_expected: Optional[Union[int, str]] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> int:
        """Get table column count with optional assertion."""
        def getter():
            return self._lib._lib.get_table_column_count(locator)

        return self._engine.verify_count_with_retry(
            getter=getter,
            operator=assertion_operator,
            expected=assertion_expected,
            timeout=timeout,
            message=message,
        )
```

### 3.2 Tree Keywords with Path Assertions

**File: `python/JavaGui/keywords/trees.py`**

```python
"""Tree keywords with AssertionEngine support."""

from typing import List, Optional
from AssertionEngine import AssertionOperator
from ..assertions.engine import get_assertion_engine


class TreeKeywords:
    """Keywords for tree operations with assertions."""

    def __init__(self, library):
        self._lib = library
        self._engine = get_assertion_engine()

    def get_selected_tree_node(
        self,
        locator: str,
        assertion_operator: Optional[AssertionOperator] = None,
        assertion_expected: Optional[str] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> str:
        """Get selected tree node path with optional assertion.

        Examples:
        | ${path}= | `Get Selected Tree Node` | JTree |
        | `Get Selected Tree Node` | JTree | == | Root/Documents |
        | `Get Selected Tree Node` | JTree | contains | Documents |
        """
        def getter():
            return self._lib._lib.get_selected_tree_node(locator) or ""

        return self._engine.verify_with_retry(
            getter=getter,
            operator=assertion_operator,
            expected=assertion_expected,
            timeout=timeout,
            message=message,
            prefix_message=f"Selected node of '{locator}'",
        )

    def get_tree_node_count(
        self,
        locator: str,
        path: Optional[str] = None,
        assertion_operator: Optional[AssertionOperator] = None,
        assertion_expected: Optional[int] = None,
        message: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> int:
        """Get count of child nodes with optional assertion.

        Examples:
        | ${count}= | `Get Tree Node Count` | JTree | Root |
        | `Get Tree Node Count` | JTree | Root | >= | 3 |
        """
        def getter():
            nodes = self._lib._lib.get_tree_data(locator)
            if path:
                # Navigate to specified path and count children
                parts = path.replace("|", "/").split("/")
                current = nodes
                for part in parts:
                    if current and "children" in current:
                        current = next(
                            (c for c in current["children"] if c.get("text") == part),
                            None
                        )
                if current and "children" in current:
                    return len(current["children"])
                return 0
            return len(nodes.get("children", [])) if nodes else 0

        return self._engine.verify_count_with_retry(
            getter=getter,
            operator=assertion_operator,
            expected=assertion_expected,
            timeout=timeout,
            message=message,
        )
```

### 3.3 Formatter Support

**File: `python/JavaGui/assertions/formatters.py`**

```python
"""Custom formatters for AssertionEngine."""

import re
from typing import Any


def normalize_spaces(value: Any) -> Any:
    """Substitute multiple spaces with single space."""
    if isinstance(value, str):
        return re.sub(r"\s+", " ", value)
    return value


def strip(value: Any) -> Any:
    """Remove leading/trailing whitespace."""
    if isinstance(value, str):
        return value.strip()
    return value


def case_insensitive(value: Any) -> Any:
    """Convert to lowercase for case-insensitive comparison."""
    if isinstance(value, str):
        return value.lower()
    return value


def strip_newlines(value: Any) -> Any:
    """Replace newlines with spaces."""
    if isinstance(value, str):
        return value.replace("\n", " ").replace("\r", "")
    return value


# Formatter registry
FORMATTERS = {
    "normalize_spaces": normalize_spaces,
    "strip": strip,
    "case_insensitive": case_insensitive,
    "strip_newlines": strip_newlines,
}


def apply_formatters(value: Any, formatter_names: list) -> Any:
    """Apply multiple formatters in sequence."""
    result = value
    for name in formatter_names:
        if name in FORMATTERS:
            result = FORMATTERS[name](result)
    return result
```

### 3.4 Claude-Flow Integration: Phase 3

```bash
# Store table/tree patterns
npx @claude-flow/cli@latest memory store \
    --namespace patterns \
    --key "table-assertion-pattern" \
    --value "Use cellValue[row,col] syntax for property access, getter captures row/col in closure"

npx @claude-flow/cli@latest memory store \
    --namespace patterns \
    --key "tree-path-assertion" \
    --value "Normalize path separators (| to /), navigate recursively for child counts"

# Train neural on successful implementations
npx @claude-flow/cli@latest neural train \
    --pattern-type keyword-impl \
    --epochs 10
```

### Phase 3 Quality Gate

| Metric | Target | Measurement |
|--------|--------|-------------|
| Table keyword coverage | 90%+ | pytest-cov |
| Tree keyword coverage | 90%+ | pytest-cov |
| Formatter tests | 100% | pytest |
| Integration tests pass | 100% | Robot tests |

---

## Phase 4: Configuration & Polish (Week 7)

### Objective
Implement configuration keywords, security controls, and documentation.

### 4.1 Set Assertion Timeout Keyword

**File: `python/JavaGui/keywords/configuration.py`**

```python
"""Configuration keywords for assertion behavior."""

from typing import Optional
from ..assertions.engine import get_assertion_engine, configure_assertion_engine


class ConfigurationKeywords:
    """Keywords for configuring assertion behavior."""

    def __init__(self, library):
        self._lib = library
        self._engine = get_assertion_engine()

    def set_assertion_timeout(self, timeout: float) -> float:
        """Set the default timeout for assertions.

        | =Argument= | =Description= |
        | ``timeout`` | Timeout in seconds for assertion retries. |

        Returns the previous timeout value.

        Example:
        | ${old}= | `Set Assertion Timeout` | 30 |
        | # ... tests with 30s timeout ... |
        | `Set Assertion Timeout` | ${old} |
        """
        old_timeout = self._engine.config.timeout
        self._engine.config.timeout = timeout
        return old_timeout

    def set_assertion_interval(self, interval: float) -> float:
        """Set the polling interval for assertion retries.

        | =Argument= | =Description= |
        | ``interval`` | Polling interval in seconds between retries. |

        Returns the previous interval value.

        Example:
        | ${old}= | `Set Assertion Interval` | 0.5 |
        """
        old_interval = self._engine.config.interval
        self._engine.config.interval = interval
        return old_interval

    def set_assertion_strict_mode(self, strict: bool) -> bool:
        """Enable or disable strict mode for operator validation.

        In strict mode, potentially dangerous operators like ``validate``
        are restricted.

        | =Argument= | =Description= |
        | ``strict`` | ``True`` to enable strict mode, ``False`` to disable. |

        Returns the previous strict mode setting.

        Example:
        | `Set Assertion Strict Mode` | True |
        """
        old_strict = self._engine.config.strict_mode
        self._engine.config.strict_mode = strict
        return old_strict

    def get_assertion_config(self) -> dict:
        """Get current assertion configuration.

        Returns a dictionary with current settings.

        Example:
        | ${config}= | `Get Assertion Config` |
        | Log | Timeout: ${config}[timeout], Interval: ${config}[interval] |
        """
        return {
            "timeout": self._engine.config.timeout,
            "interval": self._engine.config.interval,
            "strict_mode": self._engine.config.strict_mode,
        }
```

### 4.2 Security Configuration

**File: `python/JavaGui/assertions/security.py`**

```python
"""Security controls for AssertionEngine integration."""

from typing import Set
from AssertionEngine import AssertionOperator


class AssertionSecurityValidator:
    """
    Validates assertion operators for security concerns.

    The `validate` operator allows arbitrary Python code execution
    and should be restricted in production environments.
    """

    # Operators that execute arbitrary code
    CODE_EXECUTION_OPERATORS: Set[str] = {"validate"}

    # Operators that may leak information via timing
    TIMING_SENSITIVE_OPERATORS: Set[str] = {"matches", "validate"}

    @classmethod
    def is_safe_operator(cls, operator: AssertionOperator) -> bool:
        """Check if operator is safe for general use."""
        if operator is None:
            return True
        return operator.name.lower() not in cls.CODE_EXECUTION_OPERATORS

    @classmethod
    def validate_operator(
        cls,
        operator: AssertionOperator,
        allow_code_execution: bool = False,
    ) -> None:
        """
        Validate operator is allowed given security settings.

        Raises:
            SecurityError: If operator is restricted.
        """
        if operator is None:
            return

        op_name = operator.name.lower()

        if not allow_code_execution and op_name in cls.CODE_EXECUTION_OPERATORS:
            raise SecurityError(
                f"Operator '{operator.name}' is restricted because it allows "
                f"code execution. Set allow_code_execution=True to enable, "
                f"or use a safer operator like 'matches' for regex validation."
            )


class SecurityError(Exception):
    """Raised when a security restriction is violated."""
    pass
```

### 4.3 Documentation Generation

```bash
# Generate keyword documentation
python -m robot.libdoc JavaGui.Swing docs/keywords/Swing.html
python -m robot.libdoc JavaGui.Swt docs/keywords/Swt.html
python -m robot.libdoc JavaGui.Rcp docs/keywords/Rcp.html

# Claude-flow: Dispatch documentation worker
npx @claude-flow/cli@latest hooks worker dispatch --trigger document
```

### 4.4 Claude-Flow Integration: Phase 4

```bash
# Store security patterns
npx @claude-flow/cli@latest memory store \
    --namespace patterns \
    --key "assertion-security" \
    --value "Restrict validate operator in strict mode, log all operator usage for audit"

# Dispatch documentation worker
npx @claude-flow/cli@latest hooks worker dispatch \
    --trigger document \
    --context "python/JavaGui/"

# Run security audit
npx @claude-flow/cli@latest hooks worker dispatch \
    --trigger audit \
    --priority critical \
    --context "python/JavaGui/assertions/"
```

### Phase 4 Quality Gate

| Metric | Target | Measurement |
|--------|--------|-------------|
| Configuration keywords | Implemented | Manual review |
| Security controls | Reviewed | Security audit |
| Documentation generated | Complete | libdoc output |
| No security warnings | 0 | `safety check` |

---

## Phase 5: Migration Support (Week 8)

### Objective
Provide backwards compatibility and migration path for existing users.

### 5.1 Deprecation Warnings

**File: `python/JavaGui/assertions/deprecation.py`**

```python
"""Deprecation warnings for migrating to AssertionEngine patterns."""

import warnings
from functools import wraps
from typing import Callable


def deprecated_keyword(
    old_name: str,
    new_name: str,
    new_usage: str,
) -> Callable:
    """
    Decorator to mark keywords as deprecated.

    Args:
        old_name: Name of deprecated keyword.
        new_name: Name of replacement keyword.
        new_usage: Example of new usage pattern.
    """
    def decorator(func: Callable) -> Callable:
        @wraps(func)
        def wrapper(*args, **kwargs):
            warnings.warn(
                f"Keyword '{old_name}' is deprecated. "
                f"Use '{new_name}' instead.\n"
                f"Example: {new_usage}",
                DeprecationWarning,
                stacklevel=2,
            )
            return func(*args, **kwargs)
        return wrapper
    return decorator


# Mapping of old verification keywords to new Get keyword patterns
DEPRECATED_KEYWORDS = {
    "Element Text Should Be": {
        "new": "Get Text",
        "usage": "Get Text    locator    ==    expected_text",
    },
    "Element Text Should Contain": {
        "new": "Get Text",
        "usage": "Get Text    locator    contains    substring",
    },
    "Element Should Be Visible": {
        "new": "Get Element States",
        "usage": "Get Element States    locator    contains    visible",
    },
    "Element Should Not Be Visible": {
        "new": "Get Element States",
        "usage": "Get Element States    locator    *not contains    visible",
    },
    "Element Should Be Enabled": {
        "new": "Get Element States",
        "usage": "Get Element States    locator    contains    enabled",
    },
    "Element Should Be Disabled": {
        "new": "Get Element States",
        "usage": "Get Element States    locator    *not contains    enabled",
    },
    "Wait Until Element Exists": {
        "new": "Get Element Count",
        "usage": "Get Element Count    locator    >=    1    timeout=30",
    },
    "Wait Until Element Is Visible": {
        "new": "Get Element States",
        "usage": "Get Element States    locator    contains    visible    timeout=30",
    },
    "Wait Until Element Is Enabled": {
        "new": "Get Element States",
        "usage": "Get Element States    locator    contains    enabled    timeout=30",
    },
}
```

### 5.2 Keyword Aliases

```python
# In SwingLibrary class - backwards compatibility aliases

@deprecated_keyword(
    "Element Text Should Be",
    "Get Text",
    "Get Text    locator    ==    expected",
)
def element_text_should_be(self, locator: str, expected: str) -> None:
    """Deprecated: Use Get Text with == operator instead."""
    from AssertionEngine import AssertionOperator
    self._getters.get_text(locator, AssertionOperator["=="], expected)

@deprecated_keyword(
    "Element Text Should Contain",
    "Get Text",
    "Get Text    locator    contains    substring",
)
def element_text_should_contain(self, locator: str, expected: str) -> None:
    """Deprecated: Use Get Text with contains operator instead."""
    from AssertionEngine import AssertionOperator
    self._getters.get_text(locator, AssertionOperator["contains"], expected)

@deprecated_keyword(
    "Wait Until Element Exists",
    "Get Element Count",
    "Get Element Count    locator    >=    1    timeout=30",
)
def wait_until_element_exists(
    self, locator: str, timeout: Optional[float] = None
) -> None:
    """Deprecated: Use Get Element Count with timeout instead."""
    from AssertionEngine import AssertionOperator
    self._getters.get_element_count(
        locator, AssertionOperator[">="], 1, timeout=timeout
    )
```

### 5.3 Migration Guide

**File: `docs/migration/ASSERTION-ENGINE-MIGRATION.md`**

```markdown
# Migration Guide: AssertionEngine Integration

## Quick Migration Table

| Old Keyword | New Keyword | Example |
|-------------|-------------|---------|
| `Element Text Should Be` | `Get Text` | `Get Text    loc    ==    expected` |
| `Element Text Should Contain` | `Get Text` | `Get Text    loc    contains    substring` |
| `Element Should Be Visible` | `Get Element States` | `Get Element States    loc    contains    visible` |
| `Element Should Be Enabled` | `Get Element States` | `Get Element States    loc    contains    enabled` |
| `Wait Until Element Exists` | `Get Element Count` | `Get Element Count    loc    >=    1    timeout=30` |
| `Wait Until Element Is Visible` | `Get Element States` | `Get Element States    loc    contains    visible    timeout=30` |

## Benefits of Migration

1. **Reduced verbosity**: Combine get and assert in one keyword
2. **Built-in retry**: No need for explicit wait keywords
3. **Consistent API**: Same patterns as Browser Library
4. **Better errors**: Rich assertion failure messages

## Example Transformation

### Before (Old Pattern)
```robot
${text}=    Get Element Text    JLabel#status
Should Be Equal    ${text}    Ready
Wait Until Element Is Visible    JButton#submit
Element Should Be Enabled    JButton#submit
```

### After (New Pattern)
```robot
Get Text    JLabel#status    ==    Ready
Get Element States    JButton#submit    contains    ['visible', 'enabled']    timeout=30
```
```

### 5.4 Claude-Flow Integration: Phase 5

```bash
# Dispatch test gap analysis
npx @claude-flow/cli@latest hooks worker dispatch \
    --trigger testgaps \
    --context "tests/"

# Store migration patterns
npx @claude-flow/cli@latest memory store \
    --namespace patterns \
    --key "assertion-migration" \
    --value "Map verification keywords to Get+operator, map waits to Get+timeout"

# Final neural training
npx @claude-flow/cli@latest neural train \
    --pattern-type coordination \
    --epochs 15

# Session end with metrics export
npx @claude-flow/cli@latest hooks session-end \
    --generate-summary true \
    --export-metrics true
```

### Phase 5 Quality Gate

| Metric | Target | Measurement |
|--------|--------|-------------|
| Deprecation warnings work | Verified | Manual test |
| All aliases function | 100% | Integration tests |
| Migration guide complete | Reviewed | Documentation |
| Test coverage overall | 90%+ | pytest-cov |

---

## Testing Strategy

### Unit Tests: AssertionEngine Wrapper

```python
# tests/python/test_assertion_engine.py

import pytest
from JavaGui.assertions.engine import (
    JavaGuiAssertionEngine,
    AssertionConfig,
)
from AssertionEngine import AssertionOperator


class TestJavaGuiAssertionEngine:
    """Tests for AssertionEngine wrapper."""

    def test_verify_without_operator_returns_value(self):
        """No operator means just return the value."""
        engine = JavaGuiAssertionEngine()
        getter = lambda: "test value"

        result = engine.verify_with_retry(getter, None, None)
        assert result == "test value"

    def test_verify_with_equals_operator_passes(self):
        """Equals operator with matching value passes."""
        engine = JavaGuiAssertionEngine()
        getter = lambda: "expected"

        result = engine.verify_with_retry(
            getter, AssertionOperator["=="], "expected"
        )
        assert result == "expected"

    def test_verify_with_equals_operator_fails(self):
        """Equals operator with non-matching value fails."""
        engine = JavaGuiAssertionEngine(
            AssertionConfig(timeout=0.1, interval=0.05)
        )
        getter = lambda: "actual"

        with pytest.raises(AssertionError) as exc_info:
            engine.verify_with_retry(
                getter, AssertionOperator["=="], "expected"
            )

        assert "expected" in str(exc_info.value)
        assert "actual" in str(exc_info.value)

    def test_retry_succeeds_on_eventual_match(self):
        """Retry mechanism succeeds when value eventually matches."""
        engine = JavaGuiAssertionEngine(
            AssertionConfig(timeout=1.0, interval=0.1)
        )

        call_count = [0]
        def getter():
            call_count[0] += 1
            if call_count[0] < 3:
                return "loading"
            return "ready"

        result = engine.verify_with_retry(
            getter, AssertionOperator["=="], "ready"
        )
        assert result == "ready"
        assert call_count[0] >= 3

    def test_strict_mode_blocks_validate_operator(self):
        """Strict mode prevents validate operator."""
        engine = JavaGuiAssertionEngine(
            AssertionConfig(strict_mode=True)
        )
        getter = lambda: "value"

        with pytest.raises(ValueError) as exc_info:
            engine.verify_with_retry(
                getter, AssertionOperator["validate"], lambda x: True
            )

        assert "restricted" in str(exc_info.value).lower()
```

### Integration Tests: Full Keyword Execution

```robot
*** Settings ***
Library    JavaGui.Swing    timeout=5
Suite Setup    Connect To Application    swing-test-app
Suite Teardown    Disconnect

*** Test Cases ***
Test Get Text Without Assertion
    ${text}=    Get Text    JLabel#status
    Should Not Be Empty    ${text}

Test Get Text With Equals Assertion
    Get Text    JLabel#welcomeLabel    ==    Welcome

Test Get Text With Contains Assertion
    Get Text    JLabel#messageLabel    contains    Hello

Test Get Element Count
    Get Element Count    JButton    >=    1

Test Get Element States Contains Enabled
    Get Element States    JButton#submit    contains    enabled

Test Get Property Row Count
    Get Property    JTable#dataTable    rowCount    >=    5

Test Assertion Timeout Configuration
    ${old}=    Set Assertion Timeout    30
    ${config}=    Get Assertion Config
    Should Be Equal As Numbers    ${config}[timeout]    30
    Set Assertion Timeout    ${old}

Test Deprecated Keyword Still Works
    [Documentation]    Verify backwards compatibility
    Element Text Should Be    JLabel#welcomeLabel    Welcome
```

### Timeout Tests: Retry Behavior Validation

```python
# tests/python/test_retry_behavior.py

import time
import pytest
from JavaGui.assertions.engine import JavaGuiAssertionEngine, AssertionConfig


class TestRetryBehavior:
    """Tests for retry timing and behavior."""

    def test_respects_timeout(self):
        """Verify timeout is respected."""
        config = AssertionConfig(timeout=0.5, interval=0.1)
        engine = JavaGuiAssertionEngine(config)

        getter = lambda: "wrong"
        start = time.time()

        with pytest.raises(AssertionError):
            engine.verify_with_retry(
                getter,
                AssertionOperator["=="],
                "expected"
            )

        elapsed = time.time() - start
        assert 0.4 < elapsed < 0.7  # Allow some tolerance

    def test_respects_interval(self):
        """Verify polling interval is respected."""
        config = AssertionConfig(timeout=1.0, interval=0.2)
        engine = JavaGuiAssertionEngine(config)

        call_times = []
        def getter():
            call_times.append(time.time())
            return "wrong"

        with pytest.raises(AssertionError):
            engine.verify_with_retry(
                getter,
                AssertionOperator["=="],
                "expected"
            )

        # Check intervals between calls
        for i in range(1, len(call_times)):
            interval = call_times[i] - call_times[i-1]
            assert 0.15 < interval < 0.3  # Allow tolerance
```

### Security Tests: Validate Operator Restrictions

```python
# tests/python/test_assertion_security.py

import pytest
from JavaGui.assertions.security import (
    AssertionSecurityValidator,
    SecurityError,
)
from AssertionEngine import AssertionOperator


class TestAssertionSecurity:
    """Tests for assertion security controls."""

    def test_equals_is_safe(self):
        """Equals operator is safe."""
        assert AssertionSecurityValidator.is_safe_operator(
            AssertionOperator["=="]
        )

    def test_validate_is_not_safe(self):
        """Validate operator is not safe (code execution)."""
        assert not AssertionSecurityValidator.is_safe_operator(
            AssertionOperator["validate"]
        )

    def test_validate_raises_when_restricted(self):
        """Validate raises SecurityError when restricted."""
        with pytest.raises(SecurityError):
            AssertionSecurityValidator.validate_operator(
                AssertionOperator["validate"],
                allow_code_execution=False,
            )

    def test_validate_allowed_when_enabled(self):
        """Validate works when code execution enabled."""
        # Should not raise
        AssertionSecurityValidator.validate_operator(
            AssertionOperator["validate"],
            allow_code_execution=True,
        )
```

---

## Claude-Flow Self-Learning Integration Summary

### Pre-Phase Hooks

```bash
# Search existing patterns before each phase
npx @claude-flow/cli@latest memory search --query "assertionengine" --namespace patterns

# Record task start
npx @claude-flow/cli@latest hooks pre-task --task-id "adr014-phaseN" --description "..."

# Route to optimal agent
npx @claude-flow/cli@latest hooks route --task "..."
```

### Post-Phase Hooks

```bash
# Record success with quality metrics
npx @claude-flow/cli@latest hooks post-task --task-id "adr014-phaseN" --success true --quality 0.9

# Store successful patterns
npx @claude-flow/cli@latest memory store --namespace patterns --key "pattern-name" --value "..."

# Train neural patterns
npx @claude-flow/cli@latest hooks post-edit --file "..." --train-neural true
```

### Background Workers

| Phase | Worker | Purpose |
|-------|--------|---------|
| 1 | - | Foundation setup |
| 2 | `optimize` | Performance analysis |
| 3 | `map` | Codebase mapping |
| 4 | `document`, `audit` | Docs and security |
| 5 | `testgaps` | Coverage analysis |

### Memory Patterns to Store

```bash
# Phase 1
assertion-retry-wrapper: "Time deadline pattern, getter callable, separate verify functions"

# Phase 2
get-keyword-pattern: "Getter closure captures locator, engine handles retry"

# Phase 3
table-assertion-pattern: "cellValue[row,col] syntax, nested getter"
tree-path-assertion: "Normalize separators, recursive navigation"

# Phase 4
assertion-security: "Restrict validate operator, audit logging"

# Phase 5
assertion-migration: "Map verification to Get+operator, waits to Get+timeout"
```

---

## Quality Gates Summary

| Phase | Gate | Criteria |
|-------|------|----------|
| 1 | Foundation | Imports work, 95% retry wrapper coverage |
| 2 | Core Keywords | 95% coverage per keyword, all operators tested |
| 3 | Advanced | 90% table/tree coverage, integration tests pass |
| 4 | Configuration | Security reviewed, docs generated |
| 5 | Migration | Aliases work, migration guide complete |

**Overall Target**: 90%+ test coverage, 0 security vulnerabilities

---

## Risk Mitigation

### 1. `validate` Operator Security

**Risk**: Arbitrary code execution through validate operator.

**Mitigation**:
- Strict mode enabled by default
- Security validator blocks validate in strict mode
- Audit logging for all operator usage
- Documentation warns about security implications

### 2. PyO3 Type Conversion Edge Cases

**Risk**: Type mismatches between Rust and Python layers.

**Mitigation**:
- Explicit type conversion in getter functions
- Comprehensive type tests
- Fallback chains for property retrieval

### 3. Timeout Interaction with Robot Framework

**Risk**: Nested timeouts may conflict.

**Mitigation**:
- AssertionEngine timeout is independent
- Document timeout precedence
- Allow explicit timeout override

### 4. Backwards Compatibility Breaks

**Risk**: Existing tests fail after migration.

**Mitigation**:
- Deprecation warnings (not errors) initially
- All legacy keywords work via aliases
- 1-year deprecation period before removal

---

## Timeline

| Week | Phase | Deliverables |
|------|-------|--------------|
| 1-2 | Foundation | Dependency, module structure, retry wrapper |
| 3-4 | Core Keywords | 6 Get keywords with assertions |
| 5-6 | Advanced | Table, tree, list keywords, formatters |
| 7 | Configuration | Timeout/interval keywords, security, docs |
| 8 | Migration | Deprecation, aliases, migration guide |

**Total**: 8 weeks

---

## References

- [MarketSquare/AssertionEngine](https://github.com/MarketSquare/AssertionEngine) - Source library
- [robotframework-assertion-engine on PyPI](https://pypi.org/project/robotframework-assertion-engine/) - Package
- [Browser Library](https://github.com/MarketSquare/robotframework-browser) - Reference implementation
- [ADR-007: Unified Keyword API](./ADR-007-UNIFIED-KEYWORD-API.md) - API design
- [ADR-009: Implementation Plan](./ADR-009-implementation-and-migration-plan.md) - Overall migration plan
- [Claude-Flow Documentation](https://github.com/ruvnet/claude-flow) - Self-learning integration

---

## Appendix A: Full Operator Reference

| Operator | Aliases | Description | Types |
|----------|---------|-------------|-------|
| `==` | `equal`, `equals` | Exact equality | any |
| `!=` | `inequal`, `not equal` | Not equal | any |
| `contains` | `*=` | Contains substring/item | str, list |
| `*not contains` | | Does not contain | str, list |
| `matches` | `~` | Regex match | str |
| `^=` | `starts` | Starts with | str |
| `$=` | `ends` | Ends with | str |
| `>` | `greater than` | Greater than | numeric |
| `>=` | | Greater or equal | numeric |
| `<` | `less than` | Less than | numeric |
| `<=` | | Less or equal | numeric |
| `validate` | | Custom validator | any (RESTRICTED) |
| `then` | | Chained assertion | any |

---

## Appendix B: Formatter Reference

| Formatter | Description |
|-----------|-------------|
| `normalize spaces` | Multiple spaces to single |
| `strip` | Remove leading/trailing whitespace |
| `case insensitive` | Convert to lowercase |
| `strip newlines` | Replace newlines with spaces |
| `apply to expected` | Apply formatters to expected value too |
