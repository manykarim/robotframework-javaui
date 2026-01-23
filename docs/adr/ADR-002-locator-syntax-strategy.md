# ADR-002: Locator Syntax Strategy

| ADR ID | ADR-002 |
|--------|---------|
| Title | Locator Syntax Strategy |
| Status | Proposed |
| Date | 2026-01-16 |
| Authors | Architecture Team |

## Context

The library currently supports three different locator syntax styles across technologies:

### Current Locator Styles

| Technology | Primary Style | Examples |
|------------|---------------|----------|
| **Swing** | CSS-like + XPath | `JButton[name='submit']`, `#submitButton`, `//JButton[@text='OK']` |
| **SWT** | Prefix-based | `name:submitButton`, `text:Submit`, `class:Button` |
| **RCP** | Eclipse IDs + SWT | `org.eclipse.ui.views.ProblemView`, `name:okButton` |

### Current Locator Module Structure

```rust
// src/locator/mod.rs exports:
pub use ast::{Locator, ComplexSelector, Combinator, CompoundSelector, ...};
pub use expression::{LocatorExpression, SimpleLocator, CssSelector, XPathExpression, ...};
pub use matcher::{Evaluator, MatchContext, MatchResult};
pub use parser::{parse_locator, ParseError};
pub use swt_matcher::{SwtLocator, SwtMatcher, WidgetSelector, ...};
```

### Problems with Current Approach

1. **User Confusion**: Different syntax for different technologies
2. **Documentation Burden**: Must document multiple formats
3. **Test Portability**: Tests can't be easily adapted between technologies
4. **Feature Gaps**: CSS pseudo-classes work in Swing but not SWT

### Decision Drivers

- Users should not need to learn multiple locator syntaxes
- Existing tests must continue to work
- Support for technology-specific locator features
- Performance (locator parsing happens frequently)
- Extensibility for future locator types

## Decision

We will implement a **Unified Locator Syntax with Automatic Normalization** that:

1. Accepts all existing locator formats (backwards compatible)
2. Normalizes to a common internal representation
3. Provides a recommended "canonical" format for new tests
4. Supports technology-specific extensions where needed

### 1. Canonical Locator Format

The recommended format for new tests is **CSS-like selectors with attribute notation**:

```
# Primary selectors (work across all technologies)
Button                           # Type selector
Button[name='submit']            # Type + attribute
[name='submit']                  # Attribute only
#submitButton                    # ID shorthand (name attribute)

# Attribute operators
[name='exact']                   # Exact match
[text^='Start']                  # Starts with
[text$='End']                    # Ends with
[text*='contains']               # Contains
[name~='word']                   # Word match

# Pseudo-classes (universal)
:enabled                         # Element is enabled
:disabled                        # Element is disabled
:visible                         # Element is visible
:hidden                          # Element is hidden
:checked                         # Checkbox/radio is checked
:focused                         # Element has focus
:first                           # First matching element
:last                            # Last matching element
:nth(2)                          # Nth element (1-indexed)

# Combinators
Panel > Button                   # Direct child
Panel Button                     # Descendant
Button + Label                   # Adjacent sibling
Button ~ Label                   # General sibling

# XPath (for complex queries)
//Button[@name='submit']         # XPath absolute
.//Button                        # XPath relative
```

### 2. Locator Normalization Layer

```rust
/// Unified locator normalizer that converts any format to internal representation
pub struct LocatorNormalizer {
    /// Technology mode for context-aware parsing
    mode: GuiMode,
    /// Cache for parsed locators
    cache: LruCache<String, NormalizedLocator>,
}

/// Internal normalized locator representation
pub enum NormalizedLocator {
    /// Simple type selector: Button, JButton, Text
    Type(TypeSelector),
    /// Attribute-based: [name='x'], [text='y']
    Attribute(AttributeSelector),
    /// Combined: Type[attr='val']
    Compound(CompoundSelector),
    /// XPath expression
    XPath(XPathSelector),
    /// Technology-specific (Eclipse IDs, etc.)
    TechSpecific(TechSpecificSelector),
}

#[derive(Debug, Clone)]
pub struct TypeSelector {
    /// Original type name (e.g., "Button", "JButton", "Text")
    pub raw_type: String,
    /// Normalized type that works across technologies
    pub normalized_type: WidgetType,
}

#[derive(Debug, Clone, Copy)]
pub enum WidgetType {
    Button,
    TextField,
    TextArea,
    Checkbox,
    RadioButton,
    ComboBox,
    List,
    Table,
    Tree,
    Label,
    Panel,
    Menu,
    MenuItem,
    Tab,
    Dialog,
    Window,
    Custom(u32), // Hash of custom type name
}

impl LocatorNormalizer {
    pub fn normalize(&mut self, locator: &str) -> Result<NormalizedLocator, LocatorError> {
        // Check cache first
        if let Some(cached) = self.cache.get(locator) {
            return Ok(cached.clone());
        }

        let normalized = self.parse_and_normalize(locator)?;
        self.cache.put(locator.to_string(), normalized.clone());
        Ok(normalized)
    }

    fn parse_and_normalize(&self, locator: &str) -> Result<NormalizedLocator, LocatorError> {
        let trimmed = locator.trim();

        // 1. Check for XPath (starts with // or /)
        if trimmed.starts_with("//") || trimmed.starts_with("/") {
            return self.parse_xpath(trimmed);
        }

        // 2. Check for SWT-style prefix (name:, text:, class:, id:)
        if let Some(prefix_loc) = self.parse_prefix_locator(trimmed)? {
            return Ok(prefix_loc);
        }

        // 3. Check for ID shorthand (#id)
        if trimmed.starts_with('#') {
            return Ok(NormalizedLocator::Attribute(AttributeSelector {
                attribute: "name".to_string(),
                operator: MatchOperator::Equals,
                value: trimmed[1..].to_string(),
            }));
        }

        // 4. Check for Eclipse-style ID (contains dots like org.eclipse.ui.views.X)
        if self.is_eclipse_id(trimmed) {
            return Ok(NormalizedLocator::TechSpecific(TechSpecificSelector::EclipseId(
                trimmed.to_string()
            )));
        }

        // 5. Parse as CSS-like selector
        self.parse_css_selector(trimmed)
    }

    fn parse_prefix_locator(&self, locator: &str) -> Result<Option<NormalizedLocator>, LocatorError> {
        let prefixes = [
            ("name:", "name"),
            ("text:", "text"),
            ("class:", "class"),
            ("id:", "name"),        // id: maps to name attribute
            ("tooltip:", "tooltip"),
            ("index:", "index"),
        ];

        for (prefix, attr) in prefixes {
            if locator.starts_with(prefix) {
                let value = &locator[prefix.len()..];
                return Ok(Some(NormalizedLocator::Attribute(AttributeSelector {
                    attribute: attr.to_string(),
                    operator: MatchOperator::Equals,
                    value: value.to_string(),
                })));
            }
        }

        Ok(None)
    }

    fn normalize_widget_type(&self, type_name: &str) -> WidgetType {
        match type_name.to_lowercase().as_str() {
            // Button types
            "button" | "jbutton" => WidgetType::Button,
            "pushbutton" => WidgetType::Button,

            // Text input types
            "text" | "textfield" | "jtextfield" | "jtextcomponent" => WidgetType::TextField,
            "textarea" | "jtextarea" => WidgetType::TextArea,
            "styledtext" => WidgetType::TextArea,

            // Selection types
            "checkbox" | "jcheckbox" => WidgetType::Checkbox,
            "radiobutton" | "jradiobutton" => WidgetType::RadioButton,
            "combo" | "combobox" | "jcombobox" | "ccombo" => WidgetType::ComboBox,
            "list" | "jlist" => WidgetType::List,

            // Data display types
            "table" | "jtable" => WidgetType::Table,
            "tree" | "jtree" => WidgetType::Tree,

            // Other
            "label" | "jlabel" | "clabel" => WidgetType::Label,
            "panel" | "jpanel" | "composite" | "group" => WidgetType::Panel,
            "menu" | "jmenu" => WidgetType::Menu,
            "menuitem" | "jmenuitem" => WidgetType::MenuItem,
            "tabfolder" | "jtabbedpane" | "ctabfolder" => WidgetType::Tab,
            "shell" | "jdialog" | "jframe" => WidgetType::Window,

            // Unknown - use hash
            other => WidgetType::Custom(Self::hash_type(other)),
        }
    }
}
```

### 3. Type Mapping Table

Create explicit mappings between Swing and SWT widget types:

```rust
pub struct WidgetTypeMapping {
    pub swing_class: &'static str,
    pub swt_class: &'static str,
    pub canonical_name: &'static str,
    pub widget_type: WidgetType,
}

pub const WIDGET_MAPPINGS: &[WidgetTypeMapping] = &[
    WidgetTypeMapping {
        swing_class: "javax.swing.JButton",
        swt_class: "org.eclipse.swt.widgets.Button",
        canonical_name: "Button",
        widget_type: WidgetType::Button,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JTextField",
        swt_class: "org.eclipse.swt.widgets.Text",
        canonical_name: "TextField",
        widget_type: WidgetType::TextField,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JTextArea",
        swt_class: "org.eclipse.swt.custom.StyledText",
        canonical_name: "TextArea",
        widget_type: WidgetType::TextArea,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JCheckBox",
        swt_class: "org.eclipse.swt.widgets.Button", // SWT.CHECK style
        canonical_name: "Checkbox",
        widget_type: WidgetType::Checkbox,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JRadioButton",
        swt_class: "org.eclipse.swt.widgets.Button", // SWT.RADIO style
        canonical_name: "RadioButton",
        widget_type: WidgetType::RadioButton,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JComboBox",
        swt_class: "org.eclipse.swt.widgets.Combo",
        canonical_name: "ComboBox",
        widget_type: WidgetType::ComboBox,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JList",
        swt_class: "org.eclipse.swt.widgets.List",
        canonical_name: "List",
        widget_type: WidgetType::List,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JTable",
        swt_class: "org.eclipse.swt.widgets.Table",
        canonical_name: "Table",
        widget_type: WidgetType::Table,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JTree",
        swt_class: "org.eclipse.swt.widgets.Tree",
        canonical_name: "Tree",
        widget_type: WidgetType::Tree,
    },
    // ... additional mappings
];
```

### 4. Locator Examples (All Equivalent)

```robot
# All of these find the same button with name "submitButton":

# CSS-style (canonical, recommended)
Click    Button[name='submitButton']
Click    [name='submitButton']
Click    #submitButton

# SWT-style prefix (backwards compatible)
Click    name:submitButton

# XPath (for complex queries)
Click    //Button[@name='submitButton']

# Technology-specific class (when needed)
Click    JButton[name='submitButton']      # Swing-specific
Click    org.eclipse.swt.widgets.Button[name='submitButton']  # SWT-specific
```

### 5. Error Messages with Suggestions

When a locator fails, provide helpful suggestions:

```rust
impl LocatorNormalizer {
    pub fn suggest_corrections(&self, locator: &str, error: &LocatorError) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Check for Swing-specific types in SWT mode
        if self.mode == GuiMode::Swt && locator.contains("JButton") {
            suggestions.push("In SWT mode, use 'Button' instead of 'JButton'".to_string());
        }

        // Check for SWT-specific types in Swing mode
        if self.mode == GuiMode::Swing && locator.starts_with("name:") {
            suggestions.push(format!(
                "CSS-style syntax recommended: [name='{}']",
                &locator[5..]
            ));
        }

        // Check for common typos
        if locator.contains("Textfield") {
            suggestions.push("Did you mean 'TextField' (capital F)?".to_string());
        }

        suggestions
    }
}
```

## Consequences

### Positive

1. **User Flexibility**: Users can use any syntax style they prefer
2. **Backwards Compatibility**: All existing locators continue to work
3. **Reduced Learning Curve**: Users can start with familiar syntax
4. **Test Portability**: Same locators can work across technologies
5. **Better Error Messages**: Normalization layer can provide suggestions
6. **Performance**: Caching prevents repeated parsing

### Negative

1. **Implementation Complexity**: Supporting multiple formats adds code
2. **Ambiguity Risk**: Some locators might parse differently than expected
3. **Documentation Overhead**: Must document all supported formats
4. **Testing Burden**: Must test all format combinations

### Risks

1. **Edge Cases**: Complex locators might not normalize correctly
2. **Performance Impact**: Normalization adds processing overhead
3. **Format Conflicts**: Rare cases where formats could be ambiguous

## Alternatives Considered

### Alternative 1: Force Single Syntax

Require all locators to use CSS-style syntax, deprecate others.

**Rejected because**:
- Would break many existing tests
- SWT-style prefixes are intuitive for simple cases
- XPath is powerful for complex queries

### Alternative 2: Separate Parsers Per Technology

Keep separate locator parsers, no normalization.

**Rejected because**:
- Doesn't solve user confusion
- Doesn't improve test portability
- Misses opportunity for unified error handling

### Alternative 3: Runtime Format Detection Only

Detect format at runtime without normalization to internal representation.

**Rejected because**:
- Makes backend implementation harder
- Loses opportunity for optimization
- Harder to add new features consistently

## Implementation Plan

1. **Phase 1**: Implement NormalizedLocator enum and basic parsing (1 week)
2. **Phase 2**: Add CSS selector parsing with pest grammar update (1 week)
3. **Phase 3**: Implement prefix locator conversion (3 days)
4. **Phase 4**: Add XPath normalization (3 days)
5. **Phase 5**: Implement caching and performance optimization (3 days)
6. **Phase 6**: Add error suggestions and help text (3 days)
7. **Phase 7**: Update tests and documentation (1 week)

## References

- [CSS Selectors Level 4](https://www.w3.org/TR/selectors-4/)
- [XPath 3.1](https://www.w3.org/TR/xpath-31/)
- [Current Locator Implementation](/src/locator/mod.rs)
- [Unified Keywords Research - Locator Section](/docs/unify_keywords_research.md#locator-syntax-comparison)
