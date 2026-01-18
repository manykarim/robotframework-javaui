*** Settings ***
Documentation     Common resources for RCP Library tests.
...               Contains shared variables, keywords, and library imports
...               for testing Eclipse RCP application automation.

Library           JavaGui.Rcp
Library           Collections
Library           String
Library           OperatingSystem
Library           Process


*** Variables ***
# Connection Settings
${HOST}                       127.0.0.1
${PORT}                       5680
${APP_NAME}                   mock-rcp
${CONNECTION_TIMEOUT}         60
${DEFAULT_TIMEOUT}            30
${RCP_STDOUT_LOG}             ${OUTPUT DIR}${/}rcp-mock-stdout.log
${RCP_STDERR_LOG}             ${OUTPUT DIR}${/}rcp-mock-stderr.log

# Application Paths
${RCP_MOCK_JAR}               ${CURDIR}/../../../../tests/apps/rcp-mock/target/rcp-mock-test-app-1.0.0-all.jar
${AGENT_JAR}                  ${CURDIR}/../../../../agent/target/robotframework-swing-agent-1.0.0-all.jar
${RCP_PROCESS}                ${NONE}

# Common Perspective IDs
${JAVA_PERSPECTIVE}           org.eclipse.jdt.ui.JavaPerspective
${DEBUG_PERSPECTIVE}          org.eclipse.debug.ui.DebugPerspective
${RESOURCE_PERSPECTIVE}       org.eclipse.ui.resourcePerspective
${TEAM_SYNC_PERSPECTIVE}      org.eclipse.team.ui.TeamSynchronizingPerspective
${GIT_PERSPECTIVE}            org.eclipse.egit.ui.GitRepositoryExploring

# Common View IDs
${PACKAGE_EXPLORER_VIEW}      org.eclipse.jdt.ui.PackageExplorer
${PROJECT_EXPLORER_VIEW}      org.eclipse.ui.navigator.ProjectExplorer
${PROBLEMS_VIEW}              org.eclipse.ui.views.ProblemView
${CONSOLE_VIEW}               org.eclipse.ui.console.ConsoleView
${OUTLINE_VIEW}               org.eclipse.ui.views.ContentOutline
${PROPERTIES_VIEW}            org.eclipse.ui.views.PropertySheet
${ERROR_LOG_VIEW}             org.eclipse.pde.runtime.LogView
${PROGRESS_VIEW}              org.eclipse.ui.views.ProgressView
${SEARCH_VIEW}                org.eclipse.search.ui.views.SearchView
${MARKERS_VIEW}               org.eclipse.ui.views.AllMarkersView
${TASKS_VIEW}                 org.eclipse.ui.views.TaskList
${BOOKMARKS_VIEW}             org.eclipse.ui.views.BookmarkView

# Common Command IDs
${CMD_SAVE}                   org.eclipse.ui.file.save
${CMD_SAVE_ALL}               org.eclipse.ui.file.saveAll
${CMD_UNDO}                   org.eclipse.ui.edit.undo
${CMD_REDO}                   org.eclipse.ui.edit.redo
${CMD_CUT}                    org.eclipse.ui.edit.cut
${CMD_COPY}                   org.eclipse.ui.edit.copy
${CMD_PASTE}                  org.eclipse.ui.edit.paste
${CMD_DELETE}                 org.eclipse.ui.edit.delete
${CMD_SELECT_ALL}             org.eclipse.ui.edit.selectAll
${CMD_FIND}                   org.eclipse.ui.edit.findReplace
${CMD_REFRESH}                org.eclipse.ui.file.refresh
${CMD_CLOSE}                  org.eclipse.ui.file.close
${CMD_CLOSE_ALL}              org.eclipse.ui.file.closeAll
${CMD_NEW_WIZARD}             org.eclipse.ui.newWizard
${CMD_IMPORT}                 org.eclipse.ui.file.import
${CMD_EXPORT}                 org.eclipse.ui.file.export

# Common Menu Paths
${MENU_FILE}                  File
${MENU_EDIT}                  Edit
${MENU_WINDOW}                Window
${MENU_HELP}                  Help
${MENU_NEW_PROJECT}           File|New|Project...
${MENU_PREFERENCES}           Window|Preferences
${MENU_SHOW_VIEW}             Window|Show View|Other...

# Test Data Paths
${TEST_PROJECT}               test-project
${TEST_FILE_JAVA}             /test-project/src/Test.java
${TEST_FILE_XML}              /test-project/resources/config.xml
${TEST_FILE_PROPERTIES}       /test-project/resources/app.properties

# Preference Page Paths
${PREF_GENERAL}               General
${PREF_APPEARANCE}            General|Appearance
${PREF_EDITORS}               General|Editors
${PREF_TEXT_EDITORS}          General|Editors|Text Editors
${PREF_JAVA}                  Java
${PREF_JAVA_COMPILER}         Java|Compiler
${PREF_JAVA_CODE_STYLE}       Java|Code Style

# Invalid/Error Test Data
${INVALID_APP_NAME}           com.invalid.NonExistentApp
${INVALID_HOST}               invalid.host.example.com
${INVALID_PORT}               99999
${INVALID_PERSPECTIVE}        org.invalid.perspective.DoesNotExist
${INVALID_VIEW}               org.invalid.view.DoesNotExist
${INVALID_COMMAND}            org.invalid.command.doesNotExist
${INVALID_FILE}               /nonexistent/path/to/file.txt
${SHORT_TIMEOUT}              1


*** Keywords ***
Connect To RCP App
    [Documentation]    Connect to the RCP application with default settings.
    ...                Uses standard host, port, and timeout values from variables.
    Connect To SWT Application    ${APP_NAME}    ${HOST}    ${PORT}    timeout=${CONNECTION_TIMEOUT}
    Verify Connection Is Active

Connect To RCP App With Custom Settings
    [Documentation]    Connect to RCP application with custom connection settings.
    [Arguments]    ${app}=${APP_NAME}    ${host}=${HOST}    ${port}=${PORT}    ${timeout}=${CONNECTION_TIMEOUT}
    Connect To SWT Application    ${app}    ${host}    ${port}    timeout=${timeout}
    Verify Connection Is Active

Disconnect From RCP App
    [Documentation]    Safely disconnect from the RCP application.
    ...                Checks if connected before attempting disconnect.
    ${is_connected}=    Is Connected
    Run Keyword If    ${is_connected}    Disconnect
    Log    Disconnected from RCP application

Verify Connection Is Active
    [Documentation]    Verify that the connection to the RCP application is active.
    ${connected}=    Is Connected
    Should Be True    ${connected}    Expected to be connected to RCP application

Verify Connection Is Not Active
    [Documentation]    Verify that no connection to RCP application exists.
    ${connected}=    Is Connected
    Should Not Be True    ${connected}    Expected not to be connected to RCP application

Reset Test State
    [Documentation]    Reset the RCP application to a clean test state.
    ...                Closes all editors, resets perspective, and shows default views.
    # Close all open editors without saving
    ${result}=    Close All Editors    save=${FALSE}
    # Reset perspective to default layout
    Reset Perspective
    Log    Test state reset completed

Ensure Perspective Is Open
    [Documentation]    Ensure a specific perspective is open and active.
    [Arguments]    ${perspective_id}
    ${active}=    Get Active Perspective
    Return From Keyword If    '${active}' == '${perspective_id}'
    Open Perspective    ${perspective_id}
    ${new_active}=    Get Active Perspective
    Should Be Equal    ${new_active}    ${perspective_id}

Ensure View Is Visible
    [Documentation]    Ensure a specific view is visible in the workbench.
    [Arguments]    ${view_id}    ${secondary_id}=${NONE}
    Show View    ${view_id}    ${secondary_id}
    View Should Be Visible    ${view_id}

Close View If Visible
    [Documentation]    Close a view if it is currently visible.
    [Arguments]    ${view_id}    ${secondary_id}=${NONE}
    ${views}=    Get Open Views
    ${view_ids}=    Evaluate    [v.get('id', '') for v in ${views}]
    ${is_visible}=    Evaluate    '${view_id}' in ${view_ids}
    Run Keyword If    ${is_visible}    Close View    ${view_id}    ${secondary_id}

Wait For View To Be Visible
    [Documentation]    Wait until a view becomes visible in the workbench.
    [Arguments]    ${view_id}    ${timeout}=${DEFAULT_TIMEOUT}
    Wait Until Keyword Succeeds    ${timeout}s    1s    View Should Be Visible    ${view_id}

Wait For Editor To Open
    [Documentation]    Wait until an editor is open for the specified file.
    [Arguments]    ${file_path}    ${timeout}=${DEFAULT_TIMEOUT}
    Wait Until Keyword Succeeds    ${timeout}s    1s    Editor Should Be Open    ${file_path}

Editor Should Be Open
    [Documentation]    Verify that an editor is open for the specified file.
    [Arguments]    ${file_path}
    ${editors}=    Get Open Editors
    ${file_name}=    Evaluate    "${file_path}".split('/')[-1]
    ${titles}=    Evaluate    [e.get('title', '') for e in ${editors}]
    Should Contain    ${titles}    ${file_name}    Editor for ${file_path} should be open

Get Open Editor Count
    [Documentation]    Get the count of currently open editors.
    ${editors}=    Get Open Editors
    ${count}=    Get Length    ${editors}
    RETURN    ${count}

Get Open View Count
    [Documentation]    Get the count of currently open views.
    ${views}=    Get Open Views
    ${count}=    Get Length    ${views}
    RETURN    ${count}

Log Workbench State
    [Documentation]    Log the current workbench state for debugging purposes.
    ${info}=    Get Workbench Info
    Log    Workbench Info: ${info}
    ${perspective}=    Get Active Perspective
    Log    Active Perspective: ${perspective}
    ${views}=    Get Open Views
    Log    Open Views: ${views}
    ${editors}=    Get Open Editors
    Log    Open Editors: ${editors}

Capture Test Context
    [Documentation]    Capture and log test context for debugging failures.
    Run Keyword And Ignore Error    Log Workbench State
    Run Keyword And Ignore Error    Log    Connection Status: ${{ $Is Connected() }}

Suite Setup Connect
    [Documentation]    Standard suite setup that connects to the RCP application.
    Log    Starting RCP Library tests
    Connect To RCP App
    Log Workbench State

Suite Teardown Disconnect
    [Documentation]    Standard suite teardown that disconnects from RCP application.
    Run Keyword And Ignore Error    Reset Test State
    Disconnect From RCP App
    Log    RCP Library tests completed

Test Setup Reset State
    [Documentation]    Standard test setup that resets to clean state.
    Reset Test State

Test Teardown Cleanup
    [Documentation]    Standard test teardown with state capture on failure.
    Run Keyword If Test Failed    Capture Test Context

Wait For Agent To Be Ready
    [Documentation]    Wait for the RCP agent to be ready by checking port availability.
    ...                Attempts to connect to verify the agent RPC server is accepting connections.
    [Arguments]    ${port}=${PORT}    ${timeout}=10s
    Log    Waiting for agent on port ${port} to be ready (timeout: ${timeout})
    # Use Robot's built-in wait mechanism which is more reliable
    Wait Until Keyword Succeeds    ${timeout}    0.5s    Check Port Is Open    ${port}
    Log    Agent is ready on port ${port}

Check Port Is Open
    [Documentation]    Check if a port is open and accepting connections.
    [Arguments]    ${port}
    ${os}=    Get Platform System
    IF    '${os}' == 'Windows'
        ${result}=    Run Process    cmd    /c    nc -z -w1 127.0.0.1 ${port}
    ELSE
        ${result}=    Run Process    nc    -z    -w1    127.0.0.1    ${port}
    END
    Should Be Equal As Integers    ${result.rc}    0    msg=Port ${port} is not accepting connections yet

Start RCP Mock Application
    [Documentation]    Start the mock RCP application with the Robot Framework agent attached.
    ...                This also connects to the application after starting.
    [Arguments]    ${port}=${PORT}
    # Check if the JAR files exist
    File Should Exist    ${RCP_MOCK_JAR}    msg=Mock RCP JAR not found. Build with: cd tests/apps/rcp-mock && mvn package
    File Should Exist    ${AGENT_JAR}    msg=Agent JAR not found. Build with: cd agent && mvn package

    # Check if port is available
    ${os}=    Get Platform System
    IF    '${os}' == 'Windows'
        ${port_check}=    Run Process    cmd    /c    lsof -i :${port}
    ELSE
        ${port_check}=    Run Process    lsof    -i    :${port}
    END
    Should Be Equal As Integers    ${port_check.rc}    1    msg=Port ${port} is already in use. Kill the process first.

    # Build the command to start the mock RCP app with agent
    ${vmarg}=    Get RCP JVM Arg
    ${cmd}=    Set Variable    java ${vmarg} -javaagent:${AGENT_JAR}=port=${port} -jar ${RCP_MOCK_JAR}
    Log    Starting mock RCP application: ${cmd}

    # Start the process
    ${process}=    Start Process    ${cmd}    shell=True    stdout=${RCP_STDOUT_LOG}    stderr=${RCP_STDERR_LOG}
    Set Suite Variable    ${RCP_PROCESS}    ${process}

    # Verify the process started
    ${alive}=    Is Process Running    ${process}
    Should Be True    ${alive}    msg=RCP Mock process failed to start. Check logs at ${TEMPDIR}/rcp-mock-*.log

    # Wait for the agent to initialize with retry verification
    Wait For Agent To Be Ready    ${port}    timeout=10s

    # Connect to the application
    Connect To SWT Application    ${APP_NAME}    ${HOST}    ${port}    timeout=${CONNECTION_TIMEOUT}
    Verify Connection Is Active
    Log    Mock RCP application started and connected

Start RCP Mock Application Without Connect
    [Documentation]    Start the mock RCP application but do NOT connect.
    ...                Use this for connection tests that need to test connection behavior.
    [Arguments]    ${port}=${PORT}
    # Check if the JAR files exist
    File Should Exist    ${RCP_MOCK_JAR}    msg=Mock RCP JAR not found. Build with: cd tests/apps/rcp-mock && mvn package
    File Should Exist    ${AGENT_JAR}    msg=Agent JAR not found. Build with: cd agent && mvn package

    # Check if port is available
    ${os}=    Get Platform System
    IF    '${os}' == 'Windows'
        ${port_check}=    Run Process    cmd    /c    lsof -i :${port}
    ELSE
        ${port_check}=    Run Process    lsof    -i    :${port}
    END
    Should Be Equal As Integers    ${port_check.rc}    1    msg=Port ${port} is already in use. Kill the process first.

    # Build the command to start the mock RCP app with agent
    ${vmarg}=    Get RCP JVM Arg
    ${cmd}=    Set Variable    java ${vmarg} -javaagent:${AGENT_JAR}=port=${port} -jar ${RCP_MOCK_JAR}
    Log    Starting mock RCP application: ${cmd}

    # Start the process
    ${process}=    Start Process    ${cmd}    shell=True    stdout=${RCP_STDOUT_LOG}    stderr=${RCP_STDERR_LOG}
    Set Suite Variable    ${RCP_PROCESS}    ${process}

    # Verify the process started
    ${alive}=    Is Process Running    ${process}
    Should Be True    ${alive}    msg=RCP Mock process failed to start. Check logs at ${TEMPDIR}/rcp-mock-*.log

    # Wait for the agent to initialize with retry verification
    Wait For Agent To Be Ready    ${port}    timeout=10s
    Log    Mock RCP application started and agent is ready on port ${port}

Stop RCP Mock Application
    [Documentation]    Stop the mock RCP application gracefully.
    ...                Captures logs before terminating the process.
    TRY
        ${is_connected}=    Is Connected
        IF    ${is_connected}
            Log    Disconnecting from RCP application before shutdown
            Disconnect
        END
    EXCEPT    AS    ${error}
        Log    Warning: Error disconnecting: ${error}
    END

    IF    $RCP_PROCESS is not None
        TRY
            # Capture final logs before terminating
            ${alive}=    Is Process Running    ${RCP_PROCESS}
            IF    ${alive}
                ${stdout}=    Run Process    cat    ${RCP_STDOUT_LOG}
                ${stderr}=    Run Process    cat    ${RCP_STDERR_LOG}
                Log    Final Application STDOUT:\n${stdout.stdout}
                Log    Final Application STDERR:\n${stderr.stdout}
                Terminate Process    ${RCP_PROCESS}    kill=True
                Log    Mock RCP application terminated
            ELSE
                Log    Process was already terminated
            END
        EXCEPT    AS    ${error}
            Log    Warning: Error terminating process: ${error}
        END
    ELSE
        Log    No RCP process to stop
    END
    Set Suite Variable    ${RCP_PROCESS}    ${NONE}

Get RCP JVM Arg
    [Documentation]    Return JVM args needed for RCP on this OS (macOS needs -XstartOnFirstThread).
    ${os}=    Get Platform System
    IF    '${os}' == 'Darwin'
        RETURN    -XstartOnFirstThread
    END
    RETURN    ${EMPTY}

Get Platform System
    [Documentation]    Return platform.system() value (Linux/Windows/Darwin).
    ${os}=    Evaluate    platform.system()    modules=platform
    RETURN    ${os}

Suite Setup Start RCP App
    [Documentation]    Suite setup that starts the mock RCP application.
    Log    Starting RCP Library tests
    Start RCP Mock Application
    Log Workbench State

Suite Teardown Stop RCP App
    [Documentation]    Suite teardown that stops the mock RCP application.
    Run Keyword And Ignore Error    Reset Test State
    Stop RCP Mock Application
    Log    RCP Library tests completed
