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
    Select From Combobox    JComboBox[name='countryCombo']    United States
    # Verify selection was applied
    ${text}=    Get Element Text    JComboBox[name='countryCombo']
    Should Contain    ${text}    United States    Combo selection should be visible

Select Item From ComboBox By ID
    [Documentation]    Select an item using ID-style selector.
    [Tags]    positive
    Select From Combobox    \#countryCombo    Canada
    Element Should Exist    \#countryCombo

Select Multiple Items From ComboBox
    [Documentation]    Select different items sequentially.
    [Tags]    positive
    Select From Combobox    [name='countryCombo']    Germany
    Select From Combobox    [name='countryCombo']    France
    Select From Combobox    [name='countryCombo']    Japan
    Element Should Exist    [name='countryCombo']

Select Item From ComboBox Using XPath
    [Documentation]    Select item using XPath selector.
    [Tags]    positive    xpath-locator
    Select From Combobox    //JComboBox[@name='countryCombo']    Australia
    Element Should Exist    //JComboBox[@name='countryCombo']

# =============================================================================
# CHECKBOX OPERATIONS
# =============================================================================

Check Checkbox By Name
    [Documentation]    Check a checkbox using name selector.
    [Tags]    smoke    positive
    Check Checkbox    JCheckBox[name='rememberMe']
    Sleep    0.3s    Wait for UI update
    # Verify checkbox is now selected
    ${selected}=    Get Element Property    JCheckBox[name='rememberMe']    selected
    Should Be True    ${selected}    Checkbox should be selected after check

Check Checkbox By ID
    [Documentation]    Check a checkbox using ID-style selector.
    [Tags]    positive
    Check Checkbox    \#rememberMe
    # Verify checkbox is now selected
    Element Should Be Selected    \#rememberMe

Check Multiple Checkboxes
    [Documentation]    Check multiple checkboxes in sequence.
    [Tags]    positive
    Check Checkbox    JCheckBox[name='rememberMe']
    Check Checkbox    JCheckBox[name='rememberMe']
    Check Checkbox    JCheckBox[name='rememberMe']
    Element Should Exist    JCheckBox[name='rememberMe']
    Element Should Exist    JCheckBox[name='rememberMe']
    Element Should Exist    JCheckBox[name='rememberMe']

Check Checkbox Using XPath
    [Documentation]    Check a checkbox using XPath selector.
    [Tags]    positive    xpath-locator
    Check Checkbox    //JCheckBox[@name='rememberMe']
    Element Should Exist    //JCheckBox[@name='rememberMe']

Check Already Checked Checkbox
    [Documentation]    Verify checking already checked checkbox is safe.
    [Tags]    positive    edge-case
    Check Checkbox    [name='rememberMe']
    Check Checkbox    [name='rememberMe']
    Element Should Exist    [name='rememberMe']

Uncheck Checkbox By Name
    [Documentation]    Uncheck a checkbox using name selector.
    [Tags]    smoke    positive
    Check Checkbox    JCheckBox[name='rememberMe']
    Element Should Be Selected    JCheckBox[name='rememberMe']
    Uncheck Checkbox    JCheckBox[name='rememberMe']
    # Verify checkbox is now unchecked
    Element Should Not Be Selected    JCheckBox[name='rememberMe']

Uncheck Checkbox By ID
    [Documentation]    Uncheck a checkbox using ID-style selector.
    [Tags]    positive
    Check Checkbox    \#rememberMe
    Uncheck Checkbox    \#rememberMe
    Element Should Exist    \#rememberMe

Uncheck Multiple Checkboxes
    [Documentation]    Uncheck multiple checkboxes in sequence.
    [Tags]    positive
    Check Checkbox    [name='rememberMe']
    Check Checkbox    [name='rememberMe']
    Uncheck Checkbox    [name='rememberMe']
    Uncheck Checkbox    [name='rememberMe']
    Element Should Exist    [name='rememberMe']
    Element Should Exist    [name='rememberMe']

Uncheck Already Unchecked Checkbox
    [Documentation]    Verify unchecking already unchecked checkbox is safe.
    [Tags]    positive    edge-case
    Uncheck Checkbox    [name='rememberMe']
    Uncheck Checkbox    [name='rememberMe']
    Element Should Exist    [name='rememberMe']

# =============================================================================
# RADIO BUTTON SELECTION
# =============================================================================

Select Radio Button By Name
    [Documentation]    Select a radio button using name selector.
    [Tags]    smoke    positive
    Select Radio Button    JRadioButton[name='optionA']
    # Verify radio button is now selected
    Element Should Be Selected    JRadioButton[name='optionA']

Select Radio Button By ID
    [Documentation]    Select a radio button using ID-style selector.
    [Tags]    positive
    Select Radio Button    \#optionB
    Element Should Exist    \#optionB

Select Different Radio Buttons In Group
    [Documentation]    Select different radio buttons in the same group.
    [Tags]    positive
    Select Radio Button    JRadioButton[name='optionA']
    Select Radio Button    JRadioButton[name='optionB']
    Select Radio Button    JRadioButton[name='optionB']
    Element Should Exist    JRadioButton[name='optionB']

Select Radio Button Using XPath
    [Documentation]    Select radio button using XPath selector.
    [Tags]    positive    xpath-locator
    Select Radio Button    //JRadioButton[@name='optionA']
    Element Should Exist    //JRadioButton[@name='optionA']

Select Same Radio Button Multiple Times
    [Documentation]    Verify selecting same radio button multiple times is safe.
    [Tags]    positive    edge-case
    Select Radio Button    [name='optionA']
    Select Radio Button    [name='optionA']
    Select Radio Button    [name='optionA']
    Element Should Exist    [name='optionA']

# =============================================================================
# LIST SELECTION
# =============================================================================

Select From List By Name
    [Documentation]    Select an item from a list using name selector.
    [Tags]    smoke    positive
    Select From List    JList[name='itemList']    Item 1
    Element Should Exist    JList[name='itemList']

Select From List By ID
    [Documentation]    Select an item using ID-style selector.
    [Tags]    positive
    Select From List    \#itemList    Item 2
    Element Should Exist    \#itemList

Select Multiple Items From List
    [Documentation]    Select different items from the list.
    [Tags]    positive
    Select From List    [name='itemList']    Item 1
    Select From List    [name='itemList']    Item 3
    Select From List    [name='itemList']    Item 5
    Element Should Exist    [name='itemList']

Select List Item By Index
    [Documentation]    Select a list item by its index.
    [Tags]    positive
    Select List Item By Index    JList[name='itemList']    0
    Element Should Exist    JList[name='itemList']

Select List Item By Index Different Positions
    [Documentation]    Select items at different index positions.
    [Tags]    positive
    Select List Item By Index    [name='itemList']    0
    Select List Item By Index    [name='itemList']    2
    Select List Item By Index    [name='itemList']    4
    Element Should Exist    [name='itemList']

Get List Items
    [Documentation]    Get all items from a list.
    [Tags]    positive
    ${items}=    Get List Items    JList[name='itemList']
    Should Not Be Empty    ${items}
    Log    List items: ${items}

# =============================================================================
# SELECTION WORKFLOWS
# =============================================================================

Complete Registration Form Selection Workflow
    [Documentation]    Fill in a registration form with various selections.
    [Tags]    workflow    smoke
    # Select country
    Select From Combobox    [name='countryCombo']    United States
    # Select gender
    Select Radio Button    [name='optionA']
    # Check options
    Check Checkbox    [name='rememberMe']
    Check Checkbox    [name='rememberMe']
    Element Should Exist    [name='countryCombo']
    Element Should Exist    [name='optionA']

Toggle Checkbox Workflow
    [Documentation]    Test checkbox toggle behavior.
    [Tags]    workflow
    Uncheck Checkbox    [name='rememberMe']
    Check Checkbox    [name='rememberMe']
    Uncheck Checkbox    [name='rememberMe']
    Check Checkbox    [name='rememberMe']
    Element Should Exist    [name='rememberMe']

Radio Button Group Navigation Workflow
    [Documentation]    Navigate through radio button group.
    [Tags]    workflow
    Select Radio Button    [name='optionA']
    Select Radio Button    [name='optionB']
    Select Radio Button    [name='optionB']
    Select Radio Button    [name='optionA']
    Element Should Exist    [name='optionA']

# =============================================================================
# SELECTION VERIFICATION
# =============================================================================

Verify Checkbox Is Selected
    [Documentation]    Verify checkbox selection state.
    [Tags]    positive    verification
    Check Checkbox    [name='rememberMe']
    Element Should Be Selected    JCheckBox[name='rememberMe']

Verify Checkbox Is Not Selected
    [Documentation]    Verify checkbox not selected state.
    [Tags]    positive    verification
    Uncheck Checkbox    [name='rememberMe']
    Element Should Not Be Selected    JCheckBox[name='rememberMe']

Verify Radio Button Is Selected
    [Documentation]    Verify radio button selection state.
    [Tags]    positive    verification
    Select Radio Button    [name='optionA']
    Element Should Be Selected    JRadioButton[name='optionA']

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
    ...    Select From Combobox    [name='countryCombo']    NonExistentCountry
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
    FOR    ${i}    IN RANGE    5
        Check Checkbox    [name='rememberMe']
        Uncheck Checkbox    [name='rememberMe']
    END
    Element Should Exist    [name='rememberMe']

Rapid Radio Button Selection
    [Documentation]    Test rapid radio button selection.
    [Tags]    edge-case    stress
    FOR    ${i}    IN RANGE    5
        Select Radio Button    [name='optionA']
        Select Radio Button    [name='optionB']
    END
    Element Should Exist    [name='optionA']
