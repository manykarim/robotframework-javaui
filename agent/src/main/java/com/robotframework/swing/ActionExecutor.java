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
        // Get and validate component synchronously
        Component component = EdtHelper.runOnEdtAndReturn(() -> {
            Component c = getComponent(componentId);
            ensureVisible(c);
            return c;
        });

        // Perform click asynchronously to avoid blocking on modal dialogs
        EdtHelper.runOnEdtLater(() -> {
            if (component instanceof AbstractButton) {
                ((AbstractButton) component).doClick();
            } else {
                performMouseClick(component, 1);
            }
        });

        // Give the click a moment to process
        EdtHelper.sleep(100);
    }

    /**
     * Double-click on a component.
     * Uses runOnEdtLater to avoid blocking on modal dialogs.
     */
    public static void doubleClick(int componentId) {
        // Get and validate component synchronously
        Component component = EdtHelper.runOnEdtAndReturn(() -> {
            Component c = getComponent(componentId);
            ensureVisible(c);
            return c;
        });

        // Perform double-click asynchronously
        EdtHelper.runOnEdtLater(() -> performMouseClick(component, 2));

        // Give the click a moment to process
        EdtHelper.sleep(100);
    }

    /**
     * Right-click on a component.
     * Uses runOnEdtLater to avoid blocking on popup menus.
     */
    public static void rightClick(int componentId) {
        // Get and validate component synchronously
        Component component = EdtHelper.runOnEdtAndReturn(() -> {
            Component c = getComponent(componentId);
            ensureVisible(c);
            return c;
        });

        // Perform right-click asynchronously
        EdtHelper.runOnEdtLater(() -> {
            Point center = getComponentCenter(component);
            MouseEvent event = new MouseEvent(
                component,
                MouseEvent.MOUSE_CLICKED,
                System.currentTimeMillis(),
                InputEvent.BUTTON3_DOWN_MASK,
                center.x, center.y,
                1, true, MouseEvent.BUTTON3
            );
            component.dispatchEvent(event);
        });

        // Give the click a moment to process
        EdtHelper.sleep(100);
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
}
