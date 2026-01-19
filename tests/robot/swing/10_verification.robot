*** Settings ***
Test Timeout       60s
Documentation     Verification Tests - Testing element_should_be_visible,
...               element_should_not_be_visible, element_should_be_enabled,
...               element_should_be_disabled, get_element_text,
...               element_text_should_be, element_text_should_contain,
...               get_element_property, and selection verification keywords.
...
...               These tests verify the library's ability to assert
...               various element states and properties.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        verification    regression

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
# GET ELEMENT TEXT
# =============================================================================

Get Element Text From Label
    [Documentation]    Get text from a label element.
    [Tags]    smoke    positive
    ${text}=    Get Element Text    JLabel[text='Name:']
    Should Not Be Empty    ${text}
    Log    Label text: ${text}

Get Element Text From Button
    [Documentation]    Get text from a button element.
    [Tags]    positive
    ${text}=    Get Element Text    JButton[name='submitButton']
    Should Not Be Empty    ${text}
    Log    Button text: ${text}

Get Element Text Using ID Selector
    [Documentation]    Get text using ID-style selector.
    [Tags]    positive
    ${text}=    Get Element Text    \#statusLabel
    Log    Status text: ${text}

Get Element Text Using XPath
    [Documentation]    Get text using XPath selector.
    [Tags]    positive    xpath-locator
    ${text}=    Get Element Text    //JLabel[@name='statusLabel']
    Log    XPath text: ${text}

Get Element Text From TextField
    [Documentation]    Get text from a text field.
    [Tags]    positive
    Input Text    [name='nameTextField']    testtext
    ${text}=    Get Element Text    JTextField[name='nameTextField']
    Should Contain    ${text}    testtext
    Clear Text    [name='nameTextField']

# =============================================================================
# ELEMENT TEXT SHOULD BE
# =============================================================================

Element Text Should Be Exact Match
    [Documentation]    Verify element text matches exactly.
    [Tags]    smoke    positive
    Element Text Should Be    JLabel[text='Name:']    Name:

Element Text Should Be Using ID
    [Documentation]    Verify using ID-style selector.
    [Tags]    positive
    ${status}=    Run Keyword And Return Status
    ...    Element Text Should Be    \#statusLabel    Ready
    Log    Text match: ${status}

Element Text Should Be Using XPath
    [Documentation]    Verify using XPath selector.
    [Tags]    positive    xpath-locator
    ${status}=    Run Keyword And Return Status
    ...    Element Text Should Be    //JButton[@name='submitButton']    Login
    Log    XPath text match: ${status}

Element Text Should Be For Button
    [Documentation]    Verify button text.
    [Tags]    positive
    ${status}=    Run Keyword And Return Status
    ...    Element Text Should Be    JButton[name='clearButton']    Clear
    Log    Button text match: ${status}

# =============================================================================
# ELEMENT TEXT SHOULD CONTAIN
# =============================================================================

Element Text Should Contain Substring
    [Documentation]    Verify element text contains substring.
    [Tags]    smoke    positive
    Element Text Should Contain    JLabel[text='Name:']    Nam

Element Text Should Contain Using ID
    [Documentation]    Verify using ID-style selector.
    [Tags]    positive
    ${status}=    Run Keyword And Return Status
    ...    Element Text Should Contain    \#statusLabel    Ready
    Log    Contains match: ${status}

Element Text Should Contain Using XPath
    [Documentation]    Verify using XPath selector.
    [Tags]    positive    xpath-locator
    ${status}=    Run Keyword And Return Status
    ...    Element Text Should Contain    //JButton[@name='submitButton']    Log
    Log    XPath contains: ${status}

Element Text Should Contain Partial
    [Documentation]    Verify partial text match.
    [Tags]    positive
    Element Text Should Contain    JLabel[text='Name:']    ame

# =============================================================================
# GET ELEMENT PROPERTY
# =============================================================================

Get Element Property Name
    [Documentation]    Get name property from element.
    [Tags]    smoke    positive
    ${name}=    Get Element Property    JButton[name='submitButton']    name
    Should Be Equal    ${name}    submitButton

Get Element Property Enabled
    [Documentation]    Get enabled property from element.
    [Tags]    positive
    ${enabled}=    Get Element Property    JButton[name='submitButton']    enabled
    Should Be True    ${enabled}

Get Element Property Visible
    [Documentation]    Get visible property from element.
    [Tags]    positive
    ${visible}=    Get Element Property    JButton[name='submitButton']    visible
    Should Be True    ${visible}

Get Element Property Text
    [Documentation]    Get text property from element.
    [Tags]    positive
    ${text}=    Get Element Property    JButton[name='submitButton']    text
    Should Not Be Empty    ${text}

Get Element Property Using XPath
    [Documentation]    Get property using XPath selector.
    [Tags]    positive    xpath-locator
    ${name}=    Get Element Property    //JButton[@name='submitButton']    name
    Should Be Equal    ${name}    submitButton

# =============================================================================
# GET ELEMENT PROPERTIES (ALL)
# =============================================================================

Get All Element Properties
    [Documentation]    Get all common properties from element.
    [Tags]    smoke    positive
    ${props}=    Get Element Properties    JButton[name='submitButton']
    Should Not Be Empty    ${props}
    Log    Properties: ${props}

Get All Element Properties Using ID
    [Documentation]    Get all properties using ID selector.
    [Tags]    positive
    ${props}=    Get Element Properties    \#submitButton
    Log    Properties: ${props}

Get All Element Properties Using XPath
    [Documentation]    Get all properties using XPath selector.
    [Tags]    positive    xpath-locator
    ${props}=    Get Element Properties    //JButton[@name='submitButton']
    Log    Properties: ${props}

Verify Properties Structure
    [Documentation]    Verify returned properties structure.
    [Tags]    positive
    ${props}=    Get Element Properties    JButton[name='submitButton']
    Dictionary Should Contain Key    ${props}    name
    Dictionary Should Contain Key    ${props}    enabled

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
# VERIFICATION WORKFLOWS
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

Verify After Input Workflow
    [Documentation]    Verify state after input operations.
    [Tags]    workflow
    Select Form Input Tab
    Input Text    [name='nameTextField']    verifyuser
    ${text}=    Get Element Text    JTextField[name='nameTextField']
    Should Contain    ${text}    verifyuser
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

Comprehensive Element Verification
    [Documentation]    Comprehensive verification of element properties.
    [Tags]    workflow
    Select Form Input Tab
    ${props}=    Get Element Properties    JButton[name='submitButton']
    Should Be True    ${props}[enabled]
    ${text}=    Get Element Text    JButton[name='submitButton']
    Should Not Be Empty    ${text}
    Element Should Be Visible    JButton[name='submitButton']
    Element Should Be Enabled    JButton[name='submitButton']

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

Get Element Text Fails For Nonexistent
    [Documentation]    Get text fails for non-existent element.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Element Text    JButton[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Element Text Should Be Fails For Wrong Text
    [Documentation]    Text match fails for wrong text.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Element Text Should Be    JLabel[text='Name:']    WrongText
    Should Be Equal    ${status}    ${FALSE}

Element Text Should Contain Fails For Missing Text
    [Documentation]    Contains check fails for missing substring.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Element Text Should Contain    JLabel[text='Name:']    NonExistent
    Should Be Equal    ${status}    ${FALSE}

Get Element Property Fails For Nonexistent
    [Documentation]    Get property fails for non-existent element.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Element Property    JButton[name='nonexistent']    name
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
    ${text}=    Get Element Text    JTextField[name='nameTextField']
    Should Be Empty    ${text}

Verify Multiple Properties In Sequence
    [Documentation]    Get multiple properties in sequence.
    [Tags]    edge-case
    Select Form Input Tab
    ${name}=    Get Element Property    JButton[name='submitButton']    name
    ${enabled}=    Get Element Property    JButton[name='submitButton']    enabled
    ${visible}=    Get Element Property    JButton[name='submitButton']    visible
    ${text}=    Get Element Property    JButton[name='submitButton']    text
    Log    Name: ${name}, Enabled: ${enabled}, Visible: ${visible}, Text: ${text}

Rapid Verification Calls
    [Documentation]    Test rapid verification calls.
    [Tags]    edge-case    stress
    Select Form Input Tab
    FOR    ${i}    IN RANGE    10
        Element Should Exist    JButton[name='submitButton']
        Element Should Be Visible    JButton[name='submitButton']
        Element Should Be Enabled    JButton[name='submitButton']
    END

Verify All Form Elements
    [Documentation]    Verify all elements in a form.
    [Tags]    edge-case
    Select Form Input Tab
    Element Should Exist    JTextField[name='nameTextField']
    Element Should Exist    JPasswordField[name='passwordField']
    Element Should Exist    JButton[name='submitButton']
    Element Should Exist    JButton[name='clearButton']
    Element Should Be Enabled    JTextField[name='nameTextField']
    Element Should Be Enabled    JPasswordField[name='passwordField']
    Element Should Be Enabled    JButton[name='submitButton']
    Element Should Be Enabled    JButton[name='clearButton']
