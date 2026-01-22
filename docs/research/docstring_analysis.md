# Docstring Analysis Report - robotframework-swing Library

**Analysis Date:** 2026-01-20
**Analyzed Files:** 7 Python files
**Total Keywords:** ~120+

## Executive Summary

The robotframework-swing library has **generally good documentation**, with approximately:
- **75 keywords (62%)** - EXCELLENT documentation with complete examples
- **30 keywords (25%)** - GOOD but needs improvement (missing examples or details)
- **15 keywords (13%)** - CRITICAL GAPS (missing significant documentation)

The library follows Robot Framework documentation conventions well, with consistent use of:
- Argument tables (`| =Argument= | =Description= |`)
- Robot Framework test table format for examples
- AssertionEngine integration documentation

## Files Analyzed

1. `python/JavaGui/__init__.py` - Main library classes (SwingLibrary, SwtLibrary, RcpLibrary)
2. `python/JavaGui/keywords/getters.py` - GetterKeywords mixin with assertion support
3. `python/JavaGui/keywords/rcp_keywords.py` - RcpKeywords mixin for Eclipse RCP
4. `python/JavaGui/keywords/tables.py` - Table, Tree, and List keywords
5. `python/JavaGui/keywords/swt_getters.py` - SWT widget getter keywords
6. `python/JavaGui/keywords/swt_tables.py` - SWT table-specific keywords
7. `python/JavaGui/keywords/swt_trees.py` - SWT tree-specific keywords

## Docstring Structure Patterns

### Excellent Examples (Use as Templates)

#### ✅ Complete Keyword Documentation (from `get_text`):
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
    | Get Text | JLabel#msg | matches | \\\\d+ items | |
    | Get Text | JLabel#title | == | hello world | formatters=['lowercase', 'strip'] |

    Supported operators: ==, !=, <, >, <=, >=, contains, not contains,
    starts, ends, matches, validate, then
    """
```

**Why This is Excellent:**
- ✅ Clear one-line summary
- ✅ Complete argument table with all parameters
- ✅ Explains assertion vs non-assertion usage
- ✅ Multiple examples showing different scenarios
- ✅ Lists supported operators
- ✅ Documents special feature (formatters)

## Critical Documentation Gaps

### HIGH Priority (Missing Examples)

These keywords have basic documentation but **lack examples**, making them harder to use:

#### Swing Library (`__init__.py`)
1. `right_click` (line 1058) - Missing popup menu usage example
2. `element_should_be_selected` (line 1073) - Missing checkbox/radio examples
3. `element_should_not_be_selected` (line 1090) - Missing examples
4. `get_tree_nodes` (line 1328) - Missing example
5. `get_table_data` (line 1364) - Missing example of 2D list usage
6. `get_element_properties` (line 1389) - Missing example

#### RCP Keywords (`rcp_keywords.py`)
7. `view_should_be_open` (line 399) - Missing example
8. `view_should_not_be_open` (line 434) - Missing example
9. `editor_should_be_open` (line 459) - Missing example
10. `editor_should_not_be_open` (line 494) - Missing example
11. `perspective_should_be_active` (line 519) - Missing example

#### Table/Tree Keywords (`tables.py`)
12. `get_selected_table_rows` (line 205) - Missing example
13. `get_tree_node_children` (line 336) - Missing example
14. `list_selection_should_be` (line 693) - Missing example

#### SWT Keywords
15. `get_swt_tree_item_text` (line 159 in `swt_trees.py`) - Path format unclear

### MEDIUM Priority (Incomplete Documentation)

These keywords have documentation but are **missing important details**:

#### Missing Return Value Documentation
- `get_list_items` - Should specify list element type
- `get_connection_info` - Dict structure not documented
- `wait_until_element_contains` - No return value documented

#### Missing Property Lists
- `get_element_property` - No comprehensive property list
- `get_widget_property` - Limited property examples
- `get_widget_properties` - Dict structure incomplete

#### Missing Error Documentation
- **All keywords** - No documentation on exceptions (ElementNotFoundError, TimeoutError, etc.)
- No guidance on common error scenarios

#### Assertion Operator Support
- Some assertion keywords don't list supported operators
- Operator behavior for lists vs strings not always explained

#### Timeout Behavior
- Retry behavior not documented for assertion keywords
- Poll interval not mentioned in most keywords

### MINOR Priority (Enhancement Opportunities)

- Edge case examples missing (empty text, null values, etc.)
- No "See Also" cross-references between related keywords
- State combinations not documented (e.g., visible AND disabled)
- Migration notes from old API missing
- Locator syntax examples could be more comprehensive

## Docstring Pattern Analysis

### Consistent Patterns Found ✅

1. **Argument Tables**: All keywords use Robot Framework table format
   ```
   | =Argument= | =Description= |
   | arg_name | Description text |
   ```

2. **Examples**: Robot Framework test table syntax
   ```
   | Keyword | Arg1 | Arg2 | Arg3 |
   ```

3. **Assertion Keywords**: Standard documentation pattern:
   - Lists supported operators
   - Explains retry behavior
   - Shows both assertion and non-assertion usage

4. **Return Values**: Usually documented in description text

### Inconsistent Patterns ⚠️

1. **Formatter Documentation**: Only `get_text` documents formatters
   - Should other get keywords support formatters?

2. **Default Values**: Some mentioned in docs, others only in signature

3. **Locator References**: Some say "See `Locator Syntax`", others show inline

4. **Property Names**: Not consistently documented across get_property keywords

## Recommendations

### Immediate Actions (High Impact)

1. **Add Examples to 15 Critical Keywords**
   - Focus on verification keywords (should_be_open, should_exist, etc.)
   - Add popup menu example to `right_click`
   - Add 2D list example to `get_table_data`

2. **Document Return Types**
   - Add return value descriptions to all getter keywords
   - Document dict structures (keys and value types)
   - Document list element types

3. **Add Error Documentation**
   - Document common exceptions (ElementNotFoundError, TimeoutError)
   - Add "Raises" sections to keywords that can fail
   - Provide troubleshooting guidance

### Medium Priority

4. **Document Property Names**
   - Create comprehensive list of available properties
   - Show property examples for each widget type
   - Add property type information

5. **Enhance Assertion Documentation**
   - Ensure all assertion keywords list operators
   - Explain operator behavior (especially for lists)
   - Document retry and timeout behavior consistently

6. **Add Formatter Support**
   - Document which keywords support formatters
   - Create formatter reference section
   - Add formatter examples

### Low Priority (Polish)

7. **Add Cross-References**
   - Link related keywords with "See Also"
   - Reference locator syntax guide
   - Link to assertion operator documentation

8. **Add Edge Cases**
   - Show examples for empty values
   - Show examples for error conditions
   - Show examples for complex locators

9. **Create Quick Reference**
   - Table of all keywords by category
   - Common patterns and idioms
   - Migration guide from old API

## Docstring Template

For consistency, use this template when updating keywords:

```python
def keyword_name(
    self,
    arg1: str,
    arg2: Optional[int] = None,
    assertion_operator: Optional[AssertionOperator] = None,
    expected: Any = None,
    message: Optional[str] = None,
    timeout: Optional[float] = None,
) -> ReturnType:
    """[One-line summary using imperative mood.]

    [2-3 sentence detailed description explaining:
     - What the keyword does
     - When to use it
     - Key behavior details]

    | =Argument= | =Description= |
    | ``arg1`` | [Description with type info if not in signature]. |
    | ``arg2`` | [Description]. Optional - defaults to [value]. |
    | ``assertion_operator`` | Optional assertion operator (==, !=, <, >, <=, >=, contains, not contains, starts, ends, matches, validate, then). |
    | ``expected`` | Expected value when using assertion operator. |
    | ``message`` | Custom error message on assertion failure. |
    | ``timeout`` | Assertion retry timeout in seconds. Uses library default if not specified. |

    [For assertion keywords:]
    Without assertion operator, returns the [value] directly.
    With assertion operator, retries until assertion passes or timeout.

    Returns [description of return value with type].

    Raises ``ExceptionType`` if [condition].

    Example:
    | # Basic usage |
    | ${result}= | Keyword Name | arg1_value | |
    | # With assertion |
    | Keyword Name | arg1_value | == | expected_value |
    | # With timeout |
    | Keyword Name | arg1_value | contains | text | timeout=10 |
    | # Edge case |
    | Keyword Name | special_value | != | None |

    Supported operators: ==, !=, <, >, <=, >=, contains, not contains,
    starts, ends, matches, validate, then

    See Also:
    - `Related Keyword Name`

    Note:
    [Special behavior or migration notes]
    """
```

## Complete Keyword Status List

### Swing Library - Connection Keywords
| Keyword | Status | Notes |
|---------|--------|-------|
| connect_to_application | ✅ EXCELLENT | Complete with all args, multiple examples |
| disconnect | ✅ EXCELLENT | Clear and concise |
| is_connected | ✅ EXCELLENT | Clear |
| get_connection_info | ⚠️ GOOD | Missing return structure documentation |

### Swing Library - Element Finding
| Keyword | Status | Notes |
|---------|--------|-------|
| find_element | ✅ EXCELLENT | Complete |
| find_elements | ✅ EXCELLENT | Complete |
| wait_until_element_exists | ✅ EXCELLENT | Complete |
| wait_until_element_does_not_exist | ✅ EXCELLENT | Complete |
| wait_for_element | ⚠️ GOOD | Could add more examples |

### Swing Library - Click Keywords
| Keyword | Status | Notes |
|---------|--------|-------|
| click | ✅ EXCELLENT | Clear |
| click_element | ✅ EXCELLENT | Clear |
| double_click | ✅ EXCELLENT | Clear |
| click_button | ✅ EXCELLENT | Clear |
| right_click | ❌ NEEDS IMPROVEMENT | Missing popup menu example |

### Swing Library - Input Keywords
| Keyword | Status | Notes |
|---------|--------|-------|
| input_text | ✅ EXCELLENT | Explains clear parameter |
| clear_text | ✅ EXCELLENT | Clear |
| type_text | ⚠️ GOOD | Needs more detail on difference from input_text |

### Swing Library - Selection Keywords
| Keyword | Status | Notes |
|---------|--------|-------|
| select_from_combobox | ✅ EXCELLENT | Good example |
| check_checkbox | ✅ EXCELLENT | Clear |
| uncheck_checkbox | ✅ EXCELLENT | Clear |
| select_radio_button | ✅ EXCELLENT | Clear |

### Swing Library - Table Keywords
| Keyword | Status | Notes |
|---------|--------|-------|
| select_table_cell | ✅ EXCELLENT | Clear |
| select_table_row | ✅ EXCELLENT | Clear |
| get_table_data | ❌ NEEDS EXAMPLE | Missing 2D list usage example |

### Swing Library - Tree Keywords
| Keyword | Status | Notes |
|---------|--------|-------|
| expand_tree_node | ✅ EXCELLENT | Clear |
| collapse_tree_node | ✅ EXCELLENT | Clear |
| select_tree_node | ✅ EXCELLENT | Clear |
| get_selected_tree_node | ⚠️ GOOD | Missing examples |
| get_tree_nodes | ❌ NEEDS EXAMPLE | Missing example |

### Swing Library - Menu Keywords
| Keyword | Status | Notes |
|---------|--------|-------|
| select_menu | ✅ EXCELLENT | Clear |
| select_from_popup_menu | ✅ EXCELLENT | Clear |

### Swing Library - Wait Keywords
| Keyword | Status | Notes |
|---------|--------|-------|
| wait_until_element_is_visible | ✅ EXCELLENT | Complete |
| wait_until_element_is_enabled | ✅ EXCELLENT | Complete |
| wait_until_element_contains | ⚠️ GOOD | Implementation inline, no helper |

### Swing Library - Verification Keywords
| Keyword | Status | Notes |
|---------|--------|-------|
| element_should_be_visible | ✅ EXCELLENT | Clear |
| element_should_not_be_visible | ✅ EXCELLENT | Clear |
| element_should_be_enabled | ✅ EXCELLENT | Clear |
| element_should_be_disabled | ✅ EXCELLENT | Clear |
| element_text_should_be | ✅ EXCELLENT | Clear |
| element_text_should_contain | ✅ EXCELLENT | Clear |
| element_should_be_selected | ❌ NEEDS EXAMPLES | Missing checkbox/radio examples |
| element_should_not_be_selected | ❌ NEEDS EXAMPLES | Missing examples |
| element_should_exist | ⚠️ GOOD | Basic |
| element_should_not_exist | ⚠️ GOOD | Basic |

### Getter Keywords Mixin
| Keyword | Status | Notes |
|---------|--------|-------|
| get_text | ✅ EXCELLENT | With formatters! |
| get_value | ⚠️ GOOD | Could document formatters |
| get_element_count | ✅ EXCELLENT | Clear |
| get_element_states | ✅ EXCELLENT | Comprehensive |
| get_property | ⚠️ GOOD | Needs property list |
| get_properties | ⚠️ GOOD | Needs dict structure |
| set_assertion_timeout | ✅ EXCELLENT | Clear |
| set_assertion_interval | ✅ EXCELLENT | Clear |

### Table Keywords Mixin
| Keyword | Status | Notes |
|---------|--------|-------|
| get_table_cell_value | ✅ EXCELLENT | Complete |
| get_table_row_count | ✅ EXCELLENT | Complete |
| get_table_column_count | ✅ EXCELLENT | Complete |
| get_table_row_values | ✅ EXCELLENT | Complete |
| get_table_column_values | ✅ EXCELLENT | Complete |
| get_selected_table_rows | ❌ NEEDS EXAMPLE | Missing example |

### Tree Keywords Mixin
| Keyword | Status | Notes |
|---------|--------|-------|
| get_selected_tree_node | ✅ EXCELLENT | Clear |
| get_tree_node_count | ✅ EXCELLENT | Clear |
| get_tree_node_children | ❌ NEEDS EXAMPLE | Missing example |
| tree_node_should_exist | ✅ EXCELLENT | Clear |
| tree_node_should_not_exist | ✅ EXCELLENT | Clear |

### List Keywords Mixin
| Keyword | Status | Notes |
|---------|--------|-------|
| get_selected_list_item | ✅ EXCELLENT | Clear |
| get_selected_list_items | ✅ EXCELLENT | Clear |
| get_list_items | ✅ EXCELLENT | Clear |
| get_list_item_count | ✅ EXCELLENT | Clear |
| get_selected_list_index | ✅ EXCELLENT | Clear |
| list_should_contain | ✅ EXCELLENT | Clear |
| list_should_not_contain | ✅ EXCELLENT | Clear |
| list_selection_should_be | ❌ NEEDS EXAMPLE | Missing example |

### RCP Keywords Mixin
| Keyword | Status | Notes |
|---------|--------|-------|
| get_open_view_count | ✅ EXCELLENT | Complete |
| get_open_editor_count | ✅ EXCELLENT | Complete |
| get_active_perspective_id | ✅ EXCELLENT | Complete |
| get_editor_dirty_state | ✅ EXCELLENT | Complete |
| get_view_title | ✅ EXCELLENT | Complete |
| get_open_view_ids | ⚠️ GOOD | Needs list structure docs |
| get_open_editor_titles | ⚠️ GOOD | Needs list structure docs |
| get_active_editor_title | ✅ EXCELLENT | Complete |
| get_dirty_editor_count | ✅ EXCELLENT | Complete |
| view_should_be_open | ❌ NEEDS EXAMPLE | Missing example |
| view_should_not_be_open | ❌ NEEDS EXAMPLE | Missing example |
| editor_should_be_open | ❌ NEEDS EXAMPLE | Missing example |
| editor_should_not_be_open | ❌ NEEDS EXAMPLE | Missing example |
| perspective_should_be_active | ❌ NEEDS EXAMPLE | Missing example |

### SWT Getter Keywords
| Keyword | Status | Notes |
|---------|--------|-------|
| get_widget_text | ✅ EXCELLENT | Complete |
| get_widget_count | ✅ EXCELLENT | Complete |
| get_widget_property | ⚠️ GOOD | Needs property list |
| is_widget_enabled | ⚠️ BASIC | Basic example only |
| is_widget_visible | ⚠️ BASIC | Basic example only |
| is_widget_focused | ⚠️ BASIC | Basic example only |
| get_widget_states | ✅ EXCELLENT | Comprehensive |
| get_widget_properties | ⚠️ NEEDS BETTER DOCS | Dict structure unclear |

### SWT Table Keywords
| Keyword | Status | Notes |
|---------|--------|-------|
| get_swt_table_row_count | ✅ EXCELLENT | Complete |
| get_swt_table_cell | ✅ EXCELLENT | Complete |
| get_swt_table_row_values | ✅ EXCELLENT | Complete |
| get_swt_table_column_count | ✅ EXCELLENT | Complete |
| get_swt_table_column_headers | ✅ EXCELLENT | Complete |
| get_swt_selected_table_rows | ✅ EXCELLENT | Complete |
| swt_table_cell_should_contain | ⚠️ BASIC | Could use more examples |
| swt_table_row_count_should_be | ✅ EXCELLENT | Complete |
| swt_table_should_have_rows | ✅ EXCELLENT | Complete |
| swt_table_should_be_empty | ✅ EXCELLENT | Complete |

### SWT Tree Keywords
| Keyword | Status | Notes |
|---------|--------|-------|
| get_swt_selected_tree_nodes | ✅ EXCELLENT | Complete |
| get_swt_tree_node_count | ⚠️ COMPLEX | Needs better explanation |
| get_swt_tree_item_text | ⚠️ GOOD | Path format unclear |
| swt_tree_node_should_exist | ✅ EXCELLENT | Complete |
| swt_tree_node_should_not_exist | ✅ EXCELLENT | Complete |
| swt_tree_should_have_selection | ✅ EXCELLENT | Complete |
| swt_tree_selection_should_be | ✅ EXCELLENT | Complete |
| get_swt_tree_node_level | ✅ EXCELLENT | Complete |
| get_swt_tree_node_parent | ✅ EXCELLENT | Complete |

## Summary Statistics

| Category | Count | Percentage |
|----------|-------|------------|
| ✅ EXCELLENT | ~75 | 62% |
| ⚠️ NEEDS IMPROVEMENT | ~30 | 25% |
| ❌ CRITICAL GAPS | ~15 | 13% |
| **Total Keywords** | **~120** | **100%** |

## Next Steps

1. **Review this analysis** with the team
2. **Prioritize** the 15 critical gap keywords
3. **Create issues** for documentation updates
4. **Update keywords** using the provided template
5. **Validate** updated docs with actual usage
6. **Generate** Robot Framework keyword documentation (libdoc)

---

**Analysis stored in memory under namespace:** `docstring-analysis`
**Memory keys:**
- `docstring-keyword-list` - Complete keyword list with status
- `docstring-improvement-template` - Template for updates
