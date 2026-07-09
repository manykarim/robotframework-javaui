*** Settings ***
Documentation     Shared resource for the Smart Client Showcase proof suite.
...
...               Launches the third-party JGoodies Smart Client Showcase with the JavaGui
...               agent attached and provides validated-action helpers: every state-changing
...               action is paired with an assertion that reads the app state back.
...
...               OPT-IN: the showcase jar is a third-party binary kept out of git
...               (example-apps/*.jar is gitignored). The suite self-skips if it is absent,
...               so the default CI run is unaffected.

Library           JavaGui.Swing
Library           Process
Library           OperatingSystem
Library           Collections

*** Variables ***
${SHOWCASE_JAR}       ${EXECDIR}${/}example-apps${/}smart-client-showcase-24.09.0.jar
${AGENT_JAR}          ${EXECDIR}${/}agent${/}target${/}javagui-agent.jar
${AGENT_PORT}         18080
${AGENT_HOST}         127.0.0.1
${STARTUP_WAIT}       6
${NAV_WAIT}           2
${CONNECT_TIMEOUT}    30
${APP_HANDLE}         ${NONE}

*** Keywords ***
Launch Showcase
    [Documentation]    Start the showcase with the agent, then connect. Self-skips if the
    ...                jar or agent is missing (opt-in suite).
    ${has_jar}=    Run Keyword And Return Status    File Should Exist    ${SHOWCASE_JAR}
    Skip If    not ${has_jar}    Showcase jar not present (opt-in suite): ${SHOWCASE_JAR}
    ${has_agent}=    Run Keyword And Return Status    File Should Exist    ${AGENT_JAR}
    Skip If    not ${has_agent}    Agent jar not built: run 'mvn -f agent/pom.xml package'
    # Escape the '=' in the javaagent arg so RF does not treat it as a named argument.
    ${javaagent}=    Set Variable    -javaagent:${AGENT_JAR}\=port\=${AGENT_PORT}
    ${proc}=    Start Process    java    ${javaagent}    -jar    ${SHOWCASE_JAR}
    ...    alias=showcase
    ...    stdout=${OUTPUT DIR}${/}showcase-stdout.log    stderr=${OUTPUT DIR}${/}showcase-stderr.log
    Set Suite Variable    ${APP_HANDLE}    ${proc}
    Sleep    ${STARTUP_WAIT}
    Connect With Retry

Connect With Retry
    [Documentation]    Retry the connect for a while (the JVM + agent take a few seconds).
    Wait Until Keyword Succeeds    30x    2s
    ...    Connect To Application    host=${AGENT_HOST}    port=${AGENT_PORT}    timeout=${CONNECT_TIMEOUT}
    ${connected}=    Is Connected
    Should Be True    ${connected}

Close Showcase
    [Documentation]    Disconnect and terminate the showcase process.
    Run Keyword And Ignore Error    Disconnect
    Run Keyword And Ignore Error    Terminate Process    showcase

Ensure Connected
    [Documentation]    Reconnect guard against the known broken-pipe flakiness under load.
    ${ok}=    Run Keyword And Return Status    Is Connected
    IF    not ${ok}
        Connect To Application    host=${AGENT_HOST}    port=${AGENT_PORT}    timeout=${CONNECT_TIMEOUT}
    END

# ----- Navigation -----

Open Section
    [Documentation]    Click a left-nav section toggle and refresh the tree.
    [Arguments]    ${section}
    Ensure Connected
    Click    NavigationToggleButton[text='${section}']
    Sleep    ${NAV_WAIT}
    Refresh UI Tree

Navigate And Verify Section
    [Documentation]    Open a section and confirm its toggle is selected (independent read-back).
    [Arguments]    ${section}
    Open Section    ${section}
    Element Should Be Selected    NavigationToggleButton[text='${section}']

# ----- Validated action helpers (action + independent read-back) -----

Enter And Verify Text
    [Arguments]    ${locator}    ${text}
    Input Text    ${locator}    ${text}
    ${actual}=    Get Element Text    ${locator}
    Should Be Equal    ${actual}    ${text}    msg=Field did not retain the entered text

Clear And Verify Empty
    [Arguments]    ${locator}
    Clear Text    ${locator}
    ${actual}=    Get Element Text    ${locator}
    Should Be Equal    ${actual}    ${EMPTY}    msg=Field was not cleared

Select Radio And Verify
    [Documentation]    Select a radio and confirm it is selected (and optionally a sibling is not).
    [Arguments]    ${locator}    ${sibling}=${NONE}
    Select Radio Button    ${locator}
    Element Should Be Selected    ${locator}
    IF    $sibling is not None
        Element Should Not Be Selected    ${sibling}
    END

Check And Verify
    [Arguments]    ${locator}
    Check Checkbox    ${locator}
    Element Should Be Selected    ${locator}

Uncheck And Verify
    [Arguments]    ${locator}
    Uncheck Checkbox    ${locator}
    Element Should Not Be Selected    ${locator}

Select Combo And Verify
    [Arguments]    ${locator}    ${value}
    Select From Combobox    ${locator}    ${value}
    Element Text Should Be    ${locator}    ${value}
