package com.testapp.rcp;

import org.eclipse.ui.IFolderLayout;
import org.eclipse.ui.IPageLayout;
import org.eclipse.ui.IPerspectiveFactory;

/**
 * The main Test Perspective.
 * Defines the default layout with Navigator, Properties, Console, and Outline views.
 *
 * View IDs:
 * - com.testapp.rcp.views.navigator (left)
 * - com.testapp.rcp.views.outline (bottom-left)
 * - com.testapp.rcp.views.properties (right)
 * - com.testapp.rcp.views.console (bottom)
 * - com.testapp.rcp.views.tasks (stacked with console)
 */
public class Perspective implements IPerspectiveFactory {

    /** Perspective ID */
    public static final String ID = "com.testapp.rcp.perspective.main";

    /** Folder IDs */
    private static final String LEFT_FOLDER = "com.testapp.rcp.folders.left";
    private static final String RIGHT_FOLDER = "com.testapp.rcp.folders.right";
    private static final String BOTTOM_FOLDER = "com.testapp.rcp.folders.bottom";
    private static final String OUTLINE_FOLDER = "com.testapp.rcp.folders.outline";

    @Override
    public void createInitialLayout(IPageLayout layout) {
        // Get the editor area
        String editorArea = layout.getEditorArea();

        // Create left folder (Navigator view)
        IFolderLayout leftFolder = layout.createFolder(
            LEFT_FOLDER,
            IPageLayout.LEFT,
            0.25f,
            editorArea
        );
        leftFolder.addView("com.testapp.rcp.views.navigator");

        // Create outline folder below left folder
        IFolderLayout outlineFolder = layout.createFolder(
            OUTLINE_FOLDER,
            IPageLayout.BOTTOM,
            0.60f,
            LEFT_FOLDER
        );
        outlineFolder.addView("com.testapp.rcp.views.outline");

        // Create right folder (Properties view)
        IFolderLayout rightFolder = layout.createFolder(
            RIGHT_FOLDER,
            IPageLayout.RIGHT,
            0.75f,
            editorArea
        );
        rightFolder.addView("com.testapp.rcp.views.properties");

        // Create bottom folder (Console and Tasks views)
        IFolderLayout bottomFolder = layout.createFolder(
            BOTTOM_FOLDER,
            IPageLayout.BOTTOM,
            0.70f,
            editorArea
        );
        bottomFolder.addView("com.testapp.rcp.views.console");
        bottomFolder.addView("com.testapp.rcp.views.tasks");

        // Add view shortcuts (Window > Show View menu)
        layout.addShowViewShortcut("com.testapp.rcp.views.navigator");
        layout.addShowViewShortcut("com.testapp.rcp.views.properties");
        layout.addShowViewShortcut("com.testapp.rcp.views.console");
        layout.addShowViewShortcut("com.testapp.rcp.views.outline");
        layout.addShowViewShortcut("com.testapp.rcp.views.tasks");

        // Add perspective shortcuts
        layout.addPerspectiveShortcut("com.testapp.rcp.perspective.main");
        layout.addPerspectiveShortcut("com.testapp.rcp.perspective.debug");
        layout.addPerspectiveShortcut("com.testapp.rcp.perspective.data");

        // Add new wizard shortcuts (File > New menu)
        layout.addNewWizardShortcut("com.testapp.rcp.wizards.newProject");
        layout.addNewWizardShortcut("com.testapp.rcp.wizards.newFile");

        // Add action sets
        layout.addActionSet("org.eclipse.ui.edit.text.actionSet.presentation");
    }
}
