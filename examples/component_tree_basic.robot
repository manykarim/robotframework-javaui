*** Settings ***
Documentation     Basic Component Tree Examples
...               This test suite demonstrates basic component tree retrieval
...               and inspection capabilities.
Library           JavaGui.Swing
Library           Process
Suite Setup       Start Application
Suite Teardown    Stop Application

*** Variables ***
${APP_JAR}        ../demo/target/swing-demo.jar
${AGENT_JAR}      ../agent/target/javagui-agent.jar
${PORT}           5678

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
Get Full Component Tree In Text Format
    [Documentation]    Retrieve the complete component tree in human-readable text format
    [Tags]    basic    text-format

    ${tree}=    Get Component Tree    format=text
    Log    ${tree}

    # Verify tree was retrieved (should contain component names)
    Should Contain    ${tree}    JFrame
    Length Should Be    ${tree}    min_length=100

Get Component Tree In JSON Format
    [Documentation]    Retrieve component tree in JSON format for programmatic access
    [Tags]    basic    json-format

    ${json_tree}=    Get Component Tree    format=json
    Log    ${json_tree}

    # Parse JSON
    ${data}=    Evaluate    json.loads('''${json_tree}''')    modules=json

    # Verify structure
    Dictionary Should Contain Key    ${data}    roots
    Dictionary Should Contain Key    ${data}    timestamp

    # Check we have at least one root window
    ${roots}=    Get From Dictionary    ${data}    roots
    ${root_count}=    Get Length    ${roots}
    Should Be True    ${root_count} > 0    Expected at least one root window

    # Log details about first root
    ${first_root}=    Get From List    ${roots}    0
    Log    First root class: ${first_root['class']}
    Log    First root name: ${first_root['name']}

Get Component Tree With Depth Limit
    [Documentation]    Retrieve tree with limited depth for performance
    [Tags]    basic    depth-control

    # Get shallow tree (3 levels)
    ${shallow_tree}=    Get Component Tree    max_depth=3
    Log    Shallow tree (depth=3):${\n}${shallow_tree}

    # Get deeper tree (10 levels)
    ${deep_tree}=    Get Component Tree    max_depth=10
    Log    Deep tree (depth=10):${\n}${deep_tree}

    # Shallow tree should be smaller
    ${shallow_len}=    Get Length    ${shallow_tree}
    ${deep_len}=    Get Length    ${deep_tree}
    Should Be True    ${deep_len} > ${shallow_len}

Log Component Tree To Robot Log
    [Documentation]    Log tree directly to Robot Framework log
    [Tags]    basic    logging

    # Log at INFO level (visible in normal log)
    Log Component Tree    format=text    level=INFO

    # Log at DEBUG level (visible only in debug mode)
    Log Component Tree    format=json    level=DEBUG

Refresh Component Tree After UI Change
    [Documentation]    Demonstrate tree refresh after dynamic UI changes
    [Tags]    basic    refresh

    # Get initial tree
    ${before}=    Get Component Tree    max_depth=5

    # Make UI change (open a dialog or panel)
    # This example assumes there's a button that opens something
    Run Keyword And Ignore Error    Click    JButton[text='Open']
    Sleep    500ms

    # Refresh tree to see changes
    Refresh Component Tree

    # Get updated tree
    ${after}=    Get Component Tree    max_depth=5

    # Trees should potentially be different
    Log    Tree before: ${before}
    Log    Tree after: ${after}

Compare Text And JSON Output Size
    [Documentation]    Compare different output formats
    [Tags]    basic    format-comparison

    ${text}=    Get Component Tree    format=text
    ${json}=    Get Component Tree    format=json

    ${text_len}=    Get Length    ${text}
    ${json_len}=    Get Length    ${json}

    Log    Text format: ${text_len} characters
    Log    JSON format: ${json_len} characters

    # JSON is typically more verbose
    Should Be True    ${json_len} > ${text_len}
