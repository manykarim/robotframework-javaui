# Output Formats Quick Reference

Quick reference for all `Get Component Tree` output formats.

## Format Cheat Sheet

| Format | Alias | Extension | Use Case |
|--------|-------|-----------|----------|
| `json` | - | `.json` | APIs, automation |
| `xml` | - | `.xml` | XML tools, SOAP |
| `yaml` | `yml` | `.yaml`, `.yml` | Config, readability |
| `csv` | - | `.csv` | Excel, reports |
| `markdown` | `md` | `.md` | Docs, GitHub |
| `text` | - | `.txt` | Quick view |

## Quick Examples

### Basic Usage

```robot
# JSON (default)
${tree}=    Get Component Tree

# YAML
${tree}=    Get Component Tree    format=yaml

# CSV
${tree}=    Get Component Tree    format=csv

# Markdown
${tree}=    Get Component Tree    format=markdown
```

### With Filters

```robot
# YAML with depth limit
${tree}=    Get Component Tree    format=yaml    max_depth=3

# CSV with type filter
${buttons}=    Get Component Tree    format=csv    types=JButton

# Markdown visible only
${visible}=    Get Component Tree    format=md    visible_only=True
```

### Save to File

```robot
Save UI Tree    output/tree.json    format=json
Save UI Tree    output/tree.yaml    format=yaml
Save UI Tree    output/tree.csv     format=csv
Save UI Tree    output/tree.md      format=markdown
```

## Format Output Examples

### JSON Example

```json
{
  "roots": [{
    "component_type": {"simple_name": "JFrame"},
    "identity": {"name": "MainWindow"},
    "children": [...]
  }]
}
```

### YAML Example

```yaml
roots:
  - component_type:
      simple_name: JFrame
    identity:
      name: MainWindow
    children: [...]
```

### CSV Example

```csv
path,depth,type,name,text,visible,enabled,bounds_x,bounds_y,bounds_width,bounds_height
0,0,JFrame,MainWindow,App,true,true,0,0,800,600
0.0,1,JButton,btn1,Click,true,true,10,10,100,30
```

### Markdown Example

```markdown
# UI Component Tree

- **JFrame** `MainWindow` - 👁️ visible ✅ enabled
  - *Bounds:* `800×600` at `(0, 0)`
  - **JButton** `btn1` - 👁️ visible ✅ enabled
    - *Text:* `Click`
```

## Performance

All formats add **<5ms overhead** compared to JSON.

Large trees (1000+ components) format in **<50ms**.

## Common Patterns

### Export for Analysis

```robot
# Get all buttons as CSV for Excel
${buttons}=    Get Component Tree    format=csv    types=JButton
Create File    ${OUTPUT_DIR}/buttons.csv    ${buttons}
```

### Document UI Structure

```robot
# Generate Markdown documentation
${tree}=    Get Component Tree    format=markdown    max_depth=3
Create File    ${DOCS}/ui-structure.md    ${tree}
```

### Programmatic Processing

```robot
# Get as YAML for easy parsing
${yaml_tree}=    Get Component Tree    format=yaml
${parsed}=       Evaluate    yaml.safe_load('''${yaml_tree}''')    yaml
```

## Error Messages

Invalid format:
```
Unknown format: invalid. Supported formats: json, xml, text, yaml/yml, csv, markdown/md
```

## Tips

1. **Case-insensitive**: `yaml`, `YAML`, `Yaml` all work
2. **Aliases**: `yml` = `yaml`, `md` = `markdown`
3. **Combine with filters**: All formats support all filtering options
4. **CSV is best for Excel**: Direct import, tabular format
5. **Markdown is best for docs**: GitHub/GitLab friendly
6. **YAML is best for readability**: Human-friendly structure

## See Also

- [Complete Output Formats Guide](OUTPUT_FORMATS_GUIDE.md)
- [Filtering Guide](COMPONENT_TREE_FILTERING_GUIDE.md)
- [API Reference](api-reference/component-tree.md)
