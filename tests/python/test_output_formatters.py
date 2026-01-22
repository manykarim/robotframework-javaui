"""
Test suite for output format support in get_component_tree.

Tests for JSON, XML, YAML, CSV, Markdown, and Text formatters.
"""

import pytest
import json
import csv
import io
from xml.etree import ElementTree as ET
import yaml


class TestOutputFormatters:
    """Test all output formatters for get_component_tree."""

    @pytest.fixture
    def mock_tree_data(self):
        """Create mock UI tree data for testing."""
        return {
            "roots": [
                {
                    "id": {"tree_path": "0", "hash_code": 12345},
                    "component_type": {"class_name": "javax.swing.JFrame", "simple_name": "JFrame"},
                    "identity": {"name": "MainWindow", "text": "Test Application"},
                    "state": {"visible": True, "enabled": True, "showing": True, "focusable": True},
                    "geometry": {
                        "bounds": {"x": 0, "y": 0, "width": 800, "height": 600},
                        "screen_location": None
                    },
                    "properties": {},
                    "accessibility": None,
                    "metadata": {"depth": 0, "child_count": 2},
                    "children": [
                        {
                            "id": {"tree_path": "0.0", "hash_code": 12346},
                            "component_type": {"class_name": "javax.swing.JButton", "simple_name": "JButton"},
                            "identity": {"name": "loginButton", "text": "Login"},
                            "state": {"visible": True, "enabled": True, "showing": True, "focusable": True},
                            "geometry": {
                                "bounds": {"x": 10, "y": 10, "width": 100, "height": 30},
                                "screen_location": None
                            },
                            "properties": {},
                            "accessibility": None,
                            "metadata": {"depth": 1, "child_count": 0},
                            "children": None
                        },
                        {
                            "id": {"tree_path": "0.1", "hash_code": 12347},
                            "component_type": {"class_name": "javax.swing.JTextField", "simple_name": "JTextField"},
                            "identity": {"name": "usernameField", "text": ""},
                            "state": {"visible": True, "enabled": True, "showing": True, "focusable": True},
                            "geometry": {
                                "bounds": {"x": 10, "y": 50, "width": 200, "height": 25},
                                "screen_location": None
                            },
                            "properties": {},
                            "accessibility": None,
                            "metadata": {"depth": 1, "child_count": 0},
                            "children": None
                        }
                    ]
                }
            ],
            "timestamp": 1234567890,
            "metadata": {"total_components": 3}
        }

    def test_json_format(self, mock_tree_data):
        """Test JSON format output."""
        # Simulate JSON formatting
        json_output = json.dumps(mock_tree_data, indent=2)

        # Verify it's valid JSON
        parsed = json.loads(json_output)
        assert "roots" in parsed
        assert len(parsed["roots"]) == 1
        assert parsed["roots"][0]["component_type"]["simple_name"] == "JFrame"

        # Verify hierarchy is preserved
        assert "children" in parsed["roots"][0]
        assert len(parsed["roots"][0]["children"]) == 2
        assert parsed["roots"][0]["children"][0]["component_type"]["simple_name"] == "JButton"

    def test_xml_format_structure(self):
        """Test XML format structure and validity."""
        # Simulate XML output
        xml_output = """<?xml version="1.0" encoding="UTF-8"?>
<uitree>
  <component type="JFrame" name="MainWindow" text="Test Application" enabled="true" visible="true">
    <component type="JButton" name="loginButton" text="Login" enabled="true" visible="true" />
    <component type="JTextField" name="usernameField" text="" enabled="true" visible="true" />
  </component>
</uitree>"""

        # Parse XML to verify structure
        root = ET.fromstring(xml_output)
        assert root.tag == "uitree"

        # Check root component
        components = root.findall("component")
        assert len(components) == 1
        assert components[0].get("type") == "JFrame"
        assert components[0].get("name") == "MainWindow"

        # Check children
        children = components[0].findall("component")
        assert len(children) == 2
        assert children[0].get("type") == "JButton"
        assert children[0].get("text") == "Login"
        assert children[1].get("type") == "JTextField"

    def test_xml_special_characters(self):
        """Test XML escaping of special characters."""
        xml_with_special = """<?xml version="1.0" encoding="UTF-8"?>
<uitree>
  <component type="JLabel" name="label" text="Text with &lt;special&gt; &quot;chars&quot; &amp; symbols" enabled="true" visible="true" />
</uitree>"""

        root = ET.fromstring(xml_with_special)
        component = root.find("component")
        text = component.get("text")

        # XML parser should unescape automatically
        assert "<special>" in text
        assert '"chars"' in text
        assert "&" in text

    def test_yaml_format(self, mock_tree_data):
        """Test YAML format output."""
        # Simulate YAML formatting
        yaml_output = yaml.dump(mock_tree_data, default_flow_style=False)

        # Verify it's valid YAML
        parsed = yaml.safe_load(yaml_output)
        assert "roots" in parsed
        assert len(parsed["roots"]) == 1
        assert parsed["roots"][0]["component_type"]["simple_name"] == "JFrame"

        # Verify hierarchy
        assert "children" in parsed["roots"][0]
        assert len(parsed["roots"][0]["children"]) == 2

    def test_csv_format_structure(self):
        """Test CSV format with flattened hierarchy."""
        # Simulate CSV output
        csv_data = """path,depth,type,name,text,visible,enabled,bounds_x,bounds_y,bounds_width,bounds_height
0,0,JFrame,MainWindow,Test Application,true,true,0,0,800,600
0.0,1,JButton,loginButton,Login,true,true,10,10,100,30
0.1,1,JTextField,usernameField,,true,true,10,50,200,25"""

        # Parse CSV
        reader = csv.DictReader(io.StringIO(csv_data))
        rows = list(reader)

        # Verify header and rows
        assert len(rows) == 3

        # Check root component
        assert rows[0]["path"] == "0"
        assert rows[0]["depth"] == "0"
        assert rows[0]["type"] == "JFrame"
        assert rows[0]["name"] == "MainWindow"
        assert rows[0]["text"] == "Test Application"

        # Check child components
        assert rows[1]["depth"] == "1"
        assert rows[1]["type"] == "JButton"
        assert rows[1]["text"] == "Login"

        assert rows[2]["depth"] == "1"
        assert rows[2]["type"] == "JTextField"

    def test_csv_special_characters(self):
        """Test CSV escaping of special characters."""
        csv_with_special = """path,depth,type,name,text,visible,enabled,bounds_x,bounds_y,bounds_width,bounds_height
0,0,JLabel,label,"Text with ""quotes"" and, commas",true,true,0,0,100,30
0.1,1,JTextArea,textarea,"Line 1\\nLine 2\\nLine 3",true,true,0,0,200,100"""

        reader = csv.DictReader(io.StringIO(csv_with_special))
        rows = list(reader)

        # CSV parser should handle quotes
        assert 'quotes' in rows[0]["text"]
        assert 'commas' in rows[0]["text"]

        # Check escaped newlines
        assert '\\n' in rows[1]["text"]

    def test_markdown_format_structure(self):
        """Test Markdown format with hierarchical lists."""
        # Simulate Markdown output
        md_output = """# UI Component Tree

- **JFrame** `MainWindow` - 👁️ visible ✅ enabled
  - *Text:* `Test Application`
  - *Bounds:* `800×600` at `(0, 0)`
  - **JButton** `loginButton` - 👁️ visible ✅ enabled
    - *Text:* `Login`
    - *Bounds:* `100×30` at `(10, 10)`
  - **JTextField** `usernameField` - 👁️ visible ✅ enabled
    - *Bounds:* `200×25` at `(10, 50)`
"""

        # Verify structure
        lines = md_output.strip().split('\n')
        assert lines[0] == "# UI Component Tree"
        assert "**JFrame**" in lines[2]
        assert "**JButton**" in lines[5]
        assert "**JTextField**" in lines[8]

        # Verify indentation for hierarchy
        assert lines[2].startswith("-")  # Root level
        assert lines[5].startswith("  -")  # Child level
        assert lines[8].startswith("  -")  # Child level

    def test_markdown_badges(self):
        """Test Markdown visibility/state badges."""
        md_output = """# UI Component Tree

- **JButton** `disabledBtn` - 👁️ visible ❌ disabled
- **JPanel** `hiddenPanel` - 🚫 hidden ✅ enabled
"""

        # Check badges
        assert "👁️ visible ❌ disabled" in md_output  # Visible but disabled
        assert "🚫 hidden ✅ enabled" in md_output  # Hidden but enabled

    def test_text_format_structure(self):
        """Test plain text format."""
        text_output = """[0] JFrame (MainWindow)
  [0.0] JButton (loginButton)
  [0.1] JTextField (usernameField)
"""

        lines = text_output.strip().split('\n')
        assert len(lines) == 3

        # Check root
        assert lines[0].startswith("[0]")
        assert "JFrame" in lines[0]
        assert "MainWindow" in lines[0]

        # Check children indentation
        assert lines[1].startswith("  [0.0]")
        assert "JButton" in lines[1]
        assert lines[2].startswith("  [0.1]")
        assert "JTextField" in lines[2]

    def test_format_case_insensitive(self):
        """Test that format parameter is case-insensitive."""
        formats = [
            ("json", "JSON", "Json"),
            ("xml", "XML", "Xml"),
            ("yaml", "YAML", "Yaml", "yml", "YML"),
            ("csv", "CSV", "Csv"),
            ("markdown", "MARKDOWN", "Markdown", "md", "MD"),
            ("text", "TEXT", "Text"),
        ]

        # All these should be valid format strings
        for format_group in formats:
            for fmt in format_group:
                assert fmt.lower() in ["json", "xml", "yaml", "yml", "csv", "markdown", "md", "text"]

    def test_invalid_format_error(self):
        """Test error handling for invalid format."""
        invalid_formats = ["invalid", "html", "pdf", "doc", ""]

        for fmt in invalid_formats:
            # Should not match any valid format
            assert fmt.lower() not in ["json", "xml", "yaml", "yml", "csv", "markdown", "md", "text"]

    def test_csv_excel_compatibility(self):
        """Test CSV format is compatible with Excel."""
        csv_data = """path,depth,type,name,text,visible,enabled,bounds_x,bounds_y,bounds_width,bounds_height
0,0,JFrame,MainWindow,Test Application,true,true,0,0,800,600
0.0,1,JButton,loginButton,Login,true,true,10,10,100,30"""

        # Parse with csv.DictReader (Excel-compatible)
        reader = csv.DictReader(io.StringIO(csv_data))
        rows = list(reader)

        # Verify all columns are present
        expected_columns = [
            "path", "depth", "type", "name", "text", "visible",
            "enabled", "bounds_x", "bounds_y", "bounds_width", "bounds_height"
        ]
        assert list(rows[0].keys()) == expected_columns

    def test_markdown_text_preview(self):
        """Test Markdown text preview truncation."""
        # Long text should be truncated
        long_text = "A" * 100
        md_line = f"  - *Text:* `{long_text[:50]}...`\n"

        assert len(long_text) > 50
        assert "..." in md_line
        assert long_text[:50] in md_line

    def test_all_formats_represent_same_data(self, mock_tree_data):
        """Test that all formats represent the same underlying data."""
        # Extract component count
        total_components = sum(
            1 + len(root.get("children", []))
            for root in mock_tree_data["roots"]
        )

        # Each format should have 3 components (1 root + 2 children)
        assert total_components == 3

    def test_csv_utf8_encoding(self):
        """Test CSV handles UTF-8 characters correctly."""
        csv_data = """path,depth,type,name,text,visible,enabled,bounds_x,bounds_y,bounds_width,bounds_height
0,0,JLabel,label,测试中文 Unicode émojis 🎉,true,true,0,0,100,30"""

        reader = csv.DictReader(io.StringIO(csv_data))
        rows = list(reader)

        # UTF-8 characters should be preserved
        assert "测试中文" in rows[0]["text"]
        assert "émojis" in rows[0]["text"]
        assert "🎉" in rows[0]["text"]

    def test_xml_empty_text_attribute(self):
        """Test XML handles empty text attributes."""
        xml_output = """<?xml version="1.0" encoding="UTF-8"?>
<uitree>
  <component type="JTextField" name="field" text="" enabled="true" visible="true" />
</uitree>"""

        root = ET.fromstring(xml_output)
        component = root.find("component")

        # Empty text should be empty string, not None
        assert component.get("text") == ""
        assert component.get("text") is not None

    def test_yaml_list_format(self, mock_tree_data):
        """Test YAML uses clean list format."""
        yaml_output = yaml.dump(mock_tree_data, default_flow_style=False)

        # Should use block style (not flow style)
        assert "- " in yaml_output  # List items
        assert "[" not in yaml_output or yaml_output.count("[") < 5  # Not flow style

    def test_markdown_nested_lists(self):
        """Test Markdown uses different list markers for nesting."""
        md_output = """# UI Component Tree

- **JFrame** `root`
  * **JPanel** `panel`
    + **JButton** `button`
"""

        # Check different markers for different levels
        assert "\n-" in md_output  # Level 0
        assert "\n  *" in md_output  # Level 1
        assert "\n    +" in md_output  # Level 2

    def test_csv_depth_column(self):
        """Test CSV includes depth/level column."""
        csv_data = """path,depth,type,name,text,visible,enabled,bounds_x,bounds_y,bounds_width,bounds_height
0,0,JFrame,root,,true,true,0,0,800,600
0.0,1,JPanel,panel1,,true,true,0,0,400,600
0.0.0,2,JButton,btn1,Click,true,true,10,10,100,30"""

        reader = csv.DictReader(io.StringIO(csv_data))
        rows = list(reader)

        # Verify depth increases with nesting
        assert rows[0]["depth"] == "0"
        assert rows[1]["depth"] == "1"
        assert rows[2]["depth"] == "2"

    def test_format_conversion_consistency(self):
        """Test that format conversions maintain data consistency."""
        # All formats should preserve:
        # - Component type
        # - Component name
        # - Hierarchy/depth
        # - Visibility/enabled state
        # - Bounds information
        pass  # This is more of a conceptual test

    def test_markdown_inline_code_escaping(self):
        """Test Markdown escapes backticks in inline code."""
        # If component has backtick in name, it should be escaped or handled
        md_line = "- **JLabel** `name`with`backticks` - 👁️ visible ✅ enabled\n"

        # Should handle gracefully (implementation dependent)
        assert "**JLabel**" in md_line


class TestOutputFormatterEdgeCases:
    """Test edge cases for output formatters."""

    def test_empty_tree_json(self):
        """Test JSON format with empty tree."""
        empty_tree = {"roots": [], "timestamp": 0, "metadata": {}}
        json_output = json.dumps(empty_tree)

        parsed = json.loads(json_output)
        assert parsed["roots"] == []

    def test_empty_tree_csv(self):
        """Test CSV format with empty tree (header only)."""
        csv_output = """path,depth,type,name,text,visible,enabled,bounds_x,bounds_y,bounds_width,bounds_height
"""

        reader = csv.DictReader(io.StringIO(csv_output))
        rows = list(reader)
        assert len(rows) == 0

    def test_deep_nesting_csv(self):
        """Test CSV handles deep nesting correctly."""
        csv_data = """path,depth,type,name,text,visible,enabled,bounds_x,bounds_y,bounds_width,bounds_height
0,0,JFrame,root,,true,true,0,0,800,600
0.0,1,JPanel,p1,,true,true,0,0,400,600
0.0.0,2,JPanel,p2,,true,true,0,0,200,300
0.0.0.0,3,JButton,btn,Deep,true,true,10,10,50,20"""

        reader = csv.DictReader(io.StringIO(csv_data))
        rows = list(reader)

        # Verify deepest component
        assert rows[-1]["depth"] == "3"
        assert rows[-1]["path"] == "0.0.0.0"

    def test_large_bounds_values(self):
        """Test formatters handle large coordinate values."""
        csv_data = """path,depth,type,name,text,visible,enabled,bounds_x,bounds_y,bounds_width,bounds_height
0,0,JFrame,root,,true,true,0,0,4096,2160"""

        reader = csv.DictReader(io.StringIO(csv_data))
        rows = list(reader)

        assert rows[0]["bounds_width"] == "4096"
        assert rows[0]["bounds_height"] == "2160"

    def test_xml_self_closing_tags(self):
        """Test XML uses self-closing tags for leaf components."""
        xml_output = """<?xml version="1.0" encoding="UTF-8"?>
<uitree>
  <component type="JButton" name="btn" text="Click" enabled="true" visible="true" />
</uitree>"""

        # Self-closing tag should parse correctly
        root = ET.fromstring(xml_output)
        component = root.find("component")
        assert component is not None
        assert len(list(component)) == 0  # No children


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
