package com.robotframework.swt;

import com.google.gson.*;

import java.lang.instrument.Instrumentation;
import java.lang.reflect.Method;
import java.lang.reflect.Field;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicReference;

/**
 * Pure reflection-based bridge to SWT classes.
 * This class uses NO static SWT imports, allowing it to work
 * regardless of classloader isolation in OSGi environments.
 */
public class SwtReflectionBridge {

    private static volatile Object displayInstance;
    private static volatile ClassLoader swtClassLoader;
    private static volatile Class<?> displayClass;
    private static volatile Class<?> shellClass;
    private static volatile Class<?> widgetClass;
    private static volatile Class<?> controlClass;
    private static volatile Class<?> compositeClass;

    private static final Map<Integer, Object> widgetCache = new ConcurrentHashMap<>();
    private static int widgetIdCounter = 1;

    /**
     * Get the SWT classloader for loading SWT classes.
     * @return The classloader used by SWT classes, or null if not initialized.
     */
    public static ClassLoader getSwtClassLoader() {
        return swtClassLoader;
    }

    /**
     * Initialize the bridge by finding the SWT Display instance.
     * Must be called after the SWT application has started.
     */
    public static boolean initialize() {
        System.err.println("[SwtBridge] Initializing...");
        System.err.flush();

        if (displayInstance != null) {
            System.err.println("[SwtBridge] Already initialized");
            return true;
        }

        // First, try to find Display via Instrumentation (most reliable)
        Object display = findDisplayViaInstrumentation();
        if (display != null) {
            return initializeWithDisplay(display);
        }

        // Fall back to searching thread classloaders
        try {
            ThreadGroup rootGroup = Thread.currentThread().getThreadGroup();
            while (rootGroup.getParent() != null) {
                rootGroup = rootGroup.getParent();
            }
            Thread[] threads = new Thread[rootGroup.activeCount() * 2];
            int count = rootGroup.enumerate(threads, true);

            System.err.println("[SwtBridge] Searching " + count + " threads for Display...");
            System.err.flush();

            // First pass: look for threads with SWT-related names
            for (int i = 0; i < count; i++) {
                Thread t = threads[i];
                if (t == null) continue;

                String name = t.getName().toLowerCase();
                if (name.contains("main") || name.contains("swt") || name.contains("display") || name.contains("ui")) {
                    System.err.println("[SwtBridge] Checking priority thread: " + t.getName());
                    System.err.flush();

                    display = findDisplayOnThread(t);
                    if (display != null) {
                        return initializeWithDisplay(display);
                    }
                }
            }

            // Second pass: check all other threads
            for (int i = 0; i < count; i++) {
                Thread t = threads[i];
                if (t == null) continue;

                display = findDisplayOnThread(t);
                if (display != null) {
                    return initializeWithDisplay(display);
                }
            }

            System.err.println("[SwtBridge] No Display found on any thread");
            System.err.flush();
            return false;

        } catch (Exception e) {
            System.err.println("[SwtBridge] Initialization error: " + e.getMessage());
            e.printStackTrace();
            return false;
        }
    }

    private static boolean initializeWithDisplay(Object display) {
        try {
            displayInstance = display;
            displayClass = display.getClass();
            swtClassLoader = displayClass.getClassLoader();

            // If swtClassLoader is null, it means the class was loaded by bootstrap classloader
            if (swtClassLoader == null) {
                // Use the classloader that loaded SwtReflectionBridge as fallback
                swtClassLoader = SwtReflectionBridge.class.getClassLoader();
            }

            // Load other SWT classes using the same classloader
            ClassLoader cl = displayClass.getClassLoader();
            if (cl == null) {
                // Bootstrap classloader - use Class.forName
                shellClass = Class.forName("org.eclipse.swt.widgets.Shell");
                widgetClass = Class.forName("org.eclipse.swt.widgets.Widget");
                controlClass = Class.forName("org.eclipse.swt.widgets.Control");
                compositeClass = Class.forName("org.eclipse.swt.widgets.Composite");
            } else {
                shellClass = cl.loadClass("org.eclipse.swt.widgets.Shell");
                widgetClass = cl.loadClass("org.eclipse.swt.widgets.Widget");
                controlClass = cl.loadClass("org.eclipse.swt.widgets.Control");
                compositeClass = cl.loadClass("org.eclipse.swt.widgets.Composite");
            }

            System.err.println("[SwtBridge] Initialized with Display: " + displayInstance);
            System.err.println("[SwtBridge] Display class: " + displayClass);
            System.err.println("[SwtBridge] SWT ClassLoader: " + swtClassLoader);
            System.err.flush();
            return true;

        } catch (Exception e) {
            System.err.println("[SwtBridge] Failed to initialize with Display: " + e.getMessage());
            e.printStackTrace();
            return false;
        }
    }

    /**
     * Find Display via Instrumentation.getAllLoadedClasses().
     * This is the most reliable approach as it finds classes regardless of classloader.
     */
    private static Object findDisplayViaInstrumentation() {
        System.err.println("[SwtBridge] Trying to find Display via Instrumentation...");
        System.err.flush();

        try {
            Instrumentation inst = SwtAgent.getInstrumentation();
            if (inst == null) {
                System.err.println("[SwtBridge] Instrumentation not available");
                System.err.flush();
                return null;
            }

            Class<?>[] allClasses = inst.getAllLoadedClasses();
            System.err.println("[SwtBridge] Searching " + allClasses.length + " loaded classes...");
            System.err.flush();

            Class<?> displayClass = null;

            // Find the Display class
            for (Class<?> clazz : allClasses) {
                if ("org.eclipse.swt.widgets.Display".equals(clazz.getName())) {
                    displayClass = clazz;
                    System.err.println("[SwtBridge] Found Display class: " + clazz);
                    System.err.println("[SwtBridge] Display classloader: " + clazz.getClassLoader());
                    System.err.flush();
                    break;
                }
            }

            if (displayClass == null) {
                System.err.println("[SwtBridge] Display class not loaded yet");
                System.err.flush();
                return null;
            }

            // Call Display.getDefault() to get the singleton
            try {
                Method getDefault = displayClass.getMethod("getDefault");
                Object display = getDefault.invoke(null);

                if (display != null) {
                    Method isDisposed = displayClass.getMethod("isDisposed");
                    if (!(Boolean) isDisposed.invoke(display)) {
                        System.err.println("[SwtBridge] Found Display via Instrumentation: " + display);
                        System.err.flush();
                        return display;
                    } else {
                        System.err.println("[SwtBridge] Display is disposed");
                    }
                } else {
                    System.err.println("[SwtBridge] Display.getDefault() returned null");
                }
            } catch (Exception e) {
                System.err.println("[SwtBridge] Error calling Display.getDefault(): " + e.getMessage());
                e.printStackTrace();
            }

            // Try Display.findDisplay(Thread) for main thread
            try {
                Method findDisplay = displayClass.getMethod("findDisplay", Thread.class);

                // Get all threads and try each
                ThreadGroup rootGroup = Thread.currentThread().getThreadGroup();
                while (rootGroup.getParent() != null) {
                    rootGroup = rootGroup.getParent();
                }
                Thread[] threads = new Thread[rootGroup.activeCount() * 2];
                int count = rootGroup.enumerate(threads, true);

                for (int i = 0; i < count; i++) {
                    Thread t = threads[i];
                    if (t == null) continue;

                    try {
                        Object display = findDisplay.invoke(null, t);
                        if (display != null) {
                            Method isDisposed = displayClass.getMethod("isDisposed");
                            if (!(Boolean) isDisposed.invoke(display)) {
                                System.err.println("[SwtBridge] Found Display on thread " + t.getName() + " via Instrumentation");
                                System.err.flush();
                                return display;
                            }
                        }
                    } catch (Exception e) {
                        // Continue
                    }
                }
            } catch (Exception e) {
                System.err.println("[SwtBridge] Error with findDisplay: " + e.getMessage());
            }

        } catch (Exception e) {
            System.err.println("[SwtBridge] Error in findDisplayViaInstrumentation: " + e.getMessage());
            e.printStackTrace();
            System.err.flush();
        }

        return null;
    }

    /**
     * Find Display instance associated with a thread using reflection.
     */
    private static Object findDisplayOnThread(Thread thread) {
        // Collect classloaders from various sources
        java.util.Set<ClassLoader> classLoaders = new java.util.LinkedHashSet<>();

        // Add thread's context classloader
        ClassLoader contextCL = thread.getContextClassLoader();
        if (contextCL != null) {
            classLoaders.add(contextCL);
            System.err.println("[SwtBridge] Thread " + thread.getName() + " contextCL: " + contextCL.getClass().getName());
            System.err.flush();
        }

        // Try to get classloader from classes in thread's stack trace
        try {
            StackTraceElement[] stack = thread.getStackTrace();
            for (StackTraceElement elem : stack) {
                String className = elem.getClassName();
                // Look for Eclipse/SWT related classes
                if (className.contains("eclipse") || className.contains("swt") ||
                    className.contains("dbeaver") || className.contains("jface")) {

                    System.err.println("[SwtBridge] Found OSGi class in stack: " + className);
                    System.err.flush();

                    // Try to load this class using various classloaders
                    try {
                        // First try: Use Class.forName with false (don't initialize) and current thread's CL
                        Class<?> c = Class.forName(className, false, contextCL);
                        ClassLoader bundleCL = c.getClassLoader();
                        if (bundleCL != null) {
                            System.err.println("[SwtBridge] Got bundle classloader: " + bundleCL.getClass().getName());
                            System.err.flush();
                            classLoaders.add(bundleCL);
                        }
                    } catch (Exception e) {
                        // Try alternative: use the class's own forName
                        try {
                            Class<?> c = Class.forName(className);
                            ClassLoader bundleCL = c.getClassLoader();
                            if (bundleCL != null) {
                                System.err.println("[SwtBridge] Got bundle classloader via forName: " + bundleCL.getClass().getName());
                                System.err.flush();
                                classLoaders.add(bundleCL);
                            }
                        } catch (Exception ignored2) {}
                    }
                }
            }
        } catch (Exception e) {
            // Ignore stack trace errors
        }

        System.err.println("[SwtBridge] Total classloaders to try: " + classLoaders.size());
        System.err.flush();

        // Try each classloader
        for (ClassLoader cl : classLoaders) {
            System.err.println("[SwtBridge] Trying classloader: " + cl.getClass().getName());
            System.err.flush();
            Object display = tryFindDisplayWithClassLoader(cl, thread);
            if (display != null) {
                return display;
            }
        }

        return null;
    }

    private static Object tryFindDisplayWithClassLoader(ClassLoader cl, Thread thread) {
        try {
            // Try to load Display class
            System.err.println("[SwtBridge] Attempting to load Display with: " + cl.getClass().getName());
            System.err.flush();
            Class<?> dispClass = cl.loadClass("org.eclipse.swt.widgets.Display");

            // Call Display.findDisplay(Thread) for the target thread
            Method findDisplay = dispClass.getMethod("findDisplay", Thread.class);
            Object display = findDisplay.invoke(null, thread);

            if (display != null) {
                // Check if not disposed
                Method isDisposed = dispClass.getMethod("isDisposed");
                Boolean disposed = (Boolean) isDisposed.invoke(display);
                if (!disposed) {
                    System.err.println("[SwtBridge] Found Display via classloader: " + cl);
                    System.err.flush();
                    return display;
                }
            }

            // Also try Display.getDefault() which returns the singleton
            try {
                Method getDefault = dispClass.getMethod("getDefault");
                display = getDefault.invoke(null);
                if (display != null) {
                    Method isDisposed = dispClass.getMethod("isDisposed");
                    if (!(Boolean) isDisposed.invoke(display)) {
                        System.err.println("[SwtBridge] Found Display via getDefault() with classloader: " + cl);
                        System.err.flush();
                        return display;
                    }
                }
            } catch (Exception ignored) {}

        } catch (ClassNotFoundException e) {
            System.err.println("[SwtBridge] ClassNotFoundException: " + e.getMessage());
            System.err.flush();
        } catch (Exception e) {
            System.err.println("[SwtBridge] Error: " + e.getClass().getSimpleName() + ": " + e.getMessage());
            System.err.flush();
        }
        return null;
    }

    /**
     * Try to find Display using the bootclasspath (if SWT is there).
     */
    private static Object findDisplayFromBootClasspath() {
        System.err.println("[SwtBridge] Trying to find Display via bootclasspath...");
        System.err.flush();

        try {
            // Try Class.forName which uses the caller's classloader chain including boot
            System.err.println("[SwtBridge] Calling Class.forName for Display...");
            System.err.flush();

            Class<?> dispClass = Class.forName("org.eclipse.swt.widgets.Display");
            System.err.println("[SwtBridge] Found Display class via Class.forName: " + dispClass);
            System.err.println("[SwtBridge] Display classloader: " + dispClass.getClassLoader());
            System.err.flush();

            // Try getDefault() first
            try {
                Method getDefault = dispClass.getMethod("getDefault");
                Object display = getDefault.invoke(null);
                if (display != null) {
                    Method isDisposed = dispClass.getMethod("isDisposed");
                    if (!(Boolean) isDisposed.invoke(display)) {
                        System.err.println("[SwtBridge] Found Display via getDefault()");
                        return display;
                    }
                }
            } catch (Exception e) {
                System.err.println("[SwtBridge] getDefault() failed: " + e.getMessage());
            }

            // Try getCurrent()
            try {
                Method getCurrent = dispClass.getMethod("getCurrent");
                Object display = getCurrent.invoke(null);
                if (display != null) {
                    Method isDisposed = dispClass.getMethod("isDisposed");
                    if (!(Boolean) isDisposed.invoke(display)) {
                        System.err.println("[SwtBridge] Found Display via getCurrent()");
                        return display;
                    }
                }
            } catch (Exception e) {
                System.err.println("[SwtBridge] getCurrent() failed: " + e.getMessage());
            }

            // Search all threads
            ThreadGroup rootGroup = Thread.currentThread().getThreadGroup();
            while (rootGroup.getParent() != null) {
                rootGroup = rootGroup.getParent();
            }
            Thread[] threads = new Thread[rootGroup.activeCount() * 2];
            int count = rootGroup.enumerate(threads, true);

            Method findDisplay = dispClass.getMethod("findDisplay", Thread.class);
            for (int i = 0; i < count; i++) {
                Thread t = threads[i];
                if (t == null) continue;

                try {
                    Object display = findDisplay.invoke(null, t);
                    if (display != null) {
                        Method isDisposed = dispClass.getMethod("isDisposed");
                        if (!(Boolean) isDisposed.invoke(display)) {
                            System.err.println("[SwtBridge] Found Display on thread: " + t.getName());
                            return display;
                        }
                    }
                } catch (Exception e) {
                    // Continue
                }
            }

        } catch (ClassNotFoundException e) {
            System.err.println("[SwtBridge] Display class not found via Class.forName");
        } catch (Exception e) {
            System.err.println("[SwtBridge] Error searching bootclasspath: " + e.getMessage());
        }
        return null;
    }

    /**
     * Get the Display instance.
     */
    public static Object getDisplay() {
        if (displayInstance == null) {
            initialize();
        }
        return displayInstance;
    }

    /**
     * Execute a runnable on the SWT UI thread and wait for completion.
     */
    public static <T> T syncExec(java.util.concurrent.Callable<T> callable) throws Exception {
        Object display = getDisplay();
        if (display == null) {
            throw new IllegalStateException("Display not available");
        }

        // Check if we're on the UI thread
        Method getThread = displayClass.getMethod("getThread");
        Thread uiThread = (Thread) getThread.invoke(display);

        if (Thread.currentThread() == uiThread) {
            return callable.call();
        }

        // Execute via asyncExec and wait
        AtomicReference<T> result = new AtomicReference<>();
        AtomicReference<Exception> exception = new AtomicReference<>();
        final Object lock = new Object();
        final boolean[] completed = {false};

        Runnable runnable = () -> {
            try {
                result.set(callable.call());
            } catch (Exception e) {
                exception.set(e);
            }
            synchronized (lock) {
                completed[0] = true;
                lock.notifyAll();
            }
        };

        Method asyncExec = displayClass.getMethod("asyncExec", Runnable.class);
        asyncExec.invoke(display, runnable);

        synchronized (lock) {
            long startTime = System.currentTimeMillis();
            long timeout = 10000;
            while (!completed[0]) {
                long elapsed = System.currentTimeMillis() - startTime;
                if (elapsed >= timeout) {
                    throw new RuntimeException("Display operation timed out");
                }
                lock.wait(timeout - elapsed);
            }
        }

        if (exception.get() != null) {
            throw exception.get();
        }

        return result.get();
    }

    /**
     * Execute a runnable asynchronously on the UI thread (fire and forget).
     */
    private static void asyncExec(Runnable runnable) throws Exception {
        Object display = getDisplay();
        if (display == null) {
            throw new IllegalStateException("Display not available");
        }

        Method asyncExecMethod = displayClass.getMethod("asyncExec", Runnable.class);
        asyncExecMethod.invoke(display, runnable);
    }

    /**
     * Get all shells from the Display.
     */
    public static JsonArray getShells() throws Exception {
        return syncExec(() -> {
            JsonArray shells = new JsonArray();

            Method getShells = displayClass.getMethod("getShells");
            Object[] shellArray = (Object[]) getShells.invoke(displayInstance);

            for (Object shell : shellArray) {
                if (shell == null) continue;

                Method isDisposed = shellClass.getMethod("isDisposed");
                if ((Boolean) isDisposed.invoke(shell)) continue;

                JsonObject shellInfo = new JsonObject();
                int id = getOrCreateWidgetId(shell);
                shellInfo.addProperty("id", id);
                shellInfo.addProperty("widgetId", id);
                shellInfo.addProperty("className", shell.getClass().getName());

                // Get text/title
                try {
                    Method getText = shellClass.getMethod("getText");
                    String text = (String) getText.invoke(shell);
                    shellInfo.addProperty("text", text);
                    shellInfo.addProperty("title", text);
                } catch (Exception e) {
                    shellInfo.addProperty("text", "");
                    shellInfo.addProperty("title", "");
                }

                // Get visibility
                try {
                    Method isVisible = controlClass.getMethod("isVisible");
                    shellInfo.addProperty("visible", (Boolean) isVisible.invoke(shell));
                } catch (Exception e) {
                    shellInfo.addProperty("visible", true);
                }

                // Get bounds
                try {
                    Method getBounds = controlClass.getMethod("getBounds");
                    Object bounds = getBounds.invoke(shell);
                    JsonObject boundsObj = new JsonObject();
                    Class<?> rectClass = bounds.getClass();
                    boundsObj.addProperty("x", rectClass.getField("x").getInt(bounds));
                    boundsObj.addProperty("y", rectClass.getField("y").getInt(bounds));
                    boundsObj.addProperty("width", rectClass.getField("width").getInt(bounds));
                    boundsObj.addProperty("height", rectClass.getField("height").getInt(bounds));
                    shellInfo.add("bounds", boundsObj);
                } catch (Exception e) {
                    // Ignore bounds errors
                }

                shells.add(shellInfo);
            }

            return shells;
        });
    }

    /**
     * Get or create a widget ID for tracking.
     * Registers widget with WidgetInspector's cache using reflection to avoid classloader issues.
     */
    private static int getOrCreateWidgetId(Object widget) {
        // Try to use WidgetInspector's cache via reflection
        // This ensures widgets are findable by SwtActionExecutor
        try {
            // Load WidgetInspector using SWT classloader to avoid classloader issues
            ClassLoader cl = swtClassLoader != null ? swtClassLoader : Thread.currentThread().getContextClassLoader();
            if (cl == null) {
                cl = SwtReflectionBridge.class.getClassLoader();
            }
            Class<?> widgetInspectorClass = cl.loadClass("com.robotframework.swt.WidgetInspector");
            Method getOrCreateIdMethod = widgetInspectorClass.getMethod("getOrCreateId", widgetClass);
            Integer id = (Integer) getOrCreateIdMethod.invoke(null, widget);
            // Also store in local cache for quick lookup
            widgetCache.put(id, widget);
            return id;
        } catch (Exception e) {
            System.err.println("[SwtBridge] Failed to register with WidgetInspector: " + e.getMessage());
            e.printStackTrace(System.err);
            // Fallback to local cache only
        }

        // Fallback: Check if widget is already in local cache
        for (Map.Entry<Integer, Object> entry : widgetCache.entrySet()) {
            if (entry.getValue() == widget) {
                return entry.getKey();
            }
        }

        // Assign new ID to local cache
        int id = widgetIdCounter++;
        widgetCache.put(id, widget);

        return id;
    }

    /**
     * Get widget by ID from local cache.
     * Returns the widget object or null if not found/disposed.
     */
    public static Object getWidgetById(int id) {
        // Use local cache only - WidgetInspector has classloader issues in OSGi
        Object localWidget = widgetCache.get(id);
        if (localWidget != null) {
            try {
                Method isDisposed = widgetClass.getMethod("isDisposed");
                if ((Boolean) isDisposed.invoke(localWidget)) {
                    widgetCache.remove(id);
                    return null;
                }
            } catch (Exception e) {
                widgetCache.remove(id);
                return null;
            }
        }
        return localWidget;
    }

    /**
     * Find widgets matching criteria.
     * Supports both direct field format (text, className, type) and
     * locatorType/value format from Rust client.
     */
    public static JsonArray findWidgets(JsonObject criteria) throws Exception {
        return syncExec(() -> {
            JsonArray results = new JsonArray();

            String text = null;
            String className = null;
            String type = null;
            String name = null;

            // Support locatorType/value format from Rust client
            if (criteria.has("locatorType") && criteria.has("value")) {
                String locatorType = criteria.get("locatorType").getAsString();
                String value = criteria.get("value").getAsString();

                switch (locatorType) {
                    case "text":
                        text = value;
                        break;
                    case "class":
                        className = value;
                        break;
                    case "type":
                        type = value;
                        break;
                    case "name":
                        name = value;
                        break;
                    case "id":
                        // For ID, we search by widget ID directly
                        try {
                            int widgetId = Integer.parseInt(value);
                            Object widget = getWidgetById(widgetId);
                            if (widget != null) {
                                results.add(getWidgetInfo(widget));
                            }
                        } catch (NumberFormatException e) {
                            // Treat as name
                            name = value;
                        }
                        return results;
                    default:
                        // Unknown locator type, treat as text
                        text = value;
                }
            } else {
                // Direct field format
                text = criteria.has("text") ? criteria.get("text").getAsString() : null;
                className = criteria.has("className") ? criteria.get("className").getAsString() : null;
                type = criteria.has("type") ? criteria.get("type").getAsString() : null;
                name = criteria.has("name") ? criteria.get("name").getAsString() : null;
            }

            // Get all shells and search through their children
            Method getShells = displayClass.getMethod("getShells");
            Object[] shells = (Object[]) getShells.invoke(displayInstance);

            for (Object shell : shells) {
                if (shell == null) continue;
                Method isDisposed = shellClass.getMethod("isDisposed");
                if ((Boolean) isDisposed.invoke(shell)) continue;

                searchWidgetTree(shell, text, className, type, name, results);
            }

            return results;
        });
    }

    private static void searchWidgetTree(Object widget, String text, String className, String type, String name, JsonArray results) {
        try {
            Method isDisposed = widgetClass.getMethod("isDisposed");
            if ((Boolean) isDisposed.invoke(widget)) return;

            boolean matches = true;

            // Check class name
            if (className != null) {
                if (!widget.getClass().getName().contains(className) &&
                    !widget.getClass().getSimpleName().equalsIgnoreCase(className)) {
                    matches = false;
                }
            }

            // Check type
            if (type != null && matches) {
                String widgetType = widget.getClass().getSimpleName().toLowerCase();
                if (!widgetType.contains(type.toLowerCase())) {
                    matches = false;
                }
            }

            // Check text
            if (text != null && matches) {
                String widgetText = getWidgetText(widget);
                if (widgetText == null || !widgetText.contains(text)) {
                    matches = false;
                }
            }

            // Check name (getData("name") or similar)
            if (name != null && matches) {
                String widgetName = getWidgetName(widget);
                if (widgetName == null || !widgetName.equals(name)) {
                    matches = false;
                }
            }

            if (matches) {
                JsonObject info = getWidgetInfo(widget);
                results.add(info);
            }

            // Search children if this is a Composite
            if (compositeClass.isInstance(widget)) {
                Method getChildren = compositeClass.getMethod("getChildren");
                Object[] children = (Object[]) getChildren.invoke(widget);
                for (Object child : children) {
                    searchWidgetTree(child, text, className, type, name, results);
                }
            }

        } catch (Exception e) {
            // Ignore errors and continue
        }
    }

    private static String getWidgetName(Object widget) {
        try {
            // Try getData("name") first (common SWT pattern)
            Method getData = widget.getClass().getMethod("getData", String.class);
            Object nameData = getData.invoke(widget, "name");
            if (nameData != null) {
                return nameData.toString();
            }
        } catch (Exception e) {
            // Fall through
        }
        try {
            // Try getData() with no args as fallback
            Method getData = widget.getClass().getMethod("getData");
            Object data = getData.invoke(widget);
            if (data instanceof String) {
                return (String) data;
            }
        } catch (Exception e) {
            // No name available
        }
        return null;
    }

    private static String getWidgetText(Object widget) {
        try {
            Method getText = widget.getClass().getMethod("getText");
            return (String) getText.invoke(widget);
        } catch (Exception e) {
            return null;
        }
    }

    private static JsonObject getWidgetInfo(Object widget) throws Exception {
        JsonObject info = new JsonObject();

        int id = getOrCreateWidgetId(widget);
        info.addProperty("id", id);
        info.addProperty("widgetId", id);
        info.addProperty("className", widget.getClass().getName());
        info.addProperty("type", widget.getClass().getSimpleName());

        String text = getWidgetText(widget);
        if (text != null) {
            info.addProperty("text", text);
        }

        // Get enabled/visible state
        if (controlClass.isInstance(widget)) {
            try {
                Method isEnabled = controlClass.getMethod("isEnabled");
                info.addProperty("enabled", (Boolean) isEnabled.invoke(widget));
            } catch (Exception e) {}

            try {
                Method isVisible = controlClass.getMethod("isVisible");
                info.addProperty("visible", (Boolean) isVisible.invoke(widget));
            } catch (Exception e) {}
        }

        return info;
    }

    /**
     * Get widget tree starting from a widget or all shells.
     */
    public static JsonArray getWidgetTree() throws Exception {
        return syncExec(() -> {
            JsonArray tree = new JsonArray();

            Method getShells = displayClass.getMethod("getShells");
            Object[] shells = (Object[]) getShells.invoke(displayInstance);

            for (Object shell : shells) {
                if (shell == null) continue;
                Method isDisposed = shellClass.getMethod("isDisposed");
                if ((Boolean) isDisposed.invoke(shell)) continue;

                JsonObject shellNode = buildWidgetTreeNode(shell, 10);
                tree.add(shellNode);
            }

            return tree;
        });
    }

    private static JsonObject buildWidgetTreeNode(Object widget, int maxDepth) throws Exception {
        JsonObject node = getWidgetInfo(widget);

        if (maxDepth > 0 && compositeClass.isInstance(widget)) {
            JsonArray children = new JsonArray();
            Method getChildren = compositeClass.getMethod("getChildren");
            Object[] childWidgets = (Object[]) getChildren.invoke(widget);

            for (Object child : childWidgets) {
                try {
                    Method isDisposed = widgetClass.getMethod("isDisposed");
                    if (!(Boolean) isDisposed.invoke(child)) {
                        children.add(buildWidgetTreeNode(child, maxDepth - 1));
                    }
                } catch (Exception e) {}
            }

            if (children.size() > 0) {
                node.add("children", children);
            }
        }

        return node;
    }

    /**
     * Perform a click on a widget.
     */
    public static void click(int widgetId) throws Exception {
        syncExec(() -> {
            Object widget = getWidgetById(widgetId);
            if (widget == null) {
                throw new IllegalArgumentException("Widget not found: " + widgetId);
            }

            // For buttons, call notifyListeners with Selection event
            try {
                Class<?> swtClass = swtClassLoader.loadClass("org.eclipse.swt.SWT");
                Field selectionField = swtClass.getField("Selection");
                int selection = selectionField.getInt(null);

                Class<?> eventClass = swtClassLoader.loadClass("org.eclipse.swt.widgets.Event");
                Object event = eventClass.getDeclaredConstructor().newInstance();

                Method notifyListeners = widgetClass.getMethod("notifyListeners", int.class, eventClass);
                notifyListeners.invoke(widget, selection, event);
            } catch (Exception e) {
                throw new RuntimeException("Click failed: " + e.getMessage(), e);
            }

            return null;
        });
    }

    /**
     * Set text on a widget.
     */
    public static void setText(int widgetId, String text) throws Exception {
        syncExec(() -> {
            Object widget = getWidgetById(widgetId);
            if (widget == null) {
                throw new IllegalArgumentException("Widget not found: " + widgetId);
            }

            try {
                Method setText = widget.getClass().getMethod("setText", String.class);
                setText.invoke(widget, text);
            } catch (Exception e) {
                throw new RuntimeException("setText failed: " + e.getMessage(), e);
            }

            return null;
        });
    }

    /**
     * Clear text from a widget.
     * Works with Text, Combo, StyledText widgets.
     */
    public static void clearText(int widgetId) throws Exception {
        syncExec(() -> {
            Object widget = getWidgetById(widgetId);
            if (widget == null) {
                throw new IllegalArgumentException("Widget not found: " + widgetId);
            }

            try {
                Method setText = widget.getClass().getMethod("setText", String.class);
                setText.invoke(widget, "");
            } catch (Exception e) {
                throw new RuntimeException("clearText failed: " + e.getMessage(), e);
            }

            return null;
        });
    }

    /**
     * Type text into a widget at cursor position.
     * Works with Text, Combo, StyledText widgets.
     */
    public static void typeText(int widgetId, String text) throws Exception {
        syncExec(() -> {
            Object widget = getWidgetById(widgetId);
            if (widget == null) {
                throw new IllegalArgumentException("Widget not found: " + widgetId);
            }

            try {
                // Get current text
                Method getText = widget.getClass().getMethod("getText");
                String currentText = (String) getText.invoke(widget);

                // Append text (simplified - appends at end)
                Method setText = widget.getClass().getMethod("setText", String.class);
                setText.invoke(widget, currentText + text);
            } catch (Exception e) {
                throw new RuntimeException("typeText failed: " + e.getMessage(), e);
            }

            return null;
        });
    }

    /**
     * Activate (bring to front) a shell.
     */
    public static void activateShell(int widgetId) throws Exception {
        syncExec(() -> {
            Object widget = getWidgetById(widgetId);
            if (widget == null) {
                throw new IllegalArgumentException("Widget not found: " + widgetId);
            }

            // Check if it's a Shell
            if (!shellClass.isInstance(widget)) {
                throw new IllegalArgumentException("Widget is not a Shell: " + widget.getClass().getName());
            }

            try {
                // Call forceActive() and setFocus()
                Method forceActive = shellClass.getMethod("forceActive");
                forceActive.invoke(widget);

                Method setFocus = shellClass.getMethod("setFocus");
                setFocus.invoke(widget);
            } catch (Exception e) {
                throw new RuntimeException("activateShell failed: " + e.getMessage(), e);
            }

            return null;
        });
    }

    /**
     * Close a shell.
     */
    public static void closeShell(int widgetId) throws Exception {
        asyncExec(() -> {
            Object widget = getWidgetById(widgetId);
            if (widget == null) {
                return; // Already closed
            }

            // Check if disposed
            try {
                Method isDisposed = widgetClass.getMethod("isDisposed");
                if ((Boolean) isDisposed.invoke(widget)) {
                    return; // Already disposed
                }
            } catch (Exception e) {
                return;
            }

            // Check if it's a Shell
            if (!shellClass.isInstance(widget)) {
                return;
            }

            try {
                Method close = shellClass.getMethod("close");
                close.invoke(widget);
            } catch (Exception e) {
                System.err.println("[SwtBridge] closeShell failed: " + e.getMessage());
            }
        });
    }

    /**
     * Double-click a widget.
     */
    public static void doubleClick(int widgetId) throws Exception {
        asyncExec(() -> {
            Object widget = getWidgetById(widgetId);
            if (widget == null) {
                return;
            }

            try {
                Class<?> swtClass = swtClassLoader.loadClass("org.eclipse.swt.SWT");
                int mouseDoubleClick = (Integer) swtClass.getField("MouseDoubleClick").get(null);

                Class<?> eventClass = swtClassLoader.loadClass("org.eclipse.swt.widgets.Event");
                Object event = eventClass.getDeclaredConstructor().newInstance();
                eventClass.getField("type").set(event, mouseDoubleClick);
                eventClass.getField("widget").set(event, widget);
                eventClass.getField("button").set(event, 1);

                Method notifyListeners = widgetClass.getMethod("notifyListeners", int.class, eventClass);
                notifyListeners.invoke(widget, mouseDoubleClick, event);
            } catch (Exception e) {
                System.err.println("[SwtBridge] doubleClick failed: " + e.getMessage());
            }
        });
    }

    /**
     * Expand a tree item.
     */
    public static void expandTreeItem(int widgetId, String path) throws Exception {
        syncExec(() -> {
            Object widget = getWidgetById(widgetId);
            if (widget == null) {
                throw new IllegalArgumentException("Widget not found: " + widgetId);
            }

            try {
                // Check if it's a Tree
                Class<?> treeClass = swtClassLoader.loadClass("org.eclipse.swt.widgets.Tree");
                if (!treeClass.isInstance(widget)) {
                    throw new IllegalArgumentException("Widget is not a Tree");
                }

                // Get items and try to expand
                Method getItems = treeClass.getMethod("getItems");
                Object[] items = (Object[]) getItems.invoke(widget);

                if (items.length > 0) {
                    Class<?> treeItemClass = swtClassLoader.loadClass("org.eclipse.swt.widgets.TreeItem");
                    Object firstItem = items[0];
                    Method setExpanded = treeItemClass.getMethod("setExpanded", boolean.class);
                    setExpanded.invoke(firstItem, true);
                }
            } catch (Exception e) {
                throw new RuntimeException("expandTreeItem failed: " + e.getMessage(), e);
            }

            return null;
        });
    }

    /**
     * Collapse a tree item.
     */
    public static void collapseTreeItem(int widgetId, String path) throws Exception {
        syncExec(() -> {
            Object widget = getWidgetById(widgetId);
            if (widget == null) {
                throw new IllegalArgumentException("Widget not found: " + widgetId);
            }

            try {
                Class<?> treeClass = swtClassLoader.loadClass("org.eclipse.swt.widgets.Tree");
                if (!treeClass.isInstance(widget)) {
                    throw new IllegalArgumentException("Widget is not a Tree");
                }

                Method getItems = treeClass.getMethod("getItems");
                Object[] items = (Object[]) getItems.invoke(widget);

                if (items.length > 0) {
                    Class<?> treeItemClass = swtClassLoader.loadClass("org.eclipse.swt.widgets.TreeItem");
                    Object firstItem = items[0];
                    Method setExpanded = treeItemClass.getMethod("setExpanded", boolean.class);
                    setExpanded.invoke(firstItem, false);
                }
            } catch (Exception e) {
                throw new RuntimeException("collapseTreeItem failed: " + e.getMessage(), e);
            }

            return null;
        });
    }

    /**
     * Clear the widget cache.
     */
    public static void clearCache() {
        widgetCache.clear();
        widgetIdCounter = 1;
    }

    /**
     * Check if bridge is initialized.
     */
    public static boolean isInitialized() {
        return displayInstance != null;
    }
}
