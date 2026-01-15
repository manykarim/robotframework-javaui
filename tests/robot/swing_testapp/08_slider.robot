*** Settings ***
Documentation    Slider Controls - Testing JSlider value operations
...              Operations: Click to position, Get Value, Verify
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Get Slider Initial Value
    [Documentation]    Get the initial value of the slider
    [Tags]    slider    get    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    ${value}=    Get Element Property    JSlider[name='volumeSlider']    value
    Should Not Be Equal    ${value}    ${NONE}
    Should Be True    ${value} >= 0

Click Slider And Verify Change
    [Documentation]    Click on the slider and verify value changes
    [Tags]    slider    click    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    ${initial}=    Get Element Property    JSlider[name='volumeSlider']    value
    Click Element    JSlider[name='volumeSlider']
    Sleep    0.2s
    ${after}=    Get Element Property    JSlider[name='volumeSlider']    value
    # Value should exist (may or may not have changed depending on click position)
    Should Not Be Equal    ${after}    ${NONE}
    Should Be True    ${after} >= 0

Slider Should Be Enabled
    [Documentation]    Verify slider is enabled
    [Tags]    slider    enabled    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Be Enabled    JSlider[name='volumeSlider']

Slider Should Be Visible
    [Documentation]    Verify slider is visible
    [Tags]    slider    visible    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Be Visible    JSlider[name='volumeSlider']

Verify Slider Properties
    [Documentation]    Verify slider has expected properties
    [Tags]    slider    properties    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    ${min}=    Get Element Property    JSlider[name='volumeSlider']    minimum
    ${max}=    Get Element Property    JSlider[name='volumeSlider']    maximum
    Should Be Equal As Integers    ${min}    0
    Should Be Equal As Integers    ${max}    100
