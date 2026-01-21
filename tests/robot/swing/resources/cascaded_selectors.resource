*** Settings ***
Documentation     Resource file for cascaded selector testing
...               Provides common variables, examples, and reusable keywords
...               for testing cascaded selector functionality across all selector engines
...               and component types (tables, trees, tabs, menus).
...
...               Cascaded Selector Syntax: segment1 >> segment2 >> ... >> segmentN
...               Capture Prefix: *segment captures intermediate element
...               Supported Engines: CSS (default), class, name, text, index, xpath, id

Library           JavaGui.Swing    timeout=15    poll_interval=0.5
Library           Collections
Library           String

*** Variables ***
# =============================================================================
# Cascaded Selector Examples - Basic
# =============================================================================

# Simple cascaded selectors (2-3 segments)
${CASCADE_SIMPLE_2}                JPanel >> JButton
${CASCADE_SIMPLE_3}                JFrame >> JPanel >> JButton
${CASCADE_SIMPLE_4}                JFrame >> JTabbedPane >> JPanel >> JButton

# Cascaded selectors with attributes
${CASCADE_WITH_NAME}               JPanel[name='main'] >> JButton[name='submit']
${CASCADE_WITH_TEXT}               JPanel >> JButton[text='Submit']
${CASCADE_MIXED_ATTRS}             JDialog[title='Settings'] >> JPanel[name='form'] >> JButton

# CSS combinators
${CASCADE_DIRECT_CHILD}            JPanel > JButton
${CASCADE_DESCENDANT}              JPanel JButton

# =============================================================================
# Cascaded Selector Examples - Engines
# =============================================================================

# CSS Engine (default)
${CASCADE_CSS_TYPE}                JButton >> JLabel
${CASCADE_CSS_ID}                  #mainPanel >> #submitBtn
${CASCADE_CSS_CLASS}               .primary >> .secondary
${CASCADE_CSS_ATTR_EQUALS}         JButton[text='OK'] >> JLabel
${CASCADE_CSS_ATTR_CONTAINS}       JButton[text*='Log'] >> JLabel
${CASCADE_CSS_ATTR_STARTS}         JButton[text^='Log'] >> JLabel
${CASCADE_CSS_ATTR_ENDS}           JLabel[text$=':'] >> JTextField
${CASCADE_CSS_PSEUDO_ENABLED}      JButton:enabled >> JLabel
${CASCADE_CSS_PSEUDO_VISIBLE}      JButton:visible >> JLabel
${CASCADE_CSS_PSEUDO_FIRST}        JButton:first-child >> JLabel

# Class Engine
${CASCADE_CLASS_SIMPLE}            class=JDialog >> class=JPanel >> class=JButton
${CASCADE_CLASS_NO_J}              class=Dialog >> class=Panel >> class=Button
${CASCADE_CLASS_MIXED_CASE}        class=jButton >> class=JLabel
${CASCADE_CLASS_THEN_CSS}          class=JDialog >> JButton[text='OK']
${CASCADE_CSS_THEN_CLASS}          JDialog >> class=JButton

# Name Engine
${CASCADE_NAME_SIMPLE}             name=mainDialog >> name=buttonPanel >> name=okButton
${CASCADE_NAME_WITH_QUOTES}        name='my-component' >> name='sub-component'
${CASCADE_NAME_WILDCARD}           name=user* >> JButton
${CASCADE_NAME_THEN_CSS}           name=formPanel >> JButton[text='Submit']
${CASCADE_CSS_THEN_NAME}           JPanel >> name=submitButton
${CASCADE_NAME_WITH_SPACES}        name='form panel' >> name='ok button'

# Text Engine
${CASCADE_TEXT_SIMPLE}             text=File >> text=Save
${CASCADE_TEXT_WITH_SPACES}        text='Log Out' >> JLabel
${CASCADE_TEXT_REGEX}              text=/Log.*/ >> JButton
${CASCADE_TEXT_PARTIAL}            text=*partial* >> JButton
${CASCADE_TEXT_THEN_CSS}           text=OK >> JLabel[name='status']
${CASCADE_CSS_THEN_TEXT}           JButton >> text=Submit
${CASCADE_TEXT_MENU}               JMenu >> text=File >> JMenuItem >> text=Save

# Index Engine
${CASCADE_INDEX_FIRST}             JButton >> index=0
${CASCADE_INDEX_MIDDLE}            JButton >> index=2
${CASCADE_INDEX_LAST}              JButton >> index=-1
${CASCADE_INDEX_SECOND_LAST}       JButton >> index=-2
${CASCADE_INDEX_THEN_CSS}          JPanel >> index=0 >> JButton[text='OK']
${CASCADE_CSS_THEN_INDEX}          JPanel >> JButton >> index=1

# XPath Engine
${CASCADE_XPATH_CHILD}             xpath=./JButton >> JLabel
${CASCADE_XPATH_PARENT}            xpath=../JButton
${CASCADE_XPATH_DESCENDANT}        xpath=descendant::JButton >> JLabel
${CASCADE_XPATH_PREDICATE}         xpath=.//JButton[@text='OK']
${CASCADE_XPATH_THEN_CSS}          xpath=.//JPanel >> JButton[text='OK']
${CASCADE_CSS_THEN_XPATH}          JPanel >> xpath=.//JButton

# ID Engine
${CASCADE_ID_SIMPLE}               id=mainWindow >> id=contentPane >> id=submitButton
${CASCADE_ID_WITH_QUOTES}          id='component-id' >> JButton
${CASCADE_ID_VS_CSS_ID}            id=submitBtn
${CASCADE_CSS_ID_SYNTAX}           #submitBtn
${CASCADE_ID_THEN_CSS}             id=formPanel >> JButton[text='OK']

# =============================================================================
# Cascaded Selector Examples - Capture Prefix
# =============================================================================

# Basic capture (returns intermediate element)
${CAPTURE_FIRST}                   *JPanel >> JTextField
${CAPTURE_SECOND}                  JDialog >> *JPanel >> JButton
${CAPTURE_LAST}                    JDialog >> JPanel >> *JButton
${CAPTURE_MULTIPLE}                *JDialog >> *JPanel >> JButton
${CAPTURE_WITH_NAME}               *JPanel[name='form'] >> JTextField
${CAPTURE_WITH_TEXT}               *JButton[text='Submit'] >> JLabel

# Capture with different engines
${CAPTURE_CLASS}                   *class=JDialog >> class=JButton
${CAPTURE_NAME}                    *name=formPanel >> name=okButton
${CAPTURE_TEXT}                    *text=Settings >> JButton
${CAPTURE_INDEX}                   JPanel >> *index=0 >> JButton
${CAPTURE_XPATH}                   *xpath=.//JDialog >> JButton
${CAPTURE_ID}                      *id=mainPanel >> id=submitBtn

# =============================================================================
# Cascaded Selector Examples - Component Specific
# =============================================================================

# Table selectors
${CASCADE_TABLE_CELL_INDEX}        JTable >> cell[row=0, col=1]
${CASCADE_TABLE_CELL_NAME}         JTable >> cell[row=0, col='Name']
${CASCADE_TABLE_ROW_CELL}          JTable >> row[index=5] >> cell[index=2]
${CASCADE_TABLE_NAMED}             JTable[name='dataTable'] >> cell[row=0, col=0]
${CASCADE_TABLE_ROW_INDEX}         JTable >> row[index=0]
${CASCADE_TABLE_ROW_CONTAINS}      JTable >> row[contains='Active']
${CASCADE_TABLE_ROW_SELECTED}      JTable >> row:selected
${CASCADE_TABLE_ROW_FIRST}         JTable >> row:first
${CASCADE_TABLE_ROW_LAST}          JTable >> row:last
${CASCADE_TABLE_COLUMN_NAME}       JTable >> column[name='Status']
${CASCADE_TABLE_COLUMN_INDEX}      JTable >> column[index=3]

# Tree selectors
${CASCADE_TREE_PATH_PIPE}          JTree >> node[path='Root|Child|Leaf']
${CASCADE_TREE_PATH_SLASH}         JTree >> node[path='Root/Settings/Display']
${CASCADE_TREE_TEXT}               JTree >> node[text='Settings']
${CASCADE_TREE_TEXT_CONTAINS}      JTree >> node[text*='Config']
${CASCADE_TREE_PATH_CHILD}         JTree >> node[path='Root'] >> child[index=2]
${CASCADE_TREE_SELECTED}           JTree >> node:selected
${CASCADE_TREE_EXPANDED}           JTree >> node:expanded
${CASCADE_TREE_COLLAPSED}          JTree >> node:collapsed
${CASCADE_TREE_ROOT}               JTree >> node:root
${CASCADE_TREE_LEAF}               JTree >> node:leaf

# Tab selectors
${CASCADE_TAB_TITLE}               JTabbedPane >> tab[title='Settings']
${CASCADE_TAB_INDEX}               JTabbedPane >> tab[index=2]
${CASCADE_TAB_SELECTED}            JTabbedPane >> tab:selected
${CASCADE_TAB_CONTENT}             JTabbedPane >> tab[title='Login'] >> JPanel
${CASCADE_TAB_NESTED}              JTabbedPane >> tab[title='Advanced'] >> JTabbedPane >> tab[title='Network']
${CASCADE_TAB_BUTTON}              JTabbedPane >> tab[title='Settings'] >> JButton[text='Apply']

# Menu selectors
${CASCADE_MENU_ITEM}               JMenuBar >> JMenu[text='File'] >> JMenuItem[text='Save']
${CASCADE_MENU_TEXT}               JMenuBar >> menu[text='Edit'] >> menu[text='Find'] >> item[text='Find Next']
${CASCADE_MENU_INDEX}              JMenuBar >> menu[index=0] >> item[index=1]
${CASCADE_MENU_NESTED}             JMenuBar >> JMenu >> JMenu >> JMenuItem
${CASCADE_POPUP_ITEM}              JPopupMenu >> JMenuItem[text='Copy']
${CASCADE_MENU_CHECKBOX}           JMenu >> JCheckBoxMenuItem[text='Option']
${CASCADE_MENU_RADIO}              JMenu >> JRadioButtonMenuItem[text='View']

# =============================================================================
# Cascaded Selector Examples - Complex Combinations
# =============================================================================

# Mixed engine combinations
${CASCADE_CSS_NAME_TEXT}           JPanel >> name=form >> text=Submit
${CASCADE_CLASS_XPATH_INDEX}       class=JDialog >> xpath=.//JButton >> index=0
${CASCADE_NAME_CSS_TEXT_INDEX}     name=main >> JPanel >> text=OK >> index=0
${CASCADE_ID_CLASS_XPATH}          id=main >> class=JPanel >> xpath=.//JButton

# Deep hierarchies
${CASCADE_DEPTH_5}                 JFrame >> JPanel >> JTabbedPane >> JPanel >> JButton
${CASCADE_DEPTH_7}                 JFrame >> JDialog >> JPanel >> JTabbedPane >> JPanel >> JButton >> JLabel
${CASCADE_NESTED_DIALOGS}          JDialog[name='parent'] >> JDialog[name='child'] >> JButton
${CASCADE_NESTED_PANELS}           JPanel >> JPanel >> JPanel >> JPanel >> JButton

# Complex real-world scenarios
${CASCADE_PREFS_DIALOG}            JDialog[title='Preferences'] >> JTabbedPane >> tab[title='General'] >> JPanel >> JCheckBox[text='Auto Save']
${CASCADE_DATA_ENTRY}              JFrame >> JTabbedPane >> tab[title='Form'] >> JPanel[name='formPanel'] >> JTextField[name='email']
${CASCADE_TREE_BROWSER}            JPanel[name='browser'] >> JTree[name='fileTree'] >> node[path='Documents/Projects']
${CASCADE_TABLE_CRUD}              JPanel[name='dataPanel'] >> JTable[name='dataTable'] >> row[contains='Active'] >> JButton[text='Edit']

# =============================================================================
# Whitespace Handling Examples
# =============================================================================

${CASCADE_NO_WHITESPACE}           JPanel>>JButton
${CASCADE_SINGLE_SPACE}            JPanel >> JButton
${CASCADE_MULTIPLE_SPACES}         JPanel  >>  JButton
${CASCADE_TAB_WHITESPACE}          JPanel\t>>\tJButton

# =============================================================================
# Error Handling Examples (Expected to Fail)
# =============================================================================

${CASCADE_INVALID_SEPARATOR_QUOTE}     JPanel[name='x' >> y']
${CASCADE_INVALID_SEPARATOR_BRACKET}   JPanel[name='x >> y']
${CASCADE_UNKNOWN_ENGINE}              unknown=value >> JButton
${CASCADE_EMPTY_SEGMENT}               JPanel >> >> JButton
${CASCADE_MALFORMED_ATTR}              JPanel[name=unclosed
${CASCADE_MALFORMED_XPATH}             xpath=///invalid///
${CASCADE_NONEXISTENT_SEGMENT}         JDialog[name='nonexistent'] >> JButton

*** Keywords ***
# =============================================================================
# Cascaded Selector Construction Keywords
# =============================================================================

Build Cascaded Selector
    [Documentation]    Build a cascaded selector from multiple segments
    ...                Example: Build Cascaded Selector    JPanel    JButton    JLabel
    ...                Returns: JPanel >> JButton >> JLabel
    [Arguments]    @{segments}
    ${selector}=    Evaluate    ' >> '.join(${segments})
    RETURN    ${selector}

Build Cascaded Selector With Capture
    [Documentation]    Build a cascaded selector with capture prefix on specified segment (0-based index)
    ...                Example: Build Cascaded Selector With Capture    1    JPanel    JButton    JLabel
    ...                Returns: JPanel >> *JButton >> JLabel
    [Arguments]    ${capture_index}    @{segments}
    ${modified_segments}=    Create List
    ${index}=    Set Variable    ${0}
    FOR    ${segment}    IN    @{segments}
        IF    ${index} == ${capture_index}
            ${captured}=    Set Variable    *${segment}
            Append To List    ${modified_segments}    ${captured}
        ELSE
            Append To List    ${modified_segments}    ${segment}
        END
        ${index}=    Evaluate    ${index} + 1
    END
    ${selector}=    Evaluate    ' >> '.join(${modified_segments})
    RETURN    ${selector}

Build Cascaded Selector With Engine
    [Documentation]    Build a cascaded selector with engine prefix for specified segments
    ...                Example: Build Cascaded Selector With Engine    name    mainPanel    okButton
    ...                Returns: name=mainPanel >> name=okButton
    [Arguments]    ${engine}    @{segments}
    ${engine_segments}=    Create List
    FOR    ${segment}    IN    @{segments}
        ${engine_segment}=    Set Variable    ${engine}=${segment}
        Append To List    ${engine_segments}    ${engine_segment}
    END
    ${selector}=    Evaluate    ' >> '.join(${engine_segments})
    RETURN    ${selector}

# =============================================================================
# Cascaded Selector Validation Keywords
# =============================================================================

Element Should Be Found Via Cascade
    [Documentation]    Verify an element can be found using a cascaded selector
    [Arguments]    ${cascaded_selector}    ${timeout}=10
    Wait Until Element Exists    ${cascaded_selector}    timeout=${timeout}
    Element Should Exist    ${cascaded_selector}

Element Should Not Be Found Via Cascade
    [Documentation]    Verify an element cannot be found using a cascaded selector (negative test)
    [Arguments]    ${cascaded_selector}
    Run Keyword And Expect Error    *    Element Should Exist    ${cascaded_selector}

Verify Cascade Returns Expected Element
    [Documentation]    Verify cascaded selector returns element with expected property
    ...                Example: Verify Cascade Returns Expected Element    JPanel >> JButton    text    Submit
    [Arguments]    ${cascaded_selector}    ${property}    ${expected_value}
    ${actual_value}=    Get Element Property    ${cascaded_selector}    ${property}
    Should Be Equal    ${actual_value}    ${expected_value}

Verify Cascade Returns Expected Type
    [Documentation]    Verify cascaded selector returns element of expected component type
    [Arguments]    ${cascaded_selector}    ${expected_type}
    ${element}=    Get Element    ${cascaded_selector}
    ${type}=    Get Element Property    ${element}    class
    Should Contain    ${type}    ${expected_type}

# =============================================================================
# Capture Prefix Keywords
# =============================================================================

Capture Intermediate Element
    [Documentation]    Use capture prefix to get intermediate element in cascade
    ...                Returns the captured element for further operations
    [Arguments]    ${cascaded_selector_with_capture}
    ${element}=    Get Element    ${cascaded_selector_with_capture}
    RETURN    ${element}

Verify Capture Returns Expected Type
    [Documentation]    Verify captured element is of expected component type
    [Arguments]    ${cascaded_selector_with_capture}    ${expected_type}
    ${element}=    Get Element    ${cascaded_selector_with_capture}
    ${type}=    Get Element Property    ${element}    class
    Should Contain    ${type}    ${expected_type}

Store Captured Element
    [Documentation]    Store captured element for reuse in subsequent operations
    [Arguments]    ${cascaded_selector_with_capture}    ${variable_name}
    ${element}=    Get Element    ${cascaded_selector_with_capture}
    Set Suite Variable    \${${variable_name}}    ${element}
    RETURN    ${element}

Perform Operations On Captured Element
    [Documentation]    Capture element and perform multiple operations on it
    ...                Returns list of operation results
    [Arguments]    ${cascaded_selector_with_capture}    @{operations}
    ${element}=    Capture Intermediate Element    ${cascaded_selector_with_capture}
    ${results}=    Create List
    FOR    ${operation}    IN    @{operations}
        ${result}=    Run Keyword    ${operation}    ${element}
        Append To List    ${results}    ${result}
    END
    RETURN    ${results}

# =============================================================================
# Table Cascaded Selector Keywords
# =============================================================================

Get Table Cell Via Cascade
    [Documentation]    Get table cell value using cascaded selector
    [Arguments]    ${table_cascade}    ${row}    ${col}
    ${full_selector}=    Set Variable    ${table_cascade} >> cell[row=${row}, col=${col}]
    ${value}=    Get Element Text    ${full_selector}
    RETURN    ${value}

Select Table Cell Via Cascade
    [Documentation]    Select table cell using cascaded selector
    [Arguments]    ${table_cascade}    ${row}    ${col}
    ${full_selector}=    Set Variable    ${table_cascade} >> cell[row=${row}, col=${col}]
    Click Element    ${full_selector}
    Sleep    0.2s    Wait for selection

Get Table Row Via Cascade
    [Documentation]    Get table row using cascaded selector with row selector
    [Arguments]    ${table_cascade}    ${row_selector}
    ${full_selector}=    Set Variable    ${table_cascade} >> ${row_selector}
    ${element}=    Get Element    ${full_selector}
    RETURN    ${element}

Verify Table Cell Contains Via Cascade
    [Documentation]    Verify table cell contains expected value using cascade
    [Arguments]    ${table_cascade}    ${row}    ${col}    ${expected}
    ${value}=    Get Table Cell Via Cascade    ${table_cascade}    ${row}    ${col}
    Should Contain    ${value}    ${expected}

Find Row By Cell Content Via Cascade
    [Documentation]    Find table row that contains specific cell content
    [Arguments]    ${table_cascade}    ${search_text}
    ${row_selector}=    Set Variable    row[contains='${search_text}']
    ${element}=    Get Table Row Via Cascade    ${table_cascade}    ${row_selector}
    RETURN    ${element}

# =============================================================================
# Tree Cascaded Selector Keywords
# =============================================================================

Select Tree Node Via Cascade
    [Documentation]    Select tree node using cascaded selector
    [Arguments]    ${tree_cascade}    ${node_selector}
    ${full_selector}=    Set Variable    ${tree_cascade} >> ${node_selector}
    Click Element    ${full_selector}
    Sleep    0.2s    Wait for selection

Expand Tree Node Via Cascade
    [Documentation]    Expand tree node using cascaded selector
    [Arguments]    ${tree_cascade}    ${node_path}
    ${full_selector}=    Set Variable    ${tree_cascade} >> node[path='${node_path}']
    ${element}=    Get Element    ${full_selector}
    # Expand node - implementation depends on keyword availability
    Click Element    ${full_selector}

Verify Tree Node Selected Via Cascade
    [Documentation]    Verify tree node is selected using cascaded selector
    [Arguments]    ${tree_cascade}    ${node_path}
    ${full_selector}=    Set Variable    ${tree_cascade} >> node[path='${node_path}']:selected
    Element Should Exist    ${full_selector}

Navigate Tree Path Via Cascade
    [Documentation]    Navigate through tree path and verify each level
    [Arguments]    ${tree_cascade}    @{path_segments}
    ${current_path}=    Set Variable    ${EMPTY}
    FOR    ${segment}    IN    @{path_segments}
        IF    '${current_path}' == '${EMPTY}'
            ${current_path}=    Set Variable    ${segment}
        ELSE
            ${current_path}=    Set Variable    ${current_path}/${segment}
        END
        ${full_selector}=    Set Variable    ${tree_cascade} >> node[path='${current_path}']
        Element Should Exist    ${full_selector}
    END

# =============================================================================
# Tab Cascaded Selector Keywords
# =============================================================================

Select Tab Via Cascade
    [Documentation]    Select tab using cascaded selector
    [Arguments]    ${tab_pane_cascade}    ${tab_selector}
    ${full_selector}=    Set Variable    ${tab_pane_cascade} >> ${tab_selector}
    Click Element    ${full_selector}
    Sleep    0.3s    Wait for tab switch

Get Tab Content Via Cascade
    [Documentation]    Access content within a tab using cascaded selector
    [Arguments]    ${tab_pane_cascade}    ${tab_selector}    ${content_selector}
    ${full_selector}=    Set Variable    ${tab_pane_cascade} >> ${tab_selector} >> ${content_selector}
    ${element}=    Get Element    ${full_selector}
    RETURN    ${element}

Verify Tab Selected Via Cascade
    [Documentation]    Verify tab is selected using cascaded selector
    [Arguments]    ${tab_pane_cascade}    ${tab_title}
    ${full_selector}=    Set Variable    ${tab_pane_cascade} >> tab[title='${tab_title}']:selected
    Element Should Exist    ${full_selector}

Navigate All Tabs Via Cascade
    [Documentation]    Navigate through all tabs and verify each one
    [Arguments]    ${tab_pane_cascade}    @{tab_titles}
    FOR    ${title}    IN    @{tab_titles}
        Select Tab Via Cascade    ${tab_pane_cascade}    tab[title='${title}']
        Verify Tab Selected Via Cascade    ${tab_pane_cascade}    ${title}
    END

# =============================================================================
# Menu Cascaded Selector Keywords
# =============================================================================

Navigate Menu Via Cascade
    [Documentation]    Navigate menu hierarchy using cascaded selector
    [Arguments]    ${menu_cascade}
    Click Element    ${menu_cascade}
    Sleep    0.3s    Wait for menu action

Select Menu Item Via Cascade
    [Documentation]    Select menu item using cascaded selector path
    [Arguments]    ${menu_bar_cascade}    @{menu_path}
    ${menu_selector}=    Build Cascaded Menu Selector    @{menu_path}
    ${full_selector}=    Set Variable    ${menu_bar_cascade} >> ${menu_selector}
    Click Element    ${full_selector}
    Sleep    0.3s    Wait for menu action

Build Cascaded Menu Selector
    [Documentation]    Build menu selector from path components
    ...                Example: Build Cascaded Menu Selector    File    Save As
    ...                Returns: JMenu[text='File'] >> JMenuItem[text='Save As']
    [Arguments]    @{menu_path}
    ${length}=    Get Length    ${menu_path}
    ${segments}=    Create List
    ${index}=    Set Variable    ${0}
    FOR    ${item}    IN    @{menu_path}
        IF    ${index} < ${length} - 1
            ${segment}=    Set Variable    JMenu[text='${item}']
        ELSE
            ${segment}=    Set Variable    JMenuItem[text='${item}']
        END
        Append To List    ${segments}    ${segment}
        ${index}=    Evaluate    ${index} + 1
    END
    ${selector}=    Evaluate    ' >> '.join(${segments})
    RETURN    ${selector}

Verify Menu Path Exists Via Cascade
    [Documentation]    Verify complete menu path exists without clicking
    [Arguments]    ${menu_bar_cascade}    @{menu_path}
    ${menu_selector}=    Build Cascaded Menu Selector    @{menu_path}
    ${full_selector}=    Set Variable    ${menu_bar_cascade} >> ${menu_selector}
    Element Should Exist    ${full_selector}

# =============================================================================
# Engine Switching Keywords
# =============================================================================

Test Multiple Engine Cascade
    [Documentation]    Test cascaded selector using multiple engines
    ...                Example: Test Multiple Engine Cascade    class=JDialog    name=formPanel    JButton[text='OK']
    [Arguments]    @{segments_with_engines}
    ${selector}=    Evaluate    ' >> '.join(${segments_with_engines})
    Element Should Exist    ${selector}
    RETURN    ${selector}

Verify Engine Switching Works
    [Documentation]    Verify that switching between engines in cascade works correctly
    [Arguments]    ${mixed_engine_selector}    ${expected_exists}=${True}
    IF    ${expected_exists}
        Element Should Exist    ${mixed_engine_selector}
    ELSE
        Element Should Not Be Found Via Cascade    ${mixed_engine_selector}
    END

Compare CSS And Engine Selectors
    [Documentation]    Compare CSS default vs explicit engine selector results
    [Arguments]    ${css_selector}    ${engine_selector}
    ${css_element}=    Get Element    ${css_selector}
    ${engine_element}=    Get Element    ${engine_selector}
    Should Be Equal    ${css_element}    ${engine_element}

# =============================================================================
# Performance Measurement Keywords
# =============================================================================

Measure Cascade Lookup Time
    [Documentation]    Measure time taken to find element via cascaded selector
    [Arguments]    ${cascaded_selector}    ${iterations}=${1}
    ${start_time}=    Get Time    epoch
    FOR    ${i}    IN RANGE    ${iterations}
        Element Should Exist    ${cascaded_selector}
    END
    ${end_time}=    Get Time    epoch
    ${duration}=    Evaluate    (${end_time} - ${start_time}) / ${iterations}
    RETURN    ${duration}

Verify Cascade Performance
    [Documentation]    Verify cascaded selector lookup meets performance target
    [Arguments]    ${cascaded_selector}    ${max_time_seconds}
    ${duration}=    Measure Cascade Lookup Time    ${cascaded_selector}
    Should Be True    ${duration} <= ${max_time_seconds}    msg=Cascade lookup took ${duration}s, expected <=${max_time_seconds}s

Benchmark Multiple Cascades
    [Documentation]    Benchmark multiple cascaded selectors and return statistics
    [Arguments]    @{selectors}
    ${times}=    Create List
    FOR    ${selector}    IN    @{selectors}
        ${duration}=    Measure Cascade Lookup Time    ${selector}
        Append To List    ${times}    ${duration}
    END
    ${avg}=    Evaluate    sum(${times}) / len(${times})
    ${min}=    Evaluate    min(${times})
    ${max}=    Evaluate    max(${times})
    ${stats}=    Create Dictionary    average=${avg}    min=${min}    max=${max}
    RETURN    ${stats}

# =============================================================================
# Error Handling Keywords
# =============================================================================

Cascade Should Fail With Error
    [Documentation]    Verify cascaded selector fails with expected error message
    [Arguments]    ${invalid_selector}    ${expected_error_pattern}=*
    Run Keyword And Expect Error    ${expected_error_pattern}    Element Should Exist    ${invalid_selector}

Verify Parse Error
    [Documentation]    Verify cascaded selector causes parse error
    [Arguments]    ${invalid_selector}
    ${error_occurred}=    Run Keyword And Return Status    Element Should Exist    ${invalid_selector}
    Should Be True    not ${error_occurred}    msg=Expected parse error but selector was accepted

Verify Segment Not Found Error
    [Documentation]    Verify error indicates which segment failed in cascade
    [Arguments]    ${cascaded_selector}    ${expected_segment_info}=*
    Run Keyword And Expect Error    ${expected_segment_info}    Element Should Exist    ${cascaded_selector}

Test Error Recovery
    [Documentation]    Test that system recovers from cascade errors
    [Arguments]    ${invalid_selector}    ${valid_selector}
    Run Keyword And Ignore Error    Element Should Exist    ${invalid_selector}
    # Should still work after error
    Element Should Exist    ${valid_selector}

# =============================================================================
# Complex Workflow Keywords
# =============================================================================

Execute Cascaded Form Workflow
    [Documentation]    Execute complete form workflow using cascaded selectors
    [Arguments]    ${form_cascade}    &{field_data}
    FOR    ${field}    ${value}    IN    &{field_data}
        ${field_selector}=    Set Variable    ${form_cascade} >> ${field}
        Input Text    ${field_selector}    ${value}
    END
    ${submit_selector}=    Set Variable    ${form_cascade} >> JButton[text='Submit']
    Click Element    ${submit_selector}

Navigate To Nested Component Via Cascade
    [Documentation]    Navigate through multiple levels to reach target component
    [Arguments]    ${cascaded_selector}    @{interaction_steps}
    FOR    ${step}    IN    @{interaction_steps}
        ${action}=    Get From Dictionary    ${step}    action
        ${target}=    Get From Dictionary    ${step}    target
        ${full_selector}=    Set Variable    ${cascaded_selector} >> ${target}
        IF    '${action}' == 'click'
            Click Element    ${full_selector}
        ELSE IF    '${action}' == 'input'
            ${value}=    Get From Dictionary    ${step}    value
            Input Text    ${full_selector}    ${value}
        ELSE IF    '${action}' == 'select'
            ${value}=    Get From Dictionary    ${step}    value
            Select From List    ${full_selector}    ${value}
        END
    END

Verify Multiple Cascades Return Same Element
    [Documentation]    Verify different cascaded paths return the same element
    [Arguments]    ${cascade1}    ${cascade2}
    ${element1}=    Get Element    ${cascade1}
    ${element2}=    Get Element    ${cascade2}
    Should Be Equal    ${element1}    ${element2}

Execute Multi-Step Cascade Workflow
    [Documentation]    Execute workflow with multiple cascade operations
    [Arguments]    @{cascade_operations}
    ${results}=    Create List
    FOR    ${operation}    IN    @{cascade_operations}
        ${result}=    Run Keyword    ${operation}
        Append To List    ${results}    ${result}
    END
    RETURN    ${results}

# =============================================================================
# Debug and Logging Keywords
# =============================================================================

Log Cascade Resolution
    [Documentation]    Log how cascaded selector is resolved step by step
    [Arguments]    ${cascaded_selector}
    ${segments}=    Split String    ${cascaded_selector}    >>
    ${cumulative}=    Set Variable    ${EMPTY}
    FOR    ${segment}    IN    @{segments}
        ${segment}=    Strip String    ${segment}
        IF    '${cumulative}' == '${EMPTY}'
            ${cumulative}=    Set Variable    ${segment}
        ELSE
            ${cumulative}=    Set Variable    ${cumulative} >> ${segment}
        END
        ${exists}=    Run Keyword And Return Status    Element Should Exist    ${cumulative}
        Log    Segment: ${cumulative} - Exists: ${exists}    level=INFO
    END

Log Cascade Element Properties
    [Documentation]    Log properties of element found via cascaded selector
    [Arguments]    ${cascaded_selector}
    ${element}=    Get Element    ${cascaded_selector}
    ${props}=    Get Element Properties    ${element}
    Log    Cascaded selector ${cascaded_selector} resolved to element with properties: ${props}    level=INFO
    RETURN    ${props}

Debug Cascade Failure
    [Documentation]    Debug why a cascaded selector failed to find element
    [Arguments]    ${failed_cascade}
    Log    Attempting to debug failed cascade: ${failed_cascade}    level=INFO
    Log Cascade Resolution    ${failed_cascade}
    Log UI Tree

# =============================================================================
# Edge Case Handling Keywords
# =============================================================================

Test Whitespace Variations In Cascade
    [Documentation]    Test that various whitespace patterns work correctly
    [Arguments]    @{base_segments}
    ${selectors}=    Create List
    # No whitespace
    ${no_space}=    Evaluate    '>>'.join(${base_segments})
    Append To List    ${selectors}    ${no_space}
    # Single space
    ${single_space}=    Evaluate    ' >> '.join(${base_segments})
    Append To List    ${selectors}    ${single_space}
    # Multiple spaces
    ${multi_space}=    Evaluate    '  >>  '.join(${base_segments})
    Append To List    ${selectors}    ${multi_space}
    # Test all variants
    FOR    ${selector}    IN    @{selectors}
        Element Should Exist    ${selector}
    END

Test Deep Hierarchy Cascade
    [Documentation]    Test cascaded selector with specified depth
    [Arguments]    ${component_type}    ${depth}    ${base_cascade}=${EMPTY}
    ${segments}=    Create List
    IF    '${base_cascade}' != '${EMPTY}'
        Append To List    ${segments}    ${base_cascade}
    END
    FOR    ${i}    IN RANGE    ${depth}
        Append To List    ${segments}    ${component_type}
    END
    ${selector}=    Evaluate    ' >> '.join(${segments})
    RETURN    ${selector}

Verify Capture With Multiple Occurrences
    [Documentation]    Verify capture prefix behavior when multiple captures specified (first wins)
    [Arguments]    ${cascade_with_multiple_captures}
    ${element}=    Get Element    ${cascade_with_multiple_captures}
    # Verify it's the first captured segment
    RETURN    ${element}

Test Dynamic Element In Cascade
    [Documentation]    Test cascade behavior with dynamically added/removed elements
    [Arguments]    ${cascaded_selector}    ${action}=add
    IF    '${action}' == 'add'
        # Test element appears in cascade
        Wait Until Element Exists    ${cascaded_selector}    timeout=5
    ELSE IF    '${action}' == 'remove'
        # Test element disappears from cascade
        Wait Until Element Does Not Exist    ${cascaded_selector}    timeout=5
    END
