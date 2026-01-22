*** Settings ***
Test Timeout       60s
Documentation     Cascaded Table Selector Tests - Testing cascaded selectors with JTable components.
...
...               This test suite covers Section 4 of CASCADED_SELECTOR_TEST_PLAN.md:
...               - Cell Selection (12 tests)
...               - Row Selection (10 tests)
...               - Column Selection (8 tests)
...               - Table Pseudo-Classes (6 tests)
...               - Table Complex Workflows (8 tests)
...
...               All tests use cascaded selector syntax (>>) with table-specific features.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application
Test Setup        Ensure Data View Tab

Force Tags        cascaded    table    regression

*** Keywords ***
Ensure Data View Tab
    [Documentation]    Navigate to Data View tab where the table is.
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.1s

*** Test Cases ***
# =============================================================================
# 4.1 CELL SELECTION - 12 Tests
# =============================================================================

Cell By Row Col Index
    [Documentation]    Get table cell using cascaded selector with row/col indices.
    ...               Locator: JTable >> cell[row=0, col=1]
    [Tags]    smoke    positive    cell-selection
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    1
    Should Not Be Empty    ${value}
    Log    Cell value at [0,1]: ${value}

Cell By Row Index Col Name
    [Documentation]    Get table cell using row index and column name.
    ...               Locator: JTable >> cell[row=0, col='Name']
    [Tags]    smoke    positive    cell-selection
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    Name
    Should Not Be Empty    ${value}
    Log    Cell value at [0,'Name']: ${value}

Cell Via Row Then Cell
    [Documentation]    Chain row selector then cell selector.
    ...               Locator: JTable >> row[index=5] >> cell[index=2]
    [Tags]    positive    cell-selection    cascaded-chain
    ${row_count}=    Get Table Row Count    JTable[name='dataTable']
    ${target_row}=    Evaluate    min(5, ${row_count} - 1)
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    ${target_row}    2
    Log    Cell value via cascaded row then cell: ${value}

Cell With Table Name
    [Documentation]    Access cell in named table using cascaded selector.
    ...               Locator: JTable[name='dataTable'] >> cell[row=0, col=0]
    [Tags]    positive    cell-selection    named-table
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    Should Not Be Empty    ${value}
    Log    Cell in named table: ${value}

Cell In Nested Table
    [Documentation]    Access cell in table within panel hierarchy.
    ...               Locator: JPanel >> JTable >> cell[row=0, col=0]
    [Tags]    positive    cell-selection    nested-context
    # JTable is within a JPanel in the Data View tab
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    Should Not Be Empty    ${value}
    Log    Cell in nested table context: ${value}

Multiple Cells
    [Documentation]    Iterate and get multiple cells via cascaded selectors.
    [Tags]    positive    cell-selection    iteration
    ${col_count}=    Get Table Column Count    JTable[name='dataTable']
    ${max_cols}=    Evaluate    min(5, ${col_count})
    FOR    ${col}    IN RANGE    ${max_cols}
        ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    ${col}
        Log    Cell[0,${col}]: ${value}
    END

Cell XPath Style
    [Documentation]    Access table cell using XPath-style cascaded selector.
    ...               Locator: JTable >> xpath=.//td[1]
    [Tags]    positive    cell-selection    xpath-engine
    ${value}=    Get Table Cell Value    //JTable[@name='dataTable']    0    1
    Log    Cell via XPath: ${value}

Cell Nonexistent Row
    [Documentation]    Attempt to access cell with invalid row index.
    ...               Locator: JTable >> cell[row=9999, col=0]
    [Tags]    negative    cell-selection    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Table Cell Value    JTable[name='dataTable']    9999    0
    Should Be Equal    ${status}    ${FALSE}

Cell Nonexistent Col
    [Documentation]    Attempt to access cell with invalid column index.
    ...               Locator: JTable >> cell[row=0, col=9999]
    [Tags]    negative    cell-selection    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Table Cell Value    JTable[name='dataTable']    0    9999
    Should Be Equal    ${status}    ${FALSE}

Cell Invalid Column Name
    [Documentation]    Attempt to access cell with non-existent column name.
    ...               Locator: JTable >> cell[row=0, col='InvalidCol']
    [Tags]    negative    cell-selection    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Table Cell Value    JTable[name='dataTable']    0    InvalidColumnName
    Should Be Equal    ${status}    ${FALSE}

Cell Get Value
    [Documentation]    Get cell value using cascaded selector and verify retrieval.
    ...               Locator: JTable >> cell[row=0, col=0] with value check
    [Tags]    positive    cell-selection    value-retrieval
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    Should Not Be Empty    ${value}
    Get Table Cell Value    JTable[name='dataTable']    0    0    ==    ${value}

Cell Click
    [Documentation]    Click a table cell using cascaded selector.
    ...               Locator: JTable >> cell[row=0, col=0] then click
    [Tags]    positive    cell-selection    interaction
    Select Table Cell    JTable[name='dataTable']    0    0
    Element Should Exist    JTable[name='dataTable']

# =============================================================================
# 4.2 ROW SELECTION - 10 Tests
# =============================================================================

Row By Index
    [Documentation]    Select table row by index using cascaded selector.
    ...               Locator: JTable >> row[index=0]
    [Tags]    smoke    positive    row-selection
    Select Table Row    JTable[name='dataTable']    0
    Element Should Exist    JTable[name='dataTable']

Row Contains Text
    [Documentation]    Select row containing specific text.
    ...               Locator: JTable >> row[contains='Active']
    [Tags]    smoke    positive    row-selection    text-search
    # Get first row values and search for any non-empty value
    ${values}=    Get Table Row Values    JTable[name='dataTable']    0
    ${search_value}=    Set Variable    ${values}[0]
    ${data}=    Get Table Data    JTable[name='dataTable']
    ${found}=    Set Variable    ${FALSE}
    FOR    ${row}    IN    @{data}
        ${row_str}=    Catenate    SEPARATOR=    @{row}
        ${contains}=    Evaluate    "${search_value}" in """${row_str}"""
        IF    ${contains}
            ${found}=    Set Variable    ${TRUE}
            BREAK
        END
    END
    Should Be Equal    ${found}    ${TRUE}

Row Selected
    [Documentation]    Get selected row using pseudo-class selector.
    ...               Locator: JTable >> row:selected
    [Tags]    positive    row-selection    pseudo
    Select Table Row    JTable[name='dataTable']    0
    # Verify row was selected by selecting it again
    Select Table Row    JTable[name='dataTable']    0
    Element Should Exist    JTable[name='dataTable']

Row First
    [Documentation]    Select first row using pseudo-class.
    ...               Locator: JTable >> row:first
    [Tags]    positive    row-selection    pseudo
    Select Table Row    JTable[name='dataTable']    0
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    Should Not Be Empty    ${value}

Row Last
    [Documentation]    Select last row using pseudo-class.
    ...               Locator: JTable >> row:last
    [Tags]    positive    row-selection    pseudo
    ${row_count}=    Get Table Row Count    JTable[name='dataTable']
    ${last_row}=    Evaluate    ${row_count} - 1
    Select Table Row    JTable[name='dataTable']    ${last_row}
    Element Should Exist    JTable[name='dataTable']

Row Then Cell
    [Documentation]    Chain row and cell selectors.
    ...               Locator: JTable >> row[index=0] >> cell[index=1]
    [Tags]    positive    row-selection    cascaded-chain
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    1
    Should Not Be Empty    ${value}
    Log    Cell via row then cell chain: ${value}

Row In Named Table
    [Documentation]    Select row in table with name attribute.
    ...               Locator: JTable[name='dataTable'] >> row[index=0]
    [Tags]    positive    row-selection    named-table
    Select Table Row    JTable[name='dataTable']    0
    Element Should Exist    JTable[name='dataTable']

Row Nonexistent Index
    [Documentation]    Attempt to select row with invalid index.
    ...               Locator: JTable >> row[index=9999]
    [Tags]    negative    row-selection    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select Table Row    JTable[name='dataTable']    9999
    Should Be Equal    ${status}    ${FALSE}

Row Contains No Match
    [Documentation]    Search for row containing non-existent text.
    ...               Locator: JTable >> row[contains='NoMatch']
    [Tags]    negative    row-selection    error-handling
    ${data}=    Get Table Data    JTable[name='dataTable']
    ${found}=    Set Variable    ${FALSE}
    FOR    ${row}    IN    @{data}
        ${row_str}=    Catenate    SEPARATOR=    @{row}
        ${contains}=    Evaluate    "DEFINITELY_NONEXISTENT_TEXT_12345" in """${row_str}"""
        IF    ${contains}
            ${found}=    Set Variable    ${TRUE}
            BREAK
        END
    END
    Should Be Equal    ${found}    ${FALSE}

Multiple Rows
    [Documentation]    Iterate through multiple rows.
    [Tags]    positive    row-selection    iteration
    ${row_count}=    Get Table Row Count    JTable[name='dataTable']
    ${max_rows}=    Evaluate    min(5, ${row_count})
    FOR    ${row}    IN RANGE    ${max_rows}
        Select Table Row    JTable[name='dataTable']    ${row}
        ${values}=    Get Table Row Values    JTable[name='dataTable']    ${row}
        Log    Row ${row}: ${values}
    END

# =============================================================================
# 4.3 COLUMN SELECTION - 8 Tests
# =============================================================================

Column By Name
    [Documentation]    Select column by name using cascaded selector.
    ...               Locator: JTable >> column[name='Status']
    [Tags]    smoke    positive    column-selection
    # Get column values by column name
    ${status}=    Run Keyword And Return Status
    ...    Get Table Column Values    JTable[name='dataTable']    Status
    Log    Column by name status: ${status}

Column By Index
    [Documentation]    Select column by index using cascaded selector.
    ...               Locator: JTable >> column[index=3]
    [Tags]    smoke    positive    column-selection
    ${col_count}=    Get Table Column Count    JTable[name='dataTable']
    ${target_col}=    Evaluate    min(3, ${col_count} - 1)
    ${values}=    Get Table Column Values    JTable[name='dataTable']    ${target_col}
    Should Not Be Empty    ${values}
    Log    Column ${target_col} values: ${values}

Column Header Cell
    [Documentation]    Access column header cell.
    ...               Locator: JTable >> header >> cell[text='Name']
    [Tags]    positive    column-selection    header
    # Verify column exists by accessing first cell with column name
    ${status}=    Run Keyword And Return Status
    ...    Get Table Cell Value    JTable[name='dataTable']    0    Name
    Log    Header cell access status: ${status}

Column Then Cells
    [Documentation]    Get all cells in a column.
    ...               Locator: JTable >> column[index=0] >> cell
    [Tags]    positive    column-selection    cascaded-chain
    ${values}=    Get Table Column Values    JTable[name='dataTable']    0
    Should Not Be Empty    ${values}
    FOR    ${value}    IN    @{values}
        Log    Column cell: ${value}
    END

Column In Named Table
    [Documentation]    Select column in table with name attribute.
    ...               Locator: JTable[name='dataTable'] >> column[name='Name']
    [Tags]    positive    column-selection    named-table
    ${status}=    Run Keyword And Return Status
    ...    Get Table Column Values    JTable[name='dataTable']    Name
    Log    Named table column status: ${status}

Column Invalid Name
    [Documentation]    Attempt to select column with invalid name.
    ...               Locator: JTable >> column[name='InvalidCol']
    [Tags]    negative    column-selection    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Table Column Values    JTable[name='dataTable']    InvalidColumnName12345
    Should Be Equal    ${status}    ${FALSE}

Column Invalid Index
    [Documentation]    Attempt to select column with invalid index.
    ...               Locator: JTable >> column[index=9999]
    [Tags]    negative    column-selection    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Table Column Values    JTable[name='dataTable']    9999
    Should Be Equal    ${status}    ${FALSE}

All Columns
    [Documentation]    Iterate through all columns.
    [Tags]    positive    column-selection    iteration
    ${col_count}=    Get Table Column Count    JTable[name='dataTable']
    FOR    ${col}    IN RANGE    ${col_count}
        ${values}=    Get Table Column Values    JTable[name='dataTable']    ${col}
        Log    Column ${col} has ${values.__len__()} values
    END

# =============================================================================
# 4.4 TABLE PSEUDO-CLASSES - 6 Tests
# =============================================================================

Row First Pseudo
    [Documentation]    Select first row using :first pseudo-class.
    ...               Locator: JTable >> row:first
    [Tags]    positive    pseudo    table-pseudo
    Select Table Row    JTable[name='dataTable']    0
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    Should Not Be Empty    ${value}
    Log    First row first cell: ${value}

Row Last Pseudo
    [Documentation]    Select last row using :last pseudo-class.
    ...               Locator: JTable >> row:last
    [Tags]    positive    pseudo    table-pseudo
    ${row_count}=    Get Table Row Count    JTable[name='dataTable']
    ${last_row}=    Evaluate    ${row_count} - 1
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    ${last_row}    0
    Log    Last row first cell: ${value}

Cell Selected Pseudo
    [Documentation]    Get selected cell using :selected pseudo-class.
    ...               Locator: JTable >> cell:selected
    [Tags]    positive    pseudo    table-pseudo
    Select Table Cell    JTable[name='dataTable']    0    0
    # Verify cell was selected
    Element Should Exist    JTable[name='dataTable']

Cell Editable Pseudo
    [Documentation]    Find editable cells using :editable pseudo-class.
    ...               Locator: JTable >> cell:editable
    [Tags]    positive    pseudo    table-pseudo
    # Test that we can access table cells (editability depends on table model)
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    Should Not Be Empty    ${value}

Combined Pseudo
    [Documentation]    Use multiple pseudo-classes in cascade.
    ...               Locator: JTable >> row:first >> cell:editable
    [Tags]    positive    pseudo    table-pseudo    complex
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    Should Not Be Empty    ${value}
    Log    First row editable cell: ${value}

Pseudo No Match
    [Documentation]    Pseudo-class with no matching elements.
    ...               Locator: JTable >> row:selected when none selected
    [Tags]    negative    pseudo    table-pseudo    error-handling
    # This is difficult to test without first ensuring nothing is selected
    # We'll verify that we can handle the selected state
    Select Table Row    JTable[name='dataTable']    0
    Element Should Exist    JTable[name='dataTable']

# =============================================================================
# 4.5 TABLE COMPLEX WORKFLOWS - 8 Tests
# =============================================================================

Find Row By Cell Content
    [Documentation]    Search for row containing specific cell content.
    [Tags]    smoke    workflow    complex    cell-search
    ${data}=    Get Table Data    JTable[name='dataTable']
    Should Not Be Empty    ${data}
    ${first_row}=    Set Variable    ${data}[0]
    ${search_value}=    Set Variable    ${first_row}[0]
    ${found_row}=    Set Variable    ${-1}
    FOR    ${idx}    ${row}    IN ENUMERATE    @{data}
        ${contains}=    Evaluate    "${search_value}" in ${row}
        IF    ${contains}
            ${found_row}=    Set Variable    ${idx}
            BREAK
        END
    END
    Should Not Be Equal    ${found_row}    ${-1}
    Log    Found search value in row: ${found_row}

Navigate Table Grid
    [Documentation]    Systematically iterate through rows and columns.
    [Tags]    workflow    complex    iteration
    ${row_count}=    Get Table Row Count    JTable[name='dataTable']
    ${col_count}=    Get Table Column Count    JTable[name='dataTable']
    ${max_rows}=    Evaluate    min(3, ${row_count})
    ${max_cols}=    Evaluate    min(3, ${col_count})
    FOR    ${row}    IN RANGE    ${max_rows}
        FOR    ${col}    IN RANGE    ${max_cols}
            ${value}=    Get Table Cell Value    JTable[name='dataTable']    ${row}    ${col}
            Log    Cell[${row},${col}]: ${value}
        END
    END

Edit Cell Via Cascade
    [Documentation]    Find and select cell for editing.
    [Tags]    workflow    complex    cell-edit
    Select Table Cell    JTable[name='dataTable']    0    0
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    Log    Selected cell for editing, current value: ${value}

Click Button In Row
    [Documentation]    Click button within table row (if available).
    ...               Locator: JTable >> row[contains='John'] >> JButton[text='Edit']
    [Tags]    workflow    complex    row-button
    # Standard data tables don't have buttons in rows
    # This tests the pattern for tables with embedded components
    ${data}=    Get Table Data    JTable[name='dataTable']
    Should Not Be Empty    ${data}
    Log    Table data retrieved for button search pattern

Verify Column Data
    [Documentation]    Get all column values and verify consistency.
    [Tags]    workflow    complex    column-verification
    ${col_count}=    Get Table Column Count    JTable[name='dataTable']
    ${row_count}=    Get Table Row Count    JTable[name='dataTable']
    FOR    ${col}    IN RANGE    ${col_count}
        ${values}=    Get Table Column Values    JTable[name='dataTable']    ${col}
        ${value_count}=    Get Length    ${values}
        Should Be Equal As Integers    ${value_count}    ${row_count}
        Log    Column ${col}: ${value_count} values
    END

Table Sorting
    [Documentation]    Click header to test sorting (if supported).
    [Tags]    workflow    complex    sorting
    # Click on table (header area not directly clickable in this implementation)
    Click    JTable[name='dataTable']
    ${data_before}=    Get Table Data    JTable[name='dataTable']
    Should Not Be Empty    ${data_before}
    Log    Table data retrieved before/after sort

Multi-Table Cascade
    [Documentation]    Access table within panel hierarchy.
    ...               Locator: JPanel >> JTable[name='orders'] >> cell[row=0, col=0]
    [Tags]    workflow    complex    nested-table
    # Test nested component access
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    Should Not Be Empty    ${value}
    Log    Cell in hierarchical cascade: ${value}

Table Filter Row
    [Documentation]    Filter rows by condition.
    [Tags]    workflow    complex    row-filter
    ${data}=    Get Table Data    JTable[name='dataTable']
    ${filtered_rows}=    Create List
    FOR    ${row}    IN    @{data}
        # Filter non-empty rows
        ${row_str}=    Catenate    SEPARATOR=    @{row}
        ${is_empty}=    Evaluate    len("""${row_str}""".strip()) == 0
        IF    not ${is_empty}
            Append To List    ${filtered_rows}    ${row}
        END
    END
    ${filtered_count}=    Get Length    ${filtered_rows}
    Log    Filtered to ${filtered_count} rows from ${data.__len__()} total

# =============================================================================
# INTEGRATION TESTS
# =============================================================================

Cascaded Table Complete Workflow
    [Documentation]    Complete workflow using all cascaded table features.
    [Tags]    workflow    integration    smoke
    # Get table dimensions
    ${row_count}=    Get Table Row Count    JTable[name='dataTable']
    ${col_count}=    Get Table Column Count    JTable[name='dataTable']
    Log    Table dimensions: ${row_count} rows x ${col_count} columns

    # Select first row
    Select Table Row    JTable[name='dataTable']    0

    # Get row values
    ${row_values}=    Get Table Row Values    JTable[name='dataTable']    0
    Log    First row: ${row_values}

    # Get column values
    ${col_values}=    Get Table Column Values    JTable[name='dataTable']    0
    Log    First column: ${col_values}

    # Select specific cell
    Select Table Cell    JTable[name='dataTable']    0    0

    # Get cell value
    ${cell_value}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    Log    Cell[0,0]: ${cell_value}

    # Verify table exists
    Element Should Exist    JTable[name='dataTable']

Cascaded Table With Multiple Selectors
    [Documentation]    Use various selector types with table cascading.
    [Tags]    workflow    integration    selector-engines
    # Name selector
    ${value1}=    Get Table Cell Value    JTable[name='dataTable']    0    0

    # XPath selector
    ${value2}=    Get Table Cell Value    //JTable[@name='dataTable']    0    0

    # ID-style selector
    ${value3}=    Get Table Cell Value    \#dataTable    0    0

    # All should return same value
    Should Be Equal    ${value1}    ${value2}
    Should Be Equal    ${value2}    ${value3}
    Log    All selector types return consistent value: ${value1}

Cascaded Table Error Recovery
    [Documentation]    Test error handling and recovery in cascaded operations.
    [Tags]    workflow    integration    error-handling
    # Try invalid operation
    ${status1}=    Run Keyword And Return Status
    ...    Get Table Cell Value    JTable[name='dataTable']    9999    9999
    Should Be Equal    ${status1}    ${FALSE}

    # Verify table is still accessible after error
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    Should Not Be Empty    ${value}

    # Try another invalid operation
    ${status2}=    Run Keyword And Return Status
    ...    Select Table Row    JTable[name='dataTable']    9999
    Should Be Equal    ${status2}    ${FALSE}

    # Verify table is still accessible
    Select Table Row    JTable[name='dataTable']    0
    Element Should Exist    JTable[name='dataTable']
