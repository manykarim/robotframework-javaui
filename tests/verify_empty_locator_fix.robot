*** Settings ***
Documentation     Verify empty locator validation works correctly
Library           JavaGui.SwtLibrary

*** Test Cases ***
Empty Locator Returns Proper Error
    [Documentation]    Verify empty locator returns clear error message
    TRY
        Find Widget    ${EMPTY}
        Fail    Should have raised error for empty locator
    EXCEPT    *empty*    type=GLOB
        Log    ✅ Empty locator correctly rejected with proper error message
    EXCEPT    *    type=GLOB    AS    ${error}
        Log    Got error: ${error}
    END
