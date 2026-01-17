//! SWT Widget model for the Python bindings and matcher
//!
//! This module provides the comprehensive widget model for SWT/RCP applications,
//! covering all SWT widget types with their specific properties and style bits.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Widget Identification
// =============================================================================

/// Unique identifier for a widget instance
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WidgetId {
    /// Native widget handle (platform-specific pointer)
    pub handle: i64,
    /// Data key if set via setData(String, Object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_key: Option<String>,
    /// Path in the widget tree (e.g., "0.1.2")
    pub tree_path: String,
    /// Depth in the tree hierarchy
    pub depth: u32,
}

impl WidgetId {
    /// Create a new WidgetId
    pub fn new(handle: i64, tree_path: String, depth: u32) -> Self {
        Self {
            handle,
            data_key: None,
            tree_path,
            depth,
        }
    }

    /// Create a WidgetId with a data key
    pub fn with_data_key(handle: i64, data_key: String, tree_path: String, depth: u32) -> Self {
        Self {
            handle,
            data_key: Some(data_key),
            tree_path,
            depth,
        }
    }
}

// =============================================================================
// SWT Widget Type Enumeration
// =============================================================================

/// All SWT widget types for categorization and matching
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SwtWidgetType {
    // === Top-level Shells ===
    /// Top-level window (org.eclipse.swt.widgets.Shell)
    Shell,
    /// Decorations base class
    Decorations,

    // === Containers ===
    /// Basic container (org.eclipse.swt.widgets.Composite)
    Composite,
    /// Grouping container with border and label
    Group,
    /// Tab container
    TabFolder,
    /// Individual tab item
    TabItem,
    /// CTabFolder (custom styled tabs)
    CTabFolder,
    /// CTabItem
    CTabItem,
    /// Sash container for resizable panels
    SashForm,
    /// Scrollable composite
    ScrolledComposite,
    /// Expandable bar
    ExpandBar,
    /// Expandable item within ExpandBar
    ExpandItem,
    /// Canvas for custom drawing
    Canvas,
    /// Coolbar container
    CoolBar,
    /// CoolBar item
    CoolItem,
    /// Toolbar container
    ToolBar,
    /// Toolbar item
    ToolItem,

    // === Basic Controls ===
    /// Push button
    Button,
    /// Text label
    Label,
    /// Single or multi-line text input
    Text,
    /// Styled text (with formatting)
    StyledText,
    /// Dropdown or list combo
    Combo,
    /// CCombo (custom combo)
    CCombo,
    /// Selection list
    List,
    /// Link widget (clickable text)
    Link,

    // === Complex Controls ===
    /// Table widget
    Table,
    /// Table column
    TableColumn,
    /// Table item (row)
    TableItem,
    /// Tree widget
    Tree,
    /// Tree column
    TreeColumn,
    /// Tree item (node)
    TreeItem,
    /// DateTime picker
    DateTime,
    /// Spinner (numeric input)
    Spinner,
    /// Scale (slider)
    Scale,
    /// Slider
    Slider,
    /// Progress bar
    ProgressBar,
    /// Browser widget (embedded browser)
    Browser,

    // === Menu System ===
    /// Menu bar or popup menu
    Menu,
    /// Menu item
    MenuItem,

    // === Dialogs ===
    /// Message box dialog
    MessageBox,
    /// Color selection dialog
    ColorDialog,
    /// Directory selection dialog
    DirectoryDialog,
    /// File selection dialog
    FileDialog,
    /// Font selection dialog
    FontDialog,
    /// Print dialog
    PrintDialog,

    // === Miscellaneous ===
    /// Separator line
    Separator,
    /// Tooltip/balloon
    ToolTip,
    /// Tray icon
    TrayItem,
    /// System tray
    Tray,
    /// Caret (text cursor)
    Caret,
    /// Tracker (resize/move feedback)
    Tracker,

    // === JFace/RCP Viewers (common in Eclipse RCP) ===
    /// TableViewer wrapper
    TableViewer,
    /// TreeViewer wrapper
    TreeViewer,
    /// ListViewer wrapper
    ListViewer,
    /// ComboViewer wrapper
    ComboViewer,

    // === Eclipse Forms (common in RCP) ===
    /// Form widget
    Form,
    /// Scrollable form
    ScrolledForm,
    /// Section (collapsible area)
    Section,
    /// Hyperlink
    Hyperlink,
    /// ImageHyperlink
    ImageHyperlink,
    /// FormText (formatted text)
    FormText,

    // === Nebula Widgets (common extensions) ===
    /// Grid widget (enhanced table)
    Grid,
    /// GridItem
    GridItem,
    /// GridColumn
    GridColumn,
    /// Gallery widget
    Gallery,
    /// GalleryItem
    GalleryItem,

    // === Custom/Unknown ===
    /// Custom or unrecognized widget
    Custom,
    /// Unknown widget type
    Unknown,
}

impl SwtWidgetType {
    /// Create from SWT class name
    pub fn from_class_name(class_name: &str) -> Self {
        let simple = class_name.rsplit('.').next().unwrap_or(class_name);

        match simple {
            // Top-level
            "Shell" => Self::Shell,
            "Decorations" => Self::Decorations,

            // Containers
            "Composite" => Self::Composite,
            "Group" => Self::Group,
            "TabFolder" => Self::TabFolder,
            "TabItem" => Self::TabItem,
            "CTabFolder" => Self::CTabFolder,
            "CTabItem" => Self::CTabItem,
            "SashForm" => Self::SashForm,
            "ScrolledComposite" => Self::ScrolledComposite,
            "ExpandBar" => Self::ExpandBar,
            "ExpandItem" => Self::ExpandItem,
            "Canvas" => Self::Canvas,
            "CoolBar" => Self::CoolBar,
            "CoolItem" => Self::CoolItem,
            "ToolBar" => Self::ToolBar,
            "ToolItem" => Self::ToolItem,

            // Basic controls
            "Button" => Self::Button,
            "Label" => Self::Label,
            "Text" => Self::Text,
            "StyledText" => Self::StyledText,
            "Combo" => Self::Combo,
            "CCombo" => Self::CCombo,
            "List" => Self::List,
            "Link" => Self::Link,

            // Complex controls
            "Table" => Self::Table,
            "TableColumn" => Self::TableColumn,
            "TableItem" => Self::TableItem,
            "Tree" => Self::Tree,
            "TreeColumn" => Self::TreeColumn,
            "TreeItem" => Self::TreeItem,
            "DateTime" => Self::DateTime,
            "Spinner" => Self::Spinner,
            "Scale" => Self::Scale,
            "Slider" => Self::Slider,
            "ProgressBar" => Self::ProgressBar,
            "Browser" => Self::Browser,

            // Menu
            "Menu" => Self::Menu,
            "MenuItem" => Self::MenuItem,

            // Dialogs
            "MessageBox" => Self::MessageBox,
            "ColorDialog" => Self::ColorDialog,
            "DirectoryDialog" => Self::DirectoryDialog,
            "FileDialog" => Self::FileDialog,
            "FontDialog" => Self::FontDialog,
            "PrintDialog" => Self::PrintDialog,

            // Misc
            "Separator" => Self::Separator,
            "ToolTip" => Self::ToolTip,
            "TrayItem" => Self::TrayItem,
            "Tray" => Self::Tray,
            "Caret" => Self::Caret,
            "Tracker" => Self::Tracker,

            // JFace Viewers
            "TableViewer" => Self::TableViewer,
            "TreeViewer" => Self::TreeViewer,
            "ListViewer" => Self::ListViewer,
            "ComboViewer" => Self::ComboViewer,

            // Eclipse Forms
            "Form" => Self::Form,
            "ScrolledForm" => Self::ScrolledForm,
            "Section" => Self::Section,
            "Hyperlink" => Self::Hyperlink,
            "ImageHyperlink" => Self::ImageHyperlink,
            "FormText" => Self::FormText,

            // Nebula
            "Grid" => Self::Grid,
            "GridItem" => Self::GridItem,
            "GridColumn" => Self::GridColumn,
            "Gallery" => Self::Gallery,
            "GalleryItem" => Self::GalleryItem,

            _ => Self::Unknown,
        }
    }

    /// Get the simple class name for this widget type
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Shell => "Shell",
            Self::Decorations => "Decorations",
            Self::Composite => "Composite",
            Self::Group => "Group",
            Self::TabFolder => "TabFolder",
            Self::TabItem => "TabItem",
            Self::CTabFolder => "CTabFolder",
            Self::CTabItem => "CTabItem",
            Self::SashForm => "SashForm",
            Self::ScrolledComposite => "ScrolledComposite",
            Self::ExpandBar => "ExpandBar",
            Self::ExpandItem => "ExpandItem",
            Self::Canvas => "Canvas",
            Self::CoolBar => "CoolBar",
            Self::CoolItem => "CoolItem",
            Self::ToolBar => "ToolBar",
            Self::ToolItem => "ToolItem",
            Self::Button => "Button",
            Self::Label => "Label",
            Self::Text => "Text",
            Self::StyledText => "StyledText",
            Self::Combo => "Combo",
            Self::CCombo => "CCombo",
            Self::List => "List",
            Self::Link => "Link",
            Self::Table => "Table",
            Self::TableColumn => "TableColumn",
            Self::TableItem => "TableItem",
            Self::Tree => "Tree",
            Self::TreeColumn => "TreeColumn",
            Self::TreeItem => "TreeItem",
            Self::DateTime => "DateTime",
            Self::Spinner => "Spinner",
            Self::Scale => "Scale",
            Self::Slider => "Slider",
            Self::ProgressBar => "ProgressBar",
            Self::Browser => "Browser",
            Self::Menu => "Menu",
            Self::MenuItem => "MenuItem",
            Self::MessageBox => "MessageBox",
            Self::ColorDialog => "ColorDialog",
            Self::DirectoryDialog => "DirectoryDialog",
            Self::FileDialog => "FileDialog",
            Self::FontDialog => "FontDialog",
            Self::PrintDialog => "PrintDialog",
            Self::Separator => "Separator",
            Self::ToolTip => "ToolTip",
            Self::TrayItem => "TrayItem",
            Self::Tray => "Tray",
            Self::Caret => "Caret",
            Self::Tracker => "Tracker",
            Self::TableViewer => "TableViewer",
            Self::TreeViewer => "TreeViewer",
            Self::ListViewer => "ListViewer",
            Self::ComboViewer => "ComboViewer",
            Self::Form => "Form",
            Self::ScrolledForm => "ScrolledForm",
            Self::Section => "Section",
            Self::Hyperlink => "Hyperlink",
            Self::ImageHyperlink => "ImageHyperlink",
            Self::FormText => "FormText",
            Self::Grid => "Grid",
            Self::GridItem => "GridItem",
            Self::GridColumn => "GridColumn",
            Self::Gallery => "Gallery",
            Self::GalleryItem => "GalleryItem",
            Self::Custom => "Custom",
            Self::Unknown => "Widget",
        }
    }

    /// Check if this is a container type
    pub fn is_container(&self) -> bool {
        matches!(
            self,
            Self::Shell
                | Self::Decorations
                | Self::Composite
                | Self::Group
                | Self::TabFolder
                | Self::CTabFolder
                | Self::SashForm
                | Self::ScrolledComposite
                | Self::ExpandBar
                | Self::Canvas
                | Self::CoolBar
                | Self::ToolBar
                | Self::Form
                | Self::ScrolledForm
                | Self::Section
        )
    }

    /// Check if this is a text input widget
    pub fn is_text_input(&self) -> bool {
        matches!(
            self,
            Self::Text | Self::StyledText | Self::Combo | Self::CCombo | Self::Spinner
        )
    }

    /// Check if this is a button-like widget
    pub fn is_button(&self) -> bool {
        matches!(self, Self::Button | Self::Link | Self::Hyperlink | Self::ImageHyperlink)
    }

    /// Check if this is a selection widget (list, table, tree)
    pub fn is_selection(&self) -> bool {
        matches!(
            self,
            Self::List
                | Self::Table
                | Self::Tree
                | Self::Combo
                | Self::CCombo
                | Self::Grid
                | Self::Gallery
        )
    }

    /// Check if this is an item type (part of a collection)
    pub fn is_item(&self) -> bool {
        matches!(
            self,
            Self::TabItem
                | Self::CTabItem
                | Self::ExpandItem
                | Self::CoolItem
                | Self::ToolItem
                | Self::TableColumn
                | Self::TableItem
                | Self::TreeColumn
                | Self::TreeItem
                | Self::MenuItem
                | Self::TrayItem
                | Self::GridItem
                | Self::GridColumn
                | Self::GalleryItem
        )
    }
}

// =============================================================================
// Widget Bounds and Geometry
// =============================================================================

/// Rectangle bounds for widget geometry
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WidgetBounds {
    /// X coordinate (relative to parent or display)
    pub x: i32,
    /// Y coordinate
    pub y: i32,
    /// Width in pixels
    pub width: i32,
    /// Height in pixels
    pub height: i32,
}

impl WidgetBounds {
    /// Create new bounds
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }

    /// Check if a point is inside these bounds
    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + self.height
    }

    /// Get the center point
    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    /// Check if these bounds intersect with another
    pub fn intersects(&self, other: &WidgetBounds) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

// =============================================================================
// Widget State
// =============================================================================

/// State flags for a widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwtWidgetState {
    /// Widget is visible
    pub visible: bool,
    /// Widget is enabled (can receive input)
    pub enabled: bool,
    /// Widget has been disposed
    pub disposed: bool,
    /// Widget has keyboard focus
    pub focused: bool,
    /// Selection state (for buttons, list items, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<bool>,
    /// Widget is currently active (for shells)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Widget is maximized (for shells)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximized: Option<bool>,
    /// Widget is minimized (for shells)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimized: Option<bool>,
    /// Text is editable (for text widgets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    /// Item is expanded (for tree items, sections)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded: Option<bool>,
    /// Item is checked (for checkboxes, table items with CHECK style)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    /// Item is grayed (for checkboxes with GRAYED style)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grayed: Option<bool>,
}

impl Default for SwtWidgetState {
    fn default() -> Self {
        Self {
            visible: true,
            enabled: true,
            disposed: false,
            focused: false,
            selection: None,
            active: None,
            maximized: None,
            minimized: None,
            editable: None,
            expanded: None,
            checked: None,
            grayed: None,
        }
    }
}

// =============================================================================
// SWT Style System
// =============================================================================

/// SWT style bits and their string representations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SwtStyle {
    /// Raw style bits (integer value)
    pub style_bits: i32,
    /// Parsed style names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style_names: Vec<String>,
}

impl SwtStyle {
    /// Create new style from bits
    pub fn new(style_bits: i32) -> Self {
        Self {
            style_bits,
            style_names: Self::parse_style_bits(style_bits),
        }
    }

    /// Check if a specific style bit is set
    pub fn has_style(&self, style: SwtStyleBit) -> bool {
        (self.style_bits & style.value()) != 0
    }

    /// Check if any of the given styles are set
    pub fn has_any_style(&self, styles: &[SwtStyleBit]) -> bool {
        styles.iter().any(|s| self.has_style(*s))
    }

    /// Parse style bits into readable names
    fn parse_style_bits(bits: i32) -> Vec<String> {
        let mut names = Vec::new();

        // Check each known style bit
        for style in SwtStyleBit::all() {
            if (bits & style.value()) != 0 {
                names.push(style.name().to_string());
            }
        }

        names
    }
}

/// Common SWT style bit constants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwtStyleBit {
    // === General Styles ===
    None,
    Default,

    // === Button Styles ===
    Push,
    Check,
    Radio,
    Toggle,
    Arrow,
    Flat,

    // === Arrow Directions ===
    Up,
    Down,
    Left,
    Right,

    // === Text Styles ===
    SingleLine,
    MultiLine,
    ReadOnly,
    Wrap,
    Password,
    Search,
    IconCancel,
    IconSearch,

    // === Alignment Styles ===
    AlignLeft,
    AlignCenter,
    AlignRight,
    AlignTop,
    AlignBottom,
    AlignFill,

    // === Border Styles ===
    Border,
    NoBorder,

    // === Selection Styles ===
    Single,
    Multi,

    // === Scrollbar Styles ===
    HScroll,
    VScroll,

    // === Shell Styles ===
    DialogTrim,
    ShellTrim,
    Resize,
    Title,
    Close,
    Min,
    Max,
    NoTrim,
    OnTop,
    Tool,
    NoFocus,
    Modal,
    ApplicationModal,
    SystemModal,
    PrimaryModal,

    // === Table/Tree Styles ===
    FullSelection,
    HideSelection,
    Virtual,
    CheckBoxes,
    HeaderVisible,
    LinesVisible,

    // === DateTime Styles ===
    Date,
    Time,
    Calendar,
    Short,
    Medium,
    Long,
    DropDown,

    // === Progress Styles ===
    Smooth,
    Indeterminate,

    // === Separator Style ===
    Horizontal,
    Vertical,
    ShadowIn,
    ShadowOut,
    ShadowNone,

    // === Menu Styles ===
    Bar,
    PopUp,
    Cascade,
    Separator,

    // === Misc ===
    DoubleBuffered,
    Transparent,
    Mirrored,
    RightToLeft,
    LeftToRight,
}

impl SwtStyleBit {
    /// Get the integer value for this style bit
    pub fn value(&self) -> i32 {
        match self {
            Self::None => 0,
            Self::Default => 1 << 0,  // SWT.DEFAULT = -1, using 1 for detection
            Self::Push => 1 << 3,
            Self::Check => 1 << 5,
            Self::Radio => 1 << 4,
            Self::Toggle => 1 << 1,
            Self::Arrow => 1 << 2,
            Self::Flat => 1 << 23,
            Self::Up => 1 << 7,
            Self::Down => 1 << 10,
            Self::Left => 1 << 14,
            Self::Right => 1 << 17,
            Self::SingleLine => 1 << 2,
            Self::MultiLine => 1 << 1,
            Self::ReadOnly => 1 << 3,
            Self::Wrap => 1 << 6,
            Self::Password => 1 << 22,
            Self::Search => 1 << 7,
            Self::IconCancel => 1 << 8,
            Self::IconSearch => 1 << 9,
            Self::AlignLeft => 1 << 14,
            Self::AlignCenter => 1 << 24,
            Self::AlignRight => 1 << 17,
            Self::AlignTop => 1 << 7,
            Self::AlignBottom => 1 << 10,
            Self::AlignFill => 1 << 18,
            Self::Border => 1 << 11,
            Self::NoBorder => 1 << 20,
            Self::Single => 1 << 2,
            Self::Multi => 1 << 1,
            Self::HScroll => 1 << 8,
            Self::VScroll => 1 << 9,
            Self::DialogTrim => (1 << 5) | (1 << 11) | (1 << 6),  // TITLE | CLOSE | BORDER
            Self::ShellTrim => (1 << 5) | (1 << 11) | (1 << 6) | (1 << 4) | (1 << 7),
            Self::Resize => 1 << 4,
            Self::Title => 1 << 5,
            Self::Close => 1 << 6,
            Self::Min => 1 << 7,
            Self::Max => 1 << 10,
            Self::NoTrim => 1 << 3,
            Self::OnTop => 1 << 14,
            Self::Tool => 1 << 2,
            Self::NoFocus => 1 << 19,
            Self::Modal => 1 << 16,
            Self::ApplicationModal => 1 << 16,
            Self::SystemModal => 1 << 17,
            Self::PrimaryModal => 1 << 15,
            Self::FullSelection => 1 << 16,
            Self::HideSelection => 1 << 15,
            Self::Virtual => 1 << 28,
            Self::CheckBoxes => 1 << 5,
            Self::HeaderVisible => 1 << 1, // Not a real style bit, used for state
            Self::LinesVisible => 1 << 2,  // Not a real style bit, used for state
            Self::Date => 1 << 5,
            Self::Time => 1 << 7,
            Self::Calendar => 1 << 10,
            Self::Short => 1 << 15,
            Self::Medium => 1 << 16,
            Self::Long => 1 << 28,
            Self::DropDown => 1 << 2,
            Self::Smooth => 1 << 16,
            Self::Indeterminate => 1 << 1,
            Self::Horizontal => 1 << 8,
            Self::Vertical => 1 << 9,
            Self::ShadowIn => 1 << 2,
            Self::ShadowOut => 1 << 3,
            Self::ShadowNone => 1 << 5,
            Self::Bar => 1 << 1,
            Self::PopUp => 1 << 3,
            Self::Cascade => 1 << 6,
            Self::Separator => 1 << 1,
            Self::DoubleBuffered => 1 << 29,
            Self::Transparent => 1 << 30,
            Self::Mirrored => 1 << 27,
            Self::RightToLeft => 1 << 26,
            Self::LeftToRight => 1 << 25,
        }
    }

    /// Get the name of this style bit
    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Default => "DEFAULT",
            Self::Push => "PUSH",
            Self::Check => "CHECK",
            Self::Radio => "RADIO",
            Self::Toggle => "TOGGLE",
            Self::Arrow => "ARROW",
            Self::Flat => "FLAT",
            Self::Up => "UP",
            Self::Down => "DOWN",
            Self::Left => "LEFT",
            Self::Right => "RIGHT",
            Self::SingleLine => "SINGLE",
            Self::MultiLine => "MULTI",
            Self::ReadOnly => "READ_ONLY",
            Self::Wrap => "WRAP",
            Self::Password => "PASSWORD",
            Self::Search => "SEARCH",
            Self::IconCancel => "ICON_CANCEL",
            Self::IconSearch => "ICON_SEARCH",
            Self::AlignLeft => "LEFT",
            Self::AlignCenter => "CENTER",
            Self::AlignRight => "RIGHT",
            Self::AlignTop => "TOP",
            Self::AlignBottom => "BOTTOM",
            Self::AlignFill => "FILL",
            Self::Border => "BORDER",
            Self::NoBorder => "NO_BORDER",
            Self::Single => "SINGLE",
            Self::Multi => "MULTI",
            Self::HScroll => "H_SCROLL",
            Self::VScroll => "V_SCROLL",
            Self::DialogTrim => "DIALOG_TRIM",
            Self::ShellTrim => "SHELL_TRIM",
            Self::Resize => "RESIZE",
            Self::Title => "TITLE",
            Self::Close => "CLOSE",
            Self::Min => "MIN",
            Self::Max => "MAX",
            Self::NoTrim => "NO_TRIM",
            Self::OnTop => "ON_TOP",
            Self::Tool => "TOOL",
            Self::NoFocus => "NO_FOCUS",
            Self::Modal => "MODAL",
            Self::ApplicationModal => "APPLICATION_MODAL",
            Self::SystemModal => "SYSTEM_MODAL",
            Self::PrimaryModal => "PRIMARY_MODAL",
            Self::FullSelection => "FULL_SELECTION",
            Self::HideSelection => "HIDE_SELECTION",
            Self::Virtual => "VIRTUAL",
            Self::CheckBoxes => "CHECK",
            Self::HeaderVisible => "HEADER_VISIBLE",
            Self::LinesVisible => "LINES_VISIBLE",
            Self::Date => "DATE",
            Self::Time => "TIME",
            Self::Calendar => "CALENDAR",
            Self::Short => "SHORT",
            Self::Medium => "MEDIUM",
            Self::Long => "LONG",
            Self::DropDown => "DROP_DOWN",
            Self::Smooth => "SMOOTH",
            Self::Indeterminate => "INDETERMINATE",
            Self::Horizontal => "HORIZONTAL",
            Self::Vertical => "VERTICAL",
            Self::ShadowIn => "SHADOW_IN",
            Self::ShadowOut => "SHADOW_OUT",
            Self::ShadowNone => "SHADOW_NONE",
            Self::Bar => "BAR",
            Self::PopUp => "POP_UP",
            Self::Cascade => "CASCADE",
            Self::Separator => "SEPARATOR",
            Self::DoubleBuffered => "DOUBLE_BUFFERED",
            Self::Transparent => "TRANSPARENT",
            Self::Mirrored => "MIRRORED",
            Self::RightToLeft => "RIGHT_TO_LEFT",
            Self::LeftToRight => "LEFT_TO_RIGHT",
        }
    }

    /// Get all style bits for parsing
    fn all() -> &'static [SwtStyleBit] {
        &[
            Self::Push, Self::Check, Self::Radio, Self::Toggle, Self::Arrow, Self::Flat,
            Self::Up, Self::Down, Self::Left, Self::Right,
            Self::MultiLine, Self::ReadOnly, Self::Wrap, Self::Password, Self::Search,
            Self::AlignCenter, Self::Border, Self::Multi,
            Self::HScroll, Self::VScroll, Self::Resize, Self::Title, Self::Close,
            Self::Min, Self::Max, Self::OnTop, Self::NoFocus, Self::Modal,
            Self::FullSelection, Self::HideSelection, Self::Virtual,
            Self::Smooth, Self::Indeterminate, Self::Horizontal, Self::Vertical,
            Self::DoubleBuffered, Self::Transparent,
        ]
    }
}

// =============================================================================
// Widget-Specific Properties
// =============================================================================

/// Widget-specific properties based on widget type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SwtWidgetProperties {
    /// Shell properties
    Shell(ShellProps),
    /// Button properties
    Button(ButtonProps),
    /// Text widget properties
    Text(TextProps),
    /// StyledText properties
    StyledText(StyledTextProps),
    /// Combo properties
    Combo(ComboProps),
    /// List properties
    List(ListProps),
    /// Table properties
    Table(TableProps),
    /// TableItem properties
    TableItem(TableItemProps),
    /// Tree properties
    Tree(TreeProps),
    /// TreeItem properties
    TreeItem(TreeItemProps),
    /// TabFolder properties
    TabFolder(TabFolderProps),
    /// TabItem properties
    TabItem(TabItemProps),
    /// Spinner properties
    Spinner(SpinnerProps),
    /// Scale/Slider properties
    Scale(ScaleProps),
    /// ProgressBar properties
    ProgressBar(ProgressBarProps),
    /// DateTime properties
    DateTime(DateTimeProps),
    /// Menu properties
    Menu(MenuProps),
    /// MenuItem properties
    MenuItem(MenuItemProps),
    /// ToolBar properties
    ToolBar(ToolBarProps),
    /// ToolItem properties
    ToolItem(ToolItemProps),
    /// Browser properties
    Browser(BrowserProps),
    /// Link properties
    Link(LinkProps),
    /// ExpandBar properties
    ExpandBar(ExpandBarProps),
    /// Section (Eclipse Forms) properties
    Section(SectionProps),
    /// Generic properties for unknown widgets
    Generic(GenericProps),
}

impl Default for SwtWidgetProperties {
    fn default() -> Self {
        Self::Generic(GenericProps::default())
    }
}

/// Shell (window) properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShellProps {
    /// Window title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Whether the shell is a modal dialog
    pub modal: bool,
    /// Shell modality type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modality: Option<String>,
    /// Alpha (transparency) value 0-255
    pub alpha: i32,
    /// Whether shell is full screen
    pub full_screen: bool,
    /// Default button widget ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_button_id: Option<i64>,
    /// IME input mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ime_input_mode: Option<i32>,
}

/// Button properties (push, check, radio, toggle, arrow)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ButtonProps {
    /// Button text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Selection state (for check, radio, toggle)
    pub selection: bool,
    /// Button style (push, check, radio, toggle, arrow)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_style: Option<String>,
    /// Alignment
    pub alignment: i32,
    /// Grayed state (for three-state checkbox)
    pub grayed: bool,
    /// Image present
    pub has_image: bool,
}

/// Text widget properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextProps {
    /// Current text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Text limit (max characters, 0 = unlimited)
    pub text_limit: i32,
    /// Character count
    pub char_count: i32,
    /// Line count
    pub line_count: i32,
    /// Caret position
    pub caret_position: i32,
    /// Selection start
    pub selection_start: i32,
    /// Selection end (exclusive)
    pub selection_end: i32,
    /// Selected text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_text: Option<String>,
    /// Number of visible columns
    pub columns: i32,
    /// Echo character (for password fields)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo_char: Option<char>,
    /// Message text (placeholder)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Top visible line index
    pub top_index: i32,
    /// Whether tabs are inserted (vs navigation)
    pub tabs: bool,
}

/// StyledText properties (enhanced text widget)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StyledTextProps {
    /// Current text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Character count
    pub char_count: i32,
    /// Line count
    pub line_count: i32,
    /// Caret offset
    pub caret_offset: i32,
    /// Selection ranges (start, end pairs)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selection_ranges: Vec<(i32, i32)>,
    /// Top visible pixel
    pub top_pixel: i32,
    /// Top visible line index
    pub top_index: i32,
    /// Horizontal scroll offset
    pub horizontal_index: i32,
    /// Word wrap enabled
    pub word_wrap: bool,
    /// Block selection mode
    pub block_selection: bool,
    /// Number of style ranges
    pub style_range_count: i32,
}

/// Combo/CCombo properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComboProps {
    /// Current text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// All items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
    /// Item count
    pub item_count: i32,
    /// Selected index (-1 if none)
    pub selection_index: i32,
    /// Text limit
    pub text_limit: i32,
    /// Visible item count (dropdown height)
    pub visible_item_count: i32,
    /// List visible (dropdown open)
    pub list_visible: bool,
}

/// List widget properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListProps {
    /// All items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
    /// Item count
    pub item_count: i32,
    /// Selected indices
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selection_indices: Vec<i32>,
    /// Selected items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selection: Vec<String>,
    /// Selection count
    pub selection_count: i32,
    /// Top visible index
    pub top_index: i32,
    /// Focus index
    pub focus_index: i32,
    /// Item height
    pub item_height: i32,
}

/// Table properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableProps {
    /// Column count
    pub column_count: i32,
    /// Item (row) count
    pub item_count: i32,
    /// Selected indices
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selection_indices: Vec<i32>,
    /// Selection count
    pub selection_count: i32,
    /// Column headers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<TableColumnInfo>,
    /// Header visible
    pub header_visible: bool,
    /// Lines visible
    pub lines_visible: bool,
    /// Top visible index
    pub top_index: i32,
    /// Sort column index (-1 if none)
    pub sort_column_index: i32,
    /// Sort direction (1 = up, 2 = down, 0 = none)
    pub sort_direction: i32,
    /// Item height
    pub item_height: i32,
    /// Header height
    pub header_height: i32,
}

/// Table column information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableColumnInfo {
    /// Column text (header)
    pub text: String,
    /// Column width
    pub width: i32,
    /// Column index
    pub index: i32,
    /// Alignment
    pub alignment: i32,
    /// Resizable
    pub resizable: bool,
    /// Moveable
    pub moveable: bool,
    /// Tooltip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
}

/// Table item (row) properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableItemProps {
    /// Item index
    pub index: i32,
    /// Cell texts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub texts: Vec<String>,
    /// Checked state
    pub checked: bool,
    /// Grayed state
    pub grayed: bool,
    /// Has image
    pub has_image: bool,
}

/// Tree properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TreeProps {
    /// Column count (0 for simple tree)
    pub column_count: i32,
    /// Total item count (all nodes)
    pub item_count: i32,
    /// Root item count
    pub root_count: i32,
    /// Selected items count
    pub selection_count: i32,
    /// Column headers (if columns exist)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<TreeColumnInfo>,
    /// Header visible
    pub header_visible: bool,
    /// Lines visible
    pub lines_visible: bool,
    /// Sort column index
    pub sort_column_index: i32,
    /// Sort direction
    pub sort_direction: i32,
    /// Item height
    pub item_height: i32,
}

/// Tree column information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TreeColumnInfo {
    /// Column text (header)
    pub text: String,
    /// Column width
    pub width: i32,
    /// Column index
    pub index: i32,
    /// Alignment
    pub alignment: i32,
    /// Resizable
    pub resizable: bool,
    /// Moveable
    pub moveable: bool,
    /// Tooltip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
}

/// Tree item (node) properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TreeItemProps {
    /// Item index among siblings
    pub index: i32,
    /// Cell texts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub texts: Vec<String>,
    /// Primary text (first column)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Expanded state
    pub expanded: bool,
    /// Checked state
    pub checked: bool,
    /// Grayed state
    pub grayed: bool,
    /// Has children
    pub has_items: bool,
    /// Child count
    pub item_count: i32,
    /// Tree path (indices from root)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tree_path: Vec<i32>,
}

/// TabFolder properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TabFolderProps {
    /// Tab item count
    pub item_count: i32,
    /// Selected tab index
    pub selection_index: i32,
    /// Tab position (top, bottom)
    pub tab_position: i32,
}

/// TabItem properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TabItemProps {
    /// Tab text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Tab tooltip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    /// Tab index
    pub index: i32,
    /// Has image
    pub has_image: bool,
    /// Has control (content)
    pub has_control: bool,
}

/// Spinner properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpinnerProps {
    /// Current selection (value)
    pub selection: i32,
    /// Minimum value
    pub minimum: i32,
    /// Maximum value
    pub maximum: i32,
    /// Increment
    pub increment: i32,
    /// Page increment
    pub page_increment: i32,
    /// Digit count (decimal places)
    pub digits: i32,
    /// Text representation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Scale/Slider properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScaleProps {
    /// Current selection (value)
    pub selection: i32,
    /// Minimum value
    pub minimum: i32,
    /// Maximum value
    pub maximum: i32,
    /// Increment
    pub increment: i32,
    /// Page increment
    pub page_increment: i32,
    /// Orientation (horizontal/vertical)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
}

/// ProgressBar properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProgressBarProps {
    /// Current selection (value)
    pub selection: i32,
    /// Minimum value
    pub minimum: i32,
    /// Maximum value
    pub maximum: i32,
    /// State (normal, error, paused)
    pub state: i32,
    /// Indeterminate mode
    pub indeterminate: bool,
}

/// DateTime properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateTimeProps {
    /// Year
    pub year: i32,
    /// Month (0-11)
    pub month: i32,
    /// Day of month (1-31)
    pub day: i32,
    /// Hours (0-23)
    pub hours: i32,
    /// Minutes (0-59)
    pub minutes: i32,
    /// Seconds (0-59)
    pub seconds: i32,
    /// Mode (date, time, calendar)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// Menu properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MenuProps {
    /// Item count
    pub item_count: i32,
    /// Menu type (bar, popup, dropdown)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_type: Option<String>,
    /// Visible (for popup menus)
    pub visible: bool,
    /// Default item index
    pub default_item_index: i32,
}

/// MenuItem properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MenuItemProps {
    /// Menu item text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Menu item ID
    pub id: i32,
    /// Enabled state
    pub enabled: bool,
    /// Selection state (for check, radio)
    pub selection: bool,
    /// Accelerator key
    pub accelerator: i32,
    /// Item type (push, check, radio, separator, cascade)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    /// Index in parent menu
    pub index: i32,
}

/// ToolBar properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolBarProps {
    /// Item count
    pub item_count: i32,
    /// Row count (for wrapping toolbars)
    pub row_count: i32,
    /// Orientation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
}

/// ToolItem properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolItemProps {
    /// Tool item text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Tooltip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    /// Enabled state
    pub enabled: bool,
    /// Selection state (for check, radio)
    pub selection: bool,
    /// Item type (push, check, radio, separator, dropdown)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    /// Width
    pub width: i32,
    /// Index in toolbar
    pub index: i32,
}

/// Browser widget properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BrowserProps {
    /// Current URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Page text (HTML content)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Browser type (native, mozilla, webkit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_type: Option<String>,
    /// JavaScript enabled
    pub javascript_enabled: bool,
    /// Back navigation available
    pub back_enabled: bool,
    /// Forward navigation available
    pub forward_enabled: bool,
}

/// Link widget properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkProps {
    /// Link text (with markup)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Link IDs extracted from text
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_ids: Vec<String>,
}

/// ExpandBar properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExpandBarProps {
    /// Item count
    pub item_count: i32,
    /// Spacing between items
    pub spacing: i32,
}

/// Section (Eclipse Forms) properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SectionProps {
    /// Section title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Expanded state
    pub expanded: bool,
    /// Title bar foreground color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_bar_foreground: Option<String>,
}

/// Generic properties for unknown/custom widgets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GenericProps {
    /// Custom properties map
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, serde_json::Value>,
}

// =============================================================================
// Complete SWT Widget Representation
// =============================================================================

/// Complete representation of an SWT widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwtWidget {
    /// Unique widget identifier
    pub id: WidgetId,
    /// Widget type
    pub widget_type: SwtWidgetType,
    /// Full Java class name
    pub class_name: String,
    /// Simple class name (without package)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_name: Option<String>,
    /// Widget text (setText/getText)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Tooltip text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    /// Custom data (via setData)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub data: HashMap<String, serde_json::Value>,
    /// Widget bounds (geometry)
    pub bounds: WidgetBounds,
    /// Widget state flags
    pub state: SwtWidgetState,
    /// SWT style information
    pub style: SwtStyle,
    /// Widget-specific properties
    pub properties: SwtWidgetProperties,
    /// Child widgets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<SwtWidget>>,
    /// Parent widget ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<WidgetId>,
    /// Sibling index (position among siblings)
    pub sibling_index: u32,
    /// Accessible name (from accessibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessible_name: Option<String>,
    /// Accessible role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessible_role: Option<String>,
}

impl SwtWidget {
    /// Create a new SwtWidget with minimal required fields
    pub fn new(id: WidgetId, widget_type: SwtWidgetType, class_name: String) -> Self {
        let simple_name = class_name.rsplit('.').next().map(|s| s.to_string());

        Self {
            id,
            widget_type,
            class_name,
            simple_name,
            text: None,
            tooltip: None,
            data: HashMap::new(),
            bounds: WidgetBounds::default(),
            state: SwtWidgetState::default(),
            style: SwtStyle::default(),
            properties: SwtWidgetProperties::default(),
            children: None,
            parent_id: None,
            sibling_index: 0,
            accessible_name: None,
            accessible_role: None,
        }
    }

    /// Get the display name for this widget
    pub fn display_name(&self) -> &str {
        self.text
            .as_deref()
            .or(self.accessible_name.as_deref())
            .or(self.simple_name.as_deref())
            .unwrap_or(self.widget_type.class_name())
    }

    /// Check if this widget matches a type name
    pub fn matches_type(&self, type_name: &str) -> bool {
        self.simple_name.as_deref() == Some(type_name)
            || self.class_name == type_name
            || self.class_name.ends_with(&format!(".{}", type_name))
            || self.widget_type.class_name() == type_name
    }

    /// Check if this widget is a container
    pub fn is_container(&self) -> bool {
        self.widget_type.is_container()
    }

    /// Get a data value by key
    pub fn get_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    /// Get all descendants (recursive children)
    pub fn descendants(&self) -> Vec<&SwtWidget> {
        let mut result = Vec::new();
        self.collect_descendants(&mut result);
        result
    }

    fn collect_descendants<'a>(&'a self, result: &mut Vec<&'a SwtWidget>) {
        if let Some(ref children) = self.children {
            for child in children {
                result.push(child);
                child.collect_descendants(result);
            }
        }
    }

    /// Find the first descendant matching a predicate
    pub fn find_descendant<F>(&self, predicate: &F) -> Option<&SwtWidget>
    where
        F: Fn(&SwtWidget) -> bool,
    {
        if let Some(ref children) = self.children {
            for child in children {
                if predicate(child) {
                    return Some(child);
                }
                if let Some(found) = child.find_descendant(predicate) {
                    return Some(found);
                }
            }
        }
        None
    }

    /// Count total widgets in this subtree
    pub fn count_widgets(&self) -> usize {
        1 + self
            .children
            .as_ref()
            .map(|c| c.iter().map(|w| w.count_widgets()).sum())
            .unwrap_or(0)
    }

    /// Get the maximum depth of this subtree
    pub fn max_depth(&self) -> usize {
        self.children
            .as_ref()
            .map(|children| {
                if children.is_empty() {
                    0
                } else {
                    1 + children.iter().map(|c| c.max_depth()).max().unwrap_or(0)
                }
            })
            .unwrap_or(0)
    }
}

// =============================================================================
// Widget Tree
// =============================================================================

/// Complete SWT widget tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwtWidgetTree {
    /// Root widgets (typically shells)
    pub roots: Vec<SwtWidget>,
    /// Tree metadata
    pub metadata: SwtTreeMetadata,
    /// Tree statistics
    pub statistics: SwtTreeStatistics,
}

impl SwtWidgetTree {
    /// Create an empty tree
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            metadata: SwtTreeMetadata::default(),
            statistics: SwtTreeStatistics::default(),
        }
    }

    /// Iterator over all widgets
    pub fn iter(&self) -> SwtWidgetTreeIter<'_> {
        SwtWidgetTreeIter {
            stack: self.roots.iter().collect(),
        }
    }

    /// Find a widget by handle
    pub fn find_by_handle(&self, handle: i64) -> Option<&SwtWidget> {
        self.iter().find(|w| w.id.handle == handle)
    }

    /// Find a widget by tree path
    pub fn find_by_path(&self, path: &str) -> Option<&SwtWidget> {
        self.iter().find(|w| w.id.tree_path == path)
    }

    /// Find widgets by type
    pub fn find_by_type(&self, widget_type: SwtWidgetType) -> Vec<&SwtWidget> {
        self.iter().filter(|w| w.widget_type == widget_type).collect()
    }
}

impl Default for SwtWidgetTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator over SWT widget tree
pub struct SwtWidgetTreeIter<'a> {
    stack: Vec<&'a SwtWidget>,
}

impl<'a> Iterator for SwtWidgetTreeIter<'a> {
    type Item = &'a SwtWidget;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(widget) = self.stack.pop() {
            if let Some(ref children) = widget.children {
                for child in children.iter().rev() {
                    self.stack.push(child);
                }
            }
            Some(widget)
        } else {
            None
        }
    }
}

/// Tree metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SwtTreeMetadata {
    /// Application name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// Display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// SWT version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swt_version: Option<String>,
    /// Platform (win32, gtk, cocoa)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// Capture timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_time: Option<String>,
}

/// Tree statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SwtTreeStatistics {
    /// Total widget count
    pub total_widgets: usize,
    /// Maximum tree depth
    pub max_depth: usize,
    /// Shell count
    pub shell_count: usize,
    /// Widget counts by type
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub widget_counts: HashMap<String, usize>,
}

// =============================================================================
// Filter and Query
// =============================================================================

/// Filter builder for widget queries
#[derive(Debug, Clone, Default)]
pub struct SwtFilterBuilder {
    visible_only: bool,
    enabled_only: bool,
    not_disposed: bool,
    max_depth: Option<u32>,
    widget_types: Vec<SwtWidgetType>,
    style_bits: Option<i32>,
}

impl SwtFilterBuilder {
    /// Create a new filter builder
    pub fn new() -> Self {
        Self {
            not_disposed: true,
            ..Self::default()
        }
    }

    /// Only match visible widgets
    pub fn visible_only(mut self) -> Self {
        self.visible_only = true;
        self
    }

    /// Only match enabled widgets
    pub fn enabled_only(mut self) -> Self {
        self.enabled_only = true;
        self
    }

    /// Include disposed widgets
    pub fn include_disposed(mut self) -> Self {
        self.not_disposed = false;
        self
    }

    /// Limit search depth
    pub fn max_depth(mut self, depth: u32) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Filter by widget type
    pub fn with_type(mut self, widget_type: SwtWidgetType) -> Self {
        self.widget_types.push(widget_type);
        self
    }

    /// Filter by style bits
    pub fn with_style(mut self, style_bits: i32) -> Self {
        self.style_bits = Some(style_bits);
        self
    }

    /// Build the filter specification
    pub fn build(self) -> SwtFilterSpec {
        SwtFilterSpec {
            visible_only: self.visible_only,
            enabled_only: self.enabled_only,
            not_disposed: self.not_disposed,
            max_depth: self.max_depth,
            widget_types: self.widget_types,
            style_bits: self.style_bits,
        }
    }
}

/// Filter specification for widget queries
#[derive(Debug, Clone, Default)]
pub struct SwtFilterSpec {
    pub visible_only: bool,
    pub enabled_only: bool,
    pub not_disposed: bool,
    pub max_depth: Option<u32>,
    pub widget_types: Vec<SwtWidgetType>,
    pub style_bits: Option<i32>,
}

impl SwtFilterSpec {
    /// Check if a widget matches this filter
    pub fn matches(&self, widget: &SwtWidget) -> bool {
        if self.visible_only && !widget.state.visible {
            return false;
        }
        if self.enabled_only && !widget.state.enabled {
            return false;
        }
        if self.not_disposed && widget.state.disposed {
            return false;
        }
        if let Some(max_depth) = self.max_depth {
            if widget.id.depth > max_depth {
                return false;
            }
        }
        if !self.widget_types.is_empty() {
            if !self.widget_types.contains(&widget.widget_type) {
                return false;
            }
        }
        if let Some(style_bits) = self.style_bits {
            if (widget.style.style_bits & style_bits) != style_bits {
                return false;
            }
        }
        true
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_id_creation() {
        let id = WidgetId::new(12345, "0.1.2".to_string(), 3);
        assert_eq!(id.handle, 12345);
        assert_eq!(id.tree_path, "0.1.2");
        assert_eq!(id.depth, 3);
        assert!(id.data_key.is_none());

        let id_with_key = WidgetId::with_data_key(
            12345,
            "testKey".to_string(),
            "0.1.2".to_string(),
            3,
        );
        assert_eq!(id_with_key.data_key, Some("testKey".to_string()));
    }

    #[test]
    fn test_widget_type_from_class_name() {
        assert_eq!(
            SwtWidgetType::from_class_name("org.eclipse.swt.widgets.Shell"),
            SwtWidgetType::Shell
        );
        assert_eq!(
            SwtWidgetType::from_class_name("org.eclipse.swt.widgets.Button"),
            SwtWidgetType::Button
        );
        assert_eq!(
            SwtWidgetType::from_class_name("Button"),
            SwtWidgetType::Button
        );
        assert_eq!(
            SwtWidgetType::from_class_name("org.eclipse.swt.custom.StyledText"),
            SwtWidgetType::StyledText
        );
        assert_eq!(
            SwtWidgetType::from_class_name("CustomWidget"),
            SwtWidgetType::Unknown
        );
    }

    #[test]
    fn test_widget_type_categories() {
        assert!(SwtWidgetType::Shell.is_container());
        assert!(SwtWidgetType::Composite.is_container());
        assert!(!SwtWidgetType::Button.is_container());

        assert!(SwtWidgetType::Text.is_text_input());
        assert!(SwtWidgetType::Combo.is_text_input());
        assert!(!SwtWidgetType::Label.is_text_input());

        assert!(SwtWidgetType::Button.is_button());
        assert!(SwtWidgetType::Link.is_button());
        assert!(!SwtWidgetType::Text.is_button());

        assert!(SwtWidgetType::TableItem.is_item());
        assert!(SwtWidgetType::TreeItem.is_item());
        assert!(!SwtWidgetType::Table.is_item());
    }

    #[test]
    fn test_widget_bounds() {
        let bounds = WidgetBounds::new(10, 20, 100, 50);

        assert!(bounds.contains(50, 40));
        assert!(bounds.contains(10, 20));
        assert!(!bounds.contains(9, 40));
        assert!(!bounds.contains(110, 40));

        assert_eq!(bounds.center(), (60, 45));

        let other = WidgetBounds::new(50, 30, 100, 50);
        assert!(bounds.intersects(&other));

        let non_intersecting = WidgetBounds::new(200, 200, 50, 50);
        assert!(!bounds.intersects(&non_intersecting));
    }

    #[test]
    fn test_swt_style() {
        let style = SwtStyle::new(SwtStyleBit::Border.value() | SwtStyleBit::Push.value());

        assert!(style.has_style(SwtStyleBit::Border));
        assert!(style.has_style(SwtStyleBit::Push));
        assert!(!style.has_style(SwtStyleBit::Check));

        assert!(style.has_any_style(&[SwtStyleBit::Border, SwtStyleBit::Multi]));
        assert!(!style.has_any_style(&[SwtStyleBit::Check, SwtStyleBit::Radio]));
    }

    #[test]
    fn test_swt_widget_creation() {
        let id = WidgetId::new(1, "0".to_string(), 0);
        let widget = SwtWidget::new(
            id.clone(),
            SwtWidgetType::Button,
            "org.eclipse.swt.widgets.Button".to_string(),
        );

        assert_eq!(widget.widget_type, SwtWidgetType::Button);
        assert_eq!(widget.simple_name, Some("Button".to_string()));
        assert!(widget.children.is_none());
        assert!(widget.state.visible);
        assert!(widget.state.enabled);
    }

    #[test]
    fn test_widget_matches_type() {
        let id = WidgetId::new(1, "0".to_string(), 0);
        let widget = SwtWidget::new(
            id,
            SwtWidgetType::Button,
            "org.eclipse.swt.widgets.Button".to_string(),
        );

        assert!(widget.matches_type("Button"));
        assert!(widget.matches_type("org.eclipse.swt.widgets.Button"));
        assert!(!widget.matches_type("Text"));
    }

    #[test]
    fn test_filter_builder() {
        let filter = SwtFilterBuilder::new()
            .visible_only()
            .enabled_only()
            .with_type(SwtWidgetType::Button)
            .max_depth(5)
            .build();

        assert!(filter.visible_only);
        assert!(filter.enabled_only);
        assert!(filter.not_disposed);
        assert_eq!(filter.max_depth, Some(5));
        assert!(filter.widget_types.contains(&SwtWidgetType::Button));
    }

    #[test]
    fn test_filter_matches() {
        let filter = SwtFilterBuilder::new()
            .visible_only()
            .enabled_only()
            .build();

        let id = WidgetId::new(1, "0".to_string(), 0);
        let mut widget = SwtWidget::new(
            id,
            SwtWidgetType::Button,
            "Button".to_string(),
        );

        assert!(filter.matches(&widget));

        widget.state.visible = false;
        assert!(!filter.matches(&widget));

        widget.state.visible = true;
        widget.state.enabled = false;
        assert!(!filter.matches(&widget));
    }

    #[test]
    fn test_widget_tree_iteration() {
        let mut tree = SwtWidgetTree::new();

        let shell_id = WidgetId::new(1, "0".to_string(), 0);
        let mut shell = SwtWidget::new(
            shell_id,
            SwtWidgetType::Shell,
            "Shell".to_string(),
        );

        let btn_id = WidgetId::new(2, "0.0".to_string(), 1);
        let button = SwtWidget::new(
            btn_id,
            SwtWidgetType::Button,
            "Button".to_string(),
        );

        shell.children = Some(vec![button]);
        tree.roots.push(shell);

        let widgets: Vec<_> = tree.iter().collect();
        assert_eq!(widgets.len(), 2);
        assert_eq!(widgets[0].widget_type, SwtWidgetType::Shell);
        assert_eq!(widgets[1].widget_type, SwtWidgetType::Button);
    }

    #[test]
    fn test_serialization() {
        let id = WidgetId::new(1, "0".to_string(), 0);
        let widget = SwtWidget::new(
            id,
            SwtWidgetType::Button,
            "org.eclipse.swt.widgets.Button".to_string(),
        );

        let json = serde_json::to_string(&widget).expect("Serialization failed");
        assert!(json.contains("Button"));

        let deserialized: SwtWidget = serde_json::from_str(&json)
            .expect("Deserialization failed");
        assert_eq!(deserialized.widget_type, SwtWidgetType::Button);
    }
}
