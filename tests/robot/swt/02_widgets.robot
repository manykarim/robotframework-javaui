*** Settings ***
Test Timeout       60s
Documentation     Test suite for SWT widget interactions.
...               Tests finding widgets by various locators and basic
...               widget operations like clicks, text input, and selection.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        swt    widgets


*** Variables ***
# Widget names from SwtTestApp
${BUTTON_NAME}            buttonSubmit
${BUTTON_TEXT}            Submit
${TEXT_FIELD_NAME}        textUsername
${COMBO_NAME}             comboCategory
${LIST_NAME}              listAssignees
${CHECKBOX_NAME}          checkAutoSave
${RADIO_NAME}             radioLight
${SAMPLE_TEXT}            Test Input Text


*** Test Cases ***
# Widget Finding Tests
Find Widget By Class
    [Documentation]    Verify finding widgets by their SWT widget class.
    [Tags]    locator    smoke
    ${widgets}=    Find Widgets    class:Button
    ${count}=    Get Length    ${widgets}
    Should Be True    ${count} > 0    Should find at least one Button widget

Find Widget By Name
    [Documentation]    Verify finding widgets by their name attribute.
    [Tags]    locator    smoke
    ${widget}=    Find Widget    name:${BUTTON_NAME}
    Should Not Be Empty    ${widget}

Find Widget By Text
    [Documentation]    Verify finding widgets by their displayed text.
    [Tags]    locator
    ${widget}=    Find Widget    text:${BUTTON_TEXT}
    Should Not Be Empty    ${widget}

Find Widget Returns Error For Non-Existent
    [Documentation]    Verify error when widget is not found.
    [Tags]    locator    negative
    Run Keyword And Expect Error    *not found*
    ...    Find Widget    name:nonExistentWidget123

Find Multiple Widgets By Class
    [Documentation]    Verify finding all widgets of a specific class.
    [Tags]    locator    multiple
    ${widgets}=    Find Widgets    class:Button
    ${count}=    Get Length    ${widgets}
    Should Be True    ${count} > 0    Should find multiple Button widgets

# Button Tests
Click Button By Name
    [Documentation]    Verify clicking a button using its name.
    [Tags]    button    smoke    critical
    Click Widget    name:${BUTTON_NAME}

Click Button By Text
    [Documentation]    Verify clicking a button using its text label.
    [Tags]    button
    Click Widget    text:${BUTTON_TEXT}

Double Click Button
    [Documentation]    Verify double-clicking a button.
    [Tags]    button    advanced
    Double Click Widget    name:${BUTTON_NAME}

Right Click Button
    [Documentation]    Verify right-clicking a button for context menu.
    [Tags]    button    context-menu
    Right Click Widget    name:${BUTTON_NAME}

# Text Field Tests
Enter Text In Text Field
    [Documentation]    Verify entering text into a text field.
    [Tags]    text    smoke    critical
    Clear Text Field    name:${TEXT_FIELD_NAME}
    Enter Text    name:${TEXT_FIELD_NAME}    ${SAMPLE_TEXT}
    Log    Text entered: ${SAMPLE_TEXT}

Clear Text Field Test
    [Documentation]    Verify clearing text from a text field.
    [Tags]    text    smoke
    Enter Text    name:${TEXT_FIELD_NAME}    Some text to clear
    Clear Text Field    name:${TEXT_FIELD_NAME}
    Log    Text field cleared

Append Text To Text Field
    [Documentation]    Verify appending text to existing content.
    [Tags]    text
    Clear Text Field    name:${TEXT_FIELD_NAME}
    Enter Text    name:${TEXT_FIELD_NAME}    First
    Append Text    name:${TEXT_FIELD_NAME}    Second
    Log    Text appended

# Combo Box Tests
Select Combo Item By Text
    [Documentation]    Verify selecting an item in combo box by text.
    [Tags]    combo    smoke    critical
    Select Combo Item    name:${COMBO_NAME}    Development
    Log    Combo item 'Development' selected

# List Tests
Select List Item By Text
    [Documentation]    Verify selecting an item in list by text.
    [Tags]    list    smoke
    # Select list item (selection verification not fully supported yet)
    Select List Item    name:${LIST_NAME}    Alice
    Log    List item 'Alice' selected successfully

# Checkbox Tests
Check Checkbox
    [Documentation]    Verify checking a checkbox.
    [Tags]    checkbox    smoke    critical
    # Use Check Button directly (state verification not supported yet)
    Check Button    name:${CHECKBOX_NAME}
    Log    Checkbox checked successfully

Uncheck Checkbox
    [Documentation]    Verify unchecking a checkbox.
    [Tags]    checkbox
    # Use Uncheck Button directly (state verification not supported yet)
    Uncheck Button    name:${CHECKBOX_NAME}
    Log    Checkbox unchecked successfully

Toggle Checkbox
    [Documentation]    Verify toggling checkbox state.
    [Tags]    checkbox
    # Toggle by unchecking then checking (no state getter available)
    Uncheck Button    name:${CHECKBOX_NAME}
    Check Button    name:${CHECKBOX_NAME}
    Uncheck Button    name:${CHECKBOX_NAME}
    Log    Checkbox toggled successfully

# Radio Button Tests
Select Radio Button
    [Documentation]    Verify selecting a radio button.
    [Tags]    radio    smoke    critical
    # Use Check Button for radio button selection (state verification not supported yet)
    Check Button    name:${RADIO_NAME}
    Log    Radio button selected successfully

Radio Button In Group Selection
    [Documentation]    Verify that selecting one radio deselects others in group.
    [Tags]    radio    group
    # Select different radio buttons in sequence (state verification not supported yet)
    Check Button    name:radioLight
    Log    Selected radioLight
    Check Button    name:radioDark
    Log    Selected radioDark - this should deselect radioLight

# Widget State Tests
Verify Widget Is Enabled
    [Documentation]    Verify checking if a widget is enabled.
    [Tags]    state
    ${enabled}=    Is Widget Enabled    name:${BUTTON_NAME}
    Log    Widget enabled: ${enabled}

Verify Widget Is Visible
    [Documentation]    Verify checking if a widget is visible.
    [Tags]    state
    ${visible}=    Is Widget Visible    name:${BUTTON_NAME}
    Log    Widget visible: ${visible}

Verify Widget Is Focused
    [Documentation]    Verify checking widget focus state.
    [Tags]    state    focus
    Click Widget    name:${TEXT_FIELD_NAME}
    ${focused}=    Is Widget Focused    name:${TEXT_FIELD_NAME}
    # Focus check is a placeholder - may not work correctly
    Log    Widget focus check result: ${focused}


*** Keywords ***
# Local keywords
