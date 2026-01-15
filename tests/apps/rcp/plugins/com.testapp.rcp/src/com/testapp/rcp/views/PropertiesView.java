package com.testapp.rcp.views;

import org.eclipse.jface.viewers.ArrayContentProvider;
import org.eclipse.jface.viewers.ColumnLabelProvider;
import org.eclipse.jface.viewers.IStructuredSelection;
import org.eclipse.jface.viewers.TableViewer;
import org.eclipse.jface.viewers.TableViewerColumn;
import org.eclipse.swt.SWT;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Label;
import org.eclipse.swt.widgets.Table;
import org.eclipse.swt.widgets.TableColumn;
import org.eclipse.ui.part.ViewPart;

/**
 * Properties View - contains a Table widget showing property name/value pairs.
 *
 * View ID: com.testapp.rcp.views.properties
 *
 * Features:
 * - Table widget with two columns (Name, Value)
 * - Sortable columns
 * - Selection events
 * - Multiple instances supported (secondary ID)
 */
public class PropertiesView extends ViewPart {

    /** View ID */
    public static final String ID = "com.testapp.rcp.views.properties";

    /** Table viewer */
    private TableViewer tableViewer;

    /** Header label */
    private Label headerLabel;

    @Override
    public void createPartControl(Composite parent) {
        // Create main composite with grid layout
        Composite composite = new Composite(parent, SWT.NONE);
        composite.setLayout(new GridLayout(1, false));

        // Header label
        headerLabel = new Label(composite, SWT.NONE);
        headerLabel.setText("Properties:");
        headerLabel.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        headerLabel.setData("name", "propertiesHeader");

        // Create table viewer
        tableViewer = new TableViewer(composite,
            SWT.MULTI | SWT.H_SCROLL | SWT.V_SCROLL | SWT.FULL_SELECTION | SWT.BORDER);

        Table table = tableViewer.getTable();
        table.setHeaderVisible(true);
        table.setLinesVisible(true);
        table.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
        table.setData("name", "propertiesTable");

        // Create columns
        createColumns();

        // Set content provider
        tableViewer.setContentProvider(ArrayContentProvider.getInstance());

        // Set input (sample data)
        tableViewer.setInput(createSampleData());

        // Add selection listener
        tableViewer.addSelectionChangedListener(event -> {
            IStructuredSelection selection = (IStructuredSelection) event.getSelection();
            if (!selection.isEmpty()) {
                Property prop = (Property) selection.getFirstElement();
                System.out.println("[PropertiesView] Selected: " + prop.getName() + " = " + prop.getValue());
            }
        });

        // Add double-click listener
        tableViewer.addDoubleClickListener(event -> {
            IStructuredSelection selection = (IStructuredSelection) event.getSelection();
            if (!selection.isEmpty()) {
                Property prop = (Property) selection.getFirstElement();
                System.out.println("[PropertiesView] Double-clicked property: " + prop.getName());
            }
        });
    }

    private void createColumns() {
        // Name column
        TableViewerColumn nameCol = new TableViewerColumn(tableViewer, SWT.NONE);
        TableColumn nameColumn = nameCol.getColumn();
        nameColumn.setText("Name");
        nameColumn.setWidth(200);
        nameColumn.setResizable(true);
        nameColumn.setMoveable(true);
        nameCol.setLabelProvider(new ColumnLabelProvider() {
            @Override
            public String getText(Object element) {
                return ((Property) element).getName();
            }
        });

        // Value column
        TableViewerColumn valueCol = new TableViewerColumn(tableViewer, SWT.NONE);
        TableColumn valueColumn = valueCol.getColumn();
        valueColumn.setText("Value");
        valueColumn.setWidth(300);
        valueColumn.setResizable(true);
        valueColumn.setMoveable(true);
        valueCol.setLabelProvider(new ColumnLabelProvider() {
            @Override
            public String getText(Object element) {
                return ((Property) element).getValue();
            }
        });

        // Type column (optional)
        TableViewerColumn typeCol = new TableViewerColumn(tableViewer, SWT.NONE);
        TableColumn typeColumn = typeCol.getColumn();
        typeColumn.setText("Type");
        typeColumn.setWidth(100);
        typeColumn.setResizable(true);
        typeColumn.setMoveable(true);
        typeCol.setLabelProvider(new ColumnLabelProvider() {
            @Override
            public String getText(Object element) {
                return ((Property) element).getType();
            }
        });
    }

    private Property[] createSampleData() {
        return new Property[] {
            new Property("name", "TestProject", "String"),
            new Property("version", "1.0.0", "String"),
            new Property("author", "Robot Framework", "String"),
            new Property("created", "2024-01-15", "Date"),
            new Property("modified", "2024-06-20", "Date"),
            new Property("size", "1024 KB", "Size"),
            new Property("files", "15", "Integer"),
            new Property("status", "Active", "Enum"),
            new Property("encoding", "UTF-8", "String"),
            new Property("readonly", "false", "Boolean"),
            new Property("path", "/workspace/TestProject", "Path"),
            new Property("type", "Java Project", "String"),
        };
    }

    @Override
    public void setFocus() {
        tableViewer.getControl().setFocus();
    }

    /**
     * Get the table viewer for testing purposes.
     */
    public TableViewer getTableViewer() {
        return tableViewer;
    }

    /**
     * Refresh the table viewer.
     */
    public void refresh() {
        tableViewer.refresh();
    }

    /**
     * Update header text.
     */
    public void setHeader(String text) {
        headerLabel.setText(text);
    }

    /**
     * Get row count.
     */
    public int getRowCount() {
        return tableViewer.getTable().getItemCount();
    }

    /**
     * Get cell value.
     */
    public String getCellValue(int row, int column) {
        Table table = tableViewer.getTable();
        if (row >= 0 && row < table.getItemCount()) {
            return table.getItem(row).getText(column);
        }
        return null;
    }

    // ========== Inner Classes ==========

    /**
     * Property model class.
     */
    public static class Property {
        private String name;
        private String value;
        private String type;

        public Property(String name, String value, String type) {
            this.name = name;
            this.value = value;
            this.type = type;
        }

        public String getName() {
            return name;
        }

        public void setName(String name) {
            this.name = name;
        }

        public String getValue() {
            return value;
        }

        public void setValue(String value) {
            this.value = value;
        }

        public String getType() {
            return type;
        }

        public void setType(String type) {
            this.type = type;
        }

        @Override
        public String toString() {
            return name + "=" + value;
        }
    }
}
