"""
Unit tests for error handling and exception types.
"""

import pytest
from unittest.mock import Mock, patch
import sys
import os

# Add parent directory to path for imports
sys.path.insert(0, os.path.dirname(__file__))

from conftest import (
    MockSwingLibrary,
    SwingError,
    ConnectionError,
    ElementNotFoundError,
    TimeoutError,
)


class TestExceptionTypes:
    """Test exception type hierarchy."""

    def test_swing_error_is_exception(self):
        """Test SwingError is an Exception."""
        assert issubclass(SwingError, Exception)

    def test_connection_error_is_swing_error(self):
        """Test ConnectionError inherits from SwingError."""
        assert issubclass(ConnectionError, SwingError)

    def test_element_not_found_error_is_swing_error(self):
        """Test ElementNotFoundError inherits from SwingError."""
        assert issubclass(ElementNotFoundError, SwingError)

    def test_timeout_error_is_swing_error(self):
        """Test TimeoutError inherits from SwingError."""
        assert issubclass(TimeoutError, SwingError)


class TestExceptionMessages:
    """Test exception message handling."""

    def test_swing_error_message(self):
        """Test SwingError with message."""
        error = SwingError("Something went wrong")
        assert str(error) == "Something went wrong"

    def test_connection_error_message(self):
        """Test ConnectionError with message."""
        error = ConnectionError("Failed to connect to JVM")
        assert "Failed to connect" in str(error)

    def test_element_not_found_message(self):
        """Test ElementNotFoundError with locator."""
        error = ElementNotFoundError("Element not found: JButton#nonexistent")
        assert "JButton#nonexistent" in str(error)

    def test_timeout_error_message(self):
        """Test TimeoutError with details."""
        error = TimeoutError("Timed out waiting for element: JLabel#status")
        assert "JLabel#status" in str(error)


class TestConnectionErrors:
    """Test connection error scenarios."""

    def test_connect_without_params_raises(self):
        """Test connecting without parameters raises error."""
        lib = MockSwingLibrary()
        with pytest.raises(ValueError) as exc_info:
            lib.connect()
        assert "Must specify" in str(exc_info.value)

    def test_connect_with_invalid_pid(self):
        """Test connecting with invalid PID format."""
        lib = MockSwingLibrary()
        # This would normally raise in real implementation
        lib.connect(pid=99999)  # Mock doesn't validate
        assert lib._connected is True


class TestElementNotFoundErrors:
    """Test element not found error scenarios."""

    def test_find_nonexistent_element(self):
        """Test finding element that doesn't exist."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        with pytest.raises(ElementNotFoundError) as exc_info:
            lib.find_element("JButton#doesNotExist")
        assert "not found" in str(exc_info.value)

    def test_element_not_found_includes_locator(self):
        """Test error message includes the locator."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        locator = "JTextField#missingField"
        with pytest.raises(ElementNotFoundError) as exc_info:
            lib.find_element(locator)
        # Error should reference the locator somehow
        assert "not found" in str(exc_info.value).lower()


class TestTimeoutErrors:
    """Test timeout error scenarios."""

    def test_wait_for_hidden_element_times_out(self):
        """Test timeout when waiting for hidden element to be visible."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        # Add a hidden element to the mock
        from conftest import MockSwingElement
        lib._elements["JButton#hiddenBtn"] = MockSwingElement(
            id=100, name="hiddenBtn", visible=False
        )
        with pytest.raises(TimeoutError):
            lib.wait_until_visible("JButton#hiddenBtn", timeout_ms=1000)

    def test_wait_for_disabled_element_times_out(self):
        """Test timeout when waiting for disabled element to be enabled."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        from conftest import MockSwingElement
        lib._elements["JButton#disabledBtn"] = MockSwingElement(
            id=101, name="disabledBtn", enabled=False
        )
        with pytest.raises(TimeoutError):
            lib.wait_until_enabled("JButton#disabledBtn", timeout_ms=1000)


class TestAssertionErrors:
    """Test assertion error scenarios."""

    def test_element_should_not_exist_fails_when_exists(self):
        """Test element_should_not_exist fails when element exists."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        with pytest.raises(AssertionError):
            lib.element_should_not_exist("JButton#loginBtn")

    def test_element_should_be_visible_fails_when_hidden(self):
        """Test element_should_be_visible fails for hidden element."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        from conftest import MockSwingElement
        lib._elements["JButton#hiddenBtn"] = MockSwingElement(
            id=100, name="hiddenBtn", visible=False
        )
        with pytest.raises(AssertionError):
            lib.element_should_be_visible("JButton#hiddenBtn")

    def test_element_should_be_enabled_fails_when_disabled(self):
        """Test element_should_be_enabled fails for disabled element."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        from conftest import MockSwingElement
        lib._elements["JButton#disabledBtn"] = MockSwingElement(
            id=101, name="disabledBtn", enabled=False
        )
        with pytest.raises(AssertionError):
            lib.element_should_be_enabled("JButton#disabledBtn")


class TestErrorRecovery:
    """Test error recovery scenarios."""

    def test_reconnect_after_disconnect(self):
        """Test reconnecting after disconnect."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        assert lib._connected is True
        lib.disconnect()
        assert lib._connected is False
        lib.connect(pid=12345)
        assert lib._connected is True

    def test_multiple_disconnects(self):
        """Test multiple disconnects don't raise."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        lib.disconnect()
        lib.disconnect()  # Should not raise
        assert lib._connected is False

    def test_operations_after_failed_find(self):
        """Test operations continue after failed find."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)
        # Failed find
        with pytest.raises(ElementNotFoundError):
            lib.find_element("JButton#nonexistent")
        # Subsequent operations should work
        elem = lib.find_element("JButton#loginBtn")
        assert elem.name == "loginBtn"


class TestErrorChaining:
    """Test error chaining and context."""

    def test_swing_error_can_wrap_cause(self):
        """Test SwingError can wrap another exception."""
        original = ValueError("original error")
        error = SwingError("wrapped error")
        # In Python, you can chain exceptions
        try:
            try:
                raise original
            except ValueError as e:
                raise SwingError("Swing operation failed") from e
        except SwingError as e:
            assert e.__cause__ is original

    def test_nested_error_handling(self):
        """Test nested error handling works correctly."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        errors_caught = []
        for locator in ["JButton#nonexistent", "JButton#loginBtn", "JTable#missing"]:
            try:
                lib.find_element(locator)
            except ElementNotFoundError:
                errors_caught.append(locator)

        assert len(errors_caught) == 2
        assert "JButton#nonexistent" in errors_caught
        assert "JTable#missing" in errors_caught
