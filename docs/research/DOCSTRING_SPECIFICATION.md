# Docstring Specification for Robot Framework JavaGui Library

## Executive Summary

This specification defines the target format for docstrings in the JavaGui library (Swing, SWT, RCP) to align with modern Robot Framework best practices as exemplified by the Browser library.

**Analysis Date**: 2026-01-20
**Target**: Robot Framework 7.4+
**Reference**: Browser Library Documentation Structure

---

## 1. Current State Analysis

### 1.1 Existing Format
Our current docstrings follow this pattern:

```python
def get_text(
    self,
    locator: str,
    assertion_operator: Optional[AssertionOperator] = None,
    expected: Any = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None,
    formatters: Optional[List[str]] = None,
) -> str:
    """Get element text with optional assertion.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |
    | ``assertion_operator`` | Optional assertion operator (==, !=, contains, etc.). |
    | ``expected`` | Expected value when using assertion operator. |
    | ``message`` | Custom error message on assertion failure. |
    | ``timeout`` | Assertion retry timeout in seconds. Default from library config. |
    | ``formatters`` | List of formatters: normalize_spaces, strip, lowercase, uppercase. |

    Without assertion operator, returns the element text directly.
    With assertion operator, retries until assertion passes or timeout.

    Example:
    | ${text}= | Get Text | JLabel#status | | |
    | Get Text | JLabel#status | == | Ready | |
    | Get Text | JLabel#status | contains | Success | timeout=10 |
    """
```

### 1.2 Strengths
✅ Consistent argument table format
✅ Multiple examples showing different use cases
✅ Clear locator syntax references
✅ Assertion operator documentation

### 1.3 Gaps Identified
❌ **No return type documentation** - Users don't know what type is returned
❌ **Old-style RF syntax in examples** - Uses `|` separators instead of modern spacing
❌ **Missing tags/categories** - No keyword grouping in documentation
❌ **No cross-references** - Related keywords not linked
❌ **No version/deprecation info** - Migration path unclear
❌ **Limited usage context** - When/why to use this keyword not explained
❌ **No performance notes** - Behavior differences not documented

---

## 2. Browser Library Analysis

### 2.1 Documentation Structure
The Browser library uses this comprehensive structure:

```
Keyword Documentation
├── Name and Signature
├── Brief Description (1-2 sentences)
├── Arguments Table
│   ├── Parameter name
│   ├── Type (with links to custom types)
│   ├── Default value
│   └── Description
├── Detailed Explanation
│   ├── Behavior notes
│   ├── Actionability checks
│   └── Mode information (strict/non-strict)
├── Multiple Examples
│   ├── Basic usage
│   ├── Advanced usage
│   └── Edge cases
├── Return Type (explicit)
├── Tags/Categories
└── Related Keywords (cross-references)
```

### 2.2 Key Features

#### 2.2.1 Rich Type Information
- Type annotations in signature
- Custom types linked in docs (e.g., `AssertionOperator`, `SelectAttribute`)
- Union types clearly documented
- Optional vs required parameters clearly marked

#### 2.2.2 Contextual Documentation
- **When to use**: Explains appropriate use cases
- **Behavior notes**: "Keyword uses strict mode" - documents search behavior
- **Performance implications**: Notes about retry behavior, timeouts
- **Related keywords**: Links to alternative approaches

#### 2.2.3 Example Quality
```robot
# Modern Robot Framework syntax (no pipes)
Get Text    selector    ==    expected value
Get Text    selector    contains    substring    timeout=10s

# With variable assignment
${text}=    Get Text    selector
Should Be Equal    ${text}    expected
```

#### 2.2.4 Semantic Markup
- Code references: `` `selector` ``
- Keyword links: Links to related keywords
- Type references: Links to type definitions
- External links: Playwright docs for actionability

---

## 3. Target Docstring Format

### 3.1 Complete Template

```python
def keyword_name(
    self,
    required_arg: str,
    optional_arg: Optional[str] = None,
    assertion_operator: Optional[AssertionOperator] = None,
    expected: Any = None,
    timeout: Optional[float] = None,
) -> ReturnType:
    """Brief one-line description of what the keyword does.

    Extended description providing context about when and why to use this keyword.
    This section can span multiple paragraphs to explain behavior, modes, and
    important considerations.

    = Arguments =

    | =Argument= | =Type= | =Description= |
    | ``required_arg`` | str | Description of required argument. See `Locator Syntax` for details. |
    | ``optional_arg`` | str | Optional description. Defaults to ``None``. |
    | ``assertion_operator`` | AssertionOperator | Optional assertion operator. See `Assertion Operators`. |
    | ``expected`` | Any | Expected value when using assertion. Type depends on operator. |
    | ``timeout`` | float | Retry timeout in seconds. Defaults to library timeout (5.0s). |

    = Return Value =

    Returns ``ReturnType``: Description of what is returned and when.

    - Without assertion: Returns the raw value
    - With assertion: Returns the value after assertion passes
    - Raises ``AssertionError`` if assertion fails after timeout

    = Behavior =

    * **Retry Logic**: Retries every 100ms until timeout when assertion provided
    * **Element Wait**: Waits for element existence before retrieval
    * **Strict Mode**: Uses strict element matching (fails if multiple matches)

    = Examples =

    Basic usage without assertion:
    | ${value}=    Keyword Name    JLabel#status
    | Should Be Equal    ${value}    Ready

    With inline assertion (recommended):
    | Keyword Name    JLabel#status    ==    Ready

    Using assertion operators:
    | Keyword Name    JLabel#count    >    0
    | Keyword Name    JLabel#message    contains    Success
    | Keyword Name    JLabel#email    matches    \\S+@\\S+\\.\\S+

    With timeout for slow updates:
    | Keyword Name    JLabel#loading    ==    Complete    timeout=10.0

    = Related Keywords =

    See also `Other Keyword`, `Alternative Keyword` for different approaches.

    = Tags =

    Getter, Assertion, Verification

    = Version History =

    New in version 3.0.0
    - Added assertion operator support
    - Deprecated: Old-style ``Verify Text`` (use assertion operators instead)
    """
```

### 3.2 Section Breakdown

#### 3.2.1 Brief Description
- **First sentence**: One-line summary for quick scanning
- **Extended description**: 1-3 paragraphs providing context
- Explain **when** to use this vs alternatives
- Note any **important behaviors** upfront

#### 3.2.2 Arguments Table
```
| =Argument= | =Type= | =Description= |
| ``arg_name`` | type | Full description with defaults and references |
```

**Requirements**:
- Use `` `backticks` `` for code/argument names
- Include type information (even if in signature)
- Reference other sections: `Locator Syntax`, `Assertion Operators`
- State defaults explicitly: "Defaults to ``None``"
- Link to type definitions for custom types

#### 3.2.3 Return Value Section
```
= Return Value =

Returns ``str``: Description of return value behavior.

- Condition 1: What is returned
- Condition 2: Alternative return
- Raises: Exception types and conditions
```

**Critical for**:
- Getter keywords (what type/format is returned)
- Keywords with conditional returns
- Keywords that may raise exceptions

#### 3.2.4 Behavior Section
```
= Behavior =

* **Feature 1**: Explanation
* **Feature 2**: Explanation
* **Important Note**: Warning or consideration
```

**Document**:
- Retry/timeout behavior
- Element waiting strategies
- Strict vs non-strict matching
- Performance characteristics
- Side effects

#### 3.2.5 Examples Section
```
= Examples =

Basic usage description:
| Keyword Name    arg1    arg2

Advanced usage description:
| ${var}=    Keyword Name    arg1    key=value
```

**Guidelines**:
- Use **modern RF syntax** (spaces, not `|` pipes)
- Provide **3-5 examples** showing:
  - Basic usage
  - With variable assignment
  - Using optional parameters
  - Edge cases
- Add brief descriptions before example groups
- Show **real-world scenarios**

#### 3.2.6 Related Keywords
```
= Related Keywords =

See also `Alternative Keyword` for different approach,
`Complementary Keyword` for related functionality.
```

#### 3.2.7 Tags
```
= Tags =

Category1, Category2, Feature
```

**Recommended categories**:
- Action keywords: `Action`, `Click`, `Input`
- Getter keywords: `Getter`, `Assertion`, `Verification`
- Wait keywords: `Wait`, `Synchronization`
- Utility keywords: `Utility`, `Helper`
- Feature areas: `Table`, `Tree`, `Menu`, `Dialog`

#### 3.2.8 Version History
```
= Version History =

New in version X.Y.Z
- Feature added
- Behavior changed

Deprecated in version X.Y.Z
- Use `New Keyword` instead
```

---

## 4. Assertion Keywords Pattern

Assertion-enabled getter keywords require special documentation:

```python
def get_something(
    self,
    locator: str,
    assertion_operator: Optional[AssertionOperator] = None,
    expected: Any = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None,
) -> ValueType:
    """Get something with optional inline assertion.

    This keyword follows the Browser library pattern where getter keywords
    can optionally include assertion logic for more concise test code.

    = Arguments =

    | =Argument= | =Type= | =Description= |
    | ``locator`` | str | Element locator. See `Locator Syntax`. |
    | ``assertion_operator`` | AssertionOperator | Optional operator: ==, !=, <, >, <=, >=, contains, not contains, starts, ends, matches, validate, then. See `Assertion Operators`. |
    | ``expected`` | Any | Expected value when using assertion operator. Type depends on operator. |
    | ``message`` | str | Custom error message on assertion failure. Defaults to auto-generated message. |
    | ``timeout`` | float | Assertion retry timeout in seconds. Defaults to library timeout. |

    = Return Value =

    Returns ``ValueType``: The retrieved value.

    - **Without assertion**: Returns immediately after retrieval
    - **With assertion**: Retries until assertion passes or timeout
    - **Raises**: ``AssertionError`` with message if assertion fails

    = Assertion Operators =

    | =Operator= | =Description= | =Example= |
    | ``==`` | Equal to | Get Something | locator | == | expected |
    | ``!=`` | Not equal to | Get Something | locator | != | unexpected |
    | ``<``, ``>`` | Numeric comparison | Get Count | locator | > | 0 |
    | ``contains`` | Substring/item present | Get Text | locator | contains | Success |
    | ``not contains`` | Substring/item absent | Get Text | locator | not contains | Error |
    | ``starts`` | Starts with | Get Text | locator | starts | Welcome |
    | ``ends`` | Ends with | Get Text | locator | ends | ! |
    | ``matches`` | Regex match | Get Text | locator | matches | \\d{3}-\\d{4} |
    | ``validate`` | Custom validator | Get Text | locator | validate | value.startswith('OK') |
    | ``then`` | Chained assertions | Get Text | locator | then | value == 'OK' or value == 'Ready' |

    = Examples =

    Without assertion (explicit verification):
    | ${text}=    Get Something    JLabel#status
    | Should Be Equal    ${text}    Ready

    With inline assertion (recommended):
    | Get Something    JLabel#status    ==    Ready

    Various operators:
    | Get Something    JLabel#count    >    0
    | Get Something    JLabel#message    contains    Success
    | Get Something    JLabel#email    matches    \\S+@\\S+
    | Get Something    JLabel#status    !=    Error

    With custom message and timeout:
    | Get Something    JLabel#loading    ==    Complete
    | ...    message=Loading did not complete
    | ...    timeout=10.0

    = Related Keywords =

    See `Element Should Contain` for old-style assertion keywords.
    """
```

---

## 5. Implementation Guidelines

### 5.1 Migration Strategy

1. **Phase 1: Core Keywords** (Highest Priority)
   - Get keywords with assertions (Get Text, Get Value, Get Count, etc.)
   - Action keywords (Click, Input Text, Select, etc.)
   - Wait keywords

2. **Phase 2: Specialized Keywords**
   - Table keywords
   - Tree keywords
   - Menu keywords
   - Dialog keywords

3. **Phase 3: Utility Keywords**
   - Screenshot keywords
   - Logging keywords
   - Configuration keywords

### 5.2 Automation Support

Use templates for consistency:

```python
# Template for getter with assertion
GETTER_ASSERTION_TEMPLATE = '''"""Get {what} with optional inline assertion.

= Arguments =

| =Argument= | =Type= | =Description= |
| ``locator`` | str | Element locator. See `Locator Syntax`. |
| ``assertion_operator`` | AssertionOperator | Optional operator. See `Assertion Operators`. |
| ``expected`` | Any | Expected value for assertion. |
| ``message`` | str | Custom error message. |
| ``timeout`` | float | Retry timeout in seconds. |

= Return Value =

Returns ``{return_type}``: {return_description}

= Examples =

| ${{value}}=    {keyword_name}    JLabel#status
| {keyword_name}    JLabel#status    ==    {example_value}

= Tags =

Getter, Assertion
"""'''
```

### 5.3 Quality Checklist

For each keyword docstring, verify:

- [ ] Brief description (1 sentence)
- [ ] Extended description (when/why to use)
- [ ] Complete arguments table with types
- [ ] Return value section (for getters)
- [ ] Behavior notes (retry, waiting, strict mode)
- [ ] 3-5 examples with modern RF syntax
- [ ] Related keywords section
- [ ] Tags for categorization
- [ ] Version/deprecation info (if applicable)
- [ ] Assertion operators table (for assertion keywords)
- [ ] Cross-references use backticks and keyword names

---

## 6. Type Reference Documentation

Document custom types in library introduction:

```python
"""
= Custom Types =

== AssertionOperator ==

String enum for assertion operators:
- Comparison: ``==``, ``!=``, ``<``, ``>``, ``<=``, ``>=``
- String: ``contains``, ``not contains``, ``starts``, ``ends``
- Pattern: ``matches`` (regex)
- Advanced: ``validate`` (Python expression), ``then`` (chained)

== ElementState ==

Dictionary with element state information:
- ``enabled``: bool - Element is enabled
- ``visible``: bool - Element is visible
- ``selected``: bool - Element is selected (checkboxes, radio buttons)
- ``focused``: bool - Element has focus

== Locator Syntax ==

CSS-like selectors for element location:

| =Syntax= | =Description= | =Example= |
| Type | Match by component class | ``JButton``, ``JLabel``, ``JTextField`` |
| #name | Match by element name/ID | ``#submitButton`` |
| [attr=value] | Match by attribute | ``[text='OK']``, ``[enabled=true]`` |
| Descendant | Match nested elements | ``JPanel JButton`` |
| Multiple | Multiple conditions | ``JButton#submit[enabled=true]`` |

Examples:
| Click Button    JButton#submit
| Get Text    JLabel[name='statusLabel']
| Input Text    JPanel#loginForm JTextField#username    admin
"""
```

---

## 7. Documentation Generation

### 7.1 Libdoc Command
```bash
python -m robot.libdoc JavaGui.SwingLibrary docs/keywords/Swing.html
python -m robot.libdoc JavaGui.SwtLibrary docs/keywords/Swt.html
python -m robot.libdoc JavaGui.RcpLibrary docs/keywords/Rcp.html
```

### 7.2 Validation
- Check generated HTML structure
- Verify all cross-references work
- Test on different browsers
- Validate search functionality

---

## 8. Comparison Summary

| Feature | Current | Browser Library | Target |
|---------|---------|-----------------|--------|
| Brief description | ✅ | ✅ | ✅ |
| Argument table | ✅ | ✅ | ✅ |
| Type information | ⚠️ Partial | ✅ Full | ✅ Full |
| Return documentation | ❌ | ✅ | ✅ |
| Behavior notes | ⚠️ Limited | ✅ | ✅ |
| Examples | ✅ | ✅ | ✅ Modern syntax |
| Example quality | ⚠️ Old syntax | ✅ Modern | ✅ Modern |
| Related keywords | ❌ | ✅ | ✅ |
| Tags | ❌ | ✅ | ✅ |
| Version info | ❌ | ✅ | ✅ |
| Assertion docs | ⚠️ Basic | ✅ Detailed | ✅ Detailed |

---

## 9. Next Steps

1. **Create templates** for each keyword type
2. **Update high-priority keywords** first (getters with assertions)
3. **Generate and review** documentation
4. **Validate** cross-references and links
5. **Publish** updated documentation
6. **Deprecate** old keywords with migration guide

---

## Appendix: Real-World Examples

### Before (Current)
```python
def get_text(self, locator: str, ...) -> str:
    """Get element text with optional assertion.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. |

    Example:
    | ${text}= | Get Text | JLabel#status | | |
    """
```

### After (Target)
```python
def get_text(
    self,
    locator: str,
    assertion_operator: Optional[AssertionOperator] = None,
    expected: Any = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None,
) -> str:
    """Get text content from an element with optional inline assertion.

    Use this keyword to retrieve text from labels, buttons, or other text-displaying
    components. Supports inline assertions following the Browser library pattern for
    more concise test code.

    = Arguments =

    | =Argument= | =Type= | =Description= |
    | ``locator`` | str | Element locator using CSS-like syntax. See `Locator Syntax`. |
    | ``assertion_operator`` | AssertionOperator | Optional assertion operator (==, !=, contains, etc.). See `Assertion Operators`. |
    | ``expected`` | Any | Expected value when using assertion operator. |
    | ``message`` | str | Custom error message on assertion failure. Defaults to auto-generated message. |
    | ``timeout`` | float | Assertion retry timeout in seconds. Defaults to library timeout (5.0s). |

    = Return Value =

    Returns ``str``: The text content of the element.

    - Without assertion: Returns immediately after text retrieval
    - With assertion: Retries every 100ms until assertion passes or timeout
    - Raises ``AssertionError`` if assertion fails after timeout

    = Behavior =

    * **Element Wait**: Automatically waits for element to exist
    * **Retry Logic**: With assertion, retries until value matches or timeout
    * **Strict Mode**: Fails if locator matches multiple elements
    * **Text Extraction**: Returns visible text content (not HTML)

    = Examples =

    Basic usage without assertion:
    | ${text}=    Get Text    JLabel#status
    | Should Be Equal    ${text}    Ready

    With inline assertion (recommended):
    | Get Text    JLabel#status    ==    Ready

    Using different operators:
    | Get Text    JLabel#message    contains    Success
    | Get Text    JLabel#count    matches    \\d+ items?
    | Get Text    JLabel#email    starts    admin@

    With custom message and timeout:
    | Get Text    JLabel#loading    ==    Complete
    | ...    message=Page did not finish loading
    | ...    timeout=10.0

    = Related Keywords =

    See `Element Text Should Be` for old-style assertion.
    See `Get Value` for input field values.

    = Tags =

    Getter, Assertion, Text, Verification

    = Version History =

    New in version 3.0.0
    - Added assertion operator support
    - Added timeout parameter
    - Deprecated: `Element Text Should Be` (use Get Text with == instead)
    """
```

---

**Document Status**: Draft for Review
**Author**: Research Agent
**Storage**: Memory namespace `docstring-patterns`
