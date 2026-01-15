package com.testapp.rcp.views;

import org.eclipse.jface.viewers.IStructuredSelection;
import org.eclipse.jface.viewers.ITreeContentProvider;
import org.eclipse.jface.viewers.LabelProvider;
import org.eclipse.jface.viewers.TreeViewer;
import org.eclipse.jface.viewers.Viewer;
import org.eclipse.swt.SWT;
import org.eclipse.swt.graphics.Image;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.ui.ISharedImages;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.part.ViewPart;

/**
 * Outline View - shows document structure in a tree.
 *
 * View ID: com.testapp.rcp.views.outline
 *
 * Features:
 * - Tree widget showing document outline
 * - Simulates code structure (classes, methods, fields)
 */
public class OutlineView extends ViewPart {

    /** View ID */
    public static final String ID = "com.testapp.rcp.views.outline";

    /** Tree viewer */
    private TreeViewer treeViewer;

    @Override
    public void createPartControl(Composite parent) {
        treeViewer = new TreeViewer(parent, SWT.MULTI | SWT.H_SCROLL | SWT.V_SCROLL | SWT.BORDER);
        treeViewer.getTree().setData("name", "outlineTree");

        treeViewer.setContentProvider(new OutlineContentProvider());
        treeViewer.setLabelProvider(new OutlineLabelProvider());
        treeViewer.setInput(createSampleOutline());
        treeViewer.expandAll();

        treeViewer.addSelectionChangedListener(event -> {
            IStructuredSelection selection = (IStructuredSelection) event.getSelection();
            if (!selection.isEmpty()) {
                OutlineNode node = (OutlineNode) selection.getFirstElement();
                System.out.println("[OutlineView] Selected: " + node.getName());
            }
        });
    }

    private OutlineNode[] createSampleOutline() {
        // Simulate Java class outline
        OutlineNode classNode = new OutlineNode("TestClass", OutlineNode.TYPE_CLASS);

        OutlineNode field1 = new OutlineNode("name : String", OutlineNode.TYPE_FIELD);
        OutlineNode field2 = new OutlineNode("count : int", OutlineNode.TYPE_FIELD);
        OutlineNode constructor = new OutlineNode("TestClass()", OutlineNode.TYPE_METHOD);
        OutlineNode method1 = new OutlineNode("getName() : String", OutlineNode.TYPE_METHOD);
        OutlineNode method2 = new OutlineNode("setName(String)", OutlineNode.TYPE_METHOD);
        OutlineNode method3 = new OutlineNode("process()", OutlineNode.TYPE_METHOD);

        classNode.addChild(field1);
        classNode.addChild(field2);
        classNode.addChild(constructor);
        classNode.addChild(method1);
        classNode.addChild(method2);
        classNode.addChild(method3);

        // Imports section
        OutlineNode imports = new OutlineNode("import declarations", OutlineNode.TYPE_IMPORT);

        return new OutlineNode[] { imports, classNode };
    }

    @Override
    public void setFocus() {
        treeViewer.getControl().setFocus();
    }

    public TreeViewer getTreeViewer() {
        return treeViewer;
    }

    // ========== Inner Classes ==========

    public static class OutlineNode {
        public static final int TYPE_CLASS = 0;
        public static final int TYPE_METHOD = 1;
        public static final int TYPE_FIELD = 2;
        public static final int TYPE_IMPORT = 3;

        private String name;
        private int type;
        private OutlineNode parent;
        private java.util.List<OutlineNode> children = new java.util.ArrayList<>();

        public OutlineNode(String name, int type) {
            this.name = name;
            this.type = type;
        }

        public String getName() { return name; }
        public int getType() { return type; }
        public OutlineNode getParent() { return parent; }
        public OutlineNode[] getChildren() { return children.toArray(new OutlineNode[0]); }
        public boolean hasChildren() { return !children.isEmpty(); }

        public void addChild(OutlineNode child) {
            children.add(child);
            child.parent = this;
        }
    }

    private static class OutlineContentProvider implements ITreeContentProvider {
        @Override
        public Object[] getElements(Object inputElement) {
            return (Object[]) inputElement;
        }

        @Override
        public Object[] getChildren(Object parentElement) {
            return ((OutlineNode) parentElement).getChildren();
        }

        @Override
        public Object getParent(Object element) {
            return ((OutlineNode) element).getParent();
        }

        @Override
        public boolean hasChildren(Object element) {
            return ((OutlineNode) element).hasChildren();
        }

        @Override
        public void dispose() {}

        @Override
        public void inputChanged(Viewer viewer, Object oldInput, Object newInput) {}
    }

    private static class OutlineLabelProvider extends LabelProvider {
        @Override
        public String getText(Object element) {
            return ((OutlineNode) element).getName();
        }

        @Override
        public Image getImage(Object element) {
            ISharedImages images = PlatformUI.getWorkbench().getSharedImages();
            // Use generic images - in real app would use JDT images
            return images.getImage(ISharedImages.IMG_OBJ_ELEMENT);
        }
    }
}
