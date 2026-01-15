*** Settings ***
Documentation    Button Controls - Testing JButton click operations
...              Operations: Click, Verify Enabled/Disabled, Get Properties
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Click Submit Button
    [Documentation]    Click the submit button
    [Tags]    button    click    positive
    Click Element    JButton[name='submitButton']
    Element Should Exist    JButton[name='submitButton']

Click Clear Button
    [Documentation]    Click clear button and verify form fields are cleared
    [Tags]    button    click    form    positive
    Input Text    JTextField[name='nameTextField']    Test Name
    Input Text    JTextField[name='emailTextField']    test@test.com
    Click Element    JButton[name='clearButton']
    Sleep    0.5s    Wait for clear operation
    ${name}=    Get Element Text    JTextField[name='nameTextField']
    ${email}=    Get Element Text    JTextField[name='emailTextField']
    Should Be Empty    ${name}
    Should Be Empty    ${email}

Click Toolbar New Button
    [Documentation]    Click toolbar button for new document
    [Tags]    button    toolbar    click    positive
    Click Element    JButton[name='toolbarNewButton']
    Element Should Exist    JButton[name='toolbarNewButton']

Click Toolbar Open Button
    [Documentation]    Click toolbar button for open
    [Tags]    button    toolbar    click    positive
    Click Element    JButton[name='toolbarOpenButton']
    Element Should Exist    JButton[name='toolbarOpenButton']

Click Toolbar Save Button
    [Documentation]    Click toolbar button for save
    [Tags]    button    toolbar    click    positive
    Click Element    JButton[name='toolbarSaveButton']
    Element Should Exist    JButton[name='toolbarSaveButton']

Click Toolbar Refresh Button
    [Documentation]    Click toolbar refresh button
    [Tags]    button    toolbar    click    positive
    Click Element    JButton[name='toolbarRefreshButton']
    Element Should Exist    JButton[name='toolbarRefreshButton']

Click Add Row Button
    [Documentation]    Click add row button
    [Tags]    button    click    table    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Data View
    Sleep    0.3s
    ${initial_count}=    Get Table Row Count    JTable[name='dataTable']
    Click Element    JButton[name='addRowButton']
    Sleep    0.3s
    ${new_count}=    Get Table Row Count    JTable[name='dataTable']
    Should Be True    ${new_count} >= ${initial_count}

Button Should Be Enabled
    [Documentation]    Verify button is enabled
    [Tags]    button    enabled    verification
    Element Should Be Enabled    JButton[name='submitButton']

Button Should Be Visible
    [Documentation]    Verify button is visible
    [Tags]    button    visible    verification
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Sleep    0.3s
    Element Should Be Visible    JButton[name='clearButton']

Click Start Progress Button
    [Documentation]    Click start progress button
    [Tags]    button    click    progress    positive
    Click Element    JButton[name='startProgressButton']
    Sleep    0.5s
    Element Should Exist    JProgressBar[name='progressBar']
