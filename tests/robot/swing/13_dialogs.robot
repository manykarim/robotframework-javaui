*** Settings ***
Documentation     Dialog Tests - Testing JDialog operations.
...
...               These tests verify the library's ability to interact with
...               JDialog components for modal and modeless dialogs.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application
Test Setup        Clear UI State Before Dialog Test

Force Tags        dialogs    regression

*** Keywords ***
Clear UI State Before Dialog Test
    [Documentation]    Ensure clean state before dialog tests.
    # Allow any pending EDT operations to complete
    Sleep    0.5s

*** Test Cases ***
# =============================================================================
# MODELESS DIALOG TESTS
# =============================================================================

Open Modeless Settings Dialog
    [Documentation]    Open the modeless settings dialog and close it.
    [Tags]    smoke    positive    modeless
    Click Element    JButton[name='openDialogButton']
    Sleep    0.5s
    Wait Until Element Exists    JDialog[name='settingsDialog']    timeout=5
    Element Should Be Visible    JDialog[name='settingsDialog']
    # Close with OK button
    Click Element    JButton[name='settingsDialogOkButton']
    Sleep    0.3s

Open Settings Dialog Via Toolbar
    [Documentation]    Open settings dialog via toolbar button.
    [Tags]    positive    toolbar
    Click Element    JButton[name='toolbarSettingsButton']
    Sleep    0.5s
    Wait Until Element Exists    JDialog[name='settingsDialog']    timeout=5
    Element Should Be Visible    JDialog[name='settingsDialog']
    # Close with Cancel button
    Click Element    JButton[name='settingsDialogCancelButton']
    Sleep    0.3s

# =============================================================================
# MODAL DIALOG TESTS
# =============================================================================

Open Modal About Dialog
    [Documentation]    Open the modal about dialog and close it.
    [Tags]    positive    modal
    Click Element    JButton[name='openModalDialogButton']
    Sleep    0.5s
    Wait Until Element Exists    JDialog[name='aboutDialog']    timeout=5
    Element Should Be Visible    JDialog[name='aboutDialog']
    # Verify dialog content
    Element Should Exist    JLabel[name='aboutAppNameLabel']
    Element Should Exist    JLabel[name='aboutVersionLabel']
    # Close with Close button
    Click Element    JButton[name='aboutCloseButton']
    Sleep    0.3s

Open About Dialog Via Menu
    [Documentation]    Open about dialog via Help menu.
    [Tags]    positive    menu
    Select Menu    Help|About
    Sleep    0.5s
    Wait Until Element Exists    JDialog[name='aboutDialog']    timeout=5
    Element Should Be Visible    JDialog[name='aboutDialog']
    Click Element    JButton[name='aboutCloseButton']
    Sleep    0.3s

# =============================================================================
# DIALOG INTERACTION
# =============================================================================

Interact With Settings Dialog Controls
    [Documentation]    Open settings dialog and interact with its controls.
    [Tags]    positive    interaction
    Click Element    JButton[name='openDialogButton']
    Sleep    0.5s
    Wait Until Element Exists    JDialog[name='settingsDialog']    timeout=5
    # The dialog content exists
    Element Should Exist    JLabel[name='settingsDialogLabel']
    # Close dialog
    Click Element    JButton[name='settingsDialogOkButton']
    Sleep    0.3s

Verify About Dialog Content
    [Documentation]    Verify the content of the About dialog.
    [Tags]    positive    content-verification
    Click Element    JButton[name='openModalDialogButton']
    Sleep    1.0s
    Wait Until Element Exists    JDialog[name='aboutDialog']    timeout=5
    Sleep    0.5s
    # Verify all labels exist
    Element Should Exist    JLabel[name='aboutAppNameLabel']
    Element Should Exist    JLabel[name='aboutVersionLabel']
    Element Should Exist    JLabel[name='aboutCopyrightLabel']
    Element Should Exist    JLabel[name='aboutDescLabel']
    # Close button should exist
    Element Should Exist    JButton[name='aboutCloseButton']
    # Close dialog - must succeed to allow next tests to run
    Sleep    1.0s
    ${closed}=    Set Variable    ${FALSE}
    FOR    ${i}    IN RANGE    10
        TRY
            # Clear cache before each attempt to get fresh component lookup
            Refresh UI Tree
            Click Element    JButton[name='aboutCloseButton']
            ${closed}=    Set Variable    ${TRUE}
            Exit For Loop
        EXCEPT    AS    ${error}
            Log    Retry ${i}: ${error}
            Sleep    0.5s
        END
    END
    # Wait for dialog to fully close
    Sleep    1.0s
    # Must have closed successfully
    Should Be True    ${closed}    Failed to close About dialog after 10 attempts

# =============================================================================
# DIALOG WORKFLOWS
# =============================================================================

Open And Close Dialog Multiple Times
    [Documentation]    Open and close dialog multiple times.
    [Tags]    positive    workflow
    # First open/close cycle
    Click Element    JButton[name='openDialogButton']
    Sleep    0.5s
    Wait Until Element Exists    JDialog[name='settingsDialog']    timeout=5
    Sleep    0.3s
    Click Element    JButton[name='settingsDialogOkButton']
    Sleep    0.5s
    # Second open/close cycle
    Click Element    JButton[name='openDialogButton']
    Sleep    0.5s
    Wait Until Element Exists    JDialog[name='settingsDialog']    timeout=5
    Sleep    0.3s
    Click Element    JButton[name='settingsDialogCancelButton']
    Sleep    0.5s
    # Third open/close cycle
    Click Element    JButton[name='openDialogButton']
    Sleep    0.5s
    Wait Until Element Exists    JDialog[name='settingsDialog']    timeout=5
    Sleep    0.3s
    Click Element    JButton[name='settingsDialogOkButton']
    Sleep    0.5s

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Find Nonexistent Dialog Fails
    [Documentation]    Finding non-existent dialog throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Element Should Exist    JDialog[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}
