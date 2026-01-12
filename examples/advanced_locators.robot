*** Settings ***
Documentation     Examples of advanced locator syntax
Library           SwingLibrary

*** Test Cases ***
CSS-Style Locators
    [Documentation]    Examples of CSS-like selectors

    # Type only
    # Click    JButton

    # ID selector
    # Click    #loginBtn

    # Type with ID
    # Click    JButton#loginBtn

    # Attribute selector
    # Click    JButton[text='OK']

    # Multiple attributes
    # Click    JButton[text='Save'][enabled='true']

    # Pseudo selectors
    # Click    JButton:enabled
    # Click    JButton:visible
    # Click    JButton:first-child
    # Click    JButton:nth-child(2)

    # Child combinator
    # Click    JPanel > JButton

    # Descendant combinator
    # Click    JFrame JPanel JButton

    Log    See documentation for full locator syntax

XPath-Style Locators
    [Documentation]    Examples of XPath-like selectors

    # Descendant axis
    # Click    //JButton

    # Child axis
    # Click    /JPanel/JButton

    # Attribute match
    # Click    //JButton[@text='OK']

    # Name attribute
    # Click    //JTextField[@name='username']

    # Index
    # Click    //JButton[1]

    Log    See documentation for full XPath syntax

Complex Locator Examples
    [Documentation]    Complex locator combinations

    # Type with attribute and pseudo
    # Click    JButton[text='Submit']:enabled

    # Path with multiple filters
    # Click    JPanel#main > JButton[text='OK']:visible

    # Nested containers
    # Click    JFrame > JPanel#content > JPanel#form > JButton#submit

    Log    Complex locators combine multiple selectors
