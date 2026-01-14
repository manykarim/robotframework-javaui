package com.testapp.rcp.views;

import org.eclipse.jface.viewers.ArrayContentProvider;
import org.eclipse.jface.viewers.CheckboxTableViewer;
import org.eclipse.jface.viewers.ColumnLabelProvider;
import org.eclipse.jface.viewers.TableViewerColumn;
import org.eclipse.swt.SWT;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Button;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Table;
import org.eclipse.ui.part.ViewPart;

/**
 * Tasks View - shows a list of tasks with checkboxes.
 *
 * View ID: com.testapp.rcp.views.tasks
 *
 * Features:
 * - Checkbox table for task completion
 * - Priority column with different values
 * - Add/Remove task buttons
 */
public class TasksView extends ViewPart {

    /** View ID */
    public static final String ID = "com.testapp.rcp.views.tasks";

    /** Table viewer with checkboxes */
    private CheckboxTableViewer tableViewer;

    /** Task list */
    private java.util.List<Task> tasks = new java.util.ArrayList<>();

    @Override
    public void createPartControl(Composite parent) {
        Composite composite = new Composite(parent, SWT.NONE);
        composite.setLayout(new GridLayout(1, false));

        // Toolbar
        Composite toolbar = new Composite(composite, SWT.NONE);
        toolbar.setLayout(new GridLayout(3, false));
        toolbar.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));

        Button addButton = new Button(toolbar, SWT.PUSH);
        addButton.setText("Add Task");
        addButton.setData("name", "addTaskButton");
        addButton.addListener(SWT.Selection, e -> addTask());

        Button removeButton = new Button(toolbar, SWT.PUSH);
        removeButton.setText("Remove");
        removeButton.setData("name", "removeTaskButton");
        removeButton.addListener(SWT.Selection, e -> removeSelectedTask());

        Button completeButton = new Button(toolbar, SWT.PUSH);
        completeButton.setText("Complete All");
        completeButton.setData("name", "completeAllButton");
        completeButton.addListener(SWT.Selection, e -> completeAllTasks());

        // Table
        tableViewer = CheckboxTableViewer.newCheckList(composite,
            SWT.MULTI | SWT.H_SCROLL | SWT.V_SCROLL | SWT.FULL_SELECTION | SWT.BORDER);

        Table table = tableViewer.getTable();
        table.setHeaderVisible(true);
        table.setLinesVisible(true);
        table.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
        table.setData("name", "tasksTable");

        createColumns();

        tableViewer.setContentProvider(ArrayContentProvider.getInstance());
        initializeTasks();
        tableViewer.setInput(tasks);

        // Handle checkbox changes
        tableViewer.addCheckStateListener(event -> {
            Task task = (Task) event.getElement();
            task.setCompleted(event.getChecked());
            System.out.println("[TasksView] Task " + (event.getChecked() ? "completed" : "uncompleted") +
                ": " + task.getDescription());
        });
    }

    private void createColumns() {
        // Description column
        TableViewerColumn descCol = new TableViewerColumn(tableViewer, SWT.NONE);
        descCol.getColumn().setText("Description");
        descCol.getColumn().setWidth(300);
        descCol.setLabelProvider(new ColumnLabelProvider() {
            @Override
            public String getText(Object element) {
                return ((Task) element).getDescription();
            }
        });

        // Priority column
        TableViewerColumn prioCol = new TableViewerColumn(tableViewer, SWT.NONE);
        prioCol.getColumn().setText("Priority");
        prioCol.getColumn().setWidth(80);
        prioCol.setLabelProvider(new ColumnLabelProvider() {
            @Override
            public String getText(Object element) {
                return ((Task) element).getPriority();
            }
        });

        // Status column
        TableViewerColumn statusCol = new TableViewerColumn(tableViewer, SWT.NONE);
        statusCol.getColumn().setText("Status");
        statusCol.getColumn().setWidth(100);
        statusCol.setLabelProvider(new ColumnLabelProvider() {
            @Override
            public String getText(Object element) {
                return ((Task) element).isCompleted() ? "Done" : "Pending";
            }
        });
    }

    private void initializeTasks() {
        tasks.add(new Task("Implement login feature", "High", false));
        tasks.add(new Task("Write unit tests", "High", false));
        tasks.add(new Task("Update documentation", "Medium", false));
        tasks.add(new Task("Review pull request", "Medium", true));
        tasks.add(new Task("Fix CSS styling", "Low", false));
        tasks.add(new Task("Optimize database queries", "High", false));
    }

    private void addTask() {
        int num = tasks.size() + 1;
        Task task = new Task("New Task " + num, "Medium", false);
        tasks.add(task);
        tableViewer.refresh();
        System.out.println("[TasksView] Added task: " + task.getDescription());
    }

    private void removeSelectedTask() {
        Object[] selected = tableViewer.getCheckedElements();
        for (Object obj : selected) {
            tasks.remove(obj);
            System.out.println("[TasksView] Removed task: " + ((Task) obj).getDescription());
        }
        tableViewer.refresh();
    }

    private void completeAllTasks() {
        for (Task task : tasks) {
            task.setCompleted(true);
            tableViewer.setChecked(task, true);
        }
        tableViewer.refresh();
        System.out.println("[TasksView] All tasks marked complete");
    }

    @Override
    public void setFocus() {
        tableViewer.getControl().setFocus();
    }

    public CheckboxTableViewer getTableViewer() {
        return tableViewer;
    }

    // ========== Inner Classes ==========

    public static class Task {
        private String description;
        private String priority;
        private boolean completed;

        public Task(String description, String priority, boolean completed) {
            this.description = description;
            this.priority = priority;
            this.completed = completed;
        }

        public String getDescription() { return description; }
        public void setDescription(String description) { this.description = description; }
        public String getPriority() { return priority; }
        public void setPriority(String priority) { this.priority = priority; }
        public boolean isCompleted() { return completed; }
        public void setCompleted(boolean completed) { this.completed = completed; }
    }
}
