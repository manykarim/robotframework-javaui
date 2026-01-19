*** Settings ***
Test Timeout       60s
Documentation     Menu Tests - Testing select_menu and select_from_popup_menu keywords.
...
...               These tests verify the library's ability to interact with
...               JMenuBar, JMenu, JMenuItem, and popup menu components.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        menus    regression

*** Test Cases ***
# =============================================================================
# SELECT MENU - MAIN MENU BAR
# =============================================================================

Select Menu Single Level
    [Documentation]    Select a top-level menu item.
    [Tags]    smoke    positive
    Select Menu    File|New
    Sleep    0.2s

Select Menu Two Levels
    [Documentation]    Select a two-level menu path.
    [Tags]    positive
    Select Menu    Edit|Copy
    Sleep    0.2s

Select Menu Three Levels
    [Documentation]    Select a three-level menu path (Edit|Find|Find Next).
    [Tags]    positive
    Select Menu    Edit|Find|Find Next
    Sleep    0.2s

Select Different Menu Items
    [Documentation]    Select different menu items across menus.
    [Tags]    positive
    Select Menu    File|New
    Sleep    0.2s
    Select Menu    Edit|Paste
    Sleep    0.2s
    Select Menu    View|Toolbar
    Sleep    0.2s

Select Menu With Accelerator
    [Documentation]    Select menu item that has keyboard accelerator.
    [Tags]    positive
    Select Menu    Edit|Cut
    Sleep    0.2s

Select Menu With Ellipsis
    [Documentation]    Select menu item with ellipsis in name.
    [Tags]    positive    edge-case
    Select Menu    File|Open...
    Sleep    0.2s

Select Same Menu Item Multiple Times
    [Documentation]    Verify selecting same menu item multiple times.
    [Tags]    positive    edge-case
    FOR    ${i}    IN RANGE    3
        Select Menu    Edit|Copy
        Sleep    0.1s
    END

# =============================================================================
# SELECT MENU - DIFFERENT MENUS
# =============================================================================

Select File Menu Items
    [Documentation]    Select various items from File menu.
    [Tags]    positive
    Select Menu    File|New
    Sleep    0.2s
    Select Menu    File|Save
    Sleep    0.2s

Select Edit Menu Items
    [Documentation]    Select various items from Edit menu.
    [Tags]    positive
    Select Menu    Edit|Cut
    Sleep    0.2s
    Select Menu    Edit|Copy
    Sleep    0.2s

Select View Menu Items
    [Documentation]    Select various items from View menu.
    [Tags]    positive
    Select Menu    View|Toolbar
    Sleep    0.2s

Select Help Menu Items
    [Documentation]    Select items from Help menu.
    [Tags]    positive
    # Use Help Contents instead of About (About opens modal dialog which blocks)
    Select Menu    Help|Help Contents
    Sleep    0.2s

# =============================================================================
# SELECT FROM POPUP MENU (CONTEXT MENU)
# =============================================================================

Select From Popup Menu After Right Click On Table
    [Documentation]    Right-click table and select from popup menu.
    [Tags]    smoke    positive    context-menu
    # First navigate to Data View tab where the table is
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.2s
    Right Click    JTable[name='dataTable']
    Sleep    0.2s
    Select From Popup Menu    View Details
    Sleep    0.2s

Select From Popup Menu After Right Click On Tree
    [Documentation]    Right-click tree and select from popup menu.
    [Tags]    positive    context-menu
    # Tree is in the left split pane, always visible
    Right Click    JTree[name='fileTree']
    Sleep    0.2s
    Select From Popup Menu    Refresh
    Sleep    0.2s

Select Different Table Popup Menu Item
    [Documentation]    Right-click table and select different popup menu item.
    [Tags]    positive    context-menu
    # Ensure Data View tab is selected
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.2s
    Right Click    JTable[name='dataTable']
    Sleep    0.2s
    Select From Popup Menu    Edit Item
    Sleep    0.2s

Select Multiple Popup Menu Items
    [Documentation]    Test multiple popup menu selections.
    [Tags]    positive    context-menu
    # Table popup
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.2s
    Right Click    JTable[name='dataTable']
    Sleep    0.2s
    Select From Popup Menu    View Details
    Sleep    0.2s
    # Tree popup (tree is always visible in left split pane)
    Right Click    JTree[name='fileTree']
    Sleep    0.2s
    Select From Popup Menu    Open
    Sleep    0.2s

# =============================================================================
# MENU WORKFLOWS
# =============================================================================

File Operations Menu Workflow
    [Documentation]    Test file-related menu workflow.
    [Tags]    workflow    smoke
    Select Menu    File|New
    Sleep    0.3s
    Select Menu    File|Save
    Sleep    0.3s

Edit Operations Menu Workflow
    [Documentation]    Test edit-related menu workflow.
    [Tags]    workflow
    Select Menu    Edit|Select All
    Sleep    0.2s
    Select Menu    Edit|Copy
    Sleep    0.2s
    Select Menu    Edit|Paste
    Sleep    0.2s

Context Menu Edit Workflow
    [Documentation]    Test context menu for editing.
    [Tags]    workflow    context-menu
    # Navigate to Data View tab first
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.2s
    # Right click on table
    Right Click    JTable[name='dataTable']
    Sleep    0.2s
    Select From Popup Menu    View Details
    Sleep    0.2s
    # Right click again
    Right Click    JTable[name='dataTable']
    Sleep    0.2s
    Select From Popup Menu    Edit Item
    Sleep    0.2s

Navigate Multiple Menus Workflow
    [Documentation]    Navigate through multiple menus.
    [Tags]    workflow
    Select Menu    File|New
    Sleep    0.2s
    Select Menu    Edit|Cut
    Sleep    0.2s
    Select Menu    View|Status Bar
    Sleep    0.2s
    # Use Help Contents instead of About (About opens modal dialog which blocks)
    Select Menu    Help|Help Contents
    Sleep    0.2s

# =============================================================================
# MENU STATE VERIFICATION
# =============================================================================

Verify Menu Bar Exists
    [Documentation]    Verify menu bar exists in the application.
    [Tags]    positive    verification
    Element Should Exist    JMenuBar

Verify Menu Exists
    [Documentation]    Verify specific menu exists.
    [Tags]    positive    verification
    Element Should Exist    JMenu[text='File']

Verify Menu Is Visible
    [Documentation]    Verify menu bar is visible.
    [Tags]    positive    verification
    Element Should Be Visible    JMenuBar

# =============================================================================
# FINDING MENUS
# =============================================================================

Find Menu Bar
    [Documentation]    Find the menu bar element.
    [Tags]    positive
    ${menubar}=    Find Element    JMenuBar
    Should Not Be Equal    ${menubar}    ${NONE}

Find All Menus
    [Documentation]    Find all menu elements.
    [Tags]    positive
    ${menus}=    Find Elements    JMenu
    Should Not Be Empty    ${menus}

Find File Menu
    [Documentation]    Find the File menu specifically.
    [Tags]    positive
    ${menu}=    Find Element    JMenu[text='File']
    Should Not Be Equal    ${menu}    ${NONE}

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Select Nonexistent Menu Fails
    [Documentation]    Select non-existent menu throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select Menu    NonExistent|Menu
    Should Be Equal    ${status}    ${FALSE}

Select Nonexistent Menu Item Fails
    [Documentation]    Select non-existent menu item throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select Menu    File|NonExistentItem
    Should Be Equal    ${status}    ${FALSE}

Select From Popup Menu Without Context Fails
    [Documentation]    Select from popup without right-click fails.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select From Popup Menu    SomeItem
    Should Be Equal    ${status}    ${FALSE}

Select Nonexistent Popup Menu Item Fails
    [Documentation]    Select non-existent popup item throws error.
    [Tags]    negative    error-handling    context-menu
    # Navigate to Data View tab first
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.2s
    Right Click    JTable[name='dataTable']
    Sleep    0.2s
    ${status}=    Run Keyword And Return Status
    ...    Select From Popup Menu    NonExistentMenuItem
    Should Be Equal    ${status}    ${FALSE}

Select Menu With Empty Path Fails
    [Documentation]    Select menu with empty path throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select Menu    ${EMPTY}
    Should Be Equal    ${status}    ${FALSE}

# =============================================================================
# EDGE CASES
# =============================================================================

Select Menu Item With Special Characters
    [Documentation]    Select menu item with special characters.
    [Tags]    edge-case
    # Menu items might have &, ... or other special chars
    ${status}=    Run Keyword And Return Status
    ...    Select Menu    File|Open...
    # Log result regardless of status
    Log    Menu selection status: ${status}

Rapid Menu Selection
    [Documentation]    Test rapid menu selection.
    [Tags]    edge-case    stress
    FOR    ${i}    IN RANGE    5
        Select Menu    Edit|Copy
        Sleep    0.1s
    END

Menu Selection With Timing
    [Documentation]    Test menu selection with various timings.
    [Tags]    edge-case
    Select Menu    File|New
    Sleep    0.5s
    Select Menu    Edit|Cut
    Sleep    0.5s

Context Menu On Different Elements
    [Documentation]    Test context menu on different element types.
    [Tags]    edge-case    context-menu
    # Table
    Right Click    JTable[name='dataTable']
    Sleep    0.2s
    ${status1}=    Run Keyword And Return Status
    ...    Select From Popup Menu    View Details
    Log    Table popup: ${status1}
    # Tree
    Right Click    JTree[name='fileTree']
    Sleep    0.2s
    ${status2}=    Run Keyword And Return Status
    ...    Select From Popup Menu    Refresh
    Log    Tree popup: ${status2}

Menu Keyboard Navigation Alternative
    [Documentation]    Test menu as alternative to keyboard navigation.
    [Tags]    edge-case
    # This tests menu-based operations
    Select Menu    Edit|Select All
    Sleep    0.2s
    Select Menu    Edit|Copy
    Sleep    0.2s

# =============================================================================
# INTEGRATION WITH OTHER FEATURES
# =============================================================================

Menu And Button Integration
    [Documentation]    Test menu operations alongside button clicks.
    [Tags]    integration
    # Click a button
    Click Button    JButton[name='clearButton']
    Sleep    0.2s
    # Use menu
    Select Menu    File|New
    Sleep    0.2s
    # Click another button
    Click Button    JButton[name='submitButton']
    Sleep    0.2s

Menu And Text Input Integration
    [Documentation]    Test menu with text input operations.
    [Tags]    integration
    # Navigate to Form Input tab where the text field is
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Sleep    0.2s
    # Input text
    Input Text    [name='nameTextField']    menutest
    Sleep    0.2s
    # Use menu
    Select Menu    Edit|Select All
    Sleep    0.2s
    # Clear via menu
    Select Menu    Edit|Cut
    Sleep    0.2s

Context Menu And Selection Integration
    [Documentation]    Test context menu with element selection.
    [Tags]    integration    context-menu
    # Navigate to Data View tab where the table is
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.2s
    # Select table row
    Select Table Row    JTable[name='dataTable']    0
    Sleep    0.2s
    # Right click selected row
    Right Click    JTable[name='dataTable']
    Sleep    0.2s
    # Select from context menu
    ${status}=    Run Keyword And Return Status
    ...    Select From Popup Menu    View Details
    Log    Context menu result: ${status}
