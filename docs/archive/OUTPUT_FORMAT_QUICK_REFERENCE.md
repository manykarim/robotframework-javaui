# Output Format Quick Reference

Quick reference guide for all `Get Component Tree` output formats.

## Supported Formats

| Format | Aliases | Best For |
|--------|---------|----------|
| **JSON** | `json`, `JSON` | APIs, automation, complete data |
| **XML** | `xml`, `XML` | XPath queries, enterprise systems |
| **YAML** | `yaml`, `yml`, `YAML`, `YML` | Configuration, human-readable data |
| **CSV** | `csv`, `CSV` | Excel analysis, spreadsheets, SQL import |
| **Markdown** | `markdown`, `md`, `MARKDOWN`, `MD` | Documentation, GitHub, reports |
| **Text** | `text`, `TEXT` | Debugging, console output |

**Note:** Format parameter is case-insensitive.

---

## Quick Examples

### Get Tree in Different Formats
```robotframework
${json}=     Get Component Tree    format=json
${xml}=      Get Component Tree    format=xml
${yaml}=     Get Component Tree    format=yaml
${csv}=      Get Component Tree    format=csv
${md}=       Get Component Tree    format=markdown
${text}=     Get Component Tree    format=text
```

### Save to File
```robotframework
Save UI Tree    ${OUTPUT_DIR}/tree.csv    format=csv
Save UI Tree    ${OUTPUT_DIR}/tree.md     format=markdown
Save UI Tree    ${OUTPUT_DIR}/tree.json   format=json
```

### With Filtering
```robotframework
# Get only buttons in CSV
${buttons}=    Get Component Tree
...    format=csv
...    types=JButton
...    visible_only=True
```

---

## Format Cheat Sheet

### CSV Columns
```
path, depth, type, name, text, visible, enabled,
bounds_x, bounds_y, bounds_width, bounds_height
```

### CSV Example
```csv
0,0,JFrame,MainWindow,Login,true,true,0,0,800,600
0.0,1,JButton,loginBtn,Click,true,true,10,10,100,30
```

### Markdown Example
```markdown
- **JFrame** `MainWindow` - 👁️ visible ✅ enabled
  * **JButton** `loginBtn` - 👁️ visible ✅ enabled
```

### XML Example
```xml
<uitree>
  <component type="JFrame" name="MainWindow" text="Login" ... >
    <component type="JButton" name="loginBtn" text="Click" ... />
  </component>
</uitree>
```

---

## When to Use Each Format

### Use JSON when:
- Building automation scripts
- Need complete data
- API integration
- Programmatic parsing

### Use XML when:
- Need XPath queries
- Enterprise integration
- Schema validation required
- XSLT transformations

### Use YAML when:
- Creating configs
- Human-readable data
- DevOps workflows
- Sharing with developers

### Use CSV when:
- Analyzing in Excel
- Creating pivot tables
- SQL database import
- Statistical analysis
- Need flat structure

### Use Markdown when:
- Writing documentation
- GitHub issues/PRs
- Creating reports
- Maximum readability
- Non-technical audience

### Use Text when:
- Quick debugging
- Console output
- Robot Framework logs
- Just exploring structure

---

## Special Characters

### CSV Escaping
```csv
# Quotes doubled
"Text with ""quotes"""

# Newlines escaped
"Line 1\nLine 2"

# Commas quoted
"Text, with, commas"
```

### XML Escaping
```xml
<!-- Auto-escaped -->
<component text="Text with &lt;brackets&gt; &amp; symbols" />
```

---

## Performance Tips

**Fastest:** Text, CSV (1.0x)
**Fast:** JSON (1.2x)
**Medium:** YAML, Markdown (1.5-2x)
**Slowest:** XML (2.0x)

**For large trees (1000+ components):** Use CSV or Text

---

## Common Workflows

### Excel Analysis
```robotframework
# 1. Export to CSV
Save UI Tree    ${OUTPUT_DIR}/components.csv    format=csv

# 2. Open in Excel
# 3. Filter by type (column C)
# 4. Create pivot tables
# 5. Analyze bounds, find outliers
```

### Documentation
```robotframework
# 1. Export to Markdown
Save UI Tree    ${OUTPUT_DIR}/ui_structure.md
...    format=markdown
...    visible_only=True
...    max_depth=3

# 2. Add to GitHub wiki or README
# 3. Renders beautifully
```

### Multi-Format Export
```robotframework
# Export all formats
${formats}=    Create List    json    xml    csv    markdown

FOR    ${fmt}    IN    @{formats}
    Save UI Tree    ${OUTPUT_DIR}/tree.${fmt}    format=${fmt}
END
```

---

## Error Messages

### Invalid Format
```
Unknown format: html.
Supported formats: json, xml, text, yaml/yml, csv, markdown/md
```

**Solution:** Use one of the supported formats (case-insensitive)

---

## File Extensions

| Format | Extension | Alternative |
|--------|-----------|-------------|
| JSON | `.json` | - |
| XML | `.xml` | - |
| YAML | `.yaml` | `.yml` |
| CSV | `.csv` | - |
| Markdown | `.md` | `.markdown` |
| Text | `.txt` | - |

---

## Advanced Features

### CSV: Import to Database
```sql
-- PostgreSQL example
COPY components(path, depth, type, name, text, visible, enabled,
                bounds_x, bounds_y, bounds_width, bounds_height)
FROM '/path/to/tree.csv'
DELIMITER ','
CSV HEADER;
```

### Markdown: GitHub Rendering
Markdown output renders beautifully on GitHub with:
- Collapsible sections (with details/summary tags)
- Emoji badges
- Code highlighting
- Nested lists

---

## Format Aliases

Both work the same:
- `yaml` = `yml`
- `markdown` = `md`
- Case doesn't matter: `JSON` = `json` = `Json`

---

## Related Keywords

```robotframework
# Log tree to RF log
Log UI Tree    format=text    max_depth=3

# Save to file
Save UI Tree    path/to/file.csv    format=csv

# Get filtered tree
${tree}=    Get Component Tree
...    format=csv
...    types=JButton,JTextField
...    visible_only=True
```

---

## Quick Reference Table

| Need | Format |
|------|--------|
| Excel analysis | CSV |
| Beautiful docs | Markdown |
| API integration | JSON |
| Config file | YAML |
| XPath queries | XML |
| Quick debug | Text |
| Complete data | JSON or YAML |
| Flat structure | CSV |
| Human readable | Markdown or YAML |

---

**For full examples and detailed documentation:**
See [`output_format_examples.md`](examples/output_format_examples.md)
