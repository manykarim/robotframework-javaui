package com.testapp.rcp;

import org.eclipse.jface.action.IContributionItem;
import org.eclipse.jface.action.ICoolBarManager;
import org.eclipse.jface.action.IMenuManager;
import org.eclipse.jface.action.IStatusLineManager;
import org.eclipse.jface.action.MenuManager;
import org.eclipse.jface.action.Separator;
import org.eclipse.jface.action.StatusLineContributionItem;
import org.eclipse.ui.IWorkbenchActionConstants;
import org.eclipse.ui.IWorkbenchWindow;
import org.eclipse.ui.actions.ActionFactory;
import org.eclipse.ui.actions.ActionFactory.IWorkbenchAction;
import org.eclipse.ui.application.ActionBarAdvisor;
import org.eclipse.ui.application.IActionBarConfigurer;

/**
 * An action bar advisor is responsible for creating, adding, and disposing
 * of the actions added to a workbench window. Each window will have its
 * own action bar advisor.
 */
public class ApplicationActionBarAdvisor extends ActionBarAdvisor {

    // Standard workbench actions
    private IWorkbenchAction exitAction;
    private IWorkbenchAction saveAction;
    private IWorkbenchAction saveAllAction;
    private IWorkbenchAction undoAction;
    private IWorkbenchAction redoAction;
    private IWorkbenchAction cutAction;
    private IWorkbenchAction copyAction;
    private IWorkbenchAction pasteAction;
    private IWorkbenchAction deleteAction;
    private IWorkbenchAction selectAllAction;
    private IWorkbenchAction preferencesAction;
    private IWorkbenchAction aboutAction;

    // Status line item
    private StatusLineContributionItem statusItem;

    public ApplicationActionBarAdvisor(IActionBarConfigurer configurer) {
        super(configurer);
    }

    @Override
    protected void makeActions(IWorkbenchWindow window) {
        // Create standard workbench actions
        exitAction = ActionFactory.QUIT.create(window);
        register(exitAction);

        saveAction = ActionFactory.SAVE.create(window);
        register(saveAction);

        saveAllAction = ActionFactory.SAVE_ALL.create(window);
        register(saveAllAction);

        undoAction = ActionFactory.UNDO.create(window);
        register(undoAction);

        redoAction = ActionFactory.REDO.create(window);
        register(redoAction);

        cutAction = ActionFactory.CUT.create(window);
        register(cutAction);

        copyAction = ActionFactory.COPY.create(window);
        register(copyAction);

        pasteAction = ActionFactory.PASTE.create(window);
        register(pasteAction);

        deleteAction = ActionFactory.DELETE.create(window);
        register(deleteAction);

        selectAllAction = ActionFactory.SELECT_ALL.create(window);
        register(selectAllAction);

        preferencesAction = ActionFactory.PREFERENCES.create(window);
        register(preferencesAction);

        aboutAction = ActionFactory.ABOUT.create(window);
        register(aboutAction);
    }

    @Override
    protected void fillMenuBar(IMenuManager menuBar) {
        // Note: Most menu contributions are done via plugin.xml
        // This method can be used for programmatic menu additions

        // Add standard Edit menu additions marker
        MenuManager editMenu = new MenuManager("&Edit", IWorkbenchActionConstants.M_EDIT);
        editMenu.add(new Separator(IWorkbenchActionConstants.EDIT_START));
        editMenu.add(undoAction);
        editMenu.add(redoAction);
        editMenu.add(new Separator());
        editMenu.add(cutAction);
        editMenu.add(copyAction);
        editMenu.add(pasteAction);
        editMenu.add(new Separator());
        editMenu.add(deleteAction);
        editMenu.add(selectAllAction);
        editMenu.add(new Separator(IWorkbenchActionConstants.EDIT_END));

        // Add additions marker for extensibility
        menuBar.add(new Separator(IWorkbenchActionConstants.MB_ADDITIONS));
    }

    @Override
    protected void fillCoolBar(ICoolBarManager coolBar) {
        // Toolbar contributions are handled via plugin.xml
        coolBar.add(new Separator(IWorkbenchActionConstants.MB_ADDITIONS));
    }

    @Override
    protected void fillStatusLine(IStatusLineManager statusLine) {
        // Add a status line contribution item
        statusItem = new StatusLineContributionItem("testapp.status");
        statusItem.setText("Ready");
        statusLine.add(statusItem);
    }

    @Override
    public void dispose() {
        // Dispose of actions
        if (exitAction != null) exitAction.dispose();
        if (saveAction != null) saveAction.dispose();
        if (saveAllAction != null) saveAllAction.dispose();
        if (undoAction != null) undoAction.dispose();
        if (redoAction != null) redoAction.dispose();
        if (cutAction != null) cutAction.dispose();
        if (copyAction != null) copyAction.dispose();
        if (pasteAction != null) pasteAction.dispose();
        if (deleteAction != null) deleteAction.dispose();
        if (selectAllAction != null) selectAllAction.dispose();
        if (preferencesAction != null) preferencesAction.dispose();
        if (aboutAction != null) aboutAction.dispose();

        super.dispose();
    }
}
