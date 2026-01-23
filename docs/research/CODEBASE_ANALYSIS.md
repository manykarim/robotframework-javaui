# Codebase Analysis Report: robotframework-swing

## Executive Summary

This analysis covers the robotframework-swing codebase to understand current implementation patterns for keyword API modernization. The library is a comprehensive Rust-based Robot Framework library for automating Java Swing, SWT, and RCP applications with PyO3 Python bindings.

**Key Findings:**
- Modern Rust implementation with PyO3 bindings (not legacy Python)
- Comprehensive locator system with CSS-like and XPath support via Pest parser
- JSON-RPC communication protocol to Java agent over TCP
- Well-structured error handling with diagnostic context
- Unified library architecture supporting Swing, SWT, and RCP

---

## 1. File Structure and Module Organization

### Source Code Structure (`src/`)

```
src/
├── lib.rs                      # Main crate entry point, PyO3 module exports
├── error.rs                    # Comprehensive error types with context
├── connection/
│   └── mod.rs                  # TCP connection management
├── core/
│   ├── mod.rs                  # Core module exports
│   ├── backend.rs              # ToolkitType enum (Swing/SWT/RCP)
│   ├── config.rs               # Configuration management
│   ├── element.rs              # Unified JavaGuiElement (1355 lines)
│   ├── element_cache.rs        # Element caching for performance
│   ├── tests.rs                # Core unit tests
│   └── type_mapping.rs         # Type normalization across toolkits
├── locator/
│   ├── mod.rs                  # Locator module exports
│   ├── parser.rs               # Pest-based locator parser (906 lines)
│   ├── ast.rs                  # Locator AST types
│   ├── matcher.rs              # Element matching logic
│   ├── swt_matcher.rs          # SWT-specific matching
│   ├── unified.rs              # Unified locator handling
│   ├── unified_tests.rs        # Locator unit tests
│   ├── cache.rs                # Locator caching
│   └── expression.rs           # Locator expressions
├── model/
│   ├── mod.rs                  # Model module exports
│   ├── component.rs            # Component model
│   ├── element.rs              # Element model
│   ├── tree.rs                 # UI tree model
│   ├── widget.rs               # Widget model
│   └── rcp.rs                  # RCP-specific models
├── protocol/
│   └── mod.rs                  # JSON-RPC protocol
└── python/
    ├── mod.rs                  # Python module exports
    ├── base_library.rs         # JavaGuiLibrary (unified, 1253 lines)
    ├── swing_library.rs        # SwingLibrary (1900+ lines)
    ├── swt_library.rs          # SwtLibrary
    ├── rcp_library.rs          # RcpLibrary
    ├── element.rs              # SwingElement PyO3 class
    ├── swt_element.rs          # SwtElement PyO3 class
    ├── unified_element.rs      # Unified element
    ├── exceptions.rs           # SwingError PyO3 exceptions
    ├── unified_exceptions.rs   # Unified exceptions
    └── tests.rs                # Python binding tests
```

### Java Agent Structure (`agent/src/main/java/`)

```
com/robotframework/
├── swing/
│   ├── Agent.java              # Swing agent entry point
│   ├── RpcServer.java          # JSON-RPC TCP server
│   ├── ActionExecutor.java     # UI action execution (400+ lines)
│   ├── ComponentInspector.java # Component tree inspection
│   └── EdtHelper.java          # EDT thread utilities
├── swt/
│   ├── SwtReflectionBridge.java     # SWT reflection utilities
│   ├── SwtReflectionRpcServer.java  # SWT RPC server
│   └── EclipseWorkbenchHelper.java  # RCP workbench utilities
└── UnifiedAgent.java           # Unified agent entry point
```

### Test Structure (`tests/`)

```
tests/
├── robot/
│   ├── swing/                  # 15 Robot test suites
│   │   ├── 01_connection.robot
│   │   ├── 02_element_finding.robot
│   │   ├── 03_buttons.robot
│   │   ├── 04_text_input.robot
│   │   ├── 05_selection.robot
│   │   ├── 06_tables.robot
│   │   ├── 07_trees.robot
│   │   ├── 08_menus.robot
│   │   ├── 09_waits.robot
│   │   ├── 10_verification.robot
│   │   ├── 11_spinner_slider.robot
│   │   ├── 12_tabs.robot
│   │   ├── 13_dialogs.robot
│   │   ├── 14_progressbar.robot
│   │   └── 15_labels.robot
│   ├── swt/                    # 6 SWT test suites
│   └── rcp/                    # 10 RCP test suites
├── python/
│   ├── test_locators.py        # Locator unit tests (341 lines)
│   ├── test_swing_element.py   # Element tests
│   ├── test_swing_library.py   # Library tests
│   ├── test_errors.py          # Error handling tests
│   └── test_integration.py     # Integration tests
└── unit/
    └── test_empty_locator_validation.py
```

---

## 2. Current Keyword Implementations

### SwingLibrary Keywords (src/python/swing_library.rs)

The SwingLibrary is the main keyword library, organized into functional categories:

#### Connection Keywords
| Keyword | Description | Location |
|---------|-------------|----------|
| `connect_to_application` | Connect to Java app via TCP | Lines 156-271 |
| `disconnect_from_application` | Close connection | Lines 273-303 |
| `is_connected` | Check connection status | Lines 305-317 |
| `get_connection_info` | Get connection details | Lines 319-341 |

#### Element Finding Keywords
| Keyword | Description | Location |
|---------|-------------|----------|
| `find_element` | Find single element | Lines 345-366 |
| `find_elements` | Find all matching elements | Lines 368-385 |
| `wait_until_element_exists` | Wait for element | Lines 387-415 |
| `wait_until_element_does_not_exist` | Wait for disappearance | Lines 417-464 |
| `wait_until_element_is_enabled` | Wait for enabled state | Lines 466-481 |
| `wait_until_element_is_visible` | Wait for visibility | Lines 483-498 |

#### Interaction Keywords
| Keyword | Description | Location |
|---------|-------------|----------|
| `click_element` | Single/double click | Lines 504-532 |
| `right_click_element` | Context menu click | Lines 534-558 |
| `click_button` | Click button by text/locator | Lines 560-583 |
| `input_text` | Type text with optional clear | Lines 585-618 |
| `clear_text` | Clear text field | Lines 620-639 |
| `select_from_combobox` | Combobox selection | Lines 641-662 |
| `check_checkbox` | Check checkbox | Lines 664-692 |
| `uncheck_checkbox` | Uncheck checkbox | Lines 694-722 |
| `select_radio_button` | Select radio button | Lines 724-742 |

#### Tab Keywords
| Keyword | Description | Location |
|---------|-------------|----------|
| `select_tab` | Select tab by title/index | Lines 748-783 |

#### List Keywords
| Keyword | Description | Location |
|---------|-------------|----------|
| `select_from_list` | Select list item by text | Lines 789-809 |
| `select_list_item_by_index` | Select list item by index | Lines 811-831 |
| `get_list_items` | Get all list items | Lines 833-861 |

#### Table Keywords
| Keyword | Description | Location |
|---------|-------------|----------|
| `get_table_row_count` | Get row count | Lines 867-888 |
| `get_table_column_count` | Get column count | Lines 900-911 |
| `get_table_cell_value` | Get cell value (row, col) | Lines 913-946 |
| `select_table_row` | Select table row | Lines 948-969 |
| `select_table_cell` | Select table cell | Lines 971-993 |

#### Tree Keywords
| Keyword | Description | Location |
|---------|-------------|----------|
| `expand_tree_node` | Expand node by path | Lines 999-1019 |
| `collapse_tree_node` | Collapse node | Lines 1021-1041 |
| `select_tree_node` | Select node by path | Lines 1043-1063 |
| `get_selected_tree_node` | Get selected node | Lines 1065-1093 |
| `get_tree_data` | Get tree structure | Lines 1095-1123 |

#### Menu Keywords
| Keyword | Description | Location |
|---------|-------------|----------|
| `select_menu` | Navigate menu path | Lines 1129-1159 |
| `select_from_popup_menu` | Select from context menu | Lines 1161-1178 |

#### Inspection Keywords
| Keyword | Description | Location |
|---------|-------------|----------|
| `get_element_text` | Get text content | Lines 1184-1211 |
| `get_element_property` | Get specific property | Lines 1213-1272 |

#### Verification Keywords
| Keyword | Description | Location |
|---------|-------------|----------|
| `element_should_be_enabled` | Assert enabled | Lines 1274-1296 |
| `element_should_be_disabled` | Assert disabled | Lines 1298-1320 |
| `element_should_be_visible` | Assert visible | Lines 1322-1344 |
| `element_should_not_be_visible` | Assert not visible | Lines 1346-1376 |
| `element_text_should_be` | Assert exact text | Lines 1378-1415 |
| `element_text_should_contain` | Assert contains text | Lines 1417-1442 |

#### UI Tree Keywords
| Keyword | Description | Location |
|---------|-------------|----------|
| `get_ui_tree` | Get component hierarchy | Lines 1448-1493 |
| `log_ui_tree` | Log tree to RF log | Lines 1495-1512 |
| `save_ui_tree` | Save tree to file | Lines 1514-1530 |

#### Configuration Keywords
| Keyword | Description | Location |
|---------|-------------|----------|
| `set_timeout` | Set default timeout | Lines 1578-1597 |
| `set_screenshot_directory` | Set screenshot path | Lines 1599-1614 |
| `close_all_dialogs` | Close all open dialogs | Lines 1616-1627 |
| `force_close_dialog` | Force close by name | Lines 1629-1654 |
| `refresh_ui_tree` | Refresh cached tree | Lines 1656-1664 |

### JavaGuiLibrary (Unified Base) Keywords (src/python/base_library.rs)

The unified base library provides toolkit-agnostic keywords:

| Category | Keywords |
|----------|----------|
| Connection | `connect_to_application`, `disconnect`, `is_connected` |
| Clicks | `click`, `double_click`, `right_click` |
| Finding | `find_element`, `find_elements` |
| Text | `input_text`, `clear_text`, `get_text` |
| Selection | `check`, `uncheck`, `select_combo_item`, `select_list_item` |
| Table | `get_table_row_count`, `get_table_cell_value`, `select_table_row` |
| Tree | `expand_tree_node`, `collapse_tree_node`, `select_tree_node` |
| Wait | `wait_until_element_exists`, `wait_until_element_is_enabled` |
| Verification | `element_should_be_visible`, `element_should_be_enabled`, `element_text_should_be` |
| Config | `set_timeout` |

---

## 3. Locator Handling Patterns

### Locator Parser (src/locator/parser.rs)

The library uses a sophisticated Pest-based parser supporting multiple locator strategies:

#### CSS-Like Selectors
```rust
// Grammar (from parser.rs)
locator = { (xpath_locator | css_locator) ~ EOI }
css_locator = { selector_list }
selector = { simple_selector_sequence ~ (combinator ~ simple_selector_sequence)* }
simple_selector_sequence = { type_or_universal? ~ (id_selector | class_selector | attrib_selector | pseudo_selector)* }
```

**Supported Patterns:**
| Pattern | Example | Description |
|---------|---------|-------------|
| Type | `JButton` | Match by class type |
| ID | `#submitButton` | Match by name attribute |
| Attribute | `[name='okBtn']` | Match by attribute |
| Attribute with type | `JButton[name='ok']` | Combined type + attribute |
| Pseudo-class | `JButton:enabled` | Match by state |
| Contains text | `:contains('Submit')` | Match by text content |
| Nth-child | `:nth-child(2)` | Match by index |
| Child combinator | `JPanel > JButton` | Direct child |
| Descendant | `JPanel JButton` | Any descendant |
| Adjacent sibling | `JLabel + JTextField` | Next sibling |

#### XPath Selectors
```rust
xpath_locator = { "//" ~ xpath_step ~ ("/" ~ xpath_step)* }
xpath_step = { xpath_node_test ~ xpath_predicates? }
xpath_predicate = { "[" ~ xpath_expr ~ "]" }
```

**Supported XPath:**
| Pattern | Example | Description |
|---------|---------|-------------|
| Simple | `//JButton` | All JButtons |
| With predicate | `//JButton[@name='ok']` | With attribute |
| Position | `//JTable/JButton[2]` | By position |
| Text | `//JButton[text()='OK']` | By text content |
| Contains | `//JLabel[contains(@text, 'Error')]` | Partial match |
| Boolean ops | `//JButton[@enabled and @visible]` | Multiple conditions |

#### Locator AST (src/locator/ast.rs)

```rust
pub enum Locator {
    Css(CssSelector),
    XPath(XPathExpression),
    Legacy(LegacyLocator),  // name:value format
}

pub struct CssSelector {
    pub selectors: Vec<SelectorChain>,
}

pub struct SelectorChain {
    pub simple_selectors: Vec<SimpleSelector>,
    pub combinator: Option<Combinator>,
}

pub enum SimpleSelector {
    Type(String),
    Id(String),
    Class(String),
    Attribute(AttributeSelector),
    Pseudo(PseudoSelector),
}
```

### Legacy Locator Support

The library maintains backward compatibility with simple locator formats:

```rust
// From swing_library.rs parse_locator()
fn parse_locator(&self, locator: &str) -> (String, String) {
    // "#name" -> ("name", "name")
    // "JButton" -> ("class", "JButton")
    // "JButton#btnName" -> ("name", "btnName")
    // "@text=Login" -> ("text", "Login")
    // "class=javax.swing.JButton" -> ("class", "javax.swing.JButton")
    // "name=myButton" -> ("name", "myButton")
}
```

---

## 4. Assertion/Verification Patterns

### Current Assertion Implementation

All assertions follow a consistent pattern using PyO3's `PyAssertionError`:

```rust
// Pattern from swing_library.rs
pub fn element_should_be_enabled(&self, locator: &str) -> PyResult<()> {
    self.ensure_connected()?;

    let element = self.find_element(locator)?;
    if !element.enabled {
        return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
            "Element '{}' is not enabled",
            locator
        )));
    }
    Ok(())
}
```

### Assertion Keywords

| Keyword | Checks | Implementation |
|---------|--------|----------------|
| `element_should_be_visible` | `visible && showing` | AssertionError if false |
| `element_should_not_be_visible` | `!visible \|\| !showing` | AssertionError if visible |
| `element_should_be_enabled` | `enabled` | AssertionError if disabled |
| `element_should_be_disabled` | `!enabled` | AssertionError if enabled |
| `element_text_should_be` | `text == expected` | AssertionError if mismatch |
| `element_text_should_contain` | `text.contains(expected)` | AssertionError if missing |
| `element_should_exist` | Element found | ElementNotFound error |
| `element_should_not_exist` | Element not found | AssertionError if found |
| `element_should_be_selected` | `selected == true` | AssertionError if not selected |

### Test Patterns (from 10_verification.robot)

```robot
# Positive assertions
Element Should Be Visible    JButton[name='submitButton']
Element Should Be Enabled    [name='submitButton']
Element Text Should Be       JLabel[text='Name:']    Name:
Element Text Should Contain  JLabel[text='Name:']    Nam

# Negative assertions with expected failure
${status}=    Run Keyword And Return Status
...    Element Should Be Visible    JButton[name='nonexistent']
Should Be Equal    ${status}    ${FALSE}
```

---

## 5. Session Management

### Connection State Management

```rust
// From swing_library.rs
pub struct ConnectionState {
    pub connected: bool,
    pub application_name: Option<String>,
    pub pid: Option<u32>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub stream: Option<TcpStream>,
    pub request_id: u64,
}
```

### Session Lifecycle

1. **Connect**: `connect_to_application(application, host, port, timeout)`
   - Establishes TCP connection to Java agent
   - Validates connection with ping/pong
   - Clears element cache

2. **Operations**: All keywords call `ensure_connected()` first
   - Validates connection state
   - Returns error if not connected

3. **Disconnect**: `disconnect_from_application()`
   - Closes TCP stream
   - Resets connection state
   - Clears element cache

### Configuration Management

```rust
pub struct LibraryConfig {
    pub timeout: f64,           // Default: 10.0 seconds
    pub poll_interval: f64,     // Default: 0.5 seconds
    pub log_actions: bool,      // Default: true
    pub screenshot_directory: String,
    pub screenshot_format: String,
}
```

### Library Scope

```rust
#[classattr]
const ROBOT_LIBRARY_SCOPE: &'static str = "GLOBAL";
```

The library uses GLOBAL scope to maintain connection across test cases.

---

## 6. Table/Tree/List Handling Patterns

### Table Keywords Pattern

```rust
// Get cell value with row/column support
pub fn get_table_cell_value(&self, locator: &str, row: i32, column: &str) -> PyResult<String> {
    let component_id = self.get_component_id(locator)?;

    // Parse column - could be index or name
    let col_value: serde_json::Value = if let Ok(col_idx) = column.parse::<i32>() {
        serde_json::json!(col_idx)
    } else {
        serde_json::json!(column)  // Column name
    };

    let result = self.send_rpc_request("getTableCellValue", serde_json::json!({
        "componentId": component_id,
        "row": row,
        "column": col_value
    }))?;

    Ok(result.as_str().unwrap_or("").to_string())
}
```

**Table RPC Methods:**
- `getTableRowCount` - Returns row count
- `getTableColumnCount` - Returns column count
- `getTableCellValue` - Gets cell by row/col
- `selectTableCell` - Selects cell
- `selectTableRow` - Selects entire row (col=0)

### Tree Keywords Pattern

```rust
// Tree path notation: "Root|Parent|Child" or "Root/Parent/Child"
pub fn expand_tree_node(&self, locator: &str, path: &str) -> PyResult<()> {
    let component_id = self.get_component_id(locator)?;

    self.send_rpc_request("expandTreeNode", serde_json::json!({
        "componentId": component_id,
        "path": path
    }))?;

    Ok(())
}
```

**Tree RPC Methods:**
- `expandTreeNode` - Expand by path
- `collapseTreeNode` - Collapse by path
- `selectTreeNode` - Select by path
- `getTreeNodes` - Get tree structure (optional `selectedOnly: true`)

### List Keywords Pattern

```rust
// Select by text or index
pub fn select_from_list(&self, locator: &str, item: &str) -> PyResult<()> {
    let component_id = self.get_component_id(locator)?;

    self.send_rpc_request("selectItem", serde_json::json!({
        "componentId": component_id,
        "value": item
    }))?;

    Ok(())
}

pub fn select_list_item_by_index(&self, locator: &str, index: i32) -> PyResult<()> {
    let component_id = self.get_component_id(locator)?;

    self.send_rpc_request("selectItem", serde_json::json!({
        "componentId": component_id,
        "index": index
    }))?;

    Ok(())
}
```

---

## 7. Error Handling Patterns

### Error Types (src/error.rs)

```rust
pub enum SwingError {
    Connection {
        message: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    ElementNotFound {
        locator: String,
        context: Option<ElementNotFoundContext>,
    },
    ActionFailed {
        action: String,
        reason: String,
    },
    Timeout {
        operation: String,
        timeout_seconds: f64,
    },
    Validation {
        message: String,
    },
    Protocol {
        message: String,
        code: Option<i32>,
    },
}
```

### Element Not Found Context

```rust
pub struct ElementNotFoundContext {
    pub searched_tree: Option<String>,
    pub similar_elements: Vec<String>,
    pub suggestions: Vec<String>,
}

impl SwingError {
    pub fn generate_suggestions(locator: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Suggest checking if element exists
        suggestions.push(format!("Verify the element '{}' exists in the UI tree", locator));

        // Suggest waiting for element
        suggestions.push("Try using 'Wait Until Element Exists' if element appears dynamically".to_string());

        // Suggest checking visibility
        if locator.contains("visible") || locator.contains("showing") {
            suggestions.push("Element might exist but not be visible - scroll into view".to_string());
        }

        suggestions
    }
}
```

### Error Recovery Methods

```rust
impl SwingError {
    pub fn is_recoverable(&self) -> bool {
        matches!(self, SwingError::Timeout { .. } | SwingError::Connection { .. })
    }

    pub fn is_connection_error(&self) -> bool {
        matches!(self, SwingError::Connection { .. })
    }
}
```

---

## 8. Unified Element Model (src/core/element.rs)

### JavaGuiElement

The `JavaGuiElement` struct provides a normalized representation across toolkits:

```rust
#[pyclass(name = "JavaGuiElement")]
pub struct JavaGuiElement {
    pub hash_code: i64,       // Unique identifier
    pub class_name: String,   // Fully qualified class
    pub simple_name: String,  // Short class name
    pub toolkit: String,      // "swing", "swt", "rcp"
    pub element_type: String, // Normalized type (Button, TextField, etc.)
    pub name: Option<String>,
    pub text: Option<String>,
    pub tooltip: Option<String>,
    pub x: i32, pub y: i32,
    pub width: i32, pub height: i32,
    pub visible: bool,
    pub enabled: bool,
    pub focused: bool,
    properties: HashMap<String, Value>,
}
```

### ElementType Enumeration

The library normalizes 30+ element types across toolkits:

| Category | Types |
|----------|-------|
| Buttons | Button, ToggleButton, CheckBox, RadioButton |
| Text Inputs | TextField, TextArea, PasswordField, Spinner |
| Selection | ComboBox, List, Table, Tree |
| Display | Label, ProgressBar, Slider |
| Containers | Panel, Frame, Dialog, Shell, Group, ScrollPane, SplitPane, TabbedPane, TabFolder |
| Menus | MenuBar, Menu, MenuItem, PopupMenu |
| Toolbars | ToolBar, ToolItem |
| RCP-specific | View, Editor, Perspective |
| Generic | Widget, Unknown |

### Type Mapping

```rust
impl ElementType {
    pub fn from_class_name(class_name: &str, toolkit: ToolkitType) -> Self {
        match toolkit {
            ToolkitType::Swing => Self::from_swing_class(simple_name),
            ToolkitType::Swt | ToolkitType::Rcp => Self::from_swt_class(simple_name),
        }
    }

    fn from_swing_class(simple_name: &str) -> Self {
        match simple_name {
            "JButton" => ElementType::Button,
            "JCheckBox" => ElementType::CheckBox,
            "JTextField" | "JFormattedTextField" => ElementType::TextField,
            "JTable" => ElementType::Table,
            "JTree" => ElementType::Tree,
            "JTabbedPane" => ElementType::TabbedPane,
            // ... more mappings
        }
    }
}
```

---

## 9. Java Agent Implementation

### RPC Server (agent/src/main/java/.../RpcServer.java)

The Java agent uses JSON-RPC 2.0 over TCP:

**RPC Methods:**
| Method | Parameters | Description |
|--------|------------|-------------|
| `ping` | None | Health check |
| `findWidgets` | locatorType, value | Find elements |
| `click` | componentId | Click element |
| `doubleClick` | componentId | Double-click |
| `rightClick` | componentId | Context click |
| `typeText` | componentId, text | Type text |
| `clearText` | componentId | Clear text |
| `selectItem` | componentId, value/index | Select item |
| `getTableRowCount` | componentId | Get row count |
| `getTableColumnCount` | componentId | Get col count |
| `getTableCellValue` | componentId, row, column | Get cell |
| `selectTableCell` | componentId, row, column | Select cell |
| `expandTreeNode` | componentId, path | Expand node |
| `collapseTreeNode` | componentId, path | Collapse node |
| `selectTreeNode` | componentId, path | Select node |
| `getTreeNodes` | componentId, selectedOnly? | Get tree data |
| `selectMenu` | path, timeout? | Navigate menu |
| `selectFromPopupMenu` | path | Select from popup |
| `getElementProperties` | componentId | Get all props |
| `getProperty` | componentId, property | Get single prop |
| `closeAllDialogs` | None | Close dialogs |
| `forceCloseDialog` | name | Close by name |

### Action Executor (agent/src/main/java/.../ActionExecutor.java)

All UI actions execute on the Event Dispatch Thread (EDT):

```java
public static void click(int componentId) {
    Component component = ComponentInspector.getComponentById(componentId);

    EdtHelper.runOnEdtLater(() -> {
        if (!component.isShowing()) {
            System.err.println("[SwingAgent] Component not visible");
            return;
        }
        if (component instanceof AbstractButton) {
            ((AbstractButton) component).doClick();
        } else {
            performMouseClick(component, 1);
        }
    });

    EdtHelper.sleep(150);
}
```

---

## 10. Consolidation Opportunities

### Keywords to Consolidate

1. **Click Variants** - Merge into single `click` with options:
   - `click_element`, `click_button` -> `click(locator, click_type='single')`
   - `double_click` option: `click(locator, click_type='double')`
   - `right_click_element` option: `click(locator, click_type='right')`

2. **Wait Keywords** - Unified `wait_for` keyword:
   - `wait_until_element_exists`
   - `wait_until_element_does_not_exist`
   - `wait_until_element_is_enabled`
   - `wait_until_element_is_visible`

   Consolidate to: `wait_for(locator, condition='exists|visible|enabled|not_exists', timeout=None)`

3. **Selection Keywords** - Unified `select` keyword:
   - `select_from_combobox`
   - `select_from_list`
   - `select_list_item_by_index`

   Consolidate to: `select(locator, item=None, index=None)`

4. **Get/Set Properties** - Unified property access:
   - `get_element_text`
   - `get_element_property`

   Consolidate to: `get_property(locator, property='text|enabled|visible|name|...')`

### Assertion Patterns to Leverage

The current assertion pattern is well-structured:

```rust
pub fn element_should_<condition>(&self, locator: &str) -> PyResult<()> {
    self.ensure_connected()?;
    let element = self.find_element(locator)?;
    if !<condition_check> {
        return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
            "Element '{}' <failure_message>",
            locator
        )));
    }
    Ok(())
}
```

This can be extended with:
- `message` parameter for custom error messages
- `ignore_case` parameter for text comparisons
- `contains` parameter for partial matching

### Locator Parser Extensions

The Pest parser can be extended for:

1. **Indexed Selectors**: `JButton:nth(2)` or `JButton[2]`
2. **Relative Locators**: `near:JLabel[text='Name:']`
3. **Chained Conditions**: `JButton:enabled:visible`
4. **Custom Functions**: `has-text('Submit')`

---

## 11. Test Coverage Analysis

### Robot Framework Tests

| Suite | Test Cases | Coverage |
|-------|------------|----------|
| 01_connection | 5+ | Connection lifecycle |
| 02_element_finding | 20+ | Locator strategies |
| 03_buttons | 15+ | Button interactions |
| 04_text_input | 20+ | Text fields, clear, type |
| 05_selection | 15+ | Checkbox, radio, combo |
| 06_tables | 30+ | Cell/row selection, data |
| 07_trees | 25+ | Expand, collapse, select |
| 08_menus | 15+ | Menu navigation |
| 09_waits | 15+ | Wait conditions |
| 10_verification | 45+ | All assertions |
| 11_spinner_slider | 10+ | Numeric inputs |
| 12_tabs | 10+ | Tab selection |
| 13_dialogs | 10+ | Dialog handling |
| 14_progressbar | 5+ | Progress indicators |
| 15_labels | 5+ | Label text |

### Python Unit Tests

| Test File | Focus |
|-----------|-------|
| test_locators.py | CSS/XPath parsing, attribute selectors |
| test_swing_element.py | Element properties, methods |
| test_swing_library.py | Library configuration |
| test_errors.py | Error handling, messages |
| test_integration.py | End-to-end scenarios |

### Coverage Gaps Identified

1. **Edge Cases**: More tests needed for:
   - Empty locators
   - Escaped special characters
   - Very long text values
   - Unicode text

2. **Error Scenarios**: Missing tests for:
   - Network disconnection recovery
   - Agent crash handling
   - Concurrent access

3. **Performance**: No tests for:
   - Large UI tree handling
   - Many rapid operations
   - Memory usage

---

## 12. Dependencies and Integration Points

### Rust Dependencies (Cargo.toml)

- **pyo3** - Python bindings
- **serde/serde_json** - JSON serialization
- **pest/pest_derive** - Parser generator
- **chrono** - Timestamps

### Java Dependencies (pom.xml)

- **gson** - JSON parsing
- **javax.swing** - Swing AWT components

### Integration Points

1. **TCP Socket**: Port 5678 (Swing) / 5679 (SWT/RCP)
2. **JSON-RPC 2.0**: Line-delimited JSON protocol
3. **Component Cache**: HashMap<hash_code, Component>
4. **UI Tree**: Cached tree structure with refresh

---

## 13. Recommendations

### High Priority

1. **Consolidate Wait Keywords** - Create unified `wait_for` with condition parameter
2. **Enhance Error Context** - Add similar element suggestions to all errors
3. **Add Retry Mechanism** - Configurable retry for flaky operations

### Medium Priority

1. **Extend Locator Syntax** - Add relative locators and indexed selectors
2. **Improve Assertion Messages** - Include actual vs expected values
3. **Add Screenshot on Failure** - Automatic screenshot capture

### Low Priority

1. **Performance Monitoring** - Add timing metrics
2. **Logging Enhancement** - Structured logging with levels
3. **Documentation Generation** - Auto-generate keyword docs

---

## Appendix: Code References

### Key Files for Modernization

| Component | File Path | Lines |
|-----------|-----------|-------|
| Main Library | `src/python/swing_library.rs` | ~1900 |
| Unified Base | `src/python/base_library.rs` | 1253 |
| Locator Parser | `src/locator/parser.rs` | 906 |
| Element Model | `src/core/element.rs` | 1355 |
| Error Types | `src/error.rs` | 252 |
| Java Actions | `agent/.../ActionExecutor.java` | 400+ |
| Java RPC | `agent/.../RpcServer.java` | 500+ |

### Important Patterns

1. **Keyword Signature**: `#[pyo3(signature = (param, optional=None))]`
2. **Connection Check**: `self.ensure_connected()?;`
3. **RPC Call**: `self.send_rpc_request("method", json!({ ... }))?;`
4. **Error Return**: `return Err(SwingError::...).into());`
5. **Element Find**: `self.find_element(locator)?;`
