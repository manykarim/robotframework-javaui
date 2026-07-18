*** Settings ***
Documentation     Shared setup + evidence helpers for the DBeaver (real public Eclipse RCP
...               application) automation experiment.
Library           JavaGui.Rcp
Library           Collections
Library           Process
Library           OperatingSystem
Library           String

*** Variables ***
${HOST}           127.0.0.1
${PORT}           5682
${SHOTS}          ${OUTPUT DIR}${/}shots
${CONNECT_TMO}    120

*** Keywords ***
Connect To DBeaver
    [Documentation]    Connect to the DBeaver RCP app via the agent. Self-skips the whole
    ...                suite if the agent port is unreachable (so this is safe to run
    ...                outside the Docker harness too).
    ${reachable}=    Evaluate    __import__('socket').socket().connect_ex(('${HOST}', ${PORT})) == 0
    Skip If    not ${reachable}    Agent not reachable on ${HOST}:${PORT} (run via tests/docker/rcp harness)
    Connect To Swt Application    rcp    ${HOST}    ${PORT}    ${CONNECT_TMO}
    ${connected}=    Is Connected
    Should Be True    ${connected}
    Wait For Workbench    45
    Grab Screenshot    01_connected

Disconnect From DBeaver
    Run Keyword And Ignore Error    Disconnect

Grab Screenshot
    [Documentation]    Whole-framebuffer PNG (real pixels the app rendered) + embed in the
    ...                Robot log for visual confirmation. This is Path A (X-server grab),
    ...                which needs no library screenshot support.
    [Arguments]    ${name}
    ${path}=    Set Variable    ${SHOTS}${/}${name}.png
    ${r}=    Run Process    import    -window    root    ${path}    stderr=STDOUT    stdout=/tmp/import.log
    Run Keyword If    ${r.rc} != 0
    ...    Run Process    bash    -c    xwd -root -silent | convert xwd:- '${path}'
    ${exists}=    Run Keyword And Return Status    File Should Exist    ${path}
    Run Keyword If    ${exists}    Log    <b>${name}</b><br><img src="shots/${name}.png" width="900">    html=True
    RETURN    ${path}

Record Outcome
    [Documentation]    Attempt an action, never abort the suite, and log exactly what the
    ...                real RCP app returned (success, error text, or exception). This is
    ...                how we collect evidence about mock-only / off-UI-thread breakage.
    [Arguments]    ${label}    ${keyword}    @{args}
    ${status}    ${result}=    Run Keyword And Ignore Error    ${keyword}    @{args}
    Log    EVIDENCE [${label}] status=${status} result=${result}
    RETURN    ${status}    ${result}
