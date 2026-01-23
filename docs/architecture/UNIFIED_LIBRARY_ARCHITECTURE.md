# Unified JavaGui Library Architecture

## Executive Summary

This document defines the implementation architecture for unifying SwingLibrary, SwtLibrary, and RcpLibrary into a cohesive robotframework-javagui library with a shared base class, technology-specific extensions, and unified locator handling.

---

## 1. Module Structure

### 1.1 Proposed Directory Layout

```
src/
├── lib.rs                      # Crate entry point, Python module registration
├── core/                       # Core abstractions (NEW)
│   ├── mod.rs
│   ├── backend.rs              # Backend trait definitions
│   ├── config.rs               # Unified configuration management
│   ├── connection.rs           # Connection management trait
│   ├── element.rs              # Unified Element abstraction
│   └── keywords.rs             # Keyword trait definitions
├── locator/                    # Locator parsing (EXISTING - enhanced)
│   ├── mod.rs
│   ├── ast.rs
│   ├── expression.rs
│   ├── matcher.rs
│   ├── parser.rs
│   ├── swt_matcher.rs
│   └── unified.rs              # Unified locator normalization (NEW)
├── model/                      # Data models (EXISTING)
│   ├── mod.rs
│   ├── component.rs
│   ├── element.rs
│   ├── tree.rs
│   ├── rcp.rs
│   └── widget.rs
├── protocol/                   # RPC protocol (EXISTING)
│   └── mod.rs
├── connection/                 # Connection handling (EXISTING)
│   └── mod.rs
└── python/                     # Python bindings (REFACTORED)
    ├── mod.rs
    ├── base_library.rs         # JavaGuiLibrary base class (NEW)
    ├── element.rs              # SwingElement (EXISTING)
    ├── swt_element.rs          # SwtElement (EXISTING)
    ├── unified_element.rs      # JavaGuiElement unified wrapper (NEW)
    ├── exceptions.rs           # Exception types (EXISTING)
    ├── swing/
    │   ├── mod.rs
    │   └── library.rs          # SwingLibrary implementation
    ├── swt/
    │   ├── mod.rs
    │   └── library.rs          # SwtLibrary implementation
    └── rcp/
        ├── mod.rs
        └── library.rs          # RcpLibrary implementation
```

### 1.2 Python Package Structure (exported via PyO3)

```
JavaGui/
├── __init__.py                 # Main exports: SwingLibrary, SwtLibrary, RcpLibrary
├── _core.cpython-*.so         # Compiled Rust module
├── core/                       # Python-side utilities (optional)
│   ├── __init__.py
│   └── compat.py              # Python compatibility utilities
└── compat/
    ├── __init__.py
    └── aliases.py             # Backwards compatibility keyword aliases
```

---

## 2. Class Hierarchy

### 2.1 Class Diagram (Text Format)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              RUST CORE LAYER                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        trait Backend                                  │   │
│  │  + connect(&mut self, config: &ConnectionConfig) -> Result<()>       │   │
│  │  + disconnect(&mut self) -> Result<()>                               │   │
│  │  + is_connected(&self) -> bool                                       │   │
│  │  + send_request(&self, method: &str, params: Value) -> Result<Value> │   │
│  │  + get_toolkit_type(&self) -> ToolkitType                            │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                         ▲                   ▲                   ▲          │
│                         │                   │                   │          │
│         ┌───────────────┴───┐   ┌──────────┴───────┐   ┌───────┴────────┐ │
│         │  SwingBackend     │   │   SwtBackend     │   │   RcpBackend   │ │
│         │  - stream         │   │   - stream       │   │   - swt: Swt   │ │
│         │  - ui_tree cache  │   │   - widget_cache │   │   - workbench  │ │
│         └───────────────────┘   └──────────────────┘   └────────────────┘ │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                   trait ElementOperations                            │   │
│  │  + find_element(&self, locator: &str) -> Result<Element>            │   │
│  │  + find_elements(&self, locator: &str) -> Result<Vec<Element>>      │   │
│  │  + click(&self, locator: &str) -> Result<()>                        │   │
│  │  + input_text(&self, locator: &str, text: &str) -> Result<()>       │   │
│  │  + get_text(&self, locator: &str) -> Result<String>                 │   │
│  │  + wait_for(&self, locator: &str, condition: Condition) -> Result<> │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                       enum ToolkitType                               │   │
│  │  Swing, Swt, Rcp                                                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                       struct JavaGuiElement                          │   │
│  │  + hash_code: i64                                                    │   │
│  │  + class_name: String                                                │   │
│  │  + simple_name: String                                               │   │
│  │  + toolkit_type: ToolkitType                                         │   │
│  │  + name: Option<String>                                              │   │
│  │  + text: Option<String>                                              │   │
│  │  + enabled: bool                                                     │   │
│  │  + visible: bool                                                     │   │
│  │  + bounds: Bounds                                                    │   │
│  │  + properties: HashMap<String, Value>                                │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                           PYTHON BINDING LAYER                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │              #[pyclass] JavaGuiLibraryBase                           │   │
│  │  (Internal base - not directly exposed to Robot Framework)           │   │
│  │  ------------------------------------------------------------------  │   │
│  │  - backend: Box<dyn Backend>                                         │   │
│  │  - config: Arc<RwLock<LibraryConfig>>                               │   │
│  │  - element_cache: Arc<RwLock<HashMap<String, JavaGuiElement>>>      │   │
│  │  ------------------------------------------------------------------  │   │
│  │  # Connection Keywords                                               │   │
│  │  + connect_impl(config: ConnectionConfig) -> Result<()>             │   │
│  │  + disconnect_impl() -> Result<()>                                  │   │
│  │  + is_connected_impl() -> bool                                      │   │
│  │  ------------------------------------------------------------------  │   │
│  │  # Element Keywords                                                  │   │
│  │  + find_element_impl(locator: &str) -> Result<JavaGuiElement>       │   │
│  │  + find_elements_impl(locator: &str) -> Result<Vec<JavaGuiElement>> │   │
│  │  + click_impl(locator: &str) -> Result<()>                          │   │
│  │  + input_text_impl(locator: &str, text: &str) -> Result<()>         │   │
│  │  + wait_until_exists_impl(locator: &str, timeout: f64) -> Result<>  │   │
│  │  ------------------------------------------------------------------  │   │
│  │  # Verification Keywords                                             │   │
│  │  + element_should_exist_impl(locator: &str) -> Result<()>           │   │
│  │  + element_should_be_visible_impl(locator: &str) -> Result<()>      │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                         ▲                   ▲                   ▲          │
│                         │                   │                   │          │
│  ┌──────────────────────┴───────────────────┴───────────────────┴──────┐  │
│  │                                                                      │  │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────┐  │  │
│  │  │  #[pyclass]     │  │  #[pyclass]     │  │  #[pyclass]         │  │  │
│  │  │  SwingLibrary   │  │  SwtLibrary     │  │  RcpLibrary         │  │  │
│  │  │  -------------  │  │  -------------  │  │  -----------------  │  │  │
│  │  │  (composition)  │  │  (composition)  │  │  (composition)      │  │  │
│  │  │  base: Base     │  │  base: Base     │  │  swt_base: Swt      │  │  │
│  │  │  -------------  │  │  -------------  │  │  -----------------  │  │  │
│  │  │  # Swing-only   │  │  # SWT-only     │  │  # RCP-specific     │  │  │
│  │  │  + select_menu  │  │  + get_shells   │  │  + open_perspective │  │  │
│  │  │  + get_table..  │  │  + activate_..  │  │  + show_view        │  │  │
│  │  │  + expand_tree  │  │  + click_widget │  │  + open_editor      │  │  │
│  │  │  + click_button │  │  + input_text   │  │  + execute_command  │  │  │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────────┘  │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Inheritance vs Composition Decision

**Chosen Approach: Composition with Delegation**

The current codebase uses **composition** (RcpLibrary contains SwtLibrary), and this pattern should be extended:

```rust
// Base functionality struct (internal, not #[pyclass])
struct JavaGuiLibraryCore<B: Backend> {
    backend: B,
    config: Arc<RwLock<LibraryConfig>>,
    element_cache: Arc<RwLock<HashMap<String, JavaGuiElement>>>,
}

impl<B: Backend> JavaGuiLibraryCore<B> {
    // All shared keyword implementations
    fn connect_impl(&mut self, config: ConnectionConfig) -> PyResult<()> { ... }
    fn find_element_impl(&self, locator: &str) -> PyResult<JavaGuiElement> { ... }
    fn click_impl(&self, locator: &str) -> PyResult<()> { ... }
    // ... etc
}

// Technology-specific libraries use composition
#[pyclass(name = "SwingLibrary")]
pub struct SwingLibrary {
    core: JavaGuiLibraryCore<SwingBackend>,
}

#[pyclass(name = "SwtLibrary")]
pub struct SwtLibrary {
    core: JavaGuiLibraryCore<SwtBackend>,
}

#[pyclass(name = "RcpLibrary")]
pub struct RcpLibrary {
    swt: SwtLibrary,  // RCP extends SWT (existing pattern)
}
```

### 2.3 Trait Definitions

```rust
// src/core/backend.rs

use serde_json::Value;
use std::time::Duration;

/// Result type for backend operations
pub type BackendResult<T> = Result<T, BackendError>;

/// Toolkit type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolkitType {
    Swing,
    Swt,
    Rcp,
}

/// Connection configuration
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub timeout: Duration,
    pub application: String,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5678,
            timeout: Duration::from_secs(30),
            application: String::new(),
        }
    }
}

/// Backend trait - defines technology-specific communication
pub trait Backend: Send + Sync {
    /// Get the toolkit type
    fn toolkit_type(&self) -> ToolkitType;

    /// Connect to the application
    fn connect(&mut self, config: &ConnectionConfig) -> BackendResult<()>;

    /// Disconnect from the application
    fn disconnect(&mut self) -> BackendResult<()>;

    /// Check if connected
    fn is_connected(&self) -> bool;

    /// Send an RPC request
    fn send_request(&self, method: &str, params: Value) -> BackendResult<Value>;

    /// Get the default port for this toolkit
    fn default_port(&self) -> u16;
}

/// Element operations trait - shared element manipulation
pub trait ElementOperations {
    /// Find a single element
    fn find_element(&self, locator: &str) -> BackendResult<JavaGuiElement>;

    /// Find all matching elements
    fn find_elements(&self, locator: &str) -> BackendResult<Vec<JavaGuiElement>>;

    /// Click on an element
    fn click(&self, locator: &str) -> BackendResult<()>;

    /// Double-click on an element
    fn double_click(&self, locator: &str) -> BackendResult<()>;

    /// Input text into an element
    fn input_text(&self, locator: &str, text: &str, clear: bool) -> BackendResult<()>;

    /// Get element text
    fn get_text(&self, locator: &str) -> BackendResult<String>;

    /// Wait for element condition
    fn wait_for_condition(
        &self,
        locator: &str,
        condition: ElementCondition,
        timeout: Duration,
    ) -> BackendResult<JavaGuiElement>;
}

/// Element conditions for wait operations
#[derive(Debug, Clone)]
pub enum ElementCondition {
    Exists,
    NotExists,
    Visible,
    NotVisible,
    Enabled,
    Disabled,
    HasText(String),
    Custom(Box<dyn Fn(&JavaGuiElement) -> bool + Send + Sync>),
}
```

---

## 3. Unified Element Abstraction

### 3.1 JavaGuiElement Structure

```rust
// src/python/unified_element.rs

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;

/// Unified element representation for all toolkits
#[pyclass(name = "JavaGuiElement")]
#[derive(Clone)]
pub struct JavaGuiElement {
    /// Unique identifier (hash code)
    #[pyo3(get)]
    pub hash_code: i64,

    /// Fully qualified class name
    #[pyo3(get)]
    pub class_name: String,

    /// Simple class name (without package)
    #[pyo3(get)]
    pub simple_name: String,

    /// Toolkit type: "swing", "swt", or "rcp"
    #[pyo3(get)]
    pub toolkit: String,

    /// Widget/component base type (normalized)
    #[pyo3(get)]
    pub element_type: String,

    /// Component name (setName() or SWT data)
    #[pyo3(get)]
    pub name: Option<String>,

    /// Text content
    #[pyo3(get)]
    pub text: Option<String>,

    /// Tooltip text
    #[pyo3(get)]
    pub tooltip: Option<String>,

    /// Bounds
    #[pyo3(get)]
    pub x: i32,
    #[pyo3(get)]
    pub y: i32,
    #[pyo3(get)]
    pub width: i32,
    #[pyo3(get)]
    pub height: i32,

    /// State
    #[pyo3(get)]
    pub visible: bool,
    #[pyo3(get)]
    pub enabled: bool,
    #[pyo3(get)]
    pub focused: bool,

    /// Additional properties (toolkit-specific)
    properties: HashMap<String, serde_json::Value>,
}

#[pymethods]
impl JavaGuiElement {
    /// Get the best identifier for logging/debugging
    #[getter]
    pub fn best_identifier(&self) -> String {
        self.name.clone()
            .or_else(|| self.text.clone())
            .or_else(|| self.tooltip.clone())
            .unwrap_or_else(|| format!("{}@{}", self.simple_name, self.hash_code))
    }

    /// Get normalized element type (cross-toolkit)
    #[getter]
    pub fn normalized_type(&self) -> String {
        normalize_element_type(&self.simple_name, &self.toolkit)
    }

    /// Check if element is a container
    pub fn is_container(&self) -> bool {
        matches!(self.element_type.as_str(),
            "Frame" | "Dialog" | "Panel" | "Shell" | "Composite" |
            "Group" | "ScrollPane" | "SplitPane" | "TabbedPane"
        )
    }

    /// Check if element is a text input
    pub fn is_text_input(&self) -> bool {
        matches!(self.element_type.as_str(),
            "TextField" | "TextArea" | "Text" | "StyledText" |
            "PasswordField" | "Spinner" | "Combo"
        )
    }

    /// Get toolkit-specific property
    #[pyo3(signature = (name, default = None))]
    pub fn get_property(&self, py: Python<'_>, name: &str, default: Option<PyObject>) -> PyObject {
        match self.properties.get(name) {
            Some(value) => json_to_pyobject(py, value),
            None => default.unwrap_or_else(|| py.None()),
        }
    }

    fn __repr__(&self) -> String {
        format!("<JavaGuiElement {}[{}] '{}' toolkit={}>",
            self.simple_name, self.hash_code, self.best_identifier(), self.toolkit)
    }
}

/// Normalize element type across toolkits
fn normalize_element_type(simple_name: &str, toolkit: &str) -> String {
    match toolkit {
        "swing" => normalize_swing_type(simple_name),
        "swt" | "rcp" => normalize_swt_type(simple_name),
        _ => simple_name.to_string(),
    }
}

fn normalize_swing_type(name: &str) -> String {
    match name {
        "JButton" => "Button",
        "JTextField" | "JTextArea" | "JEditorPane" | "JTextPane" => "TextField",
        "JPasswordField" => "PasswordField",
        "JLabel" => "Label",
        "JComboBox" => "ComboBox",
        "JList" => "List",
        "JTable" => "Table",
        "JTree" => "Tree",
        "JCheckBox" => "CheckBox",
        "JRadioButton" => "RadioButton",
        "JSlider" => "Slider",
        "JProgressBar" => "ProgressBar",
        "JTabbedPane" => "TabbedPane",
        "JPanel" => "Panel",
        "JFrame" => "Frame",
        "JDialog" => "Dialog",
        "JScrollPane" => "ScrollPane",
        "JSplitPane" => "SplitPane",
        "JMenuBar" | "JMenu" | "JMenuItem" => "Menu",
        _ => name.trim_start_matches('J').to_string(),
    }
}

fn normalize_swt_type(name: &str) -> String {
    match name {
        "Button" => "Button",  // SWT Button covers checkbox, radio, push
        "Text" | "StyledText" => "TextField",
        "Label" | "CLabel" => "Label",
        "Combo" | "CCombo" => "ComboBox",
        "List" => "List",
        "Table" => "Table",
        "Tree" => "Tree",
        "Scale" | "Slider" => "Slider",
        "ProgressBar" => "ProgressBar",
        "TabFolder" | "CTabFolder" => "TabbedPane",
        "Composite" | "Group" => "Panel",
        "Shell" => "Frame",
        "Menu" | "MenuItem" => "Menu",
        "Spinner" => "Spinner",
        _ => name.to_string(),
    }
}
```

### 3.2 Element Conversion

```rust
impl JavaGuiElement {
    /// Create from SwingElement
    pub fn from_swing(elem: &SwingElement) -> Self {
        Self {
            hash_code: elem.hash_code,
            class_name: elem.class_name.clone(),
            simple_name: elem.simple_name.clone(),
            toolkit: "swing".to_string(),
            element_type: normalize_swing_type(&elem.simple_name),
            name: elem.name.clone(),
            text: elem.text.clone(),
            tooltip: elem.tooltip.clone(),
            x: elem.x,
            y: elem.y,
            width: elem.width,
            height: elem.height,
            visible: elem.visible,
            enabled: elem.enabled,
            focused: elem.focused,
            properties: HashMap::new(), // Extract from properties_json
        }
    }

    /// Create from SwtElement
    pub fn from_swt(elem: &SwtElement) -> Self {
        Self {
            hash_code: elem.hash_code,
            class_name: elem.class_name.clone(),
            simple_name: elem.simple_name.clone(),
            toolkit: "swt".to_string(),
            element_type: normalize_swt_type(&elem.simple_name),
            name: elem.name.clone(),
            text: elem.text.clone(),
            tooltip: elem.tooltip.clone(),
            x: elem.x,
            y: elem.y,
            width: elem.width,
            height: elem.height,
            visible: elem.visible,
            enabled: elem.enabled,
            focused: elem.focused,
            properties: HashMap::new(),
        }
    }
}
```

---

## 4. Unified Locator System

### 4.1 Locator Normalization

```rust
// src/locator/unified.rs

use super::{LocatorExpression, SimpleLocator, SimpleLocatorType};

/// Unified locator that works across toolkits
#[derive(Debug, Clone)]
pub struct UnifiedLocator {
    /// Original locator string
    pub original: String,
    /// Parsed locator type
    pub locator_type: LocatorType,
    /// Primary value
    pub value: String,
    /// Additional predicates
    pub predicates: Vec<LocatorPredicate>,
}

#[derive(Debug, Clone)]
pub enum LocatorType {
    /// By name attribute: name:myButton, #myButton
    Name,
    /// By text content: text:Click Me
    Text,
    /// By class name: class:JButton, Button
    Class,
    /// By index: index:0
    Index,
    /// By ID (hash code): id:12345
    Id,
    /// CSS-like selector: JButton[text="Save"]
    Css,
    /// XPath: //JButton[@text='Save']
    XPath,
    /// Toolkit-specific: swing:JButton, swt:Button
    Toolkit(String),
}

#[derive(Debug, Clone)]
pub enum LocatorPredicate {
    /// Attribute match: [name="value"]
    Attribute { name: String, op: MatchOp, value: String },
    /// Pseudo-class: :visible, :enabled
    PseudoClass(String),
    /// Index: :nth(2)
    Index(usize),
}

#[derive(Debug, Clone)]
pub enum MatchOp {
    Equals,
    Contains,
    StartsWith,
    EndsWith,
    Regex,
}

impl UnifiedLocator {
    /// Parse a locator string into a UnifiedLocator
    pub fn parse(locator: &str) -> Result<Self, LocatorParseError> {
        let locator = locator.trim();

        // Handle explicit type:value format
        if let Some((type_part, value)) = locator.split_once(':') {
            match type_part.to_lowercase().as_str() {
                "name" => return Ok(Self::name(value)),
                "text" => return Ok(Self::text(value)),
                "class" => return Ok(Self::class(value)),
                "index" => return Ok(Self::index(value.parse().map_err(|_| LocatorParseError::InvalidIndex)?)),
                "id" => return Ok(Self::id(value)),
                "swing" | "swt" | "rcp" => return Ok(Self::toolkit(type_part, value)),
                _ => {} // Fall through to other parsing
            }
        }

        // Handle #name shorthand
        if locator.starts_with('#') {
            return Ok(Self::name(&locator[1..]));
        }

        // Handle XPath
        if locator.starts_with("//") || locator.starts_with("(//") {
            return Ok(Self::xpath(locator));
        }

        // Handle CSS-like selectors with attributes
        if locator.contains('[') {
            return Self::parse_css(locator);
        }

        // Default: treat as class name
        Ok(Self::class(locator))
    }

    /// Create name locator
    pub fn name(value: &str) -> Self {
        Self {
            original: format!("name:{}", value),
            locator_type: LocatorType::Name,
            value: value.to_string(),
            predicates: vec![],
        }
    }

    /// Create text locator
    pub fn text(value: &str) -> Self {
        Self {
            original: format!("text:{}", value),
            locator_type: LocatorType::Text,
            value: value.to_string(),
            predicates: vec![],
        }
    }

    /// Create class locator
    pub fn class(value: &str) -> Self {
        Self {
            original: value.to_string(),
            locator_type: LocatorType::Class,
            value: value.to_string(),
            predicates: vec![],
        }
    }

    /// Normalize locator for specific toolkit
    pub fn normalize_for_toolkit(&self, toolkit: ToolkitType) -> NormalizedLocator {
        NormalizedLocator {
            locator_type: self.locator_type.clone(),
            value: self.normalize_class_name(toolkit),
            predicates: self.predicates.clone(),
        }
    }

    fn normalize_class_name(&self, toolkit: ToolkitType) -> String {
        if !matches!(self.locator_type, LocatorType::Class) {
            return self.value.clone();
        }

        match toolkit {
            ToolkitType::Swing => self.normalize_for_swing(),
            ToolkitType::Swt | ToolkitType::Rcp => self.normalize_for_swt(),
        }
    }

    fn normalize_for_swing(&self) -> String {
        let value = &self.value;
        // Add J prefix if not present for common types
        if !value.starts_with('J') && !value.contains('.') {
            match value.as_str() {
                "Button" => "JButton".to_string(),
                "TextField" | "TextArea" => "JTextField".to_string(),
                "Label" => "JLabel".to_string(),
                "ComboBox" => "JComboBox".to_string(),
                "List" => "JList".to_string(),
                "Table" => "JTable".to_string(),
                "Tree" => "JTree".to_string(),
                _ => value.clone(),
            }
        } else {
            value.clone()
        }
    }

    fn normalize_for_swt(&self) -> String {
        let value = &self.value;
        // Remove J prefix for SWT
        if value.starts_with('J') {
            value[1..].to_string()
        } else {
            value.clone()
        }
    }
}
```

### 4.2 Locator Factory

```rust
// src/locator/unified.rs (continued)

/// Factory for creating toolkit-specific locator queries
pub struct LocatorFactory;

impl LocatorFactory {
    /// Convert unified locator to RPC parameters for Swing
    pub fn to_swing_params(locator: &UnifiedLocator) -> serde_json::Value {
        match &locator.locator_type {
            LocatorType::Name => json!({
                "locatorType": "name",
                "value": locator.value
            }),
            LocatorType::Text => json!({
                "locatorType": "text",
                "value": locator.value
            }),
            LocatorType::Class => json!({
                "locatorType": "class",
                "value": locator.normalize_for_swing()
            }),
            LocatorType::Id => json!({
                "locatorType": "hashCode",
                "value": locator.value.parse::<i64>().unwrap_or(0)
            }),
            _ => json!({
                "locator": locator.original
            }),
        }
    }

    /// Convert unified locator to RPC parameters for SWT
    pub fn to_swt_params(locator: &UnifiedLocator) -> serde_json::Value {
        match &locator.locator_type {
            LocatorType::Name => json!({
                "locatorType": "name",
                "value": locator.value
            }),
            LocatorType::Text => json!({
                "locatorType": "text",
                "value": locator.value
            }),
            LocatorType::Class => json!({
                "locatorType": "class",
                "value": locator.normalize_for_swt()
            }),
            LocatorType::Id => json!({
                "locatorType": "id",
                "value": locator.value.parse::<i64>().unwrap_or(0)
            }),
            _ => json!({
                "locator": locator.original
            }),
        }
    }
}
```

---

## 5. Dependency Injection & Configuration

### 5.1 Configuration Management

```rust
// src/core/config.rs

use std::time::Duration;
use std::path::PathBuf;

/// Library configuration
#[derive(Debug, Clone)]
pub struct LibraryConfig {
    /// Default timeout for wait operations
    pub timeout: Duration,
    /// Polling interval for wait operations
    pub poll_interval: Duration,
    /// Whether to take screenshots on failure
    pub screenshot_on_failure: bool,
    /// Directory for screenshots
    pub screenshot_directory: PathBuf,
    /// Screenshot format (png, jpg)
    pub screenshot_format: String,
    /// Log level
    pub log_level: LogLevel,
    /// Whether to cache elements
    pub enable_element_cache: bool,
    /// Element cache TTL
    pub cache_ttl: Duration,
}

impl Default for LibraryConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            poll_interval: Duration::from_millis(500),
            screenshot_on_failure: true,
            screenshot_directory: PathBuf::from("."),
            screenshot_format: "png".to_string(),
            log_level: LogLevel::Info,
            enable_element_cache: true,
            cache_ttl: Duration::from_secs(5),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// Connection configuration
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub timeout: Duration,
    pub application: String,
    pub toolkit: ToolkitType,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5678,
            timeout: Duration::from_secs(30),
            application: String::new(),
            toolkit: ToolkitType::Swing,
        }
    }
}

impl ConnectionConfig {
    pub fn swing(app: &str) -> Self {
        Self {
            application: app.to_string(),
            port: 5678,
            toolkit: ToolkitType::Swing,
            ..Default::default()
        }
    }

    pub fn swt(app: &str) -> Self {
        Self {
            application: app.to_string(),
            port: 5679,
            toolkit: ToolkitType::Swt,
            ..Default::default()
        }
    }

    pub fn rcp(app: &str) -> Self {
        Self {
            application: app.to_string(),
            port: 5679,
            toolkit: ToolkitType::Rcp,
            ..Default::default()
        }
    }
}
```

### 5.2 Backend Factory Pattern

```rust
// src/core/backend.rs (continued)

/// Factory for creating backends
pub struct BackendFactory;

impl BackendFactory {
    /// Create a backend for the specified toolkit
    pub fn create(toolkit: ToolkitType) -> Box<dyn Backend> {
        match toolkit {
            ToolkitType::Swing => Box::new(SwingBackend::new()),
            ToolkitType::Swt => Box::new(SwtBackend::new()),
            ToolkitType::Rcp => Box::new(RcpBackend::new()),
        }
    }

    /// Create a backend from configuration
    pub fn from_config(config: &ConnectionConfig) -> Box<dyn Backend> {
        Self::create(config.toolkit)
    }
}

/// Swing backend implementation
pub struct SwingBackend {
    stream: Option<TcpStream>,
    config: Option<ConnectionConfig>,
    request_id: u64,
}

impl Backend for SwingBackend {
    fn toolkit_type(&self) -> ToolkitType {
        ToolkitType::Swing
    }

    fn connect(&mut self, config: &ConnectionConfig) -> BackendResult<()> {
        // Implementation...
        Ok(())
    }

    fn disconnect(&mut self) -> BackendResult<()> {
        self.stream = None;
        self.config = None;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    fn send_request(&self, method: &str, params: Value) -> BackendResult<Value> {
        // JSON-RPC implementation...
        Ok(Value::Null)
    }

    fn default_port(&self) -> u16 {
        5678
    }
}
```

### 5.3 Environment Variables

```rust
// src/core/config.rs (continued)

impl LibraryConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(timeout) = std::env::var("JAVAGUI_TIMEOUT") {
            if let Ok(secs) = timeout.parse::<u64>() {
                config.timeout = Duration::from_secs(secs);
            }
        }

        if let Ok(poll) = std::env::var("JAVAGUI_POLL_INTERVAL") {
            if let Ok(ms) = poll.parse::<u64>() {
                config.poll_interval = Duration::from_millis(ms);
            }
        }

        if let Ok(dir) = std::env::var("JAVAGUI_SCREENSHOT_DIR") {
            config.screenshot_directory = PathBuf::from(dir);
        }

        if let Ok(val) = std::env::var("JAVAGUI_SCREENSHOT_ON_FAILURE") {
            config.screenshot_on_failure = val.to_lowercase() == "true";
        }

        if let Ok(level) = std::env::var("JAVAGUI_LOG_LEVEL") {
            config.log_level = match level.to_lowercase().as_str() {
                "debug" => LogLevel::Debug,
                "info" => LogLevel::Info,
                "warning" | "warn" => LogLevel::Warning,
                "error" => LogLevel::Error,
                _ => LogLevel::Info,
            };
        }

        config
    }
}
```

---

## 6. Testing Strategy

### 6.1 Test Directory Structure

```
tests/
├── rust/                       # Rust unit tests
│   ├── mod.rs
│   ├── test_locator_parsing.rs
│   ├── test_element_normalization.rs
│   ├── test_backend_factory.rs
│   └── test_config.rs
├── python/                     # Python unit tests
│   ├── conftest.py            # Fixtures and mocks
│   ├── test_swing_library.py
│   ├── test_swt_library.py
│   ├── test_rcp_library.py
│   ├── test_unified_element.py
│   ├── test_locators.py
│   └── test_integration.py
├── integration/                # Integration tests with real Java apps
│   ├── conftest.py
│   ├── test_swing_app.py
│   ├── test_swt_app.py
│   └── test_rcp_app.py
├── robot/                      # Robot Framework acceptance tests
│   ├── swing/
│   │   ├── connection.robot
│   │   ├── elements.robot
│   │   ├── tables.robot
│   │   └── trees.robot
│   ├── swt/
│   │   ├── connection.robot
│   │   ├── widgets.robot
│   │   └── shells.robot
│   └── rcp/
│       ├── workbench.robot
│       ├── perspectives.robot
│       ├── views.robot
│       └── editors.robot
└── apps/                       # Test applications
    ├── swing/
    │   └── src/main/java/testapp/SwingTestApp.java
    ├── swt/
    │   └── src/main/java/testapp/SwtTestApp.java
    └── rcp/
        └── plugins/...
```

### 6.2 Mock Strategy for Rust Core

```python
# tests/python/conftest.py

import pytest
from unittest.mock import Mock, MagicMock
from typing import Dict, Any, List, Optional

class MockBackend:
    """Mock backend for testing without real Java applications."""

    def __init__(self, toolkit: str = "swing"):
        self.toolkit = toolkit
        self._connected = False
        self._elements: Dict[str, MockElement] = {}
        self._request_log: List[Dict] = []
        self._setup_default_elements()

    def connect(self, config: Dict[str, Any]) -> None:
        self._connected = True

    def disconnect(self) -> None:
        self._connected = False

    def is_connected(self) -> bool:
        return self._connected

    def send_request(self, method: str, params: Dict) -> Dict:
        self._request_log.append({"method": method, "params": params})
        return self._handle_request(method, params)

    def _handle_request(self, method: str, params: Dict) -> Dict:
        handlers = {
            "ping": lambda p: "pong",
            "findWidgets": self._find_widgets,
            "findComponents": self._find_components,
            "click": lambda p: {"success": True},
            "typeText": lambda p: {"success": True},
        }
        handler = handlers.get(method, lambda p: {"error": f"Unknown method: {method}"})
        return handler(params)

class MockElement:
    """Mock element for testing."""

    def __init__(
        self,
        hash_code: int,
        class_name: str,
        name: Optional[str] = None,
        text: Optional[str] = None,
        enabled: bool = True,
        visible: bool = True,
    ):
        self.hash_code = hash_code
        self.class_name = class_name
        self.simple_name = class_name.split(".")[-1]
        self.name = name
        self.text = text
        self.enabled = enabled
        self.visible = visible
        self.x = 0
        self.y = 0
        self.width = 100
        self.height = 30

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": self.hash_code,
            "hashCode": self.hash_code,
            "class": self.class_name,
            "className": self.class_name,
            "simpleClass": self.simple_name,
            "name": self.name,
            "text": self.text,
            "enabled": self.enabled,
            "visible": self.visible,
            "x": self.x,
            "y": self.y,
            "width": self.width,
            "height": self.height,
        }

@pytest.fixture
def mock_swing_backend():
    """Fixture for mock Swing backend."""
    return MockBackend(toolkit="swing")

@pytest.fixture
def mock_swt_backend():
    """Fixture for mock SWT backend."""
    return MockBackend(toolkit="swt")

@pytest.fixture
def mock_elements():
    """Fixture providing common mock elements."""
    return {
        "button": MockElement(1, "javax.swing.JButton", name="okBtn", text="OK"),
        "textfield": MockElement(2, "javax.swing.JTextField", name="input"),
        "label": MockElement(3, "javax.swing.JLabel", text="Status"),
        "table": MockElement(4, "javax.swing.JTable", name="dataTable"),
        "tree": MockElement(5, "javax.swing.JTree", name="fileTree"),
    }
```

### 6.3 Integration Test Approach

```python
# tests/integration/conftest.py

import pytest
import subprocess
import time
import socket
from pathlib import Path

class JavaAppManager:
    """Manages Java test applications for integration testing."""

    def __init__(self, app_type: str):
        self.app_type = app_type
        self.process = None
        self.port = self._get_port()

    def _get_port(self) -> int:
        return {"swing": 5678, "swt": 5679, "rcp": 5679}[self.app_type]

    def start(self, timeout: int = 30) -> None:
        """Start the test application."""
        jar_path = self._get_jar_path()
        agent_path = self._get_agent_path()

        cmd = [
            "java",
            f"-javaagent:{agent_path}=port={self.port}",
            "-jar", str(jar_path)
        ]

        self.process = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )

        # Wait for application to be ready
        self._wait_for_port(timeout)

    def stop(self) -> None:
        """Stop the test application."""
        if self.process:
            self.process.terminate()
            self.process.wait(timeout=10)
            self.process = None

    def _wait_for_port(self, timeout: int) -> None:
        """Wait for the agent port to be available."""
        start = time.time()
        while time.time() - start < timeout:
            try:
                sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
                sock.settimeout(1)
                sock.connect(("localhost", self.port))
                sock.close()
                return
            except (socket.error, socket.timeout):
                time.sleep(0.5)
        raise TimeoutError(f"Application did not start within {timeout}s")

@pytest.fixture(scope="session")
def swing_app():
    """Session-scoped fixture for Swing test app."""
    manager = JavaAppManager("swing")
    manager.start()
    yield manager
    manager.stop()

@pytest.fixture(scope="session")
def swt_app():
    """Session-scoped fixture for SWT test app."""
    manager = JavaAppManager("swt")
    manager.start()
    yield manager
    manager.stop()
```

---

## 7. Sequence Diagrams

### 7.1 Connection Sequence

```
User                 SwingLibrary           SwingBackend           Java Agent
 |                        |                      |                      |
 |  connect_to_app()      |                      |                      |
 |----------------------->|                      |                      |
 |                        |  connect(config)     |                      |
 |                        |--------------------->|                      |
 |                        |                      |  TCP Connect         |
 |                        |                      |--------------------->|
 |                        |                      |                      |
 |                        |                      |<---------------------|
 |                        |                      |  Connection OK       |
 |                        |                      |                      |
 |                        |  send_request(ping)  |                      |
 |                        |--------------------->|                      |
 |                        |                      |  JSON-RPC: ping      |
 |                        |                      |--------------------->|
 |                        |                      |                      |
 |                        |                      |<---------------------|
 |                        |                      |  "pong"              |
 |                        |<---------------------|                      |
 |                        |  Ok(())              |                      |
 |<-----------------------|                      |                      |
 |  Connected             |                      |                      |
```

### 7.2 Find Element Sequence

```
User             SwingLibrary          LocatorParser         Backend            Agent
 |                    |                      |                   |                  |
 |  find_element()    |                      |                   |                  |
 |  "name:okBtn"      |                      |                   |                  |
 |------------------->|                      |                   |                  |
 |                    |  parse(locator)      |                   |                  |
 |                    |--------------------->|                   |                  |
 |                    |                      |                   |                  |
 |                    |<---------------------|                   |                  |
 |                    |  UnifiedLocator      |                   |                  |
 |                    |                      |                   |                  |
 |                    |  to_swing_params()   |                   |                  |
 |                    |--------------------->|                   |                  |
 |                    |<---------------------|                   |                  |
 |                    |  RPC params          |                   |                  |
 |                    |                      |                   |                  |
 |                    |  send_request("findComponents", params)  |                  |
 |                    |----------------------------------------->|                  |
 |                    |                      |                   |  JSON-RPC        |
 |                    |                      |                   |----------------->|
 |                    |                      |                   |                  |
 |                    |                      |                   |<-----------------|
 |                    |                      |                   |  Component data  |
 |                    |<-----------------------------------------|                  |
 |                    |  JSON response       |                   |                  |
 |                    |                      |                   |                  |
 |                    |  create SwingElement from JSON           |                  |
 |                    |                      |                   |                  |
 |<-------------------|                      |                   |                  |
 |  SwingElement      |                      |                   |                  |
```

### 7.3 RCP View Operation Sequence

```
User             RcpLibrary            SwtLibrary           RcpBackend          Agent
 |                    |                     |                    |                 |
 |  show_view()       |                     |                    |                 |
 |  "PackageExplorer" |                     |                    |                 |
 |------------------->|                     |                    |                 |
 |                    |  ensure_connected() |                    |                 |
 |                    |------------------->|                     |                 |
 |                    |<-------------------|                     |                 |
 |                    |  Ok                 |                    |                 |
 |                    |                     |                    |                 |
 |                    |  send_rpc_request("rcp.showView", ...)   |                 |
 |                    |-------------------------------------------->               |
 |                    |                     |                    |  JSON-RPC       |
 |                    |                     |                    |---------------->|
 |                    |                     |                    |                 |
 |                    |                     |                    |  Workbench op   |
 |                    |                     |                    |                 |
 |                    |                     |                    |<----------------|
 |                    |<-------------------------------------------------|        |
 |                    |  Result             |                    |                 |
 |<-------------------|                     |                    |                 |
 |  View shown        |                     |                    |                 |
```

---

## 8. Migration Path

### 8.1 Backwards Compatibility

```rust
// src/python/compat/aliases.rs

use pyo3::prelude::*;

/// Keyword aliases for backwards compatibility
pub struct KeywordAliases;

impl KeywordAliases {
    /// Map old keyword names to new ones
    pub fn get_alias(old_name: &str) -> Option<&'static str> {
        match old_name {
            // SwingLibrary compatibility
            "Connect" => Some("connect_to_application"),
            "Disconnect" => Some("disconnect_from_application"),
            "Click Button" => Some("click_element"),
            "Type Into Textfield" => Some("input_text"),
            "Wait Until Keyword Succeeds" => Some("wait_until_element_exists"),

            // SwtLibrary compatibility
            "Connect To SWT Application" => Some("connect_to_application"),
            "Click Widget" => Some("click_element"),
            "Find Widget" => Some("find_element"),

            _ => None,
        }
    }
}

/// Create backwards-compatible keyword methods
#[pymethods]
impl SwingLibrary {
    // Old API aliases

    #[pyo3(name = "connect")]
    pub fn connect_legacy(
        &self,
        application: &str,
        host: Option<&str>,
        port: Option<u16>,
        timeout: Option<f64>,
    ) -> PyResult<()> {
        self.connect_to_application(
            application,
            host.unwrap_or("localhost"),
            port.unwrap_or(5678),
            timeout.unwrap_or(30.0),
        )
    }

    #[pyo3(name = "disconnect")]
    pub fn disconnect_legacy(&self) -> PyResult<()> {
        self.disconnect_from_application()
    }

    #[pyo3(name = "clickButton")]
    pub fn click_button_legacy(&self, locator: &str) -> PyResult<()> {
        self.click_element(locator, Some(1))
    }
}
```

### 8.2 Feature Flags

```toml
# Cargo.toml

[features]
default = ["swing", "swt", "rcp"]
swing = []
swt = []
rcp = ["swt"]  # RCP requires SWT
all-toolkits = ["swing", "swt", "rcp"]

# Compatibility features
legacy-api = []  # Enable old keyword names
```

---

## 9. Summary

### Key Design Decisions

1. **Composition over Inheritance**: Libraries use composition with shared core functionality
2. **Trait-based Backend Abstraction**: Technology-specific communication via Backend trait
3. **Unified Locator System**: Single locator syntax normalized per toolkit
4. **Unified Element Type**: JavaGuiElement with toolkit-specific properties
5. **Factory Pattern**: BackendFactory creates toolkit-specific backends
6. **Configuration Injection**: Environment variables and constructor parameters

### Benefits

- **Code Reuse**: 70%+ keyword logic shared across toolkits
- **Consistent API**: Same keyword names and signatures where possible
- **Easy Testing**: Mock backends enable unit testing without Java apps
- **Extensibility**: New toolkits can be added by implementing Backend trait
- **Backwards Compatibility**: Legacy keyword aliases preserved

### Next Steps

1. Implement `core/backend.rs` with Backend trait
2. Create `core/config.rs` with unified configuration
3. Refactor existing libraries to use composition
4. Implement `locator/unified.rs` for cross-toolkit locators
5. Add comprehensive test coverage
6. Update documentation with unified API reference
