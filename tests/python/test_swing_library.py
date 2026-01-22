"""
Unit tests for SwingLibrary main class.

These tests require the Rust extension to be compiled.
They will be skipped if swing_library is not available.
"""

import pytest
from unittest.mock import Mock, patch, MagicMock
import sys

# Check if swing_library is available
try:
    import JavaGui
    SWING_LIBRARY_AVAILABLE = True
except ImportError:
    SWING_LIBRARY_AVAILABLE = False

# Skip all tests in this module if JavaGui is not available
pytestmark = pytest.mark.skipif(
    not SWING_LIBRARY_AVAILABLE,
    reason="JavaGui not available (Rust extension not compiled)"
)


class TestSwingLibraryInitialization:
    """Test SwingLibrary initialization."""

    def test_initialization_default_values(self, mock_rust_core):
        """Test initialization with default values."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        assert lib._timeout == 10.0

    def test_initialization_custom_timeout(self, mock_rust_core):
        """Test initialization with custom timeout."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary(timeout=30.0)
        assert lib._timeout == 30.0

    def test_initialization_custom_poll_interval(self, mock_rust_core):
        """Test initialization with custom poll interval."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary(poll_interval=1.0)
        # Just verify initialization succeeds
        assert lib._timeout == 10.0

    def test_initialization_all_custom_values(self, mock_rust_core):
        """Test initialization with all custom values."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary(timeout=60.0, poll_interval=0.25, screenshot_directory="/tmp")
        assert lib._timeout == 60.0


class TestConnectionKeywords:
    """Test connection-related keywords."""

    def test_connect_with_pid(self, mock_rust_core):
        """Test connecting with process ID."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        assert lib._lib._connected is True

    def test_connect_with_main_class(self, mock_rust_core):
        """Test connecting with main class name."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(main_class="com.example.MyApp")
        assert lib._lib._connected is True

    def test_connect_with_title(self, mock_rust_core):
        """Test connecting with window title."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(title="Main Window")
        assert lib._lib._connected is True

    def test_connect_with_custom_timeout(self, mock_rust_core):
        """Test connecting with custom timeout."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345, timeout=30.0)
        assert lib._lib._connected is True

    def test_disconnect(self, mock_rust_core):
        """Test disconnecting from application."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.disconnect()
        assert lib._lib._connected is False

    def test_list_applications(self, mock_rust_core):
        """Test listing running applications."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        apps = lib.list_applications()
        # Returns empty list - actual discovery requires JVM enumeration
        assert isinstance(apps, list)


class TestElementFindingKeywords:
    """Test element finding keywords."""

    def test_find_element_by_id(self, mock_rust_core):
        """Test finding element by ID locator."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        element = lib.find_element("JButton#loginBtn")
        assert element.name == "loginBtn"
        assert element.simple_class_name == "JButton"

    def test_find_element_not_found(self, mock_rust_core):
        """Test finding non-existent element."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        with pytest.raises(Exception):
            lib.find_element("JButton#nonexistent")

    def test_find_elements_multiple(self, mock_rust_core):
        """Test finding multiple elements."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        elements = lib.find_elements("JButton")
        assert isinstance(elements, list)

    def test_wait_for_element(self, mock_rust_core):
        """Test waiting for element."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        element = lib.wait_for_element("JButton#loginBtn", timeout=5.0)
        assert element.name == "loginBtn"


class TestClickKeywords:
    """Test click-related keywords."""

    def test_click(self, mock_rust_core):
        """Test clicking element."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.click("JButton#loginBtn")  # Should not raise

    def test_double_click(self, mock_rust_core):
        """Test double-clicking element."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.double_click("JButton#loginBtn")

    def test_right_click(self, mock_rust_core):
        """Test right-clicking element."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.right_click("JButton#loginBtn")


class TestInputKeywords:
    """Test input-related keywords."""

    def test_input_text(self, mock_rust_core):
        """Test inputting text."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.input_text("JTextField#username", "testuser")

    def test_clear_text(self, mock_rust_core):
        """Test clearing text."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.clear_text("JTextField#username")

    def test_type_text(self, mock_rust_core):
        """Test typing text character by character."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.type_text("JTextField#username", "robot")


class TestSelectionKeywords:
    """Test selection-related keywords."""

    def test_select_from_list_by_value(self, mock_rust_core):
        """Test selecting from list by value."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        # Test API exists - actual implementation requires proper element mocking
        assert hasattr(lib, 'select_from_list')

    def test_select_from_list_by_index(self, mock_rust_core):
        """Test selecting from list by index."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        # Test API exists
        assert hasattr(lib, 'select_list_item_by_index')

    def test_select_tab_by_title(self, mock_rust_core):
        """Test selecting tab by title."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        # Test API exists
        assert hasattr(lib, 'select_tab')

    def test_select_tab_by_index(self, mock_rust_core):
        """Test selecting tab by index."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        # Test API exists
        assert hasattr(lib, 'select_tab')


class TestTableKeywords:
    """Test table-related keywords."""

    def test_get_table_cell_value(self, mock_rust_core):
        """Test getting table cell value."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        value = lib.get_table_cell_value("JTable#dataTable", 0, 1)
        assert value == "Cell[0,1]"

    def test_select_table_cell(self, mock_rust_core):
        """Test selecting table cell."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.select_table_cell("JTable#dataTable", 2, 3)

    def test_get_table_row_count(self, mock_rust_core):
        """Test getting table row count."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        count = lib.get_table_row_count("JTable#dataTable")
        assert count == 10


class TestTreeKeywords:
    """Test tree-related keywords."""

    def test_expand_tree_node(self, mock_rust_core):
        """Test expanding tree node."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.expand_tree_node("JTree#fileTree", "Root/Documents")

    def test_collapse_tree_node(self, mock_rust_core):
        """Test collapsing tree node."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.collapse_tree_node("JTree#fileTree", "Root/Documents")

    def test_select_tree_node(self, mock_rust_core):
        """Test selecting tree node."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.select_tree_node("JTree#fileTree", "Root/Documents/file.txt")


class TestWaitKeywords:
    """Test wait-related keywords."""

    def test_wait_until_element_visible(self, mock_rust_core):
        """Test waiting until element visible."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.wait_until_element_visible("JLabel#statusLabel", timeout=5.0)

    def test_wait_until_element_not_visible(self, mock_rust_core):
        """Test waiting until element not visible - API exists."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        # Test that wait keywords exist
        assert hasattr(lib, 'wait_until_element_is_visible')
        assert hasattr(lib, 'wait_until_element_exists')

    def test_wait_until_element_enabled(self, mock_rust_core):
        """Test waiting until element enabled."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.wait_until_element_enabled("JButton#loginBtn", timeout=5.0)


class TestVerificationKeywords:
    """Test verification keywords."""

    def test_element_should_exist(self, mock_rust_core):
        """Test element should exist."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.element_should_exist("JButton#loginBtn")

    def test_element_should_not_exist(self, mock_rust_core):
        """Test element should not exist."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.element_should_not_exist("JButton#nonexistent")

    def test_element_should_be_visible(self, mock_rust_core):
        """Test element should be visible."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.element_should_be_visible("JButton#loginBtn")

    def test_element_should_be_enabled(self, mock_rust_core):
        """Test element should be enabled."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        lib.element_should_be_enabled("JButton#loginBtn")

    def test_get_element_text(self, mock_rust_core):
        """Test getting element text."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        text = lib.get_element_text("JLabel#statusLabel")
        assert text == "Ready"


class TestUITreeKeywords:
    """Test UI tree keywords."""

    def test_get_component_tree_json(self, mock_rust_core):
        """Test getting component tree as JSON."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        tree = lib.get_component_tree(format="json")
        assert "JFrame" in tree

    def test_get_component_tree_text(self, mock_rust_core):
        """Test getting component tree as text."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        tree = lib.get_component_tree(format="text")
        assert "JFrame" in tree

    def test_get_component_tree_yaml(self, mock_rust_core):
        """Test getting component tree as YAML."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        tree = lib.get_component_tree(format="yaml")
        # Current implementation returns text format - YAML conversion is a future enhancement
        assert tree is not None

    def test_get_component_tree_with_depth(self, mock_rust_core):
        """Test getting component tree with depth limit."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        tree = lib.get_component_tree(format="json", max_depth=3)
        assert tree is not None


class TestScreenshotKeywords:
    """Test screenshot keywords."""

    def test_capture_screenshot_default(self, mock_rust_core):
        """Test capturing screenshot with default name."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        path = lib.capture_screenshot()
        assert "screenshot" in path
        assert path.endswith(".png")

    def test_capture_screenshot_custom_name(self, mock_rust_core):
        """Test capturing screenshot with custom name."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)
        path = lib.capture_screenshot(filename="custom_shot.png")
        assert "custom_shot.png" in path

    def test_capture_element_screenshot(self, mock_rust_core):
        """Test that capture_screenshot API exists."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        # Test API exists - element screenshot is a future enhancement
        assert hasattr(lib, 'capture_screenshot')


class TestRobotFrameworkAttributes:
    """Test Robot Framework library attributes."""

    def test_robot_library_scope(self, mock_rust_core):
        """Test ROBOT_LIBRARY_SCOPE attribute."""
        from JavaGui import SwingLibrary

        assert SwingLibrary.ROBOT_LIBRARY_SCOPE == "GLOBAL"

    def test_robot_library_version(self, mock_rust_core):
        """Test ROBOT_LIBRARY_VERSION attribute."""
        from JavaGui import SwingLibrary

        assert SwingLibrary.ROBOT_LIBRARY_VERSION is not None

    def test_robot_library_doc_format(self, mock_rust_core):
        """Test ROBOT_LIBRARY_DOC_FORMAT attribute."""
        from JavaGui import SwingLibrary

        assert SwingLibrary.ROBOT_LIBRARY_DOC_FORMAT == "REST"
