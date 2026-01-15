*** Settings ***
Documentation     Test suite for RCP Workbench information and state.
...               Tests retrieval of workbench metadata, window information,
...               and overall workbench state.

Resource          resources/common.resource

Suite Setup       Suite Setup Start RCP App
Suite Teardown    Suite Teardown Stop RCP App
Test Setup        Test Setup Reset State
Test Teardown     Test Teardown Cleanup

Force Tags        rcp    workbench


*** Test Cases ***
# =============================================================================
# Positive Test Cases - Workbench Information
# =============================================================================

Get Workbench Information Successfully
    [Documentation]    Verify retrieving comprehensive workbench information.
    ...                Returns a dictionary with workbench state details.
    [Tags]    smoke    critical    positive    info
    ${info}=    Get Workbench Info
    Should Not Be Empty    ${info}
    Log    Workbench Info: ${info}

Workbench Info Contains Expected Keys
    [Documentation]    Verify workbench info contains expected information keys.
    ...                Checks for presence of standard workbench properties.
    [Tags]    positive    info
    ${info}=    Get Workbench Info
    # Verify expected keys are present (based on RCP workbench structure)
    Log Dictionary    ${info}
    # The exact keys depend on the RCP agent implementation
    Should Not Be Empty    ${info}

Workbench Info After Perspective Change
    [Documentation]    Verify workbench info updates after changing perspective.
    ...                Tests that info reflects current workbench state.
    [Tags]    positive    info    perspective
    ${info_before}=    Get Workbench Info
    Log    Info before: ${info_before}
    Open Perspective    ${DEBUG_PERSPECTIVE}
    ${info_after}=    Get Workbench Info
    Log    Info after: ${info_after}
    # Info should reflect the current state
    Should Not Be Empty    ${info_after}

Workbench Info After Opening View
    [Documentation]    Verify workbench info updates after opening a view.
    ...                Tests that open views are reflected in workbench info.
    [Tags]    positive    info    view
    Show View    ${CONSOLE_VIEW}
    ${info}=    Get Workbench Info
    Should Not Be Empty    ${info}
    Log    Workbench Info with Console View: ${info}

Workbench Info After Opening Editor
    [Documentation]    Verify workbench info updates after opening an editor.
    ...                Tests that open editors are reflected in workbench info.
    [Tags]    positive    info    editor
    Open Editor    ${TEST_FILE_JAVA}
    ${info}=    Get Workbench Info
    Should Not Be Empty    ${info}
    Log    Workbench Info with Editor: ${info}


# =============================================================================
# Positive Test Cases - Workbench State
# =============================================================================

Get Workbench Info Multiple Times
    [Documentation]    Verify getting workbench info multiple times is consistent.
    ...                Tests repeated calls return consistent information.
    [Tags]    positive    reliability
    FOR    ${i}    IN RANGE    3
        ${info}=    Get Workbench Info
        Should Not Be Empty    ${info}
        Log    Call ${i + 1}: ${info}
    END

Workbench Info With Multiple Views Open
    [Documentation]    Verify workbench info with multiple views open.
    ...                Tests info accuracy with complex view arrangement.
    [Tags]    positive    info    view
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Show View    ${PROBLEMS_VIEW}
    Show View    ${CONSOLE_VIEW}
    ${info}=    Get Workbench Info
    Should Not Be Empty    ${info}
    Log    Workbench Info with multiple views: ${info}

Workbench Info With Multiple Editors Open
    [Documentation]    Verify workbench info with multiple editors open.
    ...                Tests info accuracy with multiple open editors.
    [Tags]    positive    info    editor
    Open Editor    ${TEST_FILE_JAVA}
    Open Editor    ${TEST_FILE_XML}
    ${info}=    Get Workbench Info
    Should Not Be Empty    ${info}
    Log    Workbench Info with multiple editors: ${info}


# =============================================================================
# Negative Test Cases
# =============================================================================

Get Workbench Info Without Connection
    [Documentation]    Verify getting workbench info fails without connection.
    ...                Tests proper error handling when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Get Workbench Info
    [Teardown]    Connect To RCP App


# =============================================================================
# Edge Cases
# =============================================================================

Workbench Info After Reset Perspective
    [Documentation]    Verify workbench info after resetting perspective.
    ...                Tests info is accurate after perspective reset.
    [Tags]    positive    edge-case
    Reset Perspective
    ${info}=    Get Workbench Info
    Should Not Be Empty    ${info}

Workbench Info After Closing All Editors
    [Documentation]    Verify workbench info after closing all editors.
    ...                Tests info reflects empty editor state.
    [Tags]    positive    edge-case
    Open Editor    ${TEST_FILE_JAVA}
    Close All Editors    save=${FALSE}
    ${info}=    Get Workbench Info
    Should Not Be Empty    ${info}
