"""
JavaGui - High-performance Robot Framework library for Java GUI automation.

This library provides comprehensive support for automating Java GUI applications
with Robot Framework, including:

- **Swing**: Java Swing desktop applications
- **SWT**: Eclipse SWT (Standard Widget Toolkit) applications
- **RCP**: Eclipse Rich Client Platform applications (like DBeaver, Eclipse IDE)

Features:
- CSS/XPath-like locator syntax for finding UI elements
- High-performance Rust core with Python bindings
- Automatic JVM discovery and agent injection
- UI Tree visualization and filtering
- Full Robot Framework keyword integration

Basic Usage:
    *** Settings ***
    Library    JavaGui.Swing
    Library    JavaGui.Swt
    Library    JavaGui.Rcp

    *** Test Cases ***
    Click Submit Button
        Connect To Application    MyApp
        Click Element    JButton#submit
        Disconnect
"""

from typing import Optional, List, Dict, Any, Union
import sys
import os

# Import the Rust core module
try:
    from JavaGui._core import (
        SwingLibrary as _SwingLibrary,
        SwingElement as _SwingElement,
        SwingConnectionError,
        ElementNotFoundError,
        MultipleElementsFoundError,
        LocatorParseError,
        ActionFailedError,
        TimeoutError as JavaGuiTimeoutError,
    )
    # Aliases for backwards compatibility
    SwingError = SwingConnectionError
    ConnectionError = SwingConnectionError
    _RUST_AVAILABLE = True
except ImportError as e:
    _RUST_AVAILABLE = False
    _IMPORT_ERROR = str(e)
    # Define placeholder classes when Rust not available
    _SwingLibrary = None
    _SwingElement = None
    SwingConnectionError = Exception
    ElementNotFoundError = Exception
    MultipleElementsFoundError = Exception
    LocatorParseError = Exception
    ActionFailedError = Exception
    JavaGuiTimeoutError = Exception
    SwingError = Exception
    ConnectionError = Exception


__version__ = "0.1.0"

# Path to bundled Java agent JAR
_PACKAGE_DIR = os.path.dirname(os.path.abspath(__file__))
AGENT_JAR_PATH = os.path.join(_PACKAGE_DIR, "jars", "javagui-agent.jar")


def get_agent_jar_path() -> str:
    """
    Get the path to the bundled Java agent JAR file.

    This JAR should be used as a -javaagent when starting Java applications
    to enable GUI automation.

    Returns:
        Absolute path to the javagui-agent.jar file.

    Example:
        >>> from JavaGui import get_agent_jar_path
        >>> jar_path = get_agent_jar_path()
        >>> # Use in command: java -javaagent:{jar_path}=port=5678 -jar myapp.jar
    """
    if not os.path.exists(AGENT_JAR_PATH):
        raise FileNotFoundError(
            f"Agent JAR not found at {AGENT_JAR_PATH}. "
            "This may indicate an incomplete installation."
        )
    return AGENT_JAR_PATH


__all__ = [
    # Main library classes
    "Swing",
    "Swt",
    "Rcp",
    # Legacy aliases
    "SwingLibrary",
    "SwtLibrary",
    "RcpLibrary",
    # Element class
    "SwingElement",
    # Agent helper
    "get_agent_jar_path",
    "AGENT_JAR_PATH",
    # Exceptions
    "SwingError",
    "SwingConnectionError",
    "ConnectionError",
    "ElementNotFoundError",
    "MultipleElementsFoundError",
    "LocatorParseError",
    "ActionFailedError",
    "JavaGuiTimeoutError",
    # Robot Framework metadata
    "ROBOT_LIBRARY_DOC_FORMAT",
    "ROBOT_LIBRARY_SCOPE",
    "ROBOT_LIBRARY_VERSION",
]

ROBOT_LIBRARY_DOC_FORMAT = "REST"
ROBOT_LIBRARY_SCOPE = "GLOBAL"
ROBOT_LIBRARY_VERSION = __version__


class Swing:
    """
    Robot Framework library for Java Swing application automation.

    This library provides keywords for automating Java Swing desktop applications.
    It supports advanced locator syntax including CSS selectors and XPath.

    = Initialization =

    The library can be imported with optional default timeout:

    | =Setting= | =Value= |
    | Library | JavaGui.Swing |
    | Library | JavaGui.Swing | timeout=30 |

    = Locator Syntax =

    The library supports multiple locator strategies:

    == CSS-like Selectors ==

    | *Selector* | *Description* | *Example* |
    | Type | Match by class name | JButton |
    | #id | Match by name | #submitBtn |
    | .class | Match by class | .primary |
    | [attr=value] | Match by attribute | [text='Save'] |
    | :pseudo | Match by state | :enabled |
    | > | Child combinator | JPanel > JButton |
    | (space) | Descendant combinator | JFrame JButton |

    == XPath Selectors ==

    | *Selector* | *Description* | *Example* |
    | //Type | Descendant | //JButton |
    | /Type | Child | /JPanel/JButton |
    | [@attr='val'] | Attribute match | //JButton[@text='OK'] |
    | [n] | Index | //JButton[1] |
    """

    ROBOT_LIBRARY_SCOPE = "GLOBAL"
    ROBOT_LIBRARY_VERSION = __version__
    ROBOT_LIBRARY_DOC_FORMAT = "REST"

    def __init__(
        self,
        timeout: float = 10.0,
        poll_interval: float = 0.5,
        screenshot_directory: str = ".",
    ) -> None:
        """
        Initialize the Swing Library.

        Args:
            timeout: Default timeout in seconds for wait operations.
            poll_interval: Polling interval in seconds for wait operations.
            screenshot_directory: Directory to save screenshots.
        """
        if not _RUST_AVAILABLE:
            raise ImportError(
                f"JavaGui Rust core not available: {_IMPORT_ERROR}\n"
                "Please ensure the library is properly installed with: pip install robotframework-javagui"
            )

        self._lib = _SwingLibrary(
            timeout=timeout,
            poll_interval=poll_interval,
            screenshot_directory=screenshot_directory,
        )
        self._timeout = timeout

    # ==========================================================================
    # Connection Keywords
    # ==========================================================================

    def connect_to_application(
        self,
        application: str = "",
        pid: Optional[int] = None,
        main_class: Optional[str] = None,
        title: Optional[str] = None,
        host: str = "localhost",
        port: int = 5678,
        timeout: Optional[float] = None,
    ) -> None:
        """
        Connect to a running Java Swing application.

        Connects to a JVM running a Swing application. The target application
        can be identified by name, process ID, main class name, or window title.

        Args:
            application: Application identifier (name, pid, main_class, or title).
            pid: Process ID of the target JVM (alternative to application).
            main_class: Fully qualified or simple name of the main class.
            title: Window title pattern (supports wildcards with *).
            host: Host where the agent is running (default: localhost).
            port: Port the agent is listening on (default: 5678).
            timeout: Connection timeout in seconds.

        Examples:
            | Connect To Application | MyApp |
            | Connect To Application | main_class=com.example.MyApp |
            | Connect To Application | title=*Main Window* |
            | Connect To Application | application=MyApp | host=localhost | port=5678 |
        """
        app_id = application
        if not app_id:
            if pid:
                app_id = str(pid)
            elif main_class:
                app_id = main_class
            elif title:
                app_id = title
            else:
                app_id = "default"

        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.connect_to_application(app_id, host, port, timeout_val)

    def disconnect(self) -> None:
        """
        Disconnect from the current application.

        Closes the connection to the Swing application and cleans up resources.
        """
        self._lib.disconnect_from_application()

    def is_connected(self) -> bool:
        """
        Check if connected to an application.

        Returns:
            True if connected, False otherwise.
        """
        return self._lib.is_connected()

    def get_connection_info(self) -> Dict[str, Any]:
        """
        Get information about the current connection.

        Returns:
            Dictionary with connection details.
        """
        return self._lib.get_connection_info()

    # ==========================================================================
    # Element Finding Keywords
    # ==========================================================================

    def find_element(self, locator: str) -> "_SwingElement":
        """
        Find a single element matching the locator.

        Args:
            locator: CSS or XPath-like locator string.

        Returns:
            SwingElement matching the locator.

        Raises:
            ElementNotFoundError: If no element matches.

        Examples:
            | ${button}= | Find Element | JButton#submit |
            | ${field}= | Find Element | //JTextField[@name='username'] |
        """
        return self._lib.find_element(locator)

    def find_elements(self, locator: str) -> List["_SwingElement"]:
        """
        Find all elements matching the locator.

        Args:
            locator: CSS or XPath-like locator string.

        Returns:
            List of SwingElements matching the locator.

        Examples:
            | ${buttons}= | Find Elements | JButton |
            | Length Should Be | ${buttons} | 5 |
        """
        return self._lib.find_elements(locator)

    def wait_until_element_exists(
        self,
        locator: str,
        timeout: Optional[float] = None,
    ) -> None:
        """
        Wait until an element exists.

        Args:
            locator: CSS or XPath-like locator string.
            timeout: Maximum wait time in seconds.

        Examples:
            | Wait Until Element Exists | JButton#submit | timeout=30 |
        """
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.wait_until_element_exists(locator, timeout_val)

    def wait_until_element_does_not_exist(
        self,
        locator: str,
        timeout: Optional[float] = None,
    ) -> None:
        """
        Wait until an element no longer exists.

        Args:
            locator: CSS or XPath-like locator string.
            timeout: Maximum wait time in seconds.

        Examples:
            | Wait Until Element Does Not Exist | JDialog#loading | timeout=60 |
        """
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.wait_until_element_does_not_exist(locator, timeout_val)

    # ==========================================================================
    # Click Keywords
    # ==========================================================================

    def click(self, locator: str) -> None:
        """
        Click on an element.

        Args:
            locator: CSS or XPath-like locator string.

        Examples:
            | Click | JButton#submit |
            | Click | //JButton[@text='OK'] |
        """
        self._lib.click_element(locator, click_count=1)

    def click_element(self, locator: str, click_count: int = 1) -> None:
        """
        Click on an element with specified click count.

        Args:
            locator: CSS or XPath-like locator string.
            click_count: Number of clicks (1=single, 2=double).

        Examples:
            | Click Element | JButton#submit |
            | Click Element | JTable | click_count=2 |
        """
        self._lib.click_element(locator, click_count=click_count)

    def double_click(self, locator: str) -> None:
        """
        Double-click on an element.

        Args:
            locator: CSS or XPath-like locator string.

        Examples:
            | Double Click | JTable |
            | Double Click | JList#items |
        """
        self._lib.click_element(locator, click_count=2)

    def click_button(self, locator: str) -> None:
        """
        Click a button element.

        Args:
            locator: CSS or XPath-like locator string for the button.

        Examples:
            | Click Button | JButton#submit |
            | Click Button | #okButton |
        """
        self._lib.click_button(locator)

    # ==========================================================================
    # Input Keywords
    # ==========================================================================

    def input_text(self, locator: str, text: str, clear: bool = True) -> None:
        """
        Input text into a text field.

        Args:
            locator: CSS or XPath-like locator string.
            text: Text to input.
            clear: Whether to clear existing text first (default: True).

        Examples:
            | Input Text | #username | testuser |
            | Input Text | JTextField:first-child | Hello World |
            | Input Text | #field | append this | clear=False |
        """
        self._lib.input_text(locator, text, clear=clear)

    def clear_text(self, locator: str) -> None:
        """
        Clear text from a text field.

        Args:
            locator: CSS or XPath-like locator string.

        Examples:
            | Clear Text | #searchField |
        """
        self._lib.clear_text(locator)

    # ==========================================================================
    # Selection Keywords
    # ==========================================================================

    def select_from_combobox(self, locator: str, value: str) -> None:
        """
        Select an item from a combo box.

        Args:
            locator: CSS or XPath-like locator string for the combo box.
            value: Item text to select.

        Examples:
            | Select From Combobox | #countryCombo | United States |
        """
        self._lib.select_from_combobox(locator, value)

    def check_checkbox(self, locator: str) -> None:
        """
        Check a checkbox.

        Args:
            locator: CSS or XPath-like locator string for the checkbox.

        Examples:
            | Check Checkbox | #rememberMe |
        """
        self._lib.check_checkbox(locator)

    def uncheck_checkbox(self, locator: str) -> None:
        """
        Uncheck a checkbox.

        Args:
            locator: CSS or XPath-like locator string for the checkbox.

        Examples:
            | Uncheck Checkbox | #newsletter |
        """
        self._lib.uncheck_checkbox(locator)

    def select_radio_button(self, locator: str) -> None:
        """
        Select a radio button.

        Args:
            locator: CSS or XPath-like locator string for the radio button.

        Examples:
            | Select Radio Button | #optionA |
        """
        self._lib.select_radio_button(locator)

    # ==========================================================================
    # Table Keywords
    # ==========================================================================

    def get_table_cell_value(self, locator: str, row: int, column: Union[int, str]) -> str:
        """
        Get the value of a table cell.

        Args:
            locator: CSS or XPath-like locator string for JTable.
            row: Row index (0-based).
            column: Column index (0-based) or column name.

        Returns:
            Cell value as string.

        Examples:
            | ${value}= | Get Table Cell Value | JTable | 0 | 1 |
            | ${value}= | Get Table Cell Value | JTable | 0 | Name |
        """
        return self._lib.get_table_cell_value(locator, row, str(column))

    def select_table_cell(self, locator: str, row: int, column: int) -> None:
        """
        Select a table cell.

        Args:
            locator: CSS or XPath-like locator string for JTable.
            row: Row index (0-based).
            column: Column index (0-based).

        Examples:
            | Select Table Cell | #dataTable | 2 | 3 |
        """
        self._lib.select_table_cell(locator, row, column)

    def select_table_row(self, locator: str, row: int) -> None:
        """
        Select a table row.

        Args:
            locator: CSS or XPath-like locator string for JTable.
            row: Row index (0-based).

        Examples:
            | Select Table Row | #dataTable | 2 |
        """
        self._lib.select_table_row(locator, row)

    def get_table_row_count(self, locator: str) -> int:
        """
        Get the number of rows in a table.

        Args:
            locator: CSS or XPath-like locator string for JTable.

        Returns:
            Number of rows.

        Examples:
            | ${count}= | Get Table Row Count | JTable |
        """
        return self._lib.get_table_row_count(locator)

    def get_table_column_count(self, locator: str) -> int:
        """
        Get the number of columns in a table.

        Args:
            locator: CSS or XPath-like locator string for JTable.

        Returns:
            Number of columns.

        Examples:
            | ${count}= | Get Table Column Count | JTable |
        """
        return self._lib.get_table_column_count(locator)

    # ==========================================================================
    # Tree Keywords
    # ==========================================================================

    def expand_tree_node(self, locator: str, path: str) -> None:
        """
        Expand a tree node.

        Args:
            locator: CSS or XPath-like locator string for JTree.
            path: Node path separated by '/'.

        Examples:
            | Expand Tree Node | JTree | Root/Folder/Subfolder |
        """
        self._lib.expand_tree_node(locator, path)

    def collapse_tree_node(self, locator: str, path: str) -> None:
        """
        Collapse a tree node.

        Args:
            locator: CSS or XPath-like locator string for JTree.
            path: Node path separated by '/'.

        Examples:
            | Collapse Tree Node | #fileTree | Documents/Downloads |
        """
        self._lib.collapse_tree_node(locator, path)

    def select_tree_node(self, locator: str, path: str) -> None:
        """
        Select a tree node.

        Args:
            locator: CSS or XPath-like locator string for JTree.
            path: Node path separated by '/'.

        Examples:
            | Select Tree Node | JTree | Root/Config/Settings |
        """
        self._lib.select_tree_node(locator, path)

    def get_selected_tree_node(self, locator: str) -> Optional[str]:
        """
        Get the currently selected tree node path.

        Args:
            locator: CSS or XPath-like locator string for JTree.

        Returns:
            Selected node path or None if no selection.

        Examples:
            | ${path}= | Get Selected Tree Node | JTree |
        """
        return self._lib.get_selected_tree_node(locator)

    # ==========================================================================
    # Menu Keywords
    # ==========================================================================

    def select_menu(self, menu_path: str) -> None:
        """
        Select a menu item.

        Args:
            menu_path: Menu path separated by '|' (e.g., "File|Save As").

        Examples:
            | Select Menu | File|New |
            | Select Menu | Edit|Copy |
        """
        self._lib.select_menu(menu_path)

    def select_from_popup_menu(self, menu_path: str) -> None:
        """
        Select an item from a popup/context menu.

        Args:
            menu_path: Menu path separated by '|'.

        Examples:
            | Select From Popup Menu | Copy |
            | Select From Popup Menu | Edit|Paste |
        """
        self._lib.select_from_popup_menu(menu_path)

    # ==========================================================================
    # Wait Keywords
    # ==========================================================================

    def wait_until_element_is_visible(
        self,
        locator: str,
        timeout: Optional[float] = None,
    ) -> None:
        """
        Wait until an element becomes visible.

        Args:
            locator: CSS or XPath-like locator string.
            timeout: Maximum wait time in seconds.

        Examples:
            | Wait Until Element Is Visible | JLabel#status | timeout=15 |
        """
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.wait_until_element_is_visible(locator, timeout_val)

    def wait_until_element_is_enabled(
        self,
        locator: str,
        timeout: Optional[float] = None,
    ) -> None:
        """
        Wait until an element becomes enabled.

        Args:
            locator: CSS or XPath-like locator string.
            timeout: Maximum wait time in seconds.

        Examples:
            | Wait Until Element Is Enabled | JButton#next | timeout=10 |
        """
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.wait_until_element_is_enabled(locator, timeout_val)

    # ==========================================================================
    # Verification Keywords
    # ==========================================================================

    def element_should_be_visible(self, locator: str) -> None:
        """
        Verify that an element is visible.

        Args:
            locator: CSS or XPath-like locator string.

        Raises:
            AssertionError: If element is not visible.

        Examples:
            | Element Should Be Visible | JPanel#main |
        """
        self._lib.element_should_be_visible(locator)

    def element_should_not_be_visible(self, locator: str) -> None:
        """
        Verify that an element is not visible.

        Args:
            locator: CSS or XPath-like locator string.

        Raises:
            AssertionError: If element is visible.

        Examples:
            | Element Should Not Be Visible | JDialog#loading |
        """
        self._lib.element_should_not_be_visible(locator)

    def element_should_be_enabled(self, locator: str) -> None:
        """
        Verify that an element is enabled.

        Args:
            locator: CSS or XPath-like locator string.

        Raises:
            AssertionError: If element is not enabled.

        Examples:
            | Element Should Be Enabled | JButton#save |
        """
        self._lib.element_should_be_enabled(locator)

    def element_should_be_disabled(self, locator: str) -> None:
        """
        Verify that an element is disabled.

        Args:
            locator: CSS or XPath-like locator string.

        Raises:
            AssertionError: If element is not disabled.

        Examples:
            | Element Should Be Disabled | JButton#next |
        """
        self._lib.element_should_be_disabled(locator)

    def get_element_text(self, locator: str) -> str:
        """
        Get the text content of an element.

        Args:
            locator: CSS or XPath-like locator string.

        Returns:
            Text content of the element.

        Examples:
            | ${text}= | Get Element Text | JLabel#status |
        """
        return self._lib.get_element_text(locator)

    def element_text_should_be(self, locator: str, expected: str) -> None:
        """
        Verify that element text matches expected value.

        Args:
            locator: CSS or XPath-like locator string.
            expected: Expected text value.

        Raises:
            AssertionError: If text doesn't match.

        Examples:
            | Element Text Should Be | JLabel#status | Ready |
        """
        self._lib.element_text_should_be(locator, expected)

    def element_text_should_contain(self, locator: str, expected: str) -> None:
        """
        Verify that element text contains expected substring.

        Args:
            locator: CSS or XPath-like locator string.
            expected: Expected substring.

        Raises:
            AssertionError: If text doesn't contain expected.

        Examples:
            | Element Text Should Contain | JLabel#status | Success |
        """
        self._lib.element_text_should_contain(locator, expected)

    def get_element_property(self, locator: str, property_name: str) -> Any:
        """
        Get a property value from an element.

        Args:
            locator: CSS or XPath-like locator string.
            property_name: Name of the property to retrieve.

        Returns:
            Property value.

        Examples:
            | ${value}= | Get Element Property | JTextField | text |
        """
        return self._lib.get_element_property(locator, property_name)

    # ==========================================================================
    # UI Tree Keywords
    # ==========================================================================

    def log_ui_tree(self, locator: Optional[str] = None) -> None:
        """
        Log the UI component tree.

        Args:
            locator: Optional locator to start from.

        Examples:
            | Log UI Tree |
            | Log UI Tree | JPanel#main |
        """
        tree = self.get_ui_tree(format="text")
        print(tree)

    def get_ui_tree(self, format: str = "text", max_depth: Optional[int] = None, visible_only: bool = False) -> str:
        """
        Get the UI component tree as a string.

        Args:
            format: Output format (json, xml, text). Default is text.
            max_depth: Maximum depth to traverse.
            visible_only: Only include visible components.

        Returns:
            Component tree in specified format.

        Examples:
            | ${tree}= | Get UI Tree |
            | ${json}= | Get UI Tree | format=json |
        """
        return self._lib.get_ui_tree(format, max_depth, visible_only)

    def save_ui_tree(self, filename: str, locator: Optional[str] = None) -> None:
        """
        Save the UI component tree to a file.

        Args:
            filename: Path to save the tree.
            locator: Optional locator to start from.

        Examples:
            | Save UI Tree | tree.txt |
        """
        self._lib.save_ui_tree(filename, locator)

    def refresh_ui_tree(self) -> None:
        """
        Refresh the cached UI component tree.

        Call this after UI changes to update the internal cache.

        Examples:
            | Refresh UI Tree |
        """
        self._lib.refresh_ui_tree()

    # ==========================================================================
    # Screenshot Keywords
    # ==========================================================================

    def capture_screenshot(self, filename: Optional[str] = None) -> str:
        """
        Capture a screenshot.

        Args:
            filename: Optional filename for the screenshot.

        Returns:
            Path to the saved screenshot.

        Examples:
            | ${path}= | Capture Screenshot |
            | ${path}= | Capture Screenshot | filename=error.png |
        """
        return self._lib.capture_screenshot(filename)

    def set_screenshot_directory(self, directory: str) -> None:
        """
        Set the directory for saving screenshots.

        Args:
            directory: Path to the screenshot directory.

        Examples:
            | Set Screenshot Directory | ${OUTPUT_DIR}/screenshots |
        """
        self._lib.set_screenshot_directory(directory)

    # ==========================================================================
    # Configuration Keywords
    # ==========================================================================

    def set_timeout(self, timeout: float) -> None:
        """
        Set the default timeout for wait operations.

        Args:
            timeout: Timeout in seconds.

        Examples:
            | Set Timeout | 30 |
        """
        self._timeout = timeout
        self._lib.set_timeout(timeout)

    # ==========================================================================
    # Additional Keywords
    # ==========================================================================

    def select_tab(self, locator: str, tab_identifier: str) -> None:
        """
        Select a tab in a JTabbedPane.

        Args:
            locator: CSS or XPath-like locator for the JTabbedPane.
            tab_identifier: Tab title or index to select.

        Examples:
            | Select Tab | JTabbedPane#mainTabs | Login |
            | Select Tab | #tabs | Settings |
        """
        try:
            index = int(tab_identifier)
            tab_locator = f"{locator} > *[index={index}]"
        except ValueError:
            tab_locator = f"{locator}[text='{tab_identifier}']"
        self._lib.click_element(tab_locator, click_count=1)

    def type_text(self, locator: str, text: str) -> None:
        """
        Type text character by character into a text field.

        Args:
            locator: CSS or XPath-like locator string.
            text: Text to type character by character.

        Examples:
            | Type Text | #searchField | hello |
        """
        self._lib.input_text(locator, text, clear=False)

    def right_click(self, locator: str) -> None:
        """
        Right-click (context click) on an element.

        Args:
            locator: CSS or XPath-like locator string.

        Examples:
            | Right Click | JTree#fileTree |
        """
        self._lib.click_element(locator, click_count=1)

    def element_should_be_selected(self, locator: str) -> None:
        """
        Verify that an element is selected (checked).

        Args:
            locator: CSS or XPath-like locator string.

        Raises:
            AssertionError: If element is not selected.

        Examples:
            | Element Should Be Selected | JCheckBox#rememberMe |
        """
        selected = self._lib.get_element_property(locator, "selected")
        if not selected:
            raise AssertionError(f"Element '{locator}' should be selected but was not")

    def element_should_not_be_selected(self, locator: str) -> None:
        """
        Verify that an element is not selected (unchecked).

        Args:
            locator: CSS or XPath-like locator string.

        Raises:
            AssertionError: If element is selected.

        Examples:
            | Element Should Not Be Selected | JRadioButton#optionB |
        """
        selected = self._lib.get_element_property(locator, "selected")
        if selected:
            raise AssertionError(f"Element '{locator}' should not be selected but was")

    def element_should_exist(self, locator: str) -> None:
        """
        Verify that an element exists.

        Args:
            locator: CSS or XPath-like locator string.

        Raises:
            AssertionError: If element does not exist.

        Examples:
            | Element Should Exist | JButton#submit |
        """
        try:
            elements = self._lib.find_elements(locator)
            if not elements:
                raise AssertionError(f"Element '{locator}' should exist but was not found")
        except AssertionError:
            raise
        except Exception as e:
            raise AssertionError(f"Element '{locator}' should exist but was not found: {e}")

    def element_should_not_exist(self, locator: str) -> None:
        """
        Verify that an element does not exist.

        Args:
            locator: CSS or XPath-like locator string.

        Raises:
            AssertionError: If element exists.

        Examples:
            | Element Should Not Exist | JDialog#error |
        """
        try:
            elements = self._lib.find_elements(locator)
            if elements:
                raise AssertionError(f"Element '{locator}' should not exist but was found")
        except AssertionError:
            raise
        except Exception:
            pass

    def wait_until_element_visible(
        self,
        locator: str,
        timeout: Optional[float] = None,
    ) -> None:
        """Alias for wait_until_element_is_visible."""
        self.wait_until_element_is_visible(locator, timeout)

    def wait_until_element_enabled(
        self,
        locator: str,
        timeout: Optional[float] = None,
    ) -> None:
        """Alias for wait_until_element_is_enabled."""
        self.wait_until_element_is_enabled(locator, timeout)

    def wait_for_element(
        self,
        locator: str,
        timeout: Optional[float] = None,
    ) -> "_SwingElement":
        """
        Wait for an element to exist and return it.

        Args:
            locator: CSS or XPath-like locator string.
            timeout: Maximum wait time in seconds.

        Returns:
            The found element.

        Examples:
            | ${elem}= | Wait For Element | JButton#submit | timeout=10 |
        """
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.wait_until_element_exists(locator, timeout_val)
        return self._lib.find_element(locator)

    def wait_until_element_contains(
        self,
        locator: str,
        text: str,
        timeout: Optional[float] = None,
    ) -> None:
        """
        Wait until element text contains the expected substring.

        Args:
            locator: CSS or XPath-like locator string.
            text: Text to wait for.
            timeout: Maximum wait time in seconds.

        Examples:
            | Wait Until Element Contains | JLabel#status | complete | timeout=10 |
        """
        import time

        timeout_val = timeout if timeout is not None else self._timeout
        end_time = time.time() + timeout_val
        poll_interval = 0.5

        while time.time() < end_time:
            try:
                actual_text = self._lib.get_element_text(locator)
                if text in actual_text:
                    return
            except Exception:
                pass
            time.sleep(poll_interval)

        raise TimeoutError(f"Element '{locator}' did not contain '{text}' within {timeout_val}s")

    def get_component_tree(
        self,
        locator: Optional[str] = None,
        format: str = "text",
        max_depth: Optional[int] = None,
    ) -> str:
        """
        Get the component tree in various formats.

        Args:
            locator: Optional locator to start from.
            format: Output format - 'text', 'json', or 'yaml'.
            max_depth: Maximum depth to traverse.

        Returns:
            Component tree as string.

        Examples:
            | ${tree}= | Get Component Tree | format=json |
            | ${tree}= | Get Component Tree | format=text | max_depth=2 |
        """
        tree_str = self._lib.get_ui_tree(locator)
        return tree_str

    def log_component_tree(self, locator: Optional[str] = None) -> None:
        """Alias for log_ui_tree."""
        self._lib.log_ui_tree(locator)

    def list_applications(self) -> List[str]:
        """
        List available Java applications to connect to.

        Returns:
            List of available application identifiers.
        """
        return []

    def get_list_items(self, locator: str) -> List[str]:
        """
        Get all items from a JList component.

        Args:
            locator: CSS or XPath-like locator string for JList.

        Returns:
            List of item texts.

        Examples:
            | ${items}= | Get List Items | JList#itemList |
        """
        items = self._lib.get_element_property(locator, "items")
        return items if items else []

    def select_from_list(self, locator: str, value: str) -> None:
        """
        Select an item from a JList component.

        Args:
            locator: CSS or XPath-like locator string for JList.
            value: Item text to select.

        Examples:
            | Select From List | JList#itemList | Item 1 |
        """
        self._lib.click_element(f"{locator}[text='{value}']", click_count=1)

    def select_list_item_by_index(self, locator: str, index: int) -> None:
        """
        Select an item from a JList by index.

        Args:
            locator: CSS or XPath-like locator string for JList.
            index: Index of the item to select (0-based).

        Examples:
            | Select List Item By Index | JList#itemList | 0 |
        """
        self._lib.click_element(f"{locator}[index={index}]", click_count=1)

    def get_tree_nodes(self, locator: str) -> List[str]:
        """
        Get all node paths from a JTree component.

        Args:
            locator: CSS or XPath-like locator string for JTree.

        Returns:
            List of node paths.

        Examples:
            | ${nodes}= | Get Tree Nodes | JTree#fileTree |
        """
        nodes = self._lib.get_element_property(locator, "nodes")
        return nodes if nodes else []

    def get_table_data(self, locator: str) -> List[List[str]]:
        """
        Get all data from a table as a 2D list.

        Args:
            locator: CSS or XPath-like locator string for JTable.

        Returns:
            2D list of cell values (rows x columns).

        Examples:
            | ${data}= | Get Table Data | JTable#dataTable |
        """
        row_count = self._lib.get_table_row_count(locator)
        col_count = self._lib.get_table_column_count(locator)
        data = []
        for row in range(row_count):
            row_data = []
            for col in range(col_count):
                value = self._lib.get_table_cell_value(locator, row, str(col))
                row_data.append(value)
            data.append(row_data)
        return data

    def get_element_properties(self, locator: str) -> Dict[str, Any]:
        """
        Get all common properties from an element.

        Args:
            locator: CSS or XPath-like locator string.

        Returns:
            Dictionary of property names and values.

        Examples:
            | ${props}= | Get Element Properties | JButton#submit |
        """
        properties = {}
        for prop in ["name", "text", "enabled", "visible", "selected"]:
            try:
                properties[prop] = self._lib.get_element_property(locator, prop)
            except Exception:
                pass
        return properties


class Swt:
    """
    Robot Framework library for Eclipse SWT (Standard Widget Toolkit) automation.

    This library provides keywords for automating SWT-based applications.
    SWT is the widget toolkit used by Eclipse IDE and many Eclipse-based applications.

    = Initialization =

    | =Setting= | =Value= |
    | Library | JavaGui.Swt |
    | Library | JavaGui.Swt | timeout=30 |

    = Locator Syntax =

    Similar to Swing, but uses SWT widget types:

    | *Selector* | *Description* | *Example* |
    | Type | Match by class name | Button |
    | #id | Match by name/data | #submitBtn |
    | [attr=value] | Match by attribute | [text='Save'] |
    | Shell | SWT Shell (window) | Shell#main |
    | Composite | Container widget | Composite > Button |
    | Text | Text input widget | Text#username |
    | Tree | Tree widget | Tree#navigator |
    | Table | Table widget | Table#data |
    """

    ROBOT_LIBRARY_SCOPE = "GLOBAL"
    ROBOT_LIBRARY_VERSION = __version__
    ROBOT_LIBRARY_DOC_FORMAT = "REST"

    def __init__(
        self,
        timeout: float = 10.0,
        poll_interval: float = 0.5,
        screenshot_directory: str = ".",
    ) -> None:
        """
        Initialize the SWT Library.

        Args:
            timeout: Default timeout in seconds for wait operations.
            poll_interval: Polling interval in seconds for wait operations.
            screenshot_directory: Directory to save screenshots.
        """
        if not _RUST_AVAILABLE:
            raise ImportError(
                f"JavaGui Rust core not available: {_IMPORT_ERROR}\n"
                "Please ensure the library is properly installed with: pip install robotframework-javagui"
            )

        self._lib = _SwingLibrary(
            timeout=timeout,
            poll_interval=poll_interval,
            screenshot_directory=screenshot_directory,
        )
        self._timeout = timeout

    # ==========================================================================
    # Connection Keywords
    # ==========================================================================

    def connect_to_swt_application(
        self,
        application: str = "",
        host: str = "localhost",
        port: int = 5679,
        timeout: Optional[float] = None,
    ) -> None:
        """
        Connect to a running SWT application.

        Args:
            application: Application identifier.
            host: Host where the agent is running (default: localhost).
            port: Port the agent is listening on (default: 5679).
            timeout: Connection timeout in seconds.

        Examples:
            | Connect To SWT Application | DBeaver |
            | Connect To SWT Application | eclipse | port=5680 |
        """
        app_id = application if application else "default"
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.connect_to_application(app_id, host, port, timeout_val)

    def disconnect(self) -> None:
        """Disconnect from the current SWT application."""
        self._lib.disconnect_from_application()

    def is_connected(self) -> bool:
        """Check if connected to an SWT application."""
        return self._lib.is_connected()

    # ==========================================================================
    # Element Finding Keywords (SWT-specific)
    # ==========================================================================

    def find_element(self, locator: str) -> "_SwingElement":
        """Find a single SWT widget matching the locator."""
        return self._lib.find_element(locator)

    def find_elements(self, locator: str) -> List["_SwingElement"]:
        """Find all SWT widgets matching the locator."""
        return self._lib.find_elements(locator)

    def wait_until_element_exists(self, locator: str, timeout: Optional[float] = None) -> None:
        """Wait until an SWT widget exists."""
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.wait_until_element_exists(locator, timeout_val)

    # ==========================================================================
    # Click Keywords
    # ==========================================================================

    def click(self, locator: str) -> None:
        """Click on an SWT widget."""
        self._lib.click_element(locator, click_count=1)

    def click_element(self, locator: str, click_count: int = 1) -> None:
        """Click on an SWT widget with specified click count."""
        self._lib.click_element(locator, click_count=click_count)

    def double_click(self, locator: str) -> None:
        """Double-click on an SWT widget."""
        self._lib.click_element(locator, click_count=2)

    def click_button(self, locator: str) -> None:
        """Click an SWT Button widget."""
        self._lib.click_button(locator)

    # ==========================================================================
    # Input Keywords
    # ==========================================================================

    def input_text(self, locator: str, text: str, clear: bool = True) -> None:
        """Input text into an SWT Text widget."""
        self._lib.input_text(locator, text, clear=clear)

    def clear_text(self, locator: str) -> None:
        """Clear text from an SWT Text widget."""
        self._lib.clear_text(locator)

    # ==========================================================================
    # Selection Keywords
    # ==========================================================================

    def select_from_combo(self, locator: str, value: str) -> None:
        """Select an item from an SWT Combo widget."""
        self._lib.select_from_combobox(locator, value)

    def check_checkbox(self, locator: str) -> None:
        """Check an SWT Button widget (checkbox style)."""
        self._lib.check_checkbox(locator)

    def uncheck_checkbox(self, locator: str) -> None:
        """Uncheck an SWT Button widget (checkbox style)."""
        self._lib.uncheck_checkbox(locator)

    def select_radio_button(self, locator: str) -> None:
        """Select an SWT Button widget (radio style)."""
        self._lib.select_radio_button(locator)

    # ==========================================================================
    # Table Keywords
    # ==========================================================================

    def get_table_cell_value(self, locator: str, row: int, column: Union[int, str]) -> str:
        """Get the value of an SWT Table cell."""
        return self._lib.get_table_cell_value(locator, row, str(column))

    def select_table_row(self, locator: str, row: int) -> None:
        """Select a row in an SWT Table."""
        self._lib.select_table_row(locator, row)

    def get_table_row_count(self, locator: str) -> int:
        """Get the number of rows in an SWT Table."""
        return self._lib.get_table_row_count(locator)

    # ==========================================================================
    # Tree Keywords
    # ==========================================================================

    def expand_tree_item(self, locator: str, path: str) -> None:
        """Expand an SWT Tree item."""
        self._lib.expand_tree_node(locator, path)

    def collapse_tree_item(self, locator: str, path: str) -> None:
        """Collapse an SWT Tree item."""
        self._lib.collapse_tree_node(locator, path)

    def select_tree_item(self, locator: str, path: str) -> None:
        """Select an SWT Tree item."""
        self._lib.select_tree_node(locator, path)

    # ==========================================================================
    # Verification Keywords
    # ==========================================================================

    def element_should_be_visible(self, locator: str) -> None:
        """Verify that an SWT widget is visible."""
        self._lib.element_should_be_visible(locator)

    def element_should_be_enabled(self, locator: str) -> None:
        """Verify that an SWT widget is enabled."""
        self._lib.element_should_be_enabled(locator)

    def element_should_be_disabled(self, locator: str) -> None:
        """Verify that an SWT widget is disabled."""
        self._lib.element_should_be_disabled(locator)

    def get_element_text(self, locator: str) -> str:
        """Get the text content of an SWT widget."""
        return self._lib.get_element_text(locator)

    def element_text_should_be(self, locator: str, expected: str) -> None:
        """Verify that SWT widget text matches expected value."""
        self._lib.element_text_should_be(locator, expected)

    # ==========================================================================
    # UI Tree Keywords
    # ==========================================================================

    def get_ui_tree(self, format: str = "text", max_depth: Optional[int] = None, visible_only: bool = False) -> str:
        """Get the SWT widget tree as a string."""
        return self._lib.get_ui_tree(format, max_depth, visible_only)

    def log_ui_tree(self, locator: Optional[str] = None) -> None:
        """Log the SWT widget tree."""
        tree = self.get_ui_tree(format="text")
        print(tree)

    # ==========================================================================
    # Configuration Keywords
    # ==========================================================================

    def set_timeout(self, timeout: float) -> None:
        """Set the default timeout for wait operations."""
        self._timeout = timeout
        self._lib.set_timeout(timeout)

    # ==========================================================================
    # SWT-Specific Widget Keywords (Aliases)
    # ==========================================================================

    def find_widget(self, locator: str) -> "_SwingElement":
        """Find a single SWT widget matching the locator (alias for find_element)."""
        return self._lib.find_element(locator)

    def find_widgets(self, locator: str) -> List["_SwingElement"]:
        """Find all SWT widgets matching the locator (alias for find_elements)."""
        return self._lib.find_elements(locator)

    def click_widget(self, locator: str) -> None:
        """Click on an SWT widget (alias for click)."""
        self._lib.click_element(locator, click_count=1)

    def wait_until_widget_exists(self, locator: str, timeout: Optional[float] = None) -> None:
        """Wait until an SWT widget exists (alias for wait_until_element_exists)."""
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.wait_until_element_exists(locator, timeout_val)

    def widget_should_be_visible(self, locator: str) -> None:
        """Verify that an SWT widget is visible (alias for element_should_be_visible)."""
        self._lib.element_should_be_visible(locator)

    def widget_should_be_enabled(self, locator: str) -> None:
        """Verify that an SWT widget is enabled (alias for element_should_be_enabled)."""
        self._lib.element_should_be_enabled(locator)

    def widget_should_be_disabled(self, locator: str) -> None:
        """Verify that an SWT widget is disabled."""
        self._lib.element_should_be_disabled(locator)

    # ==========================================================================
    # Shell Keywords
    # ==========================================================================

    def get_shells(self) -> List[Dict[str, Any]]:
        """Get all open SWT Shell windows."""
        return self._lib.rpc_call("swt.getShells", {})

    def activate_shell(self, locator: str) -> None:
        """Activate (bring to front) an SWT Shell by locator."""
        self._lib.rpc_call("swt.activateShell", {"locator": locator})

    def close_shell(self, locator: str) -> None:
        """Close an SWT Shell by locator."""
        self._lib.rpc_call("swt.closeShell", {"locator": locator})


class Rcp:
    """
    Robot Framework library for Eclipse RCP (Rich Client Platform) automation.

    This library provides keywords for automating Eclipse RCP applications,
    including workbench operations, perspectives, views, editors, commands,
    preferences, and more.

    Supports both mock RCP applications (for testing) and real Eclipse RCP
    applications like DBeaver, Eclipse IDE, etc.

    = Initialization =

    | =Setting= | =Value= |
    | Library | JavaGui.Rcp |
    | Library | JavaGui.Rcp | timeout=30 |

    = Eclipse Workbench Concepts =

    | *Concept* | *Description* |
    | Workbench | The main application window |
    | Perspective | A layout configuration of views and editors |
    | View | A panel showing information (e.g., Package Explorer) |
    | Editor | An area for editing files |
    | Command | An executable action (e.g., Save, Copy) |
    """

    ROBOT_LIBRARY_SCOPE = "GLOBAL"
    ROBOT_LIBRARY_VERSION = __version__
    ROBOT_LIBRARY_DOC_FORMAT = "REST"

    def __init__(
        self,
        timeout: float = 10.0,
        poll_interval: float = 0.5,
        screenshot_directory: str = ".",
    ) -> None:
        """
        Initialize the RCP Library.

        Args:
            timeout: Default timeout in seconds for wait operations.
            poll_interval: Polling interval in seconds for wait operations.
            screenshot_directory: Directory to save screenshots.
        """
        if not _RUST_AVAILABLE:
            raise ImportError(
                f"JavaGui Rust core not available: {_IMPORT_ERROR}\n"
                "Please ensure the library is properly installed with: pip install robotframework-javagui"
            )

        self._lib = _SwingLibrary(
            timeout=timeout,
            poll_interval=poll_interval,
            screenshot_directory=screenshot_directory,
        )
        self._timeout = timeout

    # ==========================================================================
    # Connection Keywords
    # ==========================================================================

    def connect_to_swt_application(
        self,
        application: str = "",
        host: str = "localhost",
        port: int = 5680,
        timeout: Optional[float] = None,
    ) -> None:
        """
        Connect to a running Eclipse RCP application.

        Args:
            application: Application identifier.
            host: Host where the agent is running (default: localhost).
            port: Port the agent is listening on (default: 5680).
            timeout: Connection timeout in seconds.

        Examples:
            | Connect To SWT Application | DBeaver |
            | Connect To SWT Application | eclipse-rcp | port=5681 |
        """
        app_id = application if application else "default"
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.connect_to_application(app_id, host, port, timeout_val)

    def disconnect(self) -> None:
        """Disconnect from the current RCP application."""
        self._lib.disconnect_from_application()

    def is_connected(self) -> bool:
        """Check if connected to an RCP application."""
        return self._lib.is_connected()

    # ==========================================================================
    # Workbench Keywords
    # ==========================================================================

    def get_workbench_info(self) -> Dict[str, Any]:
        """
        Get information about the Eclipse workbench.

        Returns:
            Dictionary with workbench details.

        Examples:
            | ${info}= | Get Workbench Info |
        """
        return self._lib.rpc_call("rcp.getWorkbenchInfo", {})

    def wait_for_workbench(self, timeout: Optional[float] = None) -> None:
        """
        Wait for the workbench to be ready.

        Args:
            timeout: Maximum wait time in seconds.

        Examples:
            | Wait For Workbench | timeout=60 |
        """
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.rpc_call("rcp.waitForWorkbench", {"timeout": timeout_val})

    # ==========================================================================
    # Perspective Keywords
    # ==========================================================================

    def get_active_perspective(self) -> Dict[str, Any]:
        """
        Get the currently active perspective.

        Returns:
            Dictionary with perspective ID and label.

        Examples:
            | ${perspective}= | Get Active Perspective |
        """
        return self._lib.rpc_call("rcp.getActivePerspective", {})

    def open_perspective(self, perspective_id: str) -> None:
        """
        Open a perspective by ID.

        Args:
            perspective_id: Eclipse perspective ID.

        Examples:
            | Open Perspective | org.eclipse.jdt.ui.JavaPerspective |
        """
        self._lib.rpc_call("rcp.openPerspective", {"perspectiveId": perspective_id})

    def reset_perspective(self) -> None:
        """
        Reset the current perspective to its default layout.

        Examples:
            | Reset Perspective |
        """
        self._lib.rpc_call("rcp.resetPerspective", {})

    def get_open_perspectives(self) -> List[Dict[str, Any]]:
        """
        Get list of open perspectives.

        Returns:
            List of perspective information dictionaries.

        Examples:
            | ${perspectives}= | Get Open Perspectives |
        """
        return self._lib.rpc_call("rcp.getOpenPerspectives", {})

    def get_available_perspectives(self) -> List[Dict[str, Any]]:
        """
        Get list of all available perspectives.

        Returns:
            List of perspective information dictionaries.

        Examples:
            | ${perspectives}= | Get Available Perspectives |
        """
        return self._lib.rpc_call("rcp.getAvailablePerspectives", {})

    # ==========================================================================
    # View Keywords
    # ==========================================================================

    def show_view(self, view_id: str, secondary_id: Optional[str] = None) -> None:
        """
        Show a view by ID.

        Args:
            view_id: Eclipse view ID.
            secondary_id: Optional secondary ID for multiple instances.

        Examples:
            | Show View | org.eclipse.ui.views.ProblemView |
        """
        params = {"viewId": view_id}
        if secondary_id:
            params["secondaryId"] = secondary_id
        self._lib.rpc_call("rcp.showView", params)

    def close_view(self, view_id: str, secondary_id: Optional[str] = None) -> None:
        """
        Close a view by ID.

        Args:
            view_id: Eclipse view ID.
            secondary_id: Optional secondary ID for multiple instances.

        Examples:
            | Close View | org.eclipse.ui.views.ProblemView |
        """
        params = {"viewId": view_id}
        if secondary_id:
            params["secondaryId"] = secondary_id
        self._lib.rpc_call("rcp.closeView", params)

    def activate_view(self, view_id: str, secondary_id: Optional[str] = None) -> None:
        """
        Activate (bring to front) a view by ID.

        Args:
            view_id: Eclipse view ID.
            secondary_id: Optional secondary ID.

        Examples:
            | Activate View | org.eclipse.jdt.ui.PackageExplorer |
        """
        params = {"viewId": view_id}
        if secondary_id:
            params["secondaryId"] = secondary_id
        self._lib.rpc_call("rcp.activateView", params)

    def get_open_views(self) -> List[Dict[str, Any]]:
        """
        Get list of open views.

        Returns:
            List of view information dictionaries.

        Examples:
            | ${views}= | Get Open Views |
        """
        return self._lib.rpc_call("rcp.getOpenViews", {})

    def view_should_be_visible(self, view_id: str) -> None:
        """
        Verify that a view is visible.

        Args:
            view_id: Eclipse view ID.

        Raises:
            AssertionError: If view is not visible.

        Examples:
            | View Should Be Visible | org.eclipse.ui.views.ProblemView |
        """
        views = self.get_open_views()
        view_ids = [v.get("id", "") for v in views]
        if view_id not in view_ids:
            raise AssertionError(f"View '{view_id}' should be visible but was not found")

    # ==========================================================================
    # Editor Keywords
    # ==========================================================================

    def open_editor(self, file_path: str) -> None:
        """
        Open an editor for a file.

        Args:
            file_path: Path to the file to open.

        Examples:
            | Open Editor | /project/src/Main.java |
        """
        if not file_path:
            raise ValueError("File path cannot be empty")
        self._lib.rpc_call("rcp.openEditor", {"filePath": file_path})

    def close_editor(self, title: str, save: bool = False) -> None:
        """
        Close an editor by title.

        Args:
            title: Editor title (usually file name).
            save: Whether to save before closing.

        Examples:
            | Close Editor | Main.java |
            | Close Editor | Main.java | save=True |
        """
        if not title:
            raise ValueError("Editor title cannot be empty")
        self._lib.rpc_call("rcp.closeEditor", {"title": title, "save": save})

    def close_all_editors(self, save: bool = False) -> Dict[str, Any]:
        """
        Close all open editors.

        Args:
            save: Whether to save all before closing.

        Returns:
            Result of the operation.

        Examples:
            | Close All Editors |
            | Close All Editors | save=True |
        """
        return self._lib.rpc_call("rcp.closeAllEditors", {"save": save})

    def save_editor(self, title: Optional[str] = None) -> None:
        """
        Save an editor.

        Args:
            title: Editor title. If None, saves the active editor.

        Examples:
            | Save Editor |
            | Save Editor | title=Main.java |
        """
        params = {}
        if title:
            params["title"] = title
        self._lib.rpc_call("rcp.saveEditor", params)

    def save_all_editors(self) -> None:
        """
        Save all open editors.

        Examples:
            | Save All Editors |
        """
        self._lib.rpc_call("rcp.saveAllEditors", {})

    def get_active_editor(self) -> Optional[Dict[str, Any]]:
        """
        Get the currently active editor.

        Returns:
            Dictionary with editor info or None.

        Examples:
            | ${editor}= | Get Active Editor |
        """
        return self._lib.rpc_call("rcp.getActiveEditor", {})

    def activate_editor(self, title: str) -> None:
        """
        Activate an editor by title.

        Args:
            title: Editor title.

        Examples:
            | Activate Editor | Main.java |
        """
        if not title:
            raise ValueError("Editor title cannot be empty")
        self._lib.rpc_call("rcp.activateEditor", {"title": title})

    def get_open_editors(self) -> List[Dict[str, Any]]:
        """
        Get list of open editors.

        Returns:
            List of editor information dictionaries.

        Examples:
            | ${editors}= | Get Open Editors |
        """
        return self._lib.rpc_call("rcp.getOpenEditors", {})

    def editor_should_be_dirty(self, title: str) -> None:
        """
        Verify that an editor has unsaved changes.

        Args:
            title: Editor title.

        Raises:
            AssertionError: If editor is not dirty.

        Examples:
            | Editor Should Be Dirty | Main.java |
        """
        result = self._lib.rpc_call("rcp.isEditorDirty", {"title": title})
        if not result.get("dirty", False):
            raise AssertionError(f"Editor '{title}' should be dirty but is not dirty")

    def editor_should_not_be_dirty(self, title: str) -> None:
        """
        Verify that an editor has no unsaved changes.

        Args:
            title: Editor title.

        Raises:
            AssertionError: If editor is dirty.

        Examples:
            | Editor Should Not Be Dirty | Main.java |
        """
        result = self._lib.rpc_call("rcp.isEditorDirty", {"title": title})
        if result.get("dirty", False):
            raise AssertionError(f"Editor '{title}' should not be dirty but is dirty")

    def get_editor_widget(self, title: str, widget_type: str) -> "_SwingElement":
        """
        Get a widget from within an editor.

        Args:
            title: Editor title.
            widget_type: Type of widget to find.

        Returns:
            The found widget element.

        Examples:
            | ${text}= | Get Editor Widget | Main.java | StyledText |
        """
        if not title:
            raise ValueError("Editor title cannot be empty")
        return self._lib.rpc_call("rcp.getEditorWidget", {"title": title, "widgetType": widget_type})

    # ==========================================================================
    # Command Keywords
    # ==========================================================================

    def execute_command(self, command_id: str) -> None:
        """
        Execute an Eclipse command by ID.

        Args:
            command_id: Eclipse command ID.

        Examples:
            | Execute Command | org.eclipse.ui.file.save |
            | Execute Command | org.eclipse.ui.file.refresh |
        """
        if not command_id:
            raise ValueError("Command ID cannot be empty")
        self._lib.rpc_call("rcp.executeCommand", {"commandId": command_id})

    def get_available_commands(self, category: Optional[str] = None) -> List[Dict[str, Any]]:
        """
        Get list of available commands.

        Args:
            category: Optional category filter.

        Returns:
            List of command information dictionaries.

        Examples:
            | ${commands}= | Get Available Commands |
            | ${commands}= | Get Available Commands | category=File |
        """
        params = {}
        if category:
            params["category"] = category
        return self._lib.rpc_call("rcp.getAvailableCommands", params)

    # ==========================================================================
    # Menu Keywords
    # ==========================================================================

    def select_menu(self, menu_path: str) -> None:
        """
        Select a menu item by path.

        Args:
            menu_path: Menu path separated by '|'.

        Examples:
            | Select Menu | File|New|Project... |
            | Select Menu | Edit|Copy |
        """
        self._lib.rpc_call("rcp.selectMenu", {"menuPath": menu_path})

    def select_main_menu(self, menu_path: str) -> None:
        """
        Select a menu item from the main menu bar (alias for select_menu).

        Args:
            menu_path: Menu path separated by '|'.

        Examples:
            | Select Main Menu | File|New|Project... |
            | Select Main Menu | Edit|Copy |
        """
        self._lib.rpc_call("rcp.selectMenu", {"menuPath": menu_path})

    # ==========================================================================
    # Preference Keywords
    # ==========================================================================

    def open_preferences(self, page_path: Optional[str] = None) -> None:
        """
        Open the preferences dialog.

        Args:
            page_path: Optional path to a specific preference page.

        Examples:
            | Open Preferences |
            | Open Preferences | General|Appearance |
        """
        params = {}
        if page_path:
            params["pagePath"] = page_path
        self._lib.rpc_call("rcp.openPreferences", params)

    # ==========================================================================
    # Widget Keywords (via SWT base)
    # ==========================================================================

    def find_element(self, locator: str) -> "_SwingElement":
        """Find a widget in the RCP application."""
        return self._lib.find_element(locator)

    def find_elements(self, locator: str) -> List["_SwingElement"]:
        """Find all widgets matching the locator."""
        return self._lib.find_elements(locator)

    def click(self, locator: str) -> None:
        """Click on a widget."""
        self._lib.click_element(locator, click_count=1)

    def click_element(self, locator: str, click_count: int = 1) -> None:
        """Click on a widget with specified click count."""
        self._lib.click_element(locator, click_count=click_count)

    def input_text(self, locator: str, text: str, clear: bool = True) -> None:
        """Input text into a text widget."""
        self._lib.input_text(locator, text, clear=clear)

    def get_element_text(self, locator: str) -> str:
        """Get the text content of a widget."""
        return self._lib.get_element_text(locator)

    def element_should_be_visible(self, locator: str) -> None:
        """Verify that a widget is visible."""
        self._lib.element_should_be_visible(locator)

    def element_should_be_enabled(self, locator: str) -> None:
        """Verify that a widget is enabled."""
        self._lib.element_should_be_enabled(locator)

    def set_timeout(self, timeout: float) -> None:
        """Set the default timeout for wait operations."""
        self._timeout = timeout
        self._lib.set_timeout(timeout)


# Legacy aliases for backwards compatibility
SwingLibrary = Swing
SwtLibrary = Swt
RcpLibrary = Rcp


# Legacy SwingElement wrapper
class SwingElement:
    """
    Represents a Java GUI element.

    This class wraps a reference to a GUI component and provides
    methods for interaction and inspection.
    """

    def __init__(self, elem: "_SwingElement") -> None:
        """Initialize with a Rust SwingElement."""
        self._elem = elem

    @property
    def hash_code(self) -> int:
        """Get the element's hash code."""
        return self._elem.hash_code

    @property
    def class_name(self) -> str:
        """Get the element's Java class name."""
        return self._elem.class_name

    @property
    def simple_name(self) -> str:
        """Get the element's simple class name."""
        return self._elem.simple_name

    @property
    def name(self) -> Optional[str]:
        """Get the element's name property."""
        return self._elem.name

    @property
    def text(self) -> Optional[str]:
        """Get the element's text content."""
        return self._elem.text

    @property
    def visible(self) -> bool:
        """Check if the element is visible."""
        return self._elem.visible

    @property
    def enabled(self) -> bool:
        """Check if the element is enabled."""
        return self._elem.enabled

    @property
    def bounds(self) -> tuple:
        """Get the element's bounds (x, y, width, height)."""
        return self._elem.bounds

    def __repr__(self) -> str:
        name = f"[{self.name}]" if self.name else ""
        text = f'"{self.text[:20]}..."' if self.text and len(self.text) > 20 else f'"{self.text}"' if self.text else ""
        return f"<SwingElement {self.simple_name}{name} {text}>".strip()
