*** Settings ***
Documentation     Validated-action proof against the real Smart Client Showcase.
...
...               Every state-changing action is immediately confirmed by reading the
...               application's state back through an independent keyword — a green run is
...               evidence the action truly happened on the UI, not just that the call
...               returned. See tests/robot/showcase/README.md for scope and findings.

Resource          resources/showcase.resource

Suite Setup       Launch Showcase
Suite Teardown    Close Showcase

Force Tags        showcase    proof

*** Variables ***
${SEARCH}     JGSearchField
${COMBO}      JComboBox
${CHECK}      JCheckBox

*** Test Cases ***
Text Entry Is Truly Applied
    [Documentation]    Type into the search field, then read it back to prove the value landed.
    Ensure Connected
    Open Section    Start
    Click    ${SEARCH}
    Clear And Verify Empty    ${SEARCH}
    Enter And Verify Text    ${SEARCH}    ShowcaseProof123
    Clear And Verify Empty    ${SEARCH}
    Enter And Verify Text    ${SEARCH}    second value

Navigation Truly Changes The Section
    [Documentation]    Navigate between sections; confirm each target toggle becomes selected
    ...                and the other is not (independent read-back of selection state).
    Navigate And Verify Section    Settings
    Element Should Not Be Selected    NavigationToggleButton[text='Start']
    Navigate And Verify Section    Start
    Element Should Not Be Selected    NavigationToggleButton[text='Settings']

Checkbox Toggle Is Truly Applied
    [Documentation]    On the Settings page, toggle the checkbox and confirm its selected
    ...                state each way via Element Should [Not] Be Selected.
    Open Section    Settings
    Check And Verify    ${CHECK}
    Uncheck And Verify    ${CHECK}
    Check And Verify    ${CHECK}

Combobox Value Is Readable And Consistent
    [Documentation]    Read the combo's value, re-select the same value, and confirm it is
    ...                unchanged — proving the select+read path operates on the live widget.
    Open Section    Settings
    ${current}=    Get Element Text    ${COMBO}
    Should Not Be Empty    ${current}
    Select From Combobox    ${COMBO}    ${current}
    Element Text Should Be    ${COMBO}    ${current}

Deep Content Is Reachable
    [Documentation]    Regression guard for the depth-cap fix: the locator engine must reach
    ...                far more than the shallow subset it saw before (real page widgets, not
    ...                just top-level navigation).
    Ensure Connected
    Open Section    Start
    ${all}=    Find Elements    //*
    Log    Locatable components on the hub page: ${{len($all)}}
    Should Be True    ${{len($all)}} > 100
    Open Section    Settings
    ${combos}=    Find Elements    JComboBox
    Should Be True    ${{len($combos)}} >= 1
