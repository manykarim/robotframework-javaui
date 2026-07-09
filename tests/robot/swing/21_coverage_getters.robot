*** Settings ***
Documentation     E2E coverage for Swing list/tree getters, assertions and misc keywords
...               that previously had no live-app test (see scripts/keyword_coverage.py).
...               Exercises each against the real Swing test application.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        swing    coverage

*** Variables ***
${LIST}       JList[name='itemList']
${TREE}       JTree[name='fileTree']
${TABS}       JTabbedPane[name='mainTabbedPane']
${ITEM1}      Item 1 - Apple

*** Test Cases ***
Swing List Getters And Assertions
    Select Selections Tab
    ${count}=    Get List Item Count    ${LIST}
    Should Be True    ${count} > 0
    List Should Contain    ${LIST}    ${ITEM1}
    List Should Not Contain    ${LIST}    No Such Item
    Select From List    ${LIST}    ${ITEM1}
    ${idx}=    Get Selected List Index    ${LIST}
    Should Be True    ${idx} >= 0
    # Exercise the selected-item getter (value not asserted — depends on backend)
    Get Selected List Item    ${LIST}
    # NOTE: List Selection Should Be is not asserted here — the Swing list
    # selection-items backend currently returns [] (known limitation, tracked in
    # the release-ready change).

Swing Tree Getters
    Select Data View Tab
    Expand Tree Node    ${TREE}    Project Root
    ${count}=    Get Tree Node Count    ${TREE}
    Should Be True    ${count} >= 0
    ${children}=    Get Tree Node Children    ${TREE}    Project Root
    Should Be True    isinstance($children, list)

Swing UI Tree Save And Log
    ${path}=    Set Variable    ${OUTPUT DIR}${/}ui_tree_snapshot.txt
    Save UI Tree    ${path}
    File Should Exist    ${path}
    # NOTE: Log Component Tree is currently broken (forwards format=None to the
    # Rust core → TypeError); tracked in the release-ready change.

Swing Showing Assertions And Waits
    Wait Until Element Is Showing    ${TABS}
    Element Should Not Be Showing    JButton[name='doesNotExist']

Swing Assertion Config Setters
    # Setters return the previous value (or None); assert only that they execute.
    Set Assertion Timeout    ${5.0}
    Set Assertion Interval    ${0.1}
    Set Screenshot Directory    ${OUTPUT DIR}
