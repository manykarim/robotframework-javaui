"""
Unit tests for empty locator validation in SWT and Swing libraries.

Tests validate that empty locators are properly rejected before RPC calls,
preventing fatal crashes and providing clear error messages.
"""

import pytest
from JavaGui import SwtLibrary, SwingLibrary, RcpLibrary


class TestSwtEmptyLocatorValidation:
    """Test SWT library empty locator validation."""

    @pytest.fixture
    def swt_lib(self):
        """Create SWT library instance."""
        lib = SwtLibrary()
        # Don't start app for unit tests - we're testing validation only
        return lib

    def test_activate_shell_rejects_empty_locator(self, swt_lib):
        """Activate shell should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swt_lib.activate_shell("")

    def test_activate_shell_rejects_whitespace_locator(self, swt_lib):
        """Activate shell should reject whitespace-only locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swt_lib.activate_shell("   ")

    def test_close_shell_rejects_empty_locator(self, swt_lib):
        """Close shell should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swt_lib.close_shell("")

    def test_find_widget_rejects_empty_locator(self, swt_lib):
        """Find widget should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swt_lib.find_widget("")

    def test_find_widgets_rejects_empty_locator(self, swt_lib):
        """Find widgets should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swt_lib.find_widgets("")

    def test_click_widget_rejects_empty_locator(self, swt_lib):
        """Click widget should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swt_lib.click_widget("")

    def test_double_click_widget_rejects_empty_locator(self, swt_lib):
        """Double click widget should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swt_lib.double_click_widget("")

    def test_input_text_rejects_empty_locator(self, swt_lib):
        """Input text should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swt_lib.input_text("", "some text")

    def test_clear_text_rejects_empty_locator(self, swt_lib):
        """Clear text should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swt_lib.clear_text("")

    def test_select_combo_item_rejects_empty_locator(self, swt_lib):
        """Select combo item should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swt_lib.select_combo_item("", "item")

    def test_check_button_rejects_empty_locator(self, swt_lib):
        """Check button should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swt_lib.check_button("")

    def test_uncheck_button_rejects_empty_locator(self, swt_lib):
        """Uncheck button should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swt_lib.uncheck_button("")

    def test_get_widget_text_rejects_empty_locator(self, swt_lib):
        """Get widget text should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swt_lib.get_widget_text("")

    def test_error_message_clarity(self, swt_lib):
        """Error message should be clear and specific."""
        try:
            swt_lib.activate_shell("")
            pytest.fail("Should have raised exception")
        except Exception as e:
            error_msg = str(e).lower()
            # Error should be about locator, not RPC timeout
            assert "locator" in error_msg or "empty" in error_msg
            assert "timeout" not in error_msg
            assert "connection" not in error_msg


class TestSwingEmptyLocatorValidation:
    """Test Swing library empty locator validation."""

    @pytest.fixture
    def swing_lib(self):
        """Create Swing library instance."""
        lib = SwingLibrary()
        # Don't start app for unit tests - we're testing validation only
        return lib

    def test_click_element_rejects_empty_locator(self, swing_lib):
        """Click element should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swing_lib.click_element("")

    def test_click_element_rejects_whitespace_locator(self, swing_lib):
        """Click element should reject whitespace-only locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swing_lib.click_element("   ")

    def test_input_text_rejects_empty_locator(self, swing_lib):
        """Input text should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swing_lib.input_text("", "some text")

    def test_select_from_list_rejects_empty_locator(self, swing_lib):
        """Select from list should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swing_lib.select_from_list("", "item")

    def test_click_button_rejects_empty_locator(self, swing_lib):
        """Click button should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swing_lib.click_button("")

    def test_find_element_rejects_empty_locator(self, swing_lib):
        """Find element should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swing_lib.find_element("")

    def test_find_elements_rejects_empty_locator(self, swing_lib):
        """Find elements should reject empty locator."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            swing_lib.find_elements("")


class TestRcpEmptyLocatorValidation:
    """Test RCP library empty locator validation (delegates to SWT)."""

    @pytest.fixture
    def rcp_lib(self):
        """Create RCP library instance."""
        lib = RcpLibrary()
        # Don't start app for unit tests - we're testing validation only
        return lib

    def test_find_widget_rejects_empty_locator(self, rcp_lib):
        """RCP find widget should reject empty locator (via SWT delegation)."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            rcp_lib.find_widget("")

    def test_click_widget_rejects_empty_locator(self, rcp_lib):
        """RCP click widget should reject empty locator (via SWT delegation)."""
        with pytest.raises(Exception, match="[Ll]ocator cannot be empty"):
            rcp_lib.click_widget("")


class TestValidationPerformance:
    """Test that validation has minimal performance impact."""

    @pytest.fixture
    def swt_lib(self):
        """Create SWT library instance."""
        return SwtLibrary()

    def test_validation_is_fast(self, swt_lib):
        """Empty locator validation should be very fast (<1ms)."""
        import time

        iterations = 1000
        start = time.perf_counter()

        for _ in range(iterations):
            try:
                swt_lib.find_widget("")
            except Exception:
                pass  # Expected

        elapsed = time.perf_counter() - start
        avg_time_ms = (elapsed / iterations) * 1000

        # Validation should be <1ms per call (actually <0.01ms in practice)
        assert avg_time_ms < 1.0, f"Validation too slow: {avg_time_ms:.3f}ms per call"

    def test_validation_faster_than_rpc_timeout(self, swt_lib):
        """Validation should be much faster than RPC timeout would be."""
        import time

        # Test validation time
        start = time.perf_counter()
        try:
            swt_lib.find_widget("")
        except Exception:
            pass
        validation_time = time.perf_counter() - start

        # Validation should be <0.01s (RPC timeout would be 30+ seconds)
        assert validation_time < 0.01, f"Validation too slow: {validation_time:.3f}s"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
