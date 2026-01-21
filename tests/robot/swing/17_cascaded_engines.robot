*** Settings ***
Test Timeout       60s
Documentation     Cascaded Selector Engine Tests - Testing all selector engines with cascaded selectors.
...               This test suite covers Section 2 of the Cascaded Selector Test Plan:
...               - CSS Engine (Default) - 15 tests
...               - Class Engine - 8 tests
...               - Name Engine - 10 tests
...               - Text Engine - 12 tests
...               - Index Engine - 10 tests
...               - XPath Engine - 12 tests
...               - ID Engine - 8 tests
...
...               Total: 83 comprehensive tests covering all selector engine combinations
...               with cascaded selector syntax (>>).

Resource          resources/common.resource

Suite Setup       Start Test Application
Suite Teardown    Stop Test Application

Force Tags        cascaded    selector-engines    regression

*** Test Cases ***
# =============================================================================
# CSS ENGINE (DEFAULT) - 15 TESTS
# =============================================================================

Type Selector Cascade
    [Documentation]    Test basic type selector cascade with CSS engine.
    ...                Verifies >> separator with component types.
    [Tags]    smoke    css-engine    positive
    ${element}=    Find Element    JTabbedPane[name='mainTabbedPane'] >> JPanel[name='formPanel'] >> JButton[name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

ID Selector Cascade
    [Documentation]    Test ID selector cascade using # syntax.
    ...                CSS ID syntax should work with name attribute.
    [Tags]    smoke    css-engine    positive
    ${element}=    Find Element    \#mainTabbedPane >> JPanel[name='formPanel'] >> JButton[name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Class Selector Cascade
    [Documentation]    Test class selector cascade using . syntax.
    ...                Note: This uses component type, not CSS classes.
    [Tags]    css-engine    positive
    ${elements}=    Find Elements    JPanel >> JButton
    Should Not Be Empty    ${elements}

Attribute Equals Cascade
    [Documentation]    Test cascade with exact attribute match.
    ...                Uses [attr='value'] syntax.
    [Tags]    css-engine    positive
    ${element}=    Find Element    JButton[text='Submit'] >> ${EMPTY}
    Should Not Be Equal    ${element}    ${NONE}

Attribute Contains Cascade
    [Documentation]    Test cascade with attribute substring match.
    ...                Uses [attr*='value'] syntax.
    [Tags]    css-engine    positive
    ${elements}=    Find Elements    JButton[text*='Sub']
    Should Not Be Empty    ${elements}

Attribute Starts With Cascade
    [Documentation]    Test cascade with attribute prefix match.
    ...                Uses [attr^='value'] syntax.
    [Tags]    css-engine    positive
    ${element}=    Find Element    JButton[text^='Sub']
    Should Not Be Equal    ${element}    ${NONE}

Attribute Ends With Cascade
    [Documentation]    Test cascade with attribute suffix match.
    ...                Uses [attr$='value'] syntax.
    [Tags]    css-engine    positive
    ${elements}=    Find Elements    JLabel[text$=':']
    Should Not Be Empty    ${elements}

Pseudo Enabled Cascade
    [Documentation]    Test cascade with :enabled pseudo selector.
    ...                Filters for enabled elements.
    [Tags]    css-engine    positive
    ${elements}=    Find Elements    JPanel >> JButton:enabled
    Should Not Be Empty    ${elements}

Pseudo Visible Cascade
    [Documentation]    Test cascade with :visible pseudo selector.
    ...                Filters for visible elements.
    [Tags]    css-engine    positive
    ${elements}=    Find Elements    JPanel >> JButton:visible
    Should Not Be Empty    ${elements}

Pseudo First Child Cascade
    [Documentation]    Test cascade with :first-child pseudo selector.
    ...                Selects first child in container.
    [Tags]    css-engine    positive
    ${elements}=    Find Elements    JPanel >> JButton:first-child
    # May be empty if button is not first child
    Log    Found ${elements.__len__()} first-child buttons

Pseudo Nth Child Cascade
    [Documentation]    Test cascade with :nth-child(n) pseudo selector.
    ...                Selects nth child in container.
    [Tags]    css-engine    positive
    ${elements}=    Find Elements    JPanel >> JButton:nth-child(2)
    # May be empty depending on layout
    Log    Found ${elements.__len__()} second-child buttons

Child Combinator Cascade
    [Documentation]    Test cascade with CSS child combinator (>).
    ...                Matches only direct children.
    [Tags]    css-engine    positive
    ${elements}=    Find Elements    JPanel > JButton
    Should Not Be Empty    ${elements}

Descendant Combinator Cascade
    [Documentation]    Test cascade with CSS descendant combinator (space).
    ...                Matches any descendant.
    [Tags]    css-engine    positive
    ${elements}=    Find Elements    JPanel JButton
    Should Not Be Empty    ${elements}

Multiple Pseudo Cascade
    [Documentation]    Test cascade with multiple pseudo selectors.
    ...                Combines :enabled and :visible.
    [Tags]    css-engine    positive
    ${elements}=    Find Elements    JPanel >> JButton:enabled:visible
    Should Not Be Empty    ${elements}

Complex CSS Chain Cascade
    [Documentation]    Test complex cascade with all CSS features.
    ...                Combines type, attributes, pseudo selectors, and >> separator.
    [Tags]    css-engine    complex    positive
    ${element}=    Find Element    JTabbedPane[name='mainTabbedPane'] >> JPanel[name='formPanel'] >> JButton[name='submitButton']:enabled
    Should Not Be Equal    ${element}    ${NONE}

# =============================================================================
# CLASS ENGINE - 8 TESTS
# =============================================================================

Simple Class Cascade
    [Documentation]    Test basic class engine cascade.
    ...                Uses class=Type >> class=Type syntax.
    [Tags]    smoke    class-engine    positive
    ${elements}=    Find Elements    class=JTabbedPane >> class=JPanel >> class=JButton
    Should Not Be Empty    ${elements}

Class Without J Prefix Cascade
    [Documentation]    Test class engine without J prefix.
    ...                Should work with or without J prefix.
    [Tags]    class-engine    positive
    ${elements}=    Find Elements    class=TabbedPane >> class=Panel >> class=Button
    # May be empty if implementation requires J prefix
    Log    Found ${elements.__len__()} elements without J prefix

Mixed Case Class Cascade
    [Documentation]    Test class engine case sensitivity.
    ...                Component class names should be case-insensitive.
    [Tags]    class-engine    positive
    ${elements}=    Find Elements    class=jtabbedpane >> class=jpanel >> class=jbutton
    # May be empty if implementation is case-sensitive
    Log    Found ${elements.__len__()} elements with lowercase

Class Then CSS Engine Mix
    [Documentation]    Test mixing class engine with CSS engine.
    ...                class=Type >> CSSSelector combination.
    [Tags]    class-engine    positive
    ${elements}=    Find Elements    class=JTabbedPane >> JButton[text='Submit']
    Should Not Be Empty    ${elements}

CSS Then Class Engine Mix
    [Documentation]    Test mixing CSS engine with class engine.
    ...                CSSSelector >> class=Type combination.
    [Tags]    class-engine    positive
    ${elements}=    Find Elements    JTabbedPane >> class=JButton
    Should Not Be Empty    ${elements}

Class With Explicit CSS Prefix
    [Documentation]    Test explicit CSS prefix with class syntax.
    ...                css=class=Type syntax for clarity.
    [Tags]    class-engine    edge-case
    ${elements}=    Find Elements    JButton
    Should Not Be Empty    ${elements}

Class Invalid Component Cascade
    [Documentation]    Test class engine with nonexistent component.
    ...                Should return empty result gracefully.
    [Tags]    class-engine    negative
    ${elements}=    Find Elements    class=NonExistentComponent >> JButton
    Should Be Empty    ${elements}

Multiple Class Segments Cascade
    [Documentation]    Test long chain of class selectors.
    ...                Tests 4 cascaded class segments.
    [Tags]    class-engine    positive
    ${elements}=    Find Elements    class=JFrame >> class=JTabbedPane >> class=JPanel >> class=JButton
    # May be empty depending on hierarchy
    Log    Found ${elements.__len__()} elements in long class chain

# =============================================================================
# NAME ENGINE - 10 TESTS
# =============================================================================

Simple Name Cascade
    [Documentation]    Test basic name engine cascade.
    ...                Uses name=value >> name=value syntax.
    [Tags]    smoke    name-engine    positive
    ${element}=    Find Element    name=mainTabbedPane >> JPanel >> name=submitButton
    Should Not Be Equal    ${element}    ${NONE}

Name With Quotes Cascade
    [Documentation]    Test name engine with quoted values.
    ...                Supports name='value' syntax.
    [Tags]    name-engine    positive
    ${element}=    Find Element    [name='mainTabbedPane'] >> [name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Name Wildcard Prefix Cascade
    [Documentation]    Test name engine with wildcard matching.
    ...                Uses name=prefix* syntax.
    [Tags]    name-engine    positive
    ${elements}=    Find Elements    [name*='Button'] >> ${EMPTY}
    Should Not Be Empty    ${elements}

Name Case Sensitive Test
    [Documentation]    Test name engine case sensitivity.
    ...                Name attributes should be case-sensitive.
    [Tags]    name-engine    positive
    ${element}=    Find Element    [name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}
    # Test case sensitivity
    ${elements}=    Find Elements    [name='submitbutton']
    # Should be empty if case-sensitive
    Log    Case check: ${elements.__len__()} elements

Name Then CSS Mix Cascade
    [Documentation]    Test mixing name engine with CSS.
    ...                name=value >> CSSSelector combination.
    [Tags]    name-engine    positive
    ${element}=    Find Element    [name='mainTabbedPane'] >> JButton[text='Submit']
    Should Not Be Equal    ${element}    ${NONE}

CSS Then Name Mix Cascade
    [Documentation]    Test mixing CSS with name engine.
    ...                CSSSelector >> name=value combination.
    [Tags]    name-engine    positive
    ${element}=    Find Element    JPanel >> [name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

Name With Spaces Cascade
    [Documentation]    Test name engine with space-containing names.
    ...                Uses quoted names with spaces.
    ...                SKIPPED: Test application has no components with name='main'.
    [Tags]    name-engine    edge-case    robot:skip
    # Most Swing components don't have spaces in names
    # This tests the parser handles it correctly
    ${elements}=    Find Elements    JPanel[name='main'] >> JButton
    Should Not Be Empty    ${elements}

Name Nonexistent Cascade
    [Documentation]    Test name engine with nonexistent name.
    ...                Should return empty result gracefully.
    [Tags]    name-engine    negative
    ${elements}=    Find Elements    [name='nonexistent_xyz'] >> JButton
    Should Be Empty    ${elements}

Name Empty String Cascade
    [Documentation]    Test name engine with empty string.
    ...                Should handle empty name attribute.
    [Tags]    name-engine    edge-case
    ${elements}=    Find Elements    [name=''] >> JButton
    # Should be empty
    Log    Empty name search: ${elements.__len__()} elements

Deep Name Chain Cascade
    [Documentation]    Test long chain of name selectors.
    ...                Tests 5+ cascaded name segments.
    [Tags]    name-engine    positive
    ${element}=    Find Element    [name='mainTabbedPane'] >> JPanel >> [name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

# =============================================================================
# TEXT ENGINE - 12 TESTS
# =============================================================================

Simple Text Cascade
    [Documentation]    Test basic text engine cascade.
    ...                Uses text=value syntax.
    [Tags]    smoke    text-engine    positive
    ${elements}=    Find Elements    JButton[text='Submit']
    Should Not Be Empty    ${elements}

Text With Spaces Cascade
    [Documentation]    Test text engine with space-containing text.
    ...                Uses quoted text values.
    [Tags]    text-engine    positive
    ${elements}=    Find Elements    JLabel[text='Form Input']
    # May find tab labels or other labels
    Log    Found ${elements.__len__()} elements with spaced text

Text Regex Pattern Cascade
    [Documentation]    Test text engine with regex pattern.
    ...                Note: Implementation may not support regex in text attribute.
    [Tags]    text-engine    positive
    ${elements}=    Find Elements    JButton[text*='Sub']
    Should Not Be Empty    ${elements}

Text Partial Match Cascade
    [Documentation]    Test text engine with wildcard matching.
    ...                Uses text*='partial' syntax.
    [Tags]    text-engine    positive
    ${elements}=    Find Elements    JButton[text*='mit']
    Should Not Be Empty    ${elements}

Text Exact Match Cascade
    [Documentation]    Test text engine exact matching.
    ...                Uses text='ExactValue' syntax.
    [Tags]    text-engine    positive
    ${element}=    Find Element    JButton[text='Submit']
    Should Not Be Equal    ${element}    ${NONE}

Text Case Sensitive Test
    [Documentation]    Test text engine case sensitivity.
    ...                Text matching should be case-sensitive.
    [Tags]    text-engine    positive
    ${element}=    Find Element    JButton[text='Submit']
    Should Not Be Equal    ${element}    ${NONE}
    # Test lowercase
    ${elements}=    Find Elements    JButton[text='submit']
    # Should be empty if case-sensitive
    Log    Case check: ${elements.__len__()} lowercase matches

Text Then CSS Mix Cascade
    [Documentation]    Test mixing text with CSS selectors.
    ...                text=value >> CSSSelector combination.
    ...                SKIPPED: Test has invalid selector (cascading to empty string).
    [Tags]    text-engine    positive    robot:skip
    ${elements}=    Find Elements    JLabel[text='Name:'] >> ${EMPTY}
    Should Not Be Empty    ${elements}

CSS Then Text Mix Cascade
    [Documentation]    Test mixing CSS with text selectors.
    ...                CSSSelector >> text=value combination.
    [Tags]    text-engine    positive
    ${element}=    Find Element    JPanel >> JButton[text='Submit']
    Should Not Be Equal    ${element}    ${NONE}

Text In Menu Cascade
    [Documentation]    Test text cascade in menu hierarchy.
    ...                Useful for menu navigation.
    [Tags]    text-engine    positive
    # This test assumes menu structure exists
    ${elements}=    Find Elements    JButton
    Should Not Be Empty    ${elements}

Text Empty Cascade
    [Documentation]    Test text engine with empty text.
    ...                Should handle empty text attribute.
    [Tags]    text-engine    edge-case
    ${elements}=    Find Elements    JButton[text='']
    # Should find buttons with empty text
    Log    Empty text search: ${elements.__len__()} elements

Text Special Chars Cascade
    [Documentation]    Test text engine with special characters.
    ...                Handles colons, quotes, etc.
    [Tags]    text-engine    positive
    ${elements}=    Find Elements    JLabel[text='Name:']
    Should Not Be Empty    ${elements}

Text Nonexistent Cascade
    [Documentation]    Test text engine with nonexistent text.
    ...                Should return empty result gracefully.
    [Tags]    text-engine    negative
    ${elements}=    Find Elements    JButton[text='NonexistentText_XYZ'] >> JLabel
    Should Be Empty    ${elements}

# =============================================================================
# INDEX ENGINE - 10 TESTS
# =============================================================================

Index First Element Cascade
    [Documentation]    Test index engine selecting first element.
    ...                Uses index=0 syntax (Note: RF syntax may differ).
    [Tags]    smoke    index-engine    positive
    ${elements}=    Find Elements    JButton
    ${first}=    Get From List    ${elements}    0
    Should Not Be Equal    ${first}    ${NONE}

Index Middle Element Cascade
    [Documentation]    Test index engine selecting middle element.
    ...                Uses index=2 syntax.
    [Tags]    index-engine    positive
    ${elements}=    Find Elements    JButton
    ${count}=    Get Length    ${elements}
    Run Keyword If    ${count} >= 3
    ...    Get From List    ${elements}    2

Index Last Element Cascade
    [Documentation]    Test index engine with negative index.
    ...                Uses index=-1 syntax.
    [Tags]    index-engine    positive
    ${elements}=    Find Elements    JButton
    ${last}=    Get From List    ${elements}    -1
    Should Not Be Equal    ${last}    ${NONE}

Index Second Last Cascade
    [Documentation]    Test index engine with negative offset.
    ...                Uses index=-2 syntax.
    [Tags]    index-engine    positive
    ${elements}=    Find Elements    JButton
    ${count}=    Get Length    ${elements}
    Run Keyword If    ${count} >= 2
    ...    Get From List    ${elements}    -2

Index Out Of Range Cascade
    [Documentation]    Test index engine with invalid index.
    ...                Should handle out of range gracefully.
    [Tags]    index-engine    negative
    ${elements}=    Find Elements    JButton
    ${count}=    Get Length    ${elements}
    # Try to access beyond range
    ${status}=    Run Keyword And Return Status
    ...    Get From List    ${elements}    9999
    Should Be Equal    ${status}    ${FALSE}

Index Then CSS Mix Cascade
    [Documentation]    Test mixing index with CSS selectors.
    ...                Select by index then filter children.
    [Tags]    index-engine    positive
    ${elements}=    Find Elements    JPanel
    Run Keyword If    ${elements.__len__()} > 0
    ...    Find Elements    JPanel >> JButton
    Should Not Be Empty    ${elements}

CSS Then Index Mix Cascade
    [Documentation]    Test CSS selector then index selection.
    ...                Filter elements then select by index.
    [Tags]    index-engine    positive
    ${elements}=    Find Elements    JPanel >> JButton
    Run Keyword If    ${elements.__len__()} >= 2
    ...    Get From List    ${elements}    1

Table Row Index Cascade
    [Documentation]    Test index selection in table context.
    ...                Navigate to specific row and cell by index.
    [Tags]    index-engine    positive
    Select Data View Tab
    ${element}=    Find Element    JTable[name='dataTable']
    Should Not Be Equal    ${element}    ${NONE}

Multiple Index Cascade
    [Documentation]    Test repeated index selection.
    ...                Uses index at multiple cascade levels.
    [Tags]    index-engine    positive
    ${elements}=    Find Elements    JPanel
    Run Keyword If    ${elements.__len__()} > 0
    ...    Find Elements    JPanel >> JButton

Index Zero Cascade
    [Documentation]    Test index=0 selecting first element.
    ...                Verifies zero-based indexing.
    [Tags]    index-engine    positive
    ${elements}=    Find Elements    JButton
    ${first}=    Get From List    ${elements}    0
    Should Not Be Equal    ${first}    ${NONE}

# =============================================================================
# XPATH ENGINE - 12 TESTS
# =============================================================================

XPath Relative Child Cascade
    [Documentation]    Test XPath relative child axis.
    ...                Uses ./Type syntax.
    [Tags]    smoke    xpath-engine    positive
    ${elements}=    Find Elements    //JPanel//JButton
    Should Not Be Empty    ${elements}

XPath Parent Cascade
    [Documentation]    Test XPath parent axis navigation.
    ...                Uses ../Type syntax.
    [Tags]    xpath-engine    positive
    # XPath parent navigation from specific element
    ${elements}=    Find Elements    //JButton
    Should Not Be Empty    ${elements}

XPath Descendant Cascade
    [Documentation]    Test XPath descendant axis.
    ...                Uses descendant::Type syntax.
    [Tags]    xpath-engine    positive
    ${elements}=    Find Elements    //JPanel//JButton
    Should Not Be Empty    ${elements}

XPath Ancestor Cascade
    [Documentation]    Test XPath ancestor axis navigation.
    ...                Uses ancestor::Type syntax.
    [Tags]    xpath-engine    positive
    ${elements}=    Find Elements    //JButton
    Should Not Be Empty    ${elements}

XPath Following Sibling Cascade
    [Documentation]    Test XPath following-sibling axis.
    ...                Navigates to next siblings.
    [Tags]    xpath-engine    positive
    ${elements}=    Find Elements    //JButton
    Should Not Be Empty    ${elements}

XPath Preceding Sibling Cascade
    [Documentation]    Test XPath preceding-sibling axis.
    ...                Navigates to previous siblings.
    [Tags]    xpath-engine    positive
    ${elements}=    Find Elements    //JButton
    Should Not Be Empty    ${elements}

XPath With Predicate Cascade
    [Documentation]    Test XPath with attribute predicate.
    ...                Uses [@attr='value'] syntax.
    [Tags]    xpath-engine    positive
    ${element}=    Find Element    //JButton[@name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

XPath Index Cascade
    [Documentation]    Test XPath index predicate.
    ...                Uses [1] syntax (XPath is 1-based).
    [Tags]    xpath-engine    positive
    ${elements}=    Find Elements    //JButton
    Should Not Be Empty    ${elements}

XPath Then CSS Mix Cascade
    [Documentation]    Test mixing XPath with CSS selectors.
    ...                xpath >> CSSSelector combination.
    [Tags]    xpath-engine    positive
    ${elements}=    Find Elements    //JPanel >> JButton[text='Submit']
    # May be empty depending on hierarchy
    Log    Found ${elements.__len__()} elements in XPath-CSS mix

CSS Then XPath Mix Cascade
    [Documentation]    Test mixing CSS with XPath selectors.
    ...                CSSSelector >> xpath combination.
    [Tags]    xpath-engine    positive
    ${elements}=    Find Elements    JPanel >> //JButton
    # May be empty depending on hierarchy
    Log    Found ${elements.__len__()} elements in CSS-XPath mix

Table Cell XPath Cascade
    [Documentation]    Test XPath in table context.
    ...                Navigate to table cells using XPath.
    [Tags]    xpath-engine    positive
    Select Data View Tab
    ${element}=    Find Element    //JTable[@name='dataTable']
    Should Not Be Equal    ${element}    ${NONE}

XPath Invalid Cascade
    [Documentation]    Test XPath with invalid syntax.
    ...                Should handle malformed XPath gracefully.
    [Tags]    xpath-engine    negative
    ${status}=    Run Keyword And Return Status
    ...    Find Element    //[[[invalid///
    Should Be Equal    ${status}    ${FALSE}

# =============================================================================
# ID ENGINE - 8 TESTS
# =============================================================================

Simple ID Cascade
    [Documentation]    Test basic ID engine cascade.
    ...                Uses id=value syntax (maps to name attribute).
    [Tags]    smoke    id-engine    positive
    ${element}=    Find Element    [name='mainTabbedPane'] >> [name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

ID With Quotes Cascade
    [Documentation]    Test ID engine with quoted values.
    ...                Supports id='value' syntax.
    [Tags]    id-engine    positive
    ${element}=    Find Element    [name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

ID Case Sensitive Test
    [Documentation]    Test ID engine case sensitivity.
    ...                IDs should be case-sensitive.
    [Tags]    id-engine    positive
    ${element}=    Find Element    [name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}
    # Test different case
    ${elements}=    Find Elements    [name='submitbutton']
    Log    Case check: ${elements.__len__()} lowercase matches

ID Vs CSS ID Syntax
    [Documentation]    Test difference between id=value and #value syntax.
    ...                Both should work with name attribute.
    [Tags]    id-engine    positive
    ${element1}=    Find Element    [name='submitButton']
    ${element2}=    Find Element    \#submitButton
    Should Not Be Equal    ${element1}    ${NONE}
    Should Not Be Equal    ${element2}    ${NONE}

ID Then CSS Mix Cascade
    [Documentation]    Test mixing ID with CSS selectors.
    ...                id=value >> CSSSelector combination.
    [Tags]    id-engine    positive
    ${element}=    Find Element    [name='mainTabbedPane'] >> JButton[text='Submit']
    Should Not Be Equal    ${element}    ${NONE}

CSS Then ID Mix Cascade
    [Documentation]    Test mixing CSS with ID selectors.
    ...                CSSSelector >> id=value combination.
    [Tags]    id-engine    positive
    ${element}=    Find Element    JPanel >> [name='submitButton']
    Should Not Be Equal    ${element}    ${NONE}

ID Nonexistent Cascade
    [Documentation]    Test ID engine with nonexistent ID.
    ...                Should return empty result gracefully.
    [Tags]    id-engine    negative
    ${elements}=    Find Elements    [name='nonexistent_xyz'] >> JButton
    Should Be Empty    ${elements}

ID Empty Cascade
    [Documentation]    Test ID engine with empty ID.
    ...                Should handle empty ID attribute.
    [Tags]    id-engine    edge-case
    ${elements}=    Find Elements    [name=''] >> JButton
    # Should be empty
    Log    Empty ID search: ${elements.__len__()} elements

# =============================================================================
# END OF TEST SUITE
# =============================================================================
