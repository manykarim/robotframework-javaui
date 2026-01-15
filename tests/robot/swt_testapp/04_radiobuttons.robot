*** Settings ***
Documentation    SWT Radio Button Controls - Testing Button with SWT.RADIO style
...              Operations: Select, Verify Selected, Verify Mutual Exclusion
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Select Radio Button And Verify
    [Documentation]    Select a radio button and verify it is selected
    [Tags]    radio    select    positive
    Select Radio Button    Button[name='highPriorityRadio']
    Element Should Be Selected    Button[name='highPriorityRadio']

Select Different Radio Button In Same Group
    [Documentation]    Select different radio buttons and verify mutual exclusion
    [Tags]    radio    select    exclusion    positive
    Select Radio Button    Button[name='highPriorityRadio']
    Element Should Be Selected    Button[name='highPriorityRadio']
    Select Radio Button    Button[name='lowPriorityRadio']
    Element Should Be Selected    Button[name='lowPriorityRadio']
    Element Should Not Be Selected    Button[name='highPriorityRadio']

Select All Radio Buttons In Sequence
    [Documentation]    Select each radio button in sequence and verify selection
    [Tags]    radio    select    sequence    positive
    Select Radio Button    Button[name='highPriorityRadio']
    Element Should Be Selected    Button[name='highPriorityRadio']
    Select Radio Button    Button[name='normalPriorityRadio']
    Element Should Be Selected    Button[name='normalPriorityRadio']
    Element Should Not Be Selected    Button[name='highPriorityRadio']
    Select Radio Button    Button[name='lowPriorityRadio']
    Element Should Be Selected    Button[name='lowPriorityRadio']
    Element Should Not Be Selected    Button[name='normalPriorityRadio']
    Element Should Not Be Selected    Button[name='highPriorityRadio']

Radio Button Should Be Enabled
    [Documentation]    Verify radio button is enabled
    [Tags]    radio    enabled    verification
    Element Should Be Enabled    Button[name='highPriorityRadio']
    Element Should Be Enabled    Button[name='normalPriorityRadio']
    Element Should Be Enabled    Button[name='lowPriorityRadio']

Radio Button Should Be Visible
    [Documentation]    Verify radio buttons are visible
    [Tags]    radio    visible    verification
    Element Should Be Visible    Button[name='highPriorityRadio']
    Element Should Be Visible    Button[name='normalPriorityRadio']
    Element Should Be Visible    Button[name='lowPriorityRadio']
