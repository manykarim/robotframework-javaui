# Python API Reference

Python-level API reference for the JavaGui library. This is useful for developers extending the library or using it programmatically.

## Module Structure

```
JavaGui/
├── __init__.py              # Main exports (Swing, Swt, Rcp)
├── assertions/              # Assertion engine integration
│   ├── __init__.py         # Retry wrappers, ElementState
│   ├── formatters.py       # Text formatters
│   └── security.py         # Secure expression evaluator
└── keywords/               # Keyword implementations
    ├── getters.py          # Swing Get* keywords
    ├── tables.py           # Swing Table/Tree/List keywords
    ├── swt_getters.py      # SWT Get* keywords
    ├── swt_tables.py       # SWT Table keywords
    ├── swt_trees.py        # SWT Tree keywords
    └── rcp_keywords.py     # RCP-specific keywords
```

## Core Classes

### SwingLibrary

Main library class for Swing applications.

```python
from JavaGui import Swing

# Initialize library
lib = Swing()

# Connect to application
lib.connect_to_application(main_class="com.example.MyApp", port=5678)

# Use keywords
tree = lib.get_component_tree(format="json", max_depth=10)
lib.click("JButton[name='submit']")

# Disconnect
lib.disconnect()
```

**Methods:**

#### get_component_tree()

```python
def get_component_tree(
    locator: Optional[str] = None,
    format: str = "text",
    max_depth: Optional[int] = None,
    types: Optional[str] = None,
    exclude_types: Optional[str] = None,
    visible_only: bool = False,
    enabled_only: bool = False,
    focusable_only: bool = False
) -> str:
    """
    Get component tree with filtering and formatting.

    Args:
        locator: Component locator (default: root)
        format: Output format (text, json, xml, yaml, csv, markdown)
        max_depth: Maximum tree depth (1-50)
        types: Include types (comma-separated, supports wildcards)
        exclude_types: Exclude types (comma-separated, supports wildcards)
        visible_only: Include only visible components
        enabled_only: Include only enabled components
        focusable_only: Include only focusable components

    Returns:
        String representation of component tree

    Examples:
        >>> tree = lib.get_component_tree(format="json", max_depth=10)
        >>> buttons = lib.get_component_tree(types="J*Button", visible_only=True)
        >>> form = lib.get_component_tree(
        ...     locator="JPanel[name='form']",
        ...     format="json",
        ...     max_depth=5
        ... )
    """
```

#### get_component_subtree()

```python
def get_component_subtree(
    locator: str,
    format: str = "text",
    max_depth: Optional[int] = None,
    types: Optional[str] = None,
    exclude_types: Optional[str] = None,
    visible_only: bool = False,
    enabled_only: bool = False,
    focusable_only: bool = False
) -> str:
    """
    Get subtree starting from specific component.

    Faster than get_component_tree for targeted queries.

    Args:
        locator: Component locator to start from (required)
        Other parameters: Same as get_component_tree

    Returns:
        String representation of subtree

    Examples:
        >>> form = lib.get_component_subtree("JPanel[name='form']", format="json")
        >>> menu = lib.get_component_subtree("JMenuBar", max_depth=3)
    """
```

#### connect_to_application()

```python
def connect_to_application(
    main_class: Optional[str] = None,
    title: Optional[str] = None,
    host: str = "localhost",
    port: int = 5678,
    timeout: int = 30
) -> None:
    """
    Connect to running Java application.

    Args:
        main_class: Application main class name
        title: Window title to match
        host: Agent host (default: localhost)
        port: Agent port (default: 5678)
        timeout: Connection timeout in seconds (default: 30)

    Raises:
        ConnectionError: If connection fails

    Examples:
        >>> lib.connect_to_application(main_class="com.example.MyApp")
        >>> lib.connect_to_application(title="My Application", port=5679)
    """
```

#### disconnect()

```python
def disconnect() -> None:
    """
    Disconnect from application.

    Examples:
        >>> lib.disconnect()
    """
```

#### click()

```python
def click(locator: str) -> None:
    """
    Click an element.

    Args:
        locator: Element locator (CSS or XPath style)

    Examples:
        >>> lib.click("JButton[name='submit']")
        >>> lib.click("//JButton[@text='OK']")
    """
```

#### input_text()

```python
def input_text(locator: str, text: str, clear: bool = True) -> None:
    """
    Enter text into text field.

    Args:
        locator: Text field locator
        text: Text to enter
        clear: Clear field before entering text (default: True)

    Examples:
        >>> lib.input_text("JTextField[name='username']", "admin")
        >>> lib.input_text("[name='comment']", "Additional text", clear=False)
    """
```

#### get_text()

```python
def get_text(
    locator: str,
    assertion_operator: Optional[Union[str, AssertionOperator]] = None,
    expected: Any = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None,
    formatters: Optional[list] = None
) -> str:
    """
    Get element text with optional assertion.

    Args:
        locator: Element locator
        assertion_operator: Assertion operator (==, !=, contains, etc.)
        expected: Expected value for assertion
        message: Custom assertion error message
        timeout: Assertion retry timeout in seconds
        formatters: List of formatters to apply

    Returns:
        Element text (if no assertion or assertion passes)

    Raises:
        AssertionError: If assertion fails after timeout

    Examples:
        >>> text = lib.get_text("JLabel[name='status']")
        >>> lib.get_text("JLabel[name='status']", "==", "Ready", timeout=5)
        >>> lib.get_text("JLabel[name='title']", "==", "hello",
        ...              formatters=["lowercase", "strip"])
    """
```

---

### SwtLibrary

Library class for SWT applications.

```python
from JavaGui import Swt

lib = Swt()
lib.connect_to_application(main_class="com.example.SwtApp", port=5678)

# SWT-specific methods
text = lib.get_widget_text("Label[name='status']")
count = lib.get_widget_count("Button")

lib.disconnect()
```

**SWT-Specific Methods:**

#### get_widget_text()

```python
def get_widget_text(
    locator: str,
    assertion_operator: Optional[Union[str, AssertionOperator]] = None,
    expected: Any = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None
) -> str:
    """
    Get SWT widget text with optional assertion.

    Examples:
        >>> text = lib.get_widget_text("Label[name='status']")
        >>> lib.get_widget_text("Label[name='status']", "==", "Ready")
    """
```

#### get_widget_count()

```python
def get_widget_count(
    locator: str,
    assertion_operator: Optional[Union[str, AssertionOperator]] = None,
    expected: Any = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None
) -> int:
    """
    Count SWT widgets with optional assertion.

    Examples:
        >>> count = lib.get_widget_count("Button")
        >>> lib.get_widget_count("Button", ">=", 5)
    """
```

#### is_widget_enabled()

```python
def is_widget_enabled(
    locator: str,
    assertion_operator: Optional[Union[str, AssertionOperator]] = None,
    expected: Any = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None
) -> bool:
    """
    Check if SWT widget is enabled with optional assertion.

    Examples:
        >>> enabled = lib.is_widget_enabled("Button[name='submit']")
        >>> lib.is_widget_enabled("Button[name='submit']", "==", True)
    """
```

---

### RcpLibrary

Library class for Eclipse RCP applications.

```python
from JavaGui import Rcp

lib = Rcp()
lib.connect_to_application(main_class="org.eclipse.ui.PlatformUI", port=5678)

# RCP-specific methods
view_count = lib.get_open_view_count()
perspective = lib.get_active_perspective_id()
dirty = lib.get_editor_dirty_state("MyFile.java")

lib.disconnect()
```

**RCP-Specific Methods:**

#### get_open_view_count()

```python
def get_open_view_count(
    assertion_operator: Optional[Union[str, AssertionOperator]] = None,
    expected: Any = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None
) -> int:
    """
    Get count of open views with optional assertion.

    Examples:
        >>> count = lib.get_open_view_count()
        >>> lib.get_open_view_count(">=", 1)
    """
```

#### get_open_editor_count()

```python
def get_open_editor_count(
    assertion_operator: Optional[Union[str, AssertionOperator]] = None,
    expected: Any = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None
) -> int:
    """
    Get count of open editors with optional assertion.

    Examples:
        >>> count = lib.get_open_editor_count()
        >>> lib.get_open_editor_count(">", 0)
    """
```

#### get_active_perspective_id()

```python
def get_active_perspective_id(
    assertion_operator: Optional[Union[str, AssertionOperator]] = None,
    expected: Any = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None
) -> str:
    """
    Get active perspective ID with optional assertion.

    Examples:
        >>> perspective = lib.get_active_perspective_id()
        >>> lib.get_active_perspective_id("==", "org.eclipse.ui.resourcePerspective")
        >>> lib.get_active_perspective_id("contains", "resource")
    """
```

#### get_editor_dirty_state()

```python
def get_editor_dirty_state(
    title: str,
    assertion_operator: Optional[Union[str, AssertionOperator]] = None,
    expected: Any = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None
) -> bool:
    """
    Check if editor has unsaved changes with optional assertion.

    Args:
        title: Editor title/filename

    Examples:
        >>> dirty = lib.get_editor_dirty_state("MyFile.java")
        >>> lib.get_editor_dirty_state("MyFile.java", "==", False)
    """
```

---

## Assertion Engine Integration

### AssertionConfig

Configuration for assertion behavior.

```python
from JavaGui.assertions import AssertionConfig

# Get current config
config = AssertionConfig()

# Set default timeout
config.set_timeout(10.0)  # seconds

# Set retry interval
config.set_interval(0.2)  # seconds

# Get current values
timeout = config.get_timeout()  # Returns float
interval = config.get_interval()  # Returns float
```

### ElementState Enum

Element states for assertions.

```python
from JavaGui.assertions import ElementState

# Available states
ElementState.VISIBLE      # Element is visible
ElementState.HIDDEN       # Element is hidden
ElementState.ENABLED      # Element is enabled
ElementState.DISABLED     # Element is disabled
ElementState.FOCUSED      # Element has focus
ElementState.UNFOCUSED    # Element doesn't have focus
ElementState.SELECTED     # Element is selected
ElementState.UNSELECTED   # Element is not selected
ElementState.CHECKED      # Checkbox is checked
ElementState.UNCHECKED    # Checkbox is unchecked
ElementState.EDITABLE     # Element is editable
ElementState.READONLY     # Element is read-only
ElementState.EXPANDED     # Node is expanded
ElementState.COLLAPSED    # Node is collapsed
ElementState.ATTACHED     # Element is attached to DOM
ElementState.DETACHED     # Element is detached
```

### Text Formatters

Apply transformations to text before assertions.

```python
from JavaGui.assertions.formatters import (
    normalize_spaces,
    strip_whitespace,
    to_lowercase,
    to_uppercase
)

# Use formatters
text = "  Hello   World  "
normalized = normalize_spaces(text)  # "Hello World"
stripped = strip_whitespace(text)    # "Hello   World"
lower = to_lowercase(text)           # "  hello   world  "
upper = to_uppercase(text)           # "  HELLO   WORLD  "

# Chain formatters
result = strip_whitespace(normalize_spaces(text))  # "Hello World"
```

**Available Formatters:**

| Function | Description | Example |
|----------|-------------|---------|
| `normalize_spaces(text)` | Collapse multiple spaces to single space | `"a  b"` → `"a b"` |
| `strip_whitespace(text)` | Remove leading/trailing whitespace | `" abc "` → `"abc"` |
| `to_lowercase(text)` | Convert to lowercase | `"ABC"` → `"abc"` |
| `to_uppercase(text)` | Convert to uppercase | `"abc"` → `"ABC"` |

### Secure Expression Evaluator

Safe evaluation of custom validation expressions.

```python
from JavaGui.assertions.security import evaluate_expression

# Evaluate safe expressions
result = evaluate_expression("10 > 5", {"value": 10})  # True
result = evaluate_expression("'@' in value", {"value": "test@example.com"})  # True
result = evaluate_expression("len(value) > 5", {"value": "hello world"})  # True

# Blocked dangerous operations (raises SecurityError)
# evaluate_expression("__import__('os')", {})  # SecurityError
# evaluate_expression("eval('1+1')", {})       # SecurityError
# evaluate_expression("open('/etc/passwd')", {})  # SecurityError
```

**Allowed builtins:**
- `len`, `int`, `str`, `bool`, `float`, `list`, `dict`, `tuple`, `set`
- `min`, `max`, `sum`, `abs`, `round`, `sorted`, `reversed`
- `all`, `any`, `enumerate`, `zip`, `range`

**Blocked operations:**
- `eval`, `exec`, `compile`, `__import__`
- File operations (`open`, file objects)
- Attribute access (`__class__`, `__dict__`, etc.)
- Code objects and bytecode manipulation

---

## Utility Functions

### get_agent_jar_path()

Get path to bundled Java agent JAR.

```python
from JavaGui import get_agent_jar_path

jar_path = get_agent_jar_path()
print(f"Agent JAR: {jar_path}")

# Use in Java command
# java -javaagent:{jar_path}=port=5678 -jar myapp.jar
```

---

## Type Hints

The library includes comprehensive type hints for IDE support:

```python
from typing import Optional, Union, List, Any
from JavaGui import Swing
from assertionengine import AssertionOperator

lib: Swing = Swing()

# Type hints help IDEs provide autocomplete and type checking
tree: str = lib.get_component_tree(
    locator=None,
    format="json",
    max_depth=10
)

count: int = lib.get_element_count("JButton")

states: List[str] = lib.get_element_states("JButton[name='submit']")
```

---

## Exception Handling

The library raises specific exceptions for different error conditions:

```python
from JavaGui import Swing
from JavaGui.exceptions import (
    ConnectionError,
    ElementNotFoundError,
    TimeoutError,
    AssertionError
)

lib = Swing()

try:
    lib.connect_to_application(main_class="com.example.MyApp", timeout=10)
except ConnectionError as e:
    print(f"Failed to connect: {e}")

try:
    lib.click("JButton[name='nonexistent']")
except ElementNotFoundError as e:
    print(f"Element not found: {e}")

try:
    lib.get_text("JLabel[name='status']", "==", "Ready", timeout=5)
except TimeoutError as e:
    print(f"Assertion timed out: {e}")
except AssertionError as e:
    print(f"Assertion failed: {e}")
```

---

## Advanced Usage Examples

### Custom Test Framework Integration

```python
from JavaGui import Swing
import unittest

class MyAppTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.lib = Swing()
        cls.lib.connect_to_application(
            main_class="com.example.MyApp",
            port=5678
        )

    @classmethod
    def tearDownClass(cls):
        cls.lib.disconnect()

    def test_login(self):
        self.lib.input_text("JTextField[name='username']", "admin")
        self.lib.input_text("JPasswordField[name='password']", "secret")
        self.lib.click("JButton[name='login']")

        # Use assertion
        self.lib.get_text(
            "JLabel[name='status']",
            "==",
            "Login successful",
            timeout=5
        )

    def test_component_tree(self):
        import json

        tree_json = self.lib.get_component_tree(
            format="json",
            max_depth=10,
            types="J*Button",
            visible_only=True
        )

        tree = json.loads(tree_json)
        self.assertIn("type", tree)
        self.assertGreater(len(tree.get("children", [])), 0)
```

### Programmatic UI Inspection

```python
from JavaGui import Swing
import json

def inspect_application_ui(main_class: str, port: int = 5678):
    """Programmatically inspect application UI structure."""
    lib = Swing()

    try:
        # Connect
        lib.connect_to_application(main_class=main_class, port=port)

        # Get component tree
        tree_json = lib.get_component_tree(
            format="json",
            max_depth=20,
            visible_only=True
        )

        tree = json.loads(tree_json)

        # Analyze tree
        stats = analyze_component_tree(tree)

        print(f"Total components: {stats['total_count']}")
        print(f"Component types: {stats['types']}")
        print(f"Max depth: {stats['max_depth']}")

        return tree

    finally:
        lib.disconnect()

def analyze_component_tree(tree: dict) -> dict:
    """Analyze component tree structure."""
    types = set()
    count = 0

    def traverse(node, depth=0):
        nonlocal count
        count += 1
        types.add(node.get("type", "unknown"))

        max_child_depth = depth
        for child in node.get("children", []):
            child_depth = traverse(child, depth + 1)
            max_child_depth = max(max_child_depth, child_depth)

        return max_child_depth

    max_depth = traverse(tree)

    return {
        "total_count": count,
        "types": sorted(list(types)),
        "max_depth": max_depth
    }

# Usage
tree = inspect_application_ui("com.example.MyApp")
```

### Batch Component Tree Export

```python
from JavaGui import Swing
import os

def export_ui_documentation(main_class: str, output_dir: str, port: int = 5678):
    """Export UI structure in multiple formats for documentation."""
    lib = Swing()

    try:
        lib.connect_to_application(main_class=main_class, port=port)

        formats = ["text", "json", "xml", "yaml", "csv", "markdown"]

        for fmt in formats:
            tree = lib.get_component_tree(
                format=fmt,
                max_depth=15,
                visible_only=True
            )

            output_file = os.path.join(output_dir, f"ui_structure.{fmt}")
            with open(output_file, "w", encoding="utf-8") as f:
                f.write(tree)

            print(f"Exported {fmt.upper()} to {output_file}")

    finally:
        lib.disconnect()

# Usage
export_ui_documentation("com.example.MyApp", "./docs/ui")
```

---

## See Also

- [Robot Framework Keywords Reference](robot-keywords.md)
- [Quick Start Guide](../COMPONENT_TREE_QUICK_START.md)
- [Migration Guide](../MIGRATION_GUIDE.md)
- [GitHub Repository](https://github.com/manykarim/robotframework-javaui)
