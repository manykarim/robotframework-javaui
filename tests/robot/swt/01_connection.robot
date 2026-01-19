*** Settings ***
Test Timeout       60s
Documentation     Test suite for SWT application connection keywords.
...
...               Tests the following SwtLibrary keywords:
...               - connect_to_swt_application
...               - disconnect
...               - is_connected
...
...               These tests verify connection establishment, disconnection,
...               timeout handling, and connection state management.

Resource          resources/common.resource

Suite Setup       Start App Without Connect
Suite Teardown    Stop Test Application

Force Tags        swt    connection


*** Variables ***
${INVALID_HOST}           192.168.255.255
${INVALID_PORT}           59999
${INVALID_APP}            NonExistentSwtApp


*** Test Cases ***
# ============================================================================
# Positive Test Cases - connect_to_swt_application
# ============================================================================

Connect To SWT Application Successfully
    [Documentation]    Verify successful connection to a running SWT application
    ...                using default host and port.
    [Tags]    smoke    critical    positive
    Connect To SWT Application    ${SWT_APP_NAME}    ${SWT_HOST}    ${SWT_PORT}
    Verify Connection Established
    [Teardown]    Disconnect From Test App

Connect With Explicit Host And Port
    [Documentation]    Verify connection with explicitly specified host and port.
    [Tags]    smoke    positive
    Connect To SWT Application    ${SWT_APP_NAME}    host=${SWT_HOST}    port=${SWT_PORT}
    Verify Connection Established
    [Teardown]    Disconnect From Test App

Connect With Custom Timeout
    [Documentation]    Verify connection with a custom timeout value.
    [Tags]    positive    timeout
    Connect To SWT Application    ${SWT_APP_NAME}    ${SWT_HOST}    ${SWT_PORT}    timeout=${CONNECTION_TIMEOUT}
    Verify Connection Established
    [Teardown]    Disconnect From Test App

Connect With Short Valid Timeout
    [Documentation]    Verify connection succeeds with a short but valid timeout
    ...                when the application is readily available.
    [Tags]    positive    timeout
    Connect To SWT Application    ${SWT_APP_NAME}    ${SWT_HOST}    ${SWT_PORT}    timeout=5
    Verify Connection Established
    [Teardown]    Disconnect From Test App

# ============================================================================
# Negative Test Cases - connect_to_swt_application
# ============================================================================

Connection Fails With Invalid Host
    [Documentation]    Verify proper error when connecting to an invalid host.
    [Tags]    negative    error-handling
    # Accept any error for invalid host connection
    Run Keyword And Expect Error    *
    ...    Connect To SWT Application    ${SWT_APP_NAME}    ${INVALID_HOST}    ${SWT_PORT}    timeout=${SHORT_TIMEOUT}

Connection Fails With Invalid Port
    [Documentation]    Verify proper error when connecting to an invalid port.
    [Tags]    negative    error-handling
    # Accept any error for invalid port connection
    Run Keyword And Expect Error    *
    ...    Connect To SWT Application    ${SWT_APP_NAME}    ${SWT_HOST}    ${INVALID_PORT}    timeout=${SHORT_TIMEOUT}

Connection Fails With Empty Application Name
    [Documentation]    Verify proper error when application name is empty.
    [Tags]    negative    validation
    # Accept any error for empty application name
    Run Keyword And Expect Error    *
    ...    Connect To SWT Application    ${EMPTY}    ${SWT_HOST}    ${SWT_PORT}

Connection Timeout With Unreachable Host
    [Documentation]    Verify timeout behavior when the host is unreachable.
    [Tags]    negative    timeout
    # Accept any error for unreachable host
    Run Keyword And Expect Error    *
    ...    Connect To SWT Application    ${SWT_APP_NAME}    ${INVALID_HOST}    ${SWT_PORT}    timeout=${SHORT_TIMEOUT}

# ============================================================================
# Positive Test Cases - disconnect
# ============================================================================

Disconnect From Connected Application
    [Documentation]    Verify clean disconnection from a connected SWT application.
    [Tags]    smoke    positive
    Connect To Test App
    Disconnect
    Verify No Active Connection

Disconnect When Already Disconnected
    [Documentation]    Verify that disconnect is idempotent and doesn't fail
    ...                when already disconnected.
    [Tags]    positive    idempotent
    Connect To Test App
    Disconnect
    # Calling disconnect again should not raise an error
    Disconnect
    Verify No Active Connection

Multiple Sequential Connect Disconnect Cycles
    [Documentation]    Verify ability to connect and disconnect multiple times.
    [Tags]    positive    reliability
    FOR    ${i}    IN RANGE    3
        Connect To Test App
        Verify Connection Established
        Disconnect From Test App
        Verify No Active Connection
    END

# ============================================================================
# Positive Test Cases - is_connected
# ============================================================================

Is Connected Returns True When Connected
    [Documentation]    Verify is_connected returns True when connection is active.
    [Tags]    smoke    positive    status
    Connect To Test App
    ${status}=    Is Connected
    Should Be True    ${status}    is_connected should return True when connected
    [Teardown]    Disconnect From Test App

Is Connected Returns False When Not Connected
    [Documentation]    Verify is_connected returns False when no connection exists.
    [Tags]    smoke    positive    status
    Disconnect From Test App    # Ensure disconnected
    ${status}=    Is Connected
    Should Not Be True    ${status}    is_connected should return False when not connected

Is Connected Returns False After Disconnect
    [Documentation]    Verify is_connected returns False after disconnecting.
    [Tags]    positive    status
    Connect To Test App
    ${before}=    Is Connected
    Should Be True    ${before}    Should be connected before disconnect
    Disconnect
    ${after}=    Is Connected
    Should Not Be True    ${after}    Should not be connected after disconnect

Connection State Is Accurate Throughout Lifecycle
    [Documentation]    Verify connection state is accurate at each stage
    ...                of the connection lifecycle.
    [Tags]    positive    status    lifecycle
    # Initially not connected
    ${initial}=    Is Connected
    Should Not Be True    ${initial}    Should not be connected initially

    # After connecting
    Connect To Test App
    ${connected}=    Is Connected
    Should Be True    ${connected}    Should be connected after connect

    # After disconnecting
    Disconnect
    ${final}=    Is Connected
    Should Not Be True    ${final}    Should not be connected after disconnect

# ============================================================================
# Edge Cases and Robustness Tests
# ============================================================================

Reconnect After Disconnect
    [Documentation]    Verify successful reconnection after a clean disconnect.
    [Tags]    positive    reliability    reconnect
    Connect To Test App
    Verify Connection Established
    Disconnect
    Verify No Active Connection
    Connect To Test App
    Verify Connection Established
    [Teardown]    Disconnect From Test App

Rapid Connect Disconnect Cycles
    [Documentation]    Verify stability under rapid connect/disconnect cycles.
    [Tags]    positive    stress    reliability
    FOR    ${i}    IN RANGE    5
        Connect To Test App
        Disconnect
    END
    Verify No Active Connection

Connect With Various Timeout Values
    [Documentation]    Verify connection works with various valid timeout values.
    [Tags]    positive    timeout    parametric
    @{timeouts}=    Create List    5    10    30    60
    FOR    ${timeout}    IN    @{timeouts}
        Connect To SWT Application    ${SWT_APP_NAME}    ${SWT_HOST}    ${SWT_PORT}    timeout=${timeout}
        Verify Connection Established
        Disconnect
    END


*** Keywords ***
Start App Without Connect
    [Documentation]    Start the SWT test application without connecting.
    ...                This is used for connection tests where we test the connection itself.
    ${vmarg}=    Get SWT JVM Arg
    ${cmd}=    Set Variable    java ${vmarg} -javaagent:${SWT_AGENT_JAR}=port=${SWT_PORT} -jar ${TEST_APP_JAR}
    Log    Starting SWT Application: ${cmd}
    ${process}=    Start Process    ${cmd}    shell=True    alias=swt_test    stdout=${SWT_STDOUT_LOG}    stderr=${SWT_STDERR_LOG}
    Sleep    5s    Wait for application and agent to start (increased for SWT initialization)
    RETURN    ${process}
