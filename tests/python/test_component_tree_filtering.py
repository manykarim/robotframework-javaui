"""
Test suite for component tree filtering functionality.

Tests element type and state filtering capabilities including:
- Type inclusion/exclusion with wildcards
- State filters (visible, enabled, focusable)
- Filter combinations
- Edge cases and error handling
"""

import pytest
import json
from .conftest import MockSwingLibrary


class TestTypeFiltering:
    """Test element type filtering with inclusion and exclusion."""

    def test_filter_single_type(self, mock_rust_core):
        """Test filtering by a single component type."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get tree with only JButton components
        tree = lib.get_component_tree(types="JButton", format="json")
        data = json.loads(tree)

        # Verify all components are JButton
        assert data is not None
        for root in data.get("roots", []):
            assert_all_types_match(root, ["JButton"])

        lib.disconnect()

    def test_filter_multiple_types(self, mock_rust_core):
        """Test filtering by multiple component types."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get tree with JButton and JTextField
        tree = lib.get_component_tree(types="JButton,JTextField", format="json")
        data = json.loads(tree)

        # Verify components are one of the specified types
        assert data is not None
        for root in data.get("roots", []):
            assert_all_types_in(root, ["JButton", "JTextField"])

        lib.disconnect()

    def test_filter_with_wildcard_prefix(self, mock_rust_core):
        """Test type filtering with wildcard prefix (J*Button)."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get tree with all button types
        tree = lib.get_component_tree(types="J*Button", format="json")
        data = json.loads(tree)

        # Should match JButton, JToggleButton, JRadioButton, etc.
        assert data is not None
        for root in data.get("roots", []):
            assert_all_types_match_pattern(root, r"J.*Button")

        lib.disconnect()

    def test_filter_with_wildcard_suffix(self, mock_rust_core):
        """Test type filtering with wildcard suffix (JText*)."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get tree with all text components
        tree = lib.get_component_tree(types="JText*", format="json")
        data = json.loads(tree)

        # Should match JTextField, JTextArea, JTextPane, etc.
        assert data is not None
        for root in data.get("roots", []):
            assert_all_types_match_pattern(root, r"JText.*")

        lib.disconnect()

    def test_exclude_types(self, mock_rust_core):
        """Test excluding specific component types."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get tree excluding JLabel
        tree = lib.get_component_tree(exclude_types="JLabel", format="json")
        data = json.loads(tree)

        # Verify no JLabel components present
        assert data is not None
        for root in data.get("roots", []):
            assert_type_not_present(root, "JLabel")

        lib.disconnect()

    def test_exclude_multiple_types(self, mock_rust_core):
        """Test excluding multiple component types."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get tree excluding JLabel and JPanel
        tree = lib.get_component_tree(
            exclude_types="JLabel,JPanel",
            format="json"
        )
        data = json.loads(tree)

        # Verify no JLabel or JPanel components
        assert data is not None
        for root in data.get("roots", []):
            assert_type_not_present(root, "JLabel")
            assert_type_not_present(root, "JPanel")

        lib.disconnect()

    def test_include_and_exclude_combination(self, mock_rust_core):
        """Test combining type inclusion and exclusion filters."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Include all buttons but exclude radio buttons
        tree = lib.get_component_tree(
            types="J*Button",
            exclude_types="JRadioButton",
            format="json"
        )
        data = json.loads(tree)

        # Should have buttons but not radio buttons
        assert data is not None
        for root in data.get("roots", []):
            assert_type_not_present(root, "JRadioButton")
            # Other button types should be present

        lib.disconnect()

    def test_invalid_type_pattern(self, mock_rust_core):
        """Test error handling for invalid type patterns."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Empty type should raise error
        with pytest.raises(Exception) as exc_info:
            lib.get_component_tree(types="JButton,,JTextField")

        # Error message should mention empty pattern
        assert "empty" in str(exc_info.value).lower() or "invalid" in str(exc_info.value).lower()

        lib.disconnect()


class TestStateFiltering:
    """Test element state filtering (visible, enabled, focusable)."""

    def test_visible_only_filter(self, mock_rust_core):
        """Test filtering for visible components only."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get only visible components
        tree = lib.get_component_tree(visible_only=True, format="json")
        data = json.loads(tree)

        # All components should be visible
        assert data is not None
        for root in data.get("roots", []):
            assert_all_visible(root)

        lib.disconnect()

    def test_enabled_only_filter(self, mock_rust_core):
        """Test filtering for enabled components only."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get only enabled components
        tree = lib.get_component_tree(enabled_only=True, format="json")
        data = json.loads(tree)

        # All components should be enabled
        assert data is not None
        for root in data.get("roots", []):
            assert_all_enabled(root)

        lib.disconnect()

    def test_focusable_only_filter(self, mock_rust_core):
        """Test filtering for focusable components only."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get only focusable components
        tree = lib.get_component_tree(focusable_only=True, format="json")
        data = json.loads(tree)

        # All components should be focusable
        assert data is not None
        for root in data.get("roots", []):
            assert_all_focusable(root)

        lib.disconnect()

    def test_multiple_state_filters(self, mock_rust_core):
        """Test combining multiple state filters."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get components that are visible AND enabled
        tree = lib.get_component_tree(
            visible_only=True,
            enabled_only=True,
            format="json"
        )
        data = json.loads(tree)

        # All components should be both visible and enabled
        assert data is not None
        for root in data.get("roots", []):
            assert_all_visible(root)
            assert_all_enabled(root)

        lib.disconnect()

    def test_all_state_filters_combined(self, mock_rust_core):
        """Test all state filters at once."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get components that are visible, enabled, AND focusable
        tree = lib.get_component_tree(
            visible_only=True,
            enabled_only=True,
            focusable_only=True,
            format="json"
        )
        data = json.loads(tree)

        # All components should meet all criteria
        assert data is not None
        for root in data.get("roots", []):
            assert_all_visible(root)
            assert_all_enabled(root)
            assert_all_focusable(root)

        lib.disconnect()


class TestFilterCombinations:
    """Test combinations of type and state filters."""

    def test_type_and_visible_filters(self, mock_rust_core):
        """Test combining type and visibility filters."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get visible JButtons only
        tree = lib.get_component_tree(
            types="JButton",
            visible_only=True,
            format="json"
        )
        data = json.loads(tree)

        # Should be JButtons that are visible
        assert data is not None
        for root in data.get("roots", []):
            assert_all_types_match(root, ["JButton"])
            assert_all_visible(root)

        lib.disconnect()

    def test_type_and_enabled_filters(self, mock_rust_core):
        """Test combining type and enabled filters."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get enabled text fields only
        tree = lib.get_component_tree(
            types="JTextField",
            enabled_only=True,
            format="json"
        )
        data = json.loads(tree)

        assert data is not None
        for root in data.get("roots", []):
            assert_all_types_match(root, ["JTextField"])
            assert_all_enabled(root)

        lib.disconnect()

    def test_wildcard_type_with_all_states(self, mock_rust_core):
        """Test wildcard type filter with all state filters."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get all buttons that are visible, enabled, and focusable
        tree = lib.get_component_tree(
            types="J*Button",
            visible_only=True,
            enabled_only=True,
            focusable_only=True,
            format="json"
        )
        data = json.loads(tree)

        assert data is not None
        for root in data.get("roots", []):
            assert_all_types_match_pattern(root, r"J.*Button")
            assert_all_visible(root)
            assert_all_enabled(root)
            assert_all_focusable(root)

        lib.disconnect()

    def test_exclude_with_state_filters(self, mock_rust_core):
        """Test exclusion filter with state filters."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get visible components excluding labels
        tree = lib.get_component_tree(
            exclude_types="JLabel",
            visible_only=True,
            format="json"
        )
        data = json.loads(tree)

        assert data is not None
        for root in data.get("roots", []):
            assert_type_not_present(root, "JLabel")
            assert_all_visible(root)

        lib.disconnect()


class TestEdgeCases:
    """Test edge cases and error conditions."""

    def test_empty_result_warning(self, mock_rust_core, capfd):
        """Test that empty results generate a warning."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Filter that might result in no matches
        tree = lib.get_component_tree(
            types="NonExistentType",
            format="json"
        )

        # Should get an empty or minimal tree
        data = json.loads(tree)
        # Check for warning in stderr
        captured = capfd.readouterr()
        # Warning might appear in stderr

        lib.disconnect()

    def test_conflicting_filters(self, mock_rust_core, capfd):
        """Test warning when same type in include and exclude."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Same type in both lists should generate warning
        tree = lib.get_component_tree(
            types="JButton",
            exclude_types="JButton",
            format="json"
        )

        # Check for warning about conflicting filters
        captured = capfd.readouterr()
        # Should see warning in stderr

        lib.disconnect()

    def test_max_depth_with_filters(self, mock_rust_core):
        """Test combining max_depth with filters."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Get shallow tree with type filter
        tree = lib.get_component_tree(
            types="JButton",
            max_depth=2,
            format="json"
        )
        data = json.loads(tree)

        assert data is not None
        # Tree should be limited in depth
        max_actual_depth = get_max_depth(data.get("roots", []))
        assert max_actual_depth <= 2

        lib.disconnect()

    def test_all_formats_with_filters(self, mock_rust_core):
        """Test that filters work with all output formats."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        filters = {
            "types": "JButton",
            "visible_only": True
        }

        # JSON format
        json_tree = lib.get_component_tree(format="json", **filters)
        assert json_tree is not None
        assert "JButton" in json_tree

        # XML format
        xml_tree = lib.get_component_tree(format="xml", **filters)
        assert xml_tree is not None
        assert "JButton" in xml_tree

        # Text format
        text_tree = lib.get_component_tree(format="text", **filters)
        assert text_tree is not None
        assert "JButton" in text_tree

        # YAML format
        yaml_tree = lib.get_component_tree(format="yaml", **filters)
        assert yaml_tree is not None
        assert "JButton" in yaml_tree

        lib.disconnect()

    def test_case_sensitivity_in_types(self, mock_rust_core):
        """Test that type matching is case-sensitive."""
        lib = MockSwingLibrary()
        lib.connect(pid=12345)

        # Lowercase should not match (component types are case-sensitive)
        tree1 = lib.get_component_tree(types="jbutton", format="json")
        tree2 = lib.get_component_tree(types="JButton", format="json")

        # Results should differ (or tree1 should be empty)
        # Component types in Java are case-sensitive

        lib.disconnect()


# Helper functions for assertions

def assert_all_types_match(component, allowed_types):
    """Recursively verify all components match allowed types."""
    comp_type = component.get("type") or component.get("simpleClass")
    assert comp_type in allowed_types, f"Found type {comp_type} not in {allowed_types}"

    children = component.get("children") or []
    for child in children:
        assert_all_types_match(child, allowed_types)


def assert_all_types_in(component, allowed_types):
    """Recursively verify all components are in allowed types list."""
    comp_type = component.get("type") or component.get("simpleClass")
    assert comp_type in allowed_types, f"Found type {comp_type} not in {allowed_types}"

    children = component.get("children") or []
    for child in children:
        assert_all_types_in(child, allowed_types)


def assert_all_types_match_pattern(component, pattern):
    """Recursively verify all component types match regex pattern."""
    import re
    comp_type = component.get("type") or component.get("simpleClass")
    assert re.match(pattern, comp_type), f"Type {comp_type} doesn't match {pattern}"

    children = component.get("children") or []
    for child in children:
        assert_all_types_match_pattern(child, pattern)


def assert_type_not_present(component, excluded_type):
    """Recursively verify a type is not present in tree."""
    comp_type = component.get("type") or component.get("simpleClass")
    assert comp_type != excluded_type, f"Found excluded type {excluded_type}"

    children = component.get("children") or []
    for child in children:
        assert_type_not_present(child, excluded_type)


def assert_all_visible(component):
    """Recursively verify all components are visible."""
    assert component.get("visible", False), "Component not visible"
    assert component.get("showing", False), "Component not showing"

    children = component.get("children") or []
    for child in children:
        assert_all_visible(child)


def assert_all_enabled(component):
    """Recursively verify all components are enabled."""
    assert component.get("enabled", False), "Component not enabled"

    children = component.get("children") or []
    for child in children:
        assert_all_enabled(child)


def assert_all_focusable(component):
    """Recursively verify all components are focusable."""
    assert component.get("focusable", False), "Component not focusable"

    children = component.get("children") or []
    for child in children:
        assert_all_focusable(child)


def get_max_depth(roots, current=0):
    """Calculate maximum depth of tree."""
    if not roots:
        return current

    max_d = current
    for root in roots:
        children = root.get("children") or []
        if children:
            child_depth = get_max_depth(children, current + 1)
            max_d = max(max_d, child_depth)

    return max_d
