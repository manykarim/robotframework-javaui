# Robot Framework Swing Library

A high-performance Robot Framework library for automating Java Swing applications. Built with Rust and PyO3 for optimal performance, this library provides comprehensive support for testing Swing-based desktop applications.

## Features

- **High Performance**: Core library written in Rust with PyO3 bindings for Python
- **CSS-like Selectors**: Intuitive element locators similar to web testing
- **XPath Support**: Full XPath-style locator syntax for complex queries
- **Comprehensive Component Support**: Buttons, text fields, tables, trees, lists, menus, and more
- **Java Agent**: Non-invasive instrumentation via Java agent
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Locator Syntax](#locator-syntax)
- [Keywords Reference](#keywords-reference)
- [Examples](#examples)
- [Architecture](#architecture)
- [Development](#development)
- [License](#license)

## Installation

### Prerequisites

- Python 3.8 or higher
- Java 11 or higher (for running Swing applications)
- Rust toolchain (for building from source)

### Install from Source

```bash
# Clone the repository
git clone https://github.com/manykarim/robotframework-javaui.git
cd robotframework-javaui

# Install with uv (recommended)
uv pip install -e .

# Or install with pip
pip install -e .
```

### Build the Java Agent

The Java agent is required for instrumenting Swing applications:

```bash
cd agent
mvn package
```

This creates `agent/target/robotframework-swing-agent-1.0.0-jar-with-dependencies.jar`.

### Build the Demo Application (Optional)

A demo Swing application is included for testing:

```bash
cd demo
mvn package
```

## Quick Start

### 1. Start Your Swing Application with the Agent

```bash
java -javaagent:path/to/robotframework-swing-agent-1.0.0-jar-with-dependencies.jar=port=5678 -jar your-app.jar
```

### 2. Create a Robot Framework Test

```robotframework
*** Settings ***
Library    swing_library.SwingLibrary

*** Test Cases ***
Example Login Test
    Connect To Application    main_class=com.example.MyApp    host=localhost    port=5678
    Input Text    [name='username']    admin
    Input Text    [name='password']    secret
    Click    JButton[text='Login']
    Element Should Exist    JLabel[text='Welcome']
    [Teardown]    Disconnect
```

### 3. Run the Test

```bash
robot my_test.robot
```

## Locator Syntax

The library supports multiple locator strategies for finding UI elements.

### CSS-like Selectors

| Selector | Description | Example |
|----------|-------------|---------|
| `Type` | Match by component type | `JButton` |
| `[attr='value']` | Exact attribute match | `[name='loginBtn']` |
| `[attr*='value']` | Attribute contains | `[text*='Submit']` |
| `[attr^='value']` | Attribute starts with | `[name^='btn_']` |
| `[attr$='value']` | Attribute ends with | `[text$='...']` |
| `Type[attr='value']` | Type with attribute | `JButton[name='ok']` |
| `Parent > Child` | Direct child | `JPanel > JButton` |
| `Ancestor Descendant` | Descendant | `JFrame JButton` |
| `:enabled` | Enabled elements | `JButton:enabled` |
| `:visible` | Visible elements | `JLabel:visible` |
| `:first-child` | First child | `JButton:first-child` |
| `:nth-child(n)` | Nth child | `JButton:nth-child(2)` |

### XPath-style Selectors

| Selector | Description | Example |
|----------|-------------|---------|
| `//Type` | Any descendant | `//JButton` |
| `//Type[@attr='value']` | With attribute | `//JButton[@name='ok']` |
| `//Type[n]` | By index (1-based) | `//JButton[1]` |

### Combined Selectors

```robotframework
# Multiple attributes
JButton[name='submit'][text='OK']:enabled

# Nested with pseudo-selectors
JPanel[name='form'] JTextField:visible

# XPath with multiple predicates
//JTable[@name='data']//JButton[@text='Edit']
```

## Keywords Reference

### Connection Keywords

| Keyword | Arguments | Description |
|---------|-----------|-------------|
| `Connect To Application` | `main_class=`, `title=`, `host=`, `port=`, `timeout=` | Connect to a running Swing application |
| `Disconnect` | | Disconnect from the application |
| `Is Connected` | | Returns connection status |

### Element Finding

| Keyword | Arguments | Description |
|---------|-----------|-------------|
| `Find Element` | `locator` | Find single element |
| `Find Elements` | `locator` | Find all matching elements |
| `Element Should Exist` | `locator` | Assert element exists |
| `Element Should Not Exist` | `locator` | Assert element doesn't exist |

### Mouse Actions

| Keyword | Arguments | Description |
|---------|-----------|-------------|
| `Click` | `locator` | Single click |
| `Double Click` | `locator` | Double click |
| `Right Click` | `locator` | Context menu click |
| `Click Button` | `locator` | Click a button |

### Text Input

| Keyword | Arguments | Description |
|---------|-----------|-------------|
| `Input Text` | `locator`, `text`, `clear=True` | Enter text (optionally clear first) |
| `Type Text` | `locator`, `text` | Type text character by character |
| `Clear Text` | `locator` | Clear text field |
| `Get Element Text` | `locator` | Get element's text content |

### Table Operations

| Keyword | Arguments | Description |
|---------|-----------|-------------|
| `Get Table Row Count` | `locator` | Get number of rows |
| `Get Table Column Count` | `locator` | Get number of columns |
| `Get Table Cell Value` | `locator`, `row`, `column` | Get cell value |
| `Get Table Data` | `locator` | Get all table data as list |
| `Select Table Cell` | `locator`, `row`, `column` | Select a cell |
| `Select Table Row` | `locator`, `row` | Select a row |

### Tree Operations

| Keyword | Arguments | Description |
|---------|-----------|-------------|
| `Expand Tree Node` | `locator`, `path` | Expand a tree node |
| `Collapse Tree Node` | `locator`, `path` | Collapse a tree node |
| `Select Tree Node` | `locator`, `path` | Select a tree node |
| `Get Tree Nodes` | `locator` | Get all tree nodes |

### List Operations

| Keyword | Arguments | Description |
|---------|-----------|-------------|
| `Get List Items` | `locator` | Get all list items |
| `Select From List` | `locator`, `value` | Select item by value |
| `Select List Item By Index` | `locator`, `index` | Select item by index |

### Form Controls

| Keyword | Arguments | Description |
|---------|-----------|-------------|
| `Select From Combobox` | `locator`, `value` | Select dropdown value |
| `Check Checkbox` | `locator` | Check a checkbox |
| `Uncheck Checkbox` | `locator` | Uncheck a checkbox |
| `Select Radio Button` | `locator` | Select radio button |
| `Select Tab` | `locator`, `tab_name` | Select tab in tabbed pane |

### Verification

| Keyword | Arguments | Description |
|---------|-----------|-------------|
| `Element Should Be Visible` | `locator` | Assert element is visible |
| `Element Should Be Enabled` | `locator` | Assert element is enabled |
| `Element Should Be Selected` | `locator` | Assert element is selected |
| `Element Text Should Be` | `locator`, `expected` | Assert exact text match |
| `Element Text Should Contain` | `locator`, `expected` | Assert text contains |

### Wait Operations

| Keyword | Arguments | Description |
|---------|-----------|-------------|
| `Wait For Element` | `locator`, `timeout=` | Wait for element to exist |
| `Wait Until Element Visible` | `locator`, `timeout=` | Wait for visibility |
| `Wait Until Element Enabled` | `locator`, `timeout=` | Wait for enabled state |
| `Wait Until Element Contains` | `locator`, `text`, `timeout=` | Wait for text content |

### UI Tree Inspection

| Keyword | Arguments | Description |
|---------|-----------|-------------|
| `Get Ui Tree` | `format=text` | Get component hierarchy |
| `Log Ui Tree` | | Log UI tree to console |
| `Refresh Ui Tree` | | Refresh cached UI tree |

### Screenshots

| Keyword | Arguments | Description |
|---------|-----------|-------------|
| `Capture Screenshot` | `filename=` | Capture window screenshot |
| `Set Screenshot Directory` | `directory` | Set output directory |

### Properties

| Keyword | Arguments | Description |
|---------|-----------|-------------|
| `Get Element Property` | `locator`, `property` | Get specific property |
| `Get Element Properties` | `locator` | Get all properties |

## Examples

### Complete Login Test Suite

```robotframework
*** Settings ***
Documentation     Login functionality test suite
Library           swing_library.SwingLibrary
Library           Process
Suite Setup       Start Application
Suite Teardown    Stop Application

*** Variables ***
${APP_JAR}        path/to/myapp.jar
${AGENT_JAR}      path/to/robotframework-swing-agent-1.0.0-jar-with-dependencies.jar
${PORT}           5678

*** Keywords ***
Start Application
    ${cmd}=    Set Variable    java -javaagent:${AGENT_JAR}=port=${PORT} -jar ${APP_JAR}
    Start Process    ${cmd}    shell=True    alias=app
    Sleep    3s
    Connect To Application    main_class=com.example.MyApp    port=${PORT}

Stop Application
    Disconnect
    Terminate Process    app    kill=True

*** Test Cases ***
Valid Login Should Succeed
    [Documentation]    Test successful login with valid credentials
    Clear Text    JTextField[name='username']
    Clear Text    JPasswordField[name='password']
    Input Text    JTextField[name='username']    admin
    Input Text    JPasswordField[name='password']    password123
    Click    JButton[text='Login']
    Wait Until Element Visible    JLabel[text*='Welcome']    timeout=5

Invalid Login Should Show Error
    [Documentation]    Test error message with invalid credentials
    Clear Text    [name='username']
    Clear Text    [name='password']
    Input Text    [name='username']    invalid
    Input Text    [name='password']    wrong
    Click    JButton[text='Login']
    Wait Until Element Contains    JLabel[name='status']    Invalid    timeout=5

Empty Fields Should Be Rejected
    [Documentation]    Test validation for empty fields
    Clear Text    [name='username']
    Clear Text    [name='password']
    Click    JButton[text='Login']
    Element Should Exist    JLabel[text*='required']
```

### Table Data Verification

```robotframework
*** Test Cases ***
Verify Table Contains Expected Data
    [Documentation]    Verify table displays correct data
    ${row_count}=    Get Table Row Count    JTable[name='dataTable']
    Should Be True    ${row_count} >= 5

    # Verify first row
    ${name}=    Get Table Cell Value    JTable[name='dataTable']    0    1
    Should Be Equal    ${name}    John Doe

    ${email}=    Get Table Cell Value    JTable[name='dataTable']    0    2
    Should Be Equal    ${email}    john@example.com

Iterate Through Table Rows
    [Documentation]    Process all table rows
    ${data}=    Get Table Data    JTable[name='dataTable']
    FOR    ${row}    IN    @{data}
        Log    Processing: ${row}
    END

Navigate Table With Loops
    [Documentation]    Navigate through table cells
    FOR    ${row}    IN RANGE    0    3
        FOR    ${col}    IN RANGE    0    5
            ${value}=    Get Table Cell Value    [name='dataTable']    ${row}    ${col}
            Log    Cell [${row}][${col}] = ${value}
        END
    END
```

### Tree Navigation

```robotframework
*** Test Cases ***
Navigate File Tree
    [Documentation]    Navigate through tree structure
    Element Should Exist    JTree[name='fileTree']

    # Expand nodes
    Expand Tree Node    JTree[name='fileTree']    Root
    Expand Tree Node    JTree[name='fileTree']    Root/Documents

    # Select a node
    Select Tree Node    JTree[name='fileTree']    Root/Documents/Reports

    # Verify selection
    ${nodes}=    Get Tree Nodes    JTree[name='fileTree']
    Should Not Be Empty    ${nodes}
```

### Menu Operations

```robotframework
*** Test Cases ***
Navigate Application Menu
    [Documentation]    Test menu navigation
    Element Should Exist    JMenu[text='File']
    Element Should Exist    JMenu[text='Edit']
    Element Should Exist    JMenu[text='Help']

    # Click menu
    Click    JMenu[text='File']
    Sleep    0.3s
    Element Should Be Visible    JMenuItem[name='menuNew']
```

### Form Control Interactions

```robotframework
*** Test Cases ***
Fill Complete Form
    [Documentation]    Test various form controls
    # Text fields
    Input Text    JTextField[name='firstName']    John
    Input Text    JTextField[name='lastName']    Doe

    # Combo box (dropdown)
    Select From Combobox    JComboBox[name='country']    United States

    # Checkbox
    Check Checkbox    JCheckBox[name='subscribe']

    # Radio buttons
    Select Radio Button    JRadioButton[name='optionA']

    # Text area
    Input Text    JTextArea[name='comments']    This is a comment.

    # Submit
    Click    JButton[text='Submit']
```

### Wait and Synchronization

```robotframework
*** Test Cases ***
Wait For Dynamic Content
    [Documentation]    Handle asynchronous updates
    Click    JButton[name='loadData']

    # Wait for loading indicator to disappear
    Wait Until Element Not Visible    JLabel[name='loading']    timeout=10

    # Wait for data to appear
    Wait Until Element Visible    JTable[name='results']    timeout=10

    # Verify data loaded
    ${count}=    Get Table Row Count    JTable[name='results']
    Should Be True    ${count} > 0
```

### UI Tree Debugging

```robotframework
*** Test Cases ***
Debug UI Structure
    [Documentation]    Inspect application UI structure
    # Log full UI tree
    Log Ui Tree

    # Get tree as text
    ${tree}=    Get Ui Tree    format=text
    Should Contain    ${tree}    JButton
    Should Contain    ${tree}    JTextField

    # Find all buttons
    ${buttons}=    Find Elements    JButton
    ${count}=    Get Length    ${buttons}
    Log    Found ${count} buttons
```

## Architecture

```
robotframework-javaui/
├── python/                 # Python package
│   └── swing_library/      # Robot Framework library
│       └── __init__.py     # SwingLibrary class
├── src/                    # Rust source code
│   ├── lib.rs              # PyO3 bindings
│   ├── locator/            # Locator parsing (pest grammar)
│   ├── connection/         # RPC client
│   └── element/            # Element operations
├── agent/                  # Java agent
│   └── src/                # Agent source
├── demo/                   # Demo Swing application
└── tests/                  # Test suites
    └── robot/              # Robot Framework tests
```

### How It Works

1. **Java Agent**: Attaches to the JVM and provides RPC endpoints for UI inspection and control
2. **Rust Core**: High-performance element matching, locator parsing, and RPC communication
3. **Python Bindings**: PyO3-based interface exposing Robot Framework keywords
4. **Robot Framework**: Test execution and reporting

## Development

### Building from Source

```bash
# Install development dependencies
uv pip install -e ".[dev]"

# Build Rust extension
maturin develop

# Build Java agent
cd agent && mvn package

# Build demo app
cd demo && mvn package
```

### Running Tests

```bash
# Run Robot Framework tests
uv run robot tests/robot/

# Run Python unit tests
uv run pytest tests/python/

# Run specific test suite
uv run robot tests/robot/02_locators.robot
```

### Project Structure

| Directory | Description |
|-----------|-------------|
| `python/` | Python Robot Framework library |
| `src/` | Rust core library |
| `agent/` | Java instrumentation agent |
| `demo/` | Demo Swing application |
| `tests/robot/` | Robot Framework test suites |
| `tests/python/` | Python unit tests |
| `docs/` | Documentation |
| `schemas/` | JSON/YAML schemas |

## Configuration

### Agent Configuration

The Java agent accepts these JVM arguments:

```bash
java -javaagent:swing-agent.jar=port=5678,debug=true -jar app.jar
```

| Option | Default | Description |
|--------|---------|-------------|
| `port` | 5678 | RPC server port |
| `debug` | false | Enable debug logging |

### Library Configuration

```robotframework
*** Settings ***
Library    swing_library.SwingLibrary    timeout=30    screenshot_dir=screenshots
```

| Option | Default | Description |
|--------|---------|-------------|
| `timeout` | 10 | Default wait timeout (seconds) |
| `screenshot_dir` | . | Screenshot output directory |

## Troubleshooting

### Connection Issues

```
SwingConnectionError: Connection refused
```

- Ensure the application is running with the agent loaded
- Verify the port matches between agent and library
- Check firewall settings

### Element Not Found

```
ElementNotFoundError: Element not found: JButton[name='xyz']
```

- Use `Log Ui Tree` to inspect available elements
- Verify element names and attributes
- Check if element is visible/enabled

### EDT Threading Errors

```
SwingConnectionError: EDT callable failed
```

- Some operations require visible components
- Use wait keywords before interacting
- Ensure proper tab/window focus

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Apache License 2.0. See [LICENSE](LICENSE) for details.

## Acknowledgments

- [Robot Framework](https://robotframework.org/) - Test automation framework
- [PyO3](https://pyo3.rs/) - Rust bindings for Python
- [pest](https://pest.rs/) - Parser library for Rust
