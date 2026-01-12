package com.robotframework.swing;

import java.lang.instrument.Instrumentation;
import java.util.HashMap;
import java.util.Map;

/**
 * Java Agent entry point for Robot Framework Swing testing.
 * Can be loaded via -javaagent or dynamically attached.
 */
public class Agent {

    private static RpcServer rpcServer;
    private static volatile boolean initialized = false;
    private static final Object lock = new Object();

    /**
     * Entry point when loaded via -javaagent command line option.
     *
     * @param agentArgs Agent arguments in format "port=PORT,host=HOST"
     * @param inst Instrumentation instance
     */
    public static void premain(String agentArgs, Instrumentation inst) {
        initialize(agentArgs);
    }

    /**
     * Entry point when dynamically attached to a running JVM.
     *
     * @param agentArgs Agent arguments in format "port=PORT,host=HOST"
     * @param inst Instrumentation instance
     */
    public static void agentmain(String agentArgs, Instrumentation inst) {
        initialize(agentArgs);
    }

    /**
     * Initialize the agent with the given arguments.
     *
     * @param agentArgs Agent arguments string
     */
    private static void initialize(String agentArgs) {
        synchronized (lock) {
            if (initialized) {
                System.out.println("[SwingAgent] Already initialized, skipping");
                return;
            }

            Map<String, String> params = parseArgs(agentArgs);

            String host = params.getOrDefault("host", "127.0.0.1");
            int port = Integer.parseInt(params.getOrDefault("port", "18080"));

            System.out.println("[SwingAgent] Initializing with host=" + host + ", port=" + port);

            try {
                rpcServer = new RpcServer(host, port);
                Thread serverThread = new Thread(rpcServer, "SwingAgent-RpcServer");
                serverThread.setDaemon(true);
                serverThread.start();

                initialized = true;
                System.out.println("[SwingAgent] RPC server started on " + host + ":" + port);
            } catch (Exception e) {
                System.err.println("[SwingAgent] Failed to start RPC server: " + e.getMessage());
                e.printStackTrace();
            }
        }
    }

    /**
     * Parse agent arguments string into a map.
     * Format: "key1=value1,key2=value2"
     *
     * @param agentArgs Arguments string
     * @return Map of parameter names to values
     */
    private static Map<String, String> parseArgs(String agentArgs) {
        Map<String, String> params = new HashMap<>();

        if (agentArgs == null || agentArgs.trim().isEmpty()) {
            return params;
        }

        String[] pairs = agentArgs.split(",");
        for (String pair : pairs) {
            String[] keyValue = pair.split("=", 2);
            if (keyValue.length == 2) {
                params.put(keyValue[0].trim(), keyValue[1].trim());
            }
        }

        return params;
    }

    /**
     * Check if the agent is initialized.
     *
     * @return true if initialized
     */
    public static boolean isInitialized() {
        return initialized;
    }

    /**
     * Stop the RPC server and cleanup resources.
     */
    public static void shutdown() {
        synchronized (lock) {
            if (rpcServer != null) {
                rpcServer.stop();
                rpcServer = null;
            }
            initialized = false;
            System.out.println("[SwingAgent] Shutdown complete");
        }
    }
}
