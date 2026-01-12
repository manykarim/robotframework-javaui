//! UI element model representing Swing components

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single UI element in the Swing component hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElement {
    /// Unique identifier for this element instance
    pub id: String,

    /// Full Java class name (e.g., "javax.swing.JButton")
    pub class_name: String,

    /// Simple class name (e.g., "JButton")
    pub simple_class_name: String,

    /// Component name if set via setName()
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Text content (label, button text, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Tooltip text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,

    /// Accessibility properties
    pub accessible: AccessibleInfo,

    /// Visual bounds (position and size)
    pub bounds: Rectangle,

    /// Component state flags
    pub state: ElementState,

    /// Component-specific properties
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, PropertyValue>,

    /// Child elements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<UIElement>,

    /// XPath-like path to this element from root
    pub path: String,

    /// Index among siblings of the same type
    pub sibling_index: usize,

    /// Depth in the component tree (0 = root)
    pub depth: usize,
}

/// Accessibility information from javax.accessibility
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessibleInfo {
    /// Accessible role (e.g., "push button", "text field")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// Accessible name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Accessible description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Accessible value (for sliders, progress bars, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Accessible states (enabled, visible, focused, etc.)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub states: Vec<String>,

    /// Available actions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
}

/// Rectangle representing component bounds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Rectangle {
    /// X coordinate (relative to parent or screen)
    pub x: i32,
    /// Y coordinate
    pub y: i32,
    /// Width in pixels
    pub width: i32,
    /// Height in pixels
    pub height: i32,
}

impl Rectangle {
    /// Create a new Rectangle
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }

    /// Check if a point is inside this rectangle
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    /// Check if this rectangle intersects another
    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    /// Get the center point
    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }
}

/// Component state flags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ElementState {
    /// Whether the component is visible
    pub visible: bool,
    /// Whether the component is enabled (can receive input)
    pub enabled: bool,
    /// Whether the component currently has keyboard focus
    pub focused: bool,
    /// Whether the component is selected (for toggles, list items, etc.)
    pub selected: bool,
    /// Whether the component is editable (for text components)
    pub editable: bool,
    /// Whether the component is actually showing on screen
    pub showing: bool,
    /// Whether the component can receive focus
    pub focusable: bool,
}

/// Property value types for component-specific properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    /// String value
    String(String),
    /// Integer value
    Integer(i64),
    /// Floating point value
    Float(f64),
    /// Boolean value
    Boolean(bool),
    /// Null/None value
    Null,
    /// Array of values
    Array(Vec<PropertyValue>),
    /// Nested object
    Object(HashMap<String, PropertyValue>),
}

impl PropertyValue {
    /// Get as string if this is a string value
    pub fn as_str(&self) -> Option<&str> {
        match self {
            PropertyValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get as i64 if this is an integer value
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            PropertyValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Get as f64 if this is a float value
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            PropertyValue::Float(f) => Some(*f),
            PropertyValue::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Get as bool if this is a boolean value
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            PropertyValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Check if this is null
    pub fn is_null(&self) -> bool {
        matches!(self, PropertyValue::Null)
    }
}

impl UIElement {
    /// Create a new UIElement with default values
    pub fn new(id: String, class_name: String) -> Self {
        let simple_class_name = class_name
            .rsplit('.')
            .next()
            .unwrap_or(&class_name)
            .to_string();

        Self {
            id,
            class_name,
            simple_class_name,
            name: None,
            text: None,
            tooltip: None,
            accessible: AccessibleInfo::default(),
            bounds: Rectangle::default(),
            state: ElementState::default(),
            properties: HashMap::new(),
            children: Vec::new(),
            path: String::new(),
            sibling_index: 0,
            depth: 0,
        }
    }

    /// Get the display name for this element (name, text, or class)
    pub fn display_name(&self) -> &str {
        self.name
            .as_deref()
            .or(self.text.as_deref())
            .unwrap_or(&self.simple_class_name)
    }

    /// Check if this element matches a simple type name
    pub fn matches_type(&self, type_name: &str) -> bool {
        self.simple_class_name == type_name
            || self.class_name == type_name
            || self.class_name.ends_with(&format!(".{}", type_name))
            || self.class_name.ends_with(&format!("${}", type_name))
    }

    /// Get a property value by name
    pub fn get_property(&self, name: &str) -> Option<&PropertyValue> {
        self.properties.get(name)
    }

    /// Set a property value
    pub fn set_property(&mut self, name: impl Into<String>, value: PropertyValue) {
        self.properties.insert(name.into(), value);
    }

    /// Get all descendants (recursive children)
    pub fn descendants(&self) -> Vec<&UIElement> {
        let mut result = Vec::new();
        self.collect_descendants(&mut result);
        result
    }

    fn collect_descendants<'a>(&'a self, result: &mut Vec<&'a UIElement>) {
        for child in &self.children {
            result.push(child);
            child.collect_descendants(result);
        }
    }

    /// Find the first descendant matching a predicate
    pub fn find_descendant<F>(&self, predicate: F) -> Option<&UIElement>
    where
        F: Fn(&UIElement) -> bool,
    {
        for child in &self.children {
            if predicate(child) {
                return Some(child);
            }
            if let Some(found) = child.find_descendant(&predicate) {
                return Some(found);
            }
        }
        None
    }

    /// Count total elements in this subtree
    pub fn count_elements(&self) -> usize {
        1 + self.children.iter().map(|c| c.count_elements()).sum::<usize>()
    }

    /// Get the maximum depth of this subtree
    pub fn max_depth(&self) -> usize {
        if self.children.is_empty() {
            0
        } else {
            1 + self.children.iter().map(|c| c.max_depth()).max().unwrap_or(0)
        }
    }
}

/// Swing component type for fast matching
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SwingComponentType {
    // Containers
    Frame,
    Dialog,
    Panel,
    ScrollPane,
    SplitPane,
    TabbedPane,
    ToolBar,
    InternalFrame,
    DesktopPane,
    LayeredPane,
    RootPane,

    // Basic components
    Button,
    ToggleButton,
    CheckBox,
    RadioButton,
    Label,
    TextField,
    PasswordField,
    TextArea,
    EditorPane,
    TextPane,
    FormattedTextField,

    // Selection components
    ComboBox,
    List,
    Spinner,
    Slider,
    ProgressBar,

    // Complex components
    Table,
    Tree,
    ColorChooser,
    FileChooser,

    // Menu components
    MenuBar,
    Menu,
    MenuItem,
    CheckBoxMenuItem,
    RadioButtonMenuItem,
    PopupMenu,

    // Other
    Separator,
    ToolTip,
    OptionPane,
    Viewport,

    // Unknown/custom
    Unknown,
}

impl SwingComponentType {
    /// Parse from class name
    pub fn from_class_name(class_name: &str) -> Self {
        let simple = class_name.rsplit('.').next().unwrap_or(class_name);

        match simple {
            "JFrame" => Self::Frame,
            "JDialog" => Self::Dialog,
            "JPanel" => Self::Panel,
            "JScrollPane" => Self::ScrollPane,
            "JSplitPane" => Self::SplitPane,
            "JTabbedPane" => Self::TabbedPane,
            "JToolBar" => Self::ToolBar,
            "JInternalFrame" => Self::InternalFrame,
            "JDesktopPane" => Self::DesktopPane,
            "JLayeredPane" => Self::LayeredPane,
            "JRootPane" => Self::RootPane,
            "JButton" => Self::Button,
            "JToggleButton" => Self::ToggleButton,
            "JCheckBox" => Self::CheckBox,
            "JRadioButton" => Self::RadioButton,
            "JLabel" => Self::Label,
            "JTextField" => Self::TextField,
            "JPasswordField" => Self::PasswordField,
            "JTextArea" => Self::TextArea,
            "JEditorPane" => Self::EditorPane,
            "JTextPane" => Self::TextPane,
            "JFormattedTextField" => Self::FormattedTextField,
            "JComboBox" => Self::ComboBox,
            "JList" => Self::List,
            "JSpinner" => Self::Spinner,
            "JSlider" => Self::Slider,
            "JProgressBar" => Self::ProgressBar,
            "JTable" => Self::Table,
            "JTree" => Self::Tree,
            "JColorChooser" => Self::ColorChooser,
            "JFileChooser" => Self::FileChooser,
            "JMenuBar" => Self::MenuBar,
            "JMenu" => Self::Menu,
            "JMenuItem" => Self::MenuItem,
            "JCheckBoxMenuItem" => Self::CheckBoxMenuItem,
            "JRadioButtonMenuItem" => Self::RadioButtonMenuItem,
            "JPopupMenu" => Self::PopupMenu,
            "JSeparator" | "JToolBar$Separator" => Self::Separator,
            "JToolTip" => Self::ToolTip,
            "JOptionPane" => Self::OptionPane,
            "JViewport" => Self::Viewport,
            _ => Self::Unknown,
        }
    }

    /// Get the simple class name for this type
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Frame => "JFrame",
            Self::Dialog => "JDialog",
            Self::Panel => "JPanel",
            Self::ScrollPane => "JScrollPane",
            Self::SplitPane => "JSplitPane",
            Self::TabbedPane => "JTabbedPane",
            Self::ToolBar => "JToolBar",
            Self::InternalFrame => "JInternalFrame",
            Self::DesktopPane => "JDesktopPane",
            Self::LayeredPane => "JLayeredPane",
            Self::RootPane => "JRootPane",
            Self::Button => "JButton",
            Self::ToggleButton => "JToggleButton",
            Self::CheckBox => "JCheckBox",
            Self::RadioButton => "JRadioButton",
            Self::Label => "JLabel",
            Self::TextField => "JTextField",
            Self::PasswordField => "JPasswordField",
            Self::TextArea => "JTextArea",
            Self::EditorPane => "JEditorPane",
            Self::TextPane => "JTextPane",
            Self::FormattedTextField => "JFormattedTextField",
            Self::ComboBox => "JComboBox",
            Self::List => "JList",
            Self::Spinner => "JSpinner",
            Self::Slider => "JSlider",
            Self::ProgressBar => "JProgressBar",
            Self::Table => "JTable",
            Self::Tree => "JTree",
            Self::ColorChooser => "JColorChooser",
            Self::FileChooser => "JFileChooser",
            Self::MenuBar => "JMenuBar",
            Self::Menu => "JMenu",
            Self::MenuItem => "JMenuItem",
            Self::CheckBoxMenuItem => "JCheckBoxMenuItem",
            Self::RadioButtonMenuItem => "JRadioButtonMenuItem",
            Self::PopupMenu => "JPopupMenu",
            Self::Separator => "JSeparator",
            Self::ToolTip => "JToolTip",
            Self::OptionPane => "JOptionPane",
            Self::Viewport => "JViewport",
            Self::Unknown => "Component",
        }
    }

    /// Check if this is a container type
    pub fn is_container(&self) -> bool {
        matches!(
            self,
            Self::Frame
                | Self::Dialog
                | Self::Panel
                | Self::ScrollPane
                | Self::SplitPane
                | Self::TabbedPane
                | Self::ToolBar
                | Self::InternalFrame
                | Self::DesktopPane
                | Self::LayeredPane
                | Self::RootPane
        )
    }

    /// Check if this is a text input component
    pub fn is_text_input(&self) -> bool {
        matches!(
            self,
            Self::TextField
                | Self::PasswordField
                | Self::TextArea
                | Self::EditorPane
                | Self::TextPane
                | Self::FormattedTextField
        )
    }

    /// Check if this is a button-like component
    pub fn is_button(&self) -> bool {
        matches!(
            self,
            Self::Button
                | Self::ToggleButton
                | Self::CheckBox
                | Self::RadioButton
                | Self::MenuItem
                | Self::CheckBoxMenuItem
                | Self::RadioButtonMenuItem
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_contains() {
        let rect = Rectangle::new(10, 20, 100, 50);
        assert!(rect.contains(50, 40));
        assert!(!rect.contains(5, 40));
        assert!(!rect.contains(50, 100));
    }

    #[test]
    fn test_element_matches_type() {
        let elem = UIElement::new("1".to_string(), "javax.swing.JButton".to_string());
        assert!(elem.matches_type("JButton"));
        assert!(elem.matches_type("javax.swing.JButton"));
        assert!(!elem.matches_type("JLabel"));
    }

    #[test]
    fn test_component_type_parsing() {
        assert_eq!(
            SwingComponentType::from_class_name("javax.swing.JButton"),
            SwingComponentType::Button
        );
        assert_eq!(
            SwingComponentType::from_class_name("JTable"),
            SwingComponentType::Table
        );
        assert_eq!(
            SwingComponentType::from_class_name("CustomComponent"),
            SwingComponentType::Unknown
        );
    }

    #[test]
    fn test_property_value() {
        let s = PropertyValue::String("test".to_string());
        assert_eq!(s.as_str(), Some("test"));
        assert_eq!(s.as_i64(), None);

        let i = PropertyValue::Integer(42);
        assert_eq!(i.as_i64(), Some(42));
        assert_eq!(i.as_f64(), Some(42.0));
    }
}
