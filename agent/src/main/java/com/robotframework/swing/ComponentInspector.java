package com.robotframework.swing;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;

import javax.accessibility.AccessibleContext;
import javax.accessibility.AccessibleRole;
import javax.accessibility.AccessibleState;
import javax.accessibility.AccessibleStateSet;
import javax.swing.*;
import javax.swing.table.JTableHeader;
import javax.swing.text.JTextComponent;
import javax.swing.tree.TreePath;
import java.awt.*;
import java.beans.BeanInfo;
import java.beans.Introspector;
import java.beans.PropertyDescriptor;
import java.lang.reflect.Method;
import java.util.*;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * Component inspection utilities for Swing components.
 * Provides methods to inspect, find, and analyze Swing component hierarchies.
 */
public class ComponentInspector {

    private static final AtomicInteger componentIdCounter = new AtomicInteger(0);
    private static final Map<Integer, Component> componentCache = Collections.synchronizedMap(new WeakHashMap<>());
    private static final Map<Component, Integer> reverseCache = Collections.synchronizedMap(new WeakHashMap<>());

    /**
     * Get all visible frames/windows in the application.
     *
     * @return JsonArray of window information
     */
    public static JsonArray getWindows() {
        return EdtHelper.runOnEdtAndReturn(() -> {
            JsonArray windows = new JsonArray();

            for (Window window : Window.getWindows()) {
                if (window.isShowing()) {
                    JsonObject windowInfo = new JsonObject();
                    windowInfo.addProperty("id", getOrCreateId(window));
                    windowInfo.addProperty("class", window.getClass().getName());
                    windowInfo.addProperty("title", getWindowTitle(window));
                    windowInfo.addProperty("x", window.getX());
                    windowInfo.addProperty("y", window.getY());
                    windowInfo.addProperty("width", window.getWidth());
                    windowInfo.addProperty("height", window.getHeight());
                    windowInfo.addProperty("visible", window.isVisible());
                    windowInfo.addProperty("active", window.isActive());
                    windows.add(windowInfo);
                }
            }

            return windows;
        });
    }

    /**
     * Get the full component tree starting from root frames.
     *
     * @return JsonObject representing the component tree
     */
    public static JsonObject getComponentTree() {
        return EdtHelper.runOnEdtAndReturn(() -> {
            JsonObject result = new JsonObject();
            JsonArray roots = new JsonArray();

            for (Window window : Window.getWindows()) {
                if (window.isShowing()) {
                    roots.add(buildComponentNode(window, 0, 10));
                }
            }

            result.add("roots", roots);
            result.addProperty("timestamp", System.currentTimeMillis());
            return result;
        });
    }

    /**
     * Get component tree starting from a specific component.
     *
     * @param componentId Component ID to start from
     * @param maxDepth Maximum depth to traverse
     * @return JsonObject representing the component subtree
     */
    public static JsonObject getComponentTree(int componentId, int maxDepth) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            Component component = componentCache.get(componentId);
            if (component == null) {
                throw new IllegalArgumentException("Component not found: " + componentId);
            }
            return buildComponentNode(component, 0, maxDepth);
        });
    }

    /**
     * Build a JSON node for a component and its children.
     */
    private static JsonObject buildComponentNode(Component component, int depth, int maxDepth) {
        JsonObject node = new JsonObject();

        node.addProperty("id", getOrCreateId(component));
        node.addProperty("class", component.getClass().getName());
        node.addProperty("simpleClass", component.getClass().getSimpleName());
        node.addProperty("name", component.getName());

        // Basic properties
        Rectangle bounds = component.getBounds();
        node.addProperty("x", bounds.x);
        node.addProperty("y", bounds.y);
        node.addProperty("width", bounds.width);
        node.addProperty("height", bounds.height);
        node.addProperty("visible", component.isVisible());
        node.addProperty("enabled", component.isEnabled());
        node.addProperty("showing", component.isShowing());

        // Screen location
        if (component.isShowing()) {
            try {
                Point screenLoc = component.getLocationOnScreen();
                node.addProperty("screenX", screenLoc.x);
                node.addProperty("screenY", screenLoc.y);
            } catch (Exception e) {
                // Component might not be displayable
            }
        }

        // Type-specific properties
        addTypeSpecificProperties(node, component);

        // Accessible properties
        addAccessibleProperties(node, component);

        // Children
        if (depth < maxDepth && component instanceof Container) {
            Container container = (Container) component;
            JsonArray children = new JsonArray();

            for (Component child : container.getComponents()) {
                children.add(buildComponentNode(child, depth + 1, maxDepth));
            }

            node.add("children", children);
            node.addProperty("childCount", container.getComponentCount());
        }

        return node;
    }

    /**
     * Add type-specific properties based on component type.
     */
    private static void addTypeSpecificProperties(JsonObject node, Component component) {
        // Window/Frame title
        if (component instanceof Frame) {
            node.addProperty("title", ((Frame) component).getTitle());
        } else if (component instanceof Dialog) {
            node.addProperty("title", ((Dialog) component).getTitle());
        }

        // Text components
        if (component instanceof JTextComponent) {
            JTextComponent textComp = (JTextComponent) component;
            node.addProperty("text", textComp.getText());
            node.addProperty("editable", textComp.isEditable());
            node.addProperty("caretPosition", textComp.getCaretPosition());
            node.addProperty("selectionStart", textComp.getSelectionStart());
            node.addProperty("selectionEnd", textComp.getSelectionEnd());
        }

        // Labels
        if (component instanceof JLabel) {
            JLabel label = (JLabel) component;
            node.addProperty("text", label.getText());
            Component labelFor = label.getLabelFor();
            if (labelFor != null) {
                node.addProperty("labelFor", getOrCreateId(labelFor));
            }
        }

        // Buttons
        if (component instanceof AbstractButton) {
            AbstractButton button = (AbstractButton) component;
            node.addProperty("text", button.getText());
            node.addProperty("selected", button.isSelected());
            node.addProperty("actionCommand", button.getActionCommand());

            if (button.getMnemonic() != 0) {
                node.addProperty("mnemonic", String.valueOf((char) button.getMnemonic()));
            }
        }

        // ComboBox
        if (component instanceof JComboBox) {
            JComboBox<?> combo = (JComboBox<?>) component;
            node.addProperty("selectedIndex", combo.getSelectedIndex());
            Object selected = combo.getSelectedItem();
            node.addProperty("selectedItem", selected != null ? selected.toString() : null);
            node.addProperty("itemCount", combo.getItemCount());
            node.addProperty("editable", combo.isEditable());

            JsonArray items = new JsonArray();
            for (int i = 0; i < Math.min(combo.getItemCount(), 100); i++) {
                Object item = combo.getItemAt(i);
                items.add(item != null ? item.toString() : null);
            }
            node.add("items", items);
        }

        // List
        if (component instanceof JList) {
            JList<?> list = (JList<?>) component;
            node.addProperty("selectedIndex", list.getSelectedIndex());
            int[] selected = list.getSelectedIndices();
            JsonArray selectedIndices = new JsonArray();
            for (int idx : selected) {
                selectedIndices.add(idx);
            }
            node.add("selectedIndices", selectedIndices);
            node.addProperty("visibleRowCount", list.getVisibleRowCount());
        }

        // Table
        if (component instanceof JTable) {
            JTable table = (JTable) component;
            node.addProperty("rowCount", table.getRowCount());
            node.addProperty("columnCount", table.getColumnCount());
            node.addProperty("selectedRow", table.getSelectedRow());
            node.addProperty("selectedColumn", table.getSelectedColumn());

            JsonArray columns = new JsonArray();
            for (int i = 0; i < table.getColumnCount(); i++) {
                columns.add(table.getColumnName(i));
            }
            node.add("columnNames", columns);
        }

        // Tree
        if (component instanceof JTree) {
            JTree tree = (JTree) component;
            node.addProperty("rowCount", tree.getRowCount());
            node.addProperty("selectionCount", tree.getSelectionCount());
            TreePath selPath = tree.getSelectionPath();
            if (selPath != null) {
                node.addProperty("selectedPath", selPath.toString());
            }
        }

        // TabbedPane
        if (component instanceof JTabbedPane) {
            JTabbedPane tabs = (JTabbedPane) component;
            node.addProperty("tabCount", tabs.getTabCount());
            node.addProperty("selectedIndex", tabs.getSelectedIndex());

            JsonArray tabTitles = new JsonArray();
            for (int i = 0; i < tabs.getTabCount(); i++) {
                tabTitles.add(tabs.getTitleAt(i));
            }
            node.add("tabTitles", tabTitles);
        }

        // Slider
        if (component instanceof JSlider) {
            JSlider slider = (JSlider) component;
            node.addProperty("value", slider.getValue());
            node.addProperty("minimum", slider.getMinimum());
            node.addProperty("maximum", slider.getMaximum());
        }

        // Spinner
        if (component instanceof JSpinner) {
            JSpinner spinner = (JSpinner) component;
            Object value = spinner.getValue();
            node.addProperty("value", value != null ? value.toString() : null);
        }

        // ProgressBar
        if (component instanceof JProgressBar) {
            JProgressBar progress = (JProgressBar) component;
            node.addProperty("value", progress.getValue());
            node.addProperty("minimum", progress.getMinimum());
            node.addProperty("maximum", progress.getMaximum());
            node.addProperty("indeterminate", progress.isIndeterminate());
            node.addProperty("percentComplete", progress.getPercentComplete());
        }

        // Tooltip
        if (component instanceof JComponent) {
            JComponent jcomp = (JComponent) component;
            String tooltip = jcomp.getToolTipText();
            if (tooltip != null && !tooltip.isEmpty()) {
                node.addProperty("tooltip", tooltip);
            }
        }

        // Scroll position
        if (component instanceof JScrollPane) {
            JScrollPane scroll = (JScrollPane) component;
            JViewport viewport = scroll.getViewport();
            if (viewport != null) {
                Point viewPos = viewport.getViewPosition();
                node.addProperty("viewX", viewPos.x);
                node.addProperty("viewY", viewPos.y);
            }
        }
    }

    /**
     * Add accessible properties from AccessibleContext.
     */
    private static void addAccessibleProperties(JsonObject node, Component component) {
        AccessibleContext ac = component.getAccessibleContext();
        if (ac == null) {
            return;
        }

        String accessibleName = ac.getAccessibleName();
        if (accessibleName != null && !accessibleName.isEmpty()) {
            node.addProperty("accessibleName", accessibleName);
        }

        String accessibleDescription = ac.getAccessibleDescription();
        if (accessibleDescription != null && !accessibleDescription.isEmpty()) {
            node.addProperty("accessibleDescription", accessibleDescription);
        }

        AccessibleRole role = ac.getAccessibleRole();
        if (role != null) {
            node.addProperty("accessibleRole", role.toString());
        }

        AccessibleStateSet states = ac.getAccessibleStateSet();
        if (states != null) {
            JsonArray stateArray = new JsonArray();
            for (AccessibleState state : states.toArray()) {
                stateArray.add(state.toString());
            }
            node.add("accessibleStates", stateArray);
        }
    }

    /**
     * Get all properties of a component.
     *
     * @param componentId Component ID
     * @return JsonObject with all properties
     */
    public static JsonObject getComponentProperties(int componentId) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            Component component = componentCache.get(componentId);
            if (component == null) {
                throw new IllegalArgumentException("Component not found: " + componentId);
            }

            JsonObject props = new JsonObject();

            // Basic properties
            props.addProperty("id", componentId);
            props.addProperty("class", component.getClass().getName());
            props.addProperty("name", component.getName());

            // Bounds
            Rectangle bounds = component.getBounds();
            JsonObject boundsObj = new JsonObject();
            boundsObj.addProperty("x", bounds.x);
            boundsObj.addProperty("y", bounds.y);
            boundsObj.addProperty("width", bounds.width);
            boundsObj.addProperty("height", bounds.height);
            props.add("bounds", boundsObj);

            // Screen location
            if (component.isShowing()) {
                try {
                    Point screenLoc = component.getLocationOnScreen();
                    JsonObject screenLocObj = new JsonObject();
                    screenLocObj.addProperty("x", screenLoc.x);
                    screenLocObj.addProperty("y", screenLoc.y);
                    props.add("screenLocation", screenLocObj);
                } catch (Exception e) {
                    // Ignore
                }
            }

            // State flags
            props.addProperty("visible", component.isVisible());
            props.addProperty("showing", component.isShowing());
            props.addProperty("enabled", component.isEnabled());
            props.addProperty("focusable", component.isFocusable());
            props.addProperty("focused", component.isFocusOwner());
            props.addProperty("displayable", component.isDisplayable());
            props.addProperty("valid", component.isValid());

            // Colors
            Color bg = component.getBackground();
            Color fg = component.getForeground();
            if (bg != null) {
                props.addProperty("background", colorToHex(bg));
            }
            if (fg != null) {
                props.addProperty("foreground", colorToHex(fg));
            }

            // Font
            Font font = component.getFont();
            if (font != null) {
                JsonObject fontObj = new JsonObject();
                fontObj.addProperty("family", font.getFamily());
                fontObj.addProperty("name", font.getName());
                fontObj.addProperty("size", font.getSize());
                fontObj.addProperty("style", font.getStyle());
                fontObj.addProperty("bold", font.isBold());
                fontObj.addProperty("italic", font.isItalic());
                props.add("font", fontObj);
            }

            // Type-specific properties
            addTypeSpecificProperties(props, component);

            // Accessible properties
            addAccessibleProperties(props, component);

            // Try to get bean properties
            try {
                BeanInfo beanInfo = Introspector.getBeanInfo(component.getClass());
                JsonObject beanProps = new JsonObject();

                for (PropertyDescriptor pd : beanInfo.getPropertyDescriptors()) {
                    Method getter = pd.getReadMethod();
                    if (getter != null && getter.getParameterCount() == 0) {
                        try {
                            Object value = getter.invoke(component);
                            if (value != null && isPrimitiveOrString(value)) {
                                beanProps.addProperty(pd.getName(), value.toString());
                            }
                        } catch (Exception e) {
                            // Skip properties that can't be read
                        }
                    }
                }

                props.add("beanProperties", beanProps);
            } catch (Exception e) {
                // Ignore introspection failures
            }

            return props;
        });
    }

    /**
     * Find a component by locator.
     *
     * @param locator Locator object with type and value
     * @return Component ID or -1 if not found
     */
    public static int findComponent(JsonObject locator) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            String type = locator.has("type") ? locator.get("type").getAsString() : "name";
            String value = locator.get("value").getAsString();
            int parentId = locator.has("parent") ? locator.get("parent").getAsInt() : -1;
            int index = locator.has("index") ? locator.get("index").getAsInt() : 0;

            Container searchRoot = null;
            if (parentId >= 0) {
                Component parent = componentCache.get(parentId);
                if (parent instanceof Container) {
                    searchRoot = (Container) parent;
                }
            }

            List<Component> matches = new ArrayList<>();

            if (searchRoot != null) {
                findComponents(searchRoot, type, value, matches);
            } else {
                for (Window window : Window.getWindows()) {
                    if (window.isShowing()) {
                        findComponents(window, type, value, matches);
                    }
                }
            }

            if (index < matches.size()) {
                return getOrCreateId(matches.get(index));
            }

            return -1;
        });
    }

    /**
     * Find all components matching a locator.
     *
     * @param locator Locator object
     * @return Array of component IDs
     */
    public static JsonArray findAllComponents(JsonObject locator) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            String type = locator.has("type") ? locator.get("type").getAsString() : "name";
            String value = locator.get("value").getAsString();
            int parentId = locator.has("parent") ? locator.get("parent").getAsInt() : -1;

            Container searchRoot = null;
            if (parentId >= 0) {
                Component parent = componentCache.get(parentId);
                if (parent instanceof Container) {
                    searchRoot = (Container) parent;
                }
            }

            List<Component> matches = new ArrayList<>();

            if (searchRoot != null) {
                findComponents(searchRoot, type, value, matches);
            } else {
                for (Window window : Window.getWindows()) {
                    if (window.isShowing()) {
                        findComponents(window, type, value, matches);
                    }
                }
            }

            JsonArray result = new JsonArray();
            for (Component comp : matches) {
                result.add(getOrCreateId(comp));
            }
            return result;
        });
    }

    /**
     * Recursively find components matching criteria.
     */
    private static void findComponents(Container container, String type, String value, List<Component> matches) {
        if (matchesLocator(container, type, value)) {
            matches.add(container);
        }

        for (Component child : container.getComponents()) {
            if (matchesLocator(child, type, value)) {
                matches.add(child);
            }
            if (child instanceof Container) {
                findComponents((Container) child, type, value, matches);
            }
        }
    }

    /**
     * Check if a component matches the locator criteria.
     */
    private static boolean matchesLocator(Component component, String type, String value) {
        switch (type.toLowerCase()) {
            case "name":
                return value.equals(component.getName());

            case "class":
                return component.getClass().getName().equals(value) ||
                       component.getClass().getSimpleName().equals(value);

            case "text":
                String text = getComponentText(component);
                return text != null && text.equals(value);

            case "text_contains":
                String textContains = getComponentText(component);
                return textContains != null && textContains.contains(value);

            case "text_regex":
                String textRegex = getComponentText(component);
                return textRegex != null && textRegex.matches(value);

            case "tooltip":
                if (component instanceof JComponent) {
                    String tooltip = ((JComponent) component).getToolTipText();
                    return tooltip != null && tooltip.equals(value);
                }
                return false;

            case "accessible_name":
                AccessibleContext ac = component.getAccessibleContext();
                if (ac != null) {
                    String accName = ac.getAccessibleName();
                    return accName != null && accName.equals(value);
                }
                return false;

            case "id":
                Integer id = reverseCache.get(component);
                return id != null && id.toString().equals(value);

            case "xpath":
                // Simplified XPath-like matching
                return matchesXPath(component, value);

            default:
                return false;
        }
    }

    /**
     * Get text from various component types.
     */
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

    /**
     * Simplified XPath-like matching.
     */
    private static boolean matchesXPath(Component component, String xpath) {
        // Basic implementation - matches class/name patterns
        String[] parts = xpath.split("/");
        String lastPart = parts[parts.length - 1];

        if (lastPart.startsWith("@")) {
            // Attribute match
            String attr = lastPart.substring(1);
            if (attr.contains("=")) {
                String[] attrParts = attr.split("=", 2);
                String attrName = attrParts[0];
                String attrValue = attrParts[1].replace("'", "").replace("\"", "");

                switch (attrName) {
                    case "name":
                        return attrValue.equals(component.getName());
                    case "class":
                        return attrValue.equals(component.getClass().getSimpleName());
                    default:
                        return false;
                }
            }
        } else {
            // Class name match
            return component.getClass().getSimpleName().equals(lastPart);
        }

        return false;
    }

    /**
     * Get or create a unique ID for a component.
     */
    public static int getOrCreateId(Component component) {
        Integer existing = reverseCache.get(component);
        if (existing != null) {
            return existing;
        }

        int id = componentIdCounter.incrementAndGet();
        componentCache.put(id, component);
        reverseCache.put(component, id);
        return id;
    }

    /**
     * Get a component by ID.
     *
     * @param id Component ID
     * @return Component or null if not found
     */
    public static Component getComponentById(int id) {
        return componentCache.get(id);
    }

    /**
     * Get window title for various window types.
     */
    private static String getWindowTitle(Window window) {
        if (window instanceof Frame) {
            return ((Frame) window).getTitle();
        }
        if (window instanceof Dialog) {
            return ((Dialog) window).getTitle();
        }
        return window.getName();
    }

    /**
     * Convert Color to hex string.
     */
    private static String colorToHex(Color color) {
        return String.format("#%02x%02x%02x", color.getRed(), color.getGreen(), color.getBlue());
    }

    /**
     * Check if value is primitive or string.
     */
    private static boolean isPrimitiveOrString(Object value) {
        return value instanceof String ||
               value instanceof Number ||
               value instanceof Boolean ||
               value instanceof Character;
    }

    /**
     * Clear the component cache.
     */
    public static void clearCache() {
        componentCache.clear();
        reverseCache.clear();
    }
}
