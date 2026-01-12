"""
Pytest configuration and fixtures for SwingLibrary tests.
"""

import pytest
from unittest.mock import Mock, MagicMock, patch
from typing import Dict, Any, List, Optional


class MockSwingElement:
    """Mock Swing element for testing."""

    def __init__(
        self,
        id: int = 1,
        class_name: str = "javax.swing.JButton",
        name: Optional[str] = "testBtn",
        text: Optional[str] = "Click Me",
        visible: bool = True,
        enabled: bool = True,
        bounds: Dict[str, int] = None,
        properties: Dict[str, Any] = None,
    ):
        self.id = id
        self.class_name = class_name
        self.simple_class_name = class_name.split(".")[-1]
        self.name = name
        self.text = text
        self.is_visible = visible
        self.is_enabled = enabled
        self.bounds = bounds or {"x": 100, "y": 100, "width": 80, "height": 30}
        self._properties = properties or {}

    def get_property(self, name: str) -> Any:
        return self._properties.get(name)

    def get_all_properties(self) -> Dict[str, Any]:
        return self._properties.copy()

    def click(self) -> None:
        pass

    def double_click(self) -> None:
        pass

    def right_click(self) -> None:
        pass

    def input_text(self, text: str) -> None:
        pass

    def clear_text(self) -> None:
        pass


class MockSwingLibrary:
    """Mock Rust SwingLibrary core for testing."""

    def __init__(
        self,
        timeout: float = 10.0,
        poll_interval: float = 0.5,
        screenshot_directory: str = ".",
        # Legacy parameters for backwards compatibility
        timeout_ms: int = None,
        screenshot_on_failure: bool = True,
    ):
        self.timeout = timeout
        self.poll_interval = poll_interval
        self.screenshot_directory = screenshot_directory
        self.screenshot_on_failure = screenshot_on_failure
        self._connected = False
        self._elements: Dict[str, MockSwingElement] = {}
        self._setup_default_elements()

    def _setup_default_elements(self) -> None:
        """Set up default mock elements for testing."""
        self._elements = {
            "JButton#loginBtn": MockSwingElement(
                id=1, name="loginBtn", text="Login",
                class_name="javax.swing.JButton"
            ),
            "JTextField#username": MockSwingElement(
                id=2, name="username", text="",
                class_name="javax.swing.JTextField"
            ),
            "JPasswordField#password": MockSwingElement(
                id=3, name="password", text="",
                class_name="javax.swing.JPasswordField"
            ),
            "JTable#dataTable": MockSwingElement(
                id=4, name="dataTable", text=None,
                class_name="javax.swing.JTable"
            ),
            "JTree#fileTree": MockSwingElement(
                id=5, name="fileTree", text=None,
                class_name="javax.swing.JTree"
            ),
            "JLabel#statusLabel": MockSwingElement(
                id=6, name="statusLabel", text="Ready",
                class_name="javax.swing.JLabel"
            ),
        }

    def connect(
        self,
        pid: Optional[int] = None,
        main_class: Optional[str] = None,
        title: Optional[str] = None,
        timeout_ms: int = 10000,
    ) -> None:
        if not any([pid, main_class, title]):
            raise ValueError("Must specify pid, main_class, or title")
        self._connected = True

    def connect_to_application(
        self,
        application: str,
        host: str = "localhost",
        port: int = 5678,
        timeout: float = 30.0,
    ) -> None:
        """Connect to application (new API)."""
        self._connected = True

    def disconnect(self) -> None:
        self._connected = False

    def disconnect_from_application(self) -> None:
        """Disconnect from application (new API)."""
        self._connected = False

    def is_connected(self) -> bool:
        """Check if connected."""
        return self._connected

    def get_connection_info(self) -> Dict[str, Any]:
        """Get connection info."""
        return {"connected": self._connected, "host": "localhost", "port": 5678}

    def list_applications(self) -> List[Dict[str, Any]]:
        return [
            {"pid": 12345, "main_class": "com.example.App", "args": ""},
            {"pid": 67890, "main_class": "com.demo.Demo", "args": "--debug"},
        ]

    def find_element(
        self, locator: str, parent: Optional[MockSwingElement] = None
    ) -> MockSwingElement:
        if locator in self._elements:
            return self._elements[locator]
        # Simple matching for testing
        for key, elem in self._elements.items():
            if locator in key or (elem.name and locator.endswith(f"#{elem.name}")):
                return elem
        raise ElementNotFoundError(f"Element not found: {locator}")

    def find_elements(
        self, locator: str, parent: Optional[MockSwingElement] = None
    ) -> List[MockSwingElement]:
        results = []
        base_type = locator.split("#")[0].split("[")[0].split(":")[0]
        for key, elem in self._elements.items():
            if base_type in elem.class_name:
                results.append(elem)
        return results

    def wait_for_element(self, locator: str, timeout_ms: int = 10000) -> MockSwingElement:
        return self.find_element(locator)

    def click(self, locator: str) -> None:
        self.find_element(locator).click()

    def click_element(self, locator: str, click_count: int = 1) -> None:
        """Click element with count (new API)."""
        self.find_element(locator).click()

    def click_button(self, locator: str) -> None:
        """Click button (new API)."""
        self.find_element(locator).click()

    def double_click(self, locator: str) -> None:
        self.find_element(locator).double_click()

    def right_click(self, locator: str) -> None:
        self.find_element(locator).right_click()

    def input_text(self, locator: str, text: str, clear: bool = True) -> None:
        elem = self.find_element(locator)
        elem.input_text(text)

    def clear_text(self, locator: str) -> None:
        elem = self.find_element(locator)
        elem.clear_text()

    def type_text(self, locator: str, text: str) -> None:
        elem = self.find_element(locator)
        elem.input_text(text)

    def select_item(
        self, locator: str, value: Optional[str] = None, index: Optional[int] = None
    ) -> None:
        self.find_element(locator)

    def get_table_cell_value(self, locator: str, row: int, column: int) -> str:
        self.find_element(locator)
        return f"Cell[{row},{column}]"

    def select_table_cell(self, locator: str, row: int, column: int) -> None:
        self.find_element(locator)

    def get_table_row_count(self, locator: str) -> int:
        self.find_element(locator)
        return 10

    def expand_tree_node(self, locator: str, path: str) -> None:
        self.find_element(locator)

    def collapse_tree_node(self, locator: str, path: str) -> None:
        self.find_element(locator)

    def select_tree_node(self, locator: str, path: str) -> None:
        self.find_element(locator)

    def wait_until_visible(self, locator: str, timeout_ms: int = 10000) -> None:
        elem = self.find_element(locator)
        if not elem.is_visible:
            raise TimeoutError(f"Element not visible: {locator}")

    def wait_until_not_visible(self, locator: str, timeout_ms: int = 10000) -> None:
        pass

    def wait_until_enabled(self, locator: str, timeout_ms: int = 10000) -> None:
        elem = self.find_element(locator)
        if not elem.is_enabled:
            raise TimeoutError(f"Element not enabled: {locator}")

    def wait_until_element_exists(self, locator: str, timeout: float = 10.0) -> None:
        """New API wait until exists."""
        self.find_element(locator)

    def wait_until_element_does_not_exist(self, locator: str, timeout: float = 10.0) -> None:
        """New API wait until does not exist."""
        try:
            self.find_element(locator)
            raise TimeoutError(f"Element still exists: {locator}")
        except ElementNotFoundError:
            pass

    def wait_until_element_is_visible(self, locator: str, timeout: float = 10.0) -> None:
        """New API wait until visible."""
        elem = self.find_element(locator)
        if not elem.is_visible:
            raise TimeoutError(f"Element not visible: {locator}")

    def wait_until_element_is_enabled(self, locator: str, timeout: float = 10.0) -> None:
        """New API wait until enabled."""
        elem = self.find_element(locator)
        if not elem.is_enabled:
            raise TimeoutError(f"Element not enabled: {locator}")

    def element_should_exist(self, locator: str) -> None:
        self.find_element(locator)

    def element_should_not_exist(self, locator: str) -> None:
        try:
            self.find_element(locator)
            raise AssertionError(f"Element should not exist: {locator}")
        except ElementNotFoundError:
            pass

    def element_should_be_visible(self, locator: str) -> None:
        elem = self.find_element(locator)
        if not elem.is_visible:
            raise AssertionError(f"Element not visible: {locator}")

    def element_should_be_enabled(self, locator: str) -> None:
        elem = self.find_element(locator)
        if not elem.is_enabled:
            raise AssertionError(f"Element not enabled: {locator}")

    def get_element_text(self, locator: str) -> str:
        return self.find_element(locator).text or ""

    def get_component_tree(
        self, format: str = "json", max_depth: Optional[int] = None
    ) -> str:
        if format == "json":
            return '{"type": "JFrame", "name": "mainFrame", "children": []}'
        elif format == "yaml":
            return "window_title: Main Frame\nchildren: []"
        else:
            return "JFrame [mainFrame]\n  JPanel [contentPane]"

    def get_ui_tree(self, locator: Optional[str] = None) -> str:
        """New API get UI tree."""
        return "JFrame [mainFrame]\n  JPanel [contentPane]\n    JButton [loginBtn]"

    def log_ui_tree(self, locator: Optional[str] = None) -> None:
        """New API log UI tree."""
        print(self.get_ui_tree(locator))

    def save_ui_tree(self, filename: str, locator: Optional[str] = None) -> None:
        """New API save UI tree."""
        pass

    def refresh_ui_tree(self) -> None:
        """New API refresh UI tree."""
        pass

    def capture_screenshot(
        self, filename: Optional[str] = None, locator: Optional[str] = None
    ) -> str:
        if filename:
            return f"/tmp/screenshots/{filename}"
        return "/tmp/screenshots/screenshot_001.png"

    def set_screenshot_directory(self, directory: str) -> None:
        """Set screenshot directory."""
        self.screenshot_directory = directory

    def set_timeout(self, timeout: float) -> None:
        """Set timeout."""
        self.timeout = timeout

    def element_should_be_visible(self, locator: str) -> None:
        """Verify element is visible."""
        elem = self.find_element(locator)
        if not elem.is_visible:
            raise AssertionError(f"Element not visible: {locator}")

    def element_should_not_be_visible(self, locator: str) -> None:
        """Verify element is not visible."""
        try:
            elem = self.find_element(locator)
            if elem.is_visible:
                raise AssertionError(f"Element is visible: {locator}")
        except ElementNotFoundError:
            pass

    def element_should_be_enabled(self, locator: str) -> None:
        """Verify element is enabled."""
        elem = self.find_element(locator)
        if not elem.is_enabled:
            raise AssertionError(f"Element not enabled: {locator}")

    def element_should_be_disabled(self, locator: str) -> None:
        """Verify element is disabled."""
        elem = self.find_element(locator)
        if elem.is_enabled:
            raise AssertionError(f"Element is enabled: {locator}")

    def element_text_should_be(self, locator: str, expected: str) -> None:
        """Verify element text equals expected."""
        actual = self.get_element_text(locator)
        if actual != expected:
            raise AssertionError(f"Text '{actual}' != '{expected}'")

    def element_text_should_contain(self, locator: str, expected: str) -> None:
        """Verify element text contains expected."""
        actual = self.get_element_text(locator)
        if expected not in actual:
            raise AssertionError(f"Text '{actual}' does not contain '{expected}'")

    def get_element_property(self, locator: str, property_name: str) -> Any:
        """Get element property."""
        elem = self.find_element(locator)
        return elem._properties.get(property_name)

    def check_checkbox(self, locator: str) -> None:
        """Check a checkbox."""
        self.find_element(locator)

    def uncheck_checkbox(self, locator: str) -> None:
        """Uncheck a checkbox."""
        self.find_element(locator)

    def select_radio_button(self, locator: str) -> None:
        """Select a radio button."""
        self.find_element(locator)

    def select_from_combobox(self, locator: str, value: str) -> None:
        """Select from combobox."""
        self.find_element(locator)

    def select_menu(self, menu_path: str) -> None:
        """Select menu item."""
        pass

    def select_from_popup_menu(self, menu_path: str) -> None:
        """Select from popup menu."""
        pass

    def get_table_column_count(self, locator: str) -> int:
        """Get table column count."""
        self.find_element(locator)
        return 5

    def get_selected_tree_node(self, locator: str) -> Optional[str]:
        """Get selected tree node."""
        self.find_element(locator)
        return "Root/Selected"


class SwingError(Exception):
    """Base exception for Swing errors."""
    pass


class ConnectionError(SwingError):
    """Connection error."""
    pass


class ElementNotFoundError(SwingError):
    """Element not found error."""
    pass


class TimeoutError(SwingError):
    """Timeout error."""
    pass


@pytest.fixture
def mock_rust_core():
    """Fixture to mock the Rust core module."""
    mock_module = Mock()
    mock_module.SwingLibrary = MockSwingLibrary
    mock_module.SwingElement = MockSwingElement
    mock_module.SwingError = SwingError
    mock_module.ConnectionError = ConnectionError
    mock_module.ElementNotFoundError = ElementNotFoundError
    mock_module.TimeoutError = TimeoutError

    with patch.dict('sys.modules', {'swing_library._core': mock_module}):
        # Reload the module to pick up the mock
        import importlib
        import sys
        if 'swing_library' in sys.modules:
            del sys.modules['swing_library']
        yield mock_module


@pytest.fixture
def mock_element():
    """Fixture for a mock swing element."""
    return MockSwingElement()


@pytest.fixture
def mock_disabled_element():
    """Fixture for a disabled mock element."""
    return MockSwingElement(enabled=False)


@pytest.fixture
def mock_hidden_element():
    """Fixture for a hidden mock element."""
    return MockSwingElement(visible=False)


@pytest.fixture
def mock_button():
    """Fixture for a mock button element."""
    return MockSwingElement(
        id=1,
        class_name="javax.swing.JButton",
        name="submitBtn",
        text="Submit",
        properties={"mnemonic": "S", "toolTipText": "Click to submit"}
    )


@pytest.fixture
def mock_text_field():
    """Fixture for a mock text field element."""
    return MockSwingElement(
        id=2,
        class_name="javax.swing.JTextField",
        name="inputField",
        text="Initial text",
        properties={"columns": 20, "editable": True}
    )


@pytest.fixture
def mock_table():
    """Fixture for a mock table element."""
    return MockSwingElement(
        id=3,
        class_name="javax.swing.JTable",
        name="dataTable",
        text=None,
        properties={"rowCount": 10, "columnCount": 5}
    )
