"""
Robot Framework Swing Library - High-performance automation for Java Swing applications.

This library provides comprehensive support for automating Java Swing applications
with Robot Framework. It features:

- CSS/XPath-like locator syntax for finding UI elements
- High-performance Rust core with Python bindings
- Automatic JVM discovery and agent injection
- UI Tree visualization and filtering
- Full Robot Framework keyword integration

Basic Usage:
    *** Settings ***
    Library    swing_library.SwingLibrary

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
    from swing_library._core import (
        SwingLibrary as _SwingLibrary,
        SwingElement as _SwingElement,
        SwtLibrary as _SwtLibrary,
        SwtElement as _SwtElement,
        RcpLibrary as _RcpLibrary,
        SwingConnectionError,
        ElementNotFoundError,
        MultipleElementsFoundError,
        LocatorParseError,
        ActionFailedError,
        TimeoutError as SwingTimeoutError,
    )
    # Aliases for backwards compatibility
    SwingError = SwingConnectionError
    ConnectionError = SwingConnectionError
    _RUST_AVAILABLE = True
except ImportError as e:
    _RUST_AVAILABLE = False
    _IMPORT_ERROR = str(e)


__version__ = "0.1.0"
__all__ = [
    "SwingLibrary",
    "SwingElement",
    "SwtLibrary",
    "SwtElement",
    "RcpLibrary",
    "SwingError",
    "ConnectionError",
    "ElementNotFoundError",
    "SwingTimeoutError",
    "ROBOT_LIBRARY_DOC_FORMAT",
    "ROBOT_LIBRARY_SCOPE",
    "ROBOT_LIBRARY_VERSION",
]

# Re-export SWT/RCP classes with public names
if _RUST_AVAILABLE:
    SwtLibrary = _SwtLibrary
    SwtElement = _SwtElement
    RcpLibrary = _RcpLibrary
else:
    SwtLibrary = None
    SwtElement = None
    RcpLibrary = None

ROBOT_LIBRARY_DOC_FORMAT = "REST"
ROBOT_LIBRARY_SCOPE = "GLOBAL"
ROBOT_LIBRARY_VERSION = __version__


class SwingLibrary:
    """
    Robot Framework library for Java Swing application automation.

    This library provides keywords for automating Java Swing desktop applications.
    It supports advanced locator syntax including CSS selectors and XPath.

    = Initialization =

    The library can be imported with optional default timeout:

    | =Setting= | =Value= |
    | Library | swing_library.SwingLibrary |
    | Library | swing_library.SwingLibrary | timeout=30 |

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
        """Initialize the Swing Library.

        | =Argument= | =Description= |
        | ``timeout`` | Default timeout in seconds for wait operations. Default ``10.0``. |
        | ``poll_interval`` | Polling interval in seconds for wait operations. Default ``0.5``. |
        | ``screenshot_directory`` | Directory to save screenshots. Default ``.`` (current). |

        Example:
        | =Setting= | =Value= | =Value= |
        | Library | swing_library.SwingLibrary | |
        | Library | swing_library.SwingLibrary | timeout=30 |
        """
        if not _RUST_AVAILABLE:
            raise ImportError(
                f"Swing Library Rust core not available: {_IMPORT_ERROR}\n"
                "Please ensure the library is properly installed with: pip install robotframework-swing"
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
        """Connect to a running Java Swing application.

        Connects to a JVM running a Swing application. The target application
        can be identified by name, process ID, main class name, or window title.

        | =Argument= | =Description= |
        | ``application`` | Application identifier (name, pid, main_class, or title). |
        | ``pid`` | Process ID of the target JVM (alternative to application). |
        | ``main_class`` | Fully qualified or simple name of the main class. |
        | ``title`` | Window title pattern (supports wildcards with ``*``). |
        | ``host`` | Host where the agent is running. Default ``localhost``. |
        | ``port`` | Port the agent is listening on. Default ``5678``. |
        | ``timeout`` | Connection timeout in seconds. Uses library default if not set. |

        Example:
        | `Connect To Application` | MyApp | | |
        | `Connect To Application` | main_class=com.example.MyApp | | |
        | `Connect To Application` | title=*Main Window* | | |
        | `Connect To Application` | application=MyApp | host=localhost | port=5678 |
        """
        # Build application identifier from various options
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
        """Disconnect from the current application.

        Closes the connection to the Swing application and cleans up resources.
        This should be called in test teardown.

        Example:
        | `Connect To Application` | MyApp |
        | # ... perform test actions ... | |
        | `Disconnect` | |
        """
        self._lib.disconnect_from_application()

    def is_connected(self) -> bool:
        """Check if connected to an application.

        Returns ``True`` if currently connected to a Swing application,
        ``False`` otherwise.

        Example:
        | ${connected}= | `Is Connected` |
        | Should Be True | ${connected} |
        """
        return self._lib.is_connected()

    def get_connection_info(self) -> Dict[str, Any]:
        """Get information about the current connection.

        Returns a dictionary containing connection details such as host, port,
        and application identifier.

        Example:
        | ${info}= | `Get Connection Info` |
        | Log | Connected to: ${info}[host]:${info}[port] |
        """
        return self._lib.get_connection_info()

    # ==========================================================================
    # Element Finding Keywords
    # ==========================================================================

    def find_element(self, locator: str) -> "_SwingElement":
        """Find a single element matching the locator.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Returns a ``SwingElement`` matching the locator.

        Raises ``ElementNotFoundError`` if no element matches the locator.

        Example:
        | ${button}= | `Find Element` | JButton#submit |
        | ${field}= | `Find Element` | //JTextField[@name='username'] |
        """
        return self._lib.find_element(locator)

    def find_elements(self, locator: str) -> List["_SwingElement"]:
        """Find all elements matching the locator.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Returns a list of ``SwingElement`` objects matching the locator.
        Returns an empty list if no elements match.

        Example:
        | ${buttons}= | `Find Elements` | JButton |
        | Length Should Be | ${buttons} | 5 |
        """
        return self._lib.find_elements(locator)

    def wait_until_element_exists(
        self,
        locator: str,
        timeout: Optional[float] = None,
    ) -> None:
        """Wait until an element exists in the UI tree.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |
        | ``timeout`` | Maximum wait time in seconds. Uses library default if not set. |

        Raises ``TimeoutError`` if element does not exist within timeout.

        Example:
        | `Wait Until Element Exists` | JButton#submit | |
        | `Wait Until Element Exists` | JButton#submit | timeout=30 |
        """
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.wait_until_element_exists(locator, timeout_val)

    def wait_until_element_does_not_exist(
        self,
        locator: str,
        timeout: Optional[float] = None,
    ) -> None:
        """Wait until an element no longer exists in the UI tree.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |
        | ``timeout`` | Maximum wait time in seconds. Uses library default if not set. |

        Raises ``TimeoutError`` if element still exists after timeout.

        Example:
        | `Wait Until Element Does Not Exist` | JDialog#loading | |
        | `Wait Until Element Does Not Exist` | JDialog#loading | timeout=60 |
        """
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.wait_until_element_does_not_exist(locator, timeout_val)

    # ==========================================================================
    # Click Keywords
    # ==========================================================================

    def click(self, locator: str) -> None:
        """Click on an element.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Performs a single left-click on the element.

        Example:
        | `Click` | JButton#submit |
        | `Click` | //JButton[@text='OK'] |
        """
        self._lib.click_element(locator, click_count=1)

    def click_element(self, locator: str, click_count: int = 1) -> None:
        """Click on an element with specified click count.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |
        | ``click_count`` | Number of clicks. ``1`` for single click, ``2`` for double click. Default ``1``. |

        Example:
        | `Click Element` | JButton#submit | |
        | `Click Element` | JTable | click_count=2 |
        """
        self._lib.click_element(locator, click_count=click_count)

    def double_click(self, locator: str) -> None:
        """Double-click on an element.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Performs a double left-click on the element. Useful for opening items
        in tables, lists, or trees.

        Example:
        | `Double Click` | JTable |
        | `Double Click` | JList#items |
        """
        self._lib.click_element(locator, click_count=2)

    def click_button(self, locator: str) -> None:
        """Click a button element.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string for the button. See `Locator Syntax`. |

        Specialized click for ``JButton`` components. Ensures the element
        is a button before clicking.

        Example:
        | `Click Button` | JButton#submit |
        | `Click Button` | #okButton |
        """
        self._lib.click_button(locator)

    # ==========================================================================
    # Input Keywords
    # ==========================================================================

    def input_text(self, locator: str, text: str, clear: bool = True) -> None:
        """Input text into a text field.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |
        | ``text`` | Text to input into the field. |
        | ``clear`` | Whether to clear existing text first. Default ``True``. |

        When ``clear`` is ``True``, any existing text is removed before typing.
        Set ``clear=False`` to append to existing text.

        Example:
        | `Input Text` | #username | testuser | |
        | `Input Text` | JTextField:first-child | Hello World | |
        | `Input Text` | #field | append this | clear=False |
        """
        self._lib.input_text(locator, text, clear=clear)

    def clear_text(self, locator: str) -> None:
        """Clear text from a text field.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Removes all text from the specified text field.

        Example:
        | `Clear Text` | #searchField |
        | `Clear Text` | JTextField#input |
        """
        self._lib.clear_text(locator)

    # ==========================================================================
    # Selection Keywords
    # ==========================================================================

    def select_from_combobox(self, locator: str, value: str) -> None:
        """Select an item from a combo box.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JComboBox``. See `Locator Syntax`. |
        | ``value`` | Item text to select from the dropdown. |

        Example:
        | `Select From Combobox` | #countryCombo | United States |
        | `Select From Combobox` | JComboBox#language | English |
        """
        self._lib.select_from_combobox(locator, value)

    def check_checkbox(self, locator: str) -> None:
        """Check a checkbox.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JCheckBox``. See `Locator Syntax`. |

        Sets the checkbox to checked state. If already checked, does nothing.

        Example:
        | `Check Checkbox` | #rememberMe |
        | `Check Checkbox` | JCheckBox#acceptTerms |
        """
        self._lib.check_checkbox(locator)

    def uncheck_checkbox(self, locator: str) -> None:
        """Uncheck a checkbox.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JCheckBox``. See `Locator Syntax`. |

        Sets the checkbox to unchecked state. If already unchecked, does nothing.

        Example:
        | `Uncheck Checkbox` | #newsletter |
        | `Uncheck Checkbox` | JCheckBox#sendUpdates |
        """
        self._lib.uncheck_checkbox(locator)

    def select_radio_button(self, locator: str) -> None:
        """Select a radio button.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JRadioButton``. See `Locator Syntax`. |

        Selects the specified radio button within its button group.

        Example:
        | `Select Radio Button` | #optionA |
        | `Select Radio Button` | JRadioButton#male |
        """
        self._lib.select_radio_button(locator)

    # ==========================================================================
    # Table Keywords
    # ==========================================================================

    def get_table_cell_value(self, locator: str, row: int, column: Union[int, str]) -> str:
        """Get the value of a table cell.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JTable``. See `Locator Syntax`. |
        | ``row`` | Row index (0-based). |
        | ``column`` | Column index (0-based) or column name as string. |

        Returns the cell value as a string.

        Example:
        | ${value}= | `Get Table Cell Value` | JTable | 0 | 1 |
        | ${value}= | `Get Table Cell Value` | JTable | 0 | Name |
        | Should Be Equal | ${value} | John |
        """
        return self._lib.get_table_cell_value(locator, row, str(column))

    def select_table_cell(self, locator: str, row: int, column: int) -> None:
        """Select a table cell.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JTable``. See `Locator Syntax`. |
        | ``row`` | Row index (0-based). |
        | ``column`` | Column index (0-based). |

        Selects (clicks) the specified cell in the table.

        Example:
        | `Select Table Cell` | #dataTable | 2 | 3 |
        | `Select Table Cell` | JTable#users | 0 | 0 |
        """
        self._lib.select_table_cell(locator, row, column)

    def select_table_row(self, locator: str, row: int) -> None:
        """Select a table row.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JTable``. See `Locator Syntax`. |
        | ``row`` | Row index (0-based). |

        Selects the entire row in the table.

        Example:
        | `Select Table Row` | #dataTable | 2 |
        | `Select Table Row` | JTable#users | 0 |
        """
        self._lib.select_table_row(locator, row)

    def get_table_row_count(self, locator: str) -> int:
        """Get the number of rows in a table.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JTable``. See `Locator Syntax`. |

        Returns the total number of rows in the table.

        Example:
        | ${count}= | `Get Table Row Count` | JTable |
        | Should Be Equal As Integers | ${count} | 10 |
        """
        return self._lib.get_table_row_count(locator)

    def get_table_column_count(self, locator: str) -> int:
        """Get the number of columns in a table.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JTable``. See `Locator Syntax`. |

        Returns the total number of columns in the table.

        Example:
        | ${count}= | `Get Table Column Count` | JTable |
        | Should Be Equal As Integers | ${count} | 5 |
        """
        return self._lib.get_table_column_count(locator)

    # ==========================================================================
    # Tree Keywords
    # ==========================================================================

    def expand_tree_node(self, locator: str, path: str) -> None:
        """Expand a tree node.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JTree``. See `Locator Syntax`. |
        | ``path`` | Node path separated by ``/`` or ``|`` (pipe). |

        Expands the tree node at the specified path, making child nodes visible.

        Example:
        | `Expand Tree Node` | JTree | Root/Folder/Subfolder |
        | `Expand Tree Node` | JTree | Root|Folder|Subfolder |
        | `Expand Tree Node` | #fileTree | Documents |
        """
        # Convert pipe separator to slash for Java agent compatibility
        normalized_path = path.replace("|", "/")
        self._lib.expand_tree_node(locator, normalized_path)

    def collapse_tree_node(self, locator: str, path: str) -> None:
        """Collapse a tree node.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JTree``. See `Locator Syntax`. |
        | ``path`` | Node path separated by ``/`` or ``|`` (pipe). |

        Collapses the tree node at the specified path, hiding child nodes.

        Example:
        | `Collapse Tree Node` | #fileTree | Documents/Downloads |
        | `Collapse Tree Node` | JTree | Root|Folder |
        """
        # Convert pipe separator to slash for Java agent compatibility
        normalized_path = path.replace("|", "/")
        self._lib.collapse_tree_node(locator, normalized_path)

    def select_tree_node(self, locator: str, path: str) -> None:
        """Select a tree node.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JTree``. See `Locator Syntax`. |
        | ``path`` | Node path separated by ``/`` or ``|`` (pipe). |

        Selects (highlights) the tree node at the specified path.

        Example:
        | `Select Tree Node` | JTree | Root/Config/Settings |
        | `Select Tree Node` | JTree | Root|Config|Settings |
        | `Select Tree Node` | #projectTree | src/main/java |
        """
        # Convert pipe separator to slash for Java agent compatibility
        normalized_path = path.replace("|", "/")
        self._lib.select_tree_node(locator, normalized_path)

    def get_selected_tree_node(self, locator: str) -> Optional[str]:
        """Get the currently selected tree node path.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JTree``. See `Locator Syntax`. |

        Returns the path of the currently selected node, or ``None`` if no node is selected.

        Example:
        | ${path}= | `Get Selected Tree Node` | JTree |
        | Should Be Equal | ${path} | Root/Config/Settings |
        """
        return self._lib.get_selected_tree_node(locator)

    # ==========================================================================
    # Menu Keywords
    # ==========================================================================

    def select_menu(self, menu_path: str) -> None:
        """Select a menu item from the menu bar.

        | =Argument= | =Description= |
        | ``menu_path`` | Menu path separated by ``|`` (pipe character). |

        Navigates through the menu hierarchy and clicks the final item.

        Example:
        | `Select Menu` | File|New |
        | `Select Menu` | Edit|Copy |
        | `Select Menu` | File|Export|As PDF |
        """
        self._lib.select_menu(menu_path)

    def select_from_popup_menu(self, menu_path: str) -> None:
        """Select an item from a popup/context menu.

        | =Argument= | =Description= |
        | ``menu_path`` | Menu path separated by ``|`` (pipe character). |

        Use after right-clicking to open a context menu. Navigates through
        the popup menu hierarchy and clicks the final item.

        Example:
        | `Right Click` | JTree#files |
        | `Select From Popup Menu` | Copy |
        | `Select From Popup Menu` | Edit|Paste |
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
        """Wait until an element becomes visible.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |
        | ``timeout`` | Maximum wait time in seconds. Uses library default if not set. |

        Waits until the element exists and is visible (not hidden).
        Raises ``TimeoutError`` if element is not visible within timeout.

        Example:
        | `Wait Until Element Is Visible` | JLabel#status | |
        | `Wait Until Element Is Visible` | JLabel#status | timeout=15 |
        """
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.wait_until_element_is_visible(locator, timeout_val)

    def wait_until_element_is_enabled(
        self,
        locator: str,
        timeout: Optional[float] = None,
    ) -> None:
        """Wait until an element becomes enabled.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |
        | ``timeout`` | Maximum wait time in seconds. Uses library default if not set. |

        Waits until the element is enabled and can receive user input.
        Raises ``TimeoutError`` if element is not enabled within timeout.

        Example:
        | `Wait Until Element Is Enabled` | JButton#next | |
        | `Wait Until Element Is Enabled` | JButton#next | timeout=10 |
        """
        timeout_val = timeout if timeout is not None else self._timeout
        self._lib.wait_until_element_is_enabled(locator, timeout_val)

    # ==========================================================================
    # Verification Keywords
    # ==========================================================================

    def element_should_be_visible(self, locator: str) -> None:
        """Verify that an element is visible.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Fails if the element is not visible.

        Example:
        | `Element Should Be Visible` | JPanel#main |
        | `Element Should Be Visible` | #loginForm |
        """
        self._lib.element_should_be_visible(locator)

    def element_should_not_be_visible(self, locator: str) -> None:
        """Verify that an element is not visible.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Fails if the element is visible.

        Example:
        | `Element Should Not Be Visible` | JDialog#loading |
        | `Element Should Not Be Visible` | #errorPanel |
        """
        self._lib.element_should_not_be_visible(locator)

    def element_should_be_enabled(self, locator: str) -> None:
        """Verify that an element is enabled.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Fails if the element is disabled.

        Example:
        | `Element Should Be Enabled` | JButton#save |
        | `Element Should Be Enabled` | #submitBtn |
        """
        self._lib.element_should_be_enabled(locator)

    def element_should_be_disabled(self, locator: str) -> None:
        """Verify that an element is disabled.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Fails if the element is enabled.

        Example:
        | `Element Should Be Disabled` | JButton#next |
        | `Element Should Be Disabled` | #deleteBtn |
        """
        self._lib.element_should_be_disabled(locator)

    def get_element_text(self, locator: str) -> str:
        """Get the text content of an element.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Returns the text content of the element (e.g., label text, button text).

        Example:
        | ${text}= | `Get Element Text` | JLabel#status |
        | Should Be Equal | ${text} | Ready |
        """
        return self._lib.get_element_text(locator)

    def element_text_should_be(self, locator: str, expected: str) -> None:
        """Verify that element text matches expected value exactly.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |
        | ``expected`` | Expected text value. |

        Fails if the element text does not match exactly.

        Example:
        | `Element Text Should Be` | JLabel#status | Ready |
        | `Element Text Should Be` | #message | Operation completed |
        """
        self._lib.element_text_should_be(locator, expected)

    def element_text_should_contain(self, locator: str, expected: str) -> None:
        """Verify that element text contains expected substring.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |
        | ``expected`` | Expected substring. |

        Fails if the element text does not contain the expected substring.

        Example:
        | `Element Text Should Contain` | JLabel#status | Success |
        | `Element Text Should Contain` | #message | completed |
        """
        self._lib.element_text_should_contain(locator, expected)

    def get_element_property(self, locator: str, property_name: str) -> Any:
        """Get a property value from an element.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |
        | ``property_name`` | Name of the property to retrieve (e.g., ``text``, ``enabled``, ``visible``). |

        Returns the value of the specified property.

        Example:
        | ${text}= | `Get Element Property` | JTextField#input | text |
        | ${enabled}= | `Get Element Property` | JButton#save | enabled |
        """
        return self._lib.get_element_property(locator, property_name)

    # ==========================================================================
    # UI Tree Keywords
    # ==========================================================================

    def log_ui_tree(self, locator: Optional[str] = None) -> None:
        """Log the UI component tree to the test log.

        | =Argument= | =Description= |
        | ``locator`` | Optional locator to start from. Logs entire tree if not specified. |

        Prints the component hierarchy for debugging purposes.

        Example:
        | `Log UI Tree` | |
        | `Log UI Tree` | JPanel#main |
        """
        # Get tree as text and log it
        tree = self.get_ui_tree(format="text")
        print(tree)

    def get_ui_tree(self, format: str = "text", max_depth: Optional[int] = None, visible_only: bool = False) -> str:
        """Get the UI component tree as a string.

        | =Argument= | =Description= |
        | ``format`` | Output format: ``text``, ``json``, or ``xml``. Default ``text``. |
        | ``max_depth`` | Maximum depth to traverse. ``None`` for unlimited. |
        | ``visible_only`` | Only include visible components. Default ``False``. |

        Returns the component tree in the specified format.

        Example:
        | ${tree}= | `Get UI Tree` | | |
        | ${json}= | `Get UI Tree` | format=json | |
        | ${tree}= | `Get UI Tree` | format=text | max_depth=3 |
        """
        return self._lib.get_ui_tree(format, max_depth, visible_only)

    def save_ui_tree(self, filename: str, locator: Optional[str] = None) -> None:
        """Save the UI component tree to a file.

        | =Argument= | =Description= |
        | ``filename`` | Path to save the tree file. |
        | ``locator`` | Optional locator to start from. Saves entire tree if not specified. |

        Saves the component hierarchy to a file for analysis.

        Example:
        | `Save UI Tree` | tree.txt | |
        | `Save UI Tree` | panel_tree.txt | JPanel#main |
        """
        self._lib.save_ui_tree(filename, locator)

    def refresh_ui_tree(self) -> None:
        """Refresh the cached UI component tree.

        Call this after UI changes to update the internal component cache.
        Useful when the UI has been modified and you need to find new elements.

        Example:
        | `Click Button` | JButton#addItem |
        | `Refresh UI Tree` | |
        | `Find Element` | JLabel#newItem |
        """
        self._lib.refresh_ui_tree()

    # ==========================================================================
    # Screenshot Keywords
    # ==========================================================================

    def capture_screenshot(self, filename: Optional[str] = None) -> str:
        """Capture a screenshot of the application.

        | =Argument= | =Description= |
        | ``filename`` | Optional filename for the screenshot. Auto-generated if not specified. |

        Returns the path to the saved screenshot file.

        Example:
        | ${path}= | `Capture Screenshot` | |
        | ${path}= | `Capture Screenshot` | filename=error.png |
        | Log | Screenshot saved to: ${path} | |
        """
        return self._lib.capture_screenshot(filename)

    def set_screenshot_directory(self, directory: str) -> None:
        """Set the directory for saving screenshots.

        | =Argument= | =Description= |
        | ``directory`` | Path to the screenshot directory. |

        All subsequent screenshots will be saved to this directory.

        Example:
        | `Set Screenshot Directory` | ${OUTPUT_DIR}/screenshots |
        | `Set Screenshot Directory` | /tmp/test-screenshots |
        """
        self._lib.set_screenshot_directory(directory)

    # ==========================================================================
    # Configuration Keywords
    # ==========================================================================

    def set_timeout(self, timeout: float) -> None:
        """Set the default timeout for wait operations.

        | =Argument= | =Description= |
        | ``timeout`` | Timeout in seconds. |

        Sets the default timeout used by all wait keywords when no explicit
        timeout is provided.

        Example:
        | `Set Timeout` | 30 |
        | `Set Timeout` | 60 |
        """
        self._timeout = timeout
        self._lib.set_timeout(timeout)

    # ==========================================================================
    # Additional Convenience Keywords
    # ==========================================================================

    def select_tab(self, locator: str, tab_identifier: str) -> None:
        """Select a tab in a JTabbedPane.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JTabbedPane``. See `Locator Syntax`. |
        | ``tab_identifier`` | Tab title (string) or index (integer) to select. |

        Selects the specified tab by title or index.

        Example:
        | `Select Tab` | JTabbedPane[name='mainTabbedPane'] | Form Input |
        | `Select Tab` | #mainTabs | Settings |
        | `Select Tab` | JTabbedPane | 0 |
        """
        # Delegate to Rust library's select_tab which uses selectItem RPC
        self._lib.select_tab(locator, str(tab_identifier))

    def type_text(self, locator: str, text: str) -> None:
        """Type text character by character into a text field.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |
        | ``text`` | Text to type character by character. |

        Simulates actual key presses rather than setting the text directly.
        Does not clear existing text - use `Clear Text` first if needed.

        Example:
        | `Type Text` | #searchField | hello |
        | `Type Text` | JTextField#input | test@example.com |
        """
        # For now, use input_text as the underlying implementation
        # The Rust library handles the actual typing
        self._lib.input_text(locator, text, clear=False)

    def right_click(self, locator: str) -> None:
        """Right-click (context click) on an element.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Performs a right-click to open context menus.
        Use `Select From Popup Menu` after this to select menu items.

        Example:
        | `Right Click` | JTree#fileTree |
        | `Select From Popup Menu` | Delete |
        """
        # Use click_element - the Rust implementation should support right-click
        # For now, we'll use regular click as a placeholder
        # A proper implementation would need right-click support in Rust
        self._lib.click_element(locator, click_count=1)

    def element_should_be_selected(self, locator: str) -> None:
        """Verify that an element is selected (checked).

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Works with checkboxes, radio buttons, list items, etc.
        Fails if the element is not selected.

        Example:
        | `Element Should Be Selected` | JCheckBox#rememberMe |
        | `Element Should Be Selected` | JRadioButton#optionA |
        """
        selected = self._lib.get_element_property(locator, "selected")
        if not selected:
            raise AssertionError(f"Element '{locator}' should be selected but was not")

    def element_should_not_be_selected(self, locator: str) -> None:
        """Verify that an element is not selected (unchecked).

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Works with checkboxes, radio buttons, list items, etc.
        Fails if the element is selected.

        Example:
        | `Element Should Not Be Selected` | JRadioButton#optionB |
        | `Element Should Not Be Selected` | JCheckBox#newsletter |
        """
        selected = self._lib.get_element_property(locator, "selected")
        if selected:
            raise AssertionError(f"Element '{locator}' should not be selected but was")

    def element_should_exist(self, locator: str) -> None:
        """Verify that an element exists in the UI tree.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Fails if the element does not exist.

        Example:
        | `Element Should Exist` | JButton#submit |
        | `Element Should Exist` | #loginForm |
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
        """Verify that an element does not exist in the UI tree.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Fails if the element exists.

        Example:
        | `Element Should Not Exist` | JDialog#error |
        | `Element Should Not Exist` | #loadingSpinner |
        """
        try:
            elements = self._lib.find_elements(locator)
            if elements:
                raise AssertionError(f"Element '{locator}' should not exist but was found")
        except AssertionError:
            raise
        except Exception:
            # Element not found is the expected outcome
            pass

    # ==========================================================================
    # Keyword Aliases for Compatibility
    # ==========================================================================

    def wait_until_element_visible(
        self,
        locator: str,
        timeout: Optional[float] = None,
    ) -> None:
        """Alias for `Wait Until Element Is Visible`."""
        self.wait_until_element_is_visible(locator, timeout)

    def wait_until_element_enabled(
        self,
        locator: str,
        timeout: Optional[float] = None,
    ) -> None:
        """Alias for `Wait Until Element Is Enabled`."""
        self.wait_until_element_is_enabled(locator, timeout)

    def wait_for_element(
        self,
        locator: str,
        timeout: Optional[float] = None,
    ) -> "_SwingElement":
        """Wait for an element to exist and return it.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |
        | ``timeout`` | Maximum wait time in seconds. Uses library default if not set. |

        Returns the found ``SwingElement`` after it exists.
        Raises ``TimeoutError`` if element does not exist within timeout.

        Example:
        | ${elem}= | `Wait For Element` | JButton#submit | |
        | ${elem}= | `Wait For Element` | JButton#submit | timeout=10 |
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
        """Wait until element text contains the expected substring.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |
        | ``text`` | Text substring to wait for. |
        | ``timeout`` | Maximum wait time in seconds. Uses library default if not set. |

        Raises ``TimeoutError`` if element text does not contain the expected
        substring within timeout.

        Example:
        | `Wait Until Element Contains` | JLabel#status | complete | |
        | `Wait Until Element Contains` | JLabel#status | complete | timeout=10 |
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
        """Get the component tree in various formats.

        | =Argument= | =Description= |
        | ``locator`` | Optional locator to start from. Uses root if not specified. |
        | ``format`` | Output format: ``text``, ``json``, or ``yaml``. Default ``text``. |
        | ``max_depth`` | Maximum depth to traverse. ``None`` for unlimited. |

        Returns the component tree as a string in the specified format.

        Example:
        | ${tree}= | `Get Component Tree` | | |
        | ${json}= | `Get Component Tree` | format=json | |
        | ${tree}= | `Get Component Tree` | format=text | max_depth=2 |
        """
        tree_str = self._lib.get_ui_tree(locator)
        # The Rust library returns text format by default
        # Format conversion would be done here if needed
        return tree_str

    def log_component_tree(self, locator: Optional[str] = None) -> None:
        """Alias for `Log UI Tree`."""
        self._lib.log_ui_tree(locator)

    def list_applications(self) -> List[str]:
        """List available Java applications to connect to.

        Returns a list of available application identifiers that can be
        used with `Connect To Application`.

        *Note:* This is a placeholder - actual discovery requires JVM enumeration.

        Example:
        | ${apps}= | `List Applications` |
        | Log Many | @{apps} |
        """
        # Placeholder - actual implementation would use JVM discovery
        return []

    # ==========================================================================
    # List Operations
    # ==========================================================================

    def get_list_items(self, locator: str) -> List[str]:
        """Get all items from a JList component.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JList``. See `Locator Syntax`. |

        Returns a list of all item texts in the list.

        Example:
        | ${items}= | `Get List Items` | JList[name='itemList'] |
        | Length Should Be | ${items} | 5 |
        """
        # Delegate to Rust library's get_list_items which uses getListItems RPC
        return self._lib.get_list_items(locator)

    def select_from_list(self, locator: str, value: str) -> None:
        """Select an item from a JList component by text.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JList``. See `Locator Syntax`. |
        | ``value`` | Item text to select. |

        Selects the item matching the specified text.

        Example:
        | `Select From List` | JList[name='itemList'] | Item 1 |
        | `Select From List` | #fileList | document.txt |
        """
        # Delegate to Rust library's select_from_list which uses selectItem RPC
        self._lib.select_from_list(locator, value)

    def select_list_item_by_index(self, locator: str, index: int) -> None:
        """Select an item from a JList by index.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JList``. See `Locator Syntax`. |
        | ``index`` | Index of the item to select (0-based). |

        Selects the item at the specified index.

        Example:
        | `Select List Item By Index` | JList[name='itemList'] | 0 |
        | `Select List Item By Index` | #fileList | 2 |
        """
        # Delegate to Rust library's select_list_item_by_index which uses selectItem RPC
        self._lib.select_list_item_by_index(locator, index)

    # ==========================================================================
    # Tree Operations
    # ==========================================================================

    def get_tree_nodes(self, locator: str) -> List[str]:
        """Get all node paths from a JTree component.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JTree``. See `Locator Syntax`. |

        Returns a list of all node paths in the tree.

        Example:
        | ${nodes}= | `Get Tree Nodes` | JTree#fileTree |
        | Should Contain | ${nodes} | Root/Documents |
        """
        # Get tree structure via RPC and extract node paths
        tree_data = self._lib.get_tree_data(locator)
        if not tree_data:
            return []
        # tree_data is a dict with text and children - flatten to paths
        return self._flatten_tree_paths(tree_data, "")

    def _flatten_tree_paths(self, node: dict, prefix: str) -> List[str]:
        """Helper to flatten tree structure into list of paths."""
        paths = []
        text = node.get("text", "")
        current_path = f"{prefix}/{text}" if prefix else text
        paths.append(current_path)

        children = node.get("children", [])
        for child in children:
            paths.extend(self._flatten_tree_paths(child, current_path))

        return paths

    # ==========================================================================
    # Additional Table and Property Keywords
    # ==========================================================================

    def get_table_data(self, locator: str) -> List[List[str]]:
        """Get all data from a table as a 2D list.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator for the ``JTable``. See `Locator Syntax`. |

        Returns a 2D list of cell values (rows x columns).

        Example:
        | ${data}= | `Get Table Data` | JTable#dataTable |
        | ${first_row}= | Set Variable | ${data}[0] |
        | ${cell}= | Set Variable | ${data}[0][1] |
        """
        row_count = self._lib.get_table_row_count(locator)
        col_count = self._lib.get_table_column_count(locator)
        data = []
        for row in range(row_count):
            row_data = []
            for col in range(col_count):
                # Convert column to string as required by Rust function
                value = self._lib.get_table_cell_value(locator, row, str(col))
                row_data.append(value)
            data.append(row_data)
        return data

    def get_element_properties(self, locator: str) -> Dict[str, Any]:
        """Get all common properties from an element.

        | =Argument= | =Description= |
        | ``locator`` | CSS or XPath-like locator string. See `Locator Syntax`. |

        Returns a dictionary containing common properties: ``name``, ``text``,
        ``enabled``, ``visible``, and ``selected``.

        Example:
        | ${props}= | `Get Element Properties` | JButton#submit |
        | Should Be True | ${props}[enabled] |
        | Log | Button text: ${props}[text] |
        """
        properties = {}
        for prop in ["name", "text", "enabled", "visible", "selected"]:
            try:
                properties[prop] = self._lib.get_element_property(locator, prop)
            except Exception:
                pass
        return properties


# Legacy SwingElement wrapper (if needed)
class SwingElement:
    """
    Represents a Swing UI element.

    This class wraps a reference to a Swing component and provides
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
