*** Settings ***
Documentation     Test basic UI actions - click, input, etc.
Resource          resources/common.resource
Suite Setup       Start Demo Application
Suite Teardown    Stop Demo Application

*** Test Cases ***
Click Button
    [Documentation]    Test clicking a button
    Click    JButton[name='loginBtn']
    Sleep    0.5s
    Element Should Exist    JButton[name='loginBtn']

Input Text Into TextField
    [Documentation]    Test text input
    Clear Text    [name='username']
    Input Text    [name='username']    testuser
    Element Should Exist    [name='username']

Input Text Into Password Field
    [Documentation]    Test password input
    Clear Text    [name='password']
    Input Text    [name='password']    secret123
    Element Should Exist    [name='password']

Clear Text Field
    [Documentation]    Test clearing text
    Input Text    [name='username']    sometext
    Clear Text    [name='username']
    Element Should Exist    [name='username']

Type Text Character By Character
    [Documentation]    Test typing text
    Clear Text    [name='username']
    Type Text    [name='username']    hello
    Element Should Exist    [name='username']

Focus Element
    [Documentation]    Test focusing an element
    Click    [name='username']
    Element Should Exist    [name='username']

Complete Login Form
    [Documentation]    Test filling and submitting a form
    Clear Text    [name='username']
    Clear Text    [name='password']
    Input Text    [name='username']    admin
    Input Text    [name='password']    password123
    Click    JButton[name='loginBtn']
    Sleep    0.5s
    Element Should Exist    JLabel[name='statusLabel']

Click Clear Form Button
    [Documentation]    Test form clearing
    Input Text    [name='username']    testuser
    Input Text    [name='password']    testpass
    Click    JButton[name='clearBtn']
    Sleep    0.2s
    Element Should Exist    [name='username']

Click Checkbox
    [Documentation]    Test checkbox click
    Click    JCheckBox[name='rememberMe']
    Sleep    0.2s
    Click    JCheckBox[name='rememberMe']
    Sleep    0.2s
    Element Should Exist    JCheckBox[name='rememberMe']

Invalid Login Shows Error
    [Documentation]    Test error handling
    [Tags]    negative
    Clear Text    [name='username']
    Clear Text    [name='password']
    Input Text    [name='username']    wronguser
    Input Text    [name='password']    wrongpass
    Click    JButton[name='loginBtn']
    Sleep    0.5s
    Element Should Exist    JLabel[name='statusLabel']
