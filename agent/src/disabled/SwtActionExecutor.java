package com.robotframework.swt;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import com.google.gson.JsonPrimitive;

import org.eclipse.swt.SWT;
import org.eclipse.swt.custom.CCombo;
import org.eclipse.swt.custom.CTabFolder;
import org.eclipse.swt.custom.CTabItem;
import org.eclipse.swt.graphics.GC;
import org.eclipse.swt.graphics.Image;
import org.eclipse.swt.graphics.ImageData;
import org.eclipse.swt.graphics.ImageLoader;
import org.eclipse.swt.graphics.Point;
import org.eclipse.swt.graphics.Rectangle;
import org.eclipse.swt.widgets.*;

import java.io.ByteArrayOutputStream;
import java.util.Base64;

/**
 * Executor for UI actions on SWT widgets.
 * All actions are executed on the Display thread using DisplayHelper.syncExec().
 */
public class SwtActionExecutor {

    static {
        System.err.println("[SwtAgent] SwtActionExecutor class loading...");
        System.err.println("[SwtAgent] SwtActionExecutor classloader: " + SwtActionExecutor.class.getClassLoader());
        System.err.println("[SwtAgent] Thread: " + Thread.currentThread().getName());
        System.err.println("[SwtAgent] Thread classloader: " + Thread.currentThread().getContextClassLoader());
        System.err.flush();
    }

    /**
     * Initialize this class early from the SWT context.
     * Should be called from DisplayHelper or SwtReflectionBridge initialization.
     */
    public static void initialize() {
        System.err.println("[SwtAgent] SwtActionExecutor.initialize() called");
        System.err.flush();
    }

    /**
     * Click on a widget.
     * Uses asyncExec to avoid blocking on modal dialogs.
     *
     * @param widgetId Widget ID to click
     */
    public static void click(int widgetId) {
        // Get and validate widget synchronously
        Widget widget = DisplayHelper.syncExecAndReturn(() -> {
            Widget w = getWidget(widgetId);
            ensureVisible(w);
            return w;
        });

        // Perform click asynchronously to avoid blocking on modal dialogs
        DisplayHelper.asyncExec(() -> {
            if (widget.isDisposed()) {
                return;
            }

            if (widget instanceof Button) {
                Button button = (Button) widget;
                // For check/radio buttons, toggle selection
                int style = button.getStyle();
                if ((style & SWT.CHECK) != 0 || (style & SWT.RADIO) != 0) {
                    button.setSelection(!button.getSelection());
                }
                // Notify selection listeners
                Event event = new Event();
                event.widget = button;
                button.notifyListeners(SWT.Selection, event);
            } else if (widget instanceof Control) {
                performMouseClick((Control) widget, 1);
            }
        });

        // Give the click a moment to process
        DisplayHelper.sleep(100);
    }

    /**
     * Double-click on a widget.
     * Uses asyncExec to avoid blocking on modal dialogs.
     *
     * @param widgetId Widget ID to double-click
     */
    public static void doubleClick(int widgetId) {
        // Get and validate widget synchronously
        Widget widget = DisplayHelper.syncExecAndReturn(() -> {
            Widget w = getWidget(widgetId);
            ensureVisible(w);
            return w;
        });

        // Perform double-click asynchronously
        DisplayHelper.asyncExec(() -> {
            if (widget.isDisposed()) {
                return;
            }

            if (widget instanceof Control) {
                performMouseClick((Control) widget, 2);
            }
        });

        // Give the click a moment to process
        DisplayHelper.sleep(100);
    }

    /**
     * Right-click on a widget (context menu).
     *
     * @param widgetId Widget ID to right-click
     */
    public static void rightClick(int widgetId) {
        // Get and validate widget synchronously
        Widget widget = DisplayHelper.syncExecAndReturn(() -> {
            Widget w = getWidget(widgetId);
            ensureVisible(w);
            return w;
        });

        // Perform right-click asynchronously
        DisplayHelper.asyncExec(() -> {
            if (widget.isDisposed() || !(widget instanceof Control)) {
                return;
            }

            Control control = (Control) widget;
            Point center = getControlCenter(control);

            Event event = new Event();
            event.widget = control;
            event.x = center.x;
            event.y = center.y;
            event.button = 3;
            event.count = 1;
            event.stateMask = 0;

            control.notifyListeners(SWT.MouseDown, event);
            control.notifyListeners(SWT.MouseUp, event);
            control.notifyListeners(SWT.MenuDetect, event);
        });

        // Give the click a moment to process
        DisplayHelper.sleep(100);
    }

    /**
     * Set text in a text widget (clears existing text first).
     *
     * @param widgetId Widget ID
     * @param text Text to set
     */
    public static void setText(int widgetId, String text) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            ensureVisible(widget);

            if (widget instanceof Text) {
                Text textWidget = (Text) widget;
                if (!textWidget.getEditable()) {
                    throw new IllegalStateException("Text widget is not editable");
                }
                textWidget.setText(text);
            } else if (widget instanceof Combo) {
                Combo combo = (Combo) widget;
                if ((combo.getStyle() & SWT.READ_ONLY) != 0) {
                    throw new IllegalStateException("Combo is read-only");
                }
                combo.setText(text);
            } else if (widget instanceof org.eclipse.swt.custom.StyledText) {
                org.eclipse.swt.custom.StyledText styledText = (org.eclipse.swt.custom.StyledText) widget;
                if (!styledText.getEditable()) {
                    throw new IllegalStateException("StyledText widget is not editable");
                }
                styledText.setText(text);
            } else {
                throw new IllegalArgumentException("Widget does not support text input");
            }
        });
    }

    /**
     * Type text into a widget (appends at cursor position).
     *
     * @param widgetId Widget ID
     * @param text Text to type
     */
    public static void typeText(int widgetId, String text) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            ensureVisible(widget);

            if (widget instanceof Text) {
                Text textWidget = (Text) widget;
                if (!textWidget.getEditable()) {
                    throw new IllegalStateException("Text widget is not editable");
                }
                textWidget.setFocus();
                int caretPos = textWidget.getCaretPosition();
                String currentText = textWidget.getText();
                String newText = currentText.substring(0, caretPos) + text + currentText.substring(caretPos);
                textWidget.setText(newText);
                textWidget.setSelection(caretPos + text.length());
            } else if (widget instanceof Combo) {
                Combo combo = (Combo) widget;
                if ((combo.getStyle() & SWT.READ_ONLY) != 0) {
                    throw new IllegalStateException("Combo is read-only");
                }
                combo.setFocus();
                String current = combo.getText();
                combo.setText(current + text);
            } else if (widget instanceof org.eclipse.swt.custom.StyledText) {
                org.eclipse.swt.custom.StyledText styledText = (org.eclipse.swt.custom.StyledText) widget;
                if (!styledText.getEditable()) {
                    throw new IllegalStateException("StyledText widget is not editable");
                }
                styledText.setFocus();
                int caretPos = styledText.getCaretOffset();
                styledText.insert(text);
                styledText.setCaretOffset(caretPos + text.length());
            } else {
                throw new IllegalArgumentException("Widget does not support text input");
            }
        });
    }

    /**
     * Clear text from a widget.
     *
     * @param widgetId Widget ID
     */
    public static void clearText(int widgetId) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);

            if (widget instanceof Text) {
                Text textWidget = (Text) widget;
                if (!textWidget.getEditable()) {
                    throw new IllegalStateException("Text widget is not editable");
                }
                textWidget.setText("");
            } else if (widget instanceof Combo) {
                Combo combo = (Combo) widget;
                if ((combo.getStyle() & SWT.READ_ONLY) != 0) {
                    throw new IllegalStateException("Combo is read-only");
                }
                combo.setText("");
            } else if (widget instanceof org.eclipse.swt.custom.StyledText) {
                org.eclipse.swt.custom.StyledText styledText = (org.eclipse.swt.custom.StyledText) widget;
                if (!styledText.getEditable()) {
                    throw new IllegalStateException("StyledText widget is not editable");
                }
                styledText.setText("");
            } else {
                throw new IllegalArgumentException("Widget does not support text clearing");
            }
        });
    }

    /**
     * Select an item from a list, combo, or similar widget.
     *
     * @param widgetId Widget ID
     * @param item Item to select (by text or index as string)
     */
    public static void selectItem(int widgetId, String item) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);

            if (widget instanceof Combo) {
                Combo combo = (Combo) widget;
                // Try as index first
                try {
                    int index = Integer.parseInt(item);
                    if (index >= 0 && index < combo.getItemCount()) {
                        combo.select(index);
                        notifySelectionChanged(combo);
                        return;
                    }
                } catch (NumberFormatException e) {
                    // Not an index, try as text
                }
                // Try as text
                int index = combo.indexOf(item);
                if (index >= 0) {
                    combo.select(index);
                    notifySelectionChanged(combo);
                } else {
                    throw new IllegalArgumentException("Item not found: " + item);
                }
            } else if (widget instanceof CCombo) {
                CCombo ccombo = (CCombo) widget;
                // Try as index first
                try {
                    int index = Integer.parseInt(item);
                    if (index >= 0 && index < ccombo.getItemCount()) {
                        ccombo.select(index);
                        notifySelectionChanged(ccombo);
                        return;
                    }
                } catch (NumberFormatException e) {
                    // Not an index, try as text
                }
                // Try as text
                int index = ccombo.indexOf(item);
                if (index >= 0) {
                    ccombo.select(index);
                    notifySelectionChanged(ccombo);
                } else {
                    throw new IllegalArgumentException("Item not found in CCombo: " + item);
                }
            } else if (widget instanceof org.eclipse.swt.widgets.List) {
                org.eclipse.swt.widgets.List list = (org.eclipse.swt.widgets.List) widget;
                // Try as index first
                try {
                    int index = Integer.parseInt(item);
                    if (index >= 0 && index < list.getItemCount()) {
                        list.select(index);
                        notifySelectionChanged(list);
                        return;
                    }
                } catch (NumberFormatException e) {
                    // Not an index, try as text
                }
                // Try as text
                int index = list.indexOf(item);
                if (index >= 0) {
                    list.select(index);
                    notifySelectionChanged(list);
                } else {
                    throw new IllegalArgumentException("Item not found: " + item);
                }
            } else if (widget instanceof TabFolder) {
                TabFolder tabFolder = (TabFolder) widget;
                // Try as index first
                try {
                    int index = Integer.parseInt(item);
                    if (index >= 0 && index < tabFolder.getItemCount()) {
                        tabFolder.setSelection(index);
                        notifySelectionChanged(tabFolder);
                        return;
                    }
                } catch (NumberFormatException e) {
                    // Not an index, try as text
                }
                // Try as text
                for (int i = 0; i < tabFolder.getItemCount(); i++) {
                    if (item.equals(tabFolder.getItem(i).getText())) {
                        tabFolder.setSelection(i);
                        notifySelectionChanged(tabFolder);
                        return;
                    }
                }
                throw new IllegalArgumentException("Tab not found: " + item);
            } else if (widget instanceof CTabFolder) {
                CTabFolder tabFolder = (CTabFolder) widget;
                // Try as index first
                try {
                    int index = Integer.parseInt(item);
                    if (index >= 0 && index < tabFolder.getItemCount()) {
                        tabFolder.setSelection(index);
                        notifySelectionChanged(tabFolder);
                        return;
                    }
                } catch (NumberFormatException e) {
                    // Not an index, try as text
                }
                // Try as text
                for (int i = 0; i < tabFolder.getItemCount(); i++) {
                    if (item.equals(tabFolder.getItem(i).getText())) {
                        tabFolder.setSelection(i);
                        notifySelectionChanged(tabFolder);
                        return;
                    }
                }
                throw new IllegalArgumentException("Tab not found: " + item);
            } else {
                throw new IllegalArgumentException("Widget does not support item selection");
            }
        });
    }

    /**
     * Select a table row by index.
     *
     * @param widgetId Widget ID (must be a Table)
     * @param row Row index to select
     */
    public static void selectTableRow(int widgetId, int row) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            if (row < 0 || row >= table.getItemCount()) {
                throw new IndexOutOfBoundsException("Row index out of bounds: " + row);
            }

            table.select(row);
            notifySelectionChanged(table);
        });
    }

    /**
     * Select a table cell.
     *
     * @param widgetId Widget ID (must be a Table)
     * @param row Row index
     * @param column Column index
     */
    public static void selectTableCell(int widgetId, int row, int column) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            if (row < 0 || row >= table.getItemCount()) {
                throw new IndexOutOfBoundsException("Row index out of bounds: " + row);
            }
            if (column < 0 || column >= table.getColumnCount()) {
                throw new IndexOutOfBoundsException("Column index out of bounds: " + column);
            }

            table.select(row);
            notifySelectionChanged(table);
        });
    }

    /**
     * Get table cell value.
     *
     * @param widgetId Widget ID (must be a Table)
     * @param row Row index
     * @param column Column index
     * @return Cell value as string
     */
    public static JsonPrimitive getTableCellValue(int widgetId, int row, int column) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            if (row < 0 || row >= table.getItemCount()) {
                throw new IndexOutOfBoundsException("Row index out of bounds: " + row);
            }

            TableItem item = table.getItem(row);
            String value = item.getText(column);
            return new JsonPrimitive(value != null ? value : "");
        });
    }

    /**
     * Get all table data.
     *
     * @param widgetId Widget ID (must be a Table)
     * @return JsonObject with table data
     */
    public static JsonObject getTableData(int widgetId) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            JsonObject result = new JsonObject();
            result.addProperty("rowCount", table.getItemCount());
            result.addProperty("columnCount", table.getColumnCount());

            // Column names
            JsonArray columns = new JsonArray();
            for (TableColumn col : table.getColumns()) {
                columns.add(col.getText());
            }
            result.add("columns", columns);

            // Row data
            JsonArray rows = new JsonArray();
            int maxRows = Math.min(table.getItemCount(), 1000);
            for (int i = 0; i < maxRows; i++) {
                TableItem item = table.getItem(i);
                JsonArray rowData = new JsonArray();
                for (int j = 0; j < table.getColumnCount(); j++) {
                    String value = item.getText(j);
                    rowData.add(value != null ? value : "");
                }
                rows.add(rowData);
            }
            result.add("rows", rows);

            return result;
        });
    }

    /**
     * Get all cell values for a specific table row.
     *
     * @param widgetId Widget ID (must be a Table)
     * @param row Row index
     * @return JsonArray with all cell values for the row
     */
    public static JsonArray getTableRowValues(int widgetId, int row) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            if (row < 0 || row >= table.getItemCount()) {
                throw new IndexOutOfBoundsException("Row index out of bounds: " + row +
                    " (table has " + table.getItemCount() + " rows)");
            }

            TableItem item = table.getItem(row);
            JsonArray rowValues = new JsonArray();

            int columnCount = table.getColumnCount();
            // If table has no explicit columns, still get the first column (index 0)
            if (columnCount == 0) {
                columnCount = 1;
            }

            for (int j = 0; j < columnCount; j++) {
                String value = item.getText(j);
                rowValues.add(value != null ? value : "");
            }

            return rowValues;
        });
    }

    /**
     * Select multiple table rows by their indices.
     *
     * @param widgetId Widget ID (must be a Table)
     * @param rows Array of row indices to select
     */
    public static void selectTableRows(int widgetId, int[] rows) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            int itemCount = table.getItemCount();

            // Validate all row indices first
            for (int row : rows) {
                if (row < 0 || row >= itemCount) {
                    throw new IndexOutOfBoundsException("Row index out of bounds: " + row +
                        " (table has " + itemCount + " rows)");
                }
            }

            // Check if multi-select is supported
            int style = table.getStyle();
            if ((style & SWT.MULTI) == 0 && rows.length > 1) {
                throw new IllegalStateException("Table does not support multi-selection (SWT.MULTI style not set)");
            }

            // Select all specified rows
            table.select(rows);
            notifySelectionChanged(table);
        });
    }

    /**
     * Deselect all rows in a table.
     *
     * @param widgetId Widget ID (must be a Table)
     */
    public static void deselectAllTableRows(int widgetId) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            table.deselectAll();
            notifySelectionChanged(table);
        });
    }

    /**
     * Select a table row by matching a cell value in a specific column.
     *
     * @param widgetId Widget ID (must be a Table)
     * @param column Column index to search in
     * @param value Value to match
     * @return The index of the selected row, or -1 if not found
     */
    public static int selectTableRowByValue(int widgetId, int column, String value) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            int columnCount = table.getColumnCount();

            // Handle tables with no explicit columns
            if (columnCount == 0) {
                columnCount = 1;
            }

            if (column < 0 || column >= columnCount) {
                throw new IndexOutOfBoundsException("Column index out of bounds: " + column +
                    " (table has " + columnCount + " columns)");
            }

            // Search for the row with matching value
            int itemCount = table.getItemCount();
            for (int i = 0; i < itemCount; i++) {
                TableItem item = table.getItem(i);
                String cellValue = item.getText(column);
                if (value.equals(cellValue)) {
                    table.select(i);
                    // Scroll to make the selected row visible
                    table.showSelection();
                    notifySelectionChanged(table);
                    return i;
                }
            }

            // Not found
            return -1;
        });
    }

    /**
     * Select a range of consecutive table rows.
     *
     * @param widgetId Widget ID (must be a Table)
     * @param startRow Starting row index (inclusive)
     * @param endRow Ending row index (inclusive)
     */
    public static void selectTableRowRange(int widgetId, int startRow, int endRow) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            int itemCount = table.getItemCount();

            // Validate bounds
            if (startRow < 0 || startRow >= itemCount) {
                throw new IndexOutOfBoundsException("Start row index out of bounds: " + startRow +
                    " (table has " + itemCount + " rows)");
            }
            if (endRow < 0 || endRow >= itemCount) {
                throw new IndexOutOfBoundsException("End row index out of bounds: " + endRow +
                    " (table has " + itemCount + " rows)");
            }
            if (startRow > endRow) {
                throw new IllegalArgumentException("Start row (" + startRow +
                    ") must be less than or equal to end row (" + endRow + ")");
            }

            // Check if multi-select is supported for ranges > 1 row
            int style = table.getStyle();
            if ((style & SWT.MULTI) == 0 && startRow != endRow) {
                throw new IllegalStateException("Table does not support multi-selection (SWT.MULTI style not set)");
            }

            // Select the range
            table.setSelection(startRow, endRow);
            notifySelectionChanged(table);
        });
    }

    /**
     * Set the value of a table cell.
     * Note: This only works for tables that support direct text editing.
     * For tables using CellEditors, more complex interaction may be needed.
     *
     * @param widgetId Widget ID (must be a Table)
     * @param row Row index
     * @param column Column index
     * @param value New value for the cell
     */
    public static void setTableCellValue(int widgetId, int row, int column, String value) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            int itemCount = table.getItemCount();
            int columnCount = table.getColumnCount();

            // Handle tables with no explicit columns
            if (columnCount == 0) {
                columnCount = 1;
            }

            // Validate bounds
            if (row < 0 || row >= itemCount) {
                throw new IndexOutOfBoundsException("Row index out of bounds: " + row +
                    " (table has " + itemCount + " rows)");
            }
            if (column < 0 || column >= columnCount) {
                throw new IndexOutOfBoundsException("Column index out of bounds: " + column +
                    " (table has " + columnCount + " columns)");
            }

            // Get the table item and set the text
            TableItem item = table.getItem(row);
            item.setText(column, value != null ? value : "");

            // Fire a modify event to notify any listeners
            Event event = new Event();
            event.widget = table;
            event.item = item;
            event.index = column;
            table.notifyListeners(SWT.SetData, event);
        });
    }

    /**
     * Click on a table column header (typically for sorting).
     *
     * @param widgetId Widget ID (must be a Table)
     * @param column Column index to click
     */
    public static void clickTableColumnHeader(int widgetId, int column) {
        // Get and validate widget synchronously
        Widget widget = DisplayHelper.syncExecAndReturn(() -> {
            Widget w = getWidget(widgetId);
            if (!(w instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) w;
            int columnCount = table.getColumnCount();

            if (columnCount == 0) {
                throw new IllegalStateException("Table has no columns");
            }
            if (column < 0 || column >= columnCount) {
                throw new IndexOutOfBoundsException("Column index out of bounds: " + column +
                    " (table has " + columnCount + " columns)");
            }

            return w;
        });

        // Perform click asynchronously to handle any dialogs that might appear
        DisplayHelper.asyncExec(() -> {
            if (widget.isDisposed()) {
                return;
            }

            Table table = (Table) widget;
            TableColumn tableColumn = table.getColumn(column);

            // Simulate a click on the column header by sending Selection event
            Event event = new Event();
            event.widget = tableColumn;
            event.type = SWT.Selection;
            tableColumn.notifyListeners(SWT.Selection, event);
        });

        // Give the click a moment to process
        DisplayHelper.sleep(100);
    }

    /**
     * Get the currently selected row indices in a table.
     *
     * @param widgetId Widget ID (must be a Table)
     * @return JsonArray of selected row indices
     */
    public static JsonArray getTableSelectedRows(int widgetId) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            int[] selectedIndices = table.getSelectionIndices();

            JsonArray result = new JsonArray();
            for (int index : selectedIndices) {
                result.add(index);
            }

            return result;
        });
    }

    /**
     * Check if a specific table row is selected.
     *
     * @param widgetId Widget ID (must be a Table)
     * @param row Row index to check
     * @return true if the row is selected, false otherwise
     */
    public static boolean isTableRowSelected(int widgetId, int row) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            if (row < 0 || row >= table.getItemCount()) {
                throw new IndexOutOfBoundsException("Row index out of bounds: " + row);
            }

            return table.isSelected(row);
        });
    }

    /**
     * Scroll table to make a specific row visible.
     *
     * @param widgetId Widget ID (must be a Table)
     * @param row Row index to scroll to
     */
    public static void scrollToTableRow(int widgetId, int row) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            if (row < 0 || row >= table.getItemCount()) {
                throw new IndexOutOfBoundsException("Row index out of bounds: " + row);
            }

            // Show the item at the specified row
            TableItem item = table.getItem(row);
            table.showItem(item);
        });
    }

    /**
     * Get information about table columns.
     *
     * @param widgetId Widget ID (must be a Table)
     * @return JsonArray with column information (text, width, resizable, moveable)
     */
    public static JsonArray getTableColumns(int widgetId) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Table)) {
                throw new IllegalArgumentException("Widget is not a Table");
            }

            Table table = (Table) widget;
            JsonArray columns = new JsonArray();

            for (int i = 0; i < table.getColumnCount(); i++) {
                TableColumn col = table.getColumn(i);
                JsonObject colInfo = new JsonObject();
                colInfo.addProperty("index", i);
                colInfo.addProperty("text", col.getText());
                colInfo.addProperty("width", col.getWidth());
                colInfo.addProperty("resizable", col.getResizable());
                colInfo.addProperty("moveable", col.getMoveable());
                colInfo.addProperty("alignment", getAlignmentString(col.getAlignment()));
                colInfo.addProperty("toolTipText", col.getToolTipText());
                columns.add(colInfo);
            }

            return columns;
        });
    }

    /**
     * Convert SWT alignment constant to string.
     */
    private static String getAlignmentString(int alignment) {
        if ((alignment & SWT.LEFT) != 0) return "LEFT";
        if ((alignment & SWT.CENTER) != 0) return "CENTER";
        if ((alignment & SWT.RIGHT) != 0) return "RIGHT";
        return "LEFT"; // default
    }

    /**
     * Select a tree item by path.
     *
     * @param widgetId Widget ID (must be a Tree)
     * @param path Path to the tree item (e.g., "Parent/Child/Leaf")
     */
    public static void selectTreeItem(int widgetId, String path) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Tree)) {
                throw new IllegalArgumentException("Widget is not a Tree");
            }

            Tree tree = (Tree) widget;
            TreeItem item = findTreeItem(tree, path);
            if (item == null) {
                throw new IllegalArgumentException("Tree item not found: " + path);
            }

            tree.setSelection(item);
            notifySelectionChanged(tree);
        });
    }

    /**
     * Expand a tree item by path.
     *
     * @param widgetId Widget ID (must be a Tree)
     * @param path Path to the tree item
     */
    public static void expandTreeItem(int widgetId, String path) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Tree)) {
                throw new IllegalArgumentException("Widget is not a Tree");
            }

            Tree tree = (Tree) widget;
            TreeItem item = findTreeItem(tree, path);
            if (item == null) {
                throw new IllegalArgumentException("Tree item not found: " + path);
            }

            item.setExpanded(true);
            Event event = new Event();
            event.widget = tree;
            event.item = item;
            tree.notifyListeners(SWT.Expand, event);
        });
    }

    /**
     * Collapse a tree item by path.
     *
     * @param widgetId Widget ID (must be a Tree)
     * @param path Path to the tree item
     */
    public static void collapseTreeItem(int widgetId, String path) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Tree)) {
                throw new IllegalArgumentException("Widget is not a Tree");
            }

            Tree tree = (Tree) widget;
            TreeItem item = findTreeItem(tree, path);
            if (item == null) {
                throw new IllegalArgumentException("Tree item not found: " + path);
            }

            item.setExpanded(false);
            Event event = new Event();
            event.widget = tree;
            event.item = item;
            tree.notifyListeners(SWT.Collapse, event);
        });
    }

    /**
     * Get tree structure.
     *
     * @param widgetId Widget ID (must be a Tree)
     * @return JsonObject with tree structure
     */
    public static JsonObject getTreeData(int widgetId) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Tree)) {
                throw new IllegalArgumentException("Widget is not a Tree");
            }

            Tree tree = (Tree) widget;
            JsonObject result = new JsonObject();
            result.addProperty("itemCount", tree.getItemCount());

            JsonArray items = new JsonArray();
            for (TreeItem item : tree.getItems()) {
                items.add(buildTreeItemJson(item));
            }
            result.add("items", items);

            return result;
        });
    }

    /**
     * Activate (bring to front) a shell.
     *
     * @param widgetId Widget ID (must be a Shell)
     */
    public static void activateShell(int widgetId) {
        System.err.println("[SwtAgent] activateShell executor: widgetId=" + widgetId);
        System.err.flush();
        DisplayHelper.syncExec(() -> {
            System.err.println("[SwtAgent] activateShell: inside syncExec lambda");
            System.err.flush();
            Widget widget = getWidget(widgetId);
            System.err.println("[SwtAgent] activateShell: widget=" + widget);
            System.err.flush();
            if (!(widget instanceof Shell)) {
                throw new IllegalArgumentException("Widget is not a Shell: " + widget);
            }
            Shell shell = (Shell) widget;
            System.err.println("[SwtAgent] activateShell: calling forceActive on " + shell.getText());
            System.err.flush();
            shell.forceActive();
            shell.setFocus();
            System.err.println("[SwtAgent] activateShell: forceActive/setFocus completed");
            System.err.flush();
        });
        System.err.println("[SwtAgent] activateShell executor: syncExec completed");
        System.err.flush();
    }

    /**
     * Close a shell.
     *
     * @param widgetId Widget ID (must be a Shell)
     */
    public static void closeShell(int widgetId) {
        DisplayHelper.asyncExec(() -> {
            Widget widget = WidgetInspector.getWidgetById(widgetId);
            if (widget == null || widget.isDisposed()) {
                return;
            }
            if (!(widget instanceof Shell)) {
                throw new IllegalArgumentException("Widget is not a Shell");
            }
            Shell shell = (Shell) widget;
            shell.close();
        });
        DisplayHelper.sleep(100);
    }

    /**
     * Focus a widget.
     *
     * @param widgetId Widget ID
     */
    public static void focus(int widgetId) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            if (widget instanceof Control) {
                ((Control) widget).setFocus();
            }
        });
    }

    /**
     * Get element bounds.
     *
     * @param widgetId Widget ID
     * @return JsonObject with bounds
     */
    public static JsonObject getElementBounds(int widgetId) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Control)) {
                throw new IllegalArgumentException("Widget is not a Control");
            }

            Control control = (Control) widget;
            Rectangle bounds = control.getBounds();

            JsonObject result = new JsonObject();
            result.addProperty("x", bounds.x);
            result.addProperty("y", bounds.y);
            result.addProperty("width", bounds.width);
            result.addProperty("height", bounds.height);

            if (control.isVisible()) {
                try {
                    Point screenLoc = control.toDisplay(0, 0);
                    result.addProperty("screenX", screenLoc.x);
                    result.addProperty("screenY", screenLoc.y);
                } catch (Exception e) {
                    // Ignore
                }
            }

            return result;
        });
    }

    /**
     * Get element text.
     *
     * @param widgetId Widget ID
     * @return Text content
     */
    public static JsonPrimitive getElementText(int widgetId) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            String text = getWidgetText(widget);
            return new JsonPrimitive(text != null ? text : "");
        });
    }

    /**
     * Capture screenshot of a widget or the entire display.
     *
     * @param widgetId Widget ID (-1 for full screen)
     * @return Base64-encoded PNG image
     */
    public static JsonPrimitive captureScreenshot(int widgetId) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Display display = DisplayHelper.getDisplay();
            if (display == null || display.isDisposed()) {
                throw new IllegalStateException("Display is not available");
            }

            Image image;
            GC gc;

            if (widgetId >= 0) {
                Widget widget = getWidget(widgetId);
                if (!(widget instanceof Control)) {
                    throw new IllegalArgumentException("Widget is not a Control");
                }

                Control control = (Control) widget;
                if (!control.isVisible()) {
                    throw new IllegalStateException("Widget is not visible");
                }

                Rectangle bounds = control.getBounds();
                image = new Image(display, bounds.width, bounds.height);
                gc = new GC(image);
                control.print(gc);
            } else {
                // Full display screenshot
                Rectangle bounds = display.getBounds();
                image = new Image(display, bounds.width, bounds.height);
                gc = new GC(display);
                gc.copyArea(image, 0, 0);
            }

            gc.dispose();

            // Convert to base64
            ImageLoader loader = new ImageLoader();
            loader.data = new ImageData[]{image.getImageData()};
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            loader.save(baos, SWT.IMAGE_PNG);
            image.dispose();

            byte[] bytes = baos.toByteArray();
            String base64 = Base64.getEncoder().encodeToString(bytes);

            return new JsonPrimitive("data:image/png;base64," + base64);
        });
    }

    // Helper methods

    private static Widget getWidget(int widgetId) {
        Widget widget = WidgetInspector.getWidgetById(widgetId);
        if (widget == null) {
            throw new IllegalArgumentException("Widget not found: " + widgetId);
        }
        if (widget.isDisposed()) {
            throw new IllegalArgumentException("Widget is disposed: " + widgetId);
        }
        return widget;
    }

    private static void ensureVisible(Widget widget) {
        if (widget instanceof Control) {
            Control control = (Control) widget;
            if (!control.isVisible()) {
                throw new IllegalStateException("Widget is not visible");
            }
        }
    }

    private static Point getControlCenter(Control control) {
        Rectangle bounds = control.getBounds();
        return new Point(bounds.width / 2, bounds.height / 2);
    }

    private static void performMouseClick(Control control, int clickCount) {
        Point center = getControlCenter(control);

        Event mouseDown = new Event();
        mouseDown.widget = control;
        mouseDown.x = center.x;
        mouseDown.y = center.y;
        mouseDown.button = 1;
        mouseDown.count = clickCount;

        Event mouseUp = new Event();
        mouseUp.widget = control;
        mouseUp.x = center.x;
        mouseUp.y = center.y;
        mouseUp.button = 1;
        mouseUp.count = clickCount;

        control.notifyListeners(SWT.MouseDown, mouseDown);
        control.notifyListeners(SWT.MouseUp, mouseUp);

        if (clickCount == 2) {
            control.notifyListeners(SWT.MouseDoubleClick, mouseUp);
        }
    }

    private static void notifySelectionChanged(Widget widget) {
        Event event = new Event();
        event.widget = widget;
        widget.notifyListeners(SWT.Selection, event);
    }

    private static String getWidgetText(Widget widget) {
        if (widget instanceof Shell) {
            return ((Shell) widget).getText();
        }
        if (widget instanceof Button) {
            return ((Button) widget).getText();
        }
        if (widget instanceof Label) {
            return ((Label) widget).getText();
        }
        if (widget instanceof Text) {
            return ((Text) widget).getText();
        }
        if (widget instanceof Combo) {
            return ((Combo) widget).getText();
        }
        if (widget instanceof Group) {
            return ((Group) widget).getText();
        }
        if (widget instanceof Link) {
            return ((Link) widget).getText();
        }
        if (widget instanceof org.eclipse.swt.custom.StyledText) {
            return ((org.eclipse.swt.custom.StyledText) widget).getText();
        }
        return null;
    }

    private static TreeItem findTreeItem(Tree tree, String path) {
        String[] parts = path.split("/");
        if (parts.length == 0) {
            return null;
        }

        TreeItem current = null;
        TreeItem[] items = tree.getItems();

        for (String part : parts) {
            TreeItem found = null;
            TreeItem[] searchItems = current == null ? items : current.getItems();

            for (TreeItem item : searchItems) {
                if (item.getText().equals(part)) {
                    found = item;
                    break;
                }
            }

            if (found == null) {
                return null;
            }
            current = found;
        }

        return current;
    }

    private static JsonObject buildTreeItemJson(TreeItem item) {
        JsonObject json = new JsonObject();
        json.addProperty("text", item.getText());
        json.addProperty("expanded", item.getExpanded());
        json.addProperty("checked", item.getChecked());
        json.addProperty("grayed", item.getGrayed());

        JsonArray children = new JsonArray();
        for (TreeItem child : item.getItems()) {
            children.add(buildTreeItemJson(child));
        }
        json.add("children", children);

        return json;
    }

    /**
     * Select multiple tree nodes by their names/paths.
     * Supports multi-selection if the tree has SWT.MULTI style.
     *
     * @param widgetId Widget ID (must be a Tree)
     * @param nodes Array of node paths to select (e.g., ["Parent/Child1", "Parent/Child2"])
     */
    public static void selectTreeNodes(int widgetId, String[] nodes) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Tree)) {
                throw new IllegalArgumentException("Widget is not a Tree");
            }

            Tree tree = (Tree) widget;
            java.util.List<TreeItem> itemsToSelect = new java.util.ArrayList<>();

            for (String nodePath : nodes) {
                // Use findTreeItemByName which handles both paths (with /) and simple names
                TreeItem item = findTreeItemByName(tree, nodePath);
                if (item != null) {
                    itemsToSelect.add(item);
                } else {
                    throw new IllegalArgumentException("Tree node not found: " + nodePath);
                }
            }

            if (!itemsToSelect.isEmpty()) {
                tree.setSelection(itemsToSelect.toArray(new TreeItem[0]));
                notifySelectionChanged(tree);
            }
        });
    }

    /**
     * Get the parent node name of a tree node.
     * Searches recursively through the tree to find the node.
     *
     * @param widgetId Widget ID (must be a Tree)
     * @param nodeName Name of the node to find the parent for
     * @return Parent node name, or empty string if node is at root level or not found
     */
    public static JsonPrimitive getTreeNodeParent(int widgetId, String nodeName) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Tree)) {
                throw new IllegalArgumentException("Widget is not a Tree");
            }

            Tree tree = (Tree) widget;
            TreeItem foundItem = findTreeItemByName(tree, nodeName);

            if (foundItem == null) {
                throw new IllegalArgumentException("Tree node not found: " + nodeName);
            }

            TreeItem parentItem = foundItem.getParentItem();
            if (parentItem == null) {
                // Node is at root level, no parent
                return new JsonPrimitive("");
            }

            return new JsonPrimitive(parentItem.getText());
        });
    }

    /**
     * Get the depth level of a tree node (0 for root items).
     * Searches recursively through the tree to find the node.
     *
     * @param widgetId Widget ID (must be a Tree)
     * @param nodeName Name of the node to find the level for
     * @return Depth level (0 for root items, 1 for first-level children, etc.)
     */
    public static JsonPrimitive getTreeNodeLevel(int widgetId, String nodeName) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Tree)) {
                throw new IllegalArgumentException("Widget is not a Tree");
            }

            Tree tree = (Tree) widget;
            int level = findTreeItemLevel(tree, nodeName);

            if (level < 0) {
                throw new IllegalArgumentException("Tree node not found: " + nodeName);
            }

            return new JsonPrimitive(level);
        });
    }

    /**
     * Check if a tree node exists in the tree.
     * Searches recursively through all tree items.
     *
     * @param widgetId Widget ID (must be a Tree)
     * @param nodeName Name of the node to search for
     * @return true if the node exists, false otherwise
     */
    public static JsonPrimitive treeNodeExists(int widgetId, String nodeName) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Tree)) {
                throw new IllegalArgumentException("Widget is not a Tree");
            }

            Tree tree = (Tree) widget;
            TreeItem foundItem = findTreeItemByName(tree, nodeName);

            return new JsonPrimitive(foundItem != null);
        });
    }

    /**
     * Find a tree item by its name, searching recursively through all items.
     * This differs from findTreeItem which uses path notation.
     *
     * @param tree The tree to search
     * @param nodeName The node name to find
     * @return The TreeItem if found, null otherwise
     */
    private static TreeItem findTreeItemByName(Tree tree, String nodeName) {
        // First, check if nodeName is a path (contains /)
        if (nodeName.contains("/")) {
            return findTreeItem(tree, nodeName);
        }

        // Search recursively through all items
        for (TreeItem item : tree.getItems()) {
            TreeItem found = findTreeItemByNameRecursive(item, nodeName);
            if (found != null) {
                return found;
            }
        }
        return null;
    }

    /**
     * Recursively search for a tree item by name.
     *
     * @param item Current tree item to check
     * @param nodeName Name to search for
     * @return The TreeItem if found, null otherwise
     */
    private static TreeItem findTreeItemByNameRecursive(TreeItem item, String nodeName) {
        if (item.getText().equals(nodeName)) {
            return item;
        }

        // Search children
        for (TreeItem child : item.getItems()) {
            TreeItem found = findTreeItemByNameRecursive(child, nodeName);
            if (found != null) {
                return found;
            }
        }
        return null;
    }

    /**
     * Find the depth level of a tree item by name.
     *
     * @param tree The tree to search
     * @param nodeName The node name to find
     * @return The depth level (0 for root), or -1 if not found
     */
    private static int findTreeItemLevel(Tree tree, String nodeName) {
        // First, check if nodeName is a path (contains /)
        if (nodeName.contains("/")) {
            TreeItem item = findTreeItem(tree, nodeName);
            if (item == null) {
                return -1;
            }
            return calculateTreeItemLevel(item);
        }

        // Search recursively through all items
        for (TreeItem item : tree.getItems()) {
            int level = findTreeItemLevelRecursive(item, nodeName, 0);
            if (level >= 0) {
                return level;
            }
        }
        return -1;
    }

    /**
     * Recursively search for a tree item's level by name.
     *
     * @param item Current tree item to check
     * @param nodeName Name to search for
     * @param currentLevel Current depth level
     * @return The depth level if found, -1 otherwise
     */
    private static int findTreeItemLevelRecursive(TreeItem item, String nodeName, int currentLevel) {
        if (item.getText().equals(nodeName)) {
            return currentLevel;
        }

        // Search children
        for (TreeItem child : item.getItems()) {
            int level = findTreeItemLevelRecursive(child, nodeName, currentLevel + 1);
            if (level >= 0) {
                return level;
            }
        }
        return -1;
    }

    /**
     * Calculate the depth level of a tree item by traversing up to the root.
     *
     * @param item The tree item
     * @return The depth level (0 for root items)
     */
    private static int calculateTreeItemLevel(TreeItem item) {
        int level = 0;
        TreeItem parent = item.getParentItem();
        while (parent != null) {
            level++;
            parent = parent.getParentItem();
        }
        return level;
    }

    /**
     * Get currently selected tree nodes.
     *
     * @param widgetId The tree widget ID
     * @return JSON array of selected node names
     */
    public static JsonArray getSelectedTreeNodes(int widgetId) {
        return DisplayHelper.syncExecAndReturn(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Tree)) {
                throw new IllegalArgumentException("Widget is not a Tree");
            }

            Tree tree = (Tree) widget;
            TreeItem[] selection = tree.getSelection();
            JsonArray result = new JsonArray();

            for (TreeItem item : selection) {
                result.add(item.getText());
            }

            return result;
        });
    }

    /**
     * Deselect all tree nodes.
     *
     * @param widgetId The tree widget ID
     */
    public static void deselectAllTreeNodes(int widgetId) {
        DisplayHelper.syncExec(() -> {
            Widget widget = getWidget(widgetId);
            if (!(widget instanceof Tree)) {
                throw new IllegalArgumentException("Widget is not a Tree");
            }

            Tree tree = (Tree) widget;
            tree.deselectAll();
        });
    }
}
