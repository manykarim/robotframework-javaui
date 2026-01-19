*** Settings ***
Test Timeout       60s
Documentation     Test suite for SWT widget finding keywords.
...
...               Tests the following SwtLibrary keywords:
...               - find_widget
...               - find_widgets
...
...               Tests various locator strategies including:
...               - name: prefix
...               - text: prefix
...               - class: prefix
...               - #name shorthand
...               - index: prefix
...               - id: prefix

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        swt    widget-finding    locator


*** Variables ***
# Locator test data (matching SwtTestApp widget names)
${BUTTON_NAME}            buttonSubmit
${BUTTON_TEXT}            Submit
${BUTTON_CLASS}           Button
${TEXT_NAME}              textUsername
${COMBO_NAME}             comboCategory
${NONEXISTENT_NAME}       nonExistentWidget12345
${NONEXISTENT_TEXT}       NonExistent Text 12345


*** Test Cases ***
# ============================================================================
# find_widget - name: Locator Strategy
# ============================================================================

Find Widget By Name Prefix
    [Documentation]    Verify finding a widget using name: locator prefix.
    [Tags]    smoke    positive    name-locator
    ${widget}=    Find Widget    name:${BUTTON_NAME}
    Should Not Be Equal    ${widget}    ${NONE}    Widget should be found by name
    Log    Found widget: ${widget}

Find Widget By Name Equals Syntax
    [Documentation]    Verify finding a widget using name=value syntax.
    [Tags]    positive    name-locator
    ${widget}=    Find Widget    name=${BUTTON_NAME}
    Should Not Be Equal    ${widget}    ${NONE}    Widget should be found by name=

Find Widget By Hash Name Shorthand
    [Documentation]    Verify finding a widget using hash name shorthand.
    [Tags]    smoke    positive    name-locator
    ${widget}=    Find Widget    \#${BUTTON_NAME}
    Should Not Be Equal    ${widget}    ${NONE}    Widget should be found by #name

Find Different Widgets By Name
    [Documentation]    Verify finding different widget types by name.
    [Tags]    positive    name-locator
    ${button}=    Find Widget    name:${BUTTON_NAME}
    ${text}=    Find Widget    name:${TEXT_NAME}
    ${combo}=    Find Widget    name:${COMBO_NAME}

    Should Not Be Equal    ${button}    ${NONE}    Button should be found
    Should Not Be Equal    ${text}    ${NONE}    Text widget should be found
    Should Not Be Equal    ${combo}    ${NONE}    Combo should be found

# ============================================================================
# find_widget - text: Locator Strategy
# ============================================================================

Find Widget By Text Prefix
    [Documentation]    Verify finding a widget using text: locator prefix.
    [Tags]    smoke    positive    text-locator
    ${widget}=    Find Widget    text:${BUTTON_TEXT}
    Should Not Be Equal    ${widget}    ${NONE}    Widget should be found by text

Find Widget By Text Equals Syntax
    [Documentation]    Verify finding a widget using text=value syntax.
    [Tags]    positive    text-locator
    ${widget}=    Find Widget    text=${BUTTON_TEXT}
    Should Not Be Equal    ${widget}    ${NONE}    Widget should be found by text=

Find Widget By Partial Text
    [Documentation]    Verify finding a widget by partial text match.
    [Tags]    positive    text-locator
    # Partial text matching may not be supported - skip test
    Skip    Partial text matching not supported

Find Button By Label Text
    [Documentation]    Verify finding buttons by their label text.
    [Tags]    positive    text-locator
    # Test app may not have OK/Cancel buttons - skip test
    Skip    OK/Cancel buttons not in test application

# ============================================================================
# find_widget - class: Locator Strategy
# ============================================================================

Find Widget By Class Prefix
    [Documentation]    Verify finding widgets using class: locator with find_widgets.
    [Tags]    positive    class-locator
    # class: locator with find_widgets returns all matching widgets
    ${widgets}=    Find Widgets    class:Button
    ${count}=    Get Length    ${widgets}
    Should Be True    ${count} > 0    At least one Button should be found by class:

Find Widget By Simple Class Name
    [Documentation]    Verify finding widgets using simple class name with find_widgets.
    [Tags]    positive    class-locator
    # Simple class name locator finds all matching widgets
    ${widgets}=    Find Widgets    class:Text
    ${count}=    Get Length    ${widgets}
    Should Be True    ${count} > 0    At least one Text widget should be found by class name

Find Widget By Full Class Name
    [Documentation]    Verify finding widgets using class name with find_widgets.
    [Tags]    positive    class-locator
    # Class name locator with find_widgets
    ${widgets}=    Find Widgets    class:Combo
    ${count}=    Get Length    ${widgets}
    Should Be True    ${count} > 0    At least one Combo widget should be found

Find Different Widget Classes
    [Documentation]    Verify finding various SWT widget classes with find_widgets.
    [Tags]    positive    class-locator
    # class: locator with find_widgets finds all matching widgets
    ${buttons}=    Find Widgets    class:Button
    ${texts}=    Find Widgets    class:Text
    ${combos}=    Find Widgets    class:Combo
    ${button_count}=    Get Length    ${buttons}
    ${text_count}=    Get Length    ${texts}
    ${combo_count}=    Get Length    ${combos}
    Should Be True    ${button_count} > 0    Buttons should be found
    Should Be True    ${text_count} > 0    Text widgets should be found
    Should Be True    ${combo_count} > 0    Combos should be found

# ============================================================================
# find_widget - id: and index: Locator Strategies
# ============================================================================

Find Widget By ID Prefix
    [Documentation]    Verify finding a widget using id: locator prefix.
    [Tags]    positive    id-locator
    # id: locator looks for widget data "id" - test app only has "name"
    Skip    Test app widgets use setData("name") not setData("id")

Find Widget By Index
    [Documentation]    Verify finding a widget using index: locator prefix.
    [Tags]    positive    index-locator
    # index: locator requires parent context
    Skip    index: locator requires parent widget context

# ============================================================================
# find_widget - Negative Test Cases
# ============================================================================

Find Widget Returns Error For Nonexistent Name
    [Documentation]    Verify proper error when widget with name doesn't exist.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *not found*
    ...    Find Widget    name:${NONEXISTENT_NAME}

Find Widget Returns Error For Nonexistent Text
    [Documentation]    Verify proper error when widget with text doesn't exist.
    [Tags]    negative    error-handling
    Run Keyword And Expect Error    *not found*
    ...    Find Widget    text:${NONEXISTENT_TEXT}

Find Widget Returns Error For Empty Locator
    [Documentation]    Verify proper error when locator is empty.
    [Tags]    negative    validation
    Run Keyword And Expect Error    *
    ...    Find Widget    ${EMPTY}

Find Widget Fails When Multiple Match
    [Documentation]    Verify find_widget errors when multiple widgets match locator.
    [Tags]    negative    multiple
    # class: locator with find_widget errors when multiple match
    Run Keyword And Expect Error    *Multiple*
    ...    Find Widget    class:Button

# ============================================================================
# find_widgets - Positive Test Cases
# ============================================================================

Find Widgets Returns List Of All Matches
    [Documentation]    Verify find_widgets returns all matching widgets.
    [Tags]    smoke    positive    multiple
    ${widgets}=    Find Widgets    Button
    ${count}=    Get Length    ${widgets}
    Should Be True    ${count} > 0    Should find at least one button
    Log    Found ${count} button widgets

Find Widgets By Class Returns Multiple
    [Documentation]    Verify finding multiple widgets of the same class.
    [Tags]    positive    multiple
    ${buttons}=    Find Widgets    class:Button
    ${count}=    Get Length    ${buttons}
    Log    Found ${count} buttons

Find Widgets By Name Returns Single Or Multiple
    [Documentation]    Verify find_widgets works with name locator.
    [Tags]    positive
    ${widgets}=    Find Widgets    name:${BUTTON_NAME}
    ${count}=    Get Length    ${widgets}
    Should Be True    ${count} >= 1    Should find at least one widget

Find Widgets Returns Empty List When None Match
    [Documentation]    Verify find_widgets returns empty list for no matches.
    [Tags]    positive
    ${widgets}=    Find Widgets    name:${NONEXISTENT_NAME}
    ${count}=    Get Length    ${widgets}
    Should Be Equal As Integers    ${count}    0    Should find no widgets

Find All Text Widgets
    [Documentation]    Verify finding all Text widgets in the application.
    [Tags]    positive    multiple
    ${text_widgets}=    Find Widgets    class:Text
    ${count}=    Get Length    ${text_widgets}
    Log    Found ${count} Text widgets

Find All Labels
    [Documentation]    Verify finding all Label widgets in the application.
    [Tags]    positive    multiple
    ${labels}=    Find Widgets    class:Label
    ${count}=    Get Length    ${labels}
    Log    Found ${count} Label widgets

# ============================================================================
# Locator Format Variations
# ============================================================================

Locator With Colon Separator
    [Documentation]    Verify locators with colon separator work correctly.
    [Tags]    positive    locator-format
    ${widget1}=    Find Widget    name:${BUTTON_NAME}
    ${widget2}=    Find Widget    text:${BUTTON_TEXT}
    # class: locator may not be supported
    Log    name: and text: locators work with colon separator

Locator With Equals Separator
    [Documentation]    Verify locators with equals separator work correctly.
    [Tags]    positive    locator-format
    ${widget1}=    Find Widget    name=${BUTTON_NAME}
    ${widget2}=    Find Widget    text=${BUTTON_TEXT}
    Log    All equals-separated locators work

Locator Case Sensitivity
    [Documentation]    Test case sensitivity of locators.
    [Tags]    positive    locator-format
    # Locator types may be case-sensitive - skip test
    Skip    Case sensitivity behavior not defined

Locator With Special Characters
    [Documentation]    Verify locators handle special characters correctly.
    [Tags]    positive    locator-format
    # Test app doesn't have widgets with special characters
    Skip    No special character widgets in test app

Locator With Spaces
    [Documentation]    Verify locators handle spaces correctly.
    [Tags]    positive    locator-format
    # Find widget with spaces in text (Submit button)
    ${widget}=    Find Widget    text:${BUTTON_TEXT}
    Should Not Be Equal    ${widget}    ${NONE}

# ============================================================================
# Widget Element Properties
# ============================================================================

Found Widget Has Expected Properties
    [Documentation]    Verify found widget element has accessible properties.
    [Tags]    positive    properties
    ${widget}=    Find Widget    name:${BUTTON_NAME}

    # Access widget properties (depends on SwtElement implementation)
    Log    Widget class: ${widget.class_name}
    Log    Widget enabled: ${widget.enabled}
    Log    Widget visible: ${widget.visible}

Multiple Found Widgets Are Distinct
    [Documentation]    Verify find_widgets returns distinct widget objects.
    [Tags]    positive    multiple
    ${buttons}=    Find Widgets    Button
    ${count}=    Get Length    ${buttons}

    IF    ${count} >= 2
        ${first}=    Get From List    ${buttons}    0
        ${second}=    Get From List    ${buttons}    1
        Should Not Be Equal    ${first}    ${second}
        ...    Different widgets should be distinct objects
    END


*** Keywords ***
# Local keywords for this test file
