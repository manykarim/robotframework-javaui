*** Settings ***
Documentation    ComboBox Controls - Testing JComboBox select/get operations
...              Operations: Select By Text, Get Selected, Verify Options
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Select ComboBox Item By Text
    [Documentation]    Select an item from combobox by its text value
    [Tags]    combobox    select    text    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Select From Combobox    JComboBox[name='categoryComboBox']    Books
    ${selected}=    Get Element Text    JComboBox[name='categoryComboBox']
    Should Be Equal    ${selected}    Books

Select Multiple ComboBox Items Sequentially
    [Documentation]    Select different items from combobox in sequence
    [Tags]    combobox    select    sequence    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    # Select each category
    Select From Combobox    JComboBox[name='categoryComboBox']    Electronics
    ${selected}=    Get Element Text    JComboBox[name='categoryComboBox']
    Should Be Equal    ${selected}    Electronics

    Select From Combobox    JComboBox[name='categoryComboBox']    Clothing
    ${selected}=    Get Element Text    JComboBox[name='categoryComboBox']
    Should Be Equal    ${selected}    Clothing

    Select From Combobox    JComboBox[name='categoryComboBox']    Home & Garden
    ${selected}=    Get Element Text    JComboBox[name='categoryComboBox']
    Should Be Equal    ${selected}    Home & Garden

    Select From Combobox    JComboBox[name='categoryComboBox']    Sports
    ${selected}=    Get Element Text    JComboBox[name='categoryComboBox']
    Should Be Equal    ${selected}    Sports

    Select From Combobox    JComboBox[name='categoryComboBox']    Toys
    ${selected}=    Get Element Text    JComboBox[name='categoryComboBox']
    Should Be Equal    ${selected}    Toys

ComboBox Should Be Enabled
    [Documentation]    Verify combobox is enabled
    [Tags]    combobox    enabled    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Be Enabled    JComboBox[name='categoryComboBox']

ComboBox Should Be Visible
    [Documentation]    Verify combobox is visible
    [Tags]    combobox    visible    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Be Visible    JComboBox[name='categoryComboBox']

Settings Tab Theme ComboBox
    [Documentation]    Test theme combobox on Settings tab
    [Tags]    combobox    settings    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Settings
    Sleep    0.3s
    Select From Combobox    JComboBox[name='themeComboBox']    Dark
    ${selected}=    Get Element Text    JComboBox[name='themeComboBox']
    Should Be Equal    ${selected}    Dark

Settings Tab Language ComboBox
    [Documentation]    Test language combobox on Settings tab
    [Tags]    combobox    settings    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Settings
    Sleep    0.3s
    Select From Combobox    JComboBox[name='languageComboBox']    French
    ${selected}=    Get Element Text    JComboBox[name='languageComboBox']
    Should Be Equal    ${selected}    French
