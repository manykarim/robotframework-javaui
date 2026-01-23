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

    def test_format_parameter_passed_correctly(self):
        """Verify format parameter is passed correctly."""
        from JavaGui import SwingLibrary

        # Create mock Rust core
        mock_lib = Mock()
        mock_lib.get_component_tree = Mock(return_value="test tree")

        # Create library instance and inject mock
        lib = SwingLibrary()
        lib._lib = mock_lib

        # Call get_component_tree with format
        result = lib.get_component_tree(format="json")

        # Verify correct parameter passing with named args
        mock_lib.get_component_tree.assert_called_once_with(
            locator=None,
            format="json",
            max_depth=None,
            types=None,
            exclude_types=None,
            visible_only=False,
            enabled_only=False,
            focusable_only=False
        )
        assert result == "test tree"

    def test_max_depth_parameter_passed_correctly(self):
        """Verify max_depth parameter is passed correctly."""
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_component_tree = Mock(return_value="test tree")

        lib = SwingLibrary()
        lib._lib = mock_lib

        # Call with max_depth
        result = lib.get_component_tree(max_depth=5)

        # Verify with named args
        mock_lib.get_component_tree.assert_called_once_with(
            locator=None,
            format="text",
            max_depth=5,
            types=None,
            exclude_types=None,
            visible_only=False,
            enabled_only=False,
            focusable_only=False
        )

    def test_all_parameters_passed_correctly(self):
        """Verify all parameters passed correctly."""
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_component_tree = Mock(return_value="test tree")

        lib = SwingLibrary()
        lib._lib = mock_lib

        # Call with all parameters
        result = lib.get_component_tree(format="xml", max_depth=10)

        # Verify with named args
        mock_lib.get_component_tree.assert_called_once_with(
            locator=None,
            format="xml",
            max_depth=10,
            types=None,
            exclude_types=None,
            visible_only=False,
            enabled_only=False,
            focusable_only=False
        )

    def test_locator_shows_deprecation_warning(self):
        """Verify locator parameter shows deprecation warning."""
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_component_tree = Mock(return_value="test tree")

        lib = SwingLibrary()
        lib._lib = mock_lib

        # Call with locator should trigger warning
        with pytest.warns(DeprecationWarning, match="locator.*not yet supported"):
            result = lib.get_component_tree(locator="JPanel#main")

        # Verify locator was passed (backend ignores it)
        mock_lib.get_component_tree.assert_called_once_with(
            locator="JPanel#main",
            format="text",
            max_depth=None,
            types=None,
            exclude_types=None,
            visible_only=False,
            enabled_only=False,
            focusable_only=False
        )


class TestSaveUITreeFix:
    """Verify save_ui_tree bug fix."""

    def test_format_parameter_supported(self):
        """Verify format parameter is supported in save_ui_tree."""
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

            # Verify get_ui_tree called with json format
            mock_lib.get_ui_tree.assert_called_once_with("json", None, False)

            # Verify file written
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()
            assert content == '{"type": "JFrame"}'
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_max_depth_parameter_supported(self):
        """Verify max_depth parameter is supported in save_ui_tree."""
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value="limited tree")

        lib = SwingLibrary()
        lib._lib = mock_lib

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            # Save with max_depth
            lib.save_ui_tree(temp_file, max_depth=3)

            # Verify get_ui_tree called with max_depth
            mock_lib.get_ui_tree.assert_called_once_with("text", 3, False)
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_all_parameters_supported(self):
        """Verify all parameters work together."""
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

            # Verify all parameters passed
            mock_lib.get_ui_tree.assert_called_once_with("xml", 5, False)

            # Verify file content
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()
            assert content == '<component type="JFrame"/>'
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_utf8_encoding(self):
        """Verify UTF-8 encoding in file output."""
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

            # Verify UTF-8 encoding
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()
            assert content == "JFrame テスト 树 🌳"
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)


class TestBugRegressionVerification:
    """Regression tests proving the bugs are fixed."""

    def test_bug_fix_format_not_replaced_by_locator(self):
        """
        REGRESSION TEST: Prove format parameter is correctly passed.

        Now uses named parameters to the Rust backend.
        """
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_component_tree = Mock(return_value="tree")

        lib = SwingLibrary()
        lib._lib = mock_lib

        # Call with format="json"
        lib.get_component_tree(format="json")

        # Verify format parameter is correctly passed with named args
        call_kwargs = mock_lib.get_component_tree.call_args[1]
        assert call_kwargs['format'] == "json", "BUG: format parameter not passed correctly!"
        assert call_kwargs['max_depth'] is None, "max_depth should be None"
        assert call_kwargs['locator'] is None, "locator should be None"
        assert call_kwargs['visible_only'] is False, "visible_only should be False"

    def test_bug_fix_save_supports_format(self):
        """
        REGRESSION TEST: Prove save_ui_tree supports format parameter.

        Old buggy behavior: format parameter not supported.
        New correct behavior: format parameter fully supported.
        """
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value='{"tree": "json"}')

        lib = SwingLibrary()
        lib._lib = mock_lib

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.json') as f:
            temp_file = f.name

        try:
            # This should work (previously didn't support format)
            lib.save_ui_tree(temp_file, format="json")

            # Verify format was passed
            args = mock_lib.get_ui_tree.call_args[0]
            assert args[0] == "json", "BUG: format parameter not supported!"
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_bug_fix_save_supports_max_depth(self):
        """
        REGRESSION TEST: Prove save_ui_tree supports max_depth parameter.

        Old buggy behavior: max_depth parameter not supported.
        New correct behavior: max_depth parameter fully supported.
        """
        from JavaGui import SwingLibrary

        mock_lib = Mock()
        mock_lib.get_ui_tree = Mock(return_value="limited tree")

        lib = SwingLibrary()
        lib._lib = mock_lib

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            # This should work (previously didn't support max_depth)
            lib.save_ui_tree(temp_file, max_depth=5)

            # Verify max_depth was passed
            args = mock_lib.get_ui_tree.call_args[0]
            assert args[1] == 5, "BUG: max_depth parameter not supported!"
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)
