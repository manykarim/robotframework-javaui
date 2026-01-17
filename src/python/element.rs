//! SwingElement Python wrapper
//!
//! This module provides a Python-friendly wrapper around UIComponent
//! for interacting with individual Swing elements.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use crate::model::{
    ComponentProperties,
    SwingBaseType, UIComponent,
};


/// Represents a reference to a Swing UI element
///
/// This class wraps a UIComponent and provides methods for interaction
/// and property access in Robot Framework tests.
#[pyclass(name = "SwingElement")]
#[derive(Clone)]
pub struct SwingElement {
    /// Internal component data
    #[pyo3(get)]
    pub hash_code: i64,

    #[pyo3(get)]
    pub tree_path: String,

    #[pyo3(get)]
    pub depth: u32,

    /// Component type info
    #[pyo3(get)]
    pub class_name: String,

    #[pyo3(get)]
    pub simple_name: String,

    #[pyo3(get)]
    pub base_type: String,

    /// Identity properties
    #[pyo3(get)]
    pub name: Option<String>,

    #[pyo3(get)]
    pub text: Option<String>,

    #[pyo3(get)]
    pub title: Option<String>,

    #[pyo3(get)]
    pub tooltip: Option<String>,

    #[pyo3(get)]
    pub action_command: Option<String>,

    #[pyo3(get)]
    pub internal_name: Option<String>,

    /// Geometry
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
    pub showing: bool,

    #[pyo3(get)]
    pub enabled: bool,

    #[pyo3(get)]
    pub focusable: bool,

    #[pyo3(get)]
    pub focused: bool,

    #[pyo3(get)]
    pub selected: Option<bool>,

    #[pyo3(get)]
    pub editable: Option<bool>,

    /// Accessibility
    #[pyo3(get)]
    pub accessible_name: Option<String>,

    #[pyo3(get)]
    pub accessible_description: Option<String>,

    /// Child count
    #[pyo3(get)]
    pub child_count: u32,

    /// Reference to parent library for actions (optional)
    #[pyo3(get)]
    pub is_connected: bool,

    /// Store component-specific properties as JSON
    properties_json: String,

    /// Children cache
    children_cache: Option<Vec<SwingElement>>,
}

#[pymethods]
impl SwingElement {
    /// Create a new SwingElement (primarily for internal use)
    #[new]
    #[pyo3(signature = (
        hash_code,
        tree_path,
        class_name,
        simple_name = None,
        name = None,
        text = None,
        enabled = true,
        visible = true
    ))]
    pub fn new(
        hash_code: i64,
        tree_path: String,
        class_name: String,
        simple_name: Option<String>,
        name: Option<String>,
        text: Option<String>,
        enabled: bool,
        visible: bool,
    ) -> Self {
        let base_type = SwingBaseType::from_class_name(&class_name);
        Self {
            hash_code,
            tree_path: tree_path.clone(),
            depth: tree_path.matches('.').count() as u32,
            class_name: class_name.clone(),
            simple_name: simple_name.unwrap_or_else(|| {
                class_name.rsplit('.').next().unwrap_or(&class_name).to_string()
            }),
            base_type: format!("{:?}", base_type),
            name,
            text,
            title: None,
            tooltip: None,
            action_command: None,
            internal_name: None,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            visible,
            showing: visible,
            enabled,
            focusable: true,
            focused: false,
            selected: None,
            editable: None,
            accessible_name: None,
            accessible_description: None,
            child_count: 0,
            is_connected: false,
            properties_json: "{}".to_string(),
            children_cache: None,
        }
    }

    /// Get a unique identifier for this element
    #[getter]
    pub fn id(&self) -> String {
        format!("{}@{}", self.tree_path, self.hash_code)
    }

    /// Get the bounds as a tuple (x, y, width, height)
    #[getter]
    pub fn bounds(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.width, self.height)
    }

    /// Get properties JSON (for internal use)
    pub fn get_properties_json(&self) -> &str {
        &self.properties_json
    }

    /// Get the best identifier for this element
    #[getter]
    pub fn best_identifier(&self) -> Option<String> {
        self.internal_name
            .clone()
            .or_else(|| self.name.clone())
            .or_else(|| self.text.clone())
            .or_else(|| self.title.clone())
    }

    /// Check if the element is a container type
    pub fn is_container(&self) -> bool {
        matches!(
            self.base_type.as_str(),
            "Frame"
                | "Dialog"
                | "Panel"
                | "ScrollPane"
                | "SplitPane"
                | "TabbedPane"
                | "ToolBar"
                | "InternalFrame"
                | "LayeredPane"
                | "RootPane"
                | "DesktopPane"
                | "ContentPane"
        )
    }

    /// Check if the element is a text component
    pub fn is_text_component(&self) -> bool {
        matches!(
            self.base_type.as_str(),
            "TextField"
                | "TextArea"
                | "PasswordField"
                | "EditorPane"
                | "TextPane"
                | "FormattedTextField"
        )
    }

    /// Check if the element is a menu component
    pub fn is_menu_component(&self) -> bool {
        matches!(
            self.base_type.as_str(),
            "MenuBar"
                | "Menu"
                | "MenuItem"
                | "PopupMenu"
                | "CheckBoxMenuItem"
                | "RadioButtonMenuItem"
        )
    }

    /// Get a specific property by name
    ///
    /// Args:
    ///     property_name: Name of the property to retrieve
    ///
    /// Returns:
    ///     The property value or None if not found
    #[pyo3(signature = (property_name))]
    pub fn get_property(&self, py: Python<'_>, property_name: &str) -> PyResult<PyObject> {
        // Try standard properties first
        match property_name {
            "name" => return Ok(self.name.clone().into_py(py)),
            "text" => return Ok(self.text.clone().into_py(py)),
            "title" => return Ok(self.title.clone().into_py(py)),
            "tooltip" => return Ok(self.tooltip.clone().into_py(py)),
            "enabled" => return Ok(self.enabled.into_py(py)),
            "visible" => return Ok(self.visible.into_py(py)),
            "showing" => return Ok(self.showing.into_py(py)),
            "focused" => return Ok(self.focused.into_py(py)),
            "focusable" => return Ok(self.focusable.into_py(py)),
            "selected" => return Ok(self.selected.into_py(py)),
            "editable" => return Ok(self.editable.into_py(py)),
            "class_name" | "className" => return Ok(self.class_name.clone().into_py(py)),
            "simple_name" | "simpleName" => return Ok(self.simple_name.clone().into_py(py)),
            "x" => return Ok(self.x.into_py(py)),
            "y" => return Ok(self.y.into_py(py)),
            "width" => return Ok(self.width.into_py(py)),
            "height" => return Ok(self.height.into_py(py)),
            "hash_code" | "hashCode" => return Ok(self.hash_code.into_py(py)),
            "tree_path" | "treePath" => return Ok(self.tree_path.clone().into_py(py)),
            "depth" => return Ok(self.depth.into_py(py)),
            "child_count" | "childCount" => return Ok(self.child_count.into_py(py)),
            "accessible_name" | "accessibleName" => {
                return Ok(self.accessible_name.clone().into_py(py))
            }
            "accessible_description" | "accessibleDescription" => {
                return Ok(self.accessible_description.clone().into_py(py))
            }
            _ => {}
        }

        // Try component-specific properties from JSON
        if let Ok(props) = serde_json::from_str::<serde_json::Value>(&self.properties_json) {
            if let Some(value) = props.get(property_name) {
                return json_to_py(py, value);
            }
        }

        Ok(py.None())
    }

    /// Get all properties as a dictionary
    pub fn get_all_properties(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new(py);

        // Add standard properties
        dict.set_item("name", self.name.clone())?;
        dict.set_item("text", self.text.clone())?;
        dict.set_item("title", self.title.clone())?;
        dict.set_item("tooltip", self.tooltip.clone())?;
        dict.set_item("enabled", self.enabled)?;
        dict.set_item("visible", self.visible)?;
        dict.set_item("showing", self.showing)?;
        dict.set_item("focused", self.focused)?;
        dict.set_item("focusable", self.focusable)?;
        dict.set_item("selected", self.selected)?;
        dict.set_item("editable", self.editable)?;
        dict.set_item("class_name", self.class_name.clone())?;
        dict.set_item("simple_name", self.simple_name.clone())?;
        dict.set_item("base_type", self.base_type.clone())?;
        dict.set_item("x", self.x)?;
        dict.set_item("y", self.y)?;
        dict.set_item("width", self.width)?;
        dict.set_item("height", self.height)?;
        dict.set_item("hash_code", self.hash_code)?;
        dict.set_item("tree_path", self.tree_path.clone())?;
        dict.set_item("depth", self.depth)?;
        dict.set_item("child_count", self.child_count)?;
        dict.set_item("accessible_name", self.accessible_name.clone())?;
        dict.set_item("accessible_description", self.accessible_description.clone())?;

        // Add component-specific properties
        if let Ok(props) = serde_json::from_str::<serde_json::Value>(&self.properties_json) {
            if let Some(obj) = props.as_object() {
                for (key, value) in obj {
                    if let Ok(py_value) = json_to_py(py, value) {
                        dict.set_item(key, py_value)?;
                    }
                }
            }
        }

        Ok(dict.into())
    }

    /// Get a string representation of this element
    fn __repr__(&self) -> String {
        let identifier = self
            .best_identifier()
            .unwrap_or_else(|| self.tree_path.clone());
        format!(
            "<SwingElement {}[{}] '{}'>",
            self.simple_name, self.tree_path, identifier
        )
    }

    /// Get a string representation for str()
    fn __str__(&self) -> String {
        self.__repr__()
    }

    /// Check equality with another element
    fn __eq__(&self, other: &SwingElement) -> bool {
        self.hash_code == other.hash_code && self.tree_path == other.tree_path
    }

    /// Get hash for the element (for use in sets/dicts)
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash_code.hash(&mut hasher);
        self.tree_path.hash(&mut hasher);
        hasher.finish()
    }

    /// Convert to a dictionary representation
    pub fn to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.get_all_properties(py)
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> PyResult<String> {
        let props = ElementDict {
            hash_code: self.hash_code,
            tree_path: self.tree_path.clone(),
            depth: self.depth,
            class_name: self.class_name.clone(),
            simple_name: self.simple_name.clone(),
            base_type: self.base_type.clone(),
            name: self.name.clone(),
            text: self.text.clone(),
            title: self.title.clone(),
            tooltip: self.tooltip.clone(),
            enabled: self.enabled,
            visible: self.visible,
            showing: self.showing,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        };
        serde_json::to_string_pretty(&props)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }
}

/// Helper struct for JSON serialization
#[derive(serde::Serialize)]
struct ElementDict {
    hash_code: i64,
    tree_path: String,
    depth: u32,
    class_name: String,
    simple_name: String,
    base_type: String,
    name: Option<String>,
    text: Option<String>,
    title: Option<String>,
    tooltip: Option<String>,
    enabled: bool,
    visible: bool,
    showing: bool,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl SwingElement {
    /// Create from a UIComponent
    pub fn from_component(component: &UIComponent) -> Self {
        let properties_json = match &component.properties {
            ComponentProperties::Button(props) => serde_json::to_string(props).unwrap_or_default(),
            ComponentProperties::TextField(props) => {
                serde_json::to_string(props).unwrap_or_default()
            }
            ComponentProperties::ComboBox(props) => {
                serde_json::to_string(props).unwrap_or_default()
            }
            ComponentProperties::List(props) => serde_json::to_string(props).unwrap_or_default(),
            ComponentProperties::Table(props) => serde_json::to_string(props).unwrap_or_default(),
            ComponentProperties::Tree(props) => serde_json::to_string(props).unwrap_or_default(),
            ComponentProperties::Slider(props) => serde_json::to_string(props).unwrap_or_default(),
            ComponentProperties::ProgressBar(props) => {
                serde_json::to_string(props).unwrap_or_default()
            }
            ComponentProperties::TabbedPane(props) => {
                serde_json::to_string(props).unwrap_or_default()
            }
            ComponentProperties::Spinner(props) => serde_json::to_string(props).unwrap_or_default(),
            ComponentProperties::Menu(props) => serde_json::to_string(props).unwrap_or_default(),
            ComponentProperties::ScrollPane(props) => {
                serde_json::to_string(props).unwrap_or_default()
            }
            ComponentProperties::SplitPane(props) => {
                serde_json::to_string(props).unwrap_or_default()
            }
            ComponentProperties::Generic(props) => serde_json::to_string(props).unwrap_or_default(),
        };

        Self {
            hash_code: component.id.hash_code,
            tree_path: component.id.tree_path.clone(),
            depth: component.id.depth,
            class_name: component.component_type.class_name.clone(),
            simple_name: component.component_type.simple_name.clone(),
            base_type: format!("{:?}", component.component_type.base_type),
            name: component.identity.name.clone(),
            text: component.identity.text.clone(),
            title: component.identity.title.clone(),
            tooltip: component.identity.tooltip.clone(),
            action_command: component.identity.action_command.clone(),
            internal_name: component.identity.internal_name.clone(),
            x: component.geometry.bounds.x,
            y: component.geometry.bounds.y,
            width: component.geometry.bounds.width,
            height: component.geometry.bounds.height,
            visible: component.state.visible,
            showing: component.state.showing,
            enabled: component.state.enabled,
            focusable: component.state.focusable,
            focused: component.state.focused,
            selected: component.state.selected,
            editable: component.state.editable,
            accessible_name: component.accessibility.accessible_name.clone(),
            accessible_description: component.accessibility.accessible_description.clone(),
            child_count: component.metadata.child_count,
            is_connected: false,
            properties_json,
            children_cache: None,
        }
    }

    /// Create a list of SwingElements from children
    pub fn with_children(mut self, children: Vec<SwingElement>) -> Self {
        self.children_cache = Some(children);
        self
    }
}

/// Convert a JSON value to a Python object
fn json_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok(b.into_py(py)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_py(py))
            } else {
                Ok(py.None())
            }
        }
        serde_json::Value::String(s) => Ok(s.into_py(py)),
        serde_json::Value::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(json_to_py(py, item)?)?;
            }
            Ok(list.into())
        }
        serde_json::Value::Object(obj) => {
            let dict = PyDict::new(py);
            for (key, val) in obj {
                dict.set_item(key, json_to_py(py, val)?)?;
            }
            Ok(dict.into())
        }
    }
}
