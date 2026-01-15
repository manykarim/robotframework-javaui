package testapp;

import org.eclipse.swt.SWT;
import org.eclipse.swt.browser.Browser;
import org.eclipse.swt.custom.CCombo;
import org.eclipse.swt.custom.CTabFolder;
import org.eclipse.swt.custom.CTabItem;
import org.eclipse.swt.custom.StyledText;
import org.eclipse.swt.events.*;
import org.eclipse.swt.graphics.Color;
import org.eclipse.swt.graphics.Font;
import org.eclipse.swt.graphics.Image;
import org.eclipse.swt.layout.*;
import org.eclipse.swt.widgets.*;

/**
 * Comprehensive SWT Test Application for testing Robot Framework SwtLibrary keywords.
 *
 * This application includes all major SWT widgets organized in a logical layout with:
 * - Data names set via setData("name", "componentName") for locator support
 * - Tooltips on toolbar items
 * - Menu bar with File, Edit, and Help menus
 * - Table with 5+ rows and 3+ columns
 * - Tree with 3 levels of hierarchy
 * - Tab folder with 3+ tabs
 * - Dialog shell support
 * - Right-click context menus
 * - Configurable TCP port for SWT agent connection
 *
 * Usage: java -javaagent:robotframework-swt-agent.jar=port=18081 -jar swt-test-app.jar [port]
 */
public class SwtTestApp {

    private Display display;
    private Shell shell;
    private Text statusText;
    private ProgressBar progressBar;
    private Table dataTable;
    private Tree fileTree;
    private StyledText styledTextEditor;
    private Browser browser;
    private int agentPort = 18081;

    public static void main(String[] args) {
        // Parse command line arguments for port
        int port = 18081;
        if (args.length > 0) {
            try {
                port = Integer.parseInt(args[0]);
            } catch (NumberFormatException e) {
                System.err.println("Invalid port number, using default: " + port);
            }
        }

        SwtTestApp app = new SwtTestApp();
        app.agentPort = port;
        app.run();
    }

    public void run() {
        display = new Display();
        createMainShell();

        shell.open();
        System.out.println("[SwtTestApp] Application started. Agent port: " + agentPort);
        System.out.println("[SwtTestApp] Main shell opened: " + shell.getText());

        while (!shell.isDisposed()) {
            if (!display.readAndDispatch()) {
                display.sleep();
            }
        }

        display.dispose();
        System.out.println("[SwtTestApp] Application terminated.");
    }

    private void createMainShell() {
        shell = new Shell(display, SWT.SHELL_TRIM);
        shell.setText("SWT Test Application");
        shell.setSize(1200, 800);
        shell.setData("name", "mainShell");

        // Set layout
        GridLayout mainLayout = new GridLayout(1, false);
        mainLayout.marginWidth = 0;
        mainLayout.marginHeight = 0;
        shell.setLayout(mainLayout);

        // Create all UI components
        createMenuBar();
        createCoolBar();
        createToolBar();
        createMainContent();
        createStatusBar();

        // Center the shell on screen
        centerShell(shell);
    }

    private void createMenuBar() {
        Menu menuBar = new Menu(shell, SWT.BAR);
        shell.setMenuBar(menuBar);
        menuBar.setData("name", "mainMenuBar");

        // File menu
        MenuItem fileMenuHeader = new MenuItem(menuBar, SWT.CASCADE);
        fileMenuHeader.setText("&File");
        fileMenuHeader.setData("name", "fileMenuHeader");

        Menu fileMenu = new Menu(shell, SWT.DROP_DOWN);
        fileMenuHeader.setMenu(fileMenu);
        fileMenu.setData("name", "fileMenu");

        MenuItem newItem = new MenuItem(fileMenu, SWT.PUSH);
        newItem.setText("&New\tCtrl+N");
        newItem.setAccelerator(SWT.CTRL + 'N');
        newItem.setData("name", "menuNew");
        newItem.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                updateStatus("New file created");
            }
        });

        MenuItem openItem = new MenuItem(fileMenu, SWT.PUSH);
        openItem.setText("&Open...\tCtrl+O");
        openItem.setAccelerator(SWT.CTRL + 'O');
        openItem.setData("name", "menuOpen");
        openItem.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                openFileDialog();
            }
        });

        MenuItem saveItem = new MenuItem(fileMenu, SWT.PUSH);
        saveItem.setText("&Save\tCtrl+S");
        saveItem.setAccelerator(SWT.CTRL + 'S');
        saveItem.setData("name", "menuSave");
        saveItem.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                updateStatus("File saved");
            }
        });

        MenuItem saveAsItem = new MenuItem(fileMenu, SWT.PUSH);
        saveAsItem.setText("Save &As...");
        saveAsItem.setData("name", "menuSaveAs");

        new MenuItem(fileMenu, SWT.SEPARATOR);

        // Recent files submenu
        MenuItem recentMenuItem = new MenuItem(fileMenu, SWT.CASCADE);
        recentMenuItem.setText("Recent Files");
        recentMenuItem.setData("name", "menuRecent");

        Menu recentMenu = new Menu(shell, SWT.DROP_DOWN);
        recentMenuItem.setMenu(recentMenu);

        for (int i = 1; i <= 5; i++) {
            MenuItem recentFile = new MenuItem(recentMenu, SWT.PUSH);
            recentFile.setText("File " + i + ".txt");
            recentFile.setData("name", "recentFile" + i);
        }

        new MenuItem(fileMenu, SWT.SEPARATOR);

        MenuItem exitItem = new MenuItem(fileMenu, SWT.PUSH);
        exitItem.setText("E&xit\tAlt+F4");
        exitItem.setData("name", "menuExit");
        exitItem.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                shell.close();
            }
        });

        // Edit menu
        MenuItem editMenuHeader = new MenuItem(menuBar, SWT.CASCADE);
        editMenuHeader.setText("&Edit");
        editMenuHeader.setData("name", "editMenuHeader");

        Menu editMenu = new Menu(shell, SWT.DROP_DOWN);
        editMenuHeader.setMenu(editMenu);
        editMenu.setData("name", "editMenu");

        MenuItem undoItem = new MenuItem(editMenu, SWT.PUSH);
        undoItem.setText("&Undo\tCtrl+Z");
        undoItem.setAccelerator(SWT.CTRL + 'Z');
        undoItem.setData("name", "menuUndo");

        MenuItem redoItem = new MenuItem(editMenu, SWT.PUSH);
        redoItem.setText("&Redo\tCtrl+Y");
        redoItem.setAccelerator(SWT.CTRL + 'Y');
        redoItem.setData("name", "menuRedo");

        new MenuItem(editMenu, SWT.SEPARATOR);

        MenuItem cutItem = new MenuItem(editMenu, SWT.PUSH);
        cutItem.setText("Cu&t\tCtrl+X");
        cutItem.setAccelerator(SWT.CTRL + 'X');
        cutItem.setData("name", "menuCut");

        MenuItem copyItem = new MenuItem(editMenu, SWT.PUSH);
        copyItem.setText("&Copy\tCtrl+C");
        copyItem.setAccelerator(SWT.CTRL + 'C');
        copyItem.setData("name", "menuCopy");

        MenuItem pasteItem = new MenuItem(editMenu, SWT.PUSH);
        pasteItem.setText("&Paste\tCtrl+V");
        pasteItem.setAccelerator(SWT.CTRL + 'V');
        pasteItem.setData("name", "menuPaste");

        new MenuItem(editMenu, SWT.SEPARATOR);

        MenuItem selectAllItem = new MenuItem(editMenu, SWT.PUSH);
        selectAllItem.setText("Select &All\tCtrl+A");
        selectAllItem.setAccelerator(SWT.CTRL + 'A');
        selectAllItem.setData("name", "menuSelectAll");

        new MenuItem(editMenu, SWT.SEPARATOR);

        MenuItem preferencesItem = new MenuItem(editMenu, SWT.PUSH);
        preferencesItem.setText("&Preferences...");
        preferencesItem.setData("name", "menuPreferences");
        preferencesItem.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                showPreferencesDialog();
            }
        });

        // View menu
        MenuItem viewMenuHeader = new MenuItem(menuBar, SWT.CASCADE);
        viewMenuHeader.setText("&View");
        viewMenuHeader.setData("name", "viewMenuHeader");

        Menu viewMenu = new Menu(shell, SWT.DROP_DOWN);
        viewMenuHeader.setMenu(viewMenu);
        viewMenu.setData("name", "viewMenu");

        MenuItem showToolbarItem = new MenuItem(viewMenu, SWT.CHECK);
        showToolbarItem.setText("Show &Toolbar");
        showToolbarItem.setSelection(true);
        showToolbarItem.setData("name", "menuShowToolbar");

        MenuItem showStatusItem = new MenuItem(viewMenu, SWT.CHECK);
        showStatusItem.setText("Show &Status Bar");
        showStatusItem.setSelection(true);
        showStatusItem.setData("name", "menuShowStatus");

        new MenuItem(viewMenu, SWT.SEPARATOR);

        // Radio menu items for view mode
        MenuItem listViewItem = new MenuItem(viewMenu, SWT.RADIO);
        listViewItem.setText("&List View");
        listViewItem.setSelection(true);
        listViewItem.setData("name", "menuListView");

        MenuItem detailViewItem = new MenuItem(viewMenu, SWT.RADIO);
        detailViewItem.setText("&Detail View");
        detailViewItem.setData("name", "menuDetailView");

        MenuItem iconViewItem = new MenuItem(viewMenu, SWT.RADIO);
        iconViewItem.setText("&Icon View");
        iconViewItem.setData("name", "menuIconView");

        // Help menu
        MenuItem helpMenuHeader = new MenuItem(menuBar, SWT.CASCADE);
        helpMenuHeader.setText("&Help");
        helpMenuHeader.setData("name", "helpMenuHeader");

        Menu helpMenu = new Menu(shell, SWT.DROP_DOWN);
        helpMenuHeader.setMenu(helpMenu);
        helpMenu.setData("name", "helpMenu");

        MenuItem helpContentsItem = new MenuItem(helpMenu, SWT.PUSH);
        helpContentsItem.setText("Help &Contents\tF1");
        helpContentsItem.setAccelerator(SWT.F1);
        helpContentsItem.setData("name", "menuHelpContents");

        new MenuItem(helpMenu, SWT.SEPARATOR);

        MenuItem aboutItem = new MenuItem(helpMenu, SWT.PUSH);
        aboutItem.setText("&About");
        aboutItem.setData("name", "menuAbout");
        aboutItem.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                showAboutDialog();
            }
        });
    }

    private void createCoolBar() {
        CoolBar coolBar = new CoolBar(shell, SWT.FLAT);
        coolBar.setLayoutData(new GridData(SWT.FILL, SWT.TOP, true, false));
        coolBar.setData("name", "mainCoolBar");

        // First CoolItem with file operations
        CoolItem fileCoolItem = new CoolItem(coolBar, SWT.NONE);
        fileCoolItem.setData("name", "fileCoolItem");

        ToolBar fileToolBar = new ToolBar(coolBar, SWT.FLAT);
        fileToolBar.setData("name", "fileToolBar");

        ToolItem newTool = new ToolItem(fileToolBar, SWT.PUSH);
        newTool.setText("New");
        newTool.setToolTipText("Create new file (Ctrl+N)");
        newTool.setData("name", "toolNew");

        ToolItem openTool = new ToolItem(fileToolBar, SWT.PUSH);
        openTool.setText("Open");
        openTool.setToolTipText("Open existing file (Ctrl+O)");
        openTool.setData("name", "toolOpen");

        ToolItem saveTool = new ToolItem(fileToolBar, SWT.PUSH);
        saveTool.setText("Save");
        saveTool.setToolTipText("Save current file (Ctrl+S)");
        saveTool.setData("name", "toolSave");

        fileToolBar.pack();
        fileCoolItem.setControl(fileToolBar);
        fileCoolItem.setSize(fileCoolItem.computeSize(SWT.DEFAULT, SWT.DEFAULT));

        // Second CoolItem with edit operations
        CoolItem editCoolItem = new CoolItem(coolBar, SWT.NONE);
        editCoolItem.setData("name", "editCoolItem");

        ToolBar editToolBar = new ToolBar(coolBar, SWT.FLAT);
        editToolBar.setData("name", "editToolBar");

        ToolItem cutTool = new ToolItem(editToolBar, SWT.PUSH);
        cutTool.setText("Cut");
        cutTool.setToolTipText("Cut selection (Ctrl+X)");
        cutTool.setData("name", "toolCut");

        ToolItem copyTool = new ToolItem(editToolBar, SWT.PUSH);
        copyTool.setText("Copy");
        copyTool.setToolTipText("Copy selection (Ctrl+C)");
        copyTool.setData("name", "toolCopy");

        ToolItem pasteTool = new ToolItem(editToolBar, SWT.PUSH);
        pasteTool.setText("Paste");
        pasteTool.setToolTipText("Paste from clipboard (Ctrl+V)");
        pasteTool.setData("name", "toolPaste");

        editToolBar.pack();
        editCoolItem.setControl(editToolBar);
        editCoolItem.setSize(editCoolItem.computeSize(SWT.DEFAULT, SWT.DEFAULT));
    }

    private void createToolBar() {
        ToolBar toolBar = new ToolBar(shell, SWT.FLAT | SWT.WRAP);
        toolBar.setLayoutData(new GridData(SWT.FILL, SWT.TOP, true, false));
        toolBar.setData("name", "mainToolBar");

        // Various toolbar items with different styles
        ToolItem runTool = new ToolItem(toolBar, SWT.PUSH);
        runTool.setText("Run");
        runTool.setToolTipText("Run the current script");
        runTool.setData("name", "toolRun");
        runTool.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                runProgressDemo();
            }
        });

        ToolItem stopTool = new ToolItem(toolBar, SWT.PUSH);
        stopTool.setText("Stop");
        stopTool.setToolTipText("Stop execution");
        stopTool.setData("name", "toolStop");

        new ToolItem(toolBar, SWT.SEPARATOR);

        // Dropdown tool item
        ToolItem dropDownTool = new ToolItem(toolBar, SWT.DROP_DOWN);
        dropDownTool.setText("Options");
        dropDownTool.setToolTipText("Additional options");
        dropDownTool.setData("name", "toolOptions");

        Menu dropDownMenu = new Menu(shell, SWT.POP_UP);
        MenuItem opt1 = new MenuItem(dropDownMenu, SWT.PUSH);
        opt1.setText("Option 1");
        opt1.setData("name", "dropdownOption1");
        MenuItem opt2 = new MenuItem(dropDownMenu, SWT.PUSH);
        opt2.setText("Option 2");
        opt2.setData("name", "dropdownOption2");
        MenuItem opt3 = new MenuItem(dropDownMenu, SWT.PUSH);
        opt3.setText("Option 3");
        opt3.setData("name", "dropdownOption3");

        dropDownTool.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                if (e.detail == SWT.ARROW) {
                    ToolItem item = (ToolItem) e.widget;
                    org.eclipse.swt.graphics.Rectangle rect = item.getBounds();
                    org.eclipse.swt.graphics.Point pt = item.getParent().toDisplay(rect.x, rect.y + rect.height);
                    dropDownMenu.setLocation(pt);
                    dropDownMenu.setVisible(true);
                }
            }
        });

        new ToolItem(toolBar, SWT.SEPARATOR);

        // Check style tool items
        ToolItem checkTool1 = new ToolItem(toolBar, SWT.CHECK);
        checkTool1.setText("Bold");
        checkTool1.setToolTipText("Toggle bold text");
        checkTool1.setData("name", "toolBold");

        ToolItem checkTool2 = new ToolItem(toolBar, SWT.CHECK);
        checkTool2.setText("Italic");
        checkTool2.setToolTipText("Toggle italic text");
        checkTool2.setData("name", "toolItalic");

        new ToolItem(toolBar, SWT.SEPARATOR);

        // Radio style tool items
        ToolItem radio1 = new ToolItem(toolBar, SWT.RADIO);
        radio1.setText("Left");
        radio1.setToolTipText("Align left");
        radio1.setSelection(true);
        radio1.setData("name", "toolAlignLeft");

        ToolItem radio2 = new ToolItem(toolBar, SWT.RADIO);
        radio2.setText("Center");
        radio2.setToolTipText("Align center");
        radio2.setData("name", "toolAlignCenter");

        ToolItem radio3 = new ToolItem(toolBar, SWT.RADIO);
        radio3.setText("Right");
        radio3.setToolTipText("Align right");
        radio3.setData("name", "toolAlignRight");

        new ToolItem(toolBar, SWT.SEPARATOR);

        // Dialog buttons
        ToolItem dialogTool = new ToolItem(toolBar, SWT.PUSH);
        dialogTool.setText("Dialog");
        dialogTool.setToolTipText("Open test dialog");
        dialogTool.setData("name", "toolDialog");
        dialogTool.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                showTestDialog();
            }
        });

        ToolItem messageTool = new ToolItem(toolBar, SWT.PUSH);
        messageTool.setText("Message");
        messageTool.setToolTipText("Show message box");
        messageTool.setData("name", "toolMessage");
        messageTool.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                showMessageDialog();
            }
        });
    }

    private void createMainContent() {
        // Create a SashForm for resizable panes
        Composite mainComposite = new Composite(shell, SWT.NONE);
        mainComposite.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
        mainComposite.setLayout(new FillLayout());
        mainComposite.setData("name", "mainComposite");

        org.eclipse.swt.custom.SashForm sashForm = new org.eclipse.swt.custom.SashForm(mainComposite, SWT.HORIZONTAL);
        sashForm.setData("name", "mainSashForm");

        // Left panel - Tree navigation
        createLeftPanel(sashForm);

        // Center panel - Tab folder with main content
        createCenterPanel(sashForm);

        // Right panel - Properties/Details
        createRightPanel(sashForm);

        // Set sash weights
        sashForm.setWeights(new int[]{20, 55, 25});
    }

    private void createLeftPanel(Composite parent) {
        Group treeGroup = new Group(parent, SWT.NONE);
        treeGroup.setText("File Navigator");
        treeGroup.setLayout(new FillLayout());
        treeGroup.setData("name", "fileNavigatorGroup");

        fileTree = new Tree(treeGroup, SWT.BORDER | SWT.MULTI | SWT.V_SCROLL | SWT.H_SCROLL);
        fileTree.setData("name", "fileTree");

        // Create tree with 3 levels of hierarchy
        TreeItem root1 = new TreeItem(fileTree, SWT.NONE);
        root1.setText("Project A");
        root1.setData("name", "treeProjectA");

        TreeItem src1 = new TreeItem(root1, SWT.NONE);
        src1.setText("src");
        src1.setData("name", "treeSrc1");

        TreeItem main1 = new TreeItem(src1, SWT.NONE);
        main1.setText("main");
        main1.setData("name", "treeMain1");

        TreeItem java1 = new TreeItem(main1, SWT.NONE);
        java1.setText("java");
        java1.setData("name", "treeJava1");

        TreeItem pkg1 = new TreeItem(java1, SWT.NONE);
        pkg1.setText("com.example");
        pkg1.setData("name", "treePkg1");

        TreeItem class1 = new TreeItem(pkg1, SWT.NONE);
        class1.setText("Main.java");
        class1.setData("name", "treeMainJava");

        TreeItem class2 = new TreeItem(pkg1, SWT.NONE);
        class2.setText("Utils.java");
        class2.setData("name", "treeUtilsJava");

        TreeItem test1 = new TreeItem(main1, SWT.NONE);
        test1.setText("test");
        test1.setData("name", "treeTest1");

        TreeItem resources1 = new TreeItem(src1, SWT.NONE);
        resources1.setText("resources");
        resources1.setData("name", "treeResources1");

        TreeItem config1 = new TreeItem(resources1, SWT.NONE);
        config1.setText("config.xml");
        config1.setData("name", "treeConfigXml");

        TreeItem root2 = new TreeItem(fileTree, SWT.NONE);
        root2.setText("Project B");
        root2.setData("name", "treeProjectB");

        TreeItem docs2 = new TreeItem(root2, SWT.NONE);
        docs2.setText("docs");
        docs2.setData("name", "treeDocs2");

        TreeItem readme = new TreeItem(docs2, SWT.NONE);
        readme.setText("README.md");
        readme.setData("name", "treeReadme");

        TreeItem api = new TreeItem(docs2, SWT.NONE);
        api.setText("API.md");
        api.setData("name", "treeApiMd");

        TreeItem root3 = new TreeItem(fileTree, SWT.NONE);
        root3.setText("Libraries");
        root3.setData("name", "treeLibraries");

        TreeItem lib1 = new TreeItem(root3, SWT.NONE);
        lib1.setText("commons-lang.jar");
        lib1.setData("name", "treeCommonsLang");

        TreeItem lib2 = new TreeItem(root3, SWT.NONE);
        lib2.setText("gson.jar");
        lib2.setData("name", "treeGson");

        TreeItem lib3 = new TreeItem(root3, SWT.NONE);
        lib3.setText("junit.jar");
        lib3.setData("name", "treeJunit");

        // Expand first level
        root1.setExpanded(true);
        src1.setExpanded(true);

        // Create context menu for tree
        Menu treeContextMenu = new Menu(fileTree);
        fileTree.setMenu(treeContextMenu);
        treeContextMenu.setData("name", "treeContextMenu");

        MenuItem treeRefresh = new MenuItem(treeContextMenu, SWT.PUSH);
        treeRefresh.setText("Refresh");
        treeRefresh.setData("name", "treeMenuRefresh");

        MenuItem treeExpand = new MenuItem(treeContextMenu, SWT.PUSH);
        treeExpand.setText("Expand All");
        treeExpand.setData("name", "treeMenuExpandAll");

        MenuItem treeCollapse = new MenuItem(treeContextMenu, SWT.PUSH);
        treeCollapse.setText("Collapse All");
        treeCollapse.setData("name", "treeMenuCollapseAll");

        new MenuItem(treeContextMenu, SWT.SEPARATOR);

        MenuItem treeDelete = new MenuItem(treeContextMenu, SWT.PUSH);
        treeDelete.setText("Delete");
        treeDelete.setData("name", "treeMenuDelete");

        MenuItem treeRename = new MenuItem(treeContextMenu, SWT.PUSH);
        treeRename.setText("Rename");
        treeRename.setData("name", "treeMenuRename");

        // Tree selection listener
        fileTree.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                TreeItem[] selection = fileTree.getSelection();
                if (selection.length > 0) {
                    updateStatus("Selected: " + selection[0].getText());
                }
            }
        });
    }

    private void createCenterPanel(Composite parent) {
        TabFolder tabFolder = new TabFolder(parent, SWT.TOP);
        tabFolder.setData("name", "mainTabFolder");

        // Tab 1: Input Controls
        TabItem inputTab = new TabItem(tabFolder, SWT.NONE);
        inputTab.setText("Input Controls");
        inputTab.setData("name", "tabInput");
        inputTab.setControl(createInputControlsTab(tabFolder));

        // Tab 2: Data Table
        TabItem tableTab = new TabItem(tabFolder, SWT.NONE);
        tableTab.setText("Data Table");
        tableTab.setData("name", "tabTable");
        tableTab.setControl(createTableTab(tabFolder));

        // Tab 3: Editor
        TabItem editorTab = new TabItem(tabFolder, SWT.NONE);
        editorTab.setText("Editor");
        editorTab.setData("name", "tabEditor");
        editorTab.setControl(createEditorTab(tabFolder));

        // Tab 4: Browser
        TabItem browserTab = new TabItem(tabFolder, SWT.NONE);
        browserTab.setText("Browser");
        browserTab.setData("name", "tabBrowser");
        browserTab.setControl(createBrowserTab(tabFolder));

        // Tab 5: Advanced Controls
        TabItem advancedTab = new TabItem(tabFolder, SWT.NONE);
        advancedTab.setText("Advanced");
        advancedTab.setData("name", "tabAdvanced");
        advancedTab.setControl(createAdvancedTab(tabFolder));

        tabFolder.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                TabItem selected = tabFolder.getSelection()[0];
                updateStatus("Tab selected: " + selected.getText());
            }
        });
    }

    private Composite createInputControlsTab(Composite parent) {
        Composite composite = new Composite(parent, SWT.NONE);
        composite.setLayout(new GridLayout(2, false));
        composite.setData("name", "inputControlsComposite");

        // Single-line Text
        Label textLabel = new Label(composite, SWT.NONE);
        textLabel.setText("Username:");
        textLabel.setData("name", "labelUsername");

        Text textField = new Text(composite, SWT.BORDER | SWT.SINGLE);
        textField.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        textField.setData("name", "textUsername");
        textField.setMessage("Enter username");

        // Password Text
        Label passwordLabel = new Label(composite, SWT.NONE);
        passwordLabel.setText("Password:");
        passwordLabel.setData("name", "labelPassword");

        Text passwordField = new Text(composite, SWT.BORDER | SWT.SINGLE | SWT.PASSWORD);
        passwordField.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        passwordField.setData("name", "textPassword");
        passwordField.setMessage("Enter password");

        // Multi-line Text
        Label multiLabel = new Label(composite, SWT.NONE);
        multiLabel.setText("Description:");
        multiLabel.setData("name", "labelDescription");
        multiLabel.setLayoutData(new GridData(SWT.LEFT, SWT.TOP, false, false));

        Text multiText = new Text(composite, SWT.BORDER | SWT.MULTI | SWT.V_SCROLL | SWT.WRAP);
        GridData multiData = new GridData(SWT.FILL, SWT.FILL, true, false);
        multiData.heightHint = 60;
        multiText.setLayoutData(multiData);
        multiText.setData("name", "textDescription");
        multiText.setText("Enter multi-line description here...");

        // Regular Combo (editable)
        Label comboLabel = new Label(composite, SWT.NONE);
        comboLabel.setText("Category:");
        comboLabel.setData("name", "labelCategory");

        Combo combo = new Combo(composite, SWT.DROP_DOWN);
        combo.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        combo.setData("name", "comboCategory");
        combo.setItems(new String[]{"Development", "Testing", "Documentation", "Research", "Other", "Item with & special < chars >"});
        combo.select(0);

        // Read-only Combo
        Label roComboLabel = new Label(composite, SWT.NONE);
        roComboLabel.setText("Priority:");
        roComboLabel.setData("name", "labelPriority");

        Combo roCombo = new Combo(composite, SWT.DROP_DOWN | SWT.READ_ONLY);
        roCombo.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        roCombo.setData("name", "comboPriority");
        roCombo.setItems(new String[]{"Low", "Medium", "High", "Critical"});
        roCombo.select(1);

        // CCombo (custom combo)
        Label ccomboLabel = new Label(composite, SWT.NONE);
        ccomboLabel.setText("Status:");
        ccomboLabel.setData("name", "labelStatus");

        CCombo ccombo = new CCombo(composite, SWT.BORDER);
        ccombo.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        ccombo.setData("name", "ccomboStatus");
        ccombo.setItems(new String[]{"Development", "Testing", "Documentation", "Other"});
        ccombo.select(0);

        // List (single selection)
        Label listLabel = new Label(composite, SWT.NONE);
        listLabel.setText("Tags:");
        listLabel.setData("name", "labelTags");
        listLabel.setLayoutData(new GridData(SWT.LEFT, SWT.TOP, false, false));

        org.eclipse.swt.widgets.List list = new org.eclipse.swt.widgets.List(composite, SWT.BORDER | SWT.SINGLE | SWT.V_SCROLL);
        GridData listData = new GridData(SWT.FILL, SWT.FILL, true, false);
        listData.heightHint = 80;
        list.setLayoutData(listData);
        list.setData("name", "listTags");
        list.setItems(new String[]{"Bug", "Feature", "Enhancement", "Refactor", "Documentation", "Performance", "Security"});
        list.select(0);

        // Multi-select List
        Label multiListLabel = new Label(composite, SWT.NONE);
        multiListLabel.setText("Assignees:");
        multiListLabel.setData("name", "labelAssignees");
        multiListLabel.setLayoutData(new GridData(SWT.LEFT, SWT.TOP, false, false));

        org.eclipse.swt.widgets.List multiList = new org.eclipse.swt.widgets.List(composite, SWT.BORDER | SWT.MULTI | SWT.V_SCROLL);
        GridData multiListData = new GridData(SWT.FILL, SWT.FILL, true, false);
        multiListData.heightHint = 80;
        multiList.setLayoutData(multiListData);
        multiList.setData("name", "listAssignees");
        multiList.setItems(new String[]{"Alice", "Bob", "Charlie", "Diana", "Eve", "Frank"});

        // Buttons section
        Group buttonGroup = new Group(composite, SWT.NONE);
        buttonGroup.setText("Buttons");
        buttonGroup.setData("name", "buttonGroup");
        GridData buttonGroupData = new GridData(SWT.FILL, SWT.CENTER, true, false);
        buttonGroupData.horizontalSpan = 2;
        buttonGroup.setLayoutData(buttonGroupData);
        buttonGroup.setLayout(new GridLayout(4, false));

        Button pushButton = new Button(buttonGroup, SWT.PUSH);
        pushButton.setText("Submit");
        pushButton.setData("name", "buttonSubmit");
        pushButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                updateStatus("Submit button clicked");
            }
        });

        Button cancelButton = new Button(buttonGroup, SWT.PUSH);
        cancelButton.setText("Cancel");
        cancelButton.setData("name", "buttonCancel");

        Button toggleButton = new Button(buttonGroup, SWT.TOGGLE);
        toggleButton.setText("Toggle");
        toggleButton.setData("name", "buttonToggle");

        Button arrowButton = new Button(buttonGroup, SWT.ARROW | SWT.DOWN);
        arrowButton.setData("name", "buttonArrow");

        Button openDialogButton = new Button(buttonGroup, SWT.PUSH);
        openDialogButton.setText("Open Dialog");
        openDialogButton.setData("name", "buttonOpenDialog");
        openDialogButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                showTestDialog();
            }
        });

        Button preferencesButton = new Button(buttonGroup, SWT.PUSH);
        preferencesButton.setText("Preferences");
        preferencesButton.setData("name", "buttonPreferences");
        preferencesButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                showPreferencesDialog();
            }
        });

        // Checkboxes
        Group checkGroup = new Group(composite, SWT.NONE);
        checkGroup.setText("Options");
        checkGroup.setData("name", "optionsGroup");
        GridData checkGroupData = new GridData(SWT.FILL, SWT.CENTER, true, false);
        checkGroupData.horizontalSpan = 2;
        checkGroup.setLayoutData(checkGroupData);
        checkGroup.setLayout(new RowLayout(SWT.HORIZONTAL));

        Button check1 = new Button(checkGroup, SWT.CHECK);
        check1.setText("Enable Notifications");
        check1.setSelection(true);
        check1.setData("name", "checkNotifications");

        Button check2 = new Button(checkGroup, SWT.CHECK);
        check2.setText("Auto-save");
        check2.setData("name", "checkAutoSave");

        Button check3 = new Button(checkGroup, SWT.CHECK);
        check3.setText("Debug Mode");
        check3.setData("name", "checkDebugMode");

        // Radio buttons
        Group radioGroup = new Group(composite, SWT.NONE);
        radioGroup.setText("Theme");
        radioGroup.setData("name", "themeGroup");
        GridData radioGroupData = new GridData(SWT.FILL, SWT.CENTER, true, false);
        radioGroupData.horizontalSpan = 2;
        radioGroup.setLayoutData(radioGroupData);
        radioGroup.setLayout(new RowLayout(SWT.HORIZONTAL));

        Button radio1 = new Button(radioGroup, SWT.RADIO);
        radio1.setText("Light");
        radio1.setSelection(true);
        radio1.setData("name", "radioLight");

        Button radio2 = new Button(radioGroup, SWT.RADIO);
        radio2.setText("Dark");
        radio2.setData("name", "radioDark");

        Button radio3 = new Button(radioGroup, SWT.RADIO);
        radio3.setText("System");
        radio3.setData("name", "radioSystem");

        // Link control
        Label linkLabel = new Label(composite, SWT.NONE);
        linkLabel.setText("Help:");
        linkLabel.setData("name", "labelHelp");

        Link link = new Link(composite, SWT.NONE);
        link.setText("Visit our <a href=\"https://example.com\">documentation</a> or <a href=\"mailto:support@example.com\">contact support</a>.");
        link.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        link.setData("name", "linkHelp");
        link.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                updateStatus("Link clicked: " + e.text);
            }
        });

        return composite;
    }

    private Composite createTableTab(Composite parent) {
        Composite composite = new Composite(parent, SWT.NONE);
        composite.setLayout(new GridLayout(1, false));
        composite.setData("name", "tableComposite");

        // Search bar
        Composite searchComposite = new Composite(composite, SWT.NONE);
        searchComposite.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        searchComposite.setLayout(new GridLayout(3, false));
        searchComposite.setData("name", "searchComposite");

        Label searchLabel = new Label(searchComposite, SWT.NONE);
        searchLabel.setText("Search:");
        searchLabel.setData("name", "labelSearch");

        Text searchText = new Text(searchComposite, SWT.BORDER | SWT.SEARCH);
        searchText.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        searchText.setData("name", "textSearch");
        searchText.setMessage("Filter table...");

        Button searchButton = new Button(searchComposite, SWT.PUSH);
        searchButton.setText("Search");
        searchButton.setData("name", "buttonSearch");

        // Data table
        dataTable = new Table(composite, SWT.BORDER | SWT.FULL_SELECTION | SWT.MULTI | SWT.V_SCROLL | SWT.H_SCROLL);
        dataTable.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
        dataTable.setHeaderVisible(true);
        dataTable.setLinesVisible(true);
        dataTable.setData("name", "dataTable");

        // Create columns
        TableColumn idColumn = new TableColumn(dataTable, SWT.LEFT);
        idColumn.setText("ID");
        idColumn.setWidth(60);
        idColumn.setData("name", "columnId");

        TableColumn nameColumn = new TableColumn(dataTable, SWT.LEFT);
        nameColumn.setText("Name");
        nameColumn.setWidth(150);
        nameColumn.setData("name", "columnName");

        TableColumn statusColumn = new TableColumn(dataTable, SWT.LEFT);
        statusColumn.setText("Status");
        statusColumn.setWidth(100);
        statusColumn.setData("name", "columnStatus");

        TableColumn dateColumn = new TableColumn(dataTable, SWT.LEFT);
        dateColumn.setText("Date");
        dateColumn.setWidth(100);
        dateColumn.setData("name", "columnDate");

        TableColumn priorityColumn = new TableColumn(dataTable, SWT.LEFT);
        priorityColumn.setText("Priority");
        priorityColumn.setWidth(80);
        priorityColumn.setData("name", "columnPriority");

        // Add sample data (at least 5 rows)
        String[][] tableData = {
            {"1", "Authentication Module", "Complete", "2024-01-15", "High"},
            {"2", "User Dashboard", "In Progress", "2024-01-20", "Medium"},
            {"3", "API Integration", "Pending", "2024-02-01", "High"},
            {"4", "Database Migration", "Complete", "2024-01-10", "Critical"},
            {"5", "Performance Tests", "In Progress", "2024-01-25", "Medium"},
            {"6", "Security Audit", "Pending", "2024-02-15", "Critical"},
            {"7", "Documentation", "Complete", "2024-01-05", "Low"},
            {"8", "UI Redesign", "In Progress", "2024-02-10", "Medium"},
            {"9", "Bug Fixes", "In Progress", "2024-01-18", "High"},
            {"10", "Release Prep", "Pending", "2024-03-01", "Critical"}
        };

        for (int i = 0; i < tableData.length; i++) {
            TableItem item = new TableItem(dataTable, SWT.NONE);
            item.setText(tableData[i]);
            item.setData("name", "tableRow" + (i + 1));
        }

        // Table context menu
        Menu tableContextMenu = new Menu(dataTable);
        dataTable.setMenu(tableContextMenu);
        tableContextMenu.setData("name", "tableContextMenu");

        MenuItem tableView = new MenuItem(tableContextMenu, SWT.PUSH);
        tableView.setText("View Details");
        tableView.setData("name", "tableMenuView");

        MenuItem tableEdit = new MenuItem(tableContextMenu, SWT.PUSH);
        tableEdit.setText("Edit");
        tableEdit.setData("name", "tableMenuEdit");

        new MenuItem(tableContextMenu, SWT.SEPARATOR);

        MenuItem tableDuplicate = new MenuItem(tableContextMenu, SWT.PUSH);
        tableDuplicate.setText("Duplicate");
        tableDuplicate.setData("name", "tableMenuDuplicate");

        MenuItem tableDelete = new MenuItem(tableContextMenu, SWT.PUSH);
        tableDelete.setText("Delete");
        tableDelete.setData("name", "tableMenuDelete");

        new MenuItem(tableContextMenu, SWT.SEPARATOR);

        MenuItem tableExport = new MenuItem(tableContextMenu, SWT.PUSH);
        tableExport.setText("Export...");
        tableExport.setData("name", "tableMenuExport");

        // Action buttons
        Composite buttonComposite = new Composite(composite, SWT.NONE);
        buttonComposite.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        buttonComposite.setLayout(new RowLayout(SWT.HORIZONTAL));
        buttonComposite.setData("name", "tableButtonComposite");

        Button addButton = new Button(buttonComposite, SWT.PUSH);
        addButton.setText("Add Row");
        addButton.setData("name", "buttonAddRow");
        addButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                addTableRow();
            }
        });

        Button deleteButton = new Button(buttonComposite, SWT.PUSH);
        deleteButton.setText("Delete Selected");
        deleteButton.setData("name", "buttonDeleteRow");

        Button refreshButton = new Button(buttonComposite, SWT.PUSH);
        refreshButton.setText("Refresh");
        refreshButton.setData("name", "buttonRefreshTable");

        Button exportButton = new Button(buttonComposite, SWT.PUSH);
        exportButton.setText("Export");
        exportButton.setData("name", "buttonExportTable");

        // Selection listener
        dataTable.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                TableItem[] selection = dataTable.getSelection();
                if (selection.length > 0) {
                    updateStatus("Selected row: " + selection[0].getText(1));
                }
            }
        });

        return composite;
    }

    private Composite createEditorTab(Composite parent) {
        Composite composite = new Composite(parent, SWT.NONE);
        composite.setLayout(new FillLayout());
        composite.setData("name", "editorComposite");

        styledTextEditor = new StyledText(composite, SWT.BORDER | SWT.MULTI | SWT.V_SCROLL | SWT.H_SCROLL);
        styledTextEditor.setData("name", "styledTextEditor");
        styledTextEditor.setText(
            "// Sample Code Editor\n" +
            "public class HelloWorld {\n" +
            "    public static void main(String[] args) {\n" +
            "        System.out.println(\"Hello, World!\");\n" +
            "    }\n" +
            "}\n\n" +
            "// Try editing this code...\n" +
            "// The StyledText widget supports rich text editing.\n"
        );

        // Set some basic styling
        Font monoFont = new Font(display, "Monospace", 12, SWT.NORMAL);
        styledTextEditor.setFont(monoFont);

        // Context menu for editor
        Menu editorMenu = new Menu(styledTextEditor);
        styledTextEditor.setMenu(editorMenu);
        editorMenu.setData("name", "editorContextMenu");

        MenuItem editorCut = new MenuItem(editorMenu, SWT.PUSH);
        editorCut.setText("Cut\tCtrl+X");
        editorCut.setData("name", "editorMenuCut");
        editorCut.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                styledTextEditor.cut();
            }
        });

        MenuItem editorCopy = new MenuItem(editorMenu, SWT.PUSH);
        editorCopy.setText("Copy\tCtrl+C");
        editorCopy.setData("name", "editorMenuCopy");
        editorCopy.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                styledTextEditor.copy();
            }
        });

        MenuItem editorPaste = new MenuItem(editorMenu, SWT.PUSH);
        editorPaste.setText("Paste\tCtrl+V");
        editorPaste.setData("name", "editorMenuPaste");
        editorPaste.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                styledTextEditor.paste();
            }
        });

        new MenuItem(editorMenu, SWT.SEPARATOR);

        MenuItem editorSelectAll = new MenuItem(editorMenu, SWT.PUSH);
        editorSelectAll.setText("Select All\tCtrl+A");
        editorSelectAll.setData("name", "editorMenuSelectAll");
        editorSelectAll.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                styledTextEditor.selectAll();
            }
        });

        new MenuItem(editorMenu, SWT.SEPARATOR);

        MenuItem editorFormat = new MenuItem(editorMenu, SWT.PUSH);
        editorFormat.setText("Format Code");
        editorFormat.setData("name", "editorMenuFormat");

        return composite;
    }

    private Composite createBrowserTab(Composite parent) {
        Composite composite = new Composite(parent, SWT.NONE);
        composite.setLayout(new GridLayout(1, false));
        composite.setData("name", "browserComposite");

        // URL bar
        Composite urlComposite = new Composite(composite, SWT.NONE);
        urlComposite.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        urlComposite.setLayout(new GridLayout(4, false));
        urlComposite.setData("name", "urlComposite");

        Button backButton = new Button(urlComposite, SWT.PUSH);
        backButton.setText("<");
        backButton.setToolTipText("Go back");
        backButton.setData("name", "buttonBrowserBack");

        Button forwardButton = new Button(urlComposite, SWT.PUSH);
        forwardButton.setText(">");
        forwardButton.setToolTipText("Go forward");
        forwardButton.setData("name", "buttonBrowserForward");

        Text urlText = new Text(urlComposite, SWT.BORDER | SWT.SINGLE);
        urlText.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        urlText.setData("name", "textBrowserUrl");
        urlText.setText("about:blank");

        Button goButton = new Button(urlComposite, SWT.PUSH);
        goButton.setText("Go");
        goButton.setData("name", "buttonBrowserGo");

        // Browser widget
        try {
            browser = new Browser(composite, SWT.BORDER);
            browser.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
            browser.setData("name", "browserWidget");
            browser.setText(
                "<html><body style='font-family: sans-serif; padding: 20px;'>" +
                "<h1>SWT Browser Widget</h1>" +
                "<p>This is an embedded browser component.</p>" +
                "<p>You can navigate to web pages or display HTML content.</p>" +
                "<ul>" +
                "<li>Click 'Go' to navigate to a URL</li>" +
                "<li>Use back/forward buttons for navigation</li>" +
                "<li>HTML content can be set programmatically</li>" +
                "</ul>" +
                "</body></html>"
            );

            // Wire up navigation
            backButton.addSelectionListener(new SelectionAdapter() {
                @Override
                public void widgetSelected(SelectionEvent e) {
                    browser.back();
                }
            });

            forwardButton.addSelectionListener(new SelectionAdapter() {
                @Override
                public void widgetSelected(SelectionEvent e) {
                    browser.forward();
                }
            });

            goButton.addSelectionListener(new SelectionAdapter() {
                @Override
                public void widgetSelected(SelectionEvent e) {
                    String url = urlText.getText();
                    if (!url.startsWith("http://") && !url.startsWith("https://")) {
                        url = "https://" + url;
                    }
                    browser.setUrl(url);
                }
            });

            urlText.addKeyListener(new KeyAdapter() {
                @Override
                public void keyPressed(KeyEvent e) {
                    if (e.character == SWT.CR) {
                        goButton.notifyListeners(SWT.Selection, new Event());
                    }
                }
            });

        } catch (Throwable e) {
            // Browser not available on this platform (catches SWTError and Exception)
            System.err.println("[SwtTestApp] Browser widget not available: " + e.getMessage());
            Label browserError = new Label(composite, SWT.WRAP);
            browserError.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
            browserError.setText("Browser widget not available on this platform.\n\nError: " + e.getMessage());
            browserError.setData("name", "browserErrorLabel");
        }

        return composite;
    }

    private Composite createAdvancedTab(Composite parent) {
        Composite composite = new Composite(parent, SWT.NONE);
        composite.setLayout(new GridLayout(2, true));
        composite.setData("name", "advancedComposite");

        // Scale controls
        Group scaleGroup = new Group(composite, SWT.NONE);
        scaleGroup.setText("Scale Controls");
        scaleGroup.setLayout(new GridLayout(2, false));
        scaleGroup.setLayoutData(new GridData(SWT.FILL, SWT.TOP, true, false));
        scaleGroup.setData("name", "scaleGroup");

        Label hScaleLabel = new Label(scaleGroup, SWT.NONE);
        hScaleLabel.setText("Horizontal:");
        hScaleLabel.setData("name", "labelHScale");

        Scale hScale = new Scale(scaleGroup, SWT.HORIZONTAL);
        hScale.setMinimum(0);
        hScale.setMaximum(100);
        hScale.setSelection(50);
        hScale.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        hScale.setData("name", "scaleHorizontal");

        Label vScaleLabel = new Label(scaleGroup, SWT.NONE);
        vScaleLabel.setText("Vertical:");
        vScaleLabel.setData("name", "labelVScale");

        Scale vScale = new Scale(scaleGroup, SWT.VERTICAL);
        vScale.setMinimum(0);
        vScale.setMaximum(100);
        vScale.setSelection(75);
        GridData vScaleData = new GridData(SWT.CENTER, SWT.FILL, false, false);
        vScaleData.heightHint = 80;
        vScale.setLayoutData(vScaleData);
        vScale.setData("name", "scaleVertical");

        // Spinner controls
        Group spinnerGroup = new Group(composite, SWT.NONE);
        spinnerGroup.setText("Spinner Controls");
        spinnerGroup.setLayout(new GridLayout(2, false));
        spinnerGroup.setLayoutData(new GridData(SWT.FILL, SWT.TOP, true, false));
        spinnerGroup.setData("name", "spinnerGroup");

        Label intSpinnerLabel = new Label(spinnerGroup, SWT.NONE);
        intSpinnerLabel.setText("Integer:");
        intSpinnerLabel.setData("name", "labelIntSpinner");

        Spinner intSpinner = new Spinner(spinnerGroup, SWT.BORDER);
        intSpinner.setMinimum(0);
        intSpinner.setMaximum(100);
        intSpinner.setSelection(25);
        intSpinner.setIncrement(1);
        intSpinner.setData("name", "spinnerInteger");

        Label decSpinnerLabel = new Label(spinnerGroup, SWT.NONE);
        decSpinnerLabel.setText("Decimal:");
        decSpinnerLabel.setData("name", "labelDecSpinner");

        Spinner decSpinner = new Spinner(spinnerGroup, SWT.BORDER);
        decSpinner.setMinimum(0);
        decSpinner.setMaximum(1000);
        decSpinner.setSelection(500);
        decSpinner.setDigits(2);
        decSpinner.setIncrement(25);
        decSpinner.setData("name", "spinnerDecimal");

        // Progress bars
        Group progressGroup = new Group(composite, SWT.NONE);
        progressGroup.setText("Progress Indicators");
        progressGroup.setLayout(new GridLayout(1, false));
        progressGroup.setLayoutData(new GridData(SWT.FILL, SWT.TOP, true, false));
        progressGroup.setData("name", "progressGroup");

        Label progressLabel = new Label(progressGroup, SWT.NONE);
        progressLabel.setText("Determinate:");
        progressLabel.setData("name", "labelProgress");

        progressBar = new ProgressBar(progressGroup, SWT.HORIZONTAL);
        progressBar.setMinimum(0);
        progressBar.setMaximum(100);
        progressBar.setSelection(45);
        progressBar.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        progressBar.setData("name", "progressBar");

        Label indeterminateLabel = new Label(progressGroup, SWT.NONE);
        indeterminateLabel.setText("Indeterminate:");
        indeterminateLabel.setData("name", "labelIndeterminate");

        ProgressBar indeterminateBar = new ProgressBar(progressGroup, SWT.HORIZONTAL | SWT.INDETERMINATE);
        indeterminateBar.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        indeterminateBar.setData("name", "progressBarIndeterminate");

        // CTabFolder (custom tab folder)
        Group ctabGroup = new Group(composite, SWT.NONE);
        ctabGroup.setText("CTabFolder Example");
        ctabGroup.setLayout(new FillLayout());
        GridData ctabGroupData = new GridData(SWT.FILL, SWT.FILL, true, true);
        ctabGroupData.heightHint = 150;
        ctabGroup.setLayoutData(ctabGroupData);
        ctabGroup.setData("name", "ctabGroup");

        CTabFolder cTabFolder = new CTabFolder(ctabGroup, SWT.BORDER | SWT.CLOSE);
        cTabFolder.setSimple(false);
        cTabFolder.setData("name", "cTabFolder");

        CTabItem cTab1 = new CTabItem(cTabFolder, SWT.NONE);
        cTab1.setText("Document 1");
        cTab1.setData("name", "cTabDocument1");
        Text cTabText1 = new Text(cTabFolder, SWT.MULTI | SWT.BORDER);
        cTabText1.setText("Content of Document 1");
        cTabText1.setData("name", "cTabText1");
        cTab1.setControl(cTabText1);

        CTabItem cTab2 = new CTabItem(cTabFolder, SWT.NONE);
        cTab2.setText("Document 2");
        cTab2.setData("name", "cTabDocument2");
        Text cTabText2 = new Text(cTabFolder, SWT.MULTI | SWT.BORDER);
        cTabText2.setText("Content of Document 2");
        cTabText2.setData("name", "cTabText2");
        cTab2.setControl(cTabText2);

        CTabItem cTab3 = new CTabItem(cTabFolder, SWT.NONE);
        cTab3.setText("Document 3");
        cTab3.setData("name", "cTabDocument3");
        Text cTabText3 = new Text(cTabFolder, SWT.MULTI | SWT.BORDER);
        cTabText3.setText("Content of Document 3");
        cTabText3.setData("name", "cTabText3");
        cTab3.setControl(cTabText3);

        cTabFolder.setSelection(0);

        // DateTime controls
        Group dateTimeGroup = new Group(composite, SWT.NONE);
        dateTimeGroup.setText("Date/Time Controls");
        dateTimeGroup.setLayout(new GridLayout(2, false));
        dateTimeGroup.setLayoutData(new GridData(SWT.FILL, SWT.TOP, true, false));
        dateTimeGroup.setData("name", "dateTimeGroup");

        Label dateLabel = new Label(dateTimeGroup, SWT.NONE);
        dateLabel.setText("Date:");
        dateLabel.setData("name", "labelDate");

        DateTime dateTime = new DateTime(dateTimeGroup, SWT.DATE | SWT.DROP_DOWN);
        dateTime.setData("name", "dateTimeDate");

        Label timeLabel = new Label(dateTimeGroup, SWT.NONE);
        timeLabel.setText("Time:");
        timeLabel.setData("name", "labelTime");

        DateTime timeWidget = new DateTime(dateTimeGroup, SWT.TIME);
        timeWidget.setData("name", "dateTimeTime");

        Label calendarLabel = new Label(dateTimeGroup, SWT.NONE);
        calendarLabel.setText("Calendar:");
        calendarLabel.setData("name", "labelCalendar");
        calendarLabel.setLayoutData(new GridData(SWT.LEFT, SWT.TOP, false, false));

        DateTime calendar = new DateTime(dateTimeGroup, SWT.CALENDAR);
        calendar.setData("name", "dateTimeCalendar");

        return composite;
    }

    private void createRightPanel(Composite parent) {
        Group propertiesGroup = new Group(parent, SWT.NONE);
        propertiesGroup.setText("Properties");
        propertiesGroup.setLayout(new GridLayout(1, false));
        propertiesGroup.setData("name", "propertiesGroup");

        // Properties list
        Label propLabel = new Label(propertiesGroup, SWT.NONE);
        propLabel.setText("Selected Item Properties:");
        propLabel.setData("name", "labelProperties");

        org.eclipse.swt.widgets.List propertiesList = new org.eclipse.swt.widgets.List(propertiesGroup, SWT.BORDER | SWT.V_SCROLL);
        propertiesList.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
        propertiesList.setData("name", "propertiesList");
        propertiesList.setItems(new String[]{
            "Type: Shell",
            "Name: mainShell",
            "Width: 1200",
            "Height: 800",
            "Visible: true",
            "Enabled: true"
        });

        // Expandable bar / ExpandBar
        ExpandBar expandBar = new ExpandBar(propertiesGroup, SWT.V_SCROLL);
        expandBar.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
        expandBar.setData("name", "expandBar");

        // First expand item
        Composite expandComp1 = new Composite(expandBar, SWT.NONE);
        expandComp1.setLayout(new GridLayout(2, false));

        Label lblName = new Label(expandComp1, SWT.NONE);
        lblName.setText("Name:");
        Text txtName = new Text(expandComp1, SWT.BORDER);
        txtName.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        txtName.setData("name", "expandTextName");

        Label lblValue = new Label(expandComp1, SWT.NONE);
        lblValue.setText("Value:");
        Text txtValue = new Text(expandComp1, SWT.BORDER);
        txtValue.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        txtValue.setData("name", "expandTextValue");

        ExpandItem expandItem1 = new ExpandItem(expandBar, SWT.NONE);
        expandItem1.setText("Basic Properties");
        expandItem1.setHeight(expandComp1.computeSize(SWT.DEFAULT, SWT.DEFAULT).y);
        expandItem1.setControl(expandComp1);
        expandItem1.setExpanded(true);
        expandItem1.setData("name", "expandItemBasic");

        // Second expand item
        Composite expandComp2 = new Composite(expandBar, SWT.NONE);
        expandComp2.setLayout(new GridLayout(2, false));

        Label lblWidth = new Label(expandComp2, SWT.NONE);
        lblWidth.setText("Width:");
        Spinner spnWidth = new Spinner(expandComp2, SWT.BORDER);
        spnWidth.setMinimum(0);
        spnWidth.setMaximum(2000);
        spnWidth.setSelection(1200);
        spnWidth.setData("name", "expandSpinnerWidth");

        Label lblHeight = new Label(expandComp2, SWT.NONE);
        lblHeight.setText("Height:");
        Spinner spnHeight = new Spinner(expandComp2, SWT.BORDER);
        spnHeight.setMinimum(0);
        spnHeight.setMaximum(2000);
        spnHeight.setSelection(800);
        spnHeight.setData("name", "expandSpinnerHeight");

        ExpandItem expandItem2 = new ExpandItem(expandBar, SWT.NONE);
        expandItem2.setText("Size Properties");
        expandItem2.setHeight(expandComp2.computeSize(SWT.DEFAULT, SWT.DEFAULT).y);
        expandItem2.setControl(expandComp2);
        expandItem2.setData("name", "expandItemSize");

        // Action buttons
        Composite actionComposite = new Composite(propertiesGroup, SWT.NONE);
        actionComposite.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        actionComposite.setLayout(new RowLayout(SWT.HORIZONTAL));
        actionComposite.setData("name", "propertiesActionComposite");

        Button applyButton = new Button(actionComposite, SWT.PUSH);
        applyButton.setText("Apply");
        applyButton.setData("name", "buttonApply");

        Button resetButton = new Button(actionComposite, SWT.PUSH);
        resetButton.setText("Reset");
        resetButton.setData("name", "buttonReset");
    }

    private void createStatusBar() {
        Composite statusComposite = new Composite(shell, SWT.NONE);
        statusComposite.setLayoutData(new GridData(SWT.FILL, SWT.BOTTOM, true, false));
        statusComposite.setLayout(new GridLayout(3, false));
        statusComposite.setData("name", "statusComposite");

        statusText = new Text(statusComposite, SWT.READ_ONLY | SWT.BORDER);
        statusText.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        statusText.setData("name", "statusText");
        statusText.setText("Ready");

        Label agentLabel = new Label(statusComposite, SWT.NONE);
        agentLabel.setText("Agent Port: " + agentPort);
        agentLabel.setData("name", "labelAgentPort");

        Label versionLabel = new Label(statusComposite, SWT.NONE);
        versionLabel.setText("SWT Test App v1.0");
        versionLabel.setData("name", "labelVersion");
    }

    // Helper methods

    private void updateStatus(String message) {
        if (statusText != null && !statusText.isDisposed()) {
            statusText.setText(message);
        }
        System.out.println("[SwtTestApp] Status: " + message);
    }

    private void centerShell(Shell shell) {
        org.eclipse.swt.graphics.Rectangle screenSize = display.getPrimaryMonitor().getBounds();
        int x = (screenSize.width - shell.getSize().x) / 2;
        int y = (screenSize.height - shell.getSize().y) / 2;
        shell.setLocation(x, y);
    }

    private void openFileDialog() {
        FileDialog dialog = new FileDialog(shell, SWT.OPEN);
        dialog.setText("Open File");
        dialog.setFilterExtensions(new String[]{"*.txt", "*.java", "*.xml", "*.*"});
        dialog.setFilterNames(new String[]{"Text Files", "Java Files", "XML Files", "All Files"});
        String result = dialog.open();
        if (result != null) {
            updateStatus("Opened: " + result);
        }
    }

    private void showAboutDialog() {
        Shell aboutShell = new Shell(shell, SWT.DIALOG_TRIM | SWT.APPLICATION_MODAL);
        aboutShell.setText("About SWT Test App");
        aboutShell.setSize(400, 250);
        aboutShell.setLayout(new GridLayout(1, false));
        aboutShell.setData("name", "aboutDialog");
        centerShell(aboutShell);

        Label titleLabel = new Label(aboutShell, SWT.CENTER);
        titleLabel.setText("SWT Test Application");
        titleLabel.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        Font boldFont = new Font(display, titleLabel.getFont().getFontData()[0].getName(), 14, SWT.BOLD);
        titleLabel.setFont(boldFont);
        titleLabel.setData("name", "aboutTitleLabel");

        Label versionAboutLabel = new Label(aboutShell, SWT.CENTER);
        versionAboutLabel.setText("Version 1.0.0");
        versionAboutLabel.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        versionAboutLabel.setData("name", "aboutVersionLabel");

        Label descLabel = new Label(aboutShell, SWT.WRAP | SWT.CENTER);
        descLabel.setText("A comprehensive SWT test application for testing\nRobot Framework SwtLibrary keywords.\n\nIncludes all major SWT widgets for automated testing.");
        GridData descData = new GridData(SWT.FILL, SWT.CENTER, true, true);
        descData.widthHint = 350;
        descLabel.setLayoutData(descData);
        descLabel.setData("name", "aboutDescLabel");

        Label copyrightLabel = new Label(aboutShell, SWT.CENTER);
        copyrightLabel.setText("Copyright 2024 - Robot Framework");
        copyrightLabel.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        copyrightLabel.setData("name", "aboutCopyrightLabel");

        Button closeButton = new Button(aboutShell, SWT.PUSH);
        closeButton.setText("Close");
        closeButton.setLayoutData(new GridData(SWT.CENTER, SWT.CENTER, false, false));
        closeButton.setData("name", "aboutCloseButton");
        closeButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                aboutShell.close();
            }
        });

        aboutShell.open();
    }

    private void showTestDialog() {
        Shell dialogShell = new Shell(shell, SWT.DIALOG_TRIM | SWT.APPLICATION_MODAL | SWT.RESIZE);
        dialogShell.setText("Test Dialog");
        dialogShell.setSize(500, 400);
        dialogShell.setLayout(new GridLayout(2, false));
        dialogShell.setData("name", "testDialog");
        centerShell(dialogShell);

        // Form fields
        Label nameLabel = new Label(dialogShell, SWT.NONE);
        nameLabel.setText("Name:");
        nameLabel.setData("name", "dialogLabelName");

        Text nameText = new Text(dialogShell, SWT.BORDER);
        nameText.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        nameText.setData("name", "dialogTextName");

        Label emailLabel = new Label(dialogShell, SWT.NONE);
        emailLabel.setText("Email:");
        emailLabel.setData("name", "dialogLabelEmail");

        Text emailText = new Text(dialogShell, SWT.BORDER);
        emailText.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        emailText.setData("name", "dialogTextEmail");

        Label commentLabel = new Label(dialogShell, SWT.NONE);
        commentLabel.setText("Comment:");
        commentLabel.setData("name", "dialogLabelComment");
        commentLabel.setLayoutData(new GridData(SWT.LEFT, SWT.TOP, false, false));

        Text commentText = new Text(dialogShell, SWT.BORDER | SWT.MULTI | SWT.V_SCROLL);
        GridData commentData = new GridData(SWT.FILL, SWT.FILL, true, true);
        commentData.heightHint = 100;
        commentText.setLayoutData(commentData);
        commentText.setData("name", "dialogTextComment");

        // Checkbox
        Button agreeCheck = new Button(dialogShell, SWT.CHECK);
        agreeCheck.setText("I agree to the terms");
        GridData agreeData = new GridData(SWT.LEFT, SWT.CENTER, false, false);
        agreeData.horizontalSpan = 2;
        agreeCheck.setLayoutData(agreeData);
        agreeCheck.setData("name", "dialogCheckAgree");

        // Buttons
        Composite buttonComposite = new Composite(dialogShell, SWT.NONE);
        GridData buttonData = new GridData(SWT.RIGHT, SWT.CENTER, false, false);
        buttonData.horizontalSpan = 2;
        buttonComposite.setLayoutData(buttonData);
        buttonComposite.setLayout(new RowLayout(SWT.HORIZONTAL));
        buttonComposite.setData("name", "dialogButtonComposite");

        Button okButton = new Button(buttonComposite, SWT.PUSH);
        okButton.setText("OK");
        okButton.setData("name", "dialogButtonOk");
        okButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                updateStatus("Dialog OK clicked - Name: " + nameText.getText());
                dialogShell.close();
            }
        });

        Button cancelButton = new Button(buttonComposite, SWT.PUSH);
        cancelButton.setText("Cancel");
        cancelButton.setData("name", "dialogButtonCancel");
        cancelButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                dialogShell.close();
            }
        });

        dialogShell.open();
    }

    private void showMessageDialog() {
        MessageBox messageBox = new MessageBox(shell, SWT.ICON_INFORMATION | SWT.YES | SWT.NO);
        messageBox.setText("Confirmation");
        messageBox.setMessage("This is a test message dialog.\n\nDo you want to continue?");
        int result = messageBox.open();
        if (result == SWT.YES) {
            updateStatus("Message dialog: YES selected");
        } else {
            updateStatus("Message dialog: NO selected");
        }
    }

    private void showPreferencesDialog() {
        Shell preferencesShell = new Shell(shell, SWT.DIALOG_TRIM | SWT.APPLICATION_MODAL | SWT.RESIZE);
        preferencesShell.setText("Preferences");
        preferencesShell.setSize(500, 400);
        preferencesShell.setLayout(new GridLayout(1, false));
        preferencesShell.setData("name", "preferencesShell");
        centerShell(preferencesShell);

        // Create tab folder for preference categories
        TabFolder prefTabFolder = new TabFolder(preferencesShell, SWT.TOP);
        prefTabFolder.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
        prefTabFolder.setData("name", "preferencesTabFolder");

        // General preferences tab
        TabItem generalTab = new TabItem(prefTabFolder, SWT.NONE);
        generalTab.setText("General");
        generalTab.setData("name", "prefTabGeneral");

        Composite generalComposite = new Composite(prefTabFolder, SWT.NONE);
        generalComposite.setLayout(new GridLayout(2, false));
        generalComposite.setData("name", "prefGeneralComposite");
        generalTab.setControl(generalComposite);

        Label themeLabel = new Label(generalComposite, SWT.NONE);
        themeLabel.setText("Theme:");
        themeLabel.setData("name", "prefLabelTheme");

        Combo themeCombo = new Combo(generalComposite, SWT.DROP_DOWN | SWT.READ_ONLY);
        themeCombo.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        themeCombo.setData("name", "prefComboTheme");
        themeCombo.setItems(new String[]{"Light", "Dark", "System Default"});
        themeCombo.select(0);

        Label languageLabel = new Label(generalComposite, SWT.NONE);
        languageLabel.setText("Language:");
        languageLabel.setData("name", "prefLabelLanguage");

        Combo languageCombo = new Combo(generalComposite, SWT.DROP_DOWN | SWT.READ_ONLY);
        languageCombo.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        languageCombo.setData("name", "prefComboLanguage");
        languageCombo.setItems(new String[]{"English", "German", "French", "Spanish"});
        languageCombo.select(0);

        Button autoSaveCheck = new Button(generalComposite, SWT.CHECK);
        autoSaveCheck.setText("Enable auto-save");
        autoSaveCheck.setData("name", "prefCheckAutoSave");
        GridData autoSaveData = new GridData(SWT.LEFT, SWT.CENTER, false, false);
        autoSaveData.horizontalSpan = 2;
        autoSaveCheck.setLayoutData(autoSaveData);
        autoSaveCheck.setSelection(true);

        Button notificationsCheck = new Button(generalComposite, SWT.CHECK);
        notificationsCheck.setText("Show notifications");
        notificationsCheck.setData("name", "prefCheckNotifications");
        GridData notifData = new GridData(SWT.LEFT, SWT.CENTER, false, false);
        notifData.horizontalSpan = 2;
        notificationsCheck.setLayoutData(notifData);
        notificationsCheck.setSelection(true);

        // Editor preferences tab
        TabItem editorTab = new TabItem(prefTabFolder, SWT.NONE);
        editorTab.setText("Editor");
        editorTab.setData("name", "prefTabEditor");

        Composite editorComposite = new Composite(prefTabFolder, SWT.NONE);
        editorComposite.setLayout(new GridLayout(2, false));
        editorComposite.setData("name", "prefEditorComposite");
        editorTab.setControl(editorComposite);

        Label fontSizeLabel = new Label(editorComposite, SWT.NONE);
        fontSizeLabel.setText("Font Size:");
        fontSizeLabel.setData("name", "prefLabelFontSize");

        Spinner fontSizeSpinner = new Spinner(editorComposite, SWT.BORDER);
        fontSizeSpinner.setMinimum(8);
        fontSizeSpinner.setMaximum(72);
        fontSizeSpinner.setSelection(12);
        fontSizeSpinner.setData("name", "prefSpinnerFontSize");

        Label tabSizeLabel = new Label(editorComposite, SWT.NONE);
        tabSizeLabel.setText("Tab Size:");
        tabSizeLabel.setData("name", "prefLabelTabSize");

        Spinner tabSizeSpinner = new Spinner(editorComposite, SWT.BORDER);
        tabSizeSpinner.setMinimum(1);
        tabSizeSpinner.setMaximum(8);
        tabSizeSpinner.setSelection(4);
        tabSizeSpinner.setData("name", "prefSpinnerTabSize");

        Button lineNumbersCheck = new Button(editorComposite, SWT.CHECK);
        lineNumbersCheck.setText("Show line numbers");
        lineNumbersCheck.setData("name", "prefCheckLineNumbers");
        GridData lineNumData = new GridData(SWT.LEFT, SWT.CENTER, false, false);
        lineNumData.horizontalSpan = 2;
        lineNumbersCheck.setLayoutData(lineNumData);
        lineNumbersCheck.setSelection(true);

        Button wordWrapCheck = new Button(editorComposite, SWT.CHECK);
        wordWrapCheck.setText("Enable word wrap");
        wordWrapCheck.setData("name", "prefCheckWordWrap");
        GridData wordWrapData = new GridData(SWT.LEFT, SWT.CENTER, false, false);
        wordWrapData.horizontalSpan = 2;
        wordWrapCheck.setLayoutData(wordWrapData);

        // Button composite
        Composite buttonComposite = new Composite(preferencesShell, SWT.NONE);
        buttonComposite.setLayoutData(new GridData(SWT.RIGHT, SWT.CENTER, true, false));
        buttonComposite.setLayout(new RowLayout(SWT.HORIZONTAL));
        buttonComposite.setData("name", "prefButtonComposite");

        Button applyButton = new Button(buttonComposite, SWT.PUSH);
        applyButton.setText("Apply");
        applyButton.setData("name", "prefButtonApply");
        applyButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                updateStatus("Preferences applied");
            }
        });

        Button okButton = new Button(buttonComposite, SWT.PUSH);
        okButton.setText("OK");
        okButton.setData("name", "prefButtonOk");
        okButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                updateStatus("Preferences saved");
                preferencesShell.close();
            }
        });

        Button cancelButton = new Button(buttonComposite, SWT.PUSH);
        cancelButton.setText("Cancel");
        cancelButton.setData("name", "prefButtonCancel");
        cancelButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                preferencesShell.close();
            }
        });

        preferencesShell.open();
    }

    private void runProgressDemo() {
        updateStatus("Running progress demo...");
        progressBar.setSelection(0);

        new Thread(() -> {
            for (int i = 0; i <= 100; i += 5) {
                final int progress = i;
                try {
                    Thread.sleep(100);
                } catch (InterruptedException e) {
                    break;
                }
                if (!display.isDisposed()) {
                    display.asyncExec(() -> {
                        if (!progressBar.isDisposed()) {
                            progressBar.setSelection(progress);
                        }
                        if (progress == 100 && !statusText.isDisposed()) {
                            updateStatus("Progress complete!");
                        }
                    });
                }
            }
        }).start();
    }

    private void addTableRow() {
        if (dataTable != null && !dataTable.isDisposed()) {
            int rowNum = dataTable.getItemCount() + 1;
            TableItem item = new TableItem(dataTable, SWT.NONE);
            item.setText(new String[]{
                String.valueOf(rowNum),
                "New Item " + rowNum,
                "New",
                java.time.LocalDate.now().toString(),
                "Medium"
            });
            item.setData("name", "tableRow" + rowNum);
            updateStatus("Added row " + rowNum);
        }
    }
}
