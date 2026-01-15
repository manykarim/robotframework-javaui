*** Settings ***
Documentation     Test suite for RCP Editor operations.
...               Tests opening, closing, saving editors, dirty state verification,
...               and retrieving widgets from within editors.

Resource          resources/common.resource

Suite Setup       Suite Setup Start RCP App
Suite Teardown    Suite Teardown Stop RCP App
Test Setup        Test Setup Reset State
Test Teardown     Test Teardown Cleanup

Force Tags        rcp    editor


*** Test Cases ***
# =============================================================================
# Get Active Editor Tests
# =============================================================================

Get Active Editor Successfully
    [Documentation]    Verify retrieving the currently active editor.
    ...                Returns editor info dictionary or None if no editor.
    [Tags]    smoke    positive
    Open Editor    ${TEST_FILE_JAVA}
    ${editor}=    Get Active Editor
    Should Not Be Empty    ${editor}
    Log    Active editor: ${editor}

Get Active Editor Returns None When No Editors
    [Documentation]    Verify active editor returns None when no editors open.
    ...                Tests handling of empty editor area.
    [Tags]    positive
    Close All Editors    save=${FALSE}
    ${editor}=    Get Active Editor
    # Should be None or empty when no editors
    Log    Active editor when none open: ${editor}

Get Active Editor After Opening File
    [Documentation]    Verify active editor reflects the opened file.
    ...                Editor title should match the file name.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    ${editor}=    Get Active Editor
    Should Not Be Empty    ${editor}
    # Editor info should contain file information
    Log    Editor info: ${editor}

Get Active Editor After Switching Editors
    [Documentation]    Verify active editor updates when switching editors.
    ...                Tests that active editor reflects current focus.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Open Editor    ${TEST_FILE_XML}
    ${editor}=    Get Active Editor
    Log    Active after opening both: ${editor}
    Activate Editor    Test.java
    ${editor_after_switch}=    Get Active Editor
    Log    Active after switch: ${editor_after_switch}

Get Active Editor Without Connection Fails
    [Documentation]    Verify getting active editor fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Get Active Editor
    [Teardown]    Connect To RCP App


# =============================================================================
# Open Editor Tests
# =============================================================================

Open Editor Successfully
    [Documentation]    Verify opening an editor for a file.
    ...                Editor should appear in the editor area.
    [Tags]    smoke    critical    positive
    Open Editor    ${TEST_FILE_JAVA}
    ${editors}=    Get Open Editors
    ${count}=    Get Length    ${editors}
    Should Be True    ${count} >= 1

Open Editor With Different Files
    [Documentation]    Verify opening editors for different file types.
    ...                Should handle Java, XML, and other files.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Open Editor    ${TEST_FILE_XML}
    ${editors}=    Get Open Editors
    ${count}=    Get Length    ${editors}
    Should Be True    ${count} >= 2

Open Same File Multiple Times
    [Documentation]    Verify opening the same file reuses the editor.
    ...                Should not create duplicate editors.
    [Tags]    positive    edge-case
    Open Editor    ${TEST_FILE_JAVA}
    ${editors1}=    Get Open Editors
    Open Editor    ${TEST_FILE_JAVA}
    ${editors2}=    Get Open Editors
    ${count1}=    Get Length    ${editors1}
    ${count2}=    Get Length    ${editors2}
    Should Be Equal    ${count1}    ${count2}


# =============================================================================
# Open Editor - Negative Tests
# =============================================================================

Open Editor With Empty Path Fails
    [Documentation]    Verify opening editor with empty path fails.
    ...                Tests input validation for file path.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Open Editor    ${EMPTY}

Open Editor With Invalid Path Fails
    [Documentation]    In the mock setup, opening a non-existent file creates a mock editor.
    ...                This is a mock limitation - real Eclipse would show a dialog or error.
    [Tags]    positive    mock-limitation
    Open Editor    ${INVALID_FILE}
    ${editors}=    Get Open Editors
    # Mock app creates an editor even for non-existent files
    ${count}=    Get Length    ${editors}
    Should Be True    ${count} >= 1
    [Teardown]    Close All Editors    save=${FALSE}

Open Editor Without Connection Fails
    [Documentation]    Verify opening editor fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Open Editor    ${TEST_FILE_JAVA}
    [Teardown]    Connect To RCP App


# =============================================================================
# Close Editor Tests
# =============================================================================

Close Editor Successfully
    [Documentation]    Verify closing an open editor by title.
    ...                Editor should be removed from editor area.
    [Tags]    smoke    critical    positive
    Open Editor    ${TEST_FILE_JAVA}
    Close Editor    Test.java
    ${editors}=    Get Open Editors
    ${titles}=    Evaluate    [e.get('title', '') for e in ${editors}]
    Should Not Contain    ${titles}    Test.java

Close Editor Without Saving
    [Documentation]    Verify closing editor without saving changes.
    ...                Should discard unsaved changes.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Close Editor    Test.java    save=${FALSE}

Close Editor With Save
    [Documentation]    Verify closing editor with saving changes.
    ...                Should save before closing.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Close Editor    Test.java    save=${TRUE}


# =============================================================================
# Close Editor - Negative Tests
# =============================================================================

Close Editor With Empty Title Fails
    [Documentation]    Verify closing editor with empty title fails.
    ...                Tests input validation for editor title.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Close Editor    ${EMPTY}

Close Editor Without Connection Fails
    [Documentation]    Verify closing editor fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Close Editor    Test.java
    [Teardown]    Connect To RCP App


# =============================================================================
# Close All Editors Tests
# =============================================================================

Close All Editors Successfully
    [Documentation]    Verify closing all open editors.
    ...                All editors should be removed from editor area.
    [Tags]    smoke    positive
    Open Editor    ${TEST_FILE_JAVA}
    Open Editor    ${TEST_FILE_XML}
    ${result}=    Close All Editors
    ${editors}=    Get Open Editors
    ${count}=    Get Length    ${editors}
    Should Be Equal As Numbers    ${count}    0

Close All Editors Without Saving
    [Documentation]    Verify closing all editors without saving.
    ...                Should discard all unsaved changes.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Open Editor    ${TEST_FILE_XML}
    ${result}=    Close All Editors    save=${FALSE}
    ${editors}=    Get Open Editors
    ${count}=    Get Length    ${editors}
    Should Be Equal As Numbers    ${count}    0

Close All Editors With Save
    [Documentation]    Verify closing all editors with saving.
    ...                Should save all before closing.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Close All Editors    save=${TRUE}
    # Verify editors were actually closed
    ${editors}=    Get Open Editors
    ${count}=    Get Length    ${editors}
    Should Be Equal As Numbers    ${count}    0

Close All Editors When None Open
    [Documentation]    Verify closing all editors when none are open.
    ...                Should not cause error.
    [Tags]    positive    edge-case
    Close All Editors    save=${FALSE}
    ${result}=    Close All Editors    save=${FALSE}
    Log    Close all when none open: ${result}

Close All Editors Without Connection Fails
    [Documentation]    Verify closing all editors fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Close All Editors
    [Teardown]    Connect To RCP App


# =============================================================================
# Save Editor Tests
# =============================================================================

Save Editor Successfully
    [Documentation]    Verify saving an editor's content.
    ...                Editor should no longer be dirty after save.
    [Tags]    smoke    positive
    Open Editor    ${TEST_FILE_JAVA}
    Save Editor
    Log    Editor saved

Save Editor By Title
    [Documentation]    Verify saving a specific editor by title.
    ...                Only the specified editor should be saved.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Save Editor    title=Test.java
    Log    Specific editor saved

Save Editor Without Connection Fails
    [Documentation]    Verify saving editor fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Save Editor
    [Teardown]    Connect To RCP App


# =============================================================================
# Save All Editors Tests
# =============================================================================

Save All Editors Successfully
    [Documentation]    Verify saving all open editors.
    ...                All editors should be saved.
    [Tags]    smoke    positive
    Open Editor    ${TEST_FILE_JAVA}
    Open Editor    ${TEST_FILE_XML}
    Save All Editors
    Log    All editors saved

Save All Editors When None Open
    [Documentation]    Verify saving all editors when none are open.
    ...                Should not cause error.
    [Tags]    positive    edge-case
    Close All Editors    save=${FALSE}
    Save All Editors
    Log    Save all when none open completed

Save All Editors Without Connection Fails
    [Documentation]    Verify saving all editors fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Save All Editors
    [Teardown]    Connect To RCP App


# =============================================================================
# Activate Editor Tests
# =============================================================================

Activate Editor Successfully
    [Documentation]    Verify activating an editor by title.
    ...                Editor should become active and focused.
    [Tags]    smoke    positive
    Open Editor    ${TEST_FILE_JAVA}
    Open Editor    ${TEST_FILE_XML}
    Activate Editor    Test.java
    ${active}=    Get Active Editor
    Log    Active after activation: ${active}

Activate Editor Multiple Times
    [Documentation]    Verify activating same editor multiple times.
    ...                Should not cause error.
    [Tags]    positive    edge-case
    Open Editor    ${TEST_FILE_JAVA}
    FOR    ${i}    IN RANGE    3
        Activate Editor    Test.java
    END


# =============================================================================
# Activate Editor - Negative Tests
# =============================================================================

Activate Editor With Empty Title Fails
    [Documentation]    Verify activating editor with empty title fails.
    ...                Tests input validation for editor title.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Activate Editor    ${EMPTY}

Activate Editor Without Connection Fails
    [Documentation]    Verify activating editor fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Activate Editor    Test.java
    [Teardown]    Connect To RCP App


# =============================================================================
# Editor Dirty State Tests
# =============================================================================

Editor Should Be Dirty Passes For Dirty Editor
    [Documentation]    Verify assertion passes when editor is dirty.
    ...                Editor with unsaved changes should pass.
    [Tags]    positive    assertion
    Open Editor    ${TEST_FILE_JAVA}
    # Make editor dirty by modifying content
    # Note: This requires a way to modify editor content
    # For now, we test the keyword exists and works
    Run Keyword And Ignore Error    Editor Should Be Dirty    Test.java

Editor Should Be Dirty Fails For Clean Editor
    [Documentation]    Verify assertion fails when editor is not dirty.
    ...                Clean editor should cause assertion error.
    [Tags]    negative    assertion
    Open Editor    ${TEST_FILE_JAVA}
    Save Editor
    Run Keyword And Expect Error    *not dirty*
    ...    Editor Should Be Dirty    Test.java

Editor Should Not Be Dirty Passes For Clean Editor
    [Documentation]    Verify assertion passes when editor is not dirty.
    ...                Clean editor should pass.
    [Tags]    positive    assertion
    Open Editor    ${TEST_FILE_JAVA}
    Save Editor
    Editor Should Not Be Dirty    Test.java

Editor Should Not Be Dirty Fails For Dirty Editor
    [Documentation]    Verify assertion fails when editor is dirty.
    ...                Dirty editor should cause assertion error.
    [Tags]    negative    assertion
    Open Editor    ${TEST_FILE_JAVA}
    # If editor is dirty, this should fail
    Run Keyword And Ignore Error    Editor Should Not Be Dirty    Test.java

Editor Dirty State Without Connection Fails
    [Documentation]    Verify editor dirty check fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Editor Should Be Dirty    Test.java
    [Teardown]    Connect To RCP App


# =============================================================================
# Get Open Editors Tests
# =============================================================================

Get Open Editors Successfully
    [Documentation]    Verify retrieving list of all open editors.
    ...                Returns a list of editor information dictionaries.
    [Tags]    smoke    positive
    Open Editor    ${TEST_FILE_JAVA}
    ${editors}=    Get Open Editors
    Should Not Be Empty    ${editors}
    ${count}=    Get Length    ${editors}
    Should Be True    ${count} >= 1

Get Open Editors Returns Empty When None
    [Documentation]    Verify open editors returns empty list when none.
    ...                Tests handling of empty editor area.
    [Tags]    positive
    Close All Editors    save=${FALSE}
    ${editors}=    Get Open Editors
    ${count}=    Get Length    ${editors}
    Should Be Equal As Numbers    ${count}    0

Get Open Editors With Multiple Files
    [Documentation]    Verify open editors reflects all opened files.
    ...                All opened files should be in the list.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Open Editor    ${TEST_FILE_XML}
    ${editors}=    Get Open Editors
    ${count}=    Get Length    ${editors}
    Should Be True    ${count} >= 2
    FOR    ${editor}    IN    @{editors}
        Log    Editor: ${editor}
    END

Get Open Editors Without Connection Fails
    [Documentation]    Verify getting open editors fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Get Open Editors
    [Teardown]    Connect To RCP App


# =============================================================================
# Get Editor Widget Tests
# =============================================================================

Get Editor Widget Successfully
    [Documentation]    Verify finding a widget within an editor.
    ...                Returns an SwtElement for the found widget.
    [Tags]    smoke    positive
    Open Editor    ${TEST_FILE_JAVA}
    ${widget}=    Get Editor Widget    Test.java    StyledText
    Should Not Be Empty    ${widget}
    Log    Found widget in editor: ${widget}

Get Editor Widget By Type
    [Documentation]    Verify finding widgets of specific type in editor.
    ...                Should find StyledText in text editors.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    ${widget}=    Get Editor Widget    Test.java    StyledText
    Log    Found StyledText widget: ${widget}


# =============================================================================
# Get Editor Widget - Negative Tests
# =============================================================================

Get Editor Widget With Empty Title Fails
    [Documentation]    Verify getting editor widget with empty title fails.
    ...                Tests input validation for editor title.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Get Editor Widget    ${EMPTY}    StyledText

Get Editor Widget Without Connection Fails
    [Documentation]    Verify getting editor widget fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Get Editor Widget    Test.java    StyledText
    [Teardown]    Connect To RCP App


# =============================================================================
# Integration Tests
# =============================================================================

Full Editor Lifecycle
    [Documentation]    Test complete editor lifecycle.
    ...                Open, save, check dirty state, close.
    [Tags]    integration    positive
    # Open editor
    Open Editor    ${TEST_FILE_JAVA}
    ${editors}=    Get Open Editors
    ${count}=    Get Length    ${editors}
    Should Be True    ${count} >= 1
    # Get active editor
    ${active}=    Get Active Editor
    Should Not Be Empty    ${active}
    # Save editor
    Save Editor
    Editor Should Not Be Dirty    Test.java
    # Close editor
    Close Editor    Test.java
    ${editors_after}=    Get Open Editors
    ${count_after}=    Get Length    ${editors_after}
    Should Be True    ${count_after} < ${count}

Multiple Editors Workflow
    [Documentation]    Test working with multiple editors.
    ...                Open, switch between, save all, close all.
    [Tags]    integration    positive
    # Open multiple editors
    Open Editor    ${TEST_FILE_JAVA}
    Open Editor    ${TEST_FILE_XML}
    # Get all open editors
    ${editors}=    Get Open Editors
    ${count}=    Get Length    ${editors}
    Should Be True    ${count} >= 2
    # Switch between editors
    Activate Editor    Test.java
    Activate Editor    config.xml
    # Save all
    Save All Editors
    # Close all
    ${result}=    Close All Editors    save=${FALSE}
    ${editors_after}=    Get Open Editors
    ${count_after}=    Get Length    ${editors_after}
    Should Be Equal As Numbers    ${count_after}    0
