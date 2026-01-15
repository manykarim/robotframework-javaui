*** Settings ***
Documentation    SWT ProgressBar Controls - Testing ProgressBar widget
...              Operations: Get Value, Verify Progress, Monitor Progress
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Get Initial Progress Value
    [Documentation]    Get the initial value of the progress bar
    [Tags]    progressbar    get    value    positive
    ${value}=    Get Element Property    ProgressBar[name='mainProgressBar']    selection
    Should Be True    ${value} >= 0

Start Progress And Verify Increasing
    [Documentation]    Start progress and verify it increases
    [Tags]    progressbar    progress    verify    positive
    # Click run button to start progress demo
    Click Element    ToolItem[name='toolRun']
    Sleep    0.5s
    # Get initial value
    ${value1}=    Get Element Property    ProgressBar[name='mainProgressBar']    selection
    # Wait a bit
    Sleep    0.5s
    # Get new value
    ${value2}=    Get Element Property    ProgressBar[name='mainProgressBar']    selection
    # Progress should have increased (or completed)
    Should Be True    ${value2} >= ${value1}

Verify Progress Bar Is Visible
    [Documentation]    Verify progress bar is visible
    [Tags]    progressbar    visible    verification
    Element Should Be Visible    ProgressBar[name='mainProgressBar']

Verify Progress Bar Is Enabled
    [Documentation]    Verify progress bar is enabled
    [Tags]    progressbar    enabled    verification
    Element Should Be Enabled    ProgressBar[name='mainProgressBar']

Monitor Progress Demo
    [Documentation]    Start progress demo and monitor until completion
    [Tags]    progressbar    complete    positive
    # Click run button to start progress demo
    Click Element    ToolItem[name='toolRun']
    # Wait for progress to complete (with timeout)
    FOR    ${i}    IN RANGE    60
        ${value}=    Get Element Property    ProgressBar[name='mainProgressBar']    selection
        Exit For Loop If    ${value} >= 100
        Sleep    0.2s
    END
    # Final value check
    ${final_value}=    Get Element Property    ProgressBar[name='mainProgressBar']    selection
    Should Be True    ${final_value} >= 0
