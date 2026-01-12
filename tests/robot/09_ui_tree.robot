*** Settings ***
Documentation     Test UI tree inspection and visualization
Resource          resources/common.resource
Suite Setup       Start Demo Application
Suite Teardown    Stop Demo Application

*** Test Cases ***
Get UI Tree As Text
    [Documentation]    Get UI tree in text format
    ${tree}=    Get Ui Tree    format=text
    Should Contain    ${tree}    SwingDemoApp
    Log    ${tree}

Log UI Tree
    [Documentation]    Log the UI tree for debugging
    Log Ui Tree

Refresh UI Tree
    [Documentation]    Refresh the UI tree cache
    Refresh Ui Tree
    Element Should Exist    JButton[name='loginBtn']

Verify Tree Contains Specific Components
    [Documentation]    Check for specific components in tree
    ${tree}=    Get Ui Tree    format=text
    Should Contain    ${tree}    mainTabs
    Should Contain    ${tree}    username
    Should Contain    ${tree}    loginBtn

Find Elements In Tree
    [Documentation]    Use tree to find elements
    ${buttons}=    Find Elements    JButton
    ${count}=    Get Length    ${buttons}
    Log    Found ${count} buttons in the UI
    Should Be True    ${count} > 5

Get Properties From Tree
    [Documentation]    Get element properties using tree
    ${props}=    Get Element Properties    JButton[name='loginBtn']
    Log    Button properties: ${props}
    Should Not Be Empty    ${props}

Get Element Property Name
    [Documentation]    Get element name property
    ${name}=    Get Element Property    JButton[name='loginBtn']    name
    Should Be Equal    ${name}    loginBtn

Get Element Property Text
    [Documentation]    Get element text property
    ${text}=    Get Element Property    JButton[name='loginBtn']    text
    Should Be Equal    ${text}    Login

Get Element Property Enabled
    [Documentation]    Get element enabled property
    ${enabled}=    Get Element Property    JButton[name='loginBtn']    enabled
    Should Be True    ${enabled}
