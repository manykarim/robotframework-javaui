#!/usr/bin/env python3
"""
Example Python usage of SwingLibrary.

This demonstrates how to use SwingLibrary directly from Python
without Robot Framework.
"""

from swing_library import SwingLibrary, SwingElement


def basic_example():
    """Basic usage example."""
    # Initialize library
    lib = SwingLibrary(timeout=10.0)

    # Connect to application
    lib.connect_to_application(main_class="com.example.demo.SwingDemoApp")

    try:
        # Find elements
        button = lib.find_element("JButton#loginBtn")
        print(f"Found button: {button.simple_class_name} - {button.text}")

        # Click button
        lib.click("JButton#loginBtn")

        # Input text
        lib.input_text("JTextField#username", "testuser")

        # Get element text
        status = lib.get_element_text("JLabel#statusLabel")
        print(f"Status: {status}")

    finally:
        lib.disconnect()


def table_example():
    """Table operations example."""
    lib = SwingLibrary()
    lib.connect_to_application(main_class="com.example.demo.SwingDemoApp")

    try:
        # Select table tab
        lib.select_tab("JTabbedPane", "Data Table")

        # Get table info
        row_count = lib.get_table_row_count("JTable#dataTable")
        print(f"Table has {row_count} rows")

        # Read cells
        for row in range(min(5, row_count)):
            value = lib.get_table_cell_value("JTable#dataTable", row, 0)
            print(f"Row {row}: {value}")

        # Select cell
        lib.select_table_cell("JTable#dataTable", 0, 0)

    finally:
        lib.disconnect()


def tree_example():
    """Tree operations example."""
    lib = SwingLibrary()
    lib.connect_to_application(main_class="com.example.demo.SwingDemoApp")

    try:
        # Select tree tab
        lib.select_tab("JTabbedPane", "Tree View")

        # Expand nodes
        lib.expand_tree_node("JTree#fileTree", "Root")
        lib.expand_tree_node("JTree#fileTree", "Root/Documents")

        # Select node
        lib.select_tree_node("JTree#fileTree", "Root/Documents/file.txt")

        # Collapse
        lib.collapse_tree_node("JTree#fileTree", "Root/Documents")

    finally:
        lib.disconnect()


def find_multiple_elements():
    """Finding multiple elements example."""
    lib = SwingLibrary()
    lib.connect_to_application(main_class="com.example.demo.SwingDemoApp")

    try:
        # Find all buttons
        buttons = lib.find_elements("JButton")
        print(f"Found {len(buttons)} buttons:")
        for btn in buttons:
            print(f"  - {btn.name}: {btn.text}")

        # Find all text fields
        fields = lib.find_elements("JTextField")
        print(f"Found {len(fields)} text fields")

    finally:
        lib.disconnect()


def wait_operations():
    """Wait operations example."""
    lib = SwingLibrary(timeout=15.0)
    lib.connect_to_application(main_class="com.example.demo.SwingDemoApp")

    try:
        # Wait for element to exist
        element = lib.wait_for_element("JButton#loginBtn", timeout=10.0)
        print(f"Element appeared: {element.simple_class_name}")

        # Wait until visible
        lib.wait_until_element_visible("JButton#loginBtn", timeout=5.0)
        print("Button is visible")

        # Wait until enabled
        lib.wait_until_element_enabled("JButton#loginBtn", timeout=5.0)
        print("Button is enabled")

    finally:
        lib.disconnect()


def ui_tree_inspection():
    """UI tree inspection example."""
    lib = SwingLibrary()
    lib.connect_to_application(main_class="com.example.demo.SwingDemoApp")

    try:
        # Get tree as JSON
        json_tree = lib.get_component_tree(format="json")
        print("JSON Tree:")
        print(json_tree[:500] + "..." if len(json_tree) > 500 else json_tree)

        # Get tree as text
        text_tree = lib.get_component_tree(format="text", max_depth=3)
        print("\nText Tree (depth=3):")
        print(text_tree)

    finally:
        lib.disconnect()


def screenshot_example():
    """Screenshot capture example."""
    lib = SwingLibrary()
    lib.connect_to_application(main_class="com.example.demo.SwingDemoApp")

    try:
        # Capture full window
        path1 = lib.capture_screenshot(filename="full_window.png")
        print(f"Full screenshot saved: {path1}")

        # Capture specific element
        path2 = lib.capture_screenshot(locator="JTabbedPane#mainTabs")
        print(f"Element screenshot saved: {path2}")

    finally:
        lib.disconnect()


def element_properties():
    """Element property access example."""
    lib = SwingLibrary()
    lib.connect_to_application(main_class="com.example.demo.SwingDemoApp")

    try:
        # Find element
        button = lib.find_element("JButton#loginBtn")

        # Access properties
        print(f"ID: {button.id}")
        print(f"Class: {button.class_name}")
        print(f"Simple class: {button.simple_class_name}")
        print(f"Name: {button.name}")
        print(f"Text: {button.text}")
        print(f"Visible: {button.is_visible}")
        print(f"Enabled: {button.is_enabled}")
        print(f"Bounds: {button.bounds}")

        # Get specific property
        tooltip = button.get_property("toolTipText")
        print(f"Tooltip: {tooltip}")

        # Get all properties
        all_props = button.get_all_properties()
        print(f"All properties: {list(all_props.keys())}")

    finally:
        lib.disconnect()


def list_running_apps():
    """List running Java applications."""
    lib = SwingLibrary()

    apps = lib.list_applications()
    print(f"Found {len(apps)} Java applications:")
    for app in apps:
        print(f"  PID {app['pid']}: {app['main_class']}")


if __name__ == "__main__":
    print("SwingLibrary Python Examples")
    print("============================")
    print()
    print("Run one of the following functions:")
    print("  - basic_example()")
    print("  - table_example()")
    print("  - tree_example()")
    print("  - find_multiple_elements()")
    print("  - wait_operations()")
    print("  - ui_tree_inspection()")
    print("  - screenshot_example()")
    print("  - element_properties()")
    print("  - list_running_apps()")
