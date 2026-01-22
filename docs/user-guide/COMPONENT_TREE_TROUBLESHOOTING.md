# Component Tree Troubleshooting Guide

**Version:** 0.2.0
**Last Updated:** 2026-01-22

---

## Table of Contents

1. [Common Issues](#common-issues)
2. [Performance Problems](#performance-problems)
3. [Format and Parsing Issues](#format-and-parsing-issues)
4. [Locator Problems](#locator-problems)
5. [Platform-Specific Issues](#platform-specific-issues)
6. [Error Messages](#error-messages)
7. [Debug Techniques](#debug-techniques)

---

## Common Issues

### Issue: Tree Retrieval is Slow

**Symptoms:**
- `Get Component Tree` takes more than 5 seconds
- Application becomes unresponsive during tree retrieval
- Timeout errors

**Diagnosis:**
```robotframework
# Check how many components you're retrieving
${json}=    Get Component Tree    format=json    max_depth=50
${data}=    Evaluate    json.loads('''${json}''')    modules=json
Log    Retrieved tree data
```

**Solutions:**

1. **Use Depth Limiting**
   ```robotframework
   # Instead of unlimited depth
   ${tree}=    Get Component Tree    # Slow on large UIs

   # Use depth limit
   ${tree}=    Get Component Tree    max_depth=5    # Much faster
   ```

2. **Use Subtrees**
   ```robotframework
   # Instead of full tree
   ${full}=    Get Component Tree    # Gets everything

   # Get just what you need
   ${subtree}=    Get Component Subtree    JPanel[name='targetSection']
   ```

3. **Check Application Complexity**
   ```robotframework
   # Log shallow tree to see structure
   Log Component Tree    max_depth=2    level=INFO

   # Count visible windows
   ${json}=    Get Component Tree    max_depth=1    format=json
   ${data}=    Evaluate    json.loads('''${json}''')    modules=json
   ${roots}=    Get From Dictionary    ${data}    roots
   ${count}=    Get Length    ${roots}
   Log    ${count} root windows found
   ```

**Performance Guidelines:**

| UI Size | Recommended max_depth | Expected Time |
|---------|----------------------|---------------|
| < 100 components | Unlimited | < 1s |
| 100-500 components | 10 | 1-2s |
| 500-1000 components | 5 | 2-5s |
| 1000+ components | 3-5 | 3-10s |

---

### Issue: Tree is Empty or Incomplete

**Symptoms:**
- `Get Component Tree` returns empty string
- Tree missing expected components
- Only partial hierarchy shown

**Diagnosis:**
```robotframework
# Check if connected
${connected}=    Is Connected
Should Be True    ${connected}    Not connected to application

# Check if any windows are visible
${json}=    Get Component Tree    max_depth=1    format=json
${data}=    Evaluate    json.loads('''${json}''')    modules=json
${roots}=    Get From Dictionary    ${data}    roots
${count}=    Get Length    ${roots}
Log    Found ${count} root windows
```

**Solutions:**

1. **Ensure Application is Running**
   ```robotframework
   Connect To Application    MyApp    timeout=30
   Sleep    2s    # Give app time to initialize
   ${tree}=    Get Component Tree
   ```

2. **Refresh Tree After UI Changes**
   ```robotframework
   # After opening dialog/panel
   Click    JButton[text='Open Settings']
   Sleep    500ms
   Refresh Component Tree    # Important!
   ${tree}=    Get Component Tree
   ```

3. **Check Window Visibility**
   ```robotframework
   # Some components may be hidden
   ${json}=    Get Component Tree    format=json
   # Look for "showing": false or "visible": false
   ```

4. **Verify depth isn't too shallow**
   ```robotframework
   # If max_depth=1, you'll only see top level
   ${tree}=    Get Component Tree    max_depth=10    # Increase depth
   ```

---

### Issue: Tree Shows Old State

**Symptoms:**
- Tree doesn't reflect recent UI changes
- Components that were closed still appear
- New dialogs not in tree

**Diagnosis:**
```robotframework
# Check if tree is cached
${tree1}=    Get Component Tree    format=text
Click    JButton[text='Open Dialog']
Sleep    500ms
${tree2}=    Get Component Tree    format=text
${same}=    Evaluate    '''${tree1}''' == '''${tree2}'''
Log    Trees are same: ${same}    # Should be False
```

**Solution:**
```robotframework
# Always refresh after UI changes
Click    JButton[text='Open Dialog']
Wait Until Element Exists    JDialog[title='Settings']
Refresh Component Tree    # Force refresh
${tree}=    Get Component Tree
```

**When to Refresh:**
- After opening/closing dialogs
- After creating/destroying components
- After layout changes
- Before critical inspections

---

## Performance Problems

### Problem: Out of Memory Errors

**Symptoms:**
```
java.lang.OutOfMemoryError: Java heap space
```

**Cause:** Tree is too large (thousands of components)

**Solutions:**

1. **Reduce Depth**
   ```robotframework
   ${tree}=    Get Component Tree    max_depth=3
   ```

2. **Use Text Format Instead of JSON**
   ```robotframework
   # JSON includes all properties (larger)
   ${json}=    Get Component Tree    format=json    # Large

   # Text is more compact
   ${text}=    Get Component Tree    format=text    # Smaller
   ```

3. **Get Multiple Subtrees Instead of Full Tree**
   ```robotframework
   # Instead of one massive tree
   ${huge_tree}=    Get Component Tree

   # Get smaller sections
   ${menu_tree}=    Get Component Subtree    JMenuBar
   ${content_tree}=    Get Component Subtree    JPanel[name='content']
   ${status_tree}=    Get Component Subtree    JPanel[name='statusBar']
   ```

4. **Increase JVM Heap**
   ```bash
   # When starting application
   java -Xmx2g -javaagent:agent.jar=port=5678 -jar app.jar
   ```

---

### Problem: EDT (Event Dispatch Thread) Timeout

**Symptoms:**
```
SwingConnectionError: EDT callable failed
TimeoutError: Operation timed out on EDT
```

**Cause:** Tree retrieval blocked by long-running EDT operation

**Solutions:**

1. **Increase Library Timeout**
   ```robotframework
   *** Settings ***
   Library    JavaGui.Swing    timeout=30    # Increase from default 10

   *** Test Cases ***
   Get Tree
       # Now has 30s timeout
       ${tree}=    Get Component Tree
   ```

2. **Wait for Application to be Idle**
   ```robotframework
   # After triggering action that updates UI
   Click    JButton[text='Load Data']
   Sleep    2s    # Let background loading finish
   ${tree}=    Get Component Tree
   ```

3. **Use Smaller Depth**
   ```robotframework
   # Smaller trees are faster
   ${tree}=    Get Component Tree    max_depth=5
   ```

---

## Format and Parsing Issues

### Problem: JSON Parsing Fails

**Symptoms:**
```python
json.decoder.JSONDecodeError: Expecting value: line 1 column 1 (char 0)
```

**Diagnosis:**
```robotframework
${json}=    Get Component Tree    format=json
Log    ${json}    # Check if valid JSON
${length}=    Get Length    ${json}
Log    JSON length: ${length}
```

**Solutions:**

1. **Use Triple Quotes**
   ```robotframework
   # Wrong - syntax error with multi-line JSON
   ${data}=    Evaluate    json.loads('${json}')    modules=json

   # Right - triple quotes handle multi-line
   ${data}=    Evaluate    json.loads('''${json}''')    modules=json
   ```

2. **Save to File First**
   ```robotframework
   ${json}=    Get Component Tree    format=json
   Create File    ${TEMPDIR}/tree.json    ${json}

   # Read and parse from file
   ${content}=    Get File    ${TEMPDIR}/tree.json
   ${data}=    Evaluate    json.loads('''${content}''')    modules=json
   ```

3. **Check for Empty Response**
   ```robotframework
   ${json}=    Get Component Tree    format=json
   Should Not Be Empty    ${json}    Tree is empty
   ${data}=    Evaluate    json.loads('''${json}''')    modules=json
   ```

---

### Problem: Cannot Access JSON Properties

**Symptoms:**
```python
KeyError: 'children'
TypeError: 'NoneType' object is not subscriptable
```

**Solution:**
```robotframework
# Check if key exists before accessing
${has_children}=    Run Keyword And Return Status
...    Dictionary Should Contain Key    ${node}    children

${children}=    Run Keyword If    ${has_children}
...    Get From Dictionary    ${node}    children
...    ELSE    Create List

# Or use default value
${name}=    Get From Dictionary    ${node}    name    default=${EMPTY}
```

---

### Problem: YAML Format Not Supported

**Symptoms:**
```
ValueError: Unsupported format: yaml
```

**Current Status:**
- ✅ Text format: Fully supported
- ✅ JSON format: Fully supported
- ⚠️ YAML format: Planned for future release

**Workaround:**
```robotframework
# Get as JSON
${json}=    Get Component Tree    format=json

# Convert to YAML using Python (if needed)
${yaml}=    Evaluate    yaml.dump(json.loads('''${json}'''))    modules=yaml,json
```

---

## Locator Problems

### Problem: Subtree Locator Not Found

**Symptoms:**
```
ElementNotFoundError: Element not found: JPanel[name='target']
```

**Diagnosis:**
```robotframework
# Check if element exists
${exists}=    Run Keyword And Return Status
...    Wait Until Element Exists    JPanel[name='target']    timeout=2

Run Keyword If    not ${exists}
...    Log Component Tree    level=INFO    # See what's actually there
```

**Solutions:**

1. **Wait for Element First**
   ```robotframework
   Wait Until Element Exists    JPanel[name='target']    timeout=10
   ${subtree}=    Get Component Subtree    JPanel[name='target']
   ```

2. **Use Fallback to Full Tree**
   ```robotframework
   ${status}    ${subtree}=    Run Keyword And Ignore Error
   ...    Get Component Subtree    JPanel[name='target']

   ${tree}=    Run Keyword If    '${status}' == 'FAIL'
   ...    Get Component Tree
   ...    ELSE    Set Variable    ${subtree}
   ```

3. **Check Locator Syntax**
   ```robotframework
   # Make sure locator is valid
   # Wrong: Missing quotes
   ${tree}=    Get Component Subtree    JPanel[name=target]    # Error

   # Right: Proper quotes
   ${tree}=    Get Component Subtree    JPanel[name='target']    # OK
   ```

4. **Use Get UI Tree to Inspect First**
   ```robotframework
   # See what's available
   Log Component Tree    max_depth=5

   # From log, find correct locator
   ${subtree}=    Get Component Subtree    JPanel[name='actualName']
   ```

---

### Problem: Multiple Matches for Locator

**Symptoms:**
- Subtree contains multiple components
- Unexpected tree structure

**Solution:**
```robotframework
# Be more specific with locator
# Instead of:
${tree}=    Get Component Subtree    JButton    # Might match multiple

# Use:
${tree}=    Get Component Subtree    JButton[name='submitButton']    # Specific
```

---

## Platform-Specific Issues

### Swing-Specific Issues

**Problem: Modal Dialog Blocks Tree Retrieval**

**Solution:**
```robotframework
# Get tree before opening modal dialog
${tree_before}=    Get Component Tree

# Open modal dialog (blocks execution)
# Can't get tree while modal is open

# Close dialog first
Click    JButton[text='Close']
Refresh Component Tree
${tree_after}=    Get Component Tree
```

---

**Problem: JTable/JTree Items Not in Tree**

**Diagnosis:**
```robotframework
# Check if table is using renderer components
${json}=    Get Component Tree    format=json
# Look for JTable children - may not include row cells
```

**Note:** Table and tree items are data, not components. Use table/tree-specific keywords:
```robotframework
# Instead of tree inspection
${rows}=    Get Table Row Count    JTable[name='data']
${data}=    Get Table Cell Value    JTable[name='data']    0    1
```

---

### SWT-Specific Issues

**Problem: Widget Properties Missing**

**Cause:** SWT uses reflection-based access, some properties may not be available

**Workaround:**
```robotframework
# Use SWT-specific keywords when available
${text}=    Get Widget Text    Button[text='OK']

# Tree may have limited properties for SWT widgets
${tree}=    Get Component Tree    format=json
# Will include basic properties only
```

---

**Problem: Performance Slower Than Swing**

**Cause:** Reflection overhead in SWT agent

**Solution:**
```robotframework
# Use more aggressive depth limiting for SWT
${tree}=    Get Component Tree    max_depth=3    # Lower than Swing

# Or use text format (faster than JSON)
${tree}=    Get Component Tree    format=text
```

---

### RCP-Specific Issues

**Problem: Eclipse Workbench Structure Not Shown**

**Cause:** RCP has special structures (Views, Editors, Perspectives) that may not be in component tree

**Solution:**
```robotframework
# Use RCP-specific keywords for workbench
${view_count}=    Get Open View Count
${editor_count}=    Get Open Editor Count
${perspective}=    Get Active Perspective Id

# Component tree shows widget hierarchy only
${tree}=    Get Component Tree
```

---

## Error Messages

### `SwingConnectionError: Connection refused`

**Meaning:** Cannot connect to Java agent

**Check:**
1. Is application running?
2. Is agent loaded? (`-javaagent:path/to/agent.jar=port=5678`)
3. Is port correct?
4. Is firewall blocking connection?

**Solution:**
```robotframework
# Verify connection first
${cmd}=    Set Variable    java -javaagent:agent.jar=port=5678 -jar app.jar
Start Process    ${cmd}    shell=True    alias=app
Sleep    3s    # Wait for startup
Connect To Application    host=localhost    port=5678    timeout=30
```

---

### `ElementNotFoundError: Element not found`

**Meaning:** Locator for subtree doesn't match any component

**Solution:**
```robotframework
# Inspect tree to find correct locator
Log Component Tree    max_depth=5    level=INFO

# Use correct locator from log
${tree}=    Get Component Subtree    JPanel[name='foundInLog']
```

---

### `TimeoutError: Operation timed out`

**Meaning:** Tree retrieval took longer than timeout

**Solution:**
```robotframework
# Increase timeout
*** Settings ***
Library    JavaGui.Swing    timeout=60

# Or use depth limit
${tree}=    Get Component Tree    max_depth=5
```

---

### `TypeError: 'NoneType' object has no attribute`

**Meaning:** Trying to access property that doesn't exist

**Solution:**
```robotframework
# Check before accessing
${has_prop}=    Run Keyword And Return Status
...    Dictionary Should Contain Key    ${node}    propertyName

${value}=    Run Keyword If    ${has_prop}
...    Get From Dictionary    ${node}    propertyName
...    ELSE    Set Variable    ${default_value}
```

---

## Debug Techniques

### Technique 1: Progressive Inspection

Start shallow, drill deeper:

```robotframework
*** Test Cases ***
Debug Component Tree
    # Level 1: See high-level structure
    Log Component Tree    max_depth=2    level=INFO

    # Level 2: Identify section of interest
    # (Review log output)

    # Level 3: Get detailed view of section
    ${details}=    Get Component Subtree    JPanel[name='identified']    max_depth=10
    Log    ${details}

    # Level 4: Analyze JSON for properties
    ${json}=    Get Component Subtree    JPanel[name='identified']    format=json
    Create File    ${OUTPUT_DIR}/debug_tree.json    ${json}
```

---

### Technique 2: Compare States

```robotframework
*** Test Cases ***
Debug UI Change
    # Capture before state
    ${before}=    Get Component Tree    max_depth=5    format=text
    Create File    ${OUTPUT_DIR}/before.txt    ${before}

    # Perform action
    Click    JButton[text='Change UI']
    Sleep    500ms
    Refresh Component Tree

    # Capture after state
    ${after}=    Get Component Tree    max_depth=5    format=text
    Create File    ${OUTPUT_DIR}/after.txt    ${after}

    # Manually diff the files to see changes
    Log    Compare ${OUTPUT_DIR}/before.txt and ${OUTPUT_DIR}/after.txt
```

---

### Technique 3: JSON Property Inspection

```robotframework
*** Test Cases ***
Inspect Component Properties
    ${json}=    Get Component Tree    format=json
    ${data}=    Evaluate    json.loads('''${json}''')    modules=json

    # Pretty print for inspection
    ${pretty}=    Evaluate    json.dumps(${data}, indent=2)    modules=json
    Create File    ${OUTPUT_DIR}/tree_pretty.json    ${pretty}

    Log    Inspect ${OUTPUT_DIR}/tree_pretty.json for component properties
```

---

### Technique 4: Isolated Component Test

```robotframework
*** Test Cases ***
Test Specific Component
    # Find component
    Wait Until Element Exists    JButton[name='target']    timeout=10

    # Get just that component's tree
    ${tree}=    Get Component Subtree    JButton[name='target']    max_depth=2
    Log    ${tree}

    # Verify properties
    ${json}=    Get Component Subtree    JButton[name='target']    format=json
    ${data}=    Evaluate    json.loads('''${json}''')    modules=json

    # Check specific properties
    ${class}=    Get From Dictionary    ${data}    simpleClass
    Should Be Equal    ${class}    JButton

    ${enabled}=    Get From Dictionary    ${data}    enabled
    Should Be True    ${enabled}
```

---

### Technique 5: Performance Profiling

```robotframework
*** Test Cases ***
Profile Tree Retrieval
    # Test different configurations
    ${start}=    Get Time    epoch

    # Test 1: Full tree
    ${full}=    Get Component Tree
    ${time_full}=    Evaluate    time.time() - ${start}    modules=time
    ${size_full}=    Get Length    ${full}

    # Test 2: Depth 5
    ${start}=    Get Time    epoch
    ${d5}=    Get Component Tree    max_depth=5
    ${time_d5}=    Evaluate    time.time() - ${start}    modules=time
    ${size_d5}=    Get Length    ${d5}

    # Test 3: Subtree
    ${start}=    Get Time    epoch
    ${sub}=    Get Component Subtree    JPanel[name='main']
    ${time_sub}=    Evaluate    time.time() - ${start}    modules=time
    ${size_sub}=    Get Length    ${sub}

    # Log results
    Log    Full: ${time_full}s (${size_full} chars)
    Log    Depth 5: ${time_d5}s (${size_d5} chars)
    Log    Subtree: ${time_sub}s (${size_sub} chars)
```

---

## Getting Help

If you can't resolve your issue:

1. **Gather Information:**
   - Error message (full stack trace)
   - Robot Framework log
   - Component tree output (if available)
   - Application type (Swing/SWT/RCP)
   - Library version
   - Java version

2. **Create Minimal Example:**
   ```robotframework
   *** Test Cases ***
   Minimal Reproduction
       Connect To Application    MyApp
       ${tree}=    Get Component Tree    # Fails here
       Log    ${tree}
   ```

3. **Report Issue:**
   - GitHub: https://github.com/manykarim/robotframework-javaui/issues
   - Include: error message, minimal example, environment details
   - Use label: `component-tree`

---

## Related Documentation

- [Component Tree Guide](COMPONENT_TREE_GUIDE.md)
- [Migration Guide](COMPONENT_TREE_MIGRATION_GUIDE.md)
- [API Reference](../api-reference/)

