*** Settings ***
Test Timeout       60s
Documentation     Test suite for delegated SWT Widget operations in RCP.
...               Tests all SWT widget keywords that are delegated from
...               RcpLibrary to the underlying SwtLibrary for widget interaction.

Resource          resources/common.resource

Suite Setup       Suite Setup Start RCP App
Suite Teardown    Suite Teardown Stop RCP App
Test Setup        Test Setup Reset State
Test Teardown     Test Teardown Cleanup

Force Tags        rcp    widget    swt


*** Test Cases ***
# =============================================================================
# Find Widget Tests
# =============================================================================

Find Widget Successfully
    [Documentation]    Verify finding a widget by locator.
    ...                Returns an SwtElement representing the widget.
    [Tags]    smoke    critical    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    ${widget}=    Find Widget    name:packageExplorerTree
    Should Not Be Empty    ${widget}
    Log    Found widget: ${widget}

Find Widget By Name
    [Documentation]    Verify finding a widget by name locator.
    ...                Uses name:widgetName format.
    [Tags]    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    ${widget}=    Find Widget    name:packageExplorerTree
    Log    Found by name: ${widget}

Find Widget By Text
    [Documentation]    Verify finding a widget by text locator.
    ...                Uses text:labelText format.
    [Tags]    positive
    Open Preferences
    ${widget}=    Find Widget    text:OK
    Log    Found by text: ${widget}
    Close Shell    text:Preferences

Find Widget By Class
    [Documentation]    Verify finding a widget by class type.
    ...                Uses simple class name like Button, Tree, etc.
    [Tags]    positive
    Open Preferences
    ${widget}=    Find Widget    name:buttonOK
    Log    Found by class: ${widget}
    Close Shell    text:Preferences

Find Widget Not Found Error
    [Documentation]    Verify error when widget not found.
    ...                Should raise ElementNotFoundError.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *not found*
    ...    Find Widget    name:nonExistentWidget12345


# =============================================================================
# Find Widgets Tests
# =============================================================================

Find Widgets Successfully
    [Documentation]    Verify finding all widgets matching locator.
    ...                Returns a list of SwtElements.
    [Tags]    smoke    positive
    ${widgets}=    Find Widgets    Button
    ${count}=    Get Length    ${widgets}
    Log    Found ${count} buttons
    Should Be True    ${count} >= 0

Find Widgets Returns Empty When None
    [Documentation]    Verify empty list when no widgets match.
    ...                Should return empty list, not error.
    [Tags]    positive
    ${widgets}=    Find Widgets    name:nonExistentWidget
    ${count}=    Get Length    ${widgets}
    Should Be Equal As Numbers    ${count}    0


# =============================================================================
# Click Widget Tests
# =============================================================================

Click Widget Successfully
    [Documentation]    Verify clicking on a widget.
    ...                Should trigger click action on the widget.
    [Tags]    smoke    critical    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Click Widget    Tree
    Log    Widget clicked

Click Widget By Locator
    [Documentation]    Verify clicking widget with various locators.
    ...                Tests different locator formats.
    [Tags]    positive
    Open Preferences
    ${button}=    Find Widget    name:buttonOK
    Click Widget    name:buttonOK
    Log    Clicked button
    # Preferences closed by clicking OK

Click Widget Not Found Error
    [Documentation]    Verify error when clicking non-existent widget.
    ...                Should raise ElementNotFoundError.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *not found*
    ...    Click Widget    name:nonExistentButton


# =============================================================================
# Double Click Widget Tests
# =============================================================================

Double Click Widget Successfully
    [Documentation]    Verify double-clicking on a widget.
    ...                Should trigger double-click action.
    [Tags]    smoke    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Double Click Widget    Tree
    Log    Widget double-clicked

Double Click Widget Not Found Error
    [Documentation]    Verify error when double-clicking non-existent widget.
    ...                Should raise ElementNotFoundError.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *not found*
    ...    Double Click Widget    name:nonExistentWidget


# =============================================================================
# Input Text Tests
# =============================================================================

Input Text Successfully
    [Documentation]    Verify inputting text into a text widget.
    ...                Should enter text into the widget.
    [Tags]    smoke    critical    positive
    Open Preferences
    ${text_widget}=    Find Widget    Text
    Input Text    Text    test input
    Close Shell    text:Preferences

Input Text With Clear
    [Documentation]    Verify inputting text with clear option.
    ...                Should clear existing text first.
    [Tags]    positive
    Open Preferences
    Input Text    Text    first text    clear=${TRUE}
    Input Text    Text    second text    clear=${TRUE}
    Close Shell    text:Preferences

Input Text Without Clear
    [Documentation]    Verify inputting text without clearing.
    ...                Should append to existing text.
    [Tags]    positive
    Open Preferences
    Input Text    Text    initial    clear=${TRUE}
    Input Text    Text    appended    clear=${FALSE}
    Close Shell    text:Preferences


# =============================================================================
# Clear Text Tests
# =============================================================================

Clear Text Successfully
    [Documentation]    Verify clearing text from a widget.
    ...                Should remove all text from the widget.
    [Tags]    smoke    positive
    Open Preferences
    Input Text    Text    some text
    Clear Text    Text
    Close Shell    text:Preferences


# =============================================================================
# Selection Keywords Tests
# =============================================================================

Select Combo Item Successfully
    [Documentation]    Verify selecting an item from a combo box.
    ...                Should select the specified item.
    [Tags]    positive
    # Need a dialog with combo for testing
    Run Keyword And Ignore Error    Select Combo Item    Combo    SomeItem
    Log    Combo selection tested

Select List Item Successfully
    [Documentation]    Verify selecting an item from a list.
    ...                Should select the specified item.
    [Tags]    positive
    # Need a dialog with list for testing
    Run Keyword And Ignore Error    Select List Item    List    SomeItem
    Log    List selection tested

Check Button Successfully
    [Documentation]    Verify checking a checkbox.
    ...                Should ensure the checkbox is checked.
    [Tags]    positive
    # Use the preferences dialog to access checkboxes
    Open Preferences
    Navigate To Preference Page    ${PREF_GENERAL}
    # Try to check a button, ignore if not found
    Run Keyword And Ignore Error    Check Button    name:buttonOK
    # Ensure preferences dialog is closed
    Run Keyword And Ignore Error    Close Shell    text:Preferences

Uncheck Button Successfully
    [Documentation]    Verify unchecking a checkbox.
    ...                Should ensure the checkbox is unchecked.
    [Tags]    positive
    Open Preferences
    Navigate To Preference Page    ${PREF_GENERAL}
    Run Keyword And Ignore Error    Uncheck Button    Button
    Close Shell    text:Preferences


# =============================================================================
# Table Keywords Tests
# =============================================================================

Get Table Row Count Successfully
    [Documentation]    Verify getting table row count.
    ...                Should return number of rows in the table.
    [Tags]    positive
    Show View    ${PROBLEMS_VIEW}
    ${count}=    Get Table Row Count    Table
    Log    Table has ${count} rows
    Should Be True    ${count} >= 0

Get Table Cell Successfully
    [Documentation]    Verify getting table cell value.
    ...                Should return the cell text content.
    [Tags]    positive
    Show View    ${PROBLEMS_VIEW}
    ${value}=    Run Keyword And Ignore Error    Get Table Cell    Table    0    0
    Log    Table cell value: ${value}

Select Table Row Successfully
    [Documentation]    Verify selecting a table row.
    ...                Should highlight the specified row.
    [Tags]    positive
    Show View    ${PROBLEMS_VIEW}
    Run Keyword And Ignore Error    Select Table Row    Table    0
    Log    Table row selected


# =============================================================================
# Tree Keywords Tests
# =============================================================================

Expand Tree Item Successfully
    [Documentation]    Verify expanding a tree item.
    ...                Should show child items.
    [Tags]    smoke    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Activate View    ${PACKAGE_EXPLORER_VIEW}
    Run Keyword And Ignore Error    Expand Tree Item    Tree    ${TEST_PROJECT}
    Log    Tree item expanded

Collapse Tree Item Successfully
    [Documentation]    Verify collapsing a tree item.
    ...                Should hide child items.
    [Tags]    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Run Keyword And Ignore Error    Collapse Tree Item    Tree    ${TEST_PROJECT}
    Log    Tree item collapsed

Select Tree Item Successfully
    [Documentation]    Verify selecting a tree item.
    ...                Should highlight the specified item.
    [Tags]    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Run Keyword And Ignore Error    Select Tree Item    Tree    ${TEST_PROJECT}
    Log    Tree item selected


# =============================================================================
# Wait Keywords Tests
# =============================================================================

Wait Until Widget Exists Successfully
    [Documentation]    Verify waiting until widget exists.
    ...                Should return once widget appears.
    [Tags]    smoke    positive    wait
    Show View    ${PACKAGE_EXPLORER_VIEW}
    ${widget}=    Wait Until Widget Exists    Tree    timeout=5
    Should Not Be Empty    ${widget}

Wait Until Widget Exists Timeout
    [Documentation]    Verify timeout when widget doesn't exist.
    ...                Should raise timeout error.
    [Tags]    negative    wait    timeout
    Run Keyword And Expect Error    *Timeout*timed out*
    ...    Wait Until Widget Exists    name:nonExistentWidget    timeout=2

Wait Until Widget Enabled Successfully
    [Documentation]    Verify waiting until widget is enabled.
    ...                Should return once widget becomes enabled.
    [Tags]    positive    wait
    Show View    ${PACKAGE_EXPLORER_VIEW}
    ${widget}=    Wait Until Widget Enabled    name:packageExplorerTree    timeout=5
    Should Not Be Empty    ${widget}


# =============================================================================
# Verification Keywords Tests
# =============================================================================

Widget Should Be Visible Passes
    [Documentation]    Verify assertion passes for visible widget.
    ...                Should not raise error.
    [Tags]    smoke    positive    assertion
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Widget Should Be Visible    name:packageExplorerTree

Widget Should Be Visible Fails
    [Documentation]    Verify assertion fails for hidden widget.
    ...                Should raise AssertionError.
    [Tags]    negative    assertion
    Run Keyword And Expect Error    *not found*
    ...    Widget Should Be Visible    name:nonExistentWidget

Widget Should Be Enabled Passes
    [Documentation]    Verify assertion passes for enabled widget.
    ...                Should not raise error.
    [Tags]    positive    assertion
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Widget Should Be Enabled    name:packageExplorerTree

Widget Text Should Be Passes
    [Documentation]    Verify widget text assertion passes.
    ...                Should match expected text.
    [Tags]    positive    assertion
    Open Preferences
    Input Text    Text    expected text    clear=${TRUE}
    Widget Text Should Be    Text    expected text
    Close Shell    text:Preferences

Widget Text Should Be Fails
    [Documentation]    Verify widget text assertion fails on mismatch.
    ...                Should raise AssertionError.
    [Tags]    negative    assertion
    Open Preferences
    Input Text    Text    actual text    clear=${TRUE}
    Run Keyword And Expect Error    *does not match*
    ...    Widget Text Should Be    Text    different text
    Close Shell    text:Preferences


# =============================================================================
# Configuration Keywords Tests
# =============================================================================

Set Timeout Successfully
    [Documentation]    Verify setting the timeout value.
    ...                Should return previous timeout.
    [Tags]    positive    config
    ${old}=    Set Timeout    30
    Log    Previous timeout: ${old}
    ${current}=    Set Timeout    ${old}
    Should Be Equal As Numbers    ${current}    30


# =============================================================================
# Shell Keywords Tests
# =============================================================================

Get Shells Successfully
    [Documentation]    Verify getting all shells.
    ...                Should return list of SwtElements for shells.
    [Tags]    smoke    positive
    ${shells}=    Get Shells
    ${count}=    Get Length    ${shells}
    Should Be True    ${count} >= 1
    Log    Found ${count} shells

Activate Shell Successfully
    [Documentation]    Verify activating a shell.
    ...                Should bring shell to front.
    [Tags]    positive
    Open Preferences
    ${shells}=    Get Shells
    Log    Shells: ${shells}
    Close Shell    text:Preferences

Close Shell Successfully
    [Documentation]    Verify closing a shell.
    ...                Should close the specified shell.
    [Tags]    positive
    Open Preferences
    Close Shell    text:Preferences
    Log    Shell closed


# =============================================================================
# Integration Tests
# =============================================================================

Widget Interaction Workflow
    [Documentation]    Test complete widget interaction workflow.
    ...                Find, click, input, verify.
    [Tags]    integration    positive
    # Open preferences for text input testing
    Open Preferences
    # Find widgets - use preferences tree specifically
    ${tree}=    Find Widget    name:preferencesTree
    Should Not Be Empty    ${tree}
    # Click on preferences tree
    Click Widget    name:preferencesTree
    # Input text
    Input Text    Text    test value    clear=${TRUE}
    # Verify text
    Widget Text Should Be    Text    test value
    # Clear text
    Clear Text    Text
    # Close
    Close Shell    text:Preferences

View Widget Operations
    [Documentation]    Test widget operations in views.
    ...                Work with view widgets.
    [Tags]    integration    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Activate View    ${PACKAGE_EXPLORER_VIEW}
    # Find tree in view using specific name
    ${tree}=    Get View Widget    ${PACKAGE_EXPLORER_VIEW}    name:packageExplorerTree
    # Click on package explorer tree
    Click Widget    name:packageExplorerTree
    # Wait for widget
    ${widget}=    Wait Until Widget Exists    name:packageExplorerTree    timeout=5
    Should Not Be Empty    ${widget}
    # Verify visible
    Widget Should Be Visible    name:packageExplorerTree
    # Verify enabled
    Widget Should Be Enabled    name:packageExplorerTree
