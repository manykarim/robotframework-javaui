*** Settings ***
Documentation     Test list operations
Resource          resources/common.resource
Suite Setup       Start Demo Application
Suite Teardown    Stop Demo Application

*** Test Cases ***
List Should Be Visible
    [Documentation]    Verify list is displayed
    Element Should Be Visible    JList[name='itemList']

Find List By Name
    [Documentation]    Find list using name selector
    Element Should Exist    JList[name='itemList']

List Using XPath
    [Documentation]    Find list using XPath
    Element Should Exist    //JList[@name='itemList']
