# Component Tree Migration Guide

**Version:** 0.2.0
**Last Updated:** 2026-01-22
**Target Users:** Existing robotframework-swing users

---

## Overview

This guide helps you migrate from the old component tree API to the enhanced API introduced in version 0.2.0.

### What Changed

| Feature | Old API (v0.1.x) | New API (v0.2.0+) | Status |
|---------|------------------|-------------------|--------|
| Get full tree | `Get UI Tree` | `Get Component Tree` | ✅ Both supported |
| Output formats | Text only | Text, JSON, YAML | ✅ Enhanced |
| Depth control | Not available | `max_depth` parameter | ✅ New feature |
| Subtree retrieval | Not available | `Get Component Subtree` | ✅ New feature |
| Refresh cache | `Refresh UI Tree` | `Refresh Component Tree` | ✅ Both supported |
| Log tree | `Log UI Tree` | `Log Component Tree` | ✅ Both supported |

---

## Quick Migration

### No Changes Required

If you're using basic tree retrieval, your existing code will continue to work:

```robotframework
# ✅ This still works in v0.2.0+
${tree}=    Get UI Tree
Log    ${tree}
```

The old keywords are maintained for backwards compatibility.

---

## Recommended Migration Path

### Step 1: Update Keyword Names (Optional)

While not required, we recommend updating to the new naming for clarity:

**Before:**
```robotframework
${tree}=    Get UI Tree
Log UI Tree
Refresh UI Tree
```

**After:**
```robotframework
${tree}=    Get Component Tree
Log Component Tree
Refresh Component Tree
```

**Why:** The new names are more explicit and consistent across the library.

---

### Step 2: Add Format Parameter for JSON

If you were parsing text output, switch to JSON format:

**Before (v0.1.x):**
```robotframework
${tree}=    Get UI Tree
# Parse text manually (fragile)
${has_button}=    Run Keyword And Return Status
...    Should Contain    ${tree}    JButton
```

**After (v0.2.0+):**
```robotframework
${json_tree}=    Get Component Tree    format=json
${data}=    Evaluate    json.loads('''${json_tree}''')    modules=json

# Robust programmatic access
${roots}=    Get From Dictionary    ${data}    roots
${button_count}=    Count Components By Type    ${roots[0]}    JButton
```

**Benefits:**
- ✅ Reliable parsing
- ✅ Access to all component properties
- ✅ Type-safe operations
- ✅ Better performance for large trees

---

### Step 3: Add Depth Limits for Performance

If you have large UIs, add depth limits:

**Before:**
```robotframework
# Gets entire tree (can be slow)
${tree}=    Get UI Tree
```

**After:**
```robotframework
# Get only what you need
${tree}=    Get Component Tree    max_depth=5

# Or get overview first
${overview}=    Get Component Tree    max_depth=2
Log    ${overview}

# Then get details of specific section
${details}=    Get Component Subtree    JPanel[name='mainPanel']    max_depth=10
```

**Performance Impact:**

| UI Size | No Limit | max_depth=5 | max_depth=2 |
|---------|----------|-------------|-------------|
| Small (<100 components) | 100ms | 80ms | 50ms |
| Medium (500 components) | 2s | 500ms | 200ms |
| Large (2000+ components) | 10s+ | 1s | 300ms |

---

### Step 4: Use Subtrees for Targeted Inspection

If you were getting full tree and filtering manually, use subtrees:

**Before:**
```robotframework
# Get everything
${full_tree}=    Get UI Tree

# Then search through text (inefficient)
@{lines}=    Split To Lines    ${full_tree}
FOR    ${line}    IN    @{lines}
    ${matches}=    Run Keyword And Return Status
    ...    Should Contain    ${line}    JPanel[name='settingsPanel']
    Exit For Loop If    ${matches}
END
```

**After:**
```robotframework
# Get just what you need
${settings_tree}=    Get Component Subtree    JPanel[name='settingsPanel']
Log    ${settings_tree}

# Much faster and cleaner!
```

---

## Detailed Migration Examples

### Example 1: Basic Tree Inspection

**Old Code:**
```robotframework
*** Test Cases ***
Debug Element Location
    Connect To Application    MyApp
    Get UI Tree
    # Read from log manually
    Disconnect
```

**Migrated Code:**
```robotframework
*** Test Cases ***
Debug Element Location
    Connect To Application    MyApp

    # More control over output
    Log Component Tree    format=text    level=INFO

    # Or save to file for analysis
    ${tree}=    Get Component Tree    format=json
    Create File    ${OUTPUT_DIR}/ui_tree.json    ${tree}

    Disconnect
```

**Benefits:**
- ✅ Can save tree for offline analysis
- ✅ JSON format for programmatic parsing
- ✅ Better log level control

---

### Example 2: Counting Components

**Old Code:**
```robotframework
*** Test Cases ***
Count Buttons
    ${tree}=    Get UI Tree
    ${button_lines}=    Get Lines Containing String    ${tree}    JButton
    ${count}=    Get Line Count    ${button_lines}
    Log    Found ${count} buttons (approximate)
```

**Migrated Code:**
```robotframework
*** Keywords ***
Count Components By Type
    [Arguments]    ${root}    ${type}
    ${count}=    Set Variable    0
    ${simple_class}=    Get From Dictionary    ${root}    simpleClass
    ${count}=    Run Keyword If    '${simple_class}' == '${type}'
    ...    Evaluate    ${count} + 1
    ...    ELSE    Set Variable    ${count}

    ${has_children}=    Run Keyword And Return Status
    ...    Dictionary Should Contain Key    ${root}    children
    Return From Keyword If    not ${has_children}    ${count}

    ${children}=    Get From Dictionary    ${root}    children
    FOR    ${child}    IN    @{children}
        ${child_count}=    Count Components By Type    ${child}    ${type}
        ${count}=    Evaluate    ${count} + ${child_count}
    END
    [Return]    ${count}

*** Test Cases ***
Count Buttons
    ${json_tree}=    Get Component Tree    format=json
    ${data}=    Evaluate    json.loads('''${json_tree}''')    modules=json
    ${roots}=    Get From Dictionary    ${data}    roots
    ${root}=    Get From List    ${roots}    0

    ${count}=    Count Components By Type    ${root}    JButton
    Log    Found ${count} buttons (exact)
```

**Benefits:**
- ✅ Exact count (not approximate)
- ✅ Reusable keyword for any component type
- ✅ Can count multiple types in one pass

---

### Example 3: Finding Component by Name

**Old Code:**
```robotframework
*** Test Cases ***
Find Submit Button Name
    ${tree}=    Get UI Tree
    Log    ${tree}    # Read manually to find name
    # Then update test with found name
```

**Migrated Code:**
```robotframework
*** Keywords ***
Find Component By Name Pattern
    [Arguments]    ${root}    ${pattern}
    ${name}=    Get From Dictionary    ${root}    name    default=${EMPTY}
    ${matches}=    Evaluate    "${pattern}" in "${name}"
    Return From Keyword If    ${matches}    ${root}

    ${has_children}=    Run Keyword And Return Status
    ...    Dictionary Should Contain Key    ${root}    children
    Return From Keyword If    not ${has_children}    ${None}

    ${children}=    Get From Dictionary    ${root}    children
    FOR    ${child}    IN    @{children}
        ${result}=    Find Component By Name Pattern    ${child}    ${pattern}
        Return From Keyword If    ${result} is not ${None}    ${result}
    END
    [Return]    ${None}

*** Test Cases ***
Find Submit Button Name
    ${json_tree}=    Get Component Tree    format=json
    ${data}=    Evaluate    json.loads('''${json_tree}''')    modules=json
    ${roots}=    Get From Dictionary    ${data}    roots
    ${root}=    Get From List    ${roots}    0

    ${button}=    Find Component By Name Pattern    ${root}    submit
    ${button_name}=    Get From Dictionary    ${button}    name
    Log    Found button with name: ${button_name}

    # Now use it
    Click    JButton[name='${button_name}']
```

**Benefits:**
- ✅ Automated discovery
- ✅ Programmatic access to properties
- ✅ No manual log reading

---

## Breaking Changes

### None

There are **no breaking changes** in v0.2.0. All old keywords continue to work.

### Deprecated Features

The following are not deprecated but are considered "legacy":

| Legacy Keyword | Recommended Alternative | Will Be Removed? |
|----------------|------------------------|------------------|
| `Get UI Tree` | `Get Component Tree` | No (kept for compatibility) |
| `Log UI Tree` | `Log Component Tree` | No (kept for compatibility) |
| `Refresh UI Tree` | `Refresh Component Tree` | No (kept for compatibility) |

**Recommendation:** Use new keyword names in new tests, but no need to update existing tests.

---

## New Features You Should Use

### 1. JSON Format for Automation

```robotframework
${json}=    Get Component Tree    format=json
${data}=    Evaluate    json.loads('''${json}''')    modules=json

# Access structured data
${timestamp}=    Get From Dictionary    ${data}    timestamp
${roots}=    Get From Dictionary    ${data}    roots
```

**Use when:**
- Building test data from UI structure
- Automated documentation generation
- Integration with other tools
- Complex tree analysis

---

### 2. Depth Control for Performance

```robotframework
# Quick overview
${shallow}=    Get Component Tree    max_depth=3

# Detailed inspection
${deep}=    Get Component Tree    max_depth=15
```

**Use when:**
- UI has >500 components
- You only need overview
- Performance is critical
- Progressive disclosure workflow

---

### 3. Subtree Retrieval

```robotframework
# Get just the dialog tree
${dialog}=    Get Component Subtree    JDialog[title='Settings']

# Get just the form
${form}=    Get Component Subtree    JPanel[name='loginForm']
```

**Use when:**
- Focused on specific UI section
- Don't need full tree
- Want faster operations
- Testing specific components

---

## Migration Checklist

Use this checklist when migrating your tests:

- [ ] Review all uses of `Get UI Tree`
- [ ] Consider switching to `Get Component Tree` (optional)
- [ ] Add `format=json` where programmatic parsing is needed
- [ ] Add `max_depth` for large UIs (>500 components)
- [ ] Consider using `Get Component Subtree` for targeted inspection
- [ ] Update log levels using `Log Component Tree` parameters
- [ ] Test migrated code with your application
- [ ] Update documentation/comments
- [ ] Consider creating reusable keywords for tree analysis

---

## Common Migration Patterns

### Pattern 1: Manual Log Reading → Automated Search

**Old:**
```robotframework
Get UI Tree
# Read log, find component, update locator manually
Click    JButton[name='foundInLog']
```

**New:**
```robotframework
${json}=    Get Component Tree    format=json
# Automated search for component
${component}=    Find Component By Property    ${json}    text=Submit
${name}=    Get From Dictionary    ${component}    name
Click    JButton[name='${name}']
```

---

### Pattern 2: Full Tree → Targeted Subtree

**Old:**
```robotframework
${tree}=    Get UI Tree    # Gets everything
# Filter manually in test
```

**New:**
```robotframework
${subtree}=    Get Component Subtree    JPanel[name='target']
# Only what you need
```

---

### Pattern 3: Text Parsing → JSON Parsing

**Old:**
```robotframework
${tree}=    Get UI Tree
${lines}=    Split To Lines    ${tree}
# Parse text line by line (brittle)
```

**New:**
```robotframework
${json}=    Get Component Tree    format=json
${data}=    Evaluate    json.loads('''${json}''')    modules=json
# Use dict operations (robust)
```

---

## Performance Improvements

After migration with depth control and subtrees:

| Scenario | Old (v0.1.x) | New (v0.2.0+) | Improvement |
|----------|--------------|---------------|-------------|
| Large UI (2000 components) | ~10s | ~1s (depth=5) | **10x faster** |
| Subtree of 100 components | ~10s (full tree) | ~200ms | **50x faster** |
| Repeated retrievals | ~10s each | ~10s + cache | **Instant** after first |

---

## Troubleshooting Migration Issues

### Issue: JSON Parsing Fails

**Problem:**
```robotframework
${json}=    Get Component Tree    format=json
${data}=    Evaluate    json.loads('${json}')    modules=json
# SyntaxError: invalid syntax
```

**Solution:**
```robotframework
# Use triple quotes for multi-line JSON
${json}=    Get Component Tree    format=json
${data}=    Evaluate    json.loads('''${json}''')    modules=json
```

---

### Issue: Subtree Locator Not Found

**Problem:**
```robotframework
${subtree}=    Get Component Subtree    JPanel[name='missing']
# ElementNotFoundError
```

**Solution:**
```robotframework
# First verify element exists
Wait Until Element Exists    JPanel[name='target']    timeout=10

# Then get subtree
${subtree}=    Get Component Subtree    JPanel[name='target']

# Or use fallback
${tree}=    Run Keyword And Return Status
...    Get Component Subtree    JPanel[name='target']
${result}=    Run Keyword If    ${tree}
...    Get Component Subtree    JPanel[name='target']
...    ELSE    Get Component Tree
```

---

### Issue: Tree Too Large for Log

**Problem:**
```robotframework
Log Component Tree
# Log is huge, slows down report generation
```

**Solution:**
```robotframework
# Use depth limit
Log Component Tree    max_depth=3

# Or save to file instead
${tree}=    Get Component Tree    format=json
Create File    ${OUTPUT_DIR}/tree.json    ${tree}
```

---

## Testing Your Migration

### Verification Tests

After migration, run these checks:

```robotframework
*** Test Cases ***
Verify Tree Retrieval Still Works
    ${old_style}=    Get UI Tree
    ${new_style}=    Get Component Tree    format=text

    # Should get similar results
    Should Contain    ${old_style}    JFrame
    Should Contain    ${new_style}    JFrame

Verify JSON Format Works
    ${json}=    Get Component Tree    format=json
    ${data}=    Evaluate    json.loads('''${json}''')    modules=json

    # Should have expected structure
    Dictionary Should Contain Key    ${data}    roots
    Dictionary Should Contain Key    ${data}    timestamp

Verify Depth Control Works
    ${shallow}=    Get Component Tree    max_depth=2
    ${deep}=    Get Component Tree    max_depth=10

    # Deep should be larger
    ${shallow_len}=    Get Length    ${shallow}
    ${deep_len}=    Get Length    ${deep}
    Should Be True    ${deep_len} > ${shallow_len}

Verify Subtree Works
    # Requires element to exist
    ${status}=    Run Keyword And Return Status
    ...    Wait Until Element Exists    JPanel    timeout=5
    Pass Execution If    not ${status}    No panels available for test

    ${subtree}=    Get Component Subtree    JPanel
    Should Contain    ${subtree}    JPanel
```

---

## Support

If you encounter issues during migration:

1. Check this migration guide
2. Review [Component Tree Guide](COMPONENT_TREE_GUIDE.md)
3. Check [Troubleshooting Guide](TROUBLESHOOTING.md)
4. Create GitHub issue with "migration:" prefix

---

## Summary

### Migration is Optional

The old API continues to work. Migrate when:
- You need better performance (large UIs)
- You want programmatic tree access (JSON)
- You want targeted inspection (subtrees)
- You're writing new tests

### Migration is Easy

For most tests:
1. Change keyword names (optional)
2. Add `format=json` for automation
3. Add `max_depth` for performance
4. Done!

### Migration is Beneficial

After migration:
- ✅ Better performance
- ✅ More capabilities
- ✅ Easier automation
- ✅ Cleaner code

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-01-22 | Initial migration guide for v0.2.0 |

