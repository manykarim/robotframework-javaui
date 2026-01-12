*** Settings ***
Documentation     Test wait operations
Resource          resources/common.resource
Suite Setup       Start Demo Application
Suite Teardown    Stop Demo Application

*** Test Cases ***
Wait For Element To Exist
    [Documentation]    Wait for an element to appear
    ${element}=    Wait For Element    JTabbedPane[name='mainTabs']    timeout=10
    Should Not Be Equal    ${element}    ${NONE}

Wait Until Element Visible
    [Documentation]    Wait until element becomes visible
    Wait Until Element Visible    JButton[name='loginBtn']    timeout=10

Wait Until Element Enabled
    [Documentation]    Wait until element becomes enabled
    Wait Until Element Enabled    JButton[name='loginBtn']    timeout=10

Wait With Custom Timeout
    [Documentation]    Test custom timeout handling
    ${start}=    Get Time    epoch
    ${element}=    Wait For Element    JButton[name='loginBtn']    timeout=5
    ${end}=    Get Time    epoch
    ${duration}=    Evaluate    ${end} - ${start}
    Should Be True    ${duration} < 6

Wait For Element Not Found With Timeout
    [Documentation]    Test timeout when element doesn't exist
    [Tags]    negative
    ${status}=    Run Keyword And Return Status
    ...    Wait For Element    JButton[name='nonexistent']    timeout=3
    Should Be Equal    ${status}    ${FALSE}

Wait For Multiple Elements
    [Documentation]    Wait for multiple elements to appear
    Wait Until Element Visible    [name='username']    timeout=5
    Wait Until Element Visible    [name='password']    timeout=5
    Wait Until Element Visible    JButton[name='loginBtn']    timeout=5
    Wait Until Element Visible    JButton[name='clearBtn']    timeout=5
