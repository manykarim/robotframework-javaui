//! Complex component model for the Python bindings and matcher
//!
//! This module provides the comprehensive component model expected by
//! swing_library.rs, element.rs, and matcher.rs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for a component instance
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId {
    /// Java hashCode of the component
    pub hash_code: i64,
    /// Path in the tree (e.g., "0.1.2")
    pub tree_path: String,
    /// Depth in the tree hierarchy
    pub depth: u32,
}

impl ComponentId {
    /// Create a new ComponentId
    pub fn new(hash_code: i64, tree_path: String, depth: u32) -> Self {
        Self {
            hash_code,
            tree_path,
            depth,
        }
    }
}

/// Type information for a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentType {
    /// Full Java class name
    pub class_name: String,
    /// Simple class name (without package)
    pub simple_name: String,
    /// Base Swing component type
    pub base_type: SwingBaseType,
    /// Implemented interfaces
    pub interfaces: Vec<String>,
    /// Class hierarchy
    pub class_hierarchy: Vec<String>,
}

impl Default for ComponentType {
    fn default() -> Self {
        Self {
            class_name: "javax.swing.JComponent".to_string(),
            simple_name: "JComponent".to_string(),
            base_type: SwingBaseType::Unknown,
            interfaces: Vec::new(),
            class_hierarchy: Vec::new(),
        }
    }
}

/// Base Swing component type for categorization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SwingBaseType {
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
    ContentPane,
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
    ComboBox,
    List,
    Spinner,
    Slider,
    ProgressBar,
    Table,
    Tree,
    ColorChooser,
    FileChooser,
    MenuBar,
    Menu,
    MenuItem,
    CheckBoxMenuItem,
    RadioButtonMenuItem,
    PopupMenu,
    Separator,
    ToolTip,
    OptionPane,
    Viewport,
    Unknown,
}

impl SwingBaseType {
    /// Create from class name
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
            "JSeparator" => Self::Separator,
            "JToolTip" => Self::ToolTip,
            "JOptionPane" => Self::OptionPane,
            "JViewport" => Self::Viewport,
            _ => Self::Unknown,
        }
    }
}

/// Identity/naming information for a component
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComponentIdentity {
    /// Name set via setName()
    pub name: Option<String>,
    /// Display text (button text, label text, etc.)
    pub text: Option<String>,
    /// Internal developer name
    pub internal_name: Option<String>,
    /// Window/dialog title
    pub title: Option<String>,
    /// Tooltip text
    pub tooltip: Option<String>,
    /// Action command string
    pub action_command: Option<String>,
    /// Text from associated label (via labelFor)
    pub label_text: Option<String>,
}

/// Component geometry information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComponentGeometry {
    /// Bounds on screen
    pub bounds: Bounds,
    /// Bounds relative to parent
    pub local_bounds: Option<Bounds>,
    /// Preferred size
    pub preferred_size: Option<(i32, i32)>,
    /// Minimum size
    pub minimum_size: Option<(i32, i32)>,
    /// Maximum size
    pub maximum_size: Option<(i32, i32)>,
}

/// Rectangle bounds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Bounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Bounds {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }

    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }
}

/// Component state flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentState {
    pub visible: bool,
    pub showing: bool,
    pub enabled: bool,
    pub focusable: bool,
    pub focused: bool,
    pub selected: Option<bool>,
    pub editable: Option<bool>,
}

impl Default for ComponentState {
    fn default() -> Self {
        Self {
            visible: true,
            showing: true,
            enabled: true,
            focusable: true,
            focused: false,
            selected: None,
            editable: None,
        }
    }
}

/// Component-specific properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComponentProperties {
    Button(ButtonProperties),
    TextField(TextFieldProperties),
    ComboBox(ComboBoxProperties),
    List(ListProperties),
    Table(TableProperties),
    Tree(TreeProperties),
    Slider(SliderProperties),
    ProgressBar(ProgressBarProperties),
    TabbedPane(TabbedPaneProperties),
    Spinner(SpinnerProperties),
    Menu(MenuProperties),
    ScrollPane(ScrollPaneProperties),
    SplitPane(SplitPaneProperties),
    Generic(GenericProperties),
}

impl Default for ComponentProperties {
    fn default() -> Self {
        Self::Generic(GenericProperties::default())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ButtonProperties {
    pub mnemonic: Option<char>,
    pub selected: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextFieldProperties {
    pub columns: u32,
    pub caret_position: u32,
    pub selection_start: u32,
    pub selection_end: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComboBoxProperties {
    pub item_count: u32,
    pub selected_index: i32,
    pub items: Vec<String>,
    pub editable: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListProperties {
    pub item_count: u32,
    pub selected_indices: Vec<u32>,
    pub visible_row_count: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableProperties {
    pub row_count: u32,
    pub column_count: u32,
    pub selected_rows: Vec<u32>,
    pub selected_columns: Vec<u32>,
    pub column_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TreeProperties {
    pub row_count: u32,
    pub selected_rows: Vec<u32>,
    pub expanded_paths: Vec<String>,
    pub selected_paths: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SliderProperties {
    pub minimum: i32,
    pub maximum: i32,
    pub value: i32,
    pub orientation: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProgressBarProperties {
    pub minimum: i32,
    pub maximum: i32,
    pub value: i32,
    pub indeterminate: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TabbedPaneProperties {
    pub tab_count: u32,
    pub selected_index: i32,
    pub tab_titles: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpinnerProperties {
    pub value: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MenuProperties {
    pub item_count: u32,
    pub popup_visible: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScrollPaneProperties {
    pub horizontal_scroll_position: i32,
    pub vertical_scroll_position: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SplitPaneProperties {
    pub divider_location: i32,
    pub orientation: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GenericProperties {
    pub properties: HashMap<String, serde_json::Value>,
}

/// Accessibility information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessibilityInfo {
    pub accessible_name: Option<String>,
    pub accessible_description: Option<String>,
    pub accessible_role: Option<String>,
    pub accessible_state: Vec<String>,
    pub accessible_actions: Vec<String>,
}

/// Traversal metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TraversalMetadata {
    pub child_count: u32,
    pub sibling_index: u32,
}

/// Complete UI component representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIComponent {
    /// Unique identifier
    pub id: ComponentId,
    /// Type information
    pub component_type: ComponentType,
    /// Identity/naming
    pub identity: ComponentIdentity,
    /// Geometry
    pub geometry: ComponentGeometry,
    /// State flags
    pub state: ComponentState,
    /// Component-specific properties
    pub properties: ComponentProperties,
    /// Accessibility info
    pub accessibility: AccessibilityInfo,
    /// Child components
    pub children: Option<Vec<UIComponent>>,
    /// Parent component ID
    pub parent_id: Option<ComponentId>,
    /// Traversal metadata
    pub metadata: TraversalMetadata,
}

impl UIComponent {
    /// Create a minimal component for testing
    pub fn new(id: ComponentId, component_type: ComponentType) -> Self {
        Self {
            id,
            component_type,
            identity: ComponentIdentity::default(),
            geometry: ComponentGeometry::default(),
            state: ComponentState::default(),
            properties: ComponentProperties::default(),
            accessibility: AccessibilityInfo::default(),
            children: None,
            parent_id: None,
            metadata: TraversalMetadata::default(),
        }
    }
}

/// Complete UI tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UITree {
    /// Root components (windows)
    pub roots: Vec<UIComponent>,
    /// Tree metadata
    pub metadata: TreeMetadata,
    /// Tree statistics
    pub statistics: TreeStatistics,
}

impl UITree {
    /// Create an empty tree
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            metadata: TreeMetadata::default(),
            statistics: TreeStatistics::default(),
        }
    }

    /// Iterator over all components
    pub fn iter(&self) -> UITreeIter<'_> {
        UITreeIter {
            stack: self.roots.iter().collect(),
        }
    }
}

impl Default for UITree {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator over UI tree components
pub struct UITreeIter<'a> {
    stack: Vec<&'a UIComponent>,
}

impl<'a> Iterator for UITreeIter<'a> {
    type Item = &'a UIComponent;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(component) = self.stack.pop() {
            if let Some(ref children) = component.children {
                for child in children.iter().rev() {
                    self.stack.push(child);
                }
            }
            Some(component)
        } else {
            None
        }
    }
}

/// Tree metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TreeMetadata {
    pub window_title: Option<String>,
    pub application_name: Option<String>,
    pub capture_time: Option<String>,
}

/// Tree statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TreeStatistics {
    pub total_components: usize,
    pub max_depth: usize,
    pub component_counts: HashMap<String, usize>,
}

/// Filter builder for tree queries
#[derive(Debug, Clone, Default)]
pub struct FilterBuilder {
    visible_only: bool,
    enabled_only: bool,
    max_depth: Option<u32>,
    types: Vec<String>,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn visible_only(mut self) -> Self {
        self.visible_only = true;
        self
    }

    pub fn enabled_only(mut self) -> Self {
        self.enabled_only = true;
        self
    }

    pub fn max_depth(mut self, depth: u32) -> Self {
        self.max_depth = Some(depth);
        self
    }

    pub fn with_type(mut self, type_name: impl Into<String>) -> Self {
        self.types.push(type_name.into());
        self
    }

    pub fn build(self) -> FilterSpecification {
        FilterSpecification {
            visible_only: self.visible_only,
            enabled_only: self.enabled_only,
            max_depth: self.max_depth,
            types: self.types,
        }
    }
}

/// Filter specification for tree queries
#[derive(Debug, Clone, Default)]
pub struct FilterSpecification {
    pub visible_only: bool,
    pub enabled_only: bool,
    pub max_depth: Option<u32>,
    pub types: Vec<String>,
}

impl FilterSpecification {
    pub fn matches(&self, component: &UIComponent) -> bool {
        if self.visible_only && !component.state.visible {
            return false;
        }
        if self.enabled_only && !component.state.enabled {
            return false;
        }
        if !self.types.is_empty() {
            let matches_type = self.types.iter().any(|t| {
                component.component_type.simple_name == *t
                    || component.component_type.class_name.ends_with(t)
            });
            if !matches_type {
                return false;
            }
        }
        true
    }
}
