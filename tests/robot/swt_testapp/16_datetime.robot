*** Settings ***
Documentation    SWT DateTime Controls - Testing DateTime widget
...              Operations: Get Date, Set Date, Get Time, Set Time
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Get DateTime Date Value
    [Documentation]    Get the date from DateTime widget
    [Tags]    datetime    get    date    positive
    Select Tab    TabFolder[name='mainTabFolder']    Advanced
    Sleep    0.3s
    Select Tab    CTabFolder[name='advancedTabFolder']    DateTime
    Sleep    0.3s
    ${value}=    Get Element Property    DateTime[name='dateWidget']    year
    Should Be True    ${value} > 2000

Set DateTime Date Value
    [Documentation]    Set a date in DateTime widget
    [Tags]    datetime    set    date    positive
    Select Tab    TabFolder[name='mainTabFolder']    Advanced
    Sleep    0.3s
    Select Tab    CTabFolder[name='advancedTabFolder']    DateTime
    Sleep    0.3s
    Element Should Exist    DateTime[name='dateWidget']

Get DateTime Time Value
    [Documentation]    Get the time from DateTime widget
    [Tags]    datetime    get    time    positive
    Select Tab    TabFolder[name='mainTabFolder']    Advanced
    Sleep    0.3s
    Select Tab    CTabFolder[name='advancedTabFolder']    DateTime
    Sleep    0.3s
    ${value}=    Get Element Property    DateTime[name='timeWidget']    hours
    Should Be True    ${value} >= 0

DateTime Should Be Visible
    [Documentation]    Verify DateTime widget is visible
    [Tags]    datetime    visible    verification
    Select Tab    TabFolder[name='mainTabFolder']    Advanced
    Sleep    0.3s
    Select Tab    CTabFolder[name='advancedTabFolder']    DateTime
    Sleep    0.3s
    Element Should Be Visible    DateTime[name='dateWidget']
    Element Should Be Visible    DateTime[name='timeWidget']

DateTime Should Be Enabled
    [Documentation]    Verify DateTime widget is enabled
    [Tags]    datetime    enabled    verification
    Select Tab    TabFolder[name='mainTabFolder']    Advanced
    Sleep    0.3s
    Select Tab    CTabFolder[name='advancedTabFolder']    DateTime
    Sleep    0.3s
    Element Should Be Enabled    DateTime[name='dateWidget']
    Element Should Be Enabled    DateTime[name='timeWidget']
