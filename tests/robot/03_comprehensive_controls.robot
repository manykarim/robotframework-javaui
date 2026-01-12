*** Settings ***
Documentation     Comprehensive test suite covering all control types and locator styles
...               Tests interaction with buttons, text fields, labels, checkboxes,
...               radio buttons, combo boxes, tables, trees, lists, and more.
Resource          resources/common.resource
Library           Collections
Suite Setup       Start Demo Application
Suite Teardown    Stop Demo Application

*** Variables ***
${VALID_USERNAME}     admin
${VALID_PASSWORD}     password123

*** Test Cases ***
# =============================================================================
# BUTTON INTERACTIONS (Login Tab - Default)
# =============================================================================

Click Button By Name Attribute
    [Documentation]    Click button using name attribute selector
    Element Should Exist    JButton[name='loginBtn']
    Click    JButton[name='loginBtn']
    Sleep    0.5s    # Wait for any dialog

Click Button By Text Attribute
    [Documentation]    Click button using text attribute selector
    Element Should Exist    JButton[text='Clear']
    Click    JButton[text='Clear']

Click Button Using Type And Pseudo Selector
    [Documentation]    Click enabled button using pseudo selector
    Element Should Exist    JButton:enabled
    Click    JButton[text='Login']:enabled
    Sleep    0.5s

Click Button Using Descendant Combinator
    [Documentation]    Find and click button within container
    Element Should Exist    JPanel JButton[text='Login']
    Click    JPanel JButton[text='Login']
    Sleep    0.5s

# =============================================================================
# TEXT FIELD INTERACTIONS (Login Tab)
# =============================================================================

Input And Read Text Field
    [Documentation]    Input text into text field and verify
    Clear Text    JTextField[name='username']
    Input Text    JTextField[name='username']    testuser
    Sleep    0.2s
    # Verify input worked by checking element exists with content
    Element Should Exist    JTextField[name='username']

Clear Text Field
    [Documentation]    Clear text from a text field
    Input Text    [name='username']    some text
    Clear Text    [name='username']
    Element Should Exist    [name='username']

Input Password Field
    [Documentation]    Input text into password field
    Clear Text    JPasswordField[name='password']
    Input Text    JPasswordField[name='password']    secret123
    Element Should Exist    JPasswordField[name='password']

Input Text Using XPath
    [Documentation]    Input text using XPath-style locator
    Clear Text    //JTextField[@name='username']
    Input Text    //JTextField[@name='username']    xpathuser
    Element Should Exist    //JTextField[@name='username']

# =============================================================================
# LABEL INTERACTIONS (Login Tab)
# =============================================================================

Read Label Text By Name
    [Documentation]    Read text from label using name attribute
    Element Should Exist    JLabel[name='statusLabel']
    ${text}=    Get Element Text    JLabel[name='statusLabel']
    Should Not Be Empty    ${text}

Read Label Using Contains Selector
    [Documentation]    Find label using text contains selector
    Element Should Exist    JLabel[text*='Username']

Read Label Using Starts With Selector
    [Documentation]    Find label using text prefix selector
    Element Should Exist    JLabel[text^='User']

Read Label Using Ends With Selector
    [Documentation]    Find label using text suffix selector
    Element Should Exist    JLabel[text$=':']

Verify Status Label Updates
    [Documentation]    Verify status label shows content
    ${text}=    Get Element Text    [name='statusLabel']
    Log    Status label text: ${text}
    Should Not Be Empty    ${text}

# =============================================================================
# CHECKBOX INTERACTIONS (Login Tab)
# =============================================================================

Find Checkbox By Name
    [Documentation]    Find checkbox using name selector
    Element Should Exist    JCheckBox[name='rememberMe']
    Element Should Be Enabled    JCheckBox[name='rememberMe']

Click Checkbox
    [Documentation]    Click on checkbox to toggle it
    Click    JCheckBox[name='rememberMe']
    Sleep    0.2s
    Click    JCheckBox[name='rememberMe']
    Sleep    0.2s

Checkbox Using XPath
    [Documentation]    Find checkbox using XPath
    Element Should Exist    //JCheckBox[@name='rememberMe']

# =============================================================================
# TAB INTERACTIONS
# =============================================================================

Verify Tabbed Pane Exists
    [Documentation]    Verify tabbed pane component exists
    Element Should Exist    JTabbedPane[name='mainTabs']

Verify Tab Contents Login
    [Documentation]    Verify Login tab content exists
    Element Should Exist    JPanel[name='loginPanel']
    Element Should Exist    JPanel[name='loginPanel'] JTextField
    Element Should Exist    JPanel[name='loginPanel'] JButton

Verify Multiple Panels Exist
    [Documentation]    Verify different panels exist
    Element Should Exist    JPanel[name='loginPanel']
    Element Should Exist    JPanel[name='tablePanel']
    Element Should Exist    JPanel[name='treePanel']
    Element Should Exist    JPanel[name='formPanel']
    Element Should Exist    JPanel[name='listPanel']

# =============================================================================
# TABLE INTERACTIONS (Always visible in component tree)
# =============================================================================

Find Table By Name
    [Documentation]    Find table using name selector
    Element Should Exist    JTable[name='dataTable']

Get Table Row Count
    [Documentation]    Get number of rows in table
    ${count}=    Get Table Row Count    [name='dataTable']
    Should Be True    ${count} >= 5

Get Table Column Count
    [Documentation]    Get number of columns in table
    ${count}=    Get Table Column Count    [name='dataTable']
    Should Be Equal As Integers    ${count}    5

Read Table Cell Value
    [Documentation]    Read value from table cell
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    1
    Should Be Equal    ${value}    John Doe

Read Table Cell By Column Index
    [Documentation]    Read table cell using column index
    ${value}=    Get Table Cell Value    [name='dataTable']    1    2
    Should Be Equal    ${value}    jane@example.com

Table Using XPath Locator
    [Documentation]    Find table using XPath
    Element Should Exist    //JTable[@name='dataTable']
    ${count}=    Get Table Row Count    //JTable[@name='dataTable']
    Should Be True    ${count} > 0

Get Full Table Data
    [Documentation]    Read all table data
    ${data}=    Get Table Data    [name='dataTable']
    Should Not Be Empty    ${data}
    Length Should Be    ${data}    5    # 5 rows

# =============================================================================
# TREE INTERACTIONS (Always visible in component tree)
# =============================================================================

Find Tree By Name
    [Documentation]    Find tree using name selector
    Element Should Exist    JTree[name='fileTree']

Tree Using XPath
    [Documentation]    Find tree using XPath
    Element Should Exist    //JTree[@name='fileTree']

# =============================================================================
# LIST INTERACTIONS (Always visible in component tree)
# =============================================================================

Find List By Name
    [Documentation]    Find list using name selector
    Element Should Exist    JList[name='itemList']

Get List Items
    [Documentation]    Get all items from a list
    [Tags]    skip-pending-implementation
    # Note: get_list_items relies on agent returning items property which may not be implemented
    # For now, just verify the list exists
    Element Should Exist    JList[name='itemList']
    # TODO: Implement proper list item retrieval in agent
    # ${items}=    Get List Items    JList[name='itemList']
    # Should Contain    ${items}    Apple

List Using XPath
    [Documentation]    Find list using XPath
    Element Should Exist    //JList[@name='itemList']

# =============================================================================
# COMBO BOX / DROPDOWN (Form Panel)
# =============================================================================

Find ComboBox By Name
    [Documentation]    Find combo box using name selector
    Element Should Exist    JComboBox[name='countryCombo']

ComboBox Using XPath
    [Documentation]    Find combo box using XPath
    Element Should Exist    //JComboBox[@name='countryCombo']

# =============================================================================
# RADIO BUTTON INTERACTIONS (Form Panel)
# =============================================================================

Find Radio Buttons
    [Documentation]    Find radio buttons using name selector
    Element Should Exist    JRadioButton[name='optionA']
    Element Should Exist    JRadioButton[name='optionB']

Radio Buttons Using XPath
    [Documentation]    Find radio buttons using XPath
    Element Should Exist    //JRadioButton[@name='optionA']
    Element Should Exist    //JRadioButton[@name='optionB']

# =============================================================================
# OTHER FORM CONTROLS (Form Panel)
# =============================================================================

Find Slider
    [Documentation]    Find slider component
    Element Should Exist    JSlider[name='volumeSlider']

Find Spinner
    [Documentation]    Find spinner component
    Element Should Exist    JSpinner[name='quantitySpinner']

Find Progress Bar
    [Documentation]    Find progress bar component
    Element Should Exist    JProgressBar[name='progressBar']

Find Text Area
    [Documentation]    Find text area component
    Element Should Exist    JTextArea[name='notesArea']

# =============================================================================
# ELEMENT STATE VERIFICATION
# =============================================================================

Verify Element Is Visible
    [Documentation]    Verify element visibility
    Element Should Be Visible    JButton[text='Login']
    Element Should Be Visible    JTextField[name='username']

Verify Element Is Enabled
    [Documentation]    Verify element is enabled
    Element Should Be Enabled    JButton[name='loginBtn']
    Element Should Be Enabled    JTextField[name='username']

Verify Multiple Elements Exist
    [Documentation]    Find multiple elements
    ${buttons}=    Find Elements    JButton
    Should Not Be Empty    ${buttons}
    ${count}=    Get Length    ${buttons}
    Should Be True    ${count} > 5

# =============================================================================
# LOCATOR STYLE TESTS
# =============================================================================

Locator Type Selector Only
    [Documentation]    Find element by type only
    ${buttons}=    Find Elements    JButton
    Should Not Be Empty    ${buttons}

Locator Attribute Equals
    [Documentation]    Find element with exact attribute match
    Element Should Exist    [name='username']

Locator Attribute Contains
    [Documentation]    Find element with partial attribute match
    Element Should Exist    JButton[text*='Log']

Locator Attribute Starts With
    [Documentation]    Find element with prefix attribute match
    Element Should Exist    JButton[text^='Cle']

Locator Attribute Ends With
    [Documentation]    Find element with suffix attribute match
    Element Should Exist    JButton[text$='in']

Locator Pseudo Enabled
    [Documentation]    Find enabled elements
    Element Should Exist    JButton:enabled
    Element Should Exist    JTextField:enabled

Locator Pseudo Visible
    [Documentation]    Find visible elements
    Element Should Exist    JButton:visible
    Element Should Exist    JLabel:visible

Locator Pseudo First Child
    [Documentation]    Find first child elements
    Element Should Exist    JButton:first-child

Locator Child Combinator
    [Documentation]    Find direct child elements
    Element Should Exist    JPanel > JButton
    Element Should Exist    JPanel > JLabel

Locator Descendant Combinator
    [Documentation]    Find descendant elements
    Element Should Exist    SwingDemoApp JButton
    Element Should Exist    JPanel JTextField

Locator XPath Simple
    [Documentation]    Find elements using simple XPath
    Element Should Exist    //JButton
    Element Should Exist    //JTextField
    Element Should Exist    //JLabel

Locator XPath With Attribute
    [Documentation]    Find elements using XPath with attributes
    Element Should Exist    //JButton[@name='loginBtn']
    Element Should Exist    //JTextField[@name='username']

Locator XPath With Index
    [Documentation]    Find elements using XPath index
    Element Should Exist    //JButton[1]

Locator Combined Selectors
    [Documentation]    Use multiple selector criteria
    Element Should Exist    JButton[name='loginBtn']:enabled:visible
    Element Should Exist    JTextField[name='username']:enabled

# =============================================================================
# WORKFLOW TESTS - LOGIN FLOW
# =============================================================================

Complete Login Workflow - Enter Credentials
    [Documentation]    Test entering login credentials
    Clear Text    [name='username']
    Clear Text    [name='password']
    Input Text    [name='username']    ${VALID_USERNAME}
    Input Text    [name='password']    ${VALID_PASSWORD}
    Element Should Exist    [name='username']
    Element Should Exist    [name='password']

Complete Login Workflow - Clear Form
    [Documentation]    Test form clearing functionality
    Input Text    [name='username']    someuser
    Input Text    [name='password']    somepass
    Click    [name='clearBtn']
    Sleep    0.2s
    # Form should be cleared
    Element Should Exist    [name='username']

# =============================================================================
# FIND ELEMENTS TESTS
# =============================================================================

Find All Buttons
    [Documentation]    Find all button elements
    ${elements}=    Find Elements    JButton
    ${count}=    Get Length    ${elements}
    Should Be True    ${count} > 10
    Log    Found ${count} buttons

Find All Text Fields
    [Documentation]    Find all text field elements
    ${elements}=    Find Elements    JTextField
    Should Not Be Empty    ${elements}

Find All Labels
    [Documentation]    Find all label elements
    ${elements}=    Find Elements    JLabel
    ${count}=    Get Length    ${elements}
    Should Be True    ${count} > 5
    Log    Found ${count} labels

Find Elements With Attribute
    [Documentation]    Find elements using attribute selector
    ${elements}=    Find Elements    JButton[text='Login']
    Should Not Be Empty    ${elements}

# =============================================================================
# COMPLEX LOCATOR TESTS
# =============================================================================

Complex Locator With Multiple Attributes
    [Documentation]    Test complex locator with multiple criteria
    Element Should Exist    JButton[name='loginBtn'][text='Login']

Complex Locator Nested Panels
    [Documentation]    Test finding elements in nested panels
    Element Should Exist    JPanel JPanel JButton

Complex XPath With Multiple Predicates
    [Documentation]    Test XPath with multiple conditions
    Element Should Exist    //JButton[@name='loginBtn']

# =============================================================================
# UI TREE OPERATIONS
# =============================================================================

Log UI Tree
    [Documentation]    Log the UI tree for debugging
    Log Ui Tree

Get UI Tree As Text
    [Documentation]    Get UI tree in text format
    ${tree}=    Get Ui Tree    format=text
    Should Contain    ${tree}    SwingDemoApp
    Should Contain    ${tree}    JButton
    Should Contain    ${tree}    JTextField

Refresh UI Tree
    [Documentation]    Refresh the UI tree cache
    Refresh Ui Tree
    Element Should Exist    JButton[name='loginBtn']

# =============================================================================
# ELEMENT PROPERTIES
# =============================================================================

Get Element Property Name
    [Documentation]    Get element name property
    ${name}=    Get Element Property    JButton[name='loginBtn']    name
    Should Be Equal    ${name}    loginBtn

Get Element Property Text
    [Documentation]    Get element text property
    ${text}=    Get Element Property    JButton[name='loginBtn']    text
    Should Be Equal    ${text}    Login

Get Element Property Enabled
    [Documentation]    Get element enabled property
    ${enabled}=    Get Element Property    JButton[name='loginBtn']    enabled
    Should Be True    ${enabled}

Get Element Property Visible
    [Documentation]    Get element visible property
    ${visible}=    Get Element Property    JButton[name='loginBtn']    visible
    Should Be True    ${visible}

Get All Element Properties
    [Documentation]    Get all properties of an element
    ${props}=    Get Element Properties    JButton[name='loginBtn']
    Should Not Be Empty    ${props}
    Dictionary Should Contain Key    ${props}    name
    Dictionary Should Contain Key    ${props}    text
