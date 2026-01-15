*** Settings ***
Documentation    SWT Button Controls - Testing Push, Toggle, Arrow buttons
...              Operations: Click, Verify Enabled/Disabled, Get Properties
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Click Push Button
    [Documentation]    Click a push button
    [Tags]    button    click    positive
    Click Element    Button[name='submitButton']
    Element Should Exist    Button[name='submitButton']

Click Clear Button
    [Documentation]    Click clear button and verify form cleared
    [Tags]    button    click    positive
    Click Element    Button[name='clearButton']
    Element Should Exist    Button[name='clearButton']

Click Toolbar Run Button
    [Documentation]    Click toolbar run button
    [Tags]    button    toolbar    click    positive
    Click Element    ToolItem[name='toolRun']
    Element Should Exist    ToolItem[name='toolRun']

Click Toolbar Stop Button
    [Documentation]    Click toolbar stop button
    [Tags]    button    toolbar    click    positive
    Click Element    ToolItem[name='toolStop']
    Element Should Exist    ToolItem[name='toolStop']

Click Toolbar New Button
    [Documentation]    Click toolbar new button
    [Tags]    button    toolbar    click    positive
    Click Element    ToolItem[name='toolNew']
    Element Should Exist    ToolItem[name='toolNew']

Click Toolbar Open Button
    [Documentation]    Click toolbar open button
    [Tags]    button    toolbar    click    positive
    Click Element    ToolItem[name='toolOpen']
    Element Should Exist    ToolItem[name='toolOpen']

Click Toolbar Save Button
    [Documentation]    Click toolbar save button
    [Tags]    button    toolbar    click    positive
    Click Element    ToolItem[name='toolSave']
    Element Should Exist    ToolItem[name='toolSave']

Toggle Bold Button
    [Documentation]    Toggle bold toolbar button (CHECK style)
    [Tags]    button    toggle    positive
    Click Element    ToolItem[name='toolBold']
    Element Should Exist    ToolItem[name='toolBold']

Toggle Italic Button
    [Documentation]    Toggle italic toolbar button (CHECK style)
    [Tags]    button    toggle    positive
    Click Element    ToolItem[name='toolItalic']
    Element Should Exist    ToolItem[name='toolItalic']

Select Alignment Radio Buttons
    [Documentation]    Click radio-style alignment buttons
    [Tags]    button    radio    positive
    Click Element    ToolItem[name='toolAlignLeft']
    Click Element    ToolItem[name='toolAlignCenter']
    Click Element    ToolItem[name='toolAlignRight']
    Element Should Exist    ToolItem[name='toolAlignRight']

Button Should Be Enabled
    [Documentation]    Verify button is enabled
    [Tags]    button    enabled    verification
    Element Should Be Enabled    Button[name='submitButton']

Button Should Be Visible
    [Documentation]    Verify button is visible
    [Tags]    button    visible    verification
    Element Should Be Visible    Button[name='clearButton']

Click Arrow Buttons
    [Documentation]    Click arrow buttons for navigation
    [Tags]    button    arrow    positive
    Click Element    Button[name='arrowUpButton']
    Click Element    Button[name='arrowDownButton']
    Element Should Exist    Button[name='arrowUpButton']
