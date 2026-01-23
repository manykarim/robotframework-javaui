# Component Tree Guide

**Version:** 0.2.0
**Last Updated:** 2026-01-22
**Status:** Complete

---

## Table of Contents

1. [Overview](#overview)
2. [Quick Start](#quick-start)
3. [Keywords Reference](#keywords-reference)
4. [Output Formats](#output-formats)
5. [Advanced Features](#advanced-features)
6. [Performance Optimization](#performance-optimization)
7. [Best Practices](#best-practices)
8. [Common Use Cases](#common-use-cases)

---

## Overview

The Component Tree feature provides comprehensive UI inspection capabilities for debugging and test automation. It allows you to:

- Retrieve the complete component hierarchy
- Extract subtrees from specific components
- Control traversal depth for performance
- Export in multiple formats (text, JSON, YAML)
- Filter by component types and states

### Supported Technologies

| Technology | Status | Features |
|------------|--------|----------|
| **Swing** | Full Support | All features available |
| **SWT** | Partial Support | Basic tree retrieval |
| **RCP** | Limited Support | Full tree only |

---

## Quick Start

### Basic Tree Retrieval

```robotframework
*** Settings ***
Library    JavaGui.Swing

*** Test Cases ***
Get Full Component Tree
    Connect To Application    MyApp
    ${tree}=    Get Component Tree
    Log    ${tree}
    Disconnect
```

### Get Tree in JSON Format

```robotframework
*** Test Cases ***
Get Tree As JSON
    Connect To Application    MyApp
    ${json_tree}=    Get Component Tree    format=json
    Log    ${json_tree}
    # Parse JSON for programmatic access
    ${parsed}=    Evaluate    json.loads('''${json_tree}''')    modules=json
    Disconnect
```

### Limit Tree Depth

```robotframework
*** Test Cases ***
Get Shallow Tree
    Connect To Application    MyApp
    # Get only 3 levels deep
    ${tree}=    Get Component Tree    max_depth=3
    Log    ${tree}
    Disconnect
```

---

## Keywords Reference

### Get Component Tree

Retrieves the complete component tree starting from the root window.

**Signature:**
```robotframework
Get Component Tree
    [Arguments]    locator=${None}    format=text    max_depth=${None}
    [Returns]      String representation of the component tree
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `locator` | str | None | Optional locator to start from a specific component. If not provided, starts from root window(s). |
| `format` | str | text | Output format: `text`, `json`, or `yaml` |
| `max_depth` | int | None | Maximum depth to traverse. None = unlimited |

**Return Value:**

Returns a string containing the component tree in the specified format.

**Examples:**

```robotframework
# Get full tree in text format
${tree}=    Get Component Tree

# Get tree in JSON format
${json}=    Get Component Tree    format=json

# Limit depth to 5 levels
${tree}=    Get Component Tree    max_depth=5

# Get subtree from specific component
${subtree}=    Get Component Tree    locator=JPanel[name='mainPanel']    max_depth=3

# Get tree in YAML format
${yaml}=    Get Component Tree    format=yaml
```

**Notes:**
- Text format is human-readable and best for logging
- JSON format is best for programmatic parsing
- YAML format is best for configuration and readability
- Smaller max_depth values improve performance on large UIs

---

### Get Component Subtree

Retrieves a subtree starting from a specific component.

**Signature:**
```robotframework
Get Component Subtree
    [Arguments]    locator    format=text    max_depth=${None}
    [Returns]      String representation of the component subtree
```

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `locator` | str | Yes | Component locator to start from |
| `format` | str | No | Output format: `text`, `json`, or `yaml` (default: text) |
| `max_depth` | int | No | Maximum depth from starting component (default: unlimited) |

**Return Value:**

Returns a string containing the component subtree in the specified format.

**Examples:**

```robotframework
# Get subtree of main panel
${subtree}=    Get Component Subtree    JPanel[name='mainPanel']

# Get subtree in JSON, 2 levels deep
${json}=    Get Component Subtree    locator=//JDialog    format=json    max_depth=2

# Get subtree of table container
${tree}=    Get Component Subtree    locator=JScrollPane > JTable    format=text
```

**Use Cases:**
- Focus on specific UI sections
- Reduce output size for complex UIs
- Analyze dialog or panel structure
- Performance optimization

---

### Log Component Tree

Logs the component tree to the Robot Framework log.

**Signature:**
```robotframework
Log Component Tree
    [Arguments]    locator=${None}    format=text    level=INFO
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `locator` | str | None | Optional locator to start from |
| `format` | str | text | Output format |
| `level` | str | INFO | Log level: DEBUG, INFO, WARN, ERROR |

**Examples:**

```robotframework
# Log full tree
Log Component Tree

# Log as JSON at DEBUG level
Log Component Tree    format=json    level=DEBUG

# Log specific panel tree
Log Component Tree    locator=JPanel[name='form']    format=text
```

**Notes:**
- Use `level=DEBUG` for detailed inspection without cluttering main log
- Use `level=INFO` for test documentation
- JSON format in logs can be copy-pasted for analysis

---

### Refresh Component Tree

Refreshes the cached component tree.

**Signature:**
```robotframework
Refresh Component Tree
    [Arguments]    None
    [Returns]      None
```

**Description:**

Forces a refresh of the internal component tree cache. This is necessary after:
- Dynamic UI changes (dialogs opening/closing)
- Component creation/destruction
- Layout changes

**Examples:**

```robotframework
# Open dialog
Click    JButton[text='Open Settings']
Wait Until Element Exists    JDialog[title='Settings']

# Refresh tree to see new dialog
Refresh Component Tree

# Now get updated tree
${tree}=    Get Component Tree
```

**Notes:**
- Called automatically in many scenarios
- Manual refresh ensures latest state
- Minimal performance impact

---

## Output Formats

### Text Format

Human-readable tree structure with indentation.

**Example Output:**
```
JFrame[name='MainWindow', title='My Application']
  JMenuBar
    JMenu[text='File']
      JMenuItem[text='Open']
      JMenuItem[text='Save']
      JSeparator
      JMenuItem[text='Exit']
    JMenu[text='Edit']
      JMenuItem[text='Copy']
      JMenuItem[text='Paste']
  JPanel[name='mainPanel']
    JPanel[name='toolbar']
      JButton[text='New']
      JButton[text='Open']
      JButton[text='Save']
    JScrollPane
      JTable[name='dataTable']
        JTableHeader
          Column: Name
          Column: Age
          Column: Email
```

**Best For:**
- Debugging in Robot Framework logs
- Visual inspection
- Documentation screenshots
- Quick understanding of UI structure

---

### JSON Format

Structured data with full component details.

**Example Output:**
```json
{
  "roots": [
    {
      "id": 1,
      "class": "javax.swing.JFrame",
      "simpleClass": "JFrame",
      "name": "MainWindow",
      "text": "My Application",
      "x": 100,
      "y": 100,
      "width": 800,
      "height": 600,
      "visible": true,
      "enabled": true,
      "showing": true,
      "children": [
        {
          "id": 2,
          "class": "javax.swing.JMenuBar",
          "simpleClass": "JMenuBar",
          "children": [...]
        }
      ]
    }
  ],
  "timestamp": 1737532800000
}
```

**Properties Included:**
- `id` - Unique component identifier
- `class` - Full class name
- `simpleClass` - Simple class name
- `name` - Component name (if set)
- `text` - Text content (for labels, buttons, etc.)
- `x, y, width, height` - Component bounds
- `visible` - Visibility state
- `enabled` - Enabled state
- `showing` - Showing state (visible + all parents visible)
- `children` - Child components array
- `childCount` - Number of children

**Best For:**
- Programmatic parsing
- Automated analysis
- Integration with other tools
- Saving to files for later analysis

**Example Usage:**
```robotframework
${json}=    Get Component Tree    format=json
${data}=    Evaluate    json.loads('''${json}''')    modules=json
${root_count}=    Get Length    ${data['roots']}
Log    Found ${root_count} root windows
```

---

### YAML Format

Configuration-friendly format with good readability.

**Example Output:**
```yaml
roots:
  - id: 1
    class: javax.swing.JFrame
    simpleClass: JFrame
    name: MainWindow
    text: My Application
    x: 100
    y: 100
    width: 800
    height: 600
    visible: true
    enabled: true
    showing: true
    children:
      - id: 2
        class: javax.swing.JMenuBar
        simpleClass: JMenuBar
        children: []
timestamp: 1737532800000
```

**Best For:**
- Configuration files
- Human review (more readable than JSON)
- Documentation
- Version control comparisons

---

## Advanced Features

### Depth Control

Control how deep the tree traversal goes.

**Why Use Depth Control:**
- **Performance**: Large UIs with 1000+ components can be slow
- **Focus**: Get overview without overwhelming detail
- **Progressive Disclosure**: Start shallow, drill deeper as needed

**Examples:**

```robotframework
*** Test Cases ***
Progressive Tree Inspection
    # First, get shallow overview (2 levels)
    ${overview}=    Get Component Tree    max_depth=2
    Log    ${overview}

    # Identify interesting panel
    # Now get detailed view of that panel
    ${details}=    Get Component Subtree    JPanel[name='dataPanel']    max_depth=10
    Log    ${details}
```

**Performance Comparison:**

| Depth | Components | Time | Use Case |
|-------|-----------|------|----------|
| 2 | ~10-20 | <100ms | Quick overview |
| 5 | ~100-200 | ~500ms | Panel structure |
| 10 | ~500-1000 | ~2s | Detailed inspection |
| Unlimited | 1000+ | 5-10s | Full analysis |

---

### Filtering by Component Type

**Note:** This feature is planned for future release. Current workaround:

```robotframework
*** Test Cases ***
Find Buttons In Tree
    ${json}=    Get Component Tree    format=json
    ${data}=    Evaluate    json.loads('''${json}''')    modules=json

    # Custom Python keyword to filter
    ${buttons}=    Filter Components By Type    ${data}    JButton
    Log    Found ${buttons.__len__()} buttons
```

---

### Filtering by Visibility/State

**Note:** This feature is planned for future release. Current workaround:

```robotframework
*** Test Cases ***
Find Visible Components
    ${json}=    Get Component Tree    format=json
    ${data}=    Evaluate    json.loads('''${json}''')    modules=json

    # Filter visible components
    ${visible}=    Filter Components By State    ${data}    visible=true
```

---

## Performance Optimization

### Best Practices

1. **Use Depth Limiting**
   ```robotframework
   # Bad: Get everything (slow on large UIs)
   ${tree}=    Get Component Tree

   # Good: Limit depth
   ${tree}=    Get Component Tree    max_depth=5
   ```

2. **Use Subtrees for Specific Sections**
   ```robotframework
   # Bad: Get full tree then search
   ${tree}=    Get Component Tree
   # ... manually search tree ...

   # Good: Get just the section you need
   ${form_tree}=    Get Component Subtree    JPanel[name='loginForm']
   ```

3. **Cache Results**
   ```robotframework
   # Get once, use multiple times
   ${tree}=    Get Component Tree    format=json
   ${data}=    Evaluate    json.loads('''${json}''')    modules=json

   # Now analyze data multiple times without re-fetching
   ${button_count}=    Count Components By Type    ${data}    JButton
   ${panel_count}=    Count Components By Type    ${data}    JPanel
   ```

4. **Refresh Only When Needed**
   ```robotframework
   # Don't refresh unnecessarily
   ${tree1}=    Get Component Tree
   # ... no UI changes ...
   ${tree2}=    Get Component Tree    # Uses cache

   # Refresh after UI changes
   Click    JButton[text='Open Dialog']
   Refresh Component Tree
   ${tree3}=    Get Component Tree    # Fresh data
   ```

---

## Best Practices

### When to Use Component Tree

✅ **Use for:**
- Debugging element locators
- Documenting UI structure
- Understanding unfamiliar applications
- Verifying UI composition
- Finding component names/properties
- Test data generation

❌ **Don't use for:**
- Regular element finding (use `Find Element` instead)
- Simple property checks (use `Get Property` instead)
- Performance-critical operations
- Within loops or repeated operations

---

### Recommended Workflow

```robotframework
*** Test Cases ***
Debug Failed Locator
    # Locator isn't working
    Run Keyword And Expect Error    ElementNotFoundError
    ...    Click    JButton[name='submitBtn']

    # Get tree to investigate
    Log Component Tree    format=text    level=INFO

    # From log, discover actual name is 'submit_button'
    Click    JButton[name='submit_button']
```

---

## Common Use Cases

### Use Case 1: Finding Component Names

**Problem:** You need to click a button but don't know its name.

**Solution:**
```robotframework
*** Test Cases ***
Find Button Name
    Connect To Application    MyApp

    # Get tree with buttons visible
    ${tree}=    Get Component Tree    format=text    max_depth=5
    Log    ${tree}

    # Review log, find button name: "saveButton"
    # Now use it
    Click    JButton[name='saveButton']
```

---

### Use Case 2: Verifying Dialog Structure

**Problem:** Verify a settings dialog has all required fields.

**Solution:**
```robotframework
*** Test Cases ***
Verify Settings Dialog
    Click    JButton[text='Settings']
    Wait Until Element Exists    JDialog[title='Settings']

    # Get dialog tree
    ${dialog_tree}=    Get Component Subtree    JDialog[title='Settings']    format=json
    ${data}=    Evaluate    json.loads('''${dialog_tree}''')    modules=json

    # Verify structure
    ${text_fields}=    Count Components By Type    ${data}    JTextField
    Should Be Equal As Integers    ${text_fields}    5    Expected 5 input fields
```

---

### Use Case 3: Performance Testing Large UIs

**Problem:** Application with 2000+ components is slow to inspect.

**Solution:**
```robotframework
*** Test Cases ***
Inspect Large UI Efficiently
    # Don't get full tree (too slow)
    # Get shallow overview first
    ${overview}=    Get Component Tree    max_depth=3
    Log    ${overview}

    # Identify sections
    # Get detailed view of specific section
    ${main_section}=    Get Component Subtree
    ...    JPanel[name='mainContent']
    ...    max_depth=10
    Log    ${main_section}
```

---

### Use Case 4: Automated Documentation

**Problem:** Generate UI documentation automatically.

**Solution:**
```robotframework
*** Test Cases ***
Generate UI Documentation
    Connect To Application    MyApp

    # Get tree in JSON
    ${tree}=    Get Component Tree    format=json

    # Save to file
    Create File    ${OUTPUT_DIR}/ui_structure.json    ${tree}

    # Also save human-readable version
    ${text_tree}=    Get Component Tree    format=text
    Create File    ${OUTPUT_DIR}/ui_structure.txt    ${text_tree}
```

---

### Use Case 5: Comparing UI States

**Problem:** Verify UI changes after an action.

**Solution:**
```robotframework
*** Test Cases ***
Verify UI State Change
    # Get initial state
    ${before}=    Get Component Tree    locator=JPanel[name='form']    format=json

    # Perform action
    Click    JButton[text='Add Field']

    # Get new state
    Refresh Component Tree
    ${after}=    Get Component Tree    locator=JPanel[name='form']    format=json

    # Compare
    ${before_data}=    Evaluate    json.loads('''${before}''')    modules=json
    ${after_data}=    Evaluate    json.loads('''${after}''')    modules=json

    ${before_count}=    Get Length    ${before_data['children']}
    ${after_count}=    Get Length    ${after_data['children']}

    Should Be True    ${after_count} > ${before_count}    Field was not added
```

---

## Platform-Specific Notes

### Swing

Full support for all features:
- ✅ All output formats
- ✅ Depth control
- ✅ Subtree retrieval
- ✅ All component types
- ✅ Comprehensive property extraction

**Special Considerations:**
- JTable, JTree, and JList have child components for items
- JMenuBar children may not be visible until menu is activated
- Modal dialogs block UI inspection until closed

---

### SWT

Partial support:
- ✅ Basic tree retrieval
- ✅ Text and JSON formats
- ⚠️ Limited depth control
- ⚠️ Some component types not fully supported

**Limitations:**
- Reflection-based access may miss some properties
- Performance slower than Swing
- Not all SWT widgets supported

---

### RCP (Eclipse)

Limited support:
- ✅ Full tree retrieval
- ⚠️ Limited subtree support
- ⚠️ Workbench-specific structures not fully exposed

**Special Considerations:**
- Views, Editors, and Perspectives have special structure
- Part stacks may require special handling
- Some Eclipse-specific widgets not fully supported

---

## Troubleshooting

### Problem: Tree is Too Large

**Symptoms:**
- Operation times out
- Application becomes unresponsive
- Out of memory errors

**Solutions:**
```robotframework
# Use depth limiting
${tree}=    Get Component Tree    max_depth=5

# Get subtree instead of full tree
${subtree}=    Get Component Subtree    locator=JPanel[name='target']

# Use text format instead of JSON (smaller output)
${tree}=    Get Component Tree    format=text
```

---

### Problem: Tree is Outdated

**Symptoms:**
- Components not appearing in tree
- Tree doesn't reflect recent UI changes
- Element locators work but tree shows old structure

**Solution:**
```robotframework
# Refresh before getting tree
Refresh Component Tree
${tree}=    Get Component Tree
```

---

### Problem: Cannot Parse JSON Output

**Symptoms:**
- JSON parsing errors
- Invalid JSON format
- Encoding issues

**Solution:**
```robotframework
# Use triple quotes for JSON strings
${json}=    Get Component Tree    format=json
${data}=    Evaluate    json.loads('''${json}''')    modules=json

# Or save to file first
${json}=    Get Component Tree    format=json
Create File    ${TEMPDIR}/tree.json    ${json}
${data}=    Load JSON From File    ${TEMPDIR}/tree.json
```

---

### Problem: Subtree Locator Not Found

**Symptoms:**
- ElementNotFoundError when getting subtree
- Locator works for Click but not for Get Component Subtree

**Solution:**
```robotframework
# Make sure element exists first
Wait Until Element Exists    JPanel[name='target']    timeout=10

# Then get subtree
${subtree}=    Get Component Subtree    JPanel[name='target']

# Or use fallback to full tree
${tree}=    Run Keyword And Return Status
...    Get Component Subtree    JPanel[name='target']
Run Keyword If    ${tree} == ${False}
...    Get Component Tree
```

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.2.0 | 2026-01-22 | Enhanced component tree features, multiple formats, depth control |
| 0.1.0 | 2025-12-01 | Initial tree retrieval support |

---

## Related Documentation

- [Locator Syntax Guide](LOCATOR_SYNTAX.md)
- [Assertion Engine Guide](ASSERTION_GUIDE.md)
- [API Reference](../api-reference/)
- [Troubleshooting Guide](TROUBLESHOOTING.md)

---

## Feedback

For issues, questions, or feature requests related to component tree functionality:
- GitHub Issues: https://github.com/manykarim/robotframework-javaui/issues
- Documentation feedback: Create an issue with "docs:" prefix

