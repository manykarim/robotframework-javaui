*** Settings ***
Documentation    Text Input Controls - Testing JTextField, JPasswordField, JTextArea
...              Operations: Fill, Get, Clear, Read, Verify
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Fill Text Field And Verify
    [Documentation]    Fill a text field and verify the value was set
    [Tags]    text    fill    positive
    Input Text    JTextField[name='nameTextField']    John Doe
    ${value}=    Get Element Text    JTextField[name='nameTextField']
    Should Be Equal    ${value}    John Doe

Fill Email Field And Verify
    [Documentation]    Fill the email text field and verify
    [Tags]    text    fill    positive
    Input Text    JTextField[name='emailTextField']    john.doe@example.com
    ${value}=    Get Element Text    JTextField[name='emailTextField']
    Should Be Equal    ${value}    john.doe@example.com

Fill Password Field And Verify Not Empty
    [Documentation]    Fill password field and verify it has content (cannot read actual password)
    [Tags]    password    fill    positive
    Input Text    JPasswordField[name='passwordField']    SecretPass123
    Element Should Exist    JPasswordField[name='passwordField']

Fill Text Area And Verify
    [Documentation]    Fill a multi-line text area and verify
    [Tags]    textarea    fill    positive
    Input Text    JTextArea[name='descriptionTextArea']    Line 1 Line 2 Line 3
    ${value}=    Get Element Text    JTextArea[name='descriptionTextArea']
    Should Contain    ${value}    Line 1

Clear Text Field And Verify Empty
    [Documentation]    Clear a text field and verify it is empty
    [Tags]    text    clear    positive
    Input Text    JTextField[name='nameTextField']    Some Text
    Clear Text    JTextField[name='nameTextField']
    ${value}=    Get Element Text    JTextField[name='nameTextField']
    Should Be Empty    ${value}

Clear Text Area And Verify Empty
    [Documentation]    Clear a text area and verify it is empty
    [Tags]    textarea    clear    positive
    Input Text    JTextArea[name='descriptionTextArea']    Some text here
    Clear Text    JTextArea[name='descriptionTextArea']
    ${value}=    Get Element Text    JTextArea[name='descriptionTextArea']
    Should Be Empty    ${value}

Text Field Should Be Enabled
    [Documentation]    Verify text field is enabled for input
    [Tags]    text    enabled    verification
    Element Should Be Enabled    JTextField[name='nameTextField']

Text Field Should Be Visible
    [Documentation]    Verify text field is visible
    [Tags]    text    visible    verification
    Element Should Be Visible    JTextField[name='emailTextField']
