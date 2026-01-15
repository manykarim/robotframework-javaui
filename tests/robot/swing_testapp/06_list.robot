*** Settings ***
Documentation    List Controls - Testing JList selection operations
...              Operations: Select By Text, Select By Index, Get Selected
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Select List Item By Text
    [Documentation]    Select an item from list by its text value
    [Tags]    list    select    text    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Select From List    JList[name='itemList']    Item 3 - Cherry
    ${selected}=    Get Element Text    JList[name='itemList']
    Should Contain    ${selected}    Cherry

Select List Item By Index
    [Documentation]    Select an item from list by index
    [Tags]    list    select    index    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Select List Item By Index    JList[name='itemList']    0
    ${selected}=    Get Element Text    JList[name='itemList']
    Should Contain    ${selected}    Apple

Select Multiple List Items Sequentially
    [Documentation]    Select different items from list in sequence
    [Tags]    list    select    sequence    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    # Select each item
    Select From List    JList[name='itemList']    Item 1 - Apple
    ${selected}=    Get Element Text    JList[name='itemList']
    Should Contain    ${selected}    Apple

    Select From List    JList[name='itemList']    Item 2 - Banana
    ${selected}=    Get Element Text    JList[name='itemList']
    Should Contain    ${selected}    Banana

    Select From List    JList[name='itemList']    Item 4 - Date
    ${selected}=    Get Element Text    JList[name='itemList']
    Should Contain    ${selected}    Date

    Select From List    JList[name='itemList']    Item 7 - Grape
    ${selected}=    Get Element Text    JList[name='itemList']
    Should Contain    ${selected}    Grape

Get List Items
    [Documentation]    Get all items in the list
    [Tags]    list    get    items    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    ${items}=    Get List Items    JList[name='itemList']
    Should Not Be Empty    ${items}

List Should Be Enabled
    [Documentation]    Verify list is enabled
    [Tags]    list    enabled    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Be Enabled    JList[name='itemList']

List Should Be Visible
    [Documentation]    Verify list is visible
    [Tags]    list    visible    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Be Visible    JList[name='itemList']

Select First And Last List Items
    [Documentation]    Select the first and last items in the list
    [Tags]    list    select    boundary    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    # Select first item
    Select List Item By Index    JList[name='itemList']    0
    ${selected}=    Get Element Text    JList[name='itemList']
    Should Contain    ${selected}    Apple
    # Select last item
    Select List Item By Index    JList[name='itemList']    6
    ${selected}=    Get Element Text    JList[name='itemList']
    Should Contain    ${selected}    Grape
