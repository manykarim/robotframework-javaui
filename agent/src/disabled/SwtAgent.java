package com.robotframework.swt;

import java.lang.instrument.Instrumentation;
import java.util.HashMap;
import java.util.Map;

/**
 * Java Agent entry point for Robot Framework SWT testing.
 * Can be loaded via -javaagent or dynamically attached.
 *
 * This agent provides automation capabilities for Eclipse SWT applications,
 * handling the SWT Display thread model for thread-safe widget operations.
 */
public class SwtAgent {

    private static SwtRpcServer rpcServer;
    private static volatile boolean initialized = false;
    private static final Object lock = new Object();
    private static Instrumentation instrumentation;

    /**
     * Entry point when loaded via -javaagent command line option.
     *
     * @param agentArgs Agent arguments in format "port=PORT,host=HOST"
     * @param inst Instrumentation instance
     */
    public static void premain(String agentArgs, Instrumentation inst) {
        instrumentation = inst;
        initialize(agentArgs);
    }

    /**
     * Entry point when dynamically attached to a running JVM.
     *
     * @param agentArgs Agent arguments in format "port=PORT,host=HOST"
     * @param inst Instrumentation instance
     */
    public static void agentmain(String agentArgs, Instrumentation inst) {
        instrumentation = inst;
        initialize(agentArgs);
    }

    /**
     * Get the Instrumentation instance.
     *
     * @return Instrumentation instance, or null if not available
     */
    public static Instrumentation getInstrumentation() {
        return instrumentation;
    }

    /**
     * Initialize the agent with the given arguments.
     *
     * @param agentArgs Agent arguments string
     */
    private static void initialize(String agentArgs) {
        synchronized (lock) {
            if (initialized) {
                System.out.println("[SwtAgent] Already initialized, skipping");
                return;
            }

            Map<String, String> params = parseArgs(agentArgs);

            String host = params.getOrDefault("host", "127.0.0.1");
            int port = Integer.parseInt(params.getOrDefault("port", "18081"));

            System.out.println("[SwtAgent] Initializing with host=" + host + ", port=" + port);

            try {
                rpcServer = new SwtRpcServer(host, port);
                Thread serverThread = new Thread(rpcServer, "SwtAgent-RpcServer");
                serverThread.setDaemon(true);
                serverThread.start();

                initialized = true;
                System.out.println("[SwtAgent] RPC server started on " + host + ":" + port);
            } catch (Exception e) {
                System.err.println("[SwtAgent] Failed to start RPC server: " + e.getMessage());
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
            System.out.println("[SwtAgent] Shutdown complete");
        }
    }
}
