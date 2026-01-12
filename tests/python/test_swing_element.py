"""
Unit tests for SwingElement class.
"""

import pytest
from unittest.mock import Mock, patch
import sys
import os

# Add parent directory to path for imports
sys.path.insert(0, os.path.dirname(__file__))

from conftest import MockSwingElement


class TestSwingElementProperties:
    """Test SwingElement property access."""

    def test_element_id(self, mock_button):
        """Test getting element ID."""
        from conftest import MockSwingElement

        elem = MockSwingElement(id=42)
        assert elem.id == 42

    def test_element_class_name(self, mock_button):
        """Test getting element class name."""
        assert mock_button.class_name == "javax.swing.JButton"

    def test_element_simple_class_name(self, mock_button):
        """Test getting element simple class name."""
        assert mock_button.simple_class_name == "JButton"

    def test_element_name(self, mock_button):
        """Test getting element name."""
        assert mock_button.name == "submitBtn"

    def test_element_text(self, mock_button):
        """Test getting element text."""
        assert mock_button.text == "Submit"

    def test_element_is_visible(self, mock_element):
        """Test element visibility property."""
        assert mock_element.is_visible is True

    def test_element_is_hidden(self, mock_hidden_element):
        """Test hidden element visibility property."""
        assert mock_hidden_element.is_visible is False

    def test_element_is_enabled(self, mock_element):
        """Test element enabled property."""
        assert mock_element.is_enabled is True

    def test_element_is_disabled(self, mock_disabled_element):
        """Test disabled element enabled property."""
        assert mock_disabled_element.is_enabled is False

    def test_element_bounds(self, mock_element):
        """Test element bounds property."""
        bounds = mock_element.bounds
        assert "x" in bounds
        assert "y" in bounds
        assert "width" in bounds
        assert "height" in bounds
        assert isinstance(bounds["x"], int)


class TestSwingElementPropertyAccess:
    """Test SwingElement get_property methods."""

    def test_get_property_exists(self, mock_button):
        """Test getting existing property."""
        assert mock_button.get_property("mnemonic") == "S"

    def test_get_property_not_exists(self, mock_button):
        """Test getting non-existent property."""
        assert mock_button.get_property("nonexistent") is None

    def test_get_tooltip(self, mock_button):
        """Test getting tooltip property."""
        assert mock_button.get_property("toolTipText") == "Click to submit"

    def test_get_all_properties(self, mock_button):
        """Test getting all properties."""
        props = mock_button.get_all_properties()
        assert isinstance(props, dict)
        assert "mnemonic" in props
        assert "toolTipText" in props

    def test_get_text_field_properties(self, mock_text_field):
        """Test text field specific properties."""
        assert mock_text_field.get_property("columns") == 20
        assert mock_text_field.get_property("editable") is True

    def test_get_table_properties(self, mock_table):
        """Test table specific properties."""
        assert mock_table.get_property("rowCount") == 10
        assert mock_table.get_property("columnCount") == 5


class TestSwingElementActions:
    """Test SwingElement action methods."""

    def test_click(self, mock_button):
        """Test clicking element."""
        mock_button.click()  # Should not raise

    def test_double_click(self, mock_button):
        """Test double-clicking element."""
        mock_button.double_click()  # Should not raise

    def test_right_click(self, mock_button):
        """Test right-clicking element."""
        mock_button.right_click()  # Should not raise

    def test_input_text(self, mock_text_field):
        """Test inputting text into element."""
        mock_text_field.input_text("test input")  # Should not raise

    def test_clear_text(self, mock_text_field):
        """Test clearing text from element."""
        mock_text_field.clear_text()  # Should not raise


class TestSwingElementCreation:
    """Test SwingElement creation with various configurations."""

    def test_create_with_minimal_info(self):
        """Test creating element with minimal information."""
        elem = MockSwingElement()
        assert elem.id == 1
        assert elem.class_name == "javax.swing.JButton"

    def test_create_with_full_info(self):
        """Test creating element with full information."""
        elem = MockSwingElement(
            id=100,
            class_name="javax.swing.JLabel",
            name="statusLabel",
            text="Status: Ready",
            visible=True,
            enabled=True,
            bounds={"x": 10, "y": 20, "width": 200, "height": 25},
            properties={"font": "Arial", "foreground": "black"},
        )
        assert elem.id == 100
        assert elem.class_name == "javax.swing.JLabel"
        assert elem.simple_class_name == "JLabel"
        assert elem.name == "statusLabel"
        assert elem.text == "Status: Ready"
        assert elem.is_visible is True
        assert elem.is_enabled is True
        assert elem.bounds["width"] == 200

    def test_create_element_no_name(self):
        """Test creating element without name."""
        elem = MockSwingElement(name=None)
        assert elem.name is None

    def test_create_element_no_text(self):
        """Test creating element without text."""
        elem = MockSwingElement(text=None)
        assert elem.text is None


class TestSwingElementTypes:
    """Test different Swing element types."""

    def test_button_element(self):
        """Test button element configuration."""
        elem = MockSwingElement(
            class_name="javax.swing.JButton",
            name="okBtn",
            text="OK",
        )
        assert elem.simple_class_name == "JButton"
        assert elem.text == "OK"

    def test_text_field_element(self):
        """Test text field element configuration."""
        elem = MockSwingElement(
            class_name="javax.swing.JTextField",
            name="searchField",
            text="",
        )
        assert elem.simple_class_name == "JTextField"

    def test_label_element(self):
        """Test label element configuration."""
        elem = MockSwingElement(
            class_name="javax.swing.JLabel",
            name="infoLabel",
            text="Information",
        )
        assert elem.simple_class_name == "JLabel"

    def test_combo_box_element(self):
        """Test combo box element configuration."""
        elem = MockSwingElement(
            class_name="javax.swing.JComboBox",
            name="countryCombo",
            properties={"selectedIndex": 0, "itemCount": 5},
        )
        assert elem.simple_class_name == "JComboBox"
        assert elem.get_property("itemCount") == 5

    def test_table_element(self):
        """Test table element configuration."""
        elem = MockSwingElement(
            class_name="javax.swing.JTable",
            name="dataTable",
            text=None,
            properties={"rowCount": 100, "columnCount": 10, "selectedRow": -1},
        )
        assert elem.simple_class_name == "JTable"
        assert elem.get_property("rowCount") == 100

    def test_tree_element(self):
        """Test tree element configuration."""
        elem = MockSwingElement(
            class_name="javax.swing.JTree",
            name="fileTree",
            text=None,
            properties={"rowCount": 50, "expanded": True},
        )
        assert elem.simple_class_name == "JTree"
        assert elem.get_property("expanded") is True

    def test_list_element(self):
        """Test list element configuration."""
        elem = MockSwingElement(
            class_name="javax.swing.JList",
            name="itemList",
            text=None,
            properties={"selectedIndex": 2, "model.size": 10},
        )
        assert elem.simple_class_name == "JList"
        assert elem.get_property("selectedIndex") == 2

    def test_tabbed_pane_element(self):
        """Test tabbed pane element configuration."""
        elem = MockSwingElement(
            class_name="javax.swing.JTabbedPane",
            name="mainTabs",
            text=None,
            properties={"tabCount": 5, "selectedIndex": 0},
        )
        assert elem.simple_class_name == "JTabbedPane"
        assert elem.get_property("tabCount") == 5


class TestSwingElementBounds:
    """Test SwingElement bounds and positioning."""

    def test_default_bounds(self, mock_element):
        """Test default bounds values."""
        bounds = mock_element.bounds
        assert bounds["x"] == 100
        assert bounds["y"] == 100
        assert bounds["width"] == 80
        assert bounds["height"] == 30

    def test_custom_bounds(self):
        """Test custom bounds values."""
        elem = MockSwingElement(
            bounds={"x": 50, "y": 75, "width": 150, "height": 40}
        )
        bounds = elem.bounds
        assert bounds["x"] == 50
        assert bounds["y"] == 75
        assert bounds["width"] == 150
        assert bounds["height"] == 40

    def test_bounds_are_integers(self, mock_element):
        """Test that bounds values are integers."""
        bounds = mock_element.bounds
        for key in ["x", "y", "width", "height"]:
            assert isinstance(bounds[key], int)
