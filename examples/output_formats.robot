*** Settings ***
Documentation    Examples of all supported output formats for Get Component Tree
Library          JavaGui.SwingLibrary
Library          OperatingSystem

*** Variables ***
${OUTPUT_DIR}    ${CURDIR}/../.artifacts/output_examples

*** Test Cases ***
Example 1: Export As JSON
    [Documentation]    Export component tree in JSON format for programmatic parsing
    [Tags]    json    export
    Launch Test Application
    ${tree}=    Get Component Tree    format=json
    Log    ${tree}
    Should Contain    ${tree}    "roots"
    Should Contain    ${tree}    "component_type"
    [Teardown]    Close Test Application

Example 2: Export As XML
    [Documentation]    Export component tree in XML format for XML processing
    [Tags]    xml    export
    Launch Test Application
    ${tree}=    Get Component Tree    format=xml
    Log    ${tree}
    Should Contain    ${tree}    <?xml version="1.0" encoding="UTF-8"?>
    Should Contain    ${tree}    <uitree>
    Should Contain    ${tree}    <component type=
    [Teardown]    Close Test Application

Example 3: Export As YAML
    [Documentation]    Export component tree in YAML format for human-readable configuration
    [Tags]    yaml    export
    Launch Test Application
    ${tree}=    Get Component Tree    format=yaml
    Log    ${tree}
    Should Contain    ${tree}    roots:
    # Alternative: yml alias
    ${tree_yml}=    Get Component Tree    format=yml
    Should Be Equal    ${tree}    ${tree_yml}
    [Teardown]    Close Test Application

Example 4: Export As CSV
    [Documentation]    Export component tree in CSV format for Excel analysis
    [Tags]    csv    export    excel
    Launch Test Application
    ${tree}=    Get Component Tree    format=csv
    Log    ${tree}
    Should Contain    ${tree}    path,depth,type,name,text
    # Verify CSV structure
    @{lines}=    Split To Lines    ${tree}
    ${header}=    Get From List    ${lines}    0
    Should Be Equal    ${header}    path,depth,type,name,text,visible,enabled,bounds_x,bounds_y,bounds_width,bounds_height
    [Teardown]    Close Test Application

Example 5: Export As Markdown
    [Documentation]    Export component tree in Markdown format for documentation
    [Tags]    markdown    export    documentation
    Launch Test Application
    ${tree}=    Get Component Tree    format=markdown
    Log    ${tree}
    Should Contain    ${tree}    # UI Component Tree
    Should Contain    ${tree}    **
    Should Contain    ${tree}    -
    # Alternative: md alias
    ${tree_md}=    Get Component Tree    format=md
    Should Be Equal    ${tree}    ${tree_md}
    [Teardown]    Close Test Application

Example 6: Export As Plain Text
    [Documentation]    Export component tree in simple text format for debugging
    [Tags]    text    export    debug
    Launch Test Application
    ${tree}=    Get Component Tree    format=text
    Log    ${tree}
    Should Contain    ${tree}    [0]
    Should Match Regexp    ${tree}    \\[\\d+\\]
    [Teardown]    Close Test Application

Example 7: Save CSV To File For Excel
    [Documentation]    Export buttons to CSV file for Excel analysis
    [Tags]    csv    file    buttons
    Launch Test Application
    ${csv}=    Get Component Tree    format=csv    types=JButton
    Create Directory    ${OUTPUT_DIR}
    Create File    ${OUTPUT_DIR}/buttons.csv    ${csv}
    File Should Exist    ${OUTPUT_DIR}/buttons.csv
    ${content}=    Get File    ${OUTPUT_DIR}/buttons.csv
    Should Contain    ${content}    JButton
    [Teardown]    Close Test Application

Example 8: Generate Markdown Documentation
    [Documentation]    Create UI documentation in Markdown format
    [Tags]    markdown    documentation    file
    Launch Test Application
    ${md}=    Get Component Tree    format=markdown    max_depth=3
    Create Directory    ${OUTPUT_DIR}
    Create File    ${OUTPUT_DIR}/ui_structure.md    ${md}
    File Should Exist    ${OUTPUT_DIR}/ui_structure.md
    ${content}=    Get File    ${OUTPUT_DIR}/ui_structure.md
    Should Contain    ${content}    # UI Component Tree
    [Teardown]    Close Test Application

Example 9: Multi-Format Export
    [Documentation]    Export the same tree in all formats for different uses
    [Tags]    multi-format    export
    Launch Test Application
    Create Directory    ${OUTPUT_DIR}

    # JSON for automation
    ${json}=    Get Component Tree    format=json
    Create File    ${OUTPUT_DIR}/tree.json    ${json}

    # XML for XML tools
    ${xml}=    Get Component Tree    format=xml
    Create File    ${OUTPUT_DIR}/tree.xml    ${xml}

    # YAML for configuration
    ${yaml}=    Get Component Tree    format=yaml
    Create File    ${OUTPUT_DIR}/tree.yaml    ${yaml}

    # CSV for Excel
    ${csv}=    Get Component Tree    format=csv
    Create File    ${OUTPUT_DIR}/tree.csv    ${csv}

    # Markdown for docs
    ${md}=    Get Component Tree    format=markdown
    Create File    ${OUTPUT_DIR}/tree.md    ${md}

    # Text for debugging
    ${text}=    Get Component Tree    format=text
    Create File    ${OUTPUT_DIR}/tree.txt    ${text}

    # Verify all files exist
    File Should Exist    ${OUTPUT_DIR}/tree.json
    File Should Exist    ${OUTPUT_DIR}/tree.xml
    File Should Exist    ${OUTPUT_DIR}/tree.yaml
    File Should Exist    ${OUTPUT_DIR}/tree.csv
    File Should Exist    ${OUTPUT_DIR}/tree.md
    File Should Exist    ${OUTPUT_DIR}/tree.txt
    [Teardown]    Close Test Application

Example 10: CSV With Filters
    [Documentation]    Export filtered components to CSV
    [Tags]    csv    filter
    Launch Test Application

    # Get only visible buttons
    ${csv}=    Get Component Tree    format=csv    types=JButton    visible_only=${True}
    Should Contain    ${csv}    JButton
    Should Contain    ${csv}    true

    # Get text inputs only
    ${inputs_csv}=    Get Component Tree    format=csv    types=JTextField,JTextArea
    Should Contain    ${inputs_csv}    JTextField
    [Teardown]    Close Test Application

Example 11: Markdown With Depth Limit
    [Documentation]    Create shallow documentation with max depth
    [Tags]    markdown    depth
    Launch Test Application
    ${md_shallow}=    Get Component Tree    format=markdown    max_depth=2
    ${md_deep}=    Get Component Tree    format=markdown    max_depth=5
    # Shallow should be shorter
    ${len_shallow}=    Get Length    ${md_shallow}
    ${len_deep}=    Get Length    ${md_deep}
    Should Be True    ${len_shallow} < ${len_deep}
    [Teardown]    Close Test Application

Example 12: Case Insensitive Format Parameter
    [Documentation]    Verify format parameter is case-insensitive
    [Tags]    format    case-insensitive
    Launch Test Application
    ${json_lower}=    Get Component Tree    format=json
    ${json_upper}=    Get Component Tree    format=JSON
    ${json_mixed}=    Get Component Tree    format=Json
    Should Be Equal    ${json_lower}    ${json_upper}
    Should Be Equal    ${json_lower}    ${json_mixed}
    [Teardown]    Close Test Application

Example 13: Format Aliases
    [Documentation]    Test format aliases (yml=yaml, md=markdown)
    [Tags]    format    aliases
    Launch Test Application

    # YAML aliases
    ${yaml_long}=    Get Component Tree    format=yaml
    ${yaml_short}=    Get Component Tree    format=yml
    Should Be Equal    ${yaml_long}    ${yaml_short}

    # Markdown aliases
    ${md_long}=    Get Component Tree    format=markdown
    ${md_short}=    Get Component Tree    format=md
    Should Be Equal    ${md_long}    ${md_short}
    [Teardown]    Close Test Application

Example 14: CSV Special Characters
    [Documentation]    Verify CSV handles special characters correctly
    [Tags]    csv    special-chars
    Launch Test Application
    ${csv}=    Get Component Tree    format=csv
    # CSV should handle commas, quotes, newlines
    Should Contain    ${csv}    path,depth,type
    # If text contains commas, it should be quoted
    [Teardown]    Close Test Application

Example 15: Performance Comparison
    [Documentation]    Compare performance of different formats
    [Tags]    performance    benchmark
    Launch Test Application

    # Time each format
    ${start}=    Get Time    epoch
    ${json}=    Get Component Tree    format=json
    ${json_time}=    Evaluate    ${start}

    ${start}=    Get Time    epoch
    ${xml}=    Get Component Tree    format=xml
    ${xml_time}=    Evaluate    ${start}

    ${start}=    Get Time    epoch
    ${yaml}=    Get Component Tree    format=yaml
    ${yaml_time}=    Evaluate    ${start}

    ${start}=    Get Time    epoch
    ${csv}=    Get Component Tree    format=csv
    ${csv_time}=    Evaluate    ${start}

    ${start}=    Get Time    epoch
    ${md}=    Get Component Tree    format=markdown
    ${md_time}=    Evaluate    ${start}

    ${start}=    Get Time    epoch
    ${text}=    Get Component Tree    format=text
    ${text_time}=    Evaluate    ${start}

    Log    Format performance comparison completed
    [Teardown]    Close Test Application

*** Keywords ***
Launch Test Application
    [Documentation]    Launch a test Swing application (implement based on your test app)
    # This is a placeholder - replace with actual test application launch
    Log    Test application would be launched here
    # Example: Start Application    java -jar test-app.jar

Close Test Application
    [Documentation]    Close the test application
    # This is a placeholder - replace with actual cleanup
    Log    Test application would be closed here
    # Example: Close All Windows
