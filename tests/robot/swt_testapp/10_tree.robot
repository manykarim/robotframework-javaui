*** Settings ***
Documentation    SWT Tree Controls - Testing Tree widget
...              Operations: Expand, Collapse, Select Node, Get Selected, Navigate
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Expand Tree Node
    [Documentation]    Expand a tree node
    [Tags]    tree    expand    positive
    ${path}=    Set Variable    Project A|src
    Expand Tree Node    Tree[name='fileTree']    ${path}
    Tree Node Should Be Expanded    Tree[name='fileTree']    ${path}

Collapse Tree Node
    [Documentation]    Collapse a tree node
    [Tags]    tree    collapse    positive
    ${path}=    Set Variable    Project A|src
    Expand Tree Node    Tree[name='fileTree']    ${path}
    Sleep    0.2s
    Collapse Tree Node    Tree[name='fileTree']    ${path}
    Tree Node Should Be Collapsed    Tree[name='fileTree']    ${path}

Select Tree Node By Path
    [Documentation]    Select a tree node by its path
    [Tags]    tree    select    path    positive
    ${path}=    Set Variable    Project A|src|main
    Expand Tree Node    Tree[name='fileTree']    Project A|src
    Sleep    0.2s
    Select Tree Node    Tree[name='fileTree']    ${path}
    ${selected}=    Get Tree Selected Node    Tree[name='fileTree']
    Should Contain    ${selected}    main

Expand All Levels And Select Leaf Node
    [Documentation]    Expand all levels and select a leaf node
    [Tags]    tree    expand    select    leaf    positive
    Expand Tree Node    Tree[name='fileTree']    Project A
    Sleep    0.2s
    Expand Tree Node    Tree[name='fileTree']    Project A|src
    Sleep    0.2s
    Expand Tree Node    Tree[name='fileTree']    Project A|src|main
    Sleep    0.2s
    ${path}=    Set Variable    Project A|src|main|App.java
    Select Tree Node    Tree[name='fileTree']    ${path}
    ${selected}=    Get Tree Selected Node    Tree[name='fileTree']
    Should Contain    ${selected}    App.java

Navigate To Different Projects
    [Documentation]    Navigate to different project branches
    [Tags]    tree    navigate    positive
    Expand Tree Node    Tree[name='fileTree']    Project B
    Sleep    0.2s
    Select Tree Node    Tree[name='fileTree']    Project B|src
    ${selected}=    Get Tree Selected Node    Tree[name='fileTree']
    Should Contain    ${selected}    src

Collapse And Re-expand Node
    [Documentation]    Collapse and then re-expand a node
    [Tags]    tree    toggle    positive
    ${path}=    Set Variable    Project A|src
    Expand Tree Node    Tree[name='fileTree']    ${path}
    Tree Node Should Be Expanded    Tree[name='fileTree']    ${path}
    Collapse Tree Node    Tree[name='fileTree']    ${path}
    Tree Node Should Be Collapsed    Tree[name='fileTree']    ${path}
    Expand Tree Node    Tree[name='fileTree']    ${path}
    Tree Node Should Be Expanded    Tree[name='fileTree']    ${path}

Tree Should Be Visible
    [Documentation]    Verify tree is visible
    [Tags]    tree    visible    verification
    Element Should Be Visible    Tree[name='fileTree']

Tree Should Be Enabled
    [Documentation]    Verify tree is enabled
    [Tags]    tree    enabled    verification
    Element Should Be Enabled    Tree[name='fileTree']
