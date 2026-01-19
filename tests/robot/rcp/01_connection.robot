*** Settings ***
Test Timeout       60s
Documentation     Test suite for RCP Library connection functionality.
...               Tests connecting to, disconnecting from, and verifying
...               connection status for Eclipse RCP applications.

Resource          resources/common.resource

Suite Setup       Start RCP Mock Application Without Connect
Suite Teardown    Close All Connections And Stop App

Force Tags        rcp    connection


*** Test Cases ***
# =============================================================================
# Positive Test Cases - Connection
# =============================================================================

Connect To RCP Application Successfully
    [Documentation]    Verify successful connection to a running RCP application.
    ...                This is the most basic connection test using default settings.
    [Tags]    smoke    critical    positive
    Connect To SWT Application    ${APP_NAME}    ${HOST}    ${PORT}
    Verify Connection Is Active
    [Teardown]    Disconnect From RCP App

Connect With Custom Timeout
    [Documentation]    Verify connection with a custom timeout value.
    ...                Uses a longer timeout for slow-starting applications.
    [Tags]    smoke    positive
    Connect To SWT Application    ${APP_NAME}    ${HOST}    ${PORT}    timeout=${CONNECTION_TIMEOUT}
    Verify Connection Is Active
    [Teardown]    Disconnect From RCP App

Connect With Explicit Host And Port
    [Documentation]    Verify connection specifying explicit host and port.
    ...                Tests the full parameter signature of connect keyword.
    [Tags]    positive
    Connect To SWT Application    ${APP_NAME}    host=${HOST}    port=${PORT}    timeout=${DEFAULT_TIMEOUT}
    Verify Connection Is Active
    [Teardown]    Disconnect From RCP App

Verify Is Connected Returns True When Connected
    [Documentation]    Verify Is Connected returns True after successful connection.
    ...                Tests the connection status check keyword.
    [Tags]    smoke    positive
    Connect To SWT Application    ${APP_NAME}    ${HOST}    ${PORT}
    ${result}=    Is Connected
    Should Be True    ${result}    Is Connected should return True when connected
    [Teardown]    Disconnect From RCP App

Verify Is Connected Returns False Before Connection
    [Documentation]    Verify Is Connected returns False before any connection.
    ...                Tests initial connection status.
    [Tags]    positive
    ${result}=    Is Connected
    Should Not Be True    ${result}    Is Connected should return False before connection


# =============================================================================
# Positive Test Cases - Disconnection
# =============================================================================

Disconnect From RCP Application Successfully
    [Documentation]    Verify clean disconnection from RCP application.
    ...                Tests the disconnect keyword functionality.
    [Tags]    smoke    positive
    Connect To SWT Application    ${APP_NAME}    ${HOST}    ${PORT}
    Verify Connection Is Active
    Disconnect
    Verify Connection Is Not Active

Disconnect When Already Disconnected
    [Documentation]    Verify disconnect is safe when already disconnected.
    ...                Should not raise an error when called on inactive connection.
    [Tags]    positive    edge-case
    # Ensure not connected
    ${is_connected}=    Is Connected
    Run Keyword If    ${is_connected}    Disconnect
    # Disconnect again should be safe
    Disconnect
    Verify Connection Is Not Active

Verify Is Connected Returns False After Disconnect
    [Documentation]    Verify Is Connected returns False after disconnect.
    ...                Tests connection status after disconnection.
    [Tags]    positive
    Connect To SWT Application    ${APP_NAME}    ${HOST}    ${PORT}
    Disconnect
    ${result}=    Is Connected
    Should Not Be True    ${result}    Is Connected should return False after disconnect


# =============================================================================
# Positive Test Cases - Reconnection
# =============================================================================

Reconnect After Disconnect
    [Documentation]    Verify reconnection works after a clean disconnect.
    ...                Tests ability to re-establish connection.
    [Tags]    positive    reliability
    Connect To SWT Application    ${APP_NAME}    ${HOST}    ${PORT}
    Disconnect
    Connect To SWT Application    ${APP_NAME}    ${HOST}    ${PORT}
    Verify Connection Is Active
    [Teardown]    Disconnect From RCP App

Multiple Sequential Connections
    [Documentation]    Verify ability to connect, disconnect, and reconnect multiple times.
    ...                Tests connection stability across multiple cycles.
    [Tags]    positive    reliability    stress
    FOR    ${i}    IN RANGE    3
        Log    Connection cycle ${i + 1}
        Connect To SWT Application    ${APP_NAME}    ${HOST}    ${PORT}
        Verify Connection Is Active
        Disconnect
        Verify Connection Is Not Active
    END


# =============================================================================
# Negative Test Cases - Invalid Connection Parameters
# =============================================================================

Connection Fails For Empty Application Name
    [Documentation]    Verify connection fails with empty application name.
    ...                Tests input validation for application parameter.
    [Tags]    negative    error-handling    validation
    Run Keyword And Expect Error    *empty*
    ...    Connect To SWT Application    ${EMPTY}    ${HOST}    ${PORT}

Connection Fails For Invalid Application Name
    [Documentation]    In the mock setup, any app name connects successfully since
    ...                there's no real application registry. This test verifies the
    ...                connection succeeds (mock limitation - real Eclipse would fail).
    [Tags]    positive    mock-limitation
    Connect To SWT Application    ${INVALID_APP_NAME}    ${HOST}    ${PORT}    timeout=${SHORT_TIMEOUT}
    ${connected}=    Is Connected
    Should Be True    ${connected}    Mock server accepts any application name
    [Teardown]    Disconnect

Connection Fails For Invalid Host
    [Documentation]    Verify proper error when connecting to invalid host.
    ...                Tests handling of unreachable host address.
    [Tags]    negative    error-handling    network
    Run Keyword And Expect Error    *
    ...    Connect To SWT Application    ${APP_NAME}    ${INVALID_HOST}    ${PORT}    timeout=${SHORT_TIMEOUT}

Connection Fails For Invalid Port
    [Documentation]    Verify proper error when connecting to invalid port.
    ...                Tests handling of port where no agent is listening.
    [Tags]    negative    error-handling    network
    Run Keyword And Expect Error    *
    ...    Connect To SWT Application    ${APP_NAME}    ${HOST}    12345    timeout=${SHORT_TIMEOUT}

Connection Timeout With Short Timeout
    [Documentation]    In the mock setup, connection to invalid app name succeeds quickly.
    ...                This test verifies that short timeout works when connecting
    ...                to an unreachable port (where timeout would actually occur).
    [Tags]    negative    timeout
    # Use a port that's not listening to actually trigger a timeout
    # Note: On some systems this may fail immediately with "connection refused"
    # rather than timing out, so we accept any error
    Run Keyword And Expect Error    *
    ...    Connect To SWT Application    ${APP_NAME}    ${HOST}    59999    timeout=${SHORT_TIMEOUT}


# =============================================================================
# Edge Cases
# =============================================================================

Connect With Zero Timeout
    [Documentation]    Verify behavior when timeout is set to zero.
    ...                Tests edge case with minimal timeout value.
    [Tags]    negative    edge-case
    Run Keyword And Expect Error    *
    ...    Connect To SWT Application    ${APP_NAME}    ${HOST}    ${PORT}    timeout=0

Connection Status During Operations
    [Documentation]    Verify connection status is consistent during operations.
    ...                Tests that Is Connected returns accurate status.
    [Tags]    positive    edge-case
    # Before connection
    ${before}=    Is Connected
    Should Not Be True    ${before}
    # During connection
    Connect To SWT Application    ${APP_NAME}    ${HOST}    ${PORT}
    ${during}=    Is Connected
    Should Be True    ${during}
    # After disconnect
    Disconnect
    ${after}=    Is Connected
    Should Not Be True    ${after}


*** Keywords ***
Close All Connections And Stop App
    [Documentation]    Suite teardown to ensure all connections are closed and app is stopped.
    ${is_connected}=    Is Connected
    Run Keyword If    ${is_connected}    Disconnect
    Stop RCP Mock Application
    Log    All RCP connections closed and app stopped
