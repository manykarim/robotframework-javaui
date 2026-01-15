*** Settings ***
Library    swing_library.SwingLibrary
Library    Process
Library    OperatingSystem
Library    String

*** Variables ***
${AGENT_JAR}           ${CURDIR}${/}..${/}..${/}..${/}..${/}agent${/}target${/}robotframework-swing-agent-1.0.0-all.jar
${TEST_APP_JAR}        ${CURDIR}${/}..${/}..${/}..${/}apps${/}swing${/}target${/}swing-test-app-1.0.0.jar
${AGENT_HOST}          localhost
${AGENT_PORT}          5679
${CONNECTION_TIMEOUT}  30
${DEFAULT_TIMEOUT}     10
${MAIN_CLASS}          testapp.SwingTestApp

*** Keywords ***
Start Test Application
    [Documentation]    Start the Swing test application with the agent and connect
    ${cmd}=    Set Variable    java -javaagent:${AGENT_JAR}=port=${AGENT_PORT} -jar ${TEST_APP_JAR}
    Log    Starting: ${cmd}
    ${process}=    Start Process    ${cmd}    shell=True    alias=swing_test
    Sleep    3s    Wait for application and agent to start
    Connect To Application    main_class=${MAIN_CLASS}    host=${AGENT_HOST}    port=${AGENT_PORT}    timeout=${CONNECTION_TIMEOUT}
    # Verify connection by waiting for a known component
    Wait Until Element Exists    JTextField[name='nameTextField']    timeout=${DEFAULT_TIMEOUT}
    RETURN    ${process}

Stop Test Application
    [Documentation]    Disconnect from the application and stop the process
    Run Keyword And Ignore Error    Disconnect
    Run Keyword And Ignore Error    Terminate Process    swing_test    kill=True
