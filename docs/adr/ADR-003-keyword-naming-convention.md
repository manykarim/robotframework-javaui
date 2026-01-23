# ADR-003: Keyword Naming Convention

| ADR ID | ADR-003 |
|--------|---------|
| Title | Keyword Naming Convention |
| Status | Proposed |
| Date | 2026-01-16 |
| Authors | Architecture Team |

## Context

The library currently uses inconsistent naming conventions across technologies:

### Current Naming Patterns

| Operation | Swing Keywords | SWT Keywords | Discrepancy |
|-----------|---------------|--------------|-------------|
| Find single | `Find Element` | `Find Widget` | Element vs Widget |
| Find multiple | `Find Elements` | `Find Widgets` | Element vs Widget |
| Click | `Click Element`, `Click` | `Click Widget` | Element vs Widget |
| Double-click | `Double Click` | `Double Click Widget` | Missing suffix |
| Check state | `Element Should Be Visible` | `Widget Should Be Visible` | Element vs Widget |
| Table operations | `Get Table Cell Value` | `Get Table Cell` | Inconsistent suffix |
| Tree operations | `Expand Tree Node` | `Expand Tree Item` | Node vs Item |
| Checkbox | `Check Checkbox` | `Check Button` | Checkbox vs Button |

### Terminology Comparison

| Concept | Swing Term | SWT Term | Industry Standard |
|---------|------------|----------|-------------------|
| UI component | Component/Element | Widget | Element (Web), Widget (Desktop) |
| Top-level window | JFrame | Shell | Window |
| Dialog | JDialog | Shell (dialog style) | Dialog |
| Tree path node | Node | Item | Node |
| Selection list | JList | List | List |

### Decision Drivers

- Robot Framework ecosystem typically uses "Element" terminology (SeleniumLibrary, AppiumLibrary)
- SWT/Eclipse community uses "Widget" terminology
- Need consistent naming within the library
- Backwards compatibility with existing tests
- Discoverability in keyword documentation

## Decision

We will adopt **Element-Based Naming** as the standard, with **Technology-Specific Aliases** for backwards compatibility.

### 1. Primary Naming Convention

**Rule**: Use "Element" for operations on UI components, following Robot Framework ecosystem conventions.

```
# Canonical keyword names (new standard)
Click                          # Simple action (no suffix needed)
Click Element                  # Explicit target (optional suffix)
Double Click
Right Click
Find Element
Find Elements
Wait Until Element Exists
Wait Until Element Is Visible
Element Should Be Visible
Element Should Be Enabled
Element Text Should Be
Get Element Text
Get Element Property
```

**Rule**: Use verb-first naming for action keywords.

```
# Actions: Verb [Object] [Qualifier]
Click                           # Simple
Click Element                   # With target type
Click Button                    # With widget type
Double Click Element
Select From Combobox
Input Text
Clear Text
Expand Tree Node
```

**Rule**: Use noun-first naming for getter/query keywords.

```
# Getters: [Target] Should/Get/Is [State/Property]
Element Should Be Visible
Element Should Be Enabled
Element Text Should Be
Get Element Text
Get Table Cell Value
Get Table Row Count
Is Element Visible
Is Element Enabled
```

### 2. Complete Keyword Naming Standard

#### Connection Keywords
| Unified Name | Description |
|--------------|-------------|
| `Connect To Application` | Connect to any Java GUI application |
| `Disconnect` | Close connection |
| `Is Connected` | Check connection status |
| `Get Connection Info` | Get connection details |

#### Element Finding Keywords
| Unified Name | Description |
|--------------|-------------|
| `Find Element` | Find single element by locator |
| `Find Elements` | Find all matching elements |
| `Wait Until Element Exists` | Wait for element to appear |
| `Wait Until Element Does Not Exist` | Wait for element to disappear |
| `Wait For Element` | Wait and return element |

#### Clicking Keywords
| Unified Name | Description |
|--------------|-------------|
| `Click` | Single click on element |
| `Click Element` | Alias for Click |
| `Double Click` | Double-click on element |
| `Right Click` | Right/context click |
| `Click At` | Click at specific coordinates |

#### Text Input Keywords
| Unified Name | Description |
|--------------|-------------|
| `Input Text` | Enter text (clears first by default) |
| `Clear Text` | Clear text from element |
| `Append Text` | Add text without clearing |
| `Type Text` | Character-by-character typing |
| `Get Text` | Get element text content |

#### Selection Keywords
| Unified Name | Description |
|--------------|-------------|
| `Select From Combobox` | Select combobox item |
| `Select From List` | Select list item |
| `Select List Item By Index` | Select by index |
| `Get List Items` | Get all list items |
| `Check` | Check checkbox/toggle |
| `Uncheck` | Uncheck checkbox/toggle |
| `Select Radio` | Select radio button |

#### Table Keywords
| Unified Name | Description |
|--------------|-------------|
| `Get Table Cell Value` | Get cell value |
| `Select Table Cell` | Select specific cell |
| `Select Table Row` | Select row by index |
| `Select Table Rows` | Select multiple rows |
| `Get Table Row Count` | Count rows |
| `Get Table Column Count` | Count columns |
| `Get Table Data` | Get all table data |
| `Get Table Column Headers` | Get header labels |
| `Double Click Table Row` | Double-click row |

#### Tree Keywords
| Unified Name | Description |
|--------------|-------------|
| `Expand Tree Node` | Expand tree node |
| `Collapse Tree Node` | Collapse tree node |
| `Select Tree Node` | Select tree node |
| `Get Selected Tree Node` | Get selected node path |
| `Get Tree Nodes` | Get all node paths |
| `Tree Node Should Exist` | Assert node exists |

#### Verification Keywords
| Unified Name | Description |
|--------------|-------------|
| `Element Should Exist` | Assert element exists |
| `Element Should Not Exist` | Assert element doesn't exist |
| `Element Should Be Visible` | Assert element is visible |
| `Element Should Not Be Visible` | Assert element is not visible |
| `Element Should Be Enabled` | Assert element is enabled |
| `Element Should Be Disabled` | Assert element is disabled |
| `Element Should Be Focused` | Assert element has focus |
| `Element Text Should Be` | Assert exact text |
| `Element Text Should Contain` | Assert text contains |

#### Window/Shell Keywords
| Unified Name | Description |
|--------------|-------------|
| `Get Windows` | Get all windows/shells |
| `Activate Window` | Activate/focus window |
| `Close Window` | Close window |
| `Window Should Exist` | Assert window exists |
| `Window Title Should Be` | Assert window title |

### 3. Alias System Implementation

```rust
/// Macro to generate keyword with aliases
macro_rules! keyword_with_aliases {
    ($primary:ident, [$($alias:ident),*], $impl:expr) => {
        #[pyo3(name = stringify!($primary))]
        pub fn $primary(&self, locator: &str) -> PyResult<()> {
            $impl(locator)
        }

        $(
            #[pyo3(name = stringify!($alias))]
            pub fn $alias(&self, locator: &str) -> PyResult<()> {
                // Log deprecation warning
                log_deprecation(stringify!($alias), stringify!($primary));
                $impl(locator)
            }
        )*
    };
}

// Usage in implementation
impl JavaGuiLibrary {
    keyword_with_aliases!(
        click,                                    // Primary
        [click_element, click_widget],            // Aliases
        |locator| self.shared.click_impl(locator) // Implementation
    );

    keyword_with_aliases!(
        find_element,                             // Primary
        [find_widget],                            // Aliases
        |locator| self.shared.find_element_impl(locator)
    );
}
```

### 4. Backwards Compatibility Mapping

```rust
/// Complete mapping of deprecated to canonical keyword names
pub const KEYWORD_ALIASES: &[(&str, &str, DeprecationStatus)] = &[
    // SWT-style -> Unified
    ("Click Widget", "Click", DeprecationStatus::Deprecated),
    ("Double Click Widget", "Double Click", DeprecationStatus::Deprecated),
    ("Right Click Widget", "Right Click", DeprecationStatus::Deprecated),
    ("Find Widget", "Find Element", DeprecationStatus::Deprecated),
    ("Find Widgets", "Find Elements", DeprecationStatus::Deprecated),
    ("Wait Until Widget Exists", "Wait Until Element Exists", DeprecationStatus::Deprecated),
    ("Wait Until Widget Enabled", "Wait Until Element Is Enabled", DeprecationStatus::Deprecated),
    ("Widget Should Be Visible", "Element Should Be Visible", DeprecationStatus::Deprecated),
    ("Widget Should Be Enabled", "Element Should Be Enabled", DeprecationStatus::Deprecated),
    ("Widget Text Should Be", "Element Text Should Be", DeprecationStatus::Deprecated),

    // SWT checkbox/radio -> Unified
    ("Check Button", "Check", DeprecationStatus::Deprecated),
    ("Uncheck Button", "Uncheck", DeprecationStatus::Deprecated),

    // SWT tree -> Unified
    ("Expand Tree Item", "Expand Tree Node", DeprecationStatus::Deprecated),
    ("Collapse Tree Item", "Collapse Tree Node", DeprecationStatus::Deprecated),
    ("Select Tree Item", "Select Tree Node", DeprecationStatus::Deprecated),

    // SWT shell -> Unified window
    ("Get Shells", "Get Windows", DeprecationStatus::Active), // Keep as SWT-specific
    ("Activate Shell", "Activate Window", DeprecationStatus::Active),
    ("Close Shell", "Close Window", DeprecationStatus::Active),

    // Minor variations
    ("Get Table Cell", "Get Table Cell Value", DeprecationStatus::Deprecated),
    ("Disconnect From Application", "Disconnect", DeprecationStatus::Active), // Both valid

    // Legacy Swing variations (keep as active aliases)
    ("Click Element", "Click", DeprecationStatus::Active),
    ("Check Checkbox", "Check", DeprecationStatus::Active),
    ("Uncheck Checkbox", "Uncheck", DeprecationStatus::Active),
    ("Select Radio Button", "Select Radio", DeprecationStatus::Active),
];

#[derive(Clone, Copy)]
pub enum DeprecationStatus {
    Active,      // Both names are valid, no warning
    Deprecated,  // Works but logs warning
    Removed,     // Will be removed in next major version
}
```

### 5. Deprecation Warning Implementation

```rust
fn log_deprecation(old_name: &str, new_name: &str) {
    // Use Python's warnings module for proper deprecation handling
    Python::with_gil(|py| {
        let warnings = py.import("warnings").ok();
        if let Some(warnings) = warnings {
            let message = format!(
                "Keyword '{}' is deprecated. Use '{}' instead. \
                This keyword will be removed in version 3.0.",
                old_name, new_name
            );
            warnings.call_method1("warn", (message, py.get_type::<pyo3::exceptions::PyDeprecationWarning>())).ok();
        }
    });
}
```

### 6. Robot Framework Documentation

```robot
*** Keywords ***
# Documentation shows both names
# Click
#     [Documentation]    Click on an element.
#     ...
#     ...    Aliases: ``Click Element``, ``Click Widget`` (deprecated)
#     ...
#     ...    | =Argument= | =Description= |
#     ...    | ``locator`` | Element locator |
#     ...
#     ...    Examples:
#     ...    | `Click` | name:submitButton |
#     ...    | `Click` | Button[text='OK'] |
#     [Arguments]    ${locator}
```

## Consequences

### Positive

1. **Consistency**: Single naming convention across all technologies
2. **Ecosystem Alignment**: Matches Robot Framework "Element" convention
3. **Backwards Compatible**: Old keywords continue to work via aliases
4. **Clear Migration Path**: Deprecation warnings guide users to new names
5. **Better Autocomplete**: Consistent prefixes improve IDE experience
6. **Reduced Documentation**: Single name to document per operation

### Negative

1. **Learning Curve**: SWT users must learn new terminology
2. **Test Updates**: Existing tests should update to avoid deprecation warnings
3. **Two Names**: During transition, docs must show both old and new
4. **Eclipse Community**: "Widget" is standard Eclipse terminology

### Risks

1. **Alias Maintenance**: Must maintain aliases indefinitely for compatibility
2. **Documentation Sync**: Risk of docs being out of sync with implementation
3. **Community Resistance**: SWT users may prefer "Widget" terminology

## Alternatives Considered

### Alternative 1: Keep Widget Naming for SWT

Maintain separate naming conventions per technology.

**Rejected because**:
- Doesn't improve consistency
- Makes unified documentation harder
- Users must learn multiple conventions

### Alternative 2: Use Generic "Component" Term

Use neutral "Component" instead of Element or Widget.

**Rejected because**:
- "Component" is Java Swing specific
- Not standard in Robot Framework ecosystem
- Longer keyword names

### Alternative 3: No Deprecation, Just Aliases

Keep all names as equal alternatives forever.

**Rejected because**:
- Doesn't push toward consistency
- Documentation remains fragmented
- New users confused by multiple names

## Implementation Plan

1. **Phase 1**: Define complete keyword name mapping (this ADR) (1 day)
2. **Phase 2**: Implement alias macro system (3 days)
3. **Phase 3**: Add deprecation warning infrastructure (2 days)
4. **Phase 4**: Update all keywords with aliases (1 week)
5. **Phase 5**: Update documentation (1 week)
6. **Phase 6**: Add migration guide (2 days)

## References

- [Robot Framework SeleniumLibrary Keywords](https://robotframework.org/SeleniumLibrary/SeleniumLibrary.html)
- [Robot Framework AppiumLibrary Keywords](https://serhatbolsu.github.io/robotframework-appiumlibrary/AppiumLibrary.html)
- [Python Deprecation Warnings](https://docs.python.org/3/library/warnings.html)
- [Unified Keywords Research](/docs/unify_keywords_research.md)
