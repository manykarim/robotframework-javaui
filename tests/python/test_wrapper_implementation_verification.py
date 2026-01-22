"""
Verification tests for Python wrapper implementation.

These tests verify the Python wrapper methods exist, have correct signatures,
and properly pass parameters to the Rust backend.
"""

import pytest
import sys
import os
import tempfile
import inspect
from unittest.mock import Mock, MagicMock, patch

# Add python package to path
sys.path.insert(0, '/mnt/c/workspace/robotframework-swing/python')


class TestPythonWrapperSignatures:
    """Verify that Python wrapper methods have correct signatures."""

    def test_get_component_tree_signature(self):
        """Verify get_component_tree has all required parameters."""
        from JavaGui import SwingLibrary

        sig = inspect.signature(SwingLibrary.get_component_tree)
        params = sig.parameters

        # Verify all expected parameters exist
        assert 'self' in params
        assert 'locator' in params
        assert 'format' in params
        assert 'max_depth' in params
        assert 'types' in params
        assert 'exclude_types' in params
        assert 'visible_only' in params
        assert 'enabled_only' in params
        assert 'focusable_only' in params

        # Verify default values
        assert params['locator'].default is None
        assert params['format'].default == 'text'
        assert params['max_depth'].default is None
        assert params['types'].default is None
        assert params['exclude_types'].default is None
        assert params['visible_only'].default is False
        assert params['enabled_only'].default is False
        assert params['focusable_only'].default is False

    def test_save_ui_tree_signature(self):
        """Verify save_ui_tree has all required parameters."""
        from JavaGui import SwingLibrary

        sig = inspect.signature(SwingLibrary.save_ui_tree)
        params = sig.parameters

        # Verify all expected parameters exist
        assert 'self' in params
        assert 'filename' in params
        assert 'locator' in params
        assert 'format' in params
        assert 'max_depth' in params

        # Verify default values
        assert params['locator'].default is None
        assert params['format'].default == 'text'
        assert params['max_depth'].default is None

    def test_get_component_tree_has_docstring(self):
        """Verify get_component_tree has comprehensive documentation."""
        from JavaGui import SwingLibrary

        doc = SwingLibrary.get_component_tree.__doc__
        assert doc is not None
        assert len(doc) > 100  # Should have substantial documentation

        # Verify key documentation elements
        assert 'locator' in doc.lower()
        assert 'format' in doc.lower()
        assert 'max_depth' in doc.lower()
        assert 'types' in doc.lower() or 'type' in doc.lower()

    def test_save_ui_tree_has_docstring(self):
        """Verify save_ui_tree has comprehensive documentation."""
        from JavaGui import SwingLibrary

        doc = SwingLibrary.save_ui_tree.__doc__
        assert doc is not None
        assert len(doc) > 50  # Should have documentation

        # Verify key documentation elements
        assert 'filename' in doc.lower() or 'file' in doc.lower()
        assert 'format' in doc.lower()


class TestParameterValidation:
    """Test that Python wrapper validates parameters correctly."""

    def test_get_component_tree_validates_max_depth_type(self):
        """Verify max_depth parameter type validation."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()

        # Mock the internal library to avoid connection requirement
        lib._lib = Mock()
        lib._lib.get_component_tree = Mock(return_value="mock tree")

        # Should raise TypeError for non-integer max_depth
        with pytest.raises(TypeError, match="max_depth must be an integer"):
            lib.get_component_tree(max_depth="5")

        with pytest.raises(TypeError, match="max_depth must be an integer"):
            lib.get_component_tree(max_depth=5.5)

    def test_get_component_tree_validates_max_depth_value(self):
        """Verify max_depth parameter value validation."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib._lib = Mock()
        lib._lib.get_component_tree = Mock(return_value="mock tree")

        # Should raise ValueError for negative max_depth
        with pytest.raises(ValueError, match="max_depth must be >= 0"):
            lib.get_component_tree(max_depth=-1)

        with pytest.raises(ValueError, match="max_depth must be >= 0"):
            lib.get_component_tree(max_depth=-10)

    def test_get_component_tree_accepts_valid_max_depth(self):
        """Verify valid max_depth values are accepted."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib._lib = Mock()
        lib._lib.get_component_tree = Mock(return_value="mock tree")

        # Should accept None
        result = lib.get_component_tree(max_depth=None)
        assert result == "mock tree"

        # Should accept 0
        result = lib.get_component_tree(max_depth=0)
        assert result == "mock tree"

        # Should accept positive integers
        result = lib.get_component_tree(max_depth=5)
        assert result == "mock tree"


class TestLocatorDeprecationWarning:
    """Test that locator parameter shows deprecation warning."""

    def test_get_component_tree_warns_on_locator(self):
        """Verify deprecation warning when locator is used."""
        from JavaGui import SwingLibrary
        import warnings

        lib = SwingLibrary()
        lib._lib = Mock()
        lib._lib.get_component_tree = Mock(return_value="mock tree")

        # Should show DeprecationWarning
        with warnings.catch_warnings(record=True) as w:
            warnings.simplefilter("always")
            lib.get_component_tree(locator="JPanel#main")

            assert len(w) == 1
            assert issubclass(w[0].category, DeprecationWarning)
            assert "locator" in str(w[0].message).lower()
            assert "not yet supported" in str(w[0].message).lower()

    def test_save_ui_tree_warns_on_locator(self):
        """Verify deprecation warning in save_ui_tree when locator is used."""
        from JavaGui import SwingLibrary
        import warnings

        lib = SwingLibrary()
        lib._lib = Mock()
        lib._lib.get_ui_tree = Mock(return_value="mock tree")

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            # Should show DeprecationWarning
            with warnings.catch_warnings(record=True) as w:
                warnings.simplefilter("always")
                lib.save_ui_tree(temp_file, locator="JPanel#main")

                assert len(w) == 1
                assert issubclass(w[0].category, DeprecationWarning)
                assert "locator" in str(w[0].message).lower()
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)


class TestSaveUITreeFileOperations:
    """Test save_ui_tree file I/O operations."""

    def test_save_ui_tree_writes_file(self):
        """Verify save_ui_tree writes content to file."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib._lib = Mock()
        lib._lib.get_ui_tree = Mock(return_value="test tree content")

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            lib.save_ui_tree(temp_file)

            # Verify file exists and contains correct content
            assert os.path.exists(temp_file)
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()
            assert content == "test tree content"
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_save_ui_tree_utf8_encoding(self):
        """Verify save_ui_tree uses UTF-8 encoding."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib._lib = Mock()
        # Use Unicode characters
        lib._lib.get_ui_tree = Mock(return_value="JFrame テスト 树 🌳")

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            lib.save_ui_tree(temp_file)

            # Verify UTF-8 encoding preserved
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()
            assert content == "JFrame テスト 树 🌳"
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_save_ui_tree_passes_format_parameter(self):
        """Verify save_ui_tree passes format parameter to get_ui_tree."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib._lib = Mock()
        lib._lib.get_ui_tree = Mock(return_value='{"tree": "data"}')

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.json') as f:
            temp_file = f.name

        try:
            lib.save_ui_tree(temp_file, format="json")

            # Verify get_ui_tree called with correct format
            lib._lib.get_ui_tree.assert_called_once()
            args = lib._lib.get_ui_tree.call_args[0]
            assert args[0] == "json"  # format parameter
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_save_ui_tree_passes_max_depth_parameter(self):
        """Verify save_ui_tree passes max_depth parameter to get_ui_tree."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib._lib = Mock()
        lib._lib.get_ui_tree = Mock(return_value="limited tree")

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            lib.save_ui_tree(temp_file, max_depth=5)

            # Verify get_ui_tree called with correct max_depth
            lib._lib.get_ui_tree.assert_called_once()
            args = lib._lib.get_ui_tree.call_args[0]
            assert args[1] == 5  # max_depth parameter
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)


class TestRustBackendIntegration:
    """Test that Python wrapper correctly calls Rust backend."""

    def test_get_component_tree_calls_rust_backend(self):
        """Verify get_component_tree calls Rust backend method."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib._lib = Mock()
        lib._lib.get_component_tree = Mock(return_value="rust tree")

        result = lib.get_component_tree()

        # Should call Rust backend
        lib._lib.get_component_tree.assert_called_once()
        assert result == "rust tree"

    def test_get_component_tree_passes_all_filter_parameters(self):
        """Verify all filter parameters are passed to Rust backend."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib._lib = Mock()
        lib._lib.get_component_tree = Mock(return_value="filtered tree")

        # Call with all filtering parameters
        lib.get_component_tree(
            locator=None,
            format="json",
            max_depth=3,
            types="JButton,JTextField",
            exclude_types="JLabel",
            visible_only=True,
            enabled_only=True,
            focusable_only=False
        )

        # Verify all parameters passed
        lib._lib.get_component_tree.assert_called_once_with(
            locator=None,
            format="json",
            max_depth=3,
            types="JButton,JTextField",
            exclude_types="JLabel",
            visible_only=True,
            enabled_only=True,
            focusable_only=False
        )


if __name__ == '__main__':
    pytest.main([__file__, '-v'])
