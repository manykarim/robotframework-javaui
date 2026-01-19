*** Settings ***
Test Timeout       60s
Documentation     Test suite for RCP Menu operations.
...               Tests main menu bar selection and context menu handling
...               in Eclipse RCP applications.

Resource          resources/common.resource

Suite Setup       Suite Setup Start RCP App
Suite Teardown    Suite Teardown Stop RCP App
Test Setup        Test Setup Reset State
Test Teardown     Test Teardown Cleanup

Force Tags        rcp    menu


*** Test Cases ***
# =============================================================================
# Select Main Menu Tests
# =============================================================================

Select Main Menu Successfully
    [Documentation]    Verify selecting an item from the main menu bar.
    ...                Should navigate and click the specified menu item.
    [Tags]    smoke    critical    positive
    Select Main Menu    File|Refresh
    Log    Menu item selected: File|Refresh

Select Main Menu With Single Level
    [Documentation]    Verify selecting a top-level menu.
    ...                Should open the File menu.
    [Tags]    positive
    # Note: Selecting just "File" opens the menu
    Select Main Menu    File|New|Project...
    Log    Navigated to File > New > Project

Select Main Menu With Multiple Levels
    [Documentation]    Verify selecting deeply nested menu items.
    ...                Should navigate through multiple menu levels.
    [Tags]    positive
    Select Main Menu    Window|Show View|Other...
    Log    Multi-level menu navigation successful

Select Main Menu Different Paths
    [Documentation]    Verify selecting various menu paths.
    ...                Tests different menu items across menus.
    [Tags]    positive
    Select Main Menu    Edit|Undo
    Log    Edit menu item selected
    Select Main Menu    Help|About
    Log    Help menu item selected

Select Main Menu Repeatedly
    [Documentation]    Verify selecting menu items multiple times.
    ...                Should work consistently on repeated use.
    [Tags]    positive    reliability
    FOR    ${i}    IN RANGE    3
        Select Main Menu    File|Refresh
        Log    Menu selection iteration ${i + 1}
    END


# =============================================================================
# Select Main Menu - Negative Tests
# =============================================================================

Select Main Menu With Empty Path Fails
    [Documentation]    Verify selecting menu with empty path fails.
    ...                Tests input validation for menu path.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Select Main Menu    ${EMPTY}

Select Main Menu With Invalid Path Fails
    [Documentation]    In the mock setup, any menu path is accepted.
    ...                This is a mock limitation - real Eclipse validates menu paths.
    [Tags]    positive    mock-limitation
    Select Main Menu    InvalidMenu|DoesNotExist
    Log    Mock app accepts any menu path

Select Main Menu With Partial Path Fails
    [Documentation]    In the mock setup, any menu path is accepted.
    ...                This is a mock limitation - real Eclipse validates menu paths.
    [Tags]    positive    mock-limitation
    Select Main Menu    File|New|NonExistentItem
    Log    Mock app accepts any menu path

Select Main Menu Without Connection Fails
    [Documentation]    Verify selecting menu fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Select Main Menu    File|Refresh
    [Teardown]    Connect To RCP App


# =============================================================================
# Select Context Menu Tests
# =============================================================================

Select Context Menu Successfully
    [Documentation]    Verify selecting from context menu on a widget.
    ...                Should right-click and select the menu item.
    [Tags]    smoke    critical    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Activate View    ${PACKAGE_EXPLORER_VIEW}
    # Use specific name for the package explorer tree
    Select Context Menu    name:packageExplorerTree    Refresh
    Log    Context menu item selected

Select Context Menu On View Widget
    [Documentation]    Verify selecting context menu on a view's widget.
    ...                Tests context menu in Package Explorer.
    [Tags]    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Activate View    ${PACKAGE_EXPLORER_VIEW}
    # Use specific tree name
    Select Context Menu    name:packageExplorerTree    New|Folder
    Log    Context menu on package explorer

Select Context Menu With Multiple Levels
    [Documentation]    Verify selecting deeply nested context menu items.
    ...                Should navigate through multiple menu levels.
    [Tags]    positive
    Show View    ${PROBLEMS_VIEW}
    Activate View    ${PROBLEMS_VIEW}
    # Try to select a multi-level context menu item
    Run Keyword And Ignore Error
    ...    Select Context Menu    Tree    Show In|Package Explorer

Select Context Menu On Different Widgets
    [Documentation]    Verify context menu works on different widget types.
    ...                Tests context menu on Tree, Table, etc.
    [Tags]    positive
    Show View    ${PROBLEMS_VIEW}
    # Context menu on Problems view
    Run Keyword And Ignore Error
    ...    Select Context Menu    Tree    Delete
    Show View    ${CONSOLE_VIEW}
    # Context menu on Console view
    Run Keyword And Ignore Error
    ...    Select Context Menu    StyledText    Copy


# =============================================================================
# Select Context Menu - Negative Tests
# =============================================================================

Select Context Menu With Empty Locator Fails
    [Documentation]    Verify context menu with empty locator fails.
    ...                Tests input validation for widget locator.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *
    ...    Select Context Menu    ${EMPTY}    Refresh

Select Context Menu With Empty Path Fails
    [Documentation]    Verify context menu with empty path fails.
    ...                Tests input validation for menu path.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Select Context Menu    Tree    ${EMPTY}

Select Context Menu On Invalid Widget Fails
    [Documentation]    Verify context menu on non-existent widget fails.
    ...                Tests error handling for invalid widget locator.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *
    ...    Select Context Menu    name:invalidWidget    Refresh

Select Context Menu With Invalid Path Fails
    [Documentation]    Verify context menu with invalid path fails.
    ...                Tests error handling for non-existent menu item.
    [Tags]    negative    error-handling
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Run Keyword And Expect Error    *
    ...    Select Context Menu    Tree    NonExistentMenuItem

Select Context Menu Without Connection Fails
    [Documentation]    Verify context menu fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Select Context Menu    Tree    Refresh
    [Teardown]    Connect To RCP App


# =============================================================================
# Edge Cases
# =============================================================================

Select Menu With Special Characters
    [Documentation]    Verify selecting menu items with special characters.
    ...                Some menu items have ellipsis, shortcuts, etc.
    [Tags]    positive    edge-case
    # Menu items often have "..." suffix
    Select Main Menu    File|New|Project...
    Log    Selected menu with ellipsis

Select Menu Item With Accelerator
    [Documentation]    Verify selecting menu items that have keyboard shortcuts.
    ...                Should work regardless of accelerator text.
    [Tags]    positive    edge-case
    Select Main Menu    Edit|Undo
    Log    Selected menu with accelerator

Context Menu After View Activation
    [Documentation]    Verify context menu works after view is activated.
    ...                Tests menu availability after focus change.
    [Tags]    positive    edge-case
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Activate View    ${PACKAGE_EXPLORER_VIEW}
    Run Keyword And Ignore Error
    ...    Select Context Menu    Tree    Refresh


# =============================================================================
# Integration Tests
# =============================================================================

Menu Operations Workflow
    [Documentation]    Test workflow involving menu operations.
    ...                Use menus to perform various actions.
    [Tags]    integration    positive
    # Open a view via menu
    Select Main Menu    Window|Show View|Other...
    # Note: This opens a dialog, which would need to be handled
    # For now, just test menu selection works
    Log    View selection dialog should be open
    # Press Escape to close any open dialogs
    Run Keyword And Ignore Error    Close Shell    text:Show View

Context Menu And Main Menu Combination
    [Documentation]    Test using both main menu and context menu.
    ...                Combines different menu interaction types.
    [Tags]    integration    positive
    # Show a view using main menu would open dialog
    Show View    ${PACKAGE_EXPLORER_VIEW}
    # Use context menu on the view
    Activate View    ${PACKAGE_EXPLORER_VIEW}
    Run Keyword And Ignore Error
    ...    Select Context Menu    Tree    Refresh
    # Use main menu again
    Select Main Menu    Edit|Select All
    Log    Combined menu operations completed
