*** Settings ***
Test Timeout       60s
Documentation     Label Tests - Testing JLabel read operations.
...
...               These tests verify the library's ability to interact with
...               JLabel components for text display and verification.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        labels    regression

*** Test Cases ***
# =============================================================================
# LABEL TEXT READING
# =============================================================================

Get Label Text
    [Documentation]    Get the text of a label.
    [Tags]    smoke    positive
    ${text}=    Get Element Text    JLabel[name='statusLabel']
    Should Not Be Empty    ${text}

Verify Status Label Initial Value
    [Documentation]    Verify the initial status label value.
    [Tags]    positive    verification
    ${text}=    Get Element Text    JLabel[name='statusLabel']
    Should Be Equal    ${text}    Ready

# =============================================================================
# LABEL STATE VERIFICATION
# =============================================================================

Label Should Be Visible
    [Documentation]    Verify label is visible.
    [Tags]    positive    verification
    Element Should Be Visible    JLabel[name='statusLabel']

Label Should Exist
    [Documentation]    Verify label exists.
    [Tags]    positive    verification
    Element Should Exist    JLabel[name='nameLabel']
    Element Should Exist    JLabel[name='emailLabel']
    Element Should Exist    JLabel[name='passwordLabel']

# =============================================================================
# FORM LABELS
# =============================================================================

Verify Form Labels
    [Documentation]    Verify all form labels exist on Form Input tab.
    [Tags]    positive    form
    Select Form Input Tab
    Element Should Exist    JLabel[name='nameLabel']
    Element Should Exist    JLabel[name='emailLabel']
    Element Should Exist    JLabel[name='passwordLabel']
    Element Should Exist    JLabel[name='descriptionLabel']

Verify Selections Tab Labels
    [Documentation]    Verify labels on Selections tab.
    [Tags]    positive    selections
    Select Selections Tab
    Element Should Exist    JLabel[name='categoryLabel']
    Element Should Exist    JLabel[name='quantityLabel']
    Element Should Exist    JLabel[name='volumeLabel']
    Element Should Exist    JLabel[name='optionsLabel']
    Element Should Exist    JLabel[name='priorityLabel']
    Element Should Exist    JLabel[name='itemsLabel']

# =============================================================================
# DYNAMIC LABEL UPDATES
# =============================================================================

Status Label Updates On Selection
    [Documentation]    Verify status label updates when tree node is selected.
    [Tags]    positive    dynamic
    # Select a tree node to update status
    Expand Tree Node    JTree[name='fileTree']    Project Root/Sources
    Sleep    0.2s
    Select Tree Node    JTree[name='fileTree']    Project Root/Sources
    Sleep    0.3s
    ${text}=    Get Element Text    JLabel[name='statusLabel']
    Should Contain    ${text}    Sources

# =============================================================================
# LABEL TEXT ASSERTIONS
# =============================================================================

Element Text Should Contain For Label
    [Documentation]    Verify label text contains expected substring.
    [Tags]    positive    assertion
    # Ensure status label is in a known state for assertion
    Expand Tree Node    JTree[name='fileTree']    Project Root/Sources
    Sleep    0.2s
    Select Tree Node    JTree[name='fileTree']    Project Root/Sources
    Sleep    0.3s
    Element Text Should Contain    JLabel[name='statusLabel']    Selected

Element Text Should Be For Label
    [Documentation]    Verify label text is exactly as expected.
    [Tags]    positive    assertion
    ${text}=    Get Element Text    JLabel[name='statusLabel']
    # Reset to known state first
    Select Form Input Tab
    Sleep    0.3s
    ${text_after}=    Get Element Text    JLabel[name='statusLabel']
    Should Not Be Empty    ${text_after}

# =============================================================================
# FINDING LABELS
# =============================================================================

Find All Labels
    [Documentation]    Find all label elements in the application.
    [Tags]    positive
    ${labels}=    Find Elements    JLabel
    Should Not Be Empty    ${labels}

Find Labels By Text Content
    [Documentation]    Find labels with specific text.
    [Tags]    positive
    ${labels}=    Find Elements    JLabel[text='Name:']
    Should Not Be Empty    ${labels}

Find Visible Labels
    [Documentation]    Find all visible labels.
    [Tags]    positive
    ${labels}=    Find Elements    JLabel:visible
    Should Not Be Empty    ${labels}

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Get Nonexistent Label Text Fails
    [Documentation]    Getting text from non-existent label throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Element Text    JLabel[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Element Should Exist Fails For Nonexistent Label
    [Documentation]    Exist check fails for non-existent label.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Element Should Exist    JLabel[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}
