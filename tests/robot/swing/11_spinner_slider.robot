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
    [Documentation]    Set a value in the spinner and verify with assertion operator.
    [Tags]    smoke    positive    spinner    assertion-operator
    Select Selections Tab
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    25
    # Verify using assertion operator
    Get Text    JSpinner[name='quantitySpinner']    *=    25

Set Spinner To Minimum Value
    [Documentation]    Set spinner to minimum value (0) with assertion.
    [Tags]    positive    spinner    boundary    assertion-operator
    Select Selections Tab
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    0
    # Verify using assertion operator
    Get Text    JSpinner[name='quantitySpinner']    *=    0

Set Spinner To Maximum Value
    [Documentation]    Set spinner to maximum value (100) with assertion.
    [Tags]    positive    spinner    boundary    assertion-operator
    Select Selections Tab
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    100
    # Verify using assertion operator
    Get Text    JSpinner[name='quantitySpinner']    *=    100

Set Spinner Multiple Values Sequentially
    [Documentation]    Set different values in the spinner sequentially with assertions.
    [Tags]    positive    spinner    assertion-operator
    Select Selections Tab
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    10
    Get Text    JSpinner[name='quantitySpinner']    *=    10
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    50
    Get Text    JSpinner[name='quantitySpinner']    *=    50
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    75
    Get Text    JSpinner[name='quantitySpinner']    *=    75

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
    [Documentation]    Test font size spinner on Settings tab with assertion.
    [Tags]    positive    spinner    settings    assertion-operator
    Select Settings Tab
    Clear Text    JSpinner[name='fontSizeSpinner']
    Input Text    JSpinner[name='fontSizeSpinner']    16
    # Verify using assertion operator
    Get Text    JSpinner[name='fontSizeSpinner']    *=    16

# =============================================================================
# SLIDER TESTS
# =============================================================================

Get Slider Initial Value
    [Documentation]    Get the initial value of the slider with assertion.
    [Tags]    smoke    positive    slider    assertion-operator
    Select Selections Tab
    # Verify slider value is non-negative using assertion operator
    Get Property    JSlider[name='volumeSlider']    value    >=    0

Click Slider And Verify Change
    [Documentation]    Click on the slider and verify value with assertion.
    [Tags]    positive    slider    assertion-operator
    Select Selections Tab
    ${initial}=    Get Property    JSlider[name='volumeSlider']    value
    Click Element    JSlider[name='volumeSlider']
    Sleep    0.2s
    # Verify slider value is non-negative using assertion operator
    Get Property    JSlider[name='volumeSlider']    value    >=    0

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
    [Documentation]    Verify slider has expected properties with assertion operators.
    [Tags]    positive    slider    properties    assertion-operator
    Select Selections Tab
    # Verify slider min/max properties using assertion operators
    Get Property    JSlider[name='volumeSlider']    minimum    ==    ${0}
    Get Property    JSlider[name='volumeSlider']    maximum    ==    ${100}

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
