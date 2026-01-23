package com.robotframework.swt;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import org.eclipse.swt.widgets.*;

import java.lang.reflect.Method;
import java.util.HashMap;
import java.util.Map;

/**
 * RCP Component tree inspector that builds a hierarchical representation of
 * Eclipse RCP workbench structure.
 *
 * Traverses:
 * - Workbench Windows
 * - Perspectives
 * - Views (ViewParts)
 * - Editors (EditorParts)
 * - Toolbars
 * - Menus
 *
 * For each RCP component, exposes the underlying SWT widget tree, allowing
 * all SWT operations to be performed on RCP widgets (since RCP is built on SWT).
 */
public class RcpComponentInspector {

    // Eclipse RCP classes (loaded via reflection to avoid compile-time dependencies)
    private static Class<?> workbenchClass;
    private static Class<?> workbenchWindowClass;
    private static Class<?> workbenchPageClass;
    private static Class<?> workbenchPartClass;
    private static Class<?> viewPartClass;
    private static Class<?> editorPartClass;
    private static Class<?> perspectiveDescriptorClass;
    private static Class<?> viewReferenceClass;
    private static Class<?> editorReferenceClass;
    private static Class<?> partStackClass;

    /**
     * Get the complete RCP component tree for all workbench windows.
     *
     * @param maxDepth Maximum depth for SWT widget trees under each RCP component
     * @return JsonObject with RCP hierarchy
     */
    public static JsonObject getRcpComponentTree(int maxDepth) {
        if (!EclipseWorkbenchHelper.isEclipseAvailable()) {
            JsonObject result = new JsonObject();
            result.addProperty("error", "Eclipse RCP not available");
            result.addProperty("available", false);
            return result;
        }

        return DisplayHelper.syncExecAndReturn(() -> {
            try {
                JsonObject tree = new JsonObject();
                tree.addProperty("type", "RcpWorkbench");
                tree.addProperty("available", true);
                tree.addProperty("timestamp", System.currentTimeMillis());

                // Get workbench
                Object workbench = EclipseWorkbenchHelper.getWorkbench();
                if (workbench == null) {
                    tree.addProperty("error", "Workbench not accessible");
                    return tree;
                }

                // Initialize reflection classes if needed
                initializeReflectionClasses();

                // Get all workbench windows
                JsonArray windows = new JsonArray();
                Method getWorkbenchWindows = workbenchClass.getMethod("getWorkbenchWindows");
                Object[] workbenchWindows = (Object[]) getWorkbenchWindows.invoke(workbench);

                if (workbenchWindows != null) {
                    for (Object window : workbenchWindows) {
                        windows.add(buildWorkbenchWindowNode(window, maxDepth));
                    }
                }

                tree.add("windows", windows);
                tree.addProperty("windowCount", workbenchWindows != null ? workbenchWindows.length : 0);

                return tree;
            } catch (Exception e) {
                JsonObject error = new JsonObject();
                error.addProperty("error", e.getMessage());
                error.addProperty("exception", e.getClass().getName());
                return error;
            }
        });
    }

    /**
     * Initialize reflection classes for RCP components.
     */
    private static void initializeReflectionClasses() throws ClassNotFoundException {
        if (workbenchClass == null) {
            workbenchClass = Class.forName("org.eclipse.ui.IWorkbench");
            workbenchWindowClass = Class.forName("org.eclipse.ui.IWorkbenchWindow");
            workbenchPageClass = Class.forName("org.eclipse.ui.IWorkbenchPage");
            workbenchPartClass = Class.forName("org.eclipse.ui.IWorkbenchPart");
            viewPartClass = Class.forName("org.eclipse.ui.IViewPart");
            editorPartClass = Class.forName("org.eclipse.ui.IEditorPart");
            perspectiveDescriptorClass = Class.forName("org.eclipse.ui.IPerspectiveDescriptor");
            viewReferenceClass = Class.forName("org.eclipse.ui.IViewReference");
            editorReferenceClass = Class.forName("org.eclipse.ui.IEditorReference");

            // Try to load PartStack (internal class, may not be available)
            try {
                partStackClass = Class.forName("org.eclipse.ui.internal.PartStack");
            } catch (ClassNotFoundException e) {
                partStackClass = null; // Not critical
            }
        }
    }

    /**
     * Build a JSON node for a workbench window and its contents.
     */
    private static JsonObject buildWorkbenchWindowNode(Object window, int maxDepth) throws Exception {
        JsonObject node = new JsonObject();
        node.addProperty("type", "WorkbenchWindow");

        // Get shell (SWT widget)
        Method getShell = workbenchWindowClass.getMethod("getShell");
        Object shell = getShell.invoke(window);

        if (shell instanceof Shell) {
            Shell swtShell = (Shell) shell;
            node.addProperty("title", swtShell.getText());
            node.addProperty("active", swtShell.isVisible() && !swtShell.getMinimized());

            // Add SWT widget ID so all SWT operations can be performed
            int shellId = WidgetInspector.getOrCreateId(swtShell);
            node.addProperty("swtShellId", shellId);
            node.addProperty("swtClass", "org.eclipse.swt.widgets.Shell");

            // Optionally include full SWT widget tree for the shell
            if (maxDepth > 0) {
                JsonObject swtTree = WidgetInspector.getWidgetTree(shellId, maxDepth);
                node.add("swtWidgetTree", swtTree);
            }
        }

        // Get pages
        JsonArray pages = new JsonArray();
        Method getPages = workbenchWindowClass.getMethod("getPages");
        Object[] windowPages = (Object[]) getPages.invoke(window);

        if (windowPages != null) {
            for (Object page : windowPages) {
                pages.add(buildPageNode(page, maxDepth));
            }
        }

        node.add("pages", pages);
        node.addProperty("pageCount", windowPages != null ? windowPages.length : 0);

        // Get active page
        Method getActivePage = workbenchWindowClass.getMethod("getActivePage");
        Object activePage = getActivePage.invoke(window);
        if (activePage != null) {
            node.addProperty("activePageIndex", findPageIndex(windowPages, activePage));
        }

        return node;
    }

    /**
     * Build a JSON node for a workbench page (contains perspective, views, editors).
     */
    private static JsonObject buildPageNode(Object page, int maxDepth) throws Exception {
        JsonObject node = new JsonObject();
        node.addProperty("type", "WorkbenchPage");

        // Get perspective
        Method getPerspective = workbenchPageClass.getMethod("getPerspective");
        Object perspective = getPerspective.invoke(page);

        if (perspective != null) {
            JsonObject perspNode = new JsonObject();
            perspNode.addProperty("type", "Perspective");

            Method getId = perspectiveDescriptorClass.getMethod("getId");
            Method getLabel = perspectiveDescriptorClass.getMethod("getLabel");

            perspNode.addProperty("id", (String) getId.invoke(perspective));
            perspNode.addProperty("label", (String) getLabel.invoke(perspective));

            node.add("perspective", perspNode);
        }

        // Get views
        JsonArray views = new JsonArray();
        Method getViewReferences = workbenchPageClass.getMethod("getViewReferences");
        Object[] viewRefs = (Object[]) getViewReferences.invoke(page);

        if (viewRefs != null) {
            for (Object viewRef : viewRefs) {
                views.add(buildViewNode(viewRef, maxDepth));
            }
        }

        node.add("views", views);
        node.addProperty("viewCount", viewRefs != null ? viewRefs.length : 0);

        // Get editors
        JsonArray editors = new JsonArray();
        Method getEditorReferences = workbenchPageClass.getMethod("getEditorReferences");
        Object[] editorRefs = (Object[]) getEditorReferences.invoke(page);

        if (editorRefs != null) {
            for (Object editorRef : editorRefs) {
                editors.add(buildEditorNode(editorRef, maxDepth));
            }
        }

        node.add("editors", editors);
        node.addProperty("editorCount", editorRefs != null ? editorRefs.length : 0);

        // Get active part
        Method getActivePart = workbenchPageClass.getMethod("getActivePart");
        Object activePart = getActivePart.invoke(page);
        if (activePart != null) {
            node.addProperty("activePart", getPartName(activePart));
        }

        return node;
    }

    /**
     * Build a JSON node for a view (ViewPart).
     */
    private static JsonObject buildViewNode(Object viewRef, int maxDepth) throws Exception {
        JsonObject node = new JsonObject();
        node.addProperty("type", "ViewPart");

        // Get view metadata
        Method getId = viewReferenceClass.getMethod("getId");
        Method getSecondaryId = viewReferenceClass.getMethod("getSecondaryId");
        Method getPartName = viewReferenceClass.getMethod("getPartName");
        Method getTitle = viewReferenceClass.getMethod("getTitle");
        Method isFastView = viewReferenceClass.getMethod("isFastView");

        String viewId = (String) getId.invoke(viewRef);
        String secondaryId = (String) getSecondaryId.invoke(viewRef);
        String partName = (String) getPartName.invoke(viewRef);
        String title = (String) getTitle.invoke(viewRef);
        boolean fastView = (Boolean) isFastView.invoke(viewRef);

        node.addProperty("id", viewId);
        if (secondaryId != null && !secondaryId.isEmpty()) {
            node.addProperty("secondaryId", secondaryId);
        }
        node.addProperty("name", partName);
        node.addProperty("title", title);
        node.addProperty("fastView", fastView);

        // Get the actual view part and its SWT control
        try {
            Method getPart = viewReferenceClass.getMethod("getPart", boolean.class);
            Object viewPart = getPart.invoke(viewRef, false); // Don't restore if not created

            if (viewPart != null) {
                // Get plugin information
                try {
                    Class<?> pluginClass = Class.forName("org.eclipse.core.runtime.IPluginDescriptor");
                    Method getPluginId = viewPartClass.getMethod("getPluginId");
                    if (getPluginId != null) {
                        Object pluginId = getPluginId.invoke(viewPart);
                        if (pluginId != null) {
                            node.addProperty("pluginId", pluginId.toString());
                        }
                    }
                } catch (Exception e) {
                    // Plugin info not available, continue
                }

                // Get SWT control for the view
                Class<?> iPartClass = Class.forName("org.eclipse.ui.IWorkbenchPart");
                Method createPartControl = iPartClass.getMethod("createPartControl", Composite.class);

                // Try to get the view's SWT composite
                try {
                    // Use getSite() to get the composite
                    Method getSite = iPartClass.getMethod("getSite");
                    Object site = getSite.invoke(viewPart);

                    if (site != null) {
                        Class<?> siteClass = Class.forName("org.eclipse.ui.IWorkbenchPartSite");
                        Method getShell = siteClass.getMethod("getShell");
                        Object shell = getShell.invoke(site);

                        // Try to find the view's composite within the shell
                        // This is a simplified approach - in reality, views have complex layouts
                        if (shell instanceof Shell) {
                            Shell swtShell = (Shell) shell;
                            node.addProperty("swtShellId", WidgetInspector.getOrCreateId(swtShell));
                        }
                    }
                } catch (Exception e) {
                    // SWT control access failed, continue
                }

                // Add underlying SWT widget tree if available
                addSwtWidgetForPart(node, viewPart, maxDepth);
            } else {
                node.addProperty("partCreated", false);
            }
        } catch (Exception e) {
            node.addProperty("error", "Failed to get view part: " + e.getMessage());
        }

        return node;
    }

    /**
     * Build a JSON node for an editor (EditorPart).
     */
    private static JsonObject buildEditorNode(Object editorRef, int maxDepth) throws Exception {
        JsonObject node = new JsonObject();
        node.addProperty("type", "EditorPart");

        // Get editor metadata
        Method getId = editorReferenceClass.getMethod("getId");
        Method getName = editorReferenceClass.getMethod("getName");
        Method getTitle = editorReferenceClass.getMethod("getTitle");
        Method getTitleToolTip = editorReferenceClass.getMethod("getTitleToolTip");
        Method isDirty = editorReferenceClass.getMethod("isDirty");

        String editorId = (String) getId.invoke(editorRef);
        String name = (String) getName.invoke(editorRef);
        String title = (String) getTitle.invoke(editorRef);
        String tooltip = (String) getTitleToolTip.invoke(editorRef);
        boolean dirty = (Boolean) isDirty.invoke(editorRef);

        node.addProperty("id", editorId);
        node.addProperty("name", name);
        node.addProperty("title", title);
        node.addProperty("tooltip", tooltip);
        node.addProperty("dirty", dirty);

        // Try to get file path
        try {
            Method getEditorInput = editorReferenceClass.getMethod("getEditorInput");
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
                                node.addProperty("filePath", path.toString());
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
                            node.addProperty("filePath", path.toString());
                        }
                    }
                } catch (ClassNotFoundException e) {
                    // IPathEditorInput not available
                }
            }
        } catch (Exception e) {
            // File path not available
        }

        // Get the actual editor part and its SWT control
        try {
            Method getPart = editorReferenceClass.getMethod("getPart", boolean.class);
            Object editorPart = getPart.invoke(editorRef, false); // Don't restore if not created

            if (editorPart != null) {
                node.addProperty("partCreated", true);

                // Add underlying SWT widget tree
                addSwtWidgetForPart(node, editorPart, maxDepth);
            } else {
                node.addProperty("partCreated", false);
            }
        } catch (Exception e) {
            node.addProperty("error", "Failed to get editor part: " + e.getMessage());
        }

        return node;
    }

    /**
     * Add SWT widget information for a workbench part (view or editor).
     * Since RCP parts ARE SWT composites, we expose the underlying widget tree.
     */
    private static void addSwtWidgetForPart(JsonObject node, Object part, int maxDepth) {
        try {
            // Get the part's SWT control
            Class<?> iPartClass = Class.forName("org.eclipse.ui.IWorkbenchPart");
            Method getSite = iPartClass.getMethod("getSite");
            Object site = getSite.invoke(part);

            if (site != null) {
                // Try to get the part's parent composite
                Class<?> siteClass = Class.forName("org.eclipse.ui.IWorkbenchPartSite");

                // First try getPage() then getShell()
                try {
                    Method getPage = siteClass.getMethod("getPage");
                    Object page = getPage.invoke(site);

                    if (page != null) {
                        Class<?> pageClass = Class.forName("org.eclipse.ui.IWorkbenchPage");
                        Method getWorkbenchWindow = pageClass.getMethod("getWorkbenchWindow");
                        Object window = getWorkbenchWindow.invoke(page);

                        if (window != null) {
                            Method getShell = workbenchWindowClass.getMethod("getShell");
                            Object shell = getShell.invoke(window);

                            if (shell instanceof Shell) {
                                Shell swtShell = (Shell) shell;

                                // Find the part's composite within the shell
                                // This requires traversing the SWT tree to find the specific part control
                                Control partControl = findPartControl(swtShell, part);

                                if (partControl != null) {
                                    int controlId = WidgetInspector.getOrCreateId(partControl);
                                    node.addProperty("swtControlId", controlId);
                                    node.addProperty("swtControlClass", partControl.getClass().getName());

                                    // Include SWT widget subtree if depth allows
                                    if (maxDepth > 0) {
                                        JsonObject swtTree = WidgetInspector.getWidgetTree(controlId, maxDepth - 1);
                                        node.add("swtWidgetTree", swtTree);
                                    }
                                }
                            }
                        }
                    }
                } catch (Exception e) {
                    // Couldn't get page/shell, try direct approach
                }
            }
        } catch (Exception e) {
            // SWT widget access failed - not critical
            node.addProperty("swtWidgetError", e.getMessage());
        }
    }

    /**
     * Find the SWT control for a workbench part.
     * This is a simplified implementation - real RCP has complex part layouts.
     */
    private static Control findPartControl(Composite parent, Object part) {
        try {
            // Check if any child has the part as widget data
            for (Control child : parent.getChildren()) {
                Object data = child.getData();
                if (data == part) {
                    return child;
                }

                // Recursively search composites
                if (child instanceof Composite) {
                    Control found = findPartControl((Composite) child, part);
                    if (found != null) {
                        return found;
                    }
                }
            }
        } catch (Exception e) {
            // Ignore errors during search
        }
        return null;
    }

    /**
     * Get a part name from a workbench part.
     */
    private static String getPartName(Object part) {
        try {
            Class<?> iPartClass = Class.forName("org.eclipse.ui.IWorkbenchPart");
            Method getPartName = iPartClass.getMethod("getPartName");
            return (String) getPartName.invoke(part);
        } catch (Exception e) {
            return "Unknown";
        }
    }

    /**
     * Find the index of a page in the pages array.
     */
    private static int findPageIndex(Object[] pages, Object targetPage) {
        if (pages == null) return -1;
        for (int i = 0; i < pages.length; i++) {
            if (pages[i] == targetPage) {
                return i;
            }
        }
        return -1;
    }

    /**
     * Get a specific RCP component by path (e.g., "window[0]/page[0]/view[org.example.view]").
     */
    public static JsonObject getRcpComponent(String componentPath, int maxDepth) {
        if (!EclipseWorkbenchHelper.isEclipseAvailable()) {
            JsonObject result = new JsonObject();
            result.addProperty("error", "Eclipse RCP not available");
            return result;
        }

        // Implementation would parse the path and navigate to the specific component
        // For now, return a simple error
        JsonObject result = new JsonObject();
        result.addProperty("error", "Component path navigation not yet implemented");
        result.addProperty("path", componentPath);
        return result;
    }

    /**
     * Get all RCP views with their SWT widget IDs.
     */
    public static JsonArray getAllViews(boolean includeSwtWidgets) {
        if (!EclipseWorkbenchHelper.isEclipseAvailable()) {
            return new JsonArray();
        }

        return DisplayHelper.syncExecAndReturn(() -> {
            JsonArray views = new JsonArray();

            try {
                initializeReflectionClasses();

                Object workbench = EclipseWorkbenchHelper.getWorkbench();
                if (workbench == null) return views;

                Method getWorkbenchWindows = workbenchClass.getMethod("getWorkbenchWindows");
                Object[] windows = (Object[]) getWorkbenchWindows.invoke(workbench);

                if (windows != null) {
                    for (Object window : windows) {
                        Method getPages = workbenchWindowClass.getMethod("getPages");
                        Object[] pages = (Object[]) getPages.invoke(window);

                        if (pages != null) {
                            for (Object page : pages) {
                                Method getViewReferences = workbenchPageClass.getMethod("getViewReferences");
                                Object[] viewRefs = (Object[]) getViewReferences.invoke(page);

                                if (viewRefs != null) {
                                    for (Object viewRef : viewRefs) {
                                        JsonObject viewInfo = buildViewNode(viewRef, includeSwtWidgets ? 2 : 0);
                                        views.add(viewInfo);
                                    }
                                }
                            }
                        }
                    }
                }
            } catch (Exception e) {
                JsonObject error = new JsonObject();
                error.addProperty("error", e.getMessage());
                views.add(error);
            }

            return views;
        });
    }

    /**
     * Get all RCP editors with their SWT widget IDs.
     */
    public static JsonArray getAllEditors(boolean includeSwtWidgets) {
        if (!EclipseWorkbenchHelper.isEclipseAvailable()) {
            return new JsonArray();
        }

        return DisplayHelper.syncExecAndReturn(() -> {
            JsonArray editors = new JsonArray();

            try {
                initializeReflectionClasses();

                Object workbench = EclipseWorkbenchHelper.getWorkbench();
                if (workbench == null) return editors;

                Method getWorkbenchWindows = workbenchClass.getMethod("getWorkbenchWindows");
                Object[] windows = (Object[]) getWorkbenchWindows.invoke(workbench);

                if (windows != null) {
                    for (Object window : windows) {
                        Method getPages = workbenchWindowClass.getMethod("getPages");
                        Object[] pages = (Object[]) getPages.invoke(window);

                        if (pages != null) {
                            for (Object page : pages) {
                                Method getEditorReferences = workbenchPageClass.getMethod("getEditorReferences");
                                Object[] editorRefs = (Object[]) getEditorReferences.invoke(page);

                                if (editorRefs != null) {
                                    for (Object editorRef : editorRefs) {
                                        JsonObject editorInfo = buildEditorNode(editorRef, includeSwtWidgets ? 2 : 0);
                                        editors.add(editorInfo);
                                    }
                                }
                            }
                        }
                    }
                }
            } catch (Exception e) {
                JsonObject error = new JsonObject();
                error.addProperty("error", e.getMessage());
                editors.add(error);
            }

            return editors;
        });
    }
}
