*** Settings ***
Documentation    Radio Button Controls - Testing JRadioButton selection operations
...              Operations: Select, Verify Selected, Verify Mutual Exclusion
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Select Radio Button And Verify
    [Documentation]    Select a radio button and verify it is selected
    [Tags]    radio    select    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Select Radio Button    JRadioButton[name='highPriorityRadioButton']
    Element Should Be Selected    JRadioButton[name='highPriorityRadioButton']

Select Different Radio Button In Same Group
    [Documentation]    Select different radio buttons and verify mutual exclusion
    [Tags]    radio    select    exclusion    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Select Radio Button    JRadioButton[name='highPriorityRadioButton']
    Element Should Be Selected    JRadioButton[name='highPriorityRadioButton']
    Select Radio Button    JRadioButton[name='lowPriorityRadioButton']
    Element Should Be Selected    JRadioButton[name='lowPriorityRadioButton']
    Element Should Not Be Selected    JRadioButton[name='highPriorityRadioButton']

Select All Radio Buttons In Sequence
    [Documentation]    Select each radio button in sequence and verify selection
    [Tags]    radio    select    sequence    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Select Radio Button    JRadioButton[name='highPriorityRadioButton']
    Element Should Be Selected    JRadioButton[name='highPriorityRadioButton']
    Select Radio Button    JRadioButton[name='normalPriorityRadioButton']
    Element Should Be Selected    JRadioButton[name='normalPriorityRadioButton']
    Element Should Not Be Selected    JRadioButton[name='highPriorityRadioButton']
    Select Radio Button    JRadioButton[name='lowPriorityRadioButton']
    Element Should Be Selected    JRadioButton[name='lowPriorityRadioButton']

Radio Button Should Be Enabled
    [Documentation]    Verify radio button is enabled
    [Tags]    radio    enabled    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Be Enabled    JRadioButton[name='highPriorityRadioButton']
    Element Should Be Enabled    JRadioButton[name='normalPriorityRadioButton']
    Element Should Be Enabled    JRadioButton[name='lowPriorityRadioButton']

Radio Button Should Be Visible
    [Documentation]    Verify radio buttons are visible
    [Tags]    radio    visible    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Be Visible    JRadioButton[name='highPriorityRadioButton']
    Element Should Be Visible    JRadioButton[name='normalPriorityRadioButton']
    Element Should Be Visible    JRadioButton[name='lowPriorityRadioButton']
