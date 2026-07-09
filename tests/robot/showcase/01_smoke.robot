*** Settings ***
Documentation     Smoke proof: connect to the real Smart Client Showcase, confirm the
...               locator engine can reach deeply-nested content, and validate navigation.

Resource          resources/showcase.resource

Suite Setup       Launch Showcase
Suite Teardown    Close Showcase

Force Tags        showcase    smoke

*** Test Cases ***
Locator Engine Reaches Deep Content
    [Documentation]    After the depth fix, find must see far more than the shallow subset
    ...                and reach real page widgets (not just top-level nav).
    ${all}=    Find Elements    //*
    Log    Total locatable components: ${{len($all)}}
    Should Be True    ${{len($all)}} > 100    Locator engine still shallow; deep content unreachable

Navigation Is Validated
    [Documentation]    Navigating to Settings is confirmed by the toggle being selected.
    Navigate And Verify Section    Settings

Section Content Is Locatable
    [Documentation]    The Settings page hosts a combo box + checkbox deep in the tree;
    ...                confirm they are now locatable.
    Open Section    Settings
    ${combos}=    Find Elements    JComboBox
    ${checks}=    Find Elements    JCheckBox
    Should Be True    ${{len($combos)}} >= 1    No JComboBox reachable on Settings page
    Should Be True    ${{len($checks)}} >= 1    No JCheckBox reachable on Settings page
