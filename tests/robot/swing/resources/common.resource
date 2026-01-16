*** Settings ***
Documentation     Common resources for SwingLibrary comprehensive test suite
...               Provides library imports, common variables, and reusable keywords
...               for testing all SwingLibrary keywords against the demo application.

Library           JavaGui.Swing    timeout=15    poll_interval=0.5
Library           OperatingSystem
Library           Process
Library           Collections
Library           String

*** Variables ***
# Application paths
${DEMO_APP_JAR}           ${CURDIR}/../../../../tests/apps/swing/target/swing-test-app-1.0.0.jar
${AGENT_JAR}              ${CURDIR}/../../../../agent/target/robotframework-swing-agent-1.0.0-all.jar

# Demo application configuration
${DEMO_MAIN_CLASS}        testapp.SwingTestApp
${DEMO_WINDOW_TITLE}      Swing Test Application

# Connection configuration
${AGENT_HOST}             localhost
${AGENT_PORT}             5678
${CONNECTION_TIMEOUT}     30

# Timeouts
${DEFAULT_TIMEOUT}        10
${SHORT_TIMEOUT}          5
${LONG_TIMEOUT}           30

# Common element locators
${MAIN_TABS}              JTabbedPane[name='mainTabbedPane']
${FORM_PANEL}             JPanel[name='formPanel']
${SELECTIONS_PANEL}       JPanel[name='selectionsPanel']
${DATA_PANEL}             JPanel[name='dataPanel']
${SETTINGS_PANEL}         JPanel[name='settingsTabPanel']

# Form elements (Form Input tab)
${NAME_FIELD}             JTextField[name='nameTextField']
${EMAIL_FIELD}            JTextField[name='emailTextField']
${PASSWORD_FIELD}         JPasswordField[name='passwordField']
${DESCRIPTION_AREA}       JTextArea[name='descriptionTextArea']
${SUBMIT_BUTTON}          JButton[name='submitButton']
${CLEAR_BUTTON}           JButton[name='clearButton']
${STATUS_LABEL}           JLabel[name='statusLabel']

# Selection elements (Selections tab)
${CATEGORY_COMBO}            JComboBox[name='categoryComboBox']
${QUANTITY_SPINNER}          JSpinner[name='quantitySpinner']
${VOLUME_SLIDER}             JSlider[name='volumeSlider']
${ENABLED_CHECKBOX}          JCheckBox[name='enabledCheckBox']
${NOTIFICATIONS_CHECKBOX}    JCheckBox[name='notificationsCheckBox']
${AUTOSAVE_CHECKBOX}         JCheckBox[name='autoSaveCheckBox']
${HIGH_PRIORITY_RADIO}       JRadioButton[name='highPriorityRadioButton']
${NORMAL_PRIORITY_RADIO}     JRadioButton[name='normalPriorityRadioButton']
${LOW_PRIORITY_RADIO}        JRadioButton[name='lowPriorityRadioButton']
${ITEM_LIST}                 JList[name='itemList']

# Table elements (Data View tab)
${DATA_TABLE}             JTable[name='dataTable']

# Tree elements
${FILE_TREE}              JTree[name='fileTree']

# Dialog elements
${SETTINGS_DIALOG}        JDialog[name='settingsDialog']
${ABOUT_DIALOG}           JDialog[name='aboutDialog']

# Progress bar
${PROGRESS_BAR}           JProgressBar[name='progressBar']

# Test data
${TEST_NAME}              John Doe
${TEST_EMAIL}             john@example.com
${TEST_PASSWORD}          testpass123

# Aliases for backward compatibility with tests
${LOGIN_BUTTON}           ${SUBMIT_BUTTON}
${VALID_USERNAME}         testuser
${VALID_PASSWORD}         testpass123

*** Keywords ***
# =============================================================================
# Test Setup Keywords
# =============================================================================

Clear Login Form
    [Documentation]    Clear the form fields for text input tests
    Select Form Input Tab
    Clear Text    ${NAME_FIELD}
    Clear Text    ${PASSWORD_FIELD}

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

Select Form Input Tab
    [Documentation]    Navigate to the Form Input tab
    Select Tab    ${MAIN_TABS}    Form Input
    Wait Until Element Is Visible    ${FORM_PANEL}    timeout=${SHORT_TIMEOUT}

Select Selections Tab
    [Documentation]    Navigate to the Selections tab
    Select Tab    ${MAIN_TABS}    Selections
    Wait Until Element Is Visible    ${SELECTIONS_PANEL}    timeout=${SHORT_TIMEOUT}

Select Data View Tab
    [Documentation]    Navigate to the Data View tab
    Select Tab    ${MAIN_TABS}    Data View
    Wait Until Element Is Visible    ${DATA_TABLE}    timeout=${SHORT_TIMEOUT}

Select Settings Tab
    [Documentation]    Navigate to the Settings tab
    Select Tab    ${MAIN_TABS}    Settings
    Wait Until Element Is Visible    ${SETTINGS_PANEL}    timeout=${SHORT_TIMEOUT}

# =============================================================================
# Form Input Keywords
# =============================================================================

Clear Form
    [Documentation]    Clear all fields in the form
    Select Form Input Tab
    Clear Text    ${NAME_FIELD}
    Clear Text    ${EMAIL_FIELD}
    Clear Text    ${PASSWORD_FIELD}
    Clear Text    ${DESCRIPTION_AREA}

Enter Form Data
    [Documentation]    Enter name and email into the form
    [Arguments]    ${name}    ${email}
    Select Form Input Tab
    Clear Text    ${NAME_FIELD}
    Input Text    ${NAME_FIELD}    ${name}
    Clear Text    ${EMAIL_FIELD}
    Input Text    ${EMAIL_FIELD}    ${email}

Submit Form
    [Documentation]    Click the submit button
    Click Button    ${SUBMIT_BUTTON}
    Sleep    0.5s    Wait for form processing

Fill And Submit Form
    [Documentation]    Complete form workflow with given data
    [Arguments]    ${name}    ${email}
    Enter Form Data    ${name}    ${email}
    Submit Form

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
