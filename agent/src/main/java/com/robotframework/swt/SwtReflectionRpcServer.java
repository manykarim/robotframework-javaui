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
                return new JsonPrimitive(getTableCell(
                    getWidgetId(params),
                    params.get("row").getAsInt(),
                    params.get("column").getAsInt()
                ));

            case "selectTableRow":
                selectTableRow(getWidgetId(params), params.get("row").getAsInt());
                return new JsonPrimitive(true);

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
        Object table = SwtReflectionBridge.getWidgetById(widgetId);
        if (table == null) return 0;
        try {
            java.lang.reflect.Method getItemCount = table.getClass().getMethod("getItemCount");
            return (Integer) getItemCount.invoke(table);
        } catch (Exception e) {
            return 0;
        }
    }

    private String getTableCell(int widgetId, int row, int column) throws Exception {
        Object table = SwtReflectionBridge.getWidgetById(widgetId);
        if (table == null) return "";
        try {
            // Get the table item at row
            java.lang.reflect.Method getItem = table.getClass().getMethod("getItem", int.class);
            Object item = getItem.invoke(table, row);
            if (item == null) return "";

            // Get the text at column
            java.lang.reflect.Method getText = item.getClass().getMethod("getText", int.class);
            Object text = getText.invoke(item, column);
            return text != null ? text.toString() : "";
        } catch (Exception e) {
            return "";
        }
    }

    private void selectTableRow(int widgetId, int row) throws Exception {
        Object table = SwtReflectionBridge.getWidgetById(widgetId);
        if (table == null) return;

        SwtReflectionBridge.syncExec(() -> {
            try {
                // Get the table item at row
                java.lang.reflect.Method getItem = table.getClass().getMethod("getItem", int.class);
                Object item = getItem.invoke(table, row);
                if (item == null) return null;

                // Select the item
                java.lang.reflect.Method select = table.getClass().getMethod("select", int.class);
                select.invoke(table, row);
            } catch (Exception e) {
                System.err.println("[SwtAgent] Error selecting table row: " + e.getMessage());
            }
            return null;
        });
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
