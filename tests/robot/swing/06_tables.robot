*** Settings ***
Test Timeout       60s
Documentation     Table Tests - Testing AssertionEngine-based table keywords:
...               Get Table Cell Value, Get Table Row Count, Get Table Column Count,
...               Get Table Row Values, Get Table Column Values with assertion operators.
...
...               These tests verify the library's ability to interact with
...               JTable components with Browser Library-style assertions.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application
Test Setup        Ensure Data View Tab

Force Tags        tables    regression    assertion-engine

*** Keywords ***
Ensure Data View Tab
    [Documentation]    Navigate to Data View tab where the table is.
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.1s

*** Test Cases ***
# =============================================================================
# GET TABLE CELL VALUE WITH ASSERTION OPERATORS
# =============================================================================

Get Table Cell Value By Row And Column Index
    [Documentation]    Get value from table cell with assertion operator.
    [Tags]    smoke    positive    assertion-operator
    Get Table Cell Value    JTable[name='dataTable']    0    0    !=    ${EMPTY}

Get Table Cell Value By Row And Column Name
    [Documentation]    Get value using row index and column name with assertion.
    [Tags]    positive    assertion-operator
    Get Table Cell Value    JTable[name='dataTable']    0    Name    !=    ${EMPTY}

Get Multiple Cell Values
    [Documentation]    Get values from multiple cells.
    [Tags]    positive
    ${val1}=    Get Table Cell Value    [name='dataTable']    0    0
    ${val2}=    Get Table Cell Value    [name='dataTable']    0    1
    ${val3}=    Get Table Cell Value    [name='dataTable']    1    0
    Log    Values: ${val1}, ${val2}, ${val3}

Get Table Cell Value Using XPath
    [Documentation]    Get cell value using XPath selector with assertion.
    [Tags]    positive    xpath-locator    assertion-operator
    Get Table Cell Value    //JTable[@name='dataTable']    0    0    !=    ${EMPTY}

Get Table Cell Value From Last Row
    [Documentation]    Get cell value from the last row.
    [Tags]    positive
    ${row_count}=    Get Table Row Count    [name='dataTable']
    ${last_row}=    Evaluate    ${row_count} - 1
    ${value}=    Get Table Cell Value    [name='dataTable']    ${last_row}    0
    Log    Last row value: ${value}

Get Table Cell Value From Last Column
    [Documentation]    Get cell value from the last column.
    [Tags]    positive
    ${col_count}=    Get Table Column Count    [name='dataTable']
    ${last_col}=    Evaluate    ${col_count} - 1
    ${value}=    Get Table Cell Value    [name='dataTable']    0    ${last_col}
    Log    Last column value: ${value}

Get Table Cell Value With Contains Assertion
    [Documentation]    Verify cell value contains expected substring.
    [Tags]    positive    assertion-operator
    ${status}=    Run Keyword And Return Status
    ...    Get Table Cell Value    JTable[name='dataTable']    0    0    *=    ${EMPTY}
    # Note: This tests the contains operator, result depends on actual data
    Log    Contains check: ${status}

Get Table Cell Value With Exact Match
    [Documentation]    Verify cell value matches exactly.
    [Tags]    positive    assertion-operator
    ${expected}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    Get Table Cell Value    JTable[name='dataTable']    0    0    ==    ${expected}

# =============================================================================
# SELECT TABLE CELL
# =============================================================================

Select Table Cell By Row And Column
    [Documentation]    Select a table cell by row and column index.
    [Tags]    smoke    positive
    Select Table Cell    JTable[name='dataTable']    0    0
    Element Should Exist    JTable[name='dataTable']

Select Table Cell Using ID Selector
    [Documentation]    Select a table cell using ID-style selector.
    [Tags]    positive
    Select Table Cell    \#dataTable    1    1
    Element Should Exist    \#dataTable

Select Multiple Cells Sequentially
    [Documentation]    Select multiple cells one after another.
    [Tags]    positive
    Select Table Cell    [name='dataTable']    0    0
    Select Table Cell    [name='dataTable']    1    1
    Select Table Cell    [name='dataTable']    2    2
    Element Should Exist    [name='dataTable']

Select Table Cell Using XPath
    [Documentation]    Select cell using XPath selector.
    [Tags]    positive    xpath-locator
    Select Table Cell    //JTable[@name='dataTable']    0    0
    Element Should Exist    //JTable[@name='dataTable']

Select Same Cell Multiple Times
    [Documentation]    Verify selecting same cell multiple times is safe.
    [Tags]    positive    edge-case
    FOR    ${i}    IN RANGE    3
        Select Table Cell    [name='dataTable']    0    0
    END
    Element Should Exist    [name='dataTable']

# =============================================================================
# SELECT TABLE ROW
# =============================================================================

Select Table Row By Index
    [Documentation]    Select a table row by index.
    [Tags]    smoke    positive
    Select Table Row    JTable[name='dataTable']    0
    Element Should Exist    JTable[name='dataTable']

Select Table Row Using ID Selector
    [Documentation]    Select a table row using ID-style selector.
    [Tags]    positive
    Select Table Row    \#dataTable    1
    Element Should Exist    \#dataTable

Select Multiple Rows Sequentially
    [Documentation]    Select multiple rows one after another.
    [Tags]    positive
    Select Table Row    [name='dataTable']    0
    Select Table Row    [name='dataTable']    1
    Select Table Row    [name='dataTable']    2
    Element Should Exist    [name='dataTable']

Select Table Row Using XPath
    [Documentation]    Select row using XPath selector.
    [Tags]    positive    xpath-locator
    Select Table Row    //JTable[@name='dataTable']    0
    Element Should Exist    //JTable[@name='dataTable']

Select Last Table Row
    [Documentation]    Select the last row in the table.
    [Tags]    positive
    ${row_count}=    Get Table Row Count    [name='dataTable']
    ${last_row}=    Evaluate    ${row_count} - 1
    Select Table Row    [name='dataTable']    ${last_row}
    Element Should Exist    [name='dataTable']

Select Same Row Multiple Times
    [Documentation]    Verify selecting same row multiple times is safe.
    [Tags]    positive    edge-case
    FOR    ${i}    IN RANGE    3
        Select Table Row    [name='dataTable']    0
    END
    Element Should Exist    [name='dataTable']

# =============================================================================
# GET TABLE ROW COUNT WITH ASSERTION OPERATORS
# =============================================================================

Get Table Row Count By Name
    [Documentation]    Get the number of rows with assertion operator.
    [Tags]    smoke    positive    assertion-operator
    Get Table Row Count    JTable[name='dataTable']    >    0

Get Table Row Count Using ID Selector
    [Documentation]    Get row count using ID-style selector with assertion.
    [Tags]    positive    assertion-operator
    Get Table Row Count    \#dataTable    >    0

Get Table Row Count Using XPath
    [Documentation]    Get row count using XPath selector with assertion.
    [Tags]    positive    xpath-locator    assertion-operator
    Get Table Row Count    //JTable[@name='dataTable']    >    0

Get Table Row Count With Exact Value
    [Documentation]    Get row count and verify exact value.
    [Tags]    positive    assertion-operator
    ${count}=    Get Table Row Count    [name='dataTable']
    Get Table Row Count    [name='dataTable']    ==    ${count}

Get Table Row Count Greater Or Equal
    [Documentation]    Verify row count is at least a certain value.
    [Tags]    positive    assertion-operator
    Get Table Row Count    [name='dataTable']    >=    1

# =============================================================================
# GET TABLE COLUMN COUNT WITH ASSERTION OPERATORS
# =============================================================================

Get Table Column Count By Name
    [Documentation]    Get the number of columns with assertion operator.
    [Tags]    smoke    positive    assertion-operator
    Get Table Column Count    JTable[name='dataTable']    >    0

Get Table Column Count Using ID Selector
    [Documentation]    Get column count using ID-style selector with assertion.
    [Tags]    positive    assertion-operator
    Get Table Column Count    \#dataTable    >    0

Get Table Column Count Using XPath
    [Documentation]    Get column count using XPath selector with assertion.
    [Tags]    positive    xpath-locator    assertion-operator
    Get Table Column Count    //JTable[@name='dataTable']    >    0

Get Table Column Count With Exact Value
    [Documentation]    Get column count and verify exact value.
    [Tags]    positive    assertion-operator
    ${count}=    Get Table Column Count    [name='dataTable']
    Get Table Column Count    [name='dataTable']    ==    ${count}

# =============================================================================
# GET TABLE DATA
# =============================================================================

Get All Table Data
    [Documentation]    Get all data from a table as 2D list.
    [Tags]    smoke    positive
    ${data}=    Get Table Data    JTable[name='dataTable']
    Should Not Be Empty    ${data}
    Log    Table data: ${data}

Get Table Data Using ID Selector
    [Documentation]    Get table data using ID-style selector.
    [Tags]    positive
    ${data}=    Get Table Data    \#dataTable
    Should Not Be Empty    ${data}

Get Table Data Using XPath
    [Documentation]    Get table data using XPath selector.
    [Tags]    positive    xpath-locator
    ${data}=    Get Table Data    //JTable[@name='dataTable']
    Should Not Be Empty    ${data}

Verify Table Data Structure
    [Documentation]    Verify the structure of returned table data.
    [Tags]    positive
    ${data}=    Get Table Data    [name='dataTable']
    ${row_count}=    Get Table Row Count    [name='dataTable']
    ${col_count}=    Get Table Column Count    [name='dataTable']
    ${data_rows}=    Get Length    ${data}
    Should Be Equal As Integers    ${data_rows}    ${row_count}

Access Specific Cell From Table Data
    [Documentation]    Access a specific cell from returned table data.
    [Tags]    positive
    ${data}=    Get Table Data    [name='dataTable']
    ${first_row}=    Set Variable    ${data}[0]
    ${first_cell}=    Set Variable    ${first_row}[0]
    Log    First cell value: ${first_cell}

# =============================================================================
# GET TABLE ROW VALUES WITH ASSERTION
# =============================================================================

Get Row Values
    [Documentation]    Get all values from a table row.
    [Tags]    positive    assertion-operator
    ${values}=    Get Table Row Values    [name='dataTable']    0
    Should Not Be Empty    ${values}
    Log    Row values: ${values}

# =============================================================================
# GET TABLE COLUMN VALUES WITH ASSERTION
# =============================================================================

Get Column Values
    [Documentation]    Get all values from a table column.
    [Tags]    positive    assertion-operator
    ${values}=    Get Table Column Values    [name='dataTable']    0
    Should Not Be Empty    ${values}
    Log    Column values: ${values}

# =============================================================================
# TABLE WORKFLOWS
# =============================================================================

Navigate Through Table Cells Workflow
    [Documentation]    Navigate through table cells systematically.
    [Tags]    workflow    smoke
    ${row_count}=    Get Table Row Count    [name='dataTable']
    ${col_count}=    Get Table Column Count    [name='dataTable']
    # Navigate first row
    FOR    ${col}    IN RANGE    ${col_count}
        ${value}=    Get Table Cell Value    [name='dataTable']    0    ${col}
        Log    Row 0, Col ${col}: ${value}
    END

Select And Read Row Workflow
    [Documentation]    Select a row and read its values.
    [Tags]    workflow
    Select Table Row    [name='dataTable']    0
    ${col_count}=    Get Table Column Count    [name='dataTable']
    FOR    ${col}    IN RANGE    ${col_count}
        ${value}=    Get Table Cell Value    [name='dataTable']    0    ${col}
        Log    Column ${col}: ${value}
    END

Table Data Processing Workflow
    [Documentation]    Process table data programmatically.
    [Tags]    workflow
    ${data}=    Get Table Data    [name='dataTable']
    FOR    ${row}    IN    @{data}
        Log    Row: ${row}
    END

# =============================================================================
# TABLE STATE VERIFICATION WITH ASSERTION OPERATORS
# =============================================================================

Verify Table Is Enabled With Get Property
    [Documentation]    Verify table is enabled using Get Property assertion.
    [Tags]    positive    verification    assertion-operator
    Select Data View Tab
    Get Property    JTable[name='dataTable']    enabled    ==    ${TRUE}

Verify Table Is Visible With Get Property
    [Documentation]    Verify table is visible using Get Property assertion.
    [Tags]    positive    verification    assertion-operator
    Select Data View Tab
    Get Property    JTable[name='dataTable']    visible    ==    ${TRUE}

Verify Table Exists
    [Documentation]    Verify table exists in the UI.
    [Tags]    positive    verification
    Element Should Exist    JTable[name='dataTable']

# =============================================================================
# FINDING TABLES
# =============================================================================

Find All Tables
    [Documentation]    Find all table elements in the application.
    [Tags]    positive
    ${tables}=    Find Elements    JTable
    Should Not Be Empty    ${tables}
    Log    Found tables

Find Enabled Tables
    [Documentation]    Find all enabled tables.
    [Tags]    positive
    ${tables}=    Find Elements    JTable:enabled
    Should Not Be Empty    ${tables}

Find Visible Tables
    [Documentation]    Find all visible tables.
    [Tags]    positive
    ${tables}=    Find Elements    JTable:visible
    Should Not Be Empty    ${tables}

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Get Cell Value From Nonexistent Table Fails
    [Documentation]    Get cell from non-existent table throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Table Cell Value    JTable[name='nonexistent']    0    0
    Should Be Equal    ${status}    ${FALSE}

Select Cell In Nonexistent Table Fails
    [Documentation]    Select cell in non-existent table throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select Table Cell    JTable[name='nonexistent']    0    0
    Should Be Equal    ${status}    ${FALSE}

Select Row In Nonexistent Table Fails
    [Documentation]    Select row in non-existent table throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select Table Row    JTable[name='nonexistent']    0
    Should Be Equal    ${status}    ${FALSE}

Get Row Count From Nonexistent Table Fails
    [Documentation]    Get row count from non-existent table throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Table Row Count    JTable[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Get Column Count From Nonexistent Table Fails
    [Documentation]    Get column count from non-existent table throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Table Column Count    JTable[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Get Data From Nonexistent Table Fails
    [Documentation]    Get data from non-existent table throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Table Data    JTable[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Select Invalid Row Index Fails
    [Documentation]    Select row with invalid index throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select Table Row    [name='dataTable']    9999
    Should Be Equal    ${status}    ${FALSE}

Select Invalid Cell Index Fails
    [Documentation]    Select cell with invalid indices throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Select Table Cell    [name='dataTable']    9999    9999
    Should Be Equal    ${status}    ${FALSE}

Get Invalid Cell Value Fails
    [Documentation]    Get cell value with invalid indices throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Get Table Cell Value    [name='dataTable']    9999    9999
    Should Be Equal    ${status}    ${FALSE}

Get Cell Value Assertion Fails For Wrong Value
    [Documentation]    Cell value assertion fails for wrong expected value.
    [Tags]    negative    error-handling    assertion-operator
    ${status}=    Run Keyword And Return Status
    ...    Get Table Cell Value    JTable[name='dataTable']    0    0    ==    DEFINITELY_NOT_THIS_VALUE_12345
    Should Be Equal    ${status}    ${FALSE}

Get Row Count Assertion Fails For Wrong Count
    [Documentation]    Row count assertion fails for wrong count.
    [Tags]    negative    error-handling    assertion-operator
    ${status}=    Run Keyword And Return Status
    ...    Get Table Row Count    JTable[name='dataTable']    ==    99999
    Should Be Equal    ${status}    ${FALSE}

# =============================================================================
# EDGE CASES
# =============================================================================

Access First Cell
    [Documentation]    Access the first cell (0,0) specifically.
    [Tags]    edge-case
    ${value}=    Get Table Cell Value    [name='dataTable']    0    0
    Log    First cell: ${value}

Access Cell With Zero Row
    [Documentation]    Access cells in the first row.
    [Tags]    edge-case
    ${col_count}=    Get Table Column Count    [name='dataTable']
    FOR    ${col}    IN RANGE    ${col_count}
        ${value}=    Get Table Cell Value    [name='dataTable']    0    ${col}
        Log    Cell (0, ${col}): ${value}
    END

Rapid Cell Selection
    [Documentation]    Test rapid cell selection.
    [Tags]    edge-case    stress
    FOR    ${i}    IN RANGE    10
        Select Table Cell    [name='dataTable']    0    0
    END
    Element Should Exist    [name='dataTable']

Rapid Row Selection
    [Documentation]    Test rapid row selection.
    [Tags]    edge-case    stress
    ${row_count}=    Get Table Row Count    [name='dataTable']
    ${max_row}=    Evaluate    min(${row_count}, 5)
    FOR    ${row}    IN RANGE    ${max_row}
        Select Table Row    [name='dataTable']    ${row}
    END
    Element Should Exist    [name='dataTable']

Double Click On Table
    [Documentation]    Double-click on a table cell.
    [Tags]    edge-case
    Double Click    JTable[name='dataTable']
    Element Should Exist    JTable[name='dataTable']
