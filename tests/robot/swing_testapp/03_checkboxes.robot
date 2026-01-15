*** Settings ***
Documentation    Checkbox Controls - Testing JCheckBox check/uncheck/verify operations
...              Operations: Check, Uncheck, Verify Selected/Not Selected
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Check Checkbox And Verify Selected
    [Documentation]    Check a checkbox and verify it is selected
    [Tags]    checkbox    check    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Uncheck Checkbox    JCheckBox[name='notificationsCheckBox']
    Check Checkbox    JCheckBox[name='notificationsCheckBox']
    Element Should Be Selected    JCheckBox[name='notificationsCheckBox']

Uncheck Checkbox And Verify Not Selected
    [Documentation]    Uncheck a checkbox and verify it is not selected
    [Tags]    checkbox    uncheck    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Check Checkbox    JCheckBox[name='autoSaveCheckBox']
    Uncheck Checkbox    JCheckBox[name='autoSaveCheckBox']
    Element Should Not Be Selected    JCheckBox[name='autoSaveCheckBox']

Toggle Checkbox Multiple Times
    [Documentation]    Toggle checkbox on and off multiple times
    [Tags]    checkbox    toggle    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Check Checkbox    JCheckBox[name='notificationsCheckBox']
    Element Should Be Selected    JCheckBox[name='notificationsCheckBox']
    Uncheck Checkbox    JCheckBox[name='notificationsCheckBox']
    Element Should Not Be Selected    JCheckBox[name='notificationsCheckBox']
    Check Checkbox    JCheckBox[name='notificationsCheckBox']
    Element Should Be Selected    JCheckBox[name='notificationsCheckBox']

Check All Checkboxes
    [Documentation]    Check all checkboxes in the options panel
    [Tags]    checkbox    check    multiple    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Check Checkbox    JCheckBox[name='enabledCheckBox']
    Check Checkbox    JCheckBox[name='notificationsCheckBox']
    Check Checkbox    JCheckBox[name='autoSaveCheckBox']
    Element Should Be Selected    JCheckBox[name='enabledCheckBox']
    Element Should Be Selected    JCheckBox[name='notificationsCheckBox']
    Element Should Be Selected    JCheckBox[name='autoSaveCheckBox']

Uncheck All Checkboxes
    [Documentation]    Uncheck all checkboxes in the options panel
    [Tags]    checkbox    uncheck    multiple    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Uncheck Checkbox    JCheckBox[name='enabledCheckBox']
    Uncheck Checkbox    JCheckBox[name='notificationsCheckBox']
    Uncheck Checkbox    JCheckBox[name='autoSaveCheckBox']
    Element Should Not Be Selected    JCheckBox[name='enabledCheckBox']
    Element Should Not Be Selected    JCheckBox[name='notificationsCheckBox']
    Element Should Not Be Selected    JCheckBox[name='autoSaveCheckBox']

Checkbox Should Be Enabled
    [Documentation]    Verify checkbox is enabled
    [Tags]    checkbox    enabled    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Be Enabled    JCheckBox[name='enabledCheckBox']

Settings Tab Checkboxes
    [Documentation]    Test checkboxes on the Settings tab
    [Tags]    checkbox    settings    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Settings
    Sleep    0.3s
    Check Checkbox    JCheckBox[name='wordWrapCheckBox']
    Element Should Be Selected    JCheckBox[name='wordWrapCheckBox']
    Uncheck Checkbox    JCheckBox[name='wordWrapCheckBox']
    Element Should Not Be Selected    JCheckBox[name='wordWrapCheckBox']
