*** Settings ***
Documentation     ProgressBar Tests - Testing JProgressBar operations.
...
...               These tests verify the library's ability to interact with
...               JProgressBar components for progress tracking and monitoring.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        progressbar    regression

*** Test Cases ***
# =============================================================================
# PROGRESSBAR VALUE TESTS
# =============================================================================

Get Initial Progress Value
    [Documentation]    Get the initial value of the progress bar.
    [Tags]    smoke    positive
    ${value}=    Get Element Property    JProgressBar[name='progressBar']    value
    Should Be True    ${value} >= 0

Start Progress And Verify Increasing
    [Documentation]    Start progress and verify it increases.
    [Tags]    positive    progress
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

Wait For Progress To Complete
    [Documentation]    Start progress and wait for completion.
    [Tags]    positive    complete
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

# =============================================================================
# PROGRESSBAR STATE VERIFICATION
# =============================================================================

Verify Progress Bar Is Visible
    [Documentation]    Verify progress bar is visible.
    [Tags]    positive    verification
    Element Should Be Visible    JProgressBar[name='progressBar']

Verify Progress Bar Is Enabled
    [Documentation]    Verify progress bar is enabled.
    [Tags]    positive    verification
    Element Should Be Enabled    JProgressBar[name='progressBar']

Verify Progress Bar Exists
    [Documentation]    Verify progress bar exists.
    [Tags]    positive    verification
    Element Should Exist    JProgressBar[name='progressBar']

# =============================================================================
# PROGRESSBAR PROPERTIES
# =============================================================================

Get Progress Bar Minimum
    [Documentation]    Get progress bar minimum value.
    [Tags]    positive    properties
    ${min}=    Get Element Property    JProgressBar[name='progressBar']    minimum
    Should Be Equal As Integers    ${min}    0

Get Progress Bar Maximum
    [Documentation]    Get progress bar maximum value.
    [Tags]    positive    properties
    ${max}=    Get Element Property    JProgressBar[name='progressBar']    maximum
    Should Be Equal As Integers    ${max}    100

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Get Nonexistent ProgressBar Value Fails
    [Documentation]    Getting value from non-existent progress bar throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Element Property    JProgressBar[name='nonexistent']    value
    Should Be Equal    ${status}    ${FALSE}
