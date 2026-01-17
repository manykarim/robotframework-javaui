//! Optimized type mapping for Swing and SWT components
//!
//! This module provides fast type mapping using compile-time
//! string comparisons and organized lookup tables.
//!
//! Performance target: <100ns for type mapping

use crate::core::element::ElementType;
use crate::core::backend::ToolkitType;

/// Fast type mapping for Swing class names
///
/// This uses a match-based lookup which the compiler can optimize
/// into a jump table or binary search.
#[inline]
pub fn map_swing_type(simple_name: &str) -> ElementType {
    match simple_name {
        // Buttons (high frequency)
        "JButton" => ElementType::Button,
        "JToggleButton" => ElementType::ToggleButton,
        "JCheckBox" => ElementType::CheckBox,
        "JRadioButton" => ElementType::RadioButton,

        // Text inputs (high frequency)
        "JTextField" => ElementType::TextField,
        "JFormattedTextField" => ElementType::TextField,
        "JTextArea" => ElementType::TextArea,
        "JEditorPane" => ElementType::TextArea,
        "JTextPane" => ElementType::TextArea,
        "JPasswordField" => ElementType::PasswordField,
        "JSpinner" => ElementType::Spinner,

        // Selection components (medium frequency)
        "JComboBox" => ElementType::ComboBox,
        "JList" => ElementType::List,
        "JTable" => ElementType::Table,
        "JTree" => ElementType::Tree,

        // Display components
        "JLabel" => ElementType::Label,
        "JProgressBar" => ElementType::ProgressBar,
        "JSlider" => ElementType::Slider,

        // Containers
        "JPanel" => ElementType::Panel,
        "JFrame" => ElementType::Frame,
        "JDialog" => ElementType::Dialog,
        "JInternalFrame" => ElementType::Frame,
        "JScrollPane" => ElementType::ScrollPane,
        "JSplitPane" => ElementType::SplitPane,
        "JTabbedPane" => ElementType::TabbedPane,
        "JDesktopPane" => ElementType::Panel,
        "JLayeredPane" => ElementType::Panel,
        "JRootPane" => ElementType::Panel,
        "JViewport" => ElementType::Panel,

        // Menus
        "JMenuBar" => ElementType::MenuBar,
        "JMenu" => ElementType::Menu,
        "JMenuItem" => ElementType::MenuItem,
        "JCheckBoxMenuItem" => ElementType::MenuItem,
        "JRadioButtonMenuItem" => ElementType::MenuItem,
        "JPopupMenu" => ElementType::PopupMenu,

        // Toolbars
        "JToolBar" => ElementType::ToolBar,

        // Unknown
        _ => ElementType::Widget,
    }
}

/// Fast type mapping for SWT class names
#[inline]
pub fn map_swt_type(simple_name: &str) -> ElementType {
    match simple_name {
        // Basic widgets (high frequency)
        "Button" => ElementType::Button,
        "Text" => ElementType::TextField,
        "StyledText" => ElementType::TextArea,
        "Label" => ElementType::Label,
        "CLabel" => ElementType::Label,

        // Selection (medium frequency)
        "Combo" => ElementType::ComboBox,
        "CCombo" => ElementType::ComboBox,
        "List" => ElementType::List,
        "Table" => ElementType::Table,
        "Tree" => ElementType::Tree,
        "Spinner" => ElementType::Spinner,

        // Progress/Scale
        "ProgressBar" => ElementType::ProgressBar,
        "Scale" => ElementType::Slider,
        "Slider" => ElementType::Slider,

        // Containers
        "Composite" => ElementType::Panel,
        "ScrolledComposite" => ElementType::ScrollPane,
        "Group" => ElementType::Group,
        "Shell" => ElementType::Shell,
        "TabFolder" => ElementType::TabbedPane,
        "CTabFolder" => ElementType::TabbedPane,
        "SashForm" => ElementType::SplitPane,
        "Canvas" => ElementType::Panel,

        // Menus
        "Menu" => ElementType::Menu,
        "MenuItem" => ElementType::MenuItem,

        // Toolbars
        "ToolBar" => ElementType::ToolBar,
        "ToolItem" => ElementType::ToolItem,
        "CoolBar" => ElementType::ToolBar,
        "CoolItem" => ElementType::ToolItem,

        // RCP specific
        "ViewPart" => ElementType::View,
        "ViewSite" => ElementType::View,
        "EditorPart" => ElementType::Editor,
        "EditorSite" => ElementType::Editor,
        "WorkbenchPage" => ElementType::Panel,
        "Perspective" => ElementType::Perspective,

        // Unknown
        _ => ElementType::Widget,
    }
}

/// Fast type mapping from any class name to ElementType
#[inline]
pub fn map_element_type(class_name: &str, toolkit: ToolkitType) -> ElementType {
    // Extract simple name from fully qualified class name
    let simple_name = class_name.rsplit('.').next().unwrap_or(class_name);

    match toolkit {
        ToolkitType::Swing => map_swing_type(simple_name),
        ToolkitType::Swt | ToolkitType::Rcp => map_swt_type(simple_name),
    }
}

/// Normalize a short class name for Swing
/// Converts generic names like "Button" to "JButton"
#[inline]
pub fn normalize_swing_class(name: &str) -> &'static str {
    match name {
        "Button" | "JButton" => "JButton",
        "TextField" | "JTextField" => "JTextField",
        "TextArea" | "JTextArea" => "JTextArea",
        "Text" => "JTextField",
        "Label" | "JLabel" => "JLabel",
        "ComboBox" | "JComboBox" | "Combo" => "JComboBox",
        "List" | "JList" => "JList",
        "Table" | "JTable" => "JTable",
        "Tree" | "JTree" => "JTree",
        "CheckBox" | "JCheckBox" => "JCheckBox",
        "RadioButton" | "JRadioButton" => "JRadioButton",
        "Panel" | "JPanel" | "Composite" => "JPanel",
        "Frame" | "JFrame" | "Shell" => "JFrame",
        "Dialog" | "JDialog" => "JDialog",
        "ScrollPane" | "JScrollPane" => "JScrollPane",
        "SplitPane" | "JSplitPane" | "SashForm" => "JSplitPane",
        "TabbedPane" | "JTabbedPane" | "TabFolder" => "JTabbedPane",
        "MenuBar" | "JMenuBar" => "JMenuBar",
        "Menu" | "JMenu" => "JMenu",
        "MenuItem" | "JMenuItem" => "JMenuItem",
        "ToolBar" | "JToolBar" => "JToolBar",
        "ProgressBar" | "JProgressBar" => "JProgressBar",
        "Slider" | "JSlider" | "Scale" => "JSlider",
        "Spinner" | "JSpinner" => "JSpinner",
        "PasswordField" | "JPasswordField" => "JPasswordField",
        _ => name,
    }
}

/// Normalize a short class name for SWT
/// Converts Swing names like "JButton" to "Button"
#[inline]
pub fn normalize_swt_class(name: &str) -> &'static str {
    match name {
        "JButton" | "Button" => "Button",
        "JTextField" | "TextField" | "Text" | "JTextArea" | "TextArea" => "Text",
        "JLabel" | "Label" => "Label",
        "JComboBox" | "ComboBox" | "Combo" => "Combo",
        "JList" | "List" => "List",
        "JTable" | "Table" => "Table",
        "JTree" | "Tree" => "Tree",
        "JCheckBox" | "CheckBox" => "Button", // SWT uses Button with style
        "JRadioButton" | "RadioButton" => "Button", // SWT uses Button with style
        "JPanel" | "Panel" | "Composite" => "Composite",
        "JFrame" | "Frame" | "JDialog" | "Dialog" | "Shell" => "Shell",
        "JScrollPane" | "ScrollPane" => "ScrolledComposite",
        "JSplitPane" | "SplitPane" | "SashForm" => "SashForm",
        "JTabbedPane" | "TabbedPane" | "TabFolder" => "TabFolder",
        "JMenuBar" | "MenuBar" | "Menu" | "JMenu" => "Menu",
        "JMenuItem" | "MenuItem" => "MenuItem",
        "JToolBar" | "ToolBar" => "ToolBar",
        "JProgressBar" | "ProgressBar" => "ProgressBar",
        "JSlider" | "Slider" | "Scale" => "Scale",
        "JSpinner" | "Spinner" => "Spinner",
        "JPasswordField" | "PasswordField" => "Text", // SWT Text with password style
        "Group" => "Group",
        _ => name,
    }
}

/// Normalize class name for a specific toolkit
#[inline]
pub fn normalize_class_name(name: &str, toolkit: ToolkitType) -> &'static str {
    match toolkit {
        ToolkitType::Swing => normalize_swing_class(name),
        ToolkitType::Swt | ToolkitType::Rcp => normalize_swt_class(name),
    }
}

/// Check if a class name is a container type
#[inline]
pub fn is_container_class(simple_name: &str, toolkit: ToolkitType) -> bool {
    match toolkit {
        ToolkitType::Swing => matches!(simple_name,
            "JPanel" | "JFrame" | "JDialog" | "JInternalFrame" |
            "JScrollPane" | "JSplitPane" | "JTabbedPane" |
            "JDesktopPane" | "JLayeredPane" | "JRootPane" | "JViewport"
        ),
        ToolkitType::Swt | ToolkitType::Rcp => matches!(simple_name,
            "Composite" | "ScrolledComposite" | "Group" | "Shell" |
            "TabFolder" | "CTabFolder" | "SashForm" | "Canvas"
        ),
    }
}

/// Check if a class name is a text input type
#[inline]
pub fn is_text_input_class(simple_name: &str, toolkit: ToolkitType) -> bool {
    match toolkit {
        ToolkitType::Swing => matches!(simple_name,
            "JTextField" | "JFormattedTextField" | "JTextArea" |
            "JEditorPane" | "JTextPane" | "JPasswordField" | "JSpinner"
        ),
        ToolkitType::Swt | ToolkitType::Rcp => matches!(simple_name,
            "Text" | "StyledText" | "Spinner"
        ),
    }
}

/// Check if a class name is a button-like type
#[inline]
pub fn is_button_class(simple_name: &str, toolkit: ToolkitType) -> bool {
    match toolkit {
        ToolkitType::Swing => matches!(simple_name,
            "JButton" | "JToggleButton" | "JCheckBox" | "JRadioButton" |
            "JMenuItem" | "JCheckBoxMenuItem" | "JRadioButtonMenuItem"
        ),
        ToolkitType::Swt | ToolkitType::Rcp => matches!(simple_name,
            "Button" | "MenuItem" | "ToolItem"
        ),
    }
}

/// Lookup table for class name to type string (used for RPC)
pub struct TypeLookup;

impl TypeLookup {
    /// Get the locator type string for RPC
    #[inline]
    pub fn get_locator_type(element_type: ElementType) -> &'static str {
        match element_type {
            ElementType::Button | ElementType::ToggleButton |
            ElementType::CheckBox | ElementType::RadioButton => "button",

            ElementType::TextField | ElementType::TextArea |
            ElementType::PasswordField => "text",

            ElementType::ComboBox => "combobox",
            ElementType::List => "list",
            ElementType::Table => "table",
            ElementType::Tree => "tree",

            ElementType::Label => "label",
            ElementType::ProgressBar => "progressbar",
            ElementType::Slider => "slider",
            ElementType::Spinner => "spinner",

            ElementType::Panel | ElementType::Frame | ElementType::Dialog |
            ElementType::Shell | ElementType::Group | ElementType::ScrollPane |
            ElementType::SplitPane | ElementType::TabbedPane => "container",

            ElementType::MenuBar | ElementType::Menu |
            ElementType::MenuItem | ElementType::PopupMenu => "menu",

            ElementType::ToolBar | ElementType::ToolItem => "toolbar",

            ElementType::View | ElementType::Editor |
            ElementType::Perspective => "rcp",

            ElementType::Widget | ElementType::Unknown => "component",
        }
    }

    /// Check if element type supports text input
    #[inline]
    pub fn supports_text_input(element_type: ElementType) -> bool {
        matches!(element_type,
            ElementType::TextField | ElementType::TextArea |
            ElementType::PasswordField | ElementType::Spinner |
            ElementType::ComboBox
        )
    }

    /// Check if element type supports selection
    #[inline]
    pub fn supports_selection(element_type: ElementType) -> bool {
        matches!(element_type,
            ElementType::ComboBox | ElementType::List |
            ElementType::Table | ElementType::Tree |
            ElementType::TabbedPane | ElementType::CheckBox |
            ElementType::RadioButton | ElementType::ToggleButton
        )
    }

    /// Check if element type supports click
    #[inline]
    pub fn supports_click(element_type: ElementType) -> bool {
        matches!(element_type,
            ElementType::Button | ElementType::ToggleButton |
            ElementType::CheckBox | ElementType::RadioButton |
            ElementType::MenuItem | ElementType::ToolItem |
            ElementType::Label | ElementType::Tree | ElementType::Table
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_swing_type() {
        assert_eq!(map_swing_type("JButton"), ElementType::Button);
        assert_eq!(map_swing_type("JTextField"), ElementType::TextField);
        assert_eq!(map_swing_type("JTable"), ElementType::Table);
        assert_eq!(map_swing_type("JPanel"), ElementType::Panel);
        assert_eq!(map_swing_type("UnknownWidget"), ElementType::Widget);
    }

    #[test]
    fn test_map_swt_type() {
        assert_eq!(map_swt_type("Button"), ElementType::Button);
        assert_eq!(map_swt_type("Text"), ElementType::TextField);
        assert_eq!(map_swt_type("Table"), ElementType::Table);
        assert_eq!(map_swt_type("Composite"), ElementType::Panel);
        assert_eq!(map_swt_type("Shell"), ElementType::Shell);
    }

    #[test]
    fn test_map_element_type_from_fqn() {
        assert_eq!(
            map_element_type("javax.swing.JButton", ToolkitType::Swing),
            ElementType::Button
        );
        assert_eq!(
            map_element_type("org.eclipse.swt.widgets.Button", ToolkitType::Swt),
            ElementType::Button
        );
    }

    #[test]
    fn test_normalize_swing_class() {
        assert_eq!(normalize_swing_class("Button"), "JButton");
        assert_eq!(normalize_swing_class("JButton"), "JButton");
        assert_eq!(normalize_swing_class("TextField"), "JTextField");
        assert_eq!(normalize_swing_class("Text"), "JTextField");
    }

    #[test]
    fn test_normalize_swt_class() {
        assert_eq!(normalize_swt_class("JButton"), "Button");
        assert_eq!(normalize_swt_class("Button"), "Button");
        assert_eq!(normalize_swt_class("JPanel"), "Composite");
        assert_eq!(normalize_swt_class("Panel"), "Composite");
    }

    #[test]
    fn test_is_container_class() {
        assert!(is_container_class("JPanel", ToolkitType::Swing));
        assert!(is_container_class("JFrame", ToolkitType::Swing));
        assert!(!is_container_class("JButton", ToolkitType::Swing));

        assert!(is_container_class("Composite", ToolkitType::Swt));
        assert!(is_container_class("Shell", ToolkitType::Swt));
        assert!(!is_container_class("Button", ToolkitType::Swt));
    }

    #[test]
    fn test_is_text_input_class() {
        assert!(is_text_input_class("JTextField", ToolkitType::Swing));
        assert!(is_text_input_class("JTextArea", ToolkitType::Swing));
        assert!(!is_text_input_class("JButton", ToolkitType::Swing));

        assert!(is_text_input_class("Text", ToolkitType::Swt));
        assert!(is_text_input_class("StyledText", ToolkitType::Swt));
        assert!(!is_text_input_class("Button", ToolkitType::Swt));
    }

    #[test]
    fn test_type_lookup() {
        assert_eq!(TypeLookup::get_locator_type(ElementType::Button), "button");
        assert_eq!(TypeLookup::get_locator_type(ElementType::TextField), "text");
        assert_eq!(TypeLookup::get_locator_type(ElementType::Panel), "container");

        assert!(TypeLookup::supports_text_input(ElementType::TextField));
        assert!(!TypeLookup::supports_text_input(ElementType::Button));

        assert!(TypeLookup::supports_selection(ElementType::ComboBox));
        assert!(!TypeLookup::supports_selection(ElementType::Label));

        assert!(TypeLookup::supports_click(ElementType::Button));
        assert!(!TypeLookup::supports_click(ElementType::Panel));
    }
}
