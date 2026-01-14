*** Settings ***
Documentation     Common resources for SwingLibrary comprehensive test suite
...               Provides library imports, common variables, and reusable keywords
...               for testing all SwingLibrary keywords against the demo application.

Library           swing_library.SwingLibrary    timeout=15    poll_interval=0.5
Library           OperatingSystem
Library           Process
Library           Collections
Library           String

*** Variables ***
# Application paths
${DEMO_APP_JAR}           ${CURDIR}/../../../../demo/target/swing-demo-app-1.0.0.jar
${AGENT_JAR}              ${CURDIR}/../../../../agent/target/robotframework-swing-agent-1.0.0-all.jar

# Demo application configuration
${DEMO_MAIN_CLASS}        com.example.demo.SwingDemoApp
${DEMO_WINDOW_TITLE}      Swing Demo Application

# Connection configuration
${AGENT_HOST}             localhost
${AGENT_PORT}             5678
${CONNECTION_TIMEOUT}     30

# Timeouts
${DEFAULT_TIMEOUT}        10
${SHORT_TIMEOUT}          5
${LONG_TIMEOUT}           30

# Common element locators
${MAIN_TABS}              JTabbedPane[name='mainTabs']
${LOGIN_PANEL}            JPanel[name='loginPanel']
${TABLE_PANEL}            JPanel[name='tablePanel']
${TREE_PANEL}             JPanel[name='treePanel']
${FORM_PANEL}             JPanel[name='formPanel']
${LIST_PANEL}             JPanel[name='listPanel']

# Login form elements
${USERNAME_FIELD}         JTextField[name='username']
${PASSWORD_FIELD}         JPasswordField[name='password']
${LOGIN_BUTTON}           JButton[name='loginBtn']
${CLEAR_BUTTON}           JButton[name='clearBtn']
${REMEMBER_CHECKBOX}      JCheckBox[name='rememberMe']
${STATUS_LABEL}           JLabel[name='statusLabel']

# Table elements
${DATA_TABLE}             JTable[name='dataTable']

# Tree elements
${FILE_TREE}              JTree[name='fileTree']

# List elements
${ITEM_LIST}              JList[name='itemList']

# Form elements
${COUNTRY_COMBO}          JComboBox[name='countryCombo']
${OPTION_A_RADIO}         JRadioButton[name='optionA']
${OPTION_B_RADIO}         JRadioButton[name='optionB']
${VOLUME_SLIDER}          JSlider[name='volumeSlider']
${QUANTITY_SPINNER}       JSpinner[name='quantitySpinner']
${PROGRESS_BAR}           JProgressBar[name='progressBar']
${NOTES_AREA}             JTextArea[name='notesArea']

# Test data
${VALID_USERNAME}         admin
${VALID_PASSWORD}         password123
${TEST_USERNAME}          testuser
${TEST_PASSWORD}          testpass

*** Keywords ***
# =============================================================================
# Application Lifecycle Keywords
# =============================================================================

Start Test Application
    [Documentation]    Start the demo Swing application with the agent and connect
    ...                Returns the process handle for later cleanup.
    ${cmd}=    Set Variable    java -javaagent:${AGENT_JAR}=port=${AGENT_PORT} -jar ${DEMO_APP_JAR}
    ${process}=    Start Process    ${cmd}    shell=True    alias=swing_demo
    Sleep    3s    Wait for application and agent to start
    Connect To Application    main_class=${DEMO_MAIN_CLASS}    host=${AGENT_HOST}    port=${AGENT_PORT}    timeout=${CONNECTION_TIMEOUT}
    Wait Until Element Exists    ${MAIN_TABS}    timeout=${DEFAULT_TIMEOUT}
    RETURN    ${process}

Start Test Application Without Connect
    [Documentation]    Start the demo Swing application without connecting
    ...                Useful for connection tests that need to test various connect scenarios.
    ${cmd}=    Set Variable    java -javaagent:${AGENT_JAR}=port=${AGENT_PORT} -jar ${DEMO_APP_JAR}
    ${process}=    Start Process    ${cmd}    shell=True    alias=swing_demo
    Sleep    3s    Wait for application and agent to start
    RETURN    ${process}

Stop Test Application
    [Documentation]    Disconnect from the application and stop the process
    Run Keyword And Ignore Error    Disconnect
    Run Keyword And Ignore Error    Terminate Process    swing_demo    kill=True

Restart Test Application
    [Documentation]    Stop and restart the test application
    Stop Test Application
    Sleep    1s
    Start Test Application

# =============================================================================
# Tab Navigation Keywords
# =============================================================================

Select Login Tab
    [Documentation]    Navigate to the Login tab
    Select Tab    ${MAIN_TABS}    Login
    Wait Until Element Is Visible    ${LOGIN_PANEL}    timeout=${SHORT_TIMEOUT}

Select Table Tab
    [Documentation]    Navigate to the Table tab
    Select Tab    ${MAIN_TABS}    Table
    Wait Until Element Is Visible    ${DATA_TABLE}    timeout=${SHORT_TIMEOUT}

Select Tree Tab
    [Documentation]    Navigate to the Tree tab
    Select Tab    ${MAIN_TABS}    Tree
    Wait Until Element Is Visible    ${FILE_TREE}    timeout=${SHORT_TIMEOUT}

Select Form Tab
    [Documentation]    Navigate to the Form tab
    Select Tab    ${MAIN_TABS}    Form
    Wait Until Element Is Visible    ${FORM_PANEL}    timeout=${SHORT_TIMEOUT}

Select List Tab
    [Documentation]    Navigate to the List tab
    Select Tab    ${MAIN_TABS}    List
    Wait Until Element Is Visible    ${ITEM_LIST}    timeout=${SHORT_TIMEOUT}

# =============================================================================
# Login Form Keywords
# =============================================================================

Clear Login Form
    [Documentation]    Clear all fields in the login form
    Select Login Tab
    Clear Text    ${USERNAME_FIELD}
    Clear Text    ${PASSWORD_FIELD}

Enter Login Credentials
    [Documentation]    Enter username and password into the login form
    [Arguments]    ${username}    ${password}
    Select Login Tab
    Clear Text    ${USERNAME_FIELD}
    Input Text    ${USERNAME_FIELD}    ${username}
    Clear Text    ${PASSWORD_FIELD}
    Input Text    ${PASSWORD_FIELD}    ${password}

Submit Login Form
    [Documentation]    Click the login button
    Click Button    ${LOGIN_BUTTON}
    Sleep    0.5s    Wait for form processing

Perform Login
    [Documentation]    Complete login workflow with given credentials
    [Arguments]    ${username}    ${password}
    Enter Login Credentials    ${username}    ${password}
    Submit Login Form

Get Status Label Text
    [Documentation]    Get the current text of the status label
    ${text}=    Get Element Text    ${STATUS_LABEL}
    RETURN    ${text}

Verify Status Label Contains
    [Documentation]    Verify the status label contains expected text
    [Arguments]    ${expected}
    ${text}=    Get Status Label Text
    Should Contain    ${text}    ${expected}

# =============================================================================
# Table Helper Keywords
# =============================================================================

Get Table Cell
    [Documentation]    Get value from table cell with validation
    [Arguments]    ${locator}    ${row}    ${column}
    ${value}=    Get Table Cell Value    ${locator}    ${row}    ${column}
    RETURN    ${value}

Verify Table Cell Value
    [Documentation]    Verify a table cell contains the expected value
    [Arguments]    ${locator}    ${row}    ${column}    ${expected}
    ${actual}=    Get Table Cell Value    ${locator}    ${row}    ${column}
    Should Be Equal    ${actual}    ${expected}

Select Table Cell And Verify
    [Documentation]    Select a table cell and verify it was selected
    [Arguments]    ${locator}    ${row}    ${column}
    Select Table Cell    ${locator}    ${row}    ${column}
    Sleep    0.2s    Wait for selection

# =============================================================================
# Tree Helper Keywords
# =============================================================================

Navigate To Tree Node
    [Documentation]    Expand and select a tree node by path
    [Arguments]    ${locator}    ${path}
    Expand Tree Node    ${locator}    ${path}
    Select Tree Node    ${locator}    ${path}

Verify Tree Node Selected
    [Documentation]    Verify a specific tree node is selected
    [Arguments]    ${locator}    ${expected_path}
    ${selected}=    Get Selected Tree Node    ${locator}
    Should Be Equal    ${selected}    ${expected_path}

# =============================================================================
# Element State Verification Keywords
# =============================================================================

Element Should Be Visible And Enabled
    [Documentation]    Verify element is both visible and enabled
    [Arguments]    ${locator}
    Element Should Be Visible    ${locator}
    Element Should Be Enabled    ${locator}

Element Should Exist And Be Visible
    [Documentation]    Verify element exists and is visible
    [Arguments]    ${locator}
    Element Should Exist    ${locator}
    Element Should Be Visible    ${locator}

Wait Until Element Is Ready
    [Documentation]    Wait until element is visible and enabled
    [Arguments]    ${locator}    ${timeout}=${DEFAULT_TIMEOUT}
    Wait Until Element Is Visible    ${locator}    timeout=${timeout}
    Wait Until Element Is Enabled    ${locator}    timeout=${timeout}

# =============================================================================
# Screenshot Helper Keywords
# =============================================================================

Capture Failure Screenshot
    [Documentation]    Capture a screenshot for failure analysis
    [Arguments]    ${name}=failure
    ${timestamp}=    Get Time    epoch
    ${filename}=    Set Variable    ${name}_${timestamp}.png
    ${path}=    Capture Screenshot    ${filename}
    RETURN    ${path}

# =============================================================================
# Debug Helper Keywords
# =============================================================================

Log Current UI State
    [Documentation]    Log the current UI tree for debugging
    Log UI Tree
    ${tree}=    Get UI Tree    format=text
    Log    Current UI Tree:\n${tree}    level=DEBUG

Log Element Properties
    [Documentation]    Log all properties of an element
    [Arguments]    ${locator}
    ${props}=    Get Element Properties    ${locator}
    Log    Element ${locator} properties: ${props}    level=DEBUG
    RETURN    ${props}
