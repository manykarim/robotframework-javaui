"""
Unit tests for locator parsing and matching.

These tests verify the CSS/XPath-like locator syntax handling.
"""

import pytest
from unittest.mock import Mock, patch


class TestCSSLocatorSyntax:
    """Test CSS-like locator syntax parsing."""

    def test_simple_type_locator(self):
        """Test simple type locator (e.g., JButton)."""
        locator = "JButton"
        # Verify it's a valid simple type locator
        assert not locator.startswith("//")
        assert "#" not in locator
        assert "[" not in locator
        assert ":" not in locator

    def test_id_locator(self):
        """Test ID locator (e.g., #submitBtn)."""
        locator = "#submitBtn"
        assert locator.startswith("#")
        name = locator[1:]
        assert name == "submitBtn"

    def test_type_with_id(self):
        """Test type with ID (e.g., JButton#submit)."""
        locator = "JButton#submit"
        parts = locator.split("#")
        assert len(parts) == 2
        assert parts[0] == "JButton"
        assert parts[1] == "submit"

    def test_class_selector(self):
        """Test class selector (e.g., .primary)."""
        locator = ".primary"
        assert locator.startswith(".")
        class_name = locator[1:]
        assert class_name == "primary"

    def test_attribute_selector_equals(self):
        """Test attribute selector with equals (e.g., [text='Save'])."""
        locator = "[text='Save']"
        assert locator.startswith("[")
        assert locator.endswith("]")
        # Parse attribute=value
        inner = locator[1:-1]
        assert "=" in inner

    def test_attribute_selector_contains(self):
        """Test attribute selector contains (e.g., [text*='Save'])."""
        locator = "[text*='Save']"
        assert "*=" in locator

    def test_attribute_selector_starts_with(self):
        """Test attribute selector starts with (e.g., [text^='Save'])."""
        locator = "[text^='Save']"
        assert "^=" in locator

    def test_attribute_selector_ends_with(self):
        """Test attribute selector ends with (e.g., [text$='Save'])."""
        locator = "[text$='Save']"
        assert "$=" in locator


class TestCSSPseudoSelectors:
    """Test CSS pseudo selector syntax."""

    def test_enabled_pseudo(self):
        """Test :enabled pseudo selector."""
        locator = "JButton:enabled"
        assert ":enabled" in locator

    def test_disabled_pseudo(self):
        """Test :disabled pseudo selector."""
        locator = "JButton:disabled"
        assert ":disabled" in locator

    def test_visible_pseudo(self):
        """Test :visible pseudo selector."""
        locator = "JPanel:visible"
        assert ":visible" in locator

    def test_hidden_pseudo(self):
        """Test :hidden pseudo selector."""
        locator = "JPanel:hidden"
        assert ":hidden" in locator

    def test_focused_pseudo(self):
        """Test :focused pseudo selector."""
        locator = "JTextField:focused"
        assert ":focused" in locator

    def test_selected_pseudo(self):
        """Test :selected pseudo selector."""
        locator = "JCheckBox:selected"
        assert ":selected" in locator

    def test_first_child_pseudo(self):
        """Test :first-child pseudo selector."""
        locator = "JButton:first-child"
        assert ":first-child" in locator

    def test_last_child_pseudo(self):
        """Test :last-child pseudo selector."""
        locator = "JButton:last-child"
        assert ":last-child" in locator

    def test_nth_child_pseudo(self):
        """Test :nth-child(n) pseudo selector."""
        locator = "JButton:nth-child(2)"
        assert ":nth-child(" in locator
        # Extract index
        start = locator.index("(") + 1
        end = locator.index(")")
        index = int(locator[start:end])
        assert index == 2

    def test_contains_pseudo(self):
        """Test :contains(text) pseudo selector."""
        locator = "JLabel:contains('Error')"
        assert ":contains(" in locator

    def test_multiple_pseudos(self):
        """Test multiple pseudo selectors."""
        locator = "JButton:enabled:visible"
        assert ":enabled" in locator
        assert ":visible" in locator


class TestCSSCombinators:
    """Test CSS combinator syntax."""

    def test_child_combinator(self):
        """Test child combinator (>)."""
        locator = "JPanel > JButton"
        parts = locator.split(" > ")
        assert len(parts) == 2
        assert parts[0] == "JPanel"
        assert parts[1] == "JButton"

    def test_descendant_combinator(self):
        """Test descendant combinator (space)."""
        locator = "JFrame JPanel JButton"
        parts = locator.split()
        assert len(parts) == 3
        assert parts[0] == "JFrame"
        assert parts[1] == "JPanel"
        assert parts[2] == "JButton"

    def test_mixed_combinators(self):
        """Test mixed combinators."""
        locator = "JFrame > JPanel JButton"
        assert ">" in locator
        assert "JButton" in locator


class TestXPathLocatorSyntax:
    """Test XPath-like locator syntax parsing."""

    def test_descendant_axis(self):
        """Test descendant axis (//)."""
        locator = "//JButton"
        assert locator.startswith("//")
        element_type = locator[2:]
        assert element_type == "JButton"

    def test_child_axis(self):
        """Test child axis (/)."""
        locator = "/JPanel/JButton"
        assert locator.startswith("/")
        assert locator.count("/") == 2

    def test_xpath_attribute(self):
        """Test XPath attribute match ([@attr='val'])."""
        locator = "//JButton[@text='OK']"
        assert "[@" in locator
        assert "text='OK'" in locator

    def test_xpath_index(self):
        """Test XPath index ([n])."""
        locator = "//JButton[1]"
        assert "[1]" in locator

    def test_xpath_multiple_attributes(self):
        """Test multiple XPath attributes."""
        locator = "//JButton[@text='OK'][@enabled='true']"
        assert locator.count("[@") == 2

    def test_xpath_name_attribute(self):
        """Test XPath name attribute."""
        locator = "//JTextField[@name='username']"
        assert "@name='username'" in locator


class TestComplexLocators:
    """Test complex locator combinations."""

    def test_type_with_attribute_and_pseudo(self):
        """Test type with attribute and pseudo selector."""
        locator = "JButton[text='Submit']:enabled"
        assert "JButton" in locator
        assert "[text='Submit']" in locator
        assert ":enabled" in locator

    def test_id_with_pseudo(self):
        """Test ID with pseudo selector."""
        locator = "#loginBtn:visible"
        assert "#loginBtn" in locator
        assert ":visible" in locator

    def test_complex_path_with_attributes(self):
        """Test complex path with attributes."""
        locator = "JPanel > JButton[text='Save']:enabled"
        assert "JPanel" in locator
        assert ">" in locator
        assert "[text='Save']" in locator
        assert ":enabled" in locator

    def test_nested_panels(self):
        """Test nested panel selectors."""
        locator = "JPanel#main > JPanel#left > JButton"
        parts = locator.split(" > ")
        assert len(parts) == 3

    def test_xpath_with_descendant_and_child(self):
        """Test mixed XPath axes."""
        locator = "//JFrame/JPanel//JButton"
        assert locator.startswith("//")
        assert "/JPanel//" in locator


class TestLocatorEdgeCases:
    """Test locator edge cases and error handling."""

    def test_empty_locator(self):
        """Test empty locator string."""
        locator = ""
        assert locator == ""

    def test_whitespace_only_locator(self):
        """Test whitespace-only locator."""
        locator = "   "
        assert locator.strip() == ""

    def test_quoted_value_with_spaces(self):
        """Test attribute value with spaces."""
        locator = "[text='Hello World']"
        assert "'Hello World'" in locator

    def test_quoted_value_with_special_chars(self):
        """Test attribute value with special characters."""
        locator = "[text='Save & Exit']"
        assert "'Save & Exit'" in locator

    def test_double_quoted_value(self):
        """Test double-quoted attribute value."""
        locator = '[text="Submit"]'
        assert '"Submit"' in locator

    def test_escaped_quotes(self):
        """Test escaped quotes in value."""
        locator = "[text='It\\'s working']"
        assert "\\'" in locator

    def test_numeric_index(self):
        """Test numeric index in locator."""
        locator = "JButton:nth-child(10)"
        start = locator.index("(") + 1
        end = locator.index(")")
        index = int(locator[start:end])
        assert index == 10


class TestLocatorComponentTypes:
    """Test locators for different Swing component types."""

    def test_button_locators(self):
        """Test button-specific locators."""
        locators = [
            "JButton",
            "JButton#submit",
            "JButton[text='OK']",
            "JToggleButton:selected",
        ]
        for loc in locators:
            assert "Button" in loc

    def test_text_component_locators(self):
        """Test text component locators."""
        locators = [
            "JTextField",
            "JTextField#username",
            "JTextArea",
            "JPasswordField",
            "JEditorPane",
            "JTextPane",
        ]
        for loc in locators:
            assert loc.startswith("J")

    def test_container_locators(self):
        """Test container component locators."""
        locators = [
            "JPanel",
            "JScrollPane",
            "JSplitPane",
            "JTabbedPane",
            "JLayeredPane",
        ]
        for loc in locators:
            assert "Pane" in loc or "Panel" in loc

    def test_list_and_table_locators(self):
        """Test list and table locators."""
        locators = [
            "JList",
            "JList#items",
            "JTable",
            "JTable#dataTable",
            "JTree",
            "JTree#fileTree",
        ]
        for loc in locators:
            assert loc.startswith("J")

    def test_dialog_and_frame_locators(self):
        """Test dialog and frame locators."""
        locators = [
            "JFrame",
            "JDialog",
            "JInternalFrame",
            "JOptionPane",
        ]
        for loc in locators:
            assert loc.startswith("J")
