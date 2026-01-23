# Performance Optimization Guide - UI Component Tree

## Overview

The UI Component Tree feature in robotframework-swing has been optimized for performance when working with large applications. This guide explains how to use depth control and other optimizations effectively.

## Quick Reference

### Performance Targets

| Component Count | Depth | Expected Time | JSON Size |
|-----------------|-------|---------------|-----------|
| 100             | 1     | <10ms         | <100KB    |
| 100             | unlimited | <20ms     | <200KB    |
| 1000            | 1     | <20ms         | <500KB    |
| 1000            | 5     | <50ms         | <2MB      |
| 1000            | unlimited | <100ms    | <10MB     |
| 5000            | 1     | <30ms         | <1MB      |
| 5000            | 5     | <100ms        | <5MB      |
| 5000            | unlimited | <500ms    | <50MB     |

## Using max_depth Parameter

### Robot Framework Syntax

```robot
*** Test Cases ***
Quick Overview
    # Fast: Only top-level windows and immediate children
    ${tree}=    Get Component Tree    max_depth=1
    Log    ${tree}

Moderate Inspection
    # Balanced: Most UI structures within 5 levels
    ${tree}=    Get Component Tree    max_depth=5    format=text
    Log To Console    ${tree}

Deep Analysis
    # When you need complete visibility
    ${tree}=    Get Component Tree    max_depth=10    format=json
    Save UI Tree    /tmp/deep_tree.json

Full Tree (Unlimited)
    # Default behavior - cached after first fetch
    ${tree}=    Get Component Tree
    Log    ${tree}
```

### Python API

```python
from JavaGui import SwingLibrary

lib = SwingLibrary()
lib.connect_to_application("MyApp")

# Quick check - depth 1
quick_tree = lib.get_component_tree(max_depth=1, format="text")
print(quick_tree)

# Standard inspection - depth 5
standard_tree = lib.get_component_tree(max_depth=5, format="json")

# Full tree - cached
full_tree = lib.get_component_tree()  # Fetches and caches
full_tree_again = lib.get_component_tree()  # Uses cache (fast!)
```

## Performance Optimization Strategies

### 1. Choose Appropriate Depth

**Depth 1 - Quick Overview**
- **Use when**: Initial exploration, sanity checks
- **Returns**: Top-level windows + immediate children
- **Performance**: Fastest (10-30ms)
- **Example**:
  ```robot
  ${tree}=    Get Component Tree    max_depth=1    format=text
  Should Contain    ${tree}    JFrame
  ```

**Depth 5 - Standard Inspection**
- **Use when**: Most debugging and test development
- **Returns**: ~90% of typical UI structures
- **Performance**: Fast (50-100ms for large apps)
- **Example**:
  ```robot
  ${tree}=    Get Component Tree    max_depth=5    format=json
  ${data}=    Evaluate    json.loads('''${tree}''')
  ```

**Depth 10 - Deep Analysis**
- **Use when**: Complex nested structures, detailed investigation
- **Returns**: Very deep hierarchies
- **Performance**: Moderate (100-200ms for large apps)
- **Example**:
  ```robot
  ${tree}=    Get Component Tree    max_depth=10
  Save UI Tree    /tmp/detailed_tree.json
  ```

**Unlimited - Complete Tree**
- **Use when**: Comprehensive analysis, regression testing
- **Returns**: Everything (cached)
- **Performance**: First call slower, repeats fast
- **Example**:
  ```robot
  # First call - fetches everything
  ${full_tree}=    Get Component Tree    format=json

  # Second call - uses cache
  ${same_tree}=    Get Component Tree    format=json    # Fast!
  ```

### 2. Understand Caching Behavior

#### Cached Queries (Fast Repeats)
```robot
# Unlimited depth queries are cached
${tree1}=    Get Component Tree    # Fetches (100ms)
${tree2}=    Get Component Tree    # Cached (5ms)
${tree3}=    Get Component Tree    # Cached (5ms)
```

#### Non-Cached Queries (Always Fresh)
```robot
# Depth-limited queries always fetch fresh
${tree1}=    Get Component Tree    max_depth=5    # Fetches (50ms)
${tree2}=    Get Component Tree    max_depth=5    # Fetches again (50ms)
```

**Why?** Depth-limited trees are already fast, and caching them would require complex invalidation logic.

### 3. Choose Right Output Format

**Text Format** - Human readable, debugging
- Fastest to display
- Easy to read in logs
- Use for: Quick checks, manual inspection

```robot
${tree}=    Get Component Tree    format=text    max_depth=5
Log To Console    ${tree}
```

**JSON Format** - Programmatic processing
- Fastest to parse
- Best for automated tests
- Use for: Assertions, data extraction

```robot
${tree}=    Get Component Tree    format=json    max_depth=5
${data}=    Evaluate    json.loads('''${tree}''')
Should Be Equal    ${data['roots'][0]['class']}    JFrame
```

**XML Format** - Legacy tools, hierarchical view
- Slowest (conversion overhead)
- Compatible with XML tools
- Use for: Integration with XML-based tools

```robot
${tree}=    Get Component Tree    format=xml
Save UI Tree    /tmp/tree.xml    format=xml
```

### 4. Optimize for Large Applications

**Problem**: Application has 10,000+ components
```robot
# ❌ BAD: Unlimited depth on huge app (5+ seconds)
${tree}=    Get Component Tree

# ❌ BAD: Fetching full tree every test
Test One
    ${tree}=    Get Component Tree
    # ... test logic

Test Two
    ${tree}=    Get Component Tree    # Refetches unnecessarily
    # ... test logic
```

**Solution**: Use depth limiting and suite-level caching
```robot
*** Settings ***
Suite Setup    Cache UI Tree

*** Keywords ***
Cache UI Tree
    # Fetch once for entire suite
    ${SUITE_TREE}=    Get Component Tree    max_depth=5
    Set Suite Variable    ${SUITE_TREE}

*** Test Cases ***
Test One
    # Use cached suite variable
    Should Contain    ${SUITE_TREE}    JButton

Test Two
    # Same cached tree
    Should Contain    ${SUITE_TREE}    JPanel
```

## Common Use Cases

### Use Case 1: Find Available Components

**Goal**: What buttons/fields are visible?

```robot
${tree}=    Get Component Tree    max_depth=2    format=json
${data}=    Evaluate    json.loads('''${tree}''')

# Extract all button texts (Python inline)
${buttons}=    Evaluate    [c['text'] for c in find_all_by_type(${data}, 'JButton')]
Log Many    @{buttons}
```

### Use Case 2: Debug "Element Not Found" Error

**Goal**: Why can't test find my element?

```robot
*** Test Cases ***
Debug Missing Element
    # Get shallow tree first
    ${shallow}=    Get Component Tree    max_depth=3    format=text
    Log To Console    \n${shallow}

    # If not found, go deeper
    ${deeper}=    Get Component Tree    max_depth=7    format=text
    Log To Console    \n${deeper}

    # Check specific window
    ${window}=    Get Component Tree    locator=name:MainWindow    max_depth=5
    Should Contain    ${window}    myButton
```

### Use Case 3: Performance Regression Testing

**Goal**: Ensure UI doesn't become too complex

```robot
*** Test Cases ***
Check UI Complexity
    ${tree}=    Get Component Tree    max_depth=10    format=json
    ${data}=    Evaluate    json.loads('''${tree}''')

    ${component_count}=    Count Components    ${data}
    Should Be True    ${component_count} < 2000    UI too complex: ${component_count} components
```

### Use Case 4: CI/CD Integration

**Goal**: Generate tree snapshots for comparison

```robot
*** Settings ***
Suite Setup    Connect To Application
Suite Teardown    Save Tree Snapshot

*** Keywords ***
Save Tree Snapshot
    ${tree}=    Get Component Tree    max_depth=8    format=json
    ${filename}=    Set Variable    ui_tree_${SUITE_NAME}_${TEST_NAME}.json
    Save UI Tree    ${OUTPUT_DIR}/${filename}    format=json    max_depth=8
```

## Troubleshooting Performance

### Problem: Tree fetching is slow (>1 second)

**Diagnosis**:
```robot
${start}=    Get Time    epoch
${tree}=    Get Component Tree    max_depth=5
${end}=    Get Time    epoch
${duration}=    Evaluate    ${end} - ${start}
Log    Fetch took ${duration} seconds
```

**Solutions**:
1. Reduce depth: Try `max_depth=3` instead of `max_depth=10`
2. Check network: Is Java agent remote? Network latency adds overhead
3. Check app complexity: Use `max_depth=1` to see immediate structure size

### Problem: Out of memory errors

**Diagnosis**: Tree too large for available memory

**Solutions**:
```robot
# Instead of full tree:
${tree}=    Get Component Tree    max_depth=5    # Limits memory

# Or for specific window:
${tree}=    Get Component Tree    locator=name:MainWindow    max_depth=5
```

### Problem: Cache not working

**Diagnosis**:
```robot
# These should be fast (both use cache):
${tree1}=    Get Component Tree
${tree2}=    Get Component Tree

# These are always slow (don't use cache):
${tree3}=    Get Component Tree    max_depth=5
${tree4}=    Get Component Tree    max_depth=5
```

**Solution**: Depth-limited queries don't cache by design (they're already fast)

## Best Practices

### 1. Start Shallow, Go Deeper As Needed

```robot
*** Test Cases ***
Progressive Tree Inspection
    # Step 1: Quick overview
    ${shallow}=    Get Component Tree    max_depth=1
    Log    ${shallow}

    # Step 2: Found what you need? Stop here
    Run Keyword If    '${TARGET_COMPONENT}' in '''${shallow}'''
    ...    RETURN

    # Step 3: Go deeper only if necessary
    ${medium}=    Get Component Tree    max_depth=5
    Log    ${medium}
```

### 2. Use Depth in Test Development, Remove in Production

```robot
*** Test Cases ***
Develop New Test
    # During development - use depth for speed
    ${tree}=    Get Component Tree    max_depth=3    format=text
    Log To Console    ${tree}
    # Find elements, understand structure...

Production Test
    # In production - unlimited is cached and reliable
    ${tree}=    Get Component Tree    format=json
    # Assertions and logic...
```

### 3. Combine with Locators for Targeted Inspection

```robot
*** Test Cases ***
Inspect Specific Dialog
    # Instead of full tree
    ${dialog_tree}=    Get Component Tree
    ...    locator=class:JDialog
    ...    max_depth=5
    ...    format=text

    Should Contain    ${dialog_tree}    OK Button
```

## Performance Monitoring

### Measure Tree Fetch Time

```robot
*** Keywords ***
Get Tree With Timing
    [Arguments]    ${max_depth}=${NONE}
    ${start}=    Get Time    epoch
    ${tree}=    Get Component Tree    max_depth=${max_depth}    format=json
    ${end}=    Get Time    epoch
    ${duration}=    Evaluate    ${end} - ${start}
    Log    Tree fetch (depth=${max_depth}): ${duration}s
    [Return]    ${tree}

*** Test Cases ***
Benchmark Tree Fetching
    Get Tree With Timing    1
    Get Tree With Timing    5
    Get Tree With Timing    10
    Get Tree With Timing    ${NONE}    # Unlimited
```

## Advanced Techniques

### Parallel Tree Fetching (Multiple Windows)

```robot
*** Test Cases ***
Fetch Multiple Windows In Parallel
    # If your app has multiple windows, fetch trees separately
    ${main_tree}=    Get Component Tree
    ...    locator=name:MainWindow
    ...    max_depth=5

    ${dialog_tree}=    Get Component Tree
    ...    locator=name:SettingsDialog
    ...    max_depth=3

    # Both are independent, fast queries
```

### Tree Diffing for Regression

```robot
*** Settings ***
Library    Collections
Library    DiffLib

*** Test Cases ***
Compare UI Versions
    ${baseline}=    Load Tree From File    baseline_v1.0.json
    ${current}=    Get Component Tree    max_depth=10    format=json

    ${diff}=    Diff JSON Trees    ${baseline}    ${current}
    Should Be Empty    ${diff}    UI structure changed: ${diff}
```

## Summary

- **Use `max_depth=1-5`** for 90% of cases
- **Unlimited depth** is cached - use freely for comprehensive checks
- **Text format** for human inspection
- **JSON format** for automated processing
- **Shallow first, deepen as needed** for troubleshooting
- **Cache at suite level** for large apps

## See Also

- [Component Tree API Reference](./api/COMPONENT_TREE_API.md)
- [Performance Benchmark Results](./benchmarks/TREE_PERFORMANCE.md)
- [Troubleshooting Guide](./TROUBLESHOOTING.md)
