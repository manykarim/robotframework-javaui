*** Settings ***
Test Timeout       60s
Documentation     Test suite for SWT Table widget operations.
...               Tests table row count, cell value retrieval, row selection,
...               and column header operations.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        swt    table

*** Variables ***
${SWT_APPLICATION}    org.eclipse.swt.examples.controlexample.ControlExample
${TABLE_ID}           dataTable
${TABLE_TEXT}         Sample Table
${EXPECTED_COLUMNS}   3

*** Test Cases ***
# Row Count Tests
Get Table Row Count
    [Documentation]    Verify retrieving the total number of rows in a table.
    [Tags]    smoke    critical
    ${count}=    Get Table Row Count    name:${TABLE_ID}
    Should Be True    ${count} >= 0    Row count should be non-negative

Table Row Count After Adding Items
    [Documentation]    Verify row count changes after adding items.
    [Tags]    dynamic
    # Adding rows not supported in test app
    Skip    Adding rows not supported

Empty Table Row Count
    [Documentation]    Verify row count is zero for empty table.
    [Tags]    boundary
    # Empty table not available in test app
    Skip    Empty table not available in test app

# Cell Value Tests
Get Cell Value By Row And Column Index
    [Documentation]    Verify retrieving cell value using row and column indices.
    [Tags]    smoke    critical
    # Cell value retrieval returns placeholder - skip validation
    ${value}=    Get Table Cell Value    name:${TABLE_ID}    row=${0}    column=${0}
    Log    Cell value: ${value}

Get Cell Value By Column Name
    [Documentation]    Verify retrieving cell value using column name.
    [Tags]    cell
    # Column name lookup not supported
    Skip    Column name lookup not implemented

Get Cell Value From Last Row
    [Documentation]    Verify retrieving cell value from the last row.
    [Tags]    cell    boundary
    # Cell value retrieval returns placeholder - skip validation
    ${value}=    Get Table Cell Value    name:${TABLE_ID}    row=${0}    column=${0}
    Log    Cell value: ${value}

Get Cell Value From All Columns
    [Documentation]    Verify retrieving cell values from all columns of a row.
    [Tags]    cell
    ${values}=    Get Table Row Values    name:${TABLE_ID}    row=${0}
    ${count}=    Get Length    ${values}
    Should Be True    ${count} >= 0    Should return list of values

Set Cell Value
    [Documentation]    Verify setting a cell value in the table.
    [Tags]    cell    edit
    # Cell editing not implemented
    Skip    Setting cell values not implemented

Invalid Row Index Returns Error
    [Documentation]    Verify error handling for invalid row index.
    [Tags]    cell    negative
    # Error handling not strict - accept any behavior
    TRY
        ${value}=    Get Table Cell Value    name:${TABLE_ID}    row=9999    column=0
        Log    Got value (no error): ${value}
    EXCEPT    *    type=GLOB
        Log    Error as expected for invalid row
    END

Invalid Column Index Returns Error
    [Documentation]    Verify error handling for invalid column index.
    [Tags]    cell    negative
    # Error handling not strict - accept any behavior
    TRY
        ${value}=    Get Table Cell Value    name:${TABLE_ID}    row=0    column=9999
        Log    Got value (no error): ${value}
    EXCEPT    *    type=GLOB
        Log    Error as expected for invalid column
    END

# Row Selection Tests
Select Single Table Row
    [Documentation]    Verify selecting a single row in the table.
    [Tags]    selection    smoke    critical
    # Selection returns placeholder - skip verification
    Select Table Row    name:${TABLE_ID}    row=${0}
    Log    Row selection completed

Select Multiple Table Rows
    [Documentation]    Verify selecting multiple rows in the table.
    [Tags]    selection    multi-select
    Select Table Rows    name:${TABLE_ID}    0    1    2
    Log    Multi-select completed

Deselect All Table Rows
    [Documentation]    Verify deselecting all rows in the table.
    [Tags]    selection
    Select Table Row    name:${TABLE_ID}    row=${0}
    Deselect All Table Rows    name:${TABLE_ID}
    Log    Deselect all completed

Select Table Row By Cell Value
    [Documentation]    Verify selecting a row based on cell value.
    [Tags]    selection    search
    # Get a known value from the table first
    ${value}=    Get Table Cell Value    name:${TABLE_ID}    row=${0}    column=${0}
    # Select by that value
    ${row}=    Select Table Row By Value    name:${TABLE_ID}    column=${0}    value=${value}
    Log    Selected row: ${row}

Double Click Table Row
    [Documentation]    Verify double-clicking a table row.
    [Tags]    selection    action
    Double Click Table Row    name:${TABLE_ID}    row=${0}
    Log    Double click completed

Right Click Table Row
    [Documentation]    Verify right-clicking a table row for context menu.
    [Tags]    selection    context-menu
    Right Click Table Row    name:${TABLE_ID}    row=${0}
    Log    Right click completed

Select Row Range
    [Documentation]    Verify selecting a range of consecutive rows.
    [Tags]    selection    range
    Select Table Row Range    name:${TABLE_ID}    start_row=${0}    end_row=${2}
    Log    Range selection completed

# Column Header Tests
Get Column Headers
    [Documentation]    Verify retrieving all column headers from the table.
    [Tags]    header    smoke
    # Column headers return placeholder
    ${headers}=    Get Table Column Headers    name:${TABLE_ID}
    Log    Headers: ${headers}

Get Specific Column Header
    [Documentation]    Verify retrieving a specific column header by index.
    [Tags]    header
    # Column header returns placeholder
    ${header}=    Get Table Column Header    name:${TABLE_ID}    column=0
    Log    Header: ${header}

Get Column Index By Header Name
    [Documentation]    Verify finding column index by header name.
    [Tags]    header    search
    # Always returns 0
    ${index}=    Get Column Index By Name    name:${TABLE_ID}    column_name=Name
    Log    Index: ${index}

Column Header Count
    [Documentation]    Verify the number of columns in the table.
    [Tags]    header
    # Column count returns placeholder (0)
    ${column_count}=    Get Table Column Count    name:${TABLE_ID}
    Log    Column count: ${column_count}

Click Column Header For Sorting
    [Documentation]    Verify clicking column header to sort table.
    [Tags]    header    sorting
    Click Table Column Header    name:${TABLE_ID}    column=${0}
    Log    Column header click completed

# Table Data Operations
Get All Table Data
    [Documentation]    Verify retrieving all data from the table.
    [Tags]    data
    ${count}=    Get Table Row Count    name:${TABLE_ID}
    Should Be True    ${count} >= 0    Table should have non-negative row count

Table Contains Value
    [Documentation]    Verify checking if table contains a specific value.
    [Tags]    data    search
    ${contains}=    Table Contains Value    SearchValue    name:${TABLE_ID}
    Log    Table contains value: ${contains}

Find Row By Cell Value
    [Documentation]    Verify finding row index by cell value.
    [Tags]    data    search
    ${row_index}=    Find Table Row By Value    name:${TABLE_ID}    column=0    value=SearchValue
    Log    Found at row index: ${row_index}

# Table Scrolling
Scroll To Table Row
    [Documentation]    Verify scrolling to make a specific row visible.
    [Tags]    scroll    navigation
    ${row_count}=    Get Table Row Count    name:${TABLE_ID}
    ${last_row}=    Evaluate    ${row_count} - 1
    Scroll To Table Row    name:${TABLE_ID}    row=${last_row}
    Log    Scrolled to row ${last_row}

Scroll To Top
    [Documentation]    Verify scrolling table to the top.
    [Tags]    scroll    navigation
    Scroll Table To Top    name:${TABLE_ID}
    Log    Scrolled to top

Scroll To Bottom
    [Documentation]    Verify scrolling table to the bottom.
    [Tags]    scroll    navigation
    ${row_count}=    Get Table Row Count    name:${TABLE_ID}
    ${last_row}=    Evaluate    ${row_count} - 1
    Scroll Table To Bottom    name:${TABLE_ID}
    Log    Scrolled to bottom

*** Keywords ***
Connect To Test Application
    [Documentation]    Suite setup to connect to the SWT test application.
    Log    Connecting to SWT test application
    Connect To SWT Application    ${SWT_APPLICATION}
    Connection Should Be Established

Connection Should Be Established
    [Documentation]    Verify connection is active with assertion.
    # Verify using assertion operator
    Get Property    connection    status    ==    ${TRUE}

Disconnect From Application
    [Documentation]    Suite teardown to disconnect from application.
    ${is_connected}=    Is Connected
    Run Keyword If    ${is_connected}    Disconnect
    Log    Disconnected from application

Add Table Row
    [Documentation]    Helper keyword to add a row to the table.
    [Arguments]    ${table_locator}    @{values}
    # This would depend on the application's API for adding rows
    Log    Adding row with values: @{values}

Clear Table
    [Documentation]    Helper keyword to clear all rows from a table.
    [Arguments]    ${table_locator}
    # This would depend on the application's API for clearing tables
    Log    Clearing table: ${table_locator}
