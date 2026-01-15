*** Settings ***
Documentation    Label Controls - Testing JLabel read operations
...              Operations: Get Text, Verify Text, Verify Exists
Resource         resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Get Label Text
    [Documentation]    Get the text of a label
    [Tags]    label    get    text    positive
    ${text}=    Get Element Text    JLabel[name='statusLabel']
    Should Not Be Empty    ${text}

Verify Status Label Initial Value
    [Documentation]    Verify the initial status label value
    [Tags]    label    verify    initial    positive
    ${text}=    Get Element Text    JLabel[name='statusLabel']
    Should Be Equal    ${text}    Ready

Label Should Be Visible
    [Documentation]    Verify label is visible
    [Tags]    label    visible    verification
    Element Should Be Visible    JLabel[name='statusLabel']

Label Should Exist
    [Documentation]    Verify label exists
    [Tags]    label    exists    verification
    Element Should Exist    JLabel[name='nameLabel']
    Element Should Exist    JLabel[name='emailLabel']
    Element Should Exist    JLabel[name='passwordLabel']

Verify Form Labels
    [Documentation]    Verify all form labels exist on Form Input tab
    [Tags]    label    form    verify    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Form Input
    Sleep    0.3s
    Element Should Exist    JLabel[name='nameLabel']
    Element Should Exist    JLabel[name='emailLabel']
    Element Should Exist    JLabel[name='passwordLabel']
    Element Should Exist    JLabel[name='descriptionLabel']

Verify Selections Tab Labels
    [Documentation]    Verify labels on Selections tab
    [Tags]    label    selections    verify    positive
    Select Tab    JTabbedPane[name='mainTabbedPane']    Selections
    Sleep    0.3s
    Element Should Exist    JLabel[name='categoryLabel']
    Element Should Exist    JLabel[name='quantityLabel']
    Element Should Exist    JLabel[name='volumeLabel']
    Element Should Exist    JLabel[name='optionsLabel']
    Element Should Exist    JLabel[name='priorityLabel']
    Element Should Exist    JLabel[name='itemsLabel']

Status Label Updates On Selection
    [Documentation]    Verify status label updates when tree node is selected
    [Tags]    label    update    dynamic    positive
    # Select a tree node to update status
    Expand Tree Node    JTree[name='fileTree']    Project Root|Sources
    Sleep    0.2s
    Select Tree Node    JTree[name='fileTree']    Project Root|Sources
    Sleep    0.3s
    ${text}=    Get Element Text    JLabel[name='statusLabel']
    Should Contain    ${text}    Sources
