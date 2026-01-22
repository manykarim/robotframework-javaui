# Robot Framework Examples for SwingLibrary

This directory contains example Robot Framework test suites demonstrating various features of the SwingLibrary.

## Available Examples

### output_formats.robot

**Purpose**: Comprehensive examples of all supported output formats for the `Get Component Tree` keyword.

**What it demonstrates**:
- All 6 format types: JSON, XML, YAML, CSV, Markdown, Text
- Format aliases: `yml` (for YAML), `md` (for Markdown)
- Case-insensitive format parameters
- File export workflows
- Filtering combined with formats
- Multi-format export strategies
- Special character handling
- Performance comparison

**Test cases included** (15 total):
1. Example 1: Export As JSON
2. Example 2: Export As XML
3. Example 3: Export As YAML
4. Example 4: Export As CSV
5. Example 5: Export As Markdown
6. Example 6: Export As Plain Text
7. Example 7: Save CSV To File For Excel
8. Example 8: Generate Markdown Documentation
9. Example 9: Multi-Format Export
10. Example 10: CSV With Filters
11. Example 11: Markdown With Depth Limit
12. Example 12: Case Insensitive Format Parameter
13. Example 13: Format Aliases
14. Example 14: CSV Special Characters
15. Example 15: Performance Comparison

**How to use**:
```bash
# Run all examples
robot output_formats.robot

# Run specific example
robot -t "Example 1: Export As JSON" output_formats.robot

# Run with tag filter
robot -i csv output_formats.robot
```

**Note**: These examples use placeholder keywords for `Launch Test Application` and `Close Test Application`. You'll need to replace these with actual application launch logic for your test application.

## Quick Start

### 1. Install Dependencies

```bash
# Install Python dependencies
pip install robotframework
pip install robotframework-javagui

# Or use the project's environment
uv sync
```

### 2. Modify for Your Application

Edit `output_formats.robot` and update these keywords:

```robot
*** Keywords ***
Launch Test Application
    [Documentation]    Replace with your actual application launch
    # Example: Start Application    java -jar myapp.jar
    # Or: Start Application    java -cp . com.example.MainClass

Close Test Application
    [Documentation]    Replace with your actual cleanup
    # Example: Close All Windows
    # Or: Stop Application
```

### 3. Run Examples

```bash
# Run all examples
robot examples/output_formats.robot

# Run specific examples
robot -i yaml examples/output_formats.robot
robot -i csv examples/output_formats.robot
robot -i markdown examples/output_formats.robot
```

## Format Selection Guide

### Choose JSON when:
- You need complete data preservation
- Integrating with JSON-based APIs
- Building automation scripts
- Need programmatic parsing

### Choose XML when:
- You need XPath queries
- Integrating with XML systems
- Need schema validation
- Working with enterprise tools

### Choose YAML when:
- Creating configuration files
- Need human-readable structure
- Working with YAML-based systems
- Sharing data with developers

### Choose CSV when:
- Analyzing in Excel or Google Sheets
- Importing to databases
- Need flat data structure
- Performing statistical analysis
- Creating pivot tables or charts

### Choose Markdown when:
- Creating documentation
- Writing GitHub issues
- Generating reports
- Need maximum readability
- Sharing with non-technical users

### Choose Text when:
- Quick debugging
- Console/log output
- Need minimal format
- Just exploring the structure

## Common Patterns

### Pattern 1: Export for Excel Analysis

```robot
*** Test Cases ***
Analyze Buttons In Excel
    Launch Application
    ${csv}=    Get Component Tree    format=csv    types=JButton    visible_only=${True}
    Create File    ${OUTPUT_DIR}/visible_buttons.csv    ${csv}
    # Open in Excel for analysis
    [Teardown]    Close Application
```

### Pattern 2: Generate Documentation

```robot
*** Test Cases ***
Document UI Structure
    Launch Application
    ${md}=    Get Component Tree    format=markdown    max_depth=3
    Create File    ${OUTPUT_DIR}/ui_structure.md    ${md}
    # Commit to repository for documentation
    [Teardown]    Close Application
```

### Pattern 3: Multi-Format Archive

```robot
*** Test Cases ***
Create UI Archive
    Launch Application
    ${timestamp}=    Get Time    epoch

    # JSON for automation
    ${json}=    Get Component Tree    format=json
    Create File    ${OUTPUT_DIR}/ui_${timestamp}.json    ${json}

    # CSV for analysis
    ${csv}=    Get Component Tree    format=csv
    Create File    ${OUTPUT_DIR}/ui_${timestamp}.csv    ${csv}

    # Markdown for docs
    ${md}=    Get Component Tree    format=markdown
    Create File    ${OUTPUT_DIR}/ui_${timestamp}.md    ${md}

    [Teardown]    Close Application
```

### Pattern 4: Filtered Export

```robot
*** Test Cases ***
Export Interactive Components
    Launch Application

    # Get only focusable components (buttons, fields, etc.)
    ${csv}=    Get Component Tree
    ...    format=csv
    ...    focusable_only=${True}
    ...    visible_only=${True}

    Create File    ${OUTPUT_DIR}/interactive_components.csv    ${csv}
    [Teardown]    Close Application
```

## Tips and Tricks

### 1. Use Format Aliases for Brevity
```robot
${tree}=    Get Component Tree    format=yml    # Same as yaml
${doc}=     Get Component Tree    format=md     # Same as markdown
```

### 2. Combine Filters for Precision
```robot
# Get only visible, enabled buttons
${buttons}=    Get Component Tree
...    format=csv
...    types=JButton
...    visible_only=${True}
...    enabled_only=${True}
```

### 3. Limit Depth for Performance
```robot
# Shallow tree for quick overview
${overview}=    Get Component Tree    format=text    max_depth=2

# Deep tree for detailed analysis
${detailed}=    Get Component Tree    format=json    max_depth=10
```

### 4. Use Wildcards in Type Filters
```robot
# Get all button types (JButton, JToggleButton, JRadioButton, etc.)
${all_buttons}=    Get Component Tree    types=J*Button

# Get all text components
${text_comps}=     Get Component Tree    types=JText*
```

### 5. Create Reusable Keywords
```robot
*** Keywords ***
Save UI Snapshot
    [Arguments]    ${filename}    ${format}=json
    ${tree}=    Get Component Tree    format=${format}
    Create File    ${OUTPUT_DIR}/${filename}.${format}    ${tree}

*** Test Cases ***
Test With Snapshots
    Launch Application
    Save UI Snapshot    before_action    json
    Click Button    loginButton
    Save UI Snapshot    after_action     json
    [Teardown]    Close Application
```

## Troubleshooting

### Issue: "Unknown format" error
**Solution**: Check spelling and case. Format names are case-insensitive but must be spelled correctly: `json`, `xml`, `yaml`, `csv`, `markdown`, `text`

### Issue: CSV not opening in Excel
**Solution**: Make sure the file has `.csv` extension and UTF-8 encoding. Excel may need to be configured for UTF-8.

### Issue: Markdown not rendering badges
**Solution**: Some Markdown viewers may not support emoji. GitHub, GitLab, and most modern viewers do support them.

### Issue: Performance slow for large trees
**Solution**: Use `max_depth` to limit tree depth, or use CSV/Text format which are fastest.

## Additional Resources

- [Output Formats Guide](../docs/OUTPUT_FORMATS_GUIDE.md) - Comprehensive format documentation
- [Quick Reference](../docs/OUTPUT_FORMATS_QUICK_REFERENCE.md) - Format cheat sheet
- [Component Tree Guide](../docs/user-guide/COMPONENT_TREE_GUIDE.md) - Complete feature guide
- [API Reference](../docs/api-reference/COMPONENT_TREE_API.md) - Technical API docs

## Contributing

Feel free to add more examples! Submit a pull request with:
1. New `.robot` file with descriptive name
2. Update to this README
3. Documentation of what the example demonstrates

## License

Same as robotframework-swing main project.
