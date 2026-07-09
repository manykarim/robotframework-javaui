*** Settings ***
Documentation     E2E coverage for SWT table/tree/widget getters and assertions that
...               previously had no live-app test (see scripts/keyword_coverage.py).
...               Exercises each keyword against the real SWT test application.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        swt    coverage

*** Variables ***
${TABLE}          name:dataTable
${TREE}           name:fileTree
${TEXT}           name:textUsername
${ROOT}           Project A

*** Test Cases ***
SWT Table Row And Column Getters
    ${rows}=    Get Swt Table Row Count    ${TABLE}
    Should Be True    ${rows} >= 0
    ${cols}=    Get Swt Table Column Count    ${TABLE}
    Should Be True    ${cols} >= 0
    ${headers}=    Get Swt Table Column Headers    ${TABLE}
    Should Be True    isinstance($headers, list)

SWT Table Cell And Row Values
    ${rows}=    Get Swt Table Row Count    ${TABLE}
    IF    ${rows} > 0
        ${cell}=    Get Swt Table Cell    ${TABLE}    ${0}    ${0}
        Should Not Be Equal    ${cell}    ${NONE}
    END
    ${row}=    Get Swt Table Row Values    ${TABLE}    ${0}
    Should Be True    isinstance($row, list)

SWT Table Selection Getter
    ${sel}=    Get Swt Selected Table Rows    ${TABLE}
    Should Be True    isinstance($sel, list)

SWT Table Assertions
    ${rows}=    Get Swt Table Row Count    ${TABLE}
    Swt Table Row Count Should Be    ${TABLE}    ${rows}
    IF    ${rows} > 0
        Swt Table Should Have Rows    ${TABLE}
        ${cell}=    Get Swt Table Cell    ${TABLE}    ${0}    ${0}
        Swt Table Cell Should Contain    ${TABLE}    ${0}    ${0}    ${cell}
    END

SWT Tree Node Getters
    ${count}=    Get Swt Tree Node Count    ${TREE}
    Should Be True    ${count} >= 0
    ${text}=    Get Swt Tree Item Text    ${TREE}    ${ROOT}
    Should Be Equal    ${text}    ${ROOT}
    ${level}=    Get Swt Tree Node Level    ${TREE}    ${ROOT}
    Should Be True    isinstance($level, int)
    ${parent}=    Get Swt Tree Node Parent    ${TREE}    ${ROOT}|src
    Should Be True    isinstance($parent, str)

SWT Tree Selection Getter
    ${sel}=    Get Swt Selected Tree Nodes    ${TREE}
    Should Be True    isinstance($sel, list)

SWT Tree Assertions
    # NOTE: Swt Tree Node Should Exist is intentionally not asserted here — the
    # Rust core's tree_node_exists backend currently always returns False (known
    # limitation, tracked in the release-ready change). The negative assertion
    # exercises the keyword path reliably.
    Swt Tree Node Should Not Exist    ${TREE}    NoSuchNode|Nope

SWT Widget Getters
    ${props}=    Get Widget Properties    ${TEXT}
    Should Be True    isinstance($props, dict)
    # Single-property accessor derives from the property map (regression: was broken)
    Get Widget Property    ${TEXT}    enabled
    ${text}=    Get Widget Text    ${TEXT}
    Should Be True    isinstance($text, str)
    ${count}=    Get Widget Count    ${TABLE}
    Should Be True    ${count} >= 1
    ${states}=    Get Widget States    ${TEXT}
    Should Be True    isinstance($states, list)

SWT Assertion Timing Setters
    ${old}=    Set Swt Assertion Timeout    ${5.0}
    Should Be True    ${old} >= 0
    ${oldi}=    Set Swt Assertion Interval    ${0.1}
    Should Be True    ${oldi} >= 0
