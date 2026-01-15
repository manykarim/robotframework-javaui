*** Settings ***
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
    Input Text    JTextField[name='username']    testuser
    # Verify text was entered and is visible
    ${text}=    Get Element Text    JTextField[name='username']
    Should Be Equal    ${text}    testuser    Text should be visible in field

Input Text Into TextField By ID
    [Documentation]    Input text using ID-style selector.
    [Tags]    positive
    Input Text    \#username    testuser
    Element Should Exist    \#username

Input Text Into TextField Using XPath
    [Documentation]    Input text using XPath selector.
    [Tags]    positive    xpath-locator
    Input Text    //JTextField[@name='username']    xpathuser
    Element Should Exist    //JTextField[@name='username']

Input Text With Special Characters
    [Documentation]    Input text containing special characters.
    [Tags]    positive    edge-case
    Input Text    [name='username']    test@user.com
    Element Should Exist    [name='username']

Input Text With Numbers
    [Documentation]    Input text containing numeric characters.
    [Tags]    positive
    Input Text    [name='username']    user12345
    Element Should Exist    [name='username']

Input Text With Spaces
    [Documentation]    Input text containing spaces.
    [Tags]    positive    edge-case
    Input Text    [name='username']    first last name
    Element Should Exist    [name='username']

Input Text Overwrites Existing Content
    [Documentation]    Verify input_text clears existing text by default.
    [Tags]    positive
    Input Text    [name='username']    firsttext
    Input Text    [name='username']    secondtext
    # The second input should overwrite the first
    Element Should Exist    [name='username']

Input Text Without Clear
    [Documentation]    Input text without clearing existing content.
    [Tags]    positive
    Input Text    [name='username']    first
    Input Text    [name='username']    second    clear=False
    # Should append to existing text
    Element Should Exist    [name='username']

Input Text Empty String
    [Documentation]    Input empty string into text field.
    [Tags]    positive    edge-case
    Input Text    [name='username']    test
    Input Text    [name='username']    ${EMPTY}
    Element Should Exist    [name='username']

Input Text Long String
    [Documentation]    Input a very long string into text field.
    [Tags]    positive    edge-case
    ${long_text}=    Evaluate    'a' * 200
    Input Text    [name='username']    ${long_text}
    Element Should Exist    [name='username']

# =============================================================================
# INPUT TEXT - JTEXTAREA
# =============================================================================

Input Text Into TextArea
    [Documentation]    Input text into a JTextArea component.
    [Tags]    positive
    Input Text    JTextArea[name='notesArea']    This is a note.
    Element Should Exist    JTextArea[name='notesArea']

Input Multiline Text Into TextArea
    [Documentation]    Input multiline text into a JTextArea.
    [Tags]    positive    edge-case
    Input Text    [name='notesArea']    Line 1\nLine 2\nLine 3
    Element Should Exist    [name='notesArea']

Input Text Append To TextArea
    [Documentation]    Append text to existing TextArea content.
    [Tags]    positive
    Input Text    [name='notesArea']    First line
    Input Text    [name='notesArea']    Second line    clear=False
    Element Should Exist    [name='notesArea']

# =============================================================================
# INPUT TEXT - JPASSWORDFIELD
# =============================================================================

Input Text Into PasswordField
    [Documentation]    Input text into a JPasswordField component.
    [Tags]    smoke    positive
    Input Text    JPasswordField[name='password']    secret123
    Element Should Exist    JPasswordField[name='password']

Input Text Into PasswordField By Name
    [Documentation]    Input password using name selector.
    [Tags]    positive
    Input Text    [name='password']    mypassword
    Element Should Exist    [name='password']

Input Text Into PasswordField Using XPath
    [Documentation]    Input password using XPath selector.
    [Tags]    positive    xpath-locator
    Input Text    //JPasswordField[@name='password']    xpathpass
    Element Should Exist    //JPasswordField[@name='password']

Input Complex Password
    [Documentation]    Input password with special characters.
    [Tags]    positive    edge-case
    Input Text    [name='password']    P@ssw0rd!#$%
    Element Should Exist    [name='password']

# =============================================================================
# CLEAR TEXT
# =============================================================================

Clear Text From TextField
    [Documentation]    Clear text from a JTextField.
    [Tags]    smoke    positive
    Input Text    [name='username']    texttoremove
    Clear Text    [name='username']
    Element Should Exist    [name='username']

Clear Text From PasswordField
    [Documentation]    Clear text from a JPasswordField.
    [Tags]    positive
    Input Text    [name='password']    passwordtoremove
    Clear Text    [name='password']
    Element Should Exist    [name='password']

Clear Text From TextArea
    [Documentation]    Clear text from a JTextArea.
    [Tags]    positive
    Input Text    [name='notesArea']    notes to remove
    Clear Text    [name='notesArea']
    Element Should Exist    [name='notesArea']

Clear Already Empty TextField
    [Documentation]    Clear an already empty text field.
    [Tags]    positive    edge-case
    Clear Text    [name='username']
    Clear Text    [name='username']
    Element Should Exist    [name='username']

Clear Text Using XPath
    [Documentation]    Clear text using XPath selector.
    [Tags]    positive    xpath-locator
    Input Text    //JTextField[@name='username']    texttoremove
    Clear Text    //JTextField[@name='username']
    Element Should Exist    //JTextField[@name='username']

Clear Text Using ID Selector
    [Documentation]    Clear text using ID-style selector.
    [Tags]    positive
    Input Text    \#username    texttoremove
    Clear Text    \#username
    Element Should Exist    \#username

# =============================================================================
# TYPE TEXT
# =============================================================================

Type Text Into TextField
    [Documentation]    Type text character by character into a text field.
    [Tags]    positive
    Clear Text    [name='username']
    Type Text    [name='username']    typedtext
    Element Should Exist    [name='username']

Type Text Does Not Clear
    [Documentation]    Verify type_text appends to existing content.
    [Tags]    positive
    Input Text    [name='username']    existing
    Type Text    [name='username']    appended
    # Should have both texts
    Element Should Exist    [name='username']

Type Text Into PasswordField
    [Documentation]    Type text into a password field.
    [Tags]    positive
    Clear Text    [name='password']
    Type Text    [name='password']    typedpassword
    Element Should Exist    [name='password']

Type Text Into TextArea
    [Documentation]    Type text into a text area.
    [Tags]    positive
    Clear Text    [name='notesArea']
    Type Text    [name='notesArea']    typed note content
    Element Should Exist    [name='notesArea']

Type Text Using XPath
    [Documentation]    Type text using XPath selector.
    [Tags]    positive    xpath-locator
    Clear Text    //JTextField[@name='username']
    Type Text    //JTextField[@name='username']    xpathtypedtext
    Element Should Exist    //JTextField[@name='username']

Type Text With Special Characters
    [Documentation]    Type text containing special characters.
    [Tags]    positive    edge-case
    Clear Text    [name='username']
    Type Text    [name='username']    user@domain.com
    Element Should Exist    [name='username']

# =============================================================================
# TEXT INPUT WORKFLOWS
# =============================================================================

Complete Login Form Workflow
    [Documentation]    Fill in complete login form.
    [Tags]    workflow    smoke
    Clear Text    [name='username']
    Clear Text    [name='password']
    Input Text    [name='username']    ${VALID_USERNAME}
    Input Text    [name='password']    ${VALID_PASSWORD}
    Click Button    ${LOGIN_BUTTON}
    Sleep    0.5s

Fill And Clear Form Workflow
    [Documentation]    Fill form then clear it.
    [Tags]    workflow
    Input Text    [name='username']    testuser
    Input Text    [name='password']    testpass
    Click Button    ${CLEAR_BUTTON}
    Sleep    0.2s
    Element Should Exist    [name='username']

Edit Existing Text Workflow
    [Documentation]    Edit existing text by clearing and re-entering.
    [Tags]    workflow
    Input Text    [name='username']    originaltext
    Clear Text    [name='username']
    Input Text    [name='username']    newtext
    Element Should Exist    [name='username']

# =============================================================================
# TEXT FIELD STATE VERIFICATION
# =============================================================================

Verify TextField Is Enabled Before Input
    [Documentation]    Verify text field is enabled before input.
    [Tags]    positive    verification
    Element Should Be Enabled    [name='username']
    Input Text    [name='username']    testinput

Verify TextField Is Visible Before Input
    [Documentation]    Verify text field is visible before input.
    [Tags]    positive    verification
    Element Should Be Visible    [name='username']
    Input Text    [name='username']    testinput

Wait For TextField Before Input
    [Documentation]    Wait for text field to be ready before input.
    [Tags]    positive    wait
    Wait Until Element Is Enabled    [name='username']    timeout=${SHORT_TIMEOUT}
    Input Text    [name='username']    testinput

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
    Input Text    [name='username']    test
    Element Should Exist    [name='username']

Input Text With Tab Characters
    [Documentation]    Input text containing tab characters.
    [Tags]    edge-case
    Input Text    [name='notesArea']    col1\tcol2\tcol3
    Element Should Exist    [name='notesArea']

Rapid Text Input Sequence
    [Documentation]    Test rapid sequence of text inputs.
    [Tags]    edge-case    stress
    FOR    ${i}    IN RANGE    5
        Input Text    [name='username']    text${i}
        Sleep    0.1s
    END
    Element Should Exist    [name='username']

Input And Verify Multiple Fields
    [Documentation]    Input text into multiple fields and verify.
    [Tags]    edge-case
    Input Text    [name='username']    user1
    Input Text    [name='password']    pass1
    Element Should Exist    [name='username']
    Element Should Exist    [name='password']
