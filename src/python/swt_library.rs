//! Robot Framework keyword class for SWT automation
//!
//! This module provides the main SwtLibrary class that exposes
//! Robot Framework keywords for automating Eclipse SWT applications.

use pyo3::prelude::*;
use pyo3::types::PyList;
use std::collections::HashMap;
use std::io::{BufRead, Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use super::swt_element::SwtElement;
use super::exceptions::SwingError;

/// Helper function to convert a PyObject (which may be a string or number) to an Option<f64>
/// Robot Framework passes keyword arguments as strings, so we need to handle both cases.
fn py_to_f64(py: Python<'_>, obj: Option<PyObject>) -> Option<f64> {
    obj.and_then(|o| {
        // Try to extract as f64 first (numbers)
        if let Ok(f) = o.extract::<f64>(py) {
            return Some(f);
        }
        // Try to extract as i64 (integers)
        if let Ok(i) = o.extract::<i64>(py) {
            return Some(i as f64);
        }
        // Try to extract as string and parse
        if let Ok(s) = o.extract::<String>(py) {
            if let Ok(f) = s.parse::<f64>() {
                return Some(f);
            }
        }
        None
    })
}

/// Configuration for the SWT Library
#[derive(Clone)]
struct SwtLibraryConfig {
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

impl Default for SwtLibraryConfig {
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

/// Connection state for the SWT library
struct SwtConnectionState {
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

impl Default for SwtConnectionState {
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

impl Clone for SwtConnectionState {
    fn clone(&self) -> Self {
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

/// Robot Framework SWT Library
///
/// A high-performance library for automating Eclipse SWT applications
/// through Robot Framework.
///
/// Example (Robot Framework):
///
/// ```text
/// *** Settings ***
/// Library    SwtLibrary
///
/// *** Test Cases ***
/// Test Eclipse Dialog
///     Connect To SWT Application    eclipse    localhost    5679
///     Activate Shell    text:New Project
///     Input Text    name:projectName    MyProject
///     Click Widget    text:Finish
///     Wait Until Widget Exists    name:projectExplorer
///     [Teardown]    Disconnect
/// ```
#[pyclass(name = "SwtLibrary")]
pub struct SwtLibrary {
    /// Library configuration
    config: Arc<RwLock<SwtLibraryConfig>>,
    /// Connection state
    connection: Arc<RwLock<SwtConnectionState>>,
    /// Element cache for performance
    element_cache: Arc<RwLock<HashMap<String, SwtElement>>>,
}

#[pymethods]
impl SwtLibrary {
    /// Robot Framework library scope - GLOBAL to maintain connection across tests
    #[classattr]
    const ROBOT_LIBRARY_SCOPE: &'static str = "GLOBAL";

    /// Create a new SwtLibrary instance.
    ///
    /// | =Argument= | =Description= |
    /// | ``timeout`` | Default timeout for wait operations in seconds. Default ``10.0``. |
    ///
    /// Example:
    /// | =Setting= | =Value= | =Value= |
    /// | Library | swing_library.SwtLibrary | |
    /// | Library | swing_library.SwtLibrary | timeout=30 |
    #[new]
    #[pyo3(signature = (timeout=None))]
    pub fn new(timeout: Option<f64>) -> Self {
        let config = SwtLibraryConfig {
            timeout: timeout.unwrap_or(10.0),
            ..Default::default()
        };

        Self {
            config: Arc::new(RwLock::new(config)),
            connection: Arc::new(RwLock::new(SwtConnectionState::default())),
            element_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // ========================
    // Connection Keywords
    // ========================

    /// Connect to an SWT application.
    ///
    /// Establishes connection to a running SWT application via the SWT agent.
    ///
    /// | =Argument= | =Description= |
    /// | ``app`` | Application identifier (name or process ID). |
    /// | ``host`` | Remote host for network connections. Default ``localhost``. |
    /// | ``port`` | Port number for remote connections. Default ``5679``. |
    /// | ``timeout`` | Connection timeout in seconds. Default ``30``. |
    ///
    /// Example:
    /// | `Connect To SWT Application` | eclipse | | |
    /// | `Connect To SWT Application` | eclipse | localhost | 5679 |
    /// | `Connect To SWT Application` | myapp | 192.168.1.100 | 5679 |
    #[pyo3(signature = (app, host="localhost", port=5679, timeout=None))]
    pub fn connect_to_swt_application(
        &mut self,
        py: Python<'_>,
        app: &str,
        host: &str,
        port: u16,
        timeout: Option<PyObject>,
    ) -> PyResult<()> {
        if app.is_empty() {
            return Err(SwingError::connection("Application identifier cannot be empty").into());
        }

        let timeout_secs = py_to_f64(py, timeout).unwrap_or(30.0);
        let start_time = Instant::now();
        let total_timeout = Duration::from_secs_f64(timeout_secs);

        let mut conn = self.connection.write().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;

        // Establish TCP connection to the SWT agent with retry logic
        let addr = format!("{}:{}", host, port);

        use std::net::ToSocketAddrs;
        let socket_addr = addr.to_socket_addrs()
            .map_err(|e| SwingError::connection(format!("Failed to resolve address '{}': {}", addr, e)))?
            .next()
            .ok_or_else(|| SwingError::connection(format!("No addresses found for '{}'", addr)))?;

        // Retry connection attempts to allow SWT agent time to start
        let mut last_error = None;
        let stream = loop {
            let remaining_time = total_timeout.saturating_sub(start_time.elapsed());
            if remaining_time.is_zero() {
                break Err(last_error.unwrap_or_else(|| {
                    SwingError::connection("Connection timeout")
                }));
            }

            // Try to connect with a shorter timeout per attempt
            let attempt_timeout = std::cmp::min(remaining_time, Duration::from_secs(2));
            match TcpStream::connect_timeout(&socket_addr, attempt_timeout) {
                Ok(s) => break Ok(s),
                Err(e) => {
                    last_error = Some(SwingError::connection(format!("Failed to connect to {}: {}", addr, e)));
                    // Small delay before retry
                    std::thread::sleep(Duration::from_millis(500));
                }
            }
        }?;

        // Set stream timeouts
        stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
        stream.set_write_timeout(Some(Duration::from_secs(30))).ok();

        conn.stream = Some(stream);
        conn.connected = true;
        conn.application_name = Some(app.to_string());
        conn.host = Some(host.to_string());
        conn.port = Some(port);
        conn.request_id = 0;

        drop(conn);
        self.clear_caches()?;

        // Verify connection with ping
        let result = self.send_rpc_request("ping", serde_json::json!({}))?;
        if result.as_str() != Some("pong") {
            return Err(SwingError::connection("SWT agent did not respond to ping").into());
        }

        Ok(())
    }

    /// Disconnect from the current SWT application.
    ///
    /// Closes the connection to the SWT application and cleans up resources.
    /// This should be called in test teardown.
    ///
    /// Example:
    /// | `Connect To SWT Application` | eclipse |
    /// | # ... perform test actions ... | |
    /// | `Disconnect` | |
    pub fn disconnect(&mut self) -> PyResult<()> {
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
        self.clear_caches()?;

        Ok(())
    }

    // ========================
    // Shell Keywords
    // ========================

    /// Get all shells (top-level windows) in the SWT application.
    ///
    /// Returns a list of ``SwtElement`` objects representing all open shells.
    ///
    /// Example:
    /// | ${shells}= | `Get Shells` |
    /// | Log Many | @{shells} |
    pub fn get_shells(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("listShells", serde_json::json!({}))?;

        let list = PyList::empty(py);
        if let Some(shells) = result.as_array() {
            for shell in shells {
                if let Some(elem) = self.json_to_swt_element(shell) {
                    list.append(elem.into_py(py))?;
                }
            }
        }
        Ok(list.into())
    }

    /// Activate (bring to front) a shell.
    ///
    /// Brings the specified shell window to the foreground and gives it focus.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Shell locator (e.g., ``text:New Project``, ``name:mainShell``). |
    ///
    /// Example:
    /// | `Activate Shell` | text:New Project |
    /// | `Activate Shell` | name:preferences |
    #[pyo3(signature = (locator))]
    pub fn activate_shell(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("activateShell", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    /// Close a shell.
    ///
    /// Closes the specified shell window.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Shell locator (e.g., ``text:Preferences``, ``name:dialogShell``). |
    ///
    /// Example:
    /// | `Close Shell` | text:Preferences |
    /// | `Close Shell` | name:aboutDialog |
    #[pyo3(signature = (locator))]
    pub fn close_shell(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("closeShell", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    // ========================
    // Widget Finding Keywords
    // ========================

    /// Find a single widget matching the locator.
    ///
    /// Searches for a widget matching the given locator and returns it.
    /// Fails if no widget or multiple widgets are found.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:okButton``, ``text:OK``, ``Button``). |
    ///
    /// Returns an ``SwtElement`` representing the found widget.
    ///
    /// Raises ``ElementNotFoundError`` if no widget matches or
    /// ``MultipleElementsFoundError`` if multiple widgets match.
    ///
    /// Example:
    /// | ${widget}= | `Find Widget` | name:okButton |
    /// | ${widget}= | `Find Widget` | text:Cancel |
    /// | ${widget}= | `Find Widget` | Button |
    #[pyo3(signature = (locator))]
    pub fn find_widget(&self, locator: &str) -> PyResult<SwtElement> {
        self.ensure_connected()?;

        let widgets = self.find_widgets_internal(locator)?;

        match widgets.len() {
            0 => Err(SwingError::element_not_found(locator).into()),
            1 => Ok(widgets.into_iter().next().unwrap()),
            n => Err(SwingError::multiple_elements_found(locator, n).into()),
        }
    }

    /// Find all widgets matching the locator.
    ///
    /// Searches for all widgets matching the given locator.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``Button``, ``Text``, ``class:org.eclipse.swt.widgets.Button``). |
    ///
    /// Returns a list of ``SwtElement`` objects.
    ///
    /// Example:
    /// | ${buttons}= | `Find Widgets` | Button |
    /// | ${texts}= | `Find Widgets` | Text |
    /// | Log | Found ${buttons.__len__()} buttons |
    #[pyo3(signature = (locator))]
    pub fn find_widgets(&self, py: Python<'_>, locator: &str) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let widgets = self.find_widgets_internal(locator)?;
        let list = PyList::empty(py);
        for widget in widgets {
            list.append(widget.into_py(py))?;
        }
        Ok(list.into())
    }

    // ========================
    // Action Keywords
    // ========================

    /// Click on a widget.
    ///
    /// Performs a mouse click on the widget matching the locator.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:okButton``, ``text:OK``). |
    ///
    /// Example:
    /// | `Click Widget` | name:okButton |
    /// | `Click Widget` | text:Save |
    /// | `Click Widget` | Button |
    #[pyo3(signature = (locator))]
    pub fn click_widget(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("click", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    /// Double-click on a widget.
    ///
    /// Performs a double mouse click on the widget matching the locator.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:listItem``, ``text:file.txt``). |
    ///
    /// Example:
    /// | `Double Click Widget` | name:listItem |
    /// | `Double Click Widget` | text:Main.java |
    #[pyo3(signature = (locator))]
    pub fn double_click_widget(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("doubleClick", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    /// Input text into a text widget.
    ///
    /// Types text into the widget matching the locator. By default, clears
    /// any existing text first.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:searchField``, ``Text``). |
    /// | ``text`` | Text to input into the widget. |
    /// | ``clear`` | Clear existing text first. Default ``True``. |
    ///
    /// Example:
    /// | `Input Text` | name:searchField | hello world | |
    /// | `Input Text` | name:nameField | John Doe | clear=True |
    /// | `Input Text` | name:appendField | more text | clear=False |
    #[pyo3(signature = (locator, text, clear=true))]
    pub fn input_text(&self, locator: &str, text: &str, clear: bool) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

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

    /// Clear text from a text widget.
    ///
    /// Removes all text from the widget matching the locator.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:searchField``, ``Text``). |
    ///
    /// Example:
    /// | `Clear Text` | name:searchField |
    /// | `Clear Text` | text:Enter name |
    #[pyo3(signature = (locator))]
    pub fn clear_text(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("clearText", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    // ========================
    // Selection Keywords
    // ========================

    /// Select an item from a Combo widget.
    ///
    /// Selects an item from a dropdown combo box by its text.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Combo widget locator (e.g., ``name:typeCombo``, ``Combo``). |
    /// | ``item`` | Item text to select from the dropdown. |
    ///
    /// Example:
    /// | `Select Combo Item` | name:typeCombo | Java Project |
    /// | `Select Combo Item` | name:encoding | UTF-8 |
    #[pyo3(signature = (locator, item))]
    pub fn select_combo_item(&self, locator: &str, item: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("selectItem", serde_json::json!({
            "componentId": component_id,
            "value": item
        }))?;

        Ok(())
    }

    /// Select an item from a List widget.
    ///
    /// Selects an item from a list widget by its text.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | List widget locator (e.g., ``name:fileList``, ``List``). |
    /// | ``item`` | Item text to select from the list. |
    ///
    /// Example:
    /// | `Select List Item` | name:fileList | README.md |
    /// | `Select List Item` | name:projects | MyProject |
    #[pyo3(signature = (locator, item))]
    pub fn select_list_item(&self, locator: &str, item: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("selectItem", serde_json::json!({
            "componentId": component_id,
            "value": item
        }))?;

        Ok(())
    }

    /// Check a checkbox or toggle button.
    ///
    /// Ensures the checkbox or toggle button is checked/selected.
    /// If already checked, no action is taken.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Button widget locator (e.g., ``name:enableOption``, ``text:Enable``). |
    ///
    /// Example:
    /// | `Check Button` | name:enableOption |
    /// | `Check Button` | text:Show line numbers |
    #[pyo3(signature = (locator))]
    pub fn check_button(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        // Get current state
        let result = self.send_rpc_request("getWidgetProperties", serde_json::json!({
            "componentId": component_id
        }))?;

        let is_selected = result.get("selection").and_then(|v| v.as_bool()).unwrap_or(false);

        if !is_selected {
            self.send_rpc_request("click", serde_json::json!({
                "componentId": component_id
            }))?;
        }

        Ok(())
    }

    /// Uncheck a checkbox or toggle button.
    ///
    /// Ensures the checkbox or toggle button is unchecked/deselected.
    /// If already unchecked, no action is taken.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Button widget locator (e.g., ``name:enableOption``, ``text:Enable``). |
    ///
    /// Example:
    /// | `Uncheck Button` | name:enableOption |
    /// | `Uncheck Button` | text:Show line numbers |
    #[pyo3(signature = (locator))]
    pub fn uncheck_button(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        // Get current state
        let result = self.send_rpc_request("getWidgetProperties", serde_json::json!({
            "componentId": component_id
        }))?;

        let is_selected = result.get("selection").and_then(|v| v.as_bool()).unwrap_or(false);

        if is_selected {
            self.send_rpc_request("click", serde_json::json!({
                "componentId": component_id
            }))?;
        }

        Ok(())
    }

    // ========================
    // Table Keywords
    // ========================

    /// Get the number of rows in a table.
    ///
    /// Returns the total number of rows in the table widget.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table widget locator (e.g., ``name:resultsTable``, ``Table``). |
    ///
    /// Returns the row count as an integer.
    ///
    /// Example:
    /// | ${count}= | `Get Table Row Count` | name:resultsTable |
    /// | Should Be True | ${count} > 0 |
    #[pyo3(signature = (locator))]
    pub fn get_table_row_count(&self, locator: &str) -> PyResult<i32> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        let result = self.send_rpc_request("getTableRowCount", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(result.as_i64().unwrap_or(0) as i32)
    }

    /// Get the value of a table cell.
    ///
    /// Retrieves the text content of a specific cell in the table.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table widget locator (e.g., ``name:resultsTable``, ``Table``). |
    /// | ``row`` | Row index (0-based). |
    /// | ``col`` | Column index (0-based). |
    ///
    /// Returns the cell value as a string.
    ///
    /// Example:
    /// | ${value}= | `Get Table Cell` | name:resultsTable | 0 | 1 |
    /// | Should Be Equal | ${value} | Expected Value |
    #[pyo3(signature = (locator, row, col))]
    pub fn get_table_cell(&self, locator: &str, row: i32, col: i32) -> PyResult<String> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        let result = self.send_rpc_request("getTableCellValue", serde_json::json!({
            "componentId": component_id,
            "row": row,
            "column": col
        }))?;

        Ok(result.as_str().unwrap_or("").to_string())
    }

    /// Select a row in a table.
    ///
    /// Selects (highlights) a specific row in the table widget.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table widget locator (e.g., ``name:resultsTable``, ``Table``). |
    /// | ``row`` | Row index to select (0-based). |
    ///
    /// Example:
    /// | `Select Table Row` | name:resultsTable | 0 |
    /// | `Select Table Row` | name:resultsTable | 3 |
    #[pyo3(signature = (locator, row))]
    pub fn select_table_row(&self, locator: &str, row: i32) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("selectTableRow", serde_json::json!({
            "componentId": component_id,
            "row": row
        }))?;

        Ok(())
    }

    /// Get all cell values from a table row.
    ///
    /// Returns a list of all cell values from the specified row.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table widget locator (e.g., ``name:dataTable``). |
    /// | ``row`` | Zero-based row index. |
    ///
    /// Example:
    /// | @{values}= | `Get Table Row Values` | name:dataTable | 0 |
    #[pyo3(signature = (locator, row))]
    pub fn get_table_row_values(&self, locator: &str, row: i32) -> PyResult<Vec<String>> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        let response = self.send_rpc_request("getTableRowValues", serde_json::json!({
            "componentId": component_id,
            "row": row
        }))?;

        // Parse the JSON array response
        if let Some(result) = response.get("result") {
            if let Some(arr) = result.as_array() {
                return Ok(arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect());
            }
        }
        Ok(vec![])
    }

    /// Select multiple table rows.
    ///
    /// Selects multiple rows in a table (requires SWT.MULTI style).
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table widget locator (e.g., ``name:dataTable``). |
    /// | ``rows`` | List of zero-based row indices to select. |
    ///
    /// Example:
    /// | @{rows}= | Create List | 0 | 2 | 4 |
    /// | `Select Table Rows` | name:dataTable | ${rows} |
    #[pyo3(signature = (locator, rows))]
    pub fn select_table_rows(&self, locator: &str, rows: Vec<i32>) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("selectTableRows", serde_json::json!({
            "componentId": component_id,
            "rows": rows
        }))?;

        Ok(())
    }

    /// Deselect all table rows.
    ///
    /// Clears all row selections in a table.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table widget locator (e.g., ``name:dataTable``). |
    ///
    /// Example:
    /// | `Deselect All Table Rows` | name:dataTable |
    #[pyo3(signature = (locator))]
    pub fn deselect_all_table_rows(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("deselectAllTableRows", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    /// Select a table row by cell value.
    ///
    /// Finds and selects the first row containing the specified value in a column.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table widget locator (e.g., ``name:dataTable``). |
    /// | ``column`` | Zero-based column index to search. |
    /// | ``value`` | Value to search for. |
    ///
    /// Returns the row index that was selected, or -1 if not found.
    ///
    /// Example:
    /// | ${row}= | `Select Table Row By Value` | name:dataTable | 0 | John |
    #[pyo3(signature = (locator, column, value))]
    pub fn select_table_row_by_value(&self, locator: &str, column: i32, value: &str) -> PyResult<i32> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        let response = self.send_rpc_request("selectTableRowByValue", serde_json::json!({
            "componentId": component_id,
            "column": column,
            "value": value
        }))?;

        // Parse the row index from response
        if let Some(result) = response.get("result") {
            if let Some(row) = result.as_i64() {
                return Ok(row as i32);
            }
        }
        Ok(-1)
    }

    /// Select a range of consecutive table rows.
    ///
    /// Selects all rows from start_row to end_row (inclusive).
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table widget locator (e.g., ``name:dataTable``). |
    /// | ``start_row`` | First row index (inclusive). |
    /// | ``end_row`` | Last row index (inclusive). |
    ///
    /// Example:
    /// | `Select Table Row Range` | name:dataTable | 2 | 5 |
    #[pyo3(signature = (locator, start_row, end_row))]
    pub fn select_table_row_range(&self, locator: &str, start_row: i32, end_row: i32) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("selectTableRowRange", serde_json::json!({
            "componentId": component_id,
            "startRow": start_row,
            "endRow": end_row
        }))?;

        Ok(())
    }

    /// Click a table column header.
    ///
    /// Clicks on a column header, typically used for sorting.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table widget locator (e.g., ``name:dataTable``). |
    /// | ``column`` | Zero-based column index. |
    ///
    /// Example:
    /// | `Click Table Column Header` | name:dataTable | 0 |
    #[pyo3(signature = (locator, column))]
    pub fn click_table_column_header(&self, locator: &str, column: i32) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("clickTableColumnHeader", serde_json::json!({
            "componentId": component_id,
            "column": column
        }))?;

        Ok(())
    }

    /// Get table column information.
    ///
    /// Returns information about all columns in the table.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table widget locator (e.g., ``name:dataTable``). |
    ///
    /// Example:
    /// | @{columns}= | `Get Table Columns` | name:dataTable |
    #[pyo3(signature = (locator))]
    pub fn get_table_columns(&self, locator: &str) -> PyResult<Vec<String>> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        let response = self.send_rpc_request("getTableColumns", serde_json::json!({
            "componentId": component_id
        }))?;

        // Parse the JSON array response
        if let Some(result) = response.get("result") {
            if let Some(arr) = result.as_array() {
                return Ok(arr.iter()
                    .filter_map(|v| {
                        v.get("text").and_then(|t| t.as_str()).map(String::from)
                    })
                    .collect());
            }
        }
        Ok(vec![])
    }

    // ========================
    // Tree Keywords
    // ========================

    /// Expand a tree item.
    ///
    /// Expands a tree node to show its children. The path uses ``|`` as separator.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree widget locator (e.g., ``name:projectTree``, ``Tree``). |
    /// | ``path`` | Node path with ``|`` separators (e.g., ``Project|src|main``). |
    ///
    /// Example:
    /// | `Expand Tree Item` | name:projectTree | MyProject |
    /// | `Expand Tree Item` | name:projectTree | MyProject|src|main |
    #[pyo3(signature = (locator, path))]
    pub fn expand_tree_item(&self, locator: &str, path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("expandTreeNode", serde_json::json!({
            "componentId": component_id,
            "path": path
        }))?;

        Ok(())
    }

    /// Collapse a tree item.
    ///
    /// Collapses a tree node to hide its children. The path uses ``|`` as separator.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree widget locator (e.g., ``name:projectTree``, ``Tree``). |
    /// | ``path`` | Node path with ``|`` separators (e.g., ``Project|src``). |
    ///
    /// Example:
    /// | `Collapse Tree Item` | name:projectTree | MyProject|src |
    /// | `Collapse Tree Item` | name:projectTree | MyProject |
    #[pyo3(signature = (locator, path))]
    pub fn collapse_tree_item(&self, locator: &str, path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("collapseTreeNode", serde_json::json!({
            "componentId": component_id,
            "path": path
        }))?;

        Ok(())
    }

    /// Select a tree item.
    ///
    /// Selects a tree node by its path. The path uses ``|`` as separator.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree widget locator (e.g., ``name:projectTree``, ``Tree``). |
    /// | ``path`` | Node path with ``|`` separators (e.g., ``Project|src|Main.java``). |
    ///
    /// Example:
    /// | `Select Tree Item` | name:projectTree | MyProject |
    /// | `Select Tree Item` | name:projectTree | MyProject|src|Main.java |
    #[pyo3(signature = (locator, path))]
    pub fn select_tree_item(&self, locator: &str, path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("selectTreeNode", serde_json::json!({
            "componentId": component_id,
            "path": path
        }))?;

        Ok(())
    }

    /// Select multiple tree nodes.
    ///
    /// Selects multiple tree nodes (requires SWT.MULTI style).
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree widget locator (e.g., ``name:projectTree``). |
    /// | ``paths`` | List of node paths to select. |
    ///
    /// Example:
    /// | @{paths}= | Create List | Project|src | Project|test |
    /// | `Select Tree Nodes` | name:projectTree | ${paths} |
    #[pyo3(signature = (locator, paths))]
    pub fn select_tree_nodes(&self, locator: &str, paths: Vec<String>) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("selectTreeNodes", serde_json::json!({
            "componentId": component_id,
            "nodes": paths
        }))?;

        Ok(())
    }

    /// Get the parent of a tree node.
    ///
    /// Returns the parent node's text, or empty string if it's a root node.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree widget locator (e.g., ``name:projectTree``). |
    /// | ``node_name`` | Name of the node to get parent of. |
    ///
    /// Example:
    /// | ${parent}= | `Get Tree Node Parent` | name:projectTree | Main.java |
    #[pyo3(signature = (locator, node_name))]
    pub fn get_tree_node_parent(&self, locator: &str, node_name: &str) -> PyResult<String> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        let response = self.send_rpc_request("getTreeNodeParent", serde_json::json!({
            "componentId": component_id,
            "nodeName": node_name
        }))?;

        if let Some(result) = response.get("result") {
            if let Some(parent) = result.as_str() {
                return Ok(parent.to_string());
            }
        }
        Ok(String::new())
    }

    /// Get the depth level of a tree node.
    ///
    /// Returns 0 for root nodes, 1 for their children, etc.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree widget locator (e.g., ``name:projectTree``). |
    /// | ``node_name`` | Name of the node to get level of. |
    ///
    /// Example:
    /// | ${level}= | `Get Tree Node Level` | name:projectTree | src |
    #[pyo3(signature = (locator, node_name))]
    pub fn get_tree_node_level(&self, locator: &str, node_name: &str) -> PyResult<i32> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        let response = self.send_rpc_request("getTreeNodeLevel", serde_json::json!({
            "componentId": component_id,
            "nodeName": node_name
        }))?;

        if let Some(result) = response.get("result") {
            if let Some(level) = result.as_i64() {
                return Ok(level as i32);
            }
        }
        Ok(-1)
    }

    /// Check if a tree node exists.
    ///
    /// Returns True if a node with the given name exists in the tree.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree widget locator (e.g., ``name:projectTree``). |
    /// | ``node_name`` | Name of the node to check. |
    ///
    /// Example:
    /// | ${exists}= | `Tree Node Exists` | name:projectTree | Main.java |
    #[pyo3(signature = (locator, node_name))]
    pub fn tree_node_exists(&self, locator: &str, node_name: &str) -> PyResult<bool> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        let response = self.send_rpc_request("treeNodeExists", serde_json::json!({
            "componentId": component_id,
            "nodeName": node_name
        }))?;

        if let Some(result) = response.get("result") {
            if let Some(exists) = result.as_bool() {
                return Ok(exists);
            }
        }
        Ok(false)
    }

    /// Get selected tree nodes.
    ///
    /// Returns a list of paths of all selected nodes.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree widget locator (e.g., ``name:projectTree``). |
    ///
    /// Example:
    /// | @{selected}= | `Get Selected Tree Nodes` | name:projectTree |
    #[pyo3(signature = (locator))]
    pub fn get_selected_tree_nodes(&self, locator: &str) -> PyResult<Vec<String>> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        let response = self.send_rpc_request("getSelectedTreeNodes", serde_json::json!({
            "componentId": component_id
        }))?;

        if let Some(result) = response.get("result") {
            if let Some(arr) = result.as_array() {
                return Ok(arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect());
            }
        }
        Ok(vec![])
    }

    /// Deselect all tree nodes.
    ///
    /// Clears the selection in a tree widget.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree widget locator (e.g., ``name:projectTree``). |
    ///
    /// Example:
    /// | `Deselect All Tree Nodes` | name:projectTree |
    #[pyo3(signature = (locator))]
    pub fn deselect_all_tree_nodes(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("deselectAllTreeNodes", serde_json::json!({
            "componentId": component_id
        }))?;

        Ok(())
    }

    // ========================
    // Wait Keywords
    // ========================

    /// Wait until a widget exists.
    ///
    /// Waits until a widget matching the locator appears in the UI.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:progressDialog``, ``text:Complete``). |
    /// | ``timeout`` | Maximum wait time in seconds. Uses library default if not set. |
    ///
    /// Returns the ``SwtElement`` once found.
    ///
    /// Example:
    /// | ${widget}= | `Wait Until Widget Exists` | name:progressDialog | |
    /// | ${widget}= | `Wait Until Widget Exists` | text:Complete | timeout=30 |
    #[pyo3(signature = (locator, timeout=None))]
    pub fn wait_until_widget_exists(
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

            match self.find_widgets_internal(locator) {
                Ok(widgets) if !widgets.is_empty() => {
                    return Ok(widgets.into_iter().next().unwrap());
                }
                _ => {}
            }

            if start.elapsed() >= timeout_duration {
                return Err(SwingError::timeout(
                    format!("wait for widget '{}'", locator),
                    timeout_secs,
                ).into());
            }

            std::thread::sleep(poll_duration);
        }
    }

    /// Wait until a widget is enabled.
    ///
    /// Waits until the widget matching the locator becomes enabled.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:submitButton``, ``text:Submit``). |
    /// | ``timeout`` | Maximum wait time in seconds. Uses library default if not set. |
    ///
    /// Returns the ``SwtElement`` once enabled.
    ///
    /// Example:
    /// | ${widget}= | `Wait Until Widget Enabled` | name:submitButton | |
    /// | ${widget}= | `Wait Until Widget Enabled` | text:Next | timeout=10 |
    #[pyo3(signature = (locator, timeout=None))]
    pub fn wait_until_widget_enabled(
        &self,
        py: Python<'_>,
        locator: &str,
        timeout: Option<PyObject>,
    ) -> PyResult<SwtElement> {
        let timeout_f64 = py_to_f64(py, timeout);
        self.wait_for_widget_condition(locator, timeout_f64, |e| e.enabled, "enabled")
    }

    // ========================
    // Verification Keywords
    // ========================

    /// Verify that a widget is visible.
    ///
    /// Fails if the widget is not visible.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:warningLabel``, ``text:Warning``). |
    ///
    /// Raises ``AssertionError`` if widget is not visible.
    ///
    /// Example:
    /// | `Widget Should Be Visible` | name:warningLabel |
    /// | `Widget Should Be Visible` | text:Error message |
    #[pyo3(signature = (locator))]
    pub fn widget_should_be_visible(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let widget = self.find_widget(locator)?;
        if !widget.visible {
            return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Widget '{}' is not visible",
                locator
            )));
        }
        Ok(())
    }

    /// Verify that a widget is enabled.
    ///
    /// Fails if the widget is not enabled.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:submitButton``, ``text:Submit``). |
    ///
    /// Raises ``AssertionError`` if widget is not enabled.
    ///
    /// Example:
    /// | `Widget Should Be Enabled` | name:submitButton |
    /// | `Widget Should Be Enabled` | text:OK |
    #[pyo3(signature = (locator))]
    pub fn widget_should_be_enabled(&self, locator: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let widget = self.find_widget(locator)?;
        if !widget.enabled {
            return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Widget '{}' is not enabled",
                locator
            )));
        }
        Ok(())
    }

    /// Verify that a widget's text matches the expected value.
    ///
    /// Fails if the widget text does not exactly match the expected value.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:statusLabel``, ``Label``). |
    /// | ``expected`` | Expected text value to match. |
    ///
    /// Raises ``AssertionError`` if text doesn't match.
    ///
    /// Example:
    /// | `Widget Text Should Be` | name:statusLabel | Ready |
    /// | `Widget Text Should Be` | name:titleLabel | Welcome |
    #[pyo3(signature = (locator, expected))]
    pub fn widget_text_should_be(&self, locator: &str, expected: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let widget = self.find_widget(locator)?;
        let actual = widget.text.as_deref().unwrap_or("");

        if actual != expected {
            return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Widget text '{}' does not match expected '{}'",
                actual, expected
            )));
        }
        Ok(())
    }

    // ========================
    // Configuration Keywords
    // ========================

    /// Set the default timeout.
    ///
    /// Sets the default timeout used by wait keywords.
    ///
    /// | =Argument= | =Description= |
    /// | ``timeout`` | Timeout value in seconds. |
    ///
    /// Returns the previous timeout value.
    ///
    /// Example:
    /// | ${old}= | `Set Timeout` | 30 |
    /// | # ... perform slow operations ... | |
    /// | `Set Timeout` | ${old} |
    #[pyo3(signature = (timeout))]
    pub fn set_timeout(&self, py: Python<'_>, timeout: PyObject) -> PyResult<f64> {
        let timeout_val = py_to_f64(py, Some(timeout)).ok_or_else(|| {
            SwingError::connection("Invalid timeout value")
        })?;

        let mut config = self.config.write().map_err(|_| {
            SwingError::connection("Failed to acquire config lock")
        })?;

        let old = config.timeout;
        config.timeout = timeout_val;
        Ok(old)
    }

    /// Check if connected to an SWT application.
    ///
    /// Returns ``True`` if connected to an SWT application, ``False`` otherwise.
    ///
    /// Example:
    /// | ${connected}= | `Is Connected` |
    /// | Run Keyword If | not ${connected} | `Connect To SWT Application` | eclipse |
    pub fn is_connected(&self) -> PyResult<bool> {
        let conn = self.connection.read().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;
        Ok(conn.connected)
    }
}

// Private implementation methods
impl SwtLibrary {
    /// Ensure we're connected to an application
    fn ensure_connected(&self) -> PyResult<()> {
        let conn = self.connection.read().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;

        if !conn.connected {
            return Err(SwingError::connection("Not connected to any SWT application").into());
        }
        Ok(())
    }

    /// Send a JSON-RPC request to the SWT agent
    /// Made public to allow RcpLibrary and other extensions to use the same connection.
    pub fn send_rpc_request(&self, method: &str, params: serde_json::Value) -> PyResult<serde_json::Value> {
        let mut conn = self.connection.write().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;

        if !conn.connected {
            return Err(SwingError::connection("Not connected to any SWT application").into());
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

        // Read response - track JSON depth and consume trailing newline
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

                    // Skip leading whitespace before JSON starts
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
                                // JSON complete - consume trailing newline if present
                                stream.set_read_timeout(Some(Duration::from_millis(100))).ok();
                                let _ = stream.read(&mut byte_buf); // consume \n or \r\n
                                if byte_buf[0] == b'\r' {
                                    let _ = stream.read(&mut byte_buf); // consume \n after \r
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
                        break; // Already have complete JSON
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
            return Err(SwingError::connection("Empty response from SWT agent").into());
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

    /// Clear all caches
    fn clear_caches(&self) -> PyResult<()> {
        self.clear_element_cache()
    }

    /// Clear element cache
    fn clear_element_cache(&self) -> PyResult<()> {
        let mut cache = self.element_cache.write().map_err(|_| {
            SwingError::connection("Failed to acquire cache lock")
        })?;
        cache.clear();
        Ok(())
    }

    /// Parse locator string
    fn parse_locator(&self, locator: &str) -> (String, String) {
        let locator = locator.trim();

        // Check for explicit type=value format
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

        // Check for type:value format
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

    /// Find widgets by locator (internal)
    fn find_widgets_internal(&self, locator: &str) -> Result<Vec<SwtElement>, SwingError> {
        let (locator_type, value) = self.parse_locator(locator);

        let result = self.send_rpc_request("findWidgets", serde_json::json!({
            "locatorType": locator_type,
            "value": value
        })).map_err(|e| SwingError::element_not_found(format!("Failed to find widgets '{}': {}", locator, e)))?;

        let mut elements = Vec::new();
        if let Some(widgets) = result.as_array() {
            for widget in widgets {
                if let Some(elem) = self.json_to_swt_element(widget) {
                    elements.push(elem);
                }
            }
        }

        Ok(elements)
    }

    /// Get widget ID by locator
    fn get_widget_id(&self, locator: &str) -> Result<i64, SwingError> {
        let widgets = self.find_widgets_internal(locator)?;
        if widgets.is_empty() {
            return Err(SwingError::element_not_found(format!(
                "No widget found matching: {}",
                locator
            )));
        }
        Ok(widgets[0].hash_code)
    }

    /// Convert JSON to SwtElement
    fn json_to_swt_element(&self, json: &serde_json::Value) -> Option<SwtElement> {
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

    /// Wait for widget condition
    fn wait_for_widget_condition<F>(
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

            if let Ok(widget) = self.find_widget(locator) {
                if condition(&widget) {
                    return Ok(widget);
                }
            }

            if start.elapsed() >= timeout_duration {
                return Err(SwingError::timeout(
                    format!("wait for widget '{}' to be {}", locator, condition_name),
                    timeout_secs,
                ).into());
            }

            std::thread::sleep(poll_duration);
        }
    }
}

impl Default for SwtLibrary {
    fn default() -> Self {
        Self::new(None)
    }
}
