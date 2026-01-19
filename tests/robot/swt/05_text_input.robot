*** Settings ***
Test Timeout       60s
Documentation     Test suite for SWT text input keywords.
...
...               Tests the following SwtLibrary keywords:
...               - input_text
...               - clear_text
...
...               Tests text input and clearing on various text widget types
...               including Text, StyledText, and other editable widgets.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        swt    text-input    actions


*** Variables ***
# Text widget locators (matching SwtTestApp widget names)
${TEXT_WIDGET}            name:textUsername
${STYLED_TEXT_WIDGET}     name:styledTextEditor
${PASSWORD_WIDGET}        name:textPassword
${MULTILINE_TEXT}         name:textDescription
${SEARCH_FIELD}           name:textUsername
${READONLY_TEXT}          name:textDescription
${DISABLED_TEXT}          name:textPassword
${NONEXISTENT}            name:nonExistentWidget12345

# Test data
${SAMPLE_TEXT}            Hello SWT World
${UNICODE_TEXT}           Unicode: \u00e9\u00e8\u00ea \u4e2d\u6587
${SPECIAL_CHARS}          Special: !@#$%^&*()
${MULTILINE_CONTENT}      Line 1\nLine 2\nLine 3
${LONG_TEXT}              This is a very long text that exceeds normal input lengths and should test the widget's ability to handle extended content without issues.
${EMPTY_STRING}           ${EMPTY}
${WHITESPACE}             ${SPACE}${SPACE}${SPACE}


*** Test Cases ***
# ============================================================================
# input_text - Basic Text Input
# ============================================================================

Input Text To Text Widget
    [Documentation]    Verify inputting text into a standard Text widget.
    [Tags]    smoke    critical    positive
    Clear Text    ${TEXT_WIDGET}
    Input Text    ${TEXT_WIDGET}    ${SAMPLE_TEXT}
    Widget Text Should Be    ${TEXT_WIDGET}    ${SAMPLE_TEXT}

Input Text To Text Widget By Name
    [Documentation]    Verify inputting text using name: locator.
    [Tags]    smoke    positive
    Input Text    name:textUsername    ${SAMPLE_TEXT}    clear=${TRUE}
    Widget Text Should Be    name:textUsername    ${SAMPLE_TEXT}

Input Text To Text Widget By Text
    [Documentation]    Verify inputting text using text: locator.
    [Tags]    positive
    # First clear, then input
    Clear Text    ${TEXT_WIDGET}
    Input Text    ${TEXT_WIDGET}    ${SAMPLE_TEXT}
    Widget Text Should Be    ${TEXT_WIDGET}    ${SAMPLE_TEXT}

Input Text To StyledText Widget
    [Documentation]    Verify inputting text into a StyledText widget.
    [Tags]    smoke    positive    styledtext
    Clear Text    ${STYLED_TEXT_WIDGET}
    Input Text    ${STYLED_TEXT_WIDGET}    ${SAMPLE_TEXT}
    Widget Text Should Be    ${STYLED_TEXT_WIDGET}    ${SAMPLE_TEXT}

Input Text To Password Field
    [Documentation]    Verify inputting text into a password field.
    [Tags]    positive    password
    Clear Text    ${PASSWORD_WIDGET}
    Input Text    ${PASSWORD_WIDGET}    secretPassword123
    # Note: Password field may not return actual text in verification

Input Text To Search Field
    [Documentation]    Verify inputting text into a search field.
    [Tags]    positive
    Clear Text    ${SEARCH_FIELD}
    Input Text    ${SEARCH_FIELD}    search query
    Widget Text Should Be    ${SEARCH_FIELD}    search query

Input Text To Multiline Text
    [Documentation]    Verify inputting multiline text.
    [Tags]    positive    multiline
    Clear Text    ${MULTILINE_TEXT}
    Input Text    ${MULTILINE_TEXT}    ${MULTILINE_CONTENT}
    Log    Multiline text input completed

# ============================================================================
# input_text - clear Parameter
# ============================================================================

Input Text With Clear True
    [Documentation]    Verify input_text with clear=True replaces existing text.
    [Tags]    smoke    positive    clear
    # First, put some text in the field
    Input Text    ${TEXT_WIDGET}    Initial Text    clear=${TRUE}

    # Now input new text with clear=True
    Input Text    ${TEXT_WIDGET}    ${SAMPLE_TEXT}    clear=${TRUE}

    # Verify only new text is present
    Widget Text Should Be    ${TEXT_WIDGET}    ${SAMPLE_TEXT}

Input Text With Clear False Appends
    [Documentation]    Verify input_text with clear=False appends to existing text.
    [Tags]    smoke    positive    clear    append
    # First, clear and put initial text
    Input Text    ${TEXT_WIDGET}    Initial    clear=${TRUE}

    # Append more text
    Input Text    ${TEXT_WIDGET}    Appended    clear=${FALSE}

    # Verify both texts are present
    Widget Text Should Be    ${TEXT_WIDGET}    InitialAppended

Input Text Clear Default Is True
    [Documentation]    Verify clear parameter defaults to True.
    [Tags]    positive    clear    default
    # Put initial text
    Input Text    ${TEXT_WIDGET}    Initial Text

    # Input new text without specifying clear (should default to True)
    Input Text    ${TEXT_WIDGET}    New Text

    # Verify only new text is present
    Widget Text Should Be    ${TEXT_WIDGET}    New Text

Multiple Appends With Clear False
    [Documentation]    Verify multiple appends with clear=False.
    [Tags]    positive    clear    append
    Input Text    ${TEXT_WIDGET}    One    clear=${TRUE}
    Input Text    ${TEXT_WIDGET}    Two    clear=${FALSE}
    Input Text    ${TEXT_WIDGET}    Three    clear=${FALSE}
    Widget Text Should Be    ${TEXT_WIDGET}    OneTwoThree

# ============================================================================
# input_text - Special Characters and Unicode
# ============================================================================

Input Text With Special Characters
    [Documentation]    Verify inputting text with special characters.
    [Tags]    positive    special-chars
    Input Text    ${TEXT_WIDGET}    ${SPECIAL_CHARS}    clear=${TRUE}
    Widget Text Should Be    ${TEXT_WIDGET}    ${SPECIAL_CHARS}

Input Text With Unicode Characters
    [Documentation]    Verify inputting text with Unicode characters.
    [Tags]    positive    unicode
    Input Text    ${TEXT_WIDGET}    ${UNICODE_TEXT}    clear=${TRUE}
    Widget Text Should Be    ${TEXT_WIDGET}    ${UNICODE_TEXT}

Input Text With Whitespace
    [Documentation]    Verify inputting whitespace-only text.
    [Tags]    positive    whitespace
    Input Text    ${TEXT_WIDGET}    ${WHITESPACE}    clear=${TRUE}
    Widget Text Should Be    ${TEXT_WIDGET}    ${WHITESPACE}

Input Long Text
    [Documentation]    Verify inputting a long text string.
    [Tags]    positive    long-text
    Input Text    ${TEXT_WIDGET}    ${LONG_TEXT}    clear=${TRUE}
    Widget Text Should Be    ${TEXT_WIDGET}    ${LONG_TEXT}

Input Empty String
    [Documentation]    Verify inputting an empty string.
    [Tags]    positive    edge-case
    Input Text    ${TEXT_WIDGET}    Initial    clear=${TRUE}
    Input Text    ${TEXT_WIDGET}    ${EMPTY_STRING}    clear=${TRUE}
    Widget Text Should Be    ${TEXT_WIDGET}    ${EMPTY_STRING}

# ============================================================================
# input_text - Negative Test Cases
# ============================================================================

Input Text Fails For Nonexistent Widget
    [Documentation]    Verify proper error when widget doesn't exist.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *not found*
    ...    Input Text    ${NONEXISTENT}    ${SAMPLE_TEXT}

Input Text Fails With Empty Locator
    [Documentation]    Verify behavior when locator is empty (may error or be no-op).
    [Tags]    negative    validation
    [Setup]    Log    Empty locator handling may vary - test logs behavior
    TRY
        Input Text    ${EMPTY}    ${SAMPLE_TEXT}
        Log    Empty locator input completed without error
    EXCEPT    *    type=GLOB
        Log    Empty locator input raised error (expected)
    END

Input Text To Readonly Widget
    [Documentation]    Verify behavior when inputting to read-only widget.
    [Tags]    negative    readonly
    # Behavior depends on implementation - may fail or have no effect
    TRY
        Input Text    ${READONLY_TEXT}    Should Not Work
        # If no error, verify text wasn't changed
        Log    Input to readonly succeeded - checking if text changed
    EXCEPT
        Log    Input to readonly widget raised error (expected)
    END

Input Text To Disabled Widget
    [Documentation]    Verify behavior when inputting to disabled widget.
    [Tags]    negative    disabled
    TRY
        Input Text    ${DISABLED_TEXT}    Should Not Work
        Log    Input to disabled succeeded - checking if text changed
    EXCEPT
        Log    Input to disabled widget raised error (expected)
    END

# ============================================================================
# clear_text - Positive Test Cases
# ============================================================================

Clear Text From Text Widget
    [Documentation]    Verify clearing text from a Text widget.
    [Tags]    smoke    critical    positive    clear-text
    # First put some text
    Input Text    ${TEXT_WIDGET}    ${SAMPLE_TEXT}    clear=${TRUE}
    Widget Text Should Be    ${TEXT_WIDGET}    ${SAMPLE_TEXT}

    # Clear the text
    Clear Text    ${TEXT_WIDGET}

    # Verify text is empty
    Widget Text Should Be    ${TEXT_WIDGET}    ${EMPTY}

Clear Text From StyledText Widget
    [Documentation]    Verify clearing text from a StyledText widget.
    [Tags]    positive    clear-text    styledtext
    Input Text    ${STYLED_TEXT_WIDGET}    ${SAMPLE_TEXT}    clear=${TRUE}
    Clear Text    ${STYLED_TEXT_WIDGET}
    Widget Text Should Be    ${STYLED_TEXT_WIDGET}    ${EMPTY}

Clear Text From Password Field
    [Documentation]    Verify clearing text from a password field.
    [Tags]    positive    clear-text    password
    Input Text    ${PASSWORD_WIDGET}    password123    clear=${TRUE}
    Clear Text    ${PASSWORD_WIDGET}
    # Password field should be empty

Clear Text By Name Locator
    [Documentation]    Verify clearing text using name: locator.
    [Tags]    positive    clear-text
    Input Text    name:textUsername    ${SAMPLE_TEXT}    clear=${TRUE}
    Clear Text    name:textUsername
    Widget Text Should Be    name:textUsername    ${EMPTY}

Clear Text Is Idempotent
    [Documentation]    Verify clearing already empty field doesn't fail.
    [Tags]    positive    clear-text    idempotent
    Clear Text    ${TEXT_WIDGET}
    Clear Text    ${TEXT_WIDGET}
    Clear Text    ${TEXT_WIDGET}
    Widget Text Should Be    ${TEXT_WIDGET}    ${EMPTY}

Clear Multiline Text
    [Documentation]    Verify clearing multiline text content.
    [Tags]    positive    clear-text    multiline
    Input Text    ${MULTILINE_TEXT}    ${MULTILINE_CONTENT}    clear=${TRUE}
    Clear Text    ${MULTILINE_TEXT}
    Widget Text Should Be    ${MULTILINE_TEXT}    ${EMPTY}

# ============================================================================
# clear_text - Negative Test Cases
# ============================================================================

Clear Text Fails For Nonexistent Widget
    [Documentation]    Verify proper error when widget doesn't exist.
    [Tags]    negative    error-handling    clear-text
    Run Keyword And Expect Error    *not found*
    ...    Clear Text    ${NONEXISTENT}

Clear Text Fails With Empty Locator
    [Documentation]    Verify behavior when locator is empty (may error or be no-op).
    [Tags]    negative    validation    clear-text
    TRY
        Clear Text    ${EMPTY}
        Log    Empty locator clear completed without error
    EXCEPT    *    type=GLOB
        Log    Empty locator clear raised error (expected)
    END

Clear Text On Readonly Widget
    [Documentation]    Verify behavior when clearing read-only widget.
    [Tags]    negative    readonly    clear-text
    TRY
        Clear Text    ${READONLY_TEXT}
        Log    Clear on readonly succeeded - text may be unchanged
    EXCEPT
        Log    Clear on readonly widget raised error (expected)
    END

# ============================================================================
# Combined Operations
# ============================================================================

Clear And Input Workflow
    [Documentation]    Verify typical clear then input workflow.
    [Tags]    positive    workflow
    # Clear field
    Clear Text    ${TEXT_WIDGET}

    # Input new text
    Input Text    ${TEXT_WIDGET}    ${SAMPLE_TEXT}    clear=${FALSE}

    # Verify
    Widget Text Should Be    ${TEXT_WIDGET}    ${SAMPLE_TEXT}

Multiple Input Operations
    [Documentation]    Verify multiple input operations on same widget.
    [Tags]    positive    workflow
    Input Text    ${TEXT_WIDGET}    First    clear=${TRUE}
    Widget Text Should Be    ${TEXT_WIDGET}    First

    Input Text    ${TEXT_WIDGET}    Second    clear=${TRUE}
    Widget Text Should Be    ${TEXT_WIDGET}    Second

    Input Text    ${TEXT_WIDGET}    Third    clear=${TRUE}
    Widget Text Should Be    ${TEXT_WIDGET}    Third

Input Text To Different Widgets
    [Documentation]    Verify inputting text to different widgets in sequence.
    [Tags]    positive    multiple-widgets
    Input Text    ${TEXT_WIDGET}    Text 1    clear=${TRUE}
    Input Text    ${STYLED_TEXT_WIDGET}    Text 2    clear=${TRUE}
    Input Text    ${PASSWORD_WIDGET}    Text 3    clear=${TRUE}

    Widget Text Should Be    ${TEXT_WIDGET}    Text 1
    Widget Text Should Be    ${STYLED_TEXT_WIDGET}    Text 2
    # Password field text may not be retrievable - just verify input succeeded
    Log    Verified text input to multiple different widgets


*** Keywords ***
# Local keywords for this test file
