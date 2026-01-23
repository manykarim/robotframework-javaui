# RCP Component Tree Guide

## Overview

The RCP (Rich Client Platform) component tree feature provides comprehensive access to Eclipse RCP application structures, exposing:

- **Workbench Windows** - Top-level application windows
- **Perspectives** - Workspace layouts and configurations
- **Views** - Plugin-contributed view parts
- **Editors** - File and content editors
- **Underlying SWT Widgets** - All SWT controls used by RCP components

**Key Feature**: Since RCP is built on top of SWT, all SWT operations work on RCP components. Every RCP component exposes its underlying SWT widget IDs, allowing you to use standard SWT automation methods.

## Quick Start

### Get Complete RCP Tree

```robot
*** Test Cases ***
Inspect RCP Application
    ${tree}=    Get RCP Component Tree    max_depth=5    format=json
    Log    ${tree}
```

### Get All Views

```robot
*** Test Cases ***
List All Views
    ${views}=    Get All RCP Views    include_swt_widgets=False
    Log    Found ${views.__len__()} views
```

### Get All Editors

```robot
*** Test Cases ***
List Open Editors
    ${editors}=    Get All RCP Editors    include_swt_widgets=True
    Should Not Be Empty    ${editors}
```

## API Reference

### Get RCP Component Tree

Retrieves the complete hierarchical structure of the RCP application.

```python
get_rcp_component_tree(max_depth=5, format="json")
```

**Parameters:**
- `max_depth` (int): Maximum depth for SWT widget trees under each RCP component (default: 5)
- `format` (str): Output format - "json", "text", or "yaml" (default: "json")

**Returns:**
- String containing the RCP component tree in the specified format

**Example:**
```robot
${tree_json}=    Get RCP Component Tree    max_depth=3    format=json
${tree_text}=    Get RCP Component Tree    max_depth=2    format=text
```

### Get All RCP Views

Returns all open views in the workbench.

```python
get_all_rcp_views(include_swt_widgets=False)
```

**Parameters:**
- `include_swt_widgets` (bool): Include underlying SWT widget trees (default: False)

**Returns:**
- JSON string containing array of view objects

**Example:**
```robot
${views}=    Get All RCP Views
${views_with_widgets}=    Get All RCP Views    include_swt_widgets=True
```

### Get All RCP Editors

Returns all open editors in the workbench.

```python
get_all_rcp_editors(include_swt_widgets=False)
```

**Parameters:**
- `include_swt_widgets` (bool): Include underlying SWT widget trees (default: False)

**Returns:**
- JSON string containing array of editor objects

**Example:**
```robot
${editors}=    Get All RCP Editors
${editors_with_widgets}=    Get All RCP Editors    include_swt_widgets=True
```

### Get RCP Component

Retrieves a specific RCP component by path.

```python
get_rcp_component(path, max_depth=3)
```

**Parameters:**
- `path` (str): Component path (e.g., "window[0]/page[0]/view[org.example.view]")
- `max_depth` (int): Maximum depth for SWT widget tree (default: 3)

**Returns:**
- JSON string containing the component details

**Example:**
```robot
${view}=    Get RCP Component    path=window[0]/page[0]/view[org.eclipse.ui.navigator]
```

## RCP Tree Structure

### Workbench Window

```json
{
  "type": "WorkbenchWindow",
  "title": "Eclipse IDE",
  "active": true,
  "swtShellId": 123,
  "swtClass": "org.eclipse.swt.widgets.Shell",
  "pages": [...],
  "pageCount": 1
}
```

**Key Fields:**
- `swtShellId`: SWT widget ID for the window's Shell - use for SWT operations
- `swtClass`: SWT class name
- `pages`: Array of workbench pages

### Workbench Page

```json
{
  "type": "WorkbenchPage",
  "perspective": {
    "type": "Perspective",
    "id": "org.eclipse.ui.resourcePerspective",
    "label": "Resource"
  },
  "views": [...],
  "viewCount": 5,
  "editors": [...],
  "editorCount": 2,
  "activePart": "MyEditor.java"
}
```

**Key Fields:**
- `perspective`: Active perspective descriptor
- `views`: Array of view parts
- `editors`: Array of editor parts

### View Part

```json
{
  "type": "ViewPart",
  "id": "org.eclipse.ui.navigator.ProjectExplorer",
  "secondaryId": null,
  "name": "Project Explorer",
  "title": "Project Explorer",
  "fastView": false,
  "partCreated": true,
  "swtControlId": 456,
  "swtControlClass": "org.eclipse.swt.widgets.Composite",
  "swtWidgetTree": {...}
}
```

**Key Fields:**
- `id`: View ID (unique identifier)
- `secondaryId`: Optional secondary ID for multiple instances
- `swtControlId`: SWT widget ID for the view's control - **use for SWT operations**
- `swtWidgetTree`: Full SWT widget hierarchy (if `include_swt_widgets=True`)

### Editor Part

```json
{
  "type": "EditorPart",
  "id": "org.eclipse.ui.DefaultTextEditor",
  "name": "MyFile.java",
  "title": "MyFile.java",
  "tooltip": "/project/src/MyFile.java",
  "dirty": true,
  "filePath": "/project/src/MyFile.java",
  "partCreated": true,
  "swtControlId": 789,
  "swtWidgetTree": {...}
}
```

**Key Fields:**
- `dirty`: Whether editor has unsaved changes
- `filePath`: File system path (if applicable)
- `swtControlId`: SWT widget ID for the editor's control - **use for SWT operations**

### Perspective

```json
{
  "type": "Perspective",
  "id": "org.eclipse.jdt.ui.JavaPerspective",
  "label": "Java"
}
```

**Key Fields:**
- `id`: Perspective ID (often includes plugin namespace)
- `label`: Display name

## Using SWT Operations on RCP Components

Since RCP is built on SWT, every RCP component exposes SWT widget IDs that can be used with standard SWT operations.

### Example: Click a Button in a View

```robot
*** Test Cases ***
Click Button In View
    # Get all views with SWT widgets
    ${views}=    Get All RCP Views    include_swt_widgets=True
    ${view}=    Evaluate    json.loads('''${views}''')[0]    json

    # Get the view's SWT control ID
    ${swt_id}=    Set Variable    ${view}[swtControlId]

    # Now use standard SWT operations to find and click a button
    ${button}=    Find Widget    type=class    value=org.eclipse.swt.widgets.Button    parent=${swt_id}
    Click Widget    ${button}
```

### Example: Get Text from Editor

```robot
*** Test Cases ***
Read Editor Content
    ${editors}=    Get All RCP Editors    include_swt_widgets=True
    ${editor}=    Evaluate    json.loads('''${editors}''')[0]    json

    # Get the editor's SWT control (usually a StyledText)
    ${swt_id}=    Set Variable    ${editor}[swtControlId]

    # Find the text widget and read it
    ${text_widget}=    Find Widget    type=class    value=org.eclipse.swt.custom.StyledText    parent=${swt_id}
    ${content}=    Get Widget Property    ${text_widget}    text
    Log    Editor contains: ${content}
```

### Example: Verify Perspective

```robot
*** Test Cases ***
Verify Java Perspective Active
    ${tree}=    Get RCP Component Tree    max_depth=1    format=json
    ${tree_dict}=    Evaluate    json.loads('''${tree}''')    json

    ${page}=    Set Variable    ${tree_dict}[windows][0][pages][0]
    ${perspective}=    Set Variable    ${page}[perspective]

    Should Be Equal    ${perspective}[id]    org.eclipse.jdt.ui.JavaPerspective
    Should Be Equal    ${perspective}[label]    Java
```

## Output Formats

### JSON Format

Structured JSON with full component hierarchy and properties. Best for programmatic access.

```robot
${tree}=    Get RCP Component Tree    format=json
```

### Text Format

Human-readable indented text. Best for debugging and logging.

```robot
${tree}=    Get RCP Component Tree    format=text
Log    ${tree}
```

Example output:
```
RcpWorkbench
  WorkbenchWindow [Eclipse IDE]
    WorkbenchPage
      Perspective [Java]
        id: org.eclipse.jdt.ui.JavaPerspective
      ViewPart [Project Explorer]
      ViewPart [Package Explorer]
      EditorPart [MyFile.java]
```

### YAML Format

YAML representation for configuration or export.

```robot
${tree}=    Get RCP Component Tree    format=yaml
```

## Depth Control

Control how deep the SWT widget trees are included:

- `max_depth=0`: RCP components only, no SWT widgets
- `max_depth=1`: RCP components + 1 level of SWT widgets
- `max_depth=5`: RCP components + 5 levels of SWT widgets (default)

```robot
# Minimal - just RCP structure
${tree}=    Get RCP Component Tree    max_depth=0

# Moderate - RCP + immediate SWT children
${tree}=    Get RCP Component Tree    max_depth=2

# Full - deep SWT hierarchy
${tree}=    Get RCP Component Tree    max_depth=10
```

**Performance Note**: Higher depths include more widget information but take longer to retrieve and produce larger output.

## Common Use Cases

### 1. Verify Application Structure

```robot
*** Test Cases ***
Verify All Views Present
    ${views}=    Get All RCP Views
    ${view_list}=    Evaluate    json.loads('''${views}''')    json

    ${view_ids}=    Create List
    FOR    ${view}    IN    @{view_list}
        Append To List    ${view_ids}    ${view}[id]
    END

    Should Contain    ${view_ids}    org.eclipse.ui.navigator.ProjectExplorer
    Should Contain    ${view_ids}    org.eclipse.jdt.ui.PackageExplorer
```

### 2. Wait for Editor to Open

```robot
*** Test Cases ***
Wait For Editor
    Wait Until Keyword Succeeds    10s    1s    Editor Should Be Open    MyFile.java

*** Keywords ***
Editor Should Be Open
    [Arguments]    ${expected_name}
    ${editors}=    Get All RCP Editors
    ${editor_list}=    Evaluate    json.loads('''${editors}''')    json

    ${found}=    Set Variable    ${FALSE}
    FOR    ${editor}    IN    @{editor_list}
        ${found}=    Set Variable If    '${editor}[name]'=='${expected_name}'    ${TRUE}    ${found}
    END

    Should Be True    ${found}    Editor ${expected_name} not found
```

### 3. Interact with View Content

```robot
*** Test Cases ***
Expand Tree In Navigator
    # Get the Project Explorer view
    ${views}=    Get All RCP Views    include_swt_widgets=True
    ${navigator}=    Evaluate    [v for v in json.loads('''${views}''') if v['id']=='org.eclipse.ui.navigator.ProjectExplorer'][0]    json

    # Find the tree widget inside the view
    ${tree}=    Find Widget    type=class    value=org.eclipse.swt.widgets.Tree    parent=${navigator}[swtControlId]

    # Expand a tree item
    ${item}=    Find Tree Item    ${tree}    MyProject
    Expand Tree Item    ${item}
```

## Plugin Architecture

RCP views and editors come from plugins. The view/editor ID often includes the plugin namespace:

- `org.eclipse.ui.navigator.ProjectExplorer` - from org.eclipse.ui plugin
- `org.eclipse.jdt.ui.PackageExplorer` - from Java Development Tools
- `com.mycompany.myapp.MyView` - from custom plugin

The component tree preserves these IDs, allowing you to:
1. Verify which plugins are loaded
2. Test plugin-specific functionality
3. Handle multiple plugin versions

## Error Handling

### RCP Not Available

If the application is not an Eclipse RCP application:

```json
{
  "type": "RcpWorkbench",
  "available": false,
  "error": "Eclipse RCP not available"
}
```

### Part Not Created

Views/editors may not be instantiated yet:

```json
{
  "type": "ViewPart",
  "id": "org.example.MyView",
  "partCreated": false
}
```

In this case, `swtControlId` will not be available until the part is created.

## Performance Considerations

1. **Use `max_depth` wisely**: Higher depths mean more data to traverse
2. **Use `include_swt_widgets=False` when not needed**: Faster retrieval
3. **Cache results**: RCP tree doesn't change often during test execution
4. **Query specific views/editors**: Use `Get All RCP Views` instead of full tree when possible

## Comparison with SWT Backend

| Feature | SWT Backend | RCP Backend |
|---------|-------------|-------------|
| Widget tree | ✅ Shell → Composite → Control | ✅ Workbench → Page → View/Editor → SWT widgets |
| Perspectives | ❌ | ✅ View layouts and configurations |
| Views | ❌ | ✅ Plugin-contributed views |
| Editors | ❌ | ✅ File and content editors |
| Plugin metadata | ❌ | ✅ Plugin IDs and namespaces |
| SWT operations | ✅ | ✅ **All SWT operations work on RCP widgets** |

## Best Practices

1. **Always check `available` flag** before processing RCP tree
2. **Use `partCreated` flag** before accessing SWT widget IDs
3. **Prefer specific queries** over full tree when possible
4. **Use JSON format** for programmatic access, text for debugging
5. **Set appropriate `max_depth`** based on your needs (2-3 is usually enough)

## Troubleshooting

### Issue: "Eclipse RCP not available"

**Cause**: Application is not an Eclipse RCP application, or Eclipse APIs not on classpath.

**Solution**:
- Verify you're testing an RCP application (not plain SWT)
- Check that Eclipse platform JARs are accessible
- Use SWT backend for non-RCP applications

### Issue: "swtControlId" not present

**Cause**: View/editor part not yet created.

**Solution**:
- Check `partCreated` flag before accessing SWT IDs
- Try showing the view first (using perspective/view operations)
- Wait for the part to be instantiated

### Issue: Large tree takes too long

**Cause**: High `max_depth` or many open views/editors.

**Solution**:
- Reduce `max_depth` to 2-3
- Query specific views instead of full tree
- Use `include_swt_widgets=False`

## See Also

- [SWT Backend Guide](SWT_QUICK_START.md)
- [Component Tree API](COMPONENT_TREE_DOCUMENTATION_INDEX.md)
- [Performance Guide](USER_PERFORMANCE_GUIDE.md)
