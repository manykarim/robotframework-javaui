package com.robotframework.swing;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonNull;
import com.google.gson.JsonObject;
import com.google.gson.JsonPrimitive;

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
    // Use HashMap instead of WeakHashMap to prevent component IDs from being garbage collected
    // This is important for modal dialogs where component references might not be held during GC
    private static final Map<Integer, Component> componentCache = Collections.synchronizedMap(new java.util.HashMap<>());
    private static final Map<Component, Integer> reverseCache = Collections.synchronizedMap(new java.util.HashMap<>());

    // --- AppContext-aware window enumeration -------------------------------------------------
    // java.awt.Window.getWindows() is AppContext-scoped: an agent attached on the RPC socket
    // thread only sees windows created in ITS AppContext, so it MISSES windows created in a
    // separate AppContext — exactly what Java Web Start and applets do (proven empirically).
    // allWindows() enumerates every AppContext and merges their window lists, opening the
    // required java.desktop internals via the agent's Instrumentation. It falls back to the
    // plain Window.getWindows() when those internals cannot be opened, so behavior never
    // regresses on the common single-context case or on locked-down JVMs.
    private static volatile boolean appCtxAccessTried = false;
    private static Method windowsByCtx;   // java.awt.Window.getWindows(sun.awt.AppContext)
    private static Method getAppContexts; // sun.awt.AppContext.getAppContexts()

    static Window[] allWindows() {
        ensureAppContextAccess();
        if (windowsByCtx != null && getAppContexts != null) {
            try {
                java.util.LinkedHashSet<Window> all = new java.util.LinkedHashSet<>();
                Object ctxs = getAppContexts.invoke(null);
                if (ctxs instanceof java.util.Collection) {
                    for (Object ctx : (java.util.Collection<?>) ctxs) {
                        Object arr = windowsByCtx.invoke(null, ctx);
                        if (arr instanceof Window[]) {
                            java.util.Collections.addAll(all, (Window[]) arr);
                        }
                    }
                }
                if (!all.isEmpty()) {
                    return all.toArray(new Window[0]);
                }
            } catch (Throwable ignore) {
                // fall through to the single-context path
            }
        }
        return Window.getWindows();
    }

    private static synchronized void ensureAppContextAccess() {
        if (appCtxAccessTried) {
            return;
        }
        appCtxAccessTried = true;
        try {
            Module javaDesktop = Window.class.getModule();
            Module self = ComponentInspector.class.getModule();
            java.lang.instrument.Instrumentation inst = com.robotframework.UnifiedAgent.getInstrumentation();
            if (inst != null && javaDesktop.isNamed()) {
                // Open sun.awt (AppContext) + java.awt (private getWindows) to our module.
                java.util.Map<String, java.util.Set<Module>> pkgs = new java.util.HashMap<>();
                pkgs.put("sun.awt", java.util.Set.of(self));
                pkgs.put("java.awt", java.util.Set.of(self));
                inst.redefineModule(javaDesktop, java.util.Set.of(), pkgs, pkgs,
                        java.util.Set.of(), java.util.Map.of());
            }
            Class<?> appCtx = Class.forName("sun.awt.AppContext");
            getAppContexts = appCtx.getMethod("getAppContexts");
            getAppContexts.setAccessible(true);
            windowsByCtx = Window.class.getDeclaredMethod("getWindows", appCtx);
            windowsByCtx.setAccessible(true);
        } catch (Throwable t) {
            windowsByCtx = null;
            getAppContexts = null;  // fall back to single-context Window.getWindows()
        }
    }

    /**
     * Get all visible frames/windows in the application.
     *
     * @return JsonArray of window information
     */
    public static JsonArray getWindows() {
        return EdtHelper.runOnEdtAndReturn(() -> {
            JsonArray windows = new JsonArray();

            for (Window window : allWindows()) {
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
     * When no explicit depth is requested this returns the COMPLETE tree
     * (unbounded depth). A shallow default (previously 10) silently hid deeply
     * nested widgets from the locator engine, which fetches with no maxDepth —
     * real applications (e.g. the JGoodies Showcase) nest interactive widgets
     * well below depth 10, making them unreachable by find/click.
     *
     * @return JsonObject representing the full component tree
     */
    public static JsonObject getComponentTree() {
        return getComponentTree(Integer.MAX_VALUE);  // Unbounded: "no maxDepth" means the full tree
    }

    /**
     * Get the full component tree starting from root frames with specified max depth.
     *
     * @param maxDepth Maximum depth to traverse (0 = only roots, no children)
     * @return JsonObject representing the component tree
     */
    public static JsonObject getComponentTree(int maxDepth) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            JsonObject result = new JsonObject();
            JsonArray roots = new JsonArray();

            for (Window window : allWindows()) {
                if (window.isShowing() && !isSpyOverlay(window)) {
                    roots.add(buildComponentNode(window, 0, maxDepth));
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
    // ================= javagui-spy: hit-test / highlight / generation =================
    /** Reserved name so spy overlays never appear in their own scans. */
    public static final String SPY_OVERLAY_NAME = "__javagui_spy_overlay__";

    private static boolean isSpyOverlay(Component c) {
        return c != null && SPY_OVERLAY_NAME.equals(c.getName());
    }

    private static String textOf(Component c) {
        try {
            if (c instanceof AbstractButton) return ((AbstractButton) c).getText();
            if (c instanceof JLabel) return ((JLabel) c).getText();
            if (c instanceof JTextComponent) return ((JTextComponent) c).getText();
            if (c instanceof Frame) return ((Frame) c).getTitle();
        } catch (Exception ignore) {}
        return null;
    }

    /** Deepest visible component at screen point (x,y), with its root->leaf ancestor id path. */
    public static JsonObject hitTest(int screenX, int screenY) {
        JsonObject r = EdtHelper.runOnEdtAndReturn(() -> {
            Component target = null;
            for (Window w : allWindows()) {
                if (!w.isShowing() || isSpyOverlay(w)) continue;
                Point o;
                try { o = w.getLocationOnScreen(); } catch (Exception e) { continue; }
                Rectangle b = new Rectangle(o.x, o.y, w.getWidth(), w.getHeight());
                if (!b.contains(screenX, screenY)) continue;
                Component c = SwingUtilities.getDeepestComponentAt(w, screenX - o.x, screenY - o.y);
                if (c != null && !isSpyOverlay(c)) target = c; // later window in stacking order wins
            }
            JsonObject node = new JsonObject();
            if (target == null) { node.addProperty("hit", false); return node; }
            node.addProperty("hit", true);
            node.addProperty("id", getOrCreateId(target));
            node.addProperty("class", target.getClass().getName());
            node.addProperty("simpleClass", target.getClass().getSimpleName());
            if (target.getName() != null) node.addProperty("name", target.getName());
            String txt = textOf(target);
            if (txt != null) node.addProperty("text", txt);
            try {
                Point sp = target.getLocationOnScreen();
                node.addProperty("screenX", sp.x);
                node.addProperty("screenY", sp.y);
            } catch (Exception ignore) {}
            node.addProperty("width", target.getWidth());
            node.addProperty("height", target.getHeight());
            JsonArray path = new JsonArray();
            java.util.List<Component> chain = new java.util.ArrayList<>();
            for (Component c = target; c != null; c = c.getParent()) chain.add(c);
            java.util.Collections.reverse(chain);
            for (Component c : chain) path.add(getOrCreateId(c));
            node.add("ancestor_path", path);
            return node;
        });
        return r != null ? r : new JsonObject();
    }

    /** Flash a hollow, non-focusable, always-on-top red border around a component; auto-disposes. */
    public static JsonObject highlight(int componentId, int durationMs) {
        JsonObject res = new JsonObject();
        final Component c = componentCache.get(componentId);
        if (c == null) { res.addProperty("ok", false); res.addProperty("error", "unknown component id"); return res; }
        final int dur = durationMs <= 0 ? 1500 : Math.max(200, durationMs);
        EdtHelper.runOnEdt(() -> {
            try {
                if (!c.isShowing()) return;
                Point o = c.getLocationOnScreen();
                final JWindow w = new JWindow();
                w.setName(SPY_OVERLAY_NAME);
                w.setFocusableWindowState(false);
                w.setAlwaysOnTop(true);
                JPanel p = new JPanel() {
                    @Override protected void paintComponent(Graphics g) {
                        Graphics2D g2 = (Graphics2D) g;
                        g2.setColor(new Color(255, 60, 60));
                        g2.setStroke(new BasicStroke(3f));
                        g2.drawRect(1, 1, getWidth() - 3, getHeight() - 3);
                    }
                };
                p.setOpaque(false);
                w.setContentPane(p);
                w.setBounds(o.x - 2, o.y - 2, c.getWidth() + 4, c.getHeight() + 4);
                try {
                    java.awt.geom.Area ring = new java.awt.geom.Area(new Rectangle(0, 0, w.getWidth(), w.getHeight()));
                    ring.subtract(new java.awt.geom.Area(new Rectangle(4, 4, w.getWidth() - 8, w.getHeight() - 8)));
                    w.setShape(ring);
                } catch (Exception ignore) {}
                w.setVisible(true);
                javax.swing.Timer t = new javax.swing.Timer(dur, ev -> w.dispose());
                t.setRepeats(false);
                t.start();
            } catch (Exception ignore) {}
        });
        res.addProperty("ok", true);
        return res;
    }

    /** Cheap change token: component counts across windows + focus owner identity. */
    public static JsonObject getUiGeneration() {
        Long gen = EdtHelper.runOnEdtAndReturn(() -> {
            long g = 0;
            for (Window w : allWindows()) {
                if (!w.isShowing() || isSpyOverlay(w)) continue;
                g = g * 1000003L + countComponents(w);
            }
            Component fo = KeyboardFocusManager.getCurrentKeyboardFocusManager().getFocusOwner();
            if (fo != null) g = g * 31L + System.identityHashCode(fo);
            return g;
        });
        JsonObject res = new JsonObject();
        res.addProperty("generation", gen == null ? 0L : gen);
        return res;
    }

    private static int countComponents(Component c) {
        int n = 1;
        if (c instanceof Container) {
            for (Component ch : ((Container) c).getComponents()) n += countComponents(ch);
        }
        return n;
    }

    /** Build the same node shape hitTest returns, on the EDT. */
    private static JsonObject componentNodeOnEdt(Component target) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            JsonObject node = new JsonObject();
            node.addProperty("hit", true);
            node.addProperty("id", getOrCreateId(target));
            node.addProperty("class", target.getClass().getName());
            node.addProperty("simpleClass", target.getClass().getSimpleName());
            if (target.getName() != null) node.addProperty("name", target.getName());
            String txt = textOf(target);
            if (txt != null) node.addProperty("text", txt);
            try {
                Point sp = target.getLocationOnScreen();
                node.addProperty("screenX", sp.x);
                node.addProperty("screenY", sp.y);
            } catch (Exception ignore) {}
            node.addProperty("width", target.getWidth());
            node.addProperty("height", target.getHeight());
            JsonArray path = new JsonArray();
            java.util.List<Component> chain = new java.util.ArrayList<>();
            for (Component c = target; c != null; c = c.getParent()) chain.add(c);
            java.util.Collections.reverse(chain);
            for (Component c : chain) path.add(getOrCreateId(c));
            node.add("ancestor_path", path);
            return node;
        });
    }

    /** Wait (up to timeoutMs) for the user to Ctrl+Shift+click a widget; return the picked node.
     *  Passive: it records the click but does not suppress it (documented caveat). */
    public static JsonObject armPick(int timeoutMs) {
        final int wait = timeoutMs <= 0 ? 15000 : Math.max(1000, timeoutMs);
        final Object lock = new Object();
        final Component[] picked = {null};
        java.awt.event.AWTEventListener listener = ev -> {
            if (ev instanceof java.awt.event.MouseEvent) {
                java.awt.event.MouseEvent me = (java.awt.event.MouseEvent) ev;
                if (me.getID() == java.awt.event.MouseEvent.MOUSE_PRESSED
                        && me.isControlDown() && me.isShiftDown()) {
                    Component src = me.getComponent();
                    Component deep = (src != null)
                            ? SwingUtilities.getDeepestComponentAt(src, me.getX(), me.getY()) : null;
                    synchronized (lock) {
                        if (picked[0] == null) { picked[0] = (deep != null) ? deep : src; lock.notifyAll(); }
                    }
                }
            }
        };
        Toolkit.getDefaultToolkit().addAWTEventListener(listener, java.awt.AWTEvent.MOUSE_EVENT_MASK);
        try {
            synchronized (lock) {
                long deadline = System.currentTimeMillis() + wait;
                while (picked[0] == null && System.currentTimeMillis() < deadline) lock.wait(200);
            }
        } catch (InterruptedException ie) {
            Thread.currentThread().interrupt();
        } finally {
            Toolkit.getDefaultToolkit().removeAWTEventListener(listener);
        }
        if (picked[0] == null) {
            JsonObject miss = new JsonObject();
            miss.addProperty("hit", false);
            miss.addProperty("timeout", true);
            return miss;
        }
        return componentNodeOnEdt(picked[0]);
    }
    // ================= end javagui-spy =================

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
            String selectedText = selected != null ? selected.toString() : "";
            node.addProperty("selectedItem", selectedText);
            node.addProperty("text", selectedText);  // Also set text for get_element_text
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
            // Add text property for selected value
            Object selectedValue = list.getSelectedValue();
            node.addProperty("text", selectedValue != null ? selectedValue.toString() : "");
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
            node.addProperty("text", String.valueOf(slider.getValue()));  // text for get_element_text
        }

        // Spinner
        if (component instanceof JSpinner) {
            JSpinner spinner = (JSpinner) component;
            Object value = spinner.getValue();
            String valueStr = value != null ? value.toString() : "";
            node.addProperty("value", valueStr);
            node.addProperty("text", valueStr);  // text for get_element_text
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
                for (Window window : allWindows()) {
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
                for (Window window : allWindows()) {
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

    /**
     * Get a specific property value from a component.
     *
     * @param componentId Component ID
     * @param propertyName Property name (e.g., "value", "text", "selectedIndex")
     * @return Property value as JsonElement
     */
    public static JsonElement getProperty(int componentId, String propertyName) {
        return EdtHelper.runOnEdtAndReturn(() -> {
            Component component = componentCache.get(componentId);
            if (component == null) {
                throw new IllegalArgumentException("Component not found: " + componentId);
            }

            String propLower = propertyName.toLowerCase();

            // Handle common properties
            switch (propLower) {
                case "value":
                    if (component instanceof JProgressBar) {
                        return new JsonPrimitive(((JProgressBar) component).getValue());
                    }
                    if (component instanceof JSlider) {
                        return new JsonPrimitive(((JSlider) component).getValue());
                    }
                    if (component instanceof JSpinner) {
                        Object value = ((JSpinner) component).getValue();
                        return new JsonPrimitive(value != null ? value.toString() : "");
                    }
                    break;

                case "percentcomplete":
                    if (component instanceof JProgressBar) {
                        return new JsonPrimitive(((JProgressBar) component).getPercentComplete());
                    }
                    break;

                case "minimum":
                    if (component instanceof JProgressBar) {
                        return new JsonPrimitive(((JProgressBar) component).getMinimum());
                    }
                    if (component instanceof JSlider) {
                        return new JsonPrimitive(((JSlider) component).getMinimum());
                    }
                    break;

                case "maximum":
                    if (component instanceof JProgressBar) {
                        return new JsonPrimitive(((JProgressBar) component).getMaximum());
                    }
                    if (component instanceof JSlider) {
                        return new JsonPrimitive(((JSlider) component).getMaximum());
                    }
                    break;

                case "selectedindex":
                    if (component instanceof JTabbedPane) {
                        return new JsonPrimitive(((JTabbedPane) component).getSelectedIndex());
                    }
                    if (component instanceof JComboBox) {
                        return new JsonPrimitive(((JComboBox<?>) component).getSelectedIndex());
                    }
                    if (component instanceof JList) {
                        return new JsonPrimitive(((JList<?>) component).getSelectedIndex());
                    }
                    break;

                case "tabcount":
                    if (component instanceof JTabbedPane) {
                        return new JsonPrimitive(((JTabbedPane) component).getTabCount());
                    }
                    break;

                case "text":
                    String text = getComponentText(component);
                    return new JsonPrimitive(text != null ? text : "");

                case "enabled":
                    return new JsonPrimitive(component.isEnabled());

                case "visible":
                    return new JsonPrimitive(component.isVisible());

                case "showing":
                    return new JsonPrimitive(component.isShowing());

                case "selected":
                    if (component instanceof AbstractButton) {
                        return new JsonPrimitive(((AbstractButton) component).isSelected());
                    }
                    break;

                case "editable":
                    if (component instanceof JTextComponent) {
                        return new JsonPrimitive(((JTextComponent) component).isEditable());
                    }
                    if (component instanceof JComboBox) {
                        return new JsonPrimitive(((JComboBox<?>) component).isEditable());
                    }
                    break;

                case "indeterminate":
                    if (component instanceof JProgressBar) {
                        return new JsonPrimitive(((JProgressBar) component).isIndeterminate());
                    }
                    break;

                case "rowcount":
                    if (component instanceof JTable) {
                        return new JsonPrimitive(((JTable) component).getRowCount());
                    }
                    if (component instanceof JTree) {
                        return new JsonPrimitive(((JTree) component).getRowCount());
                    }
                    break;

                case "columncount":
                    if (component instanceof JTable) {
                        return new JsonPrimitive(((JTable) component).getColumnCount());
                    }
                    break;

                case "itemcount":
                    if (component instanceof JList) {
                        return new JsonPrimitive(((JList<?>) component).getModel().getSize());
                    }
                    if (component instanceof JComboBox) {
                        return new JsonPrimitive(((JComboBox<?>) component).getItemCount());
                    }
                    break;
            }

            // Try reflection as fallback
            try {
                String getterName = "get" + propertyName.substring(0, 1).toUpperCase() + propertyName.substring(1);
                java.lang.reflect.Method getter = component.getClass().getMethod(getterName);
                Object value = getter.invoke(component);
                if (value != null) {
                    if (value instanceof Number) {
                        return new JsonPrimitive((Number) value);
                    } else if (value instanceof Boolean) {
                        return new JsonPrimitive((Boolean) value);
                    } else {
                        return new JsonPrimitive(value.toString());
                    }
                }
            } catch (NoSuchMethodException | IllegalAccessException | java.lang.reflect.InvocationTargetException e) {
                // Ignore reflection errors
            }

            // Return null if property not found
            return JsonNull.INSTANCE;
        });
    }
}
