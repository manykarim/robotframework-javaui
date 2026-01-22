# Component Tree Quick Start Guide

**Time to complete:** 5 minutes

Get started with component tree inspection in Robot Framework Swing Library.

---

## What You'll Learn

- How to retrieve UI component trees
- How to use different output formats
- How to filter components
- How to optimize performance

---

## Prerequisites

- Robot Framework Swing Library installed
- Java application running with agent attached
- Basic familiarity with Robot Framework

---

## Step 1: Basic Tree Retrieval (1 minute)

### Get Your First Component Tree

```robotframework
*** Settings ***
Library    JavaGui.Swing

*** Test Cases ***
My First Component Tree
    Connect To Application    port=5678
    ${tree}=    Get Component Tree
    Log    ${tree}
```

**What you see:** Simple text output showing your UI hierarchy.

```
JFrame[name='MainWindow']
  JPanel[name='content']
    JButton[name='loginBtn']
    JTextField[name='username']
```

---

## Step 2: Try Different Formats (1 minute)

### JSON Format (For Programming)

```robotframework
${json}=    Get Component Tree    format=json
```

### XML Format (For Tools)

```robotframework
${xml}=    Get Component Tree    format=xml
```

### CSV Format (For Spreadsheets)

```robotframework
${csv}=    Get Component Tree    format=csv
```

**Quick Reference:**
- `text` - Simple, readable (default)
- `json` - Structured data
- `xml` - Standard markup
- `yaml` - Human-readable
- `csv` - Spreadsheet-ready
- `markdown` - Documentation

---

## Step 3: Control Depth (1 minute)

### Limit Tree Depth for Performance

```robotframework
# Quick overview (3 levels)
${shallow}=    Get Component Tree    max_depth=3

# Detailed view (10 levels)
${detailed}=    Get Component Tree    max_depth=10
```

**Performance Tip:** Use `max_depth=5` as a starting point. Adjust based on your needs.

| Depth | Speed | Use Case |
|-------|-------|----------|
| 2-3 | Very Fast | Quick overview |
| 5-7 | Fast | Normal debugging |
| 10+ | Slower | Detailed analysis |
| None | Slowest | Complete tree |

---

## Step 4: Filter Components (1 minute)

### By Type

```robotframework
# Get only buttons
${buttons}=    Get Component Tree    types=JButton

# Get buttons and text fields
${inputs}=    Get Component Tree    types=JButton,JTextField

# Get all button types (wildcard)
${all_buttons}=    Get Component Tree    types=J*Button
```

### By State

```robotframework
# Get visible components
${visible}=    Get Component Tree    visible_only=${True}

# Get enabled components
${enabled}=    Get Component Tree    enabled_only=${True}

# Combine filters
${active}=    Get Component Tree
...    types=JButton
...    visible_only=${True}
...    enabled_only=${True}
```

---

## Step 5: Real-World Example (1 minute)

### Debug a Locator Issue

```robotframework
*** Test Cases ***
Debug Element Locator
    [Documentation]    Can't find a button? Log the tree!

    # Try to click (might fail)
    Run Keyword And Expect Error    *
    ...    Click    JButton[name='mystery']

    # Log tree to see what's actually there
    Log Component Tree    format=text    level=INFO

    # Or save to file for analysis
    ${tree}=    Get Component Tree    format=json
    Create File    ${OUTPUT_DIR}/ui_structure.json    ${tree}
```

### Find All Clickable Buttons

```robotframework
*** Test Cases ***
Find Clickable Buttons
    ${buttons}=    Get Component Tree
    ...    types=J*Button
    ...    visible_only=${True}
    ...    enabled_only=${True}
    ...    format=json

    Log    Clickable buttons: ${buttons}
```

### Performance Optimization

```robotframework
*** Test Cases ***
Optimize Large UI
    # ❌ Slow: Full tree of 2000 components
    ${full}=    Get Component Tree    # 10 seconds!

    # ✅ Fast: Filtered subtree
    ${filtered}=    Get Component Subtree
    ...    JPanel[name='loginForm']
    ...    types=JButton,JTextField
    ...    max_depth=5    # 200ms!
```

---

## Common Patterns

### Pattern 1: Document UI Structure

```robotframework
${tree}=    Get Component Tree    format=markdown    max_depth=5
Create File    ${OUTPUT_DIR}/UI_STRUCTURE.md    ${tree}
```

### Pattern 2: Verify UI State

```robotframework
${before}=    Get Component Tree    max_depth=5
Click    JButton[text='Transform']
Refresh Component Tree
${after}=    Get Component Tree    max_depth=5
Should Not Be Equal    ${before}    ${after}
```

### Pattern 3: Count Components

```robotframework
${buttons}=    Get Component Tree    types=JButton    format=text
${line_count}=    Get Line Count    ${buttons}
Log    Found ${line_count} buttons
```

---

## Quick Command Reference

| What You Want | Command |
|---------------|---------|
| Basic tree | `Get Component Tree` |
| JSON format | `Get Component Tree    format=json` |
| Limit depth | `Get Component Tree    max_depth=5` |
| Only buttons | `Get Component Tree    types=JButton` |
| Visible only | `Get Component Tree    visible_only=${True}` |
| Subtree | `Get Component Subtree    JPanel[name='form']` |
| Log to output | `Log Component Tree` |
| Refresh cache | `Refresh Component Tree` |

---

## Troubleshooting

### Problem: Tree is too large

**Solution:** Use `max_depth` or type filtering:

```robotframework
${tree}=    Get Component Tree    max_depth=5
# OR
${tree}=    Get Component Tree    types=JButton,JTextField
```

### Problem: Components seem outdated

**Solution:** Refresh the tree:

```robotframework
Refresh Component Tree
${tree}=    Get Component Tree
```

### Problem: Can't find a component

**Solution:** Log the tree and inspect:

```robotframework
Log Component Tree    level=INFO
# Review Robot Framework log to find component names
```

### Problem: JSON parsing error

**Solution:** Verify format parameter:

```robotframework
# ✅ Correct
${json}=    Get Component Tree    format=json

# ❌ Wrong
${json}=    Get Component Tree    format=jason  # Typo!
```

---

## Next Steps

Now that you've mastered the basics, dive deeper:

1. **📚 Full Guide:** [Component Tree Guide](user-guide/COMPONENT_TREE_GUIDE.md)
   - Advanced filtering techniques
   - Performance optimization
   - Best practices
   - More examples

2. **📖 API Reference:** [Component Tree API](api-reference/COMPONENT_TREE_API.md)
   - Complete parameter documentation
   - Data structures
   - Error handling

3. **🔧 Troubleshooting:** [Troubleshooting Guide](user-guide/COMPONENT_TREE_TROUBLESHOOTING.md)
   - Common issues and solutions
   - Performance problems
   - Platform-specific issues

4. **💻 Examples:** [Example Tests](../examples/)
   - `component_tree_basic.robot` - Basic usage
   - `component_tree_advanced.robot` - Advanced patterns
   - `component_tree_filtering.robot` - Filtering examples
   - `component_tree_formats.robot` - All output formats

---

## Summary

You now know how to:

- ✅ Retrieve component trees with `Get Component Tree`
- ✅ Use different output formats (text, json, xml, yaml, csv, markdown)
- ✅ Control depth with `max_depth` parameter
- ✅ Filter by type and state
- ✅ Debug locator issues
- ✅ Optimize performance

**Total time:** ~5 minutes

---

## Quick Tips

💡 **Start simple:** Use `Get Component Tree` with defaults first

💡 **Add depth limit:** Use `max_depth=5` for faster retrieval

💡 **Filter early:** Use type filtering to reduce output size

💡 **Use JSON:** Use `format=json` for programmatic analysis

💡 **Log liberally:** `Log Component Tree` is your debugging friend

💡 **Refresh often:** Call `Refresh Component Tree` after UI changes

---

## One-Liner Cheat Sheet

```robotframework
# Basic
Get Component Tree

# With options
Get Component Tree    format=json    max_depth=5    types=JButton    visible_only=${True}

# Subtree
Get Component Subtree    JPanel[name='form']    format=json

# Log
Log Component Tree    level=INFO

# Refresh
Refresh Component Tree
```

---

**Ready to learn more?**

👉 Continue to [Component Tree Guide](user-guide/COMPONENT_TREE_GUIDE.md) for comprehensive documentation.

👉 Try the [Examples](../examples/) for hands-on practice.

👉 Need help? Check the [Troubleshooting Guide](user-guide/COMPONENT_TREE_TROUBLESHOOTING.md).

---

**Questions? Feedback?**

- 📖 [Full Documentation Index](COMPONENT_TREE_DOCUMENTATION_INDEX.md)
- 🐛 [Report Issues](https://github.com/manykarim/robotframework-javaui/issues)
- 💬 Label: `component-tree`
