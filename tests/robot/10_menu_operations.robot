*** Settings ***
Documentation     Test menu operations
Resource          resources/common.resource
Suite Setup       Start Demo Application
Suite Teardown    Stop Demo Application

*** Test Cases ***
Menu Bar Should Be Visible
    [Documentation]    Verify menu bar is displayed
    Element Should Be Visible    JMenuBar

Find File Menu
    [Documentation]    Find File menu
    Element Should Exist    JMenu[text='File']

Find Edit Menu
    [Documentation]    Find Edit menu
    Element Should Exist    JMenu[text='Edit']

Find Help Menu
    [Documentation]    Find Help menu
    Element Should Exist    JMenu[text='Help']

Click File Menu
    [Documentation]    Open File menu
    Click    JMenu[text='File']
    Sleep    0.5s
    Element Should Exist    JMenu[text='File']

Click Edit Menu
    [Documentation]    Open Edit menu
    Click    JMenu[text='Edit']
    Sleep    0.5s
    Element Should Exist    JMenu[text='Edit']

Click Help Menu
    [Documentation]    Open Help menu
    Click    JMenu[text='Help']
    Sleep    0.5s
    Element Should Exist    JMenu[text='Help']

Menu Existence Verification
    [Documentation]    Test menu existence
    Element Should Exist    JMenu[text='File']
    Element Should Exist    JMenu[text='Edit']
    Element Should Exist    JMenu[text='Help']
