*** Settings ***
Documentation     Component Tree Advanced Filtering Examples
...               This test suite demonstrates all filtering capabilities:
...               - Type filtering with wildcards
...               - Exclusion filtering
...               - State filtering (visible, enabled, focusable)
...               - Combined filtering
...               - Performance optimization

Library           JavaGui.Swing
Library           Process
Library           Collections
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
Filter By Single Component Type
    [Documentation]    Retrieve only specific component types
    [Tags]    filtering    basic

    # Get only buttons
    ${buttons}=    Get Component Tree    types=JButton    format=json
    Log    Buttons only:${\n}${buttons}

    # Verify result contains only buttons
    ${data}=    Evaluate    json.loads('''${buttons}''')    modules=json
    Should Contain    ${buttons}    JButton
    # Could parse and verify no other types exist

Filter By Multiple Component Types
    [Documentation]    Filter by comma-separated list of types
    [Tags]    filtering    multiple-types

    # Get buttons and text fields
    ${inputs}=    Get Component Tree    types=JButton,JTextField    format=json
    Log    Buttons and text fields:${\n}${inputs}

    # Get all text components
    ${text_components}=    Get Component Tree    types=JButton,JTextField,JTextArea,JLabel
    Log    All text components:${\n}${text_components}

Filter By Wildcard Patterns
    [Documentation]    Use wildcard patterns for flexible type matching
    [Tags]    filtering    wildcards

    # Get all button types (JButton, JToggleButton, JRadioButton, etc.)
    ${all_buttons}=    Get Component Tree    types=J*Button    format=text
    Log    All button types (J*Button):${\n}${all_buttons}
    Should Contain    ${all_buttons}    Button

    # Get all text components (JTextField, JTextArea, JTextPane, etc.)
    ${text_components}=    Get Component Tree    types=JText*    format=text
    Log    All text components (JText*):${\n}${text_components}

    # Get all J-prefixed components
    ${j_components}=    Get Component Tree    types=J*    format=text
    Log    All J-components (J*):${\n}${j_components}

Exclude Component Types
    [Documentation]    Exclude specific component types from tree
    [Tags]    filtering    exclusion

    # Get all components except labels
    ${no_labels}=    Get Component Tree    exclude_types=JLabel    format=text
    Log    Tree without labels:${\n}${no_labels}
    Should Not Contain    ${no_labels}    JLabel

    # Exclude multiple types
    ${no_panels_labels}=    Get Component Tree
    ...    exclude_types=JLabel,JPanel
    ...    format=text
    Log    Tree without labels and panels:${\n}${no_panels_labels}
    Should Not Contain    ${no_panels_labels}    JLabel
    Should Not Contain    ${no_panels_labels}    JPanel

Combine Include And Exclude Filters
    [Documentation]    Use both types and exclude_types together
    [Tags]    filtering    combined

    # Include all buttons but exclude radio buttons
    ${buttons_no_radio}=    Get Component Tree
    ...    types=J*Button
    ...    exclude_types=JRadioButton
    ...    format=text
    Log    All buttons except radio:${\n}${buttons_no_radio}
    Should Contain    ${buttons_no_radio}    JButton
    Should Not Contain    ${buttons_no_radio}    JRadioButton

    # Include text components but exclude text areas
    ${text_no_areas}=    Get Component Tree
    ...    types=JText*
    ...    exclude_types=JTextArea
    ...    format=text
    Log    Text components except areas:${\n}${text_no_areas}

Filter By Visible State
    [Documentation]    Get only visible components
    [Tags]    filtering    state    visible

    # Get all visible components
    ${visible}=    Get Component Tree    visible_only=${True}    format=json
    Log    Visible components only:${\n}${visible}

    # Parse and verify all are visible
    ${data}=    Evaluate    json.loads('''${visible}''')    modules=json
    # All components in result should have visible=true

    # Get visible buttons
    ${visible_buttons}=    Get Component Tree
    ...    types=JButton
    ...    visible_only=${True}
    ...    format=json
    Log    Visible buttons only:${\n}${visible_buttons}

Filter By Enabled State
    [Documentation]    Get only enabled components
    [Tags]    filtering    state    enabled

    # Get all enabled components
    ${enabled}=    Get Component Tree    enabled_only=${True}    format=json
    Log    Enabled components only:${\n}${enabled}

    # Get enabled buttons
    ${enabled_buttons}=    Get Component Tree
    ...    types=JButton
    ...    enabled_only=${True}
    ...    format=json
    Log    Enabled buttons only:${\n}${enabled_buttons}

Filter By Focusable State
    [Documentation]    Get only focusable components
    [Tags]    filtering    state    focusable

    # Get all focusable components
    ${focusable}=    Get Component Tree    focusable_only=${True}    format=json
    Log    Focusable components only:${\n}${focusable}

    # Get focusable input fields
    ${focusable_inputs}=    Get Component Tree
    ...    types=JTextField,JTextArea
    ...    focusable_only=${True}
    ...    format=json
    Log    Focusable input fields:${\n}${focusable_inputs}

Combine Multiple State Filters
    [Documentation]    Use multiple state filters together
    [Tags]    filtering    state    combined

    # Get visible AND enabled components
    ${visible_enabled}=    Get Component Tree
    ...    visible_only=${True}
    ...    enabled_only=${True}
    ...    format=json
    Log    Visible AND enabled:${\n}${visible_enabled}

    # Get visible, enabled, focusable components
    ${all_states}=    Get Component Tree
    ...    visible_only=${True}
    ...    enabled_only=${True}
    ...    focusable_only=${True}
    ...    format=json
    Log    Visible, enabled, and focusable:${\n}${all_states}

Advanced Combined Filtering
    [Documentation]    Combine type and state filtering
    [Tags]    filtering    advanced

    # Get visible, enabled buttons
    ${active_buttons}=    Get Component Tree
    ...    types=JButton
    ...    visible_only=${True}
    ...    enabled_only=${True}
    ...    format=json
    Log    Active (visible + enabled) buttons:${\n}${active_buttons}

    # Get enabled text inputs (no labels)
    ${enabled_inputs}=    Get Component Tree
    ...    types=JText*
    ...    exclude_types=JLabel
    ...    enabled_only=${True}
    ...    format=json
    Log    Enabled text inputs:${\n}${enabled_inputs}

    # Get focusable, visible inputs with depth limit
    ${focused_inputs}=    Get Component Tree
    ...    types=JTextField,JTextArea
    ...    visible_only=${True}
    ...    focusable_only=${True}
    ...    max_depth=10
    ...    format=json
    Log    Focused inputs (depth 10):${\n}${focused_inputs}

Performance Test - Filtering vs Full Tree
    [Documentation]    Demonstrate performance benefit of filtering
    [Tags]    filtering    performance

    # Measure full tree retrieval time
    ${start}=    Get Time    epoch
    ${full_tree}=    Get Component Tree    format=text
    ${end}=    Get Time    epoch
    ${full_time}=    Evaluate    ${end} - ${start}
    ${full_len}=    Get Length    ${full_tree}
    Log    Full tree: ${full_len} chars in ${full_time}s

    # Measure filtered tree retrieval time
    ${start}=    Get Time    epoch
    ${filtered_tree}=    Get Component Tree
    ...    types=JButton,JTextField
    ...    visible_only=${True}
    ...    enabled_only=${True}
    ...    format=text
    ${end}=    Get Time    epoch
    ${filtered_time}=    Evaluate    ${end} - ${start}
    ${filtered_len}=    Get Length    ${filtered_tree}
    Log    Filtered tree: ${filtered_len} chars in ${filtered_time}s

    # Filtered should be smaller and potentially faster
    Should Be True    ${filtered_len} < ${full_len}
    Log    Size reduction: ${filtered_len}/${full_len} = ${filtered_len / ${full_len}}

Filtering With Depth Control
    [Documentation]    Combine filtering with depth limiting
    [Tags]    filtering    depth    performance

    # Get shallow tree of buttons
    ${shallow_buttons}=    Get Component Tree
    ...    types=JButton
    ...    max_depth=3
    ...    format=text
    Log    Shallow button tree (depth 3):${\n}${shallow_buttons}

    # Get deeper tree with filtering
    ${deep_filtered}=    Get Component Tree
    ...    types=JButton,JTextField
    ...    visible_only=${True}
    ...    max_depth=10
    ...    format=json
    Log    Deep filtered tree (depth 10):${\n}${deep_filtered}

Filtering With Subtree
    [Documentation]    Apply filtering to subtrees
    [Tags]    filtering    subtree

    # First, find a panel
    Run Keyword And Ignore Error    Click    JButton[text='Open']
    Sleep    500ms
    Refresh Component Tree

    # Get subtree with filtering
    ${filtered_subtree}=    Get Component Subtree
    ...    JPanel
    ...    types=JButton
    ...    visible_only=${True}
    ...    format=json
    Log    Filtered subtree:${\n}${filtered_subtree}

Real World Example - Debug Clickable Elements
    [Documentation]    Practical example: Find all clickable elements
    [Tags]    filtering    real-world

    # Get all visible, enabled, focusable buttons
    ${clickable}=    Get Component Tree
    ...    types=J*Button
    ...    visible_only=${True}
    ...    enabled_only=${True}
    ...    focusable_only=${True}
    ...    format=json
    Log    All clickable buttons:${\n}${clickable}

    # Parse to get count
    ${data}=    Evaluate    json.loads('''${clickable}''')    modules=json
    Log    Found clickable elements

Real World Example - Form Input Fields
    [Documentation]    Practical example: Find all editable form fields
    [Tags]    filtering    real-world

    # Get all visible, enabled text input fields
    ${inputs}=    Get Component Tree
    ...    types=JTextField,JTextArea,JPasswordField
    ...    visible_only=${True}
    ...    enabled_only=${True}
    ...    format=json
    Log    All form input fields:${\n}${inputs}

Real World Example - UI Complexity Analysis
    [Documentation]    Practical example: Analyze UI complexity by component type
    [Tags]    filtering    real-world    analysis

    # Count different component types
    ${buttons}=    Get Component Tree    types=JButton    format=text
    ${button_count}=    Get Line Count    ${buttons}

    ${labels}=    Get Component Tree    types=JLabel    format=text
    ${label_count}=    Get Line Count    ${labels}

    ${panels}=    Get Component Tree    types=JPanel    format=text
    ${panel_count}=    Get Line Count    ${panels}

    Log    UI Complexity:
    Log    - Buttons: ${button_count}
    Log    - Labels: ${label_count}
    Log    - Panels: ${panel_count}
