package com.robotframework.swt;

import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Helper class that accesses Eclipse Workbench APIs via reflection.
 * This enables the RCP agent to work with REAL Eclipse RCP applications
 * (like DBeaver, Eclipse IDE, etc.) without requiring compile-time Eclipse
 * dependencies.
 *
 * DUAL-MODE OPERATION:
 * The SwtRpcServer uses a two-tier approach for RCP operations:
 * 1. First tries MockRcpApplication (for unit tests with mock app)
 * 2. Falls back to EclipseWorkbenchHelper (for real Eclipse RCP apps)
 *
 * This allows the same agent JAR to work with both:
 * - The mock RCP test application (for automated testing)
 * - Real Eclipse RCP applications (for production automation)
 *
 * When Eclipse APIs are not available (e.g., plain SWT apps), methods
 * return null/empty results rather than throwing exceptions.
 */
public class EclipseWorkbenchHelper {

    private static boolean eclipseChecked = false;
    private static boolean eclipseAvailable = false;
    private static Class<?> platformUIClass;
    private static Class<?> workbenchClass;
    private static Class<?> workbenchWindowClass;
    private static Class<?> workbenchPageClass;
    private static Class<?> perspectiveDescriptorClass;
    private static Class<?> viewReferenceClass;
    private static Class<?> editorReferenceClass;
    private static Class<?> editorInputClass;
    private static Class<?> commandServiceClass;
    private static Class<?> handlerServiceClass;

    /**
     * Check if Eclipse Workbench APIs are available at runtime.
     */
    public static boolean isEclipseAvailable() {
        if (!eclipseChecked) {
            eclipseChecked = true;
            try {
                platformUIClass = Class.forName("org.eclipse.ui.PlatformUI");
                workbenchClass = Class.forName("org.eclipse.ui.IWorkbench");
                workbenchWindowClass = Class.forName("org.eclipse.ui.IWorkbenchWindow");
                workbenchPageClass = Class.forName("org.eclipse.ui.IWorkbenchPage");
                perspectiveDescriptorClass = Class.forName("org.eclipse.ui.IPerspectiveDescriptor");
                viewReferenceClass = Class.forName("org.eclipse.ui.IViewReference");
                editorReferenceClass = Class.forName("org.eclipse.ui.IEditorReference");
                editorInputClass = Class.forName("org.eclipse.ui.IEditorInput");

                // Try to get the workbench to confirm it's running
                Method getWorkbench = platformUIClass.getMethod("getWorkbench");
                Object workbench = getWorkbench.invoke(null);
                eclipseAvailable = (workbench != null);

                if (eclipseAvailable) {
                    System.out.println("[EclipseWorkbenchHelper] Eclipse Workbench APIs detected and available");
                }
            } catch (ClassNotFoundException e) {
                // Eclipse not on classpath - normal for plain SWT apps
                eclipseAvailable = false;
            } catch (Exception e) {
                System.err.println("[EclipseWorkbenchHelper] Eclipse APIs found but workbench not running: " + e.getMessage());
                eclipseAvailable = false;
            }
        }
        return eclipseAvailable;
    }

    /**
     * Get the Eclipse workbench instance.
     */
    public static Object getWorkbench() {
        if (!isEclipseAvailable()) return null;
        try {
            Method getWorkbench = platformUIClass.getMethod("getWorkbench");
            return getWorkbench.invoke(null);
        } catch (Exception e) {
            return null;
        }
    }

    /**
     * Get the active workbench window.
     */
    public static Object getActiveWindow() {
        Object workbench = getWorkbench();
        if (workbench == null) return null;
        try {
            Method getActiveWindow = workbenchClass.getMethod("getActiveWorkbenchWindow");
            return getActiveWindow.invoke(workbench);
        } catch (Exception e) {
            return null;
        }
    }

    /**
     * Get the active workbench page.
     */
    public static Object getActivePage() {
        Object window = getActiveWindow();
        if (window == null) return null;
        try {
            Method getActivePage = workbenchWindowClass.getMethod("getActivePage");
            return getActivePage.invoke(window);
        } catch (Exception e) {
            return null;
        }
    }

    /**
     * Get the active perspective descriptor.
     */
    public static Object getActivePerspective() {
        Object page = getActivePage();
        if (page == null) return null;
        try {
            Method getPerspective = workbenchPageClass.getMethod("getPerspective");
            return getPerspective.invoke(page);
        } catch (Exception e) {
            return null;
        }
    }

    /**
     * Get the ID of the active perspective.
     */
    public static String getActivePerspectiveId() {
        Object perspective = getActivePerspective();
        if (perspective == null) return null;
        try {
            Method getId = perspectiveDescriptorClass.getMethod("getId");
            return (String) getId.invoke(perspective);
        } catch (Exception e) {
            return null;
        }
    }

    /**
     * Get the label of the active perspective.
     */
    public static String getActivePerspectiveLabel() {
        Object perspective = getActivePerspective();
        if (perspective == null) return null;
        try {
            Method getLabel = perspectiveDescriptorClass.getMethod("getLabel");
            return (String) getLabel.invoke(perspective);
        } catch (Exception e) {
            return null;
        }
    }

    /**
     * Get all available perspectives.
     */
    public static List<Map<String, String>> getAvailablePerspectives() {
        List<Map<String, String>> result = new ArrayList<>();
        Object workbench = getWorkbench();
        if (workbench == null) return result;

        try {
            Method getPerspectiveRegistry = workbenchClass.getMethod("getPerspectiveRegistry");
            Object registry = getPerspectiveRegistry.invoke(workbench);
            if (registry == null) return result;

            Class<?> registryClass = Class.forName("org.eclipse.ui.IPerspectiveRegistry");
            Method getPerspectives = registryClass.getMethod("getPerspectives");
            Object[] perspectives = (Object[]) getPerspectives.invoke(registry);

            if (perspectives != null) {
                Method getId = perspectiveDescriptorClass.getMethod("getId");
                Method getLabel = perspectiveDescriptorClass.getMethod("getLabel");

                for (Object persp : perspectives) {
                    Map<String, String> info = new HashMap<>();
                    info.put("id", (String) getId.invoke(persp));
                    info.put("label", (String) getLabel.invoke(persp));
                    result.add(info);
                }
            }
        } catch (Exception e) {
            System.err.println("[EclipseWorkbenchHelper] Error getting perspectives: " + e.getMessage());
        }
        return result;
    }

    /**
     * Open a perspective by ID.
     */
    public static boolean openPerspective(String perspectiveId) {
        Object workbench = getWorkbench();
        Object window = getActiveWindow();
        if (workbench == null || window == null) return false;

        try {
            // Get perspective registry
            Method getPerspectiveRegistry = workbenchClass.getMethod("getPerspectiveRegistry");
            Object registry = getPerspectiveRegistry.invoke(workbench);

            Class<?> registryClass = Class.forName("org.eclipse.ui.IPerspectiveRegistry");
            Method findPerspective = registryClass.getMethod("findPerspectiveWithId", String.class);
            Object perspective = findPerspective.invoke(registry, perspectiveId);

            if (perspective == null) return false;

            // Show perspective
            Method showPerspective = workbenchClass.getMethod("showPerspective", String.class, workbenchWindowClass);
            showPerspective.invoke(workbench, perspectiveId, window);
            return true;
        } catch (Exception e) {
            System.err.println("[EclipseWorkbenchHelper] Error opening perspective: " + e.getMessage());
            return false;
        }
    }

    /**
     * Get all open views.
     */
    public static List<Map<String, Object>> getOpenViews() {
        List<Map<String, Object>> result = new ArrayList<>();
        Object page = getActivePage();
        if (page == null) return result;

        try {
            Method getViewReferences = workbenchPageClass.getMethod("getViewReferences");
            Object[] viewRefs = (Object[]) getViewReferences.invoke(page);

            if (viewRefs != null) {
                Method getId = viewReferenceClass.getMethod("getId");
                Method getSecondaryId = viewReferenceClass.getMethod("getSecondaryId");
                Method getPartName = viewReferenceClass.getMethod("getPartName");
                Method getTitle = viewReferenceClass.getMethod("getTitle");

                for (Object viewRef : viewRefs) {
                    Map<String, Object> info = new HashMap<>();
                    info.put("id", getId.invoke(viewRef));
                    info.put("secondaryId", getSecondaryId.invoke(viewRef));
                    info.put("name", getPartName.invoke(viewRef));
                    info.put("title", getTitle.invoke(viewRef));
                    result.add(info);
                }
            }
        } catch (Exception e) {
            System.err.println("[EclipseWorkbenchHelper] Error getting views: " + e.getMessage());
        }
        return result;
    }

    /**
     * Show a view by ID.
     */
    public static boolean showView(String viewId, String secondaryId) {
        Object page = getActivePage();
        if (page == null) return false;

        try {
            Method showView;
            if (secondaryId != null && !secondaryId.isEmpty()) {
                showView = workbenchPageClass.getMethod("showView", String.class, String.class, int.class);
                showView.invoke(page, viewId, secondaryId, 1); // IWorkbenchPage.VIEW_ACTIVATE = 1
            } else {
                showView = workbenchPageClass.getMethod("showView", String.class);
                showView.invoke(page, viewId);
            }
            return true;
        } catch (Exception e) {
            System.err.println("[EclipseWorkbenchHelper] Error showing view: " + e.getMessage());
            return false;
        }
    }

    /**
     * Hide/close a view by ID.
     */
    public static boolean hideView(String viewId, String secondaryId) {
        Object page = getActivePage();
        if (page == null) return false;

        try {
            Method findViewReference;
            Object viewRef;
            if (secondaryId != null && !secondaryId.isEmpty()) {
                findViewReference = workbenchPageClass.getMethod("findViewReference", String.class, String.class);
                viewRef = findViewReference.invoke(page, viewId, secondaryId);
            } else {
                findViewReference = workbenchPageClass.getMethod("findViewReference", String.class);
                viewRef = findViewReference.invoke(page, viewId);
            }

            if (viewRef == null) return false;

            Method hideView = workbenchPageClass.getMethod("hideView", viewReferenceClass);
            hideView.invoke(page, viewRef);
            return true;
        } catch (Exception e) {
            System.err.println("[EclipseWorkbenchHelper] Error hiding view: " + e.getMessage());
            return false;
        }
    }

    /**
     * Activate a view by ID.
     */
    public static boolean activateView(String viewId) {
        Object page = getActivePage();
        if (page == null) return false;

        try {
            Method findView = workbenchPageClass.getMethod("findView", String.class);
            Object view = findView.invoke(page, viewId);
            if (view == null) return false;

            Method activate = workbenchPageClass.getMethod("activate", Class.forName("org.eclipse.ui.IWorkbenchPart"));
            activate.invoke(page, view);
            return true;
        } catch (Exception e) {
            System.err.println("[EclipseWorkbenchHelper] Error activating view: " + e.getMessage());
            return false;
        }
    }

    /**
     * Get all open editors.
     */
    public static List<Map<String, Object>> getOpenEditors() {
        List<Map<String, Object>> result = new ArrayList<>();
        Object page = getActivePage();
        if (page == null) return result;

        try {
            Method getEditorReferences = workbenchPageClass.getMethod("getEditorReferences");
            Object[] editorRefs = (Object[]) getEditorReferences.invoke(page);

            if (editorRefs != null) {
                Method getId = editorReferenceClass.getMethod("getId");
                Method getName = editorReferenceClass.getMethod("getName");
                Method getTitle = editorReferenceClass.getMethod("getTitle");
                Method isDirty = editorReferenceClass.getMethod("isDirty");
                Method getEditorInput = editorReferenceClass.getMethod("getEditorInput");

                for (Object editorRef : editorRefs) {
                    Map<String, Object> info = new HashMap<>();
                    info.put("id", getId.invoke(editorRef));
                    info.put("name", getName.invoke(editorRef));
                    info.put("title", getTitle.invoke(editorRef));
                    info.put("dirty", isDirty.invoke(editorRef));

                    // Try to get file path from editor input
                    try {
                        Object input = getEditorInput.invoke(editorRef);
                        if (input != null) {
                            // Try IFileEditorInput
                            try {
                                Class<?> fileEditorInputClass = Class.forName("org.eclipse.ui.IFileEditorInput");
                                if (fileEditorInputClass.isInstance(input)) {
                                    Method getFile = fileEditorInputClass.getMethod("getFile");
                                    Object file = getFile.invoke(input);
                                    if (file != null) {
                                        Class<?> iFileClass = Class.forName("org.eclipse.core.resources.IFile");
                                        Method getFullPath = iFileClass.getMethod("getFullPath");
                                        Object path = getFullPath.invoke(file);
                                        if (path != null) {
                                            info.put("filePath", path.toString());
                                        }
                                    }
                                }
                            } catch (ClassNotFoundException e) {
                                // IFileEditorInput not available
                            }

                            // Try IPathEditorInput
                            try {
                                Class<?> pathEditorInputClass = Class.forName("org.eclipse.ui.IPathEditorInput");
                                if (pathEditorInputClass.isInstance(input)) {
                                    Method getPath = pathEditorInputClass.getMethod("getPath");
                                    Object path = getPath.invoke(input);
                                    if (path != null) {
                                        info.put("filePath", path.toString());
                                    }
                                }
                            } catch (ClassNotFoundException e) {
                                // IPathEditorInput not available
                            }
                        }
                    } catch (Exception e) {
                        // Ignore errors getting file path
                    }

                    result.add(info);
                }
            }
        } catch (Exception e) {
            System.err.println("[EclipseWorkbenchHelper] Error getting editors: " + e.getMessage());
        }
        return result;
    }

    /**
     * Get the active editor.
     */
    public static Map<String, Object> getActiveEditor() {
        Object page = getActivePage();
        if (page == null) return null;

        try {
            Method getActiveEditor = workbenchPageClass.getMethod("getActiveEditor");
            Object editor = getActiveEditor.invoke(page);
            if (editor == null) return null;

            Class<?> editorPartClass = Class.forName("org.eclipse.ui.IEditorPart");
            Method getTitle = editorPartClass.getMethod("getTitle");
            Method isDirty = editorPartClass.getMethod("isDirty");
            Method getEditorInput = editorPartClass.getMethod("getEditorInput");

            Map<String, Object> info = new HashMap<>();
            info.put("title", getTitle.invoke(editor));
            info.put("dirty", isDirty.invoke(editor));

            Object input = getEditorInput.invoke(editor);
            if (input != null) {
                Method getName = editorInputClass.getMethod("getName");
                info.put("name", getName.invoke(input));
            }

            return info;
        } catch (Exception e) {
            System.err.println("[EclipseWorkbenchHelper] Error getting active editor: " + e.getMessage());
            return null;
        }
    }

    /**
     * Close all editors.
     */
    public static boolean closeAllEditors(boolean save) {
        Object page = getActivePage();
        if (page == null) return false;

        try {
            Method closeAllEditors = workbenchPageClass.getMethod("closeAllEditors", boolean.class);
            return (Boolean) closeAllEditors.invoke(page, save);
        } catch (Exception e) {
            System.err.println("[EclipseWorkbenchHelper] Error closing all editors: " + e.getMessage());
            return false;
        }
    }

    /**
     * Save all editors.
     */
    public static boolean saveAllEditors(boolean confirm) {
        Object page = getActivePage();
        if (page == null) return false;

        try {
            Method saveAllEditors = workbenchPageClass.getMethod("saveAllEditors", boolean.class);
            return (Boolean) saveAllEditors.invoke(page, confirm);
        } catch (Exception e) {
            System.err.println("[EclipseWorkbenchHelper] Error saving all editors: " + e.getMessage());
            return false;
        }
    }

    /**
     * Reset the current perspective to its default layout.
     */
    public static boolean resetPerspective() {
        Object page = getActivePage();
        if (page == null) return false;

        try {
            Method resetPerspective = workbenchPageClass.getMethod("resetPerspective");
            resetPerspective.invoke(page);
            return true;
        } catch (Exception e) {
            System.err.println("[EclipseWorkbenchHelper] Error resetting perspective: " + e.getMessage());
            return false;
        }
    }

    /**
     * Execute an Eclipse command by ID.
     */
    public static boolean executeCommand(String commandId) {
        Object workbench = getWorkbench();
        if (workbench == null) return false;

        try {
            // Get handler service
            Method getService = workbenchClass.getMethod("getService", Class.class);
            Class<?> handlerServiceClass = Class.forName("org.eclipse.ui.handlers.IHandlerService");
            Object handlerService = getService.invoke(workbench, handlerServiceClass);

            if (handlerService == null) return false;

            // Execute command
            Method executeCommand = handlerServiceClass.getMethod("executeCommand", String.class, Object.class);
            executeCommand.invoke(handlerService, commandId, null);
            return true;
        } catch (Exception e) {
            System.err.println("[EclipseWorkbenchHelper] Error executing command: " + e.getMessage());
            return false;
        }
    }

    /**
     * Get workbench window count.
     */
    public static int getWorkbenchWindowCount() {
        Object workbench = getWorkbench();
        if (workbench == null) return 0;

        try {
            Method getWorkbenchWindows = workbenchClass.getMethod("getWorkbenchWindows");
            Object[] windows = (Object[]) getWorkbenchWindows.invoke(workbench);
            return windows != null ? windows.length : 0;
        } catch (Exception e) {
            return 0;
        }
    }

    /**
     * Get workbench window title.
     */
    public static String getWorkbenchTitle() {
        Object window = getActiveWindow();
        if (window == null) return null;

        try {
            Method getShell = workbenchWindowClass.getMethod("getShell");
            Object shell = getShell.invoke(window);
            if (shell == null) return null;

            Class<?> shellClass = Class.forName("org.eclipse.swt.widgets.Shell");
            Method getText = shellClass.getMethod("getText");
            return (String) getText.invoke(shell);
        } catch (Exception e) {
            return null;
        }
    }
}
