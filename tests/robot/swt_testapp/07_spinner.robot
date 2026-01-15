*** Settings ***
Documentation    SWT Spinner Controls - Testing Spinner widget
...              Operations: Set Value, Get Value, Increment, Decrement
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Set Spinner Value And Verify
    [Documentation]    Set a value in the spinner and verify
    [Tags]    spinner    set    positive
    Set Spinner Value    Spinner[name='quantitySpinner']    25
    ${value}=    Get Spinner Value    Spinner[name='quantitySpinner']
    Should Be Equal As Integers    ${value}    25

Set Spinner To Minimum Value
    [Documentation]    Set spinner to minimum value
    [Tags]    spinner    set    boundary    positive
    Set Spinner Value    Spinner[name='quantitySpinner']    0
    ${value}=    Get Spinner Value    Spinner[name='quantitySpinner']
    Should Be Equal As Integers    ${value}    0

Set Spinner To Maximum Value
    [Documentation]    Set spinner to maximum value
    [Tags]    spinner    set    boundary    positive
    Set Spinner Value    Spinner[name='quantitySpinner']    100
    ${value}=    Get Spinner Value    Spinner[name='quantitySpinner']
    Should Be Equal As Integers    ${value}    100

Set Spinner Multiple Values
    [Documentation]    Set different values in the spinner sequentially
    [Tags]    spinner    set    sequence    positive
    Set Spinner Value    Spinner[name='quantitySpinner']    10
    ${value}=    Get Spinner Value    Spinner[name='quantitySpinner']
    Should Be Equal As Integers    ${value}    10
    Set Spinner Value    Spinner[name='quantitySpinner']    50
    ${value}=    Get Spinner Value    Spinner[name='quantitySpinner']
    Should Be Equal As Integers    ${value}    50
    Set Spinner Value    Spinner[name='quantitySpinner']    75
    ${value}=    Get Spinner Value    Spinner[name='quantitySpinner']
    Should Be Equal As Integers    ${value}    75

Spinner Should Be Enabled
    [Documentation]    Verify spinner is enabled
    [Tags]    spinner    enabled    verification
    Element Should Be Enabled    Spinner[name='quantitySpinner']

Spinner Should Be Visible
    [Documentation]    Verify spinner is visible
    [Tags]    spinner    visible    verification
    Element Should Be Visible    Spinner[name='quantitySpinner']
