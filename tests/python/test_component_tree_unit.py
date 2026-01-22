"""
Unit tests for component tree bug fixes.

Tests the bug fixes for:
1. get_component_tree - Fixed parameter passing bug (was passing locator as format)
2. save_ui_tree - Added format and max_depth parameters with proper file I/O

Direct unit tests without full library initialization.
"""

import pytest
import os
import sys
import tempfile
from unittest.mock import Mock, MagicMock, patch
import warnings

# Add python package to path
sys.path.insert(0, '/mnt/c/workspace/robotframework-swing/python')


class TestGetComponentTreeParameterPassing:
    """Test that get_component_tree passes parameters correctly to Rust backend."""

    def test_passes_format_parameter_correctly(self):
        """Test that format parameter is passed to get_ui_tree, not locator."""
        from JavaGui import SwingLibrary

        # Create mock Rust core
        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value="JFrame test tree")

        # Create library instance and inject mock
        lib = SwingLibrary()
        lib._lib = mock_lib

        # Call with format parameter
        result = lib.get_component_tree(format="json")

        # Verify get_ui_tree was called with correct parameters
        # Should be: get_ui_tree(format, max_depth, visible_only)
        mock_lib.get_ui_tree.assert_called_once_with("json", None, False)
        assert result == "JFrame test tree"

    def test_passes_max_depth_parameter_correctly(self):
        """Test that max_depth parameter is passed correctly."""
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value="JFrame test tree")

        lib = SwingLibrary()
        lib._lib = mock_lib

        # Call with max_depth parameter
        result = lib.get_component_tree(max_depth=5)

        # Should pass max_depth as second parameter
        mock_lib.get_ui_tree.assert_called_once_with("text", 5, False)

    def test_passes_all_parameters_correctly(self):
        """Test that all parameters are passed in correct order."""
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value="JFrame test tree")

        lib = SwingLibrary()
        lib._lib = mock_lib

        # Call with all parameters
        result = lib.get_component_tree(format="xml", max_depth=10)

        # Should pass all parameters in correct order
        mock_lib.get_ui_tree.assert_called_once_with("xml", 10, False)

    def test_locator_parameter_deprecated(self):
        """Test that locator parameter shows deprecation warning."""
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value="JFrame test tree")

        lib = SwingLibrary()
        lib._lib = mock_lib

        # Call with locator parameter should trigger warning
        with pytest.warns(DeprecationWarning, match="locator.*not yet supported"):
            result = lib.get_component_tree(locator="JPanel#main")

        # Should still call get_ui_tree with default parameters
        mock_lib.get_ui_tree.assert_called_once()


class TestSaveUITreeParameterPassing:
    """Test that save_ui_tree uses parameters correctly."""

    def test_saves_text_format_by_default(self):
        """Test that save_ui_tree saves text format by default."""
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value="JFrame test tree\n  JPanel content")

        lib = SwingLibrary()
        lib._lib = mock_lib

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            # Save with default parameters
            lib.save_ui_tree(temp_file)

            # Should call get_ui_tree with text format
            mock_lib.get_ui_tree.assert_called_once_with("text", None, False)

            # Verify file was written
            assert os.path.exists(temp_file)
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()
            assert content == "JFrame test tree\n  JPanel content"
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_saves_json_format(self):
        """Test that save_ui_tree can save JSON format."""
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value='{"type": "JFrame"}')

        lib = SwingLibrary()
        lib._lib = mock_lib

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.json') as f:
            temp_file = f.name

        try:
            # Save with JSON format
            lib.save_ui_tree(temp_file, format="json")

            # Should call get_ui_tree with json format
            mock_lib.get_ui_tree.assert_called_once_with("json", None, False)

            # Verify file content
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()
            assert content == '{"type": "JFrame"}'
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_saves_with_max_depth(self):
        """Test that save_ui_tree respects max_depth parameter."""
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value="JFrame limited depth")

        lib = SwingLibrary()
        lib._lib = mock_lib

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            # Save with max_depth
            lib.save_ui_tree(temp_file, max_depth=3)

            # Should pass max_depth to get_ui_tree
            mock_lib.get_ui_tree.assert_called_once_with("text", 3, False)

            # Verify file was written
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()
            assert content == "JFrame limited depth"
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_saves_with_all_parameters(self):
        """Test that save_ui_tree handles all parameters."""
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value='<component type="JFrame"/>')

        lib = SwingLibrary()
        lib._lib = mock_lib

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.xml') as f:
            temp_file = f.name

        try:
            # Save with all parameters
            lib.save_ui_tree(temp_file, format="xml", max_depth=5)

            # Should pass all parameters correctly
            mock_lib.get_ui_tree.assert_called_once_with("xml", 5, False)

            # Verify file content
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()
            assert content == '<component type="JFrame"/>'
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_locator_parameter_deprecated_in_save(self):
        """Test that locator parameter shows deprecation warning in save_ui_tree."""
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value="JFrame test tree")

        lib = SwingLibrary()
        lib._lib = mock_lib

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            # Call with locator parameter should trigger warning
            with pytest.warns(DeprecationWarning, match="locator.*not yet supported"):
                lib.save_ui_tree(temp_file, locator="JPanel#main")

            # Should still save the file
            assert os.path.exists(temp_file)
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_utf8_encoding(self):
        """Test that save_ui_tree uses UTF-8 encoding."""
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        # Include Unicode characters
        mock_lib.get_ui_tree = Mock(return_value="JFrame テスト 树 🌳")

        lib = SwingLibrary()
        lib._lib = mock_lib

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            lib.save_ui_tree(temp_file)

            # Verify file can be read with UTF-8
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()
            assert content == "JFrame テスト 树 🌳"
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)


class TestBugRegression:
    """Regression tests for the specific bugs that were fixed."""

    def test_bug_get_component_tree_locator_passed_as_format(self):
        """
        REGRESSION TEST: Bug where locator was passed as first parameter to get_ui_tree.

        Old buggy code:
            tree_str = self._lib.get_ui_tree(locator)

        This would pass locator (e.g., "JPanel#main") as the format parameter,
        causing incorrect behavior.
        """
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value="tree")

        lib = SwingLibrary()
        lib._lib = mock_lib

        # Call with format="json" - old bug would pass locator instead
        lib.get_component_tree(format="json")

        # NEW CORRECT BEHAVIOR: Should pass "json" as format parameter
        # OLD BUGGY BEHAVIOR: Would have passed None (locator default) as format
        args = mock_lib.get_ui_tree.call_args[0]
        assert args[0] == "json", "BUG: format parameter not passed correctly"
        assert args[1] is None, "max_depth should be None"
        assert args[2] is False, "visible_only should be False"

    def test_bug_save_ui_tree_missing_format_parameter(self):
        """
        REGRESSION TEST: Bug where save_ui_tree didn't support format parameter.

        Old signature:
            def save_ui_tree(self, filename: str, locator: Optional[str] = None)

        This meant you couldn't specify format for saved tree files.
        """
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value='{"tree": "json"}')

        lib = SwingLibrary()
        lib._lib = mock_lib

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.json') as f:
            temp_file = f.name

        try:
            # NEW CORRECT BEHAVIOR: Can specify format parameter
            lib.save_ui_tree(temp_file, format="json")

            # Should call get_ui_tree with json format
            args = mock_lib.get_ui_tree.call_args[0]
            assert args[0] == "json", "BUG: format parameter not supported"

            # File should be saved with JSON content
            with open(temp_file, 'r') as f:
                assert f.read() == '{"tree": "json"}'
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_bug_save_ui_tree_missing_max_depth_parameter(self):
        """
        REGRESSION TEST: Bug where save_ui_tree didn't support max_depth parameter.
        """
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value="limited tree")

        lib = SwingLibrary()
        lib._lib = mock_lib

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            # NEW CORRECT BEHAVIOR: Can specify max_depth parameter
            lib.save_ui_tree(temp_file, max_depth=5)

            # Should pass max_depth to get_ui_tree
            args = mock_lib.get_ui_tree.call_args[0]
            assert args[1] == 5, "BUG: max_depth parameter not supported"
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)
