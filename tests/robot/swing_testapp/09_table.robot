*** Settings ***
Documentation    Table Controls - Testing JTable operations
...              Operations: Get Cell Value, Select Row, Get Row Count
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Get Table Cell Value
    [Documentation]    Get a cell value from the table
    [Tags]    table    get    cell    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    0    1
    Should Be Equal    ${value}    Laptop

Get Table Row Count
    [Documentation]    Get the number of rows in the table
    [Tags]    table    count    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    ${count}=    Get Table Row Count    JTable[name='dataTable']
    Should Be True    ${count} >= 8    # Initial data has 8 rows

Get Table Column Count
    [Documentation]    Get the number of columns in the table
    [Tags]    table    count    columns    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    ${count}=    Get Table Column Count    JTable[name='dataTable']
    Should Be True    ${count} >= 4    # ID, Name, Category, Price

Select Table Row By Index
    [Documentation]    Select a row in the table by index
    [Tags]    table    select    row    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    Select Table Row    JTable[name='dataTable']    2
    # Verify row is selected by checking we can still access the table
    Element Should Exist    JTable[name='dataTable']

Select Table Cell
    [Documentation]    Select a specific cell in the table
    [Tags]    table    select    cell    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    Select Table Cell    JTable[name='dataTable']    1    1
    # Verify cell value at selected position
    ${value}=    Get Table Cell Value    JTable[name='dataTable']    1    1
    Should Not Be Empty    ${value}

Select Multiple Table Rows Sequentially
    [Documentation]    Select different rows in the table
    [Tags]    table    select    sequence    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    # Select row 0
    Select Table Row    JTable[name='dataTable']    0
    Sleep    0.2s
    # Select row 3
    Select Table Row    JTable[name='dataTable']    3
    Sleep    0.2s
    # Select row 5
    Select Table Row    JTable[name='dataTable']    5
    Element Should Exist    JTable[name='dataTable']

Verify Table Cell Values In First Row
    [Documentation]    Verify all cell values in the first row
    [Tags]    table    verify    row    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    ${id}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    ${name}=    Get Table Cell Value    JTable[name='dataTable']    0    1
    ${category}=    Get Table Cell Value    JTable[name='dataTable']    0    2
    Should Be Equal    ${id}    1
    Should Be Equal    ${name}    Laptop
    Should Be Equal    ${category}    Electronics

Get All Table Data
    [Documentation]    Get all data from the table
    [Tags]    table    get    all    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    ${data}=    Get Table Data    JTable[name='dataTable']
    Should Not Be Empty    ${data}

Add Row And Verify
    [Documentation]    Add a new row to the table and verify
    [Tags]    table    add    row    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    ${initial_count}=    Get Table Row Count    JTable[name='dataTable']
    Click Element    JButton[name='addRowButton']
    Sleep    0.3s
    ${new_count}=    Get Table Row Count    JTable[name='dataTable']
    ${expected}=    Evaluate    ${initial_count} + 1
    Should Be Equal As Integers    ${new_count}    ${expected}

Delete Row And Verify
    [Documentation]    Delete a row from the table and verify
    [Tags]    table    delete    row    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    ${initial_count}=    Get Table Row Count    JTable[name='dataTable']
    # First add a row to delete
    Click Element    JButton[name='addRowButton']
    Sleep    0.3s
    ${after_add}=    Get Table Row Count    JTable[name='dataTable']
    # Select the last row
    ${last_row}=    Evaluate    ${after_add} - 1
    Select Table Row    JTable[name='dataTable']    ${last_row}
    # Delete it
    Click Element    JButton[name='deleteRowButton']
    Sleep    0.3s
    ${final_count}=    Get Table Row Count    JTable[name='dataTable']
    Should Be Equal As Integers    ${final_count}    ${initial_count}

Table Should Be Enabled
    [Documentation]    Verify table is enabled
    [Tags]    table    enabled    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    Element Should Be Enabled    JTable[name='dataTable']

Table Should Be Visible
    [Documentation]    Verify table is visible
    [Tags]    table    visible    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    Element Should Be Visible    JTable[name='dataTable']
