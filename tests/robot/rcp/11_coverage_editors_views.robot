*** Settings ***
Test Timeout       60s
Documentation     E2E coverage for RCP editor/view/perspective keywords that previously
...               had no live-app test (see scripts/keyword_coverage.py). Runs against the
...               mock RCP application.

Resource          resources/common.resource

Suite Setup       Suite Setup Start RCP App
Suite Teardown    Suite Teardown Stop RCP App
Test Setup        Test Setup Reset State
Test Teardown     Test Teardown Cleanup

Force Tags        rcp    coverage

*** Test Cases ***
View Open And Title Getters
    Show View    ${CONSOLE_VIEW}
    View Should Be Open    ${CONSOLE_VIEW}
    ${title}=    Get View Title    ${CONSOLE_VIEW}
    Should Be True    isinstance($title, str)

View Should Not Be Open For Unopened View
    View Should Not Be Open    ${PACKAGE_EXPLORER_VIEW}

Editor Title Getters
    Open Editor    ${TEST_FILE_JAVA}
    ${active}=    Get Active Editor Title
    Should Be True    isinstance($active, str)
    ${titles}=    Get Open Editor Titles
    Should Be True    isinstance($titles, list)

Editor Dirty State Getters
    Open Editor    ${TEST_FILE_JAVA}
    ${dirty}=    Is Editor Dirty    ${TEST_FILE_JAVA}
    Should Be Equal    ${dirty}    ${False}
    ${state}=    Get Editor Dirty State    ${TEST_FILE_JAVA}
    Should Be Equal    ${state}    ${False}
    ${count}=    Get Dirty Editor Count
    Should Be True    ${count} >= 0

Editor Should Not Be Open For Unopened File
    Editor Should Not Be Open    ${TEST_FILE_XML}

Active Perspective Assertion
    Open Perspective    ${RESOURCE_PERSPECTIVE}
    Perspective Should Be Active    ${RESOURCE_PERSPECTIVE}
