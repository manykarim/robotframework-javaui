*** Settings ***
Documentation    TabbedPane Controls - Testing JTabbedPane tab selection
...              Operations: Select Tab, Verify Tab Exists
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Select Tab By Title
    [Documentation]    Select a tab by its title
    [Tags]    tabs    select    title    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Element Should Exist    JTabbedPane[name='mainTabbedPane']

Select All Tabs Sequentially
    [Documentation]    Select each tab in sequence and verify
    [Tags]    tabs    select    sequence    positive
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
    [Documentation]    Select Form Input tab and verify its contents exist
    [Tags]    tabs    content    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Sleep    0.3s
    Element Should Exist    JTextField[name='nameTextField']
    Element Should Exist    JTextField[name='emailTextField']
    Element Should Exist    JPasswordField[name='passwordField']

Verify Selections Tab Contents
    [Documentation]    Select Selections tab and verify its contents exist
    [Tags]    tabs    content    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Exist    JComboBox[name='categoryComboBox']
    Element Should Exist    JList[name='itemList']
    Element Should Exist    JSpinner[name='quantitySpinner']

Verify Data View Tab Contents
    [Documentation]    Select Data View tab and verify its contents exist
    [Tags]    tabs    content    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    Element Should Exist    JTable[name='dataTable']
    Element Should Exist    JButton[name='addRowButton']

Verify Settings Tab Contents
    [Documentation]    Select Settings tab and verify its contents exist
    [Tags]    tabs    content    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Settings
    Sleep    0.3s
    Element Should Exist    JComboBox[name='themeComboBox']
    Element Should Exist    JComboBox[name='languageComboBox']
    Element Should Exist    JSpinner[name='fontSizeSpinner']

Toggle Between Tabs
    [Documentation]    Toggle between tabs multiple times
    [Tags]    tabs    toggle    positive
    # Start at Form Input
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Element Should Exist    JTextField[name='nameTextField']

    # Switch to Settings
    Select Tab    JTabbedPane[name='mainTabbedPane']    Settings
    Element Should Exist    JComboBox[name='themeComboBox']

    # Switch back to Form Input
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Element Should Exist    JTextField[name='nameTextField']

TabbedPane Should Be Visible
    [Documentation]    Verify tabbed pane is visible
    [Tags]    tabs    visible    verification
    Element Should Be Visible    JTabbedPane[name='mainTabbedPane']

TabbedPane Should Be Enabled
    [Documentation]    Verify tabbed pane is enabled
    [Tags]    tabs    enabled    verification
    Element Should Be Enabled    JTabbedPane[name='mainTabbedPane']
