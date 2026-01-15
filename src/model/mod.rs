//! UI element and tree models for Swing, SWT, and RCP components

pub mod component;
pub mod element;
pub mod rcp;
pub mod tree;
pub mod widget;

// Re-export component types (primary model for swing_library)
pub use component::{
    AccessibilityInfo, Bounds, ButtonProperties, ComboBoxProperties, ComponentGeometry,
    ComponentId, ComponentIdentity, ComponentProperties, ComponentState, ComponentType,
    FilterBuilder, FilterSpecification, GenericProperties, ListProperties, MenuProperties,
    ProgressBarProperties, ScrollPaneProperties, SliderProperties, SpinnerProperties,
    SplitPaneProperties, SwingBaseType, TabbedPaneProperties, TableProperties,
    TextFieldProperties, TraversalMetadata, TreeMetadata, TreeProperties, TreeStatistics,
    UIComponent, UITree, UITreeIter,
};

// Re-export element types (for lower-level element access)
pub use element::{
    AccessibleInfo, ElementState, PropertyValue, Rectangle, SwingComponentType, UIElement,
};

// Re-export tree filter types (renamed to avoid conflict)
pub use tree::TreeFilter;

// Re-export SWT widget types (for SWT/RCP support)
pub use widget::{
    // Widget identification
    WidgetId, SwtWidgetType, SwtWidget, SwtWidgetTree, SwtWidgetTreeIter,
    // Widget bounds and state
    WidgetBounds, SwtWidgetState, SwtStyle, SwtStyleBit,
    // Tree metadata
    SwtTreeMetadata, SwtTreeStatistics,
    // Filter/query
    SwtFilterBuilder, SwtFilterSpec,
    // Widget-specific properties
    SwtWidgetProperties, ShellProps, ButtonProps, TextProps, StyledTextProps,
    ComboProps, ListProps, TableProps, TableColumnInfo, TableItemProps,
    TreeProps, TreeColumnInfo, TreeItemProps, TabFolderProps, TabItemProps,
    SpinnerProps, ScaleProps, ProgressBarProps, DateTimeProps,
    MenuProps, MenuItemProps, ToolBarProps, ToolItemProps,
    BrowserProps, LinkProps, ExpandBarProps, SectionProps, GenericProps,
};

// Re-export RCP types (for Eclipse RCP support)
pub use rcp::{
    // Layout types
    LayoutRelationship, ViewPosition, ViewLayoutInfo, PerspectiveLayout,
    // Perspective types
    PerspectiveDescriptor, Perspective,
    // Editor types
    EditorInputType, EditorInput, Editor,
    // View types
    View,
    // Command types
    EclipseCommand, CommandParameter,
    // Window and workbench
    CoolbarItem, MenuItem as RcpMenuItem, Menu as RcpMenu,
    WorkbenchWindow, WindowBounds, Workbench, WorkbenchState,
    // ID type
    RcpId,
};
