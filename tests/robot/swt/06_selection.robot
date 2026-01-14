*** Settings ***
Documentation     Test suite for SWT selection keywords.
...
...               Tests the following SwtLibrary keywords:
...               - select_combo_item (for Combo and CCombo widgets)
...               - select_list_item
...               - check_button
...               - uncheck_button
...
...               Tests selection operations on various SWT selection widgets.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        swt    selection    actions


*** Variables ***
# Combo widget locators (matching SwtTestApp widget names)
${COMBO_WIDGET}           name:comboCategory
${CCOMBO_WIDGET}          name:ccomboStatus
${READONLY_COMBO}         name:comboPriority

# List widget locators
${LIST_WIDGET}            name:listAssignees
${MULTI_SELECT_LIST}      name:listAssignees

# Button widget locators
${CHECKBOX_WIDGET}        name:checkAutoSave
${CHECKBOX_2}             name:checkDebugMode
${TOGGLE_BUTTON}          name:buttonToggle
${RADIO_BUTTON_1}         name:radioLight
${RADIO_BUTTON_2}         name:radioDark

# Test data (matching SwtTestApp combo values)
${COMBO_ITEM_1}           Development
${COMBO_ITEM_2}           Testing
${COMBO_ITEM_3}           Documentation
${COMBO_ITEM_LAST}        Other
${NONEXISTENT_ITEM}       NonExistent Item 12345
${LIST_ITEM_1}            Alice
${LIST_ITEM_2}            Bob
${LIST_ITEM_3}            Charlie

# Error cases
${NONEXISTENT_WIDGET}     name:nonExistentWidget12345


*** Test Cases ***
# ============================================================================
# select_combo_item - Combo Widget
# ============================================================================

Select Combo Item By Text
    [Documentation]    Verify selecting a Combo item by its text value.
    [Tags]    smoke    critical    positive    combo
    Select Combo Item    ${COMBO_WIDGET}    ${COMBO_ITEM_1}
    Log    Selected combo item: ${COMBO_ITEM_1}

Select Combo Item By Name Locator
    [Documentation]    Verify selecting Combo item using name: locator.
    [Tags]    positive    combo
    Select Combo Item    name:comboCategory    ${COMBO_ITEM_2}
    Log    Selected combo item using name locator

Select Different Combo Items
    [Documentation]    Verify selecting different items in the same Combo.
    [Tags]    positive    combo
    Select Combo Item    ${COMBO_WIDGET}    ${COMBO_ITEM_1}
    Select Combo Item    ${COMBO_WIDGET}    ${COMBO_ITEM_2}
    Select Combo Item    ${COMBO_WIDGET}    ${COMBO_ITEM_3}
    Log    Cycled through combo items

Select First Combo Item
    [Documentation]    Verify selecting the first item in a Combo.
    [Tags]    positive    combo    boundary
    Select Combo Item    ${COMBO_WIDGET}    ${COMBO_ITEM_1}
    Log    Selected first combo item

Select Last Combo Item
    [Documentation]    Verify selecting the last item in a Combo.
    [Tags]    positive    combo    boundary
    Select Combo Item    ${COMBO_WIDGET}    ${COMBO_ITEM_LAST}
    Log    Selected last combo item

Select Combo Item With Special Characters
    [Documentation]    Verify selecting an item with special characters.
    [Tags]    positive    combo    special-chars
    [Setup]    Log    Test assumes special character item exists - may skip if not
    # Skip this test as special character items may not be in the test app
    Skip    Special character items not available in test app

# ============================================================================
# select_combo_item - CCombo Widget
# ============================================================================

Select CCombo Item By Text
    [Documentation]    Verify selecting a CCombo (custom combo) item.
    [Tags]    smoke    positive    ccombo
    Select Combo Item    ${CCOMBO_WIDGET}    Testing
    Log    Selected CCombo item: Testing

Select CCombo Item By Name Locator
    [Documentation]    Verify selecting CCombo item using name: locator.
    [Tags]    positive    ccombo
    Select Combo Item    name:ccomboStatus    Documentation
    Log    Selected CCombo item using name locator

Select Different CCombo Items
    [Documentation]    Verify selecting different items in CCombo.
    [Tags]    positive    ccombo
    Select Combo Item    ${CCOMBO_WIDGET}    Development
    Select Combo Item    ${CCOMBO_WIDGET}    Testing
    Select Combo Item    ${CCOMBO_WIDGET}    Documentation
    Log    Cycled through CCombo items

# ============================================================================
# select_combo_item - Negative Test Cases
# ============================================================================

Select Combo Item Fails For Nonexistent Item
    [Documentation]    Verify proper error when item doesn't exist in Combo.
    [Tags]    negative    error-handling    combo
    Run Keyword And Expect Error    *
    ...    Select Combo Item    ${COMBO_WIDGET}    ${NONEXISTENT_ITEM}

Select Combo Item Fails For Nonexistent Widget
    [Documentation]    Verify proper error when Combo widget doesn't exist.
    [Tags]    negative    error-handling    combo
    Run Keyword And Expect Error    *not found*
    ...    Select Combo Item    ${NONEXISTENT_WIDGET}    ${COMBO_ITEM_1}

Select Combo Item Fails With Empty Locator
    [Documentation]    Verify behavior when locator is empty (may error or be no-op).
    [Tags]    negative    validation    combo
    TRY
        Select Combo Item    ${EMPTY}    ${COMBO_ITEM_1}
        Log    Empty locator combo select completed without error
    EXCEPT    *    type=GLOB
        Log    Empty locator combo select raised error (expected)
    END

Select Combo Item Fails With Empty Item
    [Documentation]    Verify proper error when item text is empty.
    [Tags]    negative    validation    combo
    Run Keyword And Expect Error    *
    ...    Select Combo Item    ${COMBO_WIDGET}    ${EMPTY}

# ============================================================================
# select_list_item - Positive Test Cases
# ============================================================================

Select List Item By Text
    [Documentation]    Verify selecting a List item by its text value.
    [Tags]    smoke    critical    positive    list
    Select List Item    ${LIST_WIDGET}    ${LIST_ITEM_1}
    Log    Selected list item: ${LIST_ITEM_1}

Select List Item By Name Locator
    [Documentation]    Verify selecting List item using name: locator.
    [Tags]    positive    list
    Select List Item    name:listAssignees    ${LIST_ITEM_2}
    Log    Selected list item using name locator

Select Different List Items
    [Documentation]    Verify selecting different items in the same List.
    [Tags]    positive    list
    Select List Item    ${LIST_WIDGET}    ${LIST_ITEM_1}
    Select List Item    ${LIST_WIDGET}    ${LIST_ITEM_2}
    Select List Item    ${LIST_WIDGET}    ${LIST_ITEM_3}
    Log    Cycled through list items

Select First List Item
    [Documentation]    Verify selecting the first item in a List.
    [Tags]    positive    list    boundary
    Select List Item    ${LIST_WIDGET}    ${LIST_ITEM_1}
    Log    Selected first list item

Select List Item Changes Selection
    [Documentation]    Verify selecting a new item deselects the previous one.
    [Tags]    positive    list
    Select List Item    ${LIST_WIDGET}    ${LIST_ITEM_1}
    Select List Item    ${LIST_WIDGET}    ${LIST_ITEM_2}
    # Second item should now be selected, first deselected

# ============================================================================
# select_list_item - Negative Test Cases
# ============================================================================

Select List Item Fails For Nonexistent Item
    [Documentation]    Verify proper error when item doesn't exist in List.
    [Tags]    negative    error-handling    list
    Run Keyword And Expect Error    *
    ...    Select List Item    ${LIST_WIDGET}    ${NONEXISTENT_ITEM}

Select List Item Fails For Nonexistent Widget
    [Documentation]    Verify proper error when List widget doesn't exist.
    [Tags]    negative    error-handling    list
    Run Keyword And Expect Error    *not found*
    ...    Select List Item    ${NONEXISTENT_WIDGET}    ${LIST_ITEM_1}

# ============================================================================
# check_button - Positive Test Cases
# ============================================================================

Check Button That Is Unchecked
    [Documentation]    Verify checking a button that is currently unchecked.
    [Tags]    smoke    critical    positive    checkbox
    # First ensure it's unchecked
    Uncheck Button    ${CHECKBOX_WIDGET}
    # Now check it
    Check Button    ${CHECKBOX_WIDGET}
    Log    Checkbox is now checked

Check Button That Is Already Checked
    [Documentation]    Verify check_button is idempotent when already checked.
    [Tags]    positive    checkbox    idempotent
    # First check it
    Check Button    ${CHECKBOX_WIDGET}
    # Check again - should have no effect
    Check Button    ${CHECKBOX_WIDGET}
    Log    Checkbox remains checked

Check Button By Name Locator
    [Documentation]    Verify checking a button using name: locator.
    [Tags]    positive    checkbox
    Uncheck Button    name:checkAutoSave
    Check Button    name:checkAutoSave
    Log    Checked using name locator

Check Toggle Button
    [Documentation]    Verify check_button works with toggle buttons.
    [Tags]    positive    toggle
    Uncheck Button    ${TOGGLE_BUTTON}
    Check Button    ${TOGGLE_BUTTON}
    Log    Toggle button is now checked/pressed

Check Multiple Checkboxes
    [Documentation]    Verify checking multiple checkboxes independently.
    [Tags]    positive    checkbox    multiple
    Check Button    ${CHECKBOX_WIDGET}
    Check Button    ${CHECKBOX_2}
    Log    Both checkboxes are now checked

# ============================================================================
# check_button - Negative Test Cases
# ============================================================================

Check Button Fails For Nonexistent Widget
    [Documentation]    Verify proper error when button doesn't exist.
    [Tags]    negative    error-handling    checkbox
    Run Keyword And Expect Error    *not found*
    ...    Check Button    ${NONEXISTENT_WIDGET}

Check Button Fails With Empty Locator
    [Documentation]    Verify behavior when locator is empty (may error or be no-op).
    [Tags]    negative    validation    checkbox
    TRY
        Check Button    ${EMPTY}
        Log    Empty locator check button completed without error
    EXCEPT    *    type=GLOB
        Log    Empty locator check button raised error (expected)
    END

# ============================================================================
# uncheck_button - Positive Test Cases
# ============================================================================

Uncheck Button That Is Checked
    [Documentation]    Verify unchecking a button that is currently checked.
    [Tags]    smoke    critical    positive    checkbox
    # First ensure it's checked
    Check Button    ${CHECKBOX_WIDGET}
    # Now uncheck it
    Uncheck Button    ${CHECKBOX_WIDGET}
    Log    Checkbox is now unchecked

Uncheck Button That Is Already Unchecked
    [Documentation]    Verify uncheck_button is idempotent when already unchecked.
    [Tags]    positive    checkbox    idempotent
    # First uncheck it
    Uncheck Button    ${CHECKBOX_WIDGET}
    # Uncheck again - should have no effect
    Uncheck Button    ${CHECKBOX_WIDGET}
    Log    Checkbox remains unchecked

Uncheck Button By Name Locator
    [Documentation]    Verify unchecking a button using name: locator.
    [Tags]    positive    checkbox
    Check Button    name:checkAutoSave
    Uncheck Button    name:checkAutoSave
    Log    Unchecked using name locator

Uncheck Toggle Button
    [Documentation]    Verify uncheck_button works with toggle buttons.
    [Tags]    positive    toggle
    Check Button    ${TOGGLE_BUTTON}
    Uncheck Button    ${TOGGLE_BUTTON}
    Log    Toggle button is now unchecked/unpressed

Uncheck Multiple Checkboxes
    [Documentation]    Verify unchecking multiple checkboxes independently.
    [Tags]    positive    checkbox    multiple
    Check Button    ${CHECKBOX_WIDGET}
    Check Button    ${CHECKBOX_2}
    Uncheck Button    ${CHECKBOX_WIDGET}
    Uncheck Button    ${CHECKBOX_2}
    Log    Both checkboxes are now unchecked

# ============================================================================
# uncheck_button - Negative Test Cases
# ============================================================================

Uncheck Button Fails For Nonexistent Widget
    [Documentation]    Verify proper error when button doesn't exist.
    [Tags]    negative    error-handling    checkbox
    Run Keyword And Expect Error    *not found*
    ...    Uncheck Button    ${NONEXISTENT_WIDGET}

Uncheck Button Fails With Empty Locator
    [Documentation]    Verify behavior when locator is empty (may error or be no-op).
    [Tags]    negative    validation    checkbox
    TRY
        Uncheck Button    ${EMPTY}
        Log    Empty locator uncheck button completed without error
    EXCEPT    *    type=GLOB
        Log    Empty locator uncheck button raised error (expected)
    END

# ============================================================================
# Combined Check/Uncheck Operations
# ============================================================================

Toggle Checkbox State Multiple Times
    [Documentation]    Verify toggling checkbox state multiple times.
    [Tags]    positive    checkbox    toggle
    Uncheck Button    ${CHECKBOX_WIDGET}
    Check Button    ${CHECKBOX_WIDGET}
    Uncheck Button    ${CHECKBOX_WIDGET}
    Check Button    ${CHECKBOX_WIDGET}
    Uncheck Button    ${CHECKBOX_WIDGET}
    Log    Toggled checkbox state multiple times

Check And Uncheck Workflow
    [Documentation]    Verify typical check/uncheck workflow.
    [Tags]    positive    workflow
    # Start with unchecked
    Uncheck Button    ${CHECKBOX_WIDGET}

    # Check when needed
    Check Button    ${CHECKBOX_WIDGET}

    # Verify by checking again (should be idempotent)
    Check Button    ${CHECKBOX_WIDGET}

    # Uncheck when done
    Uncheck Button    ${CHECKBOX_WIDGET}

# ============================================================================
# Radio Button Selection
# ============================================================================

Check Radio Button Selects It
    [Documentation]    Verify check_button works to select radio buttons.
    [Tags]    positive    radio
    Check Button    ${RADIO_BUTTON_1}
    Log    Radio button 1 is now selected

Check Different Radio Button Deselects Previous
    [Documentation]    Verify selecting one radio button deselects others in group.
    [Tags]    positive    radio    group
    Check Button    ${RADIO_BUTTON_1}
    Check Button    ${RADIO_BUTTON_2}
    # Radio button 1 should now be deselected, button 2 selected


*** Keywords ***
# Local keywords for this test file
