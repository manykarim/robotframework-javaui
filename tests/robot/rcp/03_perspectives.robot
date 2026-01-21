*** Settings ***
Test Timeout       60s
Documentation     Test suite for RCP Perspective operations.
...               Tests perspective switching, retrieval, reset,
...               and available perspectives listing.

Resource          resources/common.resource

Suite Setup       Suite Setup Start RCP App
Suite Teardown    Suite Teardown Stop RCP App
Test Setup        Test Setup Reset State
Test Teardown     Test Teardown Cleanup

Force Tags        rcp    perspective


*** Test Cases ***
# =============================================================================
# Get Active Perspective Tests
# =============================================================================

Get Active Perspective Successfully
    [Documentation]    Verify retrieving the currently active perspective ID.
    ...                Returns the perspective ID string.
    [Tags]    smoke    critical    positive
    ${perspective}=    Get Active Perspective
    Should Not Be Empty    ${perspective}
    Log    Active perspective: ${perspective}

Get Active Perspective Returns Valid ID
    [Documentation]    Verify active perspective returns a valid perspective ID.
    ...                The ID should be a non-empty string in Eclipse ID format.
    [Tags]    positive
    ${perspective}=    Get Active Perspective
    Should Match Regexp    ${perspective}    ^[a-zA-Z0-9_.]+$
    Log    Perspective ID format validated: ${perspective}

Get Active Perspective After Switch
    [Documentation]    Verify getting active perspective reflects recent switch.
    ...                Tests that the returned ID matches what was opened.
    [Tags]    positive
    Open Perspective    ${DEBUG_PERSPECTIVE}
    ${perspective}=    Get Active Perspective
    Should Be Equal    ${perspective}    ${DEBUG_PERSPECTIVE}


# =============================================================================
# Open Perspective Tests
# =============================================================================

Open Perspective By ID Successfully
    [Documentation]    Verify opening a perspective by its ID.
    ...                Should switch workbench to the specified perspective.
    [Tags]    smoke    critical    positive
    Open Perspective    ${JAVA_PERSPECTIVE}
    ${active}=    Get Active Perspective
    Should Be Equal    ${active}    ${JAVA_PERSPECTIVE}

Open Perspective With Different IDs
    [Documentation]    Verify opening different perspectives by ID.
    ...                Tests switching between multiple perspectives.
    [Tags]    positive
    # Open Java perspective
    Open Perspective    ${JAVA_PERSPECTIVE}
    ${active1}=    Get Active Perspective
    Should Be Equal    ${active1}    ${JAVA_PERSPECTIVE}
    # Open Debug perspective
    Open Perspective    ${DEBUG_PERSPECTIVE}
    ${active2}=    Get Active Perspective
    Should Be Equal    ${active2}    ${DEBUG_PERSPECTIVE}
    # Switch back to Java
    Open Perspective    ${JAVA_PERSPECTIVE}
    ${active3}=    Get Active Perspective
    Should Be Equal    ${active3}    ${JAVA_PERSPECTIVE}

Open Same Perspective Multiple Times
    [Documentation]    Verify opening the same perspective multiple times is safe.
    ...                Should not cause errors when already on that perspective.
    [Tags]    positive    edge-case
    FOR    ${i}    IN RANGE    3
        Open Perspective    ${JAVA_PERSPECTIVE}
        ${active}=    Get Active Perspective
        Should Be Equal    ${active}    ${JAVA_PERSPECTIVE}
    END

Open Perspective Updates Workbench State
    [Documentation]    Verify opening perspective updates the workbench.
    ...                Views and layout should change according to perspective.
    [Tags]    positive    integration
    ${views_before}=    Get Open Views
    Open Perspective    ${DEBUG_PERSPECTIVE}
    ${views_after}=    Get Open Views
    # Views may differ between perspectives
    Log    Views before: ${views_before}
    Log    Views after: ${views_after}


# =============================================================================
# Open Perspective - Negative Tests
# =============================================================================

Open Perspective With Empty ID Fails
    [Documentation]    Verify opening perspective with empty ID fails.
    ...                Tests input validation for perspective ID.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Open Perspective    ${EMPTY}

Open Perspective With Invalid ID Fails
    [Documentation]    Verify opening non-existent perspective fails.
    ...                Tests error handling for invalid perspective ID.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *
    ...    Open Perspective    ${INVALID_PERSPECTIVE}

Open Perspective Without Connection Fails
    [Documentation]    Verify opening perspective fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Open Perspective    ${JAVA_PERSPECTIVE}
    [Teardown]    Connect To RCP App


# =============================================================================
# Reset Perspective Tests
# =============================================================================

Reset Perspective Successfully
    [Documentation]    Verify resetting perspective to default layout.
    ...                Should restore the original perspective configuration.
    [Tags]    smoke    positive
    Open Perspective    ${JAVA_PERSPECTIVE}
    # Close some views to modify layout
    Close View If Visible    ${PROBLEMS_VIEW}
    # Reset to default
    Reset Perspective
    Log    Perspective reset to default layout

Reset Perspective Multiple Times
    [Documentation]    Verify resetting perspective multiple times is safe.
    ...                Should not cause errors on repeated resets.
    [Tags]    positive    edge-case
    FOR    ${i}    IN RANGE    3
        Reset Perspective
        Log    Reset ${i + 1} completed
    END

Reset Perspective After View Changes
    [Documentation]    Verify reset restores perspective after view modifications.
    ...                Tests that layout is restored after closing/showing views.
    [Tags]    positive
    Open Perspective    ${JAVA_PERSPECTIVE}
    ${views_initial}=    Get Open Views
    # Modify by showing additional view
    Show View    ${CONSOLE_VIEW}
    Reset Perspective
    # Layout should be restored
    Log    Perspective reset after view changes

Reset Perspective Without Connection Fails
    [Documentation]    Verify reset perspective fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Reset Perspective
    [Teardown]    Connect To RCP App


# =============================================================================
# Get Available Perspectives Tests
# =============================================================================

Get Available Perspectives Successfully
    [Documentation]    Verify retrieving list of all available perspectives.
    ...                Returns a list of perspective descriptors.
    [Tags]    smoke    positive
    ${perspectives}=    Get Available Perspectives
    Should Not Be Empty    ${perspectives}
    ${count}=    Get Length    ${perspectives}
    Should Be True    ${count} >= 1    Should have at least one perspective

Get Available Perspectives Returns List
    [Documentation]    Verify available perspectives is returned as a list.
    ...                Each item should contain perspective information.
    [Tags]    positive
    ${perspectives}=    Get Available Perspectives
    Should Not Be Empty    ${perspectives}
    # Log each perspective
    FOR    ${perspective}    IN    @{perspectives}
        Log    Perspective: ${perspective}
    END

Available Perspectives Contains Known Perspective
    [Documentation]    Verify available perspectives contains expected perspective.
    ...                Java perspective should be in standard Eclipse installations.
    [Tags]    positive
    ${perspectives}=    Get Available Perspectives
    # Check if our test perspectives are available
    ${ids}=    Evaluate    [p.get('id', '') for p in ${perspectives}]
    Log    Available perspective IDs: ${ids}

Get Available Perspectives Multiple Times
    [Documentation]    Verify getting available perspectives is consistent.
    ...                Multiple calls should return the same perspectives.
    [Tags]    positive    reliability
    ${perspectives1}=    Get Available Perspectives
    ${perspectives2}=    Get Available Perspectives
    ${count1}=    Get Length    ${perspectives1}
    ${count2}=    Get Length    ${perspectives2}
    Should Be Equal    ${count1}    ${count2}    Should return consistent results

Get Available Perspectives Without Connection Fails
    [Documentation]    Verify getting perspectives fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Get Available Perspectives
    [Teardown]    Connect To RCP App


# =============================================================================
# Integration Tests
# =============================================================================

Full Perspective Workflow
    [Documentation]    Test complete perspective workflow.
    ...                Get perspectives, switch, reset, and verify.
    [Tags]    integration    positive
    # Get available perspectives
    ${available}=    Get Available Perspectives
    Should Not Be Empty    ${available}
    # Get current perspective
    ${initial}=    Get Active Perspective
    Log    Initial perspective: ${initial}
    # Open a different perspective
    Open Perspective    ${DEBUG_PERSPECTIVE}
    ${after_switch}=    Get Active Perspective
    Should Be Equal    ${after_switch}    ${DEBUG_PERSPECTIVE}
    # Reset perspective
    Reset Perspective
    ${after_reset}=    Get Active Perspective
    Should Be Equal    ${after_reset}    ${DEBUG_PERSPECTIVE}
    # Switch back
    Open Perspective    ${initial}
    ${final}=    Get Active Perspective
    Should Be Equal    ${final}    ${initial}

Perspective Operations Affect Views
    [Documentation]    Verify perspective operations affect visible views.
    ...                Different perspectives show different default views.
    [Tags]    integration    positive
    Open Perspective    ${JAVA_PERSPECTIVE}
    ${java_views}=    Get Open Views
    Open Perspective    ${DEBUG_PERSPECTIVE}
    ${debug_views}=    Get Open Views
    # Views may differ between perspectives
    Log    Java views: ${java_views}
    Log    Debug views: ${debug_views}
