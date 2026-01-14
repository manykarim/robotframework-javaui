package com.testapp.rcp.views;

import org.eclipse.jface.action.MenuManager;
import org.eclipse.jface.viewers.IStructuredSelection;
import org.eclipse.jface.viewers.ITreeContentProvider;
import org.eclipse.jface.viewers.LabelProvider;
import org.eclipse.jface.viewers.TreeViewer;
import org.eclipse.jface.viewers.Viewer;
import org.eclipse.swt.SWT;
import org.eclipse.swt.graphics.Image;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Menu;
import org.eclipse.ui.ISharedImages;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.part.ViewPart;

/**
 * Navigator View - contains a Tree widget showing a project/file structure.
 *
 * View ID: com.testapp.rcp.views.navigator
 *
 * Features:
 * - Tree widget with expandable nodes
 * - Context menu support
 * - Selection events
 * - Custom content provider with sample data
 */
public class NavigatorView extends ViewPart {

    /** View ID */
    public static final String ID = "com.testapp.rcp.views.navigator";

    /** Tree viewer */
    private TreeViewer treeViewer;

    @Override
    public void createPartControl(Composite parent) {
        // Create the tree viewer
        treeViewer = new TreeViewer(parent, SWT.MULTI | SWT.H_SCROLL | SWT.V_SCROLL | SWT.BORDER);
        treeViewer.getTree().setData("name", "navigatorTree");

        // Set content provider
        treeViewer.setContentProvider(new NavigatorContentProvider());

        // Set label provider
        treeViewer.setLabelProvider(new NavigatorLabelProvider());

        // Set input (sample data)
        treeViewer.setInput(createSampleData());

        // Expand first level
        treeViewer.expandToLevel(2);

        // Create context menu
        createContextMenu();

        // Add selection listener
        treeViewer.addSelectionChangedListener(event -> {
            IStructuredSelection selection = (IStructuredSelection) event.getSelection();
            if (!selection.isEmpty()) {
                TreeNode node = (TreeNode) selection.getFirstElement();
                System.out.println("[NavigatorView] Selected: " + node.getName());
            }
        });

        // Add double-click listener
        treeViewer.addDoubleClickListener(event -> {
            IStructuredSelection selection = (IStructuredSelection) event.getSelection();
            if (!selection.isEmpty()) {
                TreeNode node = (TreeNode) selection.getFirstElement();
                System.out.println("[NavigatorView] Double-clicked: " + node.getName());
                // Toggle expansion on double-click
                treeViewer.setExpandedState(node, !treeViewer.getExpandedState(node));
            }
        });
    }

    private void createContextMenu() {
        MenuManager menuMgr = new MenuManager("#PopupMenu", "com.testapp.rcp.views.navigator.popup");
        Menu menu = menuMgr.createContextMenu(treeViewer.getControl());
        treeViewer.getControl().setMenu(menu);
        getSite().registerContextMenu("com.testapp.rcp.views.navigator.popup", menuMgr, treeViewer);
    }

    private TreeNode[] createSampleData() {
        // Create sample project structure
        TreeNode project1 = new TreeNode("TestProject", TreeNode.TYPE_PROJECT);
        TreeNode src = new TreeNode("src", TreeNode.TYPE_FOLDER);
        TreeNode main = new TreeNode("main", TreeNode.TYPE_FOLDER);
        TreeNode test = new TreeNode("test", TreeNode.TYPE_FOLDER);
        TreeNode mainJava = new TreeNode("Main.java", TreeNode.TYPE_FILE);
        TreeNode helperJava = new TreeNode("Helper.java", TreeNode.TYPE_FILE);
        TreeNode testJava = new TreeNode("MainTest.java", TreeNode.TYPE_FILE);
        TreeNode readme = new TreeNode("README.txt", TreeNode.TYPE_FILE);
        TreeNode config = new TreeNode("config.xml", TreeNode.TYPE_FILE);

        project1.addChild(src);
        project1.addChild(readme);
        project1.addChild(config);
        src.addChild(main);
        src.addChild(test);
        main.addChild(mainJava);
        main.addChild(helperJava);
        test.addChild(testJava);

        TreeNode project2 = new TreeNode("DemoProject", TreeNode.TYPE_PROJECT);
        TreeNode demoSrc = new TreeNode("src", TreeNode.TYPE_FOLDER);
        TreeNode demoFile = new TreeNode("Demo.java", TreeNode.TYPE_FILE);
        TreeNode dataFile = new TreeNode("data.form", TreeNode.TYPE_FILE);

        project2.addChild(demoSrc);
        project2.addChild(dataFile);
        demoSrc.addChild(demoFile);

        TreeNode project3 = new TreeNode("SampleProject", TreeNode.TYPE_PROJECT);
        TreeNode sampleDoc = new TreeNode("sample.txt", TreeNode.TYPE_FILE);
        project3.addChild(sampleDoc);

        return new TreeNode[] { project1, project2, project3 };
    }

    @Override
    public void setFocus() {
        treeViewer.getControl().setFocus();
    }

    /**
     * Get the tree viewer for testing purposes.
     */
    public TreeViewer getTreeViewer() {
        return treeViewer;
    }

    /**
     * Refresh the tree viewer.
     */
    public void refresh() {
        treeViewer.refresh();
    }

    // ========== Inner Classes ==========

    /**
     * Tree node model class.
     */
    public static class TreeNode {
        public static final int TYPE_PROJECT = 0;
        public static final int TYPE_FOLDER = 1;
        public static final int TYPE_FILE = 2;

        private String name;
        private int type;
        private TreeNode parent;
        private java.util.List<TreeNode> children = new java.util.ArrayList<>();

        public TreeNode(String name, int type) {
            this.name = name;
            this.type = type;
        }

        public String getName() {
            return name;
        }

        public int getType() {
            return type;
        }

        public TreeNode getParent() {
            return parent;
        }

        public TreeNode[] getChildren() {
            return children.toArray(new TreeNode[0]);
        }

        public boolean hasChildren() {
            return !children.isEmpty();
        }

        public void addChild(TreeNode child) {
            children.add(child);
            child.parent = this;
        }

        @Override
        public String toString() {
            return name;
        }
    }

    /**
     * Content provider for the navigator tree.
     */
    private static class NavigatorContentProvider implements ITreeContentProvider {
        @Override
        public Object[] getElements(Object inputElement) {
            return (Object[]) inputElement;
        }

        @Override
        public Object[] getChildren(Object parentElement) {
            return ((TreeNode) parentElement).getChildren();
        }

        @Override
        public Object getParent(Object element) {
            return ((TreeNode) element).getParent();
        }

        @Override
        public boolean hasChildren(Object element) {
            return ((TreeNode) element).hasChildren();
        }

        @Override
        public void dispose() {
        }

        @Override
        public void inputChanged(Viewer viewer, Object oldInput, Object newInput) {
        }
    }

    /**
     * Label provider for the navigator tree.
     */
    private static class NavigatorLabelProvider extends LabelProvider {
        @Override
        public String getText(Object element) {
            return ((TreeNode) element).getName();
        }

        @Override
        public Image getImage(Object element) {
            TreeNode node = (TreeNode) element;
            ISharedImages sharedImages = PlatformUI.getWorkbench().getSharedImages();

            switch (node.getType()) {
                case TreeNode.TYPE_PROJECT:
                    return sharedImages.getImage(ISharedImages.IMG_OBJ_PROJECT);
                case TreeNode.TYPE_FOLDER:
                    return sharedImages.getImage(ISharedImages.IMG_OBJ_FOLDER);
                case TreeNode.TYPE_FILE:
                default:
                    return sharedImages.getImage(ISharedImages.IMG_OBJ_FILE);
            }
        }
    }
}
