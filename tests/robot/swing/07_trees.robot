*** Settings ***
Documentation     Tree Tests - Testing expand_tree_node, collapse_tree_node,
...               select_tree_node, get_selected_tree_node, and get_tree_nodes keywords.
...
...               These tests verify the library's ability to interact with
...               JTree components for hierarchical navigation.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        trees    regression

*** Test Cases ***
# =============================================================================
# EXPAND TREE NODE
# =============================================================================

Expand Root Tree Node
    [Documentation]    Expand the root node of a tree.
    [Tags]    smoke    positive
    Expand Tree Node    JTree[name='fileTree']    Project Root
    # Verify child nodes are now visible (tree expanded)
    ${nodes}=    Get Tree Nodes    JTree[name='fileTree']
    Should Not Be Empty    ${nodes}    Tree should have visible nodes after expansion

Expand Tree Node By Path
    [Documentation]    Expand a tree node using path notation.
    [Tags]    positive
    Expand Tree Node    JTree[name='fileTree']    Project Root/Sources
    Element Should Exist    JTree[name='fileTree']

Expand Nested Tree Node
    [Documentation]    Expand a deeply nested tree node.
    [Tags]    positive
    Expand Tree Node    JTree[name='fileTree']    Project Root/Sources/com.example.main
    Element Should Exist    JTree[name='fileTree']

Expand Tree Node Using ID Selector
    [Documentation]    Expand tree node using ID-style selector.
    [Tags]    positive
    Expand Tree Node    \#fileTree    Project Root
    Element Should Exist    \#fileTree

Expand Tree Node Using XPath
    [Documentation]    Expand tree node using XPath selector.
    [Tags]    positive    xpath-locator
    Expand Tree Node    //JTree[@name='fileTree']    Project Root
    Element Should Exist    //JTree[@name='fileTree']

Expand Already Expanded Node
    [Documentation]    Verify expanding already expanded node is safe.
    [Tags]    positive    edge-case
    Expand Tree Node    [name='fileTree']    Project Root
    Expand Tree Node    [name='fileTree']    Project Root
    Element Should Exist    [name='fileTree']

Expand Multiple Nodes
    [Documentation]    Expand multiple nodes sequentially.
    [Tags]    positive
    Expand Tree Node    [name='fileTree']    Project Root
    Expand Tree Node    [name='fileTree']    Project Root/Sources
    Expand Tree Node    [name='fileTree']    Project Root/Resources
    Element Should Exist    [name='fileTree']

# =============================================================================
# COLLAPSE TREE NODE
# =============================================================================

Collapse Root Tree Node
    [Documentation]    Collapse the root node of a tree.
    [Tags]    smoke    positive
    Expand Tree Node    JTree[name='fileTree']    Project Root
    Collapse Tree Node    JTree[name='fileTree']    Project Root
    Element Should Exist    JTree[name='fileTree']

Collapse Tree Node By Path
    [Documentation]    Collapse a tree node using path notation.
    [Tags]    positive
    Expand Tree Node    [name='fileTree']    Project Root/Sources
    Collapse Tree Node    [name='fileTree']    Project Root/Sources
    Element Should Exist    [name='fileTree']

Collapse Nested Tree Node
    [Documentation]    Collapse a deeply nested tree node.
    [Tags]    positive
    Expand Tree Node    [name='fileTree']    Project Root/Sources/com.example.main
    Collapse Tree Node    [name='fileTree']    Project Root/Sources/com.example.main
    Element Should Exist    [name='fileTree']

Collapse Tree Node Using ID Selector
    [Documentation]    Collapse tree node using ID-style selector.
    [Tags]    positive
    Expand Tree Node    \#fileTree    Project Root
    Collapse Tree Node    \#fileTree    Project Root
    Element Should Exist    \#fileTree

Collapse Tree Node Using XPath
    [Documentation]    Collapse tree node using XPath selector.
    [Tags]    positive    xpath-locator
    Expand Tree Node    //JTree[@name='fileTree']    Project Root
    Collapse Tree Node    //JTree[@name='fileTree']    Project Root
    Element Should Exist    //JTree[@name='fileTree']

Collapse Already Collapsed Node
    [Documentation]    Verify collapsing already collapsed node is safe.
    [Tags]    positive    edge-case
    Collapse Tree Node    [name='fileTree']    Project Root
    Collapse Tree Node    [name='fileTree']    Project Root
    Element Should Exist    [name='fileTree']

# =============================================================================
# SELECT TREE NODE
# =============================================================================

Select Root Tree Node
    [Documentation]    Select the root node of a tree.
    [Tags]    smoke    positive
    Select Tree Node    JTree[name='fileTree']    Project Root
    Element Should Exist    JTree[name='fileTree']

Select Tree Node By Path
    [Documentation]    Select a tree node using path notation.
    [Tags]    positive
    Expand Tree Node    [name='fileTree']    Project Root
    Select Tree Node    JTree[name='fileTree']    Project Root/Sources
    Element Should Exist    JTree[name='fileTree']

Select Nested Tree Node
    [Documentation]    Select a deeply nested tree node.
    [Tags]    positive
    Expand Tree Node    [name='fileTree']    Project Root/Sources
    Select Tree Node    JTree[name='fileTree']    Project Root/Sources/com.example.main
    Element Should Exist    JTree[name='fileTree']

Select Tree Node Using ID Selector
    [Documentation]    Select tree node using ID-style selector.
    [Tags]    positive
    Select Tree Node    \#fileTree    Project Root
    Element Should Exist    \#fileTree

Select Tree Node Using XPath
    [Documentation]    Select tree node using XPath selector.
    [Tags]    positive    xpath-locator
    Select Tree Node    //JTree[@name='fileTree']    Project Root
    Element Should Exist    //JTree[@name='fileTree']

Select Same Node Multiple Times
    [Documentation]    Verify selecting same node multiple times is safe.
    [Tags]    positive    edge-case
    Select Tree Node    [name='fileTree']    Project Root
    Select Tree Node    [name='fileTree']    Project Root
    Select Tree Node    [name='fileTree']    Project Root
    Element Should Exist    [name='fileTree']

Select Different Nodes Sequentially
    [Documentation]    Select different nodes one after another.
    [Tags]    positive
    Expand Tree Node    [name='fileTree']    Project Root
    Select Tree Node    [name='fileTree']    Project Root/Sources
    Select Tree Node    [name='fileTree']    Project Root/Resources
    Select Tree Node    [name='fileTree']    Project Root/Tests
    Element Should Exist    [name='fileTree']

# =============================================================================
# GET SELECTED TREE NODE
# =============================================================================

Get Selected Tree Node After Selection
    [Documentation]    Get the currently selected tree node.
    [Tags]    smoke    positive
    Select Tree Node    JTree[name='fileTree']    Project Root
    ${selected}=    Get Selected Tree Node    JTree[name='fileTree']
    Should Not Be Empty    ${selected}
    Log    Selected node: ${selected}

Get Selected Tree Node Using ID Selector
    [Documentation]    Get selected node using ID-style selector.
    [Tags]    positive
    Select Tree Node    \#fileTree    Project Root
    ${selected}=    Get Selected Tree Node    \#fileTree
    Log    Selected: ${selected}

Get Selected Tree Node Using XPath
    [Documentation]    Get selected node using XPath selector.
    [Tags]    positive    xpath-locator
    Select Tree Node    //JTree[@name='fileTree']    Project Root
    ${selected}=    Get Selected Tree Node    //JTree[@name='fileTree']
    Log    Selected: ${selected}

Get Selected Node After Path Selection
    [Documentation]    Verify selected node after selecting nested path.
    [Tags]    positive
    Expand Tree Node    [name='fileTree']    Project Root
    Select Tree Node    [name='fileTree']    Project Root/Sources
    ${selected}=    Get Selected Tree Node    [name='fileTree']
    Log    Selected after path: ${selected}

# =============================================================================
# GET TREE NODES
# =============================================================================

Get All Tree Nodes
    [Documentation]    Get all nodes from a tree.
    [Tags]    smoke    positive
    ${nodes}=    Get Tree Nodes    JTree[name='fileTree']
    Should Not Be Empty    ${nodes}
    Log    Tree nodes: ${nodes}

Get Tree Nodes Using ID Selector
    [Documentation]    Get tree nodes using ID-style selector.
    [Tags]    positive
    ${nodes}=    Get Tree Nodes    \#fileTree
    Should Not Be Empty    ${nodes}

Get Tree Nodes Using XPath
    [Documentation]    Get tree nodes using XPath selector.
    [Tags]    positive    xpath-locator
    ${nodes}=    Get Tree Nodes    //JTree[@name='fileTree']
    Should Not Be Empty    ${nodes}

Verify Tree Contains Expected Nodes
    [Documentation]    Verify the tree contains expected nodes.
    [Tags]    positive
    ${nodes}=    Get Tree Nodes    [name='fileTree']
    Should Contain    ${nodes}    Project Root
    Log    Nodes found: ${nodes}

# =============================================================================
# TREE WORKFLOWS
# =============================================================================

Navigate Tree Hierarchy Workflow
    [Documentation]    Navigate through tree hierarchy.
    [Tags]    workflow    smoke
    # Expand root
    Expand Tree Node    [name='fileTree']    Project Root
    # Navigate to Documents
    Expand Tree Node    [name='fileTree']    Project Root/Sources
    # Select a document
    Select Tree Node    [name='fileTree']    Project Root/Sources/com.example.main
    # Verify selection
    ${selected}=    Get Selected Tree Node    [name='fileTree']
    Log    Final selection: ${selected}

Expand Collapse Cycle Workflow
    [Documentation]    Test expand/collapse cycle.
    [Tags]    workflow
    # Expand all
    Expand Tree Node    [name='fileTree']    Project Root
    Expand Tree Node    [name='fileTree']    Project Root/Sources
    # Collapse in reverse
    Collapse Tree Node    [name='fileTree']    Project Root/Sources
    Collapse Tree Node    [name='fileTree']    Project Root
    # Expand again
    Expand Tree Node    [name='fileTree']    Project Root
    Element Should Exist    [name='fileTree']

Browse File Tree Workflow
    [Documentation]    Simulate browsing a file tree.
    [Tags]    workflow
    # Start at root
    Select Tree Node    [name='fileTree']    Project Root
    # Browse to different folders
    Expand Tree Node    [name='fileTree']    Project Root
    Select Tree Node    [name='fileTree']    Project Root/Sources
    Expand Tree Node    [name='fileTree']    Project Root/Sources
    Select Tree Node    [name='fileTree']    Project Root/Resources
    ${selected}=    Get Selected Tree Node    [name='fileTree']
    Log    Browsed to: ${selected}

# =============================================================================
# TREE STATE VERIFICATION
# =============================================================================

Verify Tree Is Enabled
    [Documentation]    Verify tree is enabled before interaction.
    [Tags]    positive    verification
    Element Should Be Enabled    JTree[name='fileTree']

Verify Tree Is Visible
    [Documentation]    Verify tree is visible.
    [Tags]    positive    verification
    Element Should Be Visible    JTree[name='fileTree']

Verify Tree Exists
    [Documentation]    Verify tree exists in the UI.
    [Tags]    positive    verification
    Element Should Exist    JTree[name='fileTree']

# =============================================================================
# FINDING TREES
# =============================================================================

Find All Trees
    [Documentation]    Find all tree elements in the application.
    [Tags]    positive
    ${trees}=    Find Elements    JTree
    Should Not Be Empty    ${trees}

Find Enabled Trees
    [Documentation]    Find all enabled trees.
    [Tags]    positive
    ${trees}=    Find Elements    JTree:enabled
    Should Not Be Empty    ${trees}

Find Visible Trees
    [Documentation]    Find all visible trees.
    [Tags]    positive
    ${trees}=    Find Elements    JTree:visible
    Should Not Be Empty    ${trees}

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Expand Node In Nonexistent Tree Fails
    [Documentation]    Expand node in non-existent tree throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Expand Tree Node    JTree[name='nonexistent']    Project Root
    Should Be Equal    ${status}    ${FALSE}

Collapse Node In Nonexistent Tree Fails
    [Documentation]    Collapse node in non-existent tree throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Collapse Tree Node    JTree[name='nonexistent']    Project Root
    Should Be Equal    ${status}    ${FALSE}

Select Node In Nonexistent Tree Fails
    [Documentation]    Select node in non-existent tree throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select Tree Node    JTree[name='nonexistent']    Project Root
    Should Be Equal    ${status}    ${FALSE}

Get Selected Node From Nonexistent Tree Fails
    [Documentation]    Get selected node from non-existent tree throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Selected Tree Node    JTree[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Get Nodes From Nonexistent Tree Fails
    [Documentation]    Get nodes from non-existent tree throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Tree Nodes    JTree[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Expand Nonexistent Path Fails
    [Documentation]    Expand non-existent path throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Expand Tree Node    [name='fileTree']    NonExistent/Path/Here
    Should Be Equal    ${status}    ${FALSE}

Select Nonexistent Path Fails
    [Documentation]    Select non-existent path throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select Tree Node    [name='fileTree']    NonExistent/Path/Here
    Should Be Equal    ${status}    ${FALSE}

# =============================================================================
# EDGE CASES
# =============================================================================

Expand Root Only
    [Documentation]    Expand only the root node.
    [Tags]    edge-case
    Expand Tree Node    [name='fileTree']    Project Root
    Element Should Exist    [name='fileTree']

Select Root Only
    [Documentation]    Select only the root node.
    [Tags]    edge-case
    Select Tree Node    [name='fileTree']    Project Root
    ${selected}=    Get Selected Tree Node    [name='fileTree']
    Log    Root selected: ${selected}

Rapid Tree Navigation
    [Documentation]    Test rapid tree navigation.
    [Tags]    edge-case    stress
    FOR    ${i}    IN RANGE    5
        Expand Tree Node    [name='fileTree']    Project Root
        Select Tree Node    [name='fileTree']    Project Root
        Collapse Tree Node    [name='fileTree']    Project Root
    END
    Element Should Exist    [name='fileTree']

Rapid Node Selection
    [Documentation]    Test rapid node selection.
    [Tags]    edge-case    stress
    Expand Tree Node    [name='fileTree']    Project Root
    FOR    ${i}    IN RANGE    5
        Select Tree Node    [name='fileTree']    Project Root/Sources
        Select Tree Node    [name='fileTree']    Project Root/Resources
    END
    Element Should Exist    [name='fileTree']

Double Click On Tree
    [Documentation]    Double-click on a tree node.
    [Tags]    edge-case
    Double Click    JTree[name='fileTree']
    Element Should Exist    JTree[name='fileTree']

Right Click On Tree
    [Documentation]    Right-click on a tree (context menu).
    [Tags]    edge-case    context-menu
    Right Click    JTree[name='fileTree']
    Element Should Exist    JTree[name='fileTree']
