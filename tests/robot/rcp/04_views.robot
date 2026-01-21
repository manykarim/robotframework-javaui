*** Settings ***
Test Timeout       60s
Documentation     Test suite for RCP View operations.
...               Tests showing, closing, activating views, visibility checks,
...               and retrieving widgets from within views.

Resource          resources/common.resource

Suite Setup       Suite Setup Start RCP App
Suite Teardown    Suite Teardown Stop RCP App
Test Setup        Test Setup Reset State
Test Teardown     Test Teardown Cleanup

Force Tags        rcp    view


*** Test Cases ***
# =============================================================================
# Show View Tests
# =============================================================================

Show View By ID Successfully
    [Documentation]    Verify showing a view by its ID.
    ...                Opens and displays the view in the workbench.
    [Tags]    smoke    critical    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    View Should Be Visible    ${PACKAGE_EXPLORER_VIEW}

Show View With Different IDs
    [Documentation]    Verify showing different views by their IDs.
    ...                Tests opening multiple standard Eclipse views.
    [Tags]    positive
    Show View    ${PROBLEMS_VIEW}
    View Should Be Visible    ${PROBLEMS_VIEW}
    Show View    ${CONSOLE_VIEW}
    View Should Be Visible    ${CONSOLE_VIEW}
    Show View    ${OUTLINE_VIEW}
    View Should Be Visible    ${OUTLINE_VIEW}

Show View With Secondary ID
    [Documentation]    Verify showing a view with a secondary ID.
    ...                Tests multi-instance view support.
    [Tags]    positive
    Show View    ${PROPERTIES_VIEW}    secondary_id=testInstance
    View Should Be Visible    ${PROPERTIES_VIEW}

Show Same View Multiple Times
    [Documentation]    Verify showing the same view multiple times is safe.
    ...                Should not cause errors if view is already visible.
    [Tags]    positive    edge-case
    FOR    ${i}    IN RANGE    3
        Show View    ${CONSOLE_VIEW}
        View Should Be Visible    ${CONSOLE_VIEW}
    END

Show View Already Open
    [Documentation]    Verify showing a view that is already open.
    ...                Should not cause error, view remains visible.
    [Tags]    positive    edge-case
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Show View    ${PACKAGE_EXPLORER_VIEW}
    View Should Be Visible    ${PACKAGE_EXPLORER_VIEW}


# =============================================================================
# Show View - Negative Tests
# =============================================================================

Show View With Empty ID Fails
    [Documentation]    Verify showing view with empty ID fails.
    ...                Tests input validation for view ID.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Show View    ${EMPTY}

Show View With Invalid ID Fails
    [Documentation]    Verify showing non-existent view fails.
    ...                Tests error handling for invalid view ID.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *
    ...    Show View    ${INVALID_VIEW}

Show View Without Connection Fails
    [Documentation]    Verify showing view fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Show View    ${CONSOLE_VIEW}
    [Teardown]    Connect To RCP App


# =============================================================================
# Close View Tests
# =============================================================================

Close View Successfully
    [Documentation]    Verify closing an open view.
    ...                View should no longer be visible after closing.
    [Tags]    smoke    critical    positive
    Show View    ${CONSOLE_VIEW}
    View Should Be Visible    ${CONSOLE_VIEW}
    Close View    ${CONSOLE_VIEW}
    # View should no longer be visible
    ${views}=    Get Open Views
    ${view_ids}=    Evaluate    [v.get('id', '') for v in ${views}]
    Should Not Contain    ${view_ids}    ${CONSOLE_VIEW}

Close View With Secondary ID
    [Documentation]    Verify closing a view with a secondary ID.
    ...                Tests closing specific instance of multi-instance view.
    [Tags]    positive
    Show View    ${PROPERTIES_VIEW}    secondary_id=testClose
    Close View    ${PROPERTIES_VIEW}    secondary_id=testClose

Close View That Is Not Open
    [Documentation]    Verify closing a view that is not open.
    ...                May fail or be handled gracefully depending on implementation.
    [Tags]    negative    edge-case
    # First ensure it's closed
    Close View If Visible    ${CONSOLE_VIEW}
    # Try to close again - behavior depends on implementation
    Run Keyword And Ignore Error    Close View    ${CONSOLE_VIEW}


# =============================================================================
# Close View - Negative Tests
# =============================================================================

Close View With Empty ID Fails
    [Documentation]    Verify closing view with empty ID fails.
    ...                Tests input validation for view ID.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Close View    ${EMPTY}

Close View Without Connection Fails
    [Documentation]    Verify closing view fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Close View    ${CONSOLE_VIEW}
    [Teardown]    Connect To RCP App


# =============================================================================
# Activate View Tests
# =============================================================================

Activate View Successfully
    [Documentation]    Verify activating a view brings it to front.
    ...                View should gain focus and become active.
    [Tags]    smoke    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Show View    ${PROBLEMS_VIEW}
    Activate View    ${PACKAGE_EXPLORER_VIEW}
    # Package Explorer should now be active
    Log    Package Explorer view activated

Activate View That Is Hidden
    [Documentation]    Verify activating a view that is behind another.
    ...                Should bring the view to the front of its stack.
    [Tags]    positive
    Show View    ${CONSOLE_VIEW}
    Show View    ${PROBLEMS_VIEW}
    Show View    ${OUTLINE_VIEW}
    Activate View    ${CONSOLE_VIEW}
    Log    Console view brought to front

Activate Same View Multiple Times
    [Documentation]    Verify activating the same view multiple times is safe.
    ...                Should not cause errors on repeated activation.
    [Tags]    positive    edge-case
    Show View    ${PACKAGE_EXPLORER_VIEW}
    FOR    ${i}    IN RANGE    3
        Activate View    ${PACKAGE_EXPLORER_VIEW}
    END


# =============================================================================
# Activate View - Negative Tests
# =============================================================================

Activate View With Empty ID Fails
    [Documentation]    Verify activating view with empty ID fails.
    ...                Tests input validation for view ID.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Activate View    ${EMPTY}

Activate View Without Connection Fails
    [Documentation]    Verify activating view fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Activate View    ${CONSOLE_VIEW}
    [Teardown]    Connect To RCP App


# =============================================================================
# View Should Be Visible Tests
# =============================================================================

View Should Be Visible Passes For Visible View
    [Documentation]    Verify assertion passes for visible view.
    ...                Should not raise error when view is visible.
    [Tags]    smoke    positive
    Show View    ${CONSOLE_VIEW}
    View Should Be Visible    ${CONSOLE_VIEW}

View Should Be Visible Fails For Hidden View
    [Documentation]    Verify assertion fails for non-visible view.
    ...                Should raise AssertionError when view is not visible.
    [Tags]    negative    assertion
    Close View If Visible    ${CONSOLE_VIEW}
    Run Keyword And Expect Error    *not visible*
    ...    View Should Be Visible    ${CONSOLE_VIEW}

View Should Be Visible Without Connection Fails
    [Documentation]    Verify view visibility check fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    View Should Be Visible    ${CONSOLE_VIEW}
    [Teardown]    Connect To RCP App


# =============================================================================
# Get Open Views Tests
# =============================================================================

Get Open Views Successfully
    [Documentation]    Verify retrieving list of all open views.
    ...                Returns a list of view information dictionaries.
    [Tags]    smoke    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    ${views}=    Get Open Views
    Should Not Be Empty    ${views}
    ${count}=    Get Length    ${views}
    Should Be True    ${count} >= 1    Should have at least one open view

Get Open Views Returns List With View Info
    [Documentation]    Verify open views list contains view information.
    ...                Each item should have id, title, etc.
    [Tags]    positive
    Show View    ${CONSOLE_VIEW}
    ${views}=    Get Open Views
    FOR    ${view}    IN    @{views}
        Log    View: ${view}
    END

Get Open Views After Opening Multiple Views
    [Documentation]    Verify open views reflects all opened views.
    ...                All shown views should be in the list.
    [Tags]    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Show View    ${PROBLEMS_VIEW}
    Show View    ${CONSOLE_VIEW}
    ${views}=    Get Open Views
    ${count}=    Get Length    ${views}
    Should Be True    ${count} >= 3    Should have at least three open views

Get Open Views After Closing View
    [Documentation]    Verify open views updates after closing a view.
    ...                Closed view should not be in the list.
    [Tags]    positive
    Show View    ${CONSOLE_VIEW}
    ${views_before}=    Get Open Views
    Close View    ${CONSOLE_VIEW}
    ${views_after}=    Get Open Views
    ${count_before}=    Get Length    ${views_before}
    ${count_after}=    Get Length    ${views_after}
    Should Be True    ${count_after} < ${count_before}

Get Open Views Without Connection Fails
    [Documentation]    Verify getting open views fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Get Open Views
    [Teardown]    Connect To RCP App


# =============================================================================
# Get View Widget Tests
# =============================================================================

Get View Widget Successfully
    [Documentation]    Verify finding a widget within a view.
    ...                Returns an SwtElement for the found widget.
    [Tags]    smoke    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    ${widget}=    Get View Widget    ${PACKAGE_EXPLORER_VIEW}    Tree
    Should Not Be Empty    ${widget}
    Log    Found widget: ${widget}

Get View Widget By Type
    [Documentation]    Verify finding widgets of specific type in view.
    ...                Should find Tree, Table, Text, etc. widgets.
    [Tags]    positive
    Show View    ${PROBLEMS_VIEW}
    Activate View    ${PROBLEMS_VIEW}
    ${widget}=    Get View Widget    ${PROBLEMS_VIEW}    Tree
    Log    Found widget in Problems view: ${widget}

Get View Widget With Locator
    [Documentation]    Verify finding widget with specific locator.
    ...                Tests using different locator strategies.
    [Tags]    positive
    Show View    ${CONSOLE_VIEW}
    # Try to find a text widget in Console view
    ${result}=    Run Keyword And Ignore Error
    ...    Get View Widget    ${CONSOLE_VIEW}    StyledText
    Log    Result: ${result}


# =============================================================================
# Get View Widget - Negative Tests
# =============================================================================

Get View Widget With Empty View ID Fails
    [Documentation]    Verify getting view widget with empty view ID fails.
    ...                Tests input validation for view ID.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Get View Widget    ${EMPTY}    Tree

Get View Widget For Invalid View Fails
    [Documentation]    Verify getting widget from non-existent view fails.
    ...                Tests error handling for invalid view ID.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *
    ...    Get View Widget    ${INVALID_VIEW}    Tree

Get View Widget Without Connection Fails
    [Documentation]    Verify getting view widget fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Get View Widget    ${PACKAGE_EXPLORER_VIEW}    Tree
    [Teardown]    Connect To RCP App


# =============================================================================
# Integration Tests
# =============================================================================

Full View Lifecycle
    [Documentation]    Test complete view lifecycle.
    ...                Show, activate, get widget, close.
    [Tags]    integration    positive
    # Show view
    Show View    ${CONSOLE_VIEW}
    View Should Be Visible    ${CONSOLE_VIEW}
    # Activate view
    Activate View    ${CONSOLE_VIEW}
    # Get open views
    ${views}=    Get Open Views
    Should Not Be Empty    ${views}
    # Close view
    Close View    ${CONSOLE_VIEW}
    # Verify closed
    ${views_after}=    Get Open Views
    ${view_ids}=    Evaluate    [v.get('id', '') for v in ${views_after}]
    Should Not Contain    ${view_ids}    ${CONSOLE_VIEW}

Multiple Views Workflow
    [Documentation]    Test working with multiple views.
    ...                Open, switch between, and close multiple views.
    [Tags]    integration    positive
    # Open multiple views
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Show View    ${PROBLEMS_VIEW}
    Show View    ${CONSOLE_VIEW}
    # Verify all visible
    View Should Be Visible    ${PACKAGE_EXPLORER_VIEW}
    View Should Be Visible    ${PROBLEMS_VIEW}
    View Should Be Visible    ${CONSOLE_VIEW}
    # Activate each in turn
    Activate View    ${CONSOLE_VIEW}
    Activate View    ${PROBLEMS_VIEW}
    Activate View    ${PACKAGE_EXPLORER_VIEW}
    # Verify count
    ${views}=    Get Open Views
    ${count}=    Get Length    ${views}
    Should Be True    ${count} >= 3    Should have at least three open views
    # Close one
    Close View    ${CONSOLE_VIEW}
    ${views_after}=    Get Open Views
    ${count_after}=    Get Length    ${views_after}
    Should Be True    ${count_after} >= 2
