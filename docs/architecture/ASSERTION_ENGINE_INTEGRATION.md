# AssertionEngine Integration Architecture

## Overview

This document describes the integration of robotframework-assertion-engine (v3.0+) with the JavaGui library for Robot Framework.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Robot Framework Test                          │
└─────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         SwingLibrary                                 │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────┐  ┌───────────┐ │
│  │GetterKeywords│  │TableKeywords│  │TreeKeywords│  │ListKeywords│ │
│  └─────────────┘  └──────────────┘  └─────────────┘  └───────────┘ │
└─────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Assertion Layer                                 │
│  ┌───────────────────┐  ┌─────────────┐  ┌──────────────────────┐  │
│  │ with_retry_assertion│  │ Formatters  │  │SecureExpressionEval│  │
│  └───────────────────┘  └─────────────┘  └──────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                 robotframework-assertion-engine                      │
│  ┌──────────────┐  ┌───────────────┐  ┌──────────────────────────┐ │
│  │verify_assertion│  │AssertionOperator│  │float_str_verify_assertion│ │
│  └──────────────┘  └───────────────┘  └──────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
```

## Key Components

### 1. AssertionOperator Enum

The AssertionOperator enum provides standard comparison operators:

| Operator | Symbol | Description |
|----------|--------|-------------|
| equal | == | Exact equality |
| not_equal | != | Not equal |
| less | < | Less than |
| greater | > | Greater than |
| less_or_equal | <= | Less or equal |
| greater_or_equal | >= | Greater or equal |
| contains | *= | Contains substring |
| not_contains | !*= | Does not contain |
| starts | ^= | Starts with |
| ends | $= | Ends with |
| matches | ~= | Regex match |
| validate | validate | Custom expression |
| then | then | Return value only |

### 2. ElementState Flag Enum

The ElementState enum provides state flags for UI elements:

```python
class ElementState(Flag):
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
```

### 3. Retry Mechanism

The `with_retry_assertion` function provides retry logic for flaky assertions:

```python
def with_retry_assertion(
    get_value_func: Callable[[], T],
    operator: Optional[AssertionOperator],
    expected: Any,
    message: str = "",
    timeout: float = 5.0,
    interval: float = 0.1,
) -> T
```

### 4. Security Controls

The SecureExpressionEvaluator provides safe evaluation for the `validate` operator:

- **Safe Builtins**: bool, int, float, str, list, dict, len, isinstance, etc.
- **Blocked Builtins**: eval, exec, compile, open, __import__, etc.
- **Safe Modules**: re (search, match, findall, sub, split, compile)
- **AST Validation**: Parses and validates expressions before evaluation

### 5. Formatters

Text formatters for preprocessing assertion values:

| Formatter | Description |
|-----------|-------------|
| normalize_spaces | Collapse whitespace |
| strip | Remove leading/trailing whitespace |
| lowercase | Convert to lowercase |
| uppercase | Convert to uppercase |
| strip_html_tags | Remove HTML tags |

### 6. Deprecation System

The deprecation module provides backward compatibility:

```python
@deprecated(
    reason="Use Get Text instead",
    replacement="Get Text",
    version="3.0.0",
    remove_in="4.0.0"
)
def get_label_content(self, locator):
    return self.get_text(locator)
```

## Performance Characteristics

Benchmarks run on typical development hardware:

| Operation | Mean Time |
|-----------|-----------|
| Retry mechanism (immediate success) | ~1 µs |
| No operator (value return) | ~0.2 µs |
| Formatters (single) | <2 µs |
| Formatters (chained, 3) | ~5 µs |
| Security evaluator (simple) | ~20 µs |
| Security evaluator (regex) | ~40 µs |
| ElementState.from_string | ~0.6 µs |

## Configuration

### Default Timeouts

| Setting | Default | Description |
|---------|---------|-------------|
| assertion_timeout | 5.0s | Max retry time |
| assertion_interval | 0.1s | Retry interval |
| library_timeout | 10.0s | General operation timeout |

### Customization

```python
# Set via library initialization
library = SwingLibrary(timeout=30.0)

# Set via keywords
library.set_assertion_timeout(10.0)
library.set_assertion_interval(0.2)
```

## Usage Examples

### Basic Text Assertion

```robotframework
*** Test Cases ***
Verify Label Text
    Get Text    JLabel#status    ==    Ready
```

### Contains Assertion

```robotframework
*** Test Cases ***
Verify Text Contains
    Get Text    JLabel#message    *=    successfully
```

### Numeric Assertion

```robotframework
*** Test Cases ***
Verify Count Greater Than
    Get Element Count    JButton    >    5
```

### State Assertion

```robotframework
*** Test Cases ***
Verify Element States
    Get Element States    JButton#submit    contains    visible    enabled
```

### Table Cell Assertion

```robotframework
*** Test Cases ***
Verify Table Cell
    Get Table Cell Value    JTable    0    Name    ==    John
```

### Custom Validation

```robotframework
*** Test Cases ***
Verify Custom Expression
    Get Text    JLabel#count    validate    int(value) >= 10 and int(value) <= 100
```

## Files Structure

```
python/JavaGui/
├── __init__.py           # Main library with SwingLibrary class
├── deprecation.py        # Deprecation utilities
├── assertions/
│   ├── __init__.py       # Retry wrappers and ElementState
│   ├── formatters.py     # Text formatters
│   └── security.py       # Secure expression evaluator
└── keywords/
    ├── __init__.py       # Keyword module exports
    ├── getters.py        # Get* keyword mixins
    └── tables.py         # Table/Tree/List keyword mixins
```

## Test Coverage

- 89 unit tests for assertion module
- 16 performance benchmarks
- 266 total Python tests passing

## Version History

- v3.0.0: Initial AssertionEngine integration
  - Added AssertionOperator support
  - Added ElementState enum
  - Added retry mechanism
  - Added formatters
  - Added security controls
  - Added deprecation system
