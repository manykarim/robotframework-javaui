# ADR-007: Unified Keyword API Design

| ADR ID | ADR-007 |
|--------|---------|
| Title | Unified Keyword API Design |
| Status | Proposed |
| Date | 2026-01-19 |
| Authors | Architecture Team |
| Supersedes | Extends ADR-003 (Keyword Naming Convention) |

## Context

The library currently provides ~60-70 keywords across Swing, SWT, and RCP technologies. Following research into modern Robot Framework libraries (particularly Browser Library), we propose reducing this to ~20-25 core keywords with a powerful assertion engine pattern.

### Current Keyword Count Analysis

| Category | Current Swing | Current SWT | Unified Target |
|----------|--------------|-------------|----------------|
| Connection | 4 | 3 | 3 |
| Click/Action | 8 | 6 | 5 |
| Text Input | 4 | 3 | 3 |
| Selection | 6 | 5 | 2 |
| Get/Query | 15 | 12 | 6 |
| Wait | 8 | 6 | 0* |
| Verification | 12 | 8 | 0* |
| Table | 8 | 10 | 0* |
| Tree | 6 | 8 | 0* |
| Menu | 3 | 2 | 2 |
| UI Tree | 4 | 3 | 4 |
| **Total** | **~78** | **~66** | **~25** |

*Wait and verification keywords are replaced by the assertion engine in Get keywords.
*Table/tree operations are unified into Get/Select keywords with property accessors.

### Decision Drivers

- Browser Library's success with reduced keyword count and assertion engine
- Simplified API reduces learning curve
- Inline assertions reduce test verbosity
- Consistent patterns across all component types
- Full backwards compatibility via aliases

## Decision

We will implement a **Unified Keyword API** with approximately 20-25 core keywords and a built-in **Assertion Engine** following Browser Library patterns.

### 1. Core Keyword Specifications

#### 1.1 Action Keywords (5)

```python
def Click(
    locator: str,
    modifiers: Optional[List[str]] = None,
    click_count: int = 1
) -> None:
    """Click on an element.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |
    | ``modifiers`` | Keyboard modifiers: ``shift``, ``ctrl``, ``alt``, ``meta``. |
    | ``click_count`` | Number of clicks. ``2`` for double-click. Default ``1``. |

    Examples:
    | `Click` | JButton#submit |
    | `Click` | JButton#item | modifiers=['ctrl'] |
    | `Click` | JTable | click_count=2 |
    """
```

```python
def TypeText(
    locator: str,
    text: str,
    clear: bool = False,
    delay: Optional[float] = None
) -> None:
    """Type text into an element character by character.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |
    | ``text`` | Text to type. |
    | ``clear`` | Clear existing text before typing. Default ``False``. |
    | ``delay`` | Delay between keystrokes in seconds. |

    Examples:
    | `Type Text` | JTextField#username | testuser |
    | `Type Text` | JTextField#search | query | clear=True |
    """
```

```python
def ClearText(locator: str) -> None:
    """Clear all text from an element.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |

    Examples:
    | `Clear Text` | JTextField#search |
    """
```

```python
def SelectItem(
    locator: str,
    item: str,
    by: str = "value"
) -> None:
    """Select an item from a selectable component.

    Works with JComboBox, JList, JTree, JTabbedPane, JTable (row selection).

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |
    | ``item`` | Item to select. |
    | ``by`` | Selection strategy: ``value``, ``index``, ``text``, ``path``. |

    The ``by`` parameter determines how ``item`` is interpreted:
    - ``value``: Match by item value/text (default)
    - ``index``: Match by numeric index (0-based)
    - ``text``: Match by visible text (same as value for most components)
    - ``path``: Match by path for trees (e.g., "Root/Child/Grandchild")

    Examples:
    | `Select Item` | JComboBox#country | United States |
    | `Select Item` | JList#items | Item 3 | by=value |
    | `Select Item` | JList#items | 2 | by=index |
    | `Select Item` | JTree#files | Documents/Reports | by=path |
    | `Select Item` | JTabbedPane#tabs | Settings |
    | `Select Item` | JTable#data | 5 | by=index |
    """
```

```python
def SetCheckbox(
    locator: str,
    state: bool
) -> None:
    """Set checkbox or toggle button to specified state.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |
    | ``state`` | Desired state: ``True`` (checked) or ``False`` (unchecked). |

    Examples:
    | `Set Checkbox` | JCheckBox#remember | True |
    | `Set Checkbox` | JCheckBox#newsletter | False |
    | `Set Checkbox` | JToggleButton#bold | True |
    """
```

#### 1.2 Get Keywords with Assertions (6)

The core innovation: Get keywords that optionally perform assertions inline.

```python
def GetText(
    locator: str,
    operator: Optional[str] = None,
    expected: Optional[str] = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None
) -> str:
    """Get text content of an element, optionally with assertion.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |
    | ``operator`` | Assertion operator. See `Assertion Operators`. |
    | ``expected`` | Expected value for assertion. |
    | ``message`` | Custom assertion failure message. |
    | ``timeout`` | Assertion retry timeout in seconds. |

    Returns the element's text content.

    When ``operator`` is provided, performs assertion with retry:
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
```

```python
def GetValue(
    locator: str,
    operator: Optional[str] = None,
    expected: Optional[Any] = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None
) -> Any:
    """Get the value of an input element, optionally with assertion.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |
    | ``operator`` | Assertion operator. See `Assertion Operators`. |
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
```

```python
def GetElementStates(
    locator: str,
    operator: Optional[str] = None,
    expected: Optional[List[str]] = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None
) -> List[str]:
    """Get element states, optionally with assertion.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |
    | ``operator`` | Assertion operator. See `Assertion Operators`. |
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
    - ``attached`` / ``detached``

    Examples:
    | ${states}= | `Get Element States` | JButton#submit |
    | `Get Element States` | JButton#submit | contains | enabled |
    | `Get Element States` | JButton#submit | contains | ['enabled', 'visible'] |
    | `Get Element States` | JCheckBox#opt | contains | checked |
    | `Get Element States` | JButton#delete | not contains | enabled |
    """
```

```python
def GetElementCount(
    locator: str,
    operator: Optional[str] = None,
    expected: Optional[int] = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None
) -> int:
    """Get count of elements matching locator, optionally with assertion.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |
    | ``operator`` | Assertion operator. See `Assertion Operators`. |
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
```

```python
def GetProperty(
    locator: str,
    property_name: str,
    operator: Optional[str] = None,
    expected: Optional[Any] = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None
) -> Any:
    """Get a specific property of an element, optionally with assertion.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |
    | ``property_name`` | Property to retrieve. |
    | ``operator`` | Assertion operator. See `Assertion Operators`. |
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
    | `Get Property` | JButton#submit | enabled | == | True |
    | `Get Property` | JTable#data | rowCount | >= | 10 |
    | `Get Property` | JTable#data | cellValue[0,1] | == | John |
    | `Get Property` | JComboBox | selectedIndex | == | 2 |
    """
```

```python
def GetProperties(
    locator: str
) -> Dict[str, Any]:
    """Get all common properties of an element.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |

    Returns a dictionary containing all retrievable properties.

    Examples:
    | ${props}= | `Get Properties` | JButton#submit |
    | Log | Text: ${props}[text], Enabled: ${props}[enabled] |
    """
```

#### 1.3 Session Keywords (3)

```python
def ConnectToApplication(
    path_or_name: str,
    args: Optional[List[str]] = None,
    host: str = "localhost",
    port: int = 5678,
    timeout: float = 30.0
) -> None:
    """Connect to a running Java application.

    | =Argument= | =Description= |
    | ``path_or_name`` | Application name, path, main class, or window title. |
    | ``args`` | Optional arguments if launching application. |
    | ``host`` | Host where agent is running. Default ``localhost``. |
    | ``port`` | Agent port. Default ``5678``. |
    | ``timeout`` | Connection timeout in seconds. Default ``30``. |

    Examples:
    | `Connect To Application` | MyApp |
    | `Connect To Application` | com.example.MainClass |
    | `Connect To Application` | /path/to/app.jar | args=['--config', 'test'] |
    """
```

```python
def Disconnect() -> None:
    """Disconnect from the current application.

    Closes the connection and releases resources.

    Examples:
    | `Disconnect` |
    """
```

```python
def IsConnected() -> bool:
    """Check if connected to an application.

    Returns ``True`` if connected, ``False`` otherwise.

    Examples:
    | ${connected}= | `Is Connected` |
    | Should Be True | ${connected} |
    """
```

#### 1.4 Introspection Keywords (4)

```python
def GetUITree(
    locator: Optional[str] = None,
    depth: Optional[int] = None,
    format: str = "text"
) -> str:
    """Get the UI component tree.

    | =Argument= | =Description= |
    | ``locator`` | Optional root locator. Uses entire tree if not specified. |
    | ``depth`` | Maximum depth to traverse. Unlimited if not specified. |
    | ``format`` | Output format: ``text``, ``json``, ``xml``. Default ``text``. |

    Returns the component tree in the specified format.

    Examples:
    | ${tree}= | `Get UI Tree` |
    | ${json}= | `Get UI Tree` | format=json |
    | ${subtree}= | `Get UI Tree` | JPanel#main | depth=3 |
    """
```

```python
def LogUITree(
    locator: Optional[str] = None,
    depth: Optional[int] = None
) -> None:
    """Log the UI component tree to the test log.

    | =Argument= | =Description= |
    | ``locator`` | Optional root locator. Uses entire tree if not specified. |
    | ``depth`` | Maximum depth to traverse. Unlimited if not specified. |

    Examples:
    | `Log UI Tree` |
    | `Log UI Tree` | JPanel#main |
    | `Log UI Tree` | depth=5 |
    """
```

```python
def RefreshUITree() -> None:
    """Force refresh of the cached UI component tree.

    Call after UI changes to update the internal component cache.

    Examples:
    | `Click` | JButton#addItem |
    | `Refresh UI Tree` |
    | `Get Element Count` | JListItem | == | 6 |
    """
```

```python
def FindElements(
    locator: str
) -> List[Element]:
    """Find all elements matching the locator.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |

    Returns a list of element references.

    Examples:
    | ${elements}= | `Find Elements` | JButton |
    | ${count}= | Get Length | ${elements} |
    """
```

#### 1.5 Menu Keywords (2)

```python
def SelectMenu(
    menu_path: str
) -> None:
    """Select a menu item from the menu bar.

    | =Argument= | =Description= |
    | ``menu_path`` | Menu path separated by ``|``. |

    Examples:
    | `Select Menu` | File|New |
    | `Select Menu` | Edit|Copy |
    | `Select Menu` | File|Export|As PDF |
    """
```

```python
def SelectContextMenu(
    locator: str,
    menu_path: str
) -> None:
    """Right-click element and select from context menu.

    | =Argument= | =Description= |
    | ``locator`` | Element to right-click. |
    | ``menu_path`` | Menu path separated by ``|``. |

    Examples:
    | `Select Context Menu` | JTree#files | Delete |
    | `Select Context Menu` | JTable#data | Edit|Properties |
    """
```

### 2. Assertion Engine Design

#### 2.1 Supported Operators

| Operator | Aliases | Description | Value Types |
|----------|---------|-------------|-------------|
| `==` | `equals`, `equal`, `eq` | Exact equality | any |
| `!=` | `not equals`, `ne` | Not equal | any |
| `contains` | `has` | Contains substring/item | str, list |
| `not contains` | `not has` | Does not contain | str, list |
| `matches` | `regex` | Regex match | str |
| `not matches` | | Regex does not match | str |
| `>` | `greater than`, `gt` | Greater than | numeric |
| `<` | `less than`, `lt` | Less than | numeric |
| `>=` | `gte` | Greater than or equal | numeric |
| `<=` | `lte` | Less than or equal | numeric |
| `starts with` | `startswith` | String starts with | str |
| `ends with` | `endswith` | String ends with | str |
| `is` | | Identity check for states | states |
| `is not` | | Negative identity for states | states |

#### 2.2 Retry Mechanism

```rust
/// Assertion engine with configurable retry
pub struct AssertionEngine {
    default_timeout: Duration,
    poll_interval: Duration,
}

impl AssertionEngine {
    pub fn assert_with_retry<T, F>(
        &self,
        get_value: F,
        operator: &str,
        expected: &T,
        timeout: Option<Duration>,
        message: Option<&str>,
    ) -> Result<T, AssertionError>
    where
        F: Fn() -> Result<T, GuiError>,
        T: PartialEq + Display,
    {
        let timeout = timeout.unwrap_or(self.default_timeout);
        let start = Instant::now();
        let mut last_value: Option<T> = None;
        let mut last_error: Option<String> = None;

        while start.elapsed() < timeout {
            match get_value() {
                Ok(actual) => {
                    if self.evaluate(operator, &actual, expected)? {
                        return Ok(actual);
                    }
                    last_value = Some(actual);
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                }
            }
            std::thread::sleep(self.poll_interval);
        }

        // Build detailed error message
        let error_msg = self.build_error_message(
            operator,
            expected,
            last_value.as_ref(),
            last_error.as_deref(),
            timeout,
            message,
        );

        Err(AssertionError::new(error_msg))
    }

    fn evaluate<T>(&self, operator: &str, actual: &T, expected: &T) -> Result<bool, ParseError>
    where
        T: PartialEq + PartialOrd,
    {
        match operator {
            "==" | "equals" | "equal" | "eq" => Ok(actual == expected),
            "!=" | "not equals" | "ne" => Ok(actual != expected),
            ">" | "greater than" | "gt" => Ok(actual > expected),
            "<" | "less than" | "lt" => Ok(actual < expected),
            ">=" | "gte" => Ok(actual >= expected),
            "<=" | "lte" => Ok(actual <= expected),
            _ => Err(ParseError::UnknownOperator(operator.to_string())),
        }
    }
}
```

#### 2.3 Custom Assertion Messages

```robot
*** Test Cases ***
Test Custom Message
    # Default error message
    Get Text    JLabel#status    ==    Ready
    # Output on failure: AssertionError: Expected 'Ready' but got 'Loading'

    # Custom error message
    Get Text    JLabel#status    ==    Ready    message=System not initialized
    # Output on failure: AssertionError: System not initialized
    #   Expected: 'Ready'
    #   Actual: 'Loading'
```

#### 2.4 Chained Assertions (via Robot Framework)

```robot
*** Test Cases ***
Test Multiple Conditions
    # Multiple assertions on same element
    Get Element States    JButton#submit    contains    visible
    Get Element States    JButton#submit    contains    enabled
    Get Text              JButton#submit    ==          Submit

    # Or use property for multiple checks
    ${props}=    Get Properties    JButton#submit
    Should Be True    ${props}[visible]
    Should Be True    ${props}[enabled]
    Should Be Equal   ${props}[text]    Submit
```

### 3. Locator Syntax

Building on ADR-002 (Locator Syntax Strategy):

#### 3.1 Basic Locators

```robot
# Type selector
JButton
Button
Text

# Name/ID selector
JButton#submit
#submitBtn
name:submitBtn

# Attribute selector
JButton[text="Save"]
Button[enabled=true]
```

#### 3.2 Hierarchy Locators

```robot
# Direct child (>)
JFrame > JPanel > JButton

# Descendant (>> or space)
JFrame >> JButton
JFrame JButton

# Combined
JPanel#main > JButton[text="OK"]
```

#### 3.3 Filter Locators

```robot
# Multiple attributes
JButton[enabled=true, visible=true]
JButton[text="Save"][enabled=true]

# State filters
JButton:enabled
JButton:visible
JCheckBox:checked

# Index filter
JButton[index=0]
JButton:first
JButton:last
JButton:nth(2)
```

#### 3.4 XPath-Style Locators

```robot
# XPath syntax
//JButton[@text="OK"]
//JPanel/JButton
//JTable//JTableCell[@row=0][@col=1]
```

#### 3.5 Table/Tree Specific Locators

```robot
# Table cell access
JTable#data >> cell[0,1]
JTable#data >> row[0] >> cell[Name]
JTable#data[row=0, col=1]

# Tree node access
JTree#files >> node[Documents/Reports]
JTree#files >> node:selected
```

### 4. Error Handling

Building on ADR-005 (Error Handling Strategy), add assertion-specific errors:

#### 4.1 New Exception Types

```
JavaGuiError (base)
├── ... (existing hierarchy from ADR-005)
└── AssertionError
    ├── AssertionFailedError
    │   └── AssertionTimeoutError
    └── InvalidOperatorError
```

#### 4.2 Assertion Error Format

```python
class AssertionFailedError(JavaGuiError):
    """Raised when an assertion fails.

    Attributes:
        operator: The assertion operator used
        expected: The expected value
        actual: The actual value
        locator: The element locator
        timeout: The timeout used (if retried)
        message: Custom message (if provided)
    """
```

#### 4.3 Error Message Format

```
AssertionFailedError: Assertion failed after 10.0s

  Operator: ==
  Expected: 'Ready'
  Actual:   'Loading'
  Locator:  JLabel#status

Suggestions:
  - Increase timeout if the value changes slowly
  - Verify the expected value is correct
  - Check if another element matches the locator
```

### 5. Complete Keyword Reference

| Category | Keyword | Purpose |
|----------|---------|---------|
| **Session** | `Connect To Application` | Connect to Java app |
| | `Disconnect` | Close connection |
| | `Is Connected` | Check connection |
| **Action** | `Click` | Click element |
| | `Type Text` | Type into element |
| | `Clear Text` | Clear text field |
| | `Select Item` | Select from list/combo/tree/tab |
| | `Set Checkbox` | Set checkbox state |
| **Get/Assert** | `Get Text` | Get/assert text |
| | `Get Value` | Get/assert value |
| | `Get Element States` | Get/assert states |
| | `Get Element Count` | Get/assert count |
| | `Get Property` | Get/assert property |
| | `Get Properties` | Get all properties |
| **Menu** | `Select Menu` | Select menu item |
| | `Select Context Menu` | Right-click menu |
| **Introspection** | `Get UI Tree` | Get component tree |
| | `Log UI Tree` | Log component tree |
| | `Refresh UI Tree` | Refresh cache |
| | `Find Elements` | Find matching elements |

**Total: 18 core keywords**

### 6. Backwards Compatibility Aliases

```rust
/// Complete mapping of legacy keywords to unified API
pub const LEGACY_KEYWORD_ALIASES: &[(&str, &str, &str)] = &[
    // (legacy_name, unified_name, notes)

    // Click variations -> Click
    ("Click Element", "Click", ""),
    ("Click Widget", "Click", "SWT legacy"),
    ("Click Button", "Click", ""),
    ("Double Click", "Click", "Use click_count=2"),
    ("Double Click Element", "Click", "Use click_count=2"),
    ("Right Click", "Click", "Use modifiers=['right']"),

    // Input variations -> TypeText
    ("Input Text", "Type Text", ""),
    ("Enter Text", "Type Text", ""),

    // Selection variations -> SelectItem
    ("Select From Combobox", "Select Item", ""),
    ("Select From List", "Select Item", ""),
    ("Select List Item By Index", "Select Item", "Use by=index"),
    ("Select Tree Node", "Select Item", "Use by=path"),
    ("Select Tab", "Select Item", ""),
    ("Select Table Row", "Select Item", "Use by=index"),

    // Checkbox variations -> SetCheckbox
    ("Check Checkbox", "Set Checkbox", "Use state=True"),
    ("Uncheck Checkbox", "Set Checkbox", "Use state=False"),
    ("Check", "Set Checkbox", "Use state=True"),
    ("Uncheck", "Set Checkbox", "Use state=False"),
    ("Check Button", "Set Checkbox", "SWT legacy"),
    ("Uncheck Button", "Set Checkbox", "SWT legacy"),
    ("Select Radio Button", "Set Checkbox", "Use state=True"),

    // Get variations -> GetText, GetValue, GetProperty
    ("Get Element Text", "Get Text", ""),
    ("Element Text Should Be", "Get Text", "Use operator='=='"),
    ("Element Text Should Contain", "Get Text", "Use operator='contains'"),
    ("Get Table Cell Value", "Get Property", "Use property='cellValue[r,c]'"),
    ("Get Table Row Count", "Get Property", "Use property='rowCount'"),

    // State variations -> GetElementStates
    ("Element Should Be Visible", "Get Element States", "Use operator='contains', expected='visible'"),
    ("Element Should Not Be Visible", "Get Element States", "Use operator='not contains', expected='visible'"),
    ("Element Should Be Enabled", "Get Element States", "Use operator='contains', expected='enabled'"),
    ("Element Should Be Disabled", "Get Element States", "Use operator='not contains', expected='enabled'"),
    ("Element Should Exist", "Get Element Count", "Use operator='>=', expected=1"),
    ("Element Should Not Exist", "Get Element Count", "Use operator='==', expected=0"),
    ("Widget Should Be Visible", "Get Element States", "SWT legacy"),
    ("Widget Should Be Enabled", "Get Element States", "SWT legacy"),

    // Wait variations -> GetX with timeout
    ("Wait Until Element Exists", "Get Element Count", "Use operator='>=', expected=1, timeout=X"),
    ("Wait Until Element Is Visible", "Get Element States", "Use operator='contains', expected='visible', timeout=X"),
    ("Wait Until Element Is Enabled", "Get Element States", "Use operator='contains', expected='enabled', timeout=X"),
    ("Wait For Element", "Get Element Count", "Use operator='>=', expected=1, timeout=X"),

    // Tree variations -> SelectItem or GetProperty
    ("Expand Tree Node", "Select Item", "Automatic expansion"),
    ("Collapse Tree Node", "Select Item", "Use custom action"),
    ("Get Selected Tree Node", "Get Property", "Use property='selectedValue'"),

    // UI Tree
    ("Get Component Tree", "Get UI Tree", ""),
    ("Log Component Tree", "Log UI Tree", ""),
    ("Save UI Tree", "Get UI Tree", "Save externally"),

    // Find
    ("Find Element", "Find Elements", "Returns first match"),
    ("Find Widget", "Find Elements", "SWT legacy"),
    ("Find Widgets", "Find Elements", "SWT legacy"),

    // Connection
    ("Connect To Application", "Connect To Application", "Unchanged"),
    ("Disconnect", "Disconnect", "Unchanged"),
    ("Is Connected", "Is Connected", "Unchanged"),
];
```

## Consequences

### Positive

1. **Reduced Cognitive Load**: 18 keywords vs 60-70 to learn
2. **Powerful Assertions**: Inline assertions reduce test verbosity by ~30-40%
3. **Consistent Patterns**: Same pattern across all component types
4. **Built-in Retry**: Automatic retry eliminates explicit waits in most cases
5. **Better Error Messages**: Assertion failures include full context
6. **Browser Library Alignment**: Familiar patterns for modern RF users
7. **Full Compatibility**: All legacy keywords available as aliases

### Negative

1. **Migration Effort**: Users must learn new patterns for best experience
2. **Documentation**: Need both new API docs and migration guides
3. **Mental Model Shift**: "Get with Assert" pattern is different from traditional
4. **Operator Learning**: Users must learn assertion operators

### Risks

1. **Alias Complexity**: Many aliases to maintain
2. **Performance**: Retry mechanism adds overhead to assertions
3. **Backward Compatibility Bugs**: Complex aliasing may have edge cases
4. **Adoption**: Users comfortable with old API may resist change

## Alternatives Considered

### Alternative 1: Keep Separate Wait/Assert Keywords

Maintain explicit `Wait Until X` and `Element Should Be X` keywords.

**Rejected because**:
- More verbose tests
- More keywords to maintain
- Not aligned with modern Browser Library patterns

### Alternative 2: Only Reduce, No Assertion Engine

Consolidate keywords but don't add inline assertions.

**Rejected because**:
- Misses main benefit of Browser Library pattern
- Tests remain verbose
- Users still need separate wait/assert keywords

### Alternative 3: Full Browser Library Parity

Copy Browser Library API exactly.

**Rejected because**:
- Desktop UI has different paradigms than web
- Some Browser Library concepts don't apply (promises, etc.)
- Would lose Java GUI-specific features

## Implementation Plan

1. **Phase 1**: Implement assertion engine core (1 week)
2. **Phase 2**: Implement 6 Get keywords with assertion support (1 week)
3. **Phase 3**: Implement 5 action keywords (1 week)
4. **Phase 4**: Implement session and introspection keywords (3 days)
5. **Phase 5**: Implement backwards compatibility aliases (1 week)
6. **Phase 6**: Update documentation and migration guide (1 week)
7. **Phase 7**: Integration testing with existing test suites (1 week)

## References

- [ADR-003: Keyword Naming Convention](./ADR-003-keyword-naming-convention.md)
- [ADR-005: Error Handling Strategy](./ADR-005-error-handling-strategy.md)
- [Browser Library Assertion Engine](https://robotframework-browser.org/#assertions)
- [Browser Library API Reference](https://marketsquare.github.io/robotframework-browser/Browser.html)
- [Robot Framework User Guide](https://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html)
