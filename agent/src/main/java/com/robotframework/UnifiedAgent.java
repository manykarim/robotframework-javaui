package com.robotframework;

import java.lang.instrument.Instrumentation;
import java.util.HashMap;
import java.util.Map;

/**
 * Unified Java Agent entry point for Robot Framework Java GUI testing.
 * Supports Swing, SWT, and RCP applications.
 *
 * This agent automatically detects the GUI toolkit being used and starts
 * the appropriate RPC server.
 */
public class UnifiedAgent {

    private static volatile boolean initialized = false;
    private static final Object lock = new Object();
    private static Instrumentation instrumentation;
    private static Runnable rpcServer;
    private static String detectedToolkit = "unknown";

    /**
     * Entry point when loaded via -javaagent command line option.
     *
     * @param agentArgs Agent arguments in format "port=PORT,host=HOST,toolkit=swing|swt|auto"
     * @param inst Instrumentation instance
     */
    public static void premain(String agentArgs, Instrumentation inst) {
        instrumentation = inst;
        initialize(agentArgs);
    }

    /**
     * Entry point when dynamically attached to a running JVM.
     *
     * @param agentArgs Agent arguments in format "port=PORT,host=HOST,toolkit=swing|swt|auto"
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
                System.out.println("[UnifiedAgent] Already initialized, skipping");
                return;
            }

            Map<String, String> params = parseArgs(agentArgs);

            String host = params.getOrDefault("host", "127.0.0.1");
            int port = Integer.parseInt(params.getOrDefault("port", "18080"));
            String toolkit = params.getOrDefault("toolkit", "auto").toLowerCase();

            System.out.println("[UnifiedAgent] Initializing with host=" + host + ", port=" + port + ", toolkit=" + toolkit);

            try {
                // Detect or use specified toolkit
                if ("auto".equals(toolkit)) {
                    toolkit = detectToolkit();
                }
                detectedToolkit = toolkit;

                System.out.println("[UnifiedAgent] Using toolkit: " + toolkit);

                // Start appropriate RPC server
                switch (toolkit) {
                    case "swt":
                    case "rcp":
                        startSwtServer(host, port);
                        break;
                    case "swing":
                    default:
                        startSwingServer(host, port);
                        break;
                }

                initialized = true;
            } catch (Exception e) {
                System.err.println("[UnifiedAgent] Failed to start RPC server: " + e.getMessage());
                e.printStackTrace();
            }
        }
    }

    /**
     * Detect which GUI toolkit is being used.
     *
     * @return "swt" if SWT is detected, "swing" otherwise
     */
    private static String detectToolkit() {
        // Check for SWT Display class via loaded classes
        if (instrumentation != null) {
            Class<?>[] loadedClasses = instrumentation.getAllLoadedClasses();
            for (Class<?> clazz : loadedClasses) {
                String name = clazz.getName();
                if (name.startsWith("org.eclipse.swt.")) {
                    System.out.println("[UnifiedAgent] Detected SWT via loaded class: " + name);
                    return "swt";
                }
            }
        }

        // Try to load SWT Display class
        try {
            Class.forName("org.eclipse.swt.widgets.Display");
            System.out.println("[UnifiedAgent] Detected SWT via Class.forName");
            return "swt";
        } catch (ClassNotFoundException e) {
            // SWT not available
        }

        // Check thread names for SWT hints
        ThreadGroup rootGroup = Thread.currentThread().getThreadGroup();
        while (rootGroup.getParent() != null) {
            rootGroup = rootGroup.getParent();
        }
        Thread[] threads = new Thread[rootGroup.activeCount() * 2];
        int count = rootGroup.enumerate(threads, true);
        for (int i = 0; i < count; i++) {
            if (threads[i] != null) {
                String name = threads[i].getName().toLowerCase();
                if (name.contains("swt") || name.contains("eclipse")) {
                    // Try to load SWT from this thread's classloader
                    try {
                        threads[i].getContextClassLoader().loadClass("org.eclipse.swt.widgets.Display");
                        System.out.println("[UnifiedAgent] Detected SWT via thread classloader: " + threads[i].getName());
                        return "swt";
                    } catch (Exception e) {
                        // Continue checking
                    }
                }
            }
        }

        System.out.println("[UnifiedAgent] No SWT detected, defaulting to Swing");
        return "swing";
    }

    /**
     * Start the Swing RPC server.
     */
    private static void startSwingServer(String host, int port) throws Exception {
        System.out.println("[UnifiedAgent] Starting Swing RPC server on " + host + ":" + port);

        // Use reflection to start the Swing server to avoid compile-time dependencies
        Class<?> rpcServerClass = Class.forName("com.robotframework.swing.RpcServer");
        Object server = rpcServerClass.getConstructor(String.class, int.class).newInstance(host, port);

        rpcServer = (Runnable) server;
        Thread serverThread = new Thread(rpcServer, "UnifiedAgent-SwingRpcServer");
        serverThread.setDaemon(true);
        serverThread.start();

        System.out.println("[UnifiedAgent] Swing RPC server started on " + host + ":" + port);
    }

    /**
     * Start the SWT RPC server.
     */
    private static void startSwtServer(String host, int port) throws Exception {
        System.out.println("[UnifiedAgent] Starting SWT RPC server on " + host + ":" + port);

        // Use the reflection-only SWT server that has no static SWT imports
        Class<?> rpcServerClass = Class.forName("com.robotframework.swt.SwtReflectionRpcServer");
        Object server = rpcServerClass.getConstructor(String.class, int.class).newInstance(host, port);

        rpcServer = (Runnable) server;
        Thread serverThread = new Thread(rpcServer, "UnifiedAgent-SwtRpcServer");
        serverThread.setDaemon(true);
        serverThread.start();

        // Wait for server to be ready (max 5 seconds)
        java.lang.reflect.Method isReadyMethod = rpcServerClass.getMethod("isReady");
        long startTime = System.currentTimeMillis();
        while (System.currentTimeMillis() - startTime < 5000) {
            Boolean ready = (Boolean) isReadyMethod.invoke(server);
            if (ready != null && ready) {
                System.out.println("[UnifiedAgent] SWT RPC server ready on " + host + ":" + port);
                return;
            }
            Thread.sleep(50);
        }

        System.err.println("[UnifiedAgent] WARNING: SWT RPC server may not be ready yet");
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
     * Get the detected toolkit.
     *
     * @return "swing", "swt", or "unknown"
     */
    public static String getDetectedToolkit() {
        return detectedToolkit;
    }

    /**
     * Stop the RPC server and cleanup resources.
     */
    public static void shutdown() {
        synchronized (lock) {
            if (rpcServer != null) {
                // Try to stop the server via reflection
                try {
                    rpcServer.getClass().getMethod("stop").invoke(rpcServer);
                } catch (Exception e) {
                    // Ignore
                }
                rpcServer = null;
            }
            initialized = false;
            System.out.println("[UnifiedAgent] Shutdown complete");
        }
    }
}
