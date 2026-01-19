*** Settings ***
Test Timeout       60s
Documentation     Spinner and Slider Tests - Testing JSpinner and JSlider keywords.
...
...               These tests verify the library's ability to interact with
...               JSpinner (numeric input) and JSlider (range selection) components.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        spinner    slider    regression

*** Test Cases ***
# =============================================================================
# SPINNER TESTS
# =============================================================================

Set Spinner Value And Verify
    [Documentation]    Set a value in the spinner and verify.
    [Tags]    smoke    positive    spinner
    Select Selections Tab
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    25
    ${value}=    Get Element Text    JSpinner[name='quantitySpinner']
    Should Contain    ${value}    25

Set Spinner To Minimum Value
    [Documentation]    Set spinner to minimum value (0).
    [Tags]    positive    spinner    boundary
    Select Selections Tab
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    0
    ${value}=    Get Element Text    JSpinner[name='quantitySpinner']
    Should Contain    ${value}    0

Set Spinner To Maximum Value
    [Documentation]    Set spinner to maximum value (100).
    [Tags]    positive    spinner    boundary
    Select Selections Tab
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    100
    ${value}=    Get Element Text    JSpinner[name='quantitySpinner']
    Should Contain    ${value}    100

Set Spinner Multiple Values Sequentially
    [Documentation]    Set different values in the spinner sequentially.
    [Tags]    positive    spinner
    Select Selections Tab
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    10
    ${value}=    Get Element Text    JSpinner[name='quantitySpinner']
    Should Contain    ${value}    10
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    50
    ${value}=    Get Element Text    JSpinner[name='quantitySpinner']
    Should Contain    ${value}    50
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    75
    ${value}=    Get Element Text    JSpinner[name='quantitySpinner']
    Should Contain    ${value}    75

Spinner Should Be Enabled
    [Documentation]    Verify spinner is enabled.
    [Tags]    positive    spinner    verification
    Select Selections Tab
    Element Should Be Enabled    JSpinner[name='quantitySpinner']

Spinner Should Be Visible
    [Documentation]    Verify spinner is visible.
    [Tags]    positive    spinner    verification
    Select Selections Tab
    Element Should Be Visible    JSpinner[name='quantitySpinner']

Settings Tab Font Size Spinner
    [Documentation]    Test font size spinner on Settings tab.
    [Tags]    positive    spinner    settings
    Select Settings Tab
    Clear Text    JSpinner[name='fontSizeSpinner']
    Input Text    JSpinner[name='fontSizeSpinner']    16
    ${value}=    Get Element Text    JSpinner[name='fontSizeSpinner']
    Should Contain    ${value}    16

# =============================================================================
# SLIDER TESTS
# =============================================================================

Get Slider Initial Value
    [Documentation]    Get the initial value of the slider.
    [Tags]    smoke    positive    slider
    Select Selections Tab
    ${value}=    Get Element Property    JSlider[name='volumeSlider']    value
    Should Not Be Equal    ${value}    ${NONE}
    Should Be True    ${value} >= 0

Click Slider And Verify Change
    [Documentation]    Click on the slider and verify value changes.
    [Tags]    positive    slider
    Select Selections Tab
    ${initial}=    Get Element Property    JSlider[name='volumeSlider']    value
    Click Element    JSlider[name='volumeSlider']
    Sleep    0.2s
    ${after}=    Get Element Property    JSlider[name='volumeSlider']    value
    Should Not Be Equal    ${after}    ${NONE}
    Should Be True    ${after} >= 0

Slider Should Be Enabled
    [Documentation]    Verify slider is enabled.
    [Tags]    positive    slider    verification
    Select Selections Tab
    Element Should Be Enabled    JSlider[name='volumeSlider']

Slider Should Be Visible
    [Documentation]    Verify slider is visible.
    [Tags]    positive    slider    verification
    Select Selections Tab
    Element Should Be Visible    JSlider[name='volumeSlider']

Verify Slider Properties
    [Documentation]    Verify slider has expected properties.
    [Tags]    positive    slider    properties
    Select Selections Tab
    ${min}=    Get Element Property    JSlider[name='volumeSlider']    minimum
    ${max}=    Get Element Property    JSlider[name='volumeSlider']    maximum
    Should Be Equal As Integers    ${min}    0
    Should Be Equal As Integers    ${max}    100

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Set Nonexistent Spinner Fails
    [Documentation]    Setting value on non-existent spinner throws error.
    [Tags]    negative    error-handling    spinner
    ${status}=    Run Keyword And Return Status
    ...    Input Text    JSpinner[name='nonexistent']    10
    Should Be Equal    ${status}    ${FALSE}

Get Nonexistent Slider Value Fails
    [Documentation]    Getting value from non-existent slider throws error.
    [Tags]    negative    error-handling    slider
    ${status}=    Run Keyword And Return Status
    ...    Get Element Property    JSlider[name='nonexistent']    value
    Should Be Equal    ${status}    ${FALSE}
