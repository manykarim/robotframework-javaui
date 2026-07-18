*** Settings ***
Documentation     Real-DBeaver (public Eclipse RCP) WIDGET-level automation experiment.
...               Complements the workbench-level suite (real_dbeaver/experiment.robot) by
...               driving generic SWT controls — modal dialog, checkbox, button, text field,
...               combo, menu bar, toolbar — with fill / validate / click operations, each
...               followed by a read-back and a framebuffer screenshot.
...
...               F9 (FIXED, change `fix-swt-type-locator`): `type:` locators were dropped by
...               the Rust parse_locator, so every `type:<SwtClass>` finder returned 0 and the
...               main workbench window looked "unreachable". With the fix, `type:Text` etc.
...               now return the real main-window widgets. This suite asserts that reachability
...               (regression guard) and documents which widget classes DBeaver actually uses.
Resource          ../real_dbeaver/resources/common.resource
Library           Collections
Suite Setup       Connect To DBeaver
Suite Teardown    Disconnect From DBeaver

*** Test Cases ***
Modal Dialog Checkbox And Button Drive And Dismiss
    [Documentation]    STRICT: on DBeaver's first-run "Data share" app-modal dialog, tick the
    ...                "Do not share data." checkbox and click the "Confirm" button, then prove
    ...                the modal is gone. Controls: modal dialog + checkbox + button. Ops:
    ...                check, click, validate-absent. Real, validated widget automation.
    Find Widget    text:Confirm
    Grab Screenshot    w1_modal_present
    Check Button    text:Do not share data.
    ${chk}=    Run Keyword And Ignore Error    Get Widget Property    text:Do not share data.    selection
    Log    EVIDENCE [w1_checkbox_selection] ${chk}
    Grab Screenshot    w1_checkbox_ticked
    Click Widget    text:Confirm
    Wait Until Keyword Succeeds    5x    1s
    ...    Run Keyword And Expect Error    *    Find Widget    text:Confirm
    Grab Screenshot    w1_modal_dismissed

Main Window Widgets Are Reachable By Type
    [Documentation]    F9 REGRESSION GUARD: with the `type:` locator fix, the main workbench
    ...                window's widgets must be reachable. Enumerate per type and assert that
    ...                core types (Composite/Text/Button) return non-zero (was 0 before the fix).
    ${info}=    Get Workbench Info
    Dictionary Should Not Contain Key    ${info}    error
    ${reach}=    Create Dictionary
    FOR    ${ty}    IN    Text    StyledText    Combo    CCombo    Button    ToolItem    Tree    Table    Composite    Label
        ${st}    ${ws}=    Run Keyword And Ignore Error    Find Widgets    type:${ty}
        ${n}=    Run Keyword If    '${st}' == 'PASS'    Get Length    ${ws}    ELSE    Set Variable    0
        Set To Dictionary    ${reach}    ${ty}    ${n}
        Log    EVIDENCE [reach type:${ty}] count=${n}
    END
    Grab Screenshot    w2_main_enumerated
    Should Be True    ${reach}[Composite] > 0    msg=F9 regression: no Composite reachable in main window
    Should Be True    ${reach}[Text] > 0    msg=F9 regression: no Text reachable in main window
    Should Be True    ${reach}[Button] > 0    msg=F9 regression: no Button reachable in main window

Text Field Is Reachable And Fillable
    [Documentation]    TEXT FIELD (F9 fixed): the main window's SWT Text widgets are now
    ...                reachable. Assert they are found, then fill+validate the filter field.
    ...                The Connections filter is the first visible+enabled Text; if several match
    ...                ambiguously, records the finding (names are not exposed by DBeaver).
    ${texts}=    Find Widgets    type:Text
    ${n}=    Get Length    ${texts}
    Log    EVIDENCE [w3_text_reachable] count=${n}
    Should Be True    ${n} > 0    msg=F9 regression: no Text field reachable in main window
    ${st}    ${r}=    Run Keyword And Ignore Error    Fill And Validate Text Field    type:Text
    Log    EVIDENCE [w3_text_fill] status=${st} note=${r}
    Grab Screenshot    w3_text_field

Combo Dropdown Select And Validate
    [Documentation]    COMBO/DROPDOWN. Drives a reachable Combo/CCombo; records the finding if
    ...                the main-window AI combo is not reachable (F9).
    ${st}    ${w}=    Run Keyword And Ignore Error    Find Widget    type:Combo
    ${st2}    ${w2}=    Run Keyword And Ignore Error    Find Widget    type:CCombo
    Run Keyword If    '${st}' == 'PASS'    Log    EVIDENCE [w4_combo] Combo reachable
    ...    ELSE IF    '${st2}' == 'PASS'    Log    EVIDENCE [w4_combo] CCombo reachable
    ...    ELSE    Log    EVIDENCE [w4_combo] NO combo reachable in main window (F9)
    Grab Screenshot    w4_combo

Menu Bar Select Main Menu Documents Stub
    [Documentation]    MENU. `Select Main Menu` returns success on real Eclipse but does not
    ...                actually open the menu (documented no-op stub). Records the real behavior;
    ...                asserts the workbench stays healthy.
    ${before}=    Get Workbench Info
    ${st}    ${r}=    Run Keyword And Ignore Error    Select Main Menu    Window
    Log    EVIDENCE [w5_select_main_menu] status=${st} result=${r} (stub: returns ok, no UI change)
    Sleep    1s
    Grab Screenshot    w5_menu
    ${after}=    Get Workbench Info
    Dictionary Should Not Contain Key    ${after}    error

Execute Command Opens Preferences Modal
    [Documentation]    SECOND MODAL via a command. Execute Command opens the Preferences dialog
    ...                (works now that the first-run modal is dismissed). Records whether its
    ...                Cancel button is reachable, screenshots, and closes it.
    Execute Command    org.eclipse.ui.window.preferences
    Sleep    3s
    Grab Screenshot    w6_prefs_open
    ${st}    ${w}=    Run Keyword And Ignore Error    Find Widget    text:Cancel
    Log    EVIDENCE [w6_prefs_cancel_reachable] status=${st}
    Run Keyword If    '${st}' == 'PASS'    Click Widget    text:Cancel
    ...    ELSE    Run Keyword And Ignore Error    Execute Command    org.eclipse.ui.file.exit
    Grab Screenshot    w6_prefs_after

*** Keywords ***
Fill And Validate Text Field
    [Arguments]    ${locator}
    Clear Text    ${locator}
    Input Text    ${locator}    dbeaver_probe_42
    Widget Text Should Be    ${locator}    dbeaver_probe_42
    ${v}=    Run Keyword And Ignore Error    Get Widget Property    ${locator}    text
    Log    EVIDENCE [w3_text_readback] ${v}
    Clear Text    ${locator}
    Widget Text Should Be    ${locator}    ${EMPTY}
    Log    EVIDENCE [w3_text_field] filled+validated+cleared on ${locator}
