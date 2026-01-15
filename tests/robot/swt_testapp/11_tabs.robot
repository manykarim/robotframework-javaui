*** Settings ***
Documentation    SWT TabFolder Controls - Testing TabFolder and CTabFolder
...              Operations: Select Tab, Get Selected Tab, Verify Tab Exists
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Select TabFolder Tab By Title
    [Documentation]    Select a tab by its title
    [Tags]    tabs    select    title    positive
    Select Tab    TabFolder[name='mainTabFolder']    Input Controls
    ${selected}=    Get Selected Tab    TabFolder[name='mainTabFolder']
    Should Be Equal    ${selected}    Input Controls

Select All Tabs Sequentially
    [Documentation]    Select each tab in sequence and verify
    [Tags]    tabs    select    sequence    positive
    Select Tab    TabFolder[name='mainTabFolder']    Input Controls
    ${selected}=    Get Selected Tab    TabFolder[name='mainTabFolder']
    Should Be Equal    ${selected}    Input Controls

    Select Tab    TabFolder[name='mainTabFolder']    Data Table
    ${selected}=    Get Selected Tab    TabFolder[name='mainTabFolder']
    Should Be Equal    ${selected}    Data Table

    Select Tab    TabFolder[name='mainTabFolder']    Editor
    ${selected}=    Get Selected Tab    TabFolder[name='mainTabFolder']
    Should Be Equal    ${selected}    Editor

    Select Tab    TabFolder[name='mainTabFolder']    Advanced
    ${selected}=    Get Selected Tab    TabFolder[name='mainTabFolder']
    Should Be Equal    ${selected}    Advanced

Select Tab By Index
    [Documentation]    Select a tab by its index
    [Tags]    tabs    select    index    positive
    Select Tab By Index    TabFolder[name='mainTabFolder']    0
    ${selected}=    Get Selected Tab    TabFolder[name='mainTabFolder']
    Should Be Equal    ${selected}    Input Controls

    Select Tab By Index    TabFolder[name='mainTabFolder']    1
    ${selected}=    Get Selected Tab    TabFolder[name='mainTabFolder']
    Should Be Equal    ${selected}    Data Table

Get Tab Count
    [Documentation]    Get the number of tabs
    [Tags]    tabs    count    positive
    ${count}=    Get Tab Count    TabFolder[name='mainTabFolder']
    Should Be True    ${count} >= 4

Toggle Between Tabs
    [Documentation]    Toggle between tabs multiple times
    [Tags]    tabs    toggle    positive
    Select Tab    TabFolder[name='mainTabFolder']    Input Controls
    ${selected}=    Get Selected Tab    TabFolder[name='mainTabFolder']
    Should Be Equal    ${selected}    Input Controls

    Select Tab    TabFolder[name='mainTabFolder']    Advanced
    ${selected}=    Get Selected Tab    TabFolder[name='mainTabFolder']
    Should Be Equal    ${selected}    Advanced

    Select Tab    TabFolder[name='mainTabFolder']    Input Controls
    ${selected}=    Get Selected Tab    TabFolder[name='mainTabFolder']
    Should Be Equal    ${selected}    Input Controls

TabFolder Should Be Visible
    [Documentation]    Verify tab folder is visible
    [Tags]    tabs    visible    verification
    Element Should Be Visible    TabFolder[name='mainTabFolder']

TabFolder Should Be Enabled
    [Documentation]    Verify tab folder is enabled
    [Tags]    tabs    enabled    verification
    Element Should Be Enabled    TabFolder[name='mainTabFolder']

Select CTabFolder Tab
    [Documentation]    Select a tab in CTabFolder (closeable tabs)
    [Tags]    ctabs    select    positive
    Select Tab    CTabFolder[name='advancedTabFolder']    DateTime
    ${selected}=    Get Selected Tab    CTabFolder[name='advancedTabFolder']
    Should Be Equal    ${selected}    DateTime
