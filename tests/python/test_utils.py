"""
Test utilities and helper functions.
"""

import pytest
from typing import Dict, List, Any, Optional
import json
import re


class LocatorParser:
    """
    Utility class for parsing and validating locators.
    Used for testing locator syntax handling.
    """

    @staticmethod
    def is_xpath(locator: str) -> bool:
        """Check if locator is XPath-style."""
        return locator.startswith("//") or locator.startswith("/")

    @staticmethod
    def is_css(locator: str) -> bool:
        """Check if locator is CSS-style."""
        return not LocatorParser.is_xpath(locator)

    @staticmethod
    def extract_type(locator: str) -> Optional[str]:
        """Extract component type from locator."""
        if LocatorParser.is_xpath(locator):
            # XPath style: //JButton or /JPanel/JButton
            stripped = locator.lstrip('/')
            match = re.match(r'^([\w]+)', stripped)
            return match.group(1) if match else None
        else:
            # CSS style: JButton or JButton#name or JButton[attr=val]
            match = re.match(r'^([\w]+)', locator)
            return match.group(1) if match else None

    @staticmethod
    def extract_id(locator: str) -> Optional[str]:
        """Extract ID from locator (e.g., JButton#myId -> myId)."""
        match = re.search(r'#([\w-]+)', locator)
        return match.group(1) if match else None

    @staticmethod
    def extract_attributes(locator: str) -> Dict[str, str]:
        """Extract attribute selectors from locator."""
        attrs = {}
        # Match [attr='value'] or [attr="value"]
        pattern = r"\[([\w-]+)=['\"]([^'\"]+)['\"]\]"
        for match in re.finditer(pattern, locator):
            attrs[match.group(1)] = match.group(2)
        return attrs

    @staticmethod
    def extract_pseudos(locator: str) -> List[str]:
        """Extract pseudo selectors from locator."""
        pseudos = []
        # Match :pseudo or :pseudo(arg)
        pattern = r":([\w-]+(?:\([^)]*\))?)"
        for match in re.finditer(pattern, locator):
            pseudos.append(match.group(1))
        return pseudos

    @staticmethod
    def has_combinator(locator: str) -> bool:
        """Check if locator has combinators (> or space)."""
        # Check for child combinator or descendant combinator
        return " > " in locator or (
            " " in locator and not locator.startswith("//")
        )


class ComponentTreeBuilder:
    """
    Utility class for building mock component trees.
    """

    def __init__(self):
        self.components = []

    def add_frame(self, name: str, title: str = "Main Window") -> "ComponentTreeBuilder":
        """Add a JFrame to the tree."""
        self.components.append({
            "type": "JFrame",
            "name": name,
            "title": title,
            "children": []
        })
        return self

    def add_panel(self, name: str, parent: str = None) -> "ComponentTreeBuilder":
        """Add a JPanel to the tree."""
        panel = {"type": "JPanel", "name": name, "children": []}
        self._add_to_parent(panel, parent)
        return self

    def add_button(self, name: str, text: str, parent: str = None) -> "ComponentTreeBuilder":
        """Add a JButton to the tree."""
        button = {"type": "JButton", "name": name, "text": text}
        self._add_to_parent(button, parent)
        return self

    def add_text_field(self, name: str, text: str = "", parent: str = None) -> "ComponentTreeBuilder":
        """Add a JTextField to the tree."""
        field = {"type": "JTextField", "name": name, "text": text}
        self._add_to_parent(field, parent)
        return self

    def add_label(self, name: str, text: str, parent: str = None) -> "ComponentTreeBuilder":
        """Add a JLabel to the tree."""
        label = {"type": "JLabel", "name": name, "text": text}
        self._add_to_parent(label, parent)
        return self

    def _add_to_parent(self, component: Dict, parent_name: str):
        """Add component to parent or root."""
        if parent_name:
            parent = self._find_component(parent_name)
            if parent and "children" in parent:
                parent["children"].append(component)
        else:
            # Add to last frame's children or root
            if self.components and self.components[-1].get("type") == "JFrame":
                self.components[-1]["children"].append(component)
            else:
                self.components.append(component)

    def _find_component(self, name: str) -> Optional[Dict]:
        """Find component by name."""
        def search(components):
            for comp in components:
                if comp.get("name") == name:
                    return comp
                if "children" in comp:
                    found = search(comp["children"])
                    if found:
                        return found
            return None
        return search(self.components)

    def to_json(self) -> str:
        """Convert tree to JSON string."""
        return json.dumps(self.components, indent=2)

    def to_dict(self) -> List[Dict]:
        """Get tree as dictionary."""
        return self.components


class TestLocatorParser:
    """Tests for LocatorParser utility."""

    def test_is_xpath_double_slash(self):
        """Test XPath detection with //."""
        assert LocatorParser.is_xpath("//JButton") is True

    def test_is_xpath_single_slash(self):
        """Test XPath detection with /."""
        assert LocatorParser.is_xpath("/JPanel/JButton") is True

    def test_is_css_simple(self):
        """Test CSS detection for simple locator."""
        assert LocatorParser.is_css("JButton") is True
        assert LocatorParser.is_xpath("JButton") is False

    def test_extract_type_css(self):
        """Test extracting type from CSS locator."""
        assert LocatorParser.extract_type("JButton#submit") == "JButton"
        assert LocatorParser.extract_type("JTextField[text='']") == "JTextField"

    def test_extract_type_xpath(self):
        """Test extracting type from XPath locator."""
        assert LocatorParser.extract_type("//JButton") == "JButton"
        assert LocatorParser.extract_type("/JPanel/JButton") == "JPanel"

    def test_extract_id(self):
        """Test extracting ID from locator."""
        assert LocatorParser.extract_id("JButton#submit") == "submit"
        assert LocatorParser.extract_id("#loginBtn") == "loginBtn"
        assert LocatorParser.extract_id("JButton") is None

    def test_extract_attributes(self):
        """Test extracting attributes from locator."""
        attrs = LocatorParser.extract_attributes("[text='Save'][enabled='true']")
        assert attrs == {"text": "Save", "enabled": "true"}

    def test_extract_pseudos(self):
        """Test extracting pseudo selectors."""
        pseudos = LocatorParser.extract_pseudos("JButton:enabled:visible")
        assert "enabled" in pseudos
        assert "visible" in pseudos

    def test_extract_nth_child_pseudo(self):
        """Test extracting nth-child pseudo."""
        pseudos = LocatorParser.extract_pseudos("JButton:nth-child(2)")
        assert "nth-child(2)" in pseudos

    def test_has_combinator_child(self):
        """Test detecting child combinator."""
        assert LocatorParser.has_combinator("JPanel > JButton") is True

    def test_has_combinator_descendant(self):
        """Test detecting descendant combinator."""
        assert LocatorParser.has_combinator("JFrame JButton") is True

    def test_no_combinator(self):
        """Test no combinator detection."""
        assert LocatorParser.has_combinator("JButton#submit") is False


class TestComponentTreeBuilder:
    """Tests for ComponentTreeBuilder utility."""

    def test_build_simple_tree(self):
        """Test building a simple component tree."""
        builder = ComponentTreeBuilder()
        builder.add_frame("mainFrame", "My App")
        builder.add_panel("contentPane")
        builder.add_button("okBtn", "OK", "contentPane")

        tree = builder.to_dict()
        assert len(tree) == 1
        assert tree[0]["type"] == "JFrame"

    def test_to_json(self):
        """Test converting tree to JSON."""
        builder = ComponentTreeBuilder()
        builder.add_frame("mainFrame", "Test")
        builder.add_button("btn", "Click")

        json_str = builder.to_json()
        assert "JFrame" in json_str
        assert "mainFrame" in json_str

    def test_nested_components(self):
        """Test nested component structure."""
        builder = ComponentTreeBuilder()
        builder.add_frame("main", "Main")
        builder.add_panel("panel1")
        builder.add_panel("panel2", "panel1")
        builder.add_button("btn", "OK", "panel2")

        tree = builder.to_dict()
        frame = tree[0]
        assert len(frame["children"]) > 0


# Test data generators

def generate_locators(count: int = 10) -> List[str]:
    """Generate test locators."""
    types = ["JButton", "JTextField", "JLabel", "JPanel", "JTable", "JTree", "JList"]
    names = ["submit", "cancel", "save", "load", "refresh", "status", "data"]
    locators = []

    for i in range(count):
        comp_type = types[i % len(types)]
        name = names[i % len(names)]
        locators.append(f"{comp_type}#{name}{i}")

    return locators


def generate_element_data(count: int = 10) -> List[Dict[str, Any]]:
    """Generate test element data."""
    elements = []
    types = ["JButton", "JTextField", "JLabel", "JPanel"]

    for i in range(count):
        elements.append({
            "id": i,
            "type": types[i % len(types)],
            "name": f"element_{i}",
            "text": f"Text {i}",
            "visible": True,
            "enabled": i % 2 == 0,
            "bounds": {"x": i * 10, "y": i * 10, "width": 100, "height": 30}
        })

    return elements


class TestDataGenerators:
    """Test data generator utilities."""

    def test_generate_locators(self):
        """Test locator generation."""
        locators = generate_locators(5)
        assert len(locators) == 5
        assert all("#" in loc for loc in locators)

    def test_generate_element_data(self):
        """Test element data generation."""
        elements = generate_element_data(5)
        assert len(elements) == 5
        assert all("id" in e for e in elements)
        assert all("type" in e for e in elements)
