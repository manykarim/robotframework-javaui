# UI Tree Output System Architecture

## Overview

This document specifies the complete architecture for the UI Tree traversal and output system in the Robot Framework Swing library. The system is designed for high performance using Rust with Python bindings via PyO3.

## System Architecture Diagram

```
+-----------------------------------------------------------------------------------+
|                           Robot Framework Test Suite                               |
+-----------------------------------------------------------------------------------+
                                        |
                                        v
+-----------------------------------------------------------------------------------+
|                        Python Keywords Layer (robotframework-swing)                |
|  +-------------+  +------------------+  +-------------+  +--------------------+   |
|  | Get UI Tree |  | Get Component    |  | Log UI Tree |  | Export UI Tree     |   |
|  |             |  | Properties       |  |             |  | To File            |   |
|  +-------------+  +------------------+  +-------------+  +--------------------+   |
|  +-------------------------+  +-------------------------------------------+       |
|  | Find Components By      |  | Filter Components                         |       |
|  | Filter                  |  |                                           |       |
|  +-------------------------+  +-------------------------------------------+       |
+-----------------------------------------------------------------------------------+
                                        |
                                        v
+-----------------------------------------------------------------------------------+
|                           PyO3 Bindings Layer                                      |
|  +---------------+  +---------------+  +---------------+  +------------------+    |
|  | UITreeBuilder |  | TreeFormatter |  | FilterEngine  |  | CacheManager     |    |
|  +---------------+  +---------------+  +---------------+  +------------------+    |
+-----------------------------------------------------------------------------------+
                                        |
                                        v
+-----------------------------------------------------------------------------------+
|                              Rust Core Engine                                      |
|  +------------------+  +-------------------+  +------------------+                 |
|  | TreeTraverser    |  | ComponentParser   |  | PropertyExtractor|                 |
|  | (async/parallel) |  |                   |  |                  |                 |
|  +------------------+  +-------------------+  +------------------+                 |
|  +------------------+  +-------------------+  +------------------+                 |
|  | OutputFormatter  |  | FilterMatcher     |  | TreeCache        |                 |
|  | (JSON/XML/Text)  |  | (CSS/XPath-like)  |  | (LRU)            |                 |
|  +------------------+  +-------------------+  +------------------+                 |
+-----------------------------------------------------------------------------------+
                                        |
                                        v
+-----------------------------------------------------------------------------------+
|                           JNI Bridge (j4rs/jni-rs)                                 |
|  +------------------+  +-------------------+  +------------------+                 |
|  | JVM Connection   |  | Swing Introspect  |  | Property Access  |                 |
|  +------------------+  +-------------------+  +------------------+                 |
+-----------------------------------------------------------------------------------+
                                        |
                                        v
+-----------------------------------------------------------------------------------+
|                           Java Swing Application (AUT)                             |
+-----------------------------------------------------------------------------------+
```

## 1. UI Tree Data Model

### 1.1 Core Component Structure

```rust
/// Represents a single UI component in the tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIComponent {
    /// Unique identifier for this component instance
    pub id: ComponentId,

    /// Component classification
    pub component_type: ComponentType,

    /// Display and identification properties
    pub identity: ComponentIdentity,

    /// Spatial properties
    pub geometry: ComponentGeometry,

    /// State properties
    pub state: ComponentState,

    /// Component-specific properties
    pub properties: ComponentProperties,

    /// Accessibility information
    pub accessibility: AccessibilityInfo,

    /// Child components (lazy-loaded for deep trees)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<UIComponent>>,

    /// Parent reference (not serialized to avoid cycles)
    #[serde(skip)]
    pub parent_id: Option<ComponentId>,

    /// Metadata for traversal
    pub metadata: TraversalMetadata,
}

/// Unique component identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId {
    /// Java object hash code
    pub hash_code: i64,
    /// Path from root (e.g., "0.2.1.0")
    pub tree_path: String,
    /// Depth level in tree
    pub depth: u32,
}

/// Component type classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentType {
    /// Full Java class name (e.g., "javax.swing.JButton")
    pub class_name: String,
    /// Simple class name (e.g., "JButton")
    pub simple_name: String,
    /// Base Swing component type
    pub base_type: SwingBaseType,
    /// All implemented interfaces
    pub interfaces: Vec<String>,
    /// Superclass hierarchy
    pub class_hierarchy: Vec<String>,
}

/// Base Swing component types for fast matching
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SwingBaseType {
    // Containers
    Frame,
    Dialog,
    Panel,
    ScrollPane,
    SplitPane,
    TabbedPane,
    ToolBar,
    InternalFrame,
    LayeredPane,
    RootPane,

    // Basic Controls
    Button,
    ToggleButton,
    CheckBox,
    RadioButton,
    Label,
    TextField,
    TextArea,
    PasswordField,
    EditorPane,
    TextPane,
    FormattedTextField,

    // Selection Controls
    ComboBox,
    List,
    Spinner,
    Slider,

    // Complex Controls
    Table,
    Tree,
    ProgressBar,

    // Menus
    MenuBar,
    Menu,
    MenuItem,
    PopupMenu,
    CheckBoxMenuItem,
    RadioButtonMenuItem,

    // Other
    Separator,
    ToolTip,
    ColorChooser,
    FileChooser,
    OptionPane,

    // Unknown/Custom
    Custom,
}

/// Identity and naming properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentIdentity {
    /// Component name (setName())
    pub name: Option<String>,
    /// Text content (getText() if applicable)
    pub text: Option<String>,
    /// Title (for frames, dialogs, titled borders)
    pub title: Option<String>,
    /// Tooltip text
    pub tooltip: Option<String>,
    /// Action command (for buttons)
    pub action_command: Option<String>,
    /// Label (associated label text)
    pub label_text: Option<String>,
    /// Internal name (custom property often used for test automation)
    pub internal_name: Option<String>,
}

/// Spatial/geometric properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentGeometry {
    /// Position relative to parent
    pub bounds: Bounds,
    /// Position on screen
    pub screen_location: Option<Point>,
    /// Preferred size
    pub preferred_size: Dimension,
    /// Minimum size
    pub minimum_size: Dimension,
    /// Maximum size
    pub maximum_size: Dimension,
    /// Alignment
    pub alignment: Alignment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alignment {
    pub x: f32,
    pub y: f32,
}

/// State properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentState {
    /// Is component visible
    pub visible: bool,
    /// Is component showing on screen
    pub showing: bool,
    /// Is component enabled
    pub enabled: bool,
    /// Is component focusable
    pub focusable: bool,
    /// Has focus
    pub focused: bool,
    /// Is component opaque
    pub opaque: bool,
    /// Is component valid (layout calculated)
    pub valid: bool,
    /// Is double buffered
    pub double_buffered: bool,
    /// Is selected (for toggle buttons, list items, etc.)
    pub selected: Option<bool>,
    /// Is editable (for text components)
    pub editable: Option<bool>,
}

/// Component-specific properties (type-dependent)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonProperties {
    pub mnemonic: Option<char>,
    pub icon_path: Option<String>,
    pub pressed: bool,
    pub rollover: bool,
    pub content_area_filled: bool,
    pub border_painted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextFieldProperties {
    pub columns: i32,
    pub caret_position: i32,
    pub selection_start: i32,
    pub selection_end: i32,
    pub horizontal_alignment: i32,
    pub echo_char: Option<char>, // For password fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboBoxProperties {
    pub item_count: i32,
    pub selected_index: i32,
    pub selected_item: Option<String>,
    pub items: Vec<String>,
    pub is_popup_visible: bool,
    pub maximum_row_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProperties {
    pub item_count: i32,
    pub selected_indices: Vec<i32>,
    pub selected_values: Vec<String>,
    pub visible_row_count: i32,
    pub selection_mode: SelectionMode,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SelectionMode {
    Single,
    SingleInterval,
    MultipleInterval,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableProperties {
    pub row_count: i32,
    pub column_count: i32,
    pub selected_rows: Vec<i32>,
    pub selected_columns: Vec<i32>,
    pub column_names: Vec<String>,
    pub auto_resize_mode: i32,
    pub row_height: i32,
    pub show_grid: bool,
    pub cell_selection_enabled: bool,
    pub row_selection_allowed: bool,
    pub column_selection_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeProperties {
    pub row_count: i32,
    pub selection_count: i32,
    pub selected_paths: Vec<String>,
    pub expanded_paths: Vec<String>,
    pub root_visible: bool,
    pub shows_root_handles: bool,
    pub row_height: i32,
    pub large_model: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliderProperties {
    pub value: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub extent: i32,
    pub orientation: Orientation,
    pub inverted: bool,
    pub paint_ticks: bool,
    pub paint_labels: bool,
    pub snap_to_ticks: bool,
    pub major_tick_spacing: i32,
    pub minor_tick_spacing: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressBarProperties {
    pub value: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub percent_complete: f64,
    pub indeterminate: bool,
    pub string_painted: bool,
    pub progress_string: Option<String>,
    pub orientation: Orientation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabbedPaneProperties {
    pub tab_count: i32,
    pub selected_index: i32,
    pub tab_titles: Vec<String>,
    pub tab_placement: TabPlacement,
    pub tab_layout_policy: TabLayoutPolicy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TabPlacement {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TabLayoutPolicy {
    Wrap,
    Scroll,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinnerProperties {
    pub value: String,
    pub next_value: Option<String>,
    pub previous_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuProperties {
    pub item_count: i32,
    pub popup_visible: bool,
    pub delay: i32,
    pub accelerator: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollPaneProperties {
    pub horizontal_scrollbar_policy: ScrollBarPolicy,
    pub vertical_scrollbar_policy: ScrollBarPolicy,
    pub viewport_bounds: Bounds,
    pub horizontal_scroll_value: i32,
    pub vertical_scroll_value: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ScrollBarPolicy {
    Always,
    AsNeeded,
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitPaneProperties {
    pub divider_location: i32,
    pub divider_size: i32,
    pub orientation: Orientation,
    pub continuous_layout: bool,
    pub one_touch_expandable: bool,
    pub resize_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericProperties {
    /// Additional properties as key-value pairs
    pub custom_properties: HashMap<String, PropertyValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
    Array(Vec<PropertyValue>),
    Object(HashMap<String, PropertyValue>),
}

/// Accessibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityInfo {
    pub accessible_name: Option<String>,
    pub accessible_description: Option<String>,
    pub accessible_role: Option<String>,
    pub accessible_state: Vec<String>,
}

/// Traversal metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalMetadata {
    /// Index among siblings
    pub sibling_index: u32,
    /// Total sibling count
    pub sibling_count: u32,
    /// Number of children
    pub child_count: u32,
    /// Is leaf node
    pub is_leaf: bool,
    /// Traversal timestamp
    pub captured_at: String,
    /// Whether children are loaded
    pub children_loaded: bool,
}
```

### 1.2 UI Tree Container

```rust
/// Represents the complete UI tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UITree {
    /// Root components (usually frames/dialogs)
    pub roots: Vec<UIComponent>,

    /// Tree metadata
    pub metadata: TreeMetadata,

    /// Statistics about the tree
    pub statistics: TreeStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeMetadata {
    /// Application name
    pub application_name: Option<String>,
    /// Capture timestamp
    pub captured_at: String,
    /// Library version
    pub library_version: String,
    /// JVM version
    pub jvm_version: String,
    /// Filter applied (if any)
    pub filter_applied: Option<FilterSpecification>,
    /// Max depth captured
    pub max_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeStatistics {
    /// Total component count
    pub total_components: u32,
    /// Components by type
    pub components_by_type: HashMap<String, u32>,
    /// Maximum depth
    pub max_depth: u32,
    /// Visible components
    pub visible_count: u32,
    /// Enabled components
    pub enabled_count: u32,
    /// Capture duration in milliseconds
    pub capture_duration_ms: u64,
}
```

## 2. Output Formats

### 2.1 JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://robotframework-swing.io/schemas/ui-tree.json",
  "title": "UI Tree Schema",
  "description": "Schema for Robot Framework Swing UI Tree output",
  "type": "object",
  "required": ["roots", "metadata", "statistics"],
  "properties": {
    "roots": {
      "type": "array",
      "description": "Root components of the UI tree",
      "items": { "$ref": "#/definitions/UIComponent" }
    },
    "metadata": { "$ref": "#/definitions/TreeMetadata" },
    "statistics": { "$ref": "#/definitions/TreeStatistics" }
  },
  "definitions": {
    "UIComponent": {
      "type": "object",
      "required": ["id", "component_type", "identity", "geometry", "state", "metadata"],
      "properties": {
        "id": { "$ref": "#/definitions/ComponentId" },
        "component_type": { "$ref": "#/definitions/ComponentType" },
        "identity": { "$ref": "#/definitions/ComponentIdentity" },
        "geometry": { "$ref": "#/definitions/ComponentGeometry" },
        "state": { "$ref": "#/definitions/ComponentState" },
        "properties": { "$ref": "#/definitions/ComponentProperties" },
        "accessibility": { "$ref": "#/definitions/AccessibilityInfo" },
        "children": {
          "type": "array",
          "items": { "$ref": "#/definitions/UIComponent" }
        },
        "metadata": { "$ref": "#/definitions/TraversalMetadata" }
      }
    },
    "ComponentId": {
      "type": "object",
      "required": ["hash_code", "tree_path", "depth"],
      "properties": {
        "hash_code": { "type": "integer" },
        "tree_path": { "type": "string", "pattern": "^[0-9]+(\\.[0-9]+)*$" },
        "depth": { "type": "integer", "minimum": 0 }
      }
    },
    "ComponentType": {
      "type": "object",
      "required": ["class_name", "simple_name", "base_type"],
      "properties": {
        "class_name": { "type": "string" },
        "simple_name": { "type": "string" },
        "base_type": {
          "type": "string",
          "enum": ["frame", "dialog", "panel", "scroll_pane", "split_pane",
                   "tabbed_pane", "tool_bar", "internal_frame", "layered_pane",
                   "root_pane", "button", "toggle_button", "check_box", "radio_button",
                   "label", "text_field", "text_area", "password_field", "editor_pane",
                   "text_pane", "formatted_text_field", "combo_box", "list", "spinner",
                   "slider", "table", "tree", "progress_bar", "menu_bar", "menu",
                   "menu_item", "popup_menu", "check_box_menu_item", "radio_button_menu_item",
                   "separator", "tool_tip", "color_chooser", "file_chooser",
                   "option_pane", "custom"]
        },
        "interfaces": {
          "type": "array",
          "items": { "type": "string" }
        },
        "class_hierarchy": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "ComponentIdentity": {
      "type": "object",
      "properties": {
        "name": { "type": ["string", "null"] },
        "text": { "type": ["string", "null"] },
        "title": { "type": ["string", "null"] },
        "tooltip": { "type": ["string", "null"] },
        "action_command": { "type": ["string", "null"] },
        "label_text": { "type": ["string", "null"] },
        "internal_name": { "type": ["string", "null"] }
      }
    },
    "ComponentGeometry": {
      "type": "object",
      "required": ["bounds", "preferred_size", "minimum_size", "maximum_size"],
      "properties": {
        "bounds": { "$ref": "#/definitions/Bounds" },
        "screen_location": { "$ref": "#/definitions/Point" },
        "preferred_size": { "$ref": "#/definitions/Dimension" },
        "minimum_size": { "$ref": "#/definitions/Dimension" },
        "maximum_size": { "$ref": "#/definitions/Dimension" },
        "alignment": { "$ref": "#/definitions/Alignment" }
      }
    },
    "Bounds": {
      "type": "object",
      "required": ["x", "y", "width", "height"],
      "properties": {
        "x": { "type": "integer" },
        "y": { "type": "integer" },
        "width": { "type": "integer" },
        "height": { "type": "integer" }
      }
    },
    "Point": {
      "type": "object",
      "required": ["x", "y"],
      "properties": {
        "x": { "type": "integer" },
        "y": { "type": "integer" }
      }
    },
    "Dimension": {
      "type": "object",
      "required": ["width", "height"],
      "properties": {
        "width": { "type": "integer" },
        "height": { "type": "integer" }
      }
    },
    "Alignment": {
      "type": "object",
      "required": ["x", "y"],
      "properties": {
        "x": { "type": "number", "minimum": 0, "maximum": 1 },
        "y": { "type": "number", "minimum": 0, "maximum": 1 }
      }
    },
    "ComponentState": {
      "type": "object",
      "required": ["visible", "showing", "enabled", "focusable", "focused"],
      "properties": {
        "visible": { "type": "boolean" },
        "showing": { "type": "boolean" },
        "enabled": { "type": "boolean" },
        "focusable": { "type": "boolean" },
        "focused": { "type": "boolean" },
        "opaque": { "type": "boolean" },
        "valid": { "type": "boolean" },
        "double_buffered": { "type": "boolean" },
        "selected": { "type": ["boolean", "null"] },
        "editable": { "type": ["boolean", "null"] }
      }
    },
    "ComponentProperties": {
      "type": "object",
      "required": ["type"],
      "properties": {
        "type": {
          "type": "string",
          "enum": ["Button", "TextField", "ComboBox", "List", "Table",
                   "Tree", "Slider", "ProgressBar", "TabbedPane", "Spinner",
                   "Menu", "ScrollPane", "SplitPane", "Generic"]
        },
        "data": { "type": "object" }
      }
    },
    "AccessibilityInfo": {
      "type": "object",
      "properties": {
        "accessible_name": { "type": ["string", "null"] },
        "accessible_description": { "type": ["string", "null"] },
        "accessible_role": { "type": ["string", "null"] },
        "accessible_state": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "TraversalMetadata": {
      "type": "object",
      "required": ["sibling_index", "sibling_count", "child_count", "is_leaf"],
      "properties": {
        "sibling_index": { "type": "integer", "minimum": 0 },
        "sibling_count": { "type": "integer", "minimum": 1 },
        "child_count": { "type": "integer", "minimum": 0 },
        "is_leaf": { "type": "boolean" },
        "captured_at": { "type": "string", "format": "date-time" },
        "children_loaded": { "type": "boolean" }
      }
    },
    "TreeMetadata": {
      "type": "object",
      "required": ["captured_at", "library_version", "max_depth"],
      "properties": {
        "application_name": { "type": ["string", "null"] },
        "captured_at": { "type": "string", "format": "date-time" },
        "library_version": { "type": "string" },
        "jvm_version": { "type": "string" },
        "filter_applied": { "$ref": "#/definitions/FilterSpecification" },
        "max_depth": { "type": "integer", "minimum": 0 }
      }
    },
    "TreeStatistics": {
      "type": "object",
      "required": ["total_components", "max_depth"],
      "properties": {
        "total_components": { "type": "integer", "minimum": 0 },
        "components_by_type": {
          "type": "object",
          "additionalProperties": { "type": "integer" }
        },
        "max_depth": { "type": "integer", "minimum": 0 },
        "visible_count": { "type": "integer", "minimum": 0 },
        "enabled_count": { "type": "integer", "minimum": 0 },
        "capture_duration_ms": { "type": "integer", "minimum": 0 }
      }
    },
    "FilterSpecification": {
      "type": "object",
      "properties": {
        "component_types": {
          "type": "array",
          "items": { "type": "string" }
        },
        "properties": { "type": "object" },
        "max_depth": { "type": "integer", "minimum": 0 },
        "visible_only": { "type": "boolean" },
        "enabled_only": { "type": "boolean" },
        "name_pattern": { "type": "string" }
      }
    }
  }
}
```

### 2.2 XML Schema (XSD)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           targetNamespace="https://robotframework-swing.io/schemas/ui-tree"
           xmlns:uit="https://robotframework-swing.io/schemas/ui-tree"
           elementFormDefault="qualified">

  <xs:element name="UITree" type="uit:UITreeType"/>

  <xs:complexType name="UITreeType">
    <xs:sequence>
      <xs:element name="roots" type="uit:ComponentListType"/>
      <xs:element name="metadata" type="uit:TreeMetadataType"/>
      <xs:element name="statistics" type="uit:TreeStatisticsType"/>
    </xs:sequence>
    <xs:attribute name="version" type="xs:string" use="required"/>
  </xs:complexType>

  <xs:complexType name="ComponentListType">
    <xs:sequence>
      <xs:element name="component" type="uit:UIComponentType"
                  minOccurs="0" maxOccurs="unbounded"/>
    </xs:sequence>
  </xs:complexType>

  <xs:complexType name="UIComponentType">
    <xs:sequence>
      <xs:element name="id" type="uit:ComponentIdType"/>
      <xs:element name="componentType" type="uit:ComponentTypeType"/>
      <xs:element name="identity" type="uit:ComponentIdentityType"/>
      <xs:element name="geometry" type="uit:ComponentGeometryType"/>
      <xs:element name="state" type="uit:ComponentStateType"/>
      <xs:element name="properties" type="uit:ComponentPropertiesType" minOccurs="0"/>
      <xs:element name="accessibility" type="uit:AccessibilityInfoType" minOccurs="0"/>
      <xs:element name="children" type="uit:ComponentListType" minOccurs="0"/>
      <xs:element name="metadata" type="uit:TraversalMetadataType"/>
    </xs:sequence>
  </xs:complexType>

  <xs:complexType name="ComponentIdType">
    <xs:sequence>
      <xs:element name="hashCode" type="xs:long"/>
      <xs:element name="treePath" type="xs:string"/>
      <xs:element name="depth" type="xs:unsignedInt"/>
    </xs:sequence>
  </xs:complexType>

  <xs:complexType name="ComponentTypeType">
    <xs:sequence>
      <xs:element name="className" type="xs:string"/>
      <xs:element name="simpleName" type="xs:string"/>
      <xs:element name="baseType" type="uit:SwingBaseTypeType"/>
      <xs:element name="interfaces" type="uit:StringListType" minOccurs="0"/>
      <xs:element name="classHierarchy" type="uit:StringListType" minOccurs="0"/>
    </xs:sequence>
  </xs:complexType>

  <xs:simpleType name="SwingBaseTypeType">
    <xs:restriction base="xs:string">
      <xs:enumeration value="frame"/>
      <xs:enumeration value="dialog"/>
      <xs:enumeration value="panel"/>
      <xs:enumeration value="button"/>
      <xs:enumeration value="text_field"/>
      <xs:enumeration value="combo_box"/>
      <xs:enumeration value="table"/>
      <xs:enumeration value="tree"/>
      <xs:enumeration value="list"/>
      <xs:enumeration value="menu"/>
      <xs:enumeration value="custom"/>
      <!-- Additional types omitted for brevity -->
    </xs:restriction>
  </xs:simpleType>

  <xs:complexType name="ComponentIdentityType">
    <xs:sequence>
      <xs:element name="name" type="xs:string" minOccurs="0"/>
      <xs:element name="text" type="xs:string" minOccurs="0"/>
      <xs:element name="title" type="xs:string" minOccurs="0"/>
      <xs:element name="tooltip" type="xs:string" minOccurs="0"/>
      <xs:element name="actionCommand" type="xs:string" minOccurs="0"/>
      <xs:element name="labelText" type="xs:string" minOccurs="0"/>
      <xs:element name="internalName" type="xs:string" minOccurs="0"/>
    </xs:sequence>
  </xs:complexType>

  <xs:complexType name="ComponentGeometryType">
    <xs:sequence>
      <xs:element name="bounds" type="uit:BoundsType"/>
      <xs:element name="screenLocation" type="uit:PointType" minOccurs="0"/>
      <xs:element name="preferredSize" type="uit:DimensionType"/>
      <xs:element name="minimumSize" type="uit:DimensionType"/>
      <xs:element name="maximumSize" type="uit:DimensionType"/>
      <xs:element name="alignment" type="uit:AlignmentType" minOccurs="0"/>
    </xs:sequence>
  </xs:complexType>

  <xs:complexType name="BoundsType">
    <xs:attribute name="x" type="xs:int" use="required"/>
    <xs:attribute name="y" type="xs:int" use="required"/>
    <xs:attribute name="width" type="xs:int" use="required"/>
    <xs:attribute name="height" type="xs:int" use="required"/>
  </xs:complexType>

  <xs:complexType name="PointType">
    <xs:attribute name="x" type="xs:int" use="required"/>
    <xs:attribute name="y" type="xs:int" use="required"/>
  </xs:complexType>

  <xs:complexType name="DimensionType">
    <xs:attribute name="width" type="xs:int" use="required"/>
    <xs:attribute name="height" type="xs:int" use="required"/>
  </xs:complexType>

  <xs:complexType name="AlignmentType">
    <xs:attribute name="x" type="xs:float" use="required"/>
    <xs:attribute name="y" type="xs:float" use="required"/>
  </xs:complexType>

  <xs:complexType name="ComponentStateType">
    <xs:attribute name="visible" type="xs:boolean" use="required"/>
    <xs:attribute name="showing" type="xs:boolean" use="required"/>
    <xs:attribute name="enabled" type="xs:boolean" use="required"/>
    <xs:attribute name="focusable" type="xs:boolean" use="required"/>
    <xs:attribute name="focused" type="xs:boolean" use="required"/>
    <xs:attribute name="opaque" type="xs:boolean"/>
    <xs:attribute name="valid" type="xs:boolean"/>
    <xs:attribute name="doubleBuffered" type="xs:boolean"/>
    <xs:attribute name="selected" type="xs:boolean"/>
    <xs:attribute name="editable" type="xs:boolean"/>
  </xs:complexType>

  <xs:complexType name="ComponentPropertiesType">
    <xs:choice>
      <xs:element name="button" type="uit:ButtonPropertiesType"/>
      <xs:element name="textField" type="uit:TextFieldPropertiesType"/>
      <xs:element name="comboBox" type="uit:ComboBoxPropertiesType"/>
      <xs:element name="list" type="uit:ListPropertiesType"/>
      <xs:element name="table" type="uit:TablePropertiesType"/>
      <xs:element name="tree" type="uit:TreePropertiesType"/>
      <xs:element name="generic" type="uit:GenericPropertiesType"/>
    </xs:choice>
  </xs:complexType>

  <xs:complexType name="ButtonPropertiesType">
    <xs:attribute name="mnemonic" type="xs:string"/>
    <xs:attribute name="pressed" type="xs:boolean"/>
    <xs:attribute name="rollover" type="xs:boolean"/>
  </xs:complexType>

  <xs:complexType name="TextFieldPropertiesType">
    <xs:attribute name="columns" type="xs:int"/>
    <xs:attribute name="caretPosition" type="xs:int"/>
    <xs:attribute name="selectionStart" type="xs:int"/>
    <xs:attribute name="selectionEnd" type="xs:int"/>
  </xs:complexType>

  <xs:complexType name="ComboBoxPropertiesType">
    <xs:sequence>
      <xs:element name="items" type="uit:StringListType" minOccurs="0"/>
    </xs:sequence>
    <xs:attribute name="itemCount" type="xs:int"/>
    <xs:attribute name="selectedIndex" type="xs:int"/>
    <xs:attribute name="selectedItem" type="xs:string"/>
    <xs:attribute name="popupVisible" type="xs:boolean"/>
  </xs:complexType>

  <xs:complexType name="ListPropertiesType">
    <xs:sequence>
      <xs:element name="items" type="uit:StringListType" minOccurs="0"/>
      <xs:element name="selectedIndices" type="uit:IntListType" minOccurs="0"/>
    </xs:sequence>
    <xs:attribute name="itemCount" type="xs:int"/>
    <xs:attribute name="selectionMode" type="xs:string"/>
  </xs:complexType>

  <xs:complexType name="TablePropertiesType">
    <xs:sequence>
      <xs:element name="columnNames" type="uit:StringListType" minOccurs="0"/>
      <xs:element name="selectedRows" type="uit:IntListType" minOccurs="0"/>
    </xs:sequence>
    <xs:attribute name="rowCount" type="xs:int"/>
    <xs:attribute name="columnCount" type="xs:int"/>
    <xs:attribute name="rowHeight" type="xs:int"/>
  </xs:complexType>

  <xs:complexType name="TreePropertiesType">
    <xs:sequence>
      <xs:element name="selectedPaths" type="uit:StringListType" minOccurs="0"/>
      <xs:element name="expandedPaths" type="uit:StringListType" minOccurs="0"/>
    </xs:sequence>
    <xs:attribute name="rowCount" type="xs:int"/>
    <xs:attribute name="rootVisible" type="xs:boolean"/>
  </xs:complexType>

  <xs:complexType name="GenericPropertiesType">
    <xs:sequence>
      <xs:element name="property" minOccurs="0" maxOccurs="unbounded">
        <xs:complexType>
          <xs:simpleContent>
            <xs:extension base="xs:string">
              <xs:attribute name="name" type="xs:string" use="required"/>
              <xs:attribute name="type" type="xs:string"/>
            </xs:extension>
          </xs:simpleContent>
        </xs:complexType>
      </xs:element>
    </xs:sequence>
  </xs:complexType>

  <xs:complexType name="AccessibilityInfoType">
    <xs:sequence>
      <xs:element name="accessibleName" type="xs:string" minOccurs="0"/>
      <xs:element name="accessibleDescription" type="xs:string" minOccurs="0"/>
      <xs:element name="accessibleRole" type="xs:string" minOccurs="0"/>
      <xs:element name="accessibleState" type="uit:StringListType" minOccurs="0"/>
    </xs:sequence>
  </xs:complexType>

  <xs:complexType name="TraversalMetadataType">
    <xs:attribute name="siblingIndex" type="xs:unsignedInt" use="required"/>
    <xs:attribute name="siblingCount" type="xs:unsignedInt" use="required"/>
    <xs:attribute name="childCount" type="xs:unsignedInt" use="required"/>
    <xs:attribute name="isLeaf" type="xs:boolean" use="required"/>
    <xs:attribute name="capturedAt" type="xs:dateTime"/>
    <xs:attribute name="childrenLoaded" type="xs:boolean"/>
  </xs:complexType>

  <xs:complexType name="TreeMetadataType">
    <xs:sequence>
      <xs:element name="applicationName" type="xs:string" minOccurs="0"/>
      <xs:element name="filterApplied" type="uit:FilterSpecificationType" minOccurs="0"/>
    </xs:sequence>
    <xs:attribute name="capturedAt" type="xs:dateTime" use="required"/>
    <xs:attribute name="libraryVersion" type="xs:string" use="required"/>
    <xs:attribute name="jvmVersion" type="xs:string"/>
    <xs:attribute name="maxDepth" type="xs:unsignedInt" use="required"/>
  </xs:complexType>

  <xs:complexType name="TreeStatisticsType">
    <xs:sequence>
      <xs:element name="componentsByType" minOccurs="0">
        <xs:complexType>
          <xs:sequence>
            <xs:element name="entry" minOccurs="0" maxOccurs="unbounded">
              <xs:complexType>
                <xs:attribute name="type" type="xs:string" use="required"/>
                <xs:attribute name="count" type="xs:unsignedInt" use="required"/>
              </xs:complexType>
            </xs:element>
          </xs:sequence>
        </xs:complexType>
      </xs:element>
    </xs:sequence>
    <xs:attribute name="totalComponents" type="xs:unsignedInt" use="required"/>
    <xs:attribute name="maxDepth" type="xs:unsignedInt" use="required"/>
    <xs:attribute name="visibleCount" type="xs:unsignedInt"/>
    <xs:attribute name="enabledCount" type="xs:unsignedInt"/>
    <xs:attribute name="captureDurationMs" type="xs:unsignedLong"/>
  </xs:complexType>

  <xs:complexType name="FilterSpecificationType">
    <xs:sequence>
      <xs:element name="componentTypes" type="uit:StringListType" minOccurs="0"/>
    </xs:sequence>
    <xs:attribute name="maxDepth" type="xs:unsignedInt"/>
    <xs:attribute name="visibleOnly" type="xs:boolean"/>
    <xs:attribute name="enabledOnly" type="xs:boolean"/>
    <xs:attribute name="namePattern" type="xs:string"/>
  </xs:complexType>

  <xs:complexType name="StringListType">
    <xs:sequence>
      <xs:element name="item" type="xs:string" minOccurs="0" maxOccurs="unbounded"/>
    </xs:sequence>
  </xs:complexType>

  <xs:complexType name="IntListType">
    <xs:sequence>
      <xs:element name="item" type="xs:int" minOccurs="0" maxOccurs="unbounded"/>
    </xs:sequence>
  </xs:complexType>

</xs:schema>
```

### 2.3 Human-Readable Text Format

```
================================================================================
                         UI TREE - My Swing Application
================================================================================
Captured: 2024-01-15T10:30:45.123Z
Library Version: 0.1.0
JVM Version: 17.0.1
Total Components: 147 | Visible: 89 | Max Depth: 8
Capture Duration: 45ms
================================================================================

[0] JFrame "Main Window" (javax.swing.JFrame)
    |-- Name: mainFrame
    |-- Bounds: (100, 100, 800x600)
    |-- State: visible=true, enabled=true, focused=true
    |
    +-- [0.0] JMenuBar (javax.swing.JMenuBar)
    |   |-- Bounds: (0, 0, 800x25)
    |   |
    |   +-- [0.0.0] JMenu "File" (javax.swing.JMenu)
    |   |   |-- Name: fileMenu
    |   |   |-- Mnemonic: F
    |   |   |-- Items: 5
    |   |   |
    |   |   +-- [0.0.0.0] JMenuItem "New" (javax.swing.JMenuItem)
    |   |   |   |-- Name: newMenuItem
    |   |   |   |-- Accelerator: Ctrl+N
    |   |   |   +-- State: enabled=true
    |   |   |
    |   |   +-- [0.0.0.1] JMenuItem "Open" (javax.swing.JMenuItem)
    |   |   |   |-- Name: openMenuItem
    |   |   |   |-- Accelerator: Ctrl+O
    |   |   |   +-- State: enabled=true
    |   |   |
    |   |   +-- [0.0.0.2] JSeparator (javax.swing.JSeparator)
    |   |   |
    |   |   +-- [0.0.0.3] JMenuItem "Exit" (javax.swing.JMenuItem)
    |   |       |-- Name: exitMenuItem
    |   |       +-- State: enabled=true
    |   |
    |   +-- [0.0.1] JMenu "Edit" (javax.swing.JMenu)
    |       |-- Name: editMenu
    |       +-- Items: 3
    |
    +-- [0.1] JPanel "contentPanel" (javax.swing.JPanel)
        |-- Bounds: (0, 25, 800x575)
        |-- Layout: BorderLayout
        |
        +-- [0.1.0] JToolBar (javax.swing.JToolBar)
        |   |-- Bounds: (0, 0, 800x35)
        |   |-- Orientation: HORIZONTAL
        |   |
        |   +-- [0.1.0.0] JButton "New" (javax.swing.JButton)
        |   |   |-- Name: newButton
        |   |   |-- Icon: new_icon.png
        |   |   +-- State: enabled=true
        |   |
        |   +-- [0.1.0.1] JButton "Save" (javax.swing.JButton)
        |       |-- Name: saveButton
        |       +-- State: enabled=false
        |
        +-- [0.1.1] JSplitPane (javax.swing.JSplitPane)
            |-- Bounds: (0, 35, 800x540)
            |-- Divider: 200
            |-- Orientation: HORIZONTAL
            |
            +-- [0.1.1.0] JScrollPane (javax.swing.JScrollPane)
            |   |
            |   +-- [0.1.1.0.0] JTree "fileTree" (javax.swing.JTree)
            |       |-- Name: fileTree
            |       |-- Rows: 25
            |       |-- Selected: /home/user/project
            |       +-- Root Visible: true
            |
            +-- [0.1.1.1] JTabbedPane (javax.swing.JTabbedPane)
                |-- Name: editorTabs
                |-- Tabs: 3
                |-- Selected: 1
                |
                +-- [Tab 0: "untitled.txt"]
                |   +-- [0.1.1.1.0] JScrollPane
                |       +-- JTextArea (100 cols x 30 rows)
                |
                +-- [Tab 1: "Main.java" - SELECTED]
                |   +-- [0.1.1.1.1] JScrollPane
                |       +-- JEditorPane (syntax highlighting)
                |
                +-- [Tab 2: "config.xml"]
                    +-- [0.1.1.1.2] JScrollPane
                        +-- JTextPane

================================================================================
                              COMPONENT SUMMARY
================================================================================
JButton: 12 | JLabel: 23 | JTextField: 8 | JPanel: 15 | JMenu: 4
JMenuItem: 18 | JTable: 2 | JTree: 1 | JTabbedPane: 1 | Other: 63
================================================================================
```

### 2.4 Robot Framework Log Format

```robot
================================================================================
UI TREE DUMP - ${TIMESTAMP}
Application: My Swing Application
================================================================================

*** Component Hierarchy ***

Level 0: JFrame [mainFrame]
    - Class: javax.swing.JFrame
    - Title: Main Window
    - Bounds: x=100, y=100, w=800, h=600
    - Visible: True | Enabled: True | Focused: True
    - Children: 2

    Level 1: JMenuBar
        - Bounds: x=0, y=0, w=800, h=25
        - Children: 2

        Level 2: JMenu [fileMenu]
            - Text: File
            - Mnemonic: F
            - Enabled: True
            - Children: 4

    Level 1: JPanel [contentPanel]
        - Bounds: x=0, y=25, w=800, h=575
        - Children: 2

        Level 2: JButton [saveButton]
            - Text: Save
            - Enabled: False
            - Tooltip: Save the current document
            - Action Command: save

*** Locator Reference ***

| Component Type | Name/ID | Suggested Locator |
|----------------|---------|-------------------|
| JFrame | mainFrame | name:mainFrame |
| JButton | saveButton | name:saveButton |
| JButton | - | text:Save |
| JMenu | fileMenu | name:fileMenu |
| JTree | fileTree | name:fileTree |
| JTextField | searchField | name:searchField |
| JTable | dataTable | name:dataTable |

*** Quick Stats ***
Total: 147 | Visible: 89 | Enabled: 112 | Focusable: 45
================================================================================
```

## 3. Filtering System

### 3.1 Filter Specification

```rust
/// Complete filter specification for UI tree traversal
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterSpecification {
    /// Filter by component types
    pub component_types: Option<ComponentTypeFilter>,

    /// Filter by property values
    pub properties: Option<PropertyFilter>,

    /// Filter by depth level
    pub depth: Option<DepthFilter>,

    /// Filter by state
    pub state: Option<StateFilter>,

    /// Filter by name/text patterns
    pub patterns: Option<PatternFilter>,

    /// Filter by geometry/position
    pub geometry: Option<GeometryFilter>,

    /// Custom filter expression (CSS/XPath-like)
    pub expression: Option<String>,

    /// Combine multiple filters
    pub combination: FilterCombination,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum FilterCombination {
    #[default]
    And,
    Or,
}

/// Component type filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentTypeFilter {
    /// Include only these base types
    pub include_types: Option<Vec<SwingBaseType>>,
    /// Exclude these base types
    pub exclude_types: Option<Vec<SwingBaseType>>,
    /// Include by class name pattern (regex)
    pub class_pattern: Option<String>,
    /// Include components implementing these interfaces
    pub interfaces: Option<Vec<String>>,
    /// Include components extending this class
    pub extends: Option<String>,
}

/// Property value filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyFilter {
    /// Property conditions (AND combined)
    pub conditions: Vec<PropertyCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyCondition {
    /// Property path (e.g., "properties.data.value", "identity.text")
    pub path: String,
    /// Comparison operator
    pub operator: ComparisonOperator,
    /// Value to compare
    pub value: PropertyValue,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ComparisonOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    Matches,  // Regex
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    IsNull,
    IsNotNull,
    In,
    NotIn,
}

/// Depth level filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthFilter {
    /// Minimum depth (0 = root)
    pub min_depth: Option<u32>,
    /// Maximum depth
    pub max_depth: Option<u32>,
    /// Exact depth
    pub exact_depth: Option<u32>,
}

/// State filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateFilter {
    /// Filter by visibility
    pub visible: Option<bool>,
    /// Filter by showing (actually rendered)
    pub showing: Option<bool>,
    /// Filter by enabled state
    pub enabled: Option<bool>,
    /// Filter by focusable
    pub focusable: Option<bool>,
    /// Filter by focused
    pub focused: Option<bool>,
    /// Filter by selected state
    pub selected: Option<bool>,
    /// Filter by editable state
    pub editable: Option<bool>,
}

/// Pattern matching on names and text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternFilter {
    /// Name pattern (glob or regex)
    pub name_pattern: Option<PatternSpec>,
    /// Text content pattern
    pub text_pattern: Option<PatternSpec>,
    /// Tooltip pattern
    pub tooltip_pattern: Option<PatternSpec>,
    /// Title pattern
    pub title_pattern: Option<PatternSpec>,
    /// Any text property matches
    pub any_text_pattern: Option<PatternSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternSpec {
    /// The pattern string
    pub pattern: String,
    /// Pattern type
    pub pattern_type: PatternType,
    /// Case sensitive
    pub case_sensitive: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PatternType {
    /// Exact match
    Exact,
    /// Glob pattern (*, ?, [])
    Glob,
    /// Regular expression
    Regex,
    /// Contains substring
    Contains,
}

/// Geometry/position filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometryFilter {
    /// Filter by bounds intersection
    pub bounds_intersect: Option<Bounds>,
    /// Filter by bounds containment
    pub bounds_contain: Option<Bounds>,
    /// Filter by minimum size
    pub min_size: Option<Dimension>,
    /// Filter by maximum size
    pub max_size: Option<Dimension>,
    /// Filter by position (within rectangle)
    pub position_within: Option<Bounds>,
}
```

### 3.2 CSS/XPath-like Locator Syntax

```rust
/// Locator expression parser
///
/// Supported syntax examples:
///
/// CSS-like:
///   - `JButton` - All JButton components
///   - `JButton.primary` - JButton with name "primary"
///   - `JButton#saveBtn` - JButton with internal name "saveBtn"
///   - `JButton[text="Save"]` - JButton with text "Save"
///   - `JButton[text^="Save"]` - text starts with "Save"
///   - `JButton[text$="..."]` - text ends with "..."
///   - `JButton[text*="ave"]` - text contains "ave"
///   - `JButton[enabled=true]` - enabled buttons
///   - `JPanel > JButton` - direct child buttons of panels
///   - `JFrame JButton` - any descendant button in frame
///   - `JButton:first-child` - first button among siblings
///   - `JButton:visible` - visible buttons
///   - `JButton:enabled` - enabled buttons
///   - `*[name="search"]` - any component with name
///
/// XPath-like:
///   - `//JButton` - All JButton anywhere
///   - `//JButton[@text='Save']` - Button with text
///   - `//JPanel/JButton` - Direct child
///   - `//JButton[1]` - First button (1-indexed)
///   - `//JFrame//JButton` - Descendant
///   - `/JFrame/JPanel/JButton` - Exact path from root
///   - `//JButton[@enabled and @visible]` - Combined conditions
///   - `//JButton[contains(@text, 'Save')]` - Contains function
///   - `//JButton[starts-with(@name, 'btn')]` - Starts with
///   - `//JTable/row[3]/cell[2]` - Table cell access

#[derive(Debug, Clone)]
pub enum LocatorExpression {
    /// CSS selector
    Css(CssSelector),
    /// XPath expression
    XPath(XPathExpression),
    /// Simple property-based locator
    Simple(SimpleLocator),
}

#[derive(Debug, Clone)]
pub struct CssSelector {
    pub segments: Vec<CssSelectorSegment>,
}

#[derive(Debug, Clone)]
pub struct CssSelectorSegment {
    /// Component type (e.g., "JButton", "*" for any)
    pub element: String,
    /// Class selector (.name)
    pub class: Option<String>,
    /// ID selector (#internalName)
    pub id: Option<String>,
    /// Attribute selectors
    pub attributes: Vec<AttributeSelector>,
    /// Pseudo selectors
    pub pseudos: Vec<PseudoSelector>,
    /// Combinator to next segment
    pub combinator: Option<Combinator>,
}

#[derive(Debug, Clone)]
pub struct AttributeSelector {
    pub name: String,
    pub operator: Option<AttributeOperator>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum AttributeOperator {
    Equals,        // =
    NotEquals,     // !=
    Contains,      // *=
    StartsWith,    // ^=
    EndsWith,      // $=
    Matches,       // ~= (regex)
}

#[derive(Debug, Clone)]
pub enum PseudoSelector {
    FirstChild,
    LastChild,
    NthChild(i32),
    NthLastChild(i32),
    Visible,
    Hidden,
    Enabled,
    Disabled,
    Checked,
    Selected,
    Focus,
    Empty,
    Not(Box<CssSelectorSegment>),
}

#[derive(Debug, Clone, Copy)]
pub enum Combinator {
    Descendant,     // space
    Child,          // >
    NextSibling,    // +
    SubsequentSibling, // ~
}

#[derive(Debug, Clone)]
pub struct SimpleLocator {
    pub locator_type: SimpleLocatorType,
    pub value: String,
}

#[derive(Debug, Clone, Copy)]
pub enum SimpleLocatorType {
    Name,
    InternalName,
    Text,
    Tooltip,
    Class,
    Index,
    Id,
    Label,
    AccessibleName,
}

impl LocatorExpression {
    /// Parse locator string
    pub fn parse(input: &str) -> Result<Self, LocatorParseError> {
        let trimmed = input.trim();

        // Detect XPath (starts with / or //)
        if trimmed.starts_with('/') {
            return Ok(Self::XPath(XPathExpression::parse(trimmed)?));
        }

        // Detect simple locator (type:value)
        if let Some(simple) = SimpleLocator::try_parse(trimmed) {
            return Ok(Self::Simple(simple));
        }

        // Default to CSS
        Ok(Self::Css(CssSelector::parse(trimmed)?))
    }
}
```

### 3.3 Filter Builder API

```rust
/// Fluent filter builder for easy filter construction
pub struct FilterBuilder {
    filter: FilterSpecification,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            filter: FilterSpecification::default(),
        }
    }

    /// Filter by component types
    pub fn types(mut self, types: &[SwingBaseType]) -> Self {
        let type_filter = self.filter.component_types.get_or_insert_with(Default::default);
        type_filter.include_types = Some(types.to_vec());
        self
    }

    /// Exclude component types
    pub fn exclude_types(mut self, types: &[SwingBaseType]) -> Self {
        let type_filter = self.filter.component_types.get_or_insert_with(Default::default);
        type_filter.exclude_types = Some(types.to_vec());
        self
    }

    /// Filter by class name pattern
    pub fn class_pattern(mut self, pattern: &str) -> Self {
        let type_filter = self.filter.component_types.get_or_insert_with(Default::default);
        type_filter.class_pattern = Some(pattern.to_string());
        self
    }

    /// Visible components only
    pub fn visible(mut self) -> Self {
        let state_filter = self.filter.state.get_or_insert_with(Default::default);
        state_filter.visible = Some(true);
        state_filter.showing = Some(true);
        self
    }

    /// Enabled components only
    pub fn enabled(mut self) -> Self {
        let state_filter = self.filter.state.get_or_insert_with(Default::default);
        state_filter.enabled = Some(true);
        self
    }

    /// Set maximum depth
    pub fn max_depth(mut self, depth: u32) -> Self {
        let depth_filter = self.filter.depth.get_or_insert_with(Default::default);
        depth_filter.max_depth = Some(depth);
        self
    }

    /// Filter by name pattern
    pub fn name_matches(mut self, pattern: &str) -> Self {
        let pattern_filter = self.filter.patterns.get_or_insert_with(Default::default);
        pattern_filter.name_pattern = Some(PatternSpec {
            pattern: pattern.to_string(),
            pattern_type: PatternType::Glob,
            case_sensitive: false,
        });
        self
    }

    /// Filter by text content
    pub fn text_contains(mut self, text: &str) -> Self {
        let pattern_filter = self.filter.patterns.get_or_insert_with(Default::default);
        pattern_filter.text_pattern = Some(PatternSpec {
            pattern: text.to_string(),
            pattern_type: PatternType::Contains,
            case_sensitive: false,
        });
        self
    }

    /// Add property condition
    pub fn property(mut self, path: &str, op: ComparisonOperator, value: PropertyValue) -> Self {
        let prop_filter = self.filter.properties.get_or_insert_with(|| PropertyFilter {
            conditions: Vec::new(),
        });
        prop_filter.conditions.push(PropertyCondition {
            path: path.to_string(),
            operator: op,
            value,
        });
        self
    }

    /// Set custom expression
    pub fn expression(mut self, expr: &str) -> Self {
        self.filter.expression = Some(expr.to_string());
        self
    }

    /// Build the filter
    pub fn build(self) -> FilterSpecification {
        self.filter
    }
}

// Usage example:
// let filter = FilterBuilder::new()
//     .types(&[SwingBaseType::Button, SwingBaseType::TextField])
//     .visible()
//     .enabled()
//     .max_depth(5)
//     .name_matches("*btn*")
//     .build();
```

## 4. Robot Framework Keywords

### 4.1 Keyword Specifications

```robot
*** Keywords Documentation ***

# =============================================================================
# GET UI TREE
# =============================================================================
# Returns the complete UI tree starting from specified root or all windows.
#
# Arguments:
#   root            Optional root component locator. If not specified, returns
#                   all top-level windows.
#   format          Output format: json|xml|text|dict (default: dict)
#   max_depth       Maximum traversal depth (default: unlimited)
#   include_hidden  Include non-visible components (default: False)
#   include_disabled Include disabled components (default: True)
#   filter          Filter specification (CSS/XPath selector or dict)
#
# Returns:
#   UI tree in specified format
#
# Examples:
#   ${tree}=    Get UI Tree
#   ${tree}=    Get UI Tree    format=json
#   ${tree}=    Get UI Tree    root=name:mainFrame    max_depth=3
#   ${tree}=    Get UI Tree    filter=JButton:visible
#   ${tree}=    Get UI Tree    filter=//JPanel[@name='content']//JButton

# =============================================================================
# GET COMPONENT PROPERTIES
# =============================================================================
# Returns all properties of a specific component.
#
# Arguments:
#   locator         Component locator (CSS/XPath/simple)
#   properties      Specific properties to retrieve (default: all)
#   include_children Include child component info (default: False)
#   format          Output format: json|xml|dict (default: dict)
#
# Returns:
#   Component properties in specified format
#
# Examples:
#   ${props}=    Get Component Properties    name:saveButton
#   ${props}=    Get Component Properties    JButton[text="Save"]
#   ${props}=    Get Component Properties    //JTable    properties=rowCount,columnCount

# =============================================================================
# LOG UI TREE
# =============================================================================
# Logs the UI tree to Robot Framework log with formatting.
#
# Arguments:
#   root            Optional root component
#   level           Log level: INFO|DEBUG|TRACE (default: INFO)
#   format          Display format: tree|table|summary (default: tree)
#   max_depth       Maximum depth to display
#   filter          Filter specification
#   include_locators Suggest locators for each component (default: True)
#
# Examples:
#   Log UI Tree
#   Log UI Tree    level=DEBUG    format=table
#   Log UI Tree    filter=JButton    include_locators=True

# =============================================================================
# EXPORT UI TREE TO FILE
# =============================================================================
# Exports the UI tree to a file.
#
# Arguments:
#   path            Output file path
#   format          File format: json|xml|html|csv (default: auto from extension)
#   root            Optional root component
#   filter          Filter specification
#   pretty          Pretty print output (default: True)
#   include_schema  Include schema/DTD reference (default: False)
#
# Returns:
#   Path to created file
#
# Examples:
#   ${file}=    Export UI Tree To File    ${OUTPUT_DIR}/ui_tree.json
#   ${file}=    Export UI Tree To File    tree.xml    format=xml
#   ${file}=    Export UI Tree To File    buttons.json    filter=JButton

# =============================================================================
# FIND COMPONENTS BY FILTER
# =============================================================================
# Finds all components matching the filter specification.
#
# Arguments:
#   filter          Filter specification (required)
#   root            Optional search root
#   limit           Maximum results (default: unlimited)
#   include_properties Include full properties (default: False)
#
# Returns:
#   List of matching components
#
# Examples:
#   ${buttons}=    Find Components By Filter    JButton
#   ${buttons}=    Find Components By Filter    JButton:enabled    limit=10
#   ${inputs}=     Find Components By Filter    JTextField[editable=true]
#   ${items}=      Find Components By Filter    //JMenu//JMenuItem

# =============================================================================
# GET COMPONENT COUNT
# =============================================================================
# Returns count of components matching filter.
#
# Arguments:
#   filter          Filter specification (optional, default: all)
#   root            Optional search root
#
# Returns:
#   Integer count
#
# Examples:
#   ${count}=    Get Component Count
#   ${count}=    Get Component Count    JButton
#   ${count}=    Get Component Count    JButton:visible:enabled

# =============================================================================
# WAIT UNTIL UI TREE CONTAINS
# =============================================================================
# Waits until UI tree contains component matching filter.
#
# Arguments:
#   filter          Filter specification
#   timeout         Maximum wait time (default: 10s)
#   poll_interval   Check interval (default: 0.5s)
#   root            Optional search root
#
# Examples:
#   Wait Until UI Tree Contains    JButton[text="Ready"]
#   Wait Until UI Tree Contains    //JDialog[@title='Confirmation']    timeout=30s

# =============================================================================
# UI TREE SHOULD CONTAIN
# =============================================================================
# Verifies UI tree contains component matching filter.
#
# Arguments:
#   filter          Filter specification
#   message         Optional failure message
#   count           Expected count (optional)
#
# Examples:
#   UI Tree Should Contain    JButton[name='submit']
#   UI Tree Should Contain    JMenuItem    count=5

# =============================================================================
# COMPARE UI TREES
# =============================================================================
# Compares two UI tree captures and returns differences.
#
# Arguments:
#   tree1           First UI tree (or file path)
#   tree2           Second UI tree (or file path)
#   ignore_properties Properties to ignore in comparison
#   ignore_types    Component types to ignore
#
# Returns:
#   Comparison result with additions, removals, changes
#
# Examples:
#   ${tree1}=    Get UI Tree
#   # ... perform actions ...
#   ${tree2}=    Get UI Tree
#   ${diff}=     Compare UI Trees    ${tree1}    ${tree2}
```

### 4.2 Python Keyword Implementation Interface

```python
# src/robotframework_swing/keywords/ui_tree.py

from typing import Optional, List, Dict, Any, Union
from enum import Enum
from robot.api.deco import keyword, library
from robot.api import logger

from ..core import SwingLibraryCore
from ..models import UITree, UIComponent, FilterSpecification
from ..formats import JsonFormatter, XmlFormatter, TextFormatter


class OutputFormat(Enum):
    JSON = "json"
    XML = "xml"
    TEXT = "text"
    DICT = "dict"
    HTML = "html"


@library(scope='GLOBAL', version='0.1.0')
class UITreeKeywords:
    """Robot Framework keywords for UI Tree operations."""

    ROBOT_LIBRARY_SCOPE = 'GLOBAL'

    def __init__(self):
        self._core: Optional[SwingLibraryCore] = None
        self._cache: Dict[str, UITree] = {}

    @keyword("Get UI Tree")
    def get_ui_tree(
        self,
        root: Optional[str] = None,
        format: str = "dict",
        max_depth: Optional[int] = None,
        include_hidden: bool = False,
        include_disabled: bool = True,
        filter: Optional[str] = None,
        use_cache: bool = False
    ) -> Union[Dict, str]:
        """
        Returns the complete UI tree starting from specified root.

        Arguments:
        - ``root``: Optional root component locator
        - ``format``: Output format (json|xml|text|dict)
        - ``max_depth``: Maximum traversal depth
        - ``include_hidden``: Include non-visible components
        - ``include_disabled``: Include disabled components
        - ``filter``: Filter expression (CSS/XPath)
        - ``use_cache``: Use cached tree if available

        Returns the UI tree in the specified format.

        Examples:
        | ${tree}= | Get UI Tree |
        | ${tree}= | Get UI Tree | format=json |
        | ${tree}= | Get UI Tree | root=name:mainFrame | max_depth=3 |
        """
        filter_spec = self._build_filter(
            filter_expr=filter,
            max_depth=max_depth,
            include_hidden=include_hidden,
            include_disabled=include_disabled
        )

        tree = self._core.get_ui_tree(
            root_locator=root,
            filter_spec=filter_spec,
            use_cache=use_cache
        )

        return self._format_output(tree, OutputFormat(format))

    @keyword("Get Component Properties")
    def get_component_properties(
        self,
        locator: str,
        properties: Optional[List[str]] = None,
        include_children: bool = False,
        format: str = "dict"
    ) -> Union[Dict, str]:
        """
        Returns properties of a specific component.

        Arguments:
        - ``locator``: Component locator
        - ``properties``: Specific properties to retrieve
        - ``include_children``: Include child info
        - ``format``: Output format

        Examples:
        | ${props}= | Get Component Properties | name:saveButton |
        | ${props}= | Get Component Properties | JButton[text="Save"] |
        """
        component = self._core.find_component(locator)

        if properties:
            props = self._core.get_properties(component, properties)
        else:
            props = self._core.get_all_properties(
                component,
                include_children=include_children
            )

        return self._format_output(props, OutputFormat(format))

    @keyword("Log UI Tree")
    def log_ui_tree(
        self,
        root: Optional[str] = None,
        level: str = "INFO",
        format: str = "tree",
        max_depth: Optional[int] = None,
        filter: Optional[str] = None,
        include_locators: bool = True
    ) -> None:
        """
        Logs the UI tree to Robot Framework log.

        Arguments:
        - ``root``: Optional root component
        - ``level``: Log level (INFO|DEBUG|TRACE)
        - ``format``: Display format (tree|table|summary)
        - ``max_depth``: Maximum depth
        - ``filter``: Filter expression
        - ``include_locators``: Suggest locators

        Examples:
        | Log UI Tree |
        | Log UI Tree | level=DEBUG | format=table |
        """
        tree = self.get_ui_tree(
            root=root,
            format="dict",
            max_depth=max_depth,
            filter=filter
        )

        formatted = TextFormatter.format_for_log(
            tree,
            display_format=format,
            include_locators=include_locators
        )

        logger.write(formatted, level=level, html=format == "table")

    @keyword("Export UI Tree To File")
    def export_ui_tree_to_file(
        self,
        path: str,
        format: Optional[str] = None,
        root: Optional[str] = None,
        filter: Optional[str] = None,
        pretty: bool = True,
        include_schema: bool = False
    ) -> str:
        """
        Exports the UI tree to a file.

        Arguments:
        - ``path``: Output file path
        - ``format``: File format (auto-detected from extension)
        - ``root``: Optional root component
        - ``filter``: Filter expression
        - ``pretty``: Pretty print
        - ``include_schema``: Include schema reference

        Returns path to created file.

        Examples:
        | ${file}= | Export UI Tree To File | ui_tree.json |
        | ${file}= | Export UI Tree To File | tree.xml | format=xml |
        """
        tree = self.get_ui_tree(root=root, format="dict", filter=filter)

        # Auto-detect format from extension
        if format is None:
            format = self._detect_format_from_path(path)

        output_format = OutputFormat(format)
        content = self._format_output(
            tree,
            output_format,
            pretty=pretty,
            include_schema=include_schema
        )

        with open(path, 'w', encoding='utf-8') as f:
            f.write(content)

        logger.info(f"UI tree exported to: {path}")
        return path

    @keyword("Find Components By Filter")
    def find_components_by_filter(
        self,
        filter: str,
        root: Optional[str] = None,
        limit: Optional[int] = None,
        include_properties: bool = False
    ) -> List[Dict]:
        """
        Finds all components matching the filter.

        Arguments:
        - ``filter``: Filter expression (required)
        - ``root``: Optional search root
        - ``limit``: Maximum results
        - ``include_properties``: Include full properties

        Examples:
        | ${buttons}= | Find Components By Filter | JButton |
        | ${buttons}= | Find Components By Filter | JButton:enabled | limit=10 |
        """
        components = self._core.find_components(
            filter_expr=filter,
            root_locator=root,
            limit=limit
        )

        if include_properties:
            return [self._core.get_all_properties(c) for c in components]
        else:
            return [self._to_summary(c) for c in components]

    @keyword("Get Component Count")
    def get_component_count(
        self,
        filter: Optional[str] = None,
        root: Optional[str] = None
    ) -> int:
        """
        Returns count of components matching filter.

        Arguments:
        - ``filter``: Filter expression
        - ``root``: Optional search root

        Examples:
        | ${count}= | Get Component Count |
        | ${count}= | Get Component Count | JButton |
        """
        return self._core.count_components(
            filter_expr=filter,
            root_locator=root
        )

    @keyword("Wait Until UI Tree Contains")
    def wait_until_ui_tree_contains(
        self,
        filter: str,
        timeout: str = "10s",
        poll_interval: str = "0.5s",
        root: Optional[str] = None
    ) -> None:
        """
        Waits until UI tree contains matching component.

        Arguments:
        - ``filter``: Filter expression
        - ``timeout``: Maximum wait time
        - ``poll_interval``: Check interval
        - ``root``: Optional search root

        Examples:
        | Wait Until UI Tree Contains | JButton[text="Ready"] |
        | Wait Until UI Tree Contains | //JDialog | timeout=30s |
        """
        from robot.utils import timestr_to_secs

        timeout_sec = timestr_to_secs(timeout)
        interval_sec = timestr_to_secs(poll_interval)

        self._core.wait_for_component(
            filter_expr=filter,
            root_locator=root,
            timeout=timeout_sec,
            poll_interval=interval_sec
        )

    @keyword("UI Tree Should Contain")
    def ui_tree_should_contain(
        self,
        filter: str,
        message: Optional[str] = None,
        count: Optional[int] = None
    ) -> None:
        """
        Verifies UI tree contains matching component.

        Arguments:
        - ``filter``: Filter expression
        - ``message``: Optional failure message
        - ``count``: Expected count

        Examples:
        | UI Tree Should Contain | JButton[name='submit'] |
        | UI Tree Should Contain | JMenuItem | count=5 |
        """
        actual_count = self.get_component_count(filter)

        if count is not None:
            if actual_count != count:
                raise AssertionError(
                    message or f"Expected {count} components matching '{filter}', "
                               f"but found {actual_count}"
                )
        elif actual_count == 0:
            raise AssertionError(
                message or f"No components found matching '{filter}'"
            )

    @keyword("Compare UI Trees")
    def compare_ui_trees(
        self,
        tree1: Union[Dict, str],
        tree2: Union[Dict, str],
        ignore_properties: Optional[List[str]] = None,
        ignore_types: Optional[List[str]] = None
    ) -> Dict:
        """
        Compares two UI tree captures.

        Arguments:
        - ``tree1``: First UI tree or file path
        - ``tree2``: Second UI tree or file path
        - ``ignore_properties``: Properties to ignore
        - ``ignore_types``: Component types to ignore

        Returns comparison result with additions, removals, changes.

        Examples:
        | ${diff}= | Compare UI Trees | ${tree1} | ${tree2} |
        """
        t1 = self._load_tree(tree1)
        t2 = self._load_tree(tree2)

        return self._core.compare_trees(
            t1, t2,
            ignore_properties=ignore_properties or [],
            ignore_types=ignore_types or []
        )

    # Helper methods
    def _build_filter(self, **kwargs) -> FilterSpecification:
        """Build filter specification from arguments."""
        pass

    def _format_output(self, data: Any, format: OutputFormat, **kwargs) -> Union[Dict, str]:
        """Format output according to specified format."""
        pass

    def _detect_format_from_path(self, path: str) -> str:
        """Detect format from file extension."""
        pass

    def _to_summary(self, component: UIComponent) -> Dict:
        """Convert component to summary dict."""
        pass

    def _load_tree(self, tree_or_path: Union[Dict, str]) -> UITree:
        """Load tree from dict or file path."""
        pass
```

## 5. Performance Considerations

### 5.1 Lazy Loading Strategy

```rust
/// Lazy tree traversal with on-demand loading
pub struct LazyUITree {
    /// Root component references
    roots: Vec<LazyComponent>,
    /// Component cache
    cache: Arc<RwLock<ComponentCache>>,
    /// JNI connection
    jni: Arc<JniConnection>,
}

pub struct LazyComponent {
    /// Component ID
    id: ComponentId,
    /// Loaded data (if any)
    data: OnceCell<UIComponent>,
    /// Child references (lazy)
    children: OnceCell<Vec<LazyComponent>>,
    /// Reference to JNI for loading
    jni: Arc<JniConnection>,
}

impl LazyComponent {
    /// Load component data on demand
    pub fn load(&self) -> &UIComponent {
        self.data.get_or_init(|| {
            self.jni.load_component(&self.id)
        })
    }

    /// Load children on demand
    pub fn children(&self) -> &[LazyComponent] {
        self.children.get_or_init(|| {
            self.jni.load_children(&self.id)
                .into_iter()
                .map(|id| LazyComponent::new(id, Arc::clone(&self.jni)))
                .collect()
        })
    }

    /// Check if data is loaded without loading
    pub fn is_loaded(&self) -> bool {
        self.data.get().is_some()
    }
}
```

### 5.2 Caching Strategy

```rust
/// LRU cache for component data
pub struct ComponentCache {
    /// Cache entries
    entries: LinkedHashMap<ComponentId, CacheEntry>,
    /// Maximum cache size
    max_size: usize,
    /// Cache statistics
    stats: CacheStats,
}

pub struct CacheEntry {
    component: UIComponent,
    timestamp: Instant,
    access_count: u32,
}

#[derive(Default)]
pub struct CacheStats {
    hits: AtomicU64,
    misses: AtomicU64,
    evictions: AtomicU64,
}

impl ComponentCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: LinkedHashMap::new(),
            max_size,
            stats: CacheStats::default(),
        }
    }

    /// Get component from cache
    pub fn get(&mut self, id: &ComponentId) -> Option<&UIComponent> {
        if let Some(entry) = self.entries.get_refresh(id) {
            self.stats.hits.fetch_add(1, Ordering::Relaxed);
            entry.access_count += 1;
            Some(&entry.component)
        } else {
            self.stats.misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// Insert component into cache
    pub fn insert(&mut self, id: ComponentId, component: UIComponent) {
        // Evict if necessary
        while self.entries.len() >= self.max_size {
            self.entries.pop_front();
            self.stats.evictions.fetch_add(1, Ordering::Relaxed);
        }

        self.entries.insert(id, CacheEntry {
            component,
            timestamp: Instant::now(),
            access_count: 1,
        });
    }

    /// Invalidate entire cache
    pub fn invalidate(&mut self) {
        self.entries.clear();
    }

    /// Invalidate specific entry and descendants
    pub fn invalidate_subtree(&mut self, root_id: &ComponentId) {
        let prefix = &root_id.tree_path;
        self.entries.retain(|id, _| !id.tree_path.starts_with(prefix));
    }
}
```

### 5.3 Async Traversal

```rust
use tokio::sync::Semaphore;
use futures::stream::{self, StreamExt};

/// Async tree traversal with controlled concurrency
pub struct AsyncTreeTraverser {
    jni: Arc<JniConnection>,
    cache: Arc<RwLock<ComponentCache>>,
    semaphore: Arc<Semaphore>,
    max_concurrent: usize,
}

impl AsyncTreeTraverser {
    pub fn new(jni: Arc<JniConnection>, max_concurrent: usize) -> Self {
        Self {
            jni,
            cache: Arc::new(RwLock::new(ComponentCache::new(10000))),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            max_concurrent,
        }
    }

    /// Traverse tree asynchronously with controlled concurrency
    pub async fn traverse(
        &self,
        root: Option<&str>,
        filter: &FilterSpecification,
    ) -> Result<UITree, TraversalError> {
        let start = Instant::now();

        // Get root components
        let roots = self.get_roots(root).await?;

        // Traverse each root concurrently
        let traversed_roots: Vec<UIComponent> = stream::iter(roots)
            .map(|root| self.traverse_component(root, filter, 0))
            .buffer_unordered(self.max_concurrent)
            .collect()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();

        let duration = start.elapsed();

        Ok(UITree {
            roots: traversed_roots,
            metadata: TreeMetadata {
                captured_at: Utc::now().to_rfc3339(),
                library_version: env!("CARGO_PKG_VERSION").to_string(),
                max_depth: filter.depth.as_ref()
                    .and_then(|d| d.max_depth)
                    .unwrap_or(u32::MAX),
                ..Default::default()
            },
            statistics: self.calculate_statistics(&traversed_roots, duration),
        })
    }

    /// Traverse single component and its children
    async fn traverse_component(
        &self,
        component_ref: ComponentRef,
        filter: &FilterSpecification,
        depth: u32,
    ) -> Result<Option<UIComponent>, TraversalError> {
        // Acquire semaphore permit
        let _permit = self.semaphore.acquire().await?;

        // Check depth limit
        if let Some(max) = filter.depth.as_ref().and_then(|d| d.max_depth) {
            if depth > max {
                return Ok(None);
            }
        }

        // Load component data
        let component = self.load_component(&component_ref).await?;

        // Apply filter
        if !self.matches_filter(&component, filter) {
            return Ok(None);
        }

        // Get children
        let child_refs = self.get_children(&component_ref).await?;

        // Traverse children concurrently
        let children: Vec<UIComponent> = stream::iter(child_refs)
            .map(|child| self.traverse_component(child, filter, depth + 1))
            .buffer_unordered(self.max_concurrent.min(child_refs.len()))
            .collect()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(Some(UIComponent {
            children: if children.is_empty() { None } else { Some(children) },
            ..component
        }))
    }
}
```

### 5.4 Incremental Updates

```rust
/// Track changes between tree snapshots for efficient updates
pub struct TreeDiffer {
    /// Previous tree snapshot
    previous: Option<UITree>,
    /// Change listeners
    listeners: Vec<Box<dyn TreeChangeListener>>,
}

#[derive(Debug, Clone)]
pub struct TreeDiff {
    /// Added components
    pub added: Vec<UIComponent>,
    /// Removed components
    pub removed: Vec<ComponentId>,
    /// Changed components
    pub changed: Vec<ComponentChange>,
}

#[derive(Debug, Clone)]
pub struct ComponentChange {
    pub id: ComponentId,
    pub changes: Vec<PropertyChange>,
}

#[derive(Debug, Clone)]
pub struct PropertyChange {
    pub path: String,
    pub old_value: PropertyValue,
    pub new_value: PropertyValue,
}

pub trait TreeChangeListener: Send + Sync {
    fn on_component_added(&self, component: &UIComponent);
    fn on_component_removed(&self, id: &ComponentId);
    fn on_component_changed(&self, change: &ComponentChange);
}

impl TreeDiffer {
    /// Compute diff between previous and current tree
    pub fn diff(&self, current: &UITree) -> Option<TreeDiff> {
        let previous = self.previous.as_ref()?;

        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut changed = Vec::new();

        // Build maps for comparison
        let prev_map = self.build_component_map(previous);
        let curr_map = self.build_component_map(current);

        // Find added and changed
        for (id, component) in &curr_map {
            if let Some(prev_component) = prev_map.get(id) {
                let changes = self.compare_components(prev_component, component);
                if !changes.is_empty() {
                    changed.push(ComponentChange {
                        id: id.clone(),
                        changes,
                    });
                }
            } else {
                added.push(component.clone());
            }
        }

        // Find removed
        for id in prev_map.keys() {
            if !curr_map.contains_key(id) {
                removed.push(id.clone());
            }
        }

        Some(TreeDiff { added, removed, changed })
    }
}
```

## 6. Integration Architecture

### 6.1 Module Structure

```
robotframework-swing/
├── Cargo.toml
├── pyproject.toml
├── src/
│   ├── lib.rs                    # Rust library entry point
│   ├── core/
│   │   ├── mod.rs
│   │   ├── jni_bridge.rs         # JNI connection
│   │   ├── tree_traverser.rs     # Tree traversal
│   │   ├── component_loader.rs   # Component loading
│   │   └── property_extractor.rs # Property extraction
│   ├── models/
│   │   ├── mod.rs
│   │   ├── component.rs          # UIComponent struct
│   │   ├── tree.rs               # UITree struct
│   │   ├── properties.rs         # Property types
│   │   └── filter.rs             # Filter types
│   ├── filter/
│   │   ├── mod.rs
│   │   ├── specification.rs      # Filter specification
│   │   ├── matcher.rs            # Filter matching
│   │   └── locator.rs            # Locator parsing
│   ├── output/
│   │   ├── mod.rs
│   │   ├── json.rs               # JSON formatter
│   │   ├── xml.rs                # XML formatter
│   │   ├── text.rs               # Text formatter
│   │   └── robot.rs              # Robot log formatter
│   ├── cache/
│   │   ├── mod.rs
│   │   ├── lru.rs                # LRU cache
│   │   └── tree_cache.rs         # Tree-specific cache
│   └── python/
│       ├── mod.rs
│       └── bindings.rs           # PyO3 bindings
└── python/
    └── robotframework_swing/
        ├── __init__.py
        ├── keywords/
        │   ├── __init__.py
        │   ├── ui_tree.py        # UI Tree keywords
        │   ├── component.py      # Component keywords
        │   └── assertion.py      # Assertion keywords
        ├── models/
        │   ├── __init__.py
        │   └── types.py          # Python type definitions
        └── utils/
            ├── __init__.py
            └── formatters.py     # Output formatters
```

### 6.2 PyO3 Bindings

```rust
// src/python/bindings.rs

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

/// Python module initialization
#[pymodule]
fn _robotframework_swing(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyUITreeBuilder>()?;
    m.add_class::<PyFilterBuilder>()?;
    m.add_class::<PyComponentCache>()?;
    m.add_function(wrap_pyfunction!(get_ui_tree, m)?)?;
    m.add_function(wrap_pyfunction!(find_components, m)?)?;
    m.add_function(wrap_pyfunction!(parse_locator, m)?)?;
    Ok(())
}

/// Python wrapper for UITreeBuilder
#[pyclass]
pub struct PyUITreeBuilder {
    inner: UITreeBuilder,
}

#[pymethods]
impl PyUITreeBuilder {
    #[new]
    fn new() -> Self {
        Self {
            inner: UITreeBuilder::new(),
        }
    }

    /// Get the complete UI tree
    fn get_tree(
        &self,
        py: Python,
        root: Option<&str>,
        filter_spec: Option<&PyDict>,
        format: Option<&str>,
    ) -> PyResult<PyObject> {
        let filter = filter_spec
            .map(|d| self.dict_to_filter(py, d))
            .transpose()?
            .unwrap_or_default();

        let tree = py.allow_threads(|| {
            self.inner.build_tree(root, &filter)
        })?;

        match format.unwrap_or("dict") {
            "json" => Ok(serde_json::to_string(&tree)?.into_py(py)),
            "xml" => Ok(self.to_xml(&tree)?.into_py(py)),
            "dict" => self.tree_to_dict(py, &tree),
            _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Unknown format: {}", format.unwrap_or(""))
            )),
        }
    }

    /// Find components by filter
    fn find_components(
        &self,
        py: Python,
        filter_expr: &str,
        root: Option<&str>,
        limit: Option<usize>,
    ) -> PyResult<PyObject> {
        let components = py.allow_threads(|| {
            self.inner.find_components(filter_expr, root, limit)
        })?;

        let list = PyList::empty(py);
        for component in components {
            list.append(self.component_to_dict(py, &component)?)?;
        }
        Ok(list.into())
    }
}

/// Python wrapper for FilterBuilder
#[pyclass]
pub struct PyFilterBuilder {
    inner: FilterBuilder,
}

#[pymethods]
impl PyFilterBuilder {
    #[new]
    fn new() -> Self {
        Self {
            inner: FilterBuilder::new(),
        }
    }

    fn types(&mut self, types: Vec<&str>) -> PyResult<()> {
        let base_types: Vec<SwingBaseType> = types
            .iter()
            .map(|t| SwingBaseType::from_str(t))
            .collect::<Result<_, _>>()?;
        self.inner = std::mem::take(&mut self.inner).types(&base_types);
        Ok(())
    }

    fn visible(&mut self) -> PyResult<()> {
        self.inner = std::mem::take(&mut self.inner).visible();
        Ok(())
    }

    fn enabled(&mut self) -> PyResult<()> {
        self.inner = std::mem::take(&mut self.inner).enabled();
        Ok(())
    }

    fn max_depth(&mut self, depth: u32) -> PyResult<()> {
        self.inner = std::mem::take(&mut self.inner).max_depth(depth);
        Ok(())
    }

    fn name_matches(&mut self, pattern: &str) -> PyResult<()> {
        self.inner = std::mem::take(&mut self.inner).name_matches(pattern);
        Ok(())
    }

    fn expression(&mut self, expr: &str) -> PyResult<()> {
        self.inner = std::mem::take(&mut self.inner).expression(expr);
        Ok(())
    }

    fn build(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let filter = self.inner.clone().build();
            pythonize::pythonize(py, &filter).map_err(Into::into)
        })
    }
}

/// Standalone function to get UI tree
#[pyfunction]
fn get_ui_tree(
    py: Python,
    root: Option<&str>,
    filter: Option<&PyDict>,
    format: Option<&str>,
) -> PyResult<PyObject> {
    let builder = PyUITreeBuilder::new();
    builder.get_tree(py, root, filter, format)
}

/// Standalone function to find components
#[pyfunction]
fn find_components(
    py: Python,
    filter_expr: &str,
    root: Option<&str>,
    limit: Option<usize>,
) -> PyResult<PyObject> {
    let builder = PyUITreeBuilder::new();
    builder.find_components(py, filter_expr, root, limit)
}

/// Parse locator expression
#[pyfunction]
fn parse_locator(expr: &str) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let parsed = LocatorExpression::parse(expr)?;
        pythonize::pythonize(py, &parsed).map_err(Into::into)
    })
}
```

## 7. Summary

This architecture provides:

1. **Comprehensive Data Model**: Complete Swing component representation with type-specific properties
2. **Multiple Output Formats**: JSON, XML, human-readable text, and Robot Framework log formats
3. **Powerful Filtering**: CSS/XPath-like selectors with property-based filtering
4. **Performance Optimizations**: Lazy loading, LRU caching, async traversal
5. **Clean Integration**: PyO3 bindings for seamless Python/Robot Framework usage
6. **Extensibility**: Modular design for adding new component types and output formats

The system is designed to handle large UI trees efficiently while providing a user-friendly API for test automation.
