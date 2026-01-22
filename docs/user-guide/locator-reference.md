# Locator Syntax Reference

This document provides complete documentation for the locator syntax supported by the JavaGui library.

## Table of Contents

- [Overview](#overview)
- [Prefix Format](#prefix-format)
- [ID Shorthand](#id-shorthand)
- [CSS-Style Format](#css-style-format)
- [XPath Format](#xpath-format)
- [Pseudo-Classes](#pseudo-classes)
- [Combinators](#combinators)
- [Index Selectors](#index-selectors)
- [Mode-Specific Considerations](#mode-specific-considerations)
- [Best Practices](#best-practices)

## Overview

The JavaGui library supports multiple locator formats to find UI elements:

| Format | Example | Best For |
|--------|---------|----------|
| Prefix | `name:submitButton` | Simple lookups by attribute |
| ID Shorthand | `#submitButton` | Quick name-based lookups |
| CSS-Style | `JButton[text='OK']` | Complex selectors |
| XPath | `//JButton[@name='ok']` | Hierarchical searches |

All formats work across Swing, SWT, and RCP modes with appropriate type mappings.

## Prefix Format

The prefix format uses a `prefix:value` syntax for simple attribute-based lookups.

### Supported Prefixes

| Prefix | Description | Example |
|--------|-------------|---------|
| `name:` | Component name property | `name:submitButton` |
| `text:` | Displayed text content | `text:Submit` |
| `id:` | Component identifier | `id:form-submit` |
| `class:` | Java class name | `class:javax.swing.JButton` |
| `type:` | Component type (simple name) | `type:JButton` |
| `tooltip:` | Tooltip text | `tooltip:Click to submit` |
| `label:` | Associated label text | `label:Username` |
| `index:` | Component index | `index:0` |

### Examples

```robotframework
*** Test Cases ***
Prefix Format Examples
    # Find by name
    Click    name:submitButton
    Click    name:cancelButton

    # Find by displayed text
    Click    text:Submit
    Click    text:Cancel

    # Find by tooltip
    Click    tooltip:Save document

    # Find by associated label (for form fields)
    Input Text    label:Username    testuser
    Input Text    label:Password    secret

    # Find by class name
    Click    class:javax.swing.JButton
    Click    type:JButton
```

### Wildcard Support

The `text:` and `name:` prefixes support wildcards:

```robotframework
*** Test Cases ***
Wildcard Examples
    # Wildcard at end
    Click    text:Save*

    # Wildcard at start
    Click    text:*Button

    # Wildcards in middle
    Click    name:form_*_submit
```

## ID Shorthand

The `#` symbol provides a shorthand for `name:` lookups.

```robotframework
*** Test Cases ***
ID Shorthand Examples
    # These are equivalent
    Click    name:submitButton
    Click    #submitButton

    # More examples
    Input Text    #username    testuser
    Input Text    #password    secret
    Click    #loginButton
```

## CSS-Style Format

CSS-style selectors provide powerful element matching similar to web CSS selectors.

### Type Selectors

Match by component type (Java class simple name):

```robotframework
*** Test Cases ***
Type Selector Examples
    # Swing components
    Click    JButton
    Input Text    JTextField    hello
    Select From Combobox    JComboBox    Option 1

    # SWT widgets
    Click    Button
    Input Text    Text    hello

    # First match of type
    Click    JButton
```

### Attribute Selectors

Match by component attributes:

| Selector | Description | Example |
|----------|-------------|---------|
| `[attr='value']` | Exact match | `[name='submit']` |
| `[attr*='value']` | Contains | `[text*='Save']` |
| `[attr^='value']` | Starts with | `[name^='btn_']` |
| `[attr$='value']` | Ends with | `[name$='_button']` |
| `[attr]` | Attribute exists | `[tooltip]` |

```robotframework
*** Test Cases ***
Attribute Selector Examples
    # Exact match
    Click    JButton[text='OK']
    Click    JButton[name='submitButton']

    # Contains
    Click    JButton[text*='Save']

    # Starts with
    Click    JButton[name^='btn_']

    # Ends with
    Click    JButton[name$='_submit']

    # Multiple attributes
    Click    JButton[text='Save'][enabled='true']

    # Attribute exists
    Click    JButton[tooltip]
```

### Combined Type and Attribute

```robotframework
*** Test Cases ***
Combined Selectors
    # Type + ID
    Click    JButton#submit

    # Type + attribute
    Click    JButton[text='OK']

    # Type + multiple attributes
    Click    JTextField[name='email'][enabled='true']
```

## XPath Format

XPath selectors provide hierarchical element selection.

### Basic XPath

```robotframework
*** Test Cases ***
XPath Examples
    # Descendant search (anywhere in tree)
    Click    //JButton
    Click    //JButton[@name='submit']

    # Child search (direct children only)
    Click    /JFrame/JPanel/JButton

    # Attribute match
    Click    //JButton[@text='OK']

    # Multiple conditions
    Click    //JButton[@name='submit' and @enabled='true']
```

### XPath Axes

| Axis | Description | Example |
|------|-------------|---------|
| `//` | Descendant | `//JButton` |
| `/` | Child | `/JPanel/JButton` |
| `..` | Parent | `//JButton/..` |
| `following-sibling` | Next sibling | `//JLabel/following-sibling::JTextField` |
| `preceding-sibling` | Previous sibling | `//JButton/preceding-sibling::JLabel` |

```robotframework
*** Test Cases ***
XPath Axis Examples
    # Find text field after a label
    Input Text    //JLabel[@text='Username:']/following-sibling::JTextField    testuser

    # Find button in parent panel
    Click    //JTextField[@name='search']/../JButton
```

### XPath Functions

```robotframework
*** Test Cases ***
XPath Function Examples
    # Contains text
    Click    //JButton[contains(@text, 'Save')]

    # Starts with
    Click    //JButton[starts-with(@name, 'btn_')]

    # Position
    Click    //JButton[position()=1]
    Click    //JButton[last()]
```

## Pseudo-Classes

Pseudo-classes filter elements by state.

| Pseudo-Class | Description | Example |
|--------------|-------------|---------|
| `:enabled` | Enabled elements | `JButton:enabled` |
| `:disabled` | Disabled elements | `JButton:disabled` |
| `:visible` | Visible elements | `JPanel:visible` |
| `:hidden` | Hidden elements | `JPanel:hidden` |
| `:selected` | Selected items | `JCheckBox:selected` |
| `:checked` | Checked checkboxes | `JCheckBox:checked` |
| `:focus` | Focused element | `JTextField:focus` |
| `:first-child` | First child | `JButton:first-child` |
| `:last-child` | Last child | `JButton:last-child` |
| `:nth-child(n)` | Nth child | `JButton:nth-child(2)` |
| `:empty` | No children | `JPanel:empty` |

```robotframework
*** Test Cases ***
Pseudo-Class Examples
    # Only click enabled buttons
    Click    JButton[text='Submit']:enabled

    # Wait for button to be enabled
    Wait Until Element Is Enabled    JButton[text='Next']:disabled

    # Find first button in panel
    Click    JPanel#toolbar > JButton:first-child

    # Find last item in list
    Click    JList > *:last-child

    # Find third tab
    Click    JTabbedPane > *:nth-child(3)

    # Find checked checkboxes
    ${checked}=    Find Elements    JCheckBox:checked
```

## Combinators

Combinators define relationships between elements.

| Combinator | Description | Example |
|------------|-------------|---------|
| ` ` (space) | Descendant | `JPanel JButton` |
| `>` | Direct child | `JPanel > JButton` |
| `+` | Adjacent sibling | `JLabel + JTextField` |
| `~` | General sibling | `JLabel ~ JButton` |

```robotframework
*** Test Cases ***
Combinator Examples
    # Descendant (any depth)
    Click    JFrame JPanel JButton#submit

    # Direct child only
    Click    JPanel#form > JButton

    # Adjacent sibling (immediately after)
    Input Text    JLabel[text='Email:'] + JTextField    test@example.com

    # General sibling (anywhere after)
    Click    JLabel[text='Settings'] ~ JButton[text='Apply']
```

### Complex Hierarchies

```robotframework
*** Test Cases ***
Complex Hierarchy Examples
    # Button inside form panel inside main frame
    Click    JFrame#main JPanel#form JButton#submit

    # Text field directly inside form, after label
    Input Text    JPanel#form > JLabel[text='Name:'] + JTextField    John

    # Any button in the toolbar
    Click    JToolBar > JButton:first-child
```

## Index Selectors

Index selectors pick specific elements when multiple match.

```robotframework
*** Test Cases ***
Index Examples
    # First button (0-indexed)
    Click    JButton[0]

    # Third table row
    Select Table Row    JTable    2

    # Specific tab
    Click    JTabbedPane[0]

    # Combined with other selectors
    Click    JPanel#list JButton[2]
```

## Mode-Specific Considerations

### Swing Mode

Uses Java Swing component names:

| Component | Locator Examples |
|-----------|-----------------|
| Button | `JButton`, `JButton[text='OK']` |
| Text Field | `JTextField`, `JTextField#username` |
| Combo Box | `JComboBox`, `JComboBox[name='country']` |
| Table | `JTable`, `JTable#dataGrid` |
| Tree | `JTree`, `JTree[name='fileTree']` |
| List | `JList`, `JList#items` |
| Check Box | `JCheckBox`, `JCheckBox[text='Accept']` |
| Radio Button | `JRadioButton`, `JRadioButton#optionA` |

### SWT Mode

Uses SWT widget names:

| Widget | Locator Examples |
|--------|-----------------|
| Button | `Button`, `Button[text='OK']` |
| Text | `Text`, `Text#username` |
| Combo | `Combo`, `Combo[name='country']` |
| Table | `Table`, `Table#dataGrid` |
| Tree | `Tree`, `Tree[name='fileTree']` |
| List | `List`, `List#items` |
| Shell | `Shell`, `Shell[text='My Dialog']` |

### RCP Mode

Supports SWT widgets plus RCP-specific locators:

```robotframework
*** Test Cases ***
RCP-Specific Locators
    # Find widget in specific view
    Click    View#org.eclipse.ui.views.ProblemView > Button[text='Clear']

    # Find widget in editor
    Input Text    Editor#Main.java > StyledText    code here
```

## Best Practices

### 1. Prefer Stable Locators

Use name or ID-based locators over text or position:

```robotframework
*** Test Cases ***
Stable Locators
    # Good - uses stable name
    Click    #submitButton
    Click    name:submitButton

    # Less stable - text may change with i18n
    Click    text:Submit

    # Least stable - position may change
    Click    JButton[0]
```

### 2. Use Specific Locators

Be as specific as needed to uniquely identify elements:

```robotframework
*** Test Cases ***
Specific Locators
    # Too generic - may match multiple
    Click    JButton

    # Better - scoped to panel
    Click    JPanel#form JButton[text='Save']

    # Best - unique identifier
    Click    #saveButton
```

### 3. Handle Dynamic Content

Use waits for dynamically loaded content:

```robotframework
*** Test Cases ***
Dynamic Content
    Click    #loadDataButton

    # Wait for data to load
    Wait Until Element Exists    #dataTable    timeout=10

    # Now interact
    Get Table Cell Value    #dataTable    0    0
```

### 4. Use Combinators for Context

When elements don't have unique identifiers:

```robotframework
*** Test Cases ***
Context With Combinators
    # Find text field by its label
    Input Text    JLabel[text='Email:'] + JTextField    test@example.com

    # Find button in specific panel
    Click    JPanel#loginForm > JButton[text='Login']
```

### 5. Debug with UI Tree

When locators don't work, examine the actual UI:

```robotframework
*** Test Cases ***
Debug Locators
    # Print the entire UI tree
    Log UI Tree

    # Or save to file for analysis
    Save UI Tree    ${OUTPUT_DIR}/ui_tree.txt
```

## Common Patterns

### Form Filling

```robotframework
*** Keywords ***
Fill Login Form
    [Arguments]    ${username}    ${password}
    Input Text    #username    ${username}
    Input Text    #password    ${password}
    Click    #loginButton
```

### Table Interaction

```robotframework
*** Keywords ***
Select Row By Value
    [Arguments]    ${table_locator}    ${column}    ${value}
    ${row_count}=    Get Table Row Count    ${table_locator}
    FOR    ${row}    IN RANGE    ${row_count}
        ${cell_value}=    Get Table Cell Value    ${table_locator}    ${row}    ${column}
        IF    '${cell_value}' == '${value}'
            Select Table Row    ${table_locator}    ${row}
            RETURN
        END
    END
    Fail    Row with value '${value}' not found
```

### Tree Navigation

```robotframework
*** Keywords ***
Navigate To Tree Node
    [Arguments]    ${tree_locator}    ${path}
    @{nodes}=    Split String    ${path}    /
    ${current_path}=    Set Variable    ${EMPTY}
    FOR    ${node}    IN    @{nodes}
        ${current_path}=    Set Variable If    '${current_path}'==''
        ...    ${node}    ${current_path}/${node}
        Expand Tree Node    ${tree_locator}    ${current_path}
    END
    Select Tree Node    ${tree_locator}    ${path}
```

## Related Documentation

- [Unified Library Guide](unified-library.md) - Getting started guide
- [Migration Guide](migration-guide.md) - Migrating from legacy libraries
- [ADR-002: Locator Strategy](../adr/ADR-002-locator-syntax-strategy.md) - Design decisions
