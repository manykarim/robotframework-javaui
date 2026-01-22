package com.robotframework.swt;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;

import org.eclipse.swt.SWT;
import org.eclipse.swt.browser.Browser;
import org.eclipse.swt.custom.CTabFolder;
import org.eclipse.swt.custom.CTabItem;
import org.eclipse.swt.graphics.Color;
import org.eclipse.swt.graphics.Font;
import org.eclipse.swt.graphics.FontData;
import org.eclipse.swt.graphics.Point;
import org.eclipse.swt.graphics.Rectangle;
import org.eclipse.swt.widgets.*;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.WeakHashMap;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * Widget inspection utilities for SWT widgets.
 * Provides methods to inspect, find, and analyze SWT widget hierarchies.
 *
 * Uses WeakHashMap for caching to allow widgets to be garbage collected
 * when no longer referenced by the application.
 */
public class WidgetInspector {

    static {
        System.err.println("[SwtAgent] WidgetInspector class loading...");
        System.err.flush();
    }

    private static final AtomicInteger widgetIdCounter = new AtomicInteger(0);
    private static final Map<Integer, Widget> widgetCache = Collections.synchronizedMap(new WeakHashMap<>());
    private static final Map<Widget, Integer> reverseCache = Collections.synchronizedMap(new WeakHashMap<>());

    /**
     * Get all visible shells in the application.
     *
     * @return JsonArray of shell information
     */
    public static JsonArray getShells() {
        System.err.println("[SwtAgent] WidgetInspector.getShells() called");
        System.err.flush();
        return DisplayHelper.syncExecAndReturn(() -> {
            System.err.println("[SwtAgent] Inside getShells lambda on UI thread");
            System.err.flush();
            JsonArray shells = new JsonArray();
            Display display = DisplayHelper.getDisplay();

            if (display == null || display.isDisposed()) {
                return shells;
            }

            for (Shell shell : display.getShells()) {
                if (shell.isVisible()) {
                    JsonObject shellInfo = new JsonObject();
                    shellInfo.addProperty("id", getOrCreateId(shell));
                    shellInfo.addProperty("class", shell.getClass().getName());
                    shellInfo.addProperty("text", shell.getText());

                    Rectangle bounds = shell.getBounds();
                    shellInfo.addProperty("x", bounds.x);
                    shellInfo.addProperty("y", bounds.y);
                    shellInfo.addProperty("width", bounds.width);
                    shellInfo.addProperty("height", bounds.height);
                    shellInfo.addProperty("visible", shell.isVisible());
                    shellInfo.addProperty("enabled", shell.isEnabled());

                    // Check if it's the active shell
                    Shell activeShell = display.getActiveShell();
                    shellInfo.addProperty("active", shell == activeShell);

                    shells.add(shellInfo);
                }
            }

            return shells;
        });
    }

    /**
     * Get the full widget tree starting from all visible shells.
     *
     * @return JsonObject representing the widget tree
     */
    public static JsonObject getWidgetTree() {
        return DisplayHelper.syncExecAndReturn(() -> {
            JsonObject result = new JsonObject();
            JsonArray roots = new JsonArray();
            Display display = DisplayHelper.getDisplay();

            if (display != null && !display.isDisposed()) {
                for (Shell shell : display.getShells()) {
                    if (shell.isVisible()) {
                        roots.add(buildWidgetNode(shell, 0, 10));
                    }
                }
            }

            result.add("roots", roots);
            result.addProperty("timestamp", System.currentTimeMillis());
            return result;
        });
    }

    /**
     * Get widget tree starting from a specific widget.
     *
     * @param widgetId Widget ID to start from
     * @param maxDepth Maximum depth to traverse
     * @return JsonObject representing the widget subtree
     */
    public static JsonObject getWidgetTree(int widgetId, int maxDepth) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = widgetCache.get(widgetId);
            if (widget == null || widget.isDisposed()) {
                throw new IllegalArgumentException("Widget not found or disposed: " + widgetId);
            }
            return buildWidgetNode(widget, 0, maxDepth);
        });
    }

    /**
     * Build a JSON node for a widget and its children.
     */
    private static JsonObject buildWidgetNode(Widget widget, int depth, int maxDepth) {
        JsonObject node = new JsonObject();

        node.addProperty("id", getOrCreateId(widget));
        node.addProperty("class", widget.getClass().getName());
        node.addProperty("simpleClass", widget.getClass().getSimpleName());

        // Check if widget is disposed
        if (widget.isDisposed()) {
            node.addProperty("disposed", true);
            return node;
        }

        // Control-specific properties
        if (widget instanceof Control) {
            Control control = (Control) widget;

            Rectangle bounds = control.getBounds();
            node.addProperty("x", bounds.x);
            node.addProperty("y", bounds.y);
            node.addProperty("width", bounds.width);
            node.addProperty("height", bounds.height);
            node.addProperty("visible", control.isVisible());
            node.addProperty("enabled", control.isEnabled());
            node.addProperty("focusControl", control.isFocusControl());

            // Tooltip
            String tooltip = control.getToolTipText();
            if (tooltip != null && !tooltip.isEmpty()) {
                node.addProperty("tooltip", tooltip);
            }

            // Screen location (for visible controls)
            if (control.isVisible()) {
                try {
                    Point screenLoc = control.toDisplay(0, 0);
                    node.addProperty("screenX", screenLoc.x);
                    node.addProperty("screenY", screenLoc.y);
                } catch (Exception e) {
                    // Ignore - control might not be displayable
                }
            }
        }

        // Type-specific properties
        addTypeSpecificProperties(node, widget);

        // Children
        if (depth < maxDepth && widget instanceof Composite) {
            Composite composite = (Composite) widget;
            JsonArray children = new JsonArray();

            for (Control child : composite.getChildren()) {
                children.add(buildWidgetNode(child, depth + 1, maxDepth));
            }

            node.add("children", children);
            node.addProperty("childCount", composite.getChildren().length);
        }

        return node;
    }

    /**
     * Add type-specific properties based on widget type.
     */
    private static void addTypeSpecificProperties(JsonObject node, Widget widget) {
        // Shell
        if (widget instanceof Shell) {
            Shell shell = (Shell) widget;
            node.addProperty("text", shell.getText());
            node.addProperty("maximized", shell.getMaximized());
            node.addProperty("minimized", shell.getMinimized());
            node.addProperty("fullScreen", shell.getFullScreen());
        }

        // Button (push, check, radio)
        if (widget instanceof Button) {
            Button button = (Button) widget;
            node.addProperty("text", button.getText());
            node.addProperty("selection", button.getSelection());

            int style = button.getStyle();
            if ((style & SWT.CHECK) != 0) {
                node.addProperty("buttonType", "check");
            } else if ((style & SWT.RADIO) != 0) {
                node.addProperty("buttonType", "radio");
            } else if ((style & SWT.TOGGLE) != 0) {
                node.addProperty("buttonType", "toggle");
            } else if ((style & SWT.ARROW) != 0) {
                node.addProperty("buttonType", "arrow");
            } else {
                node.addProperty("buttonType", "push");
            }
        }

        // Label
        if (widget instanceof Label) {
            Label label = (Label) widget;
            node.addProperty("text", label.getText());
        }

        // Text
        if (widget instanceof Text) {
            Text text = (Text) widget;
            node.addProperty("text", text.getText());
            node.addProperty("editable", text.getEditable());
            node.addProperty("echoChar", String.valueOf(text.getEchoChar()));
            node.addProperty("textLimit", text.getTextLimit());
            node.addProperty("charCount", text.getCharCount());
            node.addProperty("lineCount", text.getLineCount());
            node.addProperty("caretPosition", text.getCaretPosition());

            Point selection = text.getSelection();
            node.addProperty("selectionStart", selection.x);
            node.addProperty("selectionEnd", selection.y);

            int style = text.getStyle();
            node.addProperty("multiLine", (style & SWT.MULTI) != 0);
            node.addProperty("password", (style & SWT.PASSWORD) != 0);
        }

        // Combo
        if (widget instanceof Combo) {
            Combo combo = (Combo) widget;
            node.addProperty("text", combo.getText());
            node.addProperty("selectionIndex", combo.getSelectionIndex());
            node.addProperty("itemCount", combo.getItemCount());

            int style = combo.getStyle();
            node.addProperty("readOnly", (style & SWT.READ_ONLY) != 0);

            JsonArray items = new JsonArray();
            for (String item : combo.getItems()) {
                items.add(item);
            }
            node.add("items", items);
        }

        // List
        if (widget instanceof org.eclipse.swt.widgets.List) {
            org.eclipse.swt.widgets.List list = (org.eclipse.swt.widgets.List) widget;
            node.addProperty("selectionIndex", list.getSelectionIndex());
            node.addProperty("itemCount", list.getItemCount());

            int[] selection = list.getSelectionIndices();
            JsonArray selectedIndices = new JsonArray();
            for (int idx : selection) {
                selectedIndices.add(idx);
            }
            node.add("selectedIndices", selectedIndices);

            JsonArray items = new JsonArray();
            for (String item : list.getItems()) {
                items.add(item);
            }
            node.add("items", items);
        }

        // Table
        if (widget instanceof Table) {
            Table table = (Table) widget;
            node.addProperty("itemCount", table.getItemCount());
            node.addProperty("columnCount", table.getColumnCount());
            node.addProperty("selectionIndex", table.getSelectionIndex());
            node.addProperty("selectionCount", table.getSelectionCount());
            node.addProperty("headerVisible", table.getHeaderVisible());
            node.addProperty("linesVisible", table.getLinesVisible());

            // Column headers
            JsonArray columns = new JsonArray();
            for (TableColumn col : table.getColumns()) {
                JsonObject colInfo = new JsonObject();
                colInfo.addProperty("text", col.getText());
                colInfo.addProperty("width", col.getWidth());
                colInfo.addProperty("resizable", col.getResizable());
                colInfo.addProperty("moveable", col.getMoveable());
                columns.add(colInfo);
            }
            node.add("columns", columns);
        }

        // Tree
        if (widget instanceof Tree) {
            Tree tree = (Tree) widget;
            node.addProperty("itemCount", tree.getItemCount());
            node.addProperty("columnCount", tree.getColumnCount());
            node.addProperty("selectionCount", tree.getSelectionCount());
            node.addProperty("headerVisible", tree.getHeaderVisible());
            node.addProperty("linesVisible", tree.getLinesVisible());

            // Column headers
            JsonArray columns = new JsonArray();
            for (TreeColumn col : tree.getColumns()) {
                JsonObject colInfo = new JsonObject();
                colInfo.addProperty("text", col.getText());
                colInfo.addProperty("width", col.getWidth());
                columns.add(colInfo);
            }
            node.add("columns", columns);
        }

        // TabFolder
        if (widget instanceof TabFolder) {
            TabFolder tabFolder = (TabFolder) widget;
            node.addProperty("selectionIndex", tabFolder.getSelectionIndex());
            node.addProperty("itemCount", tabFolder.getItemCount());

            JsonArray tabs = new JsonArray();
            for (TabItem tab : tabFolder.getItems()) {
                JsonObject tabInfo = new JsonObject();
                tabInfo.addProperty("text", tab.getText());
                tabInfo.addProperty("toolTipText", tab.getToolTipText());
                tabs.add(tabInfo);
            }
            node.add("tabs", tabs);
        }

        // CTabFolder (custom tab folder)
        if (widget instanceof CTabFolder) {
            CTabFolder tabFolder = (CTabFolder) widget;
            node.addProperty("selectionIndex", tabFolder.getSelectionIndex());
            node.addProperty("itemCount", tabFolder.getItemCount());
            node.addProperty("minimized", tabFolder.getMinimized());
            node.addProperty("maximized", tabFolder.getMaximized());

            JsonArray tabs = new JsonArray();
            for (CTabItem tab : tabFolder.getItems()) {
                JsonObject tabInfo = new JsonObject();
                tabInfo.addProperty("text", tab.getText());
                tabInfo.addProperty("toolTipText", tab.getToolTipText());
                tabInfo.addProperty("showing", tab.isShowing());
                tabs.add(tabInfo);
            }
            node.add("tabs", tabs);
        }

        // Spinner
        if (widget instanceof Spinner) {
            Spinner spinner = (Spinner) widget;
            node.addProperty("selection", spinner.getSelection());
            node.addProperty("minimum", spinner.getMinimum());
            node.addProperty("maximum", spinner.getMaximum());
            node.addProperty("increment", spinner.getIncrement());
            node.addProperty("pageIncrement", spinner.getPageIncrement());
            node.addProperty("digits", spinner.getDigits());
        }

        // Scale (slider)
        if (widget instanceof Scale) {
            Scale scale = (Scale) widget;
            node.addProperty("selection", scale.getSelection());
            node.addProperty("minimum", scale.getMinimum());
            node.addProperty("maximum", scale.getMaximum());
            node.addProperty("increment", scale.getIncrement());
            node.addProperty("pageIncrement", scale.getPageIncrement());
        }

        // Slider
        if (widget instanceof Slider) {
            Slider slider = (Slider) widget;
            node.addProperty("selection", slider.getSelection());
            node.addProperty("minimum", slider.getMinimum());
            node.addProperty("maximum", slider.getMaximum());
            node.addProperty("increment", slider.getIncrement());
            node.addProperty("pageIncrement", slider.getPageIncrement());
            node.addProperty("thumb", slider.getThumb());
        }

        // ProgressBar
        if (widget instanceof ProgressBar) {
            ProgressBar progressBar = (ProgressBar) widget;
            node.addProperty("selection", progressBar.getSelection());
            node.addProperty("minimum", progressBar.getMinimum());
            node.addProperty("maximum", progressBar.getMaximum());

            int style = progressBar.getStyle();
            node.addProperty("indeterminate", (style & SWT.INDETERMINATE) != 0);
        }

        // StyledText
        if (widget instanceof org.eclipse.swt.custom.StyledText) {
            org.eclipse.swt.custom.StyledText styledText = (org.eclipse.swt.custom.StyledText) widget;
            node.addProperty("text", styledText.getText());
            node.addProperty("editable", styledText.getEditable());
            node.addProperty("charCount", styledText.getCharCount());
            node.addProperty("lineCount", styledText.getLineCount());
            node.addProperty("caretOffset", styledText.getCaretOffset());

            Point selection = styledText.getSelection();
            node.addProperty("selectionStart", selection.x);
            node.addProperty("selectionEnd", selection.y);
        }

        // Link
        if (widget instanceof Link) {
            Link link = (Link) widget;
            node.addProperty("text", link.getText());
        }

        // Group
        if (widget instanceof Group) {
            Group group = (Group) widget;
            node.addProperty("text", group.getText());
        }

        // ToolBar
        if (widget instanceof ToolBar) {
            ToolBar toolbar = (ToolBar) widget;
            node.addProperty("itemCount", toolbar.getItemCount());
        }

        // Menu
        if (widget instanceof Menu) {
            Menu menu = (Menu) widget;
            node.addProperty("itemCount", menu.getItemCount());
            node.addProperty("visible", menu.isVisible());
            node.addProperty("enabled", menu.isEnabled());
        }

        // MenuItem
        if (widget instanceof MenuItem) {
            MenuItem menuItem = (MenuItem) widget;
            node.addProperty("text", menuItem.getText());
            node.addProperty("enabled", menuItem.isEnabled());
            node.addProperty("selection", menuItem.getSelection());

            int style = menuItem.getStyle();
            if ((style & SWT.CHECK) != 0) {
                node.addProperty("menuItemType", "check");
            } else if ((style & SWT.RADIO) != 0) {
                node.addProperty("menuItemType", "radio");
            } else if ((style & SWT.CASCADE) != 0) {
                node.addProperty("menuItemType", "cascade");
            } else if ((style & SWT.SEPARATOR) != 0) {
                node.addProperty("menuItemType", "separator");
            } else {
                node.addProperty("menuItemType", "push");
            }
        }

        // Browser
        if (widget instanceof Browser) {
            Browser browser = (Browser) widget;
            node.addProperty("url", browser.getUrl());
            node.addProperty("text", browser.getText());
        }
    }

    /**
     * Find a widget by locator criteria.
     *
     * @param locator JsonObject containing locator criteria
     * @return Widget ID or -1 if not found
     */
    public static int findWidget(JsonObject locator) {
        return DisplayHelper.syncExecAndReturn(() -> {
            String type = locator.has("type") ? locator.get("type").getAsString() : "text";
            String value = locator.get("value").getAsString();
            int parentId = locator.has("parent") ? locator.get("parent").getAsInt() : -1;
            int index = locator.has("index") ? locator.get("index").getAsInt() : 0;

            Composite searchRoot = null;
            if (parentId >= 0) {
                Widget parent = widgetCache.get(parentId);
                if (parent instanceof Composite && !parent.isDisposed()) {
                    searchRoot = (Composite) parent;
                }
            }

            List<Widget> matches = new ArrayList<>();
            Display display = DisplayHelper.getDisplay();

            if (searchRoot != null) {
                findWidgets(searchRoot, type, value, matches);
            } else if (display != null && !display.isDisposed()) {
                for (Shell shell : display.getShells()) {
                    if (shell.isVisible()) {
                        findWidgets(shell, type, value, matches);
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
     * Find all widgets matching locator criteria.
     *
     * @param locator JsonObject containing locator criteria
     * @return JsonArray of widget info objects (not just IDs)
     */
    public static JsonArray findAllWidgets(JsonObject locator) {
        return DisplayHelper.syncExecAndReturn(() -> {
            // Support both "type" and "locatorType" parameter names
            String type = "text"; // default
            if (locator.has("locatorType")) {
                type = locator.get("locatorType").getAsString();
            } else if (locator.has("type")) {
                type = locator.get("type").getAsString();
            }

            String value = locator.get("value").getAsString();
            int parentId = locator.has("parent") ? locator.get("parent").getAsInt() : -1;

            Composite searchRoot = null;
            if (parentId >= 0) {
                Widget parent = widgetCache.get(parentId);
                if (parent instanceof Composite && !parent.isDisposed()) {
                    searchRoot = (Composite) parent;
                }
            }

            List<Widget> matches = new ArrayList<>();
            Display display = DisplayHelper.getDisplay();

            if (searchRoot != null) {
                findWidgets(searchRoot, type, value, matches);
            } else if (display != null && !display.isDisposed()) {
                for (Shell shell : display.getShells()) {
                    if (shell.isVisible()) {
                        findWidgets(shell, type, value, matches);
                    }
                }
            }

            // Return full widget info objects, not just IDs
            JsonArray result = new JsonArray();
            for (Widget widget : matches) {
                result.add(buildWidgetInfo(widget));
            }
            return result;
        });
    }

    /**
     * Build basic widget info JSON object.
     */
    private static JsonObject buildWidgetInfo(Widget widget) {
        JsonObject info = new JsonObject();
        info.addProperty("id", getOrCreateId(widget));
        info.addProperty("class", widget.getClass().getName());
        info.addProperty("simpleClass", widget.getClass().getSimpleName());

        if (widget.isDisposed()) {
            info.addProperty("disposed", true);
            return info;
        }

        // Add text
        String text = getWidgetText(widget);
        if (text != null) {
            info.addProperty("text", text);
        }

        // Control-specific properties
        if (widget instanceof Control) {
            Control control = (Control) widget;

            Rectangle bounds = control.getBounds();
            info.addProperty("x", bounds.x);
            info.addProperty("y", bounds.y);
            info.addProperty("width", bounds.width);
            info.addProperty("height", bounds.height);
            info.addProperty("visible", control.isVisible());
            info.addProperty("enabled", control.isEnabled());

            String tooltip = control.getToolTipText();
            if (tooltip != null) {
                info.addProperty("tooltip", tooltip);
            }
        }

        return info;
    }

    /**
     * Recursively find widgets matching criteria.
     */
    private static void findWidgets(Composite composite, String type, String value, List<Widget> matches) {
        if (matchesLocator(composite, type, value)) {
            matches.add(composite);
        }

        for (Control child : composite.getChildren()) {
            if (matchesLocator(child, type, value)) {
                matches.add(child);
            }
            if (child instanceof Composite) {
                findWidgets((Composite) child, type, value, matches);
            }
        }
    }

    /**
     * Check if a widget matches the locator criteria.
     */
    private static boolean matchesLocator(Widget widget, String type, String value) {
        if (widget.isDisposed()) {
            return false;
        }

        switch (type.toLowerCase()) {
            case "text":
                String text = getWidgetText(widget);
                return text != null && text.equals(value);

            case "text_contains":
                String textContains = getWidgetText(widget);
                return textContains != null && textContains.contains(value);

            case "text_regex":
                String textRegex = getWidgetText(widget);
                return textRegex != null && textRegex.matches(value);

            case "class":
                return widget.getClass().getName().equals(value) ||
                       widget.getClass().getSimpleName().equals(value);

            case "tooltip":
                if (widget instanceof Control) {
                    String tooltip = ((Control) widget).getToolTipText();
                    return tooltip != null && tooltip.equals(value);
                }
                return false;

            case "id":
                Integer id = reverseCache.get(widget);
                return id != null && id.toString().equals(value);

            case "data":
                // Check widget data (commonly used for custom identification)
                Object data = widget.getData();
                if (data != null && data.toString().equals(value)) {
                    return true;
                }
                // Also check named data
                Object namedData = widget.getData(value);
                return namedData != null;

            case "data_key":
                // Find widget by data key-value pair
                if (value.contains("=")) {
                    String[] parts = value.split("=", 2);
                    String key = parts[0];
                    String expectedValue = parts[1];
                    Object dataValue = widget.getData(key);
                    return dataValue != null && dataValue.toString().equals(expectedValue);
                }
                return false;

            default:
                return false;
        }
    }

    /**
     * Get text from various widget types.
     */
    private static String getWidgetText(Widget widget) {
        if (widget instanceof Shell) {
            return ((Shell) widget).getText();
        }
        if (widget instanceof Button) {
            return ((Button) widget).getText();
        }
        if (widget instanceof Label) {
            return ((Label) widget).getText();
        }
        if (widget instanceof Text) {
            return ((Text) widget).getText();
        }
        if (widget instanceof Combo) {
            return ((Combo) widget).getText();
        }
        if (widget instanceof Group) {
            return ((Group) widget).getText();
        }
        if (widget instanceof Link) {
            return ((Link) widget).getText();
        }
        if (widget instanceof org.eclipse.swt.custom.StyledText) {
            return ((org.eclipse.swt.custom.StyledText) widget).getText();
        }
        if (widget instanceof MenuItem) {
            return ((MenuItem) widget).getText();
        }
        if (widget instanceof TabItem) {
            return ((TabItem) widget).getText();
        }
        if (widget instanceof CTabItem) {
            return ((CTabItem) widget).getText();
        }
        if (widget instanceof TreeItem) {
            return ((TreeItem) widget).getText();
        }
        if (widget instanceof TableItem) {
            return ((TableItem) widget).getText();
        }
        return null;
    }

    /**
     * Get all properties of a widget.
     *
     * @param widgetId Widget ID
     * @return JsonObject with all properties
     */
    public static JsonObject getWidgetProperties(int widgetId) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = widgetCache.get(widgetId);
            if (widget == null || widget.isDisposed()) {
                throw new IllegalArgumentException("Widget not found or disposed: " + widgetId);
            }

            JsonObject props = new JsonObject();
            props.addProperty("id", widgetId);
            props.addProperty("class", widget.getClass().getName());
            props.addProperty("disposed", widget.isDisposed());

            // Control-specific properties
            if (widget instanceof Control) {
                Control control = (Control) widget;

                Rectangle bounds = control.getBounds();
                JsonObject boundsObj = new JsonObject();
                boundsObj.addProperty("x", bounds.x);
                boundsObj.addProperty("y", bounds.y);
                boundsObj.addProperty("width", bounds.width);
                boundsObj.addProperty("height", bounds.height);
                props.add("bounds", boundsObj);

                // Screen location
                if (control.isVisible()) {
                    try {
                        Point screenLoc = control.toDisplay(0, 0);
                        JsonObject screenLocObj = new JsonObject();
                        screenLocObj.addProperty("x", screenLoc.x);
                        screenLocObj.addProperty("y", screenLoc.y);
                        props.add("screenLocation", screenLocObj);
                    } catch (Exception e) {
                        // Ignore
                    }
                }

                // State flags
                props.addProperty("visible", control.isVisible());
                props.addProperty("enabled", control.isEnabled());
                props.addProperty("focusControl", control.isFocusControl());
                props.addProperty("tooltip", control.getToolTipText());

                // Colors
                Color bg = control.getBackground();
                Color fg = control.getForeground();
                if (bg != null && !bg.isDisposed()) {
                    props.addProperty("background", colorToHex(bg));
                }
                if (fg != null && !fg.isDisposed()) {
                    props.addProperty("foreground", colorToHex(fg));
                }

                // Font
                Font font = control.getFont();
                if (font != null && !font.isDisposed()) {
                    FontData[] fontData = font.getFontData();
                    if (fontData.length > 0) {
                        JsonObject fontObj = new JsonObject();
                        fontObj.addProperty("name", fontData[0].getName());
                        fontObj.addProperty("height", fontData[0].getHeight());
                        fontObj.addProperty("style", fontData[0].getStyle());
                        props.add("font", fontObj);
                    }
                }
            }

            // Type-specific properties
            addTypeSpecificProperties(props, widget);

            // Widget data
            Object data = widget.getData();
            if (data != null) {
                props.addProperty("data", data.toString());
            }

            return props;
        });
    }

    /**
     * Get or create a unique ID for a widget.
     */
    public static int getOrCreateId(Widget widget) {
        Integer existing = reverseCache.get(widget);
        if (existing != null) {
            return existing;
        }

        int id = widgetIdCounter.incrementAndGet();
        widgetCache.put(id, widget);
        reverseCache.put(widget, id);
        return id;
    }

    /**
     * Get a widget by ID.
     *
     * @param id Widget ID
     * @return Widget or null if not found/disposed
     */
    public static Widget getWidgetById(int id) {
        Widget widget = widgetCache.get(id);
        if (widget != null && widget.isDisposed()) {
            widgetCache.remove(id);
            reverseCache.remove(widget);
            return null;
        }
        return widget;
    }

    /**
     * Convert Color to hex string.
     */
    private static String colorToHex(Color color) {
        return String.format("#%02x%02x%02x", color.getRed(), color.getGreen(), color.getBlue());
    }

    /**
     * Clear the widget cache.
     */
    public static void clearCache() {
        widgetCache.clear();
        reverseCache.clear();
    }

    /**
     * Clean up disposed widgets from cache.
     */
    public static void cleanupCache() {
        synchronized (widgetCache) {
            widgetCache.entrySet().removeIf(entry ->
                entry.getValue() == null || entry.getValue().isDisposed());
        }
        synchronized (reverseCache) {
            reverseCache.entrySet().removeIf(entry ->
                entry.getKey() == null || entry.getKey().isDisposed());
        }
    }
}
