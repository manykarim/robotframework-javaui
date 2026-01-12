*** Settings ***
Documentation     Test tree operations
Resource          resources/common.resource
Suite Setup       Start Demo Application
Suite Teardown    Stop Demo Application

*** Test Cases ***
Tree Should Be Visible
    [Documentation]    Verify tree is displayed
    Element Should Be Visible    JTree[name='fileTree']

Find Tree By Name
    [Documentation]    Find tree using name selector
    Element Should Exist    JTree[name='fileTree']

Tree Using XPath
    [Documentation]    Find tree using XPath
    Element Should Exist    //JTree[@name='fileTree']
