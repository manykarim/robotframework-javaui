*** Settings ***
Documentation     Element Finding Tests - Testing find_element, find_elements,
...               wait_until_element_exists, and wait_until_element_does_not_exist keywords.
...
...               These tests verify the library's ability to locate UI elements
...               using various locator strategies including CSS selectors and XPath.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        element-finding    regression

*** Test Cases ***
# =============================================================================
# FIND ELEMENT - BY NAME
# =============================================================================

Find Element By Name Attribute
    [Documentation]    Find a single element using the name attribute selector.
    ...                Uses CSS-style [name='value'] syntax.
    [Tags]    smoke    positive    css-locator
    ${element}=    Find Element    [name='loginBtn']
    Should Not Be Equal    ${element}    ${NONE}

Find Element By Name With Type
    [Documentation]    Find element using type and name attribute together.
    ...                Combines component type with attribute selector.
    [Tags]    smoke    positive    css-locator
    ${element}=    Find Element    JButton[name='loginBtn']
    Should Not Be Equal    ${element}    ${NONE}

Find Element By Name Using ID Syntax
    [Documentation]    Find element using CSS ID-style selector.
    ...                The # prefix matches the name attribute.
    [Tags]    positive    css-locator
    ${element}=    Find Element    \#loginBtn
    Should Not Be Equal    ${element}    ${NONE}

# =============================================================================
# FIND ELEMENT - BY TEXT
# =============================================================================

Find Element By Text Attribute
    [Documentation]    Find element by its text content using attribute selector.
    [Tags]    smoke    positive    css-locator
    ${element}=    Find Element    JButton[text='Login']
    Should Not Be Equal    ${element}    ${NONE}

Find Element By Text Contains
    [Documentation]    Find element by partial text match using *= operator.
    [Tags]    positive    css-locator
    ${element}=    Find Element    JButton[text*='Log']
    Should Not Be Equal    ${element}    ${NONE}

Find Element By Text Starts With
    [Documentation]    Find element by text prefix using ^= operator.
    [Tags]    positive    css-locator
    ${element}=    Find Element    JButton[text^='Log']
    Should Not Be Equal    ${element}    ${NONE}

Find Element By Text Ends With
    [Documentation]    Find element by text suffix using $= operator.
    [Tags]    positive    css-locator
    ${element}=    Find Element    JLabel[text$=':']
    Should Not Be Equal    ${element}    ${NONE}

# =============================================================================
# FIND ELEMENT - BY CLASS/TYPE
# =============================================================================

Find Element By Component Type
    [Documentation]    Find element by its Swing component type.
    ...                Uses the simple class name as selector.
    [Tags]    smoke    positive    css-locator
    ${element}=    Find Element    JButton
    Should Not Be Equal    ${element}    ${NONE}

Find Element By Type JTextField
    [Documentation]    Find a text field element by type.
    [Tags]    positive    css-locator
    ${element}=    Find Element    JTextField
    Should Not Be Equal    ${element}    ${NONE}

Find Element By Type JLabel
    [Documentation]    Find a label element by type.
    [Tags]    positive    css-locator
    ${element}=    Find Element    JLabel
    Should Not Be Equal    ${element}    ${NONE}

Find Element By Type JTabbedPane
    [Documentation]    Find a tabbed pane element by type.
    [Tags]    positive    css-locator
    ${element}=    Find Element    JTabbedPane
    Should Not Be Equal    ${element}    ${NONE}

Find Element By Type JTable
    [Documentation]    Find a table element by type.
    [Tags]    positive    css-locator
    ${element}=    Find Element    JTable
    Should Not Be Equal    ${element}    ${NONE}

Find Element By Type JTree
    [Documentation]    Find a tree element by type.
    [Tags]    positive    css-locator
    ${element}=    Find Element    JTree
    Should Not Be Equal    ${element}    ${NONE}

# =============================================================================
# FIND ELEMENT - XPATH SELECTORS
# =============================================================================

Find Element By XPath Simple
    [Documentation]    Find element using simple XPath syntax.
    ...                Uses //Type to find any matching component.
    [Tags]    smoke    positive    xpath-locator
    ${element}=    Find Element    //JButton
    Should Not Be Equal    ${element}    ${NONE}

Find Element By XPath With Attribute
    [Documentation]    Find element using XPath with attribute predicate.
    ...                Uses [@attr='value'] syntax.
    [Tags]    positive    xpath-locator
    ${element}=    Find Element    //JButton[@name='loginBtn']
    Should Not Be Equal    ${element}    ${NONE}

Find Element By XPath With Text
    [Documentation]    Find element using XPath with text attribute.
    [Tags]    positive    xpath-locator
    ${element}=    Find Element    //JButton[@text='Login']
    Should Not Be Equal    ${element}    ${NONE}

Find Element By XPath With Index
    [Documentation]    Find element using XPath index predicate.
    ...                Uses [n] to select nth matching element.
    [Tags]    positive    xpath-locator
    ${element}=    Find Element    //JButton[1]
    Should Not Be Equal    ${element}    ${NONE}

Find Element By XPath Descendant
    [Documentation]    Find element as descendant using XPath.
    [Tags]    positive    xpath-locator
    ${element}=    Find Element    //JPanel//JButton
    Should Not Be Equal    ${element}    ${NONE}

# =============================================================================
# FIND ELEMENT - CSS COMBINATORS
# =============================================================================

Find Element With Child Combinator
    [Documentation]    Find element using CSS child combinator (>).
    ...                Matches direct child elements only.
    [Tags]    positive    css-locator    combinator
    ${element}=    Find Element    JPanel > JButton
    Should Not Be Equal    ${element}    ${NONE}

Find Element With Descendant Combinator
    [Documentation]    Find element using CSS descendant combinator (space).
    ...                Matches any descendant elements.
    [Tags]    positive    css-locator    combinator
    ${element}=    Find Element    JPanel JButton
    Should Not Be Equal    ${element}    ${NONE}

Find Element In Nested Container
    [Documentation]    Find element within multiple nested containers.
    [Tags]    positive    css-locator    combinator
    ${element}=    Find Element    JTabbedPane JPanel JButton
    Should Not Be Equal    ${element}    ${NONE}

# =============================================================================
# FIND ELEMENT - PSEUDO SELECTORS
# =============================================================================

Find Element With Enabled Pseudo Selector
    [Documentation]    Find enabled elements using :enabled pseudo selector.
    [Tags]    positive    css-locator    pseudo-selector
    ${element}=    Find Element    JButton:enabled
    Should Not Be Equal    ${element}    ${NONE}

Find Element With Visible Pseudo Selector
    [Documentation]    Find visible elements using :visible pseudo selector.
    [Tags]    positive    css-locator    pseudo-selector
    ${element}=    Find Element    JButton:visible
    Should Not Be Equal    ${element}    ${NONE}

Find Element With First Child Pseudo Selector
    [Documentation]    Find first child elements using :first-child pseudo selector.
    [Tags]    positive    css-locator    pseudo-selector
    ${element}=    Find Element    JButton:first-child
    Should Not Be Equal    ${element}    ${NONE}

Find Element With Combined Pseudo Selectors
    [Documentation]    Find element with multiple pseudo selectors.
    [Tags]    positive    css-locator    pseudo-selector
    ${element}=    Find Element    JButton:enabled:visible
    Should Not Be Equal    ${element}    ${NONE}

# =============================================================================
# FIND ELEMENT - COMPLEX SELECTORS
# =============================================================================

Find Element With Multiple Attributes
    [Documentation]    Find element using multiple attribute selectors.
    [Tags]    positive    css-locator    complex
    ${element}=    Find Element    JButton[name='loginBtn'][text='Login']
    Should Not Be Equal    ${element}    ${NONE}

Find Element With Attribute And Pseudo Selector
    [Documentation]    Combine attribute selectors with pseudo selectors.
    [Tags]    positive    css-locator    complex
    ${element}=    Find Element    JButton[name='loginBtn']:enabled
    Should Not Be Equal    ${element}    ${NONE}

Find Element With Type Attribute And Pseudo
    [Documentation]    Full combination of type, attribute, and pseudo selectors.
    [Tags]    positive    css-locator    complex
    ${element}=    Find Element    JButton[name='loginBtn']:enabled:visible
    Should Not Be Equal    ${element}    ${NONE}

Find Element With Combinator And Attribute
    [Documentation]    Combine descendant combinator with attribute selector.
    [Tags]    positive    css-locator    complex
    ${element}=    Find Element    JPanel JButton[text='Login']
    Should Not Be Equal    ${element}    ${NONE}

# =============================================================================
# FIND ELEMENTS - MULTIPLE RESULTS
# =============================================================================

Find All Buttons
    [Documentation]    Find all button elements in the application.
    [Tags]    smoke    positive
    ${elements}=    Find Elements    JButton
    ${count}=    Get Length    ${elements}
    Should Be True    ${count} > 5    Should find multiple buttons
    Log    Found ${count} buttons

Find All Text Fields
    [Documentation]    Find all text field elements in the application.
    [Tags]    positive
    ${elements}=    Find Elements    JTextField
    Should Not Be Empty    ${elements}

Find All Labels
    [Documentation]    Find all label elements in the application.
    [Tags]    positive
    ${elements}=    Find Elements    JLabel
    ${count}=    Get Length    ${elements}
    Should Be True    ${count} > 3    Should find multiple labels

Find Elements With Attribute
    [Documentation]    Find multiple elements matching an attribute pattern.
    [Tags]    positive
    ${elements}=    Find Elements    JLabel[text$=':']
    Should Not Be Empty    ${elements}

Find Elements Returns Empty For No Match
    [Documentation]    Find elements returns empty list when no matches.
    [Tags]    positive    edge-case
    ${elements}=    Find Elements    JButton[name='nonexistent_xyz']
    ${count}=    Get Length    ${elements}
    Should Be Equal As Integers    ${count}    0

Find Elements Using XPath
    [Documentation]    Find multiple elements using XPath selector.
    [Tags]    positive    xpath-locator
    ${elements}=    Find Elements    //JButton
    ${count}=    Get Length    ${elements}
    Should Be True    ${count} > 0

# =============================================================================
# WAIT UNTIL ELEMENT EXISTS
# =============================================================================

Wait Until Element Exists By Name
    [Documentation]    Wait for an element to exist using name selector.
    [Tags]    smoke    positive    wait
    Wait Until Element Exists    [name='loginBtn']    timeout=${DEFAULT_TIMEOUT}

Wait Until Element Exists By Type
    [Documentation]    Wait for an element to exist using type selector.
    [Tags]    positive    wait
    Wait Until Element Exists    JButton    timeout=${DEFAULT_TIMEOUT}

Wait Until Element Exists By XPath
    [Documentation]    Wait for an element to exist using XPath selector.
    [Tags]    positive    wait    xpath-locator
    Wait Until Element Exists    //JButton[@name='loginBtn']    timeout=${DEFAULT_TIMEOUT}

Wait Until Element Exists With Short Timeout
    [Documentation]    Wait for an existing element with short timeout.
    [Tags]    positive    wait
    Wait Until Element Exists    ${MAIN_TABS}    timeout=${SHORT_TIMEOUT}

Wait Until Element Exists Default Timeout
    [Documentation]    Wait for element using library default timeout.
    [Tags]    positive    wait
    Wait Until Element Exists    ${LOGIN_BUTTON}

# =============================================================================
# WAIT UNTIL ELEMENT DOES NOT EXIST
# =============================================================================

Wait Until Element Does Not Exist For Missing Element
    [Documentation]    Wait confirms that a non-existent element doesn't exist.
    [Tags]    positive    wait
    Wait Until Element Does Not Exist    JButton[name='nonexistent_btn']    timeout=${SHORT_TIMEOUT}

Wait Until Element Does Not Exist With XPath
    [Documentation]    Wait for element non-existence using XPath selector.
    [Tags]    positive    wait    xpath-locator
    Wait Until Element Does Not Exist    //JDialog[@name='nonexistent']    timeout=${SHORT_TIMEOUT}

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Find Element Fails For Nonexistent
    [Documentation]    Find element throws error when element doesn't exist.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Find Element    JButton[name='definitely_nonexistent_element']
    Should Be Equal    ${status}    ${FALSE}

Find Element With Invalid Locator Syntax
    [Documentation]    Find element handles invalid locator syntax gracefully.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Find Element    [[[invalid_syntax]]]
    Should Be Equal    ${status}    ${FALSE}

Wait Until Element Exists Timeout
    [Documentation]    Wait throws timeout error when element doesn't appear.
    [Tags]    negative    error-handling    wait
    ${status}=    Run Keyword And Return Status
    ...    Wait Until Element Exists    JButton[name='never_exists']    timeout=2
    Should Be Equal    ${status}    ${FALSE}

Wait Until Element Does Not Exist Timeout
    [Documentation]    Wait throws timeout when existing element doesn't disappear.
    [Tags]    negative    error-handling    wait
    ${status}=    Run Keyword And Return Status
    ...    Wait Until Element Does Not Exist    ${LOGIN_BUTTON}    timeout=2
    Should Be Equal    ${status}    ${FALSE}

# =============================================================================
# EDGE CASES
# =============================================================================

Find Element With Empty Locator
    [Documentation]    Find element handles empty locator gracefully.
    [Tags]    edge-case    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Find Element    ${EMPTY}
    Should Be Equal    ${status}    ${FALSE}

Find Element With Special Characters In Text
    [Documentation]    Find element handles special characters in text search.
    [Tags]    edge-case
    ${element}=    Find Element    JLabel[text='Username:']
    Should Not Be Equal    ${element}    ${NONE}

Find Elements With Locator Matching Many
    [Documentation]    Find elements handles large result sets.
    [Tags]    edge-case    performance
    ${elements}=    Find Elements    *
    ${count}=    Get Length    ${elements}
    Log    Found ${count} total elements
    Should Be True    ${count} > 20    Should find many elements
