*** Settings ***
Documentation    SWT List Controls - Testing List widget
...              Operations: Select By Text, Select By Index, Get Selected, Multi-select
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Select List Item By Text
    [Documentation]    Select an item from list by its text value
    [Tags]    list    select    text    positive
    Select From List    List[name='itemList']    Item 3 - Cherry
    ${selected}=    Get List Selected Item    List[name='itemList']
    Should Be Equal    ${selected}    Item 3 - Cherry

Select List Item By Index
    [Documentation]    Select an item from list by index
    [Tags]    list    select    index    positive
    Select From List By Index    List[name='itemList']    0
    ${selected}=    Get List Selected Item    List[name='itemList']
    Should Be Equal    ${selected}    Item 1 - Apple

Select Multiple List Items Sequentially
    [Documentation]    Select different items from list in sequence
    [Tags]    list    select    sequence    positive
    Select From List    List[name='itemList']    Item 1 - Apple
    ${selected}=    Get List Selected Item    List[name='itemList']
    Should Be Equal    ${selected}    Item 1 - Apple

    Select From List    List[name='itemList']    Item 2 - Banana
    ${selected}=    Get List Selected Item    List[name='itemList']
    Should Be Equal    ${selected}    Item 2 - Banana

    Select From List    List[name='itemList']    Item 4 - Date
    ${selected}=    Get List Selected Item    List[name='itemList']
    Should Be Equal    ${selected}    Item 4 - Date

Get List Item Count
    [Documentation]    Get the number of items in the list
    [Tags]    list    count    positive
    ${count}=    Get List Item Count    List[name='itemList']
    Should Be True    ${count} >= 5

List Should Be Enabled
    [Documentation]    Verify list is enabled
    [Tags]    list    enabled    verification
    Element Should Be Enabled    List[name='itemList']

List Should Be Visible
    [Documentation]    Verify list is visible
    [Tags]    list    visible    verification
    Element Should Be Visible    List[name='itemList']

Select First And Last List Items
    [Documentation]    Select the first and last items in the list
    [Tags]    list    select    boundary    positive
    Select From List By Index    List[name='itemList']    0
    ${selected}=    Get List Selected Item    List[name='itemList']
    Should Contain    ${selected}    Item 1
    ${count}=    Get List Item Count    List[name='itemList']
    ${last_index}=    Evaluate    ${count} - 1
    Select From List By Index    List[name='itemList']    ${last_index}
    ${selected}=    Get List Selected Item    List[name='itemList']
    Should Not Be Empty    ${selected}
