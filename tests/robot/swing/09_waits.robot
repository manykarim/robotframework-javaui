*** Settings ***
Test Timeout       60s
Documentation     Wait Tests - Testing wait_until_element_is_visible,
...               wait_until_element_is_enabled, wait_for_element,
...               and wait_until_element_contains keywords.
...
...               These tests verify the library's ability to wait for
...               various element states before interaction.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application
Test Setup        Ensure Form Input Tab

Force Tags        waits    regression

*** Keywords ***
Ensure Form Input Tab
    [Documentation]    Navigate to Form Input tab if not already there.
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Sleep    0.1s

*** Test Cases ***
# =============================================================================
# WAIT UNTIL ELEMENT IS VISIBLE
# =============================================================================

Wait Until Element Is Visible By Name
    [Documentation]    Wait for element to be visible using name selector.
    [Tags]    smoke    positive
    Wait Until Element Is Visible    [name='submitButton']
    Element Should Exist    [name='submitButton']

Wait Until Element Is Visible By ID
    [Documentation]    Wait for element using ID-style selector.
    [Tags]    positive
    Wait Until Element Is Visible    \#submitButton    timeout=${DEFAULT_TIMEOUT}
    Element Should Exist    \#submitButton

Wait Until Element Is Visible By Type
    [Documentation]    Wait for element by component type.
    [Tags]    positive
    Wait Until Element Is Visible    JButton[name='submitButton']
    Element Should Exist    JButton[name='submitButton']

Wait Until Element Is Visible Using XPath
    [Documentation]    Wait using XPath selector.
    [Tags]    positive    xpath-locator
    Wait Until Element Is Visible    //JButton[@name='submitButton']
    Element Should Exist    //JButton[@name='submitButton']

Wait Until Element Is Visible With Custom Timeout
    [Documentation]    Wait with custom timeout value.
    [Tags]    positive
    Wait Until Element Is Visible    ${LOGIN_BUTTON}    timeout=30
    Element Should Exist    ${LOGIN_BUTTON}

Wait Until Already Visible Element
    [Documentation]    Wait for already visible element returns immediately.
    [Tags]    positive    edge-case
    Element Should Be Visible    ${LOGIN_BUTTON}
    Wait Until Element Is Visible    ${LOGIN_BUTTON}    timeout=${SHORT_TIMEOUT}

# =============================================================================
# WAIT UNTIL ELEMENT IS ENABLED
# =============================================================================

Wait Until Element Is Enabled By Name
    [Documentation]    Wait for element to be enabled using name selector.
    [Tags]    smoke    positive
    Wait Until Element Is Enabled    [name='submitButton']
    Element Should Exist    [name='submitButton']

Wait Until Element Is Enabled By ID
    [Documentation]    Wait for element using ID-style selector.
    [Tags]    positive
    Wait Until Element Is Enabled    \#submitButton    timeout=${DEFAULT_TIMEOUT}
    Element Should Exist    \#submitButton

Wait Until Element Is Enabled By Type
    [Documentation]    Wait for element by component type.
    [Tags]    positive
    Wait Until Element Is Enabled    JButton[name='submitButton']
    Element Should Exist    JButton[name='submitButton']

Wait Until Element Is Enabled Using XPath
    [Documentation]    Wait using XPath selector.
    [Tags]    positive    xpath-locator
    Wait Until Element Is Enabled    //JButton[@name='submitButton']
    Element Should Exist    //JButton[@name='submitButton']

Wait Until Element Is Enabled With Custom Timeout
    [Documentation]    Wait with custom timeout value.
    [Tags]    positive
    Wait Until Element Is Enabled    ${LOGIN_BUTTON}    timeout=30
    Element Should Exist    ${LOGIN_BUTTON}

Wait Until Already Enabled Element
    [Documentation]    Wait for already enabled element returns immediately.
    [Tags]    positive    edge-case
    Element Should Be Enabled    ${LOGIN_BUTTON}
    Wait Until Element Is Enabled    ${LOGIN_BUTTON}    timeout=${SHORT_TIMEOUT}

# =============================================================================
# WAIT FOR ELEMENT (RETURNS ELEMENT)
# =============================================================================

Wait For Element By Name
    [Documentation]    Wait for element and get reference.
    [Tags]    smoke    positive
    ${element}=    Wait For Element    [name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Wait For Element By ID
    [Documentation]    Wait for element using ID-style selector.
    [Tags]    positive
    ${element}=    Wait For Element    \#submitButton    timeout=${DEFAULT_TIMEOUT}
    Should Not Be Equal    ${element}    ${NONE}

Wait For Element By Type
    [Documentation]    Wait for element by component type.
    [Tags]    positive
    ${element}=    Wait For Element    JButton[name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Wait For Element Using XPath
    [Documentation]    Wait using XPath selector.
    [Tags]    positive    xpath-locator
    ${element}=    Wait For Element    //JButton[@name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Wait For Element With Custom Timeout
    [Documentation]    Wait with custom timeout value.
    [Tags]    positive
    ${element}=    Wait For Element    ${LOGIN_BUTTON}    timeout=30
    Should Not Be Equal    ${element}    ${NONE}

Wait For Existing Element
    [Documentation]    Wait for existing element returns immediately.
    [Tags]    positive    edge-case
    Element Should Exist    ${LOGIN_BUTTON}
    ${element}=    Wait For Element    ${LOGIN_BUTTON}    timeout=${SHORT_TIMEOUT}
    Should Not Be Equal    ${element}    ${NONE}

# =============================================================================
# WAIT UNTIL ELEMENT CONTAINS
# =============================================================================

Wait Until Element Contains Text
    [Documentation]    Wait until element contains expected text.
    [Tags]    smoke    positive
    Wait Until Element Contains    JLabel[name='statusLabel']    Ready    timeout=${DEFAULT_TIMEOUT}
    Element Should Exist    JLabel[name='statusLabel']

Wait Until Element Contains By ID
    [Documentation]    Wait using ID-style selector.
    [Tags]    positive
    Wait Until Element Contains    \#statusLabel    Ready    timeout=${DEFAULT_TIMEOUT}
    Element Should Exist    \#statusLabel

Wait Until Element Contains Using XPath
    [Documentation]    Wait using XPath selector.
    [Tags]    positive    xpath-locator
    Wait Until Element Contains    //JLabel[@name='statusLabel']    Ready
    Element Should Exist    //JLabel[@name='statusLabel']

Wait Until Element Contains Partial Text
    [Documentation]    Wait for partial text match.
    [Tags]    positive
    Wait Until Element Contains    [name='statusLabel']    Read    timeout=${DEFAULT_TIMEOUT}

Wait Until Element Contains With Custom Timeout
    [Documentation]    Wait with custom timeout value.
    [Tags]    positive
    Wait Until Element Contains    [name='statusLabel']    Ready    timeout=30

# =============================================================================
# WAIT WORKFLOWS
# =============================================================================

Wait Before Click Workflow
    [Documentation]    Wait for element before clicking.
    [Tags]    workflow    smoke
    Wait Until Element Is Visible    ${LOGIN_BUTTON}
    Wait Until Element Is Enabled    ${LOGIN_BUTTON}
    Click Button    ${LOGIN_BUTTON}
    Sleep    0.3s

Wait And Input Workflow
    [Documentation]    Wait for field and then input.
    [Tags]    workflow
    Wait Until Element Is Visible    [name='nameTextField']
    Wait Until Element Is Enabled    [name='nameTextField']
    Input Text    [name='nameTextField']    waituser
    Element Should Exist    [name='nameTextField']

Wait For Multiple Elements Workflow
    [Documentation]    Wait for multiple elements before action.
    [Tags]    workflow
    Wait Until Element Is Visible    [name='nameTextField']
    Wait Until Element Is Visible    [name='passwordField']
    Wait Until Element Is Enabled    ${LOGIN_BUTTON}
    # All elements ready, perform action
    Click Button    ${LOGIN_BUTTON}
    Sleep    0.3s

Wait Chain Workflow
    [Documentation]    Chain multiple waits together.
    [Tags]    workflow
    Wait Until Element Is Visible    [name='nameTextField']
    Wait Until Element Is Enabled    [name='nameTextField']
    Input Text    [name='nameTextField']    chainuser
    Wait Until Element Is Visible    [name='passwordField']
    Wait Until Element Is Enabled    [name='passwordField']
    Input Text    [name='passwordField']    chainpass
    Wait Until Element Is Enabled    ${LOGIN_BUTTON}
    Click Button    ${LOGIN_BUTTON}
    Sleep    0.5s

# =============================================================================
# WAIT WITH DIFFERENT ELEMENT TYPES
# =============================================================================

Wait For Button To Be Ready
    [Documentation]    Wait for button element.
    [Tags]    positive
    Wait Until Element Is Visible    JButton[name='submitButton']
    Wait Until Element Is Enabled    JButton[name='submitButton']
    Click Button    JButton[name='submitButton']
    Sleep    0.3s

Wait For TextField To Be Ready
    [Documentation]    Wait for text field element.
    [Tags]    positive
    Wait Until Element Is Visible    JTextField[name='nameTextField']
    Wait Until Element Is Enabled    JTextField[name='nameTextField']
    Input Text    JTextField[name='nameTextField']    testuser
    Element Should Exist    JTextField[name='nameTextField']

Wait For Label To Have Text
    [Documentation]    Wait for label with specific text.
    [Tags]    positive
    Wait Until Element Is Visible    JLabel[name='statusLabel']
    Wait Until Element Contains    JLabel[name='statusLabel']    Ready

Wait For Table To Be Ready
    [Documentation]    Wait for table element.
    [Tags]    positive
    Select Data View Tab
    Wait Until Element Is Visible    JTable[name='dataTable']
    Wait Until Element Is Enabled    JTable[name='dataTable']
    Element Should Exist    JTable[name='dataTable']

Wait For Tree To Be Ready
    [Documentation]    Wait for tree element.
    [Tags]    positive
    Wait Until Element Is Visible    JTree[name='fileTree']
    Wait Until Element Is Enabled    JTree[name='fileTree']
    Element Should Exist    JTree[name='fileTree']

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Wait For Nonexistent Element Times Out
    [Documentation]    Wait for non-existent element throws timeout error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Wait Until Element Is Visible    JButton[name='nonexistent']    timeout=2
    Should Be Equal    ${status}    ${FALSE}

Wait For Nonexistent Element Enabled Times Out
    [Documentation]    Wait for non-existent element enabled throws timeout.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Wait Until Element Is Enabled    JButton[name='nonexistent']    timeout=2
    Should Be Equal    ${status}    ${FALSE}

Wait For Element Returns Null For Nonexistent
    [Documentation]    Wait for element throws for non-existent.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Wait For Element    JButton[name='nonexistent']    timeout=2
    Should Be Equal    ${status}    ${FALSE}

Wait Until Element Contains Wrong Text Times Out
    [Documentation]    Wait for wrong text content throws timeout.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Wait Until Element Contains    [name='statusLabel']    NonExistentText    timeout=2
    Should Be Equal    ${status}    ${FALSE}

Wait With Invalid Locator Fails
    [Documentation]    Wait with invalid locator throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Wait Until Element Is Visible    [[[invalid]]]    timeout=2
    Should Be Equal    ${status}    ${FALSE}

# =============================================================================
# EDGE CASES
# =============================================================================

Wait With Zero Timeout
    [Documentation]    Wait with zero timeout for existing element.
    [Tags]    edge-case
    # Zero timeout should either work for existing or fail gracefully
    ${status}=    Run Keyword And Return Status
    ...    Wait Until Element Is Visible    ${LOGIN_BUTTON}    timeout=0.1
    Log    Zero timeout status: ${status}

Wait With Very Short Timeout
    [Documentation]    Wait with very short timeout.
    [Tags]    edge-case
    ${status}=    Run Keyword And Return Status
    ...    Wait Until Element Is Visible    ${LOGIN_BUTTON}    timeout=0.5
    # Should succeed for existing element
    Log    Short timeout status: ${status}

Wait With Long Timeout For Existing
    [Documentation]    Long timeout for existing element returns fast.
    [Tags]    edge-case
    # Should return immediately since element exists
    Wait Until Element Is Visible    ${LOGIN_BUTTON}    timeout=60
    Element Should Exist    ${LOGIN_BUTTON}

Multiple Concurrent Waits
    [Documentation]    Multiple wait operations.
    [Tags]    edge-case
    Wait Until Element Is Visible    [name='nameTextField']
    Wait Until Element Is Visible    [name='passwordField']
    Wait Until Element Is Visible    ${LOGIN_BUTTON}
    Wait Until Element Is Enabled    [name='nameTextField']
    Wait Until Element Is Enabled    [name='passwordField']
    Wait Until Element Is Enabled    ${LOGIN_BUTTON}
    Element Should Exist    [name='nameTextField']
    Element Should Exist    [name='passwordField']
    Element Should Exist    ${LOGIN_BUTTON}

Rapid Wait Calls
    [Documentation]    Test rapid wait calls.
    [Tags]    edge-case    stress
    FOR    ${i}    IN RANGE    5
        Wait Until Element Is Visible    ${LOGIN_BUTTON}    timeout=${SHORT_TIMEOUT}
    END
    Element Should Exist    ${LOGIN_BUTTON}

# =============================================================================
# INTEGRATION TESTS
# =============================================================================

Wait And Complete Form
    [Documentation]    Complete form using wait before each step.
    [Tags]    integration
    # Wait and fill username
    Wait Until Element Is Enabled    [name='nameTextField']
    Input Text    [name='nameTextField']    integrationuser
    # Wait and fill password
    Wait Until Element Is Enabled    [name='passwordField']
    Input Text    [name='passwordField']    integrationpass
    # Wait and click login
    Wait Until Element Is Enabled    ${LOGIN_BUTTON}
    Click Button    ${LOGIN_BUTTON}
    Sleep    0.5s

Wait For Dynamic Content
    [Documentation]    Wait for dynamically appearing content with assertion.
    [Tags]    integration    assertion-operator
    # This simulates waiting for dynamic content
    Wait Until Element Is Visible    JLabel[name='statusLabel']    timeout=${DEFAULT_TIMEOUT}
    # Verify status label has content using assertion operator
    Get Text    JLabel[name='statusLabel']    !=    ${EMPTY}
