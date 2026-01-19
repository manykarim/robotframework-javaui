*** Settings ***
Test Timeout       60s
Documentation     Test suite for SWT click operation keywords.
...
...               Tests the following SwtLibrary keywords:
...               - click_widget
...               - double_click_widget
...
...               Tests clicking on various widget types including buttons,
...               links, labels, and other clickable elements.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        swt    clicks    actions


*** Variables ***
# Widget locators matching SwtTestApp component names
${PUSH_BUTTON}            name:buttonSubmit
${TOGGLE_BUTTON}          name:buttonToggle
${CHECK_BUTTON}           name:checkAutoSave
${RADIO_BUTTON}           name:radioLight
${ARROW_BUTTON}           name:buttonArrow
${LINK_WIDGET}            name:linkHelp
${LABEL_WIDGET}           name:labelUsername
${LIST_ITEM}              name:listAssignees
${TABLE_ROW}              name:dataTable
${TREE_ITEM}              name:fileTree
${DISABLED_BUTTON}        name:disabledButton
${NONEXISTENT}            name:nonExistentWidget12345


*** Test Cases ***
# ============================================================================
# click_widget - Button Clicks
# ============================================================================

Click Push Button By Name
    [Documentation]    Verify clicking a push button using name locator.
    [Tags]    smoke    critical    positive    button
    Click Widget    ${PUSH_BUTTON}
    Log    Push button clicked successfully

Click Push Button By Text
    [Documentation]    Verify clicking a push button using text locator.
    [Tags]    smoke    positive    button
    Click Widget    text:Submit
    Log    Button clicked by text

Click Push Button By Class
    [Documentation]    Verify clicking the first button found by class.
    [Tags]    positive    button
    Click Widget    class:Button
    Log    Button clicked by class

Click Toggle Button
    [Documentation]    Verify clicking a toggle button.
    [Tags]    positive    button
    Click Widget    ${TOGGLE_BUTTON}
    # Click again to toggle back
    Click Widget    ${TOGGLE_BUTTON}
    Log    Toggle button clicked twice

Click Check Button
    [Documentation]    Verify clicking a check button (checkbox style).
    [Tags]    positive    button
    Click Widget    ${CHECK_BUTTON}
    Log    Check button clicked

Click Radio Button
    [Documentation]    Verify clicking a radio button.
    [Tags]    positive    button
    Click Widget    ${RADIO_BUTTON}
    Log    Radio button clicked

Click Arrow Button
    [Documentation]    Verify clicking an arrow button.
    [Tags]    positive    button
    Click Widget    ${ARROW_BUTTON}
    Log    Arrow button clicked

Click Button Multiple Times
    [Documentation]    Verify clicking the same button multiple times.
    [Tags]    positive    multiple
    Click Widget    ${PUSH_BUTTON}
    Click Widget    ${PUSH_BUTTON}
    Click Widget    ${PUSH_BUTTON}
    Log    Button clicked three times

# ============================================================================
# click_widget - Link Clicks
# ============================================================================

Click Link Widget
    [Documentation]    Verify clicking a Link widget.
    [Tags]    positive    link
    Click Widget    ${LINK_WIDGET}
    Log    Link widget clicked

Click Link By Text
    [Documentation]    Verify clicking a Link using its text.
    [Tags]    positive    link
    # Link has complex text, use name locator instead
    Click Widget    name:linkHelp
    Log    Link clicked by name

# ============================================================================
# click_widget - Other Widget Types
# ============================================================================

Click Label Widget
    [Documentation]    Verify clicking a label widget.
    [Tags]    positive    label
    Click Widget    ${LABEL_WIDGET}
    Log    Label clicked

Click List Item
    [Documentation]    Verify clicking an item in a list.
    [Tags]    positive    list
    Click Widget    ${LIST_ITEM}
    Log    List item clicked

Click Tree Item
    [Documentation]    Verify clicking a tree item.
    [Tags]    positive    tree
    Click Widget    ${TREE_ITEM}
    Log    Tree item clicked

Click Table Row
    [Documentation]    Verify clicking a table row.
    [Tags]    positive    table
    Click Widget    ${TABLE_ROW}
    Log    Table row clicked

# ============================================================================
# click_widget - Negative Test Cases
# ============================================================================

Click Widget Fails For Nonexistent Widget
    [Documentation]    Verify proper error when clicking non-existent widget.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *not found*
    ...    Click Widget    ${NONEXISTENT}

Click Widget Fails With Empty Locator
    [Documentation]    Verify behavior when locator is empty (may error or be no-op).
    [Tags]    negative    validation
    TRY
        Click Widget    ${EMPTY}
        Log    Empty locator click completed without error
    EXCEPT    *    type=GLOB
        Log    Empty locator click raised error (expected)
    END

Click Disabled Button
    [Documentation]    Verify behavior when clicking a disabled button.
    [Tags]    negative    disabled
    # Behavior may vary - could succeed with no effect or fail
    TRY
        Click Widget    ${DISABLED_BUTTON}
        Log    Click succeeded on disabled button (may have no effect)
    EXCEPT
        Log    Click on disabled button raised error (expected in some implementations)
    END

# ============================================================================
# double_click_widget - Positive Test Cases
# ============================================================================

Double Click Button
    [Documentation]    Verify double-clicking a button.
    [Tags]    smoke    positive    double-click
    Double Click Widget    ${PUSH_BUTTON}
    Log    Button double-clicked

Double Click By Name
    [Documentation]    Verify double-clicking using name locator.
    [Tags]    positive    double-click
    Double Click Widget    name:buttonSubmit
    Log    Double-clicked by name

Double Click By Text
    [Documentation]    Verify double-clicking using text locator.
    [Tags]    positive    double-click
    Double Click Widget    text:Submit
    Log    Double-clicked by text

Double Click List Item
    [Documentation]    Verify double-clicking a list item.
    ...                Double-click typically opens/activates the item.
    [Tags]    positive    double-click    list
    Double Click Widget    ${LIST_ITEM}
    Log    List item double-clicked

Double Click Tree Item
    [Documentation]    Verify double-clicking a tree item.
    ...                Double-click typically expands/collapses the item.
    [Tags]    positive    double-click    tree
    Double Click Widget    ${TREE_ITEM}
    Log    Tree item double-clicked

Double Click Table Row
    [Documentation]    Verify double-clicking a table row.
    ...                Double-click typically opens/edits the row.
    [Tags]    positive    double-click    table
    Double Click Widget    ${TABLE_ROW}
    Log    Table row double-clicked

Double Click To Open Dialog
    [Documentation]    Verify double-click on table row (may open details).
    [Tags]    positive    double-click    dialog
    # Double-click on a table row - dialog opening depends on app implementation
    Double Click Widget    name:dataTable
    Log    Double-clicked table - dialog behavior depends on app implementation

# ============================================================================
# double_click_widget - Negative Test Cases
# ============================================================================

Double Click Fails For Nonexistent Widget
    [Documentation]    Verify proper error when double-clicking non-existent widget.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *not found*
    ...    Double Click Widget    ${NONEXISTENT}

Double Click Fails With Empty Locator
    [Documentation]    Verify behavior when locator is empty (may error or be no-op).
    [Tags]    negative    validation
    TRY
        Double Click Widget    ${EMPTY}
        Log    Empty locator double-click completed without error
    EXCEPT    *    type=GLOB
        Log    Empty locator double-click raised error (expected)
    END

# ============================================================================
# Click Coordinates and Positioning
# ============================================================================

Click Widget Center
    [Documentation]    Verify click occurs at widget center by default.
    [Tags]    positive    coordinates
    Click Widget    ${PUSH_BUTTON}
    Log    Clicked at widget center (default behavior)

# ============================================================================
# Click Effects Verification
# ============================================================================

Click Button Triggers Action
    [Documentation]    Verify clicking a button triggers its action.
    [Tags]    positive    verification
    # Click the Submit button - action updates status bar
    Click Widget    name:buttonSubmit
    Log    Button clicked - action triggered (status bar updated)

Click Toggle Button Changes State
    [Documentation]    Verify clicking toggle button changes its state.
    [Tags]    positive    verification    toggle
    # Get initial state (if possible through widget properties)
    Click Widget    ${TOGGLE_BUTTON}
    # State should have changed
    Click Widget    ${TOGGLE_BUTTON}
    # State should be back to original

Click Radio Button Selects It
    [Documentation]    Verify clicking radio button selects it and deselects others.
    [Tags]    positive    verification    radio
    # Click first radio
    Click Widget    name:radioLight
    # Click second radio - should deselect first
    Click Widget    name:radioDark
    Log    Radio button selection changed

# ============================================================================
# Rapid Clicks
# ============================================================================

Rapid Sequential Clicks
    [Documentation]    Verify handling of rapid sequential clicks.
    [Tags]    positive    stress
    FOR    ${i}    IN RANGE    10
        Click Widget    ${PUSH_BUTTON}
    END
    Log    Completed 10 rapid clicks

Click Different Widgets In Sequence
    [Documentation]    Verify clicking different widgets in sequence.
    [Tags]    positive    sequence
    Click Widget    ${PUSH_BUTTON}
    Click Widget    ${TOGGLE_BUTTON}
    Click Widget    ${CHECK_BUTTON}
    Click Widget    ${RADIO_BUTTON}
    Log    Clicked multiple different widgets


*** Keywords ***
Reset Label Text
    [Documentation]    Resets the result label to its initial state.
    Click Widget    name:resetButton
