*** Settings ***
Documentation     Test connection and disconnection to Swing applications
Resource          resources/common.resource
Suite Setup       Log    Starting connection tests
Suite Teardown    Run Keyword And Ignore Error    Disconnect

*** Test Cases ***
List Available Java Applications
    [Documentation]    Verify we can list running Java applications
    [Tags]    not_implemented
    Skip    JVM discovery not yet implemented - requires agent running first

Connect To Demo Application By Main Class
    [Documentation]    Connect to demo app using main class name
    Start Demo Application
    Element Should Exist    JTabbedPane[name='mainTabs']
    [Teardown]    Stop Demo Application

Connect To Demo Application By Title
    [Documentation]    Connect to demo app using window title
    ${cmd}=    Set Variable    java -javaagent:${AGENT_JAR}=port=${AGENT_PORT} -jar ${DEMO_APP_JAR}
    ${process}=    Start Process    ${cmd}    shell=True    alias=demo_title
    Sleep    3s
    Connect To Application    title=*Demo*    host=localhost    port=${AGENT_PORT}    timeout=30
    Element Should Exist    JTabbedPane[name='mainTabs']
    [Teardown]    Run Keywords
    ...    Disconnect    AND
    ...    Terminate Process    demo_title    kill=True

Disconnect And Reconnect
    [Documentation]    Verify reconnection works
    Start Demo Application
    Element Should Exist    JTabbedPane[name='mainTabs']
    Disconnect
    Sleep    1s
    Connect To Application    main_class=${DEMO_MAIN_CLASS}    host=localhost    port=${AGENT_PORT}    timeout=30
    Element Should Exist    JTabbedPane[name='mainTabs']
    [Teardown]    Stop Demo Application

Connection Timeout Handling
    [Documentation]    Verify proper timeout handling
    [Tags]    negative
    ${status}=    Run Keyword And Return Status
    ...    Connect To Application    main_class=NonExistentApp    timeout=5
    Should Be Equal    ${status}    ${FALSE}
