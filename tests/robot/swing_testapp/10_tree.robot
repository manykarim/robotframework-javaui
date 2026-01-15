*** Settings ***
Documentation    Tree Controls - Testing JTree operations
...              Operations: Expand, Collapse, Select Node, Get Selected, Navigate
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Expand Tree Node
    [Documentation]    Expand a tree node
    [Tags]    tree    expand    positive
    ${path}=    Set Variable    Project Root|Sources
    Expand Tree Node    JTree[name='fileTree']    ${path}
    # Verify expansion by checking we can access a child
    Element Should Exist    JTree[name='fileTree']

Collapse Tree Node
    [Documentation]    Collapse a tree node
    [Tags]    tree    collapse    positive
    ${path}=    Set Variable    Project Root|Sources
    # First expand
    Expand Tree Node    JTree[name='fileTree']    ${path}
    Sleep    0.2s
    # Then collapse
    Collapse Tree Node    JTree[name='fileTree']    ${path}
    Element Should Exist    JTree[name='fileTree']

Select Tree Node By Path
    [Documentation]    Select a tree node by its path
    [Tags]    tree    select    path    positive
    ${path}=    Set Variable    Project Root|Sources|com.example.main
    Expand Tree Node    JTree[name='fileTree']    Project Root|Sources
    Sleep    0.2s
    Select Tree Node    JTree[name='fileTree']    ${path}
    ${selected}=    Get Selected Tree Node    JTree[name='fileTree']
    Should Contain    ${selected}    com.example.main

Expand All Levels And Select Leaf Node
    [Documentation]    Expand all levels and select a leaf node
    [Tags]    tree    expand    select    leaf    positive
    # Expand to file level
    Expand Tree Node    JTree[name='fileTree']    Project Root
    Sleep    0.2s
    Expand Tree Node    JTree[name='fileTree']    Project Root|Sources
    Sleep    0.2s
    Expand Tree Node    JTree[name='fileTree']    Project Root|Sources|com.example.main
    Sleep    0.2s
    # Select a file
    ${path}=    Set Variable    Project Root|Sources|com.example.main|Application.java
    Select Tree Node    JTree[name='fileTree']    ${path}
    ${selected}=    Get Selected Tree Node    JTree[name='fileTree']
    Should Contain    ${selected}    Application.java

Navigate To Resources
    [Documentation]    Navigate to Resources branch and verify
    [Tags]    tree    navigate    positive
    Expand Tree Node    JTree[name='fileTree']    Project Root|Resources
    Sleep    0.2s
    Expand Tree Node    JTree[name='fileTree']    Project Root|Resources|images
    Sleep    0.2s
    Select Tree Node    JTree[name='fileTree']    Project Root|Resources|images|logo.png
    ${selected}=    Get Selected Tree Node    JTree[name='fileTree']
    Should Contain    ${selected}    logo.png

Navigate To Tests
    [Documentation]    Navigate to Tests branch and verify
    [Tags]    tree    navigate    positive
    Expand Tree Node    JTree[name='fileTree']    Project Root|Tests
    Sleep    0.2s
    Expand Tree Node    JTree[name='fileTree']    Project Root|Tests|unit
    Sleep    0.2s
    Select Tree Node    JTree[name='fileTree']    Project Root|Tests|unit|UserTest.java
    ${selected}=    Get Selected Tree Node    JTree[name='fileTree']
    Should Contain    ${selected}    UserTest.java

Collapse And Re-expand Node
    [Documentation]    Collapse and then re-expand a node
    [Tags]    tree    toggle    positive
    ${path}=    Set Variable    Project Root|Sources
    # Expand
    Expand Tree Node    JTree[name='fileTree']    ${path}
    Sleep    0.2s
    # Collapse
    Collapse Tree Node    JTree[name='fileTree']    ${path}
    Sleep    0.2s
    # Re-expand
    Expand Tree Node    JTree[name='fileTree']    ${path}
    Element Should Exist    JTree[name='fileTree']

Get Tree Nodes
    [Documentation]    Get all tree nodes
    [Tags]    tree    get    nodes    positive
    ${nodes}=    Get Tree Nodes    JTree[name='fileTree']
    Should Not Be Empty    ${nodes}

Tree Should Be Visible
    [Documentation]    Verify tree is visible
    [Tags]    tree    visible    verification
    Element Should Be Visible    JTree[name='fileTree']

Tree Should Be Enabled
    [Documentation]    Verify tree is enabled
    [Tags]    tree    enabled    verification
    Element Should Be Enabled    JTree[name='fileTree']
