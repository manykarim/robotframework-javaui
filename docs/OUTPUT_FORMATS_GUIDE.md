# Output Formats Guide

Complete guide to all output formats supported by `Get Component Tree`.

## Supported Formats

The `Get Component Tree` keyword supports 6 output formats:

1. **JSON** - Structured data format (default)
2. **XML** - Hierarchical markup format
3. **YAML** - Human-readable structured format
4. **CSV** - Flattened tabular format
5. **Markdown** - Documentation-friendly format
6. **Text** - Simple indented text

All formats support the same filtering options and represent the same underlying component tree data.

## Format Specifications

### JSON Format

**Usage:**
```robot
${tree}=    Get Component Tree    format=json
```

**Features:**
- Structured hierarchical representation
- Full component details
- Easy to parse programmatically
- Default format

**Example Output:**
```json
{
  "roots": [
    {
      "id": {
        "tree_path": "0",
        "hash_code": 12345
      },
      "component_type": {
        "class_name": "javax.swing.JFrame",
        "simple_name": "JFrame"
      },
      "identity": {
        "name": "MainWindow",
        "text": "Test Application"
      },
      "state": {
        "visible": true,
        "enabled": true,
        "showing": true,
        "focusable": true
      },
      "geometry": {
        "bounds": {
          "x": 0,
          "y": 0,
          "width": 800,
          "height": 600
        }
      },
      "children": [
        {
          "component_type": {
            "simple_name": "JButton"
          },
          "identity": {
            "name": "loginButton",
            "text": "Login"
          }
        }
      ]
    }
  ]
}
```

### XML Format

**Usage:**
```robot
${tree}=    Get Component Tree    format=xml
```

**Features:**
- Standard XML structure
- Self-documenting with attributes
- Compatible with XML tools
- Automatic escaping of special characters

**Example Output:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<uitree>
  <component type="JFrame" name="MainWindow" text="Test Application"
             enabled="true" visible="true">
    <component type="JButton" name="loginButton" text="Login"
               enabled="true" visible="true" />
    <component type="JTextField" name="usernameField" text=""
               enabled="true" visible="true" />
  </component>
</uitree>
```

### YAML Format

**Usage:**
```robot
${tree}=    Get Component Tree    format=yaml
${tree}=    Get Component Tree    format=yml    # Alias
```

**Features:**
- Human-readable structured format
- Easier to read than JSON
- Supports comments (if manually edited)
- Block-style lists for clarity

**Example Output:**
```yaml
roots:
  - id:
      tree_path: "0"
      hash_code: 12345
    component_type:
      class_name: javax.swing.JFrame
      simple_name: JFrame
    identity:
      name: MainWindow
      text: Test Application
    state:
      visible: true
      enabled: true
      showing: true
      focusable: true
    geometry:
      bounds:
        x: 0
        y: 0
        width: 800
        height: 600
    children:
      - component_type:
          simple_name: JButton
        identity:
          name: loginButton
          text: Login
```

**Performance:**
- YAML formatting adds <5ms overhead vs JSON
- Suitable for medium-sized trees (<1000 components)

### CSV Format

**Usage:**
```robot
${tree}=    Get Component Tree    format=csv
```

**Features:**
- Flattened tabular representation
- Excel-compatible
- Easy data analysis
- Path column shows hierarchy
- Depth column for tree structure

**Column Schema:**
- `path` - Tree path (e.g., "0", "0.0", "0.1.2")
- `depth` - Nesting level (0 for root)
- `type` - Component type (simple name)
- `name` - Component name
- `text` - Component text/label
- `visible` - Visibility state
- `enabled` - Enabled state
- `bounds_x`, `bounds_y` - Position
- `bounds_width`, `bounds_height` - Size

**Example Output:**
```csv
path,depth,type,name,text,visible,enabled,bounds_x,bounds_y,bounds_width,bounds_height
0,0,JFrame,MainWindow,Test Application,true,true,0,0,800,600
0.0,1,JButton,loginButton,Login,true,true,10,10,100,30
0.1,1,JTextField,usernameField,,true,true,10,50,200,25
```

**Special Character Handling:**
- Commas in text are properly escaped
- Quotes are doubled ("") per CSV spec
- Newlines are escaped as `\n`

**Use Cases:**
- Export to Excel for analysis
- Import into databases
- Automated data processing
- Generating reports

**Performance:**
- CSV formatting adds <5ms overhead vs JSON
- Efficient for large trees (1000+ components in <50ms)

### Markdown Format

**Usage:**
```robot
${tree}=    Get Component Tree    format=markdown
${tree}=    Get Component Tree    format=md        # Alias
```

**Features:**
- Human-readable hierarchical lists
- Visual badges for state
- Inline component details
- GitHub/documentation friendly
- Different list markers per level

**Example Output:**
```markdown
# UI Component Tree

- **JFrame** `MainWindow` - 👁️ visible ✅ enabled
  - *Text:* `Test Application`
  - *Bounds:* `800×600` at `(0, 0)`
  - **JButton** `loginButton` - 👁️ visible ✅ enabled
    - *Text:* `Login`
    - *Bounds:* `100×30` at `(10, 10)`
  - **JTextField** `usernameField` - 👁️ visible ✅ enabled
    - *Bounds:* `200×25` at `(10, 50)`
```

**Visual Elements:**
- Component type in **bold**
- Component name in `code`
- State badges with emojis:
  - 👁️ visible / 🚫 hidden
  - ✅ enabled / ❌ disabled
- Text shown as italic property
- Bounds shown with × separator

**List Markers:**
- Level 0: `-` (dash)
- Level 1: `*` (asterisk)
- Level 2: `+` (plus)
- Pattern repeats for deeper levels

**Use Cases:**
- Documentation in README files
- GitHub wikis and issues
- Developer notes
- Visual tree inspection

**Performance:**
- Markdown formatting adds <5ms overhead vs JSON
- Optimized for readability over compactness

### Text Format

**Usage:**
```robot
${tree}=    Get Component Tree    format=text
```

**Features:**
- Simple indented representation
- ASCII-friendly
- No special formatting
- Compact output

**Example Output:**
```text
[0] JFrame (MainWindow)
  [0.0] JButton (loginButton)
  [0.1] JTextField (usernameField)
```

## Format Comparison

| Format   | Hierarchical | Parseable | Human-Friendly | Size | Best For |
|----------|-------------|-----------|----------------|------|----------|
| JSON     | ✅ Yes      | ✅ Easy   | ⚠️ Medium      | Medium | APIs, automation |
| XML      | ✅ Yes      | ✅ Easy   | ⚠️ Medium      | Large | XML tools, SOAP |
| YAML     | ✅ Yes      | ✅ Easy   | ✅ Yes         | Medium | Configuration, reading |
| CSV      | ❌ Flattened | ✅ Easy   | ⚠️ Medium      | Small | Excel, analysis |
| Markdown | ✅ Yes      | ⚠️ Medium | ✅ Yes         | Large | Documentation |
| Text     | ✅ Yes      | ❌ Manual | ✅ Yes         | Small | Quick inspection |

## Using Formats with Filters

All formats work seamlessly with filtering options:

```robot
# Get only buttons in CSV format
${buttons}=    Get Component Tree    format=csv    types=JButton

# Get visible components in Markdown
${visible}=    Get Component Tree    format=markdown    visible_only=True

# Get tree with depth limit in YAML
${shallow}=    Get Component Tree    format=yaml    max_depth=2

# Filter multiple types in CSV
${inputs}=    Get Component Tree    format=csv    types=JTextField,JTextArea
```

## Performance Characteristics

### Format Overhead

All new formats (YAML, CSV, Markdown) add minimal overhead:

| Format   | Overhead vs JSON | Typical Time |
|----------|------------------|--------------|
| JSON     | 0ms (baseline)   | 10-20ms      |
| YAML     | <5ms             | 12-25ms      |
| CSV      | <5ms             | 11-23ms      |
| Markdown | <5ms             | 13-26ms      |

### Large Tree Performance

For trees with 1000+ components:

| Format   | Time (1000 components) |
|----------|------------------------|
| JSON     | 30-40ms                |
| YAML     | 32-45ms                |
| CSV      | 28-42ms                |
| Markdown | 35-48ms                |

All formats meet the requirement: **Large trees format in <50ms**

## Format Selection Guide

### Choose JSON when:
- Integrating with APIs
- Need programmatic parsing
- Minimal file size important
- Default choice for automation

### Choose XML when:
- Working with XML-based tools
- Need SOAP/web service integration
- Require XML schema validation
- Legacy system compatibility

### Choose YAML when:
- Human readability is priority
- Creating configuration templates
- Documentation that's also data
- Developers will read the output

### Choose CSV when:
- Exporting to Excel/spreadsheet
- Database import needed
- Flat data analysis required
- Generating reports

### Choose Markdown when:
- Creating documentation
- GitHub/GitLab wiki content
- Visual inspection needed
- Sharing in issues/PRs

### Choose Text when:
- Quick terminal inspection
- No parsing needed
- Minimal overhead required
- Simple debugging

## Examples

### Save Different Formats

```robot
# Save as JSON
Save UI Tree    ${OUTPUT_DIR}/tree.json    format=json

# Save as YAML
Save UI Tree    ${OUTPUT_DIR}/tree.yaml    format=yaml

# Save as CSV for Excel
Save UI Tree    ${OUTPUT_DIR}/tree.csv     format=csv

# Save as Markdown for docs
Save UI Tree    ${OUTPUT_DIR}/tree.md      format=markdown
```

### Parse and Use

```robot
# Get and parse JSON
${json_tree}=    Get Component Tree    format=json
${parsed}=       Evaluate    json.loads('''${json_tree}''')    json

# Get and parse YAML
${yaml_tree}=    Get Component Tree    format=yaml
${parsed}=       Evaluate    yaml.safe_load('''${yaml_tree}''')    yaml

# Get CSV and process
${csv_tree}=     Get Component Tree    format=csv
# Can be imported into Excel or processed with csv module
```

### Compare Components

```robot
# Get buttons in different formats
${json_buttons}=    Get Component Tree    format=json    types=JButton
${csv_buttons}=     Get Component Tree    format=csv     types=JButton
${md_buttons}=      Get Component Tree    format=md      types=JButton

# JSON for automation, CSV for reporting, Markdown for docs
```

## Advanced Usage

### Custom CSV Processing

```python
import csv
import io

# Get CSV tree
csv_tree = library.get_component_tree(format='csv')

# Parse with Python
reader = csv.DictReader(io.StringIO(csv_tree))
for row in reader:
    print(f"{row['type']}: {row['name']} at depth {row['depth']}")
```

### YAML Template Generation

```robot
# Get tree as YAML
${yaml_tree}=    Get Component Tree    format=yaml    max_depth=2

# Save as template
Create File    ${TEMPLATES}/ui_template.yaml    ${yaml_tree}

# Later: compare against template to detect UI changes
```

### Markdown Documentation

```robot
# Generate component documentation
${tree}=    Get Component Tree    format=markdown

# Add to documentation
${doc}=    Catenate    SEPARATOR=\n\n
...    # Component Tree
...
...    Current UI structure:
...
...    ${tree}

Create File    ${DOCS}/components.md    ${doc}
```

## Error Handling

### Invalid Format

```robot
# This will raise an error
${tree}=    Get Component Tree    format=invalid
# Error: Unknown format: invalid. Supported formats: json, xml, text, yaml/yml, csv, markdown/md
```

### Format Validation

The library validates format parameter case-insensitively:
- `json`, `JSON`, `Json` → All valid
- `yaml`, `YAML`, `yml`, `YML` → All valid
- `markdown`, `md`, `MARKDOWN`, `MD` → All valid

## Best Practices

1. **Use JSON for automation** - Most efficient and parseable
2. **Use CSV for reporting** - Easy Excel integration
3. **Use Markdown for docs** - Human-friendly, version-control friendly
4. **Use YAML for config** - Readable but structured
5. **Apply filters** - Reduce output size by filtering early
6. **Limit depth** - Use `max_depth` for large trees
7. **Cache results** - Store formatted output if used multiple times

## Troubleshooting

### YAML Parsing Errors

```robot
# Ensure you're using safe_load for YAML
${yaml_tree}=    Get Component Tree    format=yaml
${parsed}=       Evaluate    yaml.safe_load('''${yaml_tree}''')    yaml
```

### CSV Special Characters

```robot
# CSV automatically handles commas, quotes, newlines
${csv_tree}=    Get Component Tree    format=csv
# Text with "quotes" and, commas → Properly escaped
```

### Markdown Display

```robot
# For Robot Framework log, use Log keyword
${md_tree}=    Get Component Tree    format=markdown
Log    ${md_tree}    level=INFO    html=False
```

## See Also

- [Component Tree Documentation](COMPONENT_TREE_DOCUMENTATION_INDEX.md)
- [Filtering Guide](COMPONENT_TREE_FILTERING_GUIDE.md)
- [Performance Guide](PERFORMANCE_OPTIMIZATION_GUIDE.md)
- [API Reference](api-reference/component-tree.md)
