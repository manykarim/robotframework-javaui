*** Settings ***
Documentation     Button Tests - Testing click_button, click, double_click,
...               and right_click keywords.
...
...               These tests verify the library's ability to interact with
...               button components using various click operations.

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        buttons    regression

*** Test Cases ***
# =============================================================================
# CLICK BUTTON KEYWORD
# =============================================================================

Click Button By Name
    [Documentation]    Click a button using the name attribute selector.
    ...                Uses the click_button keyword specifically for buttons.
    [Tags]    smoke    positive
    Click Button    JButton[name='loginBtn']
    Sleep    0.3s    Wait for click processing

Click Button By Text
    [Documentation]    Click a button using the text attribute selector.
    [Tags]    smoke    positive
    Click Button    JButton[text='Clear']
    Sleep    0.3s

Click Button By ID Selector
    [Documentation]    Click a button using ID-style selector (#name).
    [Tags]    positive
    Click Button    \#loginBtn
    Sleep    0.3s

Click Button By XPath
    [Documentation]    Click a button using XPath selector.
    [Tags]    positive    xpath-locator
    Click Button    //JButton[@name='clearBtn']
    Sleep    0.3s

Click Button With Combined Selectors
    [Documentation]    Click button using combined type and attribute selectors.
    [Tags]    positive
    Click Button    JButton[name='loginBtn'][text='Login']
    Sleep    0.3s

Click Button In Login Panel
    [Documentation]    Click button within a specific container using descendant.
    [Tags]    positive
    Click Button    JPanel[name='loginPanel'] JButton[name='loginBtn']
    Sleep    0.3s

# =============================================================================
# CLICK KEYWORD (GENERIC)
# =============================================================================

Click On Button Element
    [Documentation]    Use generic click keyword on a button element.
    ...                The click keyword works on any clickable element.
    [Tags]    smoke    positive
    Click    JButton[name='loginBtn']
    Sleep    0.3s

Click On Enabled Button
    [Documentation]    Click on a button verified to be enabled.
    [Tags]    positive
    Element Should Be Enabled    JButton[name='loginBtn']
    Click    JButton[name='loginBtn']:enabled
    Sleep    0.3s

Click On Visible Button
    [Documentation]    Click on a button verified to be visible.
    [Tags]    positive
    Element Should Be Visible    JButton[name='loginBtn']
    Click    JButton:visible[name='loginBtn']
    Sleep    0.3s

Click Using XPath Locator
    [Documentation]    Use generic click with XPath selector.
    [Tags]    positive    xpath-locator
    Click    //JButton[@text='Login']
    Sleep    0.3s

Click Using Descendant Combinator
    [Documentation]    Click element found with descendant combinator.
    [Tags]    positive
    Click    JPanel JButton[name='clearBtn']
    Sleep    0.3s

Click First Button
    [Documentation]    Click the first button in the application.
    [Tags]    positive
    Click    JButton:first-child
    Sleep    0.3s

# =============================================================================
# DOUBLE CLICK KEYWORD
# =============================================================================

Double Click On Button
    [Documentation]    Perform a double-click on a button element.
    ...                Some buttons may have double-click functionality.
    [Tags]    positive
    Double Click    JButton[name='loginBtn']
    Sleep    0.5s

Double Click On Table
    [Documentation]    Double-click on a table element.
    ...                Common for opening items in table views.
    [Tags]    positive
    Double Click    JTable[name='dataTable']
    Sleep    0.5s

Double Click Using XPath
    [Documentation]    Double-click using XPath selector.
    [Tags]    positive    xpath-locator
    Double Click    //JButton[@name='clearBtn']
    Sleep    0.5s

Double Click On List Item
    [Documentation]    Double-click on a list component.
    ...                Common for opening list items.
    [Tags]    positive
    Double Click    JList[name='itemList']
    Sleep    0.5s

# =============================================================================
# RIGHT CLICK KEYWORD
# =============================================================================

Right Click On Button
    [Documentation]    Perform a right-click (context click) on a button.
    ...                Opens context menu if available.
    [Tags]    positive    context-menu
    Right Click    JButton[name='loginBtn']
    Sleep    0.3s

Right Click On Table
    [Documentation]    Right-click on a table element.
    ...                Common for opening table context menus.
    [Tags]    positive    context-menu
    Right Click    JTable[name='dataTable']
    Sleep    0.3s

Right Click On Tree
    [Documentation]    Right-click on a tree element.
    ...                Common for tree node context menus.
    [Tags]    positive    context-menu
    Right Click    JTree[name='fileTree']
    Sleep    0.3s

Right Click Using XPath
    [Documentation]    Right-click using XPath selector.
    [Tags]    positive    xpath-locator    context-menu
    Right Click    //JTree[@name='fileTree']
    Sleep    0.3s

# =============================================================================
# CLICK ELEMENT KEYWORD (WITH COUNT)
# =============================================================================

Click Element Single Click
    [Documentation]    Click element with explicit single click count.
    [Tags]    positive
    Click Element    JButton[name='loginBtn']    click_count=1
    Sleep    0.3s

Click Element Double Click
    [Documentation]    Click element with double click count.
    [Tags]    positive
    Click Element    JButton[name='loginBtn']    click_count=2
    Sleep    0.5s

Click Element Triple Click
    [Documentation]    Click element with triple click count.
    ...                Useful for selecting entire lines in text fields.
    [Tags]    positive
    Click Element    JTextField[name='username']    click_count=3
    Sleep    0.3s

# =============================================================================
# BUTTON INTERACTION WORKFLOWS
# =============================================================================

Login Button Workflow
    [Documentation]    Test complete login button interaction workflow.
    [Tags]    workflow    smoke
    # Clear the form first
    Click Button    JButton[name='clearBtn']
    Sleep    0.2s
    # Enter credentials
    Input Text    [name='username']    testuser
    Input Text    [name='password']    testpass
    # Click login
    Click Button    JButton[name='loginBtn']
    Sleep    0.5s
    # Verify status label updated
    Element Should Exist    JLabel[name='statusLabel']

Clear Form Button Workflow
    [Documentation]    Test clear button functionality.
    [Tags]    workflow
    # Enter some text
    Input Text    [name='username']    someuser
    Input Text    [name='password']    somepass
    # Click clear
    Click Button    JButton[text='Clear']
    Sleep    0.3s
    # Form should still exist
    Element Should Exist    JTextField[name='username']

Multiple Button Clicks Sequence
    [Documentation]    Test sequence of button clicks.
    [Tags]    workflow
    Click Button    JButton[name='clearBtn']
    Sleep    0.2s
    Click Button    JButton[name='loginBtn']
    Sleep    0.5s
    Click Button    JButton[name='clearBtn']
    Sleep    0.2s

# =============================================================================
# BUTTON STATE VERIFICATION
# =============================================================================

Verify Button Is Enabled Before Click
    [Documentation]    Verify button is enabled before attempting to click.
    [Tags]    positive    verification
    Element Should Be Enabled    JButton[name='loginBtn']
    Click Button    JButton[name='loginBtn']

Verify Button Is Visible Before Click
    [Documentation]    Verify button is visible before attempting to click.
    [Tags]    positive    verification
    Element Should Be Visible    JButton[name='loginBtn']
    Click Button    JButton[name='loginBtn']

Verify Button Exists Before Click
    [Documentation]    Verify button exists before attempting to click.
    [Tags]    positive    verification
    Element Should Exist    JButton[name='loginBtn']
    Click Button    JButton[name='loginBtn']

# =============================================================================
# FINDING BUTTONS
# =============================================================================

Find All Buttons In Application
    [Documentation]    Find all button elements in the application.
    [Tags]    positive
    ${buttons}=    Find Elements    JButton
    ${count}=    Get Length    ${buttons}
    Should Be True    ${count} > 5
    Log    Found ${count} buttons in the application

Find Buttons In Panel
    [Documentation]    Find buttons within a specific panel.
    [Tags]    positive
    ${buttons}=    Find Elements    JPanel[name='loginPanel'] JButton
    Should Not Be Empty    ${buttons}

Find Enabled Buttons
    [Documentation]    Find all enabled buttons.
    [Tags]    positive
    ${buttons}=    Find Elements    JButton:enabled
    Should Not Be Empty    ${buttons}

Find Visible Buttons
    [Documentation]    Find all visible buttons.
    [Tags]    positive
    ${buttons}=    Find Elements    JButton:visible
    Should Not Be Empty    ${buttons}

# =============================================================================
# NEGATIVE TESTS
# =============================================================================

Click Nonexistent Button Fails
    [Documentation]    Click on non-existent button throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Click Button    JButton[name='nonexistent_button']
    Should Be Equal    ${status}    ${FALSE}

Click With Invalid Locator Fails
    [Documentation]    Click with invalid locator syntax throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Click    [[[invalid]]]
    Should Be Equal    ${status}    ${FALSE}

Double Click Nonexistent Element Fails
    [Documentation]    Double-click on non-existent element throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Double Click    JButton[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

Right Click Nonexistent Element Fails
    [Documentation]    Right-click on non-existent element throws error.
    [Tags]    negative    error-handling
    ${status}=    Run Keyword And Return Status
    ...    Right Click    JButton[name='nonexistent']
    Should Be Equal    ${status}    ${FALSE}

# =============================================================================
# EDGE CASES
# =============================================================================

Click Button With Long Text
    [Documentation]    Click button with long text content.
    [Tags]    edge-case
    ${buttons}=    Find Elements    JButton[text*='L']
    Should Not Be Empty    ${buttons}

Click Button Rapidly
    [Documentation]    Test rapid successive clicks on button.
    [Tags]    edge-case    stress
    FOR    ${i}    IN RANGE    5
        Click    JButton[name='clearBtn']
        Sleep    0.1s
    END

Click Multiple Different Buttons
    [Documentation]    Click multiple different buttons in sequence.
    [Tags]    edge-case
    Click Button    JButton[name='loginBtn']
    Sleep    0.2s
    Click Button    JButton[name='clearBtn']
    Sleep    0.2s
    Click Button    JButton[name='loginBtn']
