# User Performance Guide

## Quick Reference

### Performance at a Glance

| UI Size | Tree Retrieval | Best Practice |
|---------|----------------|---------------|
| Small (<100) | <1ms | Any approach works |
| Medium (100-500) | 2-5ms | Use depth limiting |
| Large (500-1000) | 5-10ms | Depth limiting + caching |
| Very Large (>1000) | >10ms | Critical: depth limiting |

### Common Operations Performance

| Operation | Typical Time | Notes |
|-----------|--------------|-------|
| Get full tree (1000 components) | 5ms | Fast enough for most cases |
| Get tree depth 3 | 2ms | 2.5x faster |
| Get tree depth 1 | 500µs | 10x faster |
| Find single component | 2ms | Searches entire tree |
| Cache lookup | 50ns | Very fast |
| JSON serialization | 3ms | Fastest format |

## Optimization Techniques

### 1. Use Depth Limiting

**Impact:** 2-10x speedup

Limit tree depth to only what you need:

```robot
*** Test Cases ***
Fast Window Enumeration
    # Only get top-level windows (10x faster)
    ${windows}=    Get Component Tree    max_depth=1

Fast Form Inspection
    # Get form and its immediate children (2.5x faster)
    ${form}=    Get Component Tree    max_depth=3

Full Tree When Needed
    # Only use unlimited depth when absolutely necessary
    ${tree}=    Get Component Tree    max_depth=unlimited
```

**When to use each depth:**

- **Depth 1:** Window/frame list, quick overview
- **Depth 2-3:** Form inspection, button finding
- **Depth 5:** Complex nested panels
- **Unlimited:** Full application structure analysis

### 2. Cache Tree Results

**Impact:** Eliminate repeated tree builds

```robot
*** Test Cases ***
Bad Practice - Repeated Fetching
    # Fetches tree 10 times (50ms total)
    FOR    ${i}    IN RANGE    10
        ${tree}=    Get Component Tree
        ${count}=    Count Components    ${tree}
    END

Good Practice - Cache Results
    # Fetches once, reuses (6ms total - 8x faster)
    ${tree}=    Get Component Tree
    FOR    ${i}    IN RANGE    10
        ${count}=    Count Components    ${tree}
    END

Best Practice - Suite-Level Caching
    # Cache at suite level for even better performance
    ${SUITE_TREE}=    Get Component Tree
    Set Suite Variable    ${SUITE_TREE}
```

**When to refresh cache:**

✅ **DO refresh after:**
- Dialog opens or closes
- Major UI state changes
- Window switches

❌ **DON'T refresh after:**
- Button clicks
- Text input
- Individual component updates

### 3. Use Targeted Queries

**Impact:** 2-3x speedup for finding specific components

```robot
*** Test Cases ***
Slow Approach - Get Full Tree Then Filter
    # Takes 7ms: 5ms (tree) + 2ms (filter)
    ${tree}=    Get Component Tree
    ${buttons}=    Filter Components    ${tree}    class=JButton

Fast Approach - Direct Query
    # Takes 2ms: only finds what's needed
    ${buttons}=    Find All Components    class=JButton

Fastest Approach - Single Component
    # Takes <1ms: stops at first match
    ${login_btn}=    Find Component    text=Login
```

**Choose the right tool:**

| Goal | Use | Time |
|------|-----|------|
| Find one component | `Find Component` | 1-2ms |
| Find multiple similar | `Find All Components` | 2-3ms |
| Analyze structure | `Get Component Tree` | 5ms |

### 4. Choose Efficient Formats

**Impact:** 2-3x speedup for serialization

```robot
*** Test Cases ***
Fast - JSON Format (Default)
    ${tree}=    Get Component Tree    format=json    # 3ms

Medium - Text Format
    ${tree}=    Get Component Tree    format=text    # 5ms

Slow - YAML Format
    ${tree}=    Get Component Tree    format=yaml    # 10ms
```

**Format selection guide:**

- **JSON:** Fastest, best for programmatic access
- **Text:** Good for logging and debugging
- **XML:** When XML parsing is required
- **YAML:** Only for human-readable reports
- **Markdown:** Documentation and reports
- **CSV:** Tabular data analysis

### 5. Optimize Component Finding

**Impact:** Up to 5x speedup

```robot
*** Test Cases ***
Slow - Search From Root
    # Searches entire application (5ms)
    ${field}=    Find Component    class=JTextField

Fast - Search Within Parent
    # Limits search scope (1ms - 5x faster)
    ${dialog}=    Find Component    text=Settings
    ${field}=    Find Component    class=JTextField    parent=${dialog}

Fastest - Use Unique Identifiers
    # Direct cache lookup (50ns - 100,000x faster)
    ${field}=    Find Component    name=username_field
```

**Finding strategy:**

1. **Best:** Use unique `name` attribute (50ns)
2. **Good:** Search within parent container (1ms)
3. **OK:** Search by text with parent (2ms)
4. **Slow:** Search by class from root (5ms)

## Performance Patterns

### Pattern 1: Dialog Interaction

```robot
*** Test Cases ***
Optimized Dialog Test
    # Get dialog once
    ${dialog}=    Find Component    text=Settings Dialog

    # All operations use cached dialog reference
    ${ok_btn}=    Find Component    text=OK    parent=${dialog}
    ${cancel_btn}=    Find Component    text=Cancel    parent=${dialog}
    ${field}=    Find Component    class=JTextField    parent=${dialog}

    # Fast because we're not rebuilding tree
    Input Text    ${field}    test value
    Click Button    ${ok_btn}
```

### Pattern 2: Form Validation

```robot
*** Test Cases ***
Fast Form Validation
    # Build tree once with depth limit
    ${form}=    Get Component Tree    max_depth=2

    # Extract all fields from cached tree
    ${fields}=    Get All Fields From Tree    ${form}

    # Validate without rebuilding
    FOR    ${field}    IN    @{fields}
        Validate Field    ${field}
    END
```

### Pattern 3: Batch Operations

```robot
*** Test Cases ***
Efficient Batch Processing
    # Get all components of interest once
    ${buttons}=    Find All Components    class=JButton

    # Process them without repeated queries
    FOR    ${button}    IN    @{buttons}
        ${text}=    Get Component Property    ${button}    text
        Log    Button: ${text}
        Should Not Be Empty    ${text}
    END
```

## Performance Anti-Patterns

### ❌ Anti-Pattern 1: Repeated Tree Fetching

```robot
*** Test Cases ***
BAD EXAMPLE - Don't Do This!
    # Rebuilds tree 100 times (500ms total)
    FOR    ${i}    IN RANGE    100
        ${tree}=    Get Component Tree
        Should Not Be Empty    ${tree}
    END

GOOD EXAMPLE - Do This Instead
    # Builds tree once (5ms total)
    ${tree}=    Get Component Tree
    FOR    ${i}    IN RANGE    100
        Should Not Be Empty    ${tree}
    END
```

**Cost:** 100x slower

### ❌ Anti-Pattern 2: Deep Inspection When Shallow Suffices

```robot
*** Test Cases ***
BAD EXAMPLE
    # Gets full tree when only windows needed (5ms)
    ${tree}=    Get Component Tree
    ${window_count}=    Count Top Level Windows    ${tree}

GOOD EXAMPLE
    # Limits depth to what's needed (500µs - 10x faster)
    ${tree}=    Get Component Tree    max_depth=1
    ${window_count}=    Count Top Level Windows    ${tree}
```

**Cost:** 10x slower

### ❌ Anti-Pattern 3: Full Tree for Single Component

```robot
*** Test Cases ***
BAD EXAMPLE
    # Builds full tree to find one button (5ms)
    ${tree}=    Get Component Tree
    ${button}=    Find In Tree    ${tree}    text=Login

GOOD EXAMPLE
    # Directly finds component (1ms - 5x faster)
    ${button}=    Find Component    text=Login
```

**Cost:** 5x slower

### ❌ Anti-Pattern 4: Inefficient Format Choice

```robot
*** Test Cases ***
BAD EXAMPLE
    # Uses slow YAML when JSON would work (10ms)
    ${tree}=    Get Component Tree    format=yaml
    ${data}=    Parse YAML    ${tree}

GOOD EXAMPLE
    # Uses fast JSON (3ms - 3x faster)
    ${tree}=    Get Component Tree    format=json
    ${data}=    Parse JSON    ${tree}
```

**Cost:** 3x slower

## Benchmarking Your Tests

### Measure Performance

```robot
*** Test Cases ***
Measure Tree Performance
    # Warmup
    Get Component Tree

    # Measure
    ${start}=    Get Time    epoch
    FOR    ${i}    IN RANGE    100
        Get Component Tree
    END
    ${end}=    Get Time    epoch
    ${duration}=    Evaluate    ${end} - ${start}
    ${per_call}=    Evaluate    ${duration} / 100

    Log    Average time: ${per_call}s per call
    Should Be True    ${per_call} < 0.01    Tree fetch should be <10ms
```

### Profile Your Test Suite

```robot
*** Settings ***
Library    Collections
Library    OperatingSystem

*** Test Cases ***
Profile Tree Operations
    ${metrics}=    Create Dictionary

    # Profile full tree
    ${time}=    Time Operation    Get Component Tree
    Set To Dictionary    ${metrics}    full_tree=${time}

    # Profile depth 3
    ${time}=    Time Operation    Get Component Tree    max_depth=3
    Set To Dictionary    ${metrics}    depth_3=${time}

    # Profile depth 1
    ${time}=    Time Operation    Get Component Tree    max_depth=1
    Set To Dictionary    ${metrics}    depth_1=${time}

    # Report
    Log    Performance Profile: ${metrics}

*** Keywords ***
Time Operation
    [Arguments]    ${keyword}    @{args}
    ${start}=    Get Time    epoch
    Run Keyword    ${keyword}    @{args}
    ${end}=    Get Time    epoch
    ${duration}=    Evaluate    ${end} - ${start}
    [Return]    ${duration}
```

## Performance Checklist

Use this checklist to optimize your test suite:

- [ ] Use depth limiting when full tree not needed
- [ ] Cache tree results within test cases
- [ ] Use `Find Component` instead of `Get Component Tree` for lookups
- [ ] Search within parent containers when possible
- [ ] Use JSON format unless human-readable output required
- [ ] Set unique `name` attributes on important components
- [ ] Refresh cache only when UI structure changes
- [ ] Profile slow tests to identify bottlenecks
- [ ] Use batch operations to minimize tree rebuilds
- [ ] Monitor test execution time and set thresholds

## Troubleshooting Performance Issues

### Issue: Tree retrieval is slow (>100ms)

**Diagnosis:**
1. Check component count: `${count}=    Count Components    ${tree}`
2. Measure with depth limiting: `${tree}=    Get Component Tree    max_depth=3`
3. Check for custom components with expensive properties

**Solutions:**
- Use depth limiting
- Optimize custom component properties
- Consider caching at suite level

### Issue: Tests are slow overall

**Diagnosis:**
1. Profile test execution time
2. Identify tests with repeated tree fetches
3. Look for anti-patterns (see above)

**Solutions:**
- Apply caching patterns
- Use targeted queries
- Optimize depth limits

### Issue: Memory usage is high

**Diagnosis:**
1. Check cache size: Monitor component cache growth
2. Look for leaking component references
3. Check for unnecessary tree storage

**Solutions:**
- Clear cache periodically
- Limit tree depth to reduce size
- Don't store trees in global variables

### Issue: Inconsistent performance

**Diagnosis:**
1. Run multiple iterations to get stable measurements
2. Check for EDT thread contention
3. Monitor garbage collection

**Solutions:**
- Warmup before measuring
- Ensure EDT is idle
- Increase JVM heap size if needed

## Advanced Topics

### Custom Performance Monitoring

```robot
*** Settings ***
Library    PerformanceMonitor.py

*** Test Cases ***
Monitor Suite Performance
    Start Performance Monitoring

    # Your tests here
    Run Test Suite

    ${report}=    Get Performance Report
    Log    ${report}
    Should Be True    ${report['avg_tree_time']} < 0.01
```

### Performance Regression Testing

Add performance assertions to catch regressions:

```robot
*** Test Cases ***
Performance Regression Test
    # Baseline: 5ms for 1000 components
    ${tree}=    Get Component Tree
    ${count}=    Count Components    ${tree}

    # Allow 20% variation
    ${time}=    Measure Tree Time
    ${max_allowed}=    Evaluate    0.005 * (${count} / 1000) * 1.2

    Should Be True    ${time} < ${max_allowed}
    ...    Tree build time ${time}s exceeds threshold ${max_allowed}s
```

## Summary

### Quick Wins (Easy, High Impact)

1. **Add depth limiting** - 2-10x speedup, 1 line change
2. **Cache in test case** - 5-10x speedup, 2 line change
3. **Use Find Component** - 2-3x speedup, 1 line change

### Best Practices

1. Depth limit to minimum required
2. Cache and reuse tree results
3. Use targeted queries over full tree
4. Search within parent containers
5. Choose JSON format when possible
6. Set unique names on components
7. Refresh cache only when needed

### Performance Targets

| Operation | Target | Status |
|-----------|--------|--------|
| Small UI (<100) | <1ms | ✅ Excellent |
| Medium UI (100-500) | <5ms | ✅ Excellent |
| Large UI (500-1000) | <10ms | ✅ Excellent |
| Very Large UI (>1000) | <50ms | ✅ Good |

---

**Document Version:** 1.0.0
**Last Updated:** 2026-01-22
**Feedback:** Please report performance issues to the project repository
