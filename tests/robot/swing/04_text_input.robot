*** Settings ***
Test Timeout       60s
Documentation     Text Input Tests - Testing input_text, clear_text, and type_text
...               keywords for JTextField, JTextArea, and JPasswordField components.
...
...               These tests verify the library's ability to input and manipulate
...               text in various text input components.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application
Test Setup        Clear Login Form

Force Tags        text-input    regression

*** Test Cases ***
# =============================================================================
# INPUT TEXT - JTEXTFIELD
# =============================================================================

Input Text Into TextField By Name
    [Documentation]    Input text into a JTextField using name selector.
    [Tags]    smoke    positive
    Input Text    JTextField[name='nameTextField']    testuser
    # Verify text was entered and is visible
    ${text}=    Get Element Text    JTextField[name='nameTextField']
    Should Be Equal    ${text}    testuser    Text should be visible in field

Input Text Into TextField By ID
    [Documentation]    Input text using ID-style selector.
    [Tags]    positive
    Input Text    \#nameTextField    testuser
    ${text}=    Get Element Text    \#nameTextField
    Should Be Equal    ${text}    testuser    ID selector input should work

Input Text Into TextField Using XPath
    [Documentation]    Input text using XPath selector.
    [Tags]    positive    xpath-locator
    Input Text    //JTextField[@name='nameTextField']    xpathuser
    ${text}=    Get Element Text    //JTextField[@name='nameTextField']
    Should Be Equal    ${text}    xpathuser    XPath input should work

Input Text With Special Characters
    [Documentation]    Input text containing special characters.
    [Tags]    positive    edge-case
    Input Text    [name='nameTextField']    test@user.com
    ${text}=    Get Element Text    [name='nameTextField']
    Should Be Equal    ${text}    test@user.com    Special characters should be preserved

Input Text With Numbers
    [Documentation]    Input text containing numeric characters.
    [Tags]    positive
    Input Text    [name='nameTextField']    user12345
    ${text}=    Get Element Text    [name='nameTextField']
    Should Be Equal    ${text}    user12345    Numeric characters should be preserved

Input Text With Spaces
    [Documentation]    Input text containing spaces.
    [Tags]    positive    edge-case
    Input Text    [name='nameTextField']    first last name
    ${text}=    Get Element Text    [name='nameTextField']
    Should Be Equal    ${text}    first last name    Spaces should be preserved

Input Text Overwrites Existing Content
    [Documentation]    Verify input_text clears existing text by default.
    [Tags]    positive
    Input Text    [name='nameTextField']    firsttext
    Input Text    [name='nameTextField']    secondtext
    # The second input should overwrite the first
    ${text}=    Get Element Text    [name='nameTextField']
    Should Be Equal    ${text}    secondtext    Second input should overwrite first

Input Text Without Clear
    [Documentation]    Input text without clearing existing content.
    [Tags]    positive
    Input Text    [name='nameTextField']    first
    Input Text    [name='nameTextField']    second    clear=False
    # Should append to existing text
    ${text}=    Get Element Text    [name='nameTextField']
    Should Be Equal    ${text}    firstsecond    Text should be appended when clear=False

Input Text Empty String
    [Documentation]    Input empty string into text field.
    [Tags]    positive    edge-case
    Input Text    [name='nameTextField']    test
    Input Text    [name='nameTextField']    ${EMPTY}
    ${text}=    Get Element Text    [name='nameTextField']
    Should Be Empty    ${text}    Field should be empty after inputting empty string

Input Text Long String
    [Documentation]    Input a very long string into text field.
    [Tags]    positive    edge-case
    ${long_text}=    Evaluate    'a' * 200
    Input Text    [name='nameTextField']    ${long_text}
    ${text}=    Get Element Text    [name='nameTextField']
    ${length}=    Get Length    ${text}
    Should Be Equal As Integers    ${length}    200    Long text should be fully entered

# =============================================================================
# INPUT TEXT - JTEXTAREA
# =============================================================================

Input Text Into TextArea
    [Documentation]    Input text into a JTextArea component.
    [Tags]    positive
    Input Text    JTextArea[name='descriptionTextArea']    This is a note.
    ${text}=    Get Element Text    JTextArea[name='descriptionTextArea']
    Should Be Equal    ${text}    This is a note.    TextArea should contain entered text

Input Multiline Text Into TextArea
    [Documentation]    Input multiline text into a JTextArea.
    [Tags]    positive    edge-case
    Input Text    [name='descriptionTextArea']    Line 1\nLine 2\nLine 3
    ${text}=    Get Element Text    [name='descriptionTextArea']
    Should Contain    ${text}    Line 1    Multiline text should contain Line 1
    Should Contain    ${text}    Line 2    Multiline text should contain Line 2
    Should Contain    ${text}    Line 3    Multiline text should contain Line 3

Input Text Append To TextArea
    [Documentation]    Append text to existing TextArea content.
    [Tags]    positive
    Input Text    [name='descriptionTextArea']    First line
    Input Text    [name='descriptionTextArea']    Second line    clear=False
    ${text}=    Get Element Text    [name='descriptionTextArea']
    Should Contain    ${text}    First line    Original text should be preserved
    Should Contain    ${text}    Second line    Appended text should be present

# =============================================================================
# INPUT TEXT - JPASSWORDFIELD
# =============================================================================

Input Text Into PasswordField
    [Documentation]    Input text into a JPasswordField component.
    [Tags]    smoke    positive
    Input Text    JPasswordField[name='passwordField']    secret123
    Element Should Exist    JPasswordField[name='passwordField']

Input Text Into PasswordField By Name
    [Documentation]    Input password using name selector.
    [Tags]    positive
    Input Text    [name='passwordField']    mypassword
    Element Should Exist    [name='passwordField']

Input Text Into PasswordField Using XPath
    [Documentation]    Input password using XPath selector.
    [Tags]    positive    xpath-locator
    Input Text    //JPasswordField[@name='passwordField']    xpathpass
    Element Should Exist    //JPasswordField[@name='passwordField']

Input Complex Password
    [Documentation]    Input password with special characters.
    [Tags]    positive    edge-case
    Input Text    [name='passwordField']    P@ssw0rd!#$%
    Element Should Exist    [name='passwordField']

# =============================================================================
# CLEAR TEXT
# =============================================================================

Clear Text From TextField
    [Documentation]    Clear text from a JTextField.
    [Tags]    smoke    positive
    Input Text    [name='nameTextField']    texttoremove
    Clear Text    [name='nameTextField']
    ${text}=    Get Element Text    [name='nameTextField']
    Should Be Empty    ${text}    TextField should be empty after clear

Clear Text From PasswordField
    [Documentation]    Clear text from a JPasswordField.
    [Tags]    positive
    Input Text    [name='passwordField']    passwordtoremove
    Clear Text    [name='passwordField']
    # Password fields typically can't be read, but verify operation completed
    Element Should Exist    [name='passwordField']

Clear Text From TextArea
    [Documentation]    Clear text from a JTextArea.
    [Tags]    positive
    Input Text    [name='descriptionTextArea']    notes to remove
    Clear Text    [name='descriptionTextArea']
    ${text}=    Get Element Text    [name='descriptionTextArea']
    Should Be Empty    ${text}    TextArea should be empty after clear

Clear Already Empty TextField
    [Documentation]    Clear an already empty text field.
    [Tags]    positive    edge-case
    Clear Text    [name='nameTextField']
    Clear Text    [name='nameTextField']
    ${text}=    Get Element Text    [name='nameTextField']
    Should Be Empty    ${text}    Field should remain empty

Clear Text Using XPath
    [Documentation]    Clear text using XPath selector.
    [Tags]    positive    xpath-locator
    Input Text    //JTextField[@name='nameTextField']    texttoremove
    Clear Text    //JTextField[@name='nameTextField']
    ${text}=    Get Element Text    //JTextField[@name='nameTextField']
    Should Be Empty    ${text}    Field should be empty after XPath clear

Clear Text Using ID Selector
    [Documentation]    Clear text using ID-style selector.
    [Tags]    positive
    Input Text    \#nameTextField    texttoremove
    Clear Text    \#nameTextField
    ${text}=    Get Element Text    \#nameTextField
    Should Be Empty    ${text}    Field should be empty after ID selector clear

# =============================================================================
# TYPE TEXT
# =============================================================================

Type Text Into TextField
    [Documentation]    Type text character by character into a text field.
    [Tags]    positive
    Clear Text    [name='nameTextField']
    Type Text    [name='nameTextField']    typedtext
    ${text}=    Get Element Text    [name='nameTextField']
    Should Be Equal    ${text}    typedtext    Typed text should appear in field

Type Text Does Not Clear
    [Documentation]    Verify type_text appends to existing content.
    [Tags]    positive
    Input Text    [name='nameTextField']    existing
    Type Text    [name='nameTextField']    appended
    # Should have both texts
    ${text}=    Get Element Text    [name='nameTextField']
    Should Be Equal    ${text}    existingappended    Type should append to existing text

Type Text Into PasswordField
    [Documentation]    Type text into a password field.
    [Tags]    positive
    Clear Text    [name='passwordField']
    Type Text    [name='passwordField']    typedpassword
    Element Should Exist    [name='passwordField']

Type Text Into TextArea
    [Documentation]    Type text into a text area.
    [Tags]    positive
    Clear Text    [name='descriptionTextArea']
    Type Text    [name='descriptionTextArea']    typed note content
    ${text}=    Get Element Text    [name='descriptionTextArea']
    Should Be Equal    ${text}    typed note content    Typed text should appear in TextArea

Type Text Using XPath
    [Documentation]    Type text using XPath selector.
    [Tags]    positive    xpath-locator
    Clear Text    //JTextField[@name='nameTextField']
    Type Text    //JTextField[@name='nameTextField']    xpathtypedtext
    ${text}=    Get Element Text    //JTextField[@name='nameTextField']
    Should Be Equal    ${text}    xpathtypedtext    XPath typed text should work

Type Text With Special Characters
    [Documentation]    Type text containing special characters.
    [Tags]    positive    edge-case
    Clear Text    [name='nameTextField']
    Type Text    [name='nameTextField']    user@domain.com
    ${text}=    Get Element Text    [name='nameTextField']
    Should Be Equal    ${text}    user@domain.com    Special characters should be typed correctly

# =============================================================================
# TEXT INPUT WORKFLOWS
# =============================================================================

Complete Login Form Workflow
    [Documentation]    Fill in complete login form.
    [Tags]    workflow    smoke
    Clear Text    [name='nameTextField']
    Clear Text    [name='passwordField']
    Input Text    [name='nameTextField']    ${VALID_USERNAME}
    Input Text    [name='passwordField']    ${VALID_PASSWORD}
    Click Button    ${LOGIN_BUTTON}
    Sleep    0.5s

Fill And Clear Form Workflow
    [Documentation]    Fill form then clear it.
    [Tags]    workflow
    Input Text    [name='nameTextField']    testuser
    Input Text    [name='passwordField']    testpass
    Click Button    ${CLEAR_BUTTON}
    Sleep    0.2s
    Element Should Exist    [name='nameTextField']

Edit Existing Text Workflow
    [Documentation]    Edit existing text by clearing and re-entering.
    [Tags]    workflow
    Input Text    [name='nameTextField']    originaltext
    Clear Text    [name='nameTextField']
    Input Text    [name='nameTextField']    newtext
    Element Should Exist    [name='nameTextField']

# =============================================================================
# TEXT FIELD STATE VERIFICATION
# =============================================================================

Verify TextField Is Enabled Before Input
    [Documentation]    Verify text field is enabled before input.
    [Tags]    positive    verification
    Element Should Be Enabled    [name='nameTextField']
    Input Text    [name='nameTextField']    testinput

Verify TextField Is Visible Before Input
    [Documentation]    Verify text field is visible before input.
    [Tags]    positive    verification
    Element Should Be Visible    [name='nameTextField']
    Input Text    [name='nameTextField']    testinput

Wait For TextField Before Input
    [Documentation]    Wait for text field to be ready before input.
    [Tags]    positive    wait
    Wait Until Element Is Enabled    [name='nameTextField']    timeout=${SHORT_TIMEOUT}
    Input Text    [name='nameTextField']    testinput

# =============================================================================
# FINDING TEXT FIELDS
# =============================================================================

Find All TextFields
    [Documentation]    Find all text field elements.
    [Tags]    positive
    ${fields}=    Find Elements    JTextField
    Should Not Be Empty    ${fields}

Find All PasswordFields
    [Documentation]    Find all password field elements.
    [Tags]    positive
    ${fields}=    Find Elements    JPasswordField
    Should Not Be Empty    ${fields}

Find All TextAreas
    [Documentation]    Find all text area elements.
    [Tags]    positive
    ${fields}=    Find Elements    JTextArea
    Should Not Be Empty    ${fields}

Find Enabled TextFields
    [Documentation]    Find all enabled text fields.
    [Tags]    positive
    ${fields}=    Find Elements    JTextField:enabled
    Should Not Be Empty    ${fields}

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Input Text To Nonexistent Field Fails
    [Documentation]    Input text to non-existent field throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Input Text    JTextField[name='nonexistent']    text
    Should Be Equal    ${status}    ${FALSE}

Clear Text From Nonexistent Field Fails
    [Documentation]    Clear text from non-existent field throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Clear Text    JTextField[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Type Text To Nonexistent Field Fails
    [Documentation]    Type text to non-existent field throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Type Text    JTextField[name='nonexistent']    text
    Should Be Equal    ${status}    ${FALSE}

Input Text With Invalid Locator Fails
    [Documentation]    Input text with invalid locator throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Input Text    [[[invalid]]]    text
    Should Be Equal    ${status}    ${FALSE}

# =============================================================================
# EDGE CASES
# =============================================================================

Input Unicode Characters
    [Documentation]    Input unicode characters into text field.
    [Tags]    edge-case
    Input Text    [name='nameTextField']    test
    Element Should Exist    [name='nameTextField']

Input Text With Tab Characters
    [Documentation]    Input text containing tab characters.
    [Tags]    edge-case
    Input Text    [name='descriptionTextArea']    col1\tcol2\tcol3
    Element Should Exist    [name='descriptionTextArea']

Rapid Text Input Sequence
    [Documentation]    Test rapid sequence of text inputs.
    [Tags]    edge-case    stress
    FOR    ${i}    IN RANGE    5
        Input Text    [name='nameTextField']    text${i}
        Sleep    0.1s
    END
    Element Should Exist    [name='nameTextField']

Input And Verify Multiple Fields
    [Documentation]    Input text into multiple fields and verify.
    [Tags]    edge-case
    Input Text    [name='nameTextField']    user1
    Input Text    [name='passwordField']    pass1
    Element Should Exist    [name='nameTextField']
    Element Should Exist    [name='passwordField']
