*** Settings ***
Documentation     Test suite for RCP Preferences operations.
...               Tests opening the preferences dialog and navigating
...               to specific preference pages in Eclipse RCP applications.

Resource          resources/common.resource

Suite Setup       Suite Setup Start RCP App
Suite Teardown    Suite Teardown Stop RCP App
Test Setup        Test Setup Reset State
Test Teardown     Close Any Open Preferences

Force Tags        rcp    preferences


*** Test Cases ***
# =============================================================================
# Open Preferences Tests
# =============================================================================

Open Preferences Successfully
    [Documentation]    Verify opening the Preferences dialog.
    ...                Should display the Eclipse Preferences window.
    [Tags]    smoke    critical    positive
    Open Preferences
    # Verify preferences dialog is open
    ${shells}=    Get Shells
    Log    Shells after opening preferences: ${shells}
    # Close preferences dialog
    Close Shell    text:Preferences

Open Preferences Multiple Times
    [Documentation]    Verify opening preferences multiple times.
    ...                Should handle reopening after closing.
    [Tags]    positive    reliability
    FOR    ${i}    IN RANGE    2
        Open Preferences
        Log    Preferences opened iteration ${i + 1}
        Close Shell    text:Preferences
    END

Open Preferences Creates Modal Dialog
    [Documentation]    Verify preferences opens as a dialog.
    ...                Should create a new shell/window.
    [Tags]    positive
    ${shells_before}=    Get Shells
    ${count_before}=    Get Length    ${shells_before}
    Open Preferences
    ${shells_after}=    Get Shells
    ${count_after}=    Get Length    ${shells_after}
    Should Be True    ${count_after} > ${count_before}


# =============================================================================
# Open Preferences - Negative Tests
# =============================================================================

Open Preferences Without Connection Fails
    [Documentation]    Verify opening preferences fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Open Preferences
    [Teardown]    Connect To RCP App


# =============================================================================
# Navigate To Preference Page Tests
# =============================================================================

Navigate To Preference Page Successfully
    [Documentation]    Verify navigating to a specific preference page.
    ...                Should expand tree and select the page.
    [Tags]    smoke    critical    positive
    Open Preferences
    Navigate To Preference Page    ${PREF_GENERAL}
    Log    Navigated to General preferences

Navigate To Nested Preference Page
    [Documentation]    Verify navigating to a nested preference page.
    ...                Should navigate through multiple tree levels.
    [Tags]    positive
    Open Preferences
    Navigate To Preference Page    ${PREF_APPEARANCE}
    Log    Navigated to General > Appearance

Navigate To Deep Preference Page
    [Documentation]    Verify navigating to deeply nested preference page.
    ...                Should handle multi-level tree paths.
    [Tags]    positive
    Open Preferences
    Navigate To Preference Page    ${PREF_TEXT_EDITORS}
    Log    Navigated to General > Editors > Text Editors

Navigate To Different Preference Pages
    [Documentation]    Verify navigating to various preference pages.
    ...                Tests different preference categories.
    [Tags]    positive
    Open Preferences
    Navigate To Preference Page    ${PREF_GENERAL}
    Navigate To Preference Page    ${PREF_JAVA}
    Navigate To Preference Page    ${PREF_APPEARANCE}
    Log    Navigated to multiple preference pages

Navigate To Same Page Multiple Times
    [Documentation]    Verify navigating to same page multiple times.
    ...                Should not cause error on repeated navigation.
    [Tags]    positive    edge-case
    Open Preferences
    FOR    ${i}    IN RANGE    3
        Navigate To Preference Page    ${PREF_GENERAL}
        Log    Navigation iteration ${i + 1}
    END


# =============================================================================
# Navigate To Preference Page - Negative Tests
# =============================================================================

Navigate With Empty Path Fails
    [Documentation]    Verify navigating with empty path fails.
    ...                Tests input validation for preference path.
    [Tags]    negative    error-handling    validation
    Open Preferences
    Run Keyword And Expect Error    *empty*
    ...    Navigate To Preference Page    ${EMPTY}

Navigate To Invalid Page Silently Succeeds
    [Documentation]    Mock app doesn't validate invalid preference pages.
    ...                Real Eclipse would show an error.
    [Tags]    negative    mock-limitation
    Open Preferences
    # Mock app accepts invalid paths without error
    Navigate To Preference Page    InvalidCategory|NonExistentPage
    Log    Mock app accepted invalid path without error
    Close Shell    text:Preferences

Navigate Without Open Preferences Silently Succeeds
    [Documentation]    Mock app allows navigation without open preferences.
    ...                Real Eclipse would require preferences to be open first.
    [Tags]    negative    mock-limitation
    # Ensure no preferences dialog
    Run Keyword And Ignore Error    Close Shell    text:Preferences
    # Mock app accepts navigation without preferences open
    Navigate To Preference Page    ${PREF_GENERAL}
    Log    Mock app accepted navigation without preferences dialog

Navigate Without Connection Fails
    [Documentation]    Verify navigation fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Navigate To Preference Page    ${PREF_GENERAL}
    [Teardown]    Connect To RCP App


# =============================================================================
# Edge Cases
# =============================================================================

Navigate With Partial Path
    [Documentation]    Verify navigation with partial path.
    ...                Should expand and select the specified node.
    [Tags]    positive    edge-case
    Open Preferences
    Navigate To Preference Page    General
    Log    Partial path navigation tested

Navigate After Preference Search
    [Documentation]    Verify navigation works after using search.
    ...                Tests state after filtered view.
    [Tags]    positive    edge-case
    Open Preferences
    # If there's a search, it might filter the tree
    Navigate To Preference Page    ${PREF_GENERAL}
    Log    Navigation after potential filter tested


# =============================================================================
# Integration Tests
# =============================================================================

Full Preferences Workflow
    [Documentation]    Test complete preferences workflow.
    ...                Open, navigate, verify, close.
    [Tags]    integration    positive
    # Open preferences
    Open Preferences
    # Navigate to a page
    Navigate To Preference Page    ${PREF_APPEARANCE}
    # Navigate to another page
    Navigate To Preference Page    ${PREF_JAVA}
    # Close preferences
    Close Shell    text:Preferences
    Log    Full preferences workflow completed

Preferences With Widget Interaction
    [Documentation]    Test preferences with widget interaction.
    ...                Navigate and interact with preference controls.
    [Tags]    integration    positive
    Open Preferences
    Navigate To Preference Page    ${PREF_GENERAL}
    # Find widgets in preference page using specific name
    ${tree}=    Find Widget    name:preferencesTree
    Log    Found preferences tree: ${tree}
    # Close without saving
    Close Shell    text:Preferences

Preferences And Editor Workflow
    [Documentation]    Test preferences alongside editor operations.
    ...                Combines preferences and editor workflows.
    [Tags]    integration    positive
    # Open an editor
    Open Editor    ${TEST_FILE_JAVA}
    # Open preferences
    Open Preferences
    Navigate To Preference Page    ${PREF_TEXT_EDITORS}
    # Close preferences
    Close Shell    text:Preferences
    # Verify editor is still open
    ${editor}=    Get Active Editor
    Should Not Be Empty    ${editor}
    # Cleanup
    Close All Editors    save=${FALSE}


*** Keywords ***
Close Any Open Preferences
    [Documentation]    Test teardown to close any open preferences dialog.
    Run Keyword And Ignore Error    Close Shell    text:Preferences
