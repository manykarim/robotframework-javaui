*** Settings ***
Documentation    Spinner Controls - Testing JSpinner value operations
...              Operations: Input Value, Get Value
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Set Spinner Value And Verify
    [Documentation]    Set a value in the spinner and verify
    [Tags]    spinner    set    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    25
    ${value}=    Get Element Text    JSpinner[name='quantitySpinner']
    Should Contain    ${value}    25

Set Spinner To Minimum Value
    [Documentation]    Set spinner to minimum value (0)
    [Tags]    spinner    set    boundary    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    0
    ${value}=    Get Element Text    JSpinner[name='quantitySpinner']
    Should Contain    ${value}    0

Set Spinner To Maximum Value
    [Documentation]    Set spinner to maximum value (100)
    [Tags]    spinner    set    boundary    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    100
    ${value}=    Get Element Text    JSpinner[name='quantitySpinner']
    Should Contain    ${value}    100

Set Spinner Multiple Values
    [Documentation]    Set different values in the spinner sequentially
    [Tags]    spinner    set    sequence    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    # Set to 10
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    10
    ${value}=    Get Element Text    JSpinner[name='quantitySpinner']
    Should Contain    ${value}    10
    # Set to 50
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    50
    ${value}=    Get Element Text    JSpinner[name='quantitySpinner']
    Should Contain    ${value}    50
    # Set to 75
    Clear Text    JSpinner[name='quantitySpinner']
    Input Text    JSpinner[name='quantitySpinner']    75
    ${value}=    Get Element Text    JSpinner[name='quantitySpinner']
    Should Contain    ${value}    75

Spinner Should Be Enabled
    [Documentation]    Verify spinner is enabled
    [Tags]    spinner    enabled    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Be Enabled    JSpinner[name='quantitySpinner']

Spinner Should Be Visible
    [Documentation]    Verify spinner is visible
    [Tags]    spinner    visible    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Be Visible    JSpinner[name='quantitySpinner']

Settings Tab Font Size Spinner
    [Documentation]    Test font size spinner on Settings tab
    [Tags]    spinner    settings    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Settings
    Sleep    0.3s
    Clear Text    JSpinner[name='fontSizeSpinner']
    Input Text    JSpinner[name='fontSizeSpinner']    16
    ${value}=    Get Element Text    JSpinner[name='fontSizeSpinner']
    Should Contain    ${value}    16
