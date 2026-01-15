*** Settings ***
Documentation    ProgressBar Controls - Testing JProgressBar operations
...              Operations: Get Value, Verify Progress, Monitor Progress
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Get Initial Progress Value
    [Documentation]    Get the initial value of the progress bar
    [Tags]    progressbar    get    value    positive
    ${value}=    Get Element Property    JProgressBar[name='progressBar']    value
    Should Be True    ${value} >= 0

Start Progress And Verify Increasing
    [Documentation]    Start progress and verify it increases
    [Tags]    progressbar    progress    verify    positive
    # Click start progress button
    Click Element    JButton[name='startProgressButton']
    Sleep    0.5s
    # Get initial value
    ${value1}=    Get Element Property    JProgressBar[name='progressBar']    value
    # Wait a bit
    Sleep    0.5s
    # Get new value
    ${value2}=    Get Element Property    JProgressBar[name='progressBar']    value
    # Progress should have increased (or completed at 100)
    Should Be True    ${value2} >= ${value1}

Verify Progress Bar Is Visible
    [Documentation]    Verify progress bar is visible
    [Tags]    progressbar    visible    verification
    Element Should Be Visible    JProgressBar[name='progressBar']

Verify Progress Bar Is Enabled
    [Documentation]    Verify progress bar is enabled
    [Tags]    progressbar    enabled    verification
    Element Should Be Enabled    JProgressBar[name='progressBar']

Wait For Progress To Complete
    [Documentation]    Start progress and wait for completion
    [Tags]    progressbar    complete    positive
    # Click start progress button
    Click Element    JButton[name='startProgressButton']
    # Wait for progress to complete with robust error handling
    # Progress updates: 50ms interval, +2% each time = 2.5s total
    Sleep    3s    # Wait for progress to complete
    # Retry getting final value (may need multiple attempts due to EDT timing)
    ${success}=    Set Variable    ${FALSE}
    FOR    ${i}    IN RANGE    5
        TRY
            ${final_value}=    Get Element Property    JProgressBar[name='progressBar']    value
            Should Be True    ${final_value} >= 100
            ${success}=    Set Variable    ${TRUE}
            Exit For Loop
        EXCEPT
            Sleep    0.5s
        END
    END
    Should Be True    ${success}    Progress bar should reach 100%
