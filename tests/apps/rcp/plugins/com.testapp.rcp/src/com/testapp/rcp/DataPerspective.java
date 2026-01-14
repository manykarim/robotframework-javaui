package com.testapp.rcp;

import org.eclipse.ui.IFolderLayout;
import org.eclipse.ui.IPageLayout;
import org.eclipse.ui.IPerspectiveFactory;

/**
 * The Data Perspective.
 * Provides a layout optimized for data viewing and analysis.
 * Properties view is more prominent in this perspective.
 */
public class DataPerspective implements IPerspectiveFactory {

    /** Perspective ID */
    public static final String ID = "com.testapp.rcp.perspective.data";

    /** Folder IDs */
    private static final String LEFT_FOLDER = "com.testapp.rcp.data.folders.left";
    private static final String BOTTOM_FOLDER = "com.testapp.rcp.data.folders.bottom";
    private static final String RIGHT_FOLDER = "com.testapp.rcp.data.folders.right";

    @Override
    public void createInitialLayout(IPageLayout layout) {
        String editorArea = layout.getEditorArea();

        // Left: Navigator only
        IFolderLayout leftFolder = layout.createFolder(
            LEFT_FOLDER,
            IPageLayout.LEFT,
            0.15f,
            editorArea
        );
        leftFolder.addView("com.testapp.rcp.views.navigator");

        // Right: Properties and Outline (larger area for data)
        IFolderLayout rightFolder = layout.createFolder(
            RIGHT_FOLDER,
            IPageLayout.RIGHT,
            0.55f,
            editorArea
        );
        rightFolder.addView("com.testapp.rcp.views.properties");
        rightFolder.addView("com.testapp.rcp.views.outline");

        // Bottom: Console and Tasks
        IFolderLayout bottomFolder = layout.createFolder(
            BOTTOM_FOLDER,
            IPageLayout.BOTTOM,
            0.80f,
            editorArea
        );
        bottomFolder.addView("com.testapp.rcp.views.console");
        bottomFolder.addView("com.testapp.rcp.views.tasks");

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
