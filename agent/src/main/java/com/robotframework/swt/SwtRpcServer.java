package com.robotframework.swt;

import com.google.gson.*;

import java.io.*;
import java.net.ServerSocket;
import java.net.Socket;
import java.net.SocketException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

/**
 * JSON-RPC 2.0 server for handling SWT automation requests.
 * Listens for incoming connections and dispatches method calls.
 */
public class SwtRpcServer implements Runnable {

    private final String host;
    private final int port;
    private volatile boolean running = false;
    private ServerSocket serverSocket;
    private ExecutorService executor;
    private final Gson gson;

    public SwtRpcServer(String host, int port) {
        this.host = host;
        this.port = port;
        this.gson = new GsonBuilder().create();
    }

    @Override
    public void run() {
        running = true;
        executor = Executors.newCachedThreadPool(r -> {
            Thread t = new Thread(r, "SwtAgent-RpcHandler");
            t.setDaemon(true);
            return t;
        });

        try {
            serverSocket = new ServerSocket(port);
            System.out.println("[SwtAgent] RPC server listening on " + host + ":" + port);

            while (running) {
                try {
                    Socket clientSocket = serverSocket.accept();
                    executor.submit(() -> handleClient(clientSocket));
                } catch (SocketException e) {
                    if (running) {
                        System.err.println("[SwtAgent] Socket error: " + e.getMessage());
                    }
                }
            }
        } catch (IOException e) {
            System.err.println("[SwtAgent] Failed to start server: " + e.getMessage());
        }
    }

    private void handleClient(Socket socket) {
        System.out.println("[SwtAgent] Client connected from " + socket.getRemoteSocketAddress());
        try (
            BufferedReader reader = new BufferedReader(new InputStreamReader(socket.getInputStream()));
            PrintWriter writer = new PrintWriter(new OutputStreamWriter(socket.getOutputStream()), true)
        ) {
            socket.setSoTimeout(30000);

            String line;
            while ((line = reader.readLine()) != null) {
                if (line.trim().isEmpty()) {
                    continue;
                }

                System.out.println("[SwtAgent] Received request: " + line.substring(0, Math.min(100, line.length())));
                String response = processRequest(line);
                System.out.println("[SwtAgent] Sending response: " + response.substring(0, Math.min(100, response.length())));
                writer.println(response);
                writer.flush();
            }
            System.out.println("[SwtAgent] Client closed connection normally");
        } catch (IOException e) {
            System.out.println("[SwtAgent] Client disconnected: " + e.getMessage());
        } finally {
            try {
                socket.close();
            } catch (IOException ignored) {}
        }
    }

    private String processRequest(String requestJson) {
        JsonObject response = new JsonObject();
        response.addProperty("jsonrpc", "2.0");

        try {
            JsonObject request = JsonParser.parseString(requestJson).getAsJsonObject();
            long id = request.has("id") ? request.get("id").getAsLong() : 0;
            response.addProperty("id", id);

            String method = request.get("method").getAsString();
            JsonElement params = request.has("params") ? request.get("params") : null;

            System.err.println("[SwtAgent] Processing: " + method);
            System.err.flush();
            System.err.println("[SwtAgent] About to call dispatchMethod");
            System.err.flush();
            JsonElement result = dispatchMethod(method, params);
            System.err.println("[SwtAgent] dispatchMethod returned");
            System.err.flush();
            response.add("result", result);
            System.err.println("[SwtAgent] Success: " + method);
            System.err.flush();

        } catch (JsonSyntaxException e) {
            System.err.println("[SwtAgent] Parse error: " + e.getMessage());
            response.add("error", createError(-32700, "Parse error: " + e.getMessage()));
            response.addProperty("id", JsonNull.INSTANCE.toString());
        } catch (IllegalArgumentException e) {
            System.err.println("[SwtAgent] Invalid params: " + e.getMessage());
            response.add("error", createError(-32602, "Invalid params: " + e.getMessage()));
        } catch (NoSuchMethodException e) {
            System.err.println("[SwtAgent] Method not found: " + e.getMessage());
            response.add("error", createError(-32601, "Method not found: " + e.getMessage()));
        } catch (Exception e) {
            System.err.println("[SwtAgent] Internal error: " + e.getMessage());
            e.printStackTrace();
            response.add("error", createError(-32603, "Internal error: " + e.getMessage()));
        }

        return gson.toJson(response);
    }

    /**
     * Helper method to get optional string from JSON object.
     * Returns null if key doesn't exist or value is null/JsonNull.
     */
    private String getOptionalString(JsonObject obj, String key) {
        if (!obj.has(key)) return null;
        JsonElement elem = obj.get(key);
        if (elem == null || elem.isJsonNull()) return null;
        return elem.getAsString();
    }

    private JsonElement dispatchMethod(String method, JsonElement params) throws Exception {
        System.err.println("[SwtAgent] dispatchMethod: method=" + method);
        System.err.flush();
        JsonObject paramsObj = params != null && params.isJsonObject() ? params.getAsJsonObject() : new JsonObject();
        System.err.println("[SwtAgent] dispatchMethod: entering switch");
        System.err.flush();

        switch (method) {
            // Connection/Status
            case "ping":
                return new JsonPrimitive("pong");

            case "getVersion":
                return new JsonPrimitive("1.0.0");

            case "getToolkitType":
                return new JsonPrimitive("swt");

            // Shell/Window operations
            case "listShells":
            case "listWindows":
                System.err.println("[SwtAgent] Calling SwtReflectionBridge.getShells()...");
                System.err.flush();
                try {
                    // Use reflection-based bridge to avoid classloader issues
                    return SwtReflectionBridge.getShells();
                } catch (Exception e) {
                    System.err.println("[SwtAgent] Error: " + e.getMessage());
                    e.printStackTrace();
                    throw new RuntimeException("Failed to get shells: " + e.getMessage());
                }

            case "activateShell":
                int shellId = getWidgetId(paramsObj);
                System.err.println("[SwtAgent] activateShell: shellId=" + shellId);
                System.err.flush();
                try {
                    // Use SwtReflectionBridge which uses reflection to avoid classloader issues
                    SwtReflectionBridge.activateShell(shellId);
                    System.err.println("[SwtAgent] activateShell: completed successfully");
                    System.err.flush();
                } catch (Throwable t) {
                    System.err.println("[SwtAgent] activateShell error: " + t.getClass().getName() + ": " + t.getMessage());
                    t.printStackTrace(System.err);
                    System.err.flush();
                    throw new RuntimeException("Failed to activate shell: " + t.getMessage(), t);
                }
                return JsonNull.INSTANCE;

            case "closeShell":
                try {
                    // Use SwtReflectionBridge which uses reflection
                    SwtReflectionBridge.closeShell(getWidgetId(paramsObj));
                } catch (Exception e) {
                    System.err.println("[SwtAgent] closeShell error: " + e.getMessage());
                    throw new RuntimeException("Failed to close shell: " + e.getMessage(), e);
                }
                return JsonNull.INSTANCE;

            case "getWidgetTree":
            case "getComponentTree":
                try {
                    // Use reflection-based bridge
                    return SwtReflectionBridge.getWidgetTree();
                } catch (Exception e) {
                    System.err.println("[SwtAgent] Error getting widget tree: " + e.getMessage());
                    throw new RuntimeException("Failed to get widget tree: " + e.getMessage());
                }

            // Element finding
            case "findElement":
            case "findWidget":
                try {
                    JsonArray widgets = SwtReflectionBridge.findWidgets(paramsObj);
                    if (widgets.size() > 0) {
                        return widgets.get(0).getAsJsonObject().get("widgetId");
                    }
                    return new JsonPrimitive(-1);
                } catch (Exception e) {
                    System.err.println("[SwtAgent] Error finding widget: " + e.getMessage());
                    throw new RuntimeException("Failed to find widget: " + e.getMessage());
                }

            case "findElements":
            case "findWidgets":
                try {
                    return SwtReflectionBridge.findWidgets(paramsObj);
                } catch (Exception e) {
                    System.err.println("[SwtAgent] Error finding widgets: " + e.getMessage());
                    throw new RuntimeException("Failed to find widgets: " + e.getMessage());
                }

            case "waitForElement":
            case "waitForWidget":
                return waitForWidget(paramsObj);

            // Element properties
            case "getElementProperties":
            case "getWidgetProperties":
                int propId = paramsObj.has("widgetId") ?
                    paramsObj.get("widgetId").getAsInt() :
                    paramsObj.get("componentId").getAsInt();
                return WidgetInspector.getWidgetProperties(propId);

            case "getElementBounds":
            case "getWidgetBounds":
                return SwtActionExecutor.getElementBounds(
                    paramsObj.has("widgetId") ?
                        paramsObj.get("widgetId").getAsInt() :
                        paramsObj.get("componentId").getAsInt());

            case "getElementText":
            case "getWidgetText":
                return SwtActionExecutor.getElementText(
                    paramsObj.has("widgetId") ?
                        paramsObj.get("widgetId").getAsInt() :
                        paramsObj.get("componentId").getAsInt());

            // Actions - using SwtReflectionBridge to avoid classloader issues
            case "click":
                SwtReflectionBridge.click(getWidgetId(paramsObj));
                return JsonNull.INSTANCE;

            case "doubleClick":
                SwtReflectionBridge.doubleClick(getWidgetId(paramsObj));
                return JsonNull.INSTANCE;

            case "rightClick":
                // Fall back to SwtActionExecutor for now - can add to SwtReflectionBridge later
                SwtActionExecutor.rightClick(getWidgetId(paramsObj));
                return JsonNull.INSTANCE;

            case "setText":
                SwtReflectionBridge.setText(
                    getWidgetId(paramsObj),
                    paramsObj.get("text").getAsString()
                );
                return JsonNull.INSTANCE;

            case "typeText":
                SwtReflectionBridge.typeText(
                    getWidgetId(paramsObj),
                    paramsObj.get("text").getAsString()
                );
                return JsonNull.INSTANCE;

            case "clearText":
                SwtReflectionBridge.clearText(getWidgetId(paramsObj));
                return JsonNull.INSTANCE;

            case "selectItem":
                SwtActionExecutor.selectItem(
                    getWidgetId(paramsObj),
                    paramsObj.has("value") ? paramsObj.get("value").getAsString() :
                        paramsObj.has("index") ? paramsObj.get("index").getAsString() : null
                );
                return JsonNull.INSTANCE;

            case "focus":
                SwtActionExecutor.focus(getWidgetId(paramsObj));
                return JsonNull.INSTANCE;

            // Table operations
            case "selectTableRow":
                SwtActionExecutor.selectTableRow(
                    getWidgetId(paramsObj),
                    paramsObj.get("row").getAsInt()
                );
                return JsonNull.INSTANCE;

            case "selectTableCell":
                SwtActionExecutor.selectTableCell(
                    getWidgetId(paramsObj),
                    paramsObj.get("row").getAsInt(),
                    paramsObj.get("column").getAsInt()
                );
                return JsonNull.INSTANCE;

            case "getTableCellValue":
                return SwtActionExecutor.getTableCellValue(
                    getWidgetId(paramsObj),
                    paramsObj.get("row").getAsInt(),
                    paramsObj.get("column").getAsInt()
                );

            case "getTableData":
                return SwtActionExecutor.getTableData(getWidgetId(paramsObj));

            case "getTableRowCount":
                return new JsonPrimitive(
                    DisplayHelper.syncExecAndReturn(() -> {
                        org.eclipse.swt.widgets.Widget w = WidgetInspector.getWidgetById(getWidgetId(paramsObj));
                        if (w instanceof org.eclipse.swt.widgets.Table) {
                            return ((org.eclipse.swt.widgets.Table) w).getItemCount();
                        }
                        throw new IllegalArgumentException("Widget is not a Table");
                    })
                );

            case "getTableColumnCount":
                return new JsonPrimitive(
                    DisplayHelper.syncExecAndReturn(() -> {
                        org.eclipse.swt.widgets.Widget w = WidgetInspector.getWidgetById(getWidgetId(paramsObj));
                        if (w instanceof org.eclipse.swt.widgets.Table) {
                            return ((org.eclipse.swt.widgets.Table) w).getColumnCount();
                        }
                        throw new IllegalArgumentException("Widget is not a Table");
                    })
                );

            // Enhanced table operations
            case "getTableRowValues":
                return SwtActionExecutor.getTableRowValues(
                    getWidgetId(paramsObj),
                    paramsObj.get("row").getAsInt()
                );

            case "selectTableRows":
                JsonArray rowsArray = paramsObj.get("rows").getAsJsonArray();
                int[] rowIndices = new int[rowsArray.size()];
                for (int i = 0; i < rowsArray.size(); i++) {
                    rowIndices[i] = rowsArray.get(i).getAsInt();
                }
                SwtActionExecutor.selectTableRows(getWidgetId(paramsObj), rowIndices);
                return JsonNull.INSTANCE;

            case "deselectAllTableRows":
                SwtActionExecutor.deselectAllTableRows(getWidgetId(paramsObj));
                return JsonNull.INSTANCE;

            case "selectTableRowByValue":
                return new JsonPrimitive(
                    SwtActionExecutor.selectTableRowByValue(
                        getWidgetId(paramsObj),
                        paramsObj.get("column").getAsInt(),
                        paramsObj.get("value").getAsString()
                    )
                );

            case "selectTableRowRange":
                SwtActionExecutor.selectTableRowRange(
                    getWidgetId(paramsObj),
                    paramsObj.get("startRow").getAsInt(),
                    paramsObj.get("endRow").getAsInt()
                );
                return JsonNull.INSTANCE;

            case "setTableCellValue":
                SwtActionExecutor.setTableCellValue(
                    getWidgetId(paramsObj),
                    paramsObj.get("row").getAsInt(),
                    paramsObj.get("column").getAsInt(),
                    paramsObj.get("value").getAsString()
                );
                return JsonNull.INSTANCE;

            case "clickTableColumnHeader":
                SwtActionExecutor.clickTableColumnHeader(
                    getWidgetId(paramsObj),
                    paramsObj.get("column").getAsInt()
                );
                return JsonNull.INSTANCE;

            case "getTableSelectedRows":
                return SwtActionExecutor.getTableSelectedRows(getWidgetId(paramsObj));

            case "isTableRowSelected":
                return new JsonPrimitive(
                    SwtActionExecutor.isTableRowSelected(
                        getWidgetId(paramsObj),
                        paramsObj.get("row").getAsInt()
                    )
                );

            case "scrollToTableRow":
                SwtActionExecutor.scrollToTableRow(
                    getWidgetId(paramsObj),
                    paramsObj.get("row").getAsInt()
                );
                return JsonNull.INSTANCE;

            case "getTableColumns":
                return SwtActionExecutor.getTableColumns(getWidgetId(paramsObj));

            // Tree operations
            case "selectTreeItem":
            case "selectTreeNode":
                SwtActionExecutor.selectTreeItem(
                    getWidgetId(paramsObj),
                    paramsObj.get("path").getAsString()
                );
                return JsonNull.INSTANCE;

            case "expandTreeItem":
            case "expandTreeNode":
                SwtReflectionBridge.expandTreeItem(
                    getWidgetId(paramsObj),
                    paramsObj.has("path") ? paramsObj.get("path").getAsString() : ""
                );
                return JsonNull.INSTANCE;

            case "collapseTreeItem":
            case "collapseTreeNode":
                SwtReflectionBridge.collapseTreeItem(
                    getWidgetId(paramsObj),
                    paramsObj.has("path") ? paramsObj.get("path").getAsString() : ""
                );
                return JsonNull.INSTANCE;

            case "getTreeData":
            case "getTreeNodes":
                return SwtActionExecutor.getTreeData(getWidgetId(paramsObj));

            case "selectTreeNodes":
                JsonArray nodesArray = paramsObj.has("nodes") ? paramsObj.get("nodes").getAsJsonArray() : new JsonArray();
                String[] nodesPaths = new String[nodesArray.size()];
                for (int i = 0; i < nodesArray.size(); i++) {
                    nodesPaths[i] = nodesArray.get(i).getAsString();
                }
                SwtActionExecutor.selectTreeNodes(getWidgetId(paramsObj), nodesPaths);
                return JsonNull.INSTANCE;

            case "getTreeNodeParent":
                return SwtActionExecutor.getTreeNodeParent(
                    getWidgetId(paramsObj),
                    paramsObj.get("nodeName").getAsString()
                );

            case "getTreeNodeLevel":
                return SwtActionExecutor.getTreeNodeLevel(
                    getWidgetId(paramsObj),
                    paramsObj.get("nodeName").getAsString()
                );

            case "treeNodeExists":
                return SwtActionExecutor.treeNodeExists(
                    getWidgetId(paramsObj),
                    paramsObj.get("nodeName").getAsString()
                );

            case "getSelectedTreeNodes":
                return SwtActionExecutor.getSelectedTreeNodes(getWidgetId(paramsObj));

            case "deselectAllTreeNodes":
                SwtActionExecutor.deselectAllTreeNodes(getWidgetId(paramsObj));
                return JsonNull.INSTANCE;

            // Wait operations
            case "waitUntilEnabled":
                return waitUntilEnabled(paramsObj);

            case "waitUntilVisible":
                return waitUntilVisible(paramsObj);

            case "waitUntilNotVisible":
                return waitUntilNotVisible(paramsObj);

            // Screenshot
            case "captureScreenshot":
                return SwtActionExecutor.captureScreenshot(
                    paramsObj.has("widgetId") ? paramsObj.get("widgetId").getAsInt() :
                        paramsObj.has("componentId") ? paramsObj.get("componentId").getAsInt() : -1
                );

            // Cache management
            case "clearCache":
                WidgetInspector.clearCache();
                return JsonNull.INSTANCE;

            case "cleanupCache":
                WidgetInspector.cleanupCache();
                return JsonNull.INSTANCE;

            // =============================================================
            // RCP (Eclipse Rich Client Platform) Operations
            // =============================================================

            // Workbench Information
            case "rcp.getWorkbenchInfo":
                return getWorkbenchInfo();

            case "rcp.getAvailablePerspectives":
                return getAvailablePerspectives();

            case "rcp.getActivePerspective":
                return getActivePerspective();

            case "rcp.getOpenPerspectives":
                return getOpenPerspectives();

            // Perspective Operations
            case "rcp.openPerspective":
                return openPerspective(paramsObj.get("perspectiveId").getAsString());

            case "rcp.openPerspectiveByName":
                return openPerspectiveByName(paramsObj.get("name").getAsString());

            case "rcp.closePerspective":
                return closePerspective(
                    paramsObj.has("perspectiveId") ? paramsObj.get("perspectiveId").getAsString() : null
                );

            case "rcp.closeAllPerspectives":
                return closeAllPerspectives();

            case "rcp.resetPerspective":
                return resetPerspective();

            case "rcp.savePerspectiveAs":
                return savePerspectiveAs(paramsObj.get("name").getAsString());

            // View Operations
            case "rcp.showView":
                return showView(
                    paramsObj.get("viewId").getAsString(),
                    getOptionalString(paramsObj, "secondaryId")
                );

            case "rcp.showViewByName":
                return showViewByName(paramsObj.get("name").getAsString());

            case "rcp.closeView":
                return closeView(
                    paramsObj.get("viewId").getAsString(),
                    getOptionalString(paramsObj, "secondaryId")
                );

            case "rcp.activateView":
                return activateView(paramsObj.get("viewId").getAsString());

            case "rcp.getOpenViews":
                return getOpenViews();

            case "rcp.getActiveView":
                return getActiveView();

            case "rcp.isViewVisible":
                return isViewVisible(paramsObj.get("viewId").getAsString());

            case "rcp.minimizeView":
                return minimizeView(paramsObj.get("viewId").getAsString());

            case "rcp.maximizeView":
                return maximizeView(paramsObj.get("viewId").getAsString());

            case "rcp.restoreView":
                return restoreView(paramsObj.get("viewId").getAsString());

            case "rcp.isViewMinimized":
                return isViewMinimized(paramsObj.get("viewId").getAsString());

            case "rcp.isViewMaximized":
                return isViewMaximized(paramsObj.get("viewId").getAsString());

            case "rcp.getViewTitle":
                return getViewTitle(paramsObj.get("viewId").getAsString());

            // Editor Operations
            case "rcp.openEditor":
                return openEditor(paramsObj.get("filePath").getAsString());

            case "rcp.closeEditor":
                return closeEditor(
                    paramsObj.get("filePath").getAsString(),
                    paramsObj.has("save") ? paramsObj.get("save").getAsBoolean() : true
                );

            case "rcp.closeAllEditors":
                return closeAllEditors(paramsObj.has("save") ? paramsObj.get("save").getAsBoolean() : true);

            case "rcp.activateEditor":
                return activateEditor(paramsObj.get("filePath").getAsString());

            case "rcp.saveEditor":
                return saveEditor(paramsObj.has("filePath") ? paramsObj.get("filePath").getAsString() : null);

            case "rcp.saveAllEditors":
                return saveAllEditors();

            case "rcp.getActiveEditor":
                return getActiveEditor();

            case "rcp.getOpenEditors":
                return getOpenEditors();

            case "rcp.isEditorOpen":
                return isEditorOpen(paramsObj.get("filePath").getAsString());

            case "rcp.isEditorDirty":
                return isEditorDirty(paramsObj.has("filePath") ? paramsObj.get("filePath").getAsString() : null);

            case "rcp.getEditorContent":
                return getEditorContent(paramsObj.get("filePath").getAsString());

            case "rcp.getDirtyEditorCount":
                return getDirtyEditorCount();

            case "rcp.enterTextInEditor":
                return enterTextInEditor(paramsObj.get("text").getAsString());

            // Command Operations
            case "rcp.executeCommand":
                return executeCommand(paramsObj.get("commandId").getAsString());

            case "rcp.executeMenu":
                return executeMenu(paramsObj.get("menuPath").getAsString());

            // Dialog Operations
            case "rcp.openPreferences":
                return openPreferences();

            case "rcp.getOpenDialogs":
                return getOpenDialogs();

            // Workbench Window Operations
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
                    paramsObj.has("timeout") ? paramsObj.get("timeout").getAsLong() : 30000
                );

            case "rcp.pressButton":
                return pressButton(
                    paramsObj.get("label").getAsString()
                );

            case "rcp.closeActiveDialog":
                return closeActiveDialog();

            // Additional RCP methods
            case "rcp.navigateToPreferencePage":
                return navigateToPreferencePage(paramsObj.get("path").getAsString());

            case "rcp.getViewWidget":
                return getViewWidget(
                    paramsObj.get("viewId").getAsString(),
                    paramsObj.has("locator") ? paramsObj.get("locator").getAsString() : null
                );

            case "rcp.getEditorWidget":
                return getEditorWidget(
                    paramsObj.get("title").getAsString(),
                    paramsObj.has("locator") ? paramsObj.get("locator").getAsString() : null
                );

            case "rcp.clickToolbarItem":
                return clickToolbarItem(paramsObj.get("tooltip").getAsString());

            case "rcp.getAvailableCommands":
                return getAvailableCommands();

            case "rcp.selectContextMenu":
                return selectContextMenu(paramsObj.get("path").getAsString());

            case "rcp.selectMainMenu":
                return selectMainMenu(paramsObj.get("path").getAsString());

            // RCP Component Tree methods (Phase 6)
            case "rcp.getComponentTree":
                int maxDepth = paramsObj.has("maxDepth") ? paramsObj.get("maxDepth").getAsInt() : 5;
                return RcpComponentInspector.getRcpComponentTree(maxDepth);

            case "rcp.getAllViews":
                boolean includeWidgets = paramsObj.has("includeSwtWidgets") ?
                    paramsObj.get("includeSwtWidgets").getAsBoolean() : false;
                return RcpComponentInspector.getAllViews(includeWidgets);

            case "rcp.getAllEditors":
                boolean includeEditorWidgets = paramsObj.has("includeSwtWidgets") ?
                    paramsObj.get("includeSwtWidgets").getAsBoolean() : false;
                return RcpComponentInspector.getAllEditors(includeEditorWidgets);

            case "rcp.getComponent":
                String componentPath = paramsObj.get("path").getAsString();
                int componentDepth = paramsObj.has("maxDepth") ? paramsObj.get("maxDepth").getAsInt() : 3;
                return RcpComponentInspector.getRcpComponent(componentPath, componentDepth);

            default:
                throw new NoSuchMethodException(method);
        }
    }

    private int getWidgetId(JsonObject params) {
        if (params.has("widgetId")) {
            return params.get("widgetId").getAsInt();
        }
        if (params.has("componentId")) {
            return params.get("componentId").getAsInt();
        }
        throw new IllegalArgumentException("Missing widgetId or componentId parameter");
    }

    private JsonElement waitForWidget(JsonObject params) {
        long timeout = params.has("timeout") ? params.get("timeout").getAsLong() : 10000;
        long pollInterval = params.has("pollInterval") ? params.get("pollInterval").getAsLong() : 100;

        long startTime = System.currentTimeMillis();
        while (System.currentTimeMillis() - startTime < timeout) {
            int id = WidgetInspector.findWidget(params);
            if (id >= 0) {
                return new JsonPrimitive(id);
            }
            DisplayHelper.sleep(pollInterval);
        }

        throw new IllegalStateException("Widget not found within timeout");
    }

    private JsonElement waitUntilEnabled(JsonObject params) {
        int widgetId = getWidgetId(params);
        long timeout = params.has("timeout") ? params.get("timeout").getAsLong() : 10000;

        boolean result = DisplayHelper.waitForCondition(() -> {
            org.eclipse.swt.widgets.Widget w = WidgetInspector.getWidgetById(widgetId);
            if (w == null || w.isDisposed()) {
                return false;
            }
            if (w instanceof org.eclipse.swt.widgets.Control) {
                return ((org.eclipse.swt.widgets.Control) w).isEnabled();
            }
            return true;
        }, timeout);

        return new JsonPrimitive(result);
    }

    private JsonElement waitUntilVisible(JsonObject params) {
        int widgetId = getWidgetId(params);
        long timeout = params.has("timeout") ? params.get("timeout").getAsLong() : 10000;

        boolean result = DisplayHelper.waitForCondition(() -> {
            org.eclipse.swt.widgets.Widget w = WidgetInspector.getWidgetById(widgetId);
            if (w == null || w.isDisposed()) {
                return false;
            }
            if (w instanceof org.eclipse.swt.widgets.Control) {
                return ((org.eclipse.swt.widgets.Control) w).isVisible();
            }
            return true;
        }, timeout);

        return new JsonPrimitive(result);
    }

    private JsonElement waitUntilNotVisible(JsonObject params) {
        int widgetId = getWidgetId(params);
        long timeout = params.has("timeout") ? params.get("timeout").getAsLong() : 10000;

        boolean result = DisplayHelper.waitForCondition(() -> {
            org.eclipse.swt.widgets.Widget w = WidgetInspector.getWidgetById(widgetId);
            if (w == null || w.isDisposed()) {
                return true;
            }
            if (w instanceof org.eclipse.swt.widgets.Control) {
                return !((org.eclipse.swt.widgets.Control) w).isVisible();
            }
            return false;
        }, timeout);

        return new JsonPrimitive(result);
    }

    private JsonObject createError(int code, String message) {
        JsonObject error = new JsonObject();
        error.addProperty("code", code);
        error.addProperty("message", message);
        return error;
    }

    // =============================================================
    // Mock RCP Application Support
    // =============================================================

    private static Object mockRcpApp = null;
    private static Class<?> mockRcpAppClass = null;
    private static boolean mockRcpChecked = false;

    /**
     * Try to get the MockRcpApplication instance via reflection.
     * Returns null if not available.
     */
    private Object getMockRcpApp() {
        if (!mockRcpChecked) {
            mockRcpChecked = true;
            try {
                mockRcpAppClass = Class.forName("testapp.rcp.MockRcpApplication");
                java.lang.reflect.Method getInstance = mockRcpAppClass.getMethod("getInstance");
                mockRcpApp = getInstance.invoke(null);
                if (mockRcpApp != null) {
                    System.out.println("[SwtRpcServer] MockRcpApplication detected and will be used for RCP operations");
                }
            } catch (ClassNotFoundException e) {
                // MockRcpApplication not on classpath - normal for real Eclipse apps
            } catch (Exception e) {
                System.err.println("[SwtRpcServer] Error initializing MockRcpApplication: " + e.getMessage());
            }
        }
        return mockRcpApp;
    }

    /**
     * Invoke a method on MockRcpApplication via reflection.
     */
    @SuppressWarnings("unchecked")
    private <T> T invokeMockMethod(String methodName, Class<T> returnType, Object... args) {
        Object app = getMockRcpApp();
        if (app == null) return null;
        try {
            Class<?>[] paramTypes = new Class<?>[args.length];
            for (int i = 0; i < args.length; i++) {
                if (args[i] == null) {
                    paramTypes[i] = String.class; // Default to String for null
                } else {
                    paramTypes[i] = args[i].getClass();
                    // Handle primitive wrappers
                    if (paramTypes[i] == Boolean.class) paramTypes[i] = boolean.class;
                    if (paramTypes[i] == Integer.class) paramTypes[i] = int.class;
                }
            }
            java.lang.reflect.Method method = mockRcpAppClass.getMethod(methodName, paramTypes);
            Object result = method.invoke(app, args);
            return returnType.cast(result);
        } catch (NoSuchMethodException e) {
            // Try without parameters if that fails
            try {
                java.lang.reflect.Method method = mockRcpAppClass.getMethod(methodName);
                Object result = method.invoke(app);
                return returnType.cast(result);
            } catch (Exception ex) {
                return null;
            }
        } catch (Exception e) {
            System.err.println("[SwtRpcServer] Error invoking mock method " + methodName + ": " + e.getMessage());
            return null;
        }
    }

    /**
     * Invoke a void method on MockRcpApplication.
     */
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
            System.err.println("[SwtRpcServer] Error invoking mock void method " + methodName + ": " + e.getMessage());
            return false;
        }
    }

    // =============================================================
    // RCP Helper Methods (Dual-mode: Mock + Real Eclipse support)
    // Methods first try MockRcpApplication for testing, then
    // fallback to EclipseWorkbenchHelper for real Eclipse RCP apps.
    // =============================================================

    private JsonElement getActivePerspective() {
        // Try mock first
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

        // Fallback to real Eclipse API
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
        // Try mock first
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

        // Fallback to real Eclipse API - return active perspective as the only open one
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
        // Try mock first
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

        // Fallback to real Eclipse API
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

    private JsonElement openPerspective(String perspectiveId) {
        // Try mock first
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

        // Fallback to real Eclipse API
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

    private JsonElement resetPerspective() {
        // Try mock first
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("resetPerspective")) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        // Fallback to real Eclipse API
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

    private JsonElement showView(String viewId, String secondaryId) {
        // Try mock first
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

        // Fallback to real Eclipse API
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

    private JsonElement closeView(String viewId, String secondaryId) {
        // Try mock first
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("closeView", viewId)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        // Fallback to real Eclipse API
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
        // Try mock first
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("activateView", viewId)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        // Fallback to real Eclipse API
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

    private JsonElement closeAllEditors(boolean save) {
        // Try mock first
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("closeAllEditors", save)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        // Fallback to real Eclipse API
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

    private JsonElement saveAllEditors() {
        // Try mock first
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("saveAllEditors")) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }

        // Fallback to real Eclipse API
        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            if (EclipseWorkbenchHelper.saveAllEditors(false)) {
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
        // Try mock first
        Object app = getMockRcpApp();
        if (app != null) {
            String editorName = invokeMockMethod("getActiveEditor", String.class);
            if (editorName != null) {
                JsonObject result = new JsonObject();
                result.addProperty("name", editorName);
                result.addProperty("title", editorName);
                return result;
            }
        }

        // Fallback to real Eclipse API
        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            java.util.Map<String, Object> editorInfo = EclipseWorkbenchHelper.getActiveEditor();
            if (editorInfo != null) {
                JsonObject result = new JsonObject();
                for (java.util.Map.Entry<String, Object> entry : editorInfo.entrySet()) {
                    Object value = entry.getValue();
                    if (value instanceof String) {
                        result.addProperty(entry.getKey(), (String) value);
                    } else if (value instanceof Boolean) {
                        result.addProperty(entry.getKey(), (Boolean) value);
                    }
                }
                return result;
            }
        }

        JsonObject result = new JsonObject();
        result.addProperty("error", "No RCP workbench available or no active editor");
        return result;
    }

    private JsonElement executeCommand(String commandId) {
        // Try mock first
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("executeCommand", commandId)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                result.addProperty("commandId", commandId);
                return result;
            }
        }

        // Fallback to real Eclipse API
        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            if (EclipseWorkbenchHelper.executeCommand(commandId)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                result.addProperty("commandId", commandId);
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

    private JsonElement showViewByName(String name) {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("showViewByName", name)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                return result;
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available or view not found: " + name);
        return result;
    }

    @SuppressWarnings("unchecked")
    private JsonElement getOpenViews() {
        // Try mock first
        Object app = getMockRcpApp();
        if (app != null) {
            java.util.List<?> viewList = invokeMockMethod("getOpenViews", java.util.List.class);
            if (viewList != null) {
                JsonArray views = new JsonArray();
                for (Object item : viewList) {
                    JsonObject viewObj = new JsonObject();
                    if (item instanceof java.util.Map) {
                        java.util.Map<String, Object> viewMap = (java.util.Map<String, Object>) item;
                        viewObj.addProperty("id", String.valueOf(viewMap.get("id")));
                        viewObj.addProperty("title", String.valueOf(viewMap.get("name")));
                        viewObj.addProperty("visible", Boolean.TRUE.equals(viewMap.get("visible")));
                    } else {
                        String id = String.valueOf(item);
                        viewObj.addProperty("id", id);
                        viewObj.addProperty("title", id.substring(id.lastIndexOf('.') + 1));
                    }
                    views.add(viewObj);
                }
                return views;
            }
        }

        // Fallback to real Eclipse API
        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            java.util.List<java.util.Map<String, Object>> viewList = EclipseWorkbenchHelper.getOpenViews();
            JsonArray views = new JsonArray();
            for (java.util.Map<String, Object> viewInfo : viewList) {
                JsonObject viewObj = new JsonObject();
                for (java.util.Map.Entry<String, Object> entry : viewInfo.entrySet()) {
                    Object value = entry.getValue();
                    if (value instanceof String) {
                        viewObj.addProperty(entry.getKey(), (String) value);
                    } else if (value instanceof Boolean) {
                        viewObj.addProperty(entry.getKey(), (Boolean) value);
                    }
                }
                views.add(viewObj);
            }
            return views;
        }

        return new JsonArray();
    }

    private JsonElement getActiveView() {
        Object app = getMockRcpApp();
        if (app != null) {
            String viewId = invokeMockMethod("getActiveView", String.class);
            if (viewId != null) {
                JsonObject result = new JsonObject();
                result.addProperty("id", viewId);
                result.addProperty("title", viewId.substring(viewId.lastIndexOf('.') + 1));
                return result;
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("error", "No RCP workbench available or no active view");
        return result;
    }

    private JsonElement isViewVisible(String viewId) {
        Object app = getMockRcpApp();
        if (app != null) {
            Boolean visible = invokeMockMethod("isViewVisible", Boolean.class, viewId);
            if (visible != null) {
                return new JsonPrimitive(visible);
            }
        }
        return new JsonPrimitive(false);
    }

    private JsonElement minimizeView(String viewId) {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("minimizeView", viewId)) {
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

    private JsonElement maximizeView(String viewId) {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("maximizeView", viewId)) {
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

    private JsonElement restoreView(String viewId) {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("restoreView", viewId)) {
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

    private JsonElement isViewMinimized(String viewId) {
        Object app = getMockRcpApp();
        if (app != null) {
            Boolean minimized = invokeMockMethod("isViewMinimized", Boolean.class, viewId);
            if (minimized != null) {
                return new JsonPrimitive(minimized);
            }
        }
        return new JsonPrimitive(false);
    }

    private JsonElement isViewMaximized(String viewId) {
        Object app = getMockRcpApp();
        if (app != null) {
            Boolean maximized = invokeMockMethod("isViewMaximized", Boolean.class, viewId);
            if (maximized != null) {
                return new JsonPrimitive(maximized);
            }
        }
        return new JsonPrimitive(false);
    }

    private JsonElement getViewTitle(String viewId) {
        Object app = getMockRcpApp();
        if (app != null) {
            String title = invokeMockMethod("getViewTitle", String.class, viewId);
            if (title != null) {
                return new JsonPrimitive(title);
            }
        }
        return new JsonPrimitive("");
    }

    private JsonElement openEditor(String filePath) {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("openEditor", filePath)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                result.addProperty("filePath", filePath);
                return result;
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement closeEditor(String filePath, boolean save) {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("closeEditor", filePath)) {
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
        if (app != null) {
            if (invokeMockVoidMethod("activateEditor", filePath)) {
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

    private JsonElement saveEditor(String filePath) {
        Object app = getMockRcpApp();
        if (app != null) {
            // If filePath is null or empty, save the active editor
            String pathToSave = filePath;
            if (pathToSave == null || pathToSave.isEmpty()) {
                // Get active editor's path from getActiveEditor method
                pathToSave = invokeMockMethod("getActiveEditor", String.class);
            }
            if (pathToSave != null && invokeMockVoidMethod("saveEditor", pathToSave)) {
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

    @SuppressWarnings("unchecked")
    private JsonElement getOpenEditors() {
        // Try mock first
        Object app = getMockRcpApp();
        if (app != null) {
            java.util.List<?> editorList = invokeMockMethod("getOpenEditors", java.util.List.class);
            if (editorList != null) {
                JsonArray editors = new JsonArray();
                for (Object item : editorList) {
                    JsonObject editorObj = new JsonObject();
                    if (item instanceof java.util.Map) {
                        java.util.Map<String, Object> editorMap = (java.util.Map<String, Object>) item;
                        editorObj.addProperty("id", String.valueOf(editorMap.get("id")));
                        editorObj.addProperty("title", String.valueOf(editorMap.get("title")));
                        editorObj.addProperty("filePath", String.valueOf(editorMap.get("filePath")));
                        editorObj.addProperty("dirty", Boolean.TRUE.equals(editorMap.get("dirty")));
                    } else {
                        String name = String.valueOf(item);
                        editorObj.addProperty("name", name);
                        editorObj.addProperty("title", name);
                    }
                    editors.add(editorObj);
                }
                return editors;
            }
        }

        // Fallback to real Eclipse API
        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            java.util.List<java.util.Map<String, Object>> editorList = EclipseWorkbenchHelper.getOpenEditors();
            JsonArray editors = new JsonArray();
            for (java.util.Map<String, Object> editorInfo : editorList) {
                JsonObject editorObj = new JsonObject();
                for (java.util.Map.Entry<String, Object> entry : editorInfo.entrySet()) {
                    Object value = entry.getValue();
                    if (value instanceof String) {
                        editorObj.addProperty(entry.getKey(), (String) value);
                    } else if (value instanceof Boolean) {
                        editorObj.addProperty(entry.getKey(), (Boolean) value);
                    }
                }
                editors.add(editorObj);
            }
            return editors;
        }

        return new JsonArray();
    }

    private JsonElement isEditorOpen(String filePath) {
        Object app = getMockRcpApp();
        if (app != null) {
            Boolean open = invokeMockMethod("isEditorOpen", Boolean.class, filePath);
            if (open != null) {
                return new JsonPrimitive(open);
            }
        }
        return new JsonPrimitive(false);
    }

    private JsonElement isEditorDirty(String filePath) {
        Object app = getMockRcpApp();
        if (app != null) {
            // If filePath is null or empty, check the active editor
            String pathToCheck = filePath;
            if (pathToCheck == null || pathToCheck.isEmpty()) {
                pathToCheck = invokeMockMethod("getActiveEditor", String.class);
            }
            if (pathToCheck != null) {
                Boolean dirty = invokeMockMethod("isEditorDirty", Boolean.class, pathToCheck);
                if (dirty != null) {
                    return new JsonPrimitive(dirty);
                }
            }
        }
        return new JsonPrimitive(false);
    }

    private JsonElement getEditorContent(String filePath) {
        Object app = getMockRcpApp();
        if (app != null) {
            String content = invokeMockMethod("getEditorContent", String.class, filePath);
            if (content != null) {
                return new JsonPrimitive(content);
            }
        }
        return new JsonPrimitive("");
    }

    private JsonElement getDirtyEditorCount() {
        Object app = getMockRcpApp();
        if (app != null) {
            Integer count = invokeMockMethod("getDirtyEditorCount", Integer.class);
            if (count != null) {
                return new JsonPrimitive(count);
            }
        }
        return new JsonPrimitive(0);
    }

    private JsonElement enterTextInEditor(String text) {
        Object app = getMockRcpApp();
        if (app != null) {
            if (invokeMockVoidMethod("enterTextInEditor", text)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                result.addProperty("insertedText", text);
                return result;
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available or no active text editor");
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
        if (app != null) {
            if (invokeMockVoidMethod("openPreferencesDialog")) {
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

    private JsonElement getOpenDialogs() {
        return DisplayHelper.syncExecAndReturn(() -> {
            JsonArray dialogs = new JsonArray();
            org.eclipse.swt.widgets.Display display = org.eclipse.swt.widgets.Display.getCurrent();
            if (display != null) {
                for (org.eclipse.swt.widgets.Shell shell : display.getShells()) {
                    if (shell.getParent() != null) { // Dialogs have parent shells
                        JsonObject dialogObj = new JsonObject();
                        dialogObj.addProperty("title", shell.getText());
                        dialogObj.addProperty("visible", shell.isVisible());
                        dialogs.add(dialogObj);
                    }
                }
            }
            return dialogs;
        });
    }

    private JsonElement getActiveWorkbenchWindow() {
        Object app = getMockRcpApp();
        if (app != null) {
            JsonObject result = new JsonObject();
            result.addProperty("title", "Mock RCP Application");
            result.addProperty("active", true);
            return result;
        }
        JsonObject result = new JsonObject();
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement getWorkbenchWindowCount() {
        // Try mock first
        Object app = getMockRcpApp();
        if (app != null) {
            return new JsonPrimitive(1);
        }

        // Fallback to real Eclipse API
        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            return new JsonPrimitive(EclipseWorkbenchHelper.getWorkbenchWindowCount());
        }

        return new JsonPrimitive(0);
    }

    private JsonElement getWorkbenchTitle() {
        // Try mock first
        Object app = getMockRcpApp();
        if (app != null) {
            String title = invokeMockMethod("getTitle", String.class);
            if (title != null) {
                return new JsonPrimitive(title);
            }
            return new JsonPrimitive("Mock RCP Application");
        }

        // Fallback to real Eclipse API
        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            String title = EclipseWorkbenchHelper.getWorkbenchTitle();
            if (title != null) {
                return new JsonPrimitive(title);
            }
        }

        return new JsonPrimitive("");
    }

    private JsonElement getWorkbenchState() {
        // Try mock first
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

        // Fallback to real Eclipse API
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
        // Try mock first
        Object app = getMockRcpApp();
        if (app != null) {
            // Mock app is always ready
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            result.addProperty("elapsed", 0);
            return result;
        }

        // Fallback to real Eclipse API - wait for workbench to be available
        if (EclipseWorkbenchHelper.isEclipseAvailable()) {
            JsonObject result = new JsonObject();
            result.addProperty("success", true);
            result.addProperty("elapsed", 0);
            return result;
        }

        // Wait for Eclipse workbench to become available
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

    // =============================================================
    // Additional RCP Methods
    // =============================================================

    private JsonElement navigateToPreferencePage(String pagePath) {
        Object app = getMockRcpApp();
        if (app != null) {
            // Navigate to the preference page
            if (invokeMockVoidMethod("navigateToPreferencePage", pagePath)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                result.addProperty("pagePath", pagePath);
                return result;
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    private JsonElement getViewWidget(String viewId, String widgetType) {
        System.out.println("[RPC] getViewWidget called: viewId=" + viewId + ", widgetType=" + widgetType);
        Object app = getMockRcpApp();
        System.out.println("[RPC] getMockRcpApp returned: " + (app != null ? app.getClass().getName() : "null"));
        if (app != null) {
            try {
                // Call getViewWidget on MockRcpApplication
                java.lang.reflect.Method method = app.getClass().getMethod("getViewWidget", String.class, String.class);
                System.out.println("[RPC] Found getViewWidget method, invoking...");
                @SuppressWarnings("unchecked")
                java.util.Map<String, Object> widgetMap = (java.util.Map<String, Object>) method.invoke(app, viewId, widgetType);
                System.out.println("[RPC] getViewWidget returned: " + (widgetMap != null ? widgetMap.toString() : "null"));

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
                System.err.println("[RPC] getViewWidget error: " + e.getMessage());
                e.printStackTrace();
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
                // Call getEditorWidget on MockRcpApplication
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
                System.err.println("[RPC] getEditorWidget error: " + e.getMessage());
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("error", "Widget not found in editor");
        return result;
    }

    private JsonElement clickToolbarItem(String tooltip) {
        Object app = getMockRcpApp();
        if (app != null) {
            // Find and click toolbar item via mock app
            if (invokeMockVoidMethod("clickToolbarItem", tooltip)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                result.addProperty("tooltip", tooltip);
                return result;
            }
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
            // Add some mock commands
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
        if (app != null) {
            if (invokeMockVoidMethod("executeMenu", menuPath)) {
                JsonObject result = new JsonObject();
                result.addProperty("success", true);
                result.addProperty("menuPath", menuPath);
                return result;
            }
        }
        JsonObject result = new JsonObject();
        result.addProperty("success", false);
        result.addProperty("error", "No RCP workbench available");
        return result;
    }

    // =============================================================
    // SWT-only Methods (no Eclipse UI dependencies)
    // =============================================================

    private JsonElement pressButton(String label) {
        return DisplayHelper.syncExecAndReturn(() -> {
            JsonObject result = new JsonObject();
            org.eclipse.swt.widgets.Shell activeShell = org.eclipse.swt.widgets.Display.getCurrent().getActiveShell();

            if (activeShell == null) {
                throw new IllegalStateException("No active shell found");
            }

            org.eclipse.swt.widgets.Button button = findButtonByLabel(activeShell, label);
            if (button == null) {
                throw new IllegalStateException("Button with label '" + label + "' not found");
            }

            // Click the button
            org.eclipse.swt.widgets.Event event = new org.eclipse.swt.widgets.Event();
            event.widget = button;
            button.notifyListeners(org.eclipse.swt.SWT.Selection, event);

            result.addProperty("success", true);
            result.addProperty("buttonLabel", label);
            return result;
        });
    }

    private org.eclipse.swt.widgets.Button findButtonByLabel(org.eclipse.swt.widgets.Composite parent, String label) {
        for (org.eclipse.swt.widgets.Control child : parent.getChildren()) {
            if (child instanceof org.eclipse.swt.widgets.Button) {
                org.eclipse.swt.widgets.Button button = (org.eclipse.swt.widgets.Button) child;
                if (label.equals(button.getText())) {
                    return button;
                }
            } else if (child instanceof org.eclipse.swt.widgets.Composite) {
                org.eclipse.swt.widgets.Button found = findButtonByLabel((org.eclipse.swt.widgets.Composite) child, label);
                if (found != null) {
                    return found;
                }
            }
        }
        return null;
    }

    private JsonElement closeActiveDialog() {
        return DisplayHelper.syncExecAndReturn(() -> {
            JsonObject result = new JsonObject();
            org.eclipse.swt.widgets.Shell activeShell = org.eclipse.swt.widgets.Display.getCurrent().getActiveShell();

            if (activeShell == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active shell to close");
                return result;
            }

            // Check if this is a dialog (not the main workbench window)
            org.eclipse.swt.widgets.Shell parent = activeShell.getParent() != null ?
                (org.eclipse.swt.widgets.Shell) activeShell.getParent().getShell() : null;

            if (parent == null) {
                result.addProperty("success", false);
                result.addProperty("error", "Active shell is the main window, not a dialog");
                return result;
            }

            activeShell.close();
            result.addProperty("success", true);
            return result;
        });
    }

    public void stop() {
        running = false;

        if (serverSocket != null && !serverSocket.isClosed()) {
            try {
                serverSocket.close();
            } catch (IOException ignored) {}
        }

        if (executor != null) {
            executor.shutdown();
            try {
                if (!executor.awaitTermination(5, TimeUnit.SECONDS)) {
                    executor.shutdownNow();
                }
            } catch (InterruptedException e) {
                executor.shutdownNow();
            }
        }
    }

    public boolean isRunning() {
        return running;
    }

    public int getPort() {
        return port;
    }

    /**
     * Dynamically invoke WidgetInspector methods using the application's classloader.
     * This avoids class loading issues with OSGi bundles.
     */
    private JsonElement invokeWidgetInspector(String methodName, Object... args) throws Exception {
        System.err.println("[SwtAgent] invokeWidgetInspector: " + methodName);
        System.err.flush();

        // Find a classloader that can load SWT classes
        ClassLoader appClassLoader = findSwtClassLoader();
        if (appClassLoader == null) {
            throw new IllegalStateException("Cannot find classloader with SWT classes");
        }

        System.err.println("[SwtAgent] Using classloader: " + appClassLoader);
        System.err.flush();

        // Set the context classloader for this thread
        ClassLoader originalCL = Thread.currentThread().getContextClassLoader();
        try {
            Thread.currentThread().setContextClassLoader(appClassLoader);

            // Now load WidgetInspector using this classloader
            Class<?> wiClass = appClassLoader.loadClass("com.robotframework.swt.WidgetInspector");
            System.err.println("[SwtAgent] WidgetInspector class loaded: " + wiClass);
            System.err.flush();

            // Find and invoke the method
            java.lang.reflect.Method method;
            if (args.length == 0) {
                method = wiClass.getMethod(methodName);
                Object result = method.invoke(null);
                System.err.println("[SwtAgent] Method returned: " + result);
                System.err.flush();
                return (JsonElement) result;
            } else {
                Class<?>[] argTypes = new Class[args.length];
                for (int i = 0; i < args.length; i++) {
                    argTypes[i] = args[i].getClass();
                }
                method = wiClass.getMethod(methodName, argTypes);
                Object result = method.invoke(null, args);
                return (JsonElement) result;
            }
        } finally {
            Thread.currentThread().setContextClassLoader(originalCL);
        }
    }

    /**
     * Find a classloader that has access to SWT classes.
     */
    private ClassLoader findSwtClassLoader() {
        System.err.println("[SwtAgent] Looking for SWT classloader...");
        System.err.flush();

        // First, try Class.forName which uses the caller's classloader chain
        try {
            Class<?> displayClass = Class.forName("org.eclipse.swt.widgets.Display");
            ClassLoader cl = displayClass.getClassLoader();
            System.err.println("[SwtAgent] Found SWT via Class.forName, classloader: " + cl);
            System.err.flush();
            return cl;
        } catch (ClassNotFoundException e) {
            System.err.println("[SwtAgent] Class.forName failed: " + e.getMessage());
        }

        // Try to get the classloader from the agent's own class
        try {
            ClassLoader agentCL = SwtRpcServer.class.getClassLoader();
            System.err.println("[SwtAgent] Agent classloader: " + agentCL);
            Class<?> displayClass = agentCL.loadClass("org.eclipse.swt.widgets.Display");
            System.err.println("[SwtAgent] Found SWT via agent classloader");
            return agentCL;
        } catch (Exception e) {
            System.err.println("[SwtAgent] Agent classloader failed: " + e.getMessage());
        }

        // Try thread context classloaders from all threads
        try {
            ThreadGroup rootGroup = Thread.currentThread().getThreadGroup();
            while (rootGroup.getParent() != null) {
                rootGroup = rootGroup.getParent();
            }
            Thread[] threads = new Thread[rootGroup.activeCount() * 2];
            int count = rootGroup.enumerate(threads, true);

            System.err.println("[SwtAgent] Searching " + count + " threads...");

            for (int i = 0; i < count; i++) {
                Thread t = threads[i];
                if (t == null) continue;

                // Try thread's context classloader
                ClassLoader cl = t.getContextClassLoader();
                if (cl != null) {
                    try {
                        cl.loadClass("org.eclipse.swt.widgets.Display");
                        System.err.println("[SwtAgent] Found SWT classloader via thread '" + t.getName() + "'");
                        System.err.flush();
                        return cl;
                    } catch (ClassNotFoundException ignored) {
                    }
                }

                // Try to get classloader from thread's stacktrace classes
                try {
                    StackTraceElement[] stack = t.getStackTrace();
                    for (StackTraceElement elem : stack) {
                        String className = elem.getClassName();
                        if (className.contains("eclipse") || className.contains("swt") || className.contains("dbeaver")) {
                            try {
                                Class<?> c = Class.forName(className, false, cl);
                                ClassLoader bundleCL = c.getClassLoader();
                                if (bundleCL != null) {
                                    bundleCL.loadClass("org.eclipse.swt.widgets.Display");
                                    System.err.println("[SwtAgent] Found SWT via bundle class: " + className);
                                    return bundleCL;
                                }
                            } catch (Exception ignored) {
                            }
                        }
                    }
                } catch (Exception ignored) {
                }
            }
        } catch (Exception e) {
            System.err.println("[SwtAgent] Error searching threads: " + e.getMessage());
        }

        System.err.println("[SwtAgent] Could not find SWT classloader");
        return null;
    }
}
