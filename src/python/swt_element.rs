//! SwtElement Python wrapper
//!
//! This module provides a Python-friendly wrapper for interacting with
//! individual SWT widgets.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;

/// Represents a reference to an SWT widget
///
/// This class wraps an SWT widget and provides methods for interaction
/// and property access in Robot Framework tests.
#[pyclass(name = "SwtElement")]
#[derive(Clone)]
pub struct SwtElement {
    /// Widget hash code (unique identifier)
    #[pyo3(get)]
    pub hash_code: i64,

    /// Widget class name (fully qualified)
    #[pyo3(get)]
    pub class_name: String,

    /// Simple class name
    #[pyo3(get)]
    pub simple_name: String,

    /// Widget type category
    #[pyo3(get)]
    pub widget_type: String,

    /// Widget name (from setData with SWT.DATA_NAME key)
    #[pyo3(get)]
    pub name: Option<String>,

    /// Widget text content
    #[pyo3(get)]
    pub text: Option<String>,

    /// Widget tooltip text
    #[pyo3(get)]
    pub tooltip: Option<String>,

    /// Widget bounds - x coordinate
    #[pyo3(get)]
    pub x: i32,

    /// Widget bounds - y coordinate
    #[pyo3(get)]
    pub y: i32,

    /// Widget bounds - width
    #[pyo3(get)]
    pub width: i32,

    /// Widget bounds - height
    #[pyo3(get)]
    pub height: i32,

    /// Whether the widget is visible
    #[pyo3(get)]
    pub visible: bool,

    /// Whether the widget is enabled
    #[pyo3(get)]
    pub enabled: bool,

    /// Whether the widget has focus
    #[pyo3(get)]
    pub focused: bool,

    /// Whether the widget is disposed
    #[pyo3(get)]
    pub disposed: bool,

    /// Parent widget hash code (if any)
    #[pyo3(get)]
    pub parent_hash_code: Option<i64>,

    /// Number of child widgets
    #[pyo3(get)]
    pub child_count: u32,

    /// SWT style bits
    #[pyo3(get)]
    pub style: i32,

    /// Additional properties as JSON
    properties_json: String,
}

#[pymethods]
impl SwtElement {
    /// Create a new SwtElement
    #[new]
    #[pyo3(signature = (
        hash_code,
        class_name,
        simple_name = None,
        name = None,
        text = None,
        enabled = true,
        visible = true
    ))]
    pub fn new(
        hash_code: i64,
        class_name: String,
        simple_name: Option<String>,
        name: Option<String>,
        text: Option<String>,
        enabled: bool,
        visible: bool,
    ) -> Self {
        let simple = simple_name.unwrap_or_else(|| {
            class_name.rsplit('.').next().unwrap_or(&class_name).to_string()
        });
        let widget_type = Self::detect_widget_type(&simple);

        Self {
            hash_code,
            class_name,
            simple_name: simple,
            widget_type,
            name,
            text,
            tooltip: None,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            visible,
            enabled,
            focused: false,
            disposed: false,
            parent_hash_code: None,
            child_count: 0,
            style: 0,
            properties_json: "{}".to_string(),
        }
    }

    /// Get a unique identifier for this widget
    #[getter]
    pub fn id(&self) -> String {
        format!("swt@{}", self.hash_code)
    }

    /// Get the bounds as a tuple (x, y, width, height)
    #[getter]
    pub fn bounds(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.width, self.height)
    }

    /// Get the best identifier for this widget
    #[getter]
    pub fn best_identifier(&self) -> Option<String> {
        self.name
            .clone()
            .or_else(|| self.text.clone())
            .or_else(|| self.tooltip.clone())
    }

    /// Check if the widget is a container type
    pub fn is_container(&self) -> bool {
        matches!(
            self.widget_type.as_str(),
            "Shell" | "Composite" | "Group" | "ScrolledComposite" |
            "SashForm" | "TabFolder" | "CTabFolder" | "ExpandBar"
        )
    }

    /// Check if the widget is a text input type
    pub fn is_text_widget(&self) -> bool {
        matches!(
            self.widget_type.as_str(),
            "Text" | "StyledText" | "Combo" | "Spinner"
        )
    }

    /// Check if the widget is a button type
    pub fn is_button(&self) -> bool {
        matches!(
            self.widget_type.as_str(),
            "Button" | "ToolItem" | "Link"
        )
    }

    /// Check if the widget is a selection type
    pub fn is_selection_widget(&self) -> bool {
        matches!(
            self.widget_type.as_str(),
            "List" | "Table" | "Tree" | "Combo" | "CCombo"
        )
    }

    /// Get a specific property by name
    #[pyo3(signature = (property_name))]
    pub fn get_property(&self, py: Python<'_>, property_name: &str) -> PyResult<PyObject> {
        match property_name {
            "name" => return Ok(self.name.clone().into_py(py)),
            "text" => return Ok(self.text.clone().into_py(py)),
            "tooltip" => return Ok(self.tooltip.clone().into_py(py)),
            "enabled" => return Ok(self.enabled.into_py(py)),
            "visible" => return Ok(self.visible.into_py(py)),
            "focused" => return Ok(self.focused.into_py(py)),
            "disposed" => return Ok(self.disposed.into_py(py)),
            "class_name" | "className" => return Ok(self.class_name.clone().into_py(py)),
            "simple_name" | "simpleName" => return Ok(self.simple_name.clone().into_py(py)),
            "widget_type" | "widgetType" => return Ok(self.widget_type.clone().into_py(py)),
            "x" => return Ok(self.x.into_py(py)),
            "y" => return Ok(self.y.into_py(py)),
            "width" => return Ok(self.width.into_py(py)),
            "height" => return Ok(self.height.into_py(py)),
            "hash_code" | "hashCode" => return Ok(self.hash_code.into_py(py)),
            "style" => return Ok(self.style.into_py(py)),
            "child_count" | "childCount" => return Ok(self.child_count.into_py(py)),
            _ => {}
        }

        // Try additional properties from JSON
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

        dict.set_item("hash_code", self.hash_code)?;
        dict.set_item("class_name", self.class_name.clone())?;
        dict.set_item("simple_name", self.simple_name.clone())?;
        dict.set_item("widget_type", self.widget_type.clone())?;
        dict.set_item("name", self.name.clone())?;
        dict.set_item("text", self.text.clone())?;
        dict.set_item("tooltip", self.tooltip.clone())?;
        dict.set_item("x", self.x)?;
        dict.set_item("y", self.y)?;
        dict.set_item("width", self.width)?;
        dict.set_item("height", self.height)?;
        dict.set_item("visible", self.visible)?;
        dict.set_item("enabled", self.enabled)?;
        dict.set_item("focused", self.focused)?;
        dict.set_item("disposed", self.disposed)?;
        dict.set_item("style", self.style)?;
        dict.set_item("child_count", self.child_count)?;

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

    /// Get a string representation of this widget
    fn __repr__(&self) -> String {
        let identifier = self
            .best_identifier()
            .unwrap_or_else(|| format!("{}", self.hash_code));
        format!(
            "<SwtElement {}[{}] '{}'>",
            self.simple_name, self.hash_code, identifier
        )
    }

    /// Get a string representation for str()
    fn __str__(&self) -> String {
        self.__repr__()
    }

    /// Check equality with another widget
    fn __eq__(&self, other: &SwtElement) -> bool {
        self.hash_code == other.hash_code
    }

    /// Get hash for the widget (for use in sets/dicts)
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash_code.hash(&mut hasher);
        hasher.finish()
    }

    /// Return True if this widget is valid (for bool())
    /// This allows: `if widget:` checks in Python/Robot Framework
    fn __bool__(&self) -> bool {
        !self.disposed && self.hash_code != 0
    }

    /// Return length (for len())
    /// Returns 1 for a valid widget, 0 if disposed
    /// This allows `Should Not Be Empty` to work in Robot Framework
    fn __len__(&self) -> usize {
        if self.disposed || self.hash_code == 0 { 0 } else { 1 }
    }

    /// Convert to a dictionary representation
    pub fn to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.get_all_properties(py)
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> PyResult<String> {
        let props = SwtElementDict {
            hash_code: self.hash_code,
            class_name: self.class_name.clone(),
            simple_name: self.simple_name.clone(),
            widget_type: self.widget_type.clone(),
            name: self.name.clone(),
            text: self.text.clone(),
            tooltip: self.tooltip.clone(),
            enabled: self.enabled,
            visible: self.visible,
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
struct SwtElementDict {
    hash_code: i64,
    class_name: String,
    simple_name: String,
    widget_type: String,
    name: Option<String>,
    text: Option<String>,
    tooltip: Option<String>,
    enabled: bool,
    visible: bool,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl SwtElement {
    /// Detect widget type from simple class name
    fn detect_widget_type(simple_name: &str) -> String {
        // SWT widget types
        match simple_name {
            // Controls
            "Button" => "Button".to_string(),
            "Text" => "Text".to_string(),
            "StyledText" => "StyledText".to_string(),
            "Label" => "Label".to_string(),
            "Combo" => "Combo".to_string(),
            "CCombo" => "CCombo".to_string(),
            "List" => "List".to_string(),
            "Table" => "Table".to_string(),
            "Tree" => "Tree".to_string(),
            "Link" => "Link".to_string(),
            "Scale" => "Scale".to_string(),
            "Slider" => "Slider".to_string(),
            "Spinner" => "Spinner".to_string(),
            "ProgressBar" => "ProgressBar".to_string(),
            "DateTime" => "DateTime".to_string(),
            "Browser" => "Browser".to_string(),
            "Canvas" => "Canvas".to_string(),

            // Containers
            "Shell" => "Shell".to_string(),
            "Composite" => "Composite".to_string(),
            "Group" => "Group".to_string(),
            "TabFolder" => "TabFolder".to_string(),
            "TabItem" => "TabItem".to_string(),
            "CTabFolder" => "CTabFolder".to_string(),
            "CTabItem" => "CTabItem".to_string(),
            "SashForm" => "SashForm".to_string(),
            "ScrolledComposite" => "ScrolledComposite".to_string(),
            "ExpandBar" => "ExpandBar".to_string(),
            "ExpandItem" => "ExpandItem".to_string(),

            // Menus
            "Menu" => "Menu".to_string(),
            "MenuItem" => "MenuItem".to_string(),

            // ToolBar
            "ToolBar" => "ToolBar".to_string(),
            "ToolItem" => "ToolItem".to_string(),
            "CoolBar" => "CoolBar".to_string(),
            "CoolItem" => "CoolItem".to_string(),

            // Table/Tree items
            "TableItem" => "TableItem".to_string(),
            "TableColumn" => "TableColumn".to_string(),
            "TreeItem" => "TreeItem".to_string(),
            "TreeColumn" => "TreeColumn".to_string(),

            // Others
            _ => {
                // Try to categorize based on name patterns
                if simple_name.contains("Button") {
                    "Button".to_string()
                } else if simple_name.contains("Text") {
                    "Text".to_string()
                } else if simple_name.contains("Label") {
                    "Label".to_string()
                } else if simple_name.contains("Combo") {
                    "Combo".to_string()
                } else if simple_name.contains("Table") {
                    "Table".to_string()
                } else if simple_name.contains("Tree") {
                    "Tree".to_string()
                } else if simple_name.contains("List") {
                    "List".to_string()
                } else if simple_name.contains("Shell") {
                    "Shell".to_string()
                } else if simple_name.contains("Composite") || simple_name.contains("View") {
                    "Composite".to_string()
                } else {
                    "Unknown".to_string()
                }
            }
        }
    }

    /// Create from JSON data with all properties
    pub fn from_json(json: &serde_json::Value) -> Option<Self> {
        let class_name = json.get("class").and_then(|v| v.as_str())
            .or_else(|| json.get("className").and_then(|v| v.as_str()))
            .unwrap_or("Unknown");

        let simple_name = json.get("simpleClass").and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| class_name.split('.').last().unwrap_or(class_name).to_string());

        let hash_code = json.get("id").and_then(|v| v.as_i64())
            .or_else(|| json.get("hashCode").and_then(|v| v.as_i64()))
            .unwrap_or(0);

        let mut elem = Self::new(
            hash_code,
            class_name.to_string(),
            Some(simple_name),
            json.get("name").and_then(|v| v.as_str()).map(String::from),
            json.get("text").and_then(|v| v.as_str()).map(String::from),
            json.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true),
            json.get("visible").and_then(|v| v.as_bool()).unwrap_or(true),
        );

        // Set additional properties
        elem.tooltip = json.get("tooltip").and_then(|v| v.as_str()).map(String::from);
        elem.x = json.get("x").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        elem.y = json.get("y").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        elem.width = json.get("width").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        elem.height = json.get("height").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        elem.focused = json.get("focused").and_then(|v| v.as_bool()).unwrap_or(false);
        elem.disposed = json.get("disposed").and_then(|v| v.as_bool()).unwrap_or(false);
        elem.style = json.get("style").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        elem.child_count = json.get("childCount").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        elem.parent_hash_code = json.get("parentId").and_then(|v| v.as_i64());

        Some(elem)
    }

    /// Set additional properties from JSON
    pub fn with_properties(mut self, properties_json: String) -> Self {
        self.properties_json = properties_json;
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
