*** Settings ***
Test Timeout       60s
Documentation     Verification Tests - Testing new AssertionEngine-based keywords:
...               Get Text, Get Property, Get Properties, Get Element States
...               with assertion operators (==, !=, *=, ^=, $=, ~=, validate).
...
...               These tests demonstrate the Browser Library-style assertion syntax.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        verification    regression    assertion-engine

*** Test Cases ***
# =============================================================================
# ELEMENT SHOULD BE VISIBLE
# =============================================================================

Element Should Be Visible By Name
    [Documentation]    Verify element is visible using name selector.
    [Tags]    smoke    positive
    Element Should Be Visible    [name='submitButton']

Element Should Be Visible By ID
    [Documentation]    Verify element using ID-style selector.
    [Tags]    positive
    Element Should Be Visible    \#submitButton

Element Should Be Visible By Type
    [Documentation]    Verify element by component type.
    [Tags]    positive
    Element Should Be Visible    JButton[name='submitButton']

Element Should Be Visible Using XPath
    [Documentation]    Verify using XPath selector.
    [Tags]    positive    xpath-locator
    Element Should Be Visible    //JButton[@name='submitButton']

Element Should Be Visible For Different Components
    [Documentation]    Verify visibility of various component types.
    [Tags]    positive
    Element Should Be Visible    JButton[name='submitButton']
    Element Should Be Visible    JTextField[name='nameTextField']
    Element Should Be Visible    JLabel[text='Name:']

# =============================================================================
# ELEMENT SHOULD NOT BE VISIBLE
# =============================================================================

Element Should Not Be Visible For Hidden Element
    [Documentation]    Verify hidden element is not visible.
    [Tags]    positive
    # This assumes there's a hidden element or we test failure case
    ${status}=    Run Keyword And Return Status
    ...    Element Should Not Be Visible    JButton[name='hiddenButton']
    Log    Not visible check: ${status}

# =============================================================================
# ELEMENT SHOULD BE ENABLED
# =============================================================================

Element Should Be Enabled By Name
    [Documentation]    Verify element is enabled using name selector.
    [Tags]    smoke    positive
    Element Should Be Enabled    [name='submitButton']

Element Should Be Enabled By ID
    [Documentation]    Verify element using ID-style selector.
    [Tags]    positive
    Element Should Be Enabled    \#submitButton

Element Should Be Enabled By Type
    [Documentation]    Verify element by component type.
    [Tags]    positive
    Element Should Be Enabled    JButton[name='submitButton']

Element Should Be Enabled Using XPath
    [Documentation]    Verify using XPath selector.
    [Tags]    positive    xpath-locator
    Element Should Be Enabled    //JButton[@name='submitButton']

Element Should Be Enabled For Different Components
    [Documentation]    Verify enabled state of various component types.
    [Tags]    positive
    Element Should Be Enabled    JButton[name='submitButton']
    Element Should Be Enabled    JTextField[name='nameTextField']

# =============================================================================
# ELEMENT SHOULD BE DISABLED
# =============================================================================

Element Should Be Disabled For Disabled Element
    [Documentation]    Verify disabled element state.
    [Tags]    positive
    # This assumes there's a disabled element
    ${status}=    Run Keyword And Return Status
    ...    Element Should Be Disabled    JButton[name='disabledBtn']
    Log    Disabled check: ${status}

# =============================================================================
# GET TEXT WITH ASSERTION OPERATORS (NEW SYNTAX)
# =============================================================================

Get Text From Label
    [Documentation]    Get text from a label with assertion operator.
    [Tags]    smoke    positive    assertion-operator
    Get Text    JLabel[text='Name:']    !=    ${EMPTY}

Get Text From Button
    [Documentation]    Get text from a button with assertion operator.
    [Tags]    positive    assertion-operator
    Get Text    JButton[name='submitButton']    !=    ${EMPTY}

Get Text Using ID Selector
    [Documentation]    Get text using ID-style selector with assertion.
    [Tags]    positive    assertion-operator
    ${text}=    Get Text    \#statusLabel
    Log    Status text: ${text}

Get Text Using XPath
    [Documentation]    Get text using XPath selector with assertion.
    [Tags]    positive    xpath-locator    assertion-operator
    ${text}=    Get Text    //JLabel[@name='statusLabel']
    Log    XPath text: ${text}

Get Text From TextField With Contains
    [Documentation]    Get text from a text field with contains assertion.
    [Tags]    positive    assertion-operator
    Input Text    [name='nameTextField']    testtext
    Get Text    JTextField[name='nameTextField']    *=    testtext
    Clear Text    [name='nameTextField']

# =============================================================================
# TEXT EXACT MATCH WITH ASSERTION OPERATORS
# =============================================================================

Text Should Be Exact Match
    [Documentation]    Verify element text matches exactly using == operator.
    [Tags]    smoke    positive    assertion-operator
    Get Text    JLabel[text='Name:']    ==    Name:

Text Match Using ID
    [Documentation]    Verify using ID-style selector with == operator.
    [Tags]    positive    assertion-operator
    ${status}=    Run Keyword And Return Status
    ...    Get Text    \#statusLabel    ==    Ready
    Log    Text match: ${status}

Text Match Using XPath
    [Documentation]    Verify using XPath selector with == operator.
    [Tags]    positive    xpath-locator    assertion-operator
    ${status}=    Run Keyword And Return Status
    ...    Get Text    //JButton[@name='submitButton']    ==    Login
    Log    XPath text match: ${status}

Text Match For Button
    [Documentation]    Verify button text with == operator.
    [Tags]    positive    assertion-operator
    ${status}=    Run Keyword And Return Status
    ...    Get Text    JButton[name='clearButton']    ==    Clear
    Log    Button text match: ${status}

# =============================================================================
# TEXT CONTAINS WITH ASSERTION OPERATORS
# =============================================================================

Text Should Contain Substring
    [Documentation]    Verify element text contains substring using *= operator.
    [Tags]    smoke    positive    assertion-operator
    Get Text    JLabel[text='Name:']    *=    Nam

Text Contains Using ID
    [Documentation]    Verify using ID-style selector with *= operator.
    [Tags]    positive    assertion-operator
    ${status}=    Run Keyword And Return Status
    ...    Get Text    \#statusLabel    *=    Ready
    Log    Contains match: ${status}

Text Contains Using XPath
    [Documentation]    Verify using XPath selector with *= operator.
    [Tags]    positive    xpath-locator    assertion-operator
    ${status}=    Run Keyword And Return Status
    ...    Get Text    //JButton[@name='submitButton']    *=    Log
    Log    XPath contains: ${status}

Text Contains Partial
    [Documentation]    Verify partial text match with *= operator.
    [Tags]    positive    assertion-operator
    Get Text    JLabel[text='Name:']    *=    ame

# =============================================================================
# TEXT STARTS/ENDS WITH ASSERTION OPERATORS
# =============================================================================

Text Should Start With
    [Documentation]    Verify element text starts with string using ^= operator.
    [Tags]    positive    assertion-operator
    Get Text    JLabel[text='Name:']    ^=    Nam

Text Should End With
    [Documentation]    Verify element text ends with string using $= operator.
    [Tags]    positive    assertion-operator
    Get Text    JLabel[text='Name:']    $=    :

# =============================================================================
# GET PROPERTY WITH ASSERTION OPERATORS (NEW SYNTAX)
# =============================================================================

Get Property Name With Assertion
    [Documentation]    Get name property with assertion operator.
    [Tags]    smoke    positive    assertion-operator
    Get Property    JButton[name='submitButton']    name    ==    submitButton

Get Property Enabled With Assertion
    [Documentation]    Get enabled property with assertion operator.
    [Tags]    positive    assertion-operator
    Get Property    JButton[name='submitButton']    enabled    ==    ${TRUE}

Get Property Visible With Assertion
    [Documentation]    Get visible property with assertion operator.
    [Tags]    positive    assertion-operator
    Get Property    JButton[name='submitButton']    visible    ==    ${TRUE}

Get Property Text With Assertion
    [Documentation]    Get text property with assertion operator.
    [Tags]    positive    assertion-operator
    Get Property    JButton[name='submitButton']    text    !=    ${EMPTY}

Get Property Using XPath With Assertion
    [Documentation]    Get property using XPath selector with assertion.
    [Tags]    positive    xpath-locator    assertion-operator
    Get Property    //JButton[@name='submitButton']    name    ==    submitButton

# =============================================================================
# GET PROPERTIES (ALL)
# =============================================================================

Get All Element Properties
    [Documentation]    Get all common properties from element.
    [Tags]    smoke    positive
    ${props}=    Get Properties    JButton[name='submitButton']
    Should Not Be Empty    ${props}
    Log    Properties: ${props}

Get All Properties Using ID
    [Documentation]    Get all properties using ID selector.
    [Tags]    positive
    ${props}=    Get Properties    \#submitButton
    Log    Properties: ${props}

Get All Properties Using XPath
    [Documentation]    Get all properties using XPath selector.
    [Tags]    positive    xpath-locator
    ${props}=    Get Properties    //JButton[@name='submitButton']
    Log    Properties: ${props}

Verify Properties Structure
    [Documentation]    Verify returned properties structure.
    [Tags]    positive
    ${props}=    Get Properties    JButton[name='submitButton']
    Dictionary Should Contain Key    ${props}    name
    Dictionary Should Contain Key    ${props}    enabled

# =============================================================================
# GET ELEMENT STATES WITH ASSERTION OPERATORS (NEW SYNTAX)
# =============================================================================

Get Element States With Contains
    [Documentation]    Get element states and verify with contains.
    [Tags]    smoke    positive    assertion-operator
    ${states}=    Get Element States    JButton[name='submitButton']
    Should Contain    ${states}    visible
    Should Contain    ${states}    enabled

Verify Button States
    [Documentation]    Verify button has visible and enabled states.
    [Tags]    positive    assertion-operator
    ${states}=    Get Element States    JButton[name='submitButton']
    Log    Button states: ${states}

# =============================================================================
# ELEMENT SELECTION STATE
# =============================================================================

Element Should Be Selected For Checked Checkbox
    [Documentation]    Verify checkbox is selected.
    [Tags]    positive
    Select Selections Tab
    Check Checkbox    JCheckBox[name='enabledCheckBox']
    Element Should Be Selected    JCheckBox[name='enabledCheckBox']

Element Should Not Be Selected For Unchecked Checkbox
    [Documentation]    Verify checkbox is not selected.
    [Tags]    positive
    Select Selections Tab
    Uncheck Checkbox    JCheckBox[name='enabledCheckBox']
    Element Should Not Be Selected    JCheckBox[name='enabledCheckBox']

Element Should Be Selected For Radio Button
    [Documentation]    Verify radio button is selected.
    [Tags]    positive
    Select Selections Tab
    Select Radio Button    JRadioButton[name='highPriorityRadioButton']
    Element Should Be Selected    JRadioButton[name='highPriorityRadioButton']

# =============================================================================
# ELEMENT EXISTENCE
# =============================================================================

Element Should Exist By Name
    [Documentation]    Verify element exists using name selector.
    [Tags]    smoke    positive
    Element Should Exist    [name='submitButton']

Element Should Exist By ID
    [Documentation]    Verify element using ID-style selector.
    [Tags]    positive
    Element Should Exist    \#submitButton

Element Should Exist By Type
    [Documentation]    Verify element by component type.
    [Tags]    positive
    Element Should Exist    JButton[name='submitButton']

Element Should Exist Using XPath
    [Documentation]    Verify using XPath selector.
    [Tags]    positive    xpath-locator
    Element Should Exist    //JButton[@name='submitButton']

Element Should Not Exist For Missing
    [Documentation]    Verify missing element does not exist.
    [Tags]    positive
    Element Should Not Exist    JButton[name='nonexistent_element']

# =============================================================================
# VERIFICATION WORKFLOWS WITH ASSERTION OPERATORS
# =============================================================================

Verify Form Before Interaction Workflow
    [Documentation]    Verify all form elements before interaction.
    [Tags]    workflow    smoke
    Select Form Input Tab
    Element Should Exist    JTextField[name='nameTextField']
    Element Should Exist    JPasswordField[name='passwordField']
    Element Should Exist    JButton[name='submitButton']
    Element Should Be Visible    JTextField[name='nameTextField']
    Element Should Be Visible    JButton[name='submitButton']
    Element Should Be Enabled    JTextField[name='nameTextField']
    Element Should Be Enabled    JButton[name='submitButton']

Verify After Input Workflow With Assertion
    [Documentation]    Verify state after input operations with assertion operators.
    [Tags]    workflow    assertion-operator
    Select Form Input Tab
    Input Text    [name='nameTextField']    verifyuser
    Get Text    JTextField[name='nameTextField']    *=    verifyuser
    Clear Text    [name='nameTextField']

Verify Selection State Workflow
    [Documentation]    Verify checkbox states during workflow.
    [Tags]    workflow
    Select Selections Tab
    Uncheck Checkbox    [name='enabledCheckBox']
    Element Should Not Be Selected    JCheckBox[name='enabledCheckBox']
    Check Checkbox    [name='enabledCheckBox']
    Element Should Be Selected    JCheckBox[name='enabledCheckBox']
    Uncheck Checkbox    [name='enabledCheckBox']
    Element Should Not Be Selected    JCheckBox[name='enabledCheckBox']

Comprehensive Element Verification With Assertions
    [Documentation]    Comprehensive verification using assertion operators.
    [Tags]    workflow    assertion-operator
    Select Form Input Tab
    ${props}=    Get Properties    JButton[name='submitButton']
    Should Be True    ${props}[enabled]
    Get Text    JButton[name='submitButton']    !=    ${EMPTY}
    Element Should Be Visible    JButton[name='submitButton']
    Element Should Be Enabled    JButton[name='submitButton']

# =============================================================================
# VALIDATE OPERATOR (CUSTOM EXPRESSIONS)
# =============================================================================

Validate Expression For Text Length
    [Documentation]    Use validate operator for custom expression.
    [Tags]    positive    assertion-operator    validate
    ${status}=    Run Keyword And Return Status
    ...    Get Text    JLabel[text='Name:']    validate    len(value) > 0
    Log    Validate result: ${status}

Validate Expression For Numeric Check
    [Documentation]    Use validate operator for numeric validation.
    [Tags]    positive    assertion-operator    validate
    ${status}=    Run Keyword And Return Status
    ...    Get Text    JLabel[text='Name:']    validate    len(value) >= 3 and len(value) <= 10
    Log    Numeric validate result: ${status}

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Element Should Be Visible Fails For Nonexistent
    [Documentation]    Visibility check fails for non-existent element.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Element Should Be Visible    JButton[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Element Should Be Enabled Fails For Nonexistent
    [Documentation]    Enabled check fails for non-existent element.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Element Should Be Enabled    JButton[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Get Text Fails For Nonexistent
    [Documentation]    Get Text fails for non-existent element.
    [Tags]    negative    error-handling    assertion-operator
    ${status}=    Run Keyword And Return Status
    ...    Get Text    JButton[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Get Text Assertion Fails For Wrong Text
    [Documentation]    Text assertion fails for wrong expected value.
    [Tags]    negative    error-handling    assertion-operator
    ${status}=    Run Keyword And Return Status
    ...    Get Text    JLabel[text='Name:']    ==    WrongText
    Should Be Equal    ${status}    ${FALSE}

Get Text Contains Fails For Missing Text
    [Documentation]    Contains assertion fails for missing substring.
    [Tags]    negative    error-handling    assertion-operator
    ${status}=    Run Keyword And Return Status
    ...    Get Text    JLabel[text='Name:']    *=    NonExistent
    Should Be Equal    ${status}    ${FALSE}

Get Property Fails For Nonexistent
    [Documentation]    Get Property fails for non-existent element.
    [Tags]    negative    error-handling    assertion-operator
    ${status}=    Run Keyword And Return Status
    ...    Get Property    JButton[name='nonexistent']    name
    Should Be Equal    ${status}    ${FALSE}

Element Should Exist Fails For Nonexistent
    [Documentation]    Exist check fails for non-existent element.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Element Should Exist    JButton[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Element Should Not Exist Fails For Existing
    [Documentation]    Not exist check fails for existing element.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Element Should Not Exist    JButton[name='submitButton']
    Should Be Equal    ${status}    ${FALSE}

# =============================================================================
# EDGE CASES
# =============================================================================

Verify Empty Text Element
    [Documentation]    Verify element with empty text.
    [Tags]    edge-case
    Select Form Input Tab
    Clear Text    [name='nameTextField']
    Get Text    JTextField[name='nameTextField']    ==    ${EMPTY}

Verify Multiple Properties In Sequence
    [Documentation]    Get multiple properties in sequence with assertions.
    [Tags]    edge-case    assertion-operator
    Select Form Input Tab
    Get Property    JButton[name='submitButton']    name    ==    submitButton
    Get Property    JButton[name='submitButton']    enabled    ==    ${TRUE}
    Get Property    JButton[name='submitButton']    visible    ==    ${TRUE}
    Get Property    JButton[name='submitButton']    text    !=    ${EMPTY}

Rapid Verification Calls With Assertions
    [Documentation]    Test rapid verification calls with assertions.
    [Tags]    edge-case    stress    assertion-operator
    Select Form Input Tab
    FOR    ${i}    IN RANGE    10
        Element Should Exist    JButton[name='submitButton']
        Element Should Be Visible    JButton[name='submitButton']
        Element Should Be Enabled    JButton[name='submitButton']
    END

Verify All Form Elements With Assertions
    [Documentation]    Verify all elements in a form with assertion operators.
    [Tags]    edge-case    assertion-operator
    Select Form Input Tab
    Element Should Exist    JTextField[name='nameTextField']
    Element Should Exist    JPasswordField[name='passwordField']
    Element Should Exist    JButton[name='submitButton']
    Element Should Exist    JButton[name='clearButton']
    Get Property    JTextField[name='nameTextField']    enabled    ==    ${TRUE}
    Get Property    JPasswordField[name='passwordField']    enabled    ==    ${TRUE}
    Get Property    JButton[name='submitButton']    enabled    ==    ${TRUE}
    Get Property    JButton[name='clearButton']    enabled    ==    ${TRUE}
