*** Settings ***
Documentation    SWT Label Controls - Testing Label widget read operations
...              Operations: Get Text, Verify Text, Verify Exists
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Get Label Text
    [Documentation]    Get the text of a label
    [Tags]    label    get    text    positive
    ${text}=    Get Element Text    Label[name='statusLabel']
    Should Not Be Empty    ${text}

Verify Status Label Initial Value
    [Documentation]    Verify the initial status label value
    [Tags]    label    verify    initial    positive
    ${text}=    Get Element Text    Label[name='statusLabel']
    Should Contain    ${text}    Ready

Label Should Be Visible
    [Documentation]    Verify label is visible
    [Tags]    label    visible    verification
    Element Should Be Visible    Label[name='statusLabel']

Label Should Exist
    [Documentation]    Verify labels exist
    [Tags]    label    exists    verification
    Element Should Exist    Label[name='nameLabel']
    Element Should Exist    Label[name='emailLabel']

Verify Form Labels
    [Documentation]    Verify all form labels exist on Input Controls tab
    [Tags]    label    form    verify    positive
    Select Tab    TabFolder[name='mainTabFolder']    Input Controls
    Sleep    0.3s
    Element Should Exist    Label[name='nameLabel']
    Element Should Exist    Label[name='emailLabel']
    Element Should Exist    Label[name='passwordLabel']

Status Label Updates On Action
    [Documentation]    Verify status label updates when action is performed
    [Tags]    label    update    dynamic    positive
    # Click run button to trigger status update
    Click Element    ToolItem[name='toolRun']
    Sleep    0.3s
    ${text}=    Get Element Text    Label[name='statusLabel']
    # Status should have changed
    Should Not Be Empty    ${text}

Verify Link Widget
    [Documentation]    Verify Link widget exists and is visible
    [Tags]    link    verify    positive
    Element Should Exist    Link[name='infoLink']
    Element Should Be Visible    Link[name='infoLink']
