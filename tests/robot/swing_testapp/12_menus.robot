*** Settings ***
Documentation    Menu Controls - Testing JMenuBar, JMenu, JMenuItem operations
...              Operations: Select Menu Item, Click Menu, Navigate Submenus
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Select Menu Item From File Menu
    [Documentation]    Select a menu item from the File menu
    [Tags]    menu    select    positive
    Select Menu    File|New
    Sleep    0.3s
    # Menu item was clicked - just verify no exception occurred
    Element Should Exist    JTextField[name='nameTextField']

Select Menu Item From Edit Menu
    [Documentation]    Select menu items from the Edit menu
    [Tags]    menu    select    positive
    Select Menu    Edit|Cut
    Sleep    0.2s
    Select Menu    Edit|Copy
    Sleep    0.2s
    Select Menu    Edit|Paste
    Sleep    0.2s
    Select Menu    Edit|Select All
    Sleep    0.2s
    Element Should Exist    JTextField[name='nameTextField']

Select Submenu Item
    [Documentation]    Select an item from a submenu
    [Tags]    menu    submenu    select    positive
    Select Menu    Edit|Find|Find...
    Sleep    0.3s
    Element Should Exist    JTextField[name='nameTextField']

Toggle View Menu Checkbox Items
    [Documentation]    Toggle checkbox menu items in View menu
    [Tags]    menu    checkbox    toggle    positive
    # Toggle Toolbar visibility
    Select Menu    View|Toolbar
    Sleep    0.2s
    # Toggle Status Bar visibility
    Select Menu    View|Status Bar
    Sleep    0.2s
    Element Should Exist    JTextField[name='nameTextField']

Select View Menu Radio Items
    [Documentation]    Select radio button menu items in View menu
    [Tags]    menu    radio    select    positive
    Select Menu    View|Normal View
    Sleep    0.2s
    Select Menu    View|Compact View
    Sleep    0.2s
    Select Menu    View|Normal View
    Sleep    0.2s
    Element Should Exist    JTextField[name='nameTextField']

Select Help Menu Items
    [Documentation]    Select items from the Help menu
    [Tags]    menu    select    positive
    Select Menu    Help|Help Contents
    Sleep    0.2s
    Element Should Exist    JTextField[name='nameTextField']

Open About Dialog Via Menu
    [Documentation]    Open the About dialog via Help menu
    [Tags]    menu    dialog    positive
    Select Menu    Help|About
    Sleep    0.5s
    # About dialog should be visible
    Wait Until Element Exists    JDialog[name='aboutDialog']    timeout=5
    Element Should Be Visible    JDialog[name='aboutDialog']
    # Close the dialog
    Click Element    JButton[name='aboutCloseButton']
    Sleep    0.3s

Navigate Through All Top Level Menus
    [Documentation]    Navigate through all top-level menus
    [Tags]    menu    navigate    positive
    # File menu
    Select Menu    File|Save
    Sleep    0.2s
    # Edit menu
    Select Menu    Edit|Paste
    Sleep    0.2s
    # View menu
    Select Menu    View|Toolbar
    Sleep    0.2s
    # Help menu
    Select Menu    Help|Help Contents
    Sleep    0.2s
    Element Should Exist    JTextField[name='nameTextField']

Select Find Submenu Items
    [Documentation]    Select all items in Find submenu
    [Tags]    menu    submenu    positive
    Select Menu    Edit|Find|Find...
    Sleep    0.2s
    Select Menu    Edit|Find|Find Next
    Sleep    0.2s
    Select Menu    Edit|Find|Replace...
    Sleep    0.2s
    Element Should Exist    JTextField[name='nameTextField']
