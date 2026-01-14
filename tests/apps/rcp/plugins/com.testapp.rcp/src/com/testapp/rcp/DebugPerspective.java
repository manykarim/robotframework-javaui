package com.testapp.rcp;

import org.eclipse.ui.IFolderLayout;
import org.eclipse.ui.IPageLayout;
import org.eclipse.ui.IPerspectiveFactory;

/**
 * The Debug Perspective.
 * Provides a different layout optimized for debugging tasks.
 * Console view is more prominent in this perspective.
 */
public class DebugPerspective implements IPerspectiveFactory {

    /** Perspective ID */
    public static final String ID = "com.testapp.rcp.perspective.debug";

    /** Folder IDs */
    private static final String TOP_LEFT_FOLDER = "com.testapp.rcp.debug.folders.topLeft";
    private static final String BOTTOM_LEFT_FOLDER = "com.testapp.rcp.debug.folders.bottomLeft";
    private static final String RIGHT_FOLDER = "com.testapp.rcp.debug.folders.right";
    private static final String BOTTOM_FOLDER = "com.testapp.rcp.debug.folders.bottom";

    @Override
    public void createInitialLayout(IPageLayout layout) {
        String editorArea = layout.getEditorArea();

        // Top-left: Navigator and outline stacked
        IFolderLayout topLeftFolder = layout.createFolder(
            TOP_LEFT_FOLDER,
            IPageLayout.LEFT,
            0.20f,
            editorArea
        );
        topLeftFolder.addView("com.testapp.rcp.views.navigator");
        topLeftFolder.addView("com.testapp.rcp.views.outline");

        // Bottom-left: Variables/watches placeholder
        IFolderLayout bottomLeftFolder = layout.createFolder(
            BOTTOM_LEFT_FOLDER,
            IPageLayout.BOTTOM,
            0.50f,
            TOP_LEFT_FOLDER
        );
        bottomLeftFolder.addView("com.testapp.rcp.views.tasks");

        // Right: Properties
        IFolderLayout rightFolder = layout.createFolder(
            RIGHT_FOLDER,
            IPageLayout.RIGHT,
            0.70f,
            editorArea
        );
        rightFolder.addView("com.testapp.rcp.views.properties");

        // Bottom: Console (larger in debug perspective)
        IFolderLayout bottomFolder = layout.createFolder(
            BOTTOM_FOLDER,
            IPageLayout.BOTTOM,
            0.60f,
            editorArea
        );
        bottomFolder.addView("com.testapp.rcp.views.console");

        // View shortcuts
        layout.addShowViewShortcut("com.testapp.rcp.views.navigator");
        layout.addShowViewShortcut("com.testapp.rcp.views.properties");
        layout.addShowViewShortcut("com.testapp.rcp.views.console");
        layout.addShowViewShortcut("com.testapp.rcp.views.outline");
        layout.addShowViewShortcut("com.testapp.rcp.views.tasks");

        // Perspective shortcuts
        layout.addPerspectiveShortcut("com.testapp.rcp.perspective.main");
        layout.addPerspectiveShortcut("com.testapp.rcp.perspective.debug");
        layout.addPerspectiveShortcut("com.testapp.rcp.perspective.data");
    }
}
