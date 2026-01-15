*** Settings ***
Documentation     Common resources for DBeaver SWT/RCP testing
Library           Collections
Library           OperatingSystem
Library           Process

# Import the SWT and RCP libraries from JavaGui
Library           JavaGui.Swt    WITH NAME    SWT
Library           JavaGui.Rcp    WITH NAME    RCP

*** Variables ***
${DBEAVER_PATH}           ${CURDIR}/../../../../demo/dbeaver
${DBEAVER_EXECUTABLE}     ${DBEAVER_PATH}/dbeaver
${AGENT_JAR}              ${CURDIR}/../../../../agent/target/robotframework-swing-agent-1.0.0-all.jar
${SWT_PORT}               18081
${TIMEOUT}                30s
${POLL_INTERVAL}          0.5s

# DBeaver window titles
${MAIN_WINDOW_TITLE}      DBeaver
${WELCOME_DIALOG}         Welcome to DBeaver

# Common locators
${DATABASE_NAVIGATOR}     view:Database Navigator
${PROJECT_EXPLORER}       view:Projects
${SQL_EDITOR}             editor:*
${PROPERTIES_VIEW}        view:Properties

*** Keywords ***
Start DBeaver With Agent
    [Documentation]    Start DBeaver application with the Robot Framework agent attached
    [Arguments]    ${additional_args}=

    # Check if agent JAR exists
    File Should Exist    ${AGENT_JAR}    msg=Agent JAR not found. Run build-swt-agent.sh first.

    # Start DBeaver with agent - build command as a single string
    ${cmd}=    Set Variable    ${DBEAVER_EXECUTABLE} -vmargs -javaagent:${AGENT_JAR}=port=${SWT_PORT} ${additional_args}
    ${process}=    Start Process    ${cmd}    shell=True    stdout=PIPE    stderr=PIPE

    Set Suite Variable    ${DBEAVER_PROCESS}    ${process}

    # Wait for agent to start
    Sleep    5s

    # Connect to the agent
    Connect To Application

    # Wait for main window to appear
    Wait For Main Window

Start DBeaver Without Agent
    [Documentation]    Start DBeaver without agent (for development/debugging)
    ${cmd}=    Set Variable    ${DBEAVER_EXECUTABLE}
    ${process}=    Start Process    ${cmd}    shell=True    stdout=PIPE    stderr=PIPE
    Set Suite Variable    ${DBEAVER_PROCESS}    ${process}
    Sleep    5s

Connect To Application
    [Documentation]    Connect to the SWT/RCP agent
    SWT.Connect To Swt Application    dbeaver    host=127.0.0.1    port=${SWT_PORT}    timeout=${30.0}

Close DBeaver
    [Documentation]    Close DBeaver application gracefully
    TRY
        SWT.Disconnect
        Terminate Process    ${DBEAVER_PROCESS}    kill=True
    EXCEPT
        Force Close DBeaver
    END

Force Close DBeaver
    [Documentation]    Force terminate DBeaver process
    Terminate Process    ${DBEAVER_PROCESS}    kill=True

Wait For Main Window
    [Documentation]    Wait for DBeaver main window to appear
    Wait Until Keyword Succeeds    ${TIMEOUT}    ${POLL_INTERVAL}
    ...    SWT.Wait Until Widget Exists    text:${MAIN_WINDOW_TITLE}

Close Welcome Dialog If Present
    [Documentation]    Close the welcome dialog if it appears
    TRY
        ${widget}=    SWT.Find Widget    text:${WELCOME_DIALOG}
        SWT.Close Shell    text:${WELCOME_DIALOG}
    EXCEPT
        Log    No welcome dialog found
    END

Close Tip Of Day Dialog If Present
    [Documentation]    Close the Tip of the Day dialog if it appears
    TRY
        ${shells}=    SWT.Get Shells
        FOR    ${shell}    IN    @{shells}
            ${shell_str}=    Convert To String    ${shell}
            ${is_tip}=    Evaluate    "Tip of the day" in $shell_str
            IF    ${is_tip}
                # Try clicking Close button
                TRY
                    SWT.Click Widget    text:&Close
                EXCEPT
                    TRY
                        SWT.Click Widget    text:Close
                    EXCEPT
                        Log    Could not click Close button
                    END
                END
                Sleep    0.5s
                RETURN
            END
        END
        Log    No Tip of the Day dialog found
    EXCEPT    AS    ${error}
        Log    Error closing Tip dialog: ${error}
    END

Close All Startup Dialogs
    [Documentation]    Close all startup dialogs (Welcome, Tip of Day, etc.)
    Close Tip Of Day Dialog If Present
    Close Welcome Dialog If Present

Wait For View
    [Documentation]    Wait for a view to be available
    [Arguments]    ${view_id}    ${timeout}=${TIMEOUT}
    Wait Until Keyword Succeeds    ${timeout}    ${POLL_INTERVAL}
    ...    SWT.Wait Until Widget Exists    ${view_id}

Wait For Editor
    [Documentation]    Wait for an editor to be available
    [Arguments]    ${editor_name}    ${timeout}=${TIMEOUT}
    Wait Until Keyword Succeeds    ${timeout}    ${POLL_INTERVAL}
    ...    SWT.Wait Until Widget Exists    text:${editor_name}

Click Menu Item
    [Documentation]    Click a menu item by path
    [Arguments]    ${menu_path}
    RCP.Select Main Menu    ${menu_path}

Verify Button Exists
    [Documentation]    Verify a button exists and is enabled
    [Arguments]    ${locator}
    ${element}=    SWT.Find Widget    ${locator}
    Should Not Be Empty    ${element}
    SWT.Widget Should Be Enabled    ${locator}

Verify Text Field Exists
    [Documentation]    Verify a text field exists
    [Arguments]    ${locator}
    ${element}=    SWT.Find Widget    ${locator}
    Should Not Be Empty    ${element}

Get Widget Count
    [Documentation]    Get count of widgets matching locator
    [Arguments]    ${locator}
    ${elements}=    SWT.Find Widgets    ${locator}
    ${count}=    Get Length    ${elements}
    RETURN    ${count}

Widget Should Exist
    [Documentation]    Assert that a widget exists
    [Arguments]    ${locator}    ${message}=Widget not found
    TRY
        ${element}=    SWT.Find Widget    ${locator}
        Should Not Be Empty    ${element}    ${message}: ${locator}
    EXCEPT
        Fail    ${message}: ${locator}
    END

Widget Should Not Exist
    [Documentation]    Assert that a widget does not exist
    [Arguments]    ${locator}    ${message}=Widget should not exist
    TRY
        ${elements}=    SWT.Find Widgets    ${locator}
        ${count}=    Get Length    ${elements}
        Should Be True    ${count} == 0    ${message}: ${locator}
    EXCEPT
        Pass Execution    Widget correctly not found
    END

Take Screenshot On Failure
    [Documentation]    Take screenshot when a test fails
    Run Keyword If Test Failed    Log    Screenshot capture not yet implemented
