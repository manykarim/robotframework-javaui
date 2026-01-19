*** Settings ***
Test Timeout       60s
Documentation     Connection Tests - Testing connect_to_application, disconnect,
...               is_connected, and get_connection_info keywords.
...
...               These tests verify the library's ability to connect to and
...               disconnect from Java Swing applications using various
...               identification methods.

Resource          resources/common.resource

Suite Setup       Log    Starting Connection Tests
Suite Teardown    Run Keyword And Ignore Error    Stop Test Application

Force Tags        connection    smoke

*** Test Cases ***
# =============================================================================
# CONNECT TO APPLICATION TESTS
# =============================================================================

Connect To Application By Main Class
    [Documentation]    Connect to a Swing application using the fully qualified main class name.
    ...                This is the most reliable method for identifying an application.
    [Tags]    smoke    positive
    Start Test Application Without Connect
    Connect To Application    main_class=${DEMO_MAIN_CLASS}    host=${AGENT_HOST}    port=${AGENT_PORT}    timeout=${CONNECTION_TIMEOUT}
    ${connected}=    Is Connected
    Should Be True    ${connected}    Application should be connected
    [Teardown]    Stop Test Application

Connect To Application By Window Title
    [Documentation]    Connect to a Swing application using the window title.
    ...                Supports wildcard matching with asterisk (*).
    [Tags]    smoke    positive
    Start Test Application Without Connect
    Connect To Application    title=*Demo*    host=${AGENT_HOST}    port=${AGENT_PORT}    timeout=${CONNECTION_TIMEOUT}
    ${connected}=    Is Connected
    Should Be True    ${connected}    Application should be connected
    [Teardown]    Stop Test Application

Connect To Application With Default Parameters
    [Documentation]    Connect using default host and port parameters.
    ...                Tests that the library defaults work correctly.
    [Tags]    positive
    Start Test Application Without Connect
    Connect To Application    main_class=${DEMO_MAIN_CLASS}    timeout=${CONNECTION_TIMEOUT}
    ${connected}=    Is Connected
    Should Be True    ${connected}    Application should be connected
    [Teardown]    Stop Test Application

Connect To Application With Custom Timeout
    [Documentation]    Connect to application with a custom timeout value.
    ...                Verifies timeout parameter is respected.
    [Tags]    positive
    Start Test Application Without Connect
    Connect To Application    main_class=${DEMO_MAIN_CLASS}    host=${AGENT_HOST}    port=${AGENT_PORT}    timeout=60
    ${connected}=    Is Connected
    Should Be True    ${connected}    Application should be connected
    [Teardown]    Stop Test Application

Connect To Application With Application Name
    [Documentation]    Connect using application identifier string.
    [Tags]    positive
    Start Test Application Without Connect
    Connect To Application    application=${DEMO_MAIN_CLASS}    host=${AGENT_HOST}    port=${AGENT_PORT}    timeout=${CONNECTION_TIMEOUT}
    ${connected}=    Is Connected
    Should Be True    ${connected}    Application should be connected
    [Teardown]    Stop Test Application

# =============================================================================
# DISCONNECT TESTS
# =============================================================================

Disconnect After Connect
    [Documentation]    Test that disconnect properly closes the connection.
    ...                After disconnect, is_connected should return False.
    [Tags]    smoke    positive
    Start Test Application
    ${connected_before}=    Is Connected
    Should Be True    ${connected_before}    Should be connected before disconnect
    Disconnect
    ${connected_after}=    Is Connected
    Should Not Be True    ${connected_after}    Should not be connected after disconnect
    [Teardown]    Run Keyword And Ignore Error    Terminate Process    swing_demo    kill=True

Disconnect When Not Connected
    [Documentation]    Test that disconnect is safe to call when not connected.
    ...                Should not raise an error.
    [Tags]    positive    edge-case
    Run Keyword And Ignore Error    Disconnect
    ${connected}=    Is Connected
    Should Not Be True    ${connected}    Should not be connected

Multiple Disconnect Calls
    [Documentation]    Test that calling disconnect multiple times is safe.
    ...                Should not raise errors on subsequent calls.
    [Tags]    positive    edge-case
    Start Test Application
    Disconnect
    Run Keyword And Ignore Error    Disconnect
    Run Keyword And Ignore Error    Disconnect
    ${connected}=    Is Connected
    Should Not Be True    ${connected}    Should not be connected after disconnects
    [Teardown]    Run Keyword And Ignore Error    Terminate Process    swing_demo    kill=True

# =============================================================================
# IS CONNECTED TESTS
# =============================================================================

Is Connected Returns True When Connected
    [Documentation]    Verify is_connected returns True when connected to an application.
    [Tags]    smoke    positive
    Start Test Application
    ${result}=    Is Connected
    Should Be True    ${result}    is_connected should return True
    [Teardown]    Stop Test Application

Is Connected Returns False When Not Connected
    [Documentation]    Verify is_connected returns False when not connected.
    [Tags]    smoke    positive
    Run Keyword And Ignore Error    Disconnect
    ${result}=    Is Connected
    Should Not Be True    ${result}    is_connected should return False

Is Connected After Disconnect
    [Documentation]    Verify is_connected returns False after disconnecting.
    [Tags]    positive
    Start Test Application
    Disconnect
    ${result}=    Is Connected
    Should Not Be True    ${result}    is_connected should return False after disconnect
    [Teardown]    Run Keyword And Ignore Error    Terminate Process    swing_demo    kill=True

# =============================================================================
# GET CONNECTION INFO TESTS
# =============================================================================

Get Connection Info When Connected
    [Documentation]    Retrieve connection information when connected to an application.
    ...                Should return host, port, and other connection details.
    [Tags]    smoke    positive
    Start Test Application
    ${info}=    Get Connection Info
    Should Not Be Empty    ${info}    Connection info should not be empty
    Log    Connection Info: ${info}
    [Teardown]    Stop Test Application

Get Connection Info Contains Host
    [Documentation]    Verify connection info contains the host information.
    [Tags]    positive
    Start Test Application
    ${info}=    Get Connection Info
    Dictionary Should Contain Key    ${info}    host
    Should Be Equal    ${info}[host]    ${AGENT_HOST}
    [Teardown]    Stop Test Application

Get Connection Info Contains Port
    [Documentation]    Verify connection info contains the port information.
    [Tags]    positive
    Start Test Application
    ${info}=    Get Connection Info
    Dictionary Should Contain Key    ${info}    port
    Should Be Equal As Integers    ${info}[port]    ${AGENT_PORT}
    [Teardown]    Stop Test Application

# =============================================================================
# RECONNECTION TESTS
# =============================================================================

Reconnect After Disconnect
    [Documentation]    Test the ability to reconnect after disconnecting.
    ...                Verifies connection can be re-established.
    [Tags]    positive    regression
    Start Test Application Without Connect
    Connect To Application    main_class=${DEMO_MAIN_CLASS}    host=${AGENT_HOST}    port=${AGENT_PORT}    timeout=${CONNECTION_TIMEOUT}
    ${connected1}=    Is Connected
    Should Be True    ${connected1}    First connection should succeed
    Disconnect
    Sleep    1s    Wait for connection cleanup
    Connect To Application    main_class=${DEMO_MAIN_CLASS}    host=${AGENT_HOST}    port=${AGENT_PORT}    timeout=${CONNECTION_TIMEOUT}
    ${connected2}=    Is Connected
    Should Be True    ${connected2}    Reconnection should succeed
    [Teardown]    Stop Test Application

Multiple Connect Disconnect Cycles
    [Documentation]    Test multiple connect/disconnect cycles.
    ...                Verifies stability of connection handling.
    [Tags]    positive    regression    stress
    Start Test Application Without Connect
    FOR    ${i}    IN RANGE    3
        Log    Connection cycle ${i}
        Connect To Application    main_class=${DEMO_MAIN_CLASS}    host=${AGENT_HOST}    port=${AGENT_PORT}    timeout=${CONNECTION_TIMEOUT}
        ${connected}=    Is Connected
        Should Be True    ${connected}    Connection ${i} should succeed
        Disconnect
        ${disconnected}=    Is Connected
        Should Not Be True    ${disconnected}    Disconnection ${i} should succeed
        Sleep    0.5s
    END
    [Teardown]    Run Keyword And Ignore Error    Terminate Process    swing_demo    kill=True

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Connect To Nonexistent Application
    [Documentation]    Test connection timeout when application doesn't exist.
    ...                Should fail with timeout error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Connect To Application    main_class=com.nonexistent.FakeApp    timeout=3
    Should Be Equal    ${status}    ${FALSE}    Connection to nonexistent app should fail

Connect To Wrong Port
    [Documentation]    Test connection failure when using wrong port.
    ...                Should fail with connection error.
    [Tags]    negative    error-handling
    Start Test Application Without Connect
    ${status}=    Run Keyword And Return Status
    ...    Connect To Application    main_class=${DEMO_MAIN_CLASS}    port=9999    timeout=3
    Should Be Equal    ${status}    ${FALSE}    Connection to wrong port should fail
    [Teardown]    Run Keyword And Ignore Error    Terminate Process    swing_demo    kill=True

Connect With Invalid Host
    [Documentation]    Test connection failure when using invalid host.
    ...                Should fail with connection error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Connect To Application    main_class=${DEMO_MAIN_CLASS}    host=invalid.host.local    timeout=3
    Should Be Equal    ${status}    ${FALSE}    Connection to invalid host should fail

Connection Timeout Handling
    [Documentation]    Verify proper timeout handling when connection fails.
    ...                The operation should complete within the timeout period.
    [Tags]    negative    error-handling
    ${start}=    Get Time    epoch
    ${status}=    Run Keyword And Return Status
    ...    Connect To Application    main_class=NonExistentApp    timeout=5
    ${end}=    Get Time    epoch
    ${duration}=    Evaluate    ${end} - ${start}
    Should Be True    ${duration} < 10    Timeout should be respected
    Should Be Equal    ${status}    ${FALSE}    Connection should fail
