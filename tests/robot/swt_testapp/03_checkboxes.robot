*** Settings ***
Documentation    SWT Checkbox Controls - Testing Button with SWT.CHECK style
...              Operations: Check, Uncheck, Verify Selected/Not Selected
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Check Checkbox And Verify Selected
    [Documentation]    Check a checkbox and verify it is selected
    [Tags]    checkbox    check    positive
    Uncheck Checkbox    Button[name='enabledCheckbox']
    Check Checkbox    Button[name='enabledCheckbox']
    Element Should Be Selected    Button[name='enabledCheckbox']

Uncheck Checkbox And Verify Not Selected
    [Documentation]    Uncheck a checkbox and verify it is not selected
    [Tags]    checkbox    uncheck    positive
    Check Checkbox    Button[name='notificationsCheckbox']
    Uncheck Checkbox    Button[name='notificationsCheckbox']
    Element Should Not Be Selected    Button[name='notificationsCheckbox']

Toggle Checkbox Multiple Times
    [Documentation]    Toggle checkbox on and off multiple times
    [Tags]    checkbox    toggle    positive
    # First toggle: Check
    Check Checkbox    Button[name='autoSaveCheckbox']
    Element Should Be Selected    Button[name='autoSaveCheckbox']
    # Second toggle: Uncheck
    Uncheck Checkbox    Button[name='autoSaveCheckbox']
    Element Should Not Be Selected    Button[name='autoSaveCheckbox']
    # Third toggle: Check again
    Check Checkbox    Button[name='autoSaveCheckbox']
    Element Should Be Selected    Button[name='autoSaveCheckbox']

Check All Checkboxes
    [Documentation]    Check all checkboxes in the options panel
    [Tags]    checkbox    check    multiple    positive
    Check Checkbox    Button[name='enabledCheckbox']
    Check Checkbox    Button[name='notificationsCheckbox']
    Check Checkbox    Button[name='autoSaveCheckbox']
    Element Should Be Selected    Button[name='enabledCheckbox']
    Element Should Be Selected    Button[name='notificationsCheckbox']
    Element Should Be Selected    Button[name='autoSaveCheckbox']

Uncheck All Checkboxes
    [Documentation]    Uncheck all checkboxes
    [Tags]    checkbox    uncheck    multiple    positive
    Uncheck Checkbox    Button[name='enabledCheckbox']
    Uncheck Checkbox    Button[name='notificationsCheckbox']
    Uncheck Checkbox    Button[name='autoSaveCheckbox']
    Element Should Not Be Selected    Button[name='enabledCheckbox']
    Element Should Not Be Selected    Button[name='notificationsCheckbox']
    Element Should Not Be Selected    Button[name='autoSaveCheckbox']

Checkbox Should Be Enabled
    [Documentation]    Verify checkbox is enabled
    [Tags]    checkbox    enabled    verification
    Element Should Be Enabled    Button[name='enabledCheckbox']

Checkbox Should Be Visible
    [Documentation]    Verify checkbox is visible
    [Tags]    checkbox    visible    verification
    Element Should Be Visible    Button[name='enabledCheckbox']
