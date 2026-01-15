//! Robot Framework keyword class for Eclipse RCP automation
//!
//! This module provides the RcpLibrary class that extends SwtLibrary with
//! Eclipse RCP-specific keywords for automating Eclipse-based applications.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::swt_element::SwtElement;
use super::swt_library::SwtLibrary;
use super::exceptions::{SwingError, SwingErrorKind};

/// Robot Framework RCP Library
///
/// A high-performance library for automating Eclipse RCP applications
/// through Robot Framework. Extends SwtLibrary with RCP-specific keywords
/// for workbench, perspectives, views, editors, and commands.
///
/// Example:
///     *** Settings ***
///     Library    RcpLibrary
///
///     *** Test Cases ***
///     Test Eclipse Workbench
///         Connect To SWT Application    eclipse    localhost    5679
///         Open Perspective    org.eclipse.jdt.ui.JavaPerspective
///         Show View    org.eclipse.jdt.ui.PackageExplorer
///         Open Editor    /project/src/Main.java
///         Execute Command    org.eclipse.ui.file.save
///         [Teardown]    Disconnect
#[pyclass(name = "RcpLibrary")]
pub struct RcpLibrary {
    /// Underlying SWT library for base widget operations
    swt_lib: SwtLibrary,
}

#[pymethods]
impl RcpLibrary {
    /// Robot Framework library scope - GLOBAL to maintain connection across tests
    #[classattr]
    const ROBOT_LIBRARY_SCOPE: &'static str = "GLOBAL";

    /// Create a new RcpLibrary instance.
    ///
    /// | =Argument= | =Description= |
    /// | ``timeout`` | Default timeout for wait operations in seconds. Default ``10.0``. |
    ///
    /// Example:
    /// | =Setting= | =Value= | =Value= |
    /// | Library | swing_library.RcpLibrary | |
    /// | Library | swing_library.RcpLibrary | timeout=30 |
    #[new]
    #[pyo3(signature = (timeout=None))]
    pub fn new(timeout: Option<f64>) -> Self {
        Self {
            swt_lib: SwtLibrary::new(timeout),
        }
    }

    // ========================
    // Delegated Connection Keywords
    // ========================

    /// Connect to an RCP/SWT application.
    ///
    /// Establishes connection to a running Eclipse RCP application via the SWT agent.
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
    /// | `Connect To SWT Application` | dbeaver | 192.168.1.100 | 5679 |
    #[pyo3(signature = (app, host="localhost", port=5679, timeout=None))]
    pub fn connect_to_swt_application(
        &mut self,
        py: Python<'_>,
        app: &str,
        host: &str,
        port: u16,
        timeout: Option<PyObject>,
    ) -> PyResult<()> {
        self.swt_lib.connect_to_swt_application(py, app, host, port, timeout)
    }

    /// Connect to an RCP application (alias for connect_to_swt_application).
    ///
    /// Establishes connection to a running Eclipse RCP application.
    ///
    /// | =Argument= | =Description= |
    /// | ``app`` | Application identifier (name or process ID). |
    /// | ``host`` | Remote host for network connections. Default ``localhost``. |
    /// | ``port`` | Port number for remote connections. Default ``5679``. |
    /// | ``timeout`` | Connection timeout in seconds. Default ``30``. |
    ///
    /// Example:
    /// | `Connect To Application` | eclipse | | | timeout=60 |
    #[pyo3(signature = (app, host="localhost", port=5679, timeout=None))]
    pub fn connect_to_application(
        &mut self,
        py: Python<'_>,
        app: &str,
        host: &str,
        port: u16,
        timeout: Option<PyObject>,
    ) -> PyResult<()> {
        self.swt_lib.connect_to_swt_application(py, app, host, port, timeout)
    }

    /// Disconnect from the current RCP/SWT application.
    ///
    /// Closes the connection to the RCP application and cleans up resources.
    ///
    /// Example:
    /// | `Connect To SWT Application` | eclipse |
    /// | # ... perform test actions ... | |
    /// | `Disconnect` | |
    pub fn disconnect(&mut self) -> PyResult<()> {
        self.swt_lib.disconnect()
    }

    /// Check if connected to an application.
    ///
    /// Returns ``True`` if connected to an RCP application, ``False`` otherwise.
    ///
    /// Example:
    /// | ${connected}= | `Is Connected` |
    /// | Run Keyword If | not ${connected} | `Connect To SWT Application` | eclipse |
    pub fn is_connected(&self) -> PyResult<bool> {
        self.swt_lib.is_connected()
    }

    // ========================
    // Delegated Shell Keywords
    // ========================

    /// Get all shells in the application.
    ///
    /// Returns a list of ``SwtElement`` objects representing all open shells.
    ///
    /// Example:
    /// | ${shells}= | `Get Shells` |
    /// | Log Many | @{shells} |
    pub fn get_shells(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.swt_lib.get_shells(py)
    }

    /// Activate (bring to front) a shell.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Shell locator (e.g., ``text:Preferences``, ``name:mainShell``). |
    ///
    /// Example:
    /// | `Activate Shell` | text:Preferences |
    #[pyo3(signature = (locator))]
    pub fn activate_shell(&self, locator: &str) -> PyResult<()> {
        self.swt_lib.activate_shell(locator)
    }

    /// Close a shell.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Shell locator (e.g., ``text:Preferences``, ``name:dialogShell``). |
    ///
    /// Example:
    /// | `Close Shell` | text:Preferences |
    #[pyo3(signature = (locator))]
    pub fn close_shell(&self, locator: &str) -> PyResult<()> {
        self.swt_lib.close_shell(locator)
    }

    // ========================
    // Delegated Widget Keywords
    // ========================

    /// Find a widget by locator.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:okButton``, ``text:OK``). |
    ///
    /// Returns an ``SwtElement`` representing the found widget.
    ///
    /// Example:
    /// | ${widget}= | `Find Widget` | name:okButton |
    #[pyo3(signature = (locator))]
    pub fn find_widget(&self, locator: &str) -> PyResult<SwtElement> {
        self.swt_lib.find_widget(locator)
    }

    /// Find all widgets matching locator.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``Button``, ``Text``). |
    ///
    /// Returns a list of ``SwtElement`` objects.
    ///
    /// Example:
    /// | ${buttons}= | `Find Widgets` | Button |
    #[pyo3(signature = (locator))]
    pub fn find_widgets(&self, py: Python<'_>, locator: &str) -> PyResult<PyObject> {
        self.swt_lib.find_widgets(py, locator)
    }

    /// Click on a widget.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:okButton``, ``text:OK``). |
    ///
    /// Example:
    /// | `Click Widget` | name:okButton |
    #[pyo3(signature = (locator))]
    pub fn click_widget(&self, locator: &str) -> PyResult<()> {
        self.swt_lib.click_widget(locator)
    }

    /// Double-click on a widget.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:listItem``, ``text:file.txt``). |
    ///
    /// Example:
    /// | `Double Click Widget` | name:listItem |
    #[pyo3(signature = (locator))]
    pub fn double_click_widget(&self, locator: &str) -> PyResult<()> {
        self.swt_lib.double_click_widget(locator)
    }

    /// Input text into a widget.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:searchField``, ``Text``). |
    /// | ``text`` | Text to input into the widget. |
    /// | ``clear`` | Clear existing text first. Default ``True``. |
    ///
    /// Example:
    /// | `Input Text` | name:searchField | hello world |
    #[pyo3(signature = (locator, text, clear=true))]
    pub fn input_text(&self, locator: &str, text: &str, clear: bool) -> PyResult<()> {
        self.swt_lib.input_text(locator, text, clear)
    }

    /// Clear text from a widget.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:searchField``, ``Text``). |
    ///
    /// Example:
    /// | `Clear Text` | name:searchField |
    #[pyo3(signature = (locator))]
    pub fn clear_text(&self, locator: &str) -> PyResult<()> {
        self.swt_lib.clear_text(locator)
    }

    // ========================
    // Delegated Selection Keywords
    // ========================

    /// Select an item from a Combo widget.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Combo widget locator (e.g., ``name:typeCombo``, ``Combo``). |
    /// | ``item`` | Item text to select from the dropdown. |
    ///
    /// Example:
    /// | `Select Combo Item` | name:typeCombo | Java Project |
    #[pyo3(signature = (locator, item))]
    pub fn select_combo_item(&self, locator: &str, item: &str) -> PyResult<()> {
        self.swt_lib.select_combo_item(locator, item)
    }

    /// Select an item from a List widget.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | List widget locator (e.g., ``name:fileList``, ``List``). |
    /// | ``item`` | Item text to select from the list. |
    ///
    /// Example:
    /// | `Select List Item` | name:fileList | README.md |
    #[pyo3(signature = (locator, item))]
    pub fn select_list_item(&self, locator: &str, item: &str) -> PyResult<()> {
        self.swt_lib.select_list_item(locator, item)
    }

    /// Check a checkbox or toggle button.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Button widget locator (e.g., ``name:enableOption``, ``text:Enable``). |
    ///
    /// Example:
    /// | `Check Button` | name:enableOption |
    #[pyo3(signature = (locator))]
    pub fn check_button(&self, locator: &str) -> PyResult<()> {
        self.swt_lib.check_button(locator)
    }

    /// Uncheck a checkbox or toggle button.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Button widget locator (e.g., ``name:enableOption``, ``text:Enable``). |
    ///
    /// Example:
    /// | `Uncheck Button` | name:enableOption |
    #[pyo3(signature = (locator))]
    pub fn uncheck_button(&self, locator: &str) -> PyResult<()> {
        self.swt_lib.uncheck_button(locator)
    }

    // ========================
    // Delegated Table Keywords
    // ========================

    /// Get the number of rows in a table.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table widget locator (e.g., ``name:resultsTable``, ``Table``). |
    ///
    /// Returns the row count as an integer.
    ///
    /// Example:
    /// | ${count}= | `Get Table Row Count` | name:resultsTable |
    #[pyo3(signature = (locator))]
    pub fn get_table_row_count(&self, locator: &str) -> PyResult<i32> {
        self.swt_lib.get_table_row_count(locator)
    }

    /// Get the value of a table cell.
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
    #[pyo3(signature = (locator, row, col))]
    pub fn get_table_cell(&self, locator: &str, row: i32, col: i32) -> PyResult<String> {
        self.swt_lib.get_table_cell(locator, row, col)
    }

    /// Select a row in a table.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Table widget locator (e.g., ``name:resultsTable``, ``Table``). |
    /// | ``row`` | Row index to select (0-based). |
    ///
    /// Example:
    /// | `Select Table Row` | name:resultsTable | 0 |
    #[pyo3(signature = (locator, row))]
    pub fn select_table_row(&self, locator: &str, row: i32) -> PyResult<()> {
        self.swt_lib.select_table_row(locator, row)
    }

    // ========================
    // Delegated Tree Keywords
    // ========================

    /// Expand a tree item.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree widget locator (e.g., ``name:projectTree``, ``Tree``). |
    /// | ``path`` | Node path with ``|`` separators (e.g., ``Project|src|main``). |
    ///
    /// Example:
    /// | `Expand Tree Item` | name:projectTree | MyProject|src |
    #[pyo3(signature = (locator, path))]
    pub fn expand_tree_item(&self, locator: &str, path: &str) -> PyResult<()> {
        self.swt_lib.expand_tree_item(locator, path)
    }

    /// Collapse a tree item.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree widget locator (e.g., ``name:projectTree``, ``Tree``). |
    /// | ``path`` | Node path with ``|`` separators (e.g., ``Project|src``). |
    ///
    /// Example:
    /// | `Collapse Tree Item` | name:projectTree | MyProject|src |
    #[pyo3(signature = (locator, path))]
    pub fn collapse_tree_item(&self, locator: &str, path: &str) -> PyResult<()> {
        self.swt_lib.collapse_tree_item(locator, path)
    }

    /// Select a tree item.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Tree widget locator (e.g., ``name:projectTree``, ``Tree``). |
    /// | ``path`` | Node path with ``|`` separators (e.g., ``Project|src|Main.java``). |
    ///
    /// Example:
    /// | `Select Tree Item` | name:projectTree | MyProject|src|Main.java |
    #[pyo3(signature = (locator, path))]
    pub fn select_tree_item(&self, locator: &str, path: &str) -> PyResult<()> {
        self.swt_lib.select_tree_item(locator, path)
    }

    // ========================
    // Delegated Wait Keywords
    // ========================

    /// Wait until a widget exists.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:progressDialog``, ``text:Complete``). |
    /// | ``timeout`` | Maximum wait time in seconds. Uses library default if not set. |
    ///
    /// Returns the ``SwtElement`` once found.
    ///
    /// Example:
    /// | ${widget}= | `Wait Until Widget Exists` | name:progressDialog |
    #[pyo3(signature = (locator, timeout=None))]
    pub fn wait_until_widget_exists(
        &self,
        py: Python<'_>,
        locator: &str,
        timeout: Option<PyObject>,
    ) -> PyResult<SwtElement> {
        self.swt_lib.wait_until_widget_exists(py, locator, timeout)
    }

    /// Wait until a widget is enabled.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:submitButton``, ``text:Submit``). |
    /// | ``timeout`` | Maximum wait time in seconds. Uses library default if not set. |
    ///
    /// Returns the ``SwtElement`` once enabled.
    ///
    /// Example:
    /// | ${widget}= | `Wait Until Widget Enabled` | name:submitButton |
    #[pyo3(signature = (locator, timeout=None))]
    pub fn wait_until_widget_enabled(
        &self,
        py: Python<'_>,
        locator: &str,
        timeout: Option<PyObject>,
    ) -> PyResult<SwtElement> {
        self.swt_lib.wait_until_widget_enabled(py, locator, timeout)
    }

    // ========================
    // Delegated Verification Keywords
    // ========================

    /// Verify that a widget is visible.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:warningLabel``, ``text:Warning``). |
    ///
    /// Raises ``AssertionError`` if widget is not visible.
    ///
    /// Example:
    /// | `Widget Should Be Visible` | name:warningLabel |
    #[pyo3(signature = (locator))]
    pub fn widget_should_be_visible(&self, locator: &str) -> PyResult<()> {
        self.swt_lib.widget_should_be_visible(locator)
    }

    /// Verify that a widget is enabled.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:submitButton``, ``text:Submit``). |
    ///
    /// Raises ``AssertionError`` if widget is not enabled.
    ///
    /// Example:
    /// | `Widget Should Be Enabled` | name:submitButton |
    #[pyo3(signature = (locator))]
    pub fn widget_should_be_enabled(&self, locator: &str) -> PyResult<()> {
        self.swt_lib.widget_should_be_enabled(locator)
    }

    /// Verify that a widget's text matches the expected value.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator (e.g., ``name:statusLabel``, ``Label``). |
    /// | ``expected`` | Expected text value to match. |
    ///
    /// Raises ``AssertionError`` if text doesn't match.
    ///
    /// Example:
    /// | `Widget Text Should Be` | name:statusLabel | Ready |
    #[pyo3(signature = (locator, expected))]
    pub fn widget_text_should_be(&self, locator: &str, expected: &str) -> PyResult<()> {
        self.swt_lib.widget_text_should_be(locator, expected)
    }

    /// Set the default timeout.
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
        self.swt_lib.set_timeout(py, timeout)
    }

    // ========================
    // RCP Workbench Keywords
    // ========================

    /// Get information about the Eclipse workbench.
    ///
    /// Returns a dictionary containing workbench state information including
    /// active perspective, open views, open editors, and window title.
    ///
    /// Example:
    /// | ${info}= | `Get Workbench Info` |
    /// | Log | Active perspective: ${info}[activePerspective] |
    /// | Log | Window title: ${info}[windowTitle] |
    pub fn get_workbench_info(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getWorkbenchInfo", serde_json::json!({}))?;

        let dict = PyDict::new(py);
        if let Some(obj) = result.as_object() {
            for (key, value) in obj {
                dict.set_item(key, self.json_to_py(py, value)?)?;
            }
        }
        Ok(dict.into())
    }

    // ========================
    // RCP Perspective Keywords
    // ========================

    /// Get the ID of the currently active perspective.
    ///
    /// Returns the perspective ID as a string (e.g., ``org.eclipse.jdt.ui.JavaPerspective``).
    ///
    /// Example:
    /// | ${perspective}= | `Get Active Perspective` |
    /// | Should Be Equal | ${perspective} | org.eclipse.jdt.ui.JavaPerspective |
    pub fn get_active_perspective(&self) -> PyResult<String> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getActivePerspective", serde_json::json!({}))?;

        // Result is a JSON object with "id" field
        if let Some(id) = result.get("id").and_then(|v| v.as_str()) {
            Ok(id.to_string())
        } else if let Some(s) = result.as_str() {
            Ok(s.to_string())
        } else {
            Ok(String::new())
        }
    }

    /// Open (switch to) a perspective by ID.
    ///
    /// Switches the workbench to the specified perspective.
    ///
    /// | =Argument= | =Description= |
    /// | ``perspective_id`` | The perspective ID to open. |
    ///
    /// Example:
    /// | `Open Perspective` | org.eclipse.jdt.ui.JavaPerspective |
    /// | `Open Perspective` | org.eclipse.debug.ui.DebugPerspective |
    #[pyo3(signature = (perspective_id))]
    pub fn open_perspective(&self, perspective_id: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if perspective_id.is_empty() {
            return Err(SwingError::validation("Perspective ID cannot be empty").into());
        }

        let result = self.send_rpc_request("rcp.openPerspective", serde_json::json!({
            "perspectiveId": perspective_id
        }))?;

        if let Some(obj) = result.as_object() {
            if let Some(success) = obj.get("success").and_then(|v| v.as_bool()) {
                if !success {
                    let error_msg = obj.get("error").and_then(|v| v.as_str())
                        .unwrap_or("Failed to open perspective");
                    return Err(SwingError::rcp_error(error_msg).into());
                }
            }
        }

        Ok(())
    }

    /// Reset the current perspective to its default layout.
    ///
    /// Restores the default view arrangement for the current perspective.
    ///
    /// Example:
    /// | `Reset Perspective` |
    pub fn reset_perspective(&self) -> PyResult<()> {
        self.ensure_connected()?;

        self.send_rpc_request("rcp.resetPerspective", serde_json::json!({}))?;

        Ok(())
    }

    /// Get a list of all available perspectives.
    ///
    /// Returns a list of perspective objects with ``id``, ``name``, and ``description``.
    ///
    /// Example:
    /// | ${perspectives}= | `Get Available Perspectives` |
    /// | FOR | ${p} | IN | @{perspectives} |
    /// |     | Log | ${p}[id]: ${p}[name] |
    /// | END |
    pub fn get_available_perspectives(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getAvailablePerspectives", serde_json::json!({}))?;

        let list = PyList::empty(py);
        if let Some(perspectives) = result.as_array() {
            for perspective in perspectives {
                let dict = PyDict::new(py);
                if let Some(obj) = perspective.as_object() {
                    for (key, value) in obj {
                        dict.set_item(key, self.json_to_py(py, value)?)?;
                    }
                }
                list.append(dict)?;
            }
        }
        Ok(list.into())
    }

    // ========================
    // RCP View Keywords
    // ========================

    /// Show (open) a view by ID.
    ///
    /// Opens and displays the specified view in the workbench.
    ///
    /// | =Argument= | =Description= |
    /// | ``view_id`` | The view ID to show. |
    /// | ``secondary_id`` | Optional secondary ID for multi-instance views. |
    ///
    /// Example:
    /// | `Show View` | org.eclipse.jdt.ui.PackageExplorer | |
    /// | `Show View` | org.eclipse.ui.console.ConsoleView | |
    /// | `Show View` | org.eclipse.ui.views.PropertySheet | secondary1 |
    #[pyo3(signature = (view_id, secondary_id=None))]
    pub fn show_view(&self, view_id: &str, secondary_id: Option<&str>) -> PyResult<()> {
        self.ensure_connected()?;

        if view_id.is_empty() {
            return Err(SwingError::validation("View ID cannot be empty").into());
        }

        let mut params = serde_json::json!({
            "viewId": view_id
        });

        if let Some(sid) = secondary_id {
            params["secondaryId"] = serde_json::Value::String(sid.to_string());
        }

        let result = self.send_rpc_request("rcp.showView", params)?;

        if let Some(obj) = result.as_object() {
            if let Some(success) = obj.get("success").and_then(|v| v.as_bool()) {
                if !success {
                    let error_msg = obj.get("error").and_then(|v| v.as_str())
                        .unwrap_or("Failed to show view");
                    return Err(SwingError::rcp_error(error_msg).into());
                }
            }
        }

        Ok(())
    }

    /// Close a view by ID.
    ///
    /// Closes the specified view in the workbench.
    ///
    /// | =Argument= | =Description= |
    /// | ``view_id`` | The view ID to close. |
    /// | ``secondary_id`` | Optional secondary ID for multi-instance views. |
    ///
    /// Example:
    /// | `Close View` | org.eclipse.ui.views.PropertySheet |
    /// | `Close View` | org.eclipse.ui.views.PropertySheet | secondary1 |
    #[pyo3(signature = (view_id, secondary_id=None))]
    pub fn close_view(&self, view_id: &str, secondary_id: Option<&str>) -> PyResult<()> {
        self.ensure_connected()?;

        if view_id.is_empty() {
            return Err(SwingError::validation("View ID cannot be empty").into());
        }

        let mut params = serde_json::json!({
            "viewId": view_id
        });

        if let Some(sid) = secondary_id {
            params["secondaryId"] = serde_json::Value::String(sid.to_string());
        }

        self.send_rpc_request("rcp.closeView", params)?;

        Ok(())
    }

    /// Activate (bring to front) a view by ID.
    ///
    /// Brings the specified view to the foreground and gives it focus.
    ///
    /// | =Argument= | =Description= |
    /// | ``view_id`` | The view ID to activate. |
    ///
    /// Example:
    /// | `Activate View` | org.eclipse.jdt.ui.PackageExplorer |
    /// | `Activate View` | org.eclipse.ui.console.ConsoleView |
    #[pyo3(signature = (view_id))]
    pub fn activate_view(&self, view_id: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if view_id.is_empty() {
            return Err(SwingError::validation("View ID cannot be empty").into());
        }

        self.send_rpc_request("rcp.activateView", serde_json::json!({
            "viewId": view_id
        }))?;

        Ok(())
    }

    /// Verify that a view is visible.
    ///
    /// Fails if the specified view is not currently visible.
    ///
    /// | =Argument= | =Description= |
    /// | ``view_id`` | The view ID to check. |
    ///
    /// Raises ``AssertionError`` if the view is not visible.
    ///
    /// Example:
    /// | `View Should Be Visible` | org.eclipse.jdt.ui.PackageExplorer |
    #[pyo3(signature = (view_id))]
    pub fn view_should_be_visible(&self, view_id: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.isViewVisible", serde_json::json!({
            "viewId": view_id
        }))?;

        let is_visible = result.as_bool().unwrap_or(false);
        if !is_visible {
            return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "View '{}' is not visible",
                view_id
            )));
        }
        Ok(())
    }

    /// Get a list of all currently open views.
    ///
    /// Returns a list of view objects with ``id``, ``title``, and ``partName``.
    ///
    /// Example:
    /// | ${views}= | `Get Open Views` |
    /// | FOR | ${view} | IN | @{views} |
    /// |     | Log | ${view}[id]: ${view}[title] |
    /// | END |
    pub fn get_open_views(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getOpenViews", serde_json::json!({}))?;

        let list = PyList::empty(py);
        if let Some(views) = result.as_array() {
            for view in views {
                let dict = PyDict::new(py);
                if let Some(obj) = view.as_object() {
                    for (key, value) in obj {
                        dict.set_item(key, self.json_to_py(py, value)?)?;
                    }
                }
                list.append(dict)?;
            }
        }
        Ok(list.into())
    }

    /// Find a widget within a specific view.
    ///
    /// Searches for a widget within the specified view.
    ///
    /// | =Argument= | =Description= |
    /// | ``view_id`` | The view ID containing the widget. |
    /// | ``locator`` | Widget locator within the view. |
    ///
    /// Returns an ``SwtElement`` representing the found widget.
    ///
    /// Example:
    /// | ${tree}= | `Get View Widget` | org.eclipse.jdt.ui.PackageExplorer | Tree |
    /// | `Click Widget` | ${tree} |
    #[pyo3(signature = (view_id, locator))]
    pub fn get_view_widget(&self, view_id: &str, locator: &str) -> PyResult<SwtElement> {
        self.ensure_connected()?;

        if view_id.is_empty() {
            return Err(SwingError::validation("View ID cannot be empty").into());
        }

        let result = self.send_rpc_request("rcp.getViewWidget", serde_json::json!({
            "viewId": view_id,
            "locator": locator
        }))?;

        // Check for error response
        if result.get("error").is_some() {
            return Err(SwingError::element_not_found(format!(
                "Widget '{}' not found in view '{}'",
                locator, view_id
            )).into());
        }

        self.json_to_swt_element(&result)
            .ok_or_else(|| SwingError::element_not_found(format!(
                "Widget '{}' not found in view '{}'",
                locator, view_id
            )).into())
    }

    // ========================
    // RCP Editor Keywords
    // ========================

    /// Get the currently active editor.
    ///
    /// Returns an editor object with ``title``, ``path``, and ``dirty`` state,
    /// or ``None`` if no editor is active.
    ///
    /// Example:
    /// | ${editor}= | `Get Active Editor` |
    /// | Should Not Be None | ${editor} |
    /// | Log | Editing: ${editor}[title] |
    pub fn get_active_editor(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getActiveEditor", serde_json::json!({}))?;

        if result.is_null() {
            return Ok(py.None());
        }

        let dict = PyDict::new(py);
        if let Some(obj) = result.as_object() {
            for (key, value) in obj {
                dict.set_item(key, self.json_to_py(py, value)?)?;
            }
        }
        Ok(dict.into())
    }

    /// Open a file in an editor.
    ///
    /// Opens the specified file in an Eclipse editor.
    ///
    /// | =Argument= | =Description= |
    /// | ``file_path`` | Path to the file (workspace-relative or absolute). |
    ///
    /// Example:
    /// | `Open Editor` | /MyProject/src/Main.java |
    /// | `Open Editor` | C:/workspace/project/file.txt |
    #[pyo3(signature = (file_path))]
    pub fn open_editor(&self, file_path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if file_path.is_empty() {
            return Err(SwingError::validation("File path cannot be empty").into());
        }

        self.send_rpc_request("rcp.openEditor", serde_json::json!({
            "filePath": file_path
        }))?;

        Ok(())
    }

    /// Close an editor by title.
    ///
    /// Closes the editor with the specified title.
    ///
    /// | =Argument= | =Description= |
    /// | ``title`` | Editor title (usually the filename). |
    /// | ``save`` | Save changes before closing. Default ``False``. |
    ///
    /// Example:
    /// | `Close Editor` | Main.java | save=True |
    /// | `Close Editor` | README.md | |
    #[pyo3(signature = (title, save=false))]
    pub fn close_editor(&self, title: &str, save: bool) -> PyResult<()> {
        self.ensure_connected()?;

        if title.is_empty() {
            return Err(SwingError::validation("Editor title cannot be empty").into());
        }

        self.send_rpc_request("rcp.closeEditor", serde_json::json!({
            "filePath": title,
            "save": save
        }))?;

        Ok(())
    }

    /// Close all open editors.
    ///
    /// Closes all editors in the workbench.
    ///
    /// | =Argument= | =Description= |
    /// | ``save`` | Save all changes before closing. Default ``False``. |
    ///
    /// Returns ``True`` if all editors were closed successfully.
    ///
    /// Example:
    /// | ${result}= | `Close All Editors` | save=True |
    /// | `Close All Editors` | |
    #[pyo3(signature = (save=false))]
    pub fn close_all_editors(&self, save: bool) -> PyResult<bool> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.closeAllEditors", serde_json::json!({
            "save": save
        }))?;

        // Server returns {"success": true/false}
        if let Some(obj) = result.as_object() {
            if let Some(success) = obj.get("success") {
                return Ok(success.as_bool().unwrap_or(false));
            }
        }
        Ok(result.as_bool().unwrap_or(false))
    }

    /// Save the current or specified editor.
    ///
    /// Saves the editor contents to disk.
    ///
    /// | =Argument= | =Description= |
    /// | ``title`` | Optional editor title. Saves active editor if not provided. |
    ///
    /// Example:
    /// | `Save Editor` | | | # Save active editor |
    /// | `Save Editor` | Main.java | | # Save specific editor |
    #[pyo3(signature = (title=None))]
    pub fn save_editor(&self, title: Option<&str>) -> PyResult<()> {
        self.ensure_connected()?;

        let params = match title {
            Some(t) => serde_json::json!({ "filePath": t }),
            None => serde_json::json!({}),
        };

        self.send_rpc_request("rcp.saveEditor", params)?;

        Ok(())
    }

    /// Save all open editors.
    ///
    /// Saves all editors with unsaved changes.
    ///
    /// Example:
    /// | `Save All Editors` |
    pub fn save_all_editors(&self) -> PyResult<()> {
        self.ensure_connected()?;

        self.send_rpc_request("rcp.saveAllEditors", serde_json::json!({}))?;

        Ok(())
    }

    /// Activate (bring to front) an editor by title.
    ///
    /// Brings the specified editor to the foreground and gives it focus.
    ///
    /// | =Argument= | =Description= |
    /// | ``title`` | Editor title (usually the filename). |
    ///
    /// Example:
    /// | `Activate Editor` | Main.java |
    /// | `Activate Editor` | README.md |
    #[pyo3(signature = (title))]
    pub fn activate_editor(&self, title: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if title.is_empty() {
            return Err(SwingError::validation("Editor title cannot be empty").into());
        }

        self.send_rpc_request("rcp.activateEditor", serde_json::json!({
            "filePath": title
        }))?;

        Ok(())
    }

    /// Verify that an editor has unsaved changes (is dirty).
    ///
    /// Fails if the editor does not have unsaved changes.
    ///
    /// | =Argument= | =Description= |
    /// | ``title`` | Editor title. |
    ///
    /// Raises ``AssertionError`` if the editor is not dirty.
    ///
    /// Example:
    /// | `Input Text` | editor:Main.java | new code |
    /// | `Editor Should Be Dirty` | /project/src/Main.java |
    #[pyo3(signature = (file_path))]
    pub fn editor_should_be_dirty(&self, file_path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.isEditorDirty", serde_json::json!({
            "filePath": file_path
        }))?;

        let is_dirty = result.as_bool().unwrap_or(false);
        if !is_dirty {
            return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Editor '{}' is not dirty (has no unsaved changes)",
                file_path
            )));
        }
        Ok(())
    }

    /// Verify that an editor has no unsaved changes (is not dirty).
    ///
    /// Fails if the editor has unsaved changes.
    ///
    /// | =Argument= | =Description= |
    /// | ``title`` | Editor title. |
    ///
    /// Raises ``AssertionError`` if the editor is dirty.
    ///
    /// Example:
    /// | `Save Editor` | /project/src/Main.java |
    /// | `Editor Should Not Be Dirty` | /project/src/Main.java |
    #[pyo3(signature = (file_path))]
    pub fn editor_should_not_be_dirty(&self, file_path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.isEditorDirty", serde_json::json!({
            "filePath": file_path
        }))?;

        let is_dirty = result.as_bool().unwrap_or(false);
        if is_dirty {
            return Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Editor '{}' is dirty (has unsaved changes)",
                file_path
            )));
        }
        Ok(())
    }

    /// Get a list of all currently open editors.
    ///
    /// Returns a list of editor objects with ``title``, ``path``, and ``dirty`` state.
    ///
    /// Example:
    /// | ${editors}= | `Get Open Editors` |
    /// | FOR | ${editor} | IN | @{editors} |
    /// |     | Log | ${editor}[title] - Dirty: ${editor}[dirty] |
    /// | END |
    pub fn get_open_editors(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getOpenEditors", serde_json::json!({}))?;

        let list = PyList::empty(py);
        if let Some(editors) = result.as_array() {
            for editor in editors {
                let dict = PyDict::new(py);
                if let Some(obj) = editor.as_object() {
                    for (key, value) in obj {
                        dict.set_item(key, self.json_to_py(py, value)?)?;
                    }
                }
                list.append(dict)?;
            }
        }
        Ok(list.into())
    }

    /// Find a widget within a specific editor.
    ///
    /// Searches for a widget within the specified editor.
    ///
    /// | =Argument= | =Description= |
    /// | ``title`` | Editor title. |
    /// | ``locator`` | Widget locator within the editor. |
    ///
    /// Returns an ``SwtElement`` representing the found widget.
    ///
    /// Example:
    /// | ${text}= | `Get Editor Widget` | Main.java | StyledText |
    #[pyo3(signature = (title, locator))]
    pub fn get_editor_widget(&self, title: &str, locator: &str) -> PyResult<SwtElement> {
        self.ensure_connected()?;

        if title.is_empty() {
            return Err(SwingError::validation("Editor title cannot be empty").into());
        }

        let result = self.send_rpc_request("rcp.getEditorWidget", serde_json::json!({
            "title": title,
            "locator": locator
        }))?;

        // Check for error response
        if result.get("error").is_some() {
            return Err(SwingError::element_not_found(format!(
                "Widget '{}' not found in editor '{}'",
                locator, title
            )).into());
        }

        self.json_to_swt_element(&result)
            .ok_or_else(|| SwingError::element_not_found(format!(
                "Widget '{}' not found in editor '{}'",
                locator, title
            )).into())
    }

    // ========================
    // RCP Menu Keywords
    // ========================

    /// Select an item from the main menu bar.
    ///
    /// Navigates and selects an item from the Eclipse main menu.
    ///
    /// | =Argument= | =Description= |
    /// | ``path`` | Menu path with ``|`` separator (e.g., ``File|New|Project...``). |
    ///
    /// Example:
    /// | `Select Main Menu` | File|New|Project... |
    /// | `Select Main Menu` | Edit|Find/Replace... |
    /// | `Select Main Menu` | Window|Show View|Other... |
    #[pyo3(signature = (path))]
    pub fn select_main_menu(&self, path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if path.is_empty() {
            return Err(SwingError::validation("Menu path cannot be empty").into());
        }

        self.send_rpc_request("rcp.selectMainMenu", serde_json::json!({
            "path": path
        }))?;

        Ok(())
    }

    /// Select an item from a context menu.
    ///
    /// Right-clicks on a widget and selects from the context menu.
    ///
    /// | =Argument= | =Description= |
    /// | ``locator`` | Widget locator to right-click on. |
    /// | ``path`` | Context menu path with ``|`` separator. |
    ///
    /// Example:
    /// | `Select Context Menu` | tree:packageExplorer | New|Class |
    /// | `Select Context Menu` | name:file.java | Open With|Text Editor |
    #[pyo3(signature = (locator, path))]
    pub fn select_context_menu(&self, locator: &str, path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if path.is_empty() {
            return Err(SwingError::validation("Menu path cannot be empty").into());
        }

        let component_id = self.get_widget_id(locator)?;

        self.send_rpc_request("rcp.selectContextMenu", serde_json::json!({
            "componentId": component_id,
            "path": path
        }))?;

        Ok(())
    }

    // ========================
    // RCP Command Keywords
    // ========================

    /// Execute an Eclipse command by ID.
    ///
    /// Executes commands from the Eclipse command framework, similar to
    /// using keyboard shortcuts or menu items programmatically.
    ///
    /// | =Argument= | =Description= |
    /// | ``command_id`` | The command ID to execute. |
    ///
    /// Example:
    /// | `Execute Command` | org.eclipse.ui.file.save |
    /// | `Execute Command` | org.eclipse.ui.edit.undo |
    /// | `Execute Command` | org.eclipse.jdt.ui.edit.text.java.organize.imports |
    #[pyo3(signature = (command_id))]
    pub fn execute_command(&self, command_id: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if command_id.is_empty() {
            return Err(SwingError::validation("Command ID cannot be empty").into());
        }

        self.send_rpc_request("rcp.executeCommand", serde_json::json!({
            "commandId": command_id
        }))?;

        Ok(())
    }

    /// Get a list of available commands.
    ///
    /// Returns a list of command objects with ``id``, ``name``, and ``description``.
    ///
    /// | =Argument= | =Description= |
    /// | ``category`` | Optional category filter. |
    ///
    /// Example:
    /// | ${commands}= | `Get Available Commands` | |
    /// | ${edit_commands}= | `Get Available Commands` | Edit |
    #[pyo3(signature = (category=None))]
    pub fn get_available_commands(&self, py: Python<'_>, category: Option<&str>) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let params = match category {
            Some(c) => serde_json::json!({ "category": c }),
            None => serde_json::json!({}),
        };

        let result = self.send_rpc_request("rcp.getAvailableCommands", params)?;

        let list = PyList::empty(py);
        if let Some(commands) = result.as_array() {
            for command in commands {
                let dict = PyDict::new(py);
                if let Some(obj) = command.as_object() {
                    for (key, value) in obj {
                        dict.set_item(key, self.json_to_py(py, value)?)?;
                    }
                }
                list.append(dict)?;
            }
        }
        Ok(list.into())
    }

    // ========================
    // RCP Toolbar Keywords
    // ========================

    /// Click a toolbar item by tooltip.
    ///
    /// Clicks a toolbar button identified by its tooltip text.
    ///
    /// | =Argument= | =Description= |
    /// | ``tooltip`` | The tooltip text of the toolbar item. |
    ///
    /// Example:
    /// | `Click Toolbar Item` | Save |
    /// | `Click Toolbar Item` | Run |
    /// | `Click Toolbar Item` | Debug |
    #[pyo3(signature = (tooltip))]
    pub fn click_toolbar_item(&self, tooltip: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if tooltip.is_empty() {
            return Err(SwingError::validation("Tooltip cannot be empty").into());
        }

        self.send_rpc_request("rcp.clickToolbarItem", serde_json::json!({
            "tooltip": tooltip
        }))?;

        Ok(())
    }

    // ========================
    // RCP Preferences Keywords
    // ========================

    /// Open the Preferences dialog.
    ///
    /// Opens the Eclipse Preferences dialog.
    ///
    /// Example:
    /// | `Open Preferences` |
    /// | `Navigate To Preference Page` | General|Editors|Text Editors |
    pub fn open_preferences(&self) -> PyResult<()> {
        self.ensure_connected()?;

        self.send_rpc_request("rcp.openPreferences", serde_json::json!({}))?;

        Ok(())
    }

    /// Navigate to a specific preference page within the Preferences dialog.
    ///
    /// The Preferences dialog must be open before calling this keyword.
    ///
    /// | =Argument= | =Description= |
    /// | ``path`` | Preference page path with ``|`` separator. |
    ///
    /// Example:
    /// | `Open Preferences` |
    /// | `Navigate To Preference Page` | General|Appearance |
    /// | `Navigate To Preference Page` | Java|Code Style|Formatter |
    #[pyo3(signature = (path))]
    pub fn navigate_to_preference_page(&self, path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if path.is_empty() {
            return Err(SwingError::validation("Preference page path cannot be empty").into());
        }

        self.send_rpc_request("rcp.navigateToPreferencePage", serde_json::json!({
            "path": path
        }))?;

        Ok(())
    }

    // ========================
    // Additional Workbench Keywords
    // ========================

    /// Get the active workbench window.
    ///
    /// Returns information about the currently active workbench window.
    ///
    /// Example:
    /// | ${window}= | `Get Active Workbench Window` |
    /// | Log | Window title: ${window}[title] |
    pub fn get_active_workbench_window(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getActiveWorkbenchWindow", serde_json::json!({}))?;

        let dict = PyDict::new(py);
        if let Some(obj) = result.as_object() {
            for (key, value) in obj {
                dict.set_item(key, self.json_to_py(py, value)?)?;
            }
        }
        Ok(dict.into())
    }

    /// Get the number of open workbench windows.
    ///
    /// Returns the count of open workbench windows.
    ///
    /// Example:
    /// | ${count}= | `Get Workbench Window Count` |
    /// | Should Be True | ${count} >= 1 |
    pub fn get_workbench_window_count(&self) -> PyResult<i32> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getWorkbenchWindowCount", serde_json::json!({}))?;
        Ok(result.as_i64().unwrap_or(0) as i32)
    }

    /// Get the title of the workbench window.
    ///
    /// Returns the title of the active workbench window.
    ///
    /// Example:
    /// | ${title}= | `Get Workbench Title` |
    /// | Should Contain | ${title} | Eclipse |
    pub fn get_workbench_title(&self) -> PyResult<String> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getWorkbenchTitle", serde_json::json!({}))?;
        if let Some(obj) = result.as_object() {
            if let Some(title) = obj.get("title").and_then(|v| v.as_str()) {
                return Ok(title.to_string());
            }
        }
        Ok(String::new())
    }

    /// Get the current workbench state.
    ///
    /// Returns the complete state of the workbench including running status,
    /// window count, perspective, and editor/view counts.
    ///
    /// Example:
    /// | ${state}= | `Get Workbench State` |
    /// | Log | Running: ${state}[running] |
    pub fn get_workbench_state(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getWorkbenchState", serde_json::json!({}))?;

        let dict = PyDict::new(py);
        if let Some(obj) = result.as_object() {
            for (key, value) in obj {
                dict.set_item(key, self.json_to_py(py, value)?)?;
            }
        }
        Ok(dict.into())
    }

    /// Wait for the workbench to become ready.
    ///
    /// Waits until the workbench is fully initialized and responsive.
    ///
    /// | =Argument= | =Description= |
    /// | ``timeout`` | Maximum time to wait in seconds. Default ``30``. |
    ///
    /// Example:
    /// | `Wait For Workbench` | |
    /// | `Wait For Workbench` | timeout=60 |
    #[pyo3(signature = (timeout=None))]
    pub fn wait_for_workbench(&self, timeout: Option<f64>) -> PyResult<()> {
        self.ensure_connected()?;

        let timeout_ms = (timeout.unwrap_or(30.0) * 1000.0) as i64;
        let result = self.send_rpc_request("rcp.waitForWorkbench", serde_json::json!({
            "timeout": timeout_ms
        }))?;

        if let Some(obj) = result.as_object() {
            if let Some(success) = obj.get("success").and_then(|v| v.as_bool()) {
                if !success {
                    let error_msg = obj.get("error").and_then(|v| v.as_str())
                        .unwrap_or("Workbench did not become ready");
                    return Err(SwingError::timeout(error_msg, timeout.unwrap_or(30.0)).into());
                }
            }
        }
        Ok(())
    }

    // ========================
    // Additional Perspective Keywords
    // ========================

    /// Open a perspective by its display name.
    ///
    /// Searches for and opens a perspective matching the given name.
    ///
    /// | =Argument= | =Description= |
    /// | ``name`` | The display name of the perspective. |
    ///
    /// Example:
    /// | `Open Perspective By Name` | Java |
    /// | `Open Perspective By Name` | Debug |
    #[pyo3(signature = (name))]
    pub fn open_perspective_by_name(&self, name: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if name.is_empty() {
            return Err(SwingError::validation("Perspective name cannot be empty").into());
        }

        let result = self.send_rpc_request("rcp.openPerspectiveByName", serde_json::json!({
            "name": name
        }))?;

        if let Some(obj) = result.as_object() {
            if let Some(success) = obj.get("success").and_then(|v| v.as_bool()) {
                if !success {
                    let error_msg = obj.get("error").and_then(|v| v.as_str())
                        .unwrap_or("Failed to open perspective");
                    return Err(SwingError::element_not_found(error_msg).into());
                }
            }
        }
        Ok(())
    }

    /// Close the active perspective.
    ///
    /// Closes the currently active perspective.
    ///
    /// Example:
    /// | `Close Active Perspective` |
    pub fn close_active_perspective(&self) -> PyResult<()> {
        self.ensure_connected()?;

        self.send_rpc_request("rcp.closePerspective", serde_json::json!({}))?;
        Ok(())
    }

    /// Close all perspectives.
    ///
    /// Closes all open perspectives in the workbench.
    ///
    /// Example:
    /// | `Close All Perspectives` |
    pub fn close_all_perspectives(&self) -> PyResult<()> {
        self.ensure_connected()?;

        self.send_rpc_request("rcp.closeAllPerspectives", serde_json::json!({}))?;
        Ok(())
    }

    /// Get a list of currently open perspectives.
    ///
    /// Returns a list of perspective objects with ``id`` and ``label``.
    ///
    /// Example:
    /// | ${perspectives}= | `Get Open Perspectives` |
    /// | ${count}= | Get Length | ${perspectives} |
    pub fn get_open_perspectives(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getOpenPerspectives", serde_json::json!({}))?;

        let list = PyList::empty(py);
        if let Some(perspectives) = result.as_array() {
            for perspective in perspectives {
                let dict = PyDict::new(py);
                if let Some(obj) = perspective.as_object() {
                    for (key, value) in obj {
                        dict.set_item(key, self.json_to_py(py, value)?)?;
                    }
                }
                list.append(dict)?;
            }
        }
        Ok(list.into())
    }

    /// Save the current perspective with a new name.
    ///
    /// Saves the current perspective layout under a new name.
    ///
    /// | =Argument= | =Description= |
    /// | ``name`` | Name for the saved perspective. |
    ///
    /// Example:
    /// | `Save Perspective As` | MyCustomPerspective |
    #[pyo3(signature = (name))]
    pub fn save_perspective_as(&self, name: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if name.is_empty() {
            return Err(SwingError::validation("Perspective name cannot be empty").into());
        }

        self.send_rpc_request("rcp.savePerspectiveAs", serde_json::json!({
            "name": name
        }))?;
        Ok(())
    }

    // ========================
    // Additional View Keywords
    // ========================

    /// Show a view by its display name.
    ///
    /// Searches for and shows a view matching the given name.
    ///
    /// | =Argument= | =Description= |
    /// | ``name`` | The display name of the view. |
    ///
    /// Example:
    /// | `Show View By Name` | Problems |
    /// | `Show View By Name` | Package Explorer |
    #[pyo3(signature = (name))]
    pub fn show_view_by_name(&self, name: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if name.is_empty() {
            return Err(SwingError::validation("View name cannot be empty").into());
        }

        let result = self.send_rpc_request("rcp.showViewByName", serde_json::json!({
            "name": name
        }))?;

        if let Some(obj) = result.as_object() {
            if let Some(success) = obj.get("success").and_then(|v| v.as_bool()) {
                if !success {
                    let error_msg = obj.get("error").and_then(|v| v.as_str())
                        .unwrap_or("Failed to show view");
                    return Err(SwingError::element_not_found(error_msg).into());
                }
            }
        }
        Ok(())
    }

    /// Check if a view is visible.
    ///
    /// Returns ``True`` if the view is currently visible.
    ///
    /// | =Argument= | =Description= |
    /// | ``view_id`` | The view ID to check. |
    ///
    /// Example:
    /// | ${visible}= | `Is View Visible` | org.eclipse.jdt.ui.PackageExplorer |
    /// | Should Be True | ${visible} |
    #[pyo3(signature = (view_id))]
    pub fn is_view_visible(&self, view_id: &str) -> PyResult<bool> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.isViewVisible", serde_json::json!({
            "viewId": view_id
        }))?;

        Ok(result.as_bool().unwrap_or(false))
    }

    /// Get the currently active view.
    ///
    /// Returns the ID of the currently active view, or empty string if no view is active.
    ///
    /// Example:
    /// | ${view}= | `Get Active View` |
    pub fn get_active_view(&self) -> PyResult<String> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getActiveView", serde_json::json!({}))?;

        if let Some(obj) = result.as_object() {
            if obj.get("isView").and_then(|v| v.as_bool()).unwrap_or(false) {
                if let Some(id) = obj.get("id").and_then(|v| v.as_str()) {
                    return Ok(id.to_string());
                }
            }
        }
        Ok(String::new())
    }

    /// Minimize a view.
    ///
    /// Minimizes the specified view.
    ///
    /// | =Argument= | =Description= |
    /// | ``view_id`` | The view ID to minimize. |
    ///
    /// Example:
    /// | `Minimize View` | org.eclipse.ui.views.ContentOutline |
    #[pyo3(signature = (view_id))]
    pub fn minimize_view(&self, view_id: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if view_id.is_empty() {
            return Err(SwingError::validation("View ID cannot be empty").into());
        }

        self.send_rpc_request("rcp.minimizeView", serde_json::json!({
            "viewId": view_id
        }))?;
        Ok(())
    }

    /// Maximize a view.
    ///
    /// Maximizes the specified view to full workbench size.
    ///
    /// | =Argument= | =Description= |
    /// | ``view_id`` | The view ID to maximize. |
    ///
    /// Example:
    /// | `Maximize View` | org.eclipse.jdt.ui.PackageExplorer |
    #[pyo3(signature = (view_id))]
    pub fn maximize_view(&self, view_id: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if view_id.is_empty() {
            return Err(SwingError::validation("View ID cannot be empty").into());
        }

        self.send_rpc_request("rcp.maximizeView", serde_json::json!({
            "viewId": view_id
        }))?;
        Ok(())
    }

    /// Restore a view to normal state.
    ///
    /// Restores a minimized or maximized view to its normal state.
    ///
    /// | =Argument= | =Description= |
    /// | ``view_id`` | The view ID to restore. |
    ///
    /// Example:
    /// | `Restore View` | org.eclipse.jdt.ui.PackageExplorer |
    #[pyo3(signature = (view_id))]
    pub fn restore_view(&self, view_id: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if view_id.is_empty() {
            return Err(SwingError::validation("View ID cannot be empty").into());
        }

        self.send_rpc_request("rcp.restoreView", serde_json::json!({
            "viewId": view_id
        }))?;
        Ok(())
    }

    /// Check if a view is minimized.
    ///
    /// Returns ``True`` if the view is currently minimized.
    ///
    /// | =Argument= | =Description= |
    /// | ``view_id`` | The view ID to check. |
    ///
    /// Example:
    /// | ${minimized}= | `Is View Minimized` | org.eclipse.ui.views.ContentOutline |
    #[pyo3(signature = (view_id))]
    pub fn is_view_minimized(&self, view_id: &str) -> PyResult<bool> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.isViewMinimized", serde_json::json!({
            "viewId": view_id
        }))?;

        Ok(result.as_bool().unwrap_or(false))
    }

    /// Check if a view is maximized.
    ///
    /// Returns ``True`` if the view is currently maximized.
    ///
    /// | =Argument= | =Description= |
    /// | ``view_id`` | The view ID to check. |
    ///
    /// Example:
    /// | ${maximized}= | `Is View Maximized` | org.eclipse.jdt.ui.PackageExplorer |
    #[pyo3(signature = (view_id))]
    pub fn is_view_maximized(&self, view_id: &str) -> PyResult<bool> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.isViewMaximized", serde_json::json!({
            "viewId": view_id
        }))?;

        Ok(result.as_bool().unwrap_or(false))
    }

    /// Get the title of a view.
    ///
    /// Returns the display title of the specified view.
    ///
    /// | =Argument= | =Description= |
    /// | ``view_id`` | The view ID. |
    ///
    /// Example:
    /// | ${title}= | `Get View Title` | org.eclipse.jdt.ui.PackageExplorer |
    #[pyo3(signature = (view_id))]
    pub fn get_view_title(&self, view_id: &str) -> PyResult<String> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getViewTitle", serde_json::json!({
            "viewId": view_id
        }))?;

        if let Some(obj) = result.as_object() {
            if let Some(title) = obj.get("title").and_then(|v| v.as_str()) {
                return Ok(title.to_string());
            }
        }
        Ok(String::new())
    }

    // ========================
    // Additional Editor Keywords
    // ========================

    /// Check if an editor is open.
    ///
    /// Returns ``True`` if an editor is open for the specified file.
    ///
    /// | =Argument= | =Description= |
    /// | ``file_path`` | Path to the file. |
    ///
    /// Example:
    /// | ${open}= | `Is Editor Open` | /project/src/Main.java |
    #[pyo3(signature = (file_path))]
    pub fn is_editor_open(&self, file_path: &str) -> PyResult<bool> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.isEditorOpen", serde_json::json!({
            "filePath": file_path
        }))?;

        Ok(result.as_bool().unwrap_or(false))
    }

    /// Check if an editor has unsaved changes.
    ///
    /// Returns ``True`` if the editor has unsaved changes.
    ///
    /// | =Argument= | =Description= |
    /// | ``file_path`` | Path to the file. |
    ///
    /// Example:
    /// | ${dirty}= | `Is Editor Dirty` | /project/src/Main.java |
    #[pyo3(signature = (file_path))]
    pub fn is_editor_dirty(&self, file_path: &str) -> PyResult<bool> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.isEditorDirty", serde_json::json!({
            "filePath": file_path
        }))?;

        Ok(result.as_bool().unwrap_or(false))
    }

    /// Get the content of an editor.
    ///
    /// Returns the text content of the specified editor.
    ///
    /// | =Argument= | =Description= |
    /// | ``file_path`` | Path to the file. |
    ///
    /// Example:
    /// | ${content}= | `Get Editor Content` | /project/src/Main.java |
    #[pyo3(signature = (file_path))]
    pub fn get_editor_content(&self, file_path: &str) -> PyResult<String> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getEditorContent", serde_json::json!({
            "filePath": file_path
        }))?;

        if let Some(obj) = result.as_object() {
            if let Some(content) = obj.get("content").and_then(|v| v.as_str()) {
                return Ok(content.to_string());
            }
        }
        Ok(String::new())
    }

    /// Get the count of editors with unsaved changes.
    ///
    /// Returns the number of dirty editors.
    ///
    /// Example:
    /// | ${count}= | `Get Dirty Editor Count` |
    pub fn get_dirty_editor_count(&self) -> PyResult<i32> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getDirtyEditorCount", serde_json::json!({}))?;
        Ok(result.as_i64().unwrap_or(0) as i32)
    }

    /// Enter text in the active editor.
    ///
    /// Inserts text at the current cursor position in the active editor.
    ///
    /// | =Argument= | =Description= |
    /// | ``text`` | Text to insert. |
    ///
    /// Example:
    /// | `Enter Text In Editor` | // New comment |
    #[pyo3(signature = (text))]
    pub fn enter_text_in_editor(&self, text: &str) -> PyResult<()> {
        self.ensure_connected()?;

        self.send_rpc_request("rcp.enterTextInEditor", serde_json::json!({
            "text": text
        }))?;
        Ok(())
    }

    // ========================
    // Additional Dialog Keywords
    // ========================

    /// Get a list of open dialogs.
    ///
    /// Returns a list of dialog objects with ``title``, ``visible``, and ``modal`` properties.
    ///
    /// Example:
    /// | ${dialogs}= | `Get Open Dialogs` |
    pub fn get_open_dialogs(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.ensure_connected()?;

        let result = self.send_rpc_request("rcp.getOpenDialogs", serde_json::json!({}))?;

        let list = PyList::empty(py);
        if let Some(dialogs) = result.as_array() {
            for dialog in dialogs {
                let dict = PyDict::new(py);
                if let Some(obj) = dialog.as_object() {
                    for (key, value) in obj {
                        dict.set_item(key, self.json_to_py(py, value)?)?;
                    }
                }
                list.append(dict)?;
            }
        }
        Ok(list.into())
    }

    /// Execute a menu command.
    ///
    /// Executes a menu path (e.g., ``File|Refresh``).
    ///
    /// | =Argument= | =Description= |
    /// | ``menu_path`` | Menu path with ``|`` separator. |
    ///
    /// Example:
    /// | `Execute Menu` | File|Refresh |
    #[pyo3(signature = (menu_path))]
    pub fn execute_menu(&self, menu_path: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if menu_path.is_empty() {
            return Err(SwingError::validation("Menu path cannot be empty").into());
        }

        self.send_rpc_request("rcp.executeMenu", serde_json::json!({
            "menuPath": menu_path
        }))?;
        Ok(())
    }

    // ========================
    // Button Keywords
    // ========================

    /// Press a button by its text label.
    ///
    /// Clicks a button widget identified by its label text.
    /// Commonly used for dialog buttons like OK, Cancel, Apply, etc.
    ///
    /// | =Argument= | =Description= |
    /// | ``label`` | The text label of the button to press. |
    ///
    /// Example:
    /// | `Press Button` | OK |
    /// | `Press Button` | Cancel |
    /// | `Press Button` | Apply |
    #[pyo3(signature = (label))]
    pub fn press_button(&self, label: &str) -> PyResult<()> {
        self.ensure_connected()?;

        if label.is_empty() {
            return Err(SwingError::validation("Button label cannot be empty").into());
        }

        self.send_rpc_request("rcp.pressButton", serde_json::json!({
            "label": label
        }))?;
        Ok(())
    }

    /// Close the currently active dialog.
    ///
    /// Closes the topmost dialog window if one is open.
    ///
    /// Example:
    /// | `Close Active Dialog` |
    #[pyo3(signature = ())]
    pub fn close_active_dialog(&self) -> PyResult<()> {
        self.ensure_connected()?;
        self.send_rpc_request("rcp.closeActiveDialog", serde_json::json!({}))?;
        Ok(())
    }
}

// Private implementation methods
impl RcpLibrary {
    /// Ensure we're connected to an application
    fn ensure_connected(&self) -> PyResult<()> {
        if !self.swt_lib.is_connected()? {
            return Err(SwingError::connection("Not connected to any RCP application").into());
        }
        Ok(())
    }

    /// Send a JSON-RPC request (delegated to SwtLibrary's public method)
    fn send_rpc_request(&self, method: &str, params: serde_json::Value) -> PyResult<serde_json::Value> {
        // Delegate to the underlying SwtLibrary's public send_rpc_request method
        self.swt_lib.send_rpc_request(method, params)
    }

    /// Get widget ID by locator (delegated)
    fn get_widget_id(&self, locator: &str) -> Result<i64, SwingError> {
        let widget = self.swt_lib.find_widget(locator)
            .map_err(|_| SwingError::element_not_found(locator))?;
        Ok(widget.hash_code)
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

    /// Convert JSON value to Python object
    fn json_to_py(&self, py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
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

impl Default for RcpLibrary {
    fn default() -> Self {
        Self::new(None)
    }
}
