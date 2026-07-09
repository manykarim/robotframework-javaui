*** Settings ***
Documentation     RCP validation against a REAL Eclipse workbench (not the mock).
...
...               This suite proves the rcp-real-eclipse-validation capability: the RCP
...               introspection keywords return live data from an actual running Eclipse
...               Rich Client Platform application.
...
...               PREREQUISITES (opt-in / CI job — not part of the default fast run):
...               1. Build the agent:  mvn -f agent/pom.xml package
...               2. Launch the real Eclipse RCP app headless with the agent attached:
...                  tests/apps/rcp/build-and-run-real-eclipse.sh   (uses xvfb + Eclipse 4.30)
...                  It downloads an Eclipse platform on first run and listens on ${AGENT_PORT}.
...               3. Run this suite while that process is up.
...
...               The launcher is intentionally decoupled from this suite so the (large,
...               network-dependent) Eclipse download is only paid for in the dedicated job.

Library           JavaGui.Rcp
Library           Collections
Suite Setup       Connect To Real Eclipse
Suite Teardown    Run Keyword And Ignore Error    Disconnect

*** Variables ***
${AGENT_HOST}     127.0.0.1
${AGENT_PORT}     5682
${CONNECT_TMO}    120

*** Test Cases ***
Workbench Is Reported As Available
    [Documentation]    get_workbench_info must return live workbench data, not the
    ...                "No RCP workbench available" error that a mock-only path returns.
    Wait For Workbench    30
    ${info}=    Get Workbench Info
    Dictionary Should Not Contain Key    ${info}    error
    Should Be Equal    ${info}[info]    Eclipse RCP Workbench
    Should Be True    ${info}[windowCount] >= 1

Custom Perspectives Are Discovered
    [Documentation]    The application-defined perspectives must appear in the registry.
    ${persps}=    Get Available Perspectives
    ${ids}=    Evaluate    [p['id'] for p in $persps]
    Should Contain    ${ids}    com.testapp.rcp.perspective.main
    Should Contain    ${ids}    com.testapp.rcp.perspective.data

Active Perspective Is A Real Id
    ${persp}=    Get Active Perspective Id
    Should Not Be Empty    ${persp}

Open Views Come From The Live Workbench
    [Documentation]    Open views are resolved on the SWT UI thread from the real workbench.
    ${ids}=    Get Open View Ids
    Should Not Be Empty    ${ids}
    ${views}=    Get Open Views
    Should Not Be Empty    ${views}

*** Keywords ***
Connect To Real Eclipse
    [Documentation]    Connect to the already-running real Eclipse RCP app + agent.
    ...                This is an OPT-IN suite: it self-skips unless the real Eclipse
    ...                RCP app is already running on ${AGENT_PORT} (launched via
    ...                tests/apps/rcp/build-and-run-real-eclipse.sh), so the default
    ...                CI run does not fail when that app is absent.
    ${reachable}=    Evaluate    __import__('socket').socket().connect_ex(('${AGENT_HOST}', ${AGENT_PORT})) == 0
    Skip If    not ${reachable}    Real Eclipse RCP app not running on port ${AGENT_PORT} (opt-in suite; see tests/apps/rcp/build-and-run-real-eclipse.sh)
    Connect To Swt Application    rcp    ${AGENT_HOST}    ${AGENT_PORT}    ${CONNECT_TMO}
    ${connected}=    Is Connected
    Should Be True    ${connected}
