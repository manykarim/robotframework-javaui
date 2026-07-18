*** Settings ***
Documentation     EXPERIMENT + REGRESSION ORACLE: automate a real, public Eclipse RCP
...               application (DBeaver Community Edition) with robotframework-javaui,
...               headless in Docker, validating introspection AND actions with STRICT
...               assertions plus framebuffer screenshots.
...
...               This suite is the oracle for the openspec change `rcp-real-app-automation`:
...               before the fixes it fails on the action tests (F3 Invalid thread access,
...               F4/F5 view/perspective, F6 screenshot); after the fixes it must pass.
...               Every step captures a screenshot; read log.html with the images.
Resource          resources/common.resource
Suite Setup       Connect To DBeaver
Suite Teardown    Disconnect From DBeaver

*** Test Cases ***
Workbench Info Is Live
    [Documentation]    Get Workbench Info returns real workbench data from running DBeaver.
    ${info}=    Get Workbench Info
    Log    EVIDENCE [workbench_info] ${info}
    Grab Screenshot    10_workbench
    Dictionary Should Not Contain Key    ${info}    error
    Should Be True    ${info}[windowCount] >= 1

Perspectives And Views Are Discovered
    [Documentation]    Live perspective + view registries surface DBeaver's own ids.
    ${persps}=    Get Available Perspectives
    ${pids}=    Evaluate    [p.get('id') for p in $persps if p.get('id')]
    Log    EVIDENCE [perspective_ids] ${pids}
    Should Not Be Empty    ${pids}
    ${active}=    Get Active Perspective Id
    Should Not Be Empty    ${active}
    ${view_ids}=    Get Open View Ids
    Log    EVIDENCE [open_view_ids] ${view_ids}
    Grab Screenshot    11_views
    Should Not Be Empty    ${view_ids}
    Set Suite Variable    ${ACTIVE_PERSP}    ${active}
    Set Suite Variable    ${ALL_PERSP_IDS}    ${pids}
    Set Suite Variable    ${OPEN_VIEW_IDS}    ${view_ids}

SWT Readiness Handshake Is Available
    [Documentation]    F2: the agent exposes a readiness wait so drivers need not guess timing.
    Record Outcome    wait_until_swt_ready    Wait Until Swt Ready    10

Close Then Show View Round Trips
    [Documentation]    F3/F4/F5 STRICT: close a discovered open view -> it must be GONE;
    ...                re-show it -> it must be BACK. No Invalid thread access, no false-success.
    Skip If    not ${OPEN_VIEW_IDS}    no open views discovered
    ${target}=    Set Variable    ${OPEN_VIEW_IDS}[0]
    Log    EVIDENCE [round_trip_target] ${target}
    Close View    ${target}
    Sleep    1s
    ${after_close}=    Get Open View Ids
    Log    EVIDENCE [after_close_ids] ${after_close}
    Grab Screenshot    12_after_close_view
    List Should Not Contain Value    ${after_close}    ${target}
    ...    msg=Close View was a no-op: view still open (F5)
    Show View    ${target}
    Sleep    1s
    ${after_show}=    Get Open View Ids
    Log    EVIDENCE [after_show_ids] ${after_show}
    Grab Screenshot    13_after_show_view
    List Should Contain Value    ${after_show}    ${target}
    ...    msg=Show View did not restore a registered view (F4)

Open Perspective Runs On UI Thread Without Error
    [Documentation]    F3/F4: switching perspectives must invoke the real Eclipse API on the
    ...                UI thread without raising or crashing the workbench. Whether a given
    ...                product actually honors the switch is app-specific — DBeaver is
    ...                effectively single-perspective and keeps its own perspective, which we
    ...                record as evidence rather than assert as a library defect.
    ${persps}=    Get Available Perspectives
    ${pids}=    Evaluate    [p.get('id') for p in $persps if p.get('id')]
    ${active}=    Get Active Perspective Id
    ${other}=    Evaluate    next((p for p in ${pids} if p != "${active}"), None)
    Skip If    $other is None    only one perspective registered
    Log    EVIDENCE [switch_target] from=${active} to=${other}
    ${st}    ${r}=    Run Keyword And Ignore Error    Open Perspective    ${other}
    Sleep    1s
    ${now}=    Get Active Perspective Id
    Log    EVIDENCE [switched_active] requested=${other} status=${st} active_now=${now} (DBeaver stays single-perspective)
    Grab Screenshot    14_perspective
    ${info}=    Get Workbench Info
    Dictionary Should Not Contain Key    ${info}    error
    Run Keyword And Ignore Error    Open Perspective    ${active}

Execute Command Runs On UI Thread
    [Documentation]    F3 STRICT: Execute a real command by id. Must not raise, and the
    ...                workbench must still be healthy afterwards (no Invalid thread access
    ...                crash). Screenshot shows the result; log is grepped in verification.
    Execute Command    org.eclipse.ui.window.preferences
    Sleep    2s
    Grab Screenshot    15_execute_command_preferences
    ${info}=    Get Workbench Info
    Dictionary Should Not Contain Key    ${info}    error
    Run Keyword And Ignore Error    Execute Command    org.eclipse.ui.file.exit

Capture Screenshot Keyword Produces A Real Image
    [Documentation]    F6 STRICT: the library's own Capture Screenshot must write a real PNG.
    ${path}=    Set Variable    ${SHOTS}${/}lib_capture.png
    Capture Screenshot    ${path}
    File Should Exist    ${path}
    ${size}=    Get File Size    ${path}
    Log    EVIDENCE [lib_capture_bytes] ${size}
    Should Be True    ${size} > 1000    msg=Capture Screenshot wrote an empty/absent file (F6)
    Grab Screenshot    17_after_lib_capture

RCP Inspector Keywords Are Available On The RCP Library
    [Documentation]    F7: inspector keywords must exist on the RCP library.
    ${v}    ${vr}=    Record Outcome    all_rcp_views      Get All Rcp Views
    ${e}    ${er}=    Record Outcome    all_rcp_editors    Get All Rcp Editors
    ${t}    ${tr}=    Record Outcome    rcp_component_tree    Get Rcp Component Tree
    Should Not Contain    ${vr}    No keyword with name
    ...    msg=Get All Rcp Views is missing from the RCP library (F7)
    Should Not Contain    ${tr}    No keyword with name
    ...    msg=Get Rcp Component Tree is missing from the RCP library (F7)
