# Output Format Examples for `Get Component Tree`

This document provides examples of all supported output formats for the `Get Component Tree` keyword.

## Sample UI Hierarchy

For these examples, we'll use a simple login dialog with the following structure:

```
JFrame "Login" (800×600)
├── JPanel "contentPane" (800×600)
│   ├── JLabel "titleLabel" - Text: "Welcome"
│   ├── JTextField "usernameField" (200×25)
│   ├── JPasswordField "passwordField" (200×25)
│   └── JButton "loginButton" - Text: "Login"
```

## 1. JSON Format (Default)

**Usage:**
```robotframework
${tree}=    Get Component Tree    format=json
```

**Output:**
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
        "name": "LoginFrame",
        "text": "Login"
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
        },
        "screen_location": null
      },
      "children": [
        {
          "id": {
            "tree_path": "0.0",
            "hash_code": 12346
          },
          "component_type": {
            "class_name": "javax.swing.JPanel",
            "simple_name": "JPanel"
          },
          "identity": {
            "name": "contentPane",
            "text": null
          },
          "state": {
            "visible": true,
            "enabled": true,
            "showing": true,
            "focusable": false
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
              "id": {
                "tree_path": "0.0.0",
                "hash_code": 12347
              },
              "component_type": {
                "simple_name": "JLabel"
              },
              "identity": {
                "name": "titleLabel",
                "text": "Welcome"
              },
              "state": {
                "visible": true,
                "enabled": true
              },
              "geometry": {
                "bounds": {
                  "x": 10,
                  "y": 10,
                  "width": 200,
                  "height": 30
                }
              }
            },
            {
              "id": {
                "tree_path": "0.0.1",
                "hash_code": 12348
              },
              "component_type": {
                "simple_name": "JTextField"
              },
              "identity": {
                "name": "usernameField",
                "text": ""
              },
              "geometry": {
                "bounds": {
                  "x": 10,
                  "y": 50,
                  "width": 200,
                  "height": 25
                }
              }
            },
            {
              "id": {
                "tree_path": "0.0.2",
                "hash_code": 12349
              },
              "component_type": {
                "simple_name": "JPasswordField"
              },
              "identity": {
                "name": "passwordField"
              },
              "geometry": {
                "bounds": {
                  "x": 10,
                  "y": 85,
                  "width": 200,
                  "height": 25
                }
              }
            },
            {
              "id": {
                "tree_path": "0.0.3",
                "hash_code": 12350
              },
              "component_type": {
                "simple_name": "JButton"
              },
              "identity": {
                "name": "loginButton",
                "text": "Login"
              },
              "geometry": {
                "bounds": {
                  "x": 10,
                  "y": 120,
                  "width": 100,
                  "height": 30
                }
              }
            }
          ]
        }
      ]
    }
  ],
  "timestamp": 1706345678901,
  "metadata": {
    "total_components": 6
  }
}
```

**Best for:** Programmatic parsing, integration with JSON tools, preserving complete data structure.

---

## 2. XML Format

**Usage:**
```robotframework
${tree}=    Get Component Tree    format=xml
Save UI Tree    ${OUTPUT_DIR}/ui_tree.xml    format=xml
```

**Output:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<uitree>
  <component type="JFrame" name="LoginFrame" text="Login" enabled="true" visible="true">
    <component type="JPanel" name="contentPane" text="" enabled="true" visible="true">
      <component type="JLabel" name="titleLabel" text="Welcome" enabled="true" visible="true" />
      <component type="JTextField" name="usernameField" text="" enabled="true" visible="true" />
      <component type="JPasswordField" name="passwordField" text="" enabled="true" visible="true" />
      <component type="JButton" name="loginButton" text="Login" enabled="true" visible="true" />
    </component>
  </component>
</uitree>
```

**Features:**
- Hierarchical structure using nested elements
- Self-closing tags for leaf components
- Automatic escaping of special characters (`<`, `>`, `&`, `"`)
- Compatible with XPath queries
- Can be validated with XML Schema

**Best for:** XML processing tools, XPath queries, integration with XML-based systems.

---

## 3. YAML Format

**Usage:**
```robotframework
${tree}=    Get Component Tree    format=yaml
# or
${tree}=    Get Component Tree    format=yml
```

**Output:**
```yaml
roots:
  - id:
      tree_path: '0'
      hash_code: 12345
    component_type:
      class_name: javax.swing.JFrame
      simple_name: JFrame
    identity:
      name: LoginFrame
      text: Login
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
      screen_location: null
    children:
      - id:
          tree_path: '0.0'
          hash_code: 12346
        component_type:
          simple_name: JPanel
        identity:
          name: contentPane
          text: null
        children:
          - id:
              tree_path: '0.0.0'
            component_type:
              simple_name: JLabel
            identity:
              name: titleLabel
              text: Welcome
            geometry:
              bounds:
                x: 10
                y: 10
                width: 200
                height: 30
          - id:
              tree_path: '0.0.1'
            component_type:
              simple_name: JTextField
            identity:
              name: usernameField
              text: ''
          - id:
              tree_path: '0.0.2'
            component_type:
              simple_name: JPasswordField
            identity:
              name: passwordField
          - id:
              tree_path: '0.0.3'
            component_type:
              simple_name: JButton
            identity:
              name: loginButton
              text: Login
timestamp: 1706345678901
metadata:
  total_components: 6
```

**Features:**
- Human-readable hierarchical format
- Clean, minimal syntax
- Supports both `.yaml` and `.yml` extensions
- Case-insensitive format parameter

**Best for:** Configuration files, human-readable data exchange, documentation.

---

## 4. CSV Format (Flattened)

**Usage:**
```robotframework
${tree}=    Get Component Tree    format=csv
Save UI Tree    ${OUTPUT_DIR}/ui_tree.csv    format=csv
```

**Output:**
```csv
path,depth,type,name,text,visible,enabled,bounds_x,bounds_y,bounds_width,bounds_height
0,0,JFrame,LoginFrame,Login,true,true,0,0,800,600
0.0,1,JPanel,contentPane,,true,true,0,0,800,600
0.0.0,2,JLabel,titleLabel,Welcome,true,true,10,10,200,30
0.0.1,2,JTextField,usernameField,,true,true,10,50,200,25
0.0.2,2,JPasswordField,passwordField,,true,true,10,85,200,25
0.0.3,2,JButton,loginButton,Login,true,true,10,120,100,30
```

**Features:**
- Flattened hierarchy with `depth` column
- Path notation for tree relationships (e.g., `0.0.1` is child of `0.0`)
- Excel-compatible
- UTF-8 encoding for international characters
- Automatic escaping of:
  - Quotes (doubled: `"text with ""quotes"""`)
  - Commas (quoted: `"text, with, commas"`)
  - Newlines (escaped: `\n`)

**Column Definitions:**
- `path`: Hierarchical path (e.g., `0.0.1`)
- `depth`: Nesting level (0 = root)
- `type`: Component type (simple name)
- `name`: Component name attribute
- `text`: Display text or value
- `visible`: Visibility state
- `enabled`: Enabled state
- `bounds_x`, `bounds_y`: Position
- `bounds_width`, `bounds_height`: Size

**Best for:** Spreadsheet analysis, data science, SQL import, Excel pivot tables.

---

## 5. Markdown Format

**Usage:**
```robotframework
${tree}=    Get Component Tree    format=markdown
# or
${tree}=    Get Component Tree    format=md
```

**Output:**
```markdown
# UI Component Tree

- **JFrame** `LoginFrame` - 👁️ visible ✅ enabled
  - *Text:* `Login`
  - *Bounds:* `800×600` at `(0, 0)`
  * **JPanel** `contentPane` - 👁️ visible ✅ enabled
    - *Bounds:* `800×600` at `(0, 0)`
    + **JLabel** `titleLabel` - 👁️ visible ✅ enabled
      - *Text:* `Welcome`
      - *Bounds:* `200×30` at `(10, 10)`
    + **JTextField** `usernameField` - 👁️ visible ✅ enabled
      - *Bounds:* `200×25` at `(10, 50)`
    + **JPasswordField** `passwordField` - 👁️ visible ✅ enabled
      - *Bounds:* `200×25` at `(10, 85)`
    + **JButton** `loginButton` - 👁️ visible ✅ enabled
      - *Text:* `Login`
      - *Bounds:* `100×30` at `(10, 120)`
```

**Features:**
- Hierarchical list structure with alternating markers (`-`, `*`, `+`)
- Visual badges for state:
  - 👁️ visible / 🚫 hidden
  - ✅ enabled / ❌ disabled
- Inline code formatting for identifiers
- Property details in sub-items
- Text preview (truncated at 50 chars if longer)
- Readable in plain text and renders beautifully in Markdown viewers

**Best for:** Documentation, README files, GitHub issues, human-readable reports.

---

## 6. Text Format (Simple)

**Usage:**
```robotframework
${tree}=    Get Component Tree    format=text
Log UI Tree    format=text    max_depth=3
```

**Output:**
```
[0] JFrame (LoginFrame)
  [0.0] JPanel (contentPane)
    [0.0.0] JLabel (titleLabel)
    [0.0.1] JTextField (usernameField)
    [0.0.2] JPasswordField (passwordField)
    [0.0.3] JButton (loginButton)
```

**Features:**
- Minimal, plain text format
- Indentation shows hierarchy
- Path notation in brackets
- Component type and identifier

**Best for:** Quick debugging, console output, Robot Framework logs.

---

## Format Comparison Table

| Format | File Size | Readability | Parsing | Excel | Hierarchy | Complete Data |
|--------|-----------|-------------|---------|-------|-----------|---------------|
| JSON | Medium | Fair | Easy | No | Yes | Yes |
| XML | Large | Fair | Easy | No | Yes | Yes |
| YAML | Medium | Good | Easy | No | Yes | Yes |
| CSV | Small | Good | Easy | **Yes** | Flattened | Partial |
| Markdown | Medium | **Excellent** | Hard | No | Yes | Partial |
| Text | Small | **Excellent** | Hard | No | Yes | Minimal |

---

## Advanced Usage Examples

### Filter and Export to CSV
```robotframework
# Get only buttons, export to CSV for analysis
${buttons}=    Get Component Tree
...    format=csv
...    types=JButton
...    visible_only=True

Save UI Tree    ${OUTPUT_DIR}/buttons.csv    format=csv    types=JButton
```

### Generate Documentation in Markdown
```robotframework
# Document the UI structure
${doc}=    Get Component Tree
...    format=markdown
...    max_depth=3
...    visible_only=True

Save UI Tree    ${OUTPUT_DIR}/ui_structure.md    format=markdown
```

### Excel Analysis Workflow
```robotframework
# Export tree to CSV for Excel analysis
Save UI Tree    ${OUTPUT_DIR}/ui_components.csv    format=csv

# Excel can then:
# - Filter by type (e.g., show only JTextFields)
# - Sort by depth (find deeply nested components)
# - Find components by name or text
# - Create pivot tables by component type
# - Calculate statistics (average button sizes, etc.)
```

### Multi-Format Export
```robotframework
# Export in multiple formats for different uses
${tree}=    Get Component Tree    max_depth=5

# JSON for automation
Save UI Tree    ${OUTPUT_DIR}/tree.json    format=json

# CSV for Excel analysis
Save UI Tree    ${OUTPUT_DIR}/tree.csv    format=csv

# Markdown for documentation
Save UI Tree    ${OUTPUT_DIR}/tree.md    format=markdown

# XML for schema validation
Save UI Tree    ${OUTPUT_DIR}/tree.xml    format=xml
```

---

## Special Character Handling

### CSV Escaping Examples
```csv
# Quotes are doubled
"Component with ""quotes"" in text"

# Commas require quoting
"Component with, commas, in text"

# Newlines are escaped
"Line 1\nLine 2\nLine 3"

# Unicode characters preserved
"测试中文 émojis 🎉"
```

### XML Escaping
```xml
<!-- Special characters are automatically escaped -->
<component text="Text with &lt;angle&gt; &quot;quotes&quot; &amp; ampersands" />

<!-- Becomes in parsed form: -->
<!-- Text with <angle> "quotes" & ampersands -->
```

### Markdown Escaping
```markdown
- **Component** `name with `backticks`` - handled gracefully
- **Component** `text\nwith\nnewlines` - escaped as `\n`
```

---

## Format Selection Guide

**Choose JSON when:**
- You need complete data preservation
- Integrating with JSON-based APIs
- Need programmatic parsing
- Building automation scripts

**Choose XML when:**
- You need XPath queries
- Integrating with XML systems
- Need schema validation
- Working with enterprise tools

**Choose YAML when:**
- Creating configuration files
- Need human-readable structure
- Working with YAML-based systems
- Sharing data with developers

**Choose CSV when:**
- Analyzing in Excel or Google Sheets
- Importing to databases
- Need flat data structure
- Performing statistical analysis
- Creating pivot tables or charts

**Choose Markdown when:**
- Creating documentation
- Writing GitHub issues
- Generating reports
- Need maximum readability
- Sharing with non-technical users

**Choose Text when:**
- Quick debugging
- Console/log output
- Need minimal format
- Just exploring the structure

---

## Performance Considerations

| Format | Generation Speed | File Size | Memory Usage |
|--------|-----------------|-----------|--------------|
| Text | **Fastest** | Smallest | Minimal |
| CSV | **Fastest** | Smallest | Minimal |
| JSON | Fast | Medium | Medium |
| YAML | Medium | Medium | Medium |
| Markdown | Medium | Medium | Medium |
| XML | Slowest | Largest | Highest |

For large component trees (1000+ components), prefer CSV or Text format for best performance.

---

## Error Handling

All formats handle these gracefully:
- Empty trees (no components)
- Null/missing values
- Deep nesting (100+ levels)
- Special characters
- Large coordinate values
- UTF-8 international characters

Invalid format names return a helpful error:
```
Unknown format: html. Supported formats: json, xml, text, yaml/yml, csv, markdown/md
```

Format parameter is **case-insensitive**: `JSON`, `json`, and `Json` all work.
