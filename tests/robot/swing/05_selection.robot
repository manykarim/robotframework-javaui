*** Settings ***
Documentation     Selection Tests - Testing select_from_combobox, check_checkbox,
...               uncheck_checkbox, select_radio_button, and list selection keywords.
...
...               These tests verify the library's ability to interact with
...               selection-based components like combo boxes, checkboxes, and lists.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        selection    regression

*** Test Cases ***
# =============================================================================
# COMBO BOX SELECTION
# =============================================================================

Select Item From ComboBox By Name
    [Documentation]    Select an item from a combo box using name selector.
    [Tags]    smoke    positive
    Select Selections Tab
    Select From Combobox    JComboBox[name='categoryComboBox']    Electronics
    # Verify selection was applied
    ${text}=    Get Element Text    JComboBox[name='categoryComboBox']
    Should Contain    ${text}    Electronics    Combo selection should be visible

Select Item From ComboBox By ID
    [Documentation]    Select an item using ID-style selector.
    [Tags]    positive
    Select Selections Tab
    Select From Combobox    \#categoryComboBox    Clothing
    ${text}=    Get Element Text    \#categoryComboBox
    Should Contain    ${text}    Clothing    ID selector combo selection should work

Select Multiple Items From ComboBox
    [Documentation]    Select different items sequentially.
    [Tags]    positive
    Select Selections Tab
    Select From Combobox    [name='categoryComboBox']    Books
    Select From Combobox    [name='categoryComboBox']    Sports
    Select From Combobox    [name='categoryComboBox']    Toys
    ${text}=    Get Element Text    [name='categoryComboBox']
    Should Contain    ${text}    Toys    Final selection should be Toys

Select Item From ComboBox Using XPath
    [Documentation]    Select item using XPath selector.
    [Tags]    positive    xpath-locator
    Select Selections Tab
    Select From Combobox    //JComboBox[@name='categoryComboBox']    Home & Garden
    ${text}=    Get Element Text    //JComboBox[@name='categoryComboBox']
    Should Contain    ${text}    Home & Garden    XPath combo selection should work

# =============================================================================
# CHECKBOX OPERATIONS
# =============================================================================

Check Checkbox By Name
    [Documentation]    Check a checkbox using name selector.
    [Tags]    smoke    positive
    Select Selections Tab
    Check Checkbox    JCheckBox[name='enabledCheckBox']
    Sleep    0.3s    Wait for UI update
    # Verify checkbox is now selected
    ${selected}=    Get Element Property    JCheckBox[name='enabledCheckBox']    selected
    Should Be True    ${selected}    Checkbox should be selected after check

Check Checkbox By ID
    [Documentation]    Check a checkbox using ID-style selector.
    [Tags]    positive
    Select Selections Tab
    Check Checkbox    \#enabledCheckBox
    # Verify checkbox is now selected
    Element Should Be Selected    \#enabledCheckBox

Check Multiple Checkboxes
    [Documentation]    Check multiple checkboxes in sequence.
    [Tags]    positive
    Select Selections Tab
    Check Checkbox    JCheckBox[name='enabledCheckBox']
    Check Checkbox    JCheckBox[name='notificationsCheckBox']
    Check Checkbox    JCheckBox[name='autoSaveCheckBox']
    Element Should Be Selected    JCheckBox[name='enabledCheckBox']
    Element Should Be Selected    JCheckBox[name='notificationsCheckBox']
    Element Should Be Selected    JCheckBox[name='autoSaveCheckBox']

Check Checkbox Using XPath
    [Documentation]    Check a checkbox using XPath selector.
    [Tags]    positive    xpath-locator
    Select Selections Tab
    Check Checkbox    //JCheckBox[@name='enabledCheckBox']
    Element Should Be Selected    //JCheckBox[@name='enabledCheckBox']

Check Already Checked Checkbox
    [Documentation]    Verify checking already checked checkbox is safe.
    [Tags]    positive    edge-case
    Select Selections Tab
    Check Checkbox    [name='enabledCheckBox']
    Check Checkbox    [name='enabledCheckBox']
    Element Should Be Selected    [name='enabledCheckBox']

Uncheck Checkbox By Name
    [Documentation]    Uncheck a checkbox using name selector.
    [Tags]    smoke    positive
    Select Selections Tab
    Check Checkbox    JCheckBox[name='enabledCheckBox']
    Element Should Be Selected    JCheckBox[name='enabledCheckBox']
    Uncheck Checkbox    JCheckBox[name='enabledCheckBox']
    # Verify checkbox is now unchecked
    Element Should Not Be Selected    JCheckBox[name='enabledCheckBox']

Uncheck Checkbox By ID
    [Documentation]    Uncheck a checkbox using ID-style selector.
    [Tags]    positive
    Select Selections Tab
    Check Checkbox    \#enabledCheckBox
    Uncheck Checkbox    \#enabledCheckBox
    Element Should Not Be Selected    \#enabledCheckBox

Uncheck Multiple Checkboxes
    [Documentation]    Uncheck multiple checkboxes in sequence.
    [Tags]    positive
    Select Selections Tab
    Check Checkbox    [name='enabledCheckBox']
    Check Checkbox    [name='notificationsCheckBox']
    Uncheck Checkbox    [name='enabledCheckBox']
    Uncheck Checkbox    [name='notificationsCheckBox']
    Element Should Not Be Selected    [name='enabledCheckBox']
    Element Should Not Be Selected    [name='notificationsCheckBox']

Uncheck Already Unchecked Checkbox
    [Documentation]    Verify unchecking already unchecked checkbox is safe.
    [Tags]    positive    edge-case
    Select Selections Tab
    Uncheck Checkbox    [name='enabledCheckBox']
    Uncheck Checkbox    [name='enabledCheckBox']
    Element Should Not Be Selected    [name='enabledCheckBox']

# =============================================================================
# RADIO BUTTON SELECTION
# =============================================================================

Select Radio Button By Name
    [Documentation]    Select a radio button using name selector.
    [Tags]    smoke    positive
    Select Selections Tab
    Select Radio Button    JRadioButton[name='highPriorityRadioButton']
    # Verify radio button is now selected
    Element Should Be Selected    JRadioButton[name='highPriorityRadioButton']

Select Radio Button By ID
    [Documentation]    Select a radio button using ID-style selector.
    [Tags]    positive
    Select Selections Tab
    Select Radio Button    \#normalPriorityRadioButton
    Element Should Be Selected    \#normalPriorityRadioButton

Select Different Radio Buttons In Group
    [Documentation]    Select different radio buttons in the same group.
    [Tags]    positive
    Select Selections Tab
    Select Radio Button    JRadioButton[name='highPriorityRadioButton']
    Element Should Be Selected    JRadioButton[name='highPriorityRadioButton']
    Select Radio Button    JRadioButton[name='normalPriorityRadioButton']
    Element Should Be Selected    JRadioButton[name='normalPriorityRadioButton']
    Element Should Not Be Selected    JRadioButton[name='highPriorityRadioButton']

Select Radio Button Using XPath
    [Documentation]    Select radio button using XPath selector.
    [Tags]    positive    xpath-locator
    Select Selections Tab
    Select Radio Button    //JRadioButton[@name='highPriorityRadioButton']
    Element Should Be Selected    //JRadioButton[@name='highPriorityRadioButton']

Select Same Radio Button Multiple Times
    [Documentation]    Verify selecting same radio button multiple times is safe.
    [Tags]    positive    edge-case
    Select Selections Tab
    Select Radio Button    [name='highPriorityRadioButton']
    Select Radio Button    [name='highPriorityRadioButton']
    Select Radio Button    [name='highPriorityRadioButton']
    Element Should Be Selected    [name='highPriorityRadioButton']

# =============================================================================
# LIST SELECTION
# =============================================================================

Select From List By Name
    [Documentation]    Select an item from a list using name selector.
    [Tags]    smoke    positive
    Select Selections Tab
    Select From List    JList[name='itemList']    Item 1 - Apple
    Element Should Exist    JList[name='itemList']

Select From List By ID
    [Documentation]    Select an item using ID-style selector.
    [Tags]    positive
    Select Selections Tab
    Select From List    \#itemList    Item 2 - Banana
    Element Should Exist    \#itemList

Select Multiple Items From List
    [Documentation]    Select different items from the list.
    [Tags]    positive
    Select Selections Tab
    Select From List    [name='itemList']    Item 1 - Apple
    Select From List    [name='itemList']    Item 3 - Cherry
    Select From List    [name='itemList']    Item 5 - Elderberry
    Element Should Exist    [name='itemList']

Select List Item By Index
    [Documentation]    Select a list item by its index.
    [Tags]    positive
    Select Selections Tab
    Select List Item By Index    JList[name='itemList']    0
    Element Should Exist    JList[name='itemList']

Select List Item By Index Different Positions
    [Documentation]    Select items at different index positions.
    [Tags]    positive
    Select Selections Tab
    Select List Item By Index    [name='itemList']    0
    Select List Item By Index    [name='itemList']    2
    Select List Item By Index    [name='itemList']    4
    Element Should Exist    [name='itemList']

Get List Items
    [Documentation]    Get all items from a list.
    [Tags]    positive
    Select Selections Tab
    ${items}=    Get List Items    JList[name='itemList']
    Should Not Be Empty    ${items}
    Log    List items: ${items}

# =============================================================================
# SELECTION WORKFLOWS
# =============================================================================

Complete Registration Form Selection Workflow
    [Documentation]    Fill in a registration form with various selections.
    [Tags]    workflow    smoke
    Select Selections Tab
    # Select category
    Select From Combobox    [name='categoryComboBox']    Electronics
    # Select gender
    Select Radio Button    [name='highPriorityRadioButton']
    # Check options
    Check Checkbox    [name='enabledCheckBox']
    Check Checkbox    [name='enabledCheckBox']
    Element Should Exist    [name='categoryComboBox']
    Element Should Exist    [name='highPriorityRadioButton']

Toggle Checkbox Workflow
    [Documentation]    Test checkbox toggle behavior.
    [Tags]    workflow
    Select Selections Tab
    Uncheck Checkbox    [name='enabledCheckBox']
    Check Checkbox    [name='enabledCheckBox']
    Uncheck Checkbox    [name='enabledCheckBox']
    Check Checkbox    [name='enabledCheckBox']
    Element Should Exist    [name='enabledCheckBox']

Radio Button Group Navigation Workflow
    [Documentation]    Navigate through radio button group.
    [Tags]    workflow
    Select Selections Tab
    Select Radio Button    [name='highPriorityRadioButton']
    Select Radio Button    [name='normalPriorityRadioButton']
    Select Radio Button    [name='normalPriorityRadioButton']
    Select Radio Button    [name='highPriorityRadioButton']
    Element Should Exist    [name='highPriorityRadioButton']

# =============================================================================
# SELECTION VERIFICATION
# =============================================================================

Verify Checkbox Is Selected
    [Documentation]    Verify checkbox selection state.
    [Tags]    positive    verification
    Select Selections Tab
    Check Checkbox    [name='enabledCheckBox']
    Element Should Be Selected    JCheckBox[name='enabledCheckBox']

Verify Checkbox Is Not Selected
    [Documentation]    Verify checkbox not selected state.
    [Tags]    positive    verification
    Select Selections Tab
    Uncheck Checkbox    [name='enabledCheckBox']
    Element Should Not Be Selected    JCheckBox[name='enabledCheckBox']

Verify Radio Button Is Selected
    [Documentation]    Verify radio button selection state.
    [Tags]    positive    verification
    Select Selections Tab
    Select Radio Button    [name='highPriorityRadioButton']
    Element Should Be Selected    JRadioButton[name='highPriorityRadioButton']

# =============================================================================
# FINDING SELECTION ELEMENTS
# =============================================================================

Find All Checkboxes
    [Documentation]    Find all checkbox elements.
    [Tags]    positive
    ${checkboxes}=    Find Elements    JCheckBox
    Should Not Be Empty    ${checkboxes}

Find All Radio Buttons
    [Documentation]    Find all radio button elements.
    [Tags]    positive
    ${radios}=    Find Elements    JRadioButton
    Should Not Be Empty    ${radios}

Find All ComboBoxes
    [Documentation]    Find all combo box elements.
    [Tags]    positive
    ${combos}=    Find Elements    JComboBox
    Should Not Be Empty    ${combos}

Find All Lists
    [Documentation]    Find all list elements.
    [Tags]    positive
    ${lists}=    Find Elements    JList
    Should Not Be Empty    ${lists}

Find Enabled Checkboxes
    [Documentation]    Find all enabled checkboxes.
    [Tags]    positive
    ${checkboxes}=    Find Elements    JCheckBox:enabled
    Should Not Be Empty    ${checkboxes}

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Select From Nonexistent ComboBox Fails
    [Documentation]    Select from non-existent combo box throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select From Combobox    JComboBox[name='nonexistent']    value
    Should Be Equal    ${status}    ${FALSE}

Check Nonexistent Checkbox Fails
    [Documentation]    Check non-existent checkbox throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Check Checkbox    JCheckBox[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Select Nonexistent Radio Button Fails
    [Documentation]    Select non-existent radio button throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select Radio Button    JRadioButton[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Select From Nonexistent List Fails
    [Documentation]    Select from non-existent list throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select From List    JList[name='nonexistent']    Item
    Should Be Equal    ${status}    ${FALSE}

Select Invalid ComboBox Item Fails
    [Documentation]    Select invalid item from combo box throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select From Combobox    [name='categoryComboBox']    NonExistentCountry
    Should Be Equal    ${status}    ${FALSE}

# =============================================================================
# EDGE CASES
# =============================================================================

Select First ComboBox Item
    [Documentation]    Select the first item in combo box.
    [Tags]    edge-case
    ${combos}=    Find Elements    JComboBox
    Should Not Be Empty    ${combos}

Rapid Checkbox Toggle
    [Documentation]    Test rapid checkbox toggling.
    [Tags]    edge-case    stress
    Select Selections Tab
    FOR    ${i}    IN RANGE    5
        Check Checkbox    [name='enabledCheckBox']
        Uncheck Checkbox    [name='enabledCheckBox']
    END
    Element Should Exist    [name='enabledCheckBox']

Rapid Radio Button Selection
    [Documentation]    Test rapid radio button selection.
    [Tags]    edge-case    stress
    Select Selections Tab
    FOR    ${i}    IN RANGE    5
        Select Radio Button    [name='highPriorityRadioButton']
        Select Radio Button    [name='normalPriorityRadioButton']
    END
    Element Should Exist    [name='highPriorityRadioButton']
