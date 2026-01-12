*** Settings ***
Documentation     Basic example of using SwingLibrary
Library           SwingLibrary

*** Variables ***
${APP_CLASS}      com.example.demo.SwingDemoApp
${TIMEOUT}        10s

*** Test Cases ***
Login To Application
    [Documentation]    Demonstrate login workflow
    Connect To Application    main_class=${APP_CLASS}
    Wait Until Element Visible    JButton#loginBtn    timeout=${TIMEOUT}

    # Enter credentials
    Input Text    JTextField#username    testuser
    Input Text    JPasswordField#password    password123

    # Click login
    Click    JButton#loginBtn

    # Verify status
    Wait Until Element Contains    JLabel#statusLabel    success

    # Disconnect
    Disconnect

Navigate Through Tabs
    [Documentation]    Demonstrate tab navigation
    Connect To Application    main_class=${APP_CLASS}

    # Navigate to each tab
    Select Tab    Login
    Element Should Be Visible    JButton#loginBtn

    Select Tab    Data Table
    Element Should Be Visible    JTable#dataTable

    Select Tab    Tree View
    Element Should Be Visible    JTree#fileTree

    Disconnect

Work With Data Table
    [Documentation]    Demonstrate table operations
    Connect To Application    main_class=${APP_CLASS}
    Select Tab    Data Table

    # Get row count
    ${rows}=    Get Table Row Count    JTable#dataTable
    Log    Table has ${rows} rows

    # Get cell value
    ${value}=    Get Table Cell Value    JTable#dataTable    0    1
    Log    Cell[0,1] = ${value}

    # Select a cell
    Select Table Cell    JTable#dataTable    2    0

    Disconnect

Inspect UI Tree
    [Documentation]    Demonstrate UI tree inspection
    Connect To Application    main_class=${APP_CLASS}

    # Get tree as JSON
    ${json_tree}=    Get Component Tree    format=json
    Log    ${json_tree}

    # Get tree as text
    ${text_tree}=    Get Component Tree    format=text
    Log    ${text_tree}

    Disconnect
