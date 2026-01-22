# Docstring Analysis Summary

## Quick Reference

**Analysis Date**: 2026-01-20
**Full Specification**: See `DOCSTRING_SPECIFICATION.md`
**Memory Namespace**: `docstring-patterns`

---

## Current vs Target Format

### Current Format (What We Have)

```python
def get_text(self, locator: str, ...) -> str:
    """Get element text with optional assertion.

    | =Argument= | =Description= |
    | ``locator`` | Element locator. See `Locator Syntax`. |
    | ``assertion_operator`` | Optional assertion operator (==, !=, contains, etc.). |

    Without assertion operator, returns the element text directly.
    With assertion operator, retries until assertion passes or timeout.

    Example:
    | ${text}= | Get Text | JLabel#status | | |
    | Get Text | JLabel#status | == | Ready | |
    """
```

**Strengths**:
- Consistent argument tables
- Multiple examples
- Clear descriptions

**Gaps**:
- No return type documentation
- Old-style RF syntax in examples (pipes)
- Missing tags/categories
- No cross-references
- No version info
- Limited behavior notes

---

### Target Format (Browser Library Standard)

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
    components. Supports inline assertions following the Browser library pattern.

    = Arguments =

    | =Argument= | =Type= | =Description= |
    | ``locator`` | str | Element locator using CSS-like syntax. See `Locator Syntax`. |
    | ``assertion_operator`` | AssertionOperator | Optional operator. See `Assertion Operators`. |
    | ``expected`` | Any | Expected value when using assertion. |
    | ``message`` | str | Custom error message on assertion failure. |
    | ``timeout`` | float | Retry timeout in seconds. Defaults to 5.0s. |

    = Return Value =

    Returns ``str``: The text content of the element.

    - Without assertion: Returns immediately
    - With assertion: Retries until passes or timeout
    - Raises ``AssertionError`` on failure

    = Behavior =

    * **Element Wait**: Automatically waits for element
    * **Retry Logic**: Retries every 100ms with assertion
    * **Strict Mode**: Fails if multiple matches

    = Examples =

    Basic usage:
    | ${text}=    Get Text    JLabel#status
    | Should Be Equal    ${text}    Ready

    With inline assertion (recommended):
    | Get Text    JLabel#status    ==    Ready

    Various operators:
    | Get Text    JLabel#message    contains    Success
    | Get Text    JLabel#count    matches    \\d+ items

    = Related Keywords =

    See `Get Value` for input fields, `Element Text Should Be` for old-style assertions.

    = Tags =

    Getter, Assertion, Text, Verification

    = Version History =

    New in version 3.0.0 - Added assertion operator support
    """
```

---

## Key Improvements

### 1. Structure Enhancements

| Feature | Current | Target |
|---------|---------|--------|
| Brief description | ✅ Present | ✅ Enhanced with context |
| Extended description | ⚠️ Limited | ✅ When/why to use |
| Arguments table | ✅ Present | ✅ + Type column |
| Return documentation | ❌ Missing | ✅ Complete with conditions |
| Behavior section | ⚠️ Inline | ✅ Dedicated section |
| Examples | ✅ Present | ✅ Modern syntax + grouping |
| Related keywords | ❌ Missing | ✅ Cross-references |
| Tags | ❌ Missing | ✅ Categorization |
| Version history | ❌ Missing | ✅ Migration info |

### 2. Documentation Sections

**Required sections in target format**:

1. **Brief Description** - One sentence summary
2. **Extended Description** - When/why to use (1-3 paragraphs)
3. **Arguments Table** - With types and defaults
4. **Return Value** - Type, conditions, exceptions
5. **Behavior** - Retry, waiting, strict mode
6. **Examples** - Modern RF syntax, grouped by use case
7. **Related Keywords** - Cross-references
8. **Tags** - Categories for organization
9. **Version History** - Deprecation/migration info

### 3. Example Syntax Changes

**Old Style (Current)**:
```robot
| ${text}= | Get Text | JLabel#status | | |
| Get Text | JLabel#status | == | Ready | |
```

**New Style (Target)**:
```robot
${text}=    Get Text    JLabel#status
Get Text    JLabel#status    ==    Ready
```

**Changes**:
- Remove pipe separators (`|`)
- Use 4-space indentation for readability
- No trailing empty cells
- Clearer variable assignment syntax

---

## Implementation Priorities

### Phase 1: High-Priority Keywords (Week 1-2)

**Getter keywords with assertions** (15 keywords):
- `Get Text`
- `Get Value`
- `Get Count`
- `Get Element Count`
- `Get Selected Value`
- `Get Table Cell Value`
- `Get Tree Node Text`
- All SWT/RCP equivalents

**Why first**: Most used, highest visibility, demonstrate new pattern

### Phase 2: Action Keywords (Week 3-4)

**Core actions** (20 keywords):
- `Click Button`
- `Input Text`
- `Select From List`
- `Select Radio Button`
- `Select Checkbox`
- Table/Tree actions

**Why second**: High usage, need behavior documentation

### Phase 3: Wait Keywords (Week 5)

**Synchronization** (10 keywords):
- `Wait Until Element Is Visible`
- `Wait Until Element Is Enabled`
- `Wait For Condition`

**Why third**: Complex behavior needs detailed documentation

### Phase 4: Specialized Keywords (Week 6-8)

**Domain-specific** (50+ keywords):
- RCP perspective/view keywords
- SWT shell/widget keywords
- Menu navigation
- Dialog handling
- Tree/Table specialized operations

---

## Quality Checklist

Before marking a keyword as complete, verify:

**Structure**:
- [ ] One-sentence brief description
- [ ] Extended description (when/why to use)
- [ ] Complete arguments table with types
- [ ] Return value section (for getters)
- [ ] Behavior notes section

**Content**:
- [ ] 3-5 examples with modern RF syntax
- [ ] Examples grouped by use case
- [ ] Related keywords cross-references
- [ ] Tags for categorization
- [ ] Version/deprecation info (if applicable)

**Assertion Keywords Only**:
- [ ] Assertion operators table
- [ ] Examples showing all common operators
- [ ] Return value behavior with/without assertion
- [ ] Timeout behavior documented

**Quality**:
- [ ] No typos or grammar errors
- [ ] Cross-references use backticks
- [ ] Code examples are runnable
- [ ] Links to other sections work
- [ ] Type names match actual types

---

## Template Quick Reference

### Getter with Assertion Template

```python
def get_something(
    self,
    locator: str,
    assertion_operator: Optional[AssertionOperator] = None,
    expected: Any = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None,
) -> ReturnType:
    """Get [what] with optional inline assertion.

    [Extended description - when/why to use]

    = Arguments =

    | =Argument= | =Type= | =Description= |
    | ``locator`` | str | Element locator. See `Locator Syntax`. |
    | ``assertion_operator`` | AssertionOperator | Optional operator. See `Assertion Operators`. |
    | ``expected`` | Any | Expected value for assertion. |
    | ``message`` | str | Custom error message. |
    | ``timeout`` | float | Retry timeout in seconds. Defaults to library timeout. |

    = Return Value =

    Returns ``ReturnType``: [Description]

    = Behavior =

    * **[Feature]**: [Description]

    = Examples =

    [Description]:
    | ${value}=    Get Something    locator
    | Get Something    locator    ==    expected

    = Related Keywords =

    See `Other Keyword`.

    = Tags =

    Getter, Assertion

    = Version History =

    New in version X.Y.Z
    """
```

### Action Keyword Template

```python
def do_action(
    self,
    locator: str,
    param: str,
    optional: Optional[str] = None,
) -> None:
    """Perform [action] on [target].

    [Extended description]

    = Arguments =

    | =Argument= | =Type= | =Description= |
    | ``locator`` | str | Element locator. See `Locator Syntax`. |
    | ``param`` | str | [Description]. |
    | ``optional`` | str | [Description]. Defaults to ``None``. |

    = Behavior =

    * **[Feature]**: [Description]

    = Examples =

    Basic usage:
    | Do Action    locator    param

    With optional parameter:
    | Do Action    locator    param    optional=value

    = Related Keywords =

    See `Other Action`.

    = Tags =

    Action, [Category]

    = Version History =

    New in version X.Y.Z
    """
```

---

## Browser Library Documentation Features

Based on analysis of https://marketsquare.github.io/robotframework-browser/Browser.html:

### Features to Adopt

1. **Rich Type System**
   - Type annotations in signature
   - Custom types linked in docs
   - Union types clearly shown
   - NAMED_ONLY markers for keyword arguments

2. **Contextual Information**
   - Actionability checks explained
   - Strict vs non-strict mode documented
   - Performance implications noted
   - Related Playwright docs linked

3. **Comprehensive Examples**
   - Multiple variations per keyword
   - Context setup shown (New Page, New Context)
   - Edge cases demonstrated
   - Real-world scenarios

4. **Semantic Markup**
   - Code: `` `selector` ``
   - Keywords: Links to keyword docs
   - Types: Links to type definitions
   - External: Links to external docs

### Features Not Needed

- Playwright-specific concepts (we're Java GUI focused)
- Browser context management (different domain)
- Page lifecycle (different model)

---

## Migration Strategy

### Automated Assistance

Create docstring generator script:

```python
def generate_getter_docstring(
    keyword_name: str,
    what: str,
    return_type: str,
    return_description: str,
    example_value: str,
) -> str:
    """Generate docstring from template."""
    # Use templates from specification
    pass
```

### Manual Review Needed

Each docstring requires human review for:
- Accuracy of behavior description
- Relevance of examples
- Appropriateness of related keywords
- Correctness of version information

### Testing Generated Docs

```bash
# Generate documentation
python -m robot.libdoc JavaGui.SwingLibrary docs/keywords/Swing.html

# Validate
# - Open in browser
# - Check all links work
# - Verify search functionality
# - Test on different screen sizes
```

---

## Next Steps

1. **Review this analysis** with team
2. **Select pilot keywords** (5-10 high-priority getters)
3. **Apply template** to pilot keywords
4. **Generate and review** documentation
5. **Iterate on template** based on feedback
6. **Scale to all keywords** in priority order

---

## Resources

- **Full Specification**: `DOCSTRING_SPECIFICATION.md`
- **Memory Namespace**: `docstring-patterns`
- **Browser Library**: https://marketsquare.github.io/robotframework-browser/Browser.html
- **Robot Framework Docs**: http://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html#documenting-libraries

---

**Status**: Ready for Team Review
**Next Action**: Select pilot keywords for template application
