*** Settings ***
Documentation    SWT Table Controls - Testing Table widget
...              Operations: Get Cell Value, Select Row, Get Row Count, Edit Cell
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Get Table Cell Value
    [Documentation]    Get a cell value from the table
    [Tags]    table    get    cell    positive
    ${value}=    Get Table Cell Value    Table[name='dataTable']    0    1
    Should Not Be Empty    ${value}

Get Table Row Count
    [Documentation]    Get the number of rows in the table
    [Tags]    table    count    positive
    ${count}=    Get Table Row Count    Table[name='dataTable']
    Should Be True    ${count} >= 10    # Initial data has 10 rows

Select Table Row By Index
    [Documentation]    Select a row in the table by index
    [Tags]    table    select    row    positive
    Select Table Row    Table[name='dataTable']    2
    ${selected}=    Get Table Selected Row    Table[name='dataTable']
    Should Be Equal As Integers    ${selected}    2

Select Multiple Table Rows Sequentially
    [Documentation]    Select different rows in the table
    [Tags]    table    select    sequence    positive
    Select Table Row    Table[name='dataTable']    0
    ${selected}=    Get Table Selected Row    Table[name='dataTable']
    Should Be Equal As Integers    ${selected}    0
    Select Table Row    Table[name='dataTable']    3
    ${selected}=    Get Table Selected Row    Table[name='dataTable']
    Should Be Equal As Integers    ${selected}    3
    Select Table Row    Table[name='dataTable']    5
    ${selected}=    Get Table Selected Row    Table[name='dataTable']
    Should Be Equal As Integers    ${selected}    5

Verify Table Cell Values In First Row
    [Documentation]    Verify cell values in the first row
    [Tags]    table    verify    row    positive
    ${col0}=    Get Table Cell Value    Table[name='dataTable']    0    0
    ${col1}=    Get Table Cell Value    Table[name='dataTable']    0    1
    Should Not Be Empty    ${col0}
    Should Not Be Empty    ${col1}

Table Should Be Enabled
    [Documentation]    Verify table is enabled
    [Tags]    table    enabled    verification
    Element Should Be Enabled    Table[name='dataTable']

Table Should Be Visible
    [Documentation]    Verify table is visible
    [Tags]    table    visible    verification
    Element Should Be Visible    Table[name='dataTable']

Select First And Last Table Rows
    [Documentation]    Select the first and last rows
    [Tags]    table    select    boundary    positive
    Select Table Row    Table[name='dataTable']    0
    ${selected}=    Get Table Selected Row    Table[name='dataTable']
    Should Be Equal As Integers    ${selected}    0
    ${count}=    Get Table Row Count    Table[name='dataTable']
    ${last_row}=    Evaluate    ${count} - 1
    Select Table Row    Table[name='dataTable']    ${last_row}
    ${selected}=    Get Table Selected Row    Table[name='dataTable']
    Should Be Equal As Integers    ${selected}    ${last_row}
