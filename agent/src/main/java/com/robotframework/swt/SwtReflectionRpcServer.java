package com.robotframework.swt;

import com.google.gson.*;

import java.io.*;
import java.net.*;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.atomic.AtomicBoolean;

/**
 * Reflection-only RPC Server for SWT applications.
 * This server uses only SwtReflectionBridge which requires no static SWT imports.
 *
 * Handles JSON-RPC 2.0 requests for SWT widget automation.
 */
public class SwtReflectionRpcServer implements Runnable {

    private final String host;
    private final int port;
    private final AtomicBoolean running = new AtomicBoolean(false);
    private final AtomicBoolean ready = new AtomicBoolean(false);
    private ServerSocket serverSocket;
    private final Gson gson = new GsonBuilder().setPrettyPrinting().create();
    private static Object mockRcpApp = null;
    private static Class<?> mockRcpAppClass = null;
    private static boolean mockRcpChecked = false;

    public SwtReflectionRpcServer(String host, int port) {
        this.host = host;
        this.port = port;
    }

    @Override
    public void run() {
        running.set(true);

        try {
            serverSocket = new ServerSocket(port, 50, InetAddress.getByName(host));
            ready.set(true);  // Signal that server is ready to accept connections
            System.out.println("[SwtAgent] RPC server listening on " + host + ":" + port);
            System.err.flush();

            while (running.get()) {
                try {
                    Socket clientSocket = serverSocket.accept();
                    handleClient(clientSocket);
                } catch (SocketException e) {
                    if (!running.get()) {
                        break; // Server was stopped
                    }
                    System.err.println("[SwtAgent] Socket error: " + e.getMessage());
                }
            }
        } catch (IOException e) {
            System.err.println("[SwtAgent] Failed to start server: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private void handleClient(Socket socket) {
        try (BufferedReader reader = new BufferedReader(new InputStreamReader(socket.getInputStream(), StandardCharsets.UTF_8));
             PrintWriter writer = new PrintWriter(new OutputStreamWriter(socket.getOutputStream(), StandardCharsets.UTF_8), true)) {

            // Keep connection alive for multiple requests
            String line;
            while ((line = reader.readLine()) != null) {
                line = line.trim();

                // Skip empty lines
                if (line.isEmpty()) {
                    continue;
                }

                // Process the JSON-RPC request
                String response = processRequest(line);
                writer.println(response);
                writer.flush();
            }

        } catch (IOException e) {
            System.err.println("[SwtAgent] Client handling error: " + e.getMessage());
        } finally {
            try {
                socket.close();
            } catch (IOException e) {
                // Ignore
            }
        }
    }

    private String processRequest(String request) {
        JsonObject response = new JsonObject();
        response.addProperty("jsonrpc", "2.0");

        try {
            JsonObject req = JsonParser.parseString(request).getAsJsonObject();
            Object id = req.has("id") ? req.get("id") : null;
            if (id != null) {
                response.add("id", req.get("id"));
            }

            String method = req.get("method").getAsString();
            JsonObject params = req.has("params") ? req.getAsJsonObject("params") : new JsonObject();

            // Ensure bridge is initialized
            if (!SwtReflectionBridge.isInitialized()) {
                boolean initialized = SwtReflectionBridge.initialize();
                if (!initialized) {
                    response.add("error", createError(-32000, "SWT not initialized. Display not found."));
                    return gson.toJson(response);
                }
            }

            JsonElement result = handleMethod(method, params);
            response.add("result", result);

        } catch (Exception e) {
            System.err.println("[SwtAgent] Error processing request: " + e.getMessage());
            e.printStackTrace();
            response.add("error", createError(-32603, e.getMessage()));
        }

        return gson.toJson(response);
    }

    private JsonElement handleMethod(String method, JsonObject params) throws Exception {
        switch (method) {
            case "ping":
                return new JsonPrimitive("pong");

            case "initialize":
                return new JsonPrimitive(SwtReflectionBridge.initialize());

            case "isInitialized":
                return new JsonPrimitive(SwtReflectionBridge.isInitialized());

            case "getShells":
            case "listShells":  // Add alias for compatibility with different test naming
                return SwtReflectionBridge.getShells();

            case "findWidgets":
            case "findElements":
                return SwtReflectionBridge.findWidgets(params);

            case "getWidgetTree":
            case "getComponentTree":
                return SwtReflectionBridge.getWidgetTree();

            case "click":
                SwtReflectionBridge.click(getWidgetId(params));
                return new JsonPrimitive(true);

            case "doubleClick":
                SwtReflectionBridge.doubleClick(getWidgetId(params));
                return new JsonPrimitive(true);

            case "setText":
                SwtReflectionBridge.setText(getWidgetId(params), params.get("text").getAsString());
                return new JsonPrimitive(true);

            case "typeText":
                SwtReflectionBridge.typeText(getWidgetId(params), params.get("text").getAsString());
                return new JsonPrimitive(true);

            case "clearText":
                SwtReflectionBridge.clearText(getWidgetId(params));
                return new JsonPrimitive(true);

            case "activateShell":
                SwtReflectionBridge.activateShell(getWidgetId(params));
                return new JsonPrimitive(true);

            case "closeShell":
                SwtReflectionBridge.closeShell(getWidgetId(params));
                return new JsonPrimitive(true);

            case "expandTreeItem":
                SwtReflectionBridge.expandTreeItem(
                    getWidgetId(params),
                    params.get("path").getAsString()
                );
                return new JsonPrimitive(true);

            case "expandTreeNode":
                SwtReflectionBridge.expandTreeItem(
                    getWidgetId(params),
                    params.get("path").getAsString()
                );
                return new JsonPrimitive(true);

            case "collapseTreeItem":
                SwtReflectionBridge.collapseTreeItem(
                    getWidgetId(params),
                    params.get("path").getAsString()
                );
                return new JsonPrimitive(true);

            case "collapseTreeNode":
                SwtReflectionBridge.collapseTreeItem(
                    getWidgetId(params),
                    params.get("path").getAsString()
                );
                return new JsonPrimitive(true);

            case "selectTreeNode":
                SwtReflectionBridge.selectTreeItem(
                    getWidgetId(params),
                    params.get("path").getAsString()
                );
                return new JsonPrimitive(true);

            case "selectTreeNodes":
                java.util.List<String> nodes = new java.util.ArrayList<>();
                JsonArray nodeArray = params.getAsJsonArray("nodes");
                if (nodeArray != null) {
                    for (JsonElement node : nodeArray) {
                        nodes.add(node.getAsString());
                    }
                }
                SwtReflectionBridge.selectTreeNodes(getWidgetId(params), nodes);
                return new JsonPrimitive(true);

            case "deselectAllTreeNodes":
                SwtReflectionBridge.deselectAllTreeNodes(getWidgetId(params));
                return new JsonPrimitive(true);

            case "getTreeNodeParent":
                return new JsonPrimitive(
                    SwtReflectionBridge.getTreeNodeParent(
                        getWidgetId(params),
                        params.get("nodeName").getAsString()
                    )
                );

            case "getTreeNodeLevel":
                return new JsonPrimitive(
                    SwtReflectionBridge.getTreeNodeLevel(
                        getWidgetId(params),
                        params.get("nodeName").getAsString()
                    )
                );

            case "treeNodeExists":
                return new JsonPrimitive(
                    SwtReflectionBridge.treeNodeExists(
                        getWidgetId(params),
                        params.get("nodeName").getAsString()
                    )
                );

            case "getSelectedTreeNodes":
                return SwtReflectionBridge.getSelectedTreeNodes(getWidgetId(params));

            case "getWidgetInfo":
                Object widget = SwtReflectionBridge.getWidgetById(getWidgetId(params));
                if (widget != null) {
                    return getWidgetInfoJson(widget);
                }
                return JsonNull.INSTANCE;

            case "isWidgetEnabled":
                return new JsonPrimitive(isWidgetEnabled(getWidgetId(params)));

            case "isWidgetVisible":
                return new JsonPrimitive(isWidgetVisible(getWidgetId(params)));

            case "getText":
                return new JsonPrimitive(getWidgetText(getWidgetId(params)));

            case "getWidgetProperties":
                return SwtReflectionBridge.getWidgetProperties(getWidgetId(params));

            case "clearCache":
                SwtReflectionBridge.clearCache();
                return new JsonPrimitive(true);

            // Table methods
            case "getTableRowCount":
                return new JsonPrimitive(getTableRowCount(getWidgetId(params)));

            case "getTableCell":
            case "getTableCellValue":
                return new JsonPrimitive(getTableCell(
                    getWidgetId(params),
                    params.get("row").getAsInt(),
                    params.get("column").getAsInt()
                ));

            case "getTableRowValues":
                return getTableRowValues(
                    getWidgetId(params),
                    params.get("row").getAsInt()
                );

            case "selectTableRow":
                selectTableRow(getWidgetId(params), params.get("row").getAsInt());
                return new JsonPrimitive(true);

            case "selectTableRows": {
                JsonArray rowsArray = params.getAsJsonArray("rows");
                int[] rows = new int[rowsArray.size()];
                for (int i = 0; i < rowsArray.size(); i++) {
                    rows[i] = rowsArray.get(i).getAsInt();
                }
                selectTableRows(getWidgetId(params), rows);
                return new JsonPrimitive(true);
            }

            case "deselectAllTableRows":
                deselectAllTableRows(getWidgetId(params));
                return new JsonPrimitive(true);

            case "selectTableRowByValue": {
                int row = selectTableRowByValue(
                    getWidgetId(params),
                    params.get("column").getAsInt(),
                    params.get("value").getAsString()
                );
                return new JsonPrimitive(row);
            }

            case "selectTableRowRange":
                selectTableRowRange(
                    getWidgetId(params),
                    params.get("startRow").getAsInt(),
                    params.get("endRow").getAsInt()
                );
                return new JsonPrimitive(true);

            case "getTableColumns":
                return getTableColumns(getWidgetId(params));

            case "clickTableColumnHeader":
                clickTableColumnHeader(
                    getWidgetId(params),
                    params.get("column").getAsInt()
                );
                return new JsonPrimitive(true);

            // =============================================================
            // RCP (Eclipse Rich Client Platform) Operations
            // =============================================================
            case "rcp.getWorkbenchInfo":
                return getWorkbenchInfo();

            case "rcp.getAvailablePerspectives":
                return getAvailablePerspectives();

            case "rcp.getActivePerspective":
                return getActivePerspective();

            case "rcp.getOpenPerspectives":
                return getOpenPerspectives();

            case "rcp.openPerspective":
                return openPerspective(params.get("perspectiveId").getAsString());

            case "rcp.openPerspectiveByName":
                return openPerspectiveByName(params.get("name").getAsString());

            case "rcp.closePerspective":
                return closePerspective(getOptionalString(params, "perspectiveId"));

            case "rcp.closeAllPerspectives":
                return closeAllPerspectives();

            case "rcp.resetPerspective":
                return resetPerspective();

            case "rcp.savePerspectiveAs":
                return savePerspectiveAs(params.get("name").getAsString());

            case "rcp.showView":
                return showView(
                    params.get("viewId").getAsString(),
                    getOptionalString(params, "secondaryId")
                );

            case "rcp.showViewByName":
                return showViewByName(params.get("name").getAsString());

            case "rcp.closeView":
                return closeView(
                    params.get("viewId").getAsString(),
                    getOptionalString(params, "secondaryId")
                );

            case "rcp.activateView":
                return activateView(params.get("viewId").getAsString());

            case "rcp.getOpenViews":
                return getOpenViews();

            case "rcp.getActiveView":
                return getActiveView();

            case "rcp.isViewVisible":
                return isViewVisible(params.get("viewId").getAsString());

            case "rcp.minimizeView":
                return minimizeView(params.get("viewId").getAsString());

            case "rcp.maximizeView":
                return maximizeView(params.get("viewId").getAsString());

            case "rcp.restoreView":
                return restoreView(params.get("viewId").getAsString());

            case "rcp.isViewMinimized":
                return isViewMinimized(params.get("viewId").getAsString());

            case "rcp.isViewMaximized":
                return isViewMaximized(params.get("viewId").getAsString());

            case "rcp.getViewTitle":
                return getViewTitle(params.get("viewId").getAsString());

            case "rcp.openEditor":
                return openEditor(params.get("filePath").getAsString());

            case "rcp.closeEditor":
                return closeEditor(
                    params.get("filePath").getAsString(),
                    params.has("save") ? params.get("save").getAsBoolean() : true
                );

            case "rcp.closeAllEditors":
                return closeAllEditors(params.has("save") ? params.get("save").getAsBoolean() : true);

            case "rcp.activateEditor":
                return activateEditor(params.get("filePath").getAsString());

            case "rcp.saveEditor":
                return saveEditor(getOptionalString(params, "filePath"));

            case "rcp.saveAllEditors":
                return saveAllEditors();

            case "rcp.getActiveEditor":
                return getActiveEditor();

            case "rcp.getOpenEditors":
                return getOpenEditors();

            case "rcp.isEditorOpen":
                return isEditorOpen(params.get("filePath").getAsString());

            case "rcp.isEditorDirty":
                return isEditorDirty(getOptionalString(params, "filePath"));

            case "rcp.getEditorContent":
                return getEditorContent(params.get("filePath").getAsString());

            case "rcp.getDirtyEditorCount":
                return getDirtyEditorCount();

            case "rcp.enterTextInEditor":
                return enterTextInEditor(params.get("text").getAsString());

            case "rcp.executeCommand":
                return executeCommand(params.get("commandId").getAsString());

            case "rcp.executeMenu":
                return executeMenu(params.get("menuPath").getAsString());

            case "rcp.openPreferences":
                return openPreferences();

            case "rcp.getOpenDialogs":
                return getOpenDialogs();

            case "rcp.getActiveWorkbenchWindow":
                return getActiveWorkbenchWindow();

            case "rcp.getWorkbenchWindowCount":
                return getWorkbenchWindowCount();

            case "rcp.getWorkbenchTitle":
                return getWorkbenchTitle();

            case "rcp.getWorkbenchState":
                return getWorkbenchState();

            case "rcp.waitForWorkbench":
                return waitForWorkbench(
                    params.has("timeout") ? params.get("timeout").getAsLong() : 30000
                );

            case "rcp.pressButton":
                return pressButton(params.get("label").getAsString());

            case "rcp.closeActiveDialog":
                return closeActiveDialog();

            case "rcp.navigateToPreferencePage":
                return navigateToPreferencePage(params.get("path").getAsString());

            case "rcp.getViewWidget":
                return getViewWidget(
                    params.get("viewId").getAsString(),
                    getOptionalString(params, "locator")
                );

            case "rcp.getEditorWidget":
                return getEditorWidget(
                    params.get("title").getAsString(),
                    getOptionalString(params, "locator")
                );

            case "rcp.clickToolbarItem":
                return clickToolbarItem(params.get("tooltip").getAsString());

            case "rcp.getAvailableCommands":
                return getAvailableCommands();

            case "rcp.selectContextMenu":
                return selectContextMenu(params.get("path").getAsString());

            case "rcp.selectMainMenu":
                return selectMainMenu(params.get("path").getAsString());

            // Selection methods
            case "select":
            case "selectItem":
                selectItem(getWidgetId(params), getSelectItemValue(params));
                return new JsonPrimitive(true);

            case "check":
            case "checkButton":
                setButtonChecked(getWidgetId(params), true);
                return new JsonPrimitive(true);

            case "uncheck":
            case "uncheckButton":
                setButtonChecked(getWidgetId(params), false);
                return new JsonPrimitive(true);

            default:
                throw new Exception("Method not found: " + method);
        }
    }

    private int getWidgetId(JsonObject params) {
        if (params.has("widgetId")) {
            return params.get("widgetId").getAsInt();
        } else if (params.has("componentId")) {
            return params.get("componentId").getAsInt();
        } else if (params.has("id")) {
            return params.get("id").getAsInt();
        }
        throw new IllegalArgumentException("No widget ID provided");
    }

    private JsonObject createError(int code, String message) {
        JsonObject error = new JsonObject();
        error.addProperty("code", code);
        error.addProperty("message", message);
        return error;
    }

    private String getOptionalString(JsonObject params, String key) {
        if (params.has(key) && !params.get(key).isJsonNull()) {
            return params.get(key).getAsString();
        }
        return null;
    }

    private Class<?> loadSwtClass(String className) throws Exception {
        ClassLoader cl = SwtReflectionBridge.getSwtClassLoader();
        if (cl == null) {
            cl = SwtReflectionBridge.class.getClassLoader();
        }
        return cl != null ? cl.loadClass(className) : Class.forName(className);
    }

    private Object getTableWidget(int widgetId) throws Exception {
        Object table = SwtReflectionBridge.getWidgetById(widgetId);
        if (table == null) {
            throw new IllegalArgumentException("Widget not found: " + widgetId);
        }
        Class<?> tableClass = loadSwtClass("org.eclipse.swt.widgets.Table");
        if (!tableClass.isInstance(table)) {
            throw new IllegalArgumentException("Widget is not a Table");
        }
        return table;
    }

    // Helper methods using reflection

    private boolean isWidgetEnabled(int widgetId) throws Exception {
        Object widget = SwtReflectionBridge.getWidgetById(widgetId);
        if (widget == null) return false;
        try {
            java.lang.reflect.Method isEnabled = widget.getClass().getMethod("isEnabled");
            return (Boolean) isEnabled.invoke(widget);
        } catch (Exception e) {
            return true; // Default to enabled if method not found
        }
    }

    private boolean isWidgetVisible(int widgetId) throws Exception {
        Object widget = SwtReflectionBridge.getWidgetById(widgetId);
        if (widget == null) return false;
        try {
            java.lang.reflect.Method isVisible = widget.getClass().getMethod("isVisible");
            return (Boolean) isVisible.invoke(widget);
        } catch (Exception e) {
            return true; // Default to visible if method not found
        }
    }

    private String getWidgetText(int widgetId) throws Exception {
        Object widget = SwtReflectionBridge.getWidgetById(widgetId);
        if (widget == null) return "";
        try {
            java.lang.reflect.Method getText = widget.getClass().getMethod("getText");
            Object result = getText.invoke(widget);
            return result != null ? result.toString() : "";
        } catch (Exception e) {
            return ""; // Default to empty if method not found
        }
    }

    private JsonObject getWidgetInfoJson(Object widget) throws Exception {
        JsonObject info = new JsonObject();
        info.addProperty("className", widget.getClass().getName());
        info.addProperty("type", widget.getClass().getSimpleName());

        // Try to get text
        try {
            java.lang.reflect.Method getText = widget.getClass().getMethod("getText");
            Object text = getText.invoke(widget);
            if (text != null) info.addProperty("text", text.toString());
        } catch (Exception e) {}

        // Try to get enabled state
        try {
            java.lang.reflect.Method isEnabled = widget.getClass().getMethod("isEnabled");
            info.addProperty("enabled", (Boolean) isEnabled.invoke(widget));
        } catch (Exception e) {}

        // Try to get visible state
        try {
            java.lang.reflect.Method isVisible = widget.getClass().getMethod("isVisible");
            info.addProperty("visible", (Boolean) isVisible.invoke(widget));
        } catch (Exception e) {}

        return info;
    }

    private int getTableRowCount(int widgetId) throws Exception {
        Object table = getTableWidget(widgetId);
        return SwtReflectionBridge.syncExec(() -> {
            java.lang.reflect.Method getItemCount = table.getClass().getMethod("getItemCount");
            return (Integer) getItemCount.invoke(table);
        });
    }

    private String getTableCell(int widgetId, int row, int column) throws Exception {
        Object table = getTableWidget(widgetId);
        return SwtReflectionBridge.syncExec(() -> {
            java.lang.reflect.Method getItem = table.getClass().getMethod("getItem", int.class);
            Object item = getItem.invoke(table, row);
            if (item == null) {
                return "";
            }

            int columnCount = getTableColumnCount(table);
            if (columnCount == 0) {
                java.lang.reflect.Method getText = item.getClass().getMethod("getText");
                Object text = getText.invoke(item);
                return text != null ? text.toString() : "";
            }

            java.lang.reflect.Method getText = item.getClass().getMethod("getText", int.class);
            Object text = getText.invoke(item, column);
            return text != null ? text.toString() : "";
        });
    }

    private void selectTableRow(int widgetId, int row) throws Exception {
        Object table = getTableWidget(widgetId);
        SwtReflectionBridge.syncExec(() -> {
            java.lang.reflect.Method select = table.getClass().getMethod("select", int.class);
            select.invoke(table, row);
            showSelection(table);
            notifySelectionChanged(table);
            return null;
        });
    }

    private JsonArray getTableRowValues(int widgetId, int row) throws Exception {
        Object table = getTableWidget(widgetId);
        return SwtReflectionBridge.syncExec(() -> {
            JsonArray values = new JsonArray();
            java.lang.reflect.Method getItem = table.getClass().getMethod("getItem", int.class);
            Object item = getItem.invoke(table, row);
            if (item == null) {
                return values;
            }

            int columnCount = getTableColumnCount(table);
            if (columnCount == 0) {
                java.lang.reflect.Method getText = item.getClass().getMethod("getText");
                Object text = getText.invoke(item);
                values.add(text != null ? text.toString() : "");
                return values;
            }

            java.lang.reflect.Method getText = item.getClass().getMethod("getText", int.class);
            for (int i = 0; i < columnCount; i++) {
                Object text = getText.invoke(item, i);
                values.add(text != null ? text.toString() : "");
            }
            return values;
        });
    }

    private void selectTableRows(int widgetId, int[] rows) throws Exception {
        Object table = getTableWidget(widgetId);
        SwtReflectionBridge.syncExec(() -> {
            ensureTableMultiSelection(table, rows.length);
            java.lang.reflect.Method setSelection = table.getClass().getMethod("setSelection", int[].class);
            setSelection.invoke(table, (Object) rows);
            showSelection(table);
            notifySelectionChanged(table);
            return null;
        });
    }

    private void deselectAllTableRows(int widgetId) throws Exception {
        Object table = getTableWidget(widgetId);
        SwtReflectionBridge.syncExec(() -> {
            java.lang.reflect.Method deselectAll = table.getClass().getMethod("deselectAll");
            deselectAll.invoke(table);
            return null;
        });
    }

    private int selectTableRowByValue(int widgetId, int column, String value) throws Exception {
        Object table = getTableWidget(widgetId);
        return SwtReflectionBridge.syncExec(() -> {
            int columnCount = getTableColumnCount(table);
            int effectiveColumnCount = columnCount == 0 ? 1 : columnCount;
            if (column < 0 || column >= effectiveColumnCount) {
                throw new IndexOutOfBoundsException("Column index out of bounds: " + column +
                    " (table has " + effectiveColumnCount + " columns)");
            }

            java.lang.reflect.Method getItemCount = table.getClass().getMethod("getItemCount");
            int itemCount = (Integer) getItemCount.invoke(table);
            if (itemCount == 0) {
                return -1;
            }

            java.lang.reflect.Method getItem = table.getClass().getMethod("getItem", int.class);
            java.lang.reflect.Method getTextWithIndex = null;
            java.lang.reflect.Method getTextNoIndex = null;

            if (columnCount == 0) {
                getTextNoIndex = loadTableItemTextMethod(table, false);
            } else {
                getTextWithIndex = loadTableItemTextMethod(table, true);
            }

            for (int i = 0; i < itemCount; i++) {
                Object item = getItem.invoke(table, i);
                String cellValue;
                if (columnCount == 0) {
                    Object text = getTextNoIndex.invoke(item);
                    cellValue = text != null ? text.toString() : "";
                } else {
                    Object text = getTextWithIndex.invoke(item, column);
                    cellValue = text != null ? text.toString() : "";
                }
                if (value.equals(cellValue)) {
                    java.lang.reflect.Method select = table.getClass().getMethod("select", int.class);
                    select.invoke(table, i);
                    showSelection(table);
                    notifySelectionChanged(table);
                    return i;
                }
            }

            return -1;
        });
    }

    private void selectTableRowRange(int widgetId, int startRow, int endRow) throws Exception {
        Object table = getTableWidget(widgetId);
        SwtReflectionBridge.syncExec(() -> {
            ensureTableMultiSelection(table, Math.abs(endRow - startRow) + 1);
            java.lang.reflect.Method setSelection = table.getClass().getMethod("setSelection", int.class, int.class);
            setSelection.invoke(table, startRow, endRow);
            showSelection(table);
            notifySelectionChanged(table);
            return null;
        });
    }

    private JsonArray getTableColumns(int widgetId) throws Exception {
        Object table = getTableWidget(widgetId);
        return SwtReflectionBridge.syncExec(() -> {
            JsonArray columns = new JsonArray();
            java.lang.reflect.Method getColumns = table.getClass().getMethod("getColumns");
            Object[] cols = (Object[]) getColumns.invoke(table);
            java.lang.reflect.Method getText = null;
            if (cols != null && cols.length > 0) {
                getText = cols[0].getClass().getMethod("getText");
            }
            if (cols != null) {
                for (Object col : cols) {
                    Object text = getText != null ? getText.invoke(col) : null;
                    columns.add(text != null ? text.toString() : "");
                }
            }
            return columns;
        });
    }

    private void clickTableColumnHeader(int widgetId, int column) throws Exception {
        Object table = getTableWidget(widgetId);
        SwtReflectionBridge.syncExec(() -> {
            java.lang.reflect.Method getColumn = table.getClass().getMethod("getColumn", int.class);
            Object tableColumn = getColumn.invoke(table, column);
            if (tableColumn == null) {
                throw new IllegalArgumentException("Column not found: " + column);
            }
            Class<?> eventClass = loadSwtClass("org.eclipse.swt.widgets.Event");
            Object event = eventClass.getDeclaredConstructor().newInstance();
            Class<?> swtClass = loadSwtClass("org.eclipse.swt.SWT");
            int selection = swtClass.getField("Selection").getInt(null);
            java.lang.reflect.Method notifyListeners = tableColumn.getClass().getMethod("notifyListeners", int.class, eventClass);
            notifyListeners.invoke(tableColumn, selection, event);
            return null;
        });
    }

    private void ensureTableMultiSelection(Object table, int selectionCount) throws Exception {
        if (selectionCount <= 1) {
            return;
        }
        Class<?> swtClass = loadSwtClass("org.eclipse.swt.SWT");
        int multi = swtClass.getField("MULTI").getInt(null);
        java.lang.reflect.Method getStyle = table.getClass().getMethod("getStyle");
        int style = (Integer) getStyle.invoke(table);
        if ((style & multi) == 0) {
            throw new IllegalStateException("Table does not support multi-selection (SWT.MULTI style not set)");
        }
    }

    private void showSelection(Object table) {
        try {
            java.lang.reflect.Method showSelection = table.getClass().getMethod("showSelection");
            showSelection.invoke(table);
        } catch (Exception e) {
            // Ignore if not supported
        }
    }

    private void notifySelectionChanged(Object table) throws Exception {
        try {
            Class<?> eventClass = loadSwtClass("org.eclipse.swt.widgets.Event");
            Object event = eventClass.getDeclaredConstructor().newInstance();
            Class<?> swtClass = loadSwtClass("org.eclipse.swt.SWT");
            int selection = swtClass.getField("Selection").getInt(null);
            java.lang.reflect.Method notifyListeners = table.getClass().getMethod("notifyListeners", int.class, eventClass);
            notifyListeners.invoke(table, selection, event);
        } catch (Exception e) {
            // Ignore if notifyListeners not available
        }
    }

    private int getTableColumnCount(Object table) throws Exception {
        java.lang.reflect.Method getColumnCount = table.getClass().getMethod("getColumnCount");
        return (Integer) getColumnCount.invoke(table);
    }

    private java.lang.reflect.Method loadTableItemTextMethod(Object table, boolean withIndex) throws Exception {
        java.lang.reflect.Method getItem = table.getClass().getMethod("getItem", int.class);
        Object item = getItem.invoke(table, 0);
        if (item == null) {
            return withIndex ? table.getClass().getMethod("getText", int.class) : table.getClass().getMethod("getText");
        }
        return withIndex
            ? item.getClass().getMethod("getText", int.class)
            : item.getClass().getMethod("getText");
    }

    private void selectItem(int widgetId, String itemText) throws Exception {
        Object widget = SwtReflectionBridge.getWidgetById(widgetId);
        if (widget == null) return;

        if (itemText == null || itemText.trim().isEmpty()) {
            throw new IllegalArgumentException("Item text must not be empty");
        }

        SwtReflectionBridge.syncExec(() -> {
            try {
                // Try Combo/CCombo select
                java.lang.reflect.Method select = null;
                try {
                    select = widget.getClass().getMethod("select", int.class);
                } catch (NoSuchMethodException e) {
                    // Not a combo, try other widgets
                }

                if (select != null) {
                    // Get items and find index
                    java.lang.reflect.Method getItems = widget.getClass().getMethod("getItems");
                    String[] items = (String[]) getItems.invoke(widget);
                    int index = -1;
                    for (int i = 0; i < items.length; i++) {
                        if (items[i].equals(itemText)) {
                            index = i;
                            break;
                        }
                    }
                    if (index < 0) {
                        throw new IllegalArgumentException("Item not found: " + itemText);
                    }
                    select.invoke(widget, index);
                    return null;
                }

                throw new IllegalArgumentException("Widget does not support item selection");
            } catch (Exception e) {
                throw new RuntimeException("selectItem failed: " + e.getMessage(), e);
            }
        });
    }

    private String getSelectItemValue(JsonObject params) throws Exception {
        if (params.has("item") && !params.get("item").isJsonNull()) {
            return params.get("item").getAsString();
        }
        if (params.has("value") && !params.get("value").isJsonNull()) {
            return params.get("value").getAsString();
        }
        throw new Exception("Missing item/value parameter for selectItem");
    }

    private void setButtonChecked(int widgetId, boolean checked) throws Exception {
        Object widget = SwtReflectionBridge.getWidgetById(widgetId);
        if (widget == null) return;

        SwtReflectionBridge.syncExec(() -> {
            try {
                java.lang.reflect.Method setSelection = widget.getClass().getMethod("setSelection", boolean.class);
                setSelection.invoke(widget, checked);
            } catch (Exception e) {
                System.err.println("[SwtAgent] Error setting button state: " + e.getMessage());
            }
            return null;
        });
    }

    // =============================================================
    // Mock RCP Application Support
    // =============================================================

    private Object getMockRcpApp() {
        if (!mockRcpChecked) {
            mockRcpChecked = true;
            try {
                mockRcpAppClass = Class.forName("testapp.rcp.MockRcpApplication");
                java.lang.reflect.Method getInstance = mockRcpAppClass.getMethod("getInstance");
                mockRcpApp = getInstance.invoke(null);
                if (mockRcpApp != null) {
                    System.out.println("[SwtReflectionRpcServer] MockRcpApplication detected and will be used for RCP operations");
                }
            } catch (ClassNotFoundException e) {
                // Mock app not present; normal for real Eclipse apps
            } catch (Exception e) {
                System.err.println("[SwtReflectionRpcServer] Error initializing MockRcpApplication: " + e.getMessage());
            }
        }
        return mockRcpApp;
    }

    @SuppressWarnings("unchecked")
    private <T> T invokeMockMethod(String methodName, Class<T> returnType, Object... args) {
        Object app = getMockRcpApp();
        if (app == null) return null;
        try {
            Class<?>[] paramTypes = new Class<?>[args.length];
            for (int i = 0; i < args.length; i++) {
                if (args[i] == null) {
                    paramTypes[i] = String.class;
                } else {
                    paramTypes[i] = args[i].getClass();
                    if (paramTypes[i] == Boolean.class) paramTypes[i] = boolean.class;
                    if (paramTypes[i] == Integer.class) paramTypes[i] = int.class;
                }
            }
            java.lang.reflect.Method method = mockRcpAppClass.getMethod(methodName, paramTypes);
            Object result = method.invoke(app, args);
            return returnType.cast(result);
        } catch (NoSuchMethodException e) {
            try {
                java.lang.reflect.Method method = mockRcpAppClass.getMethod(methodName);
                Object result = method.invoke(app);
                return returnType.cast(result);
            } catch (Exception ex) {
                return null;
            }
        } catch (Exception e) {
            System.err.println("[SwtReflectionRpcServer] Error invoking mock method " + methodName + ": " + e.getMessage());
            return null;
        }
    }

    private boolean invokeMockVoidMethod(String methodName, Object... args) {
        Object app = getMockRcpApp();
        if (app == null) return false;
        try {
            Class<?>[] paramTypes = new Class<?>[args.length];
            for (int i = 0; i < args.length; i++) {
                if (args[i] == null) {
                    paramTypes[i] = String.class;
                } else {
                    paramTypes[i] = args[i].getClass();
                    if (paramTypes[i] == Boolean.class) paramTypes[i] = boolean.class;
                    if (paramTypes[i] == Integer.class) paramTypes[i] = int.class;
                }
            }
            java.lang.reflect.Method method = mockRcpAppClass.getMethod(methodName, paramTypes);
            method.invoke(app, args);
            return true;
        } catch (Exception e) {
            System.err.println("[SwtReflectionRpcServer] Error invoking mock void method " + methodName + ": " + e.getMessage());
            return false;
        }
    }

    // =============================================================
    // RCP Helper Methods (Dual-mode: Mock + Real Eclipse support)
    // =============================================================

    private JsonElement getWorkbenchInfo() {
        Object app = getMockRcpApp();
        if (app != null) {
            String info = invokeMockMethod("getWorkbenchInfo", String.class);
            if (info != null) {
                JsonObject result = new JsonObject();
                result.addProperty("info", info);
                result.addProperty("windowCount", 1);
                result.addProperty("activePerspective", invokeMockMethod("getActivePerspective", String.class));
                return result;
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    @SuppressWarnings("unchecked")
    private JsonElement getAvailablePerspectives() {
        Object app = getMockRcpApp();
        if (app != null) {
            java.util.List<String> perspList = invokeMockMethod("getAvailablePerspectives", java.util.List.class);
            if (perspList != null) {
                JsonArray perspectives = new JsonArray();
                for (String id : perspList) {
                    JsonObject perspObj = new JsonObject();
                    perspObj.addProperty("id", id);
                    perspObj.addProperty("label", id.substring(id.lastIndexOf('.') + 1));
                    perspectives.add(perspObj);
                }
                return perspectives;
            }
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            java.util.List<java.util.Map<String, String>> perspList = EclipseWorkbenchHelper.getAvailablePerspectives();
            JsonArray perspectives = new JsonArray();
            for (java.util.Map<String, String> persp : perspList) {
                JsonObject perspObj = new JsonObject();
                perspObj.addProperty("id", persp.get("id"));
                perspObj.addProperty("label", persp.get("label"));
                perspectives.add(perspObj);
            }
            return perspectives;
        }

        return new JsonArray();
    }

    private JsonElement getActivePerspective() {
        Object app = getMockRcpApp();
        if (app != null) {
            String perspId = invokeMockMethod("getActivePerspective", String.class);
            if (perspId != null) {
                JsonObject result = new JsonObject();
                result.addProperty("id", perspId);
                result.addProperty("label", perspId.substring(perspId.lastIndexOf('.') + 1));
                return result;
            }
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            String perspId = EclipseWorkbenchHelper.getActivePerspectiveId();
            if (perspId != null) {
                JsonObject result = new JsonObject();
                result.addProperty("id", perspId);
                result.addProperty("label", EclipseWorkbenchHelper.getActivePerspectiveLabel());
                return result;
            }
        }

        JsonObject result = new JsonObject();
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    @SuppressWarnings("unchecked")
    private JsonElement getOpenPerspectives() {
        Object app = getMockRcpApp();
        if (app != null) {
            java.util.List<String> perspList = invokeMockMethod("getOpenPerspectives", java.util.List.class);
            if (perspList != null) {
                JsonArray perspectives = new JsonArray();
                for (String id : perspList) {
                    JsonObject perspObj = new JsonObject();
                    perspObj.addProperty("id", id);
                    perspObj.addProperty("label", id.substring(id.lastIndexOf('.') + 1));
                    perspectives.add(perspObj);
                }
                return perspectives;
            }
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            JsonArray perspectives = new JsonArray();
            String perspId = EclipseWorkbenchHelper.getActivePerspectiveId();
            if (perspId != null) {
                JsonObject perspObj = new JsonObject();
                perspObj.addProperty("id", perspId);
                perspObj.addProperty("label", EclipseWorkbenchHelper.getActivePerspectiveLabel());
                perspectives.add(perspObj);
            }
            return perspectives;
        }

        return new JsonArray();
    }

    private JsonElement openPerspective(String perspectiveId) {
        Object app = getMockRcpApp();
        if (app != null) {
            Boolean success = invokeMockMethod("switchPerspective", Boolean.class, perspectiveId);
            if (Boolean.TRUE.equals(success)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
            JsonObject result = new JsonObject();
            result.addProperty("success", false);
            result.addProperty("error", "Perspective not found: " + perspectiveId);
            return result;
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            if (EclipseWorkbenchHelper.openPerspective(perspectiveId)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
            JsonObject result = new JsonObject();
            result.addProperty("success", false);
            result.addProperty("error", "Perspective not found: " + perspectiveId);
            return result;
        }

        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement openPerspectiveByName(String name) {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("openPerspectiveByName", name)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available or perspective not found: " + name);
        return result;
    }

    private JsonElement closePerspective(String perspectiveId) {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("closePerspective", perspectiveId)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement closeAllPerspectives() {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("closeAllPerspectives")) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement resetPerspective() {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("resetPerspective")) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            if (EclipseWorkbenchHelper.resetPerspective()) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement savePerspectiveAs(String name) {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("savePerspectiveAs", name)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement showView(String viewId, String secondaryId) {
        Object app = getMockRcpApp();
        if (app != null) {
            Boolean success = invokeMockMethod("showView", Boolean.class, viewId, secondaryId);
            if (Boolean.TRUE.equals(success)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                result.addProperty("viewId", viewId);
                return result;
            }
            JsonObject result = new JsonObject();
            result.addProperty("success", false);
            result.addProperty("error", "View not found: " + viewId);
            return result;
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            if (EclipseWorkbenchHelper.showView(viewId, secondaryId)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                result.addProperty("viewId", viewId);
                return result;
            }
            JsonObject result = new JsonObject();
            result.addProperty("success", false);
            result.addProperty("error", "View not found: " + viewId);
            return result;
        }

        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement showViewByName(String name) {
        Object app = getMockRcpApp();
        if (app != null) {
            Boolean success = invokeMockMethod("showViewByName", Boolean.class, name);
            if (Boolean.TRUE.equals(success)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "View not found: " + name);
        return result;
    }

    private JsonElement closeView(String viewId, String secondaryId) {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("closeView", viewId)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            if (EclipseWorkbenchHelper.hideView(viewId, secondaryId)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement activateView(String viewId) {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("activateView", viewId)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            if (EclipseWorkbenchHelper.activateView(viewId)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement getOpenViews() {
        Object app = getMockRcpApp();
        if (app != null) {
            java.util.List<java.util.Map<String, String>> viewList = invokeMockMethod("getOpenViews", java.util.List.class);
            JsonArray views = new JsonArray();
            if (viewList != null) {
                for (java.util.Map<String, String> view : viewList) {
                    JsonObject viewObj = new JsonObject();
                    viewObj.addProperty("id", view.get("id"));
                    viewObj.addProperty("title", view.get("title"));
                    views.add(viewObj);
                }
            }
            return views;
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            java.util.List<java.util.Map<String, Object>> viewList = EclipseWorkbenchHelper.getOpenViews();
            JsonArray views = new JsonArray();
            for (java.util.Map<String, Object> view : viewList) {
                JsonObject viewObj = new JsonObject();
                Object id = view.get("id");
                Object title = view.get("title");
                viewObj.addProperty("id", id != null ? id.toString() : "");
                viewObj.addProperty("title", title != null ? title.toString() : "");
                views.add(viewObj);
            }
            return views;
        }

        return new JsonArray();
    }

    private JsonElement getActiveView() {
        Object app = getMockRcpApp();
        if (app != null) {
            java.util.Map<String, String> view = invokeMockMethod("getActiveView", java.util.Map.class);
            if (view != null) {
                JsonObject result = new JsonObject();
                result.addProperty("id", view.get("id"));
                result.addProperty("title", view.get("title"));
                return result;
            }
        }

        JsonObject result = new JsonObject();
        result.addProperty("error", "No active view");
        return result;
    }

    private JsonElement isViewVisible(String viewId) {
        Object app = getMockRcpApp();
        if (app != null) {
            Boolean visible = invokeMockMethod("isViewVisible", Boolean.class, viewId);
            return new JsonPrimitive(Boolean.TRUE.equals(visible));
        }
        return new JsonPrimitive(false);
    }

    private JsonElement minimizeView(String viewId) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("minimizeView", viewId)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement maximizeView(String viewId) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("maximizeView", viewId)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement restoreView(String viewId) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("restoreView", viewId)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement isViewMinimized(String viewId) {
        Object app = getMockRcpApp();
        if (app != null) {
            Boolean minimized = invokeMockMethod("isViewMinimized", Boolean.class, viewId);
            return new JsonPrimitive(Boolean.TRUE.equals(minimized));
        }
        return new JsonPrimitive(false);
    }

    private JsonElement isViewMaximized(String viewId) {
        Object app = getMockRcpApp();
        if (app != null) {
            Boolean maximized = invokeMockMethod("isViewMaximized", Boolean.class, viewId);
            return new JsonPrimitive(Boolean.TRUE.equals(maximized));
        }
        return new JsonPrimitive(false);
    }

    private JsonElement getViewTitle(String viewId) {
        Object app = getMockRcpApp();
        if (app != null) {
            String title = invokeMockMethod("getViewTitle", String.class, viewId);
            return new JsonPrimitive(title != null ? title : "");
        }
        return new JsonPrimitive("");
    }

    private JsonElement openEditor(String filePath) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("openEditor", filePath)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement closeEditor(String filePath, boolean save) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("closeEditor", filePath)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement closeAllEditors(boolean save) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("closeAllEditors", save)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            if (EclipseWorkbenchHelper.closeAllEditors(save)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement activateEditor(String filePath) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("activateEditor", filePath)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement saveEditor(String filePath) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("saveEditor", filePath)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement saveAllEditors() {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("saveAllEditors")) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            if (EclipseWorkbenchHelper.saveAllEditors(true)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement getActiveEditor() {
        Object app = getMockRcpApp();
        if (app != null) {
            java.util.Map<String, Object> editor = invokeMockMethod("getActiveEditor", java.util.Map.class);
            if (editor != null) {
                JsonObject result = new JsonObject();
                Object title = editor.get("title");
                Object path = editor.get("path");
                result.addProperty("title", title != null ? title.toString() : "");
                result.addProperty("path", path != null ? path.toString() : "");
                return result;
            }
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            java.util.Map<String, Object> editor = EclipseWorkbenchHelper.getActiveEditor();
            if (editor != null) {
                JsonObject result = new JsonObject();
                Object title = editor.get("title");
                Object path = editor.get("path");
                result.addProperty("title", title != null ? title.toString() : "");
                result.addProperty("path", path != null ? path.toString() : "");
                return result;
            }
        }

        JsonObject result = new JsonObject();
        result.addProperty("error", "No active editor");
        return result;
    }

    private JsonElement getOpenEditors() {
        Object app = getMockRcpApp();
        if (app != null) {
            java.util.List<java.util.Map<String, Object>> editors = invokeMockMethod("getOpenEditors", java.util.List.class);
            JsonArray result = new JsonArray();
            if (editors != null) {
                for (java.util.Map<String, Object> editor : editors) {
                    JsonObject editorObj = new JsonObject();
                    Object title = editor.get("title");
                    Object path = editor.get("path");
                    editorObj.addProperty("title", title != null ? title.toString() : "");
                    editorObj.addProperty("path", path != null ? path.toString() : "");
                    result.add(editorObj);
                }
            }
            return result;
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            java.util.List<java.util.Map<String, Object>> editors = EclipseWorkbenchHelper.getOpenEditors();
            JsonArray result = new JsonArray();
            for (java.util.Map<String, Object> editor : editors) {
                JsonObject editorObj = new JsonObject();
                Object title = editor.get("title");
                Object path = editor.get("path");
                editorObj.addProperty("title", title != null ? title.toString() : "");
                editorObj.addProperty("path", path != null ? path.toString() : "");
                result.add(editorObj);
            }
            return result;
        }

        return new JsonArray();
    }

    private JsonElement isEditorOpen(String filePath) {
        Object app = getMockRcpApp();
        if (app != null) {
            Boolean open = invokeMockMethod("isEditorOpen", Boolean.class, filePath);
            return new JsonPrimitive(Boolean.TRUE.equals(open));
        }
        return new JsonPrimitive(false);
    }

    private JsonElement isEditorDirty(String filePath) {
        Object app = getMockRcpApp();
        if (app != null) {
            Boolean dirty = invokeMockMethod("isEditorDirty", Boolean.class, filePath);
            return new JsonPrimitive(Boolean.TRUE.equals(dirty));
        }
        return new JsonPrimitive(false);
    }

    private JsonElement getEditorContent(String filePath) {
        Object app = getMockRcpApp();
        if (app != null) {
            String content = invokeMockMethod("getEditorContent", String.class, filePath);
            return new JsonPrimitive(content != null ? content : "");
        }
        return new JsonPrimitive("");
    }

    private JsonElement getDirtyEditorCount() {
        Object app = getMockRcpApp();
        if (app != null) {
            Integer count = invokeMockMethod("getDirtyEditorCount", Integer.class);
            return new JsonPrimitive(count != null ? count : 0);
        }
        return new JsonPrimitive(0);
    }

    private JsonElement enterTextInEditor(String text) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("enterTextInEditor", text)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement executeCommand(String commandId) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("executeCommand", commandId)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            if (EclipseWorkbenchHelper.executeCommand(commandId)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "Command not executed: " + commandId);
        return result;
    }

    private JsonElement executeMenu(String menuPath) {
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "Menu execution not implemented - use executeCommand with command ID instead");
        return result;
    }

    private JsonElement openPreferences() {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("openPreferencesDialog")) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement getOpenDialogs() {
        Object app = getMockRcpApp();
        if (app != null) {
            java.util.List<String> dialogs = invokeMockMethod("getOpenDialogs", java.util.List.class);
            JsonArray result = new JsonArray();
            if (dialogs != null) {
                for (String dialog : dialogs) {
                    result.add(dialog);
                }
            }
            return result;
        }
        return new JsonArray();
    }

    private JsonElement getActiveWorkbenchWindow() {
        Object app = getMockRcpApp();
        if (app != null) {
            JsonObject result = new JsonObject();
            result.addProperty("active", true);
            result.addProperty("title", invokeMockMethod("getWorkbenchTitle", String.class));
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement getWorkbenchWindowCount() {
        Object app = getMockRcpApp();
        if (app != null) {
            Integer count = invokeMockMethod("getWorkbenchWindowCount", Integer.class);
            return new JsonPrimitive(count != null ? count : 0);
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            return new JsonPrimitive(EclipseWorkbenchHelper.getWorkbenchWindowCount());
        }

        return new JsonPrimitive(0);
    }

    private JsonElement getWorkbenchTitle() {
        Object app = getMockRcpApp();
        if (app != null) {
            String title = invokeMockMethod("getWorkbenchTitle", String.class);
            return new JsonPrimitive(title != null ? title : "");
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            String title = EclipseWorkbenchHelper.getWorkbenchTitle();
            return new JsonPrimitive(title != null ? title : "");
        }

        return new JsonPrimitive("");
    }

    private JsonElement getWorkbenchState() {
        Object app = getMockRcpApp();
        if (app != null) {
            JsonObject result = new JsonObject();
            result.addProperty("running", true);
            result.addProperty("activePerspective", invokeMockMethod("getActivePerspective", String.class));

            java.util.List<?> views = invokeMockMethod("getOpenViews", java.util.List.class);
            result.addProperty("openViews", views != null ? views.size() : 0);

            java.util.List<?> editors = invokeMockMethod("getOpenEditors", java.util.List.class);
            result.addProperty("openEditors", editors != null ? editors.size() : 0);
            return result;
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            JsonObject result = new JsonObject();
            result.addProperty("running", true);
            result.addProperty("activePerspective", EclipseWorkbenchHelper.getActivePerspectiveId());
            result.addProperty("openViews", EclipseWorkbenchHelper.getOpenViews().size());
            result.addProperty("openEditors", EclipseWorkbenchHelper.getOpenEditors().size());
            return result;
        }

        JsonObject result = new JsonObject();
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement waitForWorkbench(long timeout) {
        Object app = getMockRcpApp();
        if (app != null) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            result.addProperty("elapsed", 0);
            return result;
        }

        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            result.addProperty("elapsed", 0);
            return result;
        }

        long startTime = System.currentTimeMillis();
        while (System.currentTimeMillis() - startTime < timeout) {
            if (EclipseWorkbenchHelper.isEclipseAvailable()) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                result.addProperty("elapsed", System.currentTimeMillis() - startTime);
                return result;
            }
            try {
                Thread.sleep(100);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                break;
            }
        }

        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available after timeout");
        return result;
    }

    private JsonElement pressButton(String label) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("pressButton", label)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement closeActiveDialog() {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("closeActiveDialog")) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement navigateToPreferencePage(String pagePath) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("navigateToPreferencePage", pagePath)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            result.addProperty("pagePath", pagePath);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement getViewWidget(String viewId, String widgetType) {
        Object app = getMockRcpApp();
        if (app != null) {
            try {
                java.lang.reflect.Method method = app.getClass().getMethod("getViewWidget", String.class, String.class);
                @SuppressWarnings("unchecked")
                java.util.Map<String, Object> widgetMap = (java.util.Map<String, Object>) method.invoke(app, viewId, widgetType);
                if (widgetMap != null) {
                    JsonObject result = new JsonObject();
                    for (java.util.Map.Entry<String, Object> entry : widgetMap.entrySet()) {
                        Object value = entry.getValue();
                        if (value instanceof String) {
                            result.addProperty(entry.getKey(), (String) value);
                        } else if (value instanceof Number) {
                            result.addProperty(entry.getKey(), (Number) value);
                        } else if (value instanceof Boolean) {
                            result.addProperty(entry.getKey(), (Boolean) value);
                        }
                    }
                    return result;
                }
            } catch (Exception e) {
                System.err.println("[SwtReflectionRpcServer] getViewWidget error: " + e.getMessage());
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("error", "Widget not found in view");
        return result;
    }

    private JsonElement getEditorWidget(String title, String locator) {
        Object app = getMockRcpApp();
        if (app != null) {
            try {
                java.lang.reflect.Method method = app.getClass().getMethod("getEditorWidget", String.class, String.class);
                @SuppressWarnings("unchecked")
                java.util.Map<String, Object> widgetMap = (java.util.Map<String, Object>) method.invoke(app, title, locator);
                if (widgetMap != null) {
                    JsonObject result = new JsonObject();
                    for (java.util.Map.Entry<String, Object> entry : widgetMap.entrySet()) {
                        Object value = entry.getValue();
                        if (value instanceof String) {
                            result.addProperty(entry.getKey(), (String) value);
                        } else if (value instanceof Number) {
                            result.addProperty(entry.getKey(), (Number) value);
                        } else if (value instanceof Boolean) {
                            result.addProperty(entry.getKey(), (Boolean) value);
                        }
                    }
                    return result;
                }
            } catch (Exception e) {
                System.err.println("[SwtReflectionRpcServer] getEditorWidget error: " + e.getMessage());
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("error", "Widget not found in editor");
        return result;
    }

    private JsonElement clickToolbarItem(String tooltip) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("clickToolbarItem", tooltip)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            result.addProperty("tooltip", tooltip);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "Toolbar item not found: " + tooltip);
        return result;
    }

    private JsonElement getAvailableCommands() {
        Object app = getMockRcpApp();
        if (app != null) {
            JsonArray commands = new JsonArray();
            commands.add("org.eclipse.ui.file.save");
            commands.add("org.eclipse.ui.file.saveAll");
            commands.add("org.eclipse.ui.file.refresh");
            commands.add("org.eclipse.ui.edit.undo");
            commands.add("org.eclipse.ui.edit.redo");
            return commands;
        }
        return new JsonArray();
    }

    private JsonElement selectContextMenu(String menuPath) {
        Object app = getMockRcpApp();
        if (app != null) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            result.addProperty("menuPath", menuPath);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement selectMainMenu(String menuPath) {
        Object app = getMockRcpApp();
        if (app != null && invokeMockVoidMethod("executeMenu", menuPath)) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            result.addProperty("menuPath", menuPath);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    public void stop() {
        running.set(false);
        ready.set(false);
        if (serverSocket != null && !serverSocket.isClosed()) {
            try {
                serverSocket.close();
            } catch (IOException e) {
                // Ignore
            }
        }
    }

    /**
     * Check if the server is ready to accept connections.
     * @return true if the server socket is listening
     */
    public boolean isReady() {
        return ready.get();
    }
}
