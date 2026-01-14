# Cascaded Selector Specification for robotframework-swing

## Document Information

| Field | Value |
|-------|-------|
| Version | 1.0.0 |
| Status | Draft |
| Author | Claude |
| Date | 2025-01-14 |
| Based On | [Robot Framework Browser Library](https://marketsquare.github.io/robotframework-browser/Browser.html) |

---

## 1. Overview

### 1.1 Purpose

This specification defines the cascaded selector syntax for the robotframework-swing library, enabling complex element location through chained selectors. The syntax is inspired by the [Robot Framework Browser Library](https://robotframework-browser.org/) implementation, which uses the `>>` separator to chain multiple selector strategies.

### 1.2 Goals

1. Enable navigation through complex Swing component hierarchies
2. Support mixing different selector strategies (CSS, XPath, name, text, etc.)
3. Provide a `*` prefix for capturing intermediate elements
4. Integrate seamlessly with existing Swing component types
5. Maintain backward compatibility with current locator syntax

### 1.3 References

- [Browser Library Documentation](https://marketsquare.github.io/robotframework-browser/Browser.html)
- [Forum Discussion on >> Syntax](https://forum.robotframework.org/t/explanation-on-browser-selector-with/2324)
- Current locator implementation: `/mnt/c/workspace/robotframework-swing/src/locator/`

---

## 2. Syntax Specification

### 2.1 Basic Cascaded Selector

```
selector1 >> selector2 >> selector3 >> ...
```

Each selector in the chain is evaluated in sequence:
1. `selector1` is evaluated against the root component tree
2. `selector2` is evaluated within the context of `selector1` results
3. The pattern continues until the final selector
4. **Only the final selector's matches are returned**

### 2.2 Grammar Definition

```ebnf
cascaded_locator   = selector_segment ( ">>" selector_segment )*
selector_segment   = capture_prefix? engine_prefix? selector_body
capture_prefix     = "*"
engine_prefix      = engine_name "="
engine_name        = "class" | "name" | "text" | "index" | "xpath" | "id" | "css"
selector_body      = css_selector | xpath_selector | simple_value
css_selector       = <as defined in grammar.pest>
xpath_selector     = ("/" | "//") xpath_expression
simple_value       = quoted_string | identifier
quoted_string      = "'" [^']* "'" | '"' [^"]* '"'
identifier         = [a-zA-Z_][a-zA-Z0-9_-]*
```

### 2.3 Whitespace Rules

- Whitespace around `>>` is **optional** but recommended for readability
- Leading/trailing whitespace is trimmed from each segment
- Whitespace within quoted strings is preserved

```robot
# All equivalent:
JPanel[name='main'] >> JButton[text='OK']
JPanel[name='main']>>JButton[text='OK']
JPanel[name='main']  >>  JButton[text='OK']
```

---

## 3. Selector Engines

### 3.1 Engine Overview

| Engine | Prefix | Description | Example |
|--------|--------|-------------|---------|
| CSS | `css=` (default) | CSS-style selector | `css=JButton#submit` |
| class | `class=` | Component class name | `class=JButton` |
| name | `name=` | Component name property | `name=submitBtn` |
| text | `text=` | Text content matching | `text=OK` |
| index | `index=` | Zero-based index | `index=0` |
| xpath | `xpath=` | Relative XPath | `xpath=../JButton` |
| id | `id=` | Internal name/ID | `id=mainPanel` |

### 3.2 CSS Engine (Default)

When no engine prefix is specified, the CSS engine is used. This engine supports the full CSS-like selector syntax already implemented.

**Supported Patterns:**
```
Type                      # JButton, JTextField
Type#id                   # JButton#submit
Type.class                # JButton.primary
Type[attr='value']        # JButton[text='OK']
Type[attr*='value']       # JButton[text*='Log']
Type[attr^='value']       # JButton[text^='Log']
Type[attr$='value']       # JLabel[text$=':']
Type:pseudo               # JButton:enabled
Type:nth-child(n)         # JButton:nth-child(2)
Parent > Child            # JPanel > JButton
Ancestor Descendant       # JFrame JButton
```

**Cascaded Example:**
```
JTabbedPane[name='tabs'] >> JPanel >> JButton[text='Save']
```

### 3.3 Class Engine

Matches components by their Java class simple name.

**Syntax:**
```
class=SimpleClassName
class=JButton
class=JTextField
```

**Behavior:**
- Matches `component.component_type.simple_name`
- Case-insensitive matching
- Supports both `JButton` and `Button` (J prefix optional)

**Cascaded Example:**
```
class=JDialog >> class=JPanel >> class=JButton
```

### 3.4 Name Engine

Matches components by their `name` property (set via `setName()`).

**Syntax:**
```
name=componentName
name='component-name'
```

**Behavior:**
- Matches `component.identity.name`
- Case-sensitive by default
- Supports wildcards: `name=user*` (prefix match)

**Cascaded Example:**
```
name=mainDialog >> name=buttonPanel >> name=okButton
```

### 3.5 Text Engine

Matches components by visible text content.

**Syntax:**
```
text=ExactText
text='Text with spaces'
text=/regex pattern/
```

**Behavior:**
- Matches against `identity.text`, `identity.title`, `accessibility.accessible_name`
- Exact match by default
- Regex support with `/pattern/` syntax
- Partial match with `text=*partial*`

**Cascaded Example:**
```
JMenu >> text=File >> JMenuItem >> text=Save
```

### 3.6 Index Engine

Selects component by zero-based index among siblings.

**Syntax:**
```
index=0          # First element
index=2          # Third element
index=-1         # Last element
```

**Behavior:**
- Applied to the results of the previous selector
- Negative indices count from end
- Out-of-range indices return empty result

**Cascaded Example:**
```
JTable[name='data'] >> row >> index=5 >> cell >> index=0
```

### 3.7 XPath Engine (Relative)

Supports relative XPath expressions within the context.

**Syntax:**
```
xpath=./child
xpath=../sibling
xpath=descendant::JButton
```

**Behavior:**
- XPath is relative to current context (not document root)
- Supports standard XPath axes: `parent`, `child`, `descendant`, `ancestor`, `following-sibling`, `preceding-sibling`
- Absolute paths (`/`) are treated as relative to context root

**Cascaded Example:**
```
JTable[name='users'] >> xpath=.//td[1] >> text=Active
```

### 3.8 ID Engine

Matches components by internal name or developer-assigned ID.

**Syntax:**
```
id=internalName
id='component-id'
```

**Behavior:**
- Matches `identity.internal_name` first, then `identity.name`
- Equivalent to `#name` in CSS syntax
- Case-sensitive

**Cascaded Example:**
```
id=mainWindow >> id=contentPane >> id=submitButton
```

---

## 4. Capture Prefix (`*`)

### 4.1 Purpose

The `*` prefix allows capturing an **intermediate** element in the chain rather than the final result.

### 4.2 Syntax

```
*selector >> subsequent_selector
```

### 4.3 Behavior

1. The asterisk marks which element should be returned
2. Subsequent selectors are still evaluated (for filtering)
3. If multiple elements have `*`, only the **first** marked element is captured
4. If no `*` is present, the final element is returned (default behavior)

### 4.4 Examples

**Capture Parent Container:**
```robot
# Returns the JPanel that contains a JTextField, not the JTextField itself
${panel}=    Get Element    *JPanel >> JTextField[name='username']
```

**Capture Intermediate Dialog:**
```robot
# Returns the JDialog containing a specific button
${dialog}=    Get Element    *JDialog >> JPanel >> JButton[text='OK']
```

**Capture Table Row:**
```robot
# Returns the row containing specific cell content
${row}=    Get Element    JTable >> *row >> cell[text='Active']
```

### 4.5 Multiple Operations on Captured Element

```robot
# Get the container panel
${container}=    Get Element    *JPanel[name='form'] >> JTextField

# Now perform multiple operations on the container
${buttons}=    Get Elements    ${container} >> JButton
${labels}=    Get Elements    ${container} >> JLabel
Click    ${container} >> JButton[text='Submit']
```

---

## 5. Swing Component-Specific Selectors

### 5.1 Table Selectors

**Cell Selection:**
```
JTable[name='dataTable'] >> cell[row=0, col=1]
JTable[name='dataTable'] >> cell[row=0, col='Name']
JTable >> row[index=5] >> cell[index=2]
```

**Row Selection:**
```
JTable >> row[index=0]              # First row
JTable >> row[contains='Active']    # Row containing text
JTable >> row:selected              # Currently selected row
```

**Column Selection:**
```
JTable >> column[name='Status']
JTable >> column[index=3]
JTable >> header >> cell[text='Name']
```

**Table-Specific Pseudo-Classes:**
```
JTable >> row:first
JTable >> row:last
JTable >> cell:selected
JTable >> cell:editable
```

### 5.2 Tree Selectors

**Node by Path:**
```
JTree[name='tree'] >> node[path='Root|Child|Leaf']
JTree >> node[path='Root/Settings/Display']
```

**Node by Text:**
```
JTree >> node[text='Settings']
JTree >> node[text*='Config']
```

**Tree Navigation:**
```
JTree >> node[path='Root'] >> child[index=2]
JTree >> node:selected
JTree >> node:expanded
JTree >> node[level=2]
```

**Tree-Specific Pseudo-Classes:**
```
JTree >> node:root
JTree >> node:leaf
JTree >> node:expanded
JTree >> node:collapsed
JTree >> node:selected
```

### 5.3 TabbedPane Selectors

**Tab Selection:**
```
JTabbedPane[name='tabs'] >> tab[title='Settings']
JTabbedPane >> tab[index=2]
JTabbedPane >> tab:selected
```

**Tab Content:**
```
JTabbedPane >> tab[title='Login'] >> JPanel
JTabbedPane[name='main'] >> JPanel[text='Settings']
```

### 5.4 Menu Selectors

**Menu Navigation:**
```
JMenuBar >> JMenu[text='File'] >> JMenuItem[text='Save']
JMenuBar >> menu[text='Edit'] >> menu[text='Find'] >> item[text='Find Next']
```

**Popup Menu:**
```
JPopupMenu >> JMenuItem[text='Copy']
JPopupMenu >> item[index=0]
```

### 5.5 ComboBox Selectors

**Item Selection:**
```
JComboBox[name='country'] >> item[text='USA']
JComboBox >> item[index=0]
JComboBox >> item:selected
```

### 5.6 List Selectors

**List Item Selection:**
```
JList[name='files'] >> item[text='document.txt']
JList >> item[index=5]
JList >> item:selected
```

### 5.7 Dialog/Frame Selectors

**Window Navigation:**
```
JDialog[title='Confirm'] >> JButton[text='Yes']
JFrame[title='Main'] >> JPanel >> JButton
JInternalFrame[name='editor'] >> JTextArea
```

---

## 6. AST Representation

### 6.1 CascadedLocator Structure

```rust
/// Represents a cascaded locator with multiple segments
#[derive(Debug, Clone)]
pub struct CascadedLocator {
    /// Ordered list of selector segments
    pub segments: Vec<SelectorSegment>,
    /// Original input string
    pub original: String,
}

/// A single segment in a cascaded locator
#[derive(Debug, Clone)]
pub struct SelectorSegment {
    /// Whether this segment should be captured (*)
    pub capture: bool,
    /// The selector engine to use
    pub engine: SelectorEngine,
    /// The parsed selector content
    pub selector: LocatorExpression,
    /// Raw text of this segment (for debugging)
    pub raw: String,
}

/// Selector engine types
#[derive(Debug, Clone, PartialEq)]
pub enum SelectorEngine {
    /// CSS-style selector (default)
    Css,
    /// Class name match
    Class,
    /// Name property match
    Name,
    /// Text content match
    Text,
    /// Index-based selection
    Index(i32),
    /// Relative XPath
    XPath,
    /// ID/internal name match
    Id,
}
```

### 6.2 Extended Component Selectors

```rust
/// Table-specific selector
#[derive(Debug, Clone)]
pub struct TableSelector {
    pub table_locator: Box<LocatorExpression>,
    pub target: TableTarget,
}

#[derive(Debug, Clone)]
pub enum TableTarget {
    Cell { row: RowSpec, col: ColSpec },
    Row(RowSpec),
    Column(ColSpec),
    Header,
}

#[derive(Debug, Clone)]
pub enum RowSpec {
    Index(i32),
    Contains(String),
    Selected,
    First,
    Last,
}

#[derive(Debug, Clone)]
pub enum ColSpec {
    Index(i32),
    Name(String),
}

/// Tree-specific selector
#[derive(Debug, Clone)]
pub struct TreeSelector {
    pub tree_locator: Box<LocatorExpression>,
    pub target: TreeTarget,
}

#[derive(Debug, Clone)]
pub enum TreeTarget {
    Node(NodeSpec),
    Children,
}

#[derive(Debug, Clone)]
pub enum NodeSpec {
    Path(String),          // "Root|Child|Leaf"
    Text(String),          // Direct text match
    Level(u32),            // Depth level
    Index(i32),            // Child index
    Root,
    Selected,
    Expanded,
}
```

---

## 7. Parsing Implementation

### 7.1 Parser Entry Point

```rust
/// Parse a cascaded locator string
pub fn parse_cascaded_locator(input: &str) -> Result<CascadedLocator, LocatorParseError> {
    let segments = split_cascaded_segments(input)?;
    let mut parsed_segments = Vec::new();

    for segment in segments {
        parsed_segments.push(parse_segment(&segment)?);
    }

    Ok(CascadedLocator {
        segments: parsed_segments,
        original: input.to_string(),
    })
}

/// Split input by >> separator, respecting quotes and brackets
fn split_cascaded_segments(input: &str) -> Result<Vec<String>, LocatorParseError> {
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';
    let mut bracket_depth = 0;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = c;
                current.push(c);
            }
            c if c == quote_char && in_quotes => {
                in_quotes = false;
                current.push(c);
            }
            '[' | '(' if !in_quotes => {
                bracket_depth += 1;
                current.push(c);
            }
            ']' | ')' if !in_quotes => {
                bracket_depth -= 1;
                current.push(c);
            }
            '>' if !in_quotes && bracket_depth == 0 => {
                if chars.peek() == Some(&'>') {
                    chars.next(); // consume second >
                    let trimmed = current.trim().to_string();
                    if !trimmed.is_empty() {
                        segments.push(trimmed);
                    }
                    current.clear();
                } else {
                    current.push(c); // Single > is child combinator in CSS
                }
            }
            _ => current.push(c),
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        segments.push(trimmed);
    }

    Ok(segments)
}
```

### 7.2 Segment Parser

```rust
/// Parse a single selector segment
fn parse_segment(input: &str) -> Result<SelectorSegment, LocatorParseError> {
    let trimmed = input.trim();

    // Check for capture prefix
    let (capture, rest) = if trimmed.starts_with('*') {
        (true, &trimmed[1..])
    } else {
        (false, trimmed)
    };

    // Check for engine prefix
    let (engine, selector_str) = parse_engine_prefix(rest)?;

    // Parse the selector body based on engine
    let selector = match engine {
        SelectorEngine::Css => LocatorExpression::parse(selector_str)?,
        SelectorEngine::XPath => parse_relative_xpath(selector_str)?,
        SelectorEngine::Index(idx) => create_index_selector(idx),
        SelectorEngine::Text => create_text_selector(selector_str),
        SelectorEngine::Name => create_name_selector(selector_str),
        SelectorEngine::Class => create_class_selector(selector_str),
        SelectorEngine::Id => create_id_selector(selector_str),
    };

    Ok(SelectorSegment {
        capture,
        engine,
        selector,
        raw: input.to_string(),
    })
}

/// Parse engine prefix (e.g., "css=", "text=", "xpath=")
fn parse_engine_prefix(input: &str) -> Result<(SelectorEngine, &str), LocatorParseError> {
    if let Some(eq_pos) = input.find('=') {
        let prefix = &input[..eq_pos].to_lowercase();
        let rest = &input[eq_pos + 1..];

        let engine = match prefix.as_str() {
            "css" => SelectorEngine::Css,
            "class" => SelectorEngine::Class,
            "name" => SelectorEngine::Name,
            "text" => SelectorEngine::Text,
            "xpath" => SelectorEngine::XPath,
            "id" => SelectorEngine::Id,
            "index" => {
                let idx: i32 = rest.parse().map_err(|_| {
                    LocatorParseError::new("Invalid index value")
                })?;
                return Ok((SelectorEngine::Index(idx), ""));
            }
            _ => return Ok((SelectorEngine::Css, input)), // Unknown prefix, treat as CSS
        };

        Ok((engine, rest))
    } else {
        // No prefix, default to CSS
        Ok((SelectorEngine::Css, input))
    }
}
```

---

## 8. Matching Implementation

### 8.1 Cascaded Matcher

```rust
/// Find elements matching a cascaded locator
pub fn find_cascaded<'a>(
    locator: &CascadedLocator,
    root: &'a UIComponent,
) -> Vec<&'a UIComponent> {
    let mut current_results: Vec<&UIComponent> = vec![root];
    let mut captured: Option<Vec<&UIComponent>> = None;

    for (idx, segment) in locator.segments.iter().enumerate() {
        let mut next_results = Vec::new();

        for context in &current_results {
            let matches = find_in_context(&segment.selector, context, &segment.engine);
            next_results.extend(matches);
        }

        // Handle capture prefix
        if segment.capture && captured.is_none() {
            captured = Some(next_results.clone());
        }

        current_results = next_results;

        // Early exit if no matches
        if current_results.is_empty() {
            break;
        }
    }

    // Return captured elements if any, otherwise final results
    captured.unwrap_or(current_results)
}

/// Find elements within a specific context
fn find_in_context<'a>(
    selector: &LocatorExpression,
    context: &'a UIComponent,
    engine: &SelectorEngine,
) -> Vec<&'a UIComponent> {
    match engine {
        SelectorEngine::Css => find_css_matches(selector, context),
        SelectorEngine::XPath => find_xpath_matches(selector, context),
        SelectorEngine::Class => find_by_class(selector, context),
        SelectorEngine::Name => find_by_name(selector, context),
        SelectorEngine::Text => find_by_text(selector, context),
        SelectorEngine::Id => find_by_id(selector, context),
        SelectorEngine::Index(idx) => find_by_index(*idx, context),
    }
}
```

### 8.2 Context-Aware Search

```rust
/// Search within a context component's descendants
fn find_css_matches<'a>(
    selector: &LocatorExpression,
    context: &'a UIComponent,
) -> Vec<&'a UIComponent> {
    let evaluator = Evaluator::new();
    let mut results = Vec::new();

    // Search all descendants of context
    search_descendants(context, &selector, &evaluator, &mut results);

    results
}

fn search_descendants<'a>(
    component: &'a UIComponent,
    selector: &LocatorExpression,
    evaluator: &Evaluator,
    results: &mut Vec<&'a UIComponent>,
) {
    // Check current component
    if matches_selector(component, selector, evaluator) {
        results.push(component);
    }

    // Recurse into children
    if let Some(ref children) = component.children {
        for child in children {
            search_descendants(child, selector, evaluator, results);
        }
    }
}
```

---

## 9. Integration with Python API

### 9.1 Keyword Support

```python
def get_element(self, locator: str) -> SwingElement:
    """
    Get a single element matching the locator.

    Supports cascaded selectors using >> separator.

    Examples:
        Get Element    JButton[text='OK']
        Get Element    JDialog >> JButton[text='OK']
        Get Element    *JPanel >> JTextField[name='user']
    """
    pass

def get_elements(self, locator: str) -> List[SwingElement]:
    """
    Get all elements matching the locator.

    Examples:
        Get Elements    JTable >> row >> cell
        Get Elements    JTree >> node:expanded
    """
    pass

def click(self, locator: str):
    """
    Click element matching locator.

    Examples:
        Click    JTabbedPane >> tab[title='Settings'] >> JButton[text='Apply']
    """
    pass
```

### 9.2 Element Reference Usage

```robot
*** Test Cases ***
Work With Captured Container
    # Capture the form panel
    ${form}=    Get Element    *JPanel[name='userForm'] >> JTextField

    # Use captured element as context for further operations
    Input Text    ${form} >> JTextField[name='username']    admin
    Input Text    ${form} >> JTextField[name='password']    secret
    Click    ${form} >> JButton[text='Submit']

Navigate Table Data
    # Find specific cell
    ${cell}=    Get Element    JTable[name='users'] >> cell[row=0, col='Status']
    ${text}=    Get Text    ${cell}
    Should Be Equal    ${text}    Active

    # Click action button in specific row
    Click    JTable[name='users'] >> row[contains='John'] >> JButton[text='Edit']

Tree Node Selection
    # Select a node by path
    Click    JTree[name='nav'] >> node[path='Settings|Display|Resolution']

    # Get all expanded nodes
    ${nodes}=    Get Elements    JTree >> node:expanded
    Log Many    @{nodes}
```

---

## 10. Error Handling

### 10.1 Error Types

```rust
#[derive(Debug)]
pub enum CascadedLocatorError {
    /// Invalid separator usage
    InvalidSeparator(String),
    /// Unknown engine prefix
    UnknownEngine(String),
    /// Parse error in segment
    SegmentParseError {
        segment_index: usize,
        segment_text: String,
        inner: LocatorParseError,
    },
    /// Empty segment
    EmptySegment(usize),
    /// No elements found at intermediate stage
    IntermediateNoMatch {
        segment_index: usize,
        segment_text: String,
    },
}
```

### 10.2 Error Messages

```
CascadedLocatorError: Invalid separator at position 15
  Locator: JPanel[name='x' >> JButton
                         ^
  Hint: Ensure >> separator is not inside brackets or quotes

CascadedLocatorError: No elements matched at segment 2
  Locator: JDialog >> JPanel >> JButton[text='Missing']
                               ^^^^^^^^^^^^^^^^^^^^^^^
  Context: 3 elements from previous segment
  Hint: Check if the button text is correct
```

---

## 11. Performance Considerations

### 11.1 Optimization Strategies

1. **Early Termination**: Stop searching when no matches at any stage
2. **Index Caching**: Cache parsed locators for repeated use
3. **Context Limiting**: Only search descendants of context, not full tree
4. **Result Limiting**: Support `first` modifier to stop after first match

### 11.2 Best Practices

```robot
# Good: Narrow context early
Click    JDialog[name='settings'] >> JButton[text='OK']

# Less efficient: Broad initial search
Click    * >> JDialog >> * >> JButton[text='OK']

# Good: Use index for known positions
Click    JTable >> row >> index=0 >> cell >> index=2

# Good: Use capture to reuse context
${dialog}=    Get Element    *JDialog[title='Preferences'] >> JButton
Click    ${dialog} >> text=OK
Click    ${dialog} >> text=Cancel
Click    ${dialog} >> text=Apply
```

---

## 12. Compatibility Matrix

### 12.1 Swing Component Support

| Component | Selector Support | Special Selectors |
|-----------|-----------------|-------------------|
| JFrame | Full | title, name |
| JDialog | Full | title, modal |
| JPanel | Full | name, class |
| JButton | Full | text, enabled |
| JTextField | Full | text, editable |
| JTextArea | Full | text, editable |
| JComboBox | Full | item[], selected |
| JList | Full | item[], selected |
| JTable | Full | cell[], row[], column[] |
| JTree | Full | node[], path |
| JTabbedPane | Full | tab[], selected |
| JMenu | Full | text |
| JMenuItem | Full | text |
| JToolBar | Full | name |
| JScrollPane | Full | - |
| JSplitPane | Full | - |
| JProgressBar | Partial | value |
| JSlider | Partial | value |
| JSpinner | Partial | value |

### 12.2 Selector Engine Compatibility

| Engine | CSS | Class | Name | Text | Index | XPath | ID |
|--------|-----|-------|------|------|-------|-------|-----|
| Chaining | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Capture (*) | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Pseudo-classes | Yes | No | No | No | No | Yes | No |
| Attributes | Yes | No | No | No | No | Yes | No |
| Combinators | Yes | No | No | No | No | Yes | No |

---

## 13. Examples Catalog

### 13.1 Basic Examples

```robot
# Simple chaining
JDialog >> JButton[text='OK']

# Multiple levels
JFrame >> JTabbedPane >> JPanel >> JButton

# Mixed engines
JDialog[title='Settings'] >> name=contentPanel >> text=Apply
```

### 13.2 Table Examples

```robot
# Get specific cell
JTable[name='users'] >> cell[row=0, col=1]

# Get cell by column name
JTable >> cell[row=2, col='Email']

# Click button in row containing text
JTable >> row[contains='admin'] >> JButton[text='Delete']

# Get all cells in column
${cells}=    Get Elements    JTable >> column[name='Status'] >> cell
```

### 13.3 Tree Examples

```robot
# Select node by path
Click    JTree >> node[path='Root|Settings|Display']

# Expand parent, then select child
Click    JTree >> node[path='Root|Settings']    # Expands it
Click    JTree >> node[path='Root|Settings|Display']

# Get all leaf nodes
${leaves}=    Get Elements    JTree >> node:leaf
```

### 13.4 Capture Examples

```robot
# Capture dialog for multiple operations
${dialog}=    Get Element    *JDialog[title='Edit User'] >> JTextField
Input Text    ${dialog} >> JTextField[name='name']    John
Input Text    ${dialog} >> JTextField[name='email']    john@example.com
Click    ${dialog} >> JButton[text='Save']

# Capture table row
${row}=    Get Element    JTable >> *row[contains='Active'] >> cell
${name}=    Get Text    ${row} >> cell[col='Name']
${email}=    Get Text    ${row} >> cell[col='Email']
```

### 13.5 Complex Examples

```robot
# Navigate through nested tabs
Click    JTabbedPane[name='main'] >> tab[title='Advanced'] >>
...      JTabbedPane >> tab[title='Network'] >> JButton[text='Configure']

# Find button in specific dialog and panel combination
${btn}=    Get Element
...    JDialog[title='Preferences'] >>
...    JTabbedPane >>
...    tab[title='Display'] >>
...    JPanel[name='colorSettings'] >>
...    JButton[text='Choose Color']

# Table with nested components
Click    JTable[name='orders'] >>
...      row[contains='Pending'] >>
...      cell[col='Actions'] >>
...      JComboBox >>
...      item[text='Ship']
```

---

## 14. Migration Guide

### 14.1 From Simple Locators

```robot
# Before: Multiple keyword calls
Select Tab    tabPane    Settings
Click    settingsPanel >> JButton[text='Apply']

# After: Single cascaded locator
Click    JTabbedPane[name='tabPane'] >> tab[title='Settings'] >> JButton[text='Apply']
```

### 14.2 From XPath

```robot
# Before: Full XPath
Click    //JDialog[@title='Settings']//JPanel[@name='buttons']//JButton[@text='OK']

# After: Cascaded selector (more readable)
Click    JDialog[title='Settings'] >> JPanel[name='buttons'] >> JButton[text='OK']
```

---

## 15. Future Enhancements

### 15.1 Planned Features

1. **Frame Support (`>>>`)**: Similar to Browser Library's iframe support
   ```
   JInternalFrame >>> JButton[text='OK']
   ```

2. **Wait Conditions in Chain**:
   ```
   JDialog:visible >> JButton:enabled >> text=OK
   ```

3. **Relative Position Selectors**:
   ```
   JLabel[text='Username:'] >> right-of >> JTextField
   ```

4. **Multiple Captures**:
   ```
   *JDialog >> *JPanel >> JButton    # Returns both dialog and panel
   ```

### 15.2 Potential Extensions

- Shadow DOM-like component support for complex nested panels
- Custom selector engine plugins
- Fuzzy text matching engine
- AI-assisted selector suggestions

---

## Appendix A: Grammar Reference

```pest
// Cascaded locator grammar extension for grammar.pest

cascaded_locator = { selector_segment ~ (cascade_separator ~ selector_segment)* }

cascade_separator = { explicit_ws* ~ ">>" ~ explicit_ws* }

selector_segment = {
    capture_prefix? ~ engine_prefix? ~ segment_body
}

capture_prefix = { "*" }

engine_prefix = {
    ("css" | "class" | "name" | "text" | "index" | "xpath" | "id") ~ "="
}

segment_body = {
    xpath_expr | css_selector_list | simple_value
}

simple_value = { quoted_string | simple_identifier }

simple_identifier = @{ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_" | "-")* }

// Table-specific selectors
table_selector = {
    "cell" ~ "[" ~ cell_spec ~ "]" |
    "row" ~ ("[" ~ row_spec ~ "]")? |
    "column" ~ "[" ~ column_spec ~ "]" |
    "header"
}

cell_spec = {
    "row" ~ "=" ~ integer ~ "," ~ explicit_ws* ~ "col" ~ "=" ~ (integer | quoted_string)
}

row_spec = {
    "index" ~ "=" ~ integer |
    "contains" ~ "=" ~ quoted_string
}

column_spec = {
    "index" ~ "=" ~ integer |
    "name" ~ "=" ~ quoted_string
}

// Tree-specific selectors
tree_selector = {
    "node" ~ "[" ~ node_spec ~ "]"
}

node_spec = {
    "path" ~ "=" ~ quoted_string |
    "text" ~ "=" ~ quoted_string |
    "level" ~ "=" ~ integer
}

integer = @{ "-"? ~ ASCII_DIGIT+ }
```

---

## Appendix B: Test Cases

```robot
*** Test Cases ***
Basic Cascaded Selector
    Element Should Exist    JFrame >> JButton
    Element Should Exist    JDialog >> JPanel >> JButton[text='OK']

Engine Prefix Selection
    Element Should Exist    class=JDialog >> name=submitBtn
    Element Should Exist    JTable >> xpath=.//td[1]
    Element Should Exist    JComboBox >> text=Option 1
    Element Should Exist    JList >> index=0

Capture Prefix
    ${panel}=    Get Element    *JPanel >> JTextField
    Should Not Be Empty    ${panel}
    Element Should Be Visible    ${panel}

    ${dialog}=    Get Element    *JDialog >> *JPanel >> JButton
    Click    ${dialog} >> JButton[text='Cancel']

Table Selectors
    Element Should Exist    JTable >> cell[row=0, col=0]
    Element Should Exist    JTable >> cell[row=0, col='Name']
    Element Should Exist    JTable >> row[index=0]
    Element Should Exist    JTable >> row[contains='Active']

Tree Selectors
    Element Should Exist    JTree >> node[path='Root|Child']
    Element Should Exist    JTree >> node[text='Settings']
    Element Should Exist    JTree >> node:expanded

Complex Combinations
    Element Should Exist
    ...    JFrame >> JTabbedPane >> tab[title='Settings'] >>
    ...    JPanel >> JTable >> cell[row=0, col='Action'] >> JButton

Error Cases
    [Documentation]    These should fail gracefully with clear messages
    Run Keyword And Expect Error    *No elements matched*
    ...    Get Element    JDialog >> JButton[text='NonExistent']

    Run Keyword And Expect Error    *Invalid separator*
    ...    Get Element    JPanel[name='x' >> JButton
```

---

## Document Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0 | 2025-01-14 | Claude | Initial specification |

