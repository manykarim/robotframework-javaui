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
