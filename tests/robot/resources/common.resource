*** Settings ***
Documentation     Common resources for Swing Library tests
Library           swing_library.SwingLibrary
Library           OperatingSystem
Library           Process

*** Variables ***
${DEMO_APP_JAR}           ${CURDIR}/../../../demo/target/swing-demo-app-1.0.0.jar
${DEMO_MAIN_CLASS}        com.example.demo.SwingDemoApp
${LOOKS_DEMO_JAR}         ${CURDIR}/../../../demo/looksdemo-3.13.3.jar
${SMART_CLIENT_JAR}       ${CURDIR}/../../../demo/smart-client-showcase-24.09.0.jar
${CONNECTION_TIMEOUT}     30
${AGENT_JAR}              ${CURDIR}/../../../agent/target/robotframework-swing-agent-1.0.0-all.jar
${AGENT_PORT}             5678

*** Keywords ***
Start Demo Application
    [Documentation]    Start the demo Swing application with agent
    ${cmd}=    Set Variable    java -javaagent:${AGENT_JAR}=port=${AGENT_PORT} -jar ${DEMO_APP_JAR}
    ${process}=    Start Process    ${cmd}    shell=True    alias=demo_app
    Sleep    3s    Wait for application and agent to start
    Connect To Application    main_class=${DEMO_MAIN_CLASS}    host=localhost    port=${AGENT_PORT}    timeout=${CONNECTION_TIMEOUT}
    RETURN    ${process}

Start Demo Application With Agent
    [Documentation]    Start demo application with agent pre-loaded
    [Arguments]    ${agent_port}=${AGENT_PORT}
    ${cmd}=    Set Variable    java -javaagent:${AGENT_JAR}=port=${agent_port} -jar ${DEMO_APP_JAR}
    ${process}=    Start Process    ${cmd}    shell=True    alias=demo_app
    Sleep    2s    Wait for application to start
    Connect To Application    main_class=${DEMO_MAIN_CLASS}    host=localhost    port=${agent_port}    timeout=${CONNECTION_TIMEOUT}
    RETURN    ${process}

Stop Demo Application
    [Documentation]    Stop the demo application
    Disconnect
    Terminate Process    demo_app    kill=True

Start Looks Demo
    [Documentation]    Start the JGoodies Looks Demo application with agent
    ${cmd}=    Set Variable    java -javaagent:${AGENT_JAR}=port=${AGENT_PORT} -jar ${LOOKS_DEMO_JAR}
    ${process}=    Start Process    ${cmd}    shell=True    alias=looks_demo
    Sleep    3s
    Connect To Application    title=*Looks*    host=localhost    port=${AGENT_PORT}    timeout=${CONNECTION_TIMEOUT}
    RETURN    ${process}

Stop Looks Demo
    [Documentation]    Stop the Looks Demo application
    Disconnect
    Terminate Process    looks_demo    kill=True

Verify Status Label
    [Documentation]    Verify the status label shows expected text
    [Arguments]    ${expected_text}
    ${text}=    Get Element Text    JLabel#statusLabel
    Should Contain    ${text}    ${expected_text}

Select Tab By Name
    [Documentation]    Select a tab in the main tabbed pane by name
    [Arguments]    ${tab_name}
    Select Tab    JTabbedPane#mainTabs    ${tab_name}

Login With Credentials
    [Documentation]    Perform login with given credentials
    [Arguments]    ${username}    ${password}
    Select Tab By Name    Login
    Clear Text    #username
    Input Text    #username    ${username}
    Clear Text    #password
    Input Text    #password    ${password}
    Click    JButton#loginBtn
