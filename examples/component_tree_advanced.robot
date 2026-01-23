*** Settings ***
Documentation     Advanced Component Tree Examples
...               This suite demonstrates advanced use cases including
...               subtree retrieval, performance optimization, and
...               programmatic tree analysis.
Library           JavaGui.Swing
Library           Collections
Library           OperatingSystem
Library           Process
Suite Setup       Start Application
Suite Teardown    Stop Application

*** Variables ***
${APP_JAR}        ../demo/target/swing-demo.jar
${AGENT_JAR}      ../agent/target/javagui-agent.jar
${PORT}           5678
${OUTPUT_DIR}     ${CURDIR}/output

*** Keywords ***
Start Application
    ${cmd}=    Set Variable    java -javaagent:${AGENT_JAR}=port=${PORT} -jar ${APP_JAR}
    Start Process    ${cmd}    shell=True    alias=demo_app
    Sleep    3s
    Connect To Application    main_class=com.robotframework.demo.SwingDemo    port=${PORT}

Stop Application
    Disconnect
    Terminate Process    demo_app    kill=True

Count Components By Type
    [Documentation]    Count components of a specific type in tree data
    [Arguments]    ${tree_data}    ${component_type}
    ${count}=    Set Variable    0
    ${count}=    Count Components Recursive    ${tree_data}    ${component_type}    ${count}
    [Return]    ${count}

Count Components Recursive
    [Arguments]    ${node}    ${type}    ${count}
    # Check if this node matches type
    ${simple_class}=    Get From Dictionary    ${node}    simpleClass
    ${count}=    Run Keyword If    '${simple_class}' == '${type}'
    ...    Evaluate    ${count} + 1
    ...    ELSE    Set Variable    ${count}

    # Check children
    ${has_children}=    Run Keyword And Return Status
    ...    Dictionary Should Contain Key    ${node}    children
    ${count}=    Run Keyword If    ${has_children}
    ...    Count In Children    ${node}    ${type}    ${count}
    ...    ELSE    Set Variable    ${count}

    [Return]    ${count}

Count In Children
    [Arguments]    ${node}    ${type}    ${count}
    ${children}=    Get From Dictionary    ${node}    children
    FOR    ${child}    IN    @{children}
        ${count}=    Count Components Recursive    ${child}    ${type}    ${count}
    END
    [Return]    ${count}

*** Test Cases ***
Get Subtree From Specific Component
    [Documentation]    Retrieve subtree starting from a specific component
    [Tags]    advanced    subtree

    # First, get full tree to see structure
    ${full_tree}=    Get Component Tree    format=text    max_depth=3
    Log    Full tree:${\n}${full_tree}

    # Get subtree of main panel (adjust locator as needed)
    ${subtree}=    Run Keyword And Ignore Error
    ...    Get Component Subtree    locator=JPanel    format=text
    Log    ${subtree}

    # Subtree should be smaller than full tree
    Run Keyword If    '${subtree[0]}' == 'PASS'
    ...    Should Be True    len('''${subtree[1]}''') < len('''${full_tree}''')

Progressive Tree Inspection
    [Documentation]    Start with shallow overview, then drill deeper
    [Tags]    advanced    performance

    # Step 1: Get high-level overview (2 levels)
    ${overview}=    Get Component Tree    max_depth=2
    Log    High-level overview:${\n}${overview}

    # Step 2: Based on overview, identify section to inspect
    # (In real test, you would parse overview to find interesting components)

    # Step 3: Get detailed view of specific section
    ${detailed}=    Get Component Tree    max_depth=10
    Log    Detailed view:${\n}${detailed}

Analyze Component Composition
    [Documentation]    Programmatically analyze component types in tree
    [Tags]    advanced    analysis

    # Get tree in JSON
    ${json_tree}=    Get Component Tree    format=json
    ${data}=    Evaluate    json.loads('''${json_tree}''')    modules=json

    # Get first root
    ${roots}=    Get From Dictionary    ${data}    roots
    ${root}=    Get From List    ${roots}    0

    # Count different component types
    ${button_count}=    Count Components By Type    ${root}    JButton
    ${panel_count}=    Count Components By Type    ${root}    JPanel
    ${label_count}=    Count Components By Type    ${root}    JLabel

    # Log composition
    Log    Component Composition:
    Log    - JButtons: ${button_count}
    Log    - JPanels: ${panel_count}
    Log    - JLabels: ${label_count}

    # Verify we have interactive elements
    Should Be True    ${button_count} > 0    Expected at least one button

Save Tree To File
    [Documentation]    Save component tree to file for documentation
    [Tags]    advanced    export

    # Create output directory if it doesn't exist
    Create Directory    ${OUTPUT_DIR}

    # Save text format
    ${text_tree}=    Get Component Tree    format=text
    Create File    ${OUTPUT_DIR}/component_tree.txt    ${text_tree}

    # Save JSON format
    ${json_tree}=    Get Component Tree    format=json
    Create File    ${OUTPUT_DIR}/component_tree.json    ${json_tree}

    # Verify files were created
    File Should Exist    ${OUTPUT_DIR}/component_tree.txt
    File Should Exist    ${OUTPUT_DIR}/component_tree.json

    Log    Tree saved to:
    Log    - ${OUTPUT_DIR}/component_tree.txt
    Log    - ${OUTPUT_DIR}/component_tree.json

Find Component By Property
    [Documentation]    Search tree for components with specific properties
    [Tags]    advanced    search

    # Get tree
    ${json_tree}=    Get Component Tree    format=json
    ${data}=    Evaluate    json.loads('''${json_tree}''')    modules=json

    # Get root
    ${roots}=    Get From Dictionary    ${data}    roots
    ${root}=    Get From List    ${roots}    0

    # Find all visible components
    # (In production, you'd implement a recursive search)
    ${visible_count}=    Set Variable    0
    # This is a simplified example - real implementation would recurse

    Log    Searching for visible components...

Compare UI Before And After Action
    [Documentation]    Capture tree before/after UI change for comparison
    [Tags]    advanced    comparison

    # Get initial state
    ${before_json}=    Get Component Tree    format=json    max_depth=5
    ${before}=    Evaluate    json.loads('''${before_json}''')    modules=json

    # Perform action that changes UI
    # (adjust to your application)
    Run Keyword And Ignore Error    Click    JButton
    Sleep    500ms

    # Refresh and get new state
    Refresh Component Tree
    ${after_json}=    Get Component Tree    format=json    max_depth=5
    ${after}=    Evaluate    json.loads('''${after_json}''')    modules=json

    # Compare timestamps
    ${before_ts}=    Get From Dictionary    ${before}    timestamp
    ${after_ts}=    Get From Dictionary    ${after}    timestamp
    Should Be True    ${after_ts} > ${before_ts}

    # Count components in both
    ${before_roots}=    Get From Dictionary    ${before}    roots
    ${after_roots}=    Get From Dictionary    ${after}    roots

    Log    Comparison:
    Log    - Before: ${before_roots.__len__()} roots
    Log    - After: ${after_roots.__len__()} roots

Performance Test With Different Depths
    [Documentation]    Measure performance impact of depth parameter
    [Tags]    advanced    performance

    # Test different depths
    ${start}=    Get Time    epoch

    ${tree_d2}=    Get Component Tree    max_depth=2
    ${time_d2}=    Evaluate    time.time() - ${start}    modules=time

    ${tree_d5}=    Get Component Tree    max_depth=5
    ${time_d5}=    Evaluate    time.time() - ${start}    modules=time

    ${tree_d10}=    Get Component Tree    max_depth=10
    ${time_d10}=    Evaluate    time.time() - ${start}    modules=time

    # Log results
    Log    Performance by depth:
    Log    - Depth 2: ${time_d2}s (${tree_d2.__len__()} chars)
    Log    - Depth 5: ${time_d5}s (${tree_d5.__len__()} chars)
    Log    - Depth 10: ${time_d10}s (${tree_d10.__len__()} chars)

Extract Component Names For Debugging
    [Documentation]    Extract all component names for locator building
    [Tags]    advanced    debugging

    # Get tree
    ${json_tree}=    Get Component Tree    format=json
    ${data}=    Evaluate    json.loads('''${json_tree}''')    modules=json

    # In a real implementation, you would recursively extract names
    # This is a simplified example

    Log    Use this tree to find component names for locators:
    Log    ${json_tree}

    # Save for reference
    Create Directory    ${OUTPUT_DIR}
    Create File    ${OUTPUT_DIR}/component_names.json    ${json_tree}
