# Robot Framework Keywords Reference

Complete reference for all Robot Framework keywords in the JavaGui library.

## Component Tree Keywords

### Get Component Tree

Get the UI component hierarchy with optional filtering and formatting.

**Syntax:**
```robot
Get Component Tree
    [locator=None]
    [format=text]
    [max_depth=None]
    [types=None]
    [exclude_types=None]
    [visible_only=False]
    [enabled_only=False]
    [focusable_only=False]
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `locator` | str | None (root) | Component locator to start from |
| `format` | str | text | Output format: `text`, `json`, `xml`, `yaml`, `csv`, `markdown` |
| `max_depth` | int | None (unlimited) | Maximum tree depth (1-50) |
| `types` | str | None (all) | Include types (comma-separated, supports wildcards: `*`, `?`) |
| `exclude_types` | str | None | Exclude types (comma-separated, supports wildcards) |
| `visible_only` | bool | False | Include only visible components |
| `enabled_only` | bool | False | Include only enabled components |
| `focusable_only` | bool | False | Include only focusable components |

**Returns:** String representation of component tree in specified format

**Examples:**

```robot
*** Test Cases ***
# Basic usage - full tree as text
${tree}=    Get Component Tree
Log    ${tree}

# JSON format with depth limit
${tree}=    Get Component Tree    format=json    max_depth=10

# Filter by type
${buttons}=    Get Component Tree    types=JButton    format=json

# Multiple types with wildcards
${inputs}=    Get Component Tree    types=J*Field,J*Area    format=json

# Exclude types
${no_panels}=    Get Component Tree    exclude_types=JPanel,JLabel

# State filtering
${visible}=    Get Component Tree    visible_only=${True}    format=json
${enabled}=    Get Component Tree    enabled_only=${True}    format=json

# Combined filters
${interactive}=    Get Component Tree
...    types=J*Button,JTextField
...    visible_only=${True}
...    enabled_only=${True}
...    max_depth=8
...    format=json

# Different formats
${text}=      Get Component Tree    format=text
${json}=      Get Component Tree    format=json
${xml}=       Get Component Tree    format=xml
${yaml}=      Get Component Tree    format=yaml
${csv}=       Get Component Tree    format=csv
${markdown}=  Get Component Tree    format=markdown

# Starting from specific component
${form}=    Get Component Tree    locator=JPanel[name='loginForm']    format=json
```

**Output Format Examples:**

**TEXT:**
```
JFrame [MyApp] (0)
├─ JMenuBar (1)
│  └─ JMenu [File] (2)
│     ├─ JMenuItem [Open] (3)
│     └─ JMenuItem [Save] (4)
└─ JPanel [mainPanel] (5)
   ├─ JLabel [title] (6)
   └─ JButton [submit] (7)
```

**JSON:**
```json
{
  "type": "JFrame",
  "name": "MyApp",
  "id": 0,
  "children": [
    {
      "type": "JMenuBar",
      "id": 1,
      "children": [...]
    }
  ]
}
```

**CSV:**
```csv
type,name,text,id,visible,enabled,parent_id
JFrame,MyApp,,0,true,true,
JMenuBar,,,1,true,true,0
JMenu,File,File,2,true,true,1
```

**Performance Notes:**
- For large UIs (> 500 components), use `max_depth` to limit tree traversal
- Use `Get Component Subtree` for faster queries on specific sections
- Type filtering reduces output size significantly

**See Also:**
- [Get Component Subtree](#get-component-subtree)
- [Log Component Tree](#log-component-tree)
- [Refresh Component Tree](#refresh-component-tree)

---

### Get Component Subtree

Get component tree starting from a specific component. Faster than `Get Component Tree` for targeted queries.

**Syntax:**
```robot
Get Component Subtree
    locator
    [format=text]
    [max_depth=None]
    [types=None]
    [exclude_types=None]
    [visible_only=False]
    [enabled_only=False]
    [focusable_only=False]
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `locator` | str | **Required** | Component locator to start from |
| `format` | str | text | Output format (same as Get Component Tree) |
| `max_depth` | int | None | Maximum depth from starting component |
| Other parameters | | | Same as Get Component Tree |

**Returns:** String representation of subtree in specified format

**Examples:**

```robot
*** Test Cases ***
# Get subtree from specific panel
${form}=    Get Component Subtree    JPanel[name='loginForm']
Log    ${form}

# JSON format with depth limit
${menu}=    Get Component Subtree    JMenuBar    format=json    max_depth=3

# Filter subtree by type
${buttons}=    Get Component Subtree
...    JPanel[name='toolbar']
...    types=J*Button
...    format=json

# Only visible components in subtree
${visible}=    Get Component Subtree
...    JPanel[name='main']
...    visible_only=${True}
...    format=json
```

**Performance:**
- **Up to 50x faster** than full tree scan on large UIs
- Recommended for UIs with > 500 components
- Use when you know the specific section to inspect

**Use Cases:**
- Inspecting specific dialogs or panels
- Testing specific forms or sections
- Performance-critical tree queries
- Reducing log output size

**See Also:**
- [Get Component Tree](#get-component-tree)

---

### Log Component Tree

Log the component tree to Robot Framework log at specified level.

**Syntax:**
```robot
Log Component Tree
    [locator=None]
    [format=text]
    [level=INFO]
    [max_depth=None]
    [types=None]
    [exclude_types=None]
    [visible_only=False]
    [enabled_only=False]
    [focusable_only=False]
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `level` | str | INFO | Log level: `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR` |
| Other parameters | | | Same as Get Component Tree |

**Returns:** None (logs to Robot Framework log)

**Examples:**

```robot
*** Test Cases ***
# Log full tree
Log Component Tree

# Log at DEBUG level
Log Component Tree    level=DEBUG

# Log filtered tree
Log Component Tree    types=J*Button    visible_only=${True}

# Log JSON format
Log Component Tree    format=json    max_depth=5    level=DEBUG

# Log specific subtree
Log Component Tree    locator=JPanel[name='form']    format=text
```

**See Also:**
- [Get Component Tree](#get-component-tree)

---

### Refresh Component Tree

Refresh the cached component tree. Call this after dynamic UI changes.

**Syntax:**
```robot
Refresh Component Tree
```

**Parameters:** None

**Returns:** None

**Examples:**

```robot
*** Test Cases ***
Test Dynamic UI
    # Initial tree inspection
    ${tree1}=    Get Component Tree

    # Trigger UI change
    Click    JButton[name='addPanel']

    # Refresh tree to see changes
    Refresh Component Tree

    # Get updated tree
    ${tree2}=    Get Component Tree

    Should Not Be Equal    ${tree1}    ${tree2}
```

**When to Use:**
- After adding/removing components dynamically
- After dialog open/close
- After tab switching
- After expanding/collapsing panels

---

### Get Ui Tree (Deprecated)

**⚠️ Deprecated:** Use `Get Component Tree` instead.

Legacy keyword for backward compatibility. Returns text-format component tree.

**Syntax:**
```robot
Get Ui Tree    [format=text]
```

**Migration:**
```robot
# Old way
${tree}=    Get Ui Tree

# New way (recommended)
${tree}=    Get Component Tree    format=text
```

---

### Log Ui Tree (Deprecated)

**⚠️ Deprecated:** Use `Log Component Tree` instead.

Legacy keyword for backward compatibility.

**Migration:**
```robot
# Old way
Log Ui Tree

# New way (recommended)
Log Component Tree
```

---

### Refresh Ui Tree (Deprecated)

**⚠️ Deprecated:** Use `Refresh Component Tree` instead.

**Migration:**
```robot
# Old way
Refresh Ui Tree

# New way (recommended)
Refresh Component Tree
```

---

## Connection Keywords

### Connect To Application

Connect to a running Java application with the agent loaded.

**Syntax:**
```robot
Connect To Application
    [main_class=None]
    [title=None]
    [host=localhost]
    [port=5678]
    [timeout=30]
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `main_class` | str | None | Application main class name |
| `title` | str | None | Window title to match |
| `host` | str | localhost | Agent host |
| `port` | int | 5678 | Agent port |
| `timeout` | int | 30 | Connection timeout (seconds) |

**Examples:**

```robot
*** Test Cases ***
# Connect by main class
Connect To Application    main_class=com.example.MyApp

# Connect by window title
Connect To Application    title=My Application

# Connect to remote host
Connect To Application    host=192.168.1.100    port=5678

# Custom timeout
Connect To Application    main_class=com.example.App    timeout=60
```

---

### Disconnect

Disconnect from the application.

**Syntax:**
```robot
Disconnect
```

**Examples:**

```robot
*** Test Cases ***
Test With Cleanup
    Connect To Application    main_class=com.example.App
    # ... test steps ...
    [Teardown]    Disconnect
```

---

### Is Connected

Check if currently connected to application.

**Syntax:**
```robot
Is Connected
```

**Returns:** Boolean (True if connected)

**Examples:**

```robot
*** Test Cases ***
Check Connection
    ${connected}=    Is Connected
    Run Keyword If    ${connected}    Log    Connected
    ...    ELSE    Connect To Application    main_class=com.example.App
```

---

## Element Finding Keywords

### Find Element

Find a single element matching the locator.

**Syntax:**
```robot
Find Element    locator
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `locator` | str | Element locator (CSS or XPath style) |

**Returns:** Element object

**Examples:**

```robot
*** Test Cases ***
${element}=    Find Element    JButton[name='submit']
${element}=    Find Element    //JTextField[@name='username']
${element}=    Find Element    JPanel > JButton:first-child
```

---

### Find Elements

Find all elements matching the locator.

**Syntax:**
```robot
Find Elements    locator
```

**Returns:** List of element objects

**Examples:**

```robot
*** Test Cases ***
${buttons}=    Find Elements    JButton
${fields}=     Find Elements    J*Field
${visible}=    Find Elements    JLabel:visible
```

---

### Element Should Exist

Assert that element exists.

**Syntax:**
```robot
Element Should Exist    locator    [message=None]
```

**Examples:**

```robot
*** Test Cases ***
Element Should Exist    JButton[name='submit']
Element Should Exist    JTextField[name='username']    message=Username field not found
```

---

### Element Should Not Exist

Assert that element does not exist.

**Syntax:**
```robot
Element Should Not Exist    locator    [message=None]
```

**Examples:**

```robot
*** Test Cases ***
Element Should Not Exist    JLabel[text='Error']
Element Should Not Exist    JDialog[name='errorDialog']    message=Error dialog should not appear
```

---

## Mouse Action Keywords

### Click

Single-click an element.

**Syntax:**
```robot
Click    locator
```

**Examples:**

```robot
*** Test Cases ***
Click    JButton[name='submit']
Click    JMenuItem[text='File']
Click    //JButton[@text='OK']
```

---

### Double Click

Double-click an element.

**Syntax:**
```robot
Double Click    locator
```

**Examples:**

```robot
*** Test Cases ***
Double Click    TreeItem[text='file.txt']
Double Click    JList > JLabel[text='Item 1']
```

---

### Right Click

Right-click (context menu) an element.

**Syntax:**
```robot
Right Click    locator
```

**Examples:**

```robot
*** Test Cases ***
Right Click    TreeItem[text='file.txt']
Right Click    JTextArea[name='editor']
```

---

### Click Button

Click a button element (convenience keyword).

**Syntax:**
```robot
Click Button    locator
```

**Examples:**

```robot
*** Test Cases ***
Click Button    [name='submit']
Click Button    [text='OK']
```

---

## Text Input Keywords

### Input Text

Enter text into a text field.

**Syntax:**
```robot
Input Text    locator    text    [clear=True]
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `locator` | str | Required | Text field locator |
| `text` | str | Required | Text to enter |
| `clear` | bool | True | Clear field before entering text |

**Examples:**

```robot
*** Test Cases ***
Input Text    JTextField[name='username']    admin
Input Text    [name='password']    secret123    clear=False
Input Text    JTextArea[name='comment']    Multi-line\ntext\nhere
```

---

### Type Text

Type text character by character (simulates keyboard input).

**Syntax:**
```robot
Type Text    locator    text
```

**Examples:**

```robot
*** Test Cases ***
Type Text    JTextField[name='search']    slow typing
```

---

### Clear Text

Clear a text field.

**Syntax:**
```robot
Clear Text    locator
```

**Examples:**

```robot
*** Test Cases ***
Clear Text    JTextField[name='username']
Clear Text    JTextArea[name='notes']
```

---

### Get Element Text

Get text content of an element.

**Syntax:**
```robot
Get Element Text    locator
```

**Returns:** String (element text)

**Examples:**

```robot
*** Test Cases ***
${text}=    Get Element Text    JLabel[name='status']
${value}=   Get Element Text    JTextField[name='username']
```

---

## Table Operation Keywords

### Get Table Row Count

Get number of rows in a table (supports inline assertions).

**Syntax:**
```robot
Get Table Row Count    locator    [assertion_operator]    [expected]    [message]    [timeout]
```

**Examples:**

```robot
*** Test Cases ***
# Without assertion
${count}=    Get Table Row Count    JTable[name='data']

# With assertion
Get Table Row Count    JTable[name='data']    >=    5
Get Table Row Count    JTable[name='users']    ==    10    timeout=5
```

---

### Get Table Column Count

Get number of columns in a table (supports inline assertions).

**Syntax:**
```robot
Get Table Column Count    locator    [assertion_operator]    [expected]    [message]    [timeout]
```

**Examples:**

```robot
*** Test Cases ***
${count}=    Get Table Column Count    JTable[name='data']
Get Table Column Count    JTable[name='data']    ==    5
```

---

### Get Table Cell Value

Get value of a table cell (supports inline assertions).

**Syntax:**
```robot
Get Table Cell Value    locator    row    column    [assertion_operator]    [expected]    [message]    [timeout]
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `locator` | str | Table locator |
| `row` | int | Row index (0-based) |
| `column` | int | Column index (0-based) |

**Examples:**

```robot
*** Test Cases ***
${value}=    Get Table Cell Value    JTable[name='data']    0    1
Get Table Cell Value    JTable[name='data']    0    0    ==    John Doe
Get Table Cell Value    JTable[name='data']    2    3    contains    @example.com
```

---

### Get Table Data

Get all table data as a list of rows.

**Syntax:**
```robot
Get Table Data    locator
```

**Returns:** List of lists (rows and columns)

**Examples:**

```robot
*** Test Cases ***
${data}=    Get Table Data    JTable[name='results']
${row0}=    Get From List    ${data}    0
${cell}=    Get From List    ${row0}    1
```

---

### Get Table Row Values

Get all values from a specific row (supports inline assertions).

**Syntax:**
```robot
Get Table Row Values    locator    row    [assertion_operator]    [expected]    [message]
```

**Examples:**

```robot
*** Test Cases ***
${row}=    Get Table Row Values    JTable[name='data']    0
Get Table Row Values    JTable[name='users']    0    contains    admin
```

---

### Get Table Column Values

Get all values from a specific column (supports inline assertions).

**Syntax:**
```robot
Get Table Column Values    locator    column    [assertion_operator]    [expected]    [message]
```

**Examples:**

```robot
*** Test Cases ***
${column}=    Get Table Column Values    JTable[name='data']    0
Get Table Column Values    JTable[name='users']    1    contains    john@example.com
```

---

### Select Table Cell

Select a specific table cell.

**Syntax:**
```robot
Select Table Cell    locator    row    column
```

**Examples:**

```robot
*** Test Cases ***
Select Table Cell    JTable[name='data']    0    1
Select Table Cell    JTable[name='results']    5    2
```

---

### Select Table Row

Select a table row.

**Syntax:**
```robot
Select Table Row    locator    row
```

**Examples:**

```robot
*** Test Cases ***
Select Table Row    JTable[name='data']    0
Select Table Row    JTable[name='users']    3
```

---

## Assertion-Enabled Get Keywords

All Get* keywords support inline assertions with automatic retry:

### Get Text

Get element text with optional assertion.

**Syntax:**
```robot
Get Text    locator    [assertion_operator]    [expected]    [message]    [timeout]    [formatters]
```

**Examples:**

```robot
*** Test Cases ***
${text}=    Get Text    JLabel[name='status']
Get Text    JLabel[name='status']    ==    Ready
Get Text    JLabel[name='status']    contains    Success    timeout=10
Get Text    JLabel[name='title']    ==    welcome    formatters=['lowercase']
```

---

### Get Value

Get input field value with optional assertion.

**Syntax:**
```robot
Get Value    locator    [assertion_operator]    [expected]    [message]    [timeout]
```

**Examples:**

```robot
*** Test Cases ***
${value}=    Get Value    JTextField[name='username']
Get Value    JTextField[name='username']    ==    admin
Get Value    JTextField[name='email']    contains    @example.com
```

---

### Get Element Count

Count matching elements with optional numeric assertion.

**Syntax:**
```robot
Get Element Count    locator    [assertion_operator]    [expected]    [message]    [timeout]
```

**Examples:**

```robot
*** Test Cases ***
${count}=    Get Element Count    JButton
Get Element Count    JButton    >=    5
Get Element Count    JTextField:visible    >    0
```

---

### Get Element States

Get element states with optional assertion.

**Syntax:**
```robot
Get Element States    locator    [assertion_operator]    [expected]    [message]    [timeout]
```

**Returns:** List of states: `visible`, `hidden`, `enabled`, `disabled`, `focused`, `unfocused`, `selected`, `unselected`, `checked`, `unchecked`, `editable`, `readonly`, `expanded`, `collapsed`, `attached`, `detached`

**Examples:**

```robot
*** Test Cases ***
${states}=    Get Element States    JButton[name='submit']
Get Element States    JButton[name='submit']    contains    enabled
Get Element States    JButton[name='submit']    contains    visible
```

---

### Get Property

Get element property with optional assertion.

**Syntax:**
```robot
Get Property    locator    property_name    [assertion_operator]    [expected]    [message]    [timeout]
```

**Examples:**

```robot
*** Test Cases ***
${enabled}=    Get Property    JButton[name='submit']    enabled
Get Property    JButton[name='submit']    enabled    ==    true
Get Property    JTextField[name='input']    editable    ==    true
```

---

## Configuration Keywords

### Set Assertion Timeout

Set default assertion retry timeout.

**Syntax:**
```robot
Set Assertion Timeout    timeout
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `timeout` | float | Timeout in seconds |

**Examples:**

```robot
*** Test Cases ***
Configure Timeouts
    Set Assertion Timeout    10
    Get Text    JLabel[name='slow']    ==    Loaded    # Uses 10s timeout
```

---

### Set Assertion Interval

Set retry interval between assertion attempts.

**Syntax:**
```robot
Set Assertion Interval    interval
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `interval` | float | Interval in seconds |

**Examples:**

```robot
*** Test Cases ***
Configure Retry
    Set Assertion Interval    0.5    # Check every 0.5 seconds
    Get Text    JLabel[name='status']    ==    Ready
```

---

## Assertion Operators

| Operator | Aliases | Description | Example |
|----------|---------|-------------|---------|
| `==` | `equal`, `equals`, `should be` | Exact equality | `Get Text    loc    ==    Hello` |
| `!=` | `inequal`, `should not be` | Not equal | `Get Text    loc    !=    Error` |
| `<` | `less than` | Less than (numeric) | `Get Element Count    JButton    <    10` |
| `>` | `greater than` | Greater than (numeric) | `Get Table Row Count    loc    >    0` |
| `<=` | | Less or equal | `Get Element Count    loc    <=    5` |
| `>=` | | Greater or equal | `Get Table Row Count    loc    >=    1` |
| `*=` | `contains` | Contains substring/item | `Get Text    loc    contains    success` |
| `^=` | `starts` | Starts with | `Get Text    loc    starts    Hello` |
| `$=` | `ends` | Ends with | `Get Text    loc    ends    world` |
| `matches` | | Regex match | `Get Text    loc    matches    \\d{3}-\\d{4}` |
| `validate` | | Custom expression | `Get Text    loc    validate    len(value) > 5` |
| `then` | | Return value only (no assert) | `${v}=    Get Text    loc    then    ` |

---

## Formatters

Apply text transformations before assertion:

| Formatter | Description |
|-----------|-------------|
| `normalize_spaces` | Collapse multiple whitespace to single space |
| `strip` | Remove leading/trailing whitespace |
| `lowercase` | Convert to lowercase |
| `uppercase` | Convert to uppercase |

**Examples:**

```robot
*** Test Cases ***
Get Text    JLabel[name='title']    ==    Hello World    formatters=['normalize_spaces', 'strip']
Get Text    JLabel[name='status']    ==    ready    formatters=['lowercase']
```

---

## Complete Example Test Suite

```robot
*** Settings ***
Documentation     Complete example using component tree and assertions
Library           JavaGui.Swing
Suite Setup       Connect To Application    main_class=com.example.MyApp
Suite Teardown    Disconnect

*** Test Cases ***
Inspect UI Structure
    # Get full tree
    ${tree}=    Get Component Tree    format=json    max_depth=10
    Log    ${tree}

    # Get only buttons
    ${buttons}=    Get Component Tree    types=J*Button    format=json
    Log    ${buttons}

Test Form With Assertions
    # Verify form elements exist and are visible
    Get Element States    JPanel[name='loginForm']    contains    visible
    Get Element Count    JTextField    >=    2

    # Fill form
    Input Text    JTextField[name='username']    admin
    Input Text    JPasswordField[name='password']    secret

    # Submit and verify with assertions
    Click    JButton[name='submit']
    Get Text    JLabel[name='status']    ==    Login successful    timeout=5

Validate Table Data
    # Check table has data
    Get Table Row Count    JTable[name='results']    >    0

    # Verify specific cell values
    Get Table Cell Value    JTable[name='results']    0    0    ==    John Doe
    Get Table Cell Value    JTable[name='results']    0    1    contains    @example.com

Inspect Specific Section
    # Get subtree for performance
    ${dialog}=    Get Component Subtree
    ...    JDialog[name='settings']
    ...    format=json
    ...    max_depth=5
    ...    visible_only=${True}

    Log    ${dialog}
```

---

## See Also

- [Quick Start Guide](../COMPONENT_TREE_QUICK_START.md)
- [Filtering Guide](../COMPONENT_TREE_FILTERING_GUIDE.md)
- [Output Formats Guide](../OUTPUT_FORMATS_GUIDE.md)
- [Migration Guide](../MIGRATION_GUIDE.md)
- [Performance Guide](../USER_PERFORMANCE_GUIDE.md)
