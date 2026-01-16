*** Settings ***
Documentation     Tab Tests - Testing JTabbedPane tab selection operations.
...
...               These tests verify the library's ability to interact with
...               JTabbedPane components for tab-based navigation.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        tabs    regression

*** Test Cases ***
# =============================================================================
# TAB SELECTION
# =============================================================================

Select Tab By Title
    [Documentation]    Select a tab by its title.
    [Tags]    smoke    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Element Should Exist    JTabbedPane[name='mainTabbedPane']

Select All Tabs Sequentially
    [Documentation]    Select each tab in sequence and verify.
    [Tags]    positive    workflow
    # Select Form Input
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Element Should Exist    JTabbedPane[name='mainTabbedPane']
    # Select Selections
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Element Should Exist    JTabbedPane[name='mainTabbedPane']
    # Select Data View
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Element Should Exist    JTabbedPane[name='mainTabbedPane']
    # Select Settings
    Select Tab    JTabbedPane[name='mainTabbedPane']    Settings
    Element Should Exist    JTabbedPane[name='mainTabbedPane']

Verify Form Input Tab Contents
    [Documentation]    Select Form Input tab and verify its contents exist.
    [Tags]    positive    content-verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Sleep    0.3s
    Element Should Exist    JTextField[name='nameTextField']
    Element Should Exist    JTextField[name='emailTextField']
    Element Should Exist    JPasswordField[name='passwordField']

Verify Selections Tab Contents
    [Documentation]    Select Selections tab and verify its contents exist.
    [Tags]    positive    content-verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Exist    JComboBox[name='categoryComboBox']
    Element Should Exist    JList[name='itemList']
    Element Should Exist    JSpinner[name='quantitySpinner']

Verify Data View Tab Contents
    [Documentation]    Select Data View tab and verify its contents exist.
    [Tags]    positive    content-verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    Element Should Exist    JTable[name='dataTable']
    Element Should Exist    JButton[name='addRowButton']

Verify Settings Tab Contents
    [Documentation]    Select Settings tab and verify its contents exist.
    [Tags]    positive    content-verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Settings
    Sleep    0.3s
    Element Should Exist    JComboBox[name='themeComboBox']
    Element Should Exist    JComboBox[name='languageComboBox']
    Element Should Exist    JSpinner[name='fontSizeSpinner']

Toggle Between Tabs
    [Documentation]    Toggle between tabs multiple times.
    [Tags]    positive    workflow
    # Start at Form Input
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Element Should Exist    JTextField[name='nameTextField']
    # Switch to Settings
    Select Tab    JTabbedPane[name='mainTabbedPane']    Settings
    Element Should Exist    JComboBox[name='themeComboBox']
    # Switch back to Form Input
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Element Should Exist    JTextField[name='nameTextField']

# =============================================================================
# TAB STATE VERIFICATION
# =============================================================================

TabbedPane Should Be Visible
    [Documentation]    Verify tabbed pane is visible.
    [Tags]    positive    verification
    Element Should Be Visible    JTabbedPane[name='mainTabbedPane']

TabbedPane Should Be Enabled
    [Documentation]    Verify tabbed pane is enabled.
    [Tags]    positive    verification
    Element Should Be Enabled    JTabbedPane[name='mainTabbedPane']

TabbedPane Should Exist
    [Documentation]    Verify tabbed pane exists.
    [Tags]    positive    verification
    Element Should Exist    JTabbedPane[name='mainTabbedPane']

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Select Nonexistent Tab Fails
    [Documentation]    Select non-existent tab throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select Tab    JTabbedPane[name='mainTabbedPane']    NonExistentTab
    Should Be Equal    ${status}    ${FALSE}

Select Tab In Nonexistent TabbedPane Fails
    [Documentation]    Select tab in non-existent tabbed pane throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select Tab    JTabbedPane[name='nonexistent']    Form Input
    Should Be Equal    ${status}    ${FALSE}

# =============================================================================
# EDGE CASES
# =============================================================================

Rapid Tab Switching
    [Documentation]    Test rapid tab switching.
    [Tags]    edge-case    stress
    FOR    ${i}    IN RANGE    5
        Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
        Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
        Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
        Select Tab    JTabbedPane[name='mainTabbedPane']    Settings
    END
    Element Should Exist    JTabbedPane[name='mainTabbedPane']

Select Same Tab Multiple Times
    [Documentation]    Selecting same tab multiple times is safe.
    [Tags]    edge-case
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Element Should Exist    JTabbedPane[name='mainTabbedPane']
