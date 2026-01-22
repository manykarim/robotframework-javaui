# Component Tree Quick Reference Card

**Version:** 0.2.0 | **Library:** JavaGui.Swing/Swt/Rcp

---

## Quick Syntax

### Get Full Tree
```robotframework
${tree}=    Get Component Tree                                    # Text format
${json}=    Get Component Tree    format=json                     # JSON format
${tree}=    Get Component Tree    max_depth=5                     # Depth limit
${tree}=    Get Component Tree    format=json    max_depth=5      # Combined
```

### Advanced Filtering (NEW in 0.2.0)
```robotframework
# Type filtering
${btns}=    Get Component Tree    types=JButton                   # Only buttons
${inputs}=  Get Component Tree    types=JButton,JTextField        # Multiple types
${all_btns}= Get Component Tree   types=J*Button                  # Wildcard match
${no_labels}= Get Component Tree  exclude_types=JLabel,JPanel     # Exclude types

# State filtering
${visible}= Get Component Tree    visible_only=${True}            # Only visible
${enabled}= Get Component Tree    enabled_only=${True}            # Only enabled
${focus}=   Get Component Tree    focusable_only=${True}          # Only focusable

# Combined filtering
${active_btns}= Get Component Tree    types=JButton
...    visible_only=${True}    enabled_only=${True}
```

### Get Subtree
```robotframework
${sub}=     Get Component Subtree    JPanel[name='main']          # From component
${sub}=     Get Component Subtree    locator=JPanel    format=json
${sub}=     Get Component Subtree    JDialog    max_depth=3       # With depth
```

### Log Tree
```robotframework
Log Component Tree                                                # INFO level, text
Log Component Tree    level=DEBUG                                 # DEBUG level
Log Component Tree    format=json    level=INFO                   # JSON, INFO
Log Component Tree    locator=JPanel[name='main']                 # Subtree
```

### Refresh
```robotframework
Refresh Component Tree                                            # Force refresh
```

---

## Parameters Reference

| Keyword | locator | format | max_depth | types | exclude_types | visible_only | enabled_only | focusable_only | level |
|---------|---------|--------|-----------|-------|---------------|--------------|--------------|----------------|-------|
| Get Component Tree | Optional | Optional | Optional | Optional | Optional | Optional | Optional | Optional | - |
| Get UI Tree | - | Optional | Optional | Optional | Optional | Optional | Optional | Optional | - |
| Get Component Subtree | **Required** | Optional | Optional | - | - | - | - | - | - |
| Log Component Tree | Optional | Optional | - | - | - | - | - | - | Optional |
| Refresh Component Tree | - | - | - | - | - | - | - | - | - |

---

## Format Values

| Format | Output | Best For |
|--------|--------|----------|
| `text` | Human-readable tree | Debugging, logs |
| `json` | JSON with all properties | Programmatic parsing |
| `yaml` | YAML format | *(Planned)* Configuration |

---

## Depth Guidelines

| max_depth | Components | Time | Use For |
|-----------|-----------|------|---------|
| 2 | ~10-20 | <100ms | Quick overview |
| 5 | ~100-200 | ~500ms | Panel structure |
| 10 | ~500-1000 | ~2s | Detailed inspection |
| None | All | Variable | Full analysis |

---

## Log Levels

| Level | Visibility | Use For |
|-------|-----------|----------|
| `TRACE` | TRACE mode only | Very detailed debug |
| `DEBUG` | --loglevel DEBUG | Debugging |
| `INFO` | Always visible | Documentation |
| `WARN` | Highlighted | Important notes |
| `ERROR` | Red in log | Problems |

---

## JSON Structure

```json
{
  "roots": [
    {
      "id": 1,
      "class": "javax.swing.JFrame",
      "simpleClass": "JFrame",
      "name": "MainWindow",
      "text": "App Title",
      "x": 100, "y": 100,
      "width": 800, "height": 600,
      "visible": true,
      "enabled": true,
      "showing": true,
      "children": [...]
    }
  ],
  "timestamp": 1737532800000
}
```

### Parse JSON
```robotframework
${json}=    Get Component Tree    format=json
${data}=    Evaluate    json.loads('''${json}''')    modules=json
${roots}=   Get From Dictionary    ${data}    roots
${root}=    Get From List    ${roots}    0
```

---

## Common Patterns

### Pattern 1: Debug Element
```robotframework
Log Component Tree    level=INFO
# Find element in log, then use it
Click    JButton[name='foundName']
```

### Pattern 1a: Filter While Debugging
```robotframework
# Only see buttons to find the right one
${btns}=    Get Component Tree    types=JButton    format=text
Log    ${btns}
# Only see visible, enabled components
${active}=  Get Component Tree    visible_only=${True}    enabled_only=${True}
```

### Pattern 2: Progressive Inspection
```robotframework
${overview}=    Get Component Tree    max_depth=2
${detail}=      Get Component Subtree    JPanel[name='target']    max_depth=10
```

### Pattern 3: State Comparison
```robotframework
${before}=    Get Component Tree    max_depth=5
# Perform action
Refresh Component Tree
${after}=     Get Component Tree    max_depth=5
```

### Pattern 4: Save to File
```robotframework
${json}=    Get Component Tree    format=json
Create File    ${OUTPUT_DIR}/tree.json    ${json}
```

### Pattern 5: Count Components
```robotframework
${json}=    Get Component Tree    format=json
${data}=    Evaluate    json.loads('''${json}''')    modules=json
# Use custom keyword to count
${count}=   Count Components By Type    ${data['roots'][0]}    JButton
```

---

## Error Quick Fixes

| Error | Quick Fix |
|-------|-----------|
| Tree too slow | Add `max_depth=5` |
| Tree empty | Call `Refresh Component Tree` |
| JSON parse error | Use triple quotes: `'''${json}'''` |
| Element not found | Use `Log Component Tree` to see what exists |
| EDT timeout | Increase library timeout |

---

## Locator Syntax

```robotframework
# By type
Get Component Subtree    JButton

# By name
Get Component Subtree    [name='submitBtn']

# Type + name
Get Component Subtree    JButton[name='submitBtn']

# With attribute
Get Component Subtree    JButton[text='Submit']

# Pseudo-selector
Get Component Subtree    JButton:enabled

# XPath-style
Get Component Subtree    //JButton[@name='submit']
```

---

## Platform Support

| Feature | Swing | SWT | RCP |
|---------|-------|-----|-----|
| Get Component Tree | ✅ | ⚠️ | ⚠️ |
| Get Component Subtree | ✅ | ⚠️ | ❌ |
| JSON format | ✅ | ✅ | ⚠️ |
| Depth control | ✅ | ⚠️ | ⚠️ |

✅ Full support | ⚠️ Partial | ❌ Not available

---

## Best Practices

1. ✅ **Use depth limits** for large UIs
2. ✅ **Use subtrees** instead of full tree when possible
3. ✅ **Refresh** after UI changes
4. ✅ **Use JSON** for programmatic access
5. ✅ **Use DEBUG level** for detailed logs
6. ❌ **Don't** use in tight loops
7. ❌ **Don't** forget to refresh after changes

---

## Migration from v0.1.x

| Old | New | Notes |
|-----|-----|-------|
| `Get UI Tree` | `Get Component Tree` | No breaking change |
| `Log UI Tree` | `Log Component Tree` | No breaking change |
| `Refresh UI Tree` | `Refresh Component Tree` | No breaking change |

*Old keywords still work! New keywords add features.*

---

## Quick Links

- 📚 [Full Guide](user-guide/COMPONENT_TREE_GUIDE.md)
- 🔍 [Filtering Guide](COMPONENT_TREE_FILTERING_GUIDE.md) **NEW**
- 🔄 [Migration Guide](user-guide/COMPONENT_TREE_MIGRATION_GUIDE.md)
- 🔧 [Troubleshooting](user-guide/COMPONENT_TREE_TROUBLESHOOTING.md)
- 📖 [API Reference](api-reference/COMPONENT_TREE_API.md)
- 💡 [Examples](../examples/)

---

## Need Help?

1. Check [Troubleshooting Guide](user-guide/COMPONENT_TREE_TROUBLESHOOTING.md)
2. Review [Examples](../examples/component_tree_basic.robot)
3. Search [Documentation Index](COMPONENT_TREE_DOCUMENTATION_INDEX.md)
4. Create GitHub Issue

---

**Print this page for quick reference while coding!**

