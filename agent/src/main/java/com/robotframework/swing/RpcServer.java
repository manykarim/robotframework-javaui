package com.robotframework.swing;

import com.google.gson.*;

import java.io.*;
import java.net.ServerSocket;
import java.net.Socket;
import java.net.SocketException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

/**
 * JSON-RPC 2.0 server for handling automation requests.
 * Listens for incoming connections and dispatches method calls.
 */
public class RpcServer implements Runnable {

    private final String host;
    private final int port;
    private volatile boolean running = false;
    private ServerSocket serverSocket;
    private ExecutorService executor;
    private final Gson gson;

    public RpcServer(String host, int port) {
        this.host = host;
        this.port = port;
        this.gson = new GsonBuilder().setPrettyPrinting().create();
    }

    @Override
    public void run() {
        running = true;
        executor = Executors.newCachedThreadPool(r -> {
            Thread t = new Thread(r, "SwingAgent-RpcHandler");
            t.setDaemon(true);
            return t;
        });

        try {
            serverSocket = new ServerSocket(port);
            System.out.println("[SwingAgent] RPC server listening on " + host + ":" + port);

            while (running) {
                try {
                    Socket clientSocket = serverSocket.accept();
                    executor.submit(() -> handleClient(clientSocket));
                } catch (SocketException e) {
                    if (running) {
                        System.err.println("[SwingAgent] Socket error: " + e.getMessage());
                    }
                }
            }
        } catch (IOException e) {
            System.err.println("[SwingAgent] Failed to start server: " + e.getMessage());
        }
    }

    private void handleClient(Socket socket) {
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

                String response = processRequest(line);
                writer.println(response);
            }
        } catch (IOException e) {
            // Client disconnected
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

            JsonElement result = dispatchMethod(method, params);
            response.add("result", result);

        } catch (JsonSyntaxException e) {
            response.add("error", createError(-32700, "Parse error: " + e.getMessage()));
            response.addProperty("id", JsonNull.INSTANCE.toString());
        } catch (IllegalArgumentException e) {
            response.add("error", createError(-32602, "Invalid params: " + e.getMessage()));
        } catch (NoSuchMethodException e) {
            response.add("error", createError(-32601, "Method not found: " + e.getMessage()));
        } catch (Exception e) {
            response.add("error", createError(-32603, "Internal error: " + e.getMessage()));
        }

        return gson.toJson(response);
    }

    private JsonElement dispatchMethod(String method, JsonElement params) throws Exception {
        JsonObject paramsObj = params != null && params.isJsonObject() ? params.getAsJsonObject() : new JsonObject();

        switch (method) {
            // Connection/Status
            case "ping":
                return new JsonPrimitive("pong");

            case "getVersion":
                return new JsonPrimitive("1.0.0");

            // Window/Tree operations
            case "listWindows":
                return ComponentInspector.getWindows();

            case "getComponentTree":
                if (paramsObj.has("componentId")) {
                    int compId = paramsObj.get("componentId").getAsInt();
                    int maxDepth = paramsObj.has("maxDepth") ? paramsObj.get("maxDepth").getAsInt() : 10;
                    return ComponentInspector.getComponentTree(compId, maxDepth);
                }
                return ComponentInspector.getComponentTree();

            // Element finding
            case "findElement":
                return new JsonPrimitive(ComponentInspector.findComponent(paramsObj));

            case "findElements":
                return ComponentInspector.findAllComponents(paramsObj);

            case "waitForElement":
                return waitForElement(paramsObj);

            // Element properties
            case "getElementProperties":
                int propId = paramsObj.get("componentId").getAsInt();
                return ComponentInspector.getComponentProperties(propId);

            case "getElementBounds":
                return ActionExecutor.getElementBounds(paramsObj.get("componentId").getAsInt());

            case "getElementText":
                return ActionExecutor.getElementText(paramsObj.get("componentId").getAsInt());

            case "getProperty":
                return ComponentInspector.getProperty(
                    paramsObj.get("componentId").getAsInt(),
                    paramsObj.get("property").getAsString()
                );

            // Actions
            case "click":
                ActionExecutor.click(paramsObj.get("componentId").getAsInt());
                return JsonNull.INSTANCE;

            case "doubleClick":
                ActionExecutor.doubleClick(paramsObj.get("componentId").getAsInt());
                return JsonNull.INSTANCE;

            case "rightClick":
                ActionExecutor.rightClick(paramsObj.get("componentId").getAsInt());
                return JsonNull.INSTANCE;

            case "typeText":
                ActionExecutor.typeText(
                    paramsObj.get("componentId").getAsInt(),
                    paramsObj.get("text").getAsString()
                );
                return JsonNull.INSTANCE;

            case "clearText":
                ActionExecutor.clearText(paramsObj.get("componentId").getAsInt());
                return JsonNull.INSTANCE;

            case "selectItem":
                ActionExecutor.selectItem(
                    paramsObj.get("componentId").getAsInt(),
                    paramsObj.has("index") ? paramsObj.get("index").getAsInt() : -1,
                    paramsObj.has("value") ? paramsObj.get("value").getAsString() : null
                );
                return JsonNull.INSTANCE;

            case "selectMenu":
                ActionExecutor.selectMenu(paramsObj.get("path").getAsString());
                return JsonNull.INSTANCE;

            case "focus":
                ActionExecutor.focus(paramsObj.get("componentId").getAsInt());
                return JsonNull.INSTANCE;

            // Table operations
            case "selectTableCell":
                ActionExecutor.selectTableCell(
                    paramsObj.get("componentId").getAsInt(),
                    paramsObj.get("row").getAsInt(),
                    paramsObj.get("column").getAsInt()
                );
                return JsonNull.INSTANCE;

            case "getTableCellValue":
                return ActionExecutor.getTableCellValue(
                    paramsObj.get("componentId").getAsInt(),
                    paramsObj.get("row").getAsInt(),
                    paramsObj.get("column").getAsInt()
                );

            case "setTableCellValue":
                ActionExecutor.setTableCellValue(
                    paramsObj.get("componentId").getAsInt(),
                    paramsObj.get("row").getAsInt(),
                    paramsObj.get("column").getAsInt(),
                    paramsObj.get("value").getAsString()
                );
                return JsonNull.INSTANCE;

            case "getTableRowCount":
                return ActionExecutor.getTableRowCount(paramsObj.get("componentId").getAsInt());

            case "getTableColumnCount":
                return ActionExecutor.getTableColumnCount(paramsObj.get("componentId").getAsInt());

            case "getTableData":
                return ActionExecutor.getTableData(paramsObj.get("componentId").getAsInt());

            // Tree operations
            case "expandTreeNode":
                ActionExecutor.expandTreeNode(
                    paramsObj.get("componentId").getAsInt(),
                    paramsObj.get("path").getAsString()
                );
                return JsonNull.INSTANCE;

            case "collapseTreeNode":
                ActionExecutor.collapseTreeNode(
                    paramsObj.get("componentId").getAsInt(),
                    paramsObj.get("path").getAsString()
                );
                return JsonNull.INSTANCE;

            case "selectTreeNode":
                ActionExecutor.selectTreeNode(
                    paramsObj.get("componentId").getAsInt(),
                    paramsObj.get("path").getAsString()
                );
                return JsonNull.INSTANCE;

            case "getTreeNodes":
                boolean selectedOnly = paramsObj.has("selectedOnly") && paramsObj.get("selectedOnly").getAsBoolean();
                if (selectedOnly) {
                    return ActionExecutor.getSelectedTreePath(paramsObj.get("componentId").getAsInt());
                }
                return ActionExecutor.getTreeNodes(paramsObj.get("componentId").getAsInt());

            // List operations
            case "getListItems":
                return ActionExecutor.getListItems(paramsObj.get("componentId").getAsInt());

            // Wait operations
            case "waitUntilEnabled":
                return waitUntilEnabled(paramsObj);

            case "waitUntilVisible":
                return waitUntilVisible(paramsObj);

            case "waitUntilNotVisible":
                return waitUntilNotVisible(paramsObj);

            // Screenshot
            case "captureScreenshot":
                return ActionExecutor.captureScreenshot(
                    paramsObj.has("componentId") ? paramsObj.get("componentId").getAsInt() : -1
                );

            default:
                throw new NoSuchMethodException(method);
        }
    }

    private JsonElement waitForElement(JsonObject params) {
        long timeout = params.has("timeout") ? params.get("timeout").getAsLong() : 10000;
        long pollInterval = params.has("pollInterval") ? params.get("pollInterval").getAsLong() : 100;

        long startTime = System.currentTimeMillis();
        while (System.currentTimeMillis() - startTime < timeout) {
            int id = ComponentInspector.findComponent(params);
            if (id >= 0) {
                return new JsonPrimitive(id);
            }
            EdtHelper.sleep(pollInterval);
        }

        throw new IllegalStateException("Element not found within timeout");
    }

    private JsonElement waitUntilEnabled(JsonObject params) {
        int componentId = params.get("componentId").getAsInt();
        long timeout = params.has("timeout") ? params.get("timeout").getAsLong() : 10000;

        boolean result = EdtHelper.waitForCondition(() -> {
            java.awt.Component comp = ComponentInspector.getComponentById(componentId);
            return comp != null && comp.isEnabled();
        }, timeout);

        return new JsonPrimitive(result);
    }

    private JsonElement waitUntilVisible(JsonObject params) {
        int componentId = params.get("componentId").getAsInt();
        long timeout = params.has("timeout") ? params.get("timeout").getAsLong() : 10000;

        boolean result = EdtHelper.waitForCondition(() -> {
            java.awt.Component comp = ComponentInspector.getComponentById(componentId);
            return comp != null && comp.isVisible() && comp.isShowing();
        }, timeout);

        return new JsonPrimitive(result);
    }

    private JsonElement waitUntilNotVisible(JsonObject params) {
        int componentId = params.get("componentId").getAsInt();
        long timeout = params.has("timeout") ? params.get("timeout").getAsLong() : 10000;

        boolean result = EdtHelper.waitForCondition(() -> {
            java.awt.Component comp = ComponentInspector.getComponentById(componentId);
            return comp == null || !comp.isVisible() || !comp.isShowing();
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
}
