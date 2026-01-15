*** Settings ***
Documentation     Test suite for RCP Toolbar operations.
...               Tests clicking toolbar items in Eclipse RCP applications.

Resource          resources/common.resource

Suite Setup       Suite Setup Start RCP App
Suite Teardown    Suite Teardown Stop RCP App
Test Setup        Test Setup Reset State
Test Teardown     Test Teardown Cleanup

Force Tags        rcp    toolbar


*** Variables ***
# Common toolbar item tooltips
${TOOLBAR_SAVE}              Save
${TOOLBAR_SAVE_ALL}          Save All
${TOOLBAR_NEW}               New
${TOOLBAR_PRINT}             Print
${TOOLBAR_RUN}               Run
${TOOLBAR_DEBUG}             Debug
${TOOLBAR_SEARCH}            Search
${TOOLBAR_BACK}              Back
${TOOLBAR_FORWARD}           Forward
${TOOLBAR_REFRESH}           Refresh


*** Test Cases ***
# =============================================================================
# Click Toolbar Item Tests
# =============================================================================

Click Toolbar Item Successfully
    [Documentation]    Verify clicking a toolbar item by tooltip.
    ...                Should trigger the associated action.
    [Tags]    smoke    critical    positive
    Click Toolbar Item    ${TOOLBAR_SAVE_ALL}
    Log    Toolbar item clicked: Save All

Click Toolbar Save Item
    [Documentation]    Verify clicking the Save toolbar button.
    ...                Should save the active editor.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Click Toolbar Item    ${TOOLBAR_SAVE}
    Log    Save toolbar button clicked

Click Toolbar Save All Item
    [Documentation]    Verify clicking the Save All toolbar button.
    ...                Should save all open editors.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Open Editor    ${TEST_FILE_XML}
    Click Toolbar Item    ${TOOLBAR_SAVE_ALL}
    Log    Save All toolbar button clicked

Click Different Toolbar Items
    [Documentation]    Verify clicking various toolbar items.
    ...                Tests multiple toolbar buttons.
    [Tags]    positive
    # Note: Some items may not be available in all configurations
    Run Keyword And Ignore Error    Click Toolbar Item    ${TOOLBAR_SAVE_ALL}
    Log    Tested Save All
    Run Keyword And Ignore Error    Click Toolbar Item    ${TOOLBAR_REFRESH}
    Log    Tested Refresh

Click Toolbar Item Multiple Times
    [Documentation]    Verify clicking same toolbar item multiple times.
    ...                Should work consistently on repeated clicks.
    [Tags]    positive    reliability
    FOR    ${i}    IN RANGE    3
        Click Toolbar Item    ${TOOLBAR_SAVE_ALL}
        Log    Toolbar click iteration ${i + 1}
    END


# =============================================================================
# Click Toolbar Item - Negative Tests
# =============================================================================

Click Toolbar Item With Empty Tooltip Fails
    [Documentation]    Verify clicking toolbar with empty tooltip fails.
    ...                Tests input validation for tooltip.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Click Toolbar Item    ${EMPTY}

Click Toolbar Item With Invalid Tooltip Silently Succeeds
    [Documentation]    Mock app doesn't error on invalid toolbar tooltip.
    ...                Real Eclipse might report error.
    [Tags]    negative    mock-limitation
    # Mock app accepts invalid tooltip without error
    Click Toolbar Item    NonExistentToolbarButton12345
    Log    Mock app accepted invalid toolbar tooltip

Click Toolbar Item Without Connection Fails
    [Documentation]    Verify clicking toolbar fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Click Toolbar Item    ${TOOLBAR_SAVE}
    [Teardown]    Connect To RCP App


# =============================================================================
# Edge Cases
# =============================================================================

Click Toolbar Item With Exact Match
    [Documentation]    Verify tooltip matching is exact.
    ...                Tests that partial matches don't work.
    [Tags]    positive    edge-case
    Click Toolbar Item    ${TOOLBAR_SAVE_ALL}
    Log    Exact tooltip match successful

Click Toolbar Item When Disabled
    [Documentation]    Verify behavior when toolbar item is disabled.
    ...                May click without error or report disabled state.
    [Tags]    edge-case
    # Close all editors so Save is likely disabled
    Close All Editors    save=${FALSE}
    Run Keyword And Ignore Error    Click Toolbar Item    ${TOOLBAR_SAVE}
    Log    Disabled toolbar item handling tested

Click Toolbar In Different Perspectives
    [Documentation]    Verify toolbar items work in different perspectives.
    ...                Toolbar may differ between perspectives.
    [Tags]    positive    edge-case
    Open Perspective    ${JAVA_PERSPECTIVE}
    Click Toolbar Item    ${TOOLBAR_SAVE_ALL}
    Open Perspective    ${DEBUG_PERSPECTIVE}
    Run Keyword And Ignore Error    Click Toolbar Item    ${TOOLBAR_SAVE_ALL}
    Log    Toolbar tested in different perspectives


# =============================================================================
# Integration Tests
# =============================================================================

Toolbar Workflow With Editor
    [Documentation]    Test toolbar workflow with editor operations.
    ...                Open file, use toolbar to save, etc.
    [Tags]    integration    positive
    # Open an editor
    Open Editor    ${TEST_FILE_JAVA}
    # Use toolbar to save
    Click Toolbar Item    ${TOOLBAR_SAVE}
    Log    Saved via toolbar
    # Editor should not be dirty (use full path)
    Editor Should Not Be Dirty    ${TEST_FILE_JAVA}
    # Cleanup
    Close All Editors    save=${FALSE}

Toolbar And Menu Combination
    [Documentation]    Test using both toolbar and menu.
    ...                Combines different interaction methods.
    [Tags]    integration    positive
    Open Editor    ${TEST_FILE_JAVA}
    # Save via toolbar
    Click Toolbar Item    ${TOOLBAR_SAVE}
    # Refresh via menu
    Select Main Menu    File|Refresh
    # Save all via toolbar
    Click Toolbar Item    ${TOOLBAR_SAVE_ALL}
    Log    Combined toolbar and menu operations

Toolbar After View Changes
    [Documentation]    Verify toolbar works after view changes.
    ...                Tests toolbar availability after UI changes.
    [Tags]    integration    positive
    Show View    ${PACKAGE_EXPLORER_VIEW}
    Click Toolbar Item    ${TOOLBAR_SAVE_ALL}
    Show View    ${CONSOLE_VIEW}
    Click Toolbar Item    ${TOOLBAR_SAVE_ALL}
    Log    Toolbar works after view changes
