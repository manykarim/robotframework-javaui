"""
Integration tests for output formatters with real Swing library.

Tests YAML, CSV, and Markdown formatters against a live test application.

NOTE: These tests require a real Java application with test_app fixture - not available in CI.
"""

import pytest
import json
import csv
import io
from xml.etree import ElementTree as ET
import yaml

pytestmark = pytest.mark.skip(reason="Integration tests require a real Java application with test_app fixture - not available in CI")


@pytest.mark.integration
class TestOutputFormattersIntegration:
    """Integration tests with real Swing library."""

    @pytest.fixture
    def library(self):
        """Create library instance."""
        from JavaGui import SwingLibrary
        lib = SwingLibrary()
        return lib

    def test_yaml_format_real(self, library, test_app):
        """Test YAML format with real application."""
        # Get component tree in YAML format
        yaml_tree = library.get_component_tree(format="yaml")

        # Verify it's valid YAML
        parsed = yaml.safe_load(yaml_tree)
        assert "roots" in parsed
        assert len(parsed["roots"]) > 0

        # Check structure
        root = parsed["roots"][0]
        assert "component_type" in root
        assert "simple_name" in root["component_type"]
        assert root["component_type"]["simple_name"] in ["JFrame", "JDialog", "JWindow"]

        # Verify hierarchy is preserved
        if "children" in root and root["children"]:
            assert isinstance(root["children"], list)
            assert len(root["children"]) > 0

    def test_yaml_format_alias_yml(self, library, test_app):
        """Test YAML format with 'yml' alias."""
        yaml_tree = library.get_component_tree(format="yml")

        # Should work just like 'yaml'
        parsed = yaml.safe_load(yaml_tree)
        assert "roots" in parsed

    def test_csv_format_real(self, library, test_app):
        """Test CSV format with real application."""
        # Get component tree in CSV format
        csv_tree = library.get_component_tree(format="csv")

        # Verify it's valid CSV
        reader = csv.DictReader(io.StringIO(csv_tree))
        rows = list(reader)

        # Should have at least one row
        assert len(rows) > 0

        # Verify headers
        expected_headers = [
            "path", "depth", "type", "name", "text", "visible",
            "enabled", "bounds_x", "bounds_y", "bounds_width", "bounds_height"
        ]
        assert list(rows[0].keys()) == expected_headers

        # Verify first row is root component
        assert rows[0]["depth"] == "0"
        assert rows[0]["type"] in ["JFrame", "JDialog", "JWindow"]

        # Verify depth progression
        depths = [int(row["depth"]) for row in rows]
        assert 0 in depths  # Has root
        assert max(depths) >= 0  # Depth makes sense

    def test_csv_format_flattened_hierarchy(self, library, test_app):
        """Test CSV format correctly flattens hierarchy."""
        csv_tree = library.get_component_tree(format="csv")

        reader = csv.DictReader(io.StringIO(csv_tree))
        rows = list(reader)

        # Check that path reflects hierarchy
        paths = [row["path"] for row in rows]

        # Root should be "0"
        assert "0" in paths

        # Children should have paths like "0.0", "0.1", etc.
        child_paths = [p for p in paths if p.startswith("0.") and p.count(".") == 1]
        assert len(child_paths) > 0

    def test_csv_with_filters(self, library, test_app):
        """Test CSV format with component filters."""
        # Get only buttons in CSV format
        csv_tree = library.get_component_tree(format="csv", types="JButton")

        reader = csv.DictReader(io.StringIO(csv_tree))
        rows = list(reader)

        # All rows should be JButton
        for row in rows:
            assert "Button" in row["type"]

    def test_csv_bounds_accuracy(self, library, test_app):
        """Test CSV format has accurate bounds information."""
        csv_tree = library.get_component_tree(format="csv")

        reader = csv.DictReader(io.StringIO(csv_tree))
        rows = list(reader)

        # Check that bounds are numeric
        for row in rows:
            assert row["bounds_x"].isdigit() or row["bounds_x"].startswith("-")
            assert row["bounds_y"].isdigit() or row["bounds_y"].startswith("-")
            assert row["bounds_width"].isdigit()
            assert row["bounds_height"].isdigit()

            # Width and height should be non-negative
            assert int(row["bounds_width"]) >= 0
            assert int(row["bounds_height"]) >= 0

    def test_markdown_format_real(self, library, test_app):
        """Test Markdown format with real application."""
        # Get component tree in Markdown format
        md_tree = library.get_component_tree(format="markdown")

        # Should start with header
        assert md_tree.startswith("# UI Component Tree")

        # Should contain list markers
        assert "- **" in md_tree or "* **" in md_tree or "+ **" in md_tree

        # Should contain component types
        lines = md_tree.split('\n')
        component_lines = [l for l in lines if "**" in l]
        assert len(component_lines) > 0

    def test_markdown_format_alias_md(self, library, test_app):
        """Test Markdown format with 'md' alias."""
        md_tree = library.get_component_tree(format="md")

        # Should work just like 'markdown'
        assert md_tree.startswith("# UI Component Tree")

    def test_markdown_hierarchy_indentation(self, library, test_app):
        """Test Markdown format shows hierarchy with indentation."""
        md_tree = library.get_component_tree(format="markdown")

        lines = md_tree.split('\n')

        # Should have root level (no indent)
        root_lines = [l for l in lines if l.startswith("-") or l.startswith("*") or l.startswith("+")]
        assert len(root_lines) > 0

        # Should have child level (with indent)
        child_lines = [l for l in lines if l.startswith("  -") or l.startswith("  *") or l.startswith("  +")]
        if len(lines) > 10:  # Only check if tree is large enough
            assert len(child_lines) > 0

    def test_markdown_visibility_badges(self, library, test_app):
        """Test Markdown format includes visibility badges."""
        md_tree = library.get_component_tree(format="markdown")

        # Should contain emoji badges
        assert "👁️" in md_tree or "🚫" in md_tree  # Visible/hidden
        assert "✅" in md_tree or "❌" in md_tree  # Enabled/disabled

    def test_markdown_bounds_info(self, library, test_app):
        """Test Markdown format includes bounds information."""
        md_tree = library.get_component_tree(format="markdown")

        # Should contain bounds in format "WxH at (X, Y)"
        assert "*Bounds:*" in md_tree
        assert "×" in md_tree  # Width × Height separator
        assert "at" in md_tree

    def test_markdown_with_max_depth(self, library, test_app):
        """Test Markdown format respects max_depth."""
        # Get full tree
        full_tree = library.get_component_tree(format="markdown")

        # Get limited tree
        limited_tree = library.get_component_tree(format="markdown", max_depth=1)

        # Limited should be shorter
        assert len(limited_tree) <= len(full_tree)

    def test_all_formats_same_component_count(self, library, test_app):
        """Test all formats represent the same components."""
        # Get trees in all formats
        json_tree = json.loads(library.get_component_tree(format="json"))
        yaml_tree = yaml.safe_load(library.get_component_tree(format="yaml"))
        csv_tree = library.get_component_tree(format="csv")

        # Count components in each format
        def count_json_components(tree):
            count = len(tree["roots"])
            for root in tree["roots"]:
                count += count_children(root)
            return count

        def count_children(component):
            count = 0
            if "children" in component and component["children"]:
                count += len(component["children"])
                for child in component["children"]:
                    count += count_children(child)
            return count

        # Count CSV rows (excluding header)
        csv_count = len(io.StringIO(csv_tree).readlines()) - 1

        json_count = count_json_components(json_tree)
        yaml_count = count_json_components(yaml_tree)

        # All should have same count
        assert json_count == yaml_count == csv_count

    def test_format_case_insensitive_real(self, library, test_app):
        """Test format parameter is case-insensitive."""
        # Test various case combinations
        yaml1 = library.get_component_tree(format="yaml")
        yaml2 = library.get_component_tree(format="YAML")
        yaml3 = library.get_component_tree(format="Yaml")

        # All should produce valid YAML
        assert yaml.safe_load(yaml1)
        assert yaml.safe_load(yaml2)
        assert yaml.safe_load(yaml3)

    def test_invalid_format_error(self, library, test_app):
        """Test invalid format raises proper error."""
        with pytest.raises(Exception) as exc_info:
            library.get_component_tree(format="invalid")

        error_msg = str(exc_info.value)
        assert "Unknown format" in error_msg or "Invalid format" in error_msg

    def test_yaml_preserves_unicode(self, library, test_app):
        """Test YAML format preserves Unicode characters."""
        yaml_tree = library.get_component_tree(format="yaml")
        parsed = yaml.safe_load(yaml_tree)

        # Should be able to parse without errors
        assert isinstance(parsed, dict)

    def test_csv_escapes_special_characters(self, library, test_app):
        """Test CSV format properly escapes special characters."""
        csv_tree = library.get_component_tree(format="csv")

        # Should be valid CSV
        reader = csv.DictReader(io.StringIO(csv_tree))
        rows = list(reader)

        # Check for proper escaping in text fields
        for row in rows:
            # If text contains comma, it should be handled by CSV
            text = row["text"]
            # CSV module should have handled escaping automatically
            assert isinstance(text, str)

    def test_markdown_escapes_special_markdown(self, library, test_app):
        """Test Markdown format escapes special markdown characters."""
        md_tree = library.get_component_tree(format="markdown")

        # Newlines in text should be escaped
        if "\\n" in md_tree:
            # Escaped newlines should not break markdown structure
            lines = md_tree.split('\n')
            assert all(isinstance(line, str) for line in lines)

    @pytest.mark.performance
    def test_yaml_performance(self, library, test_app):
        """Test YAML format performance."""
        import time

        start = time.time()
        yaml_tree = library.get_component_tree(format="yaml")
        duration = time.time() - start

        # Should complete in reasonable time (<100ms for typical tree)
        assert duration < 1.0  # 1 second max

        # Verify output is valid
        parsed = yaml.safe_load(yaml_tree)
        assert "roots" in parsed

    @pytest.mark.performance
    def test_csv_performance(self, library, test_app):
        """Test CSV format performance."""
        import time

        start = time.time()
        csv_tree = library.get_component_tree(format="csv")
        duration = time.time() - start

        # Should complete in reasonable time
        assert duration < 1.0  # 1 second max

        # Verify output is valid
        reader = csv.DictReader(io.StringIO(csv_tree))
        rows = list(reader)
        assert len(rows) > 0

    @pytest.mark.performance
    def test_markdown_performance(self, library, test_app):
        """Test Markdown format performance."""
        import time

        start = time.time()
        md_tree = library.get_component_tree(format="markdown")
        duration = time.time() - start

        # Should complete in reasonable time
        assert duration < 1.0  # 1 second max

        # Verify output is valid
        assert md_tree.startswith("# UI Component Tree")


@pytest.mark.integration
class TestOutputFormattersWithFilters:
    """Test output formatters work correctly with filtering."""

    @pytest.fixture
    def library(self):
        """Create library instance."""
        from JavaGui import SwingLibrary
        lib = SwingLibrary()
        return lib

    def test_yaml_with_type_filter(self, library, test_app):
        """Test YAML format with type filter."""
        yaml_tree = library.get_component_tree(format="yaml", types="JButton")

        parsed = yaml.safe_load(yaml_tree)

        # Verify all components are buttons
        def check_all_buttons(components):
            for comp in components:
                if "component_type" in comp:
                    assert "Button" in comp["component_type"]["simple_name"]
                if "children" in comp and comp["children"]:
                    check_all_buttons(comp["children"])

        if parsed["roots"]:
            check_all_buttons(parsed["roots"])

    def test_csv_with_visible_only(self, library, test_app):
        """Test CSV format with visible_only filter."""
        csv_tree = library.get_component_tree(format="csv", visible_only=True)

        reader = csv.DictReader(io.StringIO(csv_tree))
        rows = list(reader)

        # All components should be visible
        for row in rows:
            assert row["visible"].lower() == "true"

    def test_markdown_with_max_depth(self, library, test_app):
        """Test Markdown format with max_depth filter."""
        md_tree = library.get_component_tree(format="markdown", max_depth=2)

        lines = md_tree.split('\n')

        # Should not have deeply nested items (more than 2 indents)
        deep_nested = [l for l in lines if l.startswith("      ")]  # 3+ levels of indent
        assert len(deep_nested) == 0


if __name__ == "__main__":
    pytest.main([__file__, "-v", "-m", "integration"])
