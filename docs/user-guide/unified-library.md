# JavaGui Unified Library User Guide

This guide covers the unified `JavaGuiLibrary` for Robot Framework, which provides automation capabilities for Java Swing, Eclipse SWT, and Eclipse RCP applications.

## Table of Contents

- [Getting Started](#getting-started)
- [Supported Modes](#supported-modes)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Locator Syntax](#locator-syntax)
- [Keyword Reference](#keyword-reference)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

## Getting Started

The JavaGui library enables automated testing of Java desktop applications built with:

- **Swing** - Standard Java GUI toolkit
- **SWT** - Eclipse Standard Widget Toolkit
- **RCP** - Eclipse Rich Client Platform

### Installation

Install the library using pip:

```bash
pip install robotframework-javagui
```

### Java Agent Setup

Your Java application must be started with the JavaGui agent:

```bash
# Get the agent JAR path
python -c "from JavaGui import get_agent_jar_path; print(get_agent_jar_path())"

# Start your application with the agent
java -javaagent:/path/to/javagui-agent.jar=port=5678 -jar your-app.jar
```

## Supported Modes

The library operates in one of three modes depending on your target application:

| Mode | Target Applications | Import |
|------|---------------------|--------|
| `swing` | Java Swing applications | `Library    JavaGui.Swing` |
| `swt` | Eclipse SWT applications | `Library    JavaGui.Swt` |
| `rcp` | Eclipse RCP applications | `Library    JavaGui.Rcp` |

### Swing Mode

Use for standard Java Swing applications using components like `JButton`, `JTextField`, `JTable`, etc.

```robotframework
*** Settings ***
Library    JavaGui.Swing    timeout=15

*** Test Cases ***
Login Test
    Connect To Application    MySwingApp    localhost    5678
    Input Text    name:username    testuser
    Input Text    name:password    secret
    Click    name:loginButton
    Element Should Be Visible    name:welcomePanel
    Disconnect
```

### SWT Mode

Use for Eclipse SWT applications using widgets like `Button`, `Text`, `Table`, `Tree`, etc.

```robotframework
*** Settings ***
Library    JavaGui.Swt    timeout=15

*** Test Cases ***
SWT Application Test
    Connect To Swt Application    MySwtApp    localhost    5679
    Input Text    [text='Search:'] ~ Text    search query
    Click Widget    Button[text='Search']
    Wait Until Widget Exists    Table    timeout=10
    Disconnect
```

### RCP Mode

Use for Eclipse RCP applications. Provides all SWT keywords plus workbench-specific keywords for perspectives, views, and editors.

```robotframework
*** Settings ***
Library    JavaGui.Rcp    timeout=15

*** Test Cases ***
Eclipse RCP Test
    Connect To Application    MyRcpApp    localhost    5679
    Wait For Workbench    timeout=30
    Open Perspective    org.eclipse.jdt.ui.JavaPerspective
    Show View    org.eclipse.ui.views.ProblemView
    Open Editor    /project/src/Main.java
    Save Editor
    Close All Editors
    Disconnect
```

## Quick Start

### Minimal Example

```robotframework
*** Settings ***
Library    JavaGui.Swing

*** Test Cases ***
Click Submit Button
    Connect To Application    MyApp    localhost    5678
    Click    #submitButton
    Disconnect
```

### Full Example with Setup/Teardown

```robotframework
*** Settings ***
Library    JavaGui.Swing    timeout=10    screenshot_directory=${OUTPUT_DIR}

Suite Setup       Connect To Application    MyApp    localhost    5678
Suite Teardown    Disconnect

*** Test Cases ***
Test User Registration
    [Documentation]    Test the user registration form

    # Fill in the registration form
    Input Text    name:firstName    John
    Input Text    name:lastName    Doe
    Input Text    name:email    john.doe@example.com
    Input Text    name:password    SecureP@ss123

    # Select options
    Select From Combobox    name:country    United States
    Check Checkbox    name:acceptTerms

    # Submit
    Click    name:registerButton

    # Verify success
    Wait Until Element Exists    name:successMessage    timeout=5
    Element Text Should Contain    name:successMessage    Registration successful

Test Form Validation
    [Documentation]    Test form validation errors

    # Submit empty form
    Click    name:registerButton

    # Verify validation messages
    Element Should Be Visible    name:firstNameError
    Element Text Should Be    name:firstNameError    First name is required
```

## Locator Syntax

The library supports multiple locator formats to find UI elements. See the [Locator Reference](locator-reference.md) for complete documentation.

### Quick Reference

| Format | Example | Description |
|--------|---------|-------------|
| Name prefix | `name:submitButton` | Find by component name |
| ID shorthand | `#submitButton` | Find by name (shorthand) |
| Text prefix | `text:OK` | Find by displayed text |
| CSS-style | `JButton[text='Save']` | Find by type and attributes |
| XPath | `//JButton[@name='submit']` | XPath expression |
| Index | `JButton[0]` | First button (0-indexed) |

### Common Patterns

```robotframework
*** Test Cases ***
Locator Examples
    # By name
    Click    name:submitButton
    Click    #submitButton

    # By text
    Click    text:Submit
    Click    JButton[text='Submit']

    # By type
    Click    JButton#submit
    Click    JButton:first-child

    # Hierarchical
    Click    JPanel#form > JButton
    Click    JPanel#form JButton[text='OK']
```

## Keyword Reference

### Connection Keywords

| Keyword | Description |
|---------|-------------|
| `Connect To Application` | Connect to a running Java application |
| `Disconnect` | Close the connection |
| `Is Connected` | Check if connected |
| `Get Connection Info` | Get connection details |

### Element Finding Keywords

| Keyword | Description |
|---------|-------------|
| `Find Element` | Find a single element |
| `Find Elements` | Find all matching elements |
| `Wait Until Element Exists` | Wait for element to appear |
| `Wait Until Element Does Not Exist` | Wait for element to disappear |

### Click Keywords

| Keyword | Description |
|---------|-------------|
| `Click` | Single click on element |
| `Double Click` | Double click on element |
| `Right Click` | Right/context click |
| `Click Button` | Click a button element |

### Input Keywords

| Keyword | Description |
|---------|-------------|
| `Input Text` | Enter text into a field |
| `Clear Text` | Clear text from a field |
| `Type Text` | Type text character by character |

### Selection Keywords

| Keyword | Description |
|---------|-------------|
| `Select From Combobox` | Select dropdown item |
| `Check Checkbox` | Check a checkbox |
| `Uncheck Checkbox` | Uncheck a checkbox |
| `Select Radio Button` | Select a radio button |
| `Select From List` | Select list item |
| `Select Tab` | Select tab in tabbed pane |

### Table Keywords

| Keyword | Description |
|---------|-------------|
| `Get Table Cell Value` | Get cell content |
| `Select Table Cell` | Click a cell |
| `Select Table Row` | Select a row |
| `Get Table Row Count` | Count rows |
| `Get Table Column Count` | Count columns |
| `Get Table Data` | Get all table data |

### Tree Keywords

| Keyword | Description |
|---------|-------------|
| `Expand Tree Node` | Expand a node |
| `Collapse Tree Node` | Collapse a node |
| `Select Tree Node` | Select a node |
| `Get Selected Tree Node` | Get current selection |

### Menu Keywords

| Keyword | Description |
|---------|-------------|
| `Select Menu` | Select from menu bar |
| `Select From Popup Menu` | Select from context menu |

### Verification Keywords

| Keyword | Description |
|---------|-------------|
| `Element Should Be Visible` | Assert element is visible |
| `Element Should Not Be Visible` | Assert element is hidden |
| `Element Should Be Enabled` | Assert element is enabled |
| `Element Should Be Disabled` | Assert element is disabled |
| `Element Should Exist` | Assert element exists |
| `Element Should Not Exist` | Assert element doesn't exist |
| `Element Text Should Be` | Assert exact text |
| `Element Text Should Contain` | Assert text contains |

### Wait Keywords

| Keyword | Description |
|---------|-------------|
| `Wait Until Element Is Visible` | Wait for visibility |
| `Wait Until Element Is Enabled` | Wait for enabled state |
| `Wait Until Element Contains` | Wait for text content |
| `Wait For Element` | Wait and return element |

### Debugging Keywords

| Keyword | Description |
|---------|-------------|
| `Log UI Tree` | Print component hierarchy |
| `Get UI Tree` | Get tree as string |
| `Save UI Tree` | Save tree to file |
| `Refresh UI Tree` | Refresh cached tree |
| `Capture Screenshot` | Take screenshot |

### Configuration Keywords

| Keyword | Description |
|---------|-------------|
| `Set Timeout` | Set default timeout |
| `Set Screenshot Directory` | Set screenshot path |

### RCP-Specific Keywords (RCP Mode Only)

| Keyword | Description |
|---------|-------------|
| `Open Perspective` | Open perspective by ID |
| `Get Active Perspective` | Get current perspective |
| `Reset Perspective` | Reset perspective layout |
| `Show View` | Show view by ID |
| `Close View` | Close a view |
| `Activate View` | Bring view to front |
| `Open Editor` | Open file in editor |
| `Close Editor` | Close editor |
| `Close All Editors` | Close all editors |
| `Save Editor` | Save current editor |
| `Save All Editors` | Save all editors |
| `Execute Command` | Execute Eclipse command |
| `Wait For Workbench` | Wait for workbench ready |

## Configuration

### Library Initialization Options

```robotframework
*** Settings ***
# Swing with custom options
Library    JavaGui.Swing
...    timeout=15
...    poll_interval=0.5
...    screenshot_directory=${OUTPUT_DIR}/screenshots

# SWT with timeout
Library    JavaGui.Swt    timeout=30

# RCP with timeout
Library    JavaGui.Rcp    timeout=20
```

| Option | Default | Description |
|--------|---------|-------------|
| `timeout` | 10.0 | Default wait timeout in seconds |
| `poll_interval` | 0.5 | Polling interval for waits |
| `screenshot_directory` | `.` | Directory for screenshots |

### Runtime Configuration

```robotframework
*** Test Cases ***
Configure Timeouts
    # Increase timeout for slow operations
    Set Timeout    30

    # Set screenshot directory
    Set Screenshot Directory    ${OUTPUT_DIR}/screenshots
```

## Troubleshooting

### Connection Issues

**Problem**: `ConnectionRefusedError` when connecting

**Solutions**:
1. Verify the Java application is running
2. Check the agent is loaded: `java -javaagent:javagui-agent.jar=port=5678 ...`
3. Verify the port matches your connection
4. Check firewall settings

### Element Not Found

**Problem**: `ElementNotFoundError` for elements that should exist

**Solutions**:
1. Use `Log UI Tree` to see the actual component hierarchy
2. Wait for the element: `Wait Until Element Exists    name:myElement`
3. Check if the element is in a different window/dialog
4. Verify the locator syntax is correct

```robotframework
*** Test Cases ***
Debug Element Not Found
    # First, examine the UI tree
    Log UI Tree

    # Try waiting for the element
    Wait Until Element Exists    name:myButton    timeout=10

    # Then interact
    Click    name:myButton
```

### Timing Issues

**Problem**: Intermittent test failures due to timing

**Solutions**:
1. Use wait keywords before interactions
2. Increase default timeout
3. Use explicit waits for specific conditions

```robotframework
*** Test Cases ***
Handle Timing Issues
    Click    name:loadDataButton

    # Wait for loading to complete
    Wait Until Element Does Not Exist    name:loadingSpinner    timeout=30

    # Now interact with loaded data
    Click    name:dataTable
```

### Screenshot on Failure

```robotframework
*** Settings ***
Library    JavaGui.Swing    screenshot_directory=${OUTPUT_DIR}

*** Keywords ***
Safe Click
    [Arguments]    ${locator}
    ${status}=    Run Keyword And Return Status    Click    ${locator}
    Run Keyword If    not ${status}    Capture Screenshot
    Should Be True    ${status}    Click failed for ${locator}
```

## Next Steps

- [Locator Reference](locator-reference.md) - Complete locator syntax documentation
- [Migration Guide](migration-guide.md) - Migrating from legacy libraries
- [Developer Guide](../dev-guide/architecture.md) - Contributing to the library
