//! UI element and tree models for Swing components

pub mod component;
pub mod element;
pub mod tree;

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
