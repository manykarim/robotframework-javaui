*** Settings ***
Documentation    SWT Menu Controls - Testing Menu, MenuItem operations
...              Operations: Select Menu Item, Click Menu, Navigate Submenus
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Select Menu Item From File Menu
    [Documentation]    Select a menu item from the File menu
    [Tags]    menu    select    positive
    Select Menu Item    File|New
    Sleep    0.3s
    Element Should Exist    Shell[name='mainShell']

Select Menu Item From Edit Menu
    [Documentation]    Select menu items from the Edit menu
    [Tags]    menu    select    positive
    Select Menu Item    Edit|Cut
    Sleep    0.2s
    Select Menu Item    Edit|Copy
    Sleep    0.2s
    Select Menu Item    Edit|Paste
    Sleep    0.2s
    Select Menu Item    Edit|Select All
    Sleep    0.2s
    Element Should Exist    Shell[name='mainShell']

Select Submenu Item
    [Documentation]    Select an item from a submenu (Recent Files)
    [Tags]    menu    submenu    select    positive
    Select Menu Item    File|Recent Files|File 1.txt
    Sleep    0.3s
    Element Should Exist    Shell[name='mainShell']

Toggle View Menu Checkbox Items
    [Documentation]    Toggle checkbox menu items in View menu
    [Tags]    menu    checkbox    toggle    positive
    Select Menu Item    View|Show Toolbar
    Sleep    0.2s
    Select Menu Item    View|Show Status Bar
    Sleep    0.2s
    Element Should Exist    Shell[name='mainShell']

Select View Menu Radio Items
    [Documentation]    Select radio button menu items in View menu
    [Tags]    menu    radio    select    positive
    Select Menu Item    View|List View
    Sleep    0.2s
    Select Menu Item    View|Detail View
    Sleep    0.2s
    Select Menu Item    View|Icon View
    Sleep    0.2s
    Element Should Exist    Shell[name='mainShell']

Select Help Menu Items
    [Documentation]    Select items from the Help menu
    [Tags]    menu    select    positive
    Select Menu Item    Help|Help Contents
    Sleep    0.2s
    Element Should Exist    Shell[name='mainShell']

Open About Dialog Via Menu
    [Documentation]    Open the About dialog via Help menu
    [Tags]    menu    dialog    positive
    Select Menu Item    Help|About
    Sleep    0.5s
    Wait Until Element Exists    Shell[name='aboutDialog']    timeout=5
    Element Should Be Visible    Shell[name='aboutDialog']
    # Close the dialog
    Click Element    Button[name='aboutOkButton']
    Sleep    0.3s

Navigate Through All Top Level Menus
    [Documentation]    Navigate through all top-level menus
    [Tags]    menu    navigate    positive
    Select Menu Item    File|Save
    Sleep    0.2s
    Select Menu Item    Edit|Paste
    Sleep    0.2s
    Select Menu Item    View|Show Toolbar
    Sleep    0.2s
    Select Menu Item    Help|Help Contents
    Sleep    0.2s
    Element Should Exist    Shell[name='mainShell']
