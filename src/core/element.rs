//! Unified element abstraction for JavaGui library
//!
//! Provides a toolkit-agnostic element representation that normalizes
//! properties across Swing, SWT, and RCP widgets.
//!
//! # Overview
//!
//! This module provides:
//!
//! - [`JavaGuiElement`] - Unified representation of UI elements across all toolkits
//! - [`ElementType`] - Normalized element type enumeration
//! - [`Bounds`] - Rectangle bounds value object
//! - [`ElementState`] - Element state value object
//! - [`ElementId`] - Element identifier value object
//!
//! # Domain Model
//!
//! Following DDD principles, elements are represented as aggregates with:
//!
//! - **Aggregate Root**: `JavaGuiElement`
//! - **Value Objects**: `ElementId`, `ElementState`, `Bounds`
//! - **Enumeration**: `ElementType`
//!
//! # Example
//!
//! ```ignore
//! use crate::core::element::{JavaGuiElement, ElementType, Bounds};
//! use crate::core::backend::ToolkitType;
//!
//! // Create element from JSON response
//! let json = serde_json::json!({
//!     "hashCode": 12345,
//!     "className": "javax.swing.JButton",
//!     "name": "okButton",
//!     "text": "OK",
//!     "x": 10, "y": 20, "width": 100, "height": 30,
//!     "visible": true,
//!     "enabled": true,
//! });
//!
//! let element = JavaGuiElement::from_json(&json, ToolkitType::Swing).unwrap();
//! assert_eq!(element.element_type, "Button");
//! ```

use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

use super::backend::ToolkitType;

// ============================================================================
// Value Objects
// ============================================================================

/// Bounds value object representing element geometry
///
/// This is an immutable value object that represents the position and size
/// of a UI element on screen. Bounds are always in screen coordinates.
///
/// # Example
///
/// ```ignore
/// use crate::core::element::Bounds;
///
/// let bounds = Bounds::new(10, 20, 100, 50);
/// assert_eq!(bounds.center(), (60, 45));
/// assert!(bounds.contains(50, 40));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Bounds {
    /// X coordinate (left edge)
    pub x: i32,
    /// Y coordinate (top edge)
    pub y: i32,
    /// Width
    pub width: i32,
    /// Height
    pub height: i32,
}

impl Bounds {
    /// Create new bounds
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }

    /// Create bounds from a tuple
    pub fn from_tuple(tuple: (i32, i32, i32, i32)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2, tuple.3)
    }

    /// Get the center point
    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    /// Get the right edge X coordinate
    pub fn right(&self) -> i32 {
        self.x + self.width
    }

    /// Get the bottom edge Y coordinate
    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }

    /// Check if a point is within these bounds
    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.right() && py >= self.y && py < self.bottom()
    }

    /// Check if these bounds intersect with another
    pub fn intersects(&self, other: &Bounds) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    /// Check if these bounds completely contain another
    pub fn contains_bounds(&self, other: &Bounds) -> bool {
        self.x <= other.x
            && self.y <= other.y
            && self.right() >= other.right()
            && self.bottom() >= other.bottom()
    }

    /// Get the area of these bounds
    pub fn area(&self) -> i64 {
        self.width as i64 * self.height as i64
    }

    /// Check if bounds are empty (zero area)
    pub fn is_empty(&self) -> bool {
        self.width <= 0 || self.height <= 0
    }

    /// Convert to tuple
    pub fn to_tuple(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.width, self.height)
    }

    /// Offset the bounds by delta values
    pub fn offset(&self, dx: i32, dy: i32) -> Self {
        Self::new(self.x + dx, self.y + dy, self.width, self.height)
    }

    /// Expand bounds by a margin
    pub fn expand(&self, margin: i32) -> Self {
        Self::new(
            self.x - margin,
            self.y - margin,
            self.width + margin * 2,
            self.height + margin * 2,
        )
    }
}

impl fmt::Display for Bounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}x{})", self.x, self.y, self.width, self.height)
    }
}

/// Element state value object
///
/// Represents the current state of a UI element (visibility, enabled status, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ElementState {
    /// Whether the element is visible on screen
    pub visible: bool,
    /// Whether the element is enabled (can receive input)
    pub enabled: bool,
    /// Whether the element has keyboard focus
    pub focused: bool,
    /// Whether the element is selected (for toggleable elements)
    pub selected: Option<bool>,
    /// Whether the element is editable (for text inputs)
    pub editable: Option<bool>,
}

impl ElementState {
    /// Create a new element state with defaults
    pub fn new() -> Self {
        Self {
            visible: true,
            enabled: true,
            focused: false,
            selected: None,
            editable: None,
        }
    }

    /// Check if element is interactable (visible and enabled)
    pub fn is_interactable(&self) -> bool {
        self.visible && self.enabled
    }

    /// Create state from JSON
    pub fn from_json(json: &Value) -> Self {
        Self {
            visible: json.get("visible").and_then(|v| v.as_bool()).unwrap_or(true),
            enabled: json.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true),
            focused: json.get("focused").and_then(|v| v.as_bool()).unwrap_or(false),
            selected: json.get("selected").and_then(|v| v.as_bool()),
            editable: json.get("editable").and_then(|v| v.as_bool()),
        }
    }
}

/// Element identifier value object
///
/// Represents the unique identity of an element within the UI tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ElementId {
    /// Unique hash code from Java (hashCode())
    pub handle: i64,
    /// Tree path (e.g., "0/1/3" for root->child1->child3)
    pub tree_path: Option<String>,
    /// Depth in the UI tree (0 = root)
    pub depth: u32,
}

impl ElementId {
    /// Create a new element ID
    pub fn new(handle: i64) -> Self {
        Self {
            handle,
            tree_path: None,
            depth: 0,
        }
    }

    /// Create with tree path
    pub fn with_path(handle: i64, path: String, depth: u32) -> Self {
        Self {
            handle,
            tree_path: Some(path),
            depth,
        }
    }
}

impl fmt::Display for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(path) = &self.tree_path {
            write!(f, "{}@{}", path, self.handle)
        } else {
            write!(f, "@{}", self.handle)
        }
    }
}

// ============================================================================
// Element Type Enumeration
// ============================================================================

/// Normalized element type across toolkits
///
/// This enumeration maps toolkit-specific class names to
/// a unified set of element types. The mapping normalizes differences
/// between Swing and SWT naming conventions.
///
/// # Type Mappings
///
/// | Unified Type | Swing Class | SWT Class |
/// |--------------|-------------|-----------|
/// | Button | JButton | Button(PUSH) |
/// | CheckBox | JCheckBox | Button(CHECK) |
/// | RadioButton | JRadioButton | Button(RADIO) |
/// | TextField | JTextField | Text(SINGLE) |
/// | TextArea | JTextArea | Text(MULTI) |
/// | ComboBox | JComboBox | Combo |
/// | Table | JTable | Table |
/// | Tree | JTree | Tree |
/// | TabFolder | JTabbedPane | TabFolder |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementType {
    // Buttons
    /// Push button (JButton, Button)
    Button,
    /// Toggle button (JToggleButton)
    ToggleButton,
    /// Checkbox (JCheckBox, Button with CHECK style)
    CheckBox,
    /// Radio button (JRadioButton, Button with RADIO style)
    RadioButton,

    // Text inputs
    /// Single-line text field (JTextField, Text)
    TextField,
    /// Multi-line text area (JTextArea, StyledText)
    TextArea,
    /// Password input field (JPasswordField)
    PasswordField,
    /// Numeric spinner (JSpinner, Spinner)
    Spinner,

    // Selection
    /// Dropdown/combo box (JComboBox, Combo)
    ComboBox,
    /// List control (JList, List)
    List,
    /// Table/grid control (JTable, Table)
    Table,
    /// Tree control (JTree, Tree)
    Tree,

    // Display
    /// Static text label (JLabel, Label)
    Label,
    /// Progress bar (JProgressBar, ProgressBar)
    ProgressBar,
    /// Slider control (JSlider, Scale)
    Slider,

    // Containers
    /// Generic panel/composite (JPanel, Composite)
    Panel,
    /// Top-level window frame (JFrame)
    Frame,
    /// Dialog window (JDialog)
    Dialog,
    /// SWT shell (top-level window)
    Shell,
    /// Grouped panel with border (Group)
    Group,
    /// Scrollable container (JScrollPane, ScrolledComposite)
    ScrollPane,
    /// Split pane (JSplitPane, SashForm)
    SplitPane,
    /// Tabbed pane (JTabbedPane, TabFolder) - Swing name
    TabbedPane,
    /// Tab folder (same as TabbedPane) - SWT name
    TabFolder,

    // Menus
    /// Menu bar (JMenuBar)
    MenuBar,
    /// Menu (JMenu, Menu)
    Menu,
    /// Menu item (JMenuItem, MenuItem)
    MenuItem,
    /// Popup/context menu (JPopupMenu)
    PopupMenu,

    // Toolbars
    /// Toolbar (JToolBar, ToolBar)
    ToolBar,
    /// Toolbar item/button (ToolItem)
    ToolItem,

    // RCP-specific
    /// Eclipse RCP view part
    View,
    /// Eclipse RCP editor part
    Editor,
    /// Eclipse RCP perspective
    Perspective,

    // Generic/Unknown
    /// Generic widget (when type cannot be determined more specifically)
    Widget,
    /// Unknown type
    Unknown,
}

impl ElementType {
    /// Get the display name for this element type
    pub fn name(&self) -> &'static str {
        match self {
            ElementType::Button => "Button",
            ElementType::ToggleButton => "ToggleButton",
            ElementType::CheckBox => "CheckBox",
            ElementType::RadioButton => "RadioButton",
            ElementType::TextField => "TextField",
            ElementType::TextArea => "TextArea",
            ElementType::PasswordField => "PasswordField",
            ElementType::Spinner => "Spinner",
            ElementType::ComboBox => "ComboBox",
            ElementType::List => "List",
            ElementType::Table => "Table",
            ElementType::Tree => "Tree",
            ElementType::Label => "Label",
            ElementType::ProgressBar => "ProgressBar",
            ElementType::Slider => "Slider",
            ElementType::Panel => "Panel",
            ElementType::Frame => "Frame",
            ElementType::Dialog => "Dialog",
            ElementType::Shell => "Shell",
            ElementType::Group => "Group",
            ElementType::ScrollPane => "ScrollPane",
            ElementType::SplitPane => "SplitPane",
            ElementType::TabbedPane => "TabbedPane",
            ElementType::TabFolder => "TabFolder",
            ElementType::MenuBar => "MenuBar",
            ElementType::Menu => "Menu",
            ElementType::MenuItem => "MenuItem",
            ElementType::PopupMenu => "PopupMenu",
            ElementType::ToolBar => "ToolBar",
            ElementType::ToolItem => "ToolItem",
            ElementType::View => "View",
            ElementType::Editor => "Editor",
            ElementType::Perspective => "Perspective",
            ElementType::Widget => "Widget",
            ElementType::Unknown => "Unknown",
        }
    }

    /// Parse from string (case insensitive)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "button" => Some(ElementType::Button),
            "togglebutton" => Some(ElementType::ToggleButton),
            "checkbox" => Some(ElementType::CheckBox),
            "radiobutton" => Some(ElementType::RadioButton),
            "textfield" => Some(ElementType::TextField),
            "textarea" => Some(ElementType::TextArea),
            "passwordfield" => Some(ElementType::PasswordField),
            "spinner" => Some(ElementType::Spinner),
            "combobox" => Some(ElementType::ComboBox),
            "list" => Some(ElementType::List),
            "table" => Some(ElementType::Table),
            "tree" => Some(ElementType::Tree),
            "label" => Some(ElementType::Label),
            "progressbar" => Some(ElementType::ProgressBar),
            "slider" => Some(ElementType::Slider),
            "panel" => Some(ElementType::Panel),
            "frame" => Some(ElementType::Frame),
            "dialog" => Some(ElementType::Dialog),
            "shell" => Some(ElementType::Shell),
            "group" => Some(ElementType::Group),
            "scrollpane" => Some(ElementType::ScrollPane),
            "splitpane" => Some(ElementType::SplitPane),
            "tabbedpane" => Some(ElementType::TabbedPane),
            "tabfolder" => Some(ElementType::TabFolder),
            "menubar" => Some(ElementType::MenuBar),
            "menu" => Some(ElementType::Menu),
            "menuitem" => Some(ElementType::MenuItem),
            "popupmenu" => Some(ElementType::PopupMenu),
            "toolbar" => Some(ElementType::ToolBar),
            "toolitem" => Some(ElementType::ToolItem),
            "view" => Some(ElementType::View),
            "editor" => Some(ElementType::Editor),
            "perspective" => Some(ElementType::Perspective),
            "widget" => Some(ElementType::Widget),
            _ => None,
        }
    }

    /// Check if this is a text input type
    pub fn is_text_input(&self) -> bool {
        matches!(
            self,
            ElementType::TextField
                | ElementType::TextArea
                | ElementType::PasswordField
                | ElementType::Spinner
                | ElementType::ComboBox
        )
    }

    /// Check if this is a container type
    pub fn is_container(&self) -> bool {
        matches!(
            self,
            ElementType::Panel
                | ElementType::Frame
                | ElementType::Dialog
                | ElementType::Shell
                | ElementType::Group
                | ElementType::ScrollPane
                | ElementType::SplitPane
                | ElementType::TabbedPane
                | ElementType::TabFolder
        )
    }

    /// Check if this is a selection type (list, table, tree, combo)
    pub fn is_selection(&self) -> bool {
        matches!(
            self,
            ElementType::ComboBox
                | ElementType::List
                | ElementType::Table
                | ElementType::Tree
        )
    }

    /// Check if this is a menu-related type
    pub fn is_menu(&self) -> bool {
        matches!(
            self,
            ElementType::MenuBar
                | ElementType::Menu
                | ElementType::MenuItem
                | ElementType::PopupMenu
        )
    }

    /// Check if this is an RCP-specific type
    pub fn is_rcp_specific(&self) -> bool {
        matches!(
            self,
            ElementType::View | ElementType::Editor | ElementType::Perspective
        )
    }

    /// Get the unified name (for locator matching)
    ///
    /// This returns a canonical name that works across toolkits.
    pub fn unified_name(&self) -> &'static str {
        match self {
            // TabFolder and TabbedPane are unified to TabFolder
            ElementType::TabbedPane | ElementType::TabFolder => "TabFolder",
            _ => self.name(),
        }
    }

    /// Check if this is a clickable type
    pub fn is_clickable(&self) -> bool {
        matches!(
            self,
            ElementType::Button
                | ElementType::ToggleButton
                | ElementType::CheckBox
                | ElementType::RadioButton
                | ElementType::MenuItem
                | ElementType::ToolItem
        )
    }

    /// Infer element type from class name and toolkit
    pub fn from_class_name(class_name: &str, toolkit: ToolkitType) -> Self {
        let simple_name = class_name.split('.').last().unwrap_or(class_name);

        match toolkit {
            ToolkitType::Swing => Self::from_swing_class(simple_name),
            ToolkitType::Swt | ToolkitType::Rcp => Self::from_swt_class(simple_name),
        }
    }

    /// Infer element type from Swing class name
    fn from_swing_class(simple_name: &str) -> Self {
        match simple_name {
            "JButton" => ElementType::Button,
            "JToggleButton" => ElementType::ToggleButton,
            "JCheckBox" => ElementType::CheckBox,
            "JRadioButton" => ElementType::RadioButton,
            "JTextField" | "JFormattedTextField" => ElementType::TextField,
            "JTextArea" | "JEditorPane" | "JTextPane" => ElementType::TextArea,
            "JPasswordField" => ElementType::PasswordField,
            "JSpinner" => ElementType::Spinner,
            "JComboBox" => ElementType::ComboBox,
            "JList" => ElementType::List,
            "JTable" => ElementType::Table,
            "JTree" => ElementType::Tree,
            "JLabel" => ElementType::Label,
            "JProgressBar" => ElementType::ProgressBar,
            "JSlider" => ElementType::Slider,
            "JPanel" => ElementType::Panel,
            "JFrame" => ElementType::Frame,
            "JDialog" => ElementType::Dialog,
            "JScrollPane" => ElementType::ScrollPane,
            "JSplitPane" => ElementType::SplitPane,
            "JTabbedPane" => ElementType::TabbedPane,
            "JMenuBar" => ElementType::MenuBar,
            "JMenu" => ElementType::Menu,
            "JMenuItem" | "JCheckBoxMenuItem" | "JRadioButtonMenuItem" => ElementType::MenuItem,
            "JPopupMenu" => ElementType::PopupMenu,
            "JToolBar" => ElementType::ToolBar,
            _ => {
                // Try removing J prefix and matching
                if simple_name.starts_with('J') {
                    Self::from_swing_class(&simple_name[1..])
                } else {
                    ElementType::Widget
                }
            }
        }
    }

    /// Infer element type from SWT class name
    fn from_swt_class(simple_name: &str) -> Self {
        match simple_name {
            "Button" => ElementType::Button, // Note: SWT Button can be checkbox, radio, etc. based on style bits
            "Text" => ElementType::TextField,
            "StyledText" => ElementType::TextArea,
            "Spinner" => ElementType::Spinner,
            "Combo" | "CCombo" => ElementType::ComboBox,
            "List" => ElementType::List,
            "Table" => ElementType::Table,
            "Tree" => ElementType::Tree,
            "Label" | "CLabel" => ElementType::Label,
            "ProgressBar" => ElementType::ProgressBar,
            "Scale" | "Slider" => ElementType::Slider,
            "Composite" | "ScrolledComposite" => ElementType::Panel,
            "Group" => ElementType::Group,
            "Shell" => ElementType::Shell,
            "TabFolder" | "CTabFolder" => ElementType::TabFolder,
            "SashForm" => ElementType::SplitPane,
            "Menu" => ElementType::Menu,
            "MenuItem" => ElementType::MenuItem,
            "ToolBar" => ElementType::ToolBar,
            "ToolItem" => ElementType::ToolItem,
            // RCP-specific
            "ViewPart" | "ViewSite" => ElementType::View,
            "EditorPart" | "EditorSite" => ElementType::Editor,
            _ => ElementType::Widget,
        }
    }

    /// Determine more specific SWT button type from style bits
    ///
    /// SWT uses a single Button class with style bits to differentiate
    /// between push buttons, checkboxes, radio buttons, etc.
    pub fn from_swt_button_style(style: i32) -> Self {
        // SWT style constants (from org.eclipse.swt.SWT)
        const SWT_CHECK: i32 = 1 << 5;     // 32
        const SWT_RADIO: i32 = 1 << 4;     // 16
        const SWT_TOGGLE: i32 = 1 << 1;    // 2
        const SWT_ARROW: i32 = 1 << 2;     // 4

        if style & SWT_CHECK != 0 {
            ElementType::CheckBox
        } else if style & SWT_RADIO != 0 {
            ElementType::RadioButton
        } else if style & SWT_TOGGLE != 0 {
            ElementType::ToggleButton
        } else if style & SWT_ARROW != 0 {
            ElementType::Button // Arrow buttons are still buttons
        } else {
            ElementType::Button // Default push button
        }
    }
}

impl std::fmt::Display for ElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Unified element representation for all toolkits
///
/// This struct provides a consistent interface for interacting with
/// UI elements regardless of whether they come from Swing, SWT, or RCP.
#[pyclass(name = "JavaGuiElement")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JavaGuiElement {
    /// Unique identifier (hash code from Java)
    #[pyo3(get)]
    pub hash_code: i64,

    /// Fully qualified Java class name
    #[pyo3(get)]
    pub class_name: String,

    /// Simple class name (without package)
    #[pyo3(get)]
    pub simple_name: String,

    /// Toolkit type: "swing", "swt", or "rcp"
    #[pyo3(get)]
    pub toolkit: String,

    /// Normalized element type
    #[pyo3(get)]
    pub element_type: String,

    /// Component name (setName() in Swing, setData("name") in SWT)
    #[pyo3(get)]
    pub name: Option<String>,

    /// Text content
    #[pyo3(get)]
    pub text: Option<String>,

    /// Tooltip text
    #[pyo3(get)]
    pub tooltip: Option<String>,

    /// X coordinate
    #[pyo3(get)]
    pub x: i32,

    /// Y coordinate
    #[pyo3(get)]
    pub y: i32,

    /// Width
    #[pyo3(get)]
    pub width: i32,

    /// Height
    #[pyo3(get)]
    pub height: i32,

    /// Whether element is visible
    #[pyo3(get)]
    pub visible: bool,

    /// Whether element is enabled
    #[pyo3(get)]
    pub enabled: bool,

    /// Whether element has focus
    #[pyo3(get)]
    pub focused: bool,

    /// Additional toolkit-specific properties
    #[serde(default)]
    properties: HashMap<String, Value>,
}

#[pymethods]
impl JavaGuiElement {
    /// Create a new JavaGuiElement
    #[new]
    #[pyo3(signature = (hash_code, class_name, toolkit="swing"))]
    pub fn new(hash_code: i64, class_name: &str, toolkit: &str) -> Self {
        let simple_name = class_name.split('.').last().unwrap_or(class_name).to_string();
        let toolkit_type = ToolkitType::from_str(toolkit).unwrap_or(ToolkitType::Swing);
        let element_type = ElementType::from_class_name(class_name, toolkit_type);

        Self {
            hash_code,
            class_name: class_name.to_string(),
            simple_name,
            toolkit: toolkit.to_string(),
            element_type: element_type.name().to_string(),
            name: None,
            text: None,
            tooltip: None,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            visible: true,
            enabled: true,
            focused: false,
            properties: HashMap::new(),
        }
    }

    /// Get the best identifier for logging/debugging
    #[getter]
    pub fn best_identifier(&self) -> String {
        self.name
            .clone()
            .or_else(|| self.text.clone())
            .or_else(|| self.tooltip.clone())
            .unwrap_or_else(|| format!("{}@{}", self.simple_name, self.hash_code))
    }

    /// Get normalized element type enum value
    #[getter]
    pub fn normalized_type(&self) -> String {
        let toolkit_type = ToolkitType::from_str(&self.toolkit).unwrap_or(ToolkitType::Swing);
        ElementType::from_class_name(&self.class_name, toolkit_type)
            .name()
            .to_string()
    }

    /// Check if element is a container
    pub fn is_container(&self) -> bool {
        let toolkit_type = ToolkitType::from_str(&self.toolkit).unwrap_or(ToolkitType::Swing);
        ElementType::from_class_name(&self.class_name, toolkit_type).is_container()
    }

    /// Check if element is a text input
    pub fn is_text_input(&self) -> bool {
        let toolkit_type = ToolkitType::from_str(&self.toolkit).unwrap_or(ToolkitType::Swing);
        ElementType::from_class_name(&self.class_name, toolkit_type).is_text_input()
    }

    /// Check if element is clickable
    pub fn is_clickable(&self) -> bool {
        let toolkit_type = ToolkitType::from_str(&self.toolkit).unwrap_or(ToolkitType::Swing);
        ElementType::from_class_name(&self.class_name, toolkit_type).is_clickable()
    }

    /// Get bounds as tuple (x, y, width, height)
    pub fn get_bounds(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.width, self.height)
    }

    /// Get center point as tuple (x, y)
    pub fn get_center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    /// Get a toolkit-specific property
    #[pyo3(signature = (name, default = None))]
    pub fn get_property(&self, py: Python<'_>, name: &str, default: Option<PyObject>) -> PyObject {
        match self.properties.get(name) {
            Some(value) => json_to_pyobject(py, value),
            None => default.unwrap_or_else(|| py.None()),
        }
    }

    /// Set a toolkit-specific property
    pub fn set_property(&mut self, name: &str, value: &str) {
        self.properties
            .insert(name.to_string(), Value::String(value.to_string()));
    }

    /// Check if element has a property
    pub fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// Get all property names
    pub fn property_names(&self) -> Vec<String> {
        self.properties.keys().cloned().collect()
    }

    /// Convert to dictionary
    pub fn to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("hash_code", self.hash_code)?;
        dict.set_item("class_name", &self.class_name)?;
        dict.set_item("simple_name", &self.simple_name)?;
        dict.set_item("toolkit", &self.toolkit)?;
        dict.set_item("element_type", &self.element_type)?;
        dict.set_item("name", &self.name)?;
        dict.set_item("text", &self.text)?;
        dict.set_item("tooltip", &self.tooltip)?;
        dict.set_item("x", self.x)?;
        dict.set_item("y", self.y)?;
        dict.set_item("width", self.width)?;
        dict.set_item("height", self.height)?;
        dict.set_item("visible", self.visible)?;
        dict.set_item("enabled", self.enabled)?;
        dict.set_item("focused", self.focused)?;
        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        format!(
            "<JavaGuiElement {}[{}] '{}' toolkit={}>",
            self.simple_name,
            self.hash_code,
            self.best_identifier(),
            self.toolkit
        )
    }

    fn __str__(&self) -> String {
        format!(
            "{}[name={:?}, text={:?}]",
            self.simple_name, self.name, self.text
        )
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.hash_code == other.hash_code && self.toolkit == other.toolkit
    }

    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.hash_code.hash(&mut hasher);
        self.toolkit.hash(&mut hasher);
        hasher.finish()
    }
}

impl JavaGuiElement {
    /// Create from JSON response
    pub fn from_json(json: &Value, toolkit: ToolkitType) -> Option<Self> {
        let hash_code = json
            .get("hashCode")
            .or_else(|| json.get("id"))
            .and_then(|v| v.as_i64())?;

        let class_name = json
            .get("className")
            .or_else(|| json.get("class"))
            .and_then(|v| v.as_str())?
            .to_string();

        let simple_name = json
            .get("simpleName")
            .or_else(|| json.get("simpleClass"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                class_name.split('.').last().unwrap_or(&class_name).to_string()
            });

        let element_type = ElementType::from_class_name(&class_name, toolkit);

        let mut elem = Self {
            hash_code,
            class_name,
            simple_name,
            toolkit: toolkit.name().to_string(),
            element_type: element_type.name().to_string(),
            name: json.get("name").and_then(|v| v.as_str()).map(String::from),
            text: json.get("text").and_then(|v| v.as_str()).map(String::from),
            tooltip: json
                .get("tooltip")
                .or_else(|| json.get("toolTipText"))
                .and_then(|v| v.as_str())
                .map(String::from),
            x: json.get("x").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            y: json.get("y").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            width: json.get("width").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            height: json.get("height").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            visible: json.get("visible").and_then(|v| v.as_bool()).unwrap_or(true),
            enabled: json.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true),
            focused: json.get("focused").and_then(|v| v.as_bool()).unwrap_or(false),
            properties: HashMap::new(),
        };

        // Extract additional properties
        if let Some(props) = json.get("properties").and_then(|v| v.as_object()) {
            for (key, value) in props {
                elem.properties.insert(key.clone(), value.clone());
            }
        }

        Some(elem)
    }

    /// Convert to JSON
    pub fn to_json(&self) -> Value {
        serde_json::json!({
            "hashCode": self.hash_code,
            "className": self.class_name,
            "simpleName": self.simple_name,
            "toolkit": self.toolkit,
            "elementType": self.element_type,
            "name": self.name,
            "text": self.text,
            "tooltip": self.tooltip,
            "x": self.x,
            "y": self.y,
            "width": self.width,
            "height": self.height,
            "visible": self.visible,
            "enabled": self.enabled,
            "focused": self.focused,
            "properties": self.properties,
        })
    }

    /// Set element properties (for builder pattern)
    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_bounds(mut self, x: i32, y: i32, width: i32, height: i32) -> Self {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Convert JSON Value to Python object
fn json_to_pyobject(py: Python<'_>, value: &Value) -> PyObject {
    match value {
        Value::Null => py.None(),
        Value::Bool(b) => b.into_py(py),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.into_py(py)
            } else if let Some(f) = n.as_f64() {
                f.into_py(py)
            } else {
                py.None()
            }
        }
        Value::String(s) => s.into_py(py),
        Value::Array(arr) => {
            let list: Vec<PyObject> = arr.iter().map(|v| json_to_pyobject(py, v)).collect();
            list.into_py(py)
        }
        Value::Object(obj) => {
            let dict = PyDict::new(py);
            for (k, v) in obj {
                dict.set_item(k, json_to_pyobject(py, v)).ok();
            }
            dict.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Bounds Tests
    // ========================================================================

    #[test]
    fn test_bounds_new() {
        let bounds = Bounds::new(10, 20, 100, 50);
        assert_eq!(bounds.x, 10);
        assert_eq!(bounds.y, 20);
        assert_eq!(bounds.width, 100);
        assert_eq!(bounds.height, 50);
    }

    #[test]
    fn test_bounds_center() {
        let bounds = Bounds::new(10, 20, 100, 50);
        assert_eq!(bounds.center(), (60, 45));
    }

    #[test]
    fn test_bounds_right_bottom() {
        let bounds = Bounds::new(10, 20, 100, 50);
        assert_eq!(bounds.right(), 110);
        assert_eq!(bounds.bottom(), 70);
    }

    #[test]
    fn test_bounds_contains() {
        let bounds = Bounds::new(10, 20, 100, 50);
        assert!(bounds.contains(50, 40));
        assert!(bounds.contains(10, 20)); // top-left corner
        assert!(!bounds.contains(110, 70)); // bottom-right is exclusive
        assert!(!bounds.contains(5, 40)); // outside left
    }

    #[test]
    fn test_bounds_intersects() {
        let bounds1 = Bounds::new(0, 0, 100, 100);
        let bounds2 = Bounds::new(50, 50, 100, 100);
        let bounds3 = Bounds::new(200, 200, 50, 50);

        assert!(bounds1.intersects(&bounds2));
        assert!(bounds2.intersects(&bounds1));
        assert!(!bounds1.intersects(&bounds3));
    }

    #[test]
    fn test_bounds_contains_bounds() {
        let outer = Bounds::new(0, 0, 100, 100);
        let inner = Bounds::new(10, 10, 50, 50);
        let overlapping = Bounds::new(50, 50, 100, 100);

        assert!(outer.contains_bounds(&inner));
        assert!(!outer.contains_bounds(&overlapping));
        assert!(!inner.contains_bounds(&outer));
    }

    #[test]
    fn test_bounds_area() {
        let bounds = Bounds::new(0, 0, 10, 20);
        assert_eq!(bounds.area(), 200);
    }

    #[test]
    fn test_bounds_is_empty() {
        assert!(Bounds::new(0, 0, 0, 0).is_empty());
        assert!(Bounds::new(0, 0, 0, 10).is_empty());
        assert!(Bounds::new(0, 0, 10, 0).is_empty());
        assert!(Bounds::new(0, 0, -10, 10).is_empty());
        assert!(!Bounds::new(0, 0, 10, 10).is_empty());
    }

    #[test]
    fn test_bounds_offset() {
        let bounds = Bounds::new(10, 20, 100, 50);
        let offset = bounds.offset(5, -10);
        assert_eq!(offset.x, 15);
        assert_eq!(offset.y, 10);
        assert_eq!(offset.width, 100);
        assert_eq!(offset.height, 50);
    }

    #[test]
    fn test_bounds_expand() {
        let bounds = Bounds::new(10, 20, 100, 50);
        let expanded = bounds.expand(5);
        assert_eq!(expanded.x, 5);
        assert_eq!(expanded.y, 15);
        assert_eq!(expanded.width, 110);
        assert_eq!(expanded.height, 60);
    }

    #[test]
    fn test_bounds_display() {
        let bounds = Bounds::new(10, 20, 100, 50);
        assert_eq!(format!("{}", bounds), "(10, 20, 100x50)");
    }

    // ========================================================================
    // ElementState Tests
    // ========================================================================

    #[test]
    fn test_element_state_new() {
        let state = ElementState::new();
        assert!(state.visible);
        assert!(state.enabled);
        assert!(!state.focused);
        assert!(state.selected.is_none());
    }

    #[test]
    fn test_element_state_is_interactable() {
        let visible_enabled = ElementState { visible: true, enabled: true, ..Default::default() };
        let hidden = ElementState { visible: false, enabled: true, ..Default::default() };
        let disabled = ElementState { visible: true, enabled: false, ..Default::default() };

        assert!(visible_enabled.is_interactable());
        assert!(!hidden.is_interactable());
        assert!(!disabled.is_interactable());
    }

    #[test]
    fn test_element_state_from_json() {
        let json = serde_json::json!({
            "visible": true,
            "enabled": false,
            "focused": true,
            "selected": true,
        });
        let state = ElementState::from_json(&json);
        assert!(state.visible);
        assert!(!state.enabled);
        assert!(state.focused);
        assert_eq!(state.selected, Some(true));
    }

    // ========================================================================
    // ElementId Tests
    // ========================================================================

    #[test]
    fn test_element_id_new() {
        let id = ElementId::new(12345);
        assert_eq!(id.handle, 12345);
        assert!(id.tree_path.is_none());
        assert_eq!(id.depth, 0);
    }

    #[test]
    fn test_element_id_with_path() {
        let id = ElementId::with_path(12345, "0/1/3".to_string(), 3);
        assert_eq!(id.handle, 12345);
        assert_eq!(id.tree_path, Some("0/1/3".to_string()));
        assert_eq!(id.depth, 3);
    }

    #[test]
    fn test_element_id_display() {
        let id1 = ElementId::new(12345);
        assert_eq!(format!("{}", id1), "@12345");

        let id2 = ElementId::with_path(12345, "0/1".to_string(), 2);
        assert_eq!(format!("{}", id2), "0/1@12345");
    }

    // ========================================================================
    // ElementType Tests
    // ========================================================================

    #[test]
    fn test_element_type_from_swing_class() {
        assert_eq!(
            ElementType::from_class_name("javax.swing.JButton", ToolkitType::Swing),
            ElementType::Button
        );
        assert_eq!(
            ElementType::from_class_name("JTextField", ToolkitType::Swing),
            ElementType::TextField
        );
        assert_eq!(
            ElementType::from_class_name("JTable", ToolkitType::Swing),
            ElementType::Table
        );
        assert_eq!(
            ElementType::from_class_name("JTabbedPane", ToolkitType::Swing),
            ElementType::TabbedPane
        );
    }

    #[test]
    fn test_element_type_from_swt_class() {
        assert_eq!(
            ElementType::from_class_name("org.eclipse.swt.widgets.Button", ToolkitType::Swt),
            ElementType::Button
        );
        assert_eq!(
            ElementType::from_class_name("Text", ToolkitType::Swt),
            ElementType::TextField
        );
        assert_eq!(
            ElementType::from_class_name("Table", ToolkitType::Swt),
            ElementType::Table
        );
        assert_eq!(
            ElementType::from_class_name("TabFolder", ToolkitType::Swt),
            ElementType::TabFolder
        );
    }

    #[test]
    fn test_element_type_properties() {
        assert!(ElementType::TextField.is_text_input());
        assert!(!ElementType::Button.is_text_input());
        assert!(ElementType::Panel.is_container());
        assert!(ElementType::TabFolder.is_container());
        assert!(!ElementType::Label.is_container());
        assert!(ElementType::Button.is_clickable());
        assert!(!ElementType::Label.is_clickable());
    }

    #[test]
    fn test_element_type_is_selection() {
        assert!(ElementType::ComboBox.is_selection());
        assert!(ElementType::List.is_selection());
        assert!(ElementType::Table.is_selection());
        assert!(ElementType::Tree.is_selection());
        assert!(!ElementType::Button.is_selection());
    }

    #[test]
    fn test_element_type_is_menu() {
        assert!(ElementType::Menu.is_menu());
        assert!(ElementType::MenuItem.is_menu());
        assert!(ElementType::MenuBar.is_menu());
        assert!(ElementType::PopupMenu.is_menu());
        assert!(!ElementType::Button.is_menu());
    }

    #[test]
    fn test_element_type_is_rcp_specific() {
        assert!(ElementType::View.is_rcp_specific());
        assert!(ElementType::Editor.is_rcp_specific());
        assert!(ElementType::Perspective.is_rcp_specific());
        assert!(!ElementType::Button.is_rcp_specific());
    }

    #[test]
    fn test_element_type_unified_name() {
        // TabFolder and TabbedPane should both return "TabFolder"
        assert_eq!(ElementType::TabFolder.unified_name(), "TabFolder");
        assert_eq!(ElementType::TabbedPane.unified_name(), "TabFolder");
        // Others return their own name
        assert_eq!(ElementType::Button.unified_name(), "Button");
    }

    #[test]
    fn test_element_type_from_str() {
        assert_eq!(ElementType::from_str("button"), Some(ElementType::Button));
        assert_eq!(ElementType::from_str("TABFOLDER"), Some(ElementType::TabFolder));
        assert_eq!(ElementType::from_str("unknown_type"), None);
    }

    #[test]
    fn test_element_type_from_swt_button_style() {
        // SWT style constants
        const SWT_CHECK: i32 = 1 << 5;  // 32
        const SWT_RADIO: i32 = 1 << 4;  // 16
        const SWT_TOGGLE: i32 = 1 << 1; // 2

        assert_eq!(ElementType::from_swt_button_style(SWT_CHECK), ElementType::CheckBox);
        assert_eq!(ElementType::from_swt_button_style(SWT_RADIO), ElementType::RadioButton);
        assert_eq!(ElementType::from_swt_button_style(SWT_TOGGLE), ElementType::ToggleButton);
        assert_eq!(ElementType::from_swt_button_style(0), ElementType::Button);
    }

    // ========================================================================
    // JavaGuiElement Tests
    // ========================================================================

    #[test]
    fn test_java_gui_element_from_json() {
        let json = serde_json::json!({
            "hashCode": 12345,
            "className": "javax.swing.JButton",
            "name": "okButton",
            "text": "OK",
            "x": 10,
            "y": 20,
            "width": 100,
            "height": 30,
            "visible": true,
            "enabled": true,
        });

        let elem = JavaGuiElement::from_json(&json, ToolkitType::Swing).unwrap();
        assert_eq!(elem.hash_code, 12345);
        assert_eq!(elem.simple_name, "JButton");
        assert_eq!(elem.name, Some("okButton".to_string()));
        assert_eq!(elem.text, Some("OK".to_string()));
        assert_eq!(elem.element_type, "Button");
    }

    #[test]
    fn test_java_gui_element_best_identifier() {
        let elem = JavaGuiElement::new(123, "JButton", "swing")
            .with_name("testButton");
        assert_eq!(elem.best_identifier(), "testButton");

        let elem2 = JavaGuiElement::new(456, "JLabel", "swing")
            .with_text("Hello");
        assert_eq!(elem2.best_identifier(), "Hello");

        let elem3 = JavaGuiElement::new(789, "JPanel", "swing");
        assert_eq!(elem3.best_identifier(), "JPanel@789");
    }

    #[test]
    fn test_java_gui_element_bounds() {
        let elem = JavaGuiElement::new(123, "JButton", "swing")
            .with_bounds(10, 20, 100, 50);

        assert_eq!(elem.get_bounds(), (10, 20, 100, 50));
        assert_eq!(elem.get_center(), (60, 45));
    }

    #[test]
    fn test_java_gui_element_to_json() {
        let elem = JavaGuiElement::new(123, "JButton", "swing")
            .with_name("btn")
            .with_text("Click");

        let json = elem.to_json();
        assert_eq!(json["hashCode"], 123);
        assert_eq!(json["className"], "JButton");
        assert_eq!(json["name"], "btn");
        assert_eq!(json["text"], "Click");
    }
}
