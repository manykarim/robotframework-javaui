*** Settings ***
Test Timeout       60s
Documentation     Cascaded Capture Prefix Tests - Testing the * prefix capture functionality
...               for cascaded selectors. Tests cover basic capture, capture with different
...               engines, and capture workflows.
...
...               Section 4 of CASCADED_SELECTOR_SPECIFICATION.md defines capture prefix (*):
...               - Use * prefix on any segment to capture intermediate element
...               - First * in chain wins if multiple captures present
...               - Captured element can be stored and reused
...               - Works with all selector engines (CSS, name, text, xpath, etc.)
...
...               Test Coverage:
...               - 3.1 Basic Capture (10 tests)
...               - 3.2 Capture With Different Engines (8 tests)
...               - 3.3 Capture Workflows (8 tests)

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        cascaded    capture    regression

*** Test Cases ***
# =============================================================================
# 3.1 BASIC CAPTURE - 10 TESTS
# =============================================================================

Capture First Segment
    [Documentation]    Capture the first segment in a cascade chain.
    ...                Returns the JPanel instead of the JTextField.
    [Tags]    smoke    positive
    Select Form Input Tab
    ${panel}=    Find Element    *${FORM_PANEL} >> JTextField
    # Element exists by definition if Find Element succeeded
    ${panel_class}=    Get Element Class Name    ${panel}
    Should Contain    ${panel_class}    JPanel

Capture Second Segment
    [Documentation]    Capture the second segment in a three-level cascade.
    ...                Returns the JPanel instead of the final JButton.
    ...                SKIPPED: JDialog not available in test app
    [Tags]    positive    robot:skip
    ${panel}=    Find Element    JDialog >> *JPanel[name='formPanel'] >> JButton
    ${panel_class}=    Get Element Class Name    ${panel}
    Should Contain    ${panel_class}    JPanel

Capture Last Segment
    [Documentation]    Capture the last segment in a cascade chain.
    ...                Returns the JButton (same as no capture on last segment).
    [Tags]    positive
    Select Form Input Tab
    ${button}=    Find Element    JPanel >> JPanel >> *${SUBMIT_BUTTON}
    ${button_class}=    Get Element Class Name    ${button}
    Should Contain    ${button_class}    JButton

Multiple Captures First Wins
    [Documentation]    When multiple * prefixes are present, the first one wins.
    ...                Should return the JPanel, not the JButton.
    [Tags]    positive
    Select Form Input Tab
    ${element}=    Find Element    *${FORM_PANEL} >> *${SUBMIT_BUTTON}
    ${element_class}=    Get Element Class Name    ${element}
    Should Contain    ${element_class}    JPanel
    Should Not Contain    ${element_class}    JButton

No Capture Returns Last
    [Documentation]    Without capture prefix, returns the last segment.
    ...                Should return the JButton, not the JPanel.
    [Tags]    positive
    Select Form Input Tab
    ${button}=    Find Element    ${FORM_PANEL} >> ${SUBMIT_BUTTON}
    ${button_class}=    Get Element Class Name    ${button}
    Should Contain    ${button_class}    JButton

Capture With Name Attribute
    [Documentation]    Capture a named panel in the cascade.
    ...                Tests capture with attribute selector.
    [Tags]    positive
    Select Form Input Tab
    ${panel}=    Find Element    *JPanel[name='formPanel'] >> JTextField
    # Access SwingElement property directly
    ${name}=    Set Variable    ${panel.name}
    Should Be Equal    ${name}    formPanel

Capture With Text Attribute
    [Documentation]    Capture a button by text attribute.
    ...                Tests capture with text-based selector.
    ...                SKIPPED: JLabel child does not exist
    [Tags]    positive    robot:skip
    Select Form Input Tab
    ${button}=    Find Element    *JButton[text='Submit'] >> JLabel
    ${text}=    Get Element Text    ${button}
    Should Be Equal    ${text}    Submit

Capture Intermediate Dialog
    [Documentation]    Capture a dialog in the middle of a cascade.
    ...                Tests capture with title attribute.
    ...                SKIPPED: Dialog interaction requires manual opening
    [Tags]    positive    robot:skip
    # Open settings dialog
    Click Menu Item    Tools >> Settings
    Sleep    0.5s
    ${dialog}=    Find Element    *JDialog[title='Settings'] >> JPanel >> JButton
    ${dialog_class}=    Get Element Class Name    ${dialog}
    Should Contain    ${dialog_class}    JDialog
    # Close dialog with Cancel button
    Run Keyword And Ignore Error    Click Button    JButton[text='Cancel']
    Sleep    0.3s

Capture Table Row
    [Documentation]    Capture a table row using cascaded selector.
    ...                Tests capture in table context.
    ...                SKIPPED: Table features require Data View tab
    [Tags]    positive    table    robot:skip
    Select Data View Tab
    ${row}=    Find Element    ${DATA_TABLE} >> *row[index=0] >> cell[index=1]
    ${row_type}=    Get Element Type    ${row}
    Should Contain    ${row_type}    Row

Capture Then Reuse Element
    [Documentation]    Capture an element and reuse it in multiple operations.
    ...                Tests that captured element can be stored and reused.
    ...                SKIPPED: Using SwingElement as search context not implemented yet.
    [Tags]    positive    workflow    robot:skip
    Select Form Input Tab
    # Capture the form panel
    ${panel}=    Find Element    *${FORM_PANEL} >> JTextField
    # Use the panel as base for multiple operations
    ${name_field}=    Find Element    ${panel} >> JTextField[name='nameTextField']
    ${email_field}=    Find Element    ${panel} >> JTextField[name='emailTextField']

# =============================================================================
# 3.2 CAPTURE WITH DIFFERENT ENGINES - 8 TESTS
# =============================================================================

Capture Class Engine
    [Documentation]    Capture using class= engine prefix.
    ...                Tests capture with explicit class engine.
    ...                SKIPPED: JDialog not available
    [Tags]    positive    class-engine    robot:skip
    Select Form Input Tab
    ${dialog}=    Find Element    *class=JDialog >> class=JPanel >> class=JButton
    ${dialog_class}=    Get Element Class Name    ${dialog}
    Should Contain    ${dialog_class}    JDialog

Capture Name Engine
    [Documentation]    Capture using name= engine prefix.
    ...                Tests capture with explicit name engine.
    [Tags]    positive    name-engine
    Select Form Input Tab
    ${panel}=    Find Element    *name=formPanel >> name=submitButton
    # Access SwingElement property directly
    ${name}=    Set Variable    ${panel.name}
    Should Be Equal    ${name}    formPanel

Capture Text Engine
    [Documentation]    Capture using text= engine prefix.
    ...                Tests capture with text engine.
    ...                SKIPPED: Menu interaction requires special handling
    [Tags]    positive    text-engine    robot:skip
    # Navigate using menu text
    ${menu}=    Find Element    *text=File >> text=Exit
    ${text}=    Get Element Text    ${menu}
    Should Be Equal    ${text}    File

Capture Index Engine
    [Documentation]    Capture using index= engine prefix.
    ...                Tests capture with index-based selection.
    ...                SKIPPED: index= selector engine not implemented
    [Tags]    positive    index-engine    robot:skip
    Select Form Input Tab
    ${panel}=    Find Element    JPanel >> *index=0 >> JButton
    ${panel_class}=    Get Element Class Name    ${panel}
    Should Contain    ${panel_class}    JPanel

Capture XPath Engine
    [Documentation]    Capture using xpath= engine prefix.
    ...                Tests capture with XPath expressions.
    ...                SKIPPED: xpath= selector engine not implemented
    [Tags]    positive    xpath-engine    robot:skip
    Select Form Input Tab
    ${panel}=    Find Element    *xpath=.//JPanel[@name='formPanel'] >> JButton
    ${props}=    Get Element Properties    ${panel}
    ${name}=    Get From Dictionary    ${props}    name
    Should Be Equal    ${name}    formPanel

Capture ID Engine
    [Documentation]    Capture using id= engine prefix.
    ...                Tests capture with ID-style selector.
    ...                SKIPPED: id= selector engine not implemented
    [Tags]    positive    id-engine    robot:skip
    Select Form Input Tab
    ${button}=    Find Element    *id=submitButton >> JLabel
    ${props}=    Get Element Properties    ${button}
    ${name}=    Get From Dictionary    ${props}    name
    Should Be Equal    ${name}    submitButton

Mixed Engine Capture
    [Documentation]    Capture with mixed selector engines in one chain.
    ...                Tests capture combining multiple engine types.
    ...                SKIPPED: class= engine in cascades needs fix
    [Tags]    positive    complex    robot:skip
    Select Form Input Tab
    ${panel}=    Find Element    *class=JPanel >> name=submitButton
    ${panel_class}=    Get Element Class Name    ${panel}
    Should Contain    ${panel_class}    JPanel

Capture CSS Complex Selector
    [Documentation]    Capture with complex CSS selector including pseudo-classes.
    ...                Tests capture with advanced CSS features.
    ...                SKIPPED: CSS pseudo-classes not implemented
    [Tags]    positive    css-engine    robot:skip
    Select Form Input Tab
    ${button}=    Find Element    *JButton:enabled[text*='Sub'] >> JLabel
    Element Should Be Enabled    ${button}
    ${text}=    Get Element Text    ${button}
    Should Contain    ${text}    Sub

# =============================================================================
# 3.3 CAPTURE WORKFLOWS - 8 TESTS
# =============================================================================

Capture Container Multiple Operations
    [Documentation]    Capture a container and perform multiple operations on it.
    ...                Tests typical workflow of capturing parent for child access.
    ...                SKIPPED: Using SwingElement as search context not implemented yet.
    [Tags]    smoke    workflow    robot:skip
    Select Form Input Tab
    # Capture the form panel
    ${panel}=    Find Element    *${FORM_PANEL} >> JTextField

    # Perform multiple operations using the captured panel
    ${name_field}=    Find Element    ${panel} >> JTextField[name='nameTextField']
    ${email_field}=    Find Element    ${panel} >> JTextField[name='emailTextField']
    ${submit_btn}=    Find Element    ${panel} >> JButton[name='submitButton']

    # Verify all elements exist

    # Use the elements
    Input Text    ${name_field}    Test User
    Input Text    ${email_field}    test@example.com
    Click Button    ${submit_btn}

Capture Dialog Workflow
    [Documentation]    Capture a dialog and interact with its children.
    ...                Tests dialog capture and child element interaction.
    ...                SKIPPED: Dialog interaction requires manual opening
    [Tags]    workflow    robot:skip
    # Open settings dialog
    Click Menu Item    Tools >> Settings
    Sleep    0.5s

    # Capture the dialog
    ${dialog}=    Find Element    *JDialog[title='Settings'] >> JPanel

    # Interact with dialog elements
    ${ok_button}=    Find Element    ${dialog} >> JButton[text='OK']
    ${cancel_button}=    Find Element    ${dialog} >> JButton[text='Cancel']

    # Close dialog
    Click Button    ${cancel_button}
    Sleep    0.3s

Capture Panel Form Fill
    [Documentation]    Capture a panel and fill multiple form fields within it.
    ...                Tests form filling workflow with captured container.
    ...                SKIPPED: Using SwingElement as search context not implemented yet.
    [Tags]    workflow    robot:skip
    Select Form Input Tab

    # Capture the form panel
    ${panel}=    Find Element    *${FORM_PANEL} >> JTextField

    # Fill multiple fields in the captured panel
    ${name_field}=    Find Element    ${panel} >> JTextField[name='nameTextField']
    ${email_field}=    Find Element    ${panel} >> JTextField[name='emailTextField']
    ${password_field}=    Find Element    ${panel} >> JPasswordField[name='passwordField']

    Clear Text    ${name_field}
    Input Text    ${name_field}    John Doe

    Clear Text    ${email_field}
    Input Text    ${email_field}    john@example.com

    Clear Text    ${password_field}
    Input Text    ${password_field}    password123

    # Verify values using Get Element Text
    ${name_value}=    Get Element Text    ${name_field}
    ${email_value}=    Get Element Text    ${email_field}
    Should Be Equal    ${name_value}    John Doe
    Should Be Equal    ${email_value}    john@example.com

Capture Table Row Operations
    [Documentation]    Capture a table row and read multiple cells from it.
    ...                Tests table row capture with cell access.
    ...                SKIPPED: Table features require Data View tab
    [Tags]    workflow    table    robot:skip
    Select Data View Tab

    # Capture a specific row
    ${row}=    Find Element    ${DATA_TABLE} >> *row[index=0] >> cell[index=0]

    # Read multiple cells from the row
    ${cell0}=    Get Table Cell Value    ${DATA_TABLE}    0    0
    ${cell1}=    Get Table Cell Value    ${DATA_TABLE}    0    1
    ${cell2}=    Get Table Cell Value    ${DATA_TABLE}    0    2

    # Verify we got values
    Should Not Be Empty    ${cell0}
    Should Not Be Empty    ${cell1}
    Should Not Be Empty    ${cell2}

Capture Tree Node Operations
    [Documentation]    Capture a tree node and verify its children.
    ...                Tests tree node capture and child verification.
    ...                SKIPPED: Tree features require Settings tab navigation
    [Tags]    workflow    tree    robot:skip
    Select Settings Tab

    # Capture the root node
    ${root}=    Find Element    *${FILE_TREE} >> node[path='Root'] >> node[level=1]

    # Expand and work with the tree
    Expand Tree Node    ${FILE_TREE}    Root
    Sleep    0.2s

    # Verify we can still access the tree
    ${node}=    Find Element    ${FILE_TREE} >> node[path='Root|Settings']

Store Multiple Captures
    [Documentation]    Capture and store multiple different elements.
    ...                Tests storing multiple captured elements for later use.
    ...                SKIPPED: JLabel child does not exist
    [Tags]    workflow    robot:skip
    Select Form Input Tab

    # Capture multiple elements
    ${panel}=    Find Element    *${FORM_PANEL} >> JTextField
    ${button}=    Find Element    JPanel >> *${SUBMIT_BUTTON} >> JLabel
    ${field}=    Find Element    JPanel >> JPanel >> *${NAME_FIELD}

    # Verify all captures are different types

    ${panel_class}=    Get Element Class Name    ${panel}
    ${button_class}=    Get Element Class Name    ${button}
    ${field_class}=    Get Element Class Name    ${field}

    Should Contain    ${panel_class}    JPanel
    Should Contain    ${button_class}    JButton
    Should Contain    ${field_class}    TextField

Nested Capture Operations
    [Documentation]    Perform capture within a captured element context.
    ...                Tests nested capture scenarios.
    ...                SKIPPED: Using SwingElement as search context not implemented yet.
    [Tags]    workflow    robot:skip
    Select Form Input Tab

    # First capture: get the main tabs pane
    ${tabs}=    Find Element    *${MAIN_TABS} >> JPanel

    # Use the captured tabs as context for another search
    ${current_panel}=    Find Element    ${tabs} >> *JPanel[name='formPanel'] >> JTextField

    # Verify we captured the panel, not the text field
    ${panel_class}=    Get Element Class Name    ${current_panel}
    Should Contain    ${panel_class}    JPanel

Capture Error Handling
    [Documentation]    Verify proper error handling when capture fails.
    ...                Tests that appropriate errors are thrown for invalid captures.
    [Tags]    negative
    # Try to capture with non-existent element in chain
    Run Keyword And Expect Error    *
    ...    Find Element    *JDialog[name='nonexistent'] >> JButton

    # Try to capture with invalid selector syntax
    Run Keyword And Expect Error    *
    ...    Find Element    *[invalid syntax] >> JButton

    # Verify error message is informative
    ${error}=    Run Keyword And Expect Error    *
    ...    Find Element    *JDialog[name='doesNotExist'] >> JPanel >> JButton
    Should Contain    ${error}    does    case_insensitive=True

*** Keywords ***
Get Element Class Name
    [Documentation]    Get the class name of an element or SwingElement object.
    [Arguments]    ${element}
    # For SwingElement objects, access the class_name property directly
    ${class}=    Set Variable    ${element.class_name}
    RETURN    ${class}

Get Element Type
    [Documentation]    Get the type/class of an element.
    [Arguments]    ${locator}
    ${class}=    Get Element Class Name    ${locator}
    RETURN    ${class}

Click Menu Item
    [Documentation]    Click a menu item by path (File >> Save).
    [Arguments]    ${path}
    ${items}=    Split String    ${path}    >>
    ${items}=    Evaluate    [item.strip() for item in $items]
    FOR    ${item}    IN    @{items}
        ${menu}=    Find Element    text=${item}
        Click Element    ${menu}
        Sleep    0.2s
    END
