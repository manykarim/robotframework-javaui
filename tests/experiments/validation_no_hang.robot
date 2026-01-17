*** Settings ***
Documentation    Validation test for the RPC hang fix
...              Tests multiple sequential RPC calls to ensure no hangs/deadlocks
Resource         ../robot/swt/resources/common.resource
Suite Setup      Start Test Application
Suite Teardown   Stop Test Application

*** Test Cases ***
Multiple FindWidgets Calls Should Not Hang
    [Documentation]    Call findWidgets multiple times - should complete without hanging
    [Tags]    critical    hang-fix
    ${widgets1}=    SwtLibrary.Find Widgets    class:Button
    ${count1}=    Get Length    ${widgets1}
    Log    Found ${count1} buttons
    ${widgets2}=    SwtLibrary.Find Widgets    class:Text
    ${count2}=    Get Length    ${widgets2}
    Log    Found ${count2} text widgets
    ${widgets3}=    SwtLibrary.Find Widgets    class:Button
    ${count3}=    Get Length    ${widgets3}
    Log    Found ${count3} buttons again
    Should Be True    ${count1} > 0

Mixed RPC Calls Should Not Hang
    [Documentation]    Mix different RPC methods - should complete without hanging
    [Tags]    critical    hang-fix
    ${widget}=    SwtLibrary.Find Widget    name:buttonSubmit
    Log    Found: ${widget}
    ${widgets}=    SwtLibrary.Find Widgets    class:Button
    ${count}=    Get Length    ${widgets}
    Log    Found ${count} widgets
    # Just verify we can make multiple different calls without hanging
    ${widget2}=    SwtLibrary.Find Widget    name:buttonSubmit
    Should Be Equal    ${widget}    ${widget2}

Rapid Sequential Calls Should Not Hang
    [Documentation]    Rapid sequential calls - should complete without hanging
    [Tags]    critical    hang-fix
    FOR    ${i}    IN RANGE    10
        ${widgets}=    SwtLibrary.Find Widgets    class:Button
        ${count}=    Get Length    ${widgets}
        Log    Iteration ${i}: Found ${count} widgets
    END

Ten Sequential Tests Should Not Hang
    [Documentation]    Simulate 10 test cases in sequence
    [Tags]    critical    hang-fix
    FOR    ${i}    IN RANGE    10
        ${widget}=    SwtLibrary.Find Widget    name:buttonSubmit
        Should Not Be Equal    ${widget}    ${None}
    END
