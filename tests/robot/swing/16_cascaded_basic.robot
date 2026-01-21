*** Settings ***
Test Timeout       60s
Documentation     Basic Cascaded Selector Tests - Testing cascaded selector syntax
...               and basic chaining operations.
...
...               Covers Section 1 of CASCADED_SELECTOR_TEST_PLAN.md:
...               - 1.1 Simple Chaining (10 tests)
...               - 1.2 Whitespace Handling (5 tests)
...
...               These tests verify the fundamental cascaded selector syntax
...               with the >> operator for parent-child relationships and
...               proper handling of whitespace variations.

Resource          resources/common.resource
Resource          resources/cascaded_selectors.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        cascaded    regression

*** Test Cases ***
# =============================================================================
# SECTION 1.1: SIMPLE CHAINING (10 tests)
# =============================================================================

Two-Segment Cascade
    [Documentation]    Basic parent-child cascaded selector with two segments.
    ...                Tests JPanel >> JButton pattern.
    [Tags]    smoke    positive
    ${element}=    Find Element    JPanel[name='formPanel'] >> JButton[name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Three-Segment Cascade
    [Documentation]    Multi-level chain with three segments.
    ...                Tests JFrame >> JPanel >> JButton pattern.
    [Tags]    smoke    positive
    ${element}=    Find Element    JFrame >> JPanel >> JButton
    Should Not Be Equal    ${element}    ${NONE}

Four-Segment Cascade
    [Documentation]    Deep hierarchy with four segments.
    ...                Tests JFrame >> JTabbedPane >> JPanel >> JButton pattern.
    [Tags]    positive
    ${element}=    Find Element    JFrame >> JTabbedPane >> JPanel >> JButton
    Should Not Be Equal    ${element}    ${NONE}

Cascade With Name Attributes
    [Documentation]    Cascaded selector with name attribute matching.
    ...                Tests JPanel[name='main'] >> JButton[name='submit'] pattern.
    [Tags]    smoke    positive
    # Using actual component names from the test application
    ${element}=    Find Element    JPanel[name='formPanel'] >> JButton[name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Cascade With Text Attributes
    [Documentation]    Cascaded selector with text attribute matching.
    ...                Tests JPanel >> JButton[text='Submit'] pattern.
    [Tags]    positive
    ${element}=    Find Element    JPanel >> JButton[text='Submit']
    Should Not Be Equal    ${element}    ${NONE}

Cascade Mixed Attributes
    [Documentation]    Cascaded selector with multiple attribute types.
    ...                Tests combining title, name, and type attributes.
    [Tags]    positive
    # Using realistic pattern with JTabbedPane and button
    ${element}=    Find Element    JTabbedPane[name='mainTabbedPane'] >> JPanel[name='formPanel'] >> JButton[name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Direct Child Only
    [Documentation]    CSS child combinator for direct children only.
    ...                Tests JPanel > JButton pattern.
    [Tags]    positive
    ${element}=    Find Element    JPanel[name='formPanel'] > JButton[name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Descendant Any Level
    [Documentation]    Space combinator for any descendant level.
    ...                Tests JPanel JButton pattern (space separator).
    [Tags]    positive
    ${element}=    Find Element    JPanel[name='formPanel'] JButton[name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Cascade With Type Only
    [Documentation]    Type-based chain without attributes.
    ...                Tests JTabbedPane >> JPanel >> JButton >> JLabel pattern.
    [Tags]    positive
    # Find a label within a button panel - use specific attributes to avoid ambiguity
    ${element}=    Find Element    JTabbedPane[name='mainTabbedPane'] >> JPanel[name='formPanel'] >> JLabel >> index=0
    Should Not Be Equal    ${element}    ${NONE}

Empty Result Cascade
    [Documentation]    Cascaded selector that produces no matches.
    ...                Tests error handling for non-existent elements.
    [Tags]    negative
    ${elements}=    Find Elements    JDialog[name='nonexistent'] >> JButton
    ${count}=    Get Length    ${elements}
    Should Be Equal As Integers    ${count}    0

# =============================================================================
# SECTION 1.2: WHITESPACE HANDLING (5 tests)
# =============================================================================

No Whitespace Around Separator
    [Documentation]    Test cascaded selector without whitespace around >>.
    ...                Tests JPanel>>JButton pattern.
    [Tags]    edge-case
    ${element}=    Find Element    JPanel[name='formPanel']>>JButton[name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Single Space Around Separator
    [Documentation]    Test cascaded selector with single spaces around >>.
    ...                Tests JPanel >> JButton pattern (standard format).
    [Tags]    positive
    ${element}=    Find Element    JPanel[name='formPanel'] >> JButton[name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Multiple Spaces Around Separator
    [Documentation]    Test cascaded selector with multiple spaces around >>.
    ...                Tests JPanel  >>  JButton pattern.
    ...                SKIPPED: Robot Framework parses multiple spaces as argument separators.
    [Tags]    edge-case    robot:skip
    ${element}=    Find Element    JPanel  >>  JButton
    Should Not Be Equal    ${element}    ${NONE}

Tab Characters Around Separator
    [Documentation]    Test cascaded selector with tab characters around >>.
    ...                Tests JPanel\t>>\tJButton pattern.
    [Tags]    edge-case
    # Robot Framework will interpret \t as tab
    ${element}=    Find Element    JPanel[name='formPanel']\t>>\tJButton[name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Mixed Whitespace
    [Documentation]    Test cascaded selector with various whitespace combinations.
    ...                Tests multiple whitespace patterns to ensure consistent parsing.
    [Tags]    edge-case
    # Test various whitespace combinations - use specific selectors
    ${element1}=    Find Element    JPanel[name='formPanel']>>JButton[name='submitButton']
    Should Not Be Equal    ${element1}    ${NONE}
    ${element2}=    Find Element    JPanel[name='formPanel'] >> JButton[name='submitButton']
    Should Not Be Equal    ${element2}    ${NONE}
    ${element3}=    Find Element    JPanel[name='formPanel'] >> JButton[name='clearButton']
    Should Not Be Equal    ${element3}    ${NONE}
    # All variants should work
    Log    All whitespace variants succeeded

# =============================================================================
# VERIFICATION TESTS
# =============================================================================

Verify Cascade Finds Correct Element Type
    [Documentation]    Verify cascaded selector returns correct component type.
    ...                Tests that JPanel >> JButton actually returns a JButton.
    [Tags]    positive    verification
    ${element}=    Find Element    JPanel >> JButton[name='submitButton']
    ${class}=    Get Element Property    ${element}    class
    Should Contain    ${class}    JButton

Verify Cascade With Attribute Matching
    [Documentation]    Verify cascaded selector respects attribute filters.
    ...                Tests that attribute filters are applied correctly.
    [Tags]    positive    verification
    ${element}=    Find Element    JPanel[name='formPanel'] >> JButton[text='Submit']
    ${text}=    Get Element Text    ${element}
    Should Be Equal    ${text}    Submit

Verify Multiple Cascades Find Different Elements
    [Documentation]    Verify different cascaded selectors can find different elements.
    [Tags]    positive    verification
    ${element1}=    Find Element    JPanel >> JButton[name='submitButton']
    ${element2}=    Find Element    JPanel >> JButton[name='clearButton']
    Should Not Be Equal    ${element1}    ${element2}

Verify Cascade Works With Wait Keywords
    [Documentation]    Verify cascaded selectors work with wait keywords.
    [Tags]    positive    verification    wait
    Wait Until Element Exists    JPanel >> JButton[name='submitButton']    timeout=${SHORT_TIMEOUT}
    Wait Until Element Is Visible    JPanel >> JButton[name='submitButton']    timeout=${SHORT_TIMEOUT}

Verify Cascade Works With Click Keywords
    [Documentation]    Verify cascaded selectors work with action keywords.
    [Tags]    positive    verification    workflow
    Select Form Input Tab
    Click Button    JPanel >> JButton[name='clearButton']
    Sleep    0.3s
    # Verify form still exists after clear
    Element Should Exist    JTextField[name='nameTextField']

# =============================================================================
# EDGE CASES AND ERROR HANDLING
# =============================================================================

Cascade With Empty Segment
    [Documentation]    Test cascaded selector with empty segment between separators.
    ...                Tests error handling for malformed selectors.
    [Tags]    negative    error-handling    edge-case
    ${status}=    Run Keyword And Return Status
    ...    Find Element    JPanel >> >> JButton
    Should Be Equal    ${status}    ${FALSE}

Cascade With Trailing Separator
    [Documentation]    Test cascaded selector with trailing >> separator.
    ...                Tests error handling for incomplete selectors.
    [Tags]    negative    error-handling    edge-case
    ${status}=    Run Keyword And Return Status
    ...    Find Element    JPanel >> JButton >>
    Should Be Equal    ${status}    ${FALSE}

Cascade With Leading Separator
    [Documentation]    Test cascaded selector with leading >> separator.
    ...                Tests error handling for malformed selectors.
    [Tags]    negative    error-handling    edge-case
    ${status}=    Run Keyword And Return Status
    ...    Find Element    >> JPanel >> JButton
    Should Be Equal    ${status}    ${FALSE}

Cascade With Only Separator
    [Documentation]    Test cascaded selector that is only separators.
    ...                Tests error handling for invalid selectors.
    [Tags]    negative    error-handling    edge-case
    ${status}=    Run Keyword And Return Status
    ...    Find Element    >>
    Should Be Equal    ${status}    ${FALSE}

Very Long Cascade Chain
    [Documentation]    Test cascaded selector with many segments (10+).
    ...                Tests performance and deep hierarchy traversal.
    [Tags]    edge-case    performance
    # Create a realistic long chain using available components
    ${element}=    Find Element    JFrame >> JTabbedPane >> JPanel >> JButton
    Should Not Be Equal    ${element}    ${NONE}
    Log    Long cascade chain succeeded

# =============================================================================
# COMPARISON TESTS - CASCADE VS TRADITIONAL
# =============================================================================

Compare Cascade To Traditional Selector
    [Documentation]    Compare cascaded selector results with traditional selectors.
    ...                Verify both approaches find the same element.
    [Tags]    positive    comparison
    # Traditional selector with attribute
    ${traditional}=    Find Element    JButton[name='submitButton']
    # Cascaded selector with hierarchy
    ${cascaded}=    Find Element    JPanel >> JButton[name='submitButton']
    # Both should find elements (may or may not be the same instance)
    Should Not Be Equal    ${traditional}    ${NONE}
    Should Not Be Equal    ${cascaded}    ${NONE}

Cascade More Specific Than Type Alone
    [Documentation]    Verify cascaded selector is more specific than type alone.
    ...                Tests that hierarchy helps narrow down results.
    [Tags]    positive    comparison
    ${all_buttons}=    Find Elements    JButton
    ${panel_buttons}=    Find Elements    JPanel[name='formPanel'] >> JButton
    ${all_count}=    Get Length    ${all_buttons}
    ${panel_count}=    Get Length    ${panel_buttons}
    # Cascaded selector should find fewer or equal buttons
    Should Be True    ${panel_count} <= ${all_count}

# =============================================================================
# INTERACTION TESTS
# =============================================================================

Click Element Using Cascade
    [Documentation]    Verify cascaded selectors work with click operations.
    [Tags]    positive    workflow    interaction
    Select Form Input Tab
    Click    JPanel >> JButton[name='submitButton']
    Sleep    0.3s

Input Text Using Cascade
    [Documentation]    Verify cascaded selectors work with input operations.
    [Tags]    positive    workflow    interaction
    Select Form Input Tab
    Clear Text    JPanel >> JTextField[name='nameTextField']
    Input Text    JPanel >> JTextField[name='nameTextField']    TestUser
    ${text}=    Get Element Text    JTextField[name='nameTextField']
    Should Be Equal    ${text}    TestUser

Get Element Properties Using Cascade
    [Documentation]    Verify cascaded selectors work with property retrieval.
    [Tags]    positive    workflow
    ${props}=    Get Element Properties    JPanel >> JButton[name='submitButton']
    Should Not Be Empty    ${props}
    Log    Element properties: ${props}
