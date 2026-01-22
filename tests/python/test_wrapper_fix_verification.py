"""
Verification tests for Phase 1 bug fixes.

These tests verify that the Python wrapper correctly passes parameters
to the Rust backend without requiring a live Java application connection.
"""

import pytest
import sys
import os
import tempfile
from unittest.mock import Mock, patch, MagicMock

# Add python package to path
sys.path.insert(0, '/mnt/c/workspace/robotframework-swing/python')


class TestGetComponentTreeFix:
    """Verify get_component_tree bug fix."""

    @patch('JavaGui._core.SwingLibrary')
    def test_format_parameter_passed_correctly(self, mock_swing_lib_class):
        """Verify format parameter is passed, not locator."""
        # Setup mock instance
        mock_instance = Mock()
        mock_instance.get_ui_tree = Mock(return_value="test tree")
        mock_swing_lib_class.return_value = mock_instance

        # Import and create library
        from JavaGui import SwingLibrary
        lib = SwingLibrary()

        # Call get_component_tree with format
        result = lib.get_component_tree(format="json")

        # Verify correct parameter passing
        mock_instance.get_ui_tree.assert_called_once_with("json", None, False)
        assert result == "test tree"

    @patch('JavaGui._core.SwingLibrary')
    def test_max_depth_parameter_passed_correctly(self, mock_swing_lib_class):
        """Verify max_depth parameter is passed correctly."""
        mock_instance = Mock()
        mock_instance.get_ui_tree = Mock(return_value="test tree")
        mock_swing_lib_class.return_value = mock_instance

        from JavaGui import SwingLibrary
        lib = SwingLibrary()

        # Call with max_depth
        result = lib.get_component_tree(max_depth=5)

        # Verify: get_ui_tree(format="text", max_depth=5, visible_only=False)
        mock_instance.get_ui_tree.assert_called_once_with("text", 5, False)

    @patch('JavaGui._core.SwingLibrary')
    def test_all_parameters_passed_correctly(self, mock_swing_lib_class):
        """Verify all parameters passed in correct order."""
        mock_instance = Mock()
        mock_instance.get_ui_tree = Mock(return_value="test tree")
        mock_swing_lib_class.return_value = mock_instance

        from JavaGui import SwingLibrary
        lib = SwingLibrary()

        # Call with all parameters
        result = lib.get_component_tree(format="xml", max_depth=10)

        # Verify correct order and values
        mock_instance.get_ui_tree.assert_called_once_with("xml", 10, False)

    @patch('JavaGui._core.SwingLibrary')
    def test_locator_shows_deprecation_warning(self, mock_swing_lib_class):
        """Verify locator parameter shows deprecation warning."""
        mock_instance = Mock()
        mock_instance.get_ui_tree = Mock(return_value="test tree")
        mock_swing_lib_class.return_value = mock_instance

        from JavaGui import SwingLibrary
        lib = SwingLibrary()

        # Call with locator should trigger warning
        with pytest.warns(DeprecationWarning, match="locator.*not yet supported"):
            result = lib.get_component_tree(locator="JPanel#main")


class TestSaveUITreeFix:
    """Verify save_ui_tree bug fix."""

    @patch('JavaGui._core.SwingLibrary')
    def test_format_parameter_supported(self, mock_swing_lib_class):
        """Verify format parameter is supported in save_ui_tree."""
        mock_instance = Mock()
        mock_instance.get_ui_tree = Mock(return_value='{"type": "JFrame"}')
        mock_swing_lib_class.return_value = mock_instance

        from JavaGui import SwingLibrary
        lib = SwingLibrary()

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.json') as f:
            temp_file = f.name

        try:
            # Save with JSON format
            lib.save_ui_tree(temp_file, format="json")

            # Verify get_ui_tree called with json format
            mock_instance.get_ui_tree.assert_called_once_with("json", None, False)

            # Verify file written
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()
            assert content == '{"type": "JFrame"}'
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    @patch('JavaGui._core.SwingLibrary')
    def test_max_depth_parameter_supported(self, mock_swing_lib_class):
        """Verify max_depth parameter is supported in save_ui_tree."""
        mock_instance = Mock()
        mock_instance.get_ui_tree = Mock(return_value="limited tree")
        mock_swing_lib_class.return_value = mock_instance

        from JavaGui import SwingLibrary
        lib = SwingLibrary()

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            # Save with max_depth
            lib.save_ui_tree(temp_file, max_depth=3)

            # Verify get_ui_tree called with max_depth
            mock_instance.get_ui_tree.assert_called_once_with("text", 3, False)
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    @patch('JavaGui._core.SwingLibrary')
    def test_all_parameters_supported(self, mock_swing_lib_class):
        """Verify all parameters work together."""
        mock_instance = Mock()
        mock_instance.get_ui_tree = Mock(return_value='<component type="JFrame"/>')
        mock_swing_lib_class.return_value = mock_instance

        from JavaGui import SwingLibrary
        lib = SwingLibrary()

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.xml') as f:
            temp_file = f.name

        try:
            # Save with all parameters
            lib.save_ui_tree(temp_file, format="xml", max_depth=5)

            # Verify all parameters passed
            mock_instance.get_ui_tree.assert_called_once_with("xml", 5, False)

            # Verify file content
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()
            assert content == '<component type="JFrame"/>'
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    @patch('JavaGui._core.SwingLibrary')
    def test_utf8_encoding(self, mock_swing_lib_class):
        """Verify UTF-8 encoding in file output."""
        mock_instance = Mock()
        # Include Unicode characters
        mock_instance.get_ui_tree = Mock(return_value="JFrame テスト 树 🌳")
        mock_swing_lib_class.return_value = mock_instance

        from JavaGui import SwingLibrary
        lib = SwingLibrary()

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            lib.save_ui_tree(temp_file)

            # Verify UTF-8 encoding
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()
            assert content == "JFrame テスト 树 🌳"
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)


class TestBugRegressionVerification:
    """Regression tests proving the bugs are fixed."""

    @patch('JavaGui._core.SwingLibrary')
    def test_bug_fix_format_not_replaced_by_locator(self, mock_swing_lib_class):
        """
        REGRESSION TEST: Prove format parameter is NOT replaced by locator.

        Old buggy behavior would have passed locator as first argument.
        New correct behavior passes format as first argument.
        """
        mock_instance = Mock()
        mock_instance.get_ui_tree = Mock(return_value="tree")
        mock_swing_lib_class.return_value = mock_instance

        from JavaGui import SwingLibrary
        lib = SwingLibrary()

        # Call with format="json"
        lib.get_component_tree(format="json")

        # Verify first argument is "json", NOT None (the locator default)
        args = mock_instance.get_ui_tree.call_args[0]
        assert args[0] == "json", "BUG: format parameter replaced by locator!"
        assert args[1] is None, "max_depth should be None"
        assert args[2] is False, "visible_only should be False"

    @patch('JavaGui._core.SwingLibrary')
    def test_bug_fix_save_supports_format(self, mock_swing_lib_class):
        """
        REGRESSION TEST: Prove save_ui_tree supports format parameter.

        Old buggy behavior: format parameter not supported.
        New correct behavior: format parameter fully supported.
        """
        mock_instance = Mock()
        mock_instance.get_ui_tree = Mock(return_value='{"tree": "json"}')
        mock_swing_lib_class.return_value = mock_instance

        from JavaGui import SwingLibrary
        lib = SwingLibrary()

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.json') as f:
            temp_file = f.name

        try:
            # This should work (previously didn't support format)
            lib.save_ui_tree(temp_file, format="json")

            # Verify format was passed
            args = mock_instance.get_ui_tree.call_args[0]
            assert args[0] == "json", "BUG: format parameter not supported!"
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    @patch('JavaGui._core.SwingLibrary')
    def test_bug_fix_save_supports_max_depth(self, mock_swing_lib_class):
        """
        REGRESSION TEST: Prove save_ui_tree supports max_depth parameter.

        Old buggy behavior: max_depth parameter not supported.
        New correct behavior: max_depth parameter fully supported.
        """
        mock_instance = Mock()
        mock_instance.get_ui_tree = Mock(return_value="limited tree")
        mock_swing_lib_class.return_value = mock_instance

        from JavaGui import SwingLibrary
        lib = SwingLibrary()

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            # This should work (previously didn't support max_depth)
            lib.save_ui_tree(temp_file, max_depth=5)

            # Verify max_depth was passed
            args = mock_instance.get_ui_tree.call_args[0]
            assert args[1] == 5, "BUG: max_depth parameter not supported!"
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)
