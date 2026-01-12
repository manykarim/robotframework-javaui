"""
Integration tests for SwingLibrary.

These tests require a running Swing application and are marked as integration tests.
Run with: pytest -m integration
"""

import pytest
from unittest.mock import Mock
import sys
import os

# Add parent directory to path for imports
sys.path.insert(0, os.path.dirname(__file__))

from conftest import MockSwingLibrary, MockSwingElement


@pytest.mark.integration
class TestFullWorkflow:
    """Test complete workflow scenarios."""

    def test_login_workflow(self):
        """Test a complete login workflow."""
        lib = MockSwingLibrary()

        # Connect to application
        lib.connect(main_class="com.example.LoginApp")
        assert lib._connected is True

        # Find and interact with login form elements
        username_field = lib.find_element("JTextField#username")
        assert username_field is not None

        # Input credentials
        lib.input_text("JTextField#username", "testuser")
        lib.input_text("JPasswordField#password", "secret123")

        # Click login button
        lib.click("JButton#loginBtn")

        # Disconnect
        lib.disconnect()
        assert lib._connected is False

    def test_table_operations_workflow(self):
        """Test table operations workflow."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get table row count
        row_count = lib.get_table_row_count("JTable#dataTable")
        assert row_count > 0

        # Get cell values
        for row in range(min(3, row_count)):
            value = lib.get_table_cell_value("JTable#dataTable", row, 0)
            assert value is not None

        # Select a cell
        lib.select_table_cell("JTable#dataTable", 0, 0)

        lib.disconnect()

    def test_tree_navigation_workflow(self):
        """Test tree navigation workflow."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Expand nodes
        lib.expand_tree_node("JTree#fileTree", "Root")
        lib.expand_tree_node("JTree#fileTree", "Root/Documents")

        # Select a node
        lib.select_tree_node("JTree#fileTree", "Root/Documents/file.txt")

        # Collapse nodes
        lib.collapse_tree_node("JTree#fileTree", "Root/Documents")

        lib.disconnect()

    def test_form_input_workflow(self):
        """Test form input workflow."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Input text
        lib.input_text("JTextField#username", "john.doe")

        # Clear and re-input
        lib.clear_text("JTextField#username")
        lib.input_text("JTextField#username", "jane.doe")

        # Type text character by character
        lib.type_text("JTextField#username", "test")

        lib.disconnect()


@pytest.mark.integration
class TestMultiWindowWorkflow:
    """Test multi-window scenarios."""

    def test_dialog_handling(self):
        """Test dialog handling workflow."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Main window operations
        lib.click("JButton#loginBtn")

        # Component tree should include dialogs if any
        tree = lib.get_component_tree(format="json")
        assert tree is not None

        lib.disconnect()

    def test_application_listing(self):
        """Test listing running applications."""
        lib = MockSwingLibrary()

        apps = lib.list_applications()
        assert len(apps) > 0

        for app in apps:
            assert "pid" in app
            assert "main_class" in app


@pytest.mark.integration
class TestScreenshotWorkflow:
    """Test screenshot capture workflows."""

    def test_capture_on_navigation(self):
        """Test capturing screenshots during navigation."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Capture full window
        path1 = lib.capture_screenshot(filename="step1.png")
        assert path1 is not None

        # Navigate and capture again
        lib.click("JButton#loginBtn")
        path2 = lib.capture_screenshot(filename="step2.png")
        assert path2 is not None

        lib.disconnect()

    def test_capture_specific_element(self):
        """Test capturing specific element screenshot."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Capture table only
        path = lib.capture_screenshot(locator="JTable#dataTable")
        assert path is not None

        lib.disconnect()


@pytest.mark.integration
class TestWaitWorkflow:
    """Test wait-based workflows."""

    def test_wait_for_element_before_interaction(self):
        """Test waiting for element before interacting."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Wait for button to appear
        elem = lib.wait_for_element("JButton#loginBtn", timeout_ms=5000)
        assert elem is not None

        # Now interact
        lib.click("JButton#loginBtn")

        lib.disconnect()

    def test_wait_for_visibility(self):
        """Test waiting for element visibility."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Wait until visible
        lib.wait_until_visible("JButton#loginBtn", timeout_ms=5000)

        # Should be visible now
        lib.element_should_be_visible("JButton#loginBtn")

        lib.disconnect()

    def test_wait_for_enabled(self):
        """Test waiting for element to be enabled."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Wait until enabled
        lib.wait_until_enabled("JButton#loginBtn", timeout_ms=5000)

        # Should be enabled now
        lib.element_should_be_enabled("JButton#loginBtn")

        lib.disconnect()


@pytest.mark.integration
class TestComponentTreeWorkflow:
    """Test component tree inspection workflows."""

    def test_inspect_tree_formats(self):
        """Test getting tree in different formats."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # JSON format
        json_tree = lib.get_component_tree(format="json")
        assert "JFrame" in json_tree

        # Text format
        text_tree = lib.get_component_tree(format="text")
        assert "JFrame" in text_tree

        # YAML format
        yaml_tree = lib.get_component_tree(format="yaml")
        assert "window_title" in yaml_tree

        lib.disconnect()

    def test_inspect_with_depth_limit(self):
        """Test tree inspection with depth limit."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Full depth
        full_tree = lib.get_component_tree(format="json")

        # Limited depth
        limited_tree = lib.get_component_tree(format="json", max_depth=2)

        # Both should have content
        assert full_tree is not None
        assert limited_tree is not None

        lib.disconnect()
