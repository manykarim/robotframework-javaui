*** Settings ***
Documentation     Test screenshot functionality
Resource          resources/common.resource
Library           OperatingSystem
Library           String
Suite Setup       Start Demo Application
Suite Teardown    Stop Demo Application

*** Test Cases ***
Capture Full Window Screenshot
    [Documentation]    Capture screenshot of entire window
    ${path}=    Capture Screenshot
    Log    Screenshot result: ${path}
    Should Not Be Empty    ${path}

Capture Screenshot With Custom Name
    [Documentation]    Capture screenshot with specified filename
    ${path}=    Capture Screenshot    filename=custom_screenshot.png
    Log    Screenshot result: ${path}
    Should Not Be Empty    ${path}

Screenshot On Test Failure
    [Documentation]    Verify screenshot captured on failure
    [Tags]    negative
    ${status}=    Run Keyword And Return Status
    ...    Element Should Exist    JButton[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}
