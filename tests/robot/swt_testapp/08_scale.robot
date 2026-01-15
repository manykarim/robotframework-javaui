*** Settings ***
Documentation    SWT Scale Controls - Testing Scale (slider) widget
...              Operations: Set Value, Get Value, Adjust
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Set Scale Value And Verify
    [Documentation]    Set a value on the scale and verify
    [Tags]    scale    set    positive
    Set Slider Value    Scale[name='volumeScale']    75
    ${value}=    Get Slider Value    Scale[name='volumeScale']
    Should Be Equal As Integers    ${value}    75

Set Scale To Minimum Value
    [Documentation]    Set scale to minimum value
    [Tags]    scale    set    boundary    positive
    Set Slider Value    Scale[name='volumeScale']    0
    ${value}=    Get Slider Value    Scale[name='volumeScale']
    Should Be Equal As Integers    ${value}    0

Set Scale To Maximum Value
    [Documentation]    Set scale to maximum value
    [Tags]    scale    set    boundary    positive
    Set Slider Value    Scale[name='volumeScale']    100
    ${value}=    Get Slider Value    Scale[name='volumeScale']
    Should Be Equal As Integers    ${value}    100

Set Scale Multiple Values
    [Documentation]    Set different values on the scale sequentially
    [Tags]    scale    set    sequence    positive
    Set Slider Value    Scale[name='volumeScale']    25
    ${value}=    Get Slider Value    Scale[name='volumeScale']
    Should Be Equal As Integers    ${value}    25
    Set Slider Value    Scale[name='volumeScale']    50
    ${value}=    Get Slider Value    Scale[name='volumeScale']
    Should Be Equal As Integers    ${value}    50
    Set Slider Value    Scale[name='volumeScale']    75
    ${value}=    Get Slider Value    Scale[name='volumeScale']
    Should Be Equal As Integers    ${value}    75

Scale Should Be Enabled
    [Documentation]    Verify scale is enabled
    [Tags]    scale    enabled    verification
    Element Should Be Enabled    Scale[name='volumeScale']

Scale Should Be Visible
    [Documentation]    Verify scale is visible
    [Tags]    scale    visible    verification
    Element Should Be Visible    Scale[name='volumeScale']
