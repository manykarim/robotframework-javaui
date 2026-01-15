*** Settings ***
Documentation    SWT ComboBox Controls - Testing Combo and CCombo
...              Operations: Select By Text, Select By Index, Get Selected, Verify Options
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Select Combo Item By Text
    [Documentation]    Select an item from combo by its text value
    [Tags]    combo    select    text    positive
    Select From Combobox    Combo[name='categoryCombo']    Books
    ${selected}=    Get Combobox Selected Item    Combo[name='categoryCombo']
    Should Be Equal    ${selected}    Books

Select Combo Item By Index
    [Documentation]    Select an item from combo by index
    [Tags]    combo    select    index    positive
    Select From Combobox By Index    Combo[name='categoryCombo']    0
    ${selected}=    Get Combobox Selected Item    Combo[name='categoryCombo']
    Should Be Equal    ${selected}    Electronics

Select Multiple Combo Items Sequentially
    [Documentation]    Select different items from combo in sequence
    [Tags]    combo    select    sequence    positive
    Select From Combobox    Combo[name='categoryCombo']    Electronics
    ${selected}=    Get Combobox Selected Item    Combo[name='categoryCombo']
    Should Be Equal    ${selected}    Electronics

    Select From Combobox    Combo[name='categoryCombo']    Clothing
    ${selected}=    Get Combobox Selected Item    Combo[name='categoryCombo']
    Should Be Equal    ${selected}    Clothing

    Select From Combobox    Combo[name='categoryCombo']    Home & Garden
    ${selected}=    Get Combobox Selected Item    Combo[name='categoryCombo']
    Should Be Equal    ${selected}    Home & Garden

Get Combo Item Count
    [Documentation]    Get the number of items in the combo
    [Tags]    combo    count    positive
    ${count}=    Get Combobox Item Count    Combo[name='categoryCombo']
    Should Be True    ${count} >= 5

Combo Should Be Enabled
    [Documentation]    Verify combo is enabled
    [Tags]    combo    enabled    verification
    Element Should Be Enabled    Combo[name='categoryCombo']

Combo Should Be Visible
    [Documentation]    Verify combo is visible
    [Tags]    combo    visible    verification
    Element Should Be Visible    Combo[name='categoryCombo']

Select CCombo Item
    [Documentation]    Select an item from CCombo (custom combo)
    [Tags]    ccombo    select    positive
    Select From Combobox    CCombo[name='priorityCCombo']    High
    ${selected}=    Get Combobox Selected Item    CCombo[name='priorityCCombo']
    Should Be Equal    ${selected}    High

Select Theme Combo
    [Documentation]    Select theme from combo
    [Tags]    combo    select    positive
    Select From Combobox    Combo[name='themeCombo']    Dark
    ${selected}=    Get Combobox Selected Item    Combo[name='themeCombo']
    Should Be Equal    ${selected}    Dark
