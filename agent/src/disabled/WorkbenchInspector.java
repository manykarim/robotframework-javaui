package com.robotframework.rcp;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;

import org.eclipse.core.commands.Command;
import org.eclipse.core.commands.ExecutionEvent;
import org.eclipse.core.commands.IHandler;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IEditorReference;
import org.eclipse.ui.IPerspectiveDescriptor;
import org.eclipse.ui.IPerspectiveRegistry;
import org.eclipse.ui.IViewPart;
import org.eclipse.ui.IViewReference;
import org.eclipse.ui.IWorkbench;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.IWorkbenchWindow;
import org.eclipse.ui.PartInitException;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.commands.ICommandService;
import org.eclipse.ui.handlers.IHandlerService;

import java.util.concurrent.Callable;
import java.util.concurrent.atomic.AtomicReference;

/**
 * Inspector for Eclipse RCP Workbench structure.
 * Provides methods to inspect and manipulate perspectives, views, editors, and commands.
 * All methods use DisplayHelper.syncExec() for thread safety.
 */
public class WorkbenchInspector {

    /**
     * Get the complete workbench structure as JSON.
     * Includes windows, perspectives, views, and editors.
     *
     * @return JsonObject representing the workbench structure
     */
    public static JsonObject getWorkbench() {
        return syncExec(() -> {
            JsonObject result = new JsonObject();

            IWorkbench workbench = PlatformUI.getWorkbench();
            if (workbench == null) {
                result.addProperty("error", "Workbench not available");
                return result;
            }

            result.addProperty("running", !workbench.isClosing());
            result.addProperty("starting", workbench.isStarting());

            // Get all workbench windows
            JsonArray windowsArray = new JsonArray();
            IWorkbenchWindow[] windows = workbench.getWorkbenchWindows();

            for (IWorkbenchWindow window : windows) {
                JsonObject windowObj = buildWindowInfo(window);
                windowsArray.add(windowObj);
            }

            result.add("windows", windowsArray);
            result.addProperty("windowCount", windows.length);

            // Active window info
            IWorkbenchWindow activeWindow = workbench.getActiveWorkbenchWindow();
            if (activeWindow != null) {
                result.addProperty("activeWindowIndex", getWindowIndex(windows, activeWindow));
            }

            // Available perspectives
            result.add("availablePerspectives", getAvailablePerspectivesInternal());

            result.addProperty("timestamp", System.currentTimeMillis());

            return result;
        });
    }

    /**
     * Build detailed information about a workbench window.
     */
    private static JsonObject buildWindowInfo(IWorkbenchWindow window) {
        JsonObject windowObj = new JsonObject();

        // Shell info
        if (window.getShell() != null && !window.getShell().isDisposed()) {
            windowObj.addProperty("title", window.getShell().getText());
            windowObj.addProperty("x", window.getShell().getBounds().x);
            windowObj.addProperty("y", window.getShell().getBounds().y);
            windowObj.addProperty("width", window.getShell().getBounds().width);
            windowObj.addProperty("height", window.getShell().getBounds().height);
            windowObj.addProperty("visible", window.getShell().isVisible());
            windowObj.addProperty("maximized", window.getShell().getMaximized());
            windowObj.addProperty("minimized", window.getShell().getMinimized());
        }

        // Pages
        JsonArray pagesArray = new JsonArray();
        IWorkbenchPage[] pages = window.getPages();

        for (IWorkbenchPage page : pages) {
            JsonObject pageObj = buildPageInfo(page);
            pagesArray.add(pageObj);
        }

        windowObj.add("pages", pagesArray);
        windowObj.addProperty("pageCount", pages.length);

        // Active page
        IWorkbenchPage activePage = window.getActivePage();
        if (activePage != null) {
            windowObj.addProperty("activePageIndex", getPageIndex(pages, activePage));
        }

        return windowObj;
    }

    /**
     * Build detailed information about a workbench page.
     */
    private static JsonObject buildPageInfo(IWorkbenchPage page) {
        JsonObject pageObj = new JsonObject();

        pageObj.addProperty("label", page.getLabel());

        // Current perspective
        IPerspectiveDescriptor perspective = page.getPerspective();
        if (perspective != null) {
            JsonObject perspObj = new JsonObject();
            perspObj.addProperty("id", perspective.getId());
            perspObj.addProperty("label", perspective.getLabel());
            perspObj.addProperty("description", perspective.getDescription());
            pageObj.add("perspective", perspObj);
        }

        // Views
        JsonArray viewsArray = new JsonArray();
        IViewReference[] viewRefs = page.getViewReferences();

        for (IViewReference viewRef : viewRefs) {
            JsonObject viewObj = buildViewReferenceInfo(viewRef);
            viewsArray.add(viewObj);
        }

        pageObj.add("views", viewsArray);
        pageObj.addProperty("viewCount", viewRefs.length);

        // Editors
        JsonArray editorsArray = new JsonArray();
        IEditorReference[] editorRefs = page.getEditorReferences();

        for (IEditorReference editorRef : editorRefs) {
            JsonObject editorObj = buildEditorReferenceInfo(editorRef);
            editorsArray.add(editorObj);
        }

        pageObj.add("editors", editorsArray);
        pageObj.addProperty("editorCount", editorRefs.length);
        pageObj.addProperty("dirtyEditorCount", getDirtyEditorCount(editorRefs));

        // Active editor
        IEditorPart activeEditor = page.getActiveEditor();
        if (activeEditor != null) {
            pageObj.addProperty("activeEditorTitle", activeEditor.getTitle());
        }

        return pageObj;
    }

    /**
     * Build information about a view reference.
     */
    private static JsonObject buildViewReferenceInfo(IViewReference viewRef) {
        JsonObject viewObj = new JsonObject();

        viewObj.addProperty("id", viewRef.getId());
        viewObj.addProperty("secondaryId", viewRef.getSecondaryId());
        viewObj.addProperty("title", viewRef.getTitle());
        viewObj.addProperty("partName", viewRef.getPartName());
        viewObj.addProperty("contentDescription", viewRef.getContentDescription());
        viewObj.addProperty("dirty", viewRef.isDirty());
        viewObj.addProperty("fastView", viewRef.isFastView());

        // Check if the view is instantiated
        IViewPart view = viewRef.getView(false);
        viewObj.addProperty("instantiated", view != null);

        if (view != null) {
            viewObj.addProperty("class", view.getClass().getName());
            if (view.getSite() != null) {
                viewObj.addProperty("siteId", view.getSite().getId());
            }
        }

        return viewObj;
    }

    /**
     * Build information about an editor reference.
     */
    private static JsonObject buildEditorReferenceInfo(IEditorReference editorRef) {
        JsonObject editorObj = new JsonObject();

        editorObj.addProperty("id", editorRef.getId());
        editorObj.addProperty("name", editorRef.getName());
        editorObj.addProperty("title", editorRef.getTitle());
        editorObj.addProperty("partName", editorRef.getPartName());
        editorObj.addProperty("contentDescription", editorRef.getContentDescription());
        editorObj.addProperty("titleToolTip", editorRef.getTitleToolTip());
        editorObj.addProperty("dirty", editorRef.isDirty());
        editorObj.addProperty("pinned", editorRef.isPinned());

        // Check if the editor is instantiated
        IEditorPart editor = editorRef.getEditor(false);
        editorObj.addProperty("instantiated", editor != null);

        if (editor != null) {
            editorObj.addProperty("class", editor.getClass().getName());
            if (editor.getEditorInput() != null) {
                editorObj.addProperty("inputName", editor.getEditorInput().getName());
                editorObj.addProperty("inputToolTip", editor.getEditorInput().getToolTipText());
            }
        }

        return editorObj;
    }

    /**
     * Get all available perspectives.
     *
     * @return JsonArray of perspective information
     */
    public static JsonArray getAvailablePerspectives() {
        return syncExec(WorkbenchInspector::getAvailablePerspectivesInternal);
    }

    private static JsonArray getAvailablePerspectivesInternal() {
        JsonArray perspectives = new JsonArray();

        IWorkbench workbench = PlatformUI.getWorkbench();
        if (workbench == null) {
            return perspectives;
        }

        IPerspectiveRegistry registry = workbench.getPerspectiveRegistry();
        IPerspectiveDescriptor[] descriptors = registry.getPerspectives();

        for (IPerspectiveDescriptor desc : descriptors) {
            JsonObject perspObj = new JsonObject();
            perspObj.addProperty("id", desc.getId());
            perspObj.addProperty("label", desc.getLabel());
            perspObj.addProperty("description", desc.getDescription());

            // Check if this is the default perspective
            IPerspectiveDescriptor defaultPersp = registry.getDefaultPerspective() != null ?
                registry.findPerspectiveWithId(registry.getDefaultPerspective()) : null;
            perspObj.addProperty("isDefault", defaultPersp != null && desc.getId().equals(defaultPersp.getId()));

            perspectives.add(perspObj);
        }

        return perspectives;
    }

    /**
     * Open or switch to a perspective.
     *
     * @param perspectiveId The perspective ID to open
     * @return JsonObject with operation result
     */
    public static JsonObject openPerspective(String perspectiveId) {
        return syncExec(() -> {
            JsonObject result = new JsonObject();

            IWorkbench workbench = PlatformUI.getWorkbench();
            if (workbench == null) {
                result.addProperty("success", false);
                result.addProperty("error", "Workbench not available");
                return result;
            }

            IWorkbenchWindow window = workbench.getActiveWorkbenchWindow();
            if (window == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active workbench window");
                return result;
            }

            IWorkbenchPage page = window.getActivePage();
            if (page == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active page");
                return result;
            }

            IPerspectiveRegistry registry = workbench.getPerspectiveRegistry();
            IPerspectiveDescriptor perspective = registry.findPerspectiveWithId(perspectiveId);

            if (perspective == null) {
                result.addProperty("success", false);
                result.addProperty("error", "Perspective not found: " + perspectiveId);
                return result;
            }

            page.setPerspective(perspective);

            result.addProperty("success", true);
            result.addProperty("perspectiveId", perspectiveId);
            result.addProperty("perspectiveLabel", perspective.getLabel());

            return result;
        });
    }

    /**
     * Reset the current perspective layout to its default.
     *
     * @return JsonObject with operation result
     */
    public static JsonObject resetPerspective() {
        return syncExec(() -> {
            JsonObject result = new JsonObject();

            IWorkbench workbench = PlatformUI.getWorkbench();
            if (workbench == null) {
                result.addProperty("success", false);
                result.addProperty("error", "Workbench not available");
                return result;
            }

            IWorkbenchWindow window = workbench.getActiveWorkbenchWindow();
            if (window == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active workbench window");
                return result;
            }

            IWorkbenchPage page = window.getActivePage();
            if (page == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active page");
                return result;
            }

            IPerspectiveDescriptor perspective = page.getPerspective();
            if (perspective == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active perspective");
                return result;
            }

            page.resetPerspective();

            result.addProperty("success", true);
            result.addProperty("perspectiveId", perspective.getId());
            result.addProperty("perspectiveLabel", perspective.getLabel());

            return result;
        });
    }

    /**
     * Open/show a view.
     *
     * @param viewId The view ID to open
     * @param secondaryId Optional secondary ID for multi-instance views (can be null)
     * @return JsonObject with operation result
     */
    public static JsonObject showView(String viewId, String secondaryId) {
        return syncExec(() -> {
            JsonObject result = new JsonObject();

            IWorkbench workbench = PlatformUI.getWorkbench();
            if (workbench == null) {
                result.addProperty("success", false);
                result.addProperty("error", "Workbench not available");
                return result;
            }

            IWorkbenchWindow window = workbench.getActiveWorkbenchWindow();
            if (window == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active workbench window");
                return result;
            }

            IWorkbenchPage page = window.getActivePage();
            if (page == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active page");
                return result;
            }

            try {
                IViewPart view;
                if (secondaryId != null && !secondaryId.isEmpty()) {
                    view = page.showView(viewId, secondaryId, IWorkbenchPage.VIEW_ACTIVATE);
                } else {
                    view = page.showView(viewId);
                }

                result.addProperty("success", true);
                result.addProperty("viewId", viewId);
                result.addProperty("secondaryId", secondaryId);
                result.addProperty("viewTitle", view.getTitle());
                result.addProperty("viewClass", view.getClass().getName());

            } catch (PartInitException e) {
                result.addProperty("success", false);
                result.addProperty("error", "Failed to show view: " + e.getMessage());
            }

            return result;
        });
    }

    /**
     * Close a view.
     *
     * @param viewId The view ID to close
     * @param secondaryId Optional secondary ID (can be null)
     * @return JsonObject with operation result
     */
    public static JsonObject closeView(String viewId, String secondaryId) {
        return syncExec(() -> {
            JsonObject result = new JsonObject();

            IWorkbench workbench = PlatformUI.getWorkbench();
            if (workbench == null) {
                result.addProperty("success", false);
                result.addProperty("error", "Workbench not available");
                return result;
            }

            IWorkbenchWindow window = workbench.getActiveWorkbenchWindow();
            if (window == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active workbench window");
                return result;
            }

            IWorkbenchPage page = window.getActivePage();
            if (page == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active page");
                return result;
            }

            IViewReference viewRef = page.findViewReference(viewId, secondaryId);
            if (viewRef == null) {
                result.addProperty("success", false);
                result.addProperty("error", "View not found: " + viewId +
                    (secondaryId != null ? " (secondaryId: " + secondaryId + ")" : ""));
                return result;
            }

            page.hideView(viewRef);

            result.addProperty("success", true);
            result.addProperty("viewId", viewId);
            result.addProperty("secondaryId", secondaryId);

            return result;
        });
    }

    /**
     * Activate (bring to front) a view.
     *
     * @param viewId The view ID to activate
     * @return JsonObject with operation result
     */
    public static JsonObject activateView(String viewId) {
        return syncExec(() -> {
            JsonObject result = new JsonObject();

            IWorkbench workbench = PlatformUI.getWorkbench();
            if (workbench == null) {
                result.addProperty("success", false);
                result.addProperty("error", "Workbench not available");
                return result;
            }

            IWorkbenchWindow window = workbench.getActiveWorkbenchWindow();
            if (window == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active workbench window");
                return result;
            }

            IWorkbenchPage page = window.getActivePage();
            if (page == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active page");
                return result;
            }

            IViewReference viewRef = page.findViewReference(viewId);
            if (viewRef == null) {
                result.addProperty("success", false);
                result.addProperty("error", "View not found: " + viewId);
                return result;
            }

            IViewPart view = viewRef.getView(true);
            if (view == null) {
                result.addProperty("success", false);
                result.addProperty("error", "Could not instantiate view: " + viewId);
                return result;
            }

            page.activate(view);

            result.addProperty("success", true);
            result.addProperty("viewId", viewId);
            result.addProperty("viewTitle", view.getTitle());

            return result;
        });
    }

    /**
     * Get information about the active editor.
     *
     * @return JsonObject with active editor information
     */
    public static JsonObject getActiveEditor() {
        return syncExec(() -> {
            JsonObject result = new JsonObject();

            IWorkbench workbench = PlatformUI.getWorkbench();
            if (workbench == null) {
                result.addProperty("hasActiveEditor", false);
                result.addProperty("error", "Workbench not available");
                return result;
            }

            IWorkbenchWindow window = workbench.getActiveWorkbenchWindow();
            if (window == null) {
                result.addProperty("hasActiveEditor", false);
                result.addProperty("error", "No active workbench window");
                return result;
            }

            IWorkbenchPage page = window.getActivePage();
            if (page == null) {
                result.addProperty("hasActiveEditor", false);
                result.addProperty("error", "No active page");
                return result;
            }

            IEditorPart editor = page.getActiveEditor();
            if (editor == null) {
                result.addProperty("hasActiveEditor", false);
                return result;
            }

            result.addProperty("hasActiveEditor", true);
            result.addProperty("title", editor.getTitle());
            result.addProperty("titleToolTip", editor.getTitleToolTip());
            result.addProperty("dirty", editor.isDirty());
            result.addProperty("class", editor.getClass().getName());

            if (editor.getSite() != null) {
                result.addProperty("siteId", editor.getSite().getId());
            }

            if (editor.getEditorInput() != null) {
                JsonObject inputObj = new JsonObject();
                inputObj.addProperty("name", editor.getEditorInput().getName());
                inputObj.addProperty("toolTipText", editor.getEditorInput().getToolTipText());
                inputObj.addProperty("exists", editor.getEditorInput().exists());
                inputObj.addProperty("class", editor.getEditorInput().getClass().getName());
                result.add("editorInput", inputObj);
            }

            return result;
        });
    }

    /**
     * Close all editors.
     *
     * @param save Whether to save dirty editors before closing
     * @return JsonObject with operation result
     */
    public static JsonObject closeAllEditors(boolean save) {
        return syncExec(() -> {
            JsonObject result = new JsonObject();

            IWorkbench workbench = PlatformUI.getWorkbench();
            if (workbench == null) {
                result.addProperty("success", false);
                result.addProperty("error", "Workbench not available");
                return result;
            }

            IWorkbenchWindow window = workbench.getActiveWorkbenchWindow();
            if (window == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active workbench window");
                return result;
            }

            IWorkbenchPage page = window.getActivePage();
            if (page == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active page");
                return result;
            }

            int editorCount = page.getEditorReferences().length;
            boolean closed = page.closeAllEditors(save);

            result.addProperty("success", closed);
            result.addProperty("editorsClosed", editorCount);
            result.addProperty("saved", save);

            if (!closed) {
                result.addProperty("message", "Some editors may not have been closed (user cancelled or save failed)");
            }

            return result;
        });
    }

    /**
     * Save all dirty editors.
     *
     * @return JsonObject with operation result
     */
    public static JsonObject saveAllEditors() {
        return syncExec(() -> {
            JsonObject result = new JsonObject();

            IWorkbench workbench = PlatformUI.getWorkbench();
            if (workbench == null) {
                result.addProperty("success", false);
                result.addProperty("error", "Workbench not available");
                return result;
            }

            IWorkbenchWindow window = workbench.getActiveWorkbenchWindow();
            if (window == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active workbench window");
                return result;
            }

            IWorkbenchPage page = window.getActivePage();
            if (page == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active page");
                return result;
            }

            // Count dirty editors before save
            int dirtyCount = 0;
            for (IEditorReference ref : page.getEditorReferences()) {
                if (ref.isDirty()) {
                    dirtyCount++;
                }
            }

            boolean saved = page.saveAllEditors(false); // false = don't confirm

            result.addProperty("success", saved);
            result.addProperty("dirtyEditorCount", dirtyCount);

            if (!saved) {
                result.addProperty("message", "Some editors may not have been saved");
            }

            return result;
        });
    }

    /**
     * Execute an Eclipse command by ID.
     *
     * @param commandId The command ID to execute
     * @return JsonObject with operation result
     */
    public static JsonObject executeCommand(String commandId) {
        return syncExec(() -> {
            JsonObject result = new JsonObject();

            IWorkbench workbench = PlatformUI.getWorkbench();
            if (workbench == null) {
                result.addProperty("success", false);
                result.addProperty("error", "Workbench not available");
                return result;
            }

            IWorkbenchWindow window = workbench.getActiveWorkbenchWindow();
            if (window == null) {
                result.addProperty("success", false);
                result.addProperty("error", "No active workbench window");
                return result;
            }

            try {
                // Get command service
                ICommandService commandService = workbench.getService(ICommandService.class);
                if (commandService == null) {
                    result.addProperty("success", false);
                    result.addProperty("error", "Command service not available");
                    return result;
                }

                Command command = commandService.getCommand(commandId);
                if (command == null || !command.isDefined()) {
                    result.addProperty("success", false);
                    result.addProperty("error", "Command not found or not defined: " + commandId);
                    return result;
                }

                result.addProperty("commandName", command.getName());
                result.addProperty("commandDescription", command.getDescription());

                // Check if command is enabled
                IHandler handler = command.getHandler();
                if (handler == null || !handler.isEnabled()) {
                    result.addProperty("success", false);
                    result.addProperty("error", "Command is not enabled: " + commandId);
                    result.addProperty("hasHandler", handler != null);
                    return result;
                }

                // Get handler service and execute
                IHandlerService handlerService = workbench.getService(IHandlerService.class);
                if (handlerService == null) {
                    result.addProperty("success", false);
                    result.addProperty("error", "Handler service not available");
                    return result;
                }

                Object returnValue = handlerService.executeCommand(commandId, null);

                result.addProperty("success", true);
                result.addProperty("commandId", commandId);
                if (returnValue != null) {
                    result.addProperty("returnValue", returnValue.toString());
                }

            } catch (Exception e) {
                result.addProperty("success", false);
                result.addProperty("error", "Command execution failed: " + e.getMessage());
                result.addProperty("exceptionClass", e.getClass().getName());
            }

            return result;
        });
    }

    // ========== Helper Methods ==========

    /**
     * Execute a callable on the SWT display thread synchronously.
     *
     * @param callable The callable to execute
     * @param <T> Return type
     * @return The result from the callable
     */
    private static <T> T syncExec(Callable<T> callable) {
        Display display = Display.getDefault();

        if (display == null) {
            throw new IllegalStateException("No SWT Display available");
        }

        if (display.isDisposed()) {
            throw new IllegalStateException("SWT Display is disposed");
        }

        // If already on the display thread, execute directly
        if (Thread.currentThread() == display.getThread()) {
            try {
                return callable.call();
            } catch (Exception e) {
                throw new RuntimeException("Display thread action failed", e);
            }
        }

        // Execute on display thread and wait for result
        AtomicReference<T> result = new AtomicReference<>();
        AtomicReference<Exception> exception = new AtomicReference<>();

        display.syncExec(() -> {
            try {
                result.set(callable.call());
            } catch (Exception e) {
                exception.set(e);
            }
        });

        if (exception.get() != null) {
            throw new RuntimeException("Display thread action failed", exception.get());
        }

        return result.get();
    }

    /**
     * Get window index in array.
     */
    private static int getWindowIndex(IWorkbenchWindow[] windows, IWorkbenchWindow window) {
        for (int i = 0; i < windows.length; i++) {
            if (windows[i] == window) {
                return i;
            }
        }
        return -1;
    }

    /**
     * Get page index in array.
     */
    private static int getPageIndex(IWorkbenchPage[] pages, IWorkbenchPage page) {
        for (int i = 0; i < pages.length; i++) {
            if (pages[i] == page) {
                return i;
            }
        }
        return -1;
    }

    /**
     * Count dirty editors.
     */
    private static int getDirtyEditorCount(IEditorReference[] editorRefs) {
        int count = 0;
        for (IEditorReference ref : editorRefs) {
            if (ref.isDirty()) {
                count++;
            }
        }
        return count;
    }
}
