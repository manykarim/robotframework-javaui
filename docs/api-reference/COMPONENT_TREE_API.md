# Component Tree API Reference

**Library:** JavaGui.Swing, JavaGui.Swt, JavaGui.Rcp
**Version:** 0.2.0
**Last Updated:** 2026-01-22

---

## Overview

This document provides comprehensive API documentation for all component tree-related keywords in the robotframework-swing library.

---

## Keywords

### Get Component Tree

Retrieves the component tree in various formats with optional depth control.

**Signature:**
```python
def get_component_tree(
    self,
    locator: Optional[str] = None,
    format: str = "text",
    max_depth: Optional[int] = None,
) -> str
```

**Arguments:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `locator` | str | No | None | Optional locator to start from a specific component. If not provided, starts from root window(s). |
| `format` | str | No | "text" | Output format. Valid values: `"text"`, `"json"`, `"yaml"` |
| `max_depth` | int | No | None | Maximum tree depth to traverse. None means unlimited depth. |

**Return Value:**

| Type | Description |
|------|-------------|
| str | String representation of the component tree in the specified format |

**Raises:**

| Exception | When |
|-----------|------|
| `SwingConnectionError` | Not connected to application |
| `ValueError` | Invalid format specified |
| `TimeoutError` | Tree retrieval exceeds timeout |

**Examples:**

```robotframework
# Get full tree in default text format
${tree}=    Get Component Tree

# Get tree in JSON format
${json_tree}=    Get Component Tree    format=json

# Limit depth to 5 levels
${shallow_tree}=    Get Component Tree    max_depth=5

# Get tree starting from specific component
${subtree}=    Get Component Tree    locator=JPanel[name='mainPanel']    format=json

# Combine depth and format
${tree}=    Get Component Tree    format=text    max_depth=3
```

**Output Format Examples:**

**Text Format:**
```
JFrame[name='MainWindow', title='Application']
  JMenuBar
    JMenu[text='File']
      JMenuItem[text='Open']
      JMenuItem[text='Save']
  JPanel[name='content']
    JButton[name='submitBtn']
    JTextField[name='inputField']
```

**JSON Format:**
```json
{
  "roots": [
    {
      "id": 1,
      "class": "javax.swing.JFrame",
      "simpleClass": "JFrame",
      "name": "MainWindow",
      "text": "Application",
      "x": 100,
      "y": 100,
      "width": 800,
      "height": 600,
      "visible": true,
      "enabled": true,
      "showing": true,
      "children": [...]
    }
  ],
  "timestamp": 1737532800000
}
```

**JSON Property Reference:**

| Property | Type | Description |
|----------|------|-------------|
| `id` | int | Unique component identifier |
| `class` | str | Fully qualified class name |
| `simpleClass` | str | Simple class name (e.g., "JButton") |
| `name` | str | Component name (if set, otherwise null) |
| `text` | str | Text content (for labels, buttons, etc.) |
| `x` | int | X coordinate relative to parent |
| `y` | int | Y coordinate relative to parent |
| `width` | int | Component width in pixels |
| `height` | int | Component height in pixels |
| `visible` | bool | Visibility state |
| `enabled` | bool | Enabled state |
| `showing` | bool | Showing state (visible AND all parents visible) |
| `children` | array | Array of child components (if container) |
| `childCount` | int | Number of children (if container) |

**Type-Specific Properties:**

Additional properties are included based on component type:

- **JButton, JLabel, JMenuItem:** `text`, `icon`, `mnemonic`
- **JTextField, JTextArea:** `text`, `editable`, `columns`, `rows`
- **JCheckBox, JRadioButton:** `selected`, `text`
- **JComboBox, JList:** `selectedIndex`, `itemCount`
- **JTable:** `rowCount`, `columnCount`
- **JTree:** `rootVisible`, `rowCount`

**Performance Characteristics:**

| Depth | Typical Components | Approximate Time |
|-------|-------------------|------------------|
| 2 | 10-20 | <100ms |
| 5 | 100-200 | ~500ms |
| 10 | 500-1000 | 1-3s |
| Unlimited | 1000+ | 5-10s+ |

**Related Keywords:**
- `Get Component Subtree` - Get subtree from specific component
- `Log Component Tree` - Log tree to Robot Framework log
- `Refresh Component Tree` - Refresh cached tree

---

### Get Component Subtree

Retrieves a subtree starting from a specific component.

**Signature:**
```python
def get_component_subtree(
    self,
    locator: str,
    format: str = "text",
    max_depth: Optional[int] = None,
) -> str
```

**Arguments:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `locator` | str | Yes | N/A | Component locator to start from. See [Locator Syntax](#locator-syntax). |
| `format` | str | No | "text" | Output format: `"text"`, `"json"`, or `"yaml"` |
| `max_depth` | int | No | None | Maximum depth from starting component. None = unlimited |

**Return Value:**

| Type | Description |
|------|-------------|
| str | String representation of the component subtree |

**Raises:**

| Exception | When |
|-----------|------|
| `ElementNotFoundError` | Locator doesn't match any component |
| `SwingConnectionError` | Not connected to application |
| `TimeoutError` | Operation exceeds timeout |

**Examples:**

```robotframework
# Get subtree of main panel
${subtree}=    Get Component Subtree    JPanel[name='mainPanel']

# Get dialog tree in JSON format
${dialog_json}=    Get Component Subtree    locator=JDialog[title='Settings']    format=json

# Get subtree with depth limit
${tree}=    Get Component Subtree    locator=JScrollPane    max_depth=5    format=text

# Get table container structure
${table_tree}=    Get Component Subtree    JTable[name='dataTable']    format=json    max_depth=3
```

**Use Cases:**

1. **Focus on Specific Section**
   ```robotframework
   ${form_tree}=    Get Component Subtree    JPanel[name='loginForm']
   ```

2. **Dialog Inspection**
   ```robotframework
   Click    JButton[text='Settings']
   Wait Until Element Exists    JDialog[title='Settings']
   ${dialog_structure}=    Get Component Subtree    JDialog[title='Settings']
   ```

3. **Performance Optimization**
   ```robotframework
   # Instead of full tree (slow)
   ${full}=    Get Component Tree

   # Get only needed section (fast)
   ${section}=    Get Component Subtree    JPanel[name='section']
   ```

**Performance Comparison:**

| Operation | Time | Components Retrieved |
|-----------|------|---------------------|
| Full tree (2000 comp.) | 10s | 2000 |
| Subtree (100 comp.) | 200ms | 100 |
| **Speedup** | **50x** | 20x less |

**Related Keywords:**
- `Get Component Tree` - Get full tree
- `Wait Until Element Exists` - Ensure element exists before getting subtree

---

### Log Component Tree

Logs the component tree to the Robot Framework log.

**Signature:**
```python
def log_component_tree(
    self,
    locator: Optional[str] = None,
    format: str = "text",
    level: str = "INFO",
) -> None
```

**Arguments:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `locator` | str | No | None | Optional locator to start from. None = full tree |
| `format` | str | No | "text" | Output format: `"text"` or `"json"` |
| `level` | str | No | "INFO" | Log level: `"TRACE"`, `"DEBUG"`, `"INFO"`, `"WARN"`, `"ERROR"` |

**Return Value:**

None. Output is written to Robot Framework log.

**Examples:**

```robotframework
# Log full tree at INFO level
Log Component Tree

# Log tree at DEBUG level (hidden in normal runs)
Log Component Tree    level=DEBUG

# Log JSON format
Log Component Tree    format=json    level=INFO

# Log subtree
Log Component Tree    locator=JPanel[name='main']    format=text

# Log with all options
Log Component Tree    locator=JDialog    format=json    level=WARN
```

**Log Level Guidelines:**

| Level | When to Use | Visibility |
|-------|-------------|------------|
| `TRACE` | Very detailed debugging | Only in TRACE mode |
| `DEBUG` | Debugging information | Only with --loglevel DEBUG |
| `INFO` | Normal documentation | Always visible |
| `WARN` | Important notes | Highlighted in log |
| `ERROR` | Problems | Red in log |

**Use Cases:**

1. **Test Documentation**
   ```robotframework
   *** Test Cases ***
   Document UI Structure
       [Documentation]    This test documents the login dialog structure
       Click    JButton[text='Login']
       Wait Until Element Exists    JDialog[title='Login']
       Log Component Tree    locator=JDialog[title='Login']    level=INFO
   ```

2. **Debugging**
   ```robotframework
   *** Test Cases ***
   Debug Locator Issue
       # Element not found, log tree to investigate
       Run Keyword And Expect Error    ElementNotFoundError*
       ...    Click    JButton[name='unknown']

       Log Component Tree    level=INFO    # See what's actually there
   ```

3. **Different Verbosity Levels**
   ```robotframework
   # Always visible
   Log Component Tree    level=INFO

   # Only when debugging
   Log Component Tree    level=DEBUG
   ```

**Related Keywords:**
- `Get Component Tree` - Get tree as return value
- `Log` - Standard Robot Framework logging

---

### Refresh Component Tree

Refreshes the cached component tree.

**Signature:**
```python
def refresh_component_tree(self) -> None
```

**Arguments:**

None.

**Return Value:**

None.

**Examples:**

```robotframework
# Refresh after UI change
Click    JButton[text='Add Item']
Refresh Component Tree
${tree}=    Get Component Tree

# Refresh before inspection
Refresh Component Tree
Log Component Tree

# Ensure fresh state
Refresh Component Tree
${count}=    Get Element Count    JButton
```

**When to Use:**

| Scenario | Example |
|----------|---------|
| After opening dialog | `Click JButton` → `Refresh Component Tree` |
| After closing window | `Click Close` → `Refresh Component Tree` |
| After dynamic UI change | `Add/Remove components` → `Refresh Component Tree` |
| Before critical inspection | `Refresh Component Tree` → `Get Component Tree` |
| After layout change | `Resize window` → `Refresh Component Tree` |

**Performance:**

- Operation is fast (<100ms typically)
- Only refreshes cache, doesn't retrieve full tree
- Recommended to call liberally when in doubt

**Automatic Refresh:**

The library automatically refreshes in some scenarios:
- After `Connect To Application`
- After certain UI operations (clicks, inputs)
- On timeout retries

**Manual Refresh Required:**
- After opening/closing dialogs manually
- After programmatic UI changes
- Before tree inspection after unknown state changes

**Example Workflow:**

```robotframework
*** Test Cases ***
UI Change Workflow
    # Initial state
    ${before}=    Get Component Tree    max_depth=3

    # Perform action
    Click    JButton[text='Transform UI']
    Sleep    500ms

    # IMPORTANT: Refresh to see changes
    Refresh Component Tree

    # Get updated state
    ${after}=    Get Component Tree    max_depth=3

    # Verify change occurred
    Should Not Be Equal    ${before}    ${after}
```

**Related Keywords:**
- `Get Component Tree` - Automatically uses cache
- `Wait Until Element Exists` - Refreshes automatically on retries

---

## Legacy Keywords

These keywords are maintained for backwards compatibility but are considered legacy. New code should use the newer alternatives.

### Get UI Tree (Legacy)

**Status:** Maintained for backwards compatibility

**Recommendation:** Use `Get Component Tree` instead

**Signature:**
```python
def get_ui_tree(self, locator: Optional[str] = None) -> str
```

**Differences from `Get Component Tree`:**
- Only supports text format (no JSON/YAML)
- No depth control
- No format parameter

**Migration:**
```robotframework
# Old way
${tree}=    Get UI Tree

# New way (equivalent)
${tree}=    Get Component Tree    format=text

# New way (with enhancements)
${tree}=    Get Component Tree    format=json    max_depth=5
```

---

### Log UI Tree (Legacy)

**Status:** Maintained for backwards compatibility

**Recommendation:** Use `Log Component Tree` instead

**Signature:**
```python
def log_ui_tree(self, locator: Optional[str] = None) -> None
```

**Differences:**
- No format parameter (text only)
- No log level parameter (always INFO)

**Migration:**
```robotframework
# Old way
Log UI Tree

# New way (equivalent)
Log Component Tree    format=text    level=INFO

# New way (with enhancements)
Log Component Tree    format=json    level=DEBUG
```

---

### Refresh UI Tree (Legacy)

**Status:** Maintained for backwards compatibility

**Recommendation:** Use `Refresh Component Tree` instead

**Signature:**
```python
def refresh_ui_tree(self) -> None
```

**Differences:**
- None (identical functionality)

**Migration:**
```robotframework
# Old way
Refresh UI Tree

# New way
Refresh Component Tree
```

---

## Locator Syntax

All tree keywords support the library's standard locator syntax.

### CSS-like Selectors

```robotframework
# By type
Get Component Subtree    JPanel

# By name
Get Component Subtree    [name='mainPanel']

# Type with name
Get Component Subtree    JPanel[name='mainPanel']

# With attribute
Get Component Subtree    JButton[text='Submit']

# Pseudo-selectors
Get Component Subtree    JButton:enabled

# Descendant
Get Component Subtree    JFrame JPanel

# Direct child
Get Component Subtree    JFrame > JMenuBar
```

### XPath-like Selectors

```robotframework
# Any descendant
Get Component Subtree    //JButton

# With attribute
Get Component Subtree    //JButton[@name='submit']

# By index
Get Component Subtree    //JPanel[1]
```

**See Also:** [Locator Syntax Guide](../user-guide/LOCATOR_SYNTAX.md)

---

## Data Types

### Component Node (JSON)

When using `format=json`, each component is represented as a JSON object:

```typescript
interface ComponentNode {
  id: number;                    // Unique ID
  class: string;                 // Full class name
  simpleClass: string;           // Simple class name
  name?: string;                 // Component name (optional)
  text?: string;                 // Text content (optional)
  x: number;                     // X position
  y: number;                     // Y position
  width: number;                 // Width
  height: number;                // Height
  visible: boolean;              // Visible state
  enabled: boolean;              // Enabled state
  showing: boolean;              // Showing state
  children?: ComponentNode[];    // Child components
  childCount?: number;           // Number of children
  // ... additional type-specific properties
}
```

### Tree Root (JSON)

```typescript
interface TreeRoot {
  roots: ComponentNode[];        // Array of root windows
  timestamp: number;             // Retrieval timestamp (Unix ms)
}
```

---

## Error Handling

### Common Exceptions

```robotframework
*** Test Cases ***
Handle Tree Errors
    # Handle connection errors
    ${status}=    Run Keyword And Return Status
    ...    Get Component Tree

    Run Keyword If    not ${status}
    ...    Connect To Application    MyApp

    # Handle element not found
    ${status}=    Run Keyword And Return Status
    ...    Get Component Subtree    JPanel[name='missing']

    ${tree}=    Run Keyword If    ${status}
    ...    Get Component Subtree    JPanel[name='missing']
    ...    ELSE    Get Component Tree

    # Handle timeout
    TRY
        ${tree}=    Get Component Tree
    EXCEPT    TimeoutError
        Log    Tree retrieval timed out, using depth limit
        ${tree}=    Get Component Tree    max_depth=3
    END
```

---

## Best Practices

### 1. Use Appropriate Depth

```robotframework
# ❌ Bad: No limit on large UI
${tree}=    Get Component Tree

# ✅ Good: Depth limit for performance
${tree}=    Get Component Tree    max_depth=5
```

### 2. Use Subtrees for Focused Inspection

```robotframework
# ❌ Bad: Get full tree, then manually filter
${full}=    Get Component Tree
# ... search through full tree ...

# ✅ Good: Get only what you need
${section}=    Get Component Subtree    JPanel[name='section']
```

### 3. Use JSON for Programmatic Access

```robotframework
# ❌ Bad: Parse text format
${text}=    Get Component Tree    format=text
${has_button}=    Run Keyword And Return Status
...    Should Contain    ${text}    JButton

# ✅ Good: Use JSON format
${json}=    Get Component Tree    format=json
${data}=    Evaluate    json.loads('''${json}''')    modules=json
${count}=    Count Components By Type    ${data}    JButton
```

### 4. Refresh After UI Changes

```robotframework
# ❌ Bad: No refresh after change
Click    JButton[text='Open']
${tree}=    Get Component Tree    # May show old state

# ✅ Good: Refresh first
Click    JButton[text='Open']
Sleep    500ms
Refresh Component Tree
${tree}=    Get Component Tree
```

### 5. Use Appropriate Log Levels

```robotframework
# ❌ Bad: Always INFO (clutters log)
Log Component Tree    level=INFO
Log Component Tree    level=INFO
Log Component Tree    level=INFO

# ✅ Good: Use DEBUG for detailed info
Log Component Tree    level=DEBUG    # Hidden normally
Log Component Tree    level=INFO     # Only when needed
```

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.2.0 | 2026-01-22 | Added Get Component Tree, Get Component Subtree, enhanced formats |
| 0.1.0 | 2025-12-01 | Initial Get UI Tree implementation |

---

## See Also

- [Component Tree Guide](../user-guide/COMPONENT_TREE_GUIDE.md) - Comprehensive usage guide
- [Migration Guide](../user-guide/COMPONENT_TREE_MIGRATION_GUIDE.md) - Migration from v0.1.x
- [Troubleshooting Guide](../user-guide/COMPONENT_TREE_TROUBLESHOOTING.md) - Common issues and solutions
- [Locator Syntax](../user-guide/LOCATOR_SYNTAX.md) - Locator documentation

