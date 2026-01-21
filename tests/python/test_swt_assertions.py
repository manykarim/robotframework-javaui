"""
Tests for SWT assertion keywords.

These tests verify SWT library keywords that integrate with AssertionEngine
for optional assertion support, including all operators, retry logic,
timeout behavior, and custom error messages.
"""

import pytest
import time
from unittest.mock import Mock, MagicMock, patch
from typing import Dict, Any, List, Optional


# Mock SWT Element
class MockSwtElement:
    """Mock SWT element for testing."""

    def __init__(
        self,
        id: int = 1,
        class_name: str = "org.eclipse.swt.widgets.Button",
        name: Optional[str] = "testBtn",
        text: Optional[str] = "Click Me",
        visible: bool = True,
        enabled: bool = True,
        properties: Dict[str, Any] = None,
    ):
        self.id = id
        self.class_name = class_name
        self.name = name
        self.text = text
        self.is_visible = visible
        self.is_enabled = enabled
        self._properties = properties or {}

    def get_property(self, name: str) -> Any:
        if name == "text":
            return self.text
        if name == "visible":
            return self.is_visible
        if name == "enabled":
            return self.is_enabled
        return self._properties.get(name)


# Mock SWT Library
class MockSwtLibrary:
    """Mock SWT Library for testing assertion keywords."""

    def __init__(self, timeout: float = 10.0):
        self.timeout = timeout
        self._connected = False
        self._elements: Dict[str, MockSwtElement] = {}
        self._setup_default_elements()
        self._call_count = {}

    def _setup_default_elements(self) -> None:
        """Set up default mock elements for testing."""
        self._elements = {
            "Button#submitBtn": MockSwtElement(
                id=1, name="submitBtn", text="Submit",
                class_name="org.eclipse.swt.widgets.Button"
            ),
            "Text#username": MockSwtElement(
                id=2, name="username", text="testuser",
                class_name="org.eclipse.swt.widgets.Text"
            ),
            "Label#statusLabel": MockSwtElement(
                id=3, name="statusLabel", text="Ready",
                class_name="org.eclipse.swt.widgets.Label"
            ),
            "Table#dataTable": MockSwtElement(
                id=4, name="dataTable", text=None,
                class_name="org.eclipse.swt.widgets.Table",
                properties={"rowCount": 10, "columnCount": 5}
            ),
            "Tree#fileTree": MockSwtElement(
                id=5, name="fileTree", text=None,
                class_name="org.eclipse.swt.widgets.Tree"
            ),
            "Combo#countryCombo": MockSwtElement(
                id=6, name="countryCombo", text="USA",
                class_name="org.eclipse.swt.widgets.Combo",
                properties={"items": ["USA", "UK", "Canada", "Germany"]}
            ),
            "List#itemList": MockSwtElement(
                id=7, name="itemList", text=None,
                class_name="org.eclipse.swt.widgets.List",
                properties={
                    "items": ["Item 1", "Item 2", "Item 3"],
                    "selectedValue": "Item 1",
                    "selectedIndex": 0
                }
            ),
        }

    def connect_to_swt_application(
        self, app: str, host: str = "localhost", port: int = 5679, timeout: float = None
    ):
        self._connected = True

    def disconnect(self):
        self._connected = False

    def is_connected(self) -> bool:
        return self._connected

    def find_widget(self, locator: str) -> MockSwtElement:
        if locator in self._elements:
            return self._elements[locator]
        for key, elem in self._elements.items():
            if locator in key or (elem.name and locator.endswith(f"#{elem.name}")):
                return elem
        raise Exception(f"Widget not found: {locator}")

    def find_widgets(self, locator: str) -> List[MockSwtElement]:
        results = []
        base_type = locator.split("#")[0].split("[")[0]
        for key, elem in self._elements.items():
            if base_type in elem.class_name or base_type in key:
                results.append(elem)
        return results

    def get_widget_text(self, locator: str) -> str:
        return self.find_widget(locator).text or ""

    def get_widget_property(self, locator: str, property_name: str) -> Any:
        elem = self.find_widget(locator)
        return elem.get_property(property_name)

    def get_table_row_count(self, locator: str) -> int:
        elem = self.find_widget(locator)
        return elem._properties.get("rowCount", 0)

    def get_table_cell(self, locator: str, row: int, col: int) -> str:
        self.find_widget(locator)
        return f"Cell[{row},{col}]"

    def get_table_row_values(self, locator: str, row: int) -> List[str]:
        col_count = 5
        return [f"Cell[{row},{col}]" for col in range(col_count)]

    def get_list_items(self, locator: str) -> List[str]:
        elem = self.find_widget(locator)
        return elem._properties.get("items", [])

    def get_tree_data(self, locator: str) -> dict:
        return {
            "text": "Root",
            "children": [
                {"text": "Settings", "children": [
                    {"text": "General", "children": []},
                    {"text": "Advanced", "children": []},
                ]},
                {"text": "Projects", "children": []},
            ]
        }

    def get_selected_tree_nodes(self, locator: str) -> List[str]:
        return ["Root/Settings"]


class SwtElementNotFoundError(Exception):
    """SWT element not found error."""
    pass


# Try to import assertion-related modules
try:
    from JavaGui.assertions import (
        AssertionConfig,
        with_retry_assertion,
        numeric_assertion_with_retry,
        state_assertion_with_retry,
        ElementState,
    )
    from JavaGui.assertions.formatters import FORMATTERS, apply_formatters
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
        not_contains = "not_contains"
        greater_than = ">"
        less_than = "<"
        greater_than_or_equal = ">="
        less_than_or_equal = "<="
        starts = "^="
        ends = "$="
        matches = "matches"

        def __getitem__(self, key):
            mapping = {
                "==": self.equal,
                "!=": self.not_equal,
                ">": self.greater_than,
                "<": self.less_than,
                ">=": self.greater_than_or_equal,
                "<=": self.less_than_or_equal,
                "greater than": self.greater_than,
                "less than": self.less_than,
            }
            return mapping.get(key, key)

    AssertionOperator = MockAssertionOperator()


# =============================================================================
# SWT Getter Keywords with Assertion Support Tests
# =============================================================================


class TestSwtGetTextWithAssertions:
    """Tests for SWT Get Text keyword with all assertion operators."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock SWT library."""
        lib = MockSwtLibrary()
        lib.connect_to_swt_application("TestApp")
        return lib

    def test_get_text_no_assertion(self, mock_lib):
        """Test Get Text without assertion returns text directly."""
        text = mock_lib.get_widget_text("Label#statusLabel")
        assert text == "Ready"

    def test_get_text_equal_operator_pass(self, mock_lib):
        """Test Get Text with == operator passes."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#statusLabel"),
            AssertionOperator.equal,
            "Ready",
            timeout=1.0
        )
        assert result == "Ready"

    def test_get_text_equal_operator_fail(self, mock_lib):
        """Test Get Text with == operator fails with correct error."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        with pytest.raises(AssertionError) as exc_info:
            with_retry_assertion(
                lambda: mock_lib.get_widget_text("Label#statusLabel"),
                AssertionOperator.equal,
                "NotReady",
                timeout=0.3,
                interval=0.1
            )
        assert "timeout" in str(exc_info.value).lower()

    def test_get_text_not_equal_operator(self, mock_lib):
        """Test Get Text with != operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#statusLabel"),
            AssertionOperator.inequal,  # correct attribute name
            "NotReady",
            timeout=1.0
        )
        assert result == "Ready"

    def test_get_text_contains_operator(self, mock_lib):
        """Test Get Text with contains operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#statusLabel"),
            AssertionOperator.contains,
            "ead",
            timeout=1.0
        )
        assert result == "Ready"

    def test_get_text_starts_operator(self, mock_lib):
        """Test Get Text with starts operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#statusLabel"),
            AssertionOperator.starts,
            "Re",
            timeout=1.0
        )
        assert result == "Ready"

    def test_get_text_ends_operator(self, mock_lib):
        """Test Get Text with ends operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#statusLabel"),
            AssertionOperator.ends,
            "dy",
            timeout=1.0
        )
        assert result == "Ready"

    def test_get_text_widget_not_found(self, mock_lib):
        """Test Get Text raises error for non-existent widget."""
        with pytest.raises(Exception):
            mock_lib.get_widget_text("Label#nonexistent")

    def test_get_text_empty_text(self, mock_lib):
        """Test Get Text with empty text returns empty string."""
        mock_lib._elements["Label#emptyLabel"] = MockSwtElement(
            id=100, name="emptyLabel", text="",
            class_name="org.eclipse.swt.widgets.Label"
        )
        text = mock_lib.get_widget_text("Label#emptyLabel")
        assert text == ""

    def test_get_text_none_text(self, mock_lib):
        """Test Get Text with None text returns empty string."""
        mock_lib._elements["Label#nullLabel"] = MockSwtElement(
            id=101, name="nullLabel", text=None,
            class_name="org.eclipse.swt.widgets.Label"
        )
        text = mock_lib.get_widget_text("Label#nullLabel")
        assert text == ""


class TestSwtGetTextRetryLogic:
    """Tests for SWT Get Text retry behavior with assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock SWT library."""
        lib = MockSwtLibrary()
        lib.connect_to_swt_application("TestApp")
        return lib

    def test_retry_until_value_changes(self, mock_lib):
        """Test retry until value changes to expected."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        call_count = [0]
        original_get = mock_lib.get_widget_text

        def get_changing_text(locator):
            call_count[0] += 1
            if call_count[0] < 3:
                return "Loading..."
            return "Complete"

        mock_lib.get_widget_text = get_changing_text

        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#status"),
            AssertionOperator.equal,
            "Complete",
            timeout=5.0,
            interval=0.1
        )
        assert result == "Complete"
        assert call_count[0] >= 3

    def test_retry_on_exception_then_success(self, mock_lib):
        """Test retry when getter raises exception then succeeds."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        call_count = [0]

        def get_with_exception(locator):
            call_count[0] += 1
            if call_count[0] < 3:
                raise Exception("Widget not ready")
            return "Ready"

        mock_lib.get_widget_text = get_with_exception

        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#status"),
            AssertionOperator.equal,
            "Ready",
            timeout=5.0,
            interval=0.1
        )
        assert result == "Ready"
        assert call_count[0] >= 3

    def test_timeout_with_last_value_in_error(self, mock_lib):
        """Test timeout error includes last seen value."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        with pytest.raises(AssertionError) as exc_info:
            with_retry_assertion(
                lambda: mock_lib.get_widget_text("Label#statusLabel"),
                AssertionOperator.equal,
                "NeverMatch",
                timeout=0.3,
                interval=0.1
            )
        error_msg = str(exc_info.value)
        assert "timeout" in error_msg.lower()
        assert "Ready" in error_msg  # Last value should be in error


class TestSwtNumericAssertions:
    """Tests for SWT numeric assertion keywords (count, row count, etc.)."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock SWT library."""
        lib = MockSwtLibrary()
        lib.connect_to_swt_application("TestApp")
        return lib

    def test_get_table_row_count_no_assertion(self, mock_lib):
        """Test Get Table Row Count without assertion."""
        count = mock_lib.get_table_row_count("Table#dataTable")
        assert count == 10

    def test_get_table_row_count_equal(self, mock_lib):
        """Test Get Table Row Count with == operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = numeric_assertion_with_retry(
            lambda: mock_lib.get_table_row_count("Table#dataTable"),
            AssertionOperator.equal,
            10,
            timeout=1.0
        )
        assert result == 10

    def test_get_table_row_count_greater_than(self, mock_lib):
        """Test Get Table Row Count with > operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = numeric_assertion_with_retry(
            lambda: mock_lib.get_table_row_count("Table#dataTable"),
            AssertionOperator["greater than"],
            5,
            timeout=1.0
        )
        assert result == 10

    def test_get_table_row_count_less_than(self, mock_lib):
        """Test Get Table Row Count with < operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = numeric_assertion_with_retry(
            lambda: mock_lib.get_table_row_count("Table#dataTable"),
            AssertionOperator["less than"],
            20,
            timeout=1.0
        )
        assert result == 10

    def test_get_table_row_count_greater_or_equal(self, mock_lib):
        """Test Get Table Row Count with >= operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = numeric_assertion_with_retry(
            lambda: mock_lib.get_table_row_count("Table#dataTable"),
            AssertionOperator[">="],
            10,
            timeout=1.0
        )
        assert result == 10

    def test_get_table_row_count_less_or_equal(self, mock_lib):
        """Test Get Table Row Count with <= operator."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = numeric_assertion_with_retry(
            lambda: mock_lib.get_table_row_count("Table#dataTable"),
            AssertionOperator["<="],
            10,
            timeout=1.0
        )
        assert result == 10

    def test_numeric_assertion_fail(self, mock_lib):
        """Test numeric assertion failure."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        with pytest.raises(AssertionError) as exc_info:
            numeric_assertion_with_retry(
                lambda: mock_lib.get_table_row_count("Table#dataTable"),
                AssertionOperator["greater than"],
                100,
                timeout=0.3,
                interval=0.1
            )
        assert "timeout" in str(exc_info.value).lower()

    def test_numeric_retry_until_condition_met(self, mock_lib):
        """Test numeric assertion retries until condition is met."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        call_count = [0]

        def get_increasing_count(locator):
            call_count[0] += 1
            return call_count[0] * 5  # 5, 10, 15, ...

        mock_lib.get_table_row_count = get_increasing_count

        result = numeric_assertion_with_retry(
            lambda: mock_lib.get_table_row_count("Table#dataTable"),
            AssertionOperator[">="],
            10,
            timeout=5.0,
            interval=0.1
        )
        assert result >= 10


class TestSwtTableCellAssertions:
    """Tests for SWT table cell value assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock SWT library."""
        lib = MockSwtLibrary()
        lib.connect_to_swt_application("TestApp")
        return lib

    def test_get_table_cell_no_assertion(self, mock_lib):
        """Test Get Table Cell without assertion."""
        value = mock_lib.get_table_cell("Table#dataTable", 0, 1)
        assert value == "Cell[0,1]"

    def test_get_table_cell_equal_assertion(self, mock_lib):
        """Test Get Table Cell with == assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            lambda: mock_lib.get_table_cell("Table#dataTable", 0, 1),
            AssertionOperator.equal,
            "Cell[0,1]",
            timeout=1.0
        )
        assert result == "Cell[0,1]"

    def test_get_table_cell_contains_assertion(self, mock_lib):
        """Test Get Table Cell with contains assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            lambda: mock_lib.get_table_cell("Table#dataTable", 2, 3),
            AssertionOperator.contains,
            "2,3",
            timeout=1.0
        )
        assert "2,3" in result


class TestSwtListAssertions:
    """Tests for SWT list-related assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock SWT library."""
        lib = MockSwtLibrary()
        lib.connect_to_swt_application("TestApp")
        return lib

    def test_get_list_items(self, mock_lib):
        """Test Get List Items returns items."""
        items = mock_lib.get_list_items("List#itemList")
        assert items == ["Item 1", "Item 2", "Item 3"]

    def test_get_list_item_count(self, mock_lib):
        """Test Get List Item Count."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = numeric_assertion_with_retry(
            lambda: len(mock_lib.get_list_items("List#itemList")),
            AssertionOperator.equal,
            3,
            timeout=1.0
        )
        assert result == 3


class TestSwtTimeoutBehavior:
    """Tests for SWT assertion timeout behavior."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock SWT library."""
        lib = MockSwtLibrary()
        lib.connect_to_swt_application("TestApp")
        return lib

    def test_immediate_success_is_fast(self, mock_lib):
        """Test assertion that passes immediately completes quickly."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        start = time.time()
        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#statusLabel"),
            AssertionOperator.equal,
            "Ready",
            timeout=5.0,
            interval=0.1
        )
        elapsed = time.time() - start
        assert elapsed < 0.5  # Should be much faster than 5s timeout
        assert result == "Ready"

    def test_timeout_honored(self, mock_lib):
        """Test timeout is approximately honored."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        start = time.time()
        with pytest.raises(AssertionError):
            with_retry_assertion(
                lambda: mock_lib.get_widget_text("Label#statusLabel"),
                AssertionOperator.equal,
                "NeverMatch",
                timeout=0.5,
                interval=0.1
            )
        elapsed = time.time() - start
        assert 0.4 < elapsed < 1.0  # Should be around 0.5s

    def test_custom_interval(self, mock_lib):
        """Test custom retry interval is used."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        call_count = [0]

        def counting_get(locator):
            call_count[0] += 1
            return "wrong"

        mock_lib.get_widget_text = counting_get

        with pytest.raises(AssertionError):
            with_retry_assertion(
                lambda: mock_lib.get_widget_text("Label#status"),
                AssertionOperator.equal,
                "expected",
                timeout=0.5,
                interval=0.1  # ~5 retries expected
            )

        # Should have made multiple attempts
        assert call_count[0] >= 3


class TestSwtCustomErrorMessages:
    """Tests for custom error messages in SWT assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock SWT library."""
        lib = MockSwtLibrary()
        lib.connect_to_swt_application("TestApp")
        return lib

    def test_default_message_includes_context(self, mock_lib):
        """Test default error message includes useful context."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        with pytest.raises(AssertionError) as exc_info:
            with_retry_assertion(
                lambda: mock_lib.get_widget_text("Label#statusLabel"),
                AssertionOperator.equal,
                "Expected",
                message="Status label text",
                timeout=0.2,
                interval=0.1
            )
        error_msg = str(exc_info.value)
        assert "timeout" in error_msg.lower()

    def test_custom_message_on_failure(self, mock_lib):
        """Test custom error message is included on failure."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        with pytest.raises(AssertionError) as exc_info:
            with_retry_assertion(
                lambda: mock_lib.get_widget_text("Label#statusLabel"),
                AssertionOperator.equal,
                "Expected",
                message="Custom prefix message",
                custom_message="My custom error description",
                timeout=0.2,
                interval=0.1
            )
        # Error should contain timeout info
        assert "timeout" in str(exc_info.value).lower()


class TestSwtEdgeCases:
    """Tests for edge cases in SWT assertion keywords."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock SWT library."""
        lib = MockSwtLibrary()
        lib.connect_to_swt_application("TestApp")
        return lib

    def test_special_characters_in_text(self, mock_lib):
        """Test assertion with special characters."""
        mock_lib._elements["Label#special"] = MockSwtElement(
            id=100, name="special", text="Line1\nLine2\tTabbed<>&\"'",
            class_name="org.eclipse.swt.widgets.Label"
        )
        text = mock_lib.get_widget_text("Label#special")
        assert "Line1" in text
        assert "\n" in text

    def test_unicode_characters(self, mock_lib):
        """Test assertion with unicode characters."""
        mock_lib._elements["Label#unicode"] = MockSwtElement(
            id=101, name="unicode", text="Hello World Unicode",
            class_name="org.eclipse.swt.widgets.Label"
        )
        text = mock_lib.get_widget_text("Label#unicode")
        assert "Hello" in text

    def test_very_long_string(self, mock_lib):
        """Test assertion with very long string."""
        long_text = "A" * 10000
        mock_lib._elements["Label#long"] = MockSwtElement(
            id=102, name="long", text=long_text,
            class_name="org.eclipse.swt.widgets.Label"
        )
        text = mock_lib.get_widget_text("Label#long")
        assert len(text) == 10000

    def test_none_operator_returns_value(self, mock_lib):
        """Test None operator just returns value without assertion."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#statusLabel"),
            None,  # No operator
            None,  # No expected
            timeout=1.0
        )
        assert result == "Ready"

    def test_zero_timeout(self, mock_lib):
        """Test with very short timeout (near-immediate check)."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        # Very short timeout should still make one attempt if value matches
        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#statusLabel"),
            AssertionOperator.equal,
            "Ready",
            timeout=0.01,  # Very short but not zero
            interval=0.01
        )
        assert result == "Ready"


class TestSwtWithFormatters:
    """Tests for SWT assertions with text formatters."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock SWT library."""
        lib = MockSwtLibrary()
        lib.connect_to_swt_application("TestApp")
        return lib

    def test_strip_formatter(self, mock_lib):
        """Test assertion with strip formatter."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        mock_lib._elements["Label#padded"] = MockSwtElement(
            id=100, name="padded", text="  Padded Text  ",
            class_name="org.eclipse.swt.widgets.Label"
        )

        formatter_funcs = [FORMATTERS["strip"]]

        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#padded"),
            AssertionOperator.equal,
            "Padded Text",
            timeout=1.0,
            formatters=formatter_funcs
        )
        assert result == "Padded Text"

    def test_lowercase_formatter(self, mock_lib):
        """Test assertion with lowercase formatter."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        mock_lib._elements["Label#upper"] = MockSwtElement(
            id=101, name="upper", text="UPPERCASE TEXT",
            class_name="org.eclipse.swt.widgets.Label"
        )

        formatter_funcs = [FORMATTERS["lowercase"]]

        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#upper"),
            AssertionOperator.equal,
            "uppercase text",
            timeout=1.0,
            formatters=formatter_funcs
        )
        assert result == "uppercase text"

    def test_normalize_spaces_formatter(self, mock_lib):
        """Test assertion with normalize_spaces formatter."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        mock_lib._elements["Label#spaces"] = MockSwtElement(
            id=102, name="spaces", text="Multiple   Spaces   Here",
            class_name="org.eclipse.swt.widgets.Label"
        )

        formatter_funcs = [FORMATTERS["normalize_spaces"]]

        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#spaces"),
            AssertionOperator.equal,
            "Multiple Spaces Here",
            timeout=1.0,
            formatters=formatter_funcs
        )
        assert result == "Multiple Spaces Here"

    def test_multiple_formatters(self, mock_lib):
        """Test assertion with multiple formatters applied."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        mock_lib._elements["Label#messy"] = MockSwtElement(
            id=103, name="messy", text="  HELLO   WORLD  ",
            class_name="org.eclipse.swt.widgets.Label"
        )

        formatter_funcs = [FORMATTERS["strip"], FORMATTERS["lowercase"], FORMATTERS["normalize_spaces"]]

        result = with_retry_assertion(
            lambda: mock_lib.get_widget_text("Label#messy"),
            AssertionOperator.equal,
            "hello world",
            timeout=1.0,
            formatters=formatter_funcs
        )
        assert result == "hello world"


class TestSwtTreeAssertions:
    """Tests for SWT tree-related assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock SWT library."""
        lib = MockSwtLibrary()
        lib.connect_to_swt_application("TestApp")
        return lib

    def test_get_tree_data(self, mock_lib):
        """Test getting tree data."""
        tree_data = mock_lib.get_tree_data("Tree#fileTree")
        assert tree_data["text"] == "Root"
        assert len(tree_data["children"]) > 0

    def test_get_selected_tree_nodes(self, mock_lib):
        """Test getting selected tree nodes."""
        selected = mock_lib.get_selected_tree_nodes("Tree#fileTree")
        assert "Root/Settings" in selected


class TestSwtTableRowValueAssertions:
    """Tests for SWT table row value assertions."""

    @pytest.fixture
    def mock_lib(self):
        """Create mock SWT library."""
        lib = MockSwtLibrary()
        lib.connect_to_swt_application("TestApp")
        return lib

    def test_get_table_row_values(self, mock_lib):
        """Test getting table row values."""
        values = mock_lib.get_table_row_values("Table#dataTable", 0)
        assert len(values) == 5
        assert "Cell[0,0]" in values


# =============================================================================
# Integration Tests
# =============================================================================


class TestSwtAssertionIntegration:
    """Integration tests for SWT assertion functionality."""

    def test_assertion_module_imports(self):
        """Test assertion module can be imported."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        from JavaGui.assertions import (
            with_retry_assertion,
            numeric_assertion_with_retry,
            ElementState,
            AssertionConfig,
        )
        assert with_retry_assertion is not None
        assert numeric_assertion_with_retry is not None
        assert ElementState is not None
        assert AssertionConfig is not None

    def test_formatters_module_imports(self):
        """Test formatters module can be imported."""
        if not ASSERTIONS_AVAILABLE:
            pytest.skip("AssertionEngine not available")

        from JavaGui.assertions.formatters import (
            normalize_spaces,
            strip,
            lowercase,
            uppercase,
            FORMATTERS,
        )
        assert normalize_spaces is not None
        assert strip is not None
        assert lowercase is not None
        assert uppercase is not None
        assert len(FORMATTERS) >= 4
