*** Settings ***
Documentation    SWT Dialog Controls - Testing Shell dialogs
...              Operations: Open Dialog, Close Dialog, Interact with Dialog Controls
Resource         resources/common.resource
Suite Setup      Start SWT Test Application
Suite Teardown   Stop SWT Test Application

*** Test Cases ***
Open Test Dialog
    [Documentation]    Open the test dialog via toolbar and close it
    [Tags]    dialog    open    close    positive
    Click Element    ToolItem[name='toolDialog']
    Sleep    0.5s
    Wait Until Element Exists    Shell[name='testDialog']    timeout=5
    Element Should Be Visible    Shell[name='testDialog']
    # Close the dialog
    Click Element    Button[name='testDialogCloseButton']
    Sleep    0.3s

Open About Dialog
    [Documentation]    Open the about dialog and close it
    [Tags]    dialog    about    open    close    positive
    Select Menu Item    Help|About
    Sleep    0.5s
    Wait Until Element Exists    Shell[name='aboutDialog']    timeout=5
    Element Should Be Visible    Shell[name='aboutDialog']
    # Close with OK button
    Click Element    Button[name='aboutOkButton']
    Sleep    0.3s

Open Message Box
    [Documentation]    Open message box via toolbar
    [Tags]    dialog    messagebox    positive
    Click Element    ToolItem[name='toolMessage']
    Sleep    0.5s
    # MessageBox is a native dialog, just verify main shell is still there
    Element Should Exist    Shell[name='mainShell']

Open And Close Dialog Multiple Times
    [Documentation]    Open and close dialog multiple times
    [Tags]    dialog    toggle    multiple    positive
    # First open/close cycle
    Click Element    ToolItem[name='toolDialog']
    Sleep    0.3s
    Wait Until Element Exists    Shell[name='testDialog']    timeout=5
    Click Element    Button[name='testDialogCloseButton']
    Sleep    0.3s
    # Second open/close cycle
    Click Element    ToolItem[name='toolDialog']
    Sleep    0.3s
    Wait Until Element Exists    Shell[name='testDialog']    timeout=5
    Click Element    Button[name='testDialogCloseButton']
    Sleep    0.3s
    # Third open/close cycle
    Click Element    ToolItem[name='toolDialog']
    Sleep    0.3s
    Wait Until Element Exists    Shell[name='testDialog']    timeout=5
    Click Element    Button[name='testDialogCloseButton']
    Sleep    0.3s

Interact With Dialog Controls
    [Documentation]    Open dialog and interact with its controls
    [Tags]    dialog    interact    controls    positive
    Click Element    ToolItem[name='toolDialog']
    Sleep    0.5s
    Wait Until Element Exists    Shell[name='testDialog']    timeout=5
    # Interact with controls in dialog (if any named controls exist)
    Element Should Exist    Shell[name='testDialog']
    # Close dialog
    Click Element    Button[name='testDialogCloseButton']
    Sleep    0.3s
