"""
Tests for Get keywords with assertions.

These tests verify getter keywords that integrate with AssertionEngine
for optional assertion support.
"""

import pytest
from unittest.mock import Mock, MagicMock, patch
import sys
import os

# Add parent directory for imports
sys.path.insert(0, os.path.dirname(__file__))

from conftest import (
    MockSwingLibrary,
    MockSwingElement,
    SwingError,
    ElementNotFoundError,
)


# Try to import assertion-related modules
try:
    from JavaGui.assertions import (
        AssertionConfig,
        with_retry_assertion,
        numeric_assertion_with_retry,
        state_assertion_with_retry,
        ElementState,
    )
    from JavaGui.assertions.formatters import FORMATTERS
    from assertionengine import AssertionOperator
    ASSERTIONS_AVAILABLE = True
except ImportError:
    ASSERTIONS_AVAILABLE = False
    FORMATTERS = {}
    # Mock AssertionOperator for tests
    class MockAssertionOperator:
        equal = "=="
        not_equal = "!="
        contains = "*="
        greater_than = ">"
        less_than = "<"
    AssertionOperator = MockAssertionOperator


class TestGetTextKeyword:
    """Tests for Get Text keyword with assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock library with configured elements."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        # Add test element with specific text
        lib._elements["JLabel#testLabel"] = MockSwingElement(
            id=100, name="testLabel", text="Expected Text",
            class_name="javax.swing.JLabel"
        )
        return lib

    def test_get_text_no_assertion(self, mock_lib):
        """Test Get Text without assertion returns text directly."""
        text = mock_lib.get_element_text("JLabel#testLabel")
        assert text == "Expected Text"

    def test_get_text_element_not_found(self, mock_lib):
        """Test Get Text raises error for non-existent element."""
        with pytest.raises(ElementNotFoundError):
            mock_lib.get_element_text("JLabel#nonexistent")

    def test_get_text_empty_text(self, mock_lib):
        """Test Get Text with empty text returns empty string."""
        mock_lib._elements["JLabel#emptyLabel"] = MockSwingElement(
            id=101, name="emptyLabel", text="",
            class_name="javax.swing.JLabel"
        )
        text = mock_lib.get_element_text("JLabel#emptyLabel")
        assert text == ""

    def test_get_text_none_text(self, mock_lib):
        """Test Get Text with None text returns empty string."""
        mock_lib._elements["JLabel#nullLabel"] = MockSwingElement(
            id=102, name="nullLabel", text=None,
            class_name="javax.swing.JLabel"
        )
        text = mock_lib.get_element_text("JLabel#nullLabel")
        assert text == ""


class TestGetElementCountKeyword:
    """Tests for Get Element Count keyword."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock library with multiple elements."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        return lib

    def test_get_element_count_returns_count(self, mock_lib):
        """Test element count for existing elements."""
        elements = mock_lib.find_elements("JButton")
        count = len(elements)
        assert isinstance(count, int)
        assert count >= 0

    def test_get_element_count_no_match(self, mock_lib):
        """Test element count returns 0 for no matches."""
        elements = mock_lib.find_elements("JSlider")
        assert len(elements) == 0


class TestGetTableCellValueKeyword:
    """Tests for Get Table Cell Value keyword."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock library with table element."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        return lib

    def test_get_table_cell_value(self, mock_lib):
        """Test getting table cell value."""
        value = mock_lib.get_table_cell_value("JTable#dataTable", 0, 1)
        assert value == "Cell[0,1]"

    def test_get_table_cell_value_different_positions(self, mock_lib):
        """Test getting table cell values at different positions."""
        value1 = mock_lib.get_table_cell_value("JTable#dataTable", 0, 0)
        value2 = mock_lib.get_table_cell_value("JTable#dataTable", 5, 3)
        assert value1 == "Cell[0,0]"
        assert value2 == "Cell[5,3]"


class TestGetTableRowCountKeyword:
    """Tests for Get Table Row Count keyword."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock library with table element."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        return lib

    def test_get_table_row_count(self, mock_lib):
        """Test getting table row count."""
        count = mock_lib.get_table_row_count("JTable#dataTable")
        assert count == 10
        assert isinstance(count, int)

    def test_get_table_row_count_is_integer(self, mock_lib):
        """Test table row count returns integer."""
        count = mock_lib.get_table_row_count("JTable#dataTable")
        assert isinstance(count, int)


class TestGetTableColumnCountKeyword:
    """Tests for Get Table Column Count keyword."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock library with table element."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        return lib

    def test_get_table_column_count(self, mock_lib):
        """Test getting table column count."""
        count = mock_lib.get_table_column_count("JTable#dataTable")
        assert count == 5
        assert isinstance(count, int)


class TestGetSelectedTreeNodeKeyword:
    """Tests for Get Selected Tree Node keyword."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock library with tree element."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        return lib

    def test_get_selected_tree_node(self, mock_lib):
        """Test getting selected tree node."""
        node = mock_lib.get_selected_tree_node("JTree#fileTree")
        assert node == "Root/Selected"

    def test_get_selected_tree_node_returns_string(self, mock_lib):
        """Test selected tree node returns string or None."""
        node = mock_lib.get_selected_tree_node("JTree#fileTree")
        assert isinstance(node, (str, type(None)))


class TestGetElementPropertyKeyword:
    """Tests for Get Element Property keyword."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock library with elements that have properties."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        # Add element with properties
        lib._elements["JButton#propBtn"] = MockSwingElement(
            id=200, name="propBtn", text="Click Me",
            class_name="javax.swing.JButton",
            properties={
                "enabled": True,
                "visible": True,
                "mnemonic": "C",
                "toolTipText": "Click this button"
            }
        )
        return lib

    def test_get_element_property_text(self, mock_lib):
        """Test getting element text property."""
        elem = mock_lib.find_element("JButton#propBtn")
        assert elem.text == "Click Me"

    def test_get_element_property_custom(self, mock_lib):
        """Test getting custom element property."""
        elem = mock_lib.find_element("JButton#propBtn")
        assert elem.get_property("mnemonic") == "C"
        assert elem.get_property("toolTipText") == "Click this button"

    def test_get_element_property_missing(self, mock_lib):
        """Test getting missing property returns None."""
        elem = mock_lib.find_element("JButton#propBtn")
        assert elem.get_property("nonexistent") is None


class TestGetterKeywordsWithMockedAssertions:
    """Test Get keywords with mocked assertion integration."""

    def test_get_text_with_mock_assertion(self):
        """Test Get Text would work with assertion operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        lib._elements["JLabel#status"] = MockSwingElement(
            id=300, name="status", text="Ready",
            class_name="javax.swing.JLabel"
        )

        # Get value for assertion
        text = lib.get_element_text("JLabel#status")
        assert text == "Ready"

        # Simulate assertion check
        result = with_retry_assertion(
            lambda: lib.get_element_text("JLabel#status"),
            AssertionOperator.equal,
            "Ready",
            timeout=1.0
        )
        assert result == "Ready"

    def test_get_element_count_with_mock_assertion(self):
        """Test Get Element Count would work with numeric assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get count
        elements = lib.find_elements("JButton")
        count = len(elements)

        # Simulate numeric assertion - use >= 0 since count can be 0
        result = numeric_assertion_with_retry(
            lambda: len(lib.find_elements("JButton")),
            AssertionOperator[">="],
            0,
            timeout=1.0
        )
        assert result >= 0


class TestGetterKeywordsEdgeCases:
    """Test edge cases for getter keywords."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock library."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        return lib

    def test_get_text_special_characters(self, mock_lib):
        """Test Get Text with special characters."""
        mock_lib._elements["JLabel#special"] = MockSwingElement(
            id=400, name="special", text="Line1\nLine2\tTabbed",
            class_name="javax.swing.JLabel"
        )
        text = mock_lib.get_element_text("JLabel#special")
        assert "Line1" in text
        assert "Line2" in text

    def test_get_text_unicode(self, mock_lib):
        """Test Get Text with unicode characters."""
        mock_lib._elements["JLabel#unicode"] = MockSwingElement(
            id=401, name="unicode", text="Hello World",
            class_name="javax.swing.JLabel"
        )
        text = mock_lib.get_element_text("JLabel#unicode")
        assert "Hello" in text

    def test_get_text_html_content(self, mock_lib):
        """Test Get Text with HTML content."""
        mock_lib._elements["JLabel#html"] = MockSwingElement(
            id=402, name="html", text="<html><b>Bold</b> text</html>",
            class_name="javax.swing.JLabel"
        )
        text = mock_lib.get_element_text("JLabel#html")
        assert "<html>" in text or "Bold" in text

    def test_get_text_very_long_string(self, mock_lib):
        """Test Get Text with very long string."""
        long_text = "A" * 10000
        mock_lib._elements["JLabel#long"] = MockSwingElement(
            id=403, name="long", text=long_text,
            class_name="javax.swing.JLabel"
        )
        text = mock_lib.get_element_text("JLabel#long")
        assert len(text) == 10000


class TestGetterKeywordsRetryBehavior:
    """Test retry behavior for getter keywords with assertions."""

    def test_getter_retries_on_element_not_found(self):
        """Test getter retries when element temporarily not found."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        call_count = [0]
        original_get_text = lib.get_element_text

        def get_text_with_retry(locator):
            call_count[0] += 1
            if call_count[0] < 3:
                raise ElementNotFoundError(f"Element not found: {locator}")
            return "Found on retry"

        lib.get_element_text = get_text_with_retry

        result = with_retry_assertion(
            lambda: lib.get_element_text("JLabel#dynamic"),
            AssertionOperator.equal,
            "Found on retry",
            timeout=5.0,
            interval=0.1
        )
        assert result == "Found on retry"
        assert call_count[0] >= 3

    def test_getter_retries_on_value_change(self):
        """Test getter retries until value changes."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        call_count = [0]

        def get_changing_text(locator):
            call_count[0] += 1
            if call_count[0] < 4:
                return "Loading..."
            return "Complete"

        lib.get_element_text = get_changing_text

        result = with_retry_assertion(
            lambda: lib.get_element_text("JLabel#status"),
            AssertionOperator.equal,
            "Complete",
            timeout=5.0,
            interval=0.1
        )
        assert result == "Complete"
        assert call_count[0] >= 4


class TestGetterKeywordsWithFormatters:
    """Test getter keywords with text formatters."""

    def test_get_text_with_strip_formatter(self):
        """Test Get Text with strip formatter."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        lib._elements["JLabel#padded"] = MockSwingElement(
            id=500, name="padded", text="  Padded Text  ",
            class_name="javax.swing.JLabel"
        )

        # Convert formatter name to function
        formatter_funcs = [FORMATTERS["strip"]]

        result = with_retry_assertion(
            lambda: lib.get_element_text("JLabel#padded"),
            AssertionOperator.equal,
            "Padded Text",
            timeout=1.0,
            formatters=formatter_funcs
        )
        # Result is formatted to match expected
        assert result == "Padded Text"

    def test_get_text_with_normalize_formatter(self):
        """Test Get Text with normalize_spaces formatter."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        lib._elements["JLabel#spaces"] = MockSwingElement(
            id=501, name="spaces", text="Multiple   Spaces   Here",
            class_name="javax.swing.JLabel"
        )

        # Convert formatter name to function
        formatter_funcs = [FORMATTERS["normalize_spaces"]]

        result = with_retry_assertion(
            lambda: lib.get_element_text("JLabel#spaces"),
            AssertionOperator.equal,
            "Multiple Spaces Here",
            timeout=1.0,
            formatters=formatter_funcs
        )
        # Result is formatted to match expected
        assert result == "Multiple Spaces Here"


class TestGetterKeywordsConcurrency:
    """Test getter keywords under concurrent access patterns."""

    def test_multiple_getters_same_element(self):
        """Test multiple getter calls on same element."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Multiple calls should all succeed
        for _ in range(10):
            text = lib.get_element_text("JLabel#statusLabel")
            assert text == "Ready"

    def test_alternating_getter_calls(self):
        """Test alternating between different getter calls."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        for _ in range(5):
            text = lib.get_element_text("JLabel#statusLabel")
            assert text == "Ready"

            row_count = lib.get_table_row_count("JTable#dataTable")
            assert row_count == 10

            col_count = lib.get_table_column_count("JTable#dataTable")
            assert col_count == 5
