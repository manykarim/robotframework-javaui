//! Robot Framework keyword class for Swing automation
//!
//! This module provides the main SwingLibrary class that exposes
//! Robot Framework keywords for automating Java Swing applications.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::locator::{
    SimpleLocator, SimpleLocatorType, CssSelector, XPathExpression,
    AttributeOperator,
    // Pest parser and evaluator for advanced locator support
    parse_locator as pest_parse_locator, Evaluator, MatchContext,
    Locator as ParsedLocator, find_matching_components,
};
use crate::model::{
    SwingBaseType, UIComponent,
    UITree, ComponentState, ComponentType, ComponentId, ComponentIdentity, AccessibilityInfo,
    TraversalMetadata, Bounds,
};

use super::element::SwingElement;
use super::exceptions::SwingError;

/// Configuration for the Swing Library
#[derive(Clone)]
struct LibraryConfig {
    /// Default timeout for wait operations (seconds)
    timeout: f64,
    /// Default polling interval for wait operations (seconds)
    poll_interval: f64,
    /// Whether to log actions
    log_actions: bool,
    /// Screenshot directory
    screenshot_directory: String,
    /// Default screenshot format
    screenshot_format: String,
}

impl Default for LibraryConfig {
    fn default() -> Self {
        Self {
            timeout: 10.0,
            poll_interval: 0.5,
            log_actions: true,
            screenshot_directory: ".".to_string(),
            screenshot_format: "png".to_string(),
        }
    }
}

/// Connection state for the library
struct ConnectionState {
    /// Whether connected to an application
    connected: bool,
    /// Application name/identifier
    application_name: Option<String>,
    /// Process ID if applicable
    pid: Option<u32>,
    /// Host for remote connections
    host: Option<String>,
    /// Port for remote connections
    port: Option<u16>,
    /// TCP stream for JSON-RPC communication
    stream: Option<TcpStream>,
    /// Request ID counter for JSON-RPC
    request_id: u64,
}

impl Default for ConnectionState {
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

impl Clone for ConnectionState {
    fn clone(&self) -> Self {
        // Clone stream by trying to clone the underlying socket
        let stream = self.stream.as_ref().and_then(|s| s.try_clone().ok());
        Self {
            connected: self.connected,
            application_name: self.application_name.clone(),
            pid: self.pid,
            host: self.host.clone(),
            port: self.port,
            stream,
            request_id: self.request_id,
        }
    }
}

/// Robot Framework Swing Library
///
/// A high-performance library for automating Java Swing applications
/// through Robot Framework.
///
/// Example (Robot Framework):
///
/// ```text
/// *** Settings ***
/// Library    SwingLibrary
///
/// *** Test Cases ***
/// Test Login
///     Connect To Application    myapp.jar
///     Input Text    name:username    testuser
///     Input Text    name:password    secret
///     Click Button    text:Login
///     Wait Until Element Exists    name:dashboard
///     [Teardown]    Disconnect From Application
/// ```
#[pyclass(name = "SwingLibrary")]
pub struct SwingLibrary {
    /// Library configuration
    config: Arc<RwLock<LibraryConfig>>,
    /// Connection state
    connection: Arc<RwLock<ConnectionState>>,
    /// Cached UI tree
    ui_tree: Arc<RwLock<Option<UITree>>>,
    /// Element cache for performance
    element_cache: Arc<RwLock<HashMap<String, SwingElement>>>,
}

#[pymethods]
impl SwingLibrary {
    /// Create a new SwingLibrary instance
    ///
    /// Args:
    ///     timeout: Default timeout for wait operations (default: 10.0)
    ///     poll_interval: Polling interval for wait operations (default: 0.5)
    ///     screenshot_directory: Directory for screenshots (default: ".")
    #[new]
    #[pyo3(signature = (timeout=10.0, poll_interval=0.5, screenshot_directory="."))]
    pub fn new(timeout: f64, poll_interval: f64, screenshot_directory: &str) -> Self {
        let config = LibraryConfig {
            timeout,
            poll_interval,
            screenshot_directory: screenshot_directory.to_string(),
            ..Default::default()
        };

        Self {
            config: Arc::new(RwLock::new(config)),
            connection: Arc::new(RwLock::new(ConnectionState::default())),
            ui_tree: Arc::new(RwLock::new(None)),
            element_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // ========================
    // Connection Keywords
    // ========================

    /// Connect to a Java Swing application
    ///
    /// Establishes connection to a running Swing application or launches
    /// a new instance.
    ///
    /// Args:
    ///     application: Path to JAR file, class name, or process identifier
    ///     host: Remote host for network connections (default: localhost)
    ///     port: Port number for remote connections (default: 5678)
    ///     timeout: Connection timeout in seconds (default: 30)
    ///
    /// Example:
    ///     | Connect To Application | myapp.jar |
    ///     | Connect To Application | com.example.MainClass |
    ///     | Connect To Application | pid:12345 |
    ///     | Connect To Application | myapp | host=192.168.1.100 | port=5678 |
    #[pyo3(signature = (application, host="localhost", port=5678, timeout=30.0))]
    pub fn connect_to_application(
        &self,
        application: &str,
        host: &str,
        port: u16,
        timeout: f64,
    ) -> PyResult<()> {
        // Validate input
        if application.is_empty() {
            return Err(SwingError::connection("Application identifier cannot be empty").into());
        }

        // Update connection state
        let mut conn = self.connection.write().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;

        // Establish actual TCP connection to the Java agent
        let addr = format!("{}:{}", host, port);
        let timeout_duration = Duration::from_secs_f64(timeout);

        // Use ToSocketAddrs to resolve hostnames (like "localhost") to IP addresses
        use std::net::ToSocketAddrs;
        let socket_addr = addr.to_socket_addrs()
            .map_err(|e| SwingError::connection(format!("Failed to resolve address '{}': {}", addr, e)))?
            .next()
            .ok_or_else(|| SwingError::connection(format!("No addresses found for '{}'", addr)))?;

        let stream = TcpStream::connect_timeout(
            &socket_addr,
            timeout_duration,
        ).map_err(|e| SwingError::connection(format!("Failed to connect to {}: {}", addr, e)))?;

        // Set stream timeouts
        stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
        stream.set_write_timeout(Some(Duration::from_secs(30))).ok();

        conn.stream = Some(stream);
        conn.connected = true;
        conn.application_name = Some(application.to_string());
        conn.host = Some(host.to_string());
        conn.port = Some(port);
        conn.request_id = 0;

        // Clear caches
        drop(conn);
        self.clear_caches()?;

        // Ping the agent to verify connection works
        let result = self.send_rpc_request("ping", serde_json::json!({}))?;
        if result.as_str() != Some("pong") {
            return Err(SwingError::connection("Agent did not respond to ping").into());
        }

        Ok(())
    }

    /// Disconnect from the current application
    ///
    /// Closes the connection to the Swing application and cleans up resources.
    ///
    /// Example:
    ///     | Disconnect From Application |
    pub fn disconnect_from_application(&self) -> PyResult<()> {
        let mut conn = self.connection.write().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;

        if !conn.connected {
            return Ok(());
        }

        // Close the TCP stream
        conn.stream = None;
        conn.connected = false;
        conn.application_name = None;
        conn.pid = None;
        conn.host = None;
        conn.port = None;

        // Clear caches
        drop(conn);
        self.clear_caches()?;

        Ok(())
    }

    /// Check if connected to an application
    ///
    /// Returns:
    ///     True if connected, False otherwise
    pub fn is_connected(&self) -> PyResult<bool> {
        let conn = self.connection.read().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;
        Ok(conn.connected)
    }

    /// Get connection information
    ///
    /// Returns:
    ///     Dictionary with connection details
    pub fn get_connection_info(&self, py: Python<'_>) -> PyResult<PyObject> {
        let conn = self.connection.read().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;

        let dict = PyDict::new(py);
        dict.set_item("connected", conn.connected)?;
        dict.set_item("application_name", conn.application_name.clone())?;
        dict.set_item("host", conn.host.clone())?;
        dict.set_item("port", conn.port)?;
        dict.set_item("pid", conn.pid)?;

        Ok(dict.into())
    }

    // ========================
    // Element Finding Keywords
    // ========================

    /// Find a single element matching the locator
    ///
    /// Args:
    ///     locator: Element locator (CSS, XPath, or simple syntax)
    ///
    /// Returns:
    ///     SwingElement matching the locator
    ///
    /// Raises:
    ///     ElementNotFoundError: If no element matches
    ///     MultipleElementsFoundError: If multiple elements match
    ///
    /// Example:
    ///     | ${element}= | Find Element | name:saveButton |
    ///     | ${element}= | Find Element | JButton[text="Save"] |
    ///     | ${element}= | Find Element | //JButton[@text='Save'] |
    #[pyo3(signature = (locator))]
    pub fn find_element(&self, locator: &str) -> PyResult<SwingElement> {
        self.ensure_connected()?;

        let elements = self.find_elements_internal(locator)?;

        match elements.len() {
            0 => Err(SwingError::element_not_found(locator).into()),
            1 => Ok(elements.into_iter().next().unwrap()),
            n => Err(SwingError::multiple_elements_found(locator, n).into()),
        }
    }

    /// Find all elements matching the locator
    ///
    /// Args:
    ///     locator: Element locator (CSS, XPath, or simple syntax)
    ///
    /// Returns:
    ///     List of SwingElement objects
    ///
    /// Example:
    ///     | ${buttons}= | Find Elements | JButton |
    ///     | ${fields}= | Find Elements | JTextField:visible |
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

    /// Wait until an element exists in the UI
    ///
    /// Args:
    ///     locator: Element locator
    ///     timeout: Maximum wait time in seconds (default: library timeout)
    ///     poll_interval: Polling interval in seconds (default: library interval)
    ///
    /// Returns:
    ///     SwingElement when found
    ///
    /// Raises:
    ///     TimeoutError: If element not found within timeout
    ///
    /// Example:
    ///     | ${element}= | Wait Until Element Exists | name:loadingComplete | timeout=30 |
    #[pyo3(signature = (locator, timeout=None, poll_interval=None))]
    pub fn wait_until_element_exists(
        &self,
        locator: &str,
        timeout: Option<f64>,
        poll_interval: Option<f64>,
    ) -> PyResult<SwingElement> {
        self.ensure_connected()?;

        let config = self.config.read().map_err(|_| {
            SwingError::connection("Failed to acquire config lock")
        })?;

        let timeout_secs = timeout.unwrap_or(config.timeout);
        let poll_secs = poll_interval.unwrap_or(config.poll_interval);
        drop(config);

        let start = Instant::now();
        let timeout_duration = Duration::from_secs_f64(timeout_secs);
        let poll_duration = Duration::from_secs_f64(poll_secs);

        loop {
            // Clear both caches to get fresh UI state
            self.clear_element_cache()?;
            self.clear_tree_cache()?;

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
                )
                .into());
            }

            std::thread::sleep(poll_duration);
        }
    }

    /// Wait until an element no longer exists
    ///
    /// Args:
    ///     locator: Element locator
    ///     timeout: Maximum wait time in seconds
    ///
    /// Example:
    ///     | Wait Until Element Does Not Exist | name:loadingSpinner |
    #[pyo3(signature = (locator, timeout=None))]
    pub fn wait_until_element_does_not_exist(
        &self,
        locator: &str,
        timeout: Option<f64>,
    ) -> PyResult<()> {
        self.ensure_connected()?;

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

            match self.find_elements_internal(locator) {
                Ok(elements) if elements.is_empty() => return Ok(()),
                Err(_) => return Ok(()),
                _ => {}
            }

            if start.elapsed() >= timeout_duration {
                return Err(SwingError::timeout(
                    format!("wait for element '{}' to disappear", locator),
                    timeout_secs,
                )
                .into());
            }

            std::thread::sleep(poll_duration);
        }
    }

    /// Wait until element is enabled
    ///
    /// Args:
    ///     locator: Element locator
    ///     timeout: Maximum wait time in seconds
    ///
    /// Example:
    ///     | Wait Until Element Is Enabled | name:submitButton |
    #[pyo3(signature = (locator, timeout=None))]
    pub fn wait_until_element_is_enabled(
        &self,
        locator: &str,
        timeout: Option<f64>,
    ) -> PyResult<SwingElement> {
        self.wait_for_element_condition(locator, timeout, |e| e.enabled, "enabled")
    }

    /// Wait until element is visible
    ///
    /// Args:
    ///     locator: Element locator
    ///     timeout: Maximum wait time in seconds
    ///
    /// Example:
    ///     | Wait Until Element Is Visible | name:resultPanel |
    #[pyo3(signature = (locator, timeout=None))]
    pub fn wait_until_element_is_visible(
        &self,
        locator: &str,
        timeout: Option<f64>,
    ) -> PyResult<SwingElement> {
        self.wait_for_element_condition(locator, timeout, |e| e.visible && e.showing, "visible")
    }

    // ========================
    // Interaction Keywords
    // ========================

    /// Click on an element
    ///
    /// Args:
    ///     locator: Element locator
    ///     click_count: Number of clicks (default: 1, use 2 for double-click)
    ///
    /// Example:
    ///     | Click Element | name:okButton |
    ///     | Click Element | name:listItem | click_count=2 |
    #[pyo3(signature = (locator, click_count=1))]
    pub fn click_element(&self, locator: &str, click_count: u32) -> PyResult<()> {
        self.ensure_connected()?;

        // Find the element and get its component ID
        let component_id = self.get_component_id(locator)?;

        // Use RPC to click element with component ID
        if click_count == 2 {
            self.send_rpc_request("doubleClick", serde_json::json!({
                "componentId": component_id
            }))?;
        } else {
            self.send_rpc_request("click", serde_json::json!({
                "componentId": component_id
            }))?;
        }

        Ok(())
    }

    /// Right-click on an element (context click)
    ///
    /// Performs a right-click to open context/popup menus.
    /// Use `select_from_popup_menu` after this to select menu items.
    ///
    /// Args:
    ///     locator: Element locator
    ///
    /// Example:
    ///     | Right Click Element | JTree#fileTree |
    ///     | Select From Popup Menu | Delete |
    #[pyo3(signature = (locator))]
    pub fn right_click_element(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        // Find the element and get its component ID
        let component_id = self.get_component_id(locator)?;

        // Use RPC to right-click element with component ID
        self.send_rpc_request("rightClick", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    /// Click a button by text or locator
    ///
    /// A convenience keyword for clicking buttons.
    ///
    /// Args:
    ///     identifier: Button text or locator
    ///
    /// Example:
    ///     | Click Button | Save |
    ///     | Click Button | name:cancelButton |
    #[pyo3(signature = (identifier))]
    pub fn click_button(&self, identifier: &str) -> PyResult<()> {
        self.ensure_connected()?;

        // Try to find by text first, then by locator
        // Check for common locator patterns: ':' (prefix:value), '[' (attributes), '#' (ID selector), '//' (XPath)
        let locator = if identifier.contains(':') || identifier.contains('[') || identifier.starts_with('#') || identifier.starts_with("//") {
            identifier.to_string()
        } else {
            format!("JButton[text=\"{}\"]", identifier)
        };

        self.click_element(&locator, 1)
    }

    /// Input text into a text field
    ///
    /// Clears existing text and types the new text.
    ///
    /// Args:
    ///     locator: Element locator
    ///     text: Text to input
    ///     clear: Whether to clear existing text first (default: True)
    ///
    /// Example:
    ///     | Input Text | name:username | testuser |
    ///     | Input Text | name:search | new query | clear=${False} |
    #[pyo3(signature = (locator, text, clear=true))]
    pub fn input_text(&self, locator: &str, text: &str, clear: bool) -> PyResult<()> {
        self.ensure_connected()?;

        // Find the element and get its component ID
        let component_id = self.get_component_id(locator)?;

        // Clear existing text if requested
        if clear {
            self.send_rpc_request("clearText", serde_json::json!({
                "componentId": component_id
            }))?;
        }

        // Type text
        self.send_rpc_request("typeText", serde_json::json!({
            "componentId": component_id,
            "text": text
        }))?;

        Ok(())
    }

    /// Clear text from a text field
    ///
    /// Args:
    ///     locator: Element locator
    ///
    /// Example:
    ///     | Clear Text | name:searchField |
    #[pyo3(signature = (locator))]
    pub fn clear_text(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        // Find the element and get its component ID
        let component_id = self.get_component_id(locator)?;

        self.send_rpc_request("clearText", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    /// Select an item from a combo box
    ///
    /// Args:
    ///     locator: Combo box locator
    ///     item: Item to select (text or index)
    ///
    /// Example:
    ///     | Select From Combobox | name:countrySelector | United States |
    ///     | Select From Combobox | name:monthSelector | index:5 |
    #[pyo3(signature = (locator, item))]
    pub fn select_from_combobox(&self, locator: &str, item: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        self.send_rpc_request("selectItem", serde_json::json!({
            "componentId": component_id,
            "value": item
        }))?;

        Ok(())
    }

    /// Check a checkbox
    ///
    /// Args:
    ///     locator: Checkbox locator
    ///
    /// Example:
    ///     | Check Checkbox | name:rememberMe |
    #[pyo3(signature = (locator))]
    pub fn check_checkbox(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        // Get element properties first to see if already checked
        let result = self.send_rpc_request("getElementProperties", serde_json::json!({
            "componentId": component_id
        }))?;

        let already_checked = result.get("selected").and_then(|v| v.as_bool()).unwrap_or(false);

        // Click to check if not already checked
        if !already_checked {
            self.send_rpc_request("click", serde_json::json!({
                "componentId": component_id
            }))?;
        }

        Ok(())
    }

    /// Uncheck a checkbox
    ///
    /// Args:
    ///     locator: Checkbox locator
    ///
    /// Example:
    ///     | Uncheck Checkbox | name:rememberMe |
    #[pyo3(signature = (locator))]
    pub fn uncheck_checkbox(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        // Get element properties first to see if already unchecked
        let result = self.send_rpc_request("getElementProperties", serde_json::json!({
            "componentId": component_id
        }))?;

        let is_checked = result.get("selected").and_then(|v| v.as_bool()).unwrap_or(false);

        // Click to uncheck if currently checked
        if is_checked {
            self.send_rpc_request("click", serde_json::json!({
                "componentId": component_id
            }))?;
        }

        Ok(())
    }

    /// Select a radio button
    ///
    /// Args:
    ///     locator: Radio button locator
    ///
    /// Example:
    ///     | Select Radio Button | name:optionA |
    #[pyo3(signature = (locator))]
    pub fn select_radio_button(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        self.send_rpc_request("click", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    // ========================
    // Tab Keywords
    // ========================

    /// Select a tab in a JTabbedPane by title or index
    ///
    /// Args:
    ///     locator: TabbedPane locator
    ///     tab_identifier: Tab title (string) or index (integer as string)
    ///
    /// Example:
    ///     | Select Tab | JTabbedPane[name='mainTabbedPane'] | Form Input |
    ///     | Select Tab | #mainTabs | 0 |
    ///     | Select Tab | JTabbedPane | Settings |
    #[pyo3(signature = (locator, tab_identifier))]
    pub fn select_tab(&self, locator: &str, tab_identifier: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        // Try to parse as index first
        if let Ok(index) = tab_identifier.parse::<i32>() {
            // Select by index
            self.send_rpc_request("selectItem", serde_json::json!({
                "componentId": component_id,
                "index": index
            }))?;
        } else {
            // Select by tab title
            self.send_rpc_request("selectItem", serde_json::json!({
                "componentId": component_id,
                "value": tab_identifier
            }))?;
        }

        // Clear cache so new tab contents are visible
        self.clear_tree_cache()?;

        Ok(())
    }

    // ========================
    // List Keywords
    // ========================

    /// Select an item from a list
    ///
    /// Args:
    ///     locator: List locator
    ///     item: Item text to select
    ///
    /// Example:
    ///     | Select From List | JList[name='itemList'] | Option A |
    #[pyo3(signature = (locator, item))]
    pub fn select_from_list(&self, locator: &str, item: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        self.send_rpc_request("selectItem", serde_json::json!({
            "componentId": component_id,
            "value": item
        }))?;

        Ok(())
    }

    /// Select a list item by index
    ///
    /// Args:
    ///     locator: List locator
    ///     index: Index of item to select (0-based)
    ///
    /// Example:
    ///     | Select List Item By Index | JList[name='itemList'] | 2 |
    #[pyo3(signature = (locator, index))]
    pub fn select_list_item_by_index(&self, locator: &str, index: i32) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        self.send_rpc_request("selectItem", serde_json::json!({
            "componentId": component_id,
            "index": index
        }))?;

        Ok(())
    }

    /// Get all items from a list
    ///
    /// Args:
    ///     locator: List locator
    ///
    /// Returns:
    ///     List of item strings
    ///
    /// Example:
    ///     | @{items}= | Get List Items | JList[name='itemList'] |
    #[pyo3(signature = (locator))]
    pub fn get_list_items(&self, locator: &str) -> PyResult<Vec<String>> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        let result = self.send_rpc_request("getListItems", serde_json::json!({
            "componentId": component_id
        }))?;

        // Parse the JSON array of items
        let items: Vec<String> = result.as_array()
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect())
            .unwrap_or_default();

        Ok(items)
    }

    // ========================
    // Table Keywords
    // ========================

    /// Get the number of rows in a table
    ///
    /// Args:
    ///     locator: Table locator
    ///
    /// Returns:
    ///     Number of rows
    ///
    /// Example:
    ///     | ${count}= | Get Table Row Count | name:dataTable |
    #[pyo3(signature = (locator))]
    pub fn get_table_row_count(&self, locator: &str) -> PyResult<i32> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        let result = self.send_rpc_request("getTableRowCount", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(result.as_i64().unwrap_or(0) as i32)
    }

    /// Get the number of columns in a table
    ///
    /// Args:
    ///     locator: Table locator
    ///
    /// Returns:
    ///     Number of columns
    ///
    /// Example:
    ///     | ${count}= | Get Table Column Count | name:dataTable |
    #[pyo3(signature = (locator))]
    pub fn get_table_column_count(&self, locator: &str) -> PyResult<i32> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        let result = self.send_rpc_request("getTableColumnCount", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(result.as_i64().unwrap_or(0) as i32)
    }

    /// Get the value of a table cell
    ///
    /// Args:
    ///     locator: Table locator
    ///     row: Row index (0-based)
    ///     column: Column index (0-based) or column name
    ///
    /// Returns:
    ///     Cell value as string
    ///
    /// Example:
    ///     | ${value}= | Get Table Cell Value | name:dataTable | 0 | 1 |
    ///     | ${value}= | Get Table Cell Value | name:dataTable | 2 | Name |
    #[pyo3(signature = (locator, row, column))]
    pub fn get_table_cell_value(&self, locator: &str, row: i32, column: &str) -> PyResult<String> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        // Parse column - could be index or name
        let col_value: serde_json::Value = if let Ok(col_idx) = column.parse::<i32>() {
            serde_json::json!(col_idx)
        } else {
            serde_json::json!(column)
        };

        let result = self.send_rpc_request("getTableCellValue", serde_json::json!({
            "componentId": component_id,
            "row": row,
            "column": col_value
        }))?;

        Ok(result.as_str().unwrap_or("").to_string())
    }

    /// Select a row in a table
    ///
    /// Args:
    ///     locator: Table locator
    ///     row: Row index (0-based)
    ///
    /// Example:
    ///     | Select Table Row | name:dataTable | 3 |
    #[pyo3(signature = (locator, row))]
    pub fn select_table_row(&self, locator: &str, row: i32) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        self.send_rpc_request("selectTableCell", serde_json::json!({
            "componentId": component_id,
            "row": row,
            "column": 0
        }))?;

        Ok(())
    }

    /// Select a cell in a table
    ///
    /// Args:
    ///     locator: Table locator
    ///     row: Row index (0-based)
    ///     column: Column index (0-based)
    ///
    /// Example:
    ///     | Select Table Cell | name:dataTable | 2 | 3 |
    #[pyo3(signature = (locator, row, column))]
    pub fn select_table_cell(&self, locator: &str, row: i32, column: i32) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        self.send_rpc_request("selectTableCell", serde_json::json!({
            "componentId": component_id,
            "row": row,
            "column": column
        }))?;

        Ok(())
    }

    // ========================
    // Tree Keywords
    // ========================

    /// Expand a tree node
    ///
    /// Args:
    ///     locator: Tree locator
    ///     path: Node path (e.g., "Root|Parent|Child")
    ///
    /// Example:
    ///     | Expand Tree Node | name:fileTree | Root|Documents|Reports |
    #[pyo3(signature = (locator, path))]
    pub fn expand_tree_node(&self, locator: &str, path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        self.send_rpc_request("expandTreeNode", serde_json::json!({
            "componentId": component_id,
            "path": path
        }))?;

        Ok(())
    }

    /// Collapse a tree node
    ///
    /// Args:
    ///     locator: Tree locator
    ///     path: Node path
    ///
    /// Example:
    ///     | Collapse Tree Node | name:fileTree | Root|Documents |
    #[pyo3(signature = (locator, path))]
    pub fn collapse_tree_node(&self, locator: &str, path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        self.send_rpc_request("collapseTreeNode", serde_json::json!({
            "componentId": component_id,
            "path": path
        }))?;

        Ok(())
    }

    /// Select a tree node
    ///
    /// Args:
    ///     locator: Tree locator
    ///     path: Node path
    ///
    /// Example:
    ///     | Select Tree Node | name:fileTree | Root|Documents|readme.txt |
    #[pyo3(signature = (locator, path))]
    pub fn select_tree_node(&self, locator: &str, path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        self.send_rpc_request("selectTreeNode", serde_json::json!({
            "componentId": component_id,
            "path": path
        }))?;

        Ok(())
    }

    /// Get the selected tree node path
    ///
    /// Args:
    ///     locator: Tree locator
    ///
    /// Returns:
    ///     Selected node path or None
    ///
    /// Example:
    ///     | ${path}= | Get Selected Tree Node | name:fileTree |
    #[pyo3(signature = (locator))]
    pub fn get_selected_tree_node(&self, locator: &str) -> PyResult<Option<String>> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        let result = self.send_rpc_request("getTreeNodes", serde_json::json!({
            "componentId": component_id,
            "selectedOnly": true
        }))?;

        if let Some(paths) = result.as_array() {
            if let Some(first) = paths.first().and_then(|v| v.as_str()) {
                return Ok(Some(first.to_string()));
            }
        }

        Ok(None)
    }

    /// Get tree data structure
    ///
    /// Returns the tree structure with nodes and children.
    ///
    /// Args:
    ///     locator: Tree locator
    ///
    /// Returns:
    ///     Dictionary with tree structure (text, children)
    ///
    /// Example:
    ///     | ${data}= | Get Tree Data | name:fileTree |
    #[pyo3(signature = (locator))]
    pub fn get_tree_data(&self, py: Python<'_>, locator: &str) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let component_id = self.get_component_id(locator)?;

        let result = self.send_rpc_request("getTreeNodes", serde_json::json!({
            "componentId": component_id
        }))?;

        if result.is_null() {
            return Ok(py.None());
        }

        // Convert serde_json::Value to Python object
        Self::json_to_pyobject(py, result)
    }

    // ========================
    // Menu Keywords
    // ========================

    /// Select a menu item
    ///
    /// Navigates through menu hierarchy and selects the target item.
    ///
    /// Args:
    ///     path: Menu path (e.g., "File|Save As...")
    ///
    /// Example:
    ///     | Select Menu | File|New|Project |
    ///     | Select Menu | Edit|Preferences |
    #[pyo3(signature = (path, timeout=None))]
    pub fn select_menu(&self, path: &str, timeout: Option<i32>) -> PyResult<()> {
        self.ensure_connected()?;

        if path.is_empty() {
            return Err(SwingError::action_failed("select menu", "Empty menu path").into());
        }

        // Use dedicated selectMenu RPC method with optional timeout
        let mut params = serde_json::json!({
            "path": path
        });

        if let Some(timeout_ms) = timeout {
            params["timeout"] = serde_json::json!(timeout_ms);
        }

        self.send_rpc_request("selectMenu", params)?;

        Ok(())
    }

    /// Select a menu item from popup/context menu
    ///
    /// Args:
    ///     path: Menu path
    ///
    /// Example:
    ///     | Select From Popup Menu | Copy |
    ///     | Select From Popup Menu | Edit|Paste Special |
    #[pyo3(signature = (path))]
    pub fn select_from_popup_menu(&self, path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        self.send_rpc_request("selectFromPopupMenu", serde_json::json!({
            "path": path
        }))?;

        Ok(())
    }

    // ========================
    // Inspection Keywords
    // ========================

    /// Get the text content of an element
    ///
    /// Args:
    ///     locator: Element locator
    ///
    /// Returns:
    ///     Text content or empty string
    ///
    /// Example:
    ///     | ${text}= | Get Element Text | name:statusLabel |
    #[pyo3(signature = (locator))]
    pub fn get_element_text(&self, locator: &str) -> PyResult<String> {
        self.ensure_connected()?;

        // Clear cache to get fresh UI state
        self.clear_tree_cache()?;

        // Find the element and return its text property
        let elements = self.find_elements_internal(locator)?;
        if elements.is_empty() {
            return Err(SwingError::element_not_found(format!(
                "No element found matching: {}",
                locator
            )).into());
        }

        Ok(elements[0].text.clone().unwrap_or_default())
    }

    /// Get a specific property of an element
    ///
    /// Args:
    ///     locator: Element locator
    ///     property_name: Name of the property
    ///
    /// Returns:
    ///     Property value or None
    ///
    /// Example:
    ///     | ${enabled}= | Get Element Property | name:button | enabled |
    ///     | ${text}= | Get Element Property | name:field | text |
    #[pyo3(signature = (locator, property_name))]
    pub fn get_element_property(
        &self,
        py: Python<'_>,
        locator: &str,
        property_name: &str,
    ) -> PyResult<PyObject> {
        self.ensure_connected()?;

        // Clear cache to get fresh UI state for dynamic properties
        self.clear_tree_cache()?;

        let component_id = self.get_component_id(locator)?;

        // For dynamic properties, use RPC getProperty call
        // This handles value, selectedIndex, and other runtime properties
        let dynamic_props = ["value", "selectedindex", "minimum", "maximum",
                            "percentcomplete", "tabcount", "rowcount", "columncount",
                            "itemcount", "indeterminate"];

        if dynamic_props.contains(&property_name.to_lowercase().as_str()) {
            let result = self.send_rpc_request("getProperty", serde_json::json!({
                "componentId": component_id,
                "property": property_name
            }))?;

            // Convert JSON result to Python object
            return match result {
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
                _ => Ok(py.None()),
            };
        }

        // For standard properties, use cached element
        let element = self.find_element(locator)?;
        element.get_property(py, property_name)
    }

    /// Verify that an element is enabled
    ///
    /// Args:
    ///     locator: Element locator
    ///
    /// Raises:
    ///     AssertionError: If element is not enabled
    ///
    /// Example:
    ///     | Element Should Be Enabled | name:submitButton |
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

    /// Verify that an element is disabled
    ///
    /// Args:
    ///     locator: Element locator
    ///
    /// Raises:
    ///     AssertionError: If element is enabled
    ///
    /// Example:
    ///     | Element Should Be Disabled | name:submitButton |
    #[pyo3(signature = (locator))]
    pub fn element_should_be_disabled(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let element = self.find_element(locator)?;
        if element.enabled {
            return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Element '{}' is not disabled",
                locator
            )));
        }
        Ok(())
    }

    /// Verify that an element is visible
    ///
    /// Args:
    ///     locator: Element locator
    ///
    /// Raises:
    ///     AssertionError: If element is not visible
    ///
    /// Example:
    ///     | Element Should Be Visible | name:welcomeMessage |
    #[pyo3(signature = (locator))]
    pub fn element_should_be_visible(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let element = self.find_element(locator)?;
        if !element.visible || !element.showing {
            return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Element '{}' is not visible",
                locator
            )));
        }
        Ok(())
    }

    /// Verify that an element is not visible
    ///
    /// Args:
    ///     locator: Element locator
    ///
    /// Raises:
    ///     AssertionError: If element is visible
    ///
    /// Example:
    ///     | Element Should Not Be Visible | name:loadingSpinner |
    #[pyo3(signature = (locator))]
    pub fn element_should_not_be_visible(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        // Element might not exist at all
        match self.find_elements_internal(locator) {
            Ok(elements) if elements.is_empty() => Ok(()),
            Ok(elements) => {
                let element = &elements[0];
                if element.visible && element.showing {
                    Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                        "Element '{}' is visible",
                        locator
                    )))
                } else {
                    Ok(())
                }
            }
            Err(_) => Ok(()),
        }
    }

    /// Verify element text equals expected value
    ///
    /// Args:
    ///     locator: Element locator
    ///     expected: Expected text value
    ///     ignore_case: Ignore case in comparison (default: False)
    ///
    /// Raises:
    ///     AssertionError: If text doesn't match
    ///
    /// Example:
    ///     | Element Text Should Be | name:title | Welcome! |
    #[pyo3(signature = (locator, expected, ignore_case=false))]
    pub fn element_text_should_be(
        &self,
        locator: &str,
        expected: &str,
        ignore_case: bool,
    ) -> PyResult<()> {
        self.ensure_connected()?;

        let element = self.find_element(locator)?;
        let actual = element.text.as_deref().unwrap_or("");

        let matches = if ignore_case {
            actual.to_lowercase() == expected.to_lowercase()
        } else {
            actual == expected
        };

        if !matches {
            return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Element text '{}' does not match expected '{}'",
                actual, expected
            )));
        }
        Ok(())
    }

    /// Verify element text contains expected substring
    ///
    /// Args:
    ///     locator: Element locator
    ///     expected: Expected substring
    ///
    /// Raises:
    ///     AssertionError: If text doesn't contain substring
    ///
    /// Example:
    ///     | Element Text Should Contain | name:message | Success |
    #[pyo3(signature = (locator, expected))]
    pub fn element_text_should_contain(&self, locator: &str, expected: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let element = self.find_element(locator)?;
        let actual = element.text.as_deref().unwrap_or("");

        if !actual.contains(expected) {
            return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Element text '{}' does not contain '{}'",
                actual, expected
            )));
        }
        Ok(())
    }

    // ========================
    // UI Tree Keywords
    // ========================

    /// Get the complete UI tree
    ///
    /// Returns the full UI component hierarchy.
    ///
    /// Args:
    ///     format: Output format (json, xml, text) (default: json)
    ///     max_depth: Maximum depth to traverse (default: unlimited)
    ///     filter: Filter specification (optional)
    ///
    /// Returns:
    ///     UI tree in requested format
    ///
    /// Example:
    ///     | ${tree}= | Get UI Tree |
    ///     | ${tree}= | Get UI Tree | format=xml | max_depth=3 |
    #[pyo3(signature = (format="json", max_depth=None, visible_only=false))]
    pub fn get_ui_tree(
        &self,
        format: &str,
        max_depth: Option<u32>,
        visible_only: bool,
    ) -> PyResult<String> {
        self.ensure_connected()?;

        // Get UI tree with depth control at Java layer for performance
        // This ensures we don't traverse beyond max_depth in the Java agent
        let tree = self.get_or_refresh_tree_with_depth(max_depth)?;

        // Apply additional filtering if needed (visible_only)
        // Note: max_depth filtering is now done at Java layer, so we only filter for visibility here
        let tree = if visible_only {
            self.filter_tree(&tree, None, visible_only)?
        } else {
            tree
        };

        // Format output
        match format.to_lowercase().as_str() {
            "json" => serde_json::to_string_pretty(&tree)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
            "xml" => self.tree_to_xml(&tree),
            "text" => Ok(self.tree_to_text(&tree, 0)),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown format: {}. Use 'json', 'xml', or 'text'",
                format
            ))),
        }
    }

    /// Get the component tree with advanced filtering
    ///
    /// Retrieves the UI component tree with powerful filtering options for
    /// element types and states. This is ideal for debugging, documentation,
    /// or selective tree analysis.
    ///
    /// Args:
    ///     locator: Optional locator to get subtree (default: full tree)
    ///     format: Output format - "json", "xml", "text", "yaml"/"yml", "csv", or "markdown"/"md" (default: "text")
    ///     max_depth: Maximum tree depth to traverse (default: unlimited)
    ///     types: Comma-separated list of types to include (e.g., "JButton,JTextField")
    ///            Supports wildcards: "J*Button" matches JButton, JToggleButton, etc.
    ///     exclude_types: Comma-separated list of types to exclude (takes precedence over types)
    ///     visible_only: Only include visible components (default: False)
    ///     enabled_only: Only include enabled components (default: False)
    ///     focusable_only: Only include focusable components (default: False)
    ///
    /// Returns:
    ///     Component tree in requested format
    ///
    /// Formats:
    ///     - json: Structured JSON with full hierarchy
    ///     - xml: XML format with nested elements
    ///     - yaml/yml: YAML format with hierarchical structure
    ///     - csv: Flattened CSV with path, depth, and properties columns
    ///     - markdown/md: Human-readable Markdown with bullet lists
    ///     - text: Simple indented text representation
    ///
    /// Example:
    ///     | ${tree}= | Get Component Tree |
    ///     | ${tree}= | Get Component Tree | format=json | max_depth=5 |
    ///     | ${tree}= | Get Component Tree | format=yaml |
    ///     | ${tree}= | Get Component Tree | format=csv |
    ///     | ${tree}= | Get Component Tree | format=markdown |
    ///     | ${buttons}= | Get Component Tree | types=JButton | visible_only=True |
    ///     | ${inputs}= | Get Component Tree | types=JTextField,JTextArea | enabled_only=True |
    ///     | ${tree}= | Get Component Tree | types=J*Button | exclude_types=JRadioButton |
    #[pyo3(signature = (
        locator=None,
        format="text",
        max_depth=None,
        types=None,
        exclude_types=None,
        visible_only=false,
        enabled_only=false,
        focusable_only=false
    ))]
    pub fn get_component_tree(
        &self,
        locator: Option<&str>,
        format: &str,
        max_depth: Option<u32>,
        types: Option<&str>,
        exclude_types: Option<&str>,
        visible_only: bool,
        enabled_only: bool,
        focusable_only: bool,
    ) -> PyResult<String> {
        self.ensure_connected()?;

        // Get base tree (full or subtree) with depth control at Java layer for performance
        let tree = if let Some(loc) = locator {
            // Get subtree starting from locator
            let _element = self.find_element(loc)?;
            // For now, we'll get the full tree and filter from there
            // In a full implementation, we'd request a subtree from the agent
            // Use depth control at Java layer if specified
            self.get_or_refresh_tree_with_depth(max_depth)?
        } else {
            // Use depth control at Java layer if specified for performance
            self.get_or_refresh_tree_with_depth(max_depth)?
        };

        // Parse type filters
        let type_list = types.map(|t| {
            t.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
        });

        let exclude_list = exclude_types.map(|t| {
            t.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
        });

        // Validate filter combinations
        self.validate_filters(&type_list, &exclude_list)?;

        // Apply filters
        let filtered = self.filter_tree_with_filters(
            &tree,
            max_depth,
            visible_only,
            type_list,
            exclude_list,
            enabled_only,
            focusable_only,
        )?;

        // Warn if tree is empty after filtering
        if filtered.roots.is_empty() {
            eprintln!(
                "Warning: Filter criteria excluded all components. \
                 Consider adjusting filters: types={:?}, exclude_types={:?}, \
                 visible_only={}, enabled_only={}, focusable_only={}",
                types, exclude_types, visible_only, enabled_only, focusable_only
            );
        }

        // Format output (case-insensitive)
        match format.to_lowercase().as_str() {
            "json" => serde_json::to_string_pretty(&filtered)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
            "xml" => self.tree_to_xml(&filtered),
            "text" => Ok(self.tree_to_text(&filtered, 0)),
            "yaml" | "yml" => serde_yaml::to_string(&filtered)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
            "csv" => self.tree_to_csv(&filtered),
            "markdown" | "md" => Ok(self.tree_to_markdown(&filtered, 0)),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown format: {}. Supported formats: json, xml, text, yaml/yml, csv, markdown/md",
                format
            ))),
        }
    }

    /// Log the UI tree to the Robot Framework log
    ///
    /// Args:
    ///     format: Output format (json, xml, text)
    ///     max_depth: Maximum depth
    ///     level: Log level (INFO, DEBUG, TRACE)
    ///
    /// Example:
    ///     | Log UI Tree |
    ///     | Log UI Tree | format=text | max_depth=2 |
    #[pyo3(signature = (format="text", max_depth=None, level="INFO"))]
    pub fn log_ui_tree(&self, format: &str, max_depth: Option<u32>, level: &str) -> PyResult<()> {
        let tree = self.get_ui_tree(format, max_depth, false)?;

        // In actual implementation, would use Robot Framework's logging
        println!("[{}] UI Tree:\n{}", level, tree);
        Ok(())
    }

    /// Save the UI tree to a file
    ///
    /// Args:
    ///     filepath: Path to save the file
    ///     format: Output format (json, xml, text)
    ///     max_depth: Maximum depth
    ///
    /// Example:
    ///     | Save UI Tree | ${OUTPUT_DIR}/ui_tree.json |
    ///     | Save UI Tree | ${OUTPUT_DIR}/ui_tree.xml | format=xml |
    #[pyo3(signature = (filepath, format="json", max_depth=None))]
    pub fn save_ui_tree(&self, filepath: &str, format: &str, max_depth: Option<u32>) -> PyResult<()> {
        let tree = self.get_ui_tree(format, max_depth, false)?;

        std::fs::write(filepath, tree)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    // ========================
    // Screenshot Keywords
    // ========================

    /// Capture a screenshot
    ///
    /// Args:
    ///     filename: Screenshot filename (optional, auto-generated if not provided)
    ///     locator: Element locator for partial screenshot (optional)
    ///
    /// Returns:
    ///     Path to the saved screenshot
    ///
    /// Example:
    ///     | ${path}= | Capture Screenshot |
    ///     | ${path}= | Capture Screenshot | login_screen.png |
    ///     | ${path}= | Capture Screenshot | locator=name:errorDialog |
    #[pyo3(signature = (filename=None, _locator=None))]
    pub fn capture_screenshot(
        &self,
        filename: Option<&str>,
        _locator: Option<&str>,
    ) -> PyResult<String> {
        self.ensure_connected()?;

        let config = self.config.read().map_err(|_| {
            SwingError::connection("Failed to acquire config lock")
        })?;

        let filename = filename.map(String::from).unwrap_or_else(|| {
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
            format!("screenshot_{}.{}", timestamp, config.screenshot_format)
        });

        let filepath = format!("{}/{}", config.screenshot_directory, filename);
        drop(config);

        // Capture screenshot (actual implementation would capture from JVM)
        // For now, return the path that would be used
        Ok(filepath)
    }

    // ========================
    // Configuration Keywords
    // ========================

    /// Set the default timeout for wait operations
    ///
    /// Args:
    ///     timeout: Timeout in seconds
    ///
    /// Returns:
    ///     Previous timeout value
    ///
    /// Example:
    ///     | ${old}= | Set Timeout | 30 |
    #[pyo3(signature = (timeout))]
    pub fn set_timeout(&self, timeout: f64) -> PyResult<f64> {
        let mut config = self.config.write().map_err(|_| {
            SwingError::connection("Failed to acquire config lock")
        })?;

        let old = config.timeout;
        config.timeout = timeout;
        Ok(old)
    }

    /// Set the screenshot directory
    ///
    /// Args:
    ///     directory: Directory path
    ///
    /// Example:
    ///     | Set Screenshot Directory | ${OUTPUT_DIR}/screenshots |
    #[pyo3(signature = (directory))]
    pub fn set_screenshot_directory(&self, directory: &str) -> PyResult<()> {
        let mut config = self.config.write().map_err(|_| {
            SwingError::connection("Failed to acquire config lock")
        })?;

        config.screenshot_directory = directory.to_string();
        Ok(())
    }

    /// Close all open dialogs
    ///
    /// Closes all visible JDialog instances to recover from stuck dialogs.
    /// This is useful for test cleanup or error recovery.
    ///
    /// Example:
    ///     | Close All Dialogs |
    pub fn close_all_dialogs(&self) -> PyResult<()> {
        self.ensure_connected()?;
        self.send_rpc_request("closeAllDialogs", serde_json::json!({}))?;
        Ok(())
    }

    /// Force close a specific dialog by name
    ///
    /// Args:
    ///     name: The name or title of the dialog to close
    ///
    /// Returns:
    ///     True if the dialog was found and closed, False otherwise
    ///
    /// Example:
    ///     | ${closed}= | Force Close Dialog | aboutDialog |
    ///     | Should Be True | ${closed} |
    #[pyo3(signature = (name))]
    pub fn force_close_dialog(&self, name: &str) -> PyResult<bool> {
        self.ensure_connected()?;
        
        let result = self.send_rpc_request("forceCloseDialog", serde_json::json!({
            "name": name
        }))?;
        
        // Result should be a boolean
        if let Some(closed) = result.as_bool() {
            Ok(closed)
        } else {
            Ok(false)
        }
    }

        /// Refresh the UI tree cache
    ///
    /// Forces a refresh of the cached UI tree.
    ///
    /// Example:
    ///     | Refresh UI Tree |
    pub fn refresh_ui_tree(&self) -> PyResult<()> {
        self.clear_caches()
    }

    // ============================================================================
    // RCP Component Tree Methods (Phase 6)
    // ============================================================================

    /// Get RCP component tree hierarchy (workbench, perspectives, views, editors)
    ///
    /// Returns a hierarchical representation of Eclipse RCP components with their
    /// underlying SWT widgets exposed. This allows all SWT operations to work on
    /// RCP components since RCP is built on top of SWT.
    ///
    /// Args:
    ///     max_depth: Maximum depth for SWT widget trees (default: 5)
    ///     format: Output format (json, text, yaml) (default: json)
    ///
    /// Returns:
    ///     RCP component tree with workbench windows, perspectives, views, and editors
    ///
    /// Example:
    ///     | ${tree}= | Get RCP Component Tree |
    ///     | ${tree}= | Get RCP Component Tree | max_depth=3 | format=text |
    #[pyo3(signature = (max_depth=5, format="json"))]
    pub fn get_rcp_component_tree(&self, max_depth: u32, format: &str) -> PyResult<String> {
        self.ensure_connected()?;

        let params = serde_json::json!({
            "maxDepth": max_depth
        });

        let tree = self.send_rpc_request("rcp.getComponentTree", params)?;

        match format.to_lowercase().as_str() {
            "json" => serde_json::to_string_pretty(&tree)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
            "text" => Ok(self.rcp_tree_to_text(&tree, 0)),
            "yaml" | "yml" => serde_yaml::to_string(&tree)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown format: {}. Supported: json, text, yaml",
                format
            ))),
        }
    }

    /// Get all RCP views with optional SWT widget information
    ///
    /// Args:
    ///     include_swt_widgets: Include underlying SWT widget trees (default: false)
    ///
    /// Returns:
    ///     JSON array of all open views
    ///
    /// Example:
    ///     | ${views}= | Get All RCP Views |
    ///     | ${views}= | Get All RCP Views | include_swt_widgets=True |
    #[pyo3(signature = (include_swt_widgets=false))]
    pub fn get_all_rcp_views(&self, include_swt_widgets: bool) -> PyResult<String> {
        self.ensure_connected()?;

        let params = serde_json::json!({
            "includeSwtWidgets": include_swt_widgets
        });

        let result = self.send_rpc_request("rcp.getAllViews", params)?;
        serde_json::to_string_pretty(&result)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Get all RCP editors with optional SWT widget information
    ///
    /// Args:
    ///     include_swt_widgets: Include underlying SWT widget trees (default: false)
    ///
    /// Returns:
    ///     JSON array of all open editors
    ///
    /// Example:
    ///     | ${editors}= | Get All RCP Editors |
    ///     | ${editors}= | Get All RCP Editors | include_swt_widgets=True |
    #[pyo3(signature = (include_swt_widgets=false))]
    pub fn get_all_rcp_editors(&self, include_swt_widgets: bool) -> PyResult<String> {
        self.ensure_connected()?;

        let params = serde_json::json!({
            "includeSwtWidgets": include_swt_widgets
        });

        let result = self.send_rpc_request("rcp.getAllEditors", params)?;
        serde_json::to_string_pretty(&result)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Get a specific RCP component by path
    ///
    /// Args:
    ///     path: Component path (e.g., "window[0]/page[0]/view[org.example.view]")
    ///     max_depth: Maximum depth for SWT widget tree (default: 3)
    ///
    /// Returns:
    ///     RCP component with SWT widget information
    ///
    /// Example:
    ///     | ${view}= | Get RCP Component | path=window[0]/page[0]/view[org.eclipse.ui.navigator] |
    #[pyo3(signature = (path, max_depth=3))]
    pub fn get_rcp_component(&self, path: &str, max_depth: u32) -> PyResult<String> {
        self.ensure_connected()?;

        let params = serde_json::json!({
            "path": path,
            "maxDepth": max_depth
        });

        let result = self.send_rpc_request("rcp.getComponent", params)?;
        serde_json::to_string_pretty(&result)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }
}

// Private implementation methods
impl SwingLibrary {
    /// Ensure we're connected to an application
    fn ensure_connected(&self) -> PyResult<()> {
        let conn = self.connection.read().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;

        if !conn.connected {
            return Err(SwingError::connection("Not connected to any application").into());
        }
        Ok(())
    }

    /// Convert RCP tree to text format (helper method)
    fn rcp_tree_to_text(&self, tree: &serde_json::Value, indent: usize) -> String {
        let mut text = String::new();
        let spaces = "  ".repeat(indent);

        if let Some(obj) = tree.as_object() {
            // Get type and identifier
            let comp_type = obj.get("type").and_then(|v| v.as_str()).unwrap_or("Unknown");

            // Build identifier from various possible fields
            let identifier = obj.get("title")
                .or(obj.get("name"))
                .or(obj.get("id"))
                .or(obj.get("label"))
                .and_then(|v| v.as_str())
                .unwrap_or("");

            // Format the line
            if identifier.is_empty() {
                text.push_str(&format!("{}{}\n", spaces, comp_type));
            } else {
                text.push_str(&format!("{}{}: {}\n", spaces, comp_type, identifier));
            }

            // Add properties
            if let Some(dirty) = obj.get("dirty").and_then(|v| v.as_bool()) {
                text.push_str(&format!("{}  dirty: {}\n", spaces, dirty));
            }
            if let Some(file_path) = obj.get("filePath").and_then(|v| v.as_str()) {
                text.push_str(&format!("{}  file: {}\n", spaces, file_path));
            }
            if let Some(swt_id) = obj.get("swtShellId").or(obj.get("swtControlId")).and_then(|v| v.as_i64()) {
                text.push_str(&format!("{}  swtId: {}\n", spaces, swt_id));
            }

            // Recursively process children arrays
            for child_key in ["windows", "pages", "views", "editors", "children"] {
                if let Some(children) = obj.get(child_key).and_then(|v| v.as_array()) {
                    for child in children {
                        text.push_str(&self.rcp_tree_to_text(child, indent + 1));
                    }
                }
            }
        }

        text
    }

    /// Convert serde_json::Value to Python object
    fn json_to_pyobject(py: Python<'_>, value: serde_json::Value) -> PyResult<PyObject> {
        match value {
            serde_json::Value::Null => Ok(py.None()),
            serde_json::Value::Bool(b) => Ok(b.to_object(py)),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(i.to_object(py))
                } else if let Some(f) = n.as_f64() {
                    Ok(f.to_object(py))
                } else {
                    Ok(py.None())
                }
            }
            serde_json::Value::String(s) => Ok(s.to_object(py)),
            serde_json::Value::Array(arr) => {
                let list = PyList::empty(py);
                for item in arr {
                    list.append(Self::json_to_pyobject(py, item)?)?;
                }
                Ok(list.to_object(py))
            }
            serde_json::Value::Object(obj) => {
                let dict = PyDict::new(py);
                for (k, v) in obj {
                    dict.set_item(k, Self::json_to_pyobject(py, v)?)?;
                }
                Ok(dict.to_object(py))
            }
        }
    }

    /// Send a JSON-RPC request to the Java agent
    fn send_rpc_request(&self, method: &str, params: serde_json::Value) -> PyResult<serde_json::Value> {
        let mut conn = self.connection.write().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;

        if !conn.connected {
            return Err(SwingError::connection("Not connected to any application").into());
        }

        // Increment and get request ID before borrowing stream
        conn.request_id += 1;
        let request_id = conn.request_id;

        // Build JSON-RPC request
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": request_id
        });

        let request_str = serde_json::to_string(&request).map_err(|e| {
            SwingError::connection(format!("Failed to serialize request: {}", e))
        })?;

        // Now get the stream
        let stream = conn.stream.as_mut().ok_or_else(|| {
            SwingError::connection("No active connection stream")
        })?;

        // Ensure blocking mode with proper timeout
        stream.set_nonblocking(false).ok();  // Force blocking mode
        stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
        stream.set_nodelay(true).ok();  // Disable Nagle's algorithm for responsiveness

        // Send request (line-delimited JSON)
        writeln!(stream, "{}", request_str).map_err(|e| {
            SwingError::connection(format!("Failed to send request: {}", e))
        })?;
        stream.flush().map_err(|e| {
            SwingError::connection(format!("Failed to flush request: {}", e))
        })?;

        // Read response byte by byte tracking JSON depth
        // Java agent sends pretty-printed multi-line JSON, so we can't use read_line
        use std::io::Read;

        let mut response_bytes = Vec::new();
        let mut depth = 0i32;
        let mut in_string = false;
        let mut escape_next = false;
        let mut started = false;
        let mut byte_buf = [0u8; 1];

        loop {
            match stream.read(&mut byte_buf) {
                Ok(0) => {
                    // EOF - break out
                    break;
                }
                Ok(_) => {
                    let b = byte_buf[0];
                    response_bytes.push(b);

                    let c = b as char;
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
                                // JSON complete - break immediately to avoid multi-test hangs
                                // Breaking immediately prevents timeout-based blocking that could
                                // delay subsequent tests in a multi-test run.
                                break;
                            }
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // Non-blocking read would block - continue waiting
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

        // Parse response
        let response: serde_json::Value = serde_json::from_str(&response_str).map_err(|e| {
            SwingError::connection(format!("Failed to parse JSON response: {}", e))
        })?;

        // Check for error
        if let Some(error) = response.get("error") {
            let code = error.get("code").and_then(|c| c.as_i64()).unwrap_or(-1);
            let message = error.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error");
            return Err(SwingError::connection(format!("RPC error {}: {}", code, message)).into());
        }

        // Return result
        Ok(response.get("result").cloned().unwrap_or(serde_json::Value::Null))
    }

    /// Clear all caches
    fn clear_caches(&self) -> PyResult<()> {
        self.clear_element_cache()?;
        self.clear_tree_cache()?;
        Ok(())
    }

    /// Clear element cache
    fn clear_element_cache(&self) -> PyResult<()> {
        let mut cache = self.element_cache.write().map_err(|_| {
            SwingError::connection("Failed to acquire cache lock")
        })?;
        cache.clear();
        Ok(())
    }

    /// Clear tree cache
    fn clear_tree_cache(&self) -> PyResult<()> {
        let mut tree = self.ui_tree.write().map_err(|_| {
            SwingError::connection("Failed to acquire tree lock")
        })?;
        *tree = None;
        Ok(())
    }

    /// Parse locator string into (type, value) for Java agent
    fn parse_locator(&self, locator: &str) -> (String, String) {
        // Handle different locator formats:
        // "#name" -> ("name", "name")
        // "JButton" -> ("class", "JButton")
        // "JButton#btnName" -> ("name", "btnName")  // prioritize name when both present
        // "@text=Login" -> ("text", "Login")
        // "class=javax.swing.JButton" -> ("class", "javax.swing.JButton")
        // "name=myButton" -> ("name", "myButton")

        let locator = locator.trim();

        // Check for @text= prefix
        if locator.starts_with("@text=") {
            return ("text".to_string(), locator[6..].to_string());
        }

        // Check for explicit type=value format
        if let Some(eq_pos) = locator.find('=') {
            let type_part = &locator[..eq_pos];
            let value_part = &locator[eq_pos + 1..];
            match type_part {
                "class" | "name" | "text" | "index" => {
                    return (type_part.to_string(), value_part.to_string());
                }
                _ => {}
            }
        }

        // Check for #name format
        if locator.starts_with('#') {
            return ("name".to_string(), locator[1..].to_string());
        }

        // Check for Class#name format
        if let Some(hash_pos) = locator.find('#') {
            let name_part = &locator[hash_pos + 1..];
            return ("name".to_string(), name_part.to_string());
        }

        // Default: treat as class name
        // Extract simple class name if it's a full class name with package
        let simple_name = locator.split('.').last().unwrap_or(locator);
        ("class".to_string(), simple_name.to_string())
    }

    /// Find elements by locator (internal)
    ///
    /// This method uses the pest parser and evaluator to support:
    /// - CSS-like selectors: JButton, JButton#name, JButton[text='Login']
    /// - Attribute selectors: [name='value'], [text*='contains'], [text^='starts'], [text$='ends']
    /// - Pseudo selectors: :enabled, :disabled, :visible, :hidden, :first-child, :nth-child(n)
    /// - Combinators: > (child), space (descendant), + (adjacent sibling), ~ (general sibling)
    /// - XPath expressions: //JButton, //JButton[@text='Login'], //JButton[1]
    fn find_elements_internal(&self, locator: &str) -> Result<Vec<SwingElement>, SwingError> {
        // Validate empty locator
        if locator.trim().is_empty() {
            return Err(SwingError::element_not_found(
                "Locator cannot be empty".to_string()
            ));
        }

        // Get the component tree
        let tree = self.get_or_refresh_tree()
            .map_err(|_| SwingError::element_not_found(format!("Failed to get component tree for: {}", locator)))?;

        // First, try parsing with the pest parser for advanced selectors
        match pest_parse_locator(locator) {
            Ok(parsed_locator) => {
                // Use the evaluator to find matching components
                self.find_with_evaluator(&tree, &parsed_locator)
            }
            Err(_parse_error) => {
                // Fall back to simple locator parsing for basic formats
                // This handles legacy formats like "name:value", "class:value", "@text=value"
                let (locator_type, value) = self.parse_locator(locator);
                let mut elements = Vec::new();
                self.search_tree_for_elements(&tree, &locator_type, &value, &mut elements);
                Ok(elements)
            }
        }
    }

    /// Find elements using the evaluator with a parsed locator
    fn find_with_evaluator(&self, tree: &UITree, parsed_locator: &ParsedLocator) -> Result<Vec<SwingElement>, SwingError> {
        let evaluator = Evaluator::new();
        let mut all_results = Vec::new();

        // Search each root in the tree
        for root in &tree.roots {
            // Use find_matching_components which supports capture
            let components = find_matching_components(parsed_locator, root, &evaluator);

            // Convert UIComponents to SwingElements
            for component in components {
                all_results.push(self.component_to_swing_element(component));
            }
        }

        Ok(all_results)
    }

    /// Recursively find matching components using the evaluator
    fn find_matching_in_component<'a>(
        &self,
        component: &'a UIComponent,
        locator: &ParsedLocator,
        evaluator: &Evaluator,
        parent: Option<&'a UIComponent>,
        ancestors: Vec<&'a UIComponent>,
        siblings: &[&'a UIComponent],
        sibling_index: usize,
        results: &mut Vec<SwingElement>,
    ) {
        // Create match context with full ancestor chain
        let context = MatchContext::with_ancestors(
            component,
            parent,
            ancestors.clone(),
            siblings.to_vec(),
            sibling_index,
        );

        // Evaluate the locator against this component
        if evaluator.evaluate(locator, component, &context).matches {
            results.push(self.component_to_swing_element(component));
        }

        // Recursively search children - build ancestor chain
        if let Some(ref children) = component.children {
            let child_refs: Vec<&UIComponent> = children.iter().collect();
            // Build new ancestor chain: current component + existing ancestors
            let mut child_ancestors = vec![component];
            child_ancestors.extend(ancestors.iter().copied());

            for (idx, child) in children.iter().enumerate() {
                self.find_matching_in_component(
                    child,
                    locator,
                    evaluator,
                    Some(component),
                    child_ancestors.clone(),
                    &child_refs,
                    idx,
                    results,
                );
            }
        }
    }

    /// Search the component tree for matching elements (legacy fallback)
    fn search_tree_for_elements(
        &self,
        tree: &UITree,
        locator_type: &str,
        value: &str,
        results: &mut Vec<SwingElement>,
    ) {
        for root in &tree.roots {
            self.search_component_for_elements(root, locator_type, value, results);
        }
    }

    /// Recursively search a component and its children (legacy fallback)
    fn search_component_for_elements(
        &self,
        component: &UIComponent,
        locator_type: &str,
        value: &str,
        results: &mut Vec<SwingElement>,
    ) {
        // Check if this component matches
        let matches = match locator_type {
            "name" => component.identity.name.as_ref().map(|n| n == value).unwrap_or(false),
            "class" => {
                component.component_type.simple_name == value
                    || component.component_type.class_name.ends_with(&format!(".{}", value))
                    || component.component_type.class_name == value
            }
            "text" => component.identity.text.as_ref().map(|t| t.contains(value)).unwrap_or(false),
            _ => false,
        };

        if matches {
            results.push(self.component_to_swing_element(component));
        }

        // Search children
        if let Some(children) = &component.children {
            for child in children {
                self.search_component_for_elements(child, locator_type, value, results);
            }
        }
    }

    /// Convert UIComponent to SwingElement
    fn component_to_swing_element(&self, component: &UIComponent) -> SwingElement {
        // Use from_component to properly transfer all properties including selected, editable, etc.
        SwingElement::from_component(component)
    }

    /// Get the component ID (hash_code) for a locator
    /// This finds the first matching element and returns its ID for use in RPC calls
    fn get_component_id(&self, locator: &str) -> Result<i32, SwingError> {
        let elements = self.find_elements_internal(locator)?;
        if elements.is_empty() {
            return Err(SwingError::element_not_found(format!(
                "No element found matching: {}",
                locator
            )));
        }
        // Return the hash_code of the first matching element
        Ok(elements[0].hash_code as i32)
    }

    /// Convert JSON element to SwingElement
    fn json_to_swing_element(&self, json: &serde_json::Value) -> Option<SwingElement> {
        let class_name = json.get("className").and_then(|v| v.as_str()).unwrap_or("Unknown");
        let simple_name = class_name.split('.').last().unwrap_or(class_name).to_string();

        let hash_code = json.get("hashCode").and_then(|v| v.as_i64()).unwrap_or(0);
        let tree_path = json.get("treePath").and_then(|v| v.as_str()).unwrap_or("0").to_string();

        Some(SwingElement::new(
            hash_code,
            tree_path,
            class_name.to_string(),
            Some(simple_name.clone()),
            json.get("name").and_then(|v| v.as_str()).map(String::from),
            json.get("text").and_then(|v| v.as_str()).map(String::from),
            json.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true),
            json.get("visible").and_then(|v| v.as_bool()).unwrap_or(true),
        ))
    }

    /// Find by simple locator
    fn find_by_simple_locator(&self, tree: &UITree, locator: &SimpleLocator) -> Vec<SwingElement> {
        let mut results = Vec::new();

        for component in tree.iter() {
            let matches = match locator.locator_type {
                SimpleLocatorType::Name => component
                    .identity
                    .name
                    .as_ref()
                    .map(|n| n == &locator.value)
                    .unwrap_or(false),
                SimpleLocatorType::InternalName => component
                    .identity
                    .internal_name
                    .as_ref()
                    .map(|n| n == &locator.value)
                    .unwrap_or(false),
                SimpleLocatorType::Text => component
                    .identity
                    .text
                    .as_ref()
                    .map(|t| t == &locator.value)
                    .unwrap_or(false),
                SimpleLocatorType::Tooltip => component
                    .identity
                    .tooltip
                    .as_ref()
                    .map(|t| t == &locator.value)
                    .unwrap_or(false),
                SimpleLocatorType::Class => component.component_type.simple_name == locator.value
                    || component.component_type.class_name == locator.value,
                SimpleLocatorType::Index => {
                    // Index is handled specially
                    false
                }
                SimpleLocatorType::Id => {
                    component.id.tree_path == locator.value
                        || component.id.hash_code.to_string() == locator.value
                }
                SimpleLocatorType::Label => component
                    .identity
                    .label_text
                    .as_ref()
                    .map(|l| l == &locator.value)
                    .unwrap_or(false),
                SimpleLocatorType::AccessibleName => component
                    .accessibility
                    .accessible_name
                    .as_ref()
                    .map(|n| n == &locator.value)
                    .unwrap_or(false),
            };

            if matches {
                results.push(SwingElement::from_component(component));
            }
        }

        results
    }

    /// Find by CSS selector
    fn find_by_css(
        &self,
        tree: &UITree,
        css: &CssSelector,
    ) -> Vec<SwingElement> {
        // Simplified CSS matching implementation
        let mut results = Vec::new();

        if css.segments.is_empty() {
            return results;
        }

        let first_segment = &css.segments[0];

        for component in tree.iter() {
            let type_matches = first_segment.element.is_empty()
                || first_segment.element == "*"
                || component.component_type.simple_name == first_segment.element
                || component
                    .component_type
                    .simple_name
                    .eq_ignore_ascii_case(&first_segment.element);

            let id_matches = first_segment
                .id
                .as_ref()
                .map(|id| {
                    component
                        .identity
                        .internal_name
                        .as_ref()
                        .map(|n| n == id)
                        .unwrap_or(false)
                        || component
                            .identity
                            .name
                            .as_ref()
                            .map(|n| n == id)
                            .unwrap_or(false)
                })
                .unwrap_or(true);

            let attrs_match = first_segment.attributes.iter().all(|attr| {
                self.check_attribute_match(component, &attr.name, &attr.operator, &attr.value)
            });

            let pseudos_match = first_segment.pseudos.iter().all(|pseudo| {
                use crate::locator::PseudoSelector;
                match pseudo {
                    PseudoSelector::Visible => component.state.visible && component.state.showing,
                    PseudoSelector::Hidden => !component.state.visible || !component.state.showing,
                    PseudoSelector::Enabled => component.state.enabled,
                    PseudoSelector::Disabled => !component.state.enabled,
                    PseudoSelector::Selected => component.state.selected.unwrap_or(false),
                    PseudoSelector::Focus => component.state.focused,
                    PseudoSelector::Empty => component.children.as_ref().map(|c| c.is_empty()).unwrap_or(true),
                    _ => true,
                }
            });

            if type_matches && id_matches && attrs_match && pseudos_match {
                results.push(SwingElement::from_component(component));
            }
        }

        results
    }

    /// Find by XPath expression
    fn find_by_xpath(
        &self,
        tree: &UITree,
        xpath: &XPathExpression,
    ) -> Vec<SwingElement> {
        use crate::locator::XPathAxis;

        let mut results = Vec::new();

        if xpath.steps.is_empty() {
            return results;
        }

        // For multi-step XPath like //JPanel//JButton, we traverse the tree
        // tracking the path (ancestors) for each component, then verify
        // that the ancestor chain contains matches for all XPath steps.

        // Collect all components with their ancestors
        let mut components_with_ancestors: Vec<(UIComponent, Vec<UIComponent>)> = Vec::new();

        // Traverse tree and collect components with their ancestor paths
        fn collect_with_ancestors(
            component: &UIComponent,
            ancestors: Vec<UIComponent>,
            result: &mut Vec<(UIComponent, Vec<UIComponent>)>,
        ) {
            // Store this component with its ancestors
            result.push((component.clone(), ancestors.clone()));

            // Process children with updated ancestors
            if let Some(ref children) = component.children {
                let mut new_ancestors = ancestors;
                new_ancestors.push(component.clone());
                for child in children {
                    collect_with_ancestors(child, new_ancestors.clone(), result);
                }
            }
        }

        for root in &tree.roots {
            collect_with_ancestors(root, Vec::new(), &mut components_with_ancestors);
        }

        // For single step XPath, just match directly
        if xpath.steps.len() == 1 {
            let step = &xpath.steps[0];
            for (component, _) in &components_with_ancestors {
                if self.xpath_step_matches(component, step) {
                    results.push(SwingElement::from_component(component));
                }
            }
            return results;
        }

        // For multi-step XPath (e.g., //JPanel//JButton):
        // The final step matches the component, previous steps match ancestors
        let final_step = &xpath.steps[xpath.steps.len() - 1];

        for (component, ancestors) in &components_with_ancestors {
            // Check if component matches final step
            if !self.xpath_step_matches(component, final_step) {
                continue;
            }

            // Verify ancestor chain matches previous steps
            // For //JPanel//JButton:
            // - step 0: JPanel (axis=Descendant, meaning search descendants from root)
            // - step 1: JButton (axis=Descendant, meaning search descendants from JPanel)
            // So we need to find JPanel among ancestors

            let mut valid = true;
            let mut ancestor_idx = ancestors.len(); // Start from immediate parent

            // Work backwards through steps (skip final step already matched)
            for step_idx in (0..xpath.steps.len() - 1).rev() {
                let step = &xpath.steps[step_idx];

                // The axis of the NEXT step tells us how the NEXT step relates to this step
                // If next step has Descendant axis, we can skip ancestors
                // If next step has Child axis, must be immediate parent
                let next_step_axis = xpath.steps[step_idx + 1].axis;
                let is_descendant = matches!(next_step_axis, XPathAxis::Descendant | XPathAxis::DescendantOrSelf);

                // Find matching ancestor
                let mut found = false;

                if is_descendant {
                    // Can be any ancestor - search upward
                    while ancestor_idx > 0 {
                        ancestor_idx -= 1;
                        if self.xpath_step_matches(&ancestors[ancestor_idx], step) {
                            found = true;
                            break;
                        }
                    }
                } else {
                    // Must be immediate parent
                    if ancestor_idx > 0 {
                        ancestor_idx -= 1;
                        if self.xpath_step_matches(&ancestors[ancestor_idx], step) {
                            found = true;
                        }
                    }
                }

                if !found {
                    valid = false;
                    break;
                }
            }

            if valid {
                results.push(SwingElement::from_component(component));
            }
        }

        results
    }

    /// Check if a component matches an XPath step
    fn xpath_step_matches(&self, component: &UIComponent, step: &crate::locator::XPathStep) -> bool {
        use crate::locator::XPathPredicate;

        // Check node test (type name)
        let type_matches = step.node_test.is_empty()
            || step.node_test == "*"
            || component.component_type.simple_name == step.node_test
            || component.component_type.simple_name.eq_ignore_ascii_case(&step.node_test);

        if !type_matches {
            return false;
        }

        // Check predicates
        step.predicates.iter().all(|pred| {
            match pred {
                XPathPredicate::AttributeExists(attr) => {
                    self.component_has_attribute(component, attr)
                }
                XPathPredicate::AttributeEquals(attr, value) => self.check_attribute_match(
                    component,
                    attr,
                    &Some(AttributeOperator::Equals),
                    &Some(value.clone()),
                ),
                XPathPredicate::Contains(attr, value) => self.check_attribute_match(
                    component,
                    attr,
                    &Some(AttributeOperator::Contains),
                    &Some(value.clone()),
                ),
                XPathPredicate::StartsWith(attr, value) => self.check_attribute_match(
                    component,
                    attr,
                    &Some(AttributeOperator::StartsWith),
                    &Some(value.clone()),
                ),
                XPathPredicate::Index(idx) => {
                    component.metadata.sibling_index == (*idx as u32 - 1)
                }
                XPathPredicate::Expression(_) => true,
            }
        })
    }

    /// Check if a component has an attribute
    fn component_has_attribute(&self, component: &UIComponent, attr: &str) -> bool {
        match attr {
            "name" => component.identity.name.is_some(),
            "text" => component.identity.text.is_some(),
            "title" => component.identity.title.is_some(),
            "tooltip" => component.identity.tooltip.is_some(),
            "internalName" | "internal_name" => component.identity.internal_name.is_some(),
            "enabled" => true,
            "visible" => true,
            _ => false,
        }
    }

    /// Check if an attribute matches
    fn check_attribute_match(
        &self,
        component: &UIComponent,
        attr: &str,
        operator: &Option<AttributeOperator>,
        value: &Option<String>,
    ) -> bool {
        // AttributeOperator already imported at module level

        let actual = match attr {
            "name" => component.identity.name.clone(),
            "text" => component.identity.text.clone(),
            "title" => component.identity.title.clone(),
            "tooltip" => component.identity.tooltip.clone(),
            "internalName" | "internal_name" => component.identity.internal_name.clone(),
            "enabled" => Some(component.state.enabled.to_string()),
            "visible" => Some(component.state.visible.to_string()),
            "class" | "className" => Some(component.component_type.simple_name.clone()),
            _ => None,
        };

        match (operator, value, actual) {
            (None, None, Some(_)) => true, // Existence check
            (Some(op), Some(expected), Some(actual)) => match op {
                AttributeOperator::Equals => actual == *expected,
                AttributeOperator::NotEquals => actual != *expected,
                AttributeOperator::Contains => actual.contains(expected.as_str()),
                AttributeOperator::StartsWith => actual.starts_with(expected.as_str()),
                AttributeOperator::EndsWith => actual.ends_with(expected.as_str()),
                AttributeOperator::Matches => {
                    regex::Regex::new(expected)
                        .map(|r| r.is_match(&actual))
                        .unwrap_or(false)
                }
            },
            _ => false,
        }
    }

    /// Validate element for action
    fn validate_element_for_action(&self, element: &SwingElement, action: &str) -> PyResult<()> {
        if !element.enabled {
            return Err(SwingError::action_failed(
                action,
                format!("Element '{}' is not enabled", element.simple_name),
            )
            .into());
        }

        if !element.visible || !element.showing {
            return Err(SwingError::action_failed(
                action,
                format!("Element '{}' is not visible", element.simple_name),
            )
            .into());
        }

        Ok(())
    }

    /// Wait for element condition
    fn wait_for_element_condition<F>(
        &self,
        locator: &str,
        timeout: Option<f64>,
        condition: F,
        condition_name: &str,
    ) -> PyResult<SwingElement>
    where
        F: Fn(&SwingElement) -> bool,
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
                )
                .into());
            }

            std::thread::sleep(poll_duration);
        }
    }

    /// Get or refresh UI tree with optional depth limit
    fn get_or_refresh_tree_with_depth(&self, max_depth: Option<u32>) -> PyResult<UITree> {
        // If max_depth is specified, always fetch fresh to ensure depth limiting happens at Java layer
        if max_depth.is_some() {
            return self.fetch_tree_from_agent(max_depth);
        }

        // Otherwise, use cached tree if available
        let tree_guard = self.ui_tree.read().map_err(|_| {
            SwingError::connection("Failed to acquire tree lock")
        })?;

        if let Some(tree) = tree_guard.clone() {
            return Ok(tree);
        }

        drop(tree_guard);

        self.fetch_tree_from_agent(None)
    }

    /// Get or refresh UI tree (legacy method for backward compatibility)
    fn get_or_refresh_tree(&self) -> PyResult<UITree> {
        self.get_or_refresh_tree_with_depth(None)
    }

    /// Fetch tree from Java agent with optional depth limit
    fn fetch_tree_from_agent(&self, max_depth: Option<u32>) -> PyResult<UITree> {
        // Fetch fresh tree from Java agent via RPC with depth parameter
        let params = if let Some(depth) = max_depth {
            serde_json::json!({
                "maxDepth": depth
            })
        } else {
            serde_json::json!({})
        };

        let result = self.send_rpc_request("getComponentTree", params)?;

        // Convert JSON to UITree
        let tree = self.json_to_ui_tree(&result)?;

        // Cache it only if no depth limit (full tree)
        if max_depth.is_none() {
            let mut tree_guard = self.ui_tree.write().map_err(|_| {
                SwingError::connection("Failed to acquire tree lock")
            })?;
            *tree_guard = Some(tree.clone());
        }

        Ok(tree)
    }

    /// Convert JSON response to UITree
    fn json_to_ui_tree(&self, json: &serde_json::Value) -> PyResult<UITree> {
        let mut tree = UITree::new();

        // Check if result has 'roots' field (from Java agent)
        let roots_json = if let Some(roots) = json.get("roots") {
            roots
        } else {
            json
        };

        // Parse the component tree from JSON response
        if let Some(windows) = roots_json.as_array() {
            for window in windows {
                if let Some(component) = self.json_to_component(window) {
                    tree.roots.push(component);
                }
            }
        } else if roots_json.is_object() {
            // Single component or component with children
            if let Some(component) = self.json_to_component(roots_json) {
                tree.roots.push(component);
            }
        }

        Ok(tree)
    }

    /// Convert JSON to UIComponent
    fn json_to_component(&self, json: &serde_json::Value) -> Option<UIComponent> {
        use crate::model::{ComponentGeometry, ComponentProperties};

        // Java agent uses "class" not "className"
        let class_name = json.get("class").and_then(|v| v.as_str())
            .or_else(|| json.get("className").and_then(|v| v.as_str()))
            .unwrap_or("Unknown");

        // Java agent provides "simpleClass" directly
        let simple_name = json.get("simpleClass").and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| class_name.split('.').last().unwrap_or(class_name).to_string());

        let name = json.get("name").and_then(|v| v.as_str()).map(String::from);
        let text = json.get("text").and_then(|v| v.as_str()).map(String::from);
        // Java agent uses "id" not "hashCode"
        let hash_code = json.get("id").and_then(|v| v.as_i64())
            .or_else(|| json.get("hashCode").and_then(|v| v.as_i64()))
            .unwrap_or(0) as i32;

        // Parse bounds - Java agent puts x, y, width, height at top level
        let bounds = Bounds {
            x: json.get("x").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            y: json.get("y").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            width: json.get("width").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            height: json.get("height").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
        };

        // Parse children recursively
        let children = json.get("children").and_then(|c| c.as_array()).map(|arr| {
            arr.iter()
                .filter_map(|child| self.json_to_component(child))
                .collect()
        });

        let base_type = self.detect_base_type(&simple_name);

        let component = UIComponent {
            id: ComponentId {
                hash_code: hash_code as i64,
                tree_path: format!("{}", hash_code),
                depth: 0,
            },
            component_type: ComponentType {
                class_name: class_name.to_string(),
                simple_name: simple_name.clone(),
                base_type,
                interfaces: Vec::new(),
                class_hierarchy: vec![class_name.to_string()],
            },
            identity: ComponentIdentity {
                name,
                internal_name: json.get("internalName").and_then(|v| v.as_str()).map(String::from),
                text,
                title: json.get("title").and_then(|v| v.as_str()).map(String::from),
                label_text: None,
                tooltip: json.get("tooltip").and_then(|v| v.as_str()).map(String::from),
                action_command: None,
            },
            geometry: ComponentGeometry {
                bounds,
                local_bounds: None,
                preferred_size: None,
                minimum_size: None,
                maximum_size: None,
            },
            state: ComponentState {
                visible: json.get("visible").and_then(|v| v.as_bool()).unwrap_or(true),
                showing: json.get("showing").and_then(|v| v.as_bool()).unwrap_or(true),
                enabled: json.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true),
                focusable: true,
                focused: json.get("focused").and_then(|v| v.as_bool()).unwrap_or(false),
                selected: json.get("selected").and_then(|v| v.as_bool()),
                editable: json.get("editable").and_then(|v| v.as_bool()),
            },
            properties: ComponentProperties::default(),
            accessibility: AccessibilityInfo::default(),
            children,
            parent_id: None,
            metadata: TraversalMetadata::default(),
        };

        Some(component)
    }

    /// Detect base type from simple class name
    fn detect_base_type(&self, simple_name: &str) -> SwingBaseType {
        match simple_name {
            name if name.contains("Button") => SwingBaseType::Button,
            name if name.contains("TextField") || name.contains("TextArea") => SwingBaseType::TextField,
            name if name.contains("Label") => SwingBaseType::Label,
            name if name.contains("ComboBox") => SwingBaseType::ComboBox,
            name if name.contains("Table") => SwingBaseType::Table,
            name if name.contains("Tree") => SwingBaseType::Tree,
            name if name.contains("List") => SwingBaseType::List,
            name if name.contains("CheckBox") => SwingBaseType::CheckBox,
            name if name.contains("RadioButton") => SwingBaseType::RadioButton,
            name if name.contains("Panel") => SwingBaseType::Panel,
            name if name.contains("Frame") => SwingBaseType::Frame,
            name if name.contains("Dialog") => SwingBaseType::Dialog,
            name if name.contains("Menu") => SwingBaseType::Menu,
            name if name.contains("Scroll") => SwingBaseType::ScrollPane,
            name if name.contains("Tab") => SwingBaseType::TabbedPane,
            _ => SwingBaseType::Unknown,
        }
    }

    /// Filter tree by criteria
    fn filter_tree(
        &self,
        tree: &UITree,
        max_depth: Option<u32>,
        visible_only: bool,
    ) -> PyResult<UITree> {
        self.filter_tree_with_filters(
            tree,
            max_depth,
            visible_only,
            None,    // types
            None,    // exclude_types
            false,   // enabled_only
            false,   // focusable_only
        )
    }

    /// Filter tree with advanced type and state filters
    fn filter_tree_with_filters(
        &self,
        tree: &UITree,
        max_depth: Option<u32>,
        visible_only: bool,
        types: Option<Vec<String>>,
        exclude_types: Option<Vec<String>>,
        enabled_only: bool,
        focusable_only: bool,
    ) -> PyResult<UITree> {
        let mut filtered_tree = UITree {
            roots: Vec::new(),
            metadata: tree.metadata.clone(),
            statistics: tree.statistics.clone(),
        };

        // Filter each root component
        for root in &tree.roots {
            if let Some(filtered_root) = self.filter_component(
                root,
                0,
                max_depth,
                visible_only,
                &types,
                &exclude_types,
                enabled_only,
                focusable_only,
            ) {
                filtered_tree.roots.push(filtered_root);
            }
        }

        Ok(filtered_tree)
    }

    /// Recursively filter a component and its children
    fn filter_component(
        &self,
        component: &UIComponent,
        current_depth: u32,
        max_depth: Option<u32>,
        visible_only: bool,
        types: &Option<Vec<String>>,
        exclude_types: &Option<Vec<String>>,
        enabled_only: bool,
        focusable_only: bool,
    ) -> Option<UIComponent> {
        // Check max depth first
        if let Some(max) = max_depth {
            if current_depth >= max {
                return None;
            }
        }

        // Apply state filters
        if visible_only && (!component.state.visible || !component.state.showing) {
            return None;
        }

        if enabled_only && !component.state.enabled {
            return None;
        }

        if focusable_only && !component.state.focusable {
            return None;
        }

        // Apply type filters
        if !self.matches_type_filters(&component.component_type, types, exclude_types) {
            return None;
        }

        // Component matches filters, clone it
        let mut filtered = component.clone();

        // Recursively filter children
        if let Some(children) = &component.children {
            let filtered_children: Vec<UIComponent> = children
                .iter()
                .filter_map(|child| {
                    self.filter_component(
                        child,
                        current_depth + 1,
                        max_depth,
                        visible_only,
                        types,
                        exclude_types,
                        enabled_only,
                        focusable_only,
                    )
                })
                .collect();

            if filtered_children.is_empty() {
                filtered.children = None;
            } else {
                filtered.children = Some(filtered_children);
            }
        }

        Some(filtered)
    }

    /// Check if a component type matches the filter criteria
    fn matches_type_filters(
        &self,
        component_type: &ComponentType,
        types: &Option<Vec<String>>,
        exclude_types: &Option<Vec<String>>,
    ) -> bool {
        // Check exclude list first (takes precedence)
        if let Some(excludes) = exclude_types {
            for pattern in excludes {
                if self.matches_type_pattern(&component_type.simple_name, pattern)
                    || self.matches_type_pattern(&component_type.class_name, pattern)
                {
                    return false;
                }
            }
        }

        // If no include list, accept all (unless excluded above)
        if types.is_none() {
            return true;
        }

        // Check include list
        if let Some(includes) = types {
            for pattern in includes {
                if self.matches_type_pattern(&component_type.simple_name, pattern)
                    || self.matches_type_pattern(&component_type.class_name, pattern)
                {
                    return true;
                }
            }
            return false; // Doesn't match any include pattern
        }

        true
    }

    /// Match a component type against a pattern (supports wildcards)
    fn matches_type_pattern(&self, type_name: &str, pattern: &str) -> bool {
        // Exact match
        if type_name == pattern {
            return true;
        }

        // Wildcard support: J*Button matches JButton, JToggleButton, etc.
        if pattern.contains('*') {
            let regex_pattern = pattern
                .replace(".", "\\.")
                .replace("*", ".*")
                .replace("?", ".");

            if let Ok(re) = regex::Regex::new(&format!("^{}$", regex_pattern)) {
                return re.is_match(type_name);
            }
        }

        // Partial match for convenience (JButton matches javax.swing.JButton)
        if type_name.ends_with(&format!(".{}", pattern)) {
            return true;
        }

        false
    }

    /// Validate filter combinations and provide helpful error messages
    fn validate_filters(
        &self,
        types: &Option<Vec<String>>,
        exclude_types: &Option<Vec<String>>,
    ) -> PyResult<()> {
        // Check for invalid type patterns
        if let Some(type_list) = types {
            for pattern in type_list {
                if pattern.is_empty() {
                    return Err(pyo3::exceptions::PyValueError::new_err(
                        "Empty type pattern in types filter"
                    ));
                }
                // Validate wildcard patterns
                if pattern.contains('*') || pattern.contains('?') {
                    // Try to compile as regex to validate
                    let regex_pattern = pattern
                        .replace(".", "\\.")
                        .replace("*", ".*")
                        .replace("?", ".");
                    if regex::Regex::new(&format!("^{}$", regex_pattern)).is_err() {
                        return Err(pyo3::exceptions::PyValueError::new_err(
                            format!("Invalid wildcard pattern: {}", pattern)
                        ));
                    }
                }
            }
        }

        if let Some(exclude_list) = exclude_types {
            for pattern in exclude_list {
                if pattern.is_empty() {
                    return Err(pyo3::exceptions::PyValueError::new_err(
                        "Empty type pattern in exclude_types filter"
                    ));
                }
            }
        }

        // Warn if both types and exclude_types contain overlapping patterns
        if let (Some(include), Some(exclude)) = (types, exclude_types) {
            for inc_pattern in include {
                for exc_pattern in exclude {
                    if inc_pattern == exc_pattern {
                        eprintln!(
                            "Warning: Type '{}' appears in both types and exclude_types. \
                             It will be excluded (exclude_types takes precedence).",
                            inc_pattern
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /// Convert tree to XML
    fn tree_to_xml(&self, tree: &UITree) -> PyResult<String> {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<uitree>\n");

        for root in &tree.roots {
            self.component_to_xml(&mut xml, root, 1);
        }

        xml.push_str("</uitree>");
        Ok(xml)
    }

    /// Convert component to XML
    fn component_to_xml(&self, xml: &mut String, component: &UIComponent, indent: usize) {
        let spaces = "  ".repeat(indent);
        xml.push_str(&format!(
            "{}<component type=\"{}\" name=\"{}\" text=\"{}\" enabled=\"{}\" visible=\"{}\"",
            spaces,
            component.component_type.simple_name,
            component.identity.name.as_deref().unwrap_or(""),
            component.identity.text.as_deref().unwrap_or(""),
            component.state.enabled,
            component.state.visible
        ));

        if let Some(children) = &component.children {
            if children.is_empty() {
                xml.push_str(" />\n");
            } else {
                xml.push_str(">\n");
                for child in children {
                    self.component_to_xml(xml, child, indent + 1);
                }
                xml.push_str(&format!("{}</component>\n", spaces));
            }
        } else {
            xml.push_str(" />\n");
        }
    }

    /// Convert tree to text
    fn tree_to_text(&self, tree: &UITree, indent: usize) -> String {
        let mut text = String::new();

        for root in &tree.roots {
            self.component_to_text(&mut text, root, indent);
        }

        text
    }

    /// Convert component to text
    fn component_to_text(&self, text: &mut String, component: &UIComponent, indent: usize) {
        let spaces = "  ".repeat(indent);
        let identifier = component
            .identity
            .name
            .as_deref()
            .or(component.identity.text.as_deref())
            .unwrap_or("-");

        text.push_str(&format!(
            "{}[{}] {} ({})\n",
            spaces, component.id.tree_path, component.component_type.simple_name, identifier
        ));

        if let Some(children) = &component.children {
            for child in children {
                self.component_to_text(text, child, indent + 1);
            }
        }
    }

    /// Convert tree to CSV format (flattened hierarchy)
    ///
    /// Columns: path, depth, type, name, text, visible, enabled,
    ///          bounds_x, bounds_y, bounds_width, bounds_height
    fn tree_to_csv(&self, tree: &UITree) -> PyResult<String> {
        let mut csv_buffer = Vec::new();
        {
            let mut writer = csv::Writer::from_writer(&mut csv_buffer);

            // Write header
            writer.write_record(&[
                "path",
                "depth",
                "type",
                "name",
                "text",
                "visible",
                "enabled",
                "bounds_x",
                "bounds_y",
                "bounds_width",
                "bounds_height",
            ]).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

            // Write rows for each component (flattened)
            for root in &tree.roots {
                self.component_to_csv_rows(&mut writer, root, 0)?;
            }

            writer.flush()
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        }

        String::from_utf8(csv_buffer)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Write component and children as CSV rows (recursive)
    fn component_to_csv_rows(
        &self,
        writer: &mut csv::Writer<&mut Vec<u8>>,
        component: &UIComponent,
        depth: usize,
    ) -> PyResult<()> {
        // Extract text - escape quotes and newlines
        let text = component.identity.text.as_deref().unwrap_or("");
        let text_escaped = text.replace('\n', "\\n").replace('\r', "\\r");

        // Extract name
        let name = component.identity.name.as_deref().unwrap_or("");

        // Extract bounds
        let bounds = &component.geometry.bounds;

        // Write row
        writer.write_record(&[
            &component.id.tree_path,
            &depth.to_string(),
            &component.component_type.simple_name,
            name,
            &text_escaped,
            &component.state.visible.to_string(),
            &component.state.enabled.to_string(),
            &bounds.x.to_string(),
            &bounds.y.to_string(),
            &bounds.width.to_string(),
            &bounds.height.to_string(),
        ]).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        // Recursively write children
        if let Some(children) = &component.children {
            for child in children {
                self.component_to_csv_rows(writer, child, depth + 1)?;
            }
        }

        Ok(())
    }

    /// Convert tree to Markdown format
    ///
    /// Uses list syntax for hierarchy with component properties in inline code
    fn tree_to_markdown(&self, tree: &UITree, indent: usize) -> String {
        let mut md = String::from("# UI Component Tree\n\n");

        for root in &tree.roots {
            self.component_to_markdown(&mut md, root, indent);
        }

        md
    }

    /// Convert component to Markdown (recursive)
    fn component_to_markdown(&self, md: &mut String, component: &UIComponent, indent: usize) {
        let list_marker = match indent % 3 {
            0 => "-",
            1 => "*",
            _ => "+",
        };

        let spaces = "  ".repeat(indent);

        // Component identifier
        let identifier = component
            .identity
            .name
            .as_deref()
            .or(component.identity.text.as_deref())
            .unwrap_or("-");

        // Format visibility/state indicators
        let mut badges = Vec::new();
        if component.state.visible {
            badges.push("👁️ visible");
        } else {
            badges.push("🚫 hidden");
        }
        if component.state.enabled {
            badges.push("✅ enabled");
        } else {
            badges.push("❌ disabled");
        }

        // Build markdown line
        md.push_str(&format!(
            "{}{} **{}** `{}` - {}\n",
            spaces,
            list_marker,
            component.component_type.simple_name,
            identifier,
            badges.join(" ")
        ));

        // Add properties table for complex components with important data
        if component.identity.text.is_some() && !component.identity.text.as_ref().unwrap().is_empty() {
            let text = component.identity.text.as_ref().unwrap();
            // Only show text preview if it's meaningful
            if !text.trim().is_empty() && text.trim() != identifier {
                let text_preview = if text.len() > 50 {
                    format!("{}...", &text[..50])
                } else {
                    text.to_string()
                };
                md.push_str(&format!("{}  - *Text:* `{}`\n", spaces, text_preview.replace('\n', "\\n")));
            }
        }

        // Add bounds info for positioned components
        let bounds = &component.geometry.bounds;
        if bounds.width > 0 && bounds.height > 0 {
            md.push_str(&format!(
                "{}  - *Bounds:* `{}×{}` at `({}, {})`\n",
                spaces, bounds.width, bounds.height, bounds.x, bounds.y
            ));
        }

        // Recursively add children
        if let Some(children) = &component.children {
            for child in children {
                self.component_to_markdown(md, child, indent + 1);
            }
        }
    }
}

impl Default for SwingLibrary {
    fn default() -> Self {
        Self::new(10.0, 0.5, ".")
    }
}
