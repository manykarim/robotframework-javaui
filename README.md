# Robot Framework JavaGUI Library

A high-performance Robot Framework library for automating Java GUI applications including **Swing**, **SWT**, and **Eclipse RCP**. Built with Rust and PyO3 for optimal performance, this library provides comprehensive support for testing Java desktop applications.

## Features

- **Multi-Toolkit Support**: Automate Swing, SWT, and Eclipse RCP applications
- **High Performance**: Core library written in Rust with PyO3 bindings for Python
- **CSS-like Selectors**: Intuitive element locators similar to web testing
- **XPath Support**: Full XPath-style locator syntax for complex queries
- **Comprehensive Component Support**: Buttons, text fields, tables, trees, lists, menus, and more
- **Eclipse RCP Support**: Perspectives, views, editors, commands, and workbench operations
- **Bundled Java Agent**: Agent JAR included in the package - no separate installation needed
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Libraries](#libraries)
- [Locator Syntax](#locator-syntax)
- [Keywords Reference](#keywords-reference)
- [Examples](#examples)
- [Architecture](#architecture)
- [Development](#development)
- [License](#license)

## Installation

### From PyPI

```bash
pip install robotframework-javagui
```

### Prerequisites

- Python 3.8 or higher
- Java 11 or higher (for running Java applications)

### Install from Source

```bash
# Clone the repository
git clone https://github.com/robotframework/robotframework-javagui.git
cd robotframework-javagui

# Install with pip
pip install -e .

# Or build with invoke
pip install invoke
invoke build
```

## Quick Start

### 1. Start Your Java Application with the Agent

The Java agent is bundled with the package. You can get its path programmatically:

```python
from JavaGui import get_agent_jar_path

agent_jar = get_agent_jar_path()
# Use: java -javaagent:{agent_jar}=port=5678 -jar your-app.jar
```

Or start your application manually:

```bash
# Get the agent path
python -c "from JavaGui import get_agent_jar_path; print(get_agent_jar_path())"

# Start your application
java -javaagent:/path/to/javagui-agent.jar=port=5678 -jar your-app.jar
```

### 2. Create a Robot Framework Test

#### For Swing Applications

```robotframework
*** Settings ***
Library    JavaGui.Swing

*** Test Cases ***
Example Login Test
    Connect To Application    main_class=com.example.MyApp    host=localhost    port=5678
    Input Text    [name='username']    admin
    Input Text    [name='password']    secret
    Click    JButton[text='Login']
    Element Should Exist    JLabel[text='Welcome']
    [Teardown]    Disconnect
```

#### For SWT Applications

```robotframework
*** Settings ***
Library    JavaGui.Swt

*** Test Cases ***
Example SWT Test
    Connect To Swt Application    myapp    host=localhost    port=5678
    Click Widget    text:OK
    Input Text    class:Text    Hello World
    [Teardown]    Disconnect
```

#### For Eclipse RCP Applications

```robotframework
*** Settings ***
Library    JavaGui.Swt    WITH NAME    SWT
Library    JavaGui.Rcp    WITH NAME    RCP

*** Test Cases ***
Example RCP Test
    SWT.Connect To Swt Application    eclipse    host=localhost    port=5678
    RCP.Show View    org.eclipse.ui.views.ProblemView
    RCP.Select Main Menu    File|New|Project...
    [Teardown]    SWT.Disconnect
```

### 3. Run the Test

```bash
robot my_test.robot
```

## Libraries

JavaGUI provides three specialized libraries:

| Library | Import | Use Case |
|---------|--------|----------|
| **Swing** | `Library JavaGui.Swing` | Java Swing applications |
| **Swt** | `Library JavaGui.Swt` | Eclipse SWT applications |
| **Rcp** | `Library JavaGui.Rcp` | Eclipse RCP applications (DBeaver, Eclipse IDE, etc.) |

### Python Usage

```python
from JavaGui import Swing, Swt, Rcp

# Get bundled agent JAR path
from JavaGui import get_agent_jar_path, AGENT_JAR_PATH
```

### Legacy Compatibility

For backwards compatibility, the following aliases are available:

```python
from JavaGui import SwingLibrary  # Alias for Swing
from JavaGui import SwtLibrary    # Alias for Swt
from JavaGui import RcpLibrary    # Alias for Rcp
```

## Locator Syntax

The library supports multiple locator strategies for finding UI elements.

### CSS-like Selectors (Swing)

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

### SWT Widget Selectors

| Selector | Description | Example |
|----------|-------------|---------|
| `class:ClassName` | Match by SWT class | `class:Button` |
| `text:Text` | Match by text content | `text:OK` |
| `#id` | Match by widget data/name | `#submitBtn` |
| `[attr=value]` | Match by property | `[text='Save']` |

### Eclipse RCP Selectors

| Selector | Description | Example |
|----------|-------------|---------|
| `view:ViewId` | Eclipse view | `view:org.eclipse.ui.views.ProblemView` |
| `editor:EditorId` | Eclipse editor | `editor:*` |
| `perspective:PerspId` | Eclipse perspective | `perspective:org.eclipse.jdt.ui.JavaPerspective` |

### XPath-style Selectors

| Selector | Description | Example |
|----------|-------------|---------|
| `//Type` | Any descendant | `//JButton` |
| `//Type[@attr='value']` | With attribute | `//JButton[@name='ok']` |
| `//Type[n]` | By index (1-based) | `//JButton[1]` |

## Keywords Reference

### Connection Keywords

| Keyword | Library | Description |
|---------|---------|-------------|
| `Connect To Application` | Swing | Connect to a Swing application |
| `Connect To Swt Application` | Swt | Connect to an SWT application |
| `Disconnect` | All | Disconnect from the application |
| `Is Connected` | All | Returns connection status |

### Element Finding

| Keyword | Description |
|---------|-------------|
| `Find Element` | Find single element |
| `Find Elements` | Find all matching elements |
| `Find Widget` | Find SWT widget (Swt library) |
| `Find Widgets` | Find all matching SWT widgets |
| `Element Should Exist` | Assert element exists |
| `Wait Until Element Exists` | Wait for element |
| `Wait Until Widget Exists` | Wait for SWT widget |

### Mouse Actions

| Keyword | Description |
|---------|-------------|
| `Click` | Single click |
| `Click Widget` | Click SWT widget |
| `Double Click` | Double click |
| `Right Click` | Context menu click |
| `Click Button` | Click a button |

### Text Input

| Keyword | Description |
|---------|-------------|
| `Input Text` | Enter text |
| `Clear Text` | Clear text field |
| `Get Element Text` | Get element's text content |

### Table Operations

| Keyword | Description |
|---------|-------------|
| `Get Table Row Count` | Get number of rows |
| `Get Table Cell Value` | Get cell value |
| `Select Table Row` | Select a row |
| `Select Table Cell` | Select a cell |

### Tree Operations

| Keyword | Description |
|---------|-------------|
| `Expand Tree Node` / `Expand Tree Item` | Expand a tree node |
| `Collapse Tree Node` / `Collapse Tree Item` | Collapse a tree node |
| `Select Tree Node` / `Select Tree Item` | Select a tree node |

### Eclipse RCP Keywords (Rcp Library)

| Keyword | Description |
|---------|-------------|
| `Show View` | Show an Eclipse view |
| `Close View` | Close an Eclipse view |
| `Activate View` | Bring view to front |
| `Get Open Views` | List open views |
| `Open Editor` | Open an editor |
| `Close Editor` | Close an editor |
| `Close All Editors` | Close all editors |
| `Get Active Editor` | Get current editor |
| `Switch Perspective` | Change perspective |
| `Get Active Perspective` | Get current perspective |
| `Get Available Perspectives` | List all perspectives |
| `Reset Perspective` | Reset to default layout |
| `Select Main Menu` | Select menu item |
| `Execute Command` | Run Eclipse command |

### SWT Shell Keywords (Swt Library)

| Keyword | Description |
|---------|-------------|
| `Get Shells` | Get all open shells |
| `Activate Shell` | Bring shell to front |
| `Close Shell` | Close a shell |

### Verification Keywords

| Keyword | Description |
|---------|-------------|
| `Element Should Be Visible` | Assert element is visible |
| `Widget Should Be Visible` | Assert SWT widget is visible |
| `Element Should Be Enabled` | Assert element is enabled |
| `Widget Should Be Enabled` | Assert SWT widget is enabled |
| `Element Text Should Be` | Assert exact text match |

## Examples

### Swing Application Test

```robotframework
*** Settings ***
Documentation     Login functionality test suite
Library           JavaGui.Swing
Library           Process

Suite Setup       Start Application
Suite Teardown    Stop Application

*** Variables ***
${APP_JAR}        path/to/myapp.jar
${PORT}           5678

*** Keywords ***
Start Application
    ${agent}=    Evaluate    __import__('JavaGui').get_agent_jar_path()
    ${cmd}=    Set Variable    java -javaagent:${agent}=port=${PORT} -jar ${APP_JAR}
    Start Process    ${cmd}    shell=True    alias=app
    Sleep    3s
    Connect To Application    main_class=com.example.MyApp    port=${PORT}

Stop Application
    Disconnect
    Terminate Process    app    kill=True

*** Test Cases ***
Valid Login Should Succeed
    Clear Text    JTextField[name='username']
    Input Text    JTextField[name='username']    admin
    Input Text    JPasswordField[name='password']    password123
    Click    JButton[text='Login']
    Wait Until Element Exists    JLabel[text*='Welcome']    timeout=5
```

### SWT Application Test

```robotframework
*** Settings ***
Documentation     SWT Application Tests
Library           JavaGui.Swt

*** Test Cases ***
Verify Button Controls
    Connect To Swt Application    myapp    port=5678
    Widget Should Be Visible    class:Button
    Click Widget    text:OK
    Widget Should Be Enabled    text:Next
    [Teardown]    Disconnect

Work With Text Fields
    Connect To Swt Application    myapp    port=5678
    Input Text    class:Text    Hello World
    ${text}=    Get Element Text    class:Text
    Should Be Equal    ${text}    Hello World
    Clear Text    class:Text
    [Teardown]    Disconnect
```

### Eclipse RCP Application Test (DBeaver Example)

```robotframework
*** Settings ***
Documentation     DBeaver RCP Application Tests
Library           JavaGui.Swt    WITH NAME    SWT
Library           JavaGui.Rcp    WITH NAME    RCP

Suite Setup       Connect To DBeaver
Suite Teardown    SWT.Disconnect

*** Keywords ***
Connect To DBeaver
    SWT.Connect To Swt Application    dbeaver    port=18081    timeout=30

*** Test Cases ***
Verify Database Navigator View
    ${views}=    RCP.Get Open Views
    Should Not Be Empty    ${views}
    RCP.Activate View    org.jkiss.dbeaver.ui.navigator.database

Work With Perspectives
    ${perspective}=    RCP.Get Active Perspective
    Log    Current perspective: ${perspective}
    ${available}=    RCP.Get Available Perspectives
    Should Not Be Empty    ${available}

Navigate Main Menu
    RCP.Select Main Menu    File|New|Database Connection
    SWT.Wait Until Widget Exists    text:Create new connection
    SWT.Close Shell    text:Create new connection
```

### UI Tree Debugging

```robotframework
*** Test Cases ***
Debug UI Structure
    Log Ui Tree
    ${tree}=    Get Ui Tree    format=text
    Should Contain    ${tree}    JButton

    ${buttons}=    Find Elements    JButton
    ${count}=    Get Length    ${buttons}
    Log    Found ${count} buttons
```

## Architecture

```
robotframework-javagui/
├── python/                 # Python package
│   └── JavaGui/            # Robot Framework library
│       ├── __init__.py     # Swing, Swt, Rcp classes
│       └── jars/           # Bundled Java agent
│           └── javagui-agent.jar
├── src/                    # Rust source code
│   ├── lib.rs              # PyO3 bindings
│   ├── locator/            # Locator parsing (pest grammar)
│   ├── protocol/           # RPC client
│   └── python/             # Python bindings
├── agent/                  # Java agent source
│   └── src/main/java/      # Agent implementation
├── tests/                  # Test suites
│   └── robot/              # Robot Framework tests
└── tasks.py                # Build and release tasks
```

### How It Works

1. **Java Agent**: Attaches to the JVM and provides RPC endpoints for UI inspection and control
2. **Rust Core**: High-performance element matching, locator parsing, and RPC communication
3. **Python Bindings**: PyO3-based interface exposing Robot Framework keywords
4. **Robot Framework**: Test execution and reporting

### Supported GUI Toolkits

| Toolkit | Library | Description |
|---------|---------|-------------|
| **Swing** | `JavaGui.Swing` | Standard Java desktop toolkit |
| **SWT** | `JavaGui.Swt` | Eclipse Standard Widget Toolkit |
| **RCP** | `JavaGui.Rcp` | Eclipse Rich Client Platform |

## Development

### Building from Source

```bash
# Install invoke for task automation
pip install invoke

# Full build (Java agent + Rust + wheel)
invoke build

# Development build (no wheel)
invoke build-dev

# Verify installation
invoke verify

# Run tests (dry-run)
invoke test-dryrun
```

### Available Tasks

```bash
invoke --list              # List all tasks
invoke build               # Full build
invoke build-java          # Build Java agent only
invoke build-rust          # Build Rust extension only
invoke build-wheel         # Build Python wheel
invoke release-test        # Release to Test PyPI
invoke release-prod        # Release to Production PyPI
invoke release-check       # Check release readiness
invoke verify              # Verify installation
invoke test-dryrun         # Run tests in dry-run mode
invoke clean               # Clean build artifacts
invoke version             # Show/set version
```

### Running Tests

```bash
# Run all Robot Framework tests (dry-run)
invoke test-dryrun

# Run specific test suite
robot tests/robot/dbeaver/

# Run with specific log level
robot --loglevel DEBUG tests/robot/
```

## Configuration

### Agent Configuration

The Java agent accepts these JVM arguments:

```bash
java -javaagent:javagui-agent.jar=port=5678,debug=true -jar app.jar
```

| Option | Default | Description |
|--------|---------|-------------|
| `port` | 5678 | RPC server port |
| `debug` | false | Enable debug logging |

### Library Configuration

```robotframework
*** Settings ***
Library    JavaGui.Swing    timeout=30    screenshot_directory=screenshots
Library    JavaGui.Swt      timeout=30
Library    JavaGui.Rcp      timeout=30
```

| Option | Default | Description |
|--------|---------|-------------|
| `timeout` | 10 | Default wait timeout (seconds) |
| `poll_interval` | 0.5 | Polling interval for waits |
| `screenshot_directory` | . | Screenshot output directory |

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

### SWT Widget Not Found

```
ElementNotFoundError: Widget not found
```

- Use `Get Shells` to list available windows
- Try different locator strategies (class:, text:, #id)
- Ensure the widget is in the active shell

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Apache License 2.0. See [LICENSE](LICENSE) for details.

## Acknowledgments

- [Robot Framework](https://robotframework.org/) - Test automation framework
- [PyO3](https://pyo3.rs/) - Rust bindings for Python
- [pest](https://pest.rs/) - Parser library for Rust
- [Eclipse SWT](https://www.eclipse.org/swt/) - Standard Widget Toolkit
