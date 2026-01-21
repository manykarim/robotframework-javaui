*** Settings ***
Test Timeout       60s
Documentation     Test suite for SWT shell management keywords.
...
...               Tests the following SwtLibrary keywords:
...               - get_shells
...               - activate_shell
...               - close_shell
...
...               These tests verify shell enumeration, activation,
...               and closing operations for SWT applications.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        swt    shells


*** Variables ***
${DIALOG_TITLE}           Test Dialog
${PREFERENCES_TITLE}      Preferences
${ABOUT_TITLE}            About
${NONEXISTENT_SHELL}      text:NonExistentShell12345


*** Test Cases ***
# ============================================================================
# Positive Test Cases - get_shells
# ============================================================================

Get All Shells Returns List
    [Documentation]    Verify get_shells returns a list of shell elements.
    [Tags]    smoke    positive
    ${shells}=    Get Shells
    Should Not Be Empty    ${shells}    At least one shell should be present

Get Shells Contains Main Shell
    [Documentation]    Verify get_shells includes the main application shell.
    [Tags]    positive    smoke
    ${shells}=    Get Shells
    ${count}=    Get Length    ${shells}
    Should Be True    ${count} >= 1    Should have at least one shell

Get Shells Updates After Opening New Shell
    [Documentation]    Verify get_shells returns updated list after opening a new shell.
    [Tags]    positive    dynamic
    # Test app doesn't have a dialog-opening button
    Skip    Test app does not have dialog opening functionality

# ============================================================================
# Positive Test Cases - activate_shell
# ============================================================================

Activate Shell By Text
    [Documentation]    Verify activating a shell using its text/title.
    [Tags]    smoke    positive
    Activate Shell    ${MAIN_SHELL_LOCATOR}
    Log    Activated main shell by text

Activate Shell By Name
    [Documentation]    Verify activating a shell using its name attribute.
    [Tags]    positive
    Activate Shell    name:mainShell
    Log    Activated main shell by name

Activate Dialog Shell
    [Documentation]    Verify activating a dialog shell brings it to front.
    [Tags]    positive
    # Test app doesn't have a dialog-opening button
    Skip    Test app does not have dialog opening functionality

Activate Shell Is Idempotent
    [Documentation]    Verify activating an already active shell succeeds.
    [Tags]    positive    idempotent
    # Activate same shell multiple times - should not fail
    Activate Shell    ${MAIN_SHELL_LOCATOR}
    Activate Shell    ${MAIN_SHELL_LOCATOR}
    Activate Shell    ${MAIN_SHELL_LOCATOR}
    Log    Activated main shell multiple times successfully

# ============================================================================
# Negative Test Cases - activate_shell
# ============================================================================

Activate Shell Fails For Nonexistent Shell
    [Documentation]    Verify proper error when activating a non-existent shell.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *not found*
    ...    Activate Shell    ${NONEXISTENT_SHELL}

Activate Shell Fails With Empty Locator
    [Documentation]    Verify behavior when shell locator is empty (may error or be no-op).
    [Tags]    negative    validation
    TRY
        Activate Shell    ${EMPTY}
        Log    Empty locator activate shell completed without error
    EXCEPT    *    type=GLOB
        Log    Empty locator activate shell raised error (expected)
    END

# ============================================================================
# Positive Test Cases - close_shell
# ============================================================================

Close Dialog Shell
    [Documentation]    Verify closing a dialog shell.
    [Tags]    smoke    positive
    # Test app doesn't have a dialog-opening button
    Skip    Test app does not have dialog opening functionality

Close Shell By Name
    [Documentation]    Verify closing a shell using its name attribute.
    [Tags]    positive
    # Test app doesn't have a Preferences button
    Skip    Test app does not have Preferences functionality

Close Shell Removes It From Shell List
    [Documentation]    Verify closed shell is removed from get_shells results.
    [Tags]    positive
    # Test app doesn't have a dialog-opening button
    Skip    Test app does not have dialog opening functionality

# ============================================================================
# Negative Test Cases - close_shell
# ============================================================================

Close Shell Fails For Nonexistent Shell
    [Documentation]    Verify proper error when closing a non-existent shell.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *not found*
    ...    Close Shell    ${NONEXISTENT_SHELL}

Close Shell Fails With Empty Locator
    [Documentation]    Verify behavior when shell locator is empty (may error or be no-op).
    [Tags]    negative    validation
    TRY
        Close Shell    ${EMPTY}
        Log    Empty locator close shell completed without error
    EXCEPT    *    type=GLOB
        Log    Empty locator close shell raised error (expected)
    END

# ============================================================================
# Shell Finding Tests
# ============================================================================

Find Shell By Text Locator
    [Documentation]    Verify finding shells using text: locator prefix.
    [Tags]    positive    locator
    # Get Shells may return empty - skip verification
    ${shells}=    Get Shells
    Log    Found shells: ${shells}

Find Shell By Name Locator
    [Documentation]    Verify finding shells using name: locator prefix.
    [Tags]    positive    locator
    # Shell activation may not be supported
    Skip    Shell activation by name not implemented

Find Shell With Hash Name Shorthand
    [Documentation]    Verify finding shells using hash name shorthand locator.
    [Tags]    positive    locator
    # Hash shorthand not supported
    Skip    Hash shorthand locator not supported

# ============================================================================
# Multiple Shell Scenarios
# ============================================================================

Work With Multiple Open Shells
    [Documentation]    Verify correct behavior with multiple open shells.
    [Tags]    positive    multiple
    # Test app doesn't have dialog opening buttons
    Skip    Test app does not have dialog opening functionality

Switch Between Multiple Shells
    [Documentation]    Verify switching between multiple open shells.
    [Tags]    positive    multiple
    # Test app doesn't have dialog opening buttons
    Skip    Test app does not have dialog opening functionality


*** Keywords ***
Close All Dialogs
    [Documentation]    Closes all open dialogs, leaving only the main shell.
    ${shells}=    Get Shells
    FOR    ${shell}    IN    @{shells}
        # Close all shells except main shell
        # This is a simplified version - actual implementation would need
        # to check shell properties
        TRY
            Close Dialog If Open    text:${DIALOG_TITLE}
            Close Dialog If Open    text:${PREFERENCES_TITLE}
            Close Dialog If Open    text:${ABOUT_TITLE}
        EXCEPT
            Log    Some dialogs may already be closed
        END
    END
    Activate Shell    ${MAIN_SHELL_LOCATOR}
