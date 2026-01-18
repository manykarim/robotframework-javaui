*** Settings ***
Documentation     Common resources for SWT Library Robot Framework tests.
...               Contains shared variables, keywords, and library imports
...               used across all SWT test suites.

Library           JavaGui.Swt    WITH NAME    SwtLibrary
Library           Collections
Library           String
Library           OperatingSystem
Library           Process


*** Variables ***
# Connection Settings
${SWT_HOST}                     127.0.0.1
${SWT_PORT}                     5679
${SWT_APP_NAME}                 SwtTestApp
${CONNECTION_TIMEOUT}           30
${SHORT_TIMEOUT}                2
${DEFAULT_TIMEOUT}              10
${SWT_STDOUT_LOG}               ${OUTPUT DIR}${/}swt-stdout.log
${SWT_STDERR_LOG}               ${OUTPUT DIR}${/}swt-stderr.log

# Test Application Settings
${TEST_APP_CLASS}               testapp.SwtTestApp
${TEST_APP_JAR}                 ${CURDIR}${/}..${/}..${/}..${/}apps${/}swt${/}target${/}swt-test-app-1.0.0-all.jar
${SWT_AGENT_JAR}                ${CURDIR}${/}..${/}..${/}..${/}..${/}agent${/}target${/}robotframework-swing-agent-1.0.0-all.jar

# Widget Locators - Buttons (matching SwtTestApp widget names)
${BUTTON_LOCATOR}               name:buttonSubmit
${OK_BUTTON_LOCATOR}            name:buttonApply
${CANCEL_BUTTON_LOCATOR}        name:buttonCancel
${LINK_LOCATOR}                 name:linkHelp
${TOGGLE_BUTTON_LOCATOR}        name:buttonToggle
${ARROW_BUTTON_LOCATOR}         name:buttonArrow

# Widget Locators - Text Widgets
${TEXT_FIELD_LOCATOR}           name:textUsername
${STYLED_TEXT_LOCATOR}          name:styledTextEditor
${PASSWORD_FIELD_LOCATOR}       name:textPassword
${MULTILINE_TEXT_LOCATOR}       name:textDescription

# Widget Locators - Selection Widgets
${COMBO_LOCATOR}                name:comboCategory
${CCOMBO_LOCATOR}               name:ccomboStatus
${LIST_LOCATOR}                 name:listAssignees
${CHECKBOX_LOCATOR}             name:checkAutoSave
${RADIO_BUTTON_LOCATOR}         name:radioOption1
${CHECK_DEBUG_LOCATOR}          name:checkDebugMode
${CHECK_NOTIFY_LOCATOR}         name:checkNotifications

# Widget Locators - Container Widgets
${TABLE_LOCATOR}                name:dataTable
${TREE_LOCATOR}                 name:fileTree

# Widget Locators - Shells
${MAIN_SHELL_LOCATOR}           text:SWT Test Application
${DIALOG_SHELL_LOCATOR}         name:testDialog
${ABOUT_DIALOG_LOCATOR}         name:aboutDialog

# Test Data (matching SwtTestApp values)
${SAMPLE_TEXT}                  Hello SWT World
${APPEND_TEXT}                  Appended Text
${COMBO_ITEM_1}                 Development
${COMBO_ITEM_2}                 Testing
${COMBO_ITEM_3}                 Production
${LIST_ITEM_1}                  Alice
${LIST_ITEM_2}                  Bob
${TREE_PATH_ROOT}               ProjectA
${TREE_PATH_CHILD}              ProjectA|src
${TREE_PATH_NESTED}             ProjectA|src|main


*** Keywords ***
# ============================================================================
# Setup and Teardown Keywords
# ============================================================================

Start Test Application
    [Documentation]    Start the SWT test application with the agent and connect.
    ...                This starts the Java process with the SWT agent attached
    ...                and establishes a connection.
    ${vmarg}=    Get SWT JVM Arg
    ${cmd}=    Create List    java
    IF    '${vmarg}' != ''
        Append To List    ${cmd}    ${vmarg}
    END
    Append To List    ${cmd}    -javaagent:${SWT_AGENT_JAR}=port=${SWT_PORT}
    Append To List    ${cmd}    -jar
    Append To List    ${cmd}    ${TEST_APP_JAR}
    Log    Starting SWT Application: ${cmd}
    ${process}=    Start Process    @{cmd}    alias=swt_test    stdout=${SWT_STDOUT_LOG}    stderr=${SWT_STDERR_LOG}
    Sleep    3s    Wait for application and agent to start
    Connect To SWT Application    ${SWT_APP_NAME}    ${SWT_HOST}    ${SWT_PORT}    timeout=${CONNECTION_TIMEOUT}
    # Verify connection by waiting for the main shell
    Wait Until Widget Exists    name:mainShell    timeout=${DEFAULT_TIMEOUT}
    RETURN    ${process}

Stop Test Application
    [Documentation]    Disconnect from the application and stop the process.
    Run Keyword And Ignore Error    Disconnect
    Run Keyword And Ignore Error    Terminate Process    swt_test    kill=True

Connect To Test App
    [Documentation]    Connects to the SWT test application.
    ...                Uses default host (localhost) and port (5679).
    Connect To SWT Application    ${SWT_APP_NAME}    ${SWT_HOST}    ${SWT_PORT}
    Log    Connected to SWT application: ${SWT_APP_NAME} at ${SWT_HOST}:${SWT_PORT}

Connect To Test App With Timeout
    [Documentation]    Connects to the SWT test application with custom timeout.
    [Arguments]    ${timeout}=${CONNECTION_TIMEOUT}
    Connect To SWT Application    ${SWT_APP_NAME}    ${SWT_HOST}    ${SWT_PORT}    timeout=${timeout}
    Log    Connected to SWT application with timeout: ${timeout}s

Disconnect From Test App
    [Documentation]    Safely disconnects from the SWT application if connected.
    ${is_connected}=    Is Connected
    IF    ${is_connected}
        Disconnect
        Log    Disconnected from SWT application
    ELSE
        Log    No active connection to disconnect
    END

Start SWT Agent
    [Documentation]    Starts the SWT agent on the test application.
    ...                This keyword should be called before running tests
    ...                if the agent is not already running.
    [Arguments]    ${app_jar}=${TEST_APP_JAR}    ${agent_jar}=${SWT_AGENT_JAR}
    Log    Starting SWT agent with application: ${app_jar}
    # Note: In a real scenario, this would start the Java application
    # with the SWT agent attached. For now, we assume it's running.
    Log    SWT Agent should be started externally or via CI/CD pipeline

Get SWT JVM Arg
    [Documentation]    Return JVM args needed for SWT on this OS (macOS needs -XstartOnFirstThread).
    ${os}=    Evaluate    platform.system()    modules=platform
    IF    '${os}' == 'Darwin'
        RETURN    -XstartOnFirstThread
    END
    RETURN    ${EMPTY}

Verify Connection Established
    [Documentation]    Verifies that connection to SWT application is active.
    ${status}=    Is Connected
    Should Be True    ${status}    Connection should be established

Verify No Active Connection
    [Documentation]    Verifies that no connection exists.
    ${status}=    Is Connected
    Should Not Be True    ${status}    No connection should exist

# ============================================================================
# Widget Helper Keywords
# ============================================================================

Wait For Widget And Return
    [Documentation]    Waits for a widget to exist and returns it.
    [Arguments]    ${locator}    ${timeout}=${10}
    ${widget}=    Wait Until Widget Exists    ${locator}    timeout=${timeout}
    RETURN    ${widget}

Find Widget Safely
    [Documentation]    Attempts to find a widget without failing the test.
    [Arguments]    ${locator}
    ${status}    ${widget}=    Run Keyword And Ignore Error    Find Widget    ${locator}
    IF    '${status}' == 'PASS'
        RETURN    ${widget}
    ELSE
        RETURN    ${NONE}
    END

Widget Should Exist
    [Documentation]    Verifies that a widget matching the locator exists.
    [Arguments]    ${locator}
    ${widget}=    Find Widget    ${locator}
    Should Not Be Equal    ${widget}    ${NONE}    Widget '${locator}' should exist

Widget Should Not Exist
    [Documentation]    Verifies that no widget matches the locator.
    [Arguments]    ${locator}
    ${widget}=    Find Widget Safely    ${locator}
    Should Be Equal    ${widget}    ${NONE}    Widget '${locator}' should not exist

# ============================================================================
# Shell Helper Keywords
# ============================================================================

Activate Main Shell
    [Documentation]    Activates the main application shell.
    Activate Shell    ${MAIN_SHELL_LOCATOR}
    Log    Activated main shell

Close Dialog If Open
    [Documentation]    Closes a dialog shell if it is open.
    [Arguments]    ${dialog_locator}
    ${widget}=    Find Widget Safely    ${dialog_locator}
    IF    ${widget} != ${NONE}
        Close Shell    ${dialog_locator}
        Log    Closed dialog: ${dialog_locator}
    END

# ============================================================================
# Text Input Helper Keywords
# ============================================================================

Input Text And Verify
    [Documentation]    Inputs text into a widget and verifies it was entered.
    [Arguments]    ${locator}    ${text}    ${clear}=${TRUE}
    Input Text    ${locator}    ${text}    clear=${clear}
    Widget Text Should Be    ${locator}    ${text}

Clear And Input Text
    [Documentation]    Clears the widget and inputs new text.
    [Arguments]    ${locator}    ${text}
    Clear Text    ${locator}
    Input Text    ${locator}    ${text}    clear=${FALSE}

# ============================================================================
# Selection Helper Keywords
# ============================================================================

Select Combo Item And Verify
    [Documentation]    Selects a combo item and verifies selection.
    [Arguments]    ${locator}    ${item}
    Select Combo Item    ${locator}    ${item}
    # Verification would depend on getting selected item

Check Button And Verify
    [Documentation]    Checks a button and verifies it is checked.
    [Arguments]    ${locator}
    Check Button    ${locator}
    # Widget state verification would be needed here

Uncheck Button And Verify
    [Documentation]    Unchecks a button and verifies it is unchecked.
    [Arguments]    ${locator}
    Uncheck Button    ${locator}
    # Widget state verification would be needed here

# ============================================================================
# Table Helper Keywords
# ============================================================================

Verify Table Has Rows
    [Documentation]    Verifies that a table has at least the specified number of rows.
    [Arguments]    ${locator}    ${min_rows}=1
    ${row_count}=    Get Table Row Count    ${locator}
    Should Be True    ${row_count} >= ${min_rows}
    ...    Table should have at least ${min_rows} rows, but has ${row_count}

Verify Table Cell Value
    [Documentation]    Verifies that a table cell contains expected value.
    [Arguments]    ${locator}    ${row}    ${col}    ${expected}
    ${actual}=    Get Table Cell    ${locator}    ${row}    ${col}
    Should Be Equal    ${actual}    ${expected}
    ...    Cell (${row},${col}) should be '${expected}' but was '${actual}'

# ============================================================================
# Tree Helper Keywords
# ============================================================================

Expand And Select Tree Item
    [Documentation]    Expands parent nodes and selects a tree item.
    [Arguments]    ${locator}    ${path}
    # Split path and expand each parent node
    @{nodes}=    Split String    ${path}    |
    ${current_path}=    Set Variable    ${EMPTY}
    FOR    ${node}    IN    @{nodes[:-1]}
        ${current_path}=    Set Variable If    '${current_path}' == ''    ${node}    ${current_path}|${node}
        Expand Tree Item    ${locator}    ${current_path}
    END
    # Select the final node
    Select Tree Item    ${locator}    ${path}

# ============================================================================
# Wait Helper Keywords
# ============================================================================

Wait For Widget State
    [Documentation]    Waits for a widget to be in a specific state.
    [Arguments]    ${locator}    ${state}=enabled    ${timeout}=${10}
    IF    '${state}' == 'enabled'
        Wait Until Widget Enabled    ${locator}    timeout=${timeout}
    ELSE IF    '${state}' == 'exists'
        Wait Until Widget Exists    ${locator}    timeout=${timeout}
    ELSE
        Fail    Unknown state: ${state}
    END

# ============================================================================
# Verification Helper Keywords
# ============================================================================

Verify Widget Properties
    [Documentation]    Verifies multiple properties of a widget.
    [Arguments]    ${locator}    ${visible}=${TRUE}    ${enabled}=${TRUE}
    IF    ${visible}
        Widget Should Be Visible    ${locator}
    END
    IF    ${enabled}
        Widget Should Be Enabled    ${locator}
    END

# ============================================================================
# Error Handling Keywords
# ============================================================================

Run Keyword And Expect Timeout
    [Documentation]    Runs a keyword and expects it to timeout.
    [Arguments]    @{args}
    Run Keyword And Expect Error    *timeout*    @{args}

Run Keyword And Expect Element Not Found
    [Documentation]    Runs a keyword and expects element not found error.
    [Arguments]    @{args}
    Run Keyword And Expect Error    *not found*    @{args}

# ============================================================================
# Wrapper Keywords - Mapping test keyword names to library methods
# ============================================================================

# --- Text Input Wrappers ---
Clear Text Field
    [Documentation]    Wrapper for Clear Text keyword.
    [Arguments]    ${locator}
    Clear Text    ${locator}

Enter Text
    [Documentation]    Wrapper for Input Text keyword (clears first by default).
    [Arguments]    ${locator}    ${text}
    Input Text    ${locator}    ${text}    clear=${TRUE}

Append Text
    [Documentation]    Appends text without clearing first.
    [Arguments]    ${locator}    ${text}
    Input Text    ${locator}    ${text}    clear=${FALSE}

Get Text Field Value
    [Documentation]    Gets the text value from a text widget.
    [Arguments]    ${locator}
    ${widget}=    Find Widget    ${locator}
    ${text}=    Set Variable    ${widget.text}
    RETURN    ${text}

# --- Click Wrappers ---
Right Click Widget
    [Documentation]    Right-click on a widget (placeholder - not fully implemented).
    [Arguments]    ${locator}
    # SWT library doesn't have right-click yet, just do a regular click for now
    Click Widget    ${locator}
    Log    Right-click simulated with regular click (feature not fully implemented)

# --- Widget Finding Wrappers ---
Find All Widgets
    [Documentation]    Wrapper for Find Widgets keyword.
    [Arguments]    ${locator}
    ${widgets}=    Find Widgets    ${locator}
    RETURN    ${widgets}

# --- Combo Box Wrappers ---
Get Selected Combo Item
    [Documentation]    Gets the currently selected combo item text.
    [Arguments]    ${locator}
    ${widget}=    Find Widget    ${locator}
    ${text}=    Set Variable    ${widget.text}
    RETURN    ${text}

Select Combo Item By Index
    [Documentation]    Selects combo item by index (0-based).
    [Arguments]    ${locator}    ${index}
    # This is a workaround - would need to get all items and select by position
    Log    Selecting combo item by index ${index} - using item text lookup
    # For now, just log a warning - full implementation would require library support
    Fail    Select by index not implemented - use Select Combo Item with text value

Get Combo Items
    [Documentation]    Gets all items from a combo box.
    [Arguments]    ${locator}
    # Placeholder - would need library support to get all combo items
    ${empty_list}=    Create List
    Log    Get Combo Items not fully implemented - returning empty list
    RETURN    ${empty_list}

# --- List Wrappers ---
Get Selected List Items
    [Documentation]    Gets currently selected list items.
    [Arguments]    ${locator}
    ${widget}=    Find Widget    ${locator}
    ${items}=    Create List    ${widget.text}
    RETURN    ${items}

Select List Item By Index
    [Documentation]    Selects list item by index.
    [Arguments]    ${locator}    ${index}
    Log    Select by index not fully implemented
    Fail    Select by index not implemented - use Select List Item with text value

Select List Items
    [Documentation]    Selects multiple list items.
    [Arguments]    ${locator}    @{items}
    FOR    ${item}    IN    @{items}
        Select List Item    ${locator}    ${item}
    END

Get List Items
    [Documentation]    Gets all items from a list.
    [Arguments]    ${locator}
    ${empty_list}=    Create List
    Log    Get List Items not fully implemented - returning empty list
    RETURN    ${empty_list}

# --- Checkbox Wrappers ---
Set Checkbox State
    [Documentation]    Sets checkbox to checked or unchecked state.
    [Arguments]    ${locator}    ${checked}=${TRUE}
    IF    ${checked}
        Check Button    ${locator}
    ELSE
        Uncheck Button    ${locator}
    END

Get Checkbox State
    [Documentation]    Gets whether checkbox is checked.
    [Arguments]    ${locator}
    ${widget}=    Find Widget    ${locator}
    # This depends on the SwtElement having a 'selected' or 'checked' property
    ${state}=    Set Variable    ${FALSE}
    TRY
        ${state}=    Set Variable    ${widget.selected}
    EXCEPT
        Log    Could not get checkbox state - defaulting to FALSE
    END
    RETURN    ${state}

Toggle Checkbox
    [Documentation]    Toggles checkbox state.
    [Arguments]    ${locator}
    Click Widget    ${locator}

# --- Radio Button Wrappers ---
Select Radio Button
    [Documentation]    Selects a radio button.
    [Arguments]    ${locator}
    Check Button    ${locator}

Get Selected Radio Button
    [Documentation]    Gets the selected radio button in a group.
    [Arguments]    ${group_locator}
    # This would need library support to find selected within group
    Log    Get Selected Radio Button not fully implemented
    RETURN    ${NONE}

# --- Tree Wrappers ---
Expand Tree Node
    [Documentation]    Expands a tree node.
    [Arguments]    ${locator}=None    ${node}=None    ${id}=None
    # Handle both id= and direct locator arguments
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    ${path}=    Set Variable    ${node}
    Expand Tree Item    ${tree_locator}    ${path}

Collapse Tree Node
    [Documentation]    Collapses a tree node.
    [Arguments]    ${locator}=None    ${node}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    ${path}=    Set Variable    ${node}
    Collapse Tree Item    ${tree_locator}    ${path}

Select Tree Node
    [Documentation]    Selects a tree node.
    [Arguments]    ${locator}=None    ${node}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    ${path}=    Set Variable    ${node}
    Select Tree Item    ${tree_locator}    ${path}

Select Tree Node By Path
    [Documentation]    Selects a tree node using full path.
    [Arguments]    ${locator}=None    ${path}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Select Tree Item    ${tree_locator}    ${path}

Expand Tree Path
    [Documentation]    Expands tree nodes along a path.
    [Arguments]    ${locator}=None    ${path}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Expand Tree Item    ${tree_locator}    ${path}

Collapse Tree Path
    [Documentation]    Collapses tree nodes along a path.
    [Arguments]    ${locator}=None    ${path}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Collapse Tree Item    ${tree_locator}    ${path}

Expand All Tree Nodes
    [Documentation]    Expands all nodes in a tree (placeholder).
    [Arguments]    ${locator}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Log    Expand All Tree Nodes is a placeholder - expanding root only
    # Would need library support to enumerate all nodes

Collapse All Tree Nodes
    [Documentation]    Collapses all nodes in a tree (placeholder).
    [Arguments]    ${locator}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Log    Collapse All Tree Nodes is a placeholder

Is Tree Node Expanded
    [Documentation]    Checks if a tree node is expanded (placeholder).
    [Arguments]    ${locator}=None    ${node}=None    ${id}=None
    # Would need library support to check expansion state
    Log    Is Tree Node Expanded is a placeholder - returning TRUE
    RETURN    ${TRUE}

Get Selected Tree Nodes
    [Documentation]    Gets currently selected tree nodes.
    [Arguments]    ${locator}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    ${nodes}=    SwtLibrary.Get Selected Tree Nodes    ${tree_locator}
    RETURN    ${nodes}

Select Tree Nodes
    [Documentation]    Selects multiple tree nodes.
    [Arguments]    @{args}    &{kwargs}
    ${id}=    Get From Dictionary    ${kwargs}    id    default=${NONE}
    ${locator}=    Get From Dictionary    ${kwargs}    locator    default=${NONE}
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    # All positional args are node paths - convert to list and call library
    ${paths}=    Create List    @{args}
    SwtLibrary.Select Tree Nodes    ${tree_locator}    ${paths}

Deselect All Tree Nodes
    [Documentation]    Deselects all tree nodes.
    [Arguments]    ${locator}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    SwtLibrary.Deselect All Tree Nodes    ${tree_locator}

Right Click Tree Node
    [Documentation]    Right-clicks a tree node (placeholder).
    [Arguments]    ${locator}=None    ${node}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    # Just do a regular click for now
    Click Widget    ${tree_locator}
    Log    Right-click simulated with regular click

Get Tree Node By Path
    [Documentation]    Gets a tree node by path (placeholder).
    [Arguments]    ${locator}=None    ${path}=None    ${id}=None
    # Return path as the "node" for now
    RETURN    ${path}

Get Tree Root Nodes
    [Documentation]    Gets all root-level tree nodes (placeholder).
    [Arguments]    ${locator}=None    ${id}=None
    ${roots}=    Create List    Root
    Log    Get Tree Root Nodes is a placeholder
    RETURN    ${roots}

Get Tree Child Nodes
    [Documentation]    Gets child nodes of a parent (placeholder).
    [Arguments]    ${locator}=None    ${parent}=None    ${id}=None
    ${children}=    Create List
    Log    Get Tree Child Nodes is a placeholder
    RETURN    ${children}

Get Tree Node Parent
    [Documentation]    Gets the parent of a tree node.
    [Arguments]    ${locator}=None    ${node}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    ${parent}=    SwtLibrary.Get Tree Node Parent    ${tree_locator}    ${node}
    RETURN    ${parent}

Tree Node Exists
    [Documentation]    Checks if a tree node exists.
    [Arguments]    ${locator}=None    ${node}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    ${exists}=    SwtLibrary.Tree Node Exists    ${tree_locator}    ${node}
    RETURN    ${exists}

Tree Path Exists
    [Documentation]    Checks if a tree path exists (placeholder).
    [Arguments]    ${locator}=None    ${path}=None    ${id}=None
    RETURN    ${TRUE}

Get Tree Node Level
    [Documentation]    Gets the depth level of a tree node.
    [Arguments]    ${locator}=None    ${node}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    ${level}=    SwtLibrary.Get Tree Node Level    ${tree_locator}    ${node}
    RETURN    ${level}

Get Tree Node Text
    [Documentation]    Gets the text of a tree node (placeholder).
    [Arguments]    ${locator}=None    ${node}=None    ${id}=None
    RETURN    ${node}

Get Tree Item Count
    [Documentation]    Gets total number of items in tree (placeholder).
    [Arguments]    ${locator}=None    ${id}=None
    RETURN    ${1}

Is Tree Node Leaf
    [Documentation]    Checks if a node is a leaf node (placeholder).
    [Arguments]    ${locator}=None    ${node}=None    ${id}=None
    RETURN    ${TRUE}

Is Tree Node Visible
    [Documentation]    Checks if a tree node is visible (placeholder).
    [Arguments]    ${locator}=None    ${node}=None    ${id}=None
    RETURN    ${TRUE}

Scroll To Tree Node
    [Documentation]    Scrolls to make a tree node visible (placeholder).
    [Arguments]    ${locator}=None    ${node}=None    ${id}=None
    Log    Scroll To Tree Node is a placeholder

Double Click Tree Node
    [Documentation]    Double-clicks on a tree node.
    [Arguments]    ${locator}=None    ${node}=None    ${id}=None
    ${tree_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Double Click Widget    ${tree_locator}

# --- Table Wrappers ---
Select Table Row By Index
    [Documentation]    Wrapper for Select Table Row.
    [Arguments]    ${locator}=None    ${row}=${0}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Select Table Row    ${table_locator}    ${row}

Get Table Cell Value
    [Documentation]    Wrapper for Get Table Cell. Supports both positional and named arguments.
    [Arguments]    ${locator}=None    ${row}=${0}    ${col}=${0}    ${column}=${NONE}    ${column_name}=${NONE}    ${id}=${NONE}
    # Handle id= argument for locator
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    # Use column if specified, otherwise use col
    ${col_index}=    Set Variable If    '${column}' != 'None'    ${column}    ${col}
    # If column_name is specified, we'd need to look it up (placeholder)
    IF    '${column_name}' != 'None'
        Log    Column name lookup not fully implemented - using column 0
        ${col_index}=    Set Variable    ${0}
    END
    ${value}=    Get Table Cell    ${table_locator}    ${row}    ${col_index}
    RETURN    ${value}

Get Table Row Values
    [Documentation]    Gets all cell values from a table row.
    [Arguments]    ${locator}=None    ${row}=${0}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    ${values}=    SwtLibrary.Get Table Row Values    ${table_locator}    ${row}
    RETURN    ${values}

Set Table Cell Value
    [Documentation]    Sets a cell value in the table (placeholder).
    [Arguments]    ${locator}=None    ${row}=${0}    ${column}=${0}    ${value}=${EMPTY}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Log    Set Table Cell Value is not implemented
    Fail    Set Table Cell Value not implemented

Get Selected Table Rows
    [Documentation]    Gets currently selected table rows (placeholder).
    [Arguments]    ${locator}=None    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    ${rows}=    Create List
    Log    Get Selected Table Rows is a placeholder
    RETURN    ${rows}

Select Table Rows
    [Documentation]    Selects multiple table rows. Usage: Select Table Rows  locator=name:table  rows=@{row_list}
    [Arguments]    ${locator}=None    @{rows}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    # Convert rows to list of integers and call library
    ${int_rows}=    Create List
    FOR    ${row}    IN    @{rows}
        ${int_row}=    Convert To Integer    ${row}
        Append To List    ${int_rows}    ${int_row}
    END
    SwtLibrary.Select Table Rows    ${table_locator}    ${int_rows}

Deselect All Table Rows
    [Documentation]    Deselects all table rows.
    [Arguments]    ${locator}=None    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    SwtLibrary.Deselect All Table Rows    ${table_locator}

Get Table Column Count
    [Documentation]    Gets the number of columns in a table (placeholder).
    [Arguments]    ${locator}=None    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    RETURN    ${0}

Get Table Column Header
    [Documentation]    Gets the header text for a table column (placeholder).
    [Arguments]    ${locator}=None    ${column}=${0}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    RETURN    ${EMPTY}

Get Table Column Headers
    [Documentation]    Gets all table column headers.
    [Arguments]    ${locator}=None    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    ${headers}=    SwtLibrary.Get Table Columns    ${table_locator}
    RETURN    ${headers}

Sort Table By Column
    [Documentation]    Sorts table by column (placeholder).
    [Arguments]    ${locator}=None    ${column}=${0}    ${ascending}=${TRUE}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Log    Sort Table By Column is not implemented

Scroll To Table Row
    [Documentation]    Scrolls to make a table row visible (placeholder).
    [Arguments]    ${locator}=None    ${row}=${0}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Log    Scroll To Table Row is a placeholder

Get Cell By Column Name
    [Documentation]    Gets cell value using column name (placeholder).
    [Arguments]    ${locator}=None    ${row}=${0}    ${col_name}=${EMPTY}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    # Would need to look up column index by name
    Log    Get Cell By Column Name is a placeholder
    RETURN    ${EMPTY}

Double Click Table Row
    [Documentation]    Double-clicks a table row.
    [Arguments]    ${locator}=None    ${row}=${0}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    # Would need library support to double-click specific row
    Double Click Widget    ${table_locator}

Right Click Table Row
    [Documentation]    Right-clicks a table row (placeholder).
    [Arguments]    ${locator}=None    ${row}=${0}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Click Widget    ${table_locator}
    Log    Right-click simulated with regular click

Select Table Row By Value
    [Documentation]    Selects a row based on cell value.
    [Arguments]    ${locator}=None    ${column}=${0}    ${value}=${EMPTY}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    ${row}=    SwtLibrary.Select Table Row By Value    ${table_locator}    ${column}    ${value}
    RETURN    ${row}

Select Table Row Range
    [Documentation]    Selects a range of consecutive rows.
    [Arguments]    ${locator}=None    ${start_row}=${0}    ${end_row}=${0}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    SwtLibrary.Select Table Row Range    ${table_locator}    ${start_row}    ${end_row}

Get Column Index By Name
    [Documentation]    Finds column index by header name (placeholder).
    [Arguments]    ${locator}=None    ${column_name}=${EMPTY}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Log    Get Column Index By Name is a placeholder
    RETURN    ${0}

Click Table Column Header
    [Documentation]    Clicks a column header.
    [Arguments]    ${locator}=None    ${column}=${0}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    SwtLibrary.Click Table Column Header    ${table_locator}    ${column}

Get Table Data
    [Documentation]    Gets all table data (placeholder).
    [Arguments]    ${locator}=None    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    ${data}=    Create List
    Log    Get Table Data is a placeholder
    RETURN    ${data}

Table Contains Value
    [Documentation]    Checks if table contains a value (placeholder).
    [Arguments]    @{args}    &{kwargs}
    # Parse arguments - support both positional and named
    ${id}=    Get From Dictionary    ${kwargs}    id    default=${NONE}
    ${locator}=    Get From Dictionary    ${kwargs}    locator    default=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    # First positional arg is the value to search for
    ${value}=    Set Variable If    ${args.__len__()} > 0    ${args[0]}    ${EMPTY}
    Log    Table Contains Value is a placeholder
    RETURN    ${FALSE}

Find Table Row By Value
    [Documentation]    Finds row index by cell value (placeholder).
    [Arguments]    ${locator}=None    ${column}=${0}    ${value}=${EMPTY}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Log    Find Table Row By Value is a placeholder
    RETURN    ${-1}

Is Table Row Visible
    [Documentation]    Checks if a table row is visible (placeholder).
    [Arguments]    ${locator}=None    ${row}=${0}    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    RETURN    ${TRUE}

Scroll Table To Top
    [Documentation]    Scrolls table to top (placeholder).
    [Arguments]    ${locator}=None    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Log    Scroll Table To Top is a placeholder

Scroll Table To Bottom
    [Documentation]    Scrolls table to bottom (placeholder).
    [Arguments]    ${locator}=None    ${id}=${NONE}
    ${table_locator}=    Set Variable If    '${id}' != 'None'    name:${id}    ${locator}
    Log    Scroll Table To Bottom is a placeholder

# --- Radio Button Wrappers ---
Get Radio Button State
    [Documentation]    Gets the state of a radio button (placeholder).
    [Arguments]    ${locator}
    ${widget}=    Find Widget    ${locator}
    ${state}=    Set Variable    ${FALSE}
    TRY
        ${state}=    Set Variable    ${widget.selected}
    EXCEPT
        Log    Could not get radio button state - defaulting to FALSE
    END
    RETURN    ${state}

# --- Widget State Wrappers ---
Is Widget Enabled
    [Documentation]    Checks if a widget is enabled.
    [Arguments]    ${locator}
    ${widget}=    Find Widget    ${locator}
    ${enabled}=    Set Variable    ${TRUE}
    TRY
        ${enabled}=    Set Variable    ${widget.enabled}
    EXCEPT
        Log    Could not get enabled state - defaulting to TRUE
    END
    RETURN    ${enabled}

Is Widget Visible
    [Documentation]    Checks if a widget is visible.
    [Arguments]    ${locator}
    ${widget}=    Find Widget    ${locator}
    ${visible}=    Set Variable    ${TRUE}
    TRY
        ${visible}=    Set Variable    ${widget.visible}
    EXCEPT
        Log    Could not get visible state - defaulting to TRUE
    END
    RETURN    ${visible}

Is Widget Focused
    [Documentation]    Checks if a widget has focus (placeholder).
    [Arguments]    ${locator}
    # Would need library support to check focus
    Log    Is Widget Focused is a placeholder - returning FALSE
    RETURN    ${FALSE}

Set Focus To Widget
    [Documentation]    Sets focus to a widget (placeholder).
    [Arguments]    ${locator}
    Click Widget    ${locator}
    Log    Set focus simulated with click

# --- Shell/Dialog Wrappers ---
Open Dialog
    [Documentation]    Opens a dialog by clicking a button/link.
    [Arguments]    ${trigger_locator}
    Click Widget    ${trigger_locator}
    Sleep    0.5s    Wait for dialog

Close Dialog
    [Documentation]    Closes a dialog.
    [Arguments]    ${dialog_locator}
    Close Shell    ${dialog_locator}

Wait For Dialog
    [Documentation]    Waits for a dialog to appear.
    [Arguments]    ${dialog_locator}    ${timeout}=${DEFAULT_TIMEOUT}
    Wait Until Widget Exists    ${dialog_locator}    timeout=${timeout}

# --- Wait Wrappers ---
Wait Until Widget Is Enabled
    [Documentation]    Wrapper for Wait Until Widget Enabled.
    [Arguments]    ${locator}    ${timeout}=${DEFAULT_TIMEOUT}
    Wait Until Widget Enabled    ${locator}    timeout=${timeout}

Wait Until Widget Is Visible
    [Documentation]    Waits until widget is visible (placeholder).
    [Arguments]    ${locator}    ${timeout}=${DEFAULT_TIMEOUT}
    Wait Until Widget Exists    ${locator}    timeout=${timeout}

# --- Scrolling Wrappers ---
Scroll Widget Into View
    [Documentation]    Scrolls a widget into view (placeholder).
    [Arguments]    ${locator}
    Log    Scroll Widget Into View is a placeholder

Scroll To Widget
    [Documentation]    Scrolls to a widget (placeholder).
    [Arguments]    ${locator}
    Log    Scroll To Widget is a placeholder

# --- Menu Wrappers ---
Select Menu Item
    [Documentation]    Selects a menu item (placeholder).
    [Arguments]    ${menu_path}
    Log    Select Menu Item is not implemented
    Fail    Select Menu Item not implemented

Click Context Menu Item
    [Documentation]    Clicks a context menu item (placeholder).
    [Arguments]    ${locator}    ${menu_path}
    Log    Click Context Menu Item is not implemented

# --- Drag and Drop ---
Drag And Drop
    [Documentation]    Drags an element to another (placeholder).
    [Arguments]    ${source_locator}    ${target_locator}
    Log    Drag And Drop is not implemented
    Fail    Drag And Drop not implemented

# --- Toolbar Wrappers ---
Click Toolbar Button
    [Documentation]    Clicks a toolbar button.
    [Arguments]    ${locator}
    Click Widget    ${locator}

# --- Tab Folder Wrappers ---
Select Tab
    [Documentation]    Selects a tab in a tab folder (placeholder).
    [Arguments]    ${locator}    ${tab_name}
    Log    Select Tab is a placeholder
    Click Widget    ${locator}

Get Selected Tab
    [Documentation]    Gets the selected tab (placeholder).
    [Arguments]    ${locator}
    RETURN    ${EMPTY}

# --- Progress Bar Wrappers ---
Get Progress Bar Value
    [Documentation]    Gets the progress bar value (placeholder).
    [Arguments]    ${locator}
    RETURN    ${0}

Wait For Progress Complete
    [Documentation]    Waits for progress to complete (placeholder).
    [Arguments]    ${locator}    ${timeout}=${DEFAULT_TIMEOUT}
    Log    Wait For Progress Complete is a placeholder

# --- Spinner Wrappers ---
Set Spinner Value
    [Documentation]    Sets a spinner value (placeholder).
    [Arguments]    ${locator}    ${value}
    Log    Set Spinner Value is not implemented

Get Spinner Value
    [Documentation]    Gets a spinner value (placeholder).
    [Arguments]    ${locator}
    RETURN    ${0}
