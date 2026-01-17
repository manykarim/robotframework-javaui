package com.robotframework.swing;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import com.google.gson.JsonPrimitive;

import javax.imageio.ImageIO;
import javax.swing.*;
import javax.swing.table.TableModel;
import javax.swing.text.JTextComponent;
import javax.swing.tree.*;
import java.awt.*;
import java.awt.event.*;
import java.awt.image.BufferedImage;
import java.io.ByteArrayOutputStream;
import java.util.Base64;
import java.util.Enumeration;

/**
 * Executor for UI actions on Swing components.
 * All actions are executed on the Event Dispatch Thread.
 */
public class ActionExecutor {

    private static final Robot robot;

    static {
        Robot r = null;
        try {
            r = new Robot();
            r.setAutoDelay(50);
        } catch (AWTException e) {
            System.err.println("[SwingAgent] Failed to create Robot: " + e.getMessage());
        }
        robot = r;
    }

    /**
     * Click on a component.
     * Uses runOnEdtLater to avoid blocking on modal dialogs.
     */
    public static void click(int componentId) {
        // Get component from cache (non-EDT operation)
        Component component = ComponentInspector.getComponentById(componentId);
        if (component == null) {
            throw new IllegalArgumentException("Component not found: " + componentId);
        }

        // Perform entire click operation asynchronously to avoid blocking on modal dialogs
        EdtHelper.runOnEdtLater(() -> {
            // Verify component is showing before clicking
            if (!component.isShowing()) {
                System.err.println("[SwingAgent] Component not visible for click: " + componentId);
                return;
            }
            if (component instanceof AbstractButton) {
                ((AbstractButton) component).doClick();
            } else {
                performMouseClick(component, 1);
            }
        });

        // Give the click a moment to process
        EdtHelper.sleep(150);
    }

    /**
     * Double-click on a component.
     * Uses runOnEdtLater to avoid blocking on modal dialogs.
     */
    public static void doubleClick(int componentId) {
        // Get component from cache (non-EDT operation)
        Component component = ComponentInspector.getComponentById(componentId);
        if (component == null) {
            throw new IllegalArgumentException("Component not found: " + componentId);
        }

        // Perform double-click asynchronously to avoid blocking on modal dialogs
        EdtHelper.runOnEdtLater(() -> {
            // For JComponents inside scroll panes, scroll them into view first
            if (component instanceof javax.swing.JComponent && component.getParent() instanceof javax.swing.JViewport) {
                javax.swing.JComponent jcomp = (javax.swing.JComponent) component;
                java.awt.Rectangle bounds = jcomp.getBounds();
                jcomp.scrollRectToVisible(new java.awt.Rectangle(0, 0, bounds.width, Math.min(bounds.height, 20)));
            }
            performMouseClick(component, 2);
        });

        // Give the click a moment to process
        EdtHelper.sleep(150);
    }

    /**
     * Right-click on a component.
     * Dispatches synthetic mouse events with popupTrigger=true to trigger popup menus.
     * The test application's MouseListener checks isPopupTrigger() which returns true
     * when the MouseEvent constructor has popupTrigger=true.
     */
    public static void rightClick(int componentId) {
        // Get component on EDT - use runOnEdtAndReturn for synchronous access
        Component component = EdtHelper.runOnEdtAndReturn(() -> {
            Component c = getComponent(componentId);
            ensureVisible(c);
            return c;
        });

        // Dispatch synthetic mouse events asynchronously to avoid blocking
        // Both mousePressed and mouseReleased are checked for popup trigger
        // (Windows triggers on pressed, Linux/Mac on released)
        EdtHelper.runOnEdtLater(() -> {
            Point center = getComponentCenter(component);
            long time = System.currentTimeMillis();

            // MOUSE_PRESSED with popupTrigger=true
            MouseEvent pressEvent = new MouseEvent(
                component,
                MouseEvent.MOUSE_PRESSED,
                time,
                InputEvent.BUTTON3_DOWN_MASK,
                center.x, center.y,
                1,  // clickCount
                true,  // popupTrigger - THIS IS KEY
                MouseEvent.BUTTON3
            );
            component.dispatchEvent(pressEvent);
        });

        EdtHelper.sleep(50);

        EdtHelper.runOnEdtLater(() -> {
            Point center = getComponentCenter(component);
            long time = System.currentTimeMillis();

            // MOUSE_RELEASED with popupTrigger=true
            MouseEvent releaseEvent = new MouseEvent(
                component,
                MouseEvent.MOUSE_RELEASED,
                time,
                InputEvent.BUTTON3_DOWN_MASK,
                center.x, center.y,
                1,  // clickCount
                true,  // popupTrigger - THIS IS KEY
                MouseEvent.BUTTON3
            );
            component.dispatchEvent(releaseEvent);
        });

        // Give the popup time to appear
        EdtHelper.sleep(200);
    }

    /**
     * Type text into a component.
     */
    public static void typeText(int componentId, String text) {
        EdtHelper.runOnEdt(() -> {
            Component component = getComponent(componentId);
            ensureVisible(component);

            if (component instanceof JTextComponent) {
                JTextComponent textComp = (JTextComponent) component;
                textComp.requestFocusInWindow();
                EdtHelper.waitForEdt(500);

                // Append text at current caret position
                int caretPos = textComp.getCaretPosition();
                String currentText = textComp.getText();
                String newText = currentText.substring(0, caretPos) + text + currentText.substring(caretPos);
                textComp.setText(newText);
                textComp.setCaretPosition(caretPos + text.length());
            } else if (component instanceof JComboBox) {
                JComboBox<?> combo = (JComboBox<?>) component;
                if (combo.isEditable()) {
                    combo.setSelectedItem(text);
                }
            } else if (component instanceof JSpinner) {
                JSpinner spinner = (JSpinner) component;
                JComponent editor = spinner.getEditor();
                if (editor instanceof JSpinner.DefaultEditor) {
                    JTextField textField = ((JSpinner.DefaultEditor) editor).getTextField();
                    textField.requestFocusInWindow();
                    EdtHelper.waitForEdt(500);
                    // Set the text value directly
                    int caretPos = textField.getCaretPosition();
                    String currentText = textField.getText();
                    String newText = currentText.substring(0, caretPos) + text + currentText.substring(caretPos);
                    textField.setText(newText);
                    textField.setCaretPosition(caretPos + text.length());
                    // Try to commit the value
                    try {
                        spinner.commitEdit();
                    } catch (java.text.ParseException e) {
                        // Ignore parse exceptions - the text is in the field
                    }
                }
            } else {
                throw new IllegalArgumentException("Component does not support text input");
            }
        });
    }

    /**
     * Clear text from a component.
     */
    public static void clearText(int componentId) {
        EdtHelper.runOnEdt(() -> {
            Component component = getComponent(componentId);

            if (component instanceof JTextComponent) {
                ((JTextComponent) component).setText("");
            } else if (component instanceof JComboBox) {
                JComboBox<?> combo = (JComboBox<?>) component;
                if (combo.isEditable()) {
                    combo.setSelectedItem("");
                }
            } else if (component instanceof JSpinner) {
                JSpinner spinner = (JSpinner) component;
                JComponent editor = spinner.getEditor();
                if (editor instanceof JSpinner.DefaultEditor) {
                    JTextField textField = ((JSpinner.DefaultEditor) editor).getTextField();
                    textField.setText("");
                }
            } else {
                throw new IllegalArgumentException("Component does not support text clearing");
            }
        });
    }

    /**
     * Select an item from a list, combobox, or similar component.
     */
    public static void selectItem(int componentId, int index, String value) {
        EdtHelper.runOnEdt(() -> {
            Component component = getComponent(componentId);

            if (component instanceof JComboBox) {
                JComboBox<?> combo = (JComboBox<?>) component;
                if (index >= 0) {
                    combo.setSelectedIndex(index);
                } else if (value != null) {
                    for (int i = 0; i < combo.getItemCount(); i++) {
                        Object item = combo.getItemAt(i);
                        if (item != null && item.toString().equals(value)) {
                            combo.setSelectedIndex(i);
                            return;
                        }
                    }
                    throw new IllegalArgumentException("Item not found: " + value);
                }
            } else if (component instanceof JList) {
                JList<?> list = (JList<?>) component;
                if (index >= 0) {
                    list.setSelectedIndex(index);
                } else if (value != null) {
                    ListModel<?> model = list.getModel();
                    for (int i = 0; i < model.getSize(); i++) {
                        Object item = model.getElementAt(i);
                        if (item != null && item.toString().equals(value)) {
                            list.setSelectedIndex(i);
                            return;
                        }
                    }
                    throw new IllegalArgumentException("Item not found: " + value);
                }
            } else if (component instanceof JTabbedPane) {
                JTabbedPane tabs = (JTabbedPane) component;
                if (index >= 0) {
                    tabs.setSelectedIndex(index);
                } else if (value != null) {
                    for (int i = 0; i < tabs.getTabCount(); i++) {
                        if (value.equals(tabs.getTitleAt(i))) {
                            tabs.setSelectedIndex(i);
                            return;
                        }
                    }
                    throw new IllegalArgumentException("Tab not found: " + value);
                }
            } else {
                throw new IllegalArgumentException("Component does not support item selection");
            }
        });
    }

    /**
     * Select an item from a visible popup menu.
     *
     * @param path Menu path (e.g., "Copy" or "Edit|Paste Special")
     */
    public static void selectFromPopupMenu(String path) {
        String[] parts = path.split("\\|");
        if (parts.length == 0 || parts[0].isEmpty()) {
            throw new IllegalArgumentException("Empty popup menu path");
        }

        // Use arrays to capture results from EDT
        final String[] errorMessage = new String[1];
        final boolean[] actionCompleted = new boolean[1];

        // Retry loop - popup may take time to appear
        for (int attempt = 0; attempt < 10; attempt++) {
            // Reset for each attempt
            errorMessage[0] = null;
            actionCompleted[0] = false;

            // Schedule popup navigation on EDT
            EdtHelper.runOnEdtLater(() -> {
                // Find the visible popup menu
                JPopupMenu popupMenu = findVisiblePopupMenu();
                if (popupMenu == null) {
                    errorMessage[0] = "No popup menu is currently visible";
                    actionCompleted[0] = true;
                    return;
                }

                // Navigate the popup menu
                try {
                    navigatePopupMenu(popupMenu, parts);
                } catch (IllegalArgumentException e) {
                    errorMessage[0] = e.getMessage();
                }
                actionCompleted[0] = true;
            });

            // Wait for action to complete
            EdtHelper.sleep(100);

            // Check result
            if (actionCompleted[0]) {
                if (errorMessage[0] == null) {
                    // Success!
                    return;
                } else if (!errorMessage[0].contains("No popup menu is currently visible")) {
                    // Non-recoverable error (e.g., menu item not found)
                    throw new IllegalArgumentException(errorMessage[0]);
                }
                // Popup not visible yet, retry
            }

            // Wait a bit before retry
            EdtHelper.sleep(50);
        }

        // All retries exhausted
        throw new IllegalArgumentException("No popup menu is currently visible");
    }

    private static JPopupMenu findVisiblePopupMenu() {
        // Search for visible popup menus
        for (Window window : Window.getWindows()) {
            if (window instanceof JWindow || window instanceof JDialog) {
                JPopupMenu popup = findPopupInContainer(window);
                if (popup != null && popup.isVisible()) {
                    return popup;
                }
            }
        }

        // Also check MenuSelectionManager for active popups
        MenuElement[] selected = MenuSelectionManager.defaultManager().getSelectedPath();
        if (selected != null && selected.length > 0) {
            for (MenuElement elem : selected) {
                if (elem instanceof JPopupMenu) {
                    return (JPopupMenu) elem;
                }
            }
        }

        // Search all components for popup menus
        for (Window window : Window.getWindows()) {
            if (window.isVisible()) {
                JPopupMenu popup = findPopupRecursive(window);
                if (popup != null && popup.isVisible()) {
                    return popup;
                }
            }
        }

        return null;
    }

    private static JPopupMenu findPopupInContainer(Container container) {
        for (Component comp : container.getComponents()) {
            if (comp instanceof JPopupMenu && comp.isVisible()) {
                return (JPopupMenu) comp;
            }
            if (comp instanceof Container) {
                JPopupMenu popup = findPopupInContainer((Container) comp);
                if (popup != null) {
                    return popup;
                }
            }
        }
        return null;
    }

    private static JPopupMenu findPopupRecursive(Component comp) {
        if (comp instanceof JComponent) {
            JPopupMenu popup = ((JComponent) comp).getComponentPopupMenu();
            if (popup != null && popup.isVisible()) {
                return popup;
            }
        }
        if (comp instanceof Container) {
            for (Component child : ((Container) comp).getComponents()) {
                JPopupMenu popup = findPopupRecursive(child);
                if (popup != null) {
                    return popup;
                }
            }
        }
        return null;
    }

    private static void navigatePopupMenu(JPopupMenu popupMenu, String[] pathParts) {
        MenuElement currentMenu = popupMenu;

        for (int i = 0; i < pathParts.length; i++) {
            String itemName = pathParts[i].trim();
            JMenuItem foundItem = null;

            // Get menu items from current menu element
            MenuElement[] subElements;
            if (currentMenu instanceof JPopupMenu) {
                subElements = ((JPopupMenu) currentMenu).getSubElements();
            } else if (currentMenu instanceof JMenu) {
                subElements = ((JMenu) currentMenu).getPopupMenu().getSubElements();
            } else {
                subElements = currentMenu.getSubElements();
            }

            // Search for the item
            for (MenuElement elem : subElements) {
                if (elem instanceof JMenuItem) {
                    JMenuItem item = (JMenuItem) elem;
                    if (itemName.equals(item.getText())) {
                        foundItem = item;
                        break;
                    }
                }
            }

            if (foundItem == null) {
                // Close popup and throw
                MenuSelectionManager.defaultManager().clearSelectedPath();
                throw new IllegalArgumentException("Popup menu item not found: " + itemName);
            }

            // If this is a submenu and not the last item, navigate into it
            if (foundItem instanceof JMenu && i < pathParts.length - 1) {
                currentMenu = (JMenu) foundItem;
                // Hover to open submenu
                foundItem.setArmed(true);
                EdtHelper.waitForEdt(100);
            } else {
                // Click the item
                foundItem.doClick();
                break;
            }
        }
    }

    /**
     * Select a menu item by path.
     * Path format: "File|New" or "Edit|Find|Find Next"
     *
     * @param path Menu path separated by | (pipe)
     */
    public static void selectMenu(String path) {
        selectMenu(path, 5000); // Default 5 second timeout
    }

    /**
     * Select a menu item by path with configurable timeout.
     * Path format: "File|New" or "Edit|Find|Find Next"
     *
     * @param path Menu path separated by | (pipe)
     * @param timeoutMs Timeout in milliseconds for menu operations
     */
    public static void selectMenu(String path, int timeoutMs) {
        String[] parts = path.split("\\|");
        if (parts.length == 0) {
            throw new IllegalArgumentException("Empty menu path");
        }

        long startTime = System.currentTimeMillis();

        // Find the menu bar and navigate synchronously to properly propagate errors
        EdtHelper.runOnEdt(() -> {
            // Find the menu bar from any visible frame
            JMenuBar menuBar = null;
            for (Window window : Window.getWindows()) {
                if (window instanceof JFrame && window.isVisible()) {
                    JMenuBar bar = ((JFrame) window).getJMenuBar();
                    if (bar != null) {
                        menuBar = bar;
                        break;
                    }
                }
            }

            if (menuBar == null) {
                throw new IllegalArgumentException("No menu bar found");
            }

            try {
                JMenu currentMenu = null;

                // Find the top-level menu
                for (int i = 0; i < menuBar.getMenuCount(); i++) {
                    JMenu menu = menuBar.getMenu(i);
                    if (menu != null && parts[0].equals(menu.getText())) {
                        currentMenu = menu;
                        break;
                    }
                }

                if (currentMenu == null) {
                    throw new IllegalArgumentException("Menu not found: " + parts[0]);
                }

                // Check timeout
                if (System.currentTimeMillis() - startTime > timeoutMs) {
                    throw new RuntimeException("Menu selection timed out after " + timeoutMs + "ms");
                }

                // Click to open the menu
                currentMenu.doClick();
                EdtHelper.waitForEdt(200); // Increased from 100ms to 200ms for stability

                // Navigate through submenus
                for (int i = 1; i < parts.length; i++) {
                    // Check timeout before each submenu navigation
                    if (System.currentTimeMillis() - startTime > timeoutMs) {
                        MenuSelectionManager.defaultManager().clearSelectedPath();
                        throw new RuntimeException("Menu selection timed out after " + timeoutMs + "ms");
                    }

                    String itemName = parts[i];
                    JMenuItem foundItem = null;

                    for (int j = 0; j < currentMenu.getItemCount(); j++) {
                        JMenuItem item = currentMenu.getItem(j);
                        if (item != null && itemName.equals(item.getText())) {
                            foundItem = item;
                            break;
                        }
                    }

                    if (foundItem == null) {
                        // Close the menu and throw
                        MenuSelectionManager.defaultManager().clearSelectedPath();
                        throw new IllegalArgumentException("Menu item not found: " + itemName + " in path " + path);
                    }

                    // If this is a submenu, navigate into it
                    if (foundItem instanceof JMenu && i < parts.length - 1) {
                        currentMenu = (JMenu) foundItem;
                        // Hover to open submenu
                        Point loc = foundItem.getLocationOnScreen();
                        if (robot != null) {
                            robot.mouseMove(loc.x + foundItem.getWidth() / 2, loc.y + foundItem.getHeight() / 2);
                        }
                        EdtHelper.waitForEdt(250); // Increased from 150ms to 250ms for stability
                    } else {
                        // Click the menu item - use invokeLater for modal dialogs
                        final JMenuItem finalItem = foundItem;
                        SwingUtilities.invokeLater(() -> {
                            finalItem.doClick();
                        });
                        break;
                    }
                }
            } catch (Exception e) {
                // Ensure menu is closed on error
                MenuSelectionManager.defaultManager().clearSelectedPath();
                throw e;
            }
        });

        // Wait for menu action to complete - increased for modal dialogs
        EdtHelper.sleep(300); // Increased from 100ms to 300ms
    }

    /**
     * Focus a component.
     */
    public static void focus(int componentId) {
        EdtHelper.runOnEdt(() -> {
            Component component = getComponent(componentId);
            component.requestFocusInWindow();
        });
    }

    /**
     * Get element bounds.
     */
    public static JsonObject getElementBounds(int componentId) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            Component component = getComponent(componentId);
            Rectangle bounds = component.getBounds();

            JsonObject result = new JsonObject();
            result.addProperty("x", bounds.x);
            result.addProperty("y", bounds.y);
            result.addProperty("width", bounds.width);
            result.addProperty("height", bounds.height);

            if (component.isShowing()) {
                try {
                    Point screenLoc = component.getLocationOnScreen();
                    result.addProperty("screenX", screenLoc.x);
                    result.addProperty("screenY", screenLoc.y);
                } catch (Exception e) {
                    // Ignore
                }
            }

            return result;
        });
    }

    /**
     * Get element text.
     */
    public static JsonPrimitive getElementText(int componentId) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            Component component = getComponent(componentId);
            String text = getComponentText(component);
            return new JsonPrimitive(text != null ? text : "");
        });
    }

    /**
     * Select a table cell.
     */
    public static void selectTableCell(int componentId, int row, int column) {
        EdtHelper.runOnEdt(() -> {
            Component component = getComponent(componentId);
            if (!(component instanceof JTable)) {
                throw new IllegalArgumentException("Component is not a JTable");
            }

            JTable table = (JTable) component;
            // Validate row and column indices
            if (row < 0 || row >= table.getRowCount()) {
                throw new IndexOutOfBoundsException("Row index out of bounds: " + row + " (table has " + table.getRowCount() + " rows)");
            }
            if (column < 0 || column >= table.getColumnCount()) {
                throw new IndexOutOfBoundsException("Column index out of bounds: " + column + " (table has " + table.getColumnCount() + " columns)");
            }
            table.changeSelection(row, column, false, false);
        });
    }

    /**
     * Get table cell value.
     */
    public static JsonPrimitive getTableCellValue(int componentId, int row, int column) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            Component component = getComponent(componentId);
            if (!(component instanceof JTable)) {
                throw new IllegalArgumentException("Component is not a JTable");
            }

            JTable table = (JTable) component;
            Object value = table.getValueAt(row, column);
            return new JsonPrimitive(value != null ? value.toString() : "");
        });
    }

    /**
     * Set table cell value.
     */
    public static void setTableCellValue(int componentId, int row, int column, String value) {
        EdtHelper.runOnEdt(() -> {
            Component component = getComponent(componentId);
            if (!(component instanceof JTable)) {
                throw new IllegalArgumentException("Component is not a JTable");
            }

            JTable table = (JTable) component;
            table.setValueAt(value, row, column);
        });
    }

    /**
     * Get table row count.
     */
    public static JsonPrimitive getTableRowCount(int componentId) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            Component component = getComponent(componentId);
            if (!(component instanceof JTable)) {
                throw new IllegalArgumentException("Component is not a JTable");
            }

            return new JsonPrimitive(((JTable) component).getRowCount());
        });
    }

    /**
     * Get table column count.
     */
    public static JsonPrimitive getTableColumnCount(int componentId) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            Component component = getComponent(componentId);
            if (!(component instanceof JTable)) {
                throw new IllegalArgumentException("Component is not a JTable");
            }

            return new JsonPrimitive(((JTable) component).getColumnCount());
        });
    }

    /**
     * Get all table data.
     */
    public static JsonObject getTableData(int componentId) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            Component component = getComponent(componentId);
            if (!(component instanceof JTable)) {
                throw new IllegalArgumentException("Component is not a JTable");
            }

            JTable table = (JTable) component;
            TableModel model = table.getModel();

            JsonObject result = new JsonObject();
            result.addProperty("rowCount", model.getRowCount());
            result.addProperty("columnCount", model.getColumnCount());

            // Column names
            JsonArray columns = new JsonArray();
            for (int i = 0; i < model.getColumnCount(); i++) {
                columns.add(model.getColumnName(i));
            }
            result.add("columns", columns);

            // Row data
            JsonArray rows = new JsonArray();
            for (int row = 0; row < Math.min(model.getRowCount(), 1000); row++) {
                JsonArray rowData = new JsonArray();
                for (int col = 0; col < model.getColumnCount(); col++) {
                    Object value = model.getValueAt(row, col);
                    rowData.add(value != null ? value.toString() : null);
                }
                rows.add(rowData);
            }
            result.add("rows", rows);

            return result;
        });
    }

    /**
     * Expand tree node.
     */
    public static void expandTreeNode(int componentId, String path) {
        EdtHelper.runOnEdt(() -> {
            Component component = getComponent(componentId);
            if (!(component instanceof JTree)) {
                throw new IllegalArgumentException("Component is not a JTree");
            }

            JTree tree = (JTree) component;
            TreePath treePath = findTreePath(tree, path);
            if (treePath != null) {
                tree.expandPath(treePath);
            } else {
                throw new IllegalArgumentException("Tree path not found: " + path);
            }
        });
    }

    /**
     * Collapse tree node.
     */
    public static void collapseTreeNode(int componentId, String path) {
        EdtHelper.runOnEdt(() -> {
            Component component = getComponent(componentId);
            if (!(component instanceof JTree)) {
                throw new IllegalArgumentException("Component is not a JTree");
            }

            JTree tree = (JTree) component;
            TreePath treePath = findTreePath(tree, path);
            if (treePath != null) {
                tree.collapsePath(treePath);
            } else {
                throw new IllegalArgumentException("Tree path not found: " + path);
            }
        });
    }

    /**
     * Select tree node.
     */
    public static void selectTreeNode(int componentId, String path) {
        EdtHelper.runOnEdt(() -> {
            Component component = getComponent(componentId);
            if (!(component instanceof JTree)) {
                throw new IllegalArgumentException("Component is not a JTree");
            }

            JTree tree = (JTree) component;
            TreePath treePath = findTreePath(tree, path);
            if (treePath != null) {
                tree.setSelectionPath(treePath);
                tree.scrollPathToVisible(treePath);
            } else {
                throw new IllegalArgumentException("Tree path not found: " + path);
            }
        });
    }

    /**
     * Get selected tree path as an array containing the path string.
     */
    public static JsonArray getSelectedTreePath(int componentId) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            Component component = getComponent(componentId);
            if (!(component instanceof JTree)) {
                throw new IllegalArgumentException("Component is not a JTree");
            }

            JTree tree = (JTree) component;
            TreePath selPath = tree.getSelectionPath();
            JsonArray result = new JsonArray();

            if (selPath != null) {
                // Build path string from path components
                StringBuilder pathStr = new StringBuilder();
                Object[] pathNodes = selPath.getPath();
                for (int i = 0; i < pathNodes.length; i++) {
                    if (i > 0) pathStr.append("/");
                    pathStr.append(pathNodes[i].toString());
                }
                result.add(pathStr.toString());
            }

            return result;
        });
    }

    /**
     * Get tree nodes.
     */
    public static JsonObject getTreeNodes(int componentId) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            Component component = getComponent(componentId);
            if (!(component instanceof JTree)) {
                throw new IllegalArgumentException("Component is not a JTree");
            }

            JTree tree = (JTree) component;
            TreeModel model = tree.getModel();
            Object root = model.getRoot();

            return buildTreeNodeJson(model, root);
        });
    }

    private static JsonObject buildTreeNodeJson(TreeModel model, Object node) {
        JsonObject json = new JsonObject();
        json.addProperty("text", node.toString());
        json.addProperty("leaf", model.isLeaf(node));

        JsonArray children = new JsonArray();
        int childCount = model.getChildCount(node);
        for (int i = 0; i < childCount; i++) {
            children.add(buildTreeNodeJson(model, model.getChild(node, i)));
        }
        json.add("children", children);

        return json;
    }

    /**
     * Get list items.
     */
    public static JsonArray getListItems(int componentId) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            Component component = getComponent(componentId);
            if (!(component instanceof JList)) {
                throw new IllegalArgumentException("Component is not a JList");
            }

            JList<?> list = (JList<?>) component;
            ListModel<?> model = list.getModel();

            JsonArray items = new JsonArray();
            for (int i = 0; i < model.getSize(); i++) {
                Object item = model.getElementAt(i);
                items.add(item != null ? item.toString() : null);
            }

            return items;
        });
    }

    /**
     * Capture screenshot.
     */
    public static JsonPrimitive captureScreenshot(int componentId) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            BufferedImage image;

            if (componentId >= 0) {
                Component component = getComponent(componentId);
                if (!component.isShowing()) {
                    throw new IllegalStateException("Component is not visible");
                }

                Point location = component.getLocationOnScreen();
                Dimension size = component.getSize();
                Rectangle rect = new Rectangle(location.x, location.y, size.width, size.height);

                if (robot != null) {
                    image = robot.createScreenCapture(rect);
                } else {
                    // Fallback: render component to image
                    image = new BufferedImage(size.width, size.height, BufferedImage.TYPE_INT_RGB);
                    Graphics2D g = image.createGraphics();
                    component.paint(g);
                    g.dispose();
                }
            } else {
                // Full screen capture
                if (robot == null) {
                    throw new IllegalStateException("Robot not available for screenshot");
                }

                Dimension screenSize = Toolkit.getDefaultToolkit().getScreenSize();
                image = robot.createScreenCapture(new Rectangle(screenSize));
            }

            // Convert to base64
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            ImageIO.write(image, "png", baos);
            byte[] bytes = baos.toByteArray();
            String base64 = Base64.getEncoder().encodeToString(bytes);

            return new JsonPrimitive("data:image/png;base64," + base64);
        });
    }

    // Helper methods

    private static Component getComponent(int componentId) {
        Component component = ComponentInspector.getComponentById(componentId);
        if (component == null) {
            throw new IllegalArgumentException("Component not found: " + componentId);
        }
        return component;
    }

    private static void ensureVisible(Component component) {
        if (!component.isShowing()) {
            throw new IllegalStateException("Component is not visible");
        }
    }

    private static Point getComponentCenter(Component component) {
        int x = component.getWidth() / 2;
        int y = component.getHeight() / 2;
        return new Point(x, y);
    }

    private static void performMouseClick(Component component, int clickCount) {
        Point center = getComponentCenter(component);

        MouseEvent pressed = new MouseEvent(
            component,
            MouseEvent.MOUSE_PRESSED,
            System.currentTimeMillis(),
            InputEvent.BUTTON1_DOWN_MASK,
            center.x, center.y,
            clickCount, false, MouseEvent.BUTTON1
        );

        MouseEvent released = new MouseEvent(
            component,
            MouseEvent.MOUSE_RELEASED,
            System.currentTimeMillis(),
            InputEvent.BUTTON1_DOWN_MASK,
            center.x, center.y,
            clickCount, false, MouseEvent.BUTTON1
        );

        MouseEvent clicked = new MouseEvent(
            component,
            MouseEvent.MOUSE_CLICKED,
            System.currentTimeMillis(),
            InputEvent.BUTTON1_DOWN_MASK,
            center.x, center.y,
            clickCount, false, MouseEvent.BUTTON1
        );

        component.dispatchEvent(pressed);
        component.dispatchEvent(released);
        component.dispatchEvent(clicked);
    }

    private static String getComponentText(Component component) {
        if (component instanceof JTextComponent) {
            return ((JTextComponent) component).getText();
        }
        if (component instanceof JLabel) {
            return ((JLabel) component).getText();
        }
        if (component instanceof AbstractButton) {
            return ((AbstractButton) component).getText();
        }
        if (component instanceof Frame) {
            return ((Frame) component).getTitle();
        }
        if (component instanceof Dialog) {
            return ((Dialog) component).getTitle();
        }
        if (component instanceof JList) {
            JList<?> list = (JList<?>) component;
            Object selected = list.getSelectedValue();
            return selected != null ? selected.toString() : "";
        }
        if (component instanceof JComboBox) {
            JComboBox<?> combo = (JComboBox<?>) component;
            Object selected = combo.getSelectedItem();
            return selected != null ? selected.toString() : "";
        }
        if (component instanceof JSpinner) {
            JSpinner spinner = (JSpinner) component;
            Object value = spinner.getValue();
            return value != null ? value.toString() : "";
        }
        return null;
    }

    private static TreePath findTreePath(JTree tree, String pathStr) {
        String[] parts = pathStr.split("/");
        TreeModel model = tree.getModel();
        Object root = model.getRoot();

        if (parts.length == 0 || (parts.length == 1 && parts[0].isEmpty())) {
            return new TreePath(root);
        }

        int startIndex = 0;
        if (root.toString().equals(parts[0])) {
            startIndex = 1;
        }

        Object[] pathObjects = new Object[parts.length - startIndex + 1];
        pathObjects[0] = root;

        Object current = root;
        for (int i = startIndex; i < parts.length; i++) {
            Object child = findChild(model, current, parts[i]);
            if (child == null) {
                return null;
            }
            pathObjects[i - startIndex + 1] = child;
            current = child;
        }

        return new TreePath(pathObjects);
    }

    private static Object findChild(TreeModel model, Object parent, String name) {
        int childCount = model.getChildCount(parent);
        for (int i = 0; i < childCount; i++) {
            Object child = model.getChild(parent, i);
            if (child.toString().equals(name)) {
                return child;
            }
        }
        return null;
    }

    /**
     * Close all open dialogs to recover from stuck state.
     * This is a recovery mechanism for tests that get stuck on modal dialogs.
     */
    public static void closeAllDialogs() {
        EdtHelper.runOnEdt(() -> {
            // Close all JDialog instances
            for (Window window : Window.getWindows()) {
                if (window instanceof JDialog && window.isVisible()) {
                    try {
                        System.out.println("[SwingAgent] Closing dialog: " + ((JDialog) window).getTitle());
                        window.dispose();
                    } catch (Exception e) {
                        System.err.println("[SwingAgent] Failed to close dialog: " + e.getMessage());
                    }
                }
            }

            // Clear any menu selections
            try {
                MenuSelectionManager.defaultManager().clearSelectedPath();
            } catch (Exception e) {
                System.err.println("[SwingAgent] Failed to clear menu selection: " + e.getMessage());
            }
        });

        // Give EDT time to process cleanup
        EdtHelper.sleep(200);
    }

    /**
     * Force close a specific dialog by name.
     *
     * @param dialogName The name of the dialog to close
     * @return true if dialog was found and closed, false otherwise
     */
    public static boolean forceCloseDialog(String dialogName) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            for (Window window : Window.getWindows()) {
                if (window instanceof JDialog && window.isVisible()) {
                    JDialog dialog = (JDialog) window;
                    if (dialogName.equals(dialog.getName()) ||
                        dialogName.equals(dialog.getTitle())) {
                        try {
                            System.out.println("[SwingAgent] Force closing dialog: " + dialogName);
                            dialog.dispose();
                            EdtHelper.sleep(100);
                            return true;
                        } catch (Exception e) {
                            System.err.println("[SwingAgent] Failed to force close dialog: " + e.getMessage());
                        }
                    }
                }
            }
            return false;
        });
    }
}
