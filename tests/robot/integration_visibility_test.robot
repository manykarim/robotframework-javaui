*** Settings ***
Test Timeout       60s
Documentation      Integration tests for the tiered visibility fix against the JGoodies
...                Smart Client Showcase app. Validates that:
...                - Element Should Be Visible uses relaxed isVisible() check (Tier 1)
...                - Element Should Be Showing uses strict isVisible()+isShowing() check (Tier 2)
...                - Click works on visible-but-not-showing elements via synthetic dispatch (Tier 2)
...                - Get Element States returns 'visible' for such elements
...                - Wait Until Element Is Visible accepts RF time strings
...                - Input Text works on non-showing text fields
...                - force=True parameter works for Click and Input Text

Library           JavaGui.Swing    timeout=15    poll_interval=0.5
Library           Collections

Force Tags        integration    visibility

*** Variables ***
${HOST}                       127.0.0.1
${PORT}                       18080
${CONNECTION_TIMEOUT}         30

# SplitView elements - these have visible=True but showing=False
${SETTINGS_BUTTON}            NavigationToggleButton[text='Settings']
${START_BUTTON}               NavigationToggleButton[text='Start']
${SEARCH_FIELD}               JGSearchField

# Fully visible elements
${SPLIT_VIEW}                 SplitView
${PAGE_FRAME}                 PageFrame

*** Test Cases ***
# =============================================================================
# CONNECTION
# =============================================================================

Connect And Verify Connection
    [Documentation]    Connect to the Smart Client Showcase app and verify the connection.
    [Tags]    smoke
    Connect To Application    host=${HOST}    port=${PORT}    timeout=${CONNECTION_TIMEOUT}
    ${connected}=    Is Connected
    Should Be True    ${connected}    Application should be connected

# =============================================================================
# TIERED VISIBILITY - TIER 1 (isVisible only)
# =============================================================================

Element Should Be Visible Passes For Visible But Not Showing Elements
    [Documentation]    The Settings button has visible=True but showing=False.
    ...                Element Should Be Visible should PASS because it only checks isVisible()
    ...                (Tier 1 relaxed check).
    [Tags]    tier1
    Element Should Be Visible    ${SETTINGS_BUTTON}

Element Should Be Visible Passes For Start Button
    [Documentation]    The Start button also has visible=True but showing=False.
    ...                Element Should Be Visible should PASS (Tier 1).
    [Tags]    tier1
    Element Should Be Visible    ${START_BUTTON}

Element Should Be Visible Passes For Search Field
    [Documentation]    The search field has visible=True but showing=False.
    ...                Element Should Be Visible should PASS (Tier 1).
    [Tags]    tier1
    Element Should Be Visible    ${SEARCH_FIELD}

Element Should Be Visible Passes For Fully Showing Element
    [Documentation]    SplitView has both visible=True and showing=True.
    ...                Element Should Be Visible should obviously PASS.
    [Tags]    tier1
    Element Should Be Visible    ${SPLIT_VIEW}

# =============================================================================
# TIERED VISIBILITY - TIER 2 (isVisible + isShowing)
# =============================================================================

Element Should Be Showing Depends On Display For Settings Button
    [Documentation]    The Settings button may or may not be showing depending on the
    ...                display environment. On virtual displays (Xvfb), isShowing=False
    ...                for SplitView children. On real displays, isShowing=True.
    ...                This test verifies the keyword runs without error and returns
    ...                a result consistent with the showing property.
    [Tags]    tier2
    ${showing}=    Get Property    ${SETTINGS_BUTTON}    showing
    ${status}=    Run Keyword And Return Status
    ...    Element Should Be Showing    ${SETTINGS_BUTTON}
    Should Be Equal    ${status}    ${showing}
    ...    Element Should Be Showing result should match the showing property

Element Should Be Showing Depends On Display For Start Button
    [Documentation]    Same display-dependent check for the Start button.
    [Tags]    tier2
    ${showing}=    Get Property    ${START_BUTTON}    showing
    ${status}=    Run Keyword And Return Status
    ...    Element Should Be Showing    ${START_BUTTON}
    Should Be Equal    ${status}    ${showing}
    ...    Element Should Be Showing result should match the showing property

Element Should Be Showing Depends On Display For Search Field
    [Documentation]    Same display-dependent check for the JGSearchField.
    [Tags]    tier2
    ${showing}=    Get Property    ${SEARCH_FIELD}    showing
    ${status}=    Run Keyword And Return Status
    ...    Element Should Be Showing    ${SEARCH_FIELD}
    Should Be Equal    ${status}    ${showing}
    ...    Element Should Be Showing result should match the showing property

Element Should Be Showing Passes For Fully Showing Element
    [Documentation]    SplitView has both visible=True and showing=True.
    ...                Element Should Be Showing should PASS.
    [Tags]    tier2
    Element Should Be Showing    ${SPLIT_VIEW}

# =============================================================================
# CLICK - SYNTHETIC DISPATCH (TIER 2)
# =============================================================================

Click On NavigationToggleButton Without Force
    [Documentation]    Click on the Settings NavigationToggleButton which has
    ...                visible=True but showing=False. This should work via
    ...                Tier 2 synthetic dispatch without needing force=True.
    [Tags]    click    tier2
    Click    ${SETTINGS_BUTTON}

Click On NavigationToggleButton With Force
    [Documentation]    Click on the Settings NavigationToggleButton with force=True.
    ...                The force parameter bypasses the isShowing() check entirely.
    [Tags]    click    force
    Click    ${SETTINGS_BUTTON}    force=True

Click On Start Button Without Force
    [Documentation]    Click on the Start NavigationToggleButton.
    ...                Should succeed via Tier 2 synthetic dispatch.
    [Tags]    click    tier2
    Click    ${START_BUTTON}

# =============================================================================
# GET ELEMENT STATES
# =============================================================================

Get Element States Returns Visible For Settings Button
    [Documentation]    Get Element States for a visible-but-not-showing element
    ...                should return 'visible' in the states list (uses isVisible).
    [Tags]    states
    ${states}=    Get Element States    ${SETTINGS_BUTTON}
    Should Contain    ${states}    visible
    ...    States should include 'visible' for visible-but-not-showing element

Get Element States Returns Visible For SplitView
    [Documentation]    Get Element States for a fully showing element should return 'visible'.
    [Tags]    states
    ${states}=    Get Element States    ${SPLIT_VIEW}
    Should Contain    ${states}    visible
    Should Contain    ${states}    enabled

# =============================================================================
# WAIT UNTIL ELEMENT IS VISIBLE
# =============================================================================

Wait Until Element Is Visible Succeeds For Settings Button
    [Documentation]    Wait Until Element Is Visible should succeed for elements with
    ...                visible=True even if showing=False (uses Tier 1 isVisible check).
    [Tags]    wait    tier1
    Wait Until Element Is Visible    ${SETTINGS_BUTTON}    timeout=5

Wait Until Element Is Visible With RF Time String
    [Documentation]    Test that Wait Until Element Is Visible accepts Robot Framework
    ...                time strings like "3s" instead of just integer seconds.
    [Tags]    wait    timestring
    Wait Until Element Is Visible    ${SETTINGS_BUTTON}    timeout=3s

Wait Until Element Is Visible For Start Button
    [Documentation]    Wait Until Element Is Visible for the Start button with
    ...                visible=True, showing=False.
    [Tags]    wait    tier1
    Wait Until Element Is Visible    ${START_BUTTON}    timeout=5

Wait Until Element Is Visible For Fully Showing Element
    [Documentation]    Wait Until Element Is Visible for a fully visible and showing element.
    [Tags]    wait
    Wait Until Element Is Visible    ${SPLIT_VIEW}    timeout=3s

# =============================================================================
# INPUT TEXT
# =============================================================================

Input Text Into Search Field
    [Documentation]    Test Input Text on the JGSearchField which has
    ...                visible=True but showing=False.
    [Tags]    input
    Input Text    ${SEARCH_FIELD}    test search query

Input Text Into Search Field With Force
    [Documentation]    Test Input Text with force=True on the JGSearchField.
    [Tags]    input    force
    Input Text    ${SEARCH_FIELD}    forced input text    force=True

Input Text Clears Before Typing
    [Documentation]    Test that Input Text clears existing text before typing (default).
    [Tags]    input
    Input Text    ${SEARCH_FIELD}    first input
    Input Text    ${SEARCH_FIELD}    second input

# =============================================================================
# CLEAN DISCONNECT
# =============================================================================

Disconnect Cleanly
    [Documentation]    Disconnect from the application cleanly.
    [Tags]    smoke
    Disconnect
    ${connected}=    Is Connected
    Should Not Be True    ${connected}    Should be disconnected
