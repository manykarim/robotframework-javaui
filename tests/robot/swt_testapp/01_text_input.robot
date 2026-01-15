*** Settings ***
Documentation    SWT Text Input Controls - Testing Text, Password, Multi-line Text
...              Operations: Fill, Get, Clear, Read, Verify
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Fill Text Field And Verify
    [Documentation]    Fill a text field and verify the value was set
    [Tags]    text    fill    positive
    Fill Text Field    Text[name='nameText']    John Doe
    ${value}=    Get Text Field Value    Text[name='nameText']
    Should Be Equal    ${value}    John Doe

Fill Email Text And Verify
    [Documentation]    Fill the email text field and verify
    [Tags]    text    fill    positive
    Fill Text Field    Text[name='emailText']    john.doe@example.com
    ${value}=    Get Text Field Value    Text[name='emailText']
    Should Be Equal    ${value}    john.doe@example.com

Fill Password Text And Verify Not Empty
    [Documentation]    Fill password field and verify it has content
    [Tags]    password    fill    positive
    Fill Text Field    Text[name='passwordText']    SecretPass123
    Element Should Exist    Text[name='passwordText']

Fill Multi-line Text And Verify
    [Documentation]    Fill a multi-line text area and verify
    [Tags]    textarea    fill    positive
    ${multiline_text}=    Set Variable    Line 1\nLine 2\nLine 3
    Fill Text Field    Text[name='multiLineText']    ${multiline_text}
    ${value}=    Get Text Field Value    Text[name='multiLineText']
    Should Contain    ${value}    Line 1

Clear Text Field And Verify Empty
    [Documentation]    Clear a text field and verify it is empty
    [Tags]    text    clear    positive
    Fill Text Field    Text[name='nameText']    Some Text
    Clear Text Field    Text[name='nameText']
    ${value}=    Get Text Field Value    Text[name='nameText']
    Should Be Empty    ${value}

Text Field Should Be Enabled
    [Documentation]    Verify text field is enabled for input
    [Tags]    text    enabled    verification
    Element Should Be Enabled    Text[name='nameText']

Text Field Should Be Visible
    [Documentation]    Verify text field is visible
    [Tags]    text    visible    verification
    Element Should Be Visible    Text[name='emailText']

Fill Search Text And Verify
    [Documentation]    Fill the search text field
    [Tags]    text    search    positive
    Fill Text Field    Text[name='searchText']    search query
    ${value}=    Get Text Field Value    Text[name='searchText']
    Should Be Equal    ${value}    search query
