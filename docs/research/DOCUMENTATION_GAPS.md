# Documentation Gaps Analysis

**Analysis Date**: 2026-01-20
**Compared Against**: Robot Framework Browser Library
**Current Libraries**: JavaGui.Swing, JavaGui.Swt, JavaGui.Rcp

---

## Executive Summary

This document identifies specific gaps between our current keyword documentation and the Browser library standard, which is considered best-in-class for Robot Framework libraries.

---

## Gap Categories

### 1. Missing Return Type Documentation

**Impact**: HIGH
**Affected Keywords**: All getter keywords (~30 keywords)

**Current State**:
```python
def get_text(self, locator: str, ...) -> str:
    """Get element text with optional assertion.

    Example:
    | ${text}= | Get Text | JLabel#status |
    """
```

**Problem**: Users don't know:
- What type is returned
- What happens if element not found
- What value is returned on assertion failure
- Whether exceptions are raised

**Browser Library Example**:
```python
"""
= Return Value =

Returns ``str``: The text content of the element.

- Returns immediately without assertion
- Retries and returns matched value with assertion
- Raises ``AssertionError`` if assertion fails
"""
```

**Required Fix**:
- Add "= Return Value =" section to all getters
- Document return type explicitly
- Document exception conditions
- Document return behavior with/without assertions

**Affected Keywords**:
- `Get Text` / `Get Element Text`
- `Get Value` / `Get Element Value`
- `Get Count` / `Get Element Count`
- `Get Selected Value` / `Get Selected Index`
- `Get Table Cell Value` / `Get Table Row Count`
- `Get Tree Node Text` / `Get Tree Node Count`
- All SWT/RCP equivalents (~30 total)

---

### 2. Old-Style Robot Framework Syntax in Examples

**Impact**: MEDIUM-HIGH
**Affected Keywords**: All keywords with examples (~100+ keywords)

**Current State**:
```robot
| ${text}= | Get Text | JLabel#status | | |
| Get Text | JLabel#status | == | Ready | |
```

**Problems**:
- Uses deprecated pipe separators
- Has trailing empty cells
- Hard to read and maintain
- Doesn't match modern RF best practices
- Confusing for new users learning RF 7.x

**Browser Library Example**:
```robot
${text}=    Get Text    JLabel#status
Get Text    JLabel#status    ==    Ready
```

**Required Fix**:
- Remove all pipe separators (`|`)
- Use 4-space indentation between arguments
- Remove trailing empty cells
- Use modern variable assignment syntax
- Update all examples across all three libraries

**Scope**:
- Swing: ~40 keywords with examples
- SWT: ~30 keywords with examples
- RCP: ~35 keywords with examples
- **Total**: ~105 keyword examples to update

---

### 3. Missing Tags/Categories

**Impact**: MEDIUM
**Affected Keywords**: All keywords (~100+ keywords)

**Current State**:
No tags present in any keyword documentation.

**Problem**:
- Users can't browse by category
- No filtering in generated HTML docs
- Hard to find related functionality
- Poor organization for large libraries

**Browser Library Example**:
```python
"""
= Tags =

PageContent, Assertion, Getter
"""
```

**Required Fix**:

Define tag taxonomy:

**Action Tags**:
- `Action` - All action keywords
- `Click` - Click-related actions
- `Input` - Text input actions
- `Selection` - Selection/choice actions

**Getter Tags**:
- `Getter` - All getter keywords
- `Assertion` - Keywords supporting assertions
- `Verification` - Old-style verification keywords

**Component Tags**:
- `Button` - Button-specific keywords
- `Table` - Table operations
- `Tree` - Tree operations
- `Menu` - Menu operations
- `Dialog` - Dialog handling
- `Text` - Text components
- `List` - List/combo components

**Framework Tags** (RCP):
- `Perspective` - Perspective management
- `View` - View operations
- `Editor` - Editor operations
- `Preference` - Preference handling

**Wait Tags**:
- `Wait` - All wait keywords
- `Synchronization` - Timing/sync keywords

**Utility Tags**:
- `Utility` - Helper keywords
- `Debugging` - Debug/troubleshooting
- `Screenshot` - Screenshot keywords
- `Logging` - Logging keywords

**Scope**: Add 2-4 tags to each of ~100+ keywords

---

### 4. Missing Cross-References

**Impact**: MEDIUM
**Affected Keywords**: Most keywords lack cross-references

**Current State**:
No "See also" or related keyword references.

**Problem**:
- Users don't discover alternative approaches
- Migration path from old to new keywords unclear
- Related functionality not connected
- Poor discoverability

**Browser Library Example**:
```python
"""
= Related Keywords =

See also `Get Attribute` for element attributes,
`Get Property` for JavaScript properties,
`Element Should Contain` for old-style assertions.
"""
```

**Required Fix**:

Map related keywords:

**Getter/Action Pairs**:
- `Get Text` ↔ `Element Text Should Be`
- `Get Value` ↔ `Element Value Should Be`
- `Get Count` ↔ `Element Count Should Be`

**Old/New Migration**:
- `Element Text Should Be` → `Get Text` with `==`
- `Element Should Be Visible` → `Get Element States` with assertion
- `Wait Until Element Is Visible` → `Get Element States` with timeout

**Specialized/General**:
- `Click Button` → `Click Element`
- `Input Text` ↔ `Type Text` ↔ `Clear Text`
- `Select From List` ↔ `Get Selected Value`

**Component Families**:
- Table keywords: `Get Table Cell Value` ↔ `Set Table Cell Value` ↔ `Get Table Row Count`
- Tree keywords: `Get Tree Node Text` ↔ `Select Tree Node` ↔ `Expand Tree Node`
- Menu keywords: `Select Menu Item` ↔ `Menu Should Be Enabled` ↔ `Get Menu Items`

**Scope**: Add "= Related Keywords =" to ~80 keywords

---

### 5. Missing Behavior Documentation

**Impact**: HIGH
**Affected Keywords**: Most complex keywords (~60 keywords)

**Current State**:
Behavior information scattered in description paragraphs, not structured.

**Problem**:
- Retry behavior not clearly documented
- Timeout semantics unclear
- Element waiting strategy not explained
- Strict vs non-strict matching not mentioned
- Performance implications unknown

**Browser Library Example**:
```python
"""
= Behavior =

* **Retry Logic**: Retries every 100ms until timeout when assertion provided
* **Element Wait**: Waits for element existence before retrieval (up to 5s)
* **Strict Mode**: Uses strict element matching (fails if multiple matches)
* **Performance**: Non-blocking; continues test execution during retries
"""
```

**Required Fix**:

Add "= Behavior =" section documenting:

**For Assertion Keywords**:
- Retry interval (100ms default)
- Timeout behavior
- What triggers retry (element not found, value mismatch, both)
- Whether element must exist before assertion

**For Wait Keywords**:
- Polling interval
- What condition is checked
- Whether timeout is failure or returns false
- Implicit vs explicit waits

**For Action Keywords**:
- Element readiness checks (enabled, visible)
- Error conditions (element not found, not enabled, not visible)
- Side effects (focus changes, events triggered)
- Thread safety

**For Getter Keywords**:
- Caching behavior
- Stale element handling
- What triggers element re-lookup
- Performance considerations for loops

**Scope**: ~60 keywords need behavior section

---

### 6. Missing Version/Deprecation Information

**Impact**: MEDIUM
**Affected Keywords**: ~20 deprecated/changed keywords

**Current State**:
No version history in any keyword.

**Problem**:
- Users don't know which keywords are deprecated
- Migration path unclear
- Breaking changes not documented
- Feature introduction not tracked

**Browser Library Example**:
```python
"""
= Version History =

New in version 3.0.0
- Added assertion operator support
- Added formatters parameter

Deprecated in version 3.0.0
- Use `Get Text` with assertion operators instead of `Element Text Should Be`
- Old-style verification will be removed in 4.0.0
"""
```

**Required Fix**:

**Newly Deprecated** (use assertions instead):
- `Element Text Should Be` → `Get Text` with `==`
- `Element Text Should Contain` → `Get Text` with `contains`
- `Element Value Should Be` → `Get Value` with `==`
- `Element Should Be Visible` → `Get Element States` with assertion
- `Element Should Be Enabled` → `Get Element States` with assertion
- (~15 total)

**New in 3.0.0**:
- All getter keywords with assertion support
- Assertion engine integration
- Formatter support
- Unified architecture
- (~30 total)

**Scope**: ~50 keywords need version information

---

### 7. Incomplete Assertion Operator Documentation

**Impact**: HIGH
**Affected Keywords**: All assertion-enabled getters (~30 keywords)

**Current State**:
```python
"""
Supported operators: ==, !=, <, >, <=, >=, contains, not contains,
starts, ends, matches, validate, then
"""
```

**Problem**:
- Operators listed but not explained
- No examples for each operator
- Advanced operators (validate, then) not documented
- Type compatibility not mentioned

**Browser Library Example**:
```python
"""
= Assertion Operators =

| =Operator= | =Description= | =Example= |
| ``==`` | Exact equality | Get Text | locator | == | Expected Text |
| ``!=`` | Not equal | Get Text | locator | != | Wrong Text |
| ``>`` | Greater than (numeric) | Get Count | locator | > | 0 |
| ``<`` | Less than (numeric) | Get Count | locator | < | 10 |
| ``contains`` | Substring present | Get Text | locator | contains | Success |
| ``matches`` | Regex match | Get Text | locator | matches | \\d{3}-\\d{4} |
| ``validate`` | Python expression | Get Text | locator | validate | value.startswith('OK') |
| ``then`` | Chained condition | Get Text | locator | then | value == 'A' or value == 'B' |
"""
```

**Required Fix**:

Add complete operator table to all assertion keywords:

**Comparison Operators**:
- `==` - Exact equality (any type)
- `!=` - Not equal (any type)
- `<`, `>`, `<=`, `>=` - Numeric/string comparison

**String Operators**:
- `contains` - Substring present
- `not contains` - Substring absent
- `starts` - Starts with prefix
- `ends` - Ends with suffix

**Pattern Operators**:
- `matches` - Regular expression match
- `validate` - Python expression (advanced)
- `then` - Chained assertions (advanced)

**List/Collection Operators** (for multi-value getters):
- `*in` - Item in collection
- `*not in` - Item not in collection

**Examples for Each**:
Include real, runnable examples for every operator.

**Scope**: 30 assertion-enabled keywords need complete operator tables

---

### 8. Missing Formatter Documentation

**Impact**: MEDIUM
**Affected Keywords**: Keywords supporting formatters (~15 keywords)

**Current State**:
```python
"""
| ``formatters`` | List of formatters: normalize_spaces, strip, lowercase, uppercase. |
"""
```

**Problem**:
- Formatters mentioned but not explained
- No examples of formatter usage
- Formatter chaining not documented
- Order of application unclear

**Browser Library Example**:
```python
"""
= Formatters =

Formatters transform the retrieved value before assertion:

| =Formatter= | =Description= | =Example Result= |
| ``strip`` | Remove leading/trailing whitespace | "  text  " → "text" |
| ``lowercase`` | Convert to lowercase | "TEXT" → "text" |
| ``uppercase`` | Convert to uppercase | "text" → "TEXT" |
| ``normalize_spaces`` | Collapse multiple spaces | "a    b" → "a b" |

Formatters are applied in order:
| Get Text | locator | == | hello world
| ...    formatters=['strip', 'lowercase']
"""
```

**Required Fix**:

Add formatter documentation to relevant keywords:

**Available Formatters**:
1. `strip` - Remove whitespace
2. `lowercase` - To lowercase
3. `uppercase` - To uppercase
4. `normalize_spaces` - Collapse spaces
5. (Future: `remove_html`, `decode_entities`, `trim_lines`)

**Document**:
- What each formatter does
- Order of application (left to right)
- How to chain formatters
- Use cases for each

**Scope**: ~15 keywords with formatter support

---

### 9. Poor Example Organization

**Impact**: MEDIUM
**Affected Keywords**: Most keywords (~80 keywords)

**Current State**:
Examples mixed together without clear grouping.

**Problem**:
- Hard to find relevant example
- Progressive complexity not shown
- Edge cases mixed with basic usage
- No clear learning path

**Browser Library Example**:
```python
"""
= Examples =

Basic usage without assertion:
| ${text}=    Get Text    selector
| Should Be Equal    ${text}    expected

With inline assertion (recommended):
| Get Text    selector    ==    expected

Using different operators:
| Get Text    selector    contains    substring
| Get Text    selector    matches    \\d+
| Get Text    selector    >    0

Advanced usage with timeout and formatters:
| Get Text    selector    ==    EXPECTED
| ...    timeout=10.0
| ...    formatters=['lowercase']
| ...    message=Custom error message
"""
```

**Required Fix**:

Organize examples by complexity level:

**Level 1: Basic**
- Simple usage
- No optional parameters
- Variable assignment

**Level 2: Intermediate**
- With assertions
- Common optional parameters
- Real-world scenarios

**Level 3: Advanced**
- Multiple optional parameters
- Complex assertions (regex, validate)
- Timeout handling
- Error scenarios

**Scope**: ~80 keywords need example reorganization

---

### 10. Missing Type System Documentation

**Impact**: MEDIUM
**Affected**: Library-level documentation

**Current State**:
Custom types not documented in library introduction.

**Problem**:
- Users don't understand `AssertionOperator` type
- `ElementState` dictionary structure unclear
- Locator syntax not thoroughly documented
- Type constraints not explained

**Browser Library Example**:
```python
"""
= Custom Types =

== AssertionOperator ==

String enum for assertion operators:
- Comparison: ``==``, ``!=``, ``<``, ``>``, ``<=``, ``>=``
- String: ``contains``, ``not contains``, ``starts``, ``ends``
- Pattern: ``matches`` (regex)
- Advanced: ``validate`` (Python), ``then`` (chained)

== ElementState ==

Dictionary with element state:
{
    "enabled": bool,
    "visible": bool,
    "focused": bool,
    "selected": bool  # for checkboxes/radios
}
"""
```

**Required Fix**:

Add to library introduction:

**1. AssertionOperator Type**
- All valid values
- Grouped by category
- Type compatibility notes

**2. ElementState Type**
- Dictionary structure
- All possible keys
- Value meanings

**3. Locator Syntax**
- Complete syntax reference
- All selector types
- Precedence rules
- Complex selectors

**4. Component Types**
- Valid Swing component types
- Valid SWT widget types
- Valid RCP element types

**Scope**: Library-level documentation enhancement

---

## Priority Matrix

| Gap | Impact | Effort | Priority | Keywords Affected |
|-----|--------|--------|----------|-------------------|
| Return Type Docs | HIGH | LOW | **P1** | ~30 |
| Behavior Section | HIGH | MEDIUM | **P1** | ~60 |
| Assertion Operators | HIGH | LOW | **P1** | ~30 |
| Example Syntax | MEDIUM | LOW | **P2** | ~100+ |
| Tags/Categories | MEDIUM | LOW | **P2** | ~100+ |
| Cross-References | MEDIUM | MEDIUM | **P2** | ~80 |
| Example Organization | MEDIUM | MEDIUM | **P3** | ~80 |
| Version History | MEDIUM | LOW | **P3** | ~50 |
| Formatter Docs | MEDIUM | LOW | **P3** | ~15 |
| Type System | MEDIUM | MEDIUM | **P3** | Library-level |

---

## Recommended Action Plan

### Phase 1: Critical Fixes (Week 1-2)
**Focus**: Assertion-enabled getters

Fix for ~30 getter keywords:
1. Add Return Value section
2. Add Behavior section with retry/timeout info
3. Add complete Assertion Operators table
4. Update examples to modern syntax

**Keywords**: Get Text, Get Value, Get Count, Get Element States, etc.

### Phase 2: Example Updates (Week 3)
**Focus**: All keywords

1. Update all examples to modern RF syntax
2. Add example grouping (basic/intermediate/advanced)
3. Ensure all examples are runnable

**Keywords**: All ~100+ keywords

### Phase 3: Metadata (Week 4)
**Focus**: Discoverability

1. Add tags to all keywords
2. Add cross-references to related keywords
3. Add version history where applicable

**Keywords**: All ~100+ keywords

### Phase 4: Advanced Documentation (Week 5-6)
**Focus**: Edge cases and advanced usage

1. Add formatter documentation
2. Add type system documentation
3. Enhance behavior sections for complex keywords
4. Add troubleshooting notes

---

## Measurement Criteria

### Success Metrics

**Completeness**:
- ✅ 100% of getters have Return Value section
- ✅ 100% of keywords have modern RF examples
- ✅ 100% of keywords have tags
- ✅ 80% of keywords have cross-references

**Quality**:
- ✅ All examples are runnable
- ✅ All assertion operators documented with examples
- ✅ All custom types documented
- ✅ No broken cross-references

**Usability**:
- ✅ User can find keywords by category (tags)
- ✅ User can discover related functionality (cross-refs)
- ✅ User understands return types and exceptions
- ✅ User knows when keyword was added/deprecated

---

## Tools and Automation

### Validation Script

```python
def validate_docstring(keyword_func):
    """Validate docstring completeness."""
    doc = keyword_func.__doc__

    checks = {
        'has_brief': bool(doc and len(doc.split('\n')[0]) < 100),
        'has_args_table': '| =Argument= |' in doc,
        'has_return_value': '= Return Value =' in doc if is_getter(keyword_func) else True,
        'has_behavior': '= Behavior =' in doc,
        'has_examples': '= Examples =' in doc,
        'modern_syntax': '|' not in extract_examples(doc),
        'has_tags': '= Tags =' in doc,
        'has_related': '= Related Keywords =' in doc,
    }

    return checks
```

### Documentation Generator

```python
def generate_getter_docstring(
    keyword_name: str,
    brief: str,
    extended: str,
    return_type: str,
    return_desc: str,
    tags: List[str],
    related: List[str],
) -> str:
    """Generate complete docstring from template."""
    # Use specification templates
    pass
```

---

## Resources

- **Full Specification**: `DOCSTRING_SPECIFICATION.md`
- **Quick Reference**: `DOCSTRING_ANALYSIS_SUMMARY.md`
- **Memory Namespace**: `docstring-patterns`
- **Browser Library**: https://marketsquare.github.io/robotframework-browser/Browser.html

---

**Status**: Ready for Implementation
**Next Action**: Begin Phase 1 with pilot keywords
**Estimated Timeline**: 6 weeks for complete migration
