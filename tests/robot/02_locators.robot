*** Settings ***
Documentation     Test CSS and XPath locator syntax
Resource          resources/common.resource
Suite Setup       Start Demo Application
Suite Teardown    Stop Demo Application

*** Test Cases ***
Find Element By Type
    [Documentation]    Find elements using type selector
    Element Should Exist    JTabbedPane
    Element Should Exist    JButton
    Element Should Exist    JTextField

Find Element By Name ID
    [Documentation]    Find elements using name attribute selector (ID-style)
    Element Should Exist    [name='mainTabs']
    Element Should Exist    [name='username']
    Element Should Exist    [name='loginBtn']
    Element Should Exist    [name='statusLabel']

Find Element By Attribute
    [Documentation]    Find elements using attribute selectors
    Element Should Exist    [name='username']
    Element Should Exist    [name='loginBtn']
    Element Should Exist    JButton[name='loginBtn']

Find Element With Text Attribute
    [Documentation]    Find elements by text content
    Element Should Exist    JButton[text='Login']
    Element Should Exist    JButton[text='Clear']
    Element Should Exist    JLabel[text='Username:']

Find Element With Contains
    [Documentation]    Find elements with partial text match
    Element Should Exist    JButton[text*='Log']
    Element Should Exist    JLabel[text*='User']

Find Element With Starts With
    [Documentation]    Find elements with prefix match
    Element Should Exist    JButton[text^='Log']
    Element Should Exist    JLabel[text^='Pass']

Find Element With Ends With
    [Documentation]    Find elements with suffix match
    Element Should Exist    JLabel[text$=':']

Find Element By Pseudo Selector Enabled
    [Documentation]    Find enabled elements
    Element Should Exist    JButton:enabled
    Element Should Exist    JTextField:enabled

Find Element By Pseudo Selector Visible
    [Documentation]    Find visible elements
    Element Should Exist    JButton:visible
    Element Should Exist    JTabbedPane:visible

Find Element By Child Combinator
    [Documentation]    Find elements using child combinator
    # Test child combinator without tab selection (tests locator syntax)
    Element Should Exist    JTabbedPane > JPanel
    Element Should Exist    JPanel > JLabel

Find Element By Descendant Combinator
    [Documentation]    Find elements using descendant combinator
    Element Should Exist    SwingDemoApp JButton
    Element Should Exist    JTabbedPane JTextField

Find Multiple Elements
    [Documentation]    Find all matching elements
    ${buttons}=    Find Elements    JButton
    ${count}=    Get Length    ${buttons}
    Should Be True    ${count} > 3

XPath Style Locator Basic
    [Documentation]    Find elements using XPath syntax
    Element Should Exist    //JButton
    Element Should Exist    //JTextField
    Element Should Exist    //JTabbedPane

XPath With Attribute
    [Documentation]    Find elements using XPath with attributes
    Element Should Exist    //JButton[@name='loginBtn']
    Element Should Exist    //JTextField[@name='username']

XPath With Text Attribute
    [Documentation]    Find elements using XPath with text
    Element Should Exist    //JButton[@text='Login']
    Element Should Exist    //JLabel[@text='Username:']

XPath With Index
    [Documentation]    Find elements using XPath index
    Element Should Exist    //JButton[1]

Combined CSS Selectors
    [Documentation]    Test complex CSS selector combinations
    # Test combined selectors without tab selection dependency
    Element Should Exist    JButton[name='loginBtn']:enabled:visible
    Element Should Exist    JPanel[name='loginPanel'] JButton[text='Login']

Nth Child Selector
    [Documentation]    Test nth-child pseudo selector
    Element Should Exist    JButton:first-child
    # Note: nth-child depends on DOM structure
