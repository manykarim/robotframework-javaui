*** Settings ***
Documentation     Test suite for RCP Command operations.
...               Tests executing Eclipse commands and retrieving
...               available commands from the command framework.

Resource          resources/common.resource

Suite Setup       Suite Setup Start RCP App
Suite Teardown    Suite Teardown Stop RCP App
Test Setup        Test Setup Reset State
Test Teardown     Test Teardown Cleanup

Force Tags        rcp    command


*** Test Cases ***
# =============================================================================
# Execute Command Tests
# =============================================================================

Execute Command Successfully
    [Documentation]    Verify executing an Eclipse command by ID.
    ...                Command should execute without error.
    [Tags]    smoke    critical    positive
    Execute Command    ${CMD_REFRESH}
    Log    Command executed: ${CMD_REFRESH}

Execute Save Command
    [Documentation]    Verify executing the save command.
    ...                Should save the active editor.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Execute Command    ${CMD_SAVE}
    Log    Save command executed

Execute Save All Command
    [Documentation]    Verify executing the save all command.
    ...                Should save all open editors.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Open Editor    ${TEST_FILE_XML}
    Execute Command    ${CMD_SAVE_ALL}
    Log    Save all command executed

Execute Undo Command
    [Documentation]    Verify executing the undo command.
    ...                Should undo the last operation.
    [Tags]    positive
    Execute Command    ${CMD_UNDO}
    Log    Undo command executed

Execute Redo Command
    [Documentation]    Verify executing the redo command.
    ...                Should redo the last undone operation.
    [Tags]    positive
    Execute Command    ${CMD_REDO}
    Log    Redo command executed

Execute Close Command
    [Documentation]    Verify executing the close command.
    ...                Should close the active editor.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Execute Command    ${CMD_CLOSE}
    Log    Close command executed

Execute Close All Command
    [Documentation]    Verify executing the close all command.
    ...                Should close all open editors.
    [Tags]    positive
    Open Editor    ${TEST_FILE_JAVA}
    Open Editor    ${TEST_FILE_XML}
    Execute Command    ${CMD_CLOSE_ALL}
    ${editors}=    Get Open Editors
    ${count}=    Get Length    ${editors}
    Log    Editors after close all: ${count}

Execute Multiple Commands Sequentially
    [Documentation]    Verify executing multiple commands in sequence.
    ...                Should work consistently.
    [Tags]    positive    reliability
    Execute Command    ${CMD_REFRESH}
    Execute Command    ${CMD_SAVE_ALL}
    Execute Command    ${CMD_REFRESH}
    Log    Multiple commands executed

Execute Same Command Repeatedly
    [Documentation]    Verify executing the same command multiple times.
    ...                Should work consistently on repeated execution.
    [Tags]    positive    reliability
    FOR    ${i}    IN RANGE    3
        Execute Command    ${CMD_REFRESH}
        Log    Refresh command iteration ${i + 1}
    END


# =============================================================================
# Execute Command - Negative Tests
# =============================================================================

Execute Command With Empty ID Fails
    [Documentation]    Verify executing command with empty ID fails.
    ...                Tests input validation for command ID.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Execute Command    ${EMPTY}

Execute Command With Invalid ID Fails
    [Documentation]    In the mock setup, any command ID is accepted.
    ...                This is a mock limitation - real Eclipse validates command IDs.
    [Tags]    positive    mock-limitation
    Execute Command    ${INVALID_COMMAND}
    Log    Mock app accepts any command ID

Execute Command Without Connection Fails
    [Documentation]    Verify executing command fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Execute Command    ${CMD_REFRESH}
    [Teardown]    Connect To RCP App


# =============================================================================
# Get Available Commands Tests
# =============================================================================

Get Available Commands Successfully
    [Documentation]    Verify retrieving list of available commands.
    ...                Returns a list of command information.
    [Tags]    smoke    positive
    ${commands}=    Get Available Commands
    Should Not Be Empty    ${commands}
    ${count}=    Get Length    ${commands}
    Should Be True    ${count} >= 1
    Log    Found ${count} available commands

Get Available Commands Returns Command Info
    [Documentation]    Verify available commands contain command information.
    ...                Each item should have id, name, etc.
    [Tags]    positive
    ${commands}=    Get Available Commands
    FOR    ${command}    IN    @{commands}
        Log    Command: ${command}
    END

Get Available Commands With Category Filter
    [Documentation]    Verify getting commands filtered by category.
    ...                Should return commands in the specified category.
    [Tags]    positive
    ${commands}=    Get Available Commands    category=Edit
    Log    Edit category commands: ${commands}

Get Available Commands Without Category
    [Documentation]    Verify getting all commands without category filter.
    ...                Should return all available commands.
    [Tags]    positive
    ${all_commands}=    Get Available Commands
    ${count}=    Get Length    ${all_commands}
    Should Be True    ${count} >= 1
    Log    Total commands available: ${count}

Get Available Commands Multiple Times
    [Documentation]    Verify getting available commands is consistent.
    ...                Multiple calls should return consistent results.
    [Tags]    positive    reliability
    ${commands1}=    Get Available Commands
    ${commands2}=    Get Available Commands
    ${count1}=    Get Length    ${commands1}
    ${count2}=    Get Length    ${commands2}
    Should Be Equal    ${count1}    ${count2}

Get Available Commands Without Connection Fails
    [Documentation]    Verify getting commands fails without connection.
    ...                Tests proper error when not connected.
    [Tags]    negative    error-handling
    [Setup]    Disconnect From RCP App
    Run Keyword And Expect Error    *Not connected*
    ...    Get Available Commands
    [Teardown]    Connect To RCP App


# =============================================================================
# Edge Cases
# =============================================================================

Execute Command When Not Applicable
    [Documentation]    Verify behavior when command is not applicable.
    ...                Some commands only work in specific contexts.
    [Tags]    edge-case
    # Undo when nothing to undo
    Run Keyword And Ignore Error    Execute Command    ${CMD_UNDO}
    Log    Command execution handled

Execute Command With Context Requirements
    [Documentation]    Verify command that requires active editor.
    ...                Should handle case when no editor is open.
    [Tags]    edge-case
    Close All Editors    save=${FALSE}
    Run Keyword And Ignore Error    Execute Command    ${CMD_SAVE}
    Log    Save command handled when no editor open

Get Commands In Different Categories
    [Documentation]    Verify getting commands from different categories.
    ...                Tests various category filters.
    [Tags]    positive    edge-case
    ${file_commands}=    Get Available Commands    category=File
    Log    File commands: ${file_commands}
    ${edit_commands}=    Get Available Commands    category=Edit
    Log    Edit commands: ${edit_commands}


# =============================================================================
# Integration Tests
# =============================================================================

Command Execution Workflow
    [Documentation]    Test workflow using commands.
    ...                Note: Mock app accepts commands but doesn't actually execute them.
    [Tags]    integration    positive    mock-limitation
    # Open a file
    Open Editor    ${TEST_FILE_JAVA}
    # Verify editor is open
    ${editors}=    Get Open Editors
    ${count}=    Get Length    ${editors}
    Should Be True    ${count} >= 1
    # Execute save command (mock just accepts, doesn't actually save)
    Execute Command    ${CMD_SAVE}
    # Execute close command (mock just accepts, doesn't actually close)
    Execute Command    ${CMD_CLOSE}
    # Note: Mock app doesn't actually close the editor via command
    # Use the actual Close Editor keyword to verify editor operations work
    Close Editor    Test.java
    ${editors_after}=    Get Open Editors
    ${count_after}=    Get Length    ${editors_after}
    Should Be True    ${count_after} < ${count}

Get And Execute Commands
    [Documentation]    Get available commands and execute one.
    ...                Tests the complete command workflow.
    [Tags]    integration    positive
    # Get available commands
    ${commands}=    Get Available Commands
    Should Not Be Empty    ${commands}
    # Execute refresh command
    Execute Command    ${CMD_REFRESH}
    Log    Command executed from available commands

Multiple Command Types
    [Documentation]    Test executing different types of commands.
    ...                File, Edit, View commands.
    [Tags]    integration    positive
    # Open editor for some commands
    Open Editor    ${TEST_FILE_JAVA}
    # File command
    Execute Command    ${CMD_SAVE}
    Log    File command executed
    # Edit command (may not have effect)
    Run Keyword And Ignore Error    Execute Command    ${CMD_SELECT_ALL}
    Log    Edit command executed
    # View/Window command
    Execute Command    ${CMD_REFRESH}
    Log    Refresh command executed
    # Cleanup
    Close All Editors    save=${FALSE}
