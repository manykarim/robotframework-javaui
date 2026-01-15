package testapp.rcp;

import org.eclipse.swt.SWT;
import org.eclipse.swt.custom.CTabFolder;
import org.eclipse.swt.custom.CTabFolder2Adapter;
import org.eclipse.swt.custom.CTabFolderEvent;
import org.eclipse.swt.custom.CTabItem;
import org.eclipse.swt.custom.SashForm;
import org.eclipse.swt.custom.StyledText;
import org.eclipse.swt.events.*;
import org.eclipse.swt.graphics.*;
import org.eclipse.swt.layout.*;
import org.eclipse.swt.widgets.*;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

/**
 * Mock RCP Application - Simulates Eclipse RCP Workbench for testing.
 *
 * This application provides a simplified implementation of Eclipse's workbench
 * concepts including perspectives, views, and editors, suitable for testing
 * with the Robot Framework SWT library.
 */
public class MockRcpApplication {

    private Display display;
    private Shell shell;
    private CTabFolder perspectiveBar;
    private SashForm mainSash;
    private SashForm leftSash;
    private SashForm centerSash;
    private CTabFolder viewFolder;
    private CTabFolder editorFolder;
    private CTabFolder bottomFolder;
    private CTabFolder rightFolder;

    // Mock data structures for RCP concepts
    private String activePerspectiveId = "com.testapp.rcp.perspective.main";
    private Map<String, PerspectiveInfo> perspectives = new ConcurrentHashMap<>();
    private Map<String, ViewInfo> views = new ConcurrentHashMap<>();
    private Map<String, EditorInfo> editors = new ConcurrentHashMap<>();
    private Map<String, CTabItem> viewTabs = new ConcurrentHashMap<>();
    private Map<String, CTabItem> editorTabs = new ConcurrentHashMap<>();
    private java.util.List<String> openDialogs = new ArrayList<>();

    private String workbenchTitle = "Mock RCP Workbench";
    private Label statusLabel;
    private ToolBar mainToolBar;

    // Static instance for RPC access
    private static MockRcpApplication instance;

    public static MockRcpApplication getInstance() {
        return instance;
    }

    public static void main(String[] args) {
        System.out.println("[MockRcpApp] Starting application...");
        MockRcpApplication app = new MockRcpApplication();
        app.run();
    }

    public void run() {
        instance = this;
        display = Display.getDefault();

        createShell();
        initializePerspectives();
        initializeViews();
        createUI();

        // Show the default perspective
        switchPerspective(activePerspectiveId);

        shell.open();
        System.out.println("[MockRcpApp] Application started successfully");

        while (!shell.isDisposed()) {
            if (!display.readAndDispatch()) {
                display.sleep();
            }
        }

        display.dispose();
        System.out.println("[MockRcpApp] Application closed");
    }

    private void createShell() {
        shell = new Shell(display, SWT.SHELL_TRIM);
        shell.setText(workbenchTitle);
        shell.setSize(1200, 800);
        shell.setData("name", "mainShell");
        shell.setData("workbench", "true");

        // Center the shell
        Monitor primary = display.getPrimaryMonitor();
        Rectangle bounds = primary.getBounds();
        Rectangle rect = shell.getBounds();
        int x = bounds.x + (bounds.width - rect.width) / 2;
        int y = bounds.y + (bounds.height - rect.height) / 2;
        shell.setLocation(x, y);

        shell.setLayout(new GridLayout(1, false));
    }

    private void initializePerspectives() {
        // Main Test Perspective
        PerspectiveInfo mainPerspective = new PerspectiveInfo(
            "com.testapp.rcp.perspective.main",
            "Test Perspective",
            "Main perspective for testing"
        );
        perspectives.put(mainPerspective.id, mainPerspective);

        // Debug Perspective
        PerspectiveInfo debugPerspective = new PerspectiveInfo(
            "com.testapp.rcp.perspective.debug",
            "Debug Perspective",
            "Debugging perspective"
        );
        perspectives.put(debugPerspective.id, debugPerspective);

        // Eclipse Debug Perspective (standard ID)
        PerspectiveInfo eclipseDebugPerspective = new PerspectiveInfo(
            "org.eclipse.debug.ui.DebugPerspective",
            "Debug",
            "Eclipse debug perspective"
        );
        perspectives.put(eclipseDebugPerspective.id, eclipseDebugPerspective);

        // Data Perspective
        PerspectiveInfo dataPerspective = new PerspectiveInfo(
            "com.testapp.rcp.perspective.data",
            "Data Perspective",
            "Data management perspective"
        );
        perspectives.put(dataPerspective.id, dataPerspective);

        // Java Perspective (for compatibility with standard Eclipse IDs)
        PerspectiveInfo javaPerspective = new PerspectiveInfo(
            "org.eclipse.jdt.ui.JavaPerspective",
            "Java",
            "Java development perspective"
        );
        perspectives.put(javaPerspective.id, javaPerspective);

        // Resource Perspective
        PerspectiveInfo resourcePerspective = new PerspectiveInfo(
            "org.eclipse.ui.resourcePerspective",
            "Resource",
            "Resource perspective"
        );
        perspectives.put(resourcePerspective.id, resourcePerspective);

        // Team Synchronizing Perspective
        PerspectiveInfo teamSyncPerspective = new PerspectiveInfo(
            "org.eclipse.team.ui.TeamSynchronizingPerspective",
            "Team Synchronizing",
            "Team synchronization perspective"
        );
        perspectives.put(teamSyncPerspective.id, teamSyncPerspective);

        // Git Perspective
        PerspectiveInfo gitPerspective = new PerspectiveInfo(
            "org.eclipse.egit.ui.GitRepositoryExploring",
            "Git",
            "Git repository exploring"
        );
        perspectives.put(gitPerspective.id, gitPerspective);
    }

    private void initializeViews() {
        views.put("com.testapp.rcp.views.navigator", new ViewInfo(
            "com.testapp.rcp.views.navigator", "Navigator", "Project navigator view"));
        views.put("com.testapp.rcp.views.properties", new ViewInfo(
            "com.testapp.rcp.views.properties", "Properties", "Properties view"));
        views.put("com.testapp.rcp.views.console", new ViewInfo(
            "com.testapp.rcp.views.console", "Console", "Console output view"));
        views.put("com.testapp.rcp.views.outline", new ViewInfo(
            "com.testapp.rcp.views.outline", "Outline", "Document outline view"));
        views.put("com.testapp.rcp.views.tasks", new ViewInfo(
            "com.testapp.rcp.views.tasks", "Tasks", "Task list view"));

        // Standard Eclipse view IDs for compatibility
        views.put("org.eclipse.jdt.ui.PackageExplorer", new ViewInfo(
            "org.eclipse.jdt.ui.PackageExplorer", "Package Explorer", "Java package explorer"));
        views.put("org.eclipse.ui.navigator.ProjectExplorer", new ViewInfo(
            "org.eclipse.ui.navigator.ProjectExplorer", "Project Explorer", "Project explorer"));
        views.put("org.eclipse.ui.views.ProblemView", new ViewInfo(
            "org.eclipse.ui.views.ProblemView", "Problems", "Problems view"));
        views.put("org.eclipse.ui.console.ConsoleView", new ViewInfo(
            "org.eclipse.ui.console.ConsoleView", "Console", "Console view"));
        views.put("org.eclipse.ui.views.ContentOutline", new ViewInfo(
            "org.eclipse.ui.views.ContentOutline", "Outline", "Content outline view"));
        views.put("org.eclipse.ui.views.PropertySheet", new ViewInfo(
            "org.eclipse.ui.views.PropertySheet", "Properties", "Property sheet view"));
        views.put("org.eclipse.pde.runtime.LogView", new ViewInfo(
            "org.eclipse.pde.runtime.LogView", "Error Log", "Error log view"));
        views.put("org.eclipse.ui.views.ProgressView", new ViewInfo(
            "org.eclipse.ui.views.ProgressView", "Progress", "Progress view"));
        views.put("org.eclipse.search.ui.views.SearchView", new ViewInfo(
            "org.eclipse.search.ui.views.SearchView", "Search", "Search results view"));
        views.put("org.eclipse.ui.views.AllMarkersView", new ViewInfo(
            "org.eclipse.ui.views.AllMarkersView", "Markers", "All markers view"));
        views.put("org.eclipse.ui.views.TaskList", new ViewInfo(
            "org.eclipse.ui.views.TaskList", "Tasks", "Task list view"));
        views.put("org.eclipse.ui.views.BookmarkView", new ViewInfo(
            "org.eclipse.ui.views.BookmarkView", "Bookmarks", "Bookmarks view"));
    }

    private void createUI() {
        createMenuBar();
        createToolBar();
        createWorkbenchArea();
        createStatusBar();
    }

    private void createMenuBar() {
        Menu menuBar = new Menu(shell, SWT.BAR);
        shell.setMenuBar(menuBar);

        // File Menu
        MenuItem fileItem = new MenuItem(menuBar, SWT.CASCADE);
        fileItem.setText("&File");
        Menu fileMenu = new Menu(shell, SWT.DROP_DOWN);
        fileItem.setMenu(fileMenu);

        MenuItem newItem = new MenuItem(fileMenu, SWT.PUSH);
        newItem.setText("&New Project...\tCtrl+N");
        newItem.setAccelerator(SWT.CTRL | 'N');
        newItem.setData("commandId", "com.testapp.rcp.commands.newProject");
        newItem.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                showNewProjectDialog();
            }
        });

        new MenuItem(fileMenu, SWT.SEPARATOR);

        MenuItem saveItem = new MenuItem(fileMenu, SWT.PUSH);
        saveItem.setText("&Save\tCtrl+S");
        saveItem.setAccelerator(SWT.CTRL | 'S');
        saveItem.setData("commandId", "org.eclipse.ui.file.save");
        saveItem.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                saveActiveEditor();
            }
        });

        MenuItem saveAllItem = new MenuItem(fileMenu, SWT.PUSH);
        saveAllItem.setText("Save &All\tCtrl+Shift+S");
        saveAllItem.setData("commandId", "org.eclipse.ui.file.saveAll");

        new MenuItem(fileMenu, SWT.SEPARATOR);

        MenuItem refreshItem = new MenuItem(fileMenu, SWT.PUSH);
        refreshItem.setText("&Refresh\tF5");
        refreshItem.setAccelerator(SWT.F5);
        refreshItem.setData("commandId", "org.eclipse.ui.file.refresh");

        new MenuItem(fileMenu, SWT.SEPARATOR);

        MenuItem exitItem = new MenuItem(fileMenu, SWT.PUSH);
        exitItem.setText("E&xit");
        exitItem.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                shell.close();
            }
        });

        // Edit Menu
        MenuItem editItem = new MenuItem(menuBar, SWT.CASCADE);
        editItem.setText("&Edit");
        Menu editMenu = new Menu(shell, SWT.DROP_DOWN);
        editItem.setMenu(editMenu);

        MenuItem undoItem = new MenuItem(editMenu, SWT.PUSH);
        undoItem.setText("&Undo\tCtrl+Z");
        undoItem.setData("commandId", "org.eclipse.ui.edit.undo");

        MenuItem redoItem = new MenuItem(editMenu, SWT.PUSH);
        redoItem.setText("&Redo\tCtrl+Y");
        redoItem.setData("commandId", "org.eclipse.ui.edit.redo");

        new MenuItem(editMenu, SWT.SEPARATOR);

        MenuItem cutItem = new MenuItem(editMenu, SWT.PUSH);
        cutItem.setText("Cu&t\tCtrl+X");
        cutItem.setData("commandId", "org.eclipse.ui.edit.cut");

        MenuItem copyItem = new MenuItem(editMenu, SWT.PUSH);
        copyItem.setText("&Copy\tCtrl+C");
        copyItem.setData("commandId", "org.eclipse.ui.edit.copy");

        MenuItem pasteItem = new MenuItem(editMenu, SWT.PUSH);
        pasteItem.setText("&Paste\tCtrl+V");
        pasteItem.setData("commandId", "org.eclipse.ui.edit.paste");

        // Window Menu
        MenuItem windowItem = new MenuItem(menuBar, SWT.CASCADE);
        windowItem.setText("&Window");
        Menu windowMenu = new Menu(shell, SWT.DROP_DOWN);
        windowItem.setMenu(windowMenu);

        MenuItem showViewItem = new MenuItem(windowMenu, SWT.CASCADE);
        showViewItem.setText("Show &View");
        Menu showViewMenu = new Menu(shell, SWT.DROP_DOWN);
        showViewItem.setMenu(showViewMenu);

        for (ViewInfo view : views.values()) {
            MenuItem viewMenuItem = new MenuItem(showViewMenu, SWT.PUSH);
            viewMenuItem.setText(view.name);
            final String viewId = view.id;
            viewMenuItem.addSelectionListener(new SelectionAdapter() {
                @Override
                public void widgetSelected(SelectionEvent e) {
                    showView(viewId, null);
                }
            });
        }

        new MenuItem(windowMenu, SWT.SEPARATOR);

        MenuItem openPerspectiveItem = new MenuItem(windowMenu, SWT.CASCADE);
        openPerspectiveItem.setText("Open &Perspective");
        Menu perspectiveMenu = new Menu(shell, SWT.DROP_DOWN);
        openPerspectiveItem.setMenu(perspectiveMenu);

        for (PerspectiveInfo persp : perspectives.values()) {
            MenuItem perspMenuItem = new MenuItem(perspectiveMenu, SWT.PUSH);
            perspMenuItem.setText(persp.name);
            final String perspId = persp.id;
            perspMenuItem.addSelectionListener(new SelectionAdapter() {
                @Override
                public void widgetSelected(SelectionEvent e) {
                    switchPerspective(perspId);
                }
            });
        }

        new MenuItem(windowMenu, SWT.SEPARATOR);

        MenuItem prefsItem = new MenuItem(windowMenu, SWT.PUSH);
        prefsItem.setText("&Preferences...");
        prefsItem.setData("commandId", "org.eclipse.ui.window.preferences");
        prefsItem.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                showPreferencesDialog();
            }
        });

        // Help Menu
        MenuItem helpItem = new MenuItem(menuBar, SWT.CASCADE);
        helpItem.setText("&Help");
        Menu helpMenu = new Menu(shell, SWT.DROP_DOWN);
        helpItem.setMenu(helpMenu);

        MenuItem aboutItem = new MenuItem(helpMenu, SWT.PUSH);
        aboutItem.setText("&About");
        aboutItem.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                showAboutDialog();
            }
        });
    }

    private void createToolBar() {
        mainToolBar = new ToolBar(shell, SWT.FLAT | SWT.WRAP | SWT.RIGHT);
        mainToolBar.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        mainToolBar.setData("name", "mainToolBar");

        ToolItem newButton = new ToolItem(mainToolBar, SWT.PUSH);
        newButton.setText("New");
        newButton.setToolTipText("New Project");
        newButton.setData("name", "toolbarNew");
        newButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                showNewProjectDialog();
            }
        });

        new ToolItem(mainToolBar, SWT.SEPARATOR);

        ToolItem saveButton = new ToolItem(mainToolBar, SWT.PUSH);
        saveButton.setText("Save");
        saveButton.setToolTipText("Save");
        saveButton.setData("name", "toolbarSave");

        ToolItem saveAllButton = new ToolItem(mainToolBar, SWT.PUSH);
        saveAllButton.setText("Save All");
        saveAllButton.setToolTipText("Save All");
        saveAllButton.setData("name", "toolbarSaveAll");

        new ToolItem(mainToolBar, SWT.SEPARATOR);

        ToolItem refreshButton = new ToolItem(mainToolBar, SWT.PUSH);
        refreshButton.setText("Refresh");
        refreshButton.setToolTipText("Refresh (F5)");
        refreshButton.setData("name", "toolbarRefresh");
    }

    private void createWorkbenchArea() {
        mainSash = new SashForm(shell, SWT.HORIZONTAL);
        mainSash.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
        mainSash.setData("name", "workbenchArea");

        // Left pane - Navigator views
        leftSash = new SashForm(mainSash, SWT.VERTICAL);
        leftSash.setData("name", "leftPane");

        viewFolder = new CTabFolder(leftSash, SWT.BORDER | SWT.BOTTOM);
        viewFolder.setSimple(false);
        viewFolder.setData("name", "viewFolder");

        // Create default views
        createViewTab(viewFolder, "com.testapp.rcp.views.navigator");

        // Outline view in lower left
        CTabFolder outlineFolder = new CTabFolder(leftSash, SWT.BORDER | SWT.BOTTOM);
        outlineFolder.setSimple(false);
        outlineFolder.setData("name", "outlineFolder");
        createViewInFolder(outlineFolder, "com.testapp.rcp.views.outline");

        leftSash.setWeights(new int[]{60, 40});

        // Center pane - Editors and bottom views
        centerSash = new SashForm(mainSash, SWT.VERTICAL);
        centerSash.setData("name", "centerPane");

        editorFolder = new CTabFolder(centerSash, SWT.BORDER | SWT.TOP);
        editorFolder.setSimple(false);
        editorFolder.setData("name", "editorFolder");

        // Add close button to tabs
        editorFolder.addCTabFolder2Listener(new CTabFolder2Adapter() {
            @Override
            public void close(CTabFolderEvent event) {
                CTabItem item = (CTabItem) event.item;
                String editorId = (String) item.getData("editorId");
                if (editorId != null) {
                    editors.remove(editorId);
                    editorTabs.remove(editorId);
                }
            }
        });

        // Bottom views (Console, Tasks)
        bottomFolder = new CTabFolder(centerSash, SWT.BORDER | SWT.BOTTOM);
        bottomFolder.setSimple(false);
        bottomFolder.setData("name", "bottomFolder");
        createViewInFolder(bottomFolder, "com.testapp.rcp.views.console");
        createViewInFolder(bottomFolder, "com.testapp.rcp.views.tasks");

        centerSash.setWeights(new int[]{70, 30});

        // Right pane - Properties view
        rightFolder = new CTabFolder(mainSash, SWT.BORDER | SWT.BOTTOM);
        rightFolder.setSimple(false);
        rightFolder.setData("name", "rightFolder");
        createViewInFolder(rightFolder, "com.testapp.rcp.views.properties");

        mainSash.setWeights(new int[]{20, 60, 20});
    }

    private void createViewTab(CTabFolder folder, String viewId) {
        ViewInfo viewInfo = views.get(viewId);
        if (viewInfo == null) return;

        CTabItem item = new CTabItem(folder, SWT.NONE);
        item.setText(viewInfo.name);
        item.setData("viewId", viewId);
        item.setData("name", "view_" + viewInfo.name.toLowerCase().replace(" ", "_"));

        Composite content = createViewContent(folder, viewId);
        item.setControl(content);

        viewTabs.put(viewId, item);
        viewInfo.isVisible = true;
        folder.setSelection(item);
    }

    private void createViewInFolder(CTabFolder folder, String viewId) {
        createViewTab(folder, viewId);
    }

    private Composite createViewContent(Composite parent, String viewId) {
        Composite content = new Composite(parent, SWT.NONE);
        content.setLayout(new FillLayout());
        content.setData("name", "viewContent_" + viewId);
        content.setData("viewId", viewId);

        ViewInfo viewInfo = views.get(viewId);

        if (viewId.contains("navigator") || viewId.contains("PackageExplorer") || viewId.contains("ProjectExplorer")) {
            // Tree view for navigator - use unique name based on view type
            Tree tree = new Tree(content, SWT.BORDER | SWT.V_SCROLL | SWT.H_SCROLL);
            // Generate unique name: packageExplorerTree, projectExplorerTree, navigatorTree
            String treeName = viewId.contains("PackageExplorer") ? "packageExplorerTree" :
                             viewId.contains("ProjectExplorer") ? "projectExplorerTree" : "navigatorTree";
            tree.setData("name", treeName);
            tree.setData("viewId", viewId);

            TreeItem project1 = new TreeItem(tree, SWT.NONE);
            project1.setText("test-project");
            project1.setData("path", "/test-project");

            TreeItem src = new TreeItem(project1, SWT.NONE);
            src.setText("src");
            src.setData("path", "/test-project/src");

            TreeItem testJava = new TreeItem(src, SWT.NONE);
            testJava.setText("Test.java");
            testJava.setData("path", "/test-project/src/Test.java");
            testJava.setData("filePath", "/test-project/src/Test.java");

            TreeItem main = new TreeItem(src, SWT.NONE);
            main.setText("Main.java");
            main.setData("path", "/test-project/src/Main.java");

            TreeItem resources = new TreeItem(project1, SWT.NONE);
            resources.setText("resources");
            resources.setData("path", "/test-project/resources");

            TreeItem configXml = new TreeItem(resources, SWT.NONE);
            configXml.setText("config.xml");
            configXml.setData("path", "/test-project/resources/config.xml");

            project1.setExpanded(true);

            // Double-click to open editor
            tree.addListener(SWT.DefaultSelection, event -> {
                TreeItem[] selection = tree.getSelection();
                if (selection.length > 0) {
                    String filePath = (String) selection[0].getData("filePath");
                    if (filePath != null) {
                        openEditor(filePath);
                    }
                }
            });

        } else if (viewId.contains("console") || viewId.contains("ConsoleView")) {
            // Console text view
            StyledText console = new StyledText(content, SWT.BORDER | SWT.V_SCROLL | SWT.H_SCROLL | SWT.READ_ONLY);
            console.setData("name", "consoleText");
            console.setData("viewId", viewId);
            console.setText("Mock RCP Application Console\n==================\n");
            console.append("Application started at " + new java.util.Date() + "\n");

        } else if (viewId.contains("properties") || viewId.contains("PropertySheet")) {
            // Properties table
            Table table = new Table(content, SWT.BORDER | SWT.FULL_SELECTION);
            table.setHeaderVisible(true);
            table.setLinesVisible(true);
            table.setData("name", "propertiesTable");
            table.setData("viewId", viewId);

            TableColumn nameCol = new TableColumn(table, SWT.NONE);
            nameCol.setText("Property");
            nameCol.setWidth(150);

            TableColumn valueCol = new TableColumn(table, SWT.NONE);
            valueCol.setText("Value");
            valueCol.setWidth(200);

            TableItem item1 = new TableItem(table, SWT.NONE);
            item1.setText(new String[]{"Name", "test-project"});

            TableItem item2 = new TableItem(table, SWT.NONE);
            item2.setText(new String[]{"Type", "Java Project"});

            TableItem item3 = new TableItem(table, SWT.NONE);
            item3.setText(new String[]{"Location", "/workspace/test-project"});

        } else if (viewId.contains("outline") || viewId.contains("ContentOutline")) {
            // Outline tree
            Tree outline = new Tree(content, SWT.BORDER | SWT.V_SCROLL);
            outline.setData("name", "outlineTree");
            outline.setData("viewId", viewId);

            TreeItem classItem = new TreeItem(outline, SWT.NONE);
            classItem.setText("TestClass");

            TreeItem methodItem = new TreeItem(classItem, SWT.NONE);
            methodItem.setText("main(String[])");

            TreeItem fieldItem = new TreeItem(classItem, SWT.NONE);
            fieldItem.setText("field1: String");

            classItem.setExpanded(true);

        } else if (viewId.contains("tasks") || viewId.contains("TaskList")) {
            // Tasks table
            Table taskTable = new Table(content, SWT.BORDER | SWT.FULL_SELECTION | SWT.CHECK);
            taskTable.setHeaderVisible(true);
            taskTable.setLinesVisible(true);
            taskTable.setData("name", "tasksTable");
            taskTable.setData("viewId", viewId);

            TableColumn descCol = new TableColumn(taskTable, SWT.NONE);
            descCol.setText("Description");
            descCol.setWidth(250);

            TableColumn resourceCol = new TableColumn(taskTable, SWT.NONE);
            resourceCol.setText("Resource");
            resourceCol.setWidth(100);

            TableColumn locationCol = new TableColumn(taskTable, SWT.NONE);
            locationCol.setText("Location");
            locationCol.setWidth(80);

            TableItem task1 = new TableItem(taskTable, SWT.NONE);
            task1.setText(new String[]{"TODO: Implement feature", "Test.java", "Line 42"});

            TableItem task2 = new TableItem(taskTable, SWT.NONE);
            task2.setText(new String[]{"FIXME: Fix bug", "Main.java", "Line 10"});

        } else if (viewId.contains("ProblemView") || viewId.contains("problems")) {
            // Problems view - create a Tree for showing errors/warnings (like Eclipse markers)
            Tree problemsTree = new Tree(content, SWT.BORDER | SWT.V_SCROLL | SWT.H_SCROLL);
            problemsTree.setData("name", "problemsTree");
            problemsTree.setData("viewId", viewId);

            TreeItem errorsGroup = new TreeItem(problemsTree, SWT.NONE);
            errorsGroup.setText("Errors (2)");
            errorsGroup.setData("type", "errors");

            TreeItem error1 = new TreeItem(errorsGroup, SWT.NONE);
            error1.setText("Syntax error in Test.java line 10");
            error1.setData("severity", "error");

            TreeItem error2 = new TreeItem(errorsGroup, SWT.NONE);
            error2.setText("Missing import in Main.java");
            error2.setData("severity", "error");

            TreeItem warningsGroup = new TreeItem(problemsTree, SWT.NONE);
            warningsGroup.setText("Warnings (1)");
            warningsGroup.setData("type", "warnings");

            TreeItem warning1 = new TreeItem(warningsGroup, SWT.NONE);
            warning1.setText("Unused variable 'temp' in Utils.java");
            warning1.setData("severity", "warning");

            errorsGroup.setExpanded(true);
            warningsGroup.setExpanded(true);

        } else {
            // Default label view
            Label label = new Label(content, SWT.CENTER);
            label.setText(viewInfo != null ? viewInfo.name : "View");
            label.setData("name", "viewLabel");
        }

        return content;
    }

    private void createStatusBar() {
        Composite statusBar = new Composite(shell, SWT.NONE);
        statusBar.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        statusBar.setLayout(new GridLayout(3, false));
        statusBar.setData("name", "statusBar");

        statusLabel = new Label(statusBar, SWT.NONE);
        statusLabel.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        statusLabel.setText("Ready");
        statusLabel.setData("name", "statusLabel");

        Label perspectiveLabel = new Label(statusBar, SWT.NONE);
        perspectiveLabel.setLayoutData(new GridData(SWT.RIGHT, SWT.CENTER, false, false));
        PerspectiveInfo currentPersp = perspectives.get(activePerspectiveId);
        perspectiveLabel.setText(currentPersp != null ? currentPersp.name : "");
        perspectiveLabel.setData("name", "perspectiveLabel");
    }

    // ================== RCP API Methods ==================

    public synchronized String getWorkbenchInfo() {
        return "{\"title\":\"" + workbenchTitle + "\",\"windowCount\":1,\"activePerspective\":\"" + activePerspectiveId + "\"}";
    }

    public synchronized String getActivePerspective() {
        return activePerspectiveId;
    }

    public synchronized java.util.List<String> getAvailablePerspectives() {
        return new ArrayList<>(perspectives.keySet());
    }

    public synchronized java.util.List<String> getOpenPerspectives() {
        // In our mock, all perspectives are "open" (available)
        return new ArrayList<>(perspectives.keySet());
    }

    public synchronized boolean switchPerspective(String perspectiveId) {
        if (perspectives.containsKey(perspectiveId)) {
            activePerspectiveId = perspectiveId;
            display.asyncExec(() -> {
                PerspectiveInfo persp = perspectives.get(perspectiveId);
                shell.setText(workbenchTitle + " - " + persp.name);
                statusLabel.setText("Switched to " + persp.name);
            });
            return true;
        }
        return false;
    }

    public synchronized void openPerspectiveByName(String name) {
        for (PerspectiveInfo persp : perspectives.values()) {
            if (persp.name.equalsIgnoreCase(name) || persp.name.contains(name)) {
                switchPerspective(persp.id);
                return;
            }
        }
    }

    public synchronized void resetPerspective() {
        display.asyncExec(() -> {
            statusLabel.setText("Perspective reset");
        });
    }

    public synchronized void closePerspective(String perspectiveId) {
        // In our mock, we just switch to another perspective
        if (perspectiveId.equals(activePerspectiveId)) {
            for (String id : perspectives.keySet()) {
                if (!id.equals(perspectiveId)) {
                    switchPerspective(id);
                    break;
                }
            }
        }
    }

    public synchronized void closeAllPerspectives() {
        // Reset to default
        activePerspectiveId = "com.testapp.rcp.perspective.main";
    }

    public synchronized boolean showView(String viewId, String secondaryId) {
        ViewInfo view = views.get(viewId);
        if (view == null) return false;

        view.isVisible = true;

        display.syncExec(() -> {
            // Check if tab already exists
            if (!viewTabs.containsKey(viewId)) {
                createViewTab(viewFolder, viewId);
            }
            CTabItem tab = viewTabs.get(viewId);
            if (tab != null && !tab.isDisposed()) {
                tab.getParent().setSelection(tab);
            }
            statusLabel.setText("Showing view: " + view.name);
        });
        return true;
    }

    public synchronized void closeView(String viewId) {
        ViewInfo view = views.get(viewId);
        if (view != null) {
            view.isVisible = false;
        }

        display.asyncExec(() -> {
            CTabItem tab = viewTabs.get(viewId);
            if (tab != null && !tab.isDisposed()) {
                tab.dispose();
                viewTabs.remove(viewId);
            }
        });
    }

    public synchronized void activateView(String viewId) {
        display.asyncExec(() -> {
            CTabItem tab = viewTabs.get(viewId);
            if (tab != null && !tab.isDisposed()) {
                tab.getParent().setSelection(tab);
                tab.getParent().setFocus();
            }
        });
    }

    /**
     * Get a widget from a view by type or locator.
     * @param viewId The view ID
     * @param locator Widget type (e.g., "Tree", "Table") or name locator (e.g., "name:myWidget")
     * @return Map with widget info, or null if not found
     */
    public synchronized Map<String, Object> getViewWidget(String viewId, String locator) {
        System.out.println("[DEBUG] getViewWidget called: viewId=" + viewId + ", locator=" + locator);
        System.out.println("[DEBUG] viewTabs keys: " + viewTabs.keySet());

        final Map<String, Object>[] result = new Map[1];

        display.syncExec(() -> {
            CTabItem tab = viewTabs.get(viewId);
            if (tab == null || tab.isDisposed()) {
                System.out.println("[DEBUG] Tab not found or disposed for viewId: " + viewId);
                result[0] = null;
                return;
            }

            Control content = tab.getControl();
            if (content == null) {
                System.out.println("[DEBUG] Tab has no control");
                result[0] = null;
                return;
            }

            System.out.println("[DEBUG] Content type: " + content.getClass().getSimpleName());

            // Find widget by type or locator
            Control found = findWidgetInControl(content, locator);
            if (found != null) {
                System.out.println("[DEBUG] Found widget: " + found.getClass().getSimpleName());
                result[0] = controlToMap(found);
                return;
            }

            System.out.println("[DEBUG] Widget not found with locator: " + locator);
            result[0] = null;
        });

        return result[0];
    }

    private Control findWidgetInControl(Control control, String locator) {
        // Check if locator is a name locator (e.g., "name:myWidget")
        if (locator != null && locator.startsWith("name:")) {
            String name = locator.substring(5);
            return findControlByName(control, name);
        }

        // Otherwise, treat as widget type (e.g., "Tree", "Table")
        return findControlByType(control, locator);
    }

    private Control findControlByName(Control control, String name) {
        Object controlName = control.getData("name");
        if (name.equals(controlName)) {
            return control;
        }

        if (control instanceof Composite) {
            for (Control child : ((Composite) control).getChildren()) {
                Control found = findControlByName(child, name);
                if (found != null) {
                    return found;
                }
            }
        }

        return null;
    }

    private Control findControlByType(Control control, String typeName) {
        if (typeName == null) {
            return control;
        }

        String simpleClassName = control.getClass().getSimpleName();
        if (simpleClassName.equals(typeName) || simpleClassName.equalsIgnoreCase(typeName)) {
            return control;
        }

        if (control instanceof Composite) {
            for (Control child : ((Composite) control).getChildren()) {
                Control found = findControlByType(child, typeName);
                if (found != null) {
                    return found;
                }
            }
        }

        return null;
    }

    private Map<String, Object> controlToMap(Control control) {
        Map<String, Object> result = new HashMap<>();
        result.put("class", control.getClass().getName());
        result.put("simpleClass", control.getClass().getSimpleName());
        result.put("id", control.hashCode());
        result.put("hashCode", control.hashCode());
        result.put("enabled", control.isEnabled());
        result.put("visible", control.isVisible());

        Object name = control.getData("name");
        if (name != null) {
            result.put("name", name.toString());
        }

        if (control instanceof Text) {
            result.put("text", ((Text) control).getText());
        } else if (control instanceof Label) {
            result.put("text", ((Label) control).getText());
        } else if (control instanceof Button) {
            result.put("text", ((Button) control).getText());
        } else if (control instanceof StyledText) {
            result.put("text", ((StyledText) control).getText());
        }

        return result;
    }

    public synchronized java.util.List<Map<String, Object>> getOpenViews() {
        java.util.List<Map<String, Object>> result = new ArrayList<>();
        for (Map.Entry<String, ViewInfo> entry : views.entrySet()) {
            if (entry.getValue().isVisible) {
                Map<String, Object> viewMap = new HashMap<>();
                viewMap.put("id", entry.getKey());
                viewMap.put("name", entry.getValue().name);
                viewMap.put("visible", true);
                result.add(viewMap);
            }
        }
        return result;
    }

    public synchronized boolean isViewVisible(String viewId) {
        ViewInfo view = views.get(viewId);
        return view != null && view.isVisible;
    }

    public synchronized String getActiveView() {
        // Return the currently selected view
        CTabItem selection = viewFolder.getSelection();
        if (selection != null) {
            return (String) selection.getData("viewId");
        }
        return null;
    }

    public synchronized void minimizeView(String viewId) {
        ViewInfo view = views.get(viewId);
        if (view != null) {
            view.isMinimized = true;
        }
    }

    public synchronized void maximizeView(String viewId) {
        ViewInfo view = views.get(viewId);
        if (view != null) {
            view.isMaximized = true;
        }
    }

    public synchronized void restoreView(String viewId) {
        ViewInfo view = views.get(viewId);
        if (view != null) {
            view.isMinimized = false;
            view.isMaximized = false;
        }
    }

    public synchronized boolean isViewMinimized(String viewId) {
        ViewInfo view = views.get(viewId);
        return view != null && view.isMinimized;
    }

    public synchronized boolean isViewMaximized(String viewId) {
        ViewInfo view = views.get(viewId);
        return view != null && view.isMaximized;
    }

    public synchronized String getViewTitle(String viewId) {
        ViewInfo view = views.get(viewId);
        return view != null ? view.name : null;
    }

    public synchronized void openEditor(String filePath) {
        String editorId = "editor_" + filePath.hashCode();
        String fileName = filePath.substring(filePath.lastIndexOf('/') + 1);

        EditorInfo editor = new EditorInfo(editorId, fileName, filePath);
        editor.content = "// File: " + filePath + "\n// Mock editor content\n\npublic class " +
                        fileName.replace(".java", "") + " {\n    public static void main(String[] args) {\n        System.out.println(\"Hello\");\n    }\n}\n";
        editors.put(filePath, editor);

        display.syncExec(() -> {
            // Check if already open
            if (editorTabs.containsKey(filePath)) {
                CTabItem tab = editorTabs.get(filePath);
                editorFolder.setSelection(tab);
                return;
            }

            CTabItem item = new CTabItem(editorFolder, SWT.CLOSE);
            item.setText(fileName);
            item.setData("editorId", filePath);
            item.setData("name", "editor_" + fileName);

            StyledText text = new StyledText(editorFolder, SWT.BORDER | SWT.V_SCROLL | SWT.H_SCROLL);
            text.setText(editor.content);
            text.setData("name", "editorText");
            text.setData("filePath", filePath);

            text.addModifyListener(e -> {
                EditorInfo ed = editors.get(filePath);
                if (ed != null) {
                    ed.isDirty = true;
                    if (!item.getText().startsWith("*")) {
                        item.setText("*" + item.getText());
                    }
                }
            });

            item.setControl(text);
            editorTabs.put(filePath, item);
            editorFolder.setSelection(item);

            statusLabel.setText("Opened: " + fileName);
        });
    }

    public synchronized void closeEditor(String filePath) {
        // Support lookup by title as well as filePath
        final String pathToRemove = resolveEditorPath(filePath);
        editors.remove(pathToRemove);

        display.asyncExec(() -> {
            CTabItem tab = editorTabs.get(pathToRemove);
            if (tab != null && !tab.isDisposed()) {
                tab.dispose();
                editorTabs.remove(pathToRemove);
            }
        });
    }

    public synchronized void closeAllEditors(boolean save) {
        editors.clear();

        display.asyncExec(() -> {
            for (CTabItem item : editorFolder.getItems()) {
                item.dispose();
            }
            editorTabs.clear();
        });
    }

    /**
     * Get a widget from an editor by type or locator.
     * @param titleOrPath Editor title (e.g., "Test.java") or file path
     * @param locator Widget type (e.g., "StyledText") or name locator (e.g., "name:editorText")
     * @return Map with widget info, or null if not found
     */
    public synchronized Map<String, Object> getEditorWidget(String titleOrPath, String locator) {
        final Map<String, Object>[] result = new Map[1];

        display.syncExec(() -> {
            String actualPath = resolveEditorPath(titleOrPath);
            CTabItem tab = editorTabs.get(actualPath);
            if (tab == null || tab.isDisposed()) {
                result[0] = null;
                return;
            }

            Control content = tab.getControl();
            if (content == null) {
                result[0] = null;
                return;
            }

            // Find widget by type or locator
            Control found = findWidgetInControl(content, locator);
            if (found != null) {
                result[0] = controlToMap(found);
                return;
            }

            result[0] = null;
        });

        return result[0];
    }

    public synchronized void activateEditor(String filePath) {
        // Support lookup by title as well as filePath
        String actualPath = resolveEditorPath(filePath);
        display.asyncExec(() -> {
            CTabItem tab = editorTabs.get(actualPath);
            if (tab != null && !tab.isDisposed()) {
                editorFolder.setSelection(tab);
            }
        });
    }

    // Helper method to resolve editor path from title or filePath
    private String resolveEditorPath(String filePathOrTitle) {
        if (editors.containsKey(filePathOrTitle)) {
            return filePathOrTitle;
        }
        // Try to find by title
        for (Map.Entry<String, EditorInfo> entry : editors.entrySet()) {
            if (entry.getValue().title.equals(filePathOrTitle)) {
                return entry.getKey();
            }
        }
        return filePathOrTitle; // Return original if not found
    }

    public synchronized void saveEditor(String filePath) {
        // Support lookup by title as well as filePath
        String actualPath = resolveEditorPath(filePath);
        EditorInfo editor = editors.get(actualPath);
        if (editor != null) {
            editor.isDirty = false;

            display.asyncExec(() -> {
                CTabItem tab = editorTabs.get(actualPath);
                if (tab != null && !tab.isDisposed()) {
                    String title = tab.getText();
                    if (title.startsWith("*")) {
                        tab.setText(title.substring(1));
                    }
                }
            });
        }
    }

    public synchronized void saveAllEditors() {
        for (EditorInfo editor : editors.values()) {
            editor.isDirty = false;
        }

        display.asyncExec(() -> {
            for (CTabItem item : editorFolder.getItems()) {
                String title = item.getText();
                if (title.startsWith("*")) {
                    item.setText(title.substring(1));
                }
            }
        });
    }

    private void saveActiveEditor() {
        CTabItem selection = editorFolder.getSelection();
        if (selection != null) {
            String filePath = (String) selection.getData("editorId");
            if (filePath != null) {
                saveEditor(filePath);
            }
        }
    }

    public synchronized java.util.List<Map<String, Object>> getOpenEditors() {
        java.util.List<Map<String, Object>> result = new ArrayList<>();
        for (Map.Entry<String, EditorInfo> entry : editors.entrySet()) {
            Map<String, Object> editorMap = new HashMap<>();
            editorMap.put("id", entry.getKey());
            editorMap.put("title", entry.getValue().title);
            editorMap.put("filePath", entry.getValue().filePath);
            editorMap.put("dirty", entry.getValue().isDirty);
            result.add(editorMap);
        }
        return result;
    }

    public synchronized String getActiveEditor() {
        CTabItem selection = editorFolder.getSelection();
        if (selection != null) {
            return (String) selection.getData("editorId");
        }
        return null;
    }

    public synchronized boolean isEditorOpen(String filePath) {
        // Support lookup by title as well as filePath
        String actualPath = resolveEditorPath(filePath);
        return editors.containsKey(actualPath);
    }

    public synchronized boolean isEditorDirty(String filePath) {
        // Support lookup by title as well as filePath
        String actualPath = resolveEditorPath(filePath);
        EditorInfo editor = editors.get(actualPath);
        return editor != null && editor.isDirty;
    }

    public synchronized String getEditorContent(String filePath) {
        // Support lookup by title as well as filePath
        String actualPath = resolveEditorPath(filePath);
        EditorInfo editor = editors.get(actualPath);
        return editor != null ? editor.content : null;
    }

    public synchronized int getDirtyEditorCount() {
        int count = 0;
        for (EditorInfo editor : editors.values()) {
            if (editor.isDirty) count++;
        }
        return count;
    }

    public synchronized void enterTextInEditor(String text) {
        display.asyncExec(() -> {
            CTabItem selection = editorFolder.getSelection();
            if (selection != null) {
                Control control = selection.getControl();
                if (control instanceof StyledText) {
                    StyledText styledText = (StyledText) control;
                    styledText.append("\n" + text);

                    String filePath = (String) selection.getData("editorId");
                    EditorInfo editor = editors.get(filePath);
                    if (editor != null) {
                        editor.content = styledText.getText();
                        editor.isDirty = true;
                    }
                }
            }
        });
    }

    public synchronized void executeCommand(String commandId) {
        display.asyncExec(() -> {
            statusLabel.setText("Executed command: " + commandId);
        });
    }

    public synchronized void executeMenu(String menuPath) {
        display.asyncExec(() -> {
            statusLabel.setText("Executed menu: " + menuPath);
        });
    }

    public synchronized void clickToolbarItem(String tooltip) {
        display.asyncExec(() -> {
            // Find toolbar item by tooltip
            for (ToolItem item : mainToolBar.getItems()) {
                String itemTooltip = item.getToolTipText();
                if (itemTooltip != null && itemTooltip.equals(tooltip)) {
                    // Simulate click
                    item.notifyListeners(SWT.Selection, new Event());
                    statusLabel.setText("Clicked toolbar: " + tooltip);
                    return;
                }
            }
            statusLabel.setText("Toolbar item not found: " + tooltip);
        });
    }

    public synchronized void openPreferencesDialog() {
        display.asyncExec(() -> {
            showPreferencesDialog();
        });
    }

    public synchronized void navigateToPreferencePage(String path) {
        // This is a simplified implementation - in real Eclipse this would navigate the tree
        display.asyncExec(() -> {
            statusLabel.setText("Navigated to preference: " + path);
        });
    }

    public synchronized java.util.List<String> getOpenDialogs() {
        return new ArrayList<>(openDialogs);
    }

    public synchronized void pressButton(String label) {
        display.asyncExec(() -> {
            Shell activeShell = display.getActiveShell();
            if (activeShell != null && activeShell != shell) {
                // Find button in dialog and click it
                Button button = findButtonByLabel(activeShell, label);
                if (button != null) {
                    button.notifyListeners(SWT.Selection, new Event());
                }
            }
        });
    }

    private Button findButtonByLabel(Composite parent, String label) {
        for (Control child : parent.getChildren()) {
            if (child instanceof Button) {
                Button button = (Button) child;
                if (label.equals(button.getText())) {
                    return button;
                }
            } else if (child instanceof Composite) {
                Button found = findButtonByLabel((Composite) child, label);
                if (found != null) return found;
            }
        }
        return null;
    }

    public synchronized void closeActiveDialog() {
        display.asyncExec(() -> {
            Shell activeShell = display.getActiveShell();
            if (activeShell != null && activeShell != shell) {
                activeShell.close();
            }
        });
    }

    public synchronized String getWorkbenchTitle() {
        return shell.getText();
    }

    public synchronized String getWorkbenchState() {
        return "{\"ready\":true,\"starting\":false,\"closing\":false}";
    }

    public synchronized int getWorkbenchWindowCount() {
        return 1;
    }

    // ================== Dialog Methods ==================

    private void showPreferencesDialog() {
        Shell dialog = new Shell(shell, SWT.DIALOG_TRIM | SWT.APPLICATION_MODAL | SWT.RESIZE);
        dialog.setText("Preferences");
        dialog.setSize(600, 400);
        dialog.setLayout(new GridLayout(2, false));
        dialog.setData("name", "preferencesDialog");

        openDialogs.add("Preferences");

        // Left side - tree of categories
        Tree categoryTree = new Tree(dialog, SWT.BORDER);
        categoryTree.setLayoutData(new GridData(SWT.FILL, SWT.FILL, false, true));
        categoryTree.setData("name", "preferencesTree");

        TreeItem general = new TreeItem(categoryTree, SWT.NONE);
        general.setText("General");

        TreeItem appearance = new TreeItem(general, SWT.NONE);
        appearance.setText("Appearance");

        TreeItem editors = new TreeItem(general, SWT.NONE);
        editors.setText("Editors");

        TreeItem java = new TreeItem(categoryTree, SWT.NONE);
        java.setText("Java");

        TreeItem compiler = new TreeItem(java, SWT.NONE);
        compiler.setText("Compiler");

        general.setExpanded(true);

        // Right side - preferences content
        Composite content = new Composite(dialog, SWT.BORDER);
        content.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
        content.setLayout(new GridLayout(2, false));

        Label label = new Label(content, SWT.NONE);
        label.setText("Select a category to configure");

        // Buttons
        Composite buttons = new Composite(dialog, SWT.NONE);
        buttons.setLayoutData(new GridData(SWT.RIGHT, SWT.CENTER, false, false, 2, 1));
        buttons.setLayout(new RowLayout());

        Button okButton = new Button(buttons, SWT.PUSH);
        okButton.setText("OK");
        okButton.setData("name", "buttonOK");
        okButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                openDialogs.remove("Preferences");
                dialog.close();
            }
        });

        Button cancelButton = new Button(buttons, SWT.PUSH);
        cancelButton.setText("Cancel");
        cancelButton.setData("name", "buttonCancel");
        cancelButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                openDialogs.remove("Preferences");
                dialog.close();
            }
        });

        Button applyButton = new Button(buttons, SWT.PUSH);
        applyButton.setText("Apply");
        applyButton.setData("name", "buttonApply");

        dialog.addDisposeListener(e -> openDialogs.remove("Preferences"));

        dialog.open();
    }

    private void showNewProjectDialog() {
        Shell dialog = new Shell(shell, SWT.DIALOG_TRIM | SWT.APPLICATION_MODAL);
        dialog.setText("New Project");
        dialog.setSize(400, 300);
        dialog.setLayout(new GridLayout(2, false));
        dialog.setData("name", "newProjectDialog");

        openDialogs.add("New Project");

        Label nameLabel = new Label(dialog, SWT.NONE);
        nameLabel.setText("Project name:");

        Text nameText = new Text(dialog, SWT.BORDER);
        nameText.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        nameText.setData("name", "projectNameText");

        Label locationLabel = new Label(dialog, SWT.NONE);
        locationLabel.setText("Location:");

        Text locationText = new Text(dialog, SWT.BORDER);
        locationText.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        locationText.setText("/workspace");
        locationText.setData("name", "projectLocationText");

        // Spacer
        new Label(dialog, SWT.NONE);
        new Label(dialog, SWT.NONE);

        Composite buttons = new Composite(dialog, SWT.NONE);
        buttons.setLayoutData(new GridData(SWT.RIGHT, SWT.CENTER, false, false, 2, 1));
        buttons.setLayout(new RowLayout());

        Button finishButton = new Button(buttons, SWT.PUSH);
        finishButton.setText("Finish");
        finishButton.setData("name", "buttonFinish");
        finishButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                openDialogs.remove("New Project");
                dialog.close();
                statusLabel.setText("Created project: " + nameText.getText());
            }
        });

        Button cancelButton = new Button(buttons, SWT.PUSH);
        cancelButton.setText("Cancel");
        cancelButton.setData("name", "buttonCancel");
        cancelButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                openDialogs.remove("New Project");
                dialog.close();
            }
        });

        dialog.addDisposeListener(e -> openDialogs.remove("New Project"));

        dialog.open();
    }

    private void showAboutDialog() {
        Shell dialog = new Shell(shell, SWT.DIALOG_TRIM | SWT.APPLICATION_MODAL);
        dialog.setText("About");
        dialog.setSize(300, 200);
        dialog.setLayout(new GridLayout(1, false));
        dialog.setData("name", "aboutDialog");

        openDialogs.add("About");

        Label title = new Label(dialog, SWT.CENTER);
        title.setText("Mock RCP Application");
        title.setLayoutData(new GridData(SWT.CENTER, SWT.CENTER, true, false));

        Label version = new Label(dialog, SWT.CENTER);
        version.setText("Version 1.0.0");
        version.setLayoutData(new GridData(SWT.CENTER, SWT.CENTER, true, false));

        Label description = new Label(dialog, SWT.CENTER | SWT.WRAP);
        description.setText("A mock Eclipse RCP application for\nRobot Framework testing.");
        description.setLayoutData(new GridData(SWT.CENTER, SWT.CENTER, true, true));

        Button okButton = new Button(dialog, SWT.PUSH);
        okButton.setText("OK");
        okButton.setLayoutData(new GridData(SWT.CENTER, SWT.CENTER, true, false));
        okButton.setData("name", "buttonOK");
        okButton.addSelectionListener(new SelectionAdapter() {
            @Override
            public void widgetSelected(SelectionEvent e) {
                openDialogs.remove("About");
                dialog.close();
            }
        });

        dialog.addDisposeListener(e -> openDialogs.remove("About"));

        dialog.open();
    }

    // ================== Data Classes ==================

    static class PerspectiveInfo {
        String id;
        String name;
        String description;
        boolean isOpen = true;

        PerspectiveInfo(String id, String name, String description) {
            this.id = id;
            this.name = name;
            this.description = description;
        }
    }

    static class ViewInfo {
        String id;
        String name;
        String description;
        boolean isVisible = false;
        boolean isMinimized = false;
        boolean isMaximized = false;

        ViewInfo(String id, String name, String description) {
            this.id = id;
            this.name = name;
            this.description = description;
        }
    }

    static class EditorInfo {
        String id;
        String title;
        String filePath;
        String content = "";
        boolean isDirty = false;

        EditorInfo(String id, String title, String filePath) {
            this.id = id;
            this.title = title;
            this.filePath = filePath;
        }
    }
}
