*** Settings ***
Documentation     Test table operations
Resource          resources/common.resource
Suite Setup       Start Demo Application
Suite Teardown    Stop Demo Application

*** Test Cases ***
Find Table By Name
    [Documentation]    Find table using name selector
    Element Should Exist    JTable[name='dataTable']

Table Should Be Visible
    [Documentation]    Verify table is displayed
    Element Should Be Visible    JTable[name='dataTable']

Table Using XPath
    [Documentation]    Find table using XPath
    Element Should Exist    //JTable[@name='dataTable']

Get Table Row Count
    [Documentation]    Get the number of rows in a table
    ${count}=    Get Table Row Count    JTable[name='dataTable']
    Should Be True    ${count} >= 5
    Log    Table has ${count} rows

Get Table Column Count
    [Documentation]    Get number of columns in table
    ${count}=    Get Table Column Count    [name='dataTable']
    Should Be Equal As Integers    ${count}    5

Get Table Cell Value
    [Documentation]    Get specific cell values
    ${id}=    Get Table Cell Value    JTable[name='dataTable']    0    0
    Should Be Equal    ${id}    1

    ${name}=    Get Table Cell Value    JTable[name='dataTable']    0    1
    Should Be Equal    ${name}    John Doe

    ${email}=    Get Table Cell Value    JTable[name='dataTable']    0    2
    Should Be Equal    ${email}    john@example.com

Read Table Cell By Column Index
    [Documentation]    Read table cell using column index
    ${value}=    Get Table Cell Value    [name='dataTable']    1    2
    Should Be Equal    ${value}    jane@example.com

Navigate Table Cells
    [Documentation]    Navigate through table cells
    FOR    ${row}    IN RANGE    0    3
        FOR    ${col}    IN RANGE    0    5
            ${value}=    Get Table Cell Value    JTable[name='dataTable']    ${row}    ${col}
            Log    Cell [${row}][${col}] = ${value}
        END
    END
