*** Settings ***
Library    swing_library.SwingLibrary
Library    Process
Library    OperatingSystem
Library    String

*** Variables ***
${AGENT_JAR}           ${CURDIR}${/}..${/}..${/}..${/}..${/}agent${/}target${/}robotframework-swt-agent-1.0.0-all.jar
${TEST_APP_JAR}        ${CURDIR}${/}..${/}..${/}..${/}apps${/}swt${/}target${/}swt-test-app-1.0.0-all.jar
${AGENT_HOST}          localhost
${AGENT_PORT}          18082
${CONNECTION_TIMEOUT}  30
${DEFAULT_TIMEOUT}     10

*** Keywords ***
Start SWT Test Application
    [Documentation]    Start the SWT test application with the agent and connect
    ${cmd}=    Set Variable    java -javaagent:${AGENT_JAR}=port=${AGENT_PORT} -jar ${TEST_APP_JAR}
    Log    Starting: ${cmd}
    ${process}=    Start Process    ${cmd}    shell=True    alias=swt_test
    Sleep    4s    Wait for SWT application and agent to start
    Connect To Application    host=${AGENT_HOST}    port=${AGENT_PORT}    timeout=${CONNECTION_TIMEOUT}
    Wait Until Element Exists    Shell[name='mainShell']    timeout=${DEFAULT_TIMEOUT}
    RETURN    ${process}

Stop SWT Test Application
    [Documentation]    Disconnect from the application and stop the process
    Run Keyword And Ignore Error    Disconnect
    Run Keyword And Ignore Error    Terminate Process    swt_test    kill=True
