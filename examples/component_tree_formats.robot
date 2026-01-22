*** Settings ***
Documentation     Component Tree Output Formats Examples
...               This test suite demonstrates all 6 output formats:
...               1. Text (default) - Simple indented format
...               2. JSON - Structured data with full details
...               3. XML - Hierarchical markup
...               4. YAML - Human-readable structured format
...               5. CSV - Flattened tabular format
...               6. Markdown - Documentation-friendly table

Library           JavaGui.Swing
Library           Process
Library           OperatingSystem
Suite Setup       Start Application
Suite Teardown    Stop Application

*** Variables ***
${APP_JAR}        ../demo/target/swing-demo.jar
${AGENT_JAR}      ../agent/target/javagui-agent.jar
${PORT}           5678
${OUTPUT_DIR}     ${CURDIR}/../test-output

*** Keywords ***
Start Application
    ${cmd}=    Set Variable    java -javaagent:${AGENT_JAR}=port=${PORT} -jar ${APP_JAR}
    Start Process    ${cmd}    shell=True    alias=demo_app
    Sleep    3s
    Connect To Application    main_class=com.robotframework.demo.SwingDemo    port=${PORT}

Stop Application
    Disconnect
    Terminate Process    demo_app    kill=True

*** Test Cases ***
Format 1 - Text Format (Default)
    [Documentation]    Simple indented text format - easy to read, good for logs
    [Tags]    formats    text

    ${tree}=    Get Component Tree    format=text
    Log    TEXT FORMAT (default):${\n}${tree}

    # Verify it's text format (contains indentation)
    Should Contain    ${tree}    JFrame
    ${lines}=    Split To Lines    ${tree}
    ${line_count}=    Get Length    ${lines}
    Log    Text format has ${line_count} lines

    # Text format characteristics
    Log    ✅ Pros: Simple, readable, compact
    Log    ✅ Good for: Quick debugging, log files
    Log    ❌ Cons: Not programmatically parseable

Format 1b - Text Format With Depth Limit
    [Documentation]    Text format with controlled depth
    [Tags]    formats    text    depth

    ${shallow}=    Get Component Tree    format=text    max_depth=3
    ${deep}=    Get Component Tree    format=text    max_depth=10

    Log    Shallow (depth 3):${\n}${shallow}
    Log    Deep (depth 10):${\n}${deep}

    ${shallow_len}=    Get Length    ${shallow}
    ${deep_len}=    Get Length    ${deep}
    Should Be True    ${deep_len} > ${shallow_len}

Format 2 - JSON Format
    [Documentation]    Structured JSON format - full details, programmatically parseable
    [Tags]    formats    json

    ${json}=    Get Component Tree    format=json
    Log    JSON FORMAT:${\n}${json}

    # Parse JSON to verify structure
    ${data}=    Evaluate    json.loads('''${json}''')    modules=json

    # Verify JSON structure
    Dictionary Should Contain Key    ${data}    roots

    # Get roots array
    ${roots}=    Get From Dictionary    ${data}    roots
    ${root_count}=    Get Length    ${roots}
    Should Be True    ${root_count} > 0

    # Inspect first root
    ${first_root}=    Get From List    ${roots}    0
    Log    First root: ${first_root}

    # JSON format should have detailed structure
    Dictionary Should Contain Key    ${first_root}    id
    Dictionary Should Contain Key    ${first_root}    component_type
    Dictionary Should Contain Key    ${first_root}    identity
    Dictionary Should Contain Key    ${first_root}    state
    Dictionary Should Contain Key    ${first_root}    geometry

    # JSON format characteristics
    Log    ✅ Pros: Structured, parseable, complete data
    Log    ✅ Good for: Automation, data analysis, API responses
    Log    ❌ Cons: Verbose, harder to read manually

Format 2b - JSON Format Programmatic Analysis
    [Documentation]    Demonstrate programmatic analysis of JSON format
    [Tags]    formats    json    analysis

    ${json}=    Get Component Tree    format=json    max_depth=5

    # Parse and analyze
    ${data}=    Evaluate    json.loads('''${json}''')    modules=json

    # Count components recursively (would need helper function)
    Log    JSON data retrieved and parsed successfully

    # Example: Extract all component names
    Log    Can programmatically extract and analyze component data

Format 3 - XML Format
    [Documentation]    Hierarchical XML markup format
    [Tags]    formats    xml

    ${xml}=    Get Component Tree    format=xml
    Log    XML FORMAT:${\n}${xml}

    # Verify XML structure
    Should Start With    ${xml}    <?xml version="1.0"
    Should Contain    ${xml}    <uitree>
    Should Contain    ${xml}    <component
    Should Contain    ${xml}    </uitree>

    # XML format characteristics
    Log    ✅ Pros: Standard format, self-documenting, tool support
    Log    ✅ Good for: Integration with XML tools, documentation
    Log    ❌ Cons: Verbose, requires XML parser

Format 3b - XML Format Save To File
    [Documentation]    Save XML to file for external processing
    [Tags]    formats    xml    file

    ${xml}=    Get Component Tree    format=xml    max_depth=5
    Create File    ${OUTPUT_DIR}/component_tree.xml    ${xml}
    Log    Saved XML to ${OUTPUT_DIR}/component_tree.xml

    # Verify file was created
    File Should Exist    ${OUTPUT_DIR}/component_tree.xml
    ${size}=    Get File Size    ${OUTPUT_DIR}/component_tree.xml
    Log    XML file size: ${size} bytes

Format 4 - YAML Format
    [Documentation]    Human-readable YAML format
    [Tags]    formats    yaml

    ${yaml}=    Get Component Tree    format=yaml
    Log    YAML FORMAT:${\n}${yaml}

    # Verify YAML structure
    Should Contain    ${yaml}    roots:
    Should Contain    ${yaml}    component_type:
    Should Contain    ${yaml}    state:

    # YAML format characteristics
    Log    ✅ Pros: Human-readable, structured, cleaner than JSON
    Log    ✅ Good for: Configuration, documentation, data exchange
    Log    ❌ Cons: Requires YAML parser, indentation-sensitive

Format 4b - YAML Format With Alias
    [Documentation]    YAML format using 'yml' alias
    [Tags]    formats    yaml

    ${yaml1}=    Get Component Tree    format=yaml    max_depth=3
    ${yaml2}=    Get Component Tree    format=yml    max_depth=3

    # Both should work (yml is alias for yaml)
    Should Not Be Empty    ${yaml1}
    Should Not Be Empty    ${yaml2}

Format 5 - CSV Format
    [Documentation]    Flattened tabular CSV format
    [Tags]    formats    csv

    ${csv}=    Get Component Tree    format=csv
    Log    CSV FORMAT:${\n}${csv}

    # Verify CSV structure (should have headers)
    ${lines}=    Split To Lines    ${csv}
    ${header}=    Get From List    ${lines}    0
    Log    CSV Headers: ${header}

    Should Contain    ${header}    type
    Should Contain    ${header}    name

    # CSV format characteristics
    Log    ✅ Pros: Spreadsheet-compatible, simple structure
    Log    ✅ Good for: Excel analysis, data import, reporting
    Log    ❌ Cons: Loses hierarchy, flattened structure

Format 5b - CSV Format Save For Spreadsheet
    [Documentation]    Save CSV for Excel/spreadsheet analysis
    [Tags]    formats    csv    file

    ${csv}=    Get Component Tree    format=csv    max_depth=10
    Create File    ${OUTPUT_DIR}/component_tree.csv    ${csv}
    Log    Saved CSV to ${OUTPUT_DIR}/component_tree.csv

    # Count rows
    ${lines}=    Split To Lines    ${csv}
    ${row_count}=    Get Length    ${lines}
    Log    CSV has ${row_count} rows (including header)

    # CSV can be opened in Excel for analysis
    Log    Open ${OUTPUT_DIR}/component_tree.csv in Excel for analysis

Format 6 - Markdown Format
    [Documentation]    Documentation-friendly markdown table format
    [Tags]    formats    markdown

    ${markdown}=    Get Component Tree    format=markdown
    Log    MARKDOWN FORMAT:${\n}${markdown}

    # Verify markdown structure (should have table headers)
    Should Contain    ${markdown}    |
    Should Contain    ${markdown}    Type
    Should Contain    ${markdown}    Name

    # Markdown format characteristics
    Log    ✅ Pros: Documentation-friendly, GitHub-compatible, readable
    Log    ✅ Good for: README files, documentation, reports
    Log    ❌ Cons: Limited structure, loses hierarchy

Format 6b - Markdown Format For Documentation
    [Documentation]    Save markdown for documentation
    [Tags]    formats    markdown    file

    ${markdown}=    Get Component Tree    format=markdown    max_depth=5
    Create File    ${OUTPUT_DIR}/component_tree.md    ${markdown}
    Log    Saved Markdown to ${OUTPUT_DIR}/component_tree.md

    # Add header and save as complete document
    ${doc}=    Catenate    SEPARATOR=\n\n
    ...    # UI Component Tree
    ...    Generated: ${CURDIR}
    ...    \n
    ...    ${markdown}
    Create File    ${OUTPUT_DIR}/component_tree_doc.md    ${doc}

Format Comparison - Size
    [Documentation]    Compare output sizes of different formats
    [Tags]    formats    comparison    size

    # Get same tree in all formats
    ${text}=    Get Component Tree    format=text    max_depth=5
    ${json}=    Get Component Tree    format=json    max_depth=5
    ${xml}=    Get Component Tree    format=xml    max_depth=5
    ${yaml}=    Get Component Tree    format=yaml    max_depth=5
    ${csv}=    Get Component Tree    format=csv    max_depth=5
    ${markdown}=    Get Component Tree    format=markdown    max_depth=5

    # Measure sizes
    ${text_size}=    Get Length    ${text}
    ${json_size}=    Get Length    ${json}
    ${xml_size}=    Get Length    ${xml}
    ${yaml_size}=    Get Length    ${yaml}
    ${csv_size}=    Get Length    ${csv}
    ${markdown_size}=    Get Length    ${markdown}

    # Log comparison
    Log    Format Size Comparison (depth=5):
    Log    - Text: ${text_size} chars
    Log    - JSON: ${json_size} chars
    Log    - XML: ${xml_size} chars
    Log    - YAML: ${yaml_size} chars
    Log    - CSV: ${csv_size} chars
    Log    - Markdown: ${markdown_size} chars

    # Typically: Text < CSV < Markdown < YAML < JSON < XML
    Should Be True    ${text_size} > 0

Format Comparison - Use Cases
    [Documentation]    Demonstrate appropriate use cases for each format
    [Tags]    formats    comparison    use-cases

    Log    FORMAT USE CASES:
    Log    \n
    Log    1. TEXT - Quick debugging, log files
    Log    2. JSON - Automation, APIs, data analysis
    Log    3. XML - Tool integration, XML processors
    Log    4. YAML - Config files, human-readable data
    Log    5. CSV - Spreadsheets, tabular analysis
    Log    6. MARKDOWN - Documentation, GitHub README

Real World Example - Save All Formats
    [Documentation]    Save tree in all formats for different purposes
    [Tags]    formats    real-world

    # Get tree once
    ${max_depth}=    Set Variable    5

    # Save all formats
    ${text}=    Get Component Tree    format=text    max_depth=${max_depth}
    Create File    ${OUTPUT_DIR}/ui_tree.txt    ${text}

    ${json}=    Get Component Tree    format=json    max_depth=${max_depth}
    Create File    ${OUTPUT_DIR}/ui_tree.json    ${json}

    ${xml}=    Get Component Tree    format=xml    max_depth=${max_depth}
    Create File    ${OUTPUT_DIR}/ui_tree.xml    ${xml}

    ${yaml}=    Get Component Tree    format=yaml    max_depth=${max_depth}
    Create File    ${OUTPUT_DIR}/ui_tree.yaml    ${yaml}

    ${csv}=    Get Component Tree    format=csv    max_depth=${max_depth}
    Create File    ${OUTPUT_DIR}/ui_tree.csv    ${csv}

    ${markdown}=    Get Component Tree    format=markdown    max_depth=${max_depth}
    Create File    ${OUTPUT_DIR}/ui_tree.md    ${markdown}

    Log    Saved all 6 formats to ${OUTPUT_DIR}/

Real World Example - JSON For CI/CD
    [Documentation]    Use JSON format for CI/CD pipeline
    [Tags]    formats    real-world    ci-cd

    # Get detailed JSON tree
    ${json}=    Get Component Tree    format=json

    # Save for CI/CD artifact
    Create File    ${OUTPUT_DIR}/ui_structure.json    ${json}

    # In CI/CD, this could be:
    # - Uploaded as build artifact
    # - Compared with previous builds
    # - Analyzed for regressions
    # - Used for automated documentation

    Log    JSON artifact created for CI/CD pipeline

Real World Example - CSV For Reporting
    [Documentation]    Use CSV format for management reports
    [Tags]    formats    real-world    reporting

    # Get CSV of all visible, enabled components
    ${csv}=    Get Component Tree
    ...    format=csv
    ...    visible_only=${True}
    ...    enabled_only=${True}
    ...    max_depth=10

    Create File    ${OUTPUT_DIR}/ui_components_report.csv    ${csv}

    Log    Component report created: ${OUTPUT_DIR}/ui_components_report.csv
    Log    Open in Excel to create pivot tables and charts

Real World Example - Markdown For Documentation
    [Documentation]    Use Markdown format for GitHub documentation
    [Tags]    formats    real-world    documentation

    # Get markdown of main form
    ${markdown}=    Get Component Tree
    ...    format=markdown
    ...    max_depth=3

    # Create documentation page
    ${doc}=    Catenate    SEPARATOR=\n\n
    ...    # Login Form UI Structure
    ...    \n
    ...    This document describes the UI structure of the login form.
    ...    \n
    ...    ## Component Hierarchy
    ...    \n
    ...    ${markdown}
    ...    \n
    ...    ## Testing Notes
    ...    \n
    ...    - All buttons are accessible
    ...    - Form uses standard components
    ...    - No custom widgets

    Create File    ${OUTPUT_DIR}/LOGIN_FORM.md    ${doc}
    Log    Documentation created: ${OUTPUT_DIR}/LOGIN_FORM.md
