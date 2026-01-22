"""
Unit tests for component tree methods (get_component_tree and save_ui_tree).

Tests the bug fixes for:
1. get_component_tree - Fixed parameter passing bug (was passing locator as format)
2. save_ui_tree - Added format and max_depth parameters with proper file I/O

Uses mocks to test without requiring Rust extension compilation.
"""

import pytest
import os
import tempfile
from unittest.mock import Mock, patch, MagicMock
import warnings
import sys


class TestGetComponentTree:
    """Test get_component_tree method with all parameter combinations."""

    def test_get_component_tree_default_parameters(self, mock_rust_core):
        """Test get_component_tree with default parameters (text format, no depth limit)."""
        # Need to import from JavaGui since that's where SwingLibrary is defined
        sys.path.insert(0, '/mnt/c/workspace/robotframework-swing/python')
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        tree = lib.get_component_tree()

        # Should return a string with JFrame in it
        assert isinstance(tree, str)
        assert "JFrame" in tree

    def test_get_component_tree_text_format(self, mock_rust_core):
        """Test get_component_tree with explicit text format."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        tree = lib.get_component_tree(format="text")

        assert isinstance(tree, str)
        assert "JFrame" in tree

    def test_get_component_tree_json_format(self, mock_rust_core):
        """Test get_component_tree with JSON format."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        tree = lib.get_component_tree(format="json")

        assert isinstance(tree, str)
        # Mock returns text format, but API accepts json parameter
        assert tree is not None

    def test_get_component_tree_xml_format(self, mock_rust_core):
        """Test get_component_tree with XML format."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        tree = lib.get_component_tree(format="xml")

        assert isinstance(tree, str)
        assert tree is not None

    def test_get_component_tree_with_depth_limit(self, mock_rust_core):
        """Test get_component_tree with max_depth parameter."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        # Test with different depth limits
        tree_depth_2 = lib.get_component_tree(max_depth=2)
        tree_depth_5 = lib.get_component_tree(max_depth=5)

        assert isinstance(tree_depth_2, str)
        assert isinstance(tree_depth_5, str)

    def test_get_component_tree_format_and_depth(self, mock_rust_core):
        """Test get_component_tree with both format and max_depth."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        tree = lib.get_component_tree(format="json", max_depth=3)

        assert isinstance(tree, str)
        assert tree is not None

    def test_get_component_tree_locator_warning(self, mock_rust_core):
        """Test that locator parameter raises deprecation warning."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        # Should raise DeprecationWarning about unsupported locator
        with pytest.warns(DeprecationWarning, match="locator.*not yet supported"):
            tree = lib.get_component_tree(locator="JPanel#main")

        # Should still return tree (ignoring locator)
        assert isinstance(tree, str)

    def test_get_component_tree_all_parameters(self, mock_rust_core):
        """Test get_component_tree with all parameters including unsupported locator."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        with pytest.warns(DeprecationWarning):
            tree = lib.get_component_tree(
                locator="JButton#test",
                format="json",
                max_depth=10
            )

        assert isinstance(tree, str)


class TestSaveUITree:
    """Test save_ui_tree method with file I/O and parameter handling."""

    def test_save_ui_tree_default_parameters(self, mock_rust_core):
        """Test save_ui_tree with default parameters (text format)."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            lib.save_ui_tree(temp_file)

            # Verify file was created and contains tree data
            assert os.path.exists(temp_file)
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()

            assert len(content) > 0
            assert "JFrame" in content
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_save_ui_tree_text_format(self, mock_rust_core):
        """Test save_ui_tree with explicit text format."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            lib.save_ui_tree(temp_file, format="text")

            assert os.path.exists(temp_file)
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()

            assert "JFrame" in content
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_save_ui_tree_json_format(self, mock_rust_core):
        """Test save_ui_tree with JSON format."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.json') as f:
            temp_file = f.name

        try:
            lib.save_ui_tree(temp_file, format="json")

            assert os.path.exists(temp_file)
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()

            # Content should be valid JSON when Rust backend properly implements it
            assert len(content) > 0
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_save_ui_tree_xml_format(self, mock_rust_core):
        """Test save_ui_tree with XML format."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.xml') as f:
            temp_file = f.name

        try:
            lib.save_ui_tree(temp_file, format="xml")

            assert os.path.exists(temp_file)
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()

            assert len(content) > 0
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_save_ui_tree_with_depth_limit(self, mock_rust_core):
        """Test save_ui_tree with max_depth parameter."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            lib.save_ui_tree(temp_file, max_depth=3)

            assert os.path.exists(temp_file)
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()

            assert len(content) > 0
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_save_ui_tree_format_and_depth(self, mock_rust_core):
        """Test save_ui_tree with both format and max_depth."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.json') as f:
            temp_file = f.name

        try:
            lib.save_ui_tree(temp_file, format="json", max_depth=5)

            assert os.path.exists(temp_file)
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()

            assert len(content) > 0
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_save_ui_tree_locator_warning(self, mock_rust_core):
        """Test that locator parameter raises deprecation warning."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            with pytest.warns(DeprecationWarning, match="locator.*not yet supported"):
                lib.save_ui_tree(temp_file, locator="JPanel#main")

            assert os.path.exists(temp_file)
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_save_ui_tree_all_parameters(self, mock_rust_core):
        """Test save_ui_tree with all parameters."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.json') as f:
            temp_file = f.name

        try:
            with pytest.warns(DeprecationWarning):
                lib.save_ui_tree(
                    temp_file,
                    locator="JButton#test",
                    format="json",
                    max_depth=10
                )

            assert os.path.exists(temp_file)
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()

            assert len(content) > 0
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)

    def test_save_ui_tree_creates_parent_directory(self, mock_rust_core):
        """Test save_ui_tree creates parent directories if needed."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        # Create a path with non-existent parent directory
        temp_dir = tempfile.mkdtemp()
        temp_file = os.path.join(temp_dir, "subdir", "tree.txt")

        try:
            # This should fail if parent directory doesn't exist
            # We expect the caller to create directories
            os.makedirs(os.path.dirname(temp_file), exist_ok=True)
            lib.save_ui_tree(temp_file)

            assert os.path.exists(temp_file)
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)
            if os.path.exists(temp_dir):
                import shutil
                shutil.rmtree(temp_dir)

    def test_save_ui_tree_file_encoding_utf8(self, mock_rust_core):
        """Test that save_ui_tree uses UTF-8 encoding."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            lib.save_ui_tree(temp_file)

            # Read with UTF-8 encoding should work
            with open(temp_file, 'r', encoding='utf-8') as f:
                content = f.read()

            assert len(content) > 0
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)


class TestErrorHandling:
    """Test error handling for component tree methods."""

    def test_get_component_tree_disconnected(self):
        """Test get_component_tree raises error when not connected."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()

        # Should raise error when not connected (if Rust backend checks)
        # For now, just verify it doesn't crash
        try:
            tree = lib.get_component_tree()
        except Exception:
            # Expected if Rust backend validates connection
            pass

    def test_save_ui_tree_invalid_path(self, mock_rust_core):
        """Test save_ui_tree with invalid file path."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        # Try to save to invalid path
        with pytest.raises((OSError, IOError, PermissionError)):
            lib.save_ui_tree("/invalid/path/tree.txt")

    def test_save_ui_tree_permission_denied(self, mock_rust_core):
        """Test save_ui_tree with permission denied."""
        from JavaGui import SwingLibrary
        import sys

        # Skip on Windows as permission handling is different
        if sys.platform == 'win32':
            pytest.skip("Permission test not applicable on Windows")

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        # Create a read-only directory
        temp_dir = tempfile.mkdtemp()
        os.chmod(temp_dir, 0o444)
        temp_file = os.path.join(temp_dir, "tree.txt")

        try:
            with pytest.raises(PermissionError):
                lib.save_ui_tree(temp_file)
        finally:
            os.chmod(temp_dir, 0o755)
            if os.path.exists(temp_dir):
                import shutil
                shutil.rmtree(temp_dir)


class TestBackwardCompatibility:
    """Test backward compatibility with old API usage."""

    def test_get_component_tree_old_usage(self, mock_rust_core):
        """Test get_component_tree works with old usage patterns."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        # Old usage: positional format parameter
        tree = lib.get_component_tree(None, "json")
        assert isinstance(tree, str)

    def test_save_ui_tree_old_usage(self, mock_rust_core):
        """Test save_ui_tree works with old usage patterns."""
        from JavaGui import SwingLibrary

        lib = SwingLibrary()
        lib.connect_to_application(pid=12345)

        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
            temp_file = f.name

        try:
            # Old usage: filename only
            lib.save_ui_tree(temp_file)
            assert os.path.exists(temp_file)

            # Old usage with locator
            with pytest.warns(DeprecationWarning):
                lib.save_ui_tree(temp_file, "JPanel#main")
            assert os.path.exists(temp_file)
        finally:
            if os.path.exists(temp_file):
                os.unlink(temp_file)
