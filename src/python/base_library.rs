//! JavaGuiLibrary - Unified Robot Framework keyword class for Java GUI automation
//!
//! This module provides the base JavaGuiLibrary class that supports Swing, SWT, and RCP
//! automation through a unified API. The legacy SwingLibrary, SwtLibrary, and RcpLibrary
//! classes delegate to this implementation.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use super::swt_element::SwtElement;
use super::exceptions::SwingError;

/// Configuration for the unified library
#[derive(Clone)]
pub struct UnifiedLibraryConfig {
    /// Mode: "swing", "swt", or "rcp"
    pub mode: String,
    /// Default timeout for wait operations (seconds)
    pub timeout: f64,
    /// Default polling interval for wait operations (seconds)
    pub poll_interval: f64,
    /// Whether to log actions
    pub log_actions: bool,
    /// Screenshot directory
    pub screenshot_directory: String,
}

impl Default for UnifiedLibraryConfig {
    fn default() -> Self {
        Self {
            mode: "swing".to_string(),
            timeout: 10.0,
            poll_interval: 0.5,
            log_actions: true,
            screenshot_directory: ".".to_string(),
        }
    }
}

/// Connection state for the library
pub struct UnifiedConnectionState {
    /// Whether connected to an application
    pub connected: bool,
    /// Application name/identifier
    pub application_name: Option<String>,
    /// Process ID if applicable
    pub pid: Option<u32>,
    /// Host for remote connections
    pub host: Option<String>,
    /// Port for remote connections
    pub port: Option<u16>,
    /// TCP stream for JSON-RPC communication
    pub stream: Option<TcpStream>,
    /// Request ID counter for JSON-RPC
    pub request_id: u64,
}

impl Default for UnifiedConnectionState {
    fn default() -> Self {
        Self {
            connected: false,
            application_name: None,
            pid: None,
            host: None,
            port: None,
            stream: None,
            request_id: 0,
        }
    }
}

/// Convert Python object to f64, handling various number types
pub fn py_to_f64(py: Python<'_>, obj: Option<PyObject>) -> Option<f64> {
    obj.and_then(|o| {
        if let Ok(i) = o.extract::<i64>(py) {
            Some(i as f64)
        } else if let Ok(f) = o.extract::<f64>(py) {
            Some(f)
        } else {
            None
        }
    })
}

/// Unified Robot Framework Java GUI Library
///
/// A high-performance library for automating Java Swing, SWT, and RCP applications
/// through Robot Framework. This is the base implementation that supports all toolkits.
///
/// Example (Robot Framework):
///
/// ```text
/// *** Settings ***
/// Library    JavaGuiLibrary    mode=swing
///
/// *** Test Cases ***
/// Test Login
///     Connect To Application    myapp.jar
///     Input Text    name:username    testuser
///     Click    name:loginButton
///     [Teardown]    Disconnect
/// ```
#[pyclass(name = "JavaGuiLibrary")]
pub struct JavaGuiLibrary {
    /// Library configuration
    pub config: Arc<RwLock<UnifiedLibraryConfig>>,
    /// Connection state
    pub connection: Arc<RwLock<UnifiedConnectionState>>,
    /// Element cache for performance
    pub element_cache: Arc<RwLock<HashMap<String, i64>>>,
}

#[pymethods]
impl JavaGuiLibrary {
    /// Robot Framework library scope - GLOBAL to maintain connection across tests
    #[classattr]
    const ROBOT_LIBRARY_SCOPE: &'static str = "GLOBAL";

    /// Create a new JavaGuiLibrary instance
    ///
    /// | =Argument= | =Description= |
    /// | ``mode`` | Toolkit mode: "swing", "swt", or "rcp". Default ``swing``. |
    /// | ``timeout`` | Default timeout for wait operations in seconds. Default ``10.0``. |
    /// | ``poll_interval`` | Polling interval for wait operations in seconds. Default ``0.5``. |
    /// | ``screenshot_directory`` | Directory for screenshots. Default ``.``. |
    ///
    /// Example:
    /// | =Setting= | =Value= | =Value= |
    /// | Library | JavaGuiLibrary | mode=swing |
    /// | Library | JavaGuiLibrary | mode=swt | timeout=30 |
    /// | Library | JavaGuiLibrary | mode=rcp | |
    #[new]
    #[pyo3(signature = (mode="swing", timeout=10.0, poll_interval=0.5, screenshot_directory="."))]
    pub fn new(mode: &str, timeout: f64, poll_interval: f64, screenshot_directory: &str) -> PyResult<Self> {
        // Validate mode
        let mode_lower = mode.to_lowercase();
        if !["swing", "swt", "rcp"].contains(&mode_lower.as_str()) {
            return Err(SwingError::validation(format!(
                "Invalid mode '{}'. Must be 'swing', 'swt', or 'rcp'",
                mode
            )).into());
        }

        let config = UnifiedLibraryConfig {
            mode: mode_lower,
            timeout,
            poll_interval,
            screenshot_directory: screenshot_directory.to_string(),
            ..Default::default()
        };

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            connection: Arc::new(RwLock::new(UnifiedConnectionState::default())),
            element_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Get the current mode
    pub fn get_mode(&self) -> PyResult<String> {
        let config = self.config.read().map_err(|_| {
            SwingError::connection("Failed to acquire config lock")
        })?;
        Ok(config.mode.clone())
    }

    // ========================
    // Connection Keywords
    // ========================

    /// Connect to a Java GUI application
    ///
    /// Establishes connection to a running application via the Java agent.
    ///
    /// | =Argument= | =Description= |
    /// | ``application`` | Application identifier (name or process ID). |
    /// | ``host`` | Remote host for network connections. Default ``localhost``. |
    /// | ``port`` | Port number for remote connections. Default ``5678`` for Swing, ``5679`` for SWT/RCP. |
    /// | ``timeout`` | Connection timeout in seconds. Default ``30``. |
    ///
    /// Example:
    /// | `Connect To Application` | myapp | | |
    /// | `Connect To Application` | eclipse | localhost | 5679 |
    #[pyo3(signature = (application, host="localhost", port=None, timeout=None))]
    pub fn connect_to_application(
        &self,
        py: Python<'_>,
        application: &str,
        host: &str,
        port: Option<u16>,
        timeout: Option<PyObject>,
    ) -> PyResult<()> {
        // Determine default port based on mode
        let config = self.config.read().map_err(|_| {
            SwingError::connection("Failed to acquire config lock")
        })?;
        let default_port = if config.mode == "swing" { 5678 } else { 5679 };
        let actual_port = port.unwrap_or(default_port);
        let timeout_secs = py_to_f64(py, timeout).unwrap_or(30.0);
        drop(config);

        // Validate input
        if application.is_empty() {
            return Err(SwingError::connection("Application identifier cannot be empty").into());
        }

        // Update connection state
        let mut conn = self.connection.write().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;

        // Establish TCP connection
        let addr = format!("{}:{}", host, actual_port);
        let timeout_duration = Duration::from_secs_f64(timeout_secs);

        use std::net::ToSocketAddrs;
        let socket_addr = addr.to_socket_addrs()
            .map_err(|e| SwingError::connection(format!("Failed to resolve address '{}': {}", addr, e)))?
            .next()
            .ok_or_else(|| SwingError::connection(format!("No addresses found for '{}'", addr)))?;

        let stream = TcpStream::connect_timeout(&socket_addr, timeout_duration)
            .map_err(|e| SwingError::connection(format!("Failed to connect to {}: {}", addr, e)))?;

        stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
        stream.set_write_timeout(Some(Duration::from_secs(30))).ok();
        stream.set_nodelay(true).ok();

        conn.stream = Some(stream);
        conn.connected = true;
        conn.application_name = Some(application.to_string());
        conn.host = Some(host.to_string());
        conn.port = Some(actual_port);
        conn.request_id = 0;

        drop(conn);
        self.clear_element_cache()?;

        // Ping the agent
        let result = self.send_rpc_request("ping", serde_json::json!({}))?;
        if result.as_str() != Some("pong") {
            return Err(SwingError::connection("Agent did not respond to ping").into());
        }

        Ok(())
    }

    /// Disconnect from the current application
    ///
    /// Closes the connection and cleans up resources.
    ///
    /// Example:
    /// | `Disconnect` |
    pub fn disconnect(&self) -> PyResult<()> {
        let mut conn = self.connection.write().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;

        if !conn.connected {
            return Ok(());
        }

        conn.stream = None;
        conn.connected = false;
        conn.application_name = None;
        conn.pid = None;
        conn.host = None;
        conn.port = None;

        drop(conn);
        self.clear_element_cache()?;

        Ok(())
    }

    /// Check if connected to an application
    ///
    /// Returns ``True`` if connected, ``False`` otherwise.
    ///
    /// Example:
    /// | ${connected}= | `Is Connected` |
    pub fn is_connected(&self) -> PyResult<bool> {
        let conn = self.connection.read().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;
        Ok(conn.connected)
    }

    // ========================
    // Unified Click Keywords
    // ========================

    /// Click on an element
    ///
    /// Performs a single mouse click on the element matching the locator.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator (e.g., ``name:okButton``, ``text:OK``). |
    ///
    /// Example:
    /// | `Click` | name:okButton |
    /// | `Click` | text:Submit |
    #[pyo3(signature = (locator))]
    pub fn click(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        self.send_rpc_request("click", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    /// Double-click on an element
    ///
    /// Performs a double mouse click on the element matching the locator.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator (e.g., ``name:listItem``, ``text:file.txt``). |
    ///
    /// Example:
    /// | `Double Click` | name:listItem |
    #[pyo3(signature = (locator))]
    pub fn double_click(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        self.send_rpc_request("doubleClick", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    /// Right-click on an element
    ///
    /// Performs a context (right) mouse click on the element matching the locator.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator. |
    ///
    /// Example:
    /// | `Right Click` | name:treeItem |
    #[pyo3(signature = (locator))]
    pub fn right_click(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        self.send_rpc_request("rightClick", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    // ========================
    // Unified Find Keywords
    // ========================

    /// Find a single element matching the locator
    ///
    /// Returns the element if exactly one match is found.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator. |
    ///
    /// Returns a ``JavaGuiElement`` (or toolkit-specific element type).
    ///
    /// Example:
    /// | ${element}= | `Find Element` | name:okButton |
    #[pyo3(signature = (locator))]
    pub fn find_element(&self, locator: &str) -> PyResult<SwtElement> {
        self.ensure_connected()?;

        let elements = self.find_elements_internal(locator)?;
        if elements.is_empty() {
            return Err(SwingError::element_not_found(locator).into());
        }
        Ok(elements.into_iter().next().unwrap())
    }

    /// Find all elements matching the locator
    ///
    /// Returns a list of all matching elements.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator. |
    ///
    /// Example:
    /// | ${elements}= | `Find Elements` | Button |
    #[pyo3(signature = (locator))]
    pub fn find_elements(&self, py: Python<'_>, locator: &str) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let elements = self.find_elements_internal(locator)?;
        let list = PyList::empty(py);
        for elem in elements {
            list.append(elem.into_py(py))?;
        }
        Ok(list.into())
    }

    // ========================
    // Unified Text Input Keywords
    // ========================

    /// Input text into a text field
    ///
    /// Types text into the element matching the locator.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator. |
    /// | ``text`` | Text to input. |
    /// | ``clear`` | Clear existing text first. Default ``True``. |
    ///
    /// Example:
    /// | `Input Text` | name:searchField | hello world |
    #[pyo3(signature = (locator, text, clear=true))]
    pub fn input_text(&self, locator: &str, text: &str, clear: bool) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        if clear {
            self.send_rpc_request("clearText", serde_json::json!({
                "componentId": component_id
            }))?;
        }

        self.send_rpc_request("typeText", serde_json::json!({
            "componentId": component_id,
            "text": text
        }))?;

        Ok(())
    }

    /// Clear text from a text field
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator. |
    ///
    /// Example:
    /// | `Clear Text` | name:searchField |
    #[pyo3(signature = (locator))]
    pub fn clear_text(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        self.send_rpc_request("clearText", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    /// Get text from an element
    ///
    /// Returns the text content of the element.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator. |
    ///
    /// Example:
    /// | ${text}= | `Get Text` | name:statusLabel |
    #[pyo3(signature = (locator))]
    pub fn get_text(&self, locator: &str) -> PyResult<String> {
        self.ensure_connected()?;

        let element = self.find_element(locator)?;
        Ok(element.text.unwrap_or_default())
    }

    // ========================
    // Unified Selection Keywords
    // ========================

    /// Check a checkbox or toggle button
    ///
    /// Ensures the checkbox/button is checked (selected).
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator. |
    ///
    /// Example:
    /// | `Check` | name:rememberMe |
    #[pyo3(signature = (locator))]
    pub fn check(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        // Get current state
        let result = self.send_rpc_request("getElementProperties", serde_json::json!({
            "componentId": component_id
        }))?;

        let already_checked = result.get("selected").and_then(|v| v.as_bool()).unwrap_or(false);

        if !already_checked {
            self.send_rpc_request("click", serde_json::json!({
                "componentId": component_id
            }))?;
        }

        Ok(())
    }

    /// Uncheck a checkbox or toggle button
    ///
    /// Ensures the checkbox/button is unchecked (deselected).
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator. |
    ///
    /// Example:
    /// | `Uncheck` | name:rememberMe |
    #[pyo3(signature = (locator))]
    pub fn uncheck(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        // Get current state
        let result = self.send_rpc_request("getElementProperties", serde_json::json!({
            "componentId": component_id
        }))?;

        let is_checked = result.get("selected").and_then(|v| v.as_bool()).unwrap_or(false);

        if is_checked {
            self.send_rpc_request("click", serde_json::json!({
                "componentId": component_id
            }))?;
        }

        Ok(())
    }

    /// Select an item from a combo box
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Combo box locator. |
    /// | ``item`` | Item text to select. |
    ///
    /// Example:
    /// | `Select Combo Item` | name:typeCombo | Java Project |
    #[pyo3(signature = (locator, item))]
    pub fn select_combo_item(&self, locator: &str, item: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        self.send_rpc_request("selectItem", serde_json::json!({
            "componentId": component_id,
            "value": item
        }))?;

        Ok(())
    }

    /// Select an item from a list
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | List locator. |
    /// | ``item`` | Item text to select. |
    ///
    /// Example:
    /// | `Select List Item` | name:fileList | README.md |
    #[pyo3(signature = (locator, item))]
    pub fn select_list_item(&self, locator: &str, item: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        self.send_rpc_request("selectItem", serde_json::json!({
            "componentId": component_id,
            "value": item
        }))?;

        Ok(())
    }

    // ========================
    // Unified Table Keywords
    // ========================

    /// Get table row count
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table locator. |
    ///
    /// Example:
    /// | ${count}= | `Get Table Row Count` | name:resultsTable |
    #[pyo3(signature = (locator))]
    pub fn get_table_row_count(&self, locator: &str) -> PyResult<i32> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        let response = self.send_rpc_request("getTableRowCount", serde_json::json!({
            "componentId": component_id
        }))?;

        if let Some(count) = response.get("result").and_then(|v| v.as_i64()) {
            return Ok(count as i32);
        }
        if let Some(count) = response.as_i64() {
            return Ok(count as i32);
        }
        Ok(0)
    }

    /// Get table cell value
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table locator. |
    /// | ``row`` | Row index (0-based). |
    /// | ``col`` | Column index (0-based). |
    ///
    /// Example:
    /// | ${value}= | `Get Table Cell Value` | name:resultsTable | 0 | 1 |
    #[pyo3(signature = (locator, row, col))]
    pub fn get_table_cell_value(&self, locator: &str, row: i32, col: i32) -> PyResult<String> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        let response = self.send_rpc_request("getTableCellValue", serde_json::json!({
            "componentId": component_id,
            "row": row,
            "column": col
        }))?;

        if let Some(result) = response.get("result") {
            if let Some(value) = result.as_str() {
                return Ok(value.to_string());
            }
        }
        if let Some(value) = response.as_str() {
            return Ok(value.to_string());
        }
        Ok(String::new())
    }

    /// Select a table row
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table locator. |
    /// | ``row`` | Row index (0-based). |
    ///
    /// Example:
    /// | `Select Table Row` | name:resultsTable | 0 |
    #[pyo3(signature = (locator, row))]
    pub fn select_table_row(&self, locator: &str, row: i32) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        self.send_rpc_request("selectTableRow", serde_json::json!({
            "componentId": component_id,
            "row": row
        }))?;

        Ok(())
    }

    // ========================
    // Unified Tree Keywords
    // ========================

    /// Expand a tree node
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree locator. |
    /// | ``path`` | Node path with ``|`` separators (e.g., ``Project|src|main``). |
    ///
    /// Example:
    /// | `Expand Tree Node` | name:projectTree | MyProject|src |
    #[pyo3(signature = (locator, path))]
    pub fn expand_tree_node(&self, locator: &str, path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        self.send_rpc_request("expandTreeNode", serde_json::json!({
            "componentId": component_id,
            "path": path
        }))?;

        Ok(())
    }

    /// Collapse a tree node
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree locator. |
    /// | ``path`` | Node path with ``|`` separators. |
    ///
    /// Example:
    /// | `Collapse Tree Node` | name:projectTree | MyProject|src |
    #[pyo3(signature = (locator, path))]
    pub fn collapse_tree_node(&self, locator: &str, path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        self.send_rpc_request("collapseTreeNode", serde_json::json!({
            "componentId": component_id,
            "path": path
        }))?;

        Ok(())
    }

    /// Select a tree node
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree locator. |
    /// | ``path`` | Node path with ``|`` separators. |
    ///
    /// Example:
    /// | `Select Tree Node` | name:projectTree | MyProject|src|Main.java |
    #[pyo3(signature = (locator, path))]
    pub fn select_tree_node(&self, locator: &str, path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_element_id(locator)?;

        self.send_rpc_request("selectTreeNode", serde_json::json!({
            "componentId": component_id,
            "path": path
        }))?;

        Ok(())
    }

    // ========================
    // Unified Wait Keywords
    // ========================

    /// Wait until element exists
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator. |
    /// | ``timeout`` | Maximum wait time in seconds. Uses library default if not set. |
    ///
    /// Example:
    /// | ${element}= | `Wait Until Element Exists` | name:progressDialog |
    #[pyo3(signature = (locator, timeout=None))]
    pub fn wait_until_element_exists(
        &self,
        py: Python<'_>,
        locator: &str,
        timeout: Option<PyObject>,
    ) -> PyResult<SwtElement> {
        self.ensure_connected()?;

        let config = self.config.read().map_err(|_| {
            SwingError::connection("Failed to acquire config lock")
        })?;

        let timeout_secs = py_to_f64(py, timeout).unwrap_or(config.timeout);
        let poll_secs = config.poll_interval;
        drop(config);

        let start = Instant::now();
        let timeout_duration = Duration::from_secs_f64(timeout_secs);
        let poll_duration = Duration::from_secs_f64(poll_secs);

        loop {
            self.clear_element_cache()?;

            match self.find_elements_internal(locator) {
                Ok(elements) if !elements.is_empty() => {
                    return Ok(elements.into_iter().next().unwrap());
                }
                _ => {}
            }

            if start.elapsed() >= timeout_duration {
                return Err(SwingError::timeout(
                    format!("wait for element '{}'", locator),
                    timeout_secs,
                ).into());
            }

            std::thread::sleep(poll_duration);
        }
    }

    /// Wait until element is enabled
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator. |
    /// | ``timeout`` | Maximum wait time in seconds. |
    ///
    /// Example:
    /// | ${element}= | `Wait Until Element Is Enabled` | name:submitButton |
    #[pyo3(signature = (locator, timeout=None))]
    pub fn wait_until_element_is_enabled(
        &self,
        py: Python<'_>,
        locator: &str,
        timeout: Option<PyObject>,
    ) -> PyResult<SwtElement> {
        let timeout_f64 = py_to_f64(py, timeout);
        self.wait_for_element_condition(locator, timeout_f64, |e| e.enabled, "enabled")
    }

    // ========================
    // Unified Verification Keywords
    // ========================

    /// Verify element is visible
    ///
    /// Fails if the element is not visible.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator. |
    ///
    /// Example:
    /// | `Element Should Be Visible` | name:warningLabel |
    #[pyo3(signature = (locator))]
    pub fn element_should_be_visible(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let element = self.find_element(locator)?;
        if !element.visible {
            return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Element '{}' is not visible",
                locator
            )));
        }
        Ok(())
    }

    /// Verify element is enabled
    ///
    /// Fails if the element is not enabled.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator. |
    ///
    /// Example:
    /// | `Element Should Be Enabled` | name:submitButton |
    #[pyo3(signature = (locator))]
    pub fn element_should_be_enabled(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let element = self.find_element(locator)?;
        if !element.enabled {
            return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Element '{}' is not enabled",
                locator
            )));
        }
        Ok(())
    }

    /// Verify element text matches expected value
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Element locator. |
    /// | ``expected`` | Expected text value. |
    ///
    /// Example:
    /// | `Element Text Should Be` | name:statusLabel | Ready |
    #[pyo3(signature = (locator, expected))]
    pub fn element_text_should_be(&self, locator: &str, expected: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let element = self.find_element(locator)?;
        let actual = element.text.as_deref().unwrap_or("");

        if actual != expected {
            return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Element text '{}' does not match expected '{}'",
                actual, expected
            )));
        }
        Ok(())
    }

    // ========================
    // Configuration Keywords
    // ========================

    /// Set the default timeout
    ///
    /// | =Argument= | =Description= |
    /// | ``timeout`` | Timeout value in seconds. |
    ///
    /// Returns the previous timeout value.
    ///
    /// Example:
    /// | ${old}= | `Set Timeout` | 30 |
    #[pyo3(signature = (timeout))]
    pub fn set_timeout(&self, py: Python<'_>, timeout: PyObject) -> PyResult<f64> {
        let timeout_val = py_to_f64(py, Some(timeout)).ok_or_else(|| {
            SwingError::validation("Invalid timeout value")
        })?;

        let mut config = self.config.write().map_err(|_| {
            SwingError::connection("Failed to acquire config lock")
        })?;

        let old = config.timeout;
        config.timeout = timeout_val;
        Ok(old)
    }
}

// Private implementation methods
impl JavaGuiLibrary {
    /// Ensure we're connected to an application
    pub fn ensure_connected(&self) -> PyResult<()> {
        let conn = self.connection.read().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;

        if !conn.connected {
            let config = self.config.read().map_err(|_| {
                SwingError::connection("Failed to acquire config lock")
            })?;
            return Err(SwingError::connection(format!(
                "Not connected to any {} application",
                config.mode.to_uppercase()
            )).into());
        }
        Ok(())
    }

    /// Send a JSON-RPC request
    pub fn send_rpc_request(&self, method: &str, params: serde_json::Value) -> PyResult<serde_json::Value> {
        let mut conn = self.connection.write().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;

        if !conn.connected {
            return Err(SwingError::connection("Not connected to any application").into());
        }

        conn.request_id += 1;
        let request_id = conn.request_id;

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": request_id
        });

        let request_str = serde_json::to_string(&request).map_err(|e| {
            SwingError::connection(format!("Failed to serialize request: {}", e))
        })?;

        let stream = conn.stream.as_mut().ok_or_else(|| {
            SwingError::connection("No active connection stream")
        })?;

        stream.set_nonblocking(false).ok();
        stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
        stream.set_nodelay(true).ok();

        writeln!(stream, "{}", request_str).map_err(|e| {
            SwingError::connection(format!("Failed to send request: {}", e))
        })?;
        stream.flush().map_err(|e| {
            SwingError::connection(format!("Failed to flush request: {}", e))
        })?;

        // Read response - track JSON depth
        let mut response_bytes = Vec::new();
        let mut depth = 0i32;
        let mut in_string = false;
        let mut escape_next = false;
        let mut started = false;
        let mut byte_buf = [0u8; 1];

        loop {
            match stream.read(&mut byte_buf) {
                Ok(0) => {
                    return Err(SwingError::connection("Connection closed by server").into());
                }
                Ok(_) => {
                    let b = byte_buf[0];
                    let c = b as char;

                    if !started && (c == '\n' || c == '\r' || c == ' ' || c == '\t') {
                        continue;
                    }

                    response_bytes.push(b);

                    if escape_next {
                        escape_next = false;
                        continue;
                    }
                    if c == '\\' && in_string {
                        escape_next = true;
                        continue;
                    }
                    if c == '"' {
                        in_string = !in_string;
                    }
                    if !in_string {
                        if c == '{' {
                            depth += 1;
                            started = true;
                        } else if c == '}' {
                            depth -= 1;
                            if started && depth == 0 {
                                stream.set_read_timeout(Some(Duration::from_millis(100))).ok();
                                let _ = stream.read(&mut byte_buf);
                                if byte_buf[0] == b'\r' {
                                    let _ = stream.read(&mut byte_buf);
                                }
                                stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
                                break;
                            }
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock ||
                             e.kind() == std::io::ErrorKind::TimedOut => {
                    if started && depth == 0 {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(10));
                    continue;
                }
                Err(e) => {
                    return Err(SwingError::connection(format!("Failed to read response: {}", e)).into());
                }
            }
        }

        let response_str = String::from_utf8(response_bytes)
            .map_err(|e| SwingError::connection(format!("Invalid UTF-8: {}", e)))?;

        if response_str.is_empty() {
            return Err(SwingError::connection("Empty response from agent").into());
        }

        let response: serde_json::Value = serde_json::from_str(&response_str).map_err(|e| {
            SwingError::connection(format!("Failed to parse JSON response: {}", e))
        })?;

        if let Some(error) = response.get("error") {
            let code = error.get("code").and_then(|c| c.as_i64()).unwrap_or(-1);
            let message = error.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error");
            return Err(SwingError::connection(format!("RPC error {}: {}", code, message)).into());
        }

        Ok(response.get("result").cloned().unwrap_or(serde_json::Value::Null))
    }

    /// Clear element cache
    pub fn clear_element_cache(&self) -> PyResult<()> {
        let mut cache = self.element_cache.write().map_err(|_| {
            SwingError::connection("Failed to acquire cache lock")
        })?;
        cache.clear();
        Ok(())
    }

    /// Parse locator string
    fn parse_locator(&self, locator: &str) -> (String, String) {
        let locator = locator.trim();

        // Check for type=value or type:value format
        if let Some(eq_pos) = locator.find('=') {
            let type_part = &locator[..eq_pos];
            let value_part = &locator[eq_pos + 1..];
            match type_part {
                "class" | "name" | "text" | "index" | "id" => {
                    return (type_part.to_string(), value_part.to_string());
                }
                _ => {}
            }
        }

        if let Some(colon_pos) = locator.find(':') {
            let type_part = &locator[..colon_pos];
            let value_part = &locator[colon_pos + 1..];
            match type_part {
                "class" | "name" | "text" | "index" | "id" => {
                    return (type_part.to_string(), value_part.to_string());
                }
                _ => {}
            }
        }

        // Check for #name format
        if locator.starts_with('#') {
            return ("name".to_string(), locator[1..].to_string());
        }

        // Default: treat as class name
        let simple_name = locator.split('.').last().unwrap_or(locator);
        ("class".to_string(), simple_name.to_string())
    }

    /// Find elements by locator (internal)
    pub fn find_elements_internal(&self, locator: &str) -> Result<Vec<SwtElement>, SwingError> {
        let (locator_type, value) = self.parse_locator(locator);

        let result = self.send_rpc_request("findWidgets", serde_json::json!({
            "locatorType": locator_type,
            "value": value
        })).map_err(|e| SwingError::element_not_found(format!("Failed to find elements '{}': {}", locator, e)))?;

        let mut elements = Vec::new();
        if let Some(widgets) = result.as_array() {
            for widget in widgets {
                if let Some(elem) = self.json_to_element(widget) {
                    elements.push(elem);
                }
            }
        }

        Ok(elements)
    }

    /// Get element ID by locator
    pub fn get_element_id(&self, locator: &str) -> Result<i64, SwingError> {
        let elements = self.find_elements_internal(locator)?;
        if elements.is_empty() {
            return Err(SwingError::element_not_found(format!(
                "No element found matching: {}",
                locator
            )));
        }
        Ok(elements[0].hash_code)
    }

    /// Convert JSON to element
    fn json_to_element(&self, json: &serde_json::Value) -> Option<SwtElement> {
        let class_name = json.get("class").and_then(|v| v.as_str())
            .or_else(|| json.get("className").and_then(|v| v.as_str()))
            .unwrap_or("Unknown");
        let simple_name = json.get("simpleClass").and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| class_name.split('.').last().unwrap_or(class_name).to_string());

        let hash_code = json.get("id").and_then(|v| v.as_i64())
            .or_else(|| json.get("hashCode").and_then(|v| v.as_i64()))
            .unwrap_or(0);

        Some(SwtElement::new(
            hash_code,
            class_name.to_string(),
            Some(simple_name),
            json.get("name").and_then(|v| v.as_str()).map(String::from),
            json.get("text").and_then(|v| v.as_str()).map(String::from),
            json.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true),
            json.get("visible").and_then(|v| v.as_bool()).unwrap_or(true),
        ))
    }

    /// Wait for element condition
    fn wait_for_element_condition<F>(
        &self,
        locator: &str,
        timeout: Option<f64>,
        condition: F,
        condition_name: &str,
    ) -> PyResult<SwtElement>
    where
        F: Fn(&SwtElement) -> bool,
    {
        let config = self.config.read().map_err(|_| {
            SwingError::connection("Failed to acquire config lock")
        })?;

        let timeout_secs = timeout.unwrap_or(config.timeout);
        let poll_secs = config.poll_interval;
        drop(config);

        let start = Instant::now();
        let timeout_duration = Duration::from_secs_f64(timeout_secs);
        let poll_duration = Duration::from_secs_f64(poll_secs);

        loop {
            self.clear_element_cache()?;

            if let Ok(element) = self.find_element(locator) {
                if condition(&element) {
                    return Ok(element);
                }
            }

            if start.elapsed() >= timeout_duration {
                return Err(SwingError::timeout(
                    format!("wait for element '{}' to be {}", locator, condition_name),
                    timeout_secs,
                ).into());
            }

            std::thread::sleep(poll_duration);
        }
    }

    /// Convert JSON value to Python object
    pub fn json_to_py(&self, py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
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
                    list.append(self.json_to_py(py, item)?)?;
                }
                Ok(list.into())
            }
            serde_json::Value::Object(obj) => {
                let dict = PyDict::new(py);
                for (key, val) in obj {
                    dict.set_item(key, self.json_to_py(py, val)?)?;
                }
                Ok(dict.into())
            }
        }
    }
}

impl Default for JavaGuiLibrary {
    fn default() -> Self {
        Self::new("swing", 10.0, 0.5, ".").unwrap()
    }
}
