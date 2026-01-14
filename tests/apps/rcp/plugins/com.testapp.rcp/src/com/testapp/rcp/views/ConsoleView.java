package com.testapp.rcp.views;

import org.eclipse.swt.SWT;
import org.eclipse.swt.custom.StyledText;
import org.eclipse.swt.graphics.Color;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Button;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Display;
import org.eclipse.swt.widgets.Label;
import org.eclipse.ui.part.ViewPart;

import java.text.SimpleDateFormat;
import java.util.Date;

/**
 * Console View - contains a StyledText widget for log output.
 *
 * View ID: com.testapp.rcp.views.console
 *
 * Features:
 * - StyledText widget with scrolling
 * - Color-coded log messages
 * - Clear button
 * - Timestamp formatting
 */
public class ConsoleView extends ViewPart {

    /** View ID */
    public static final String ID = "com.testapp.rcp.views.console";

    /** Console output widget */
    private StyledText consoleText;

    /** Clear button */
    private Button clearButton;

    /** Date formatter */
    private SimpleDateFormat dateFormat = new SimpleDateFormat("HH:mm:ss.SSS");

    /** Colors */
    private Color infoColor;
    private Color warnColor;
    private Color errorColor;
    private Color debugColor;

    @Override
    public void createPartControl(Composite parent) {
        // Initialize colors
        Display display = parent.getDisplay();
        infoColor = display.getSystemColor(SWT.COLOR_DARK_GREEN);
        warnColor = display.getSystemColor(SWT.COLOR_DARK_YELLOW);
        errorColor = display.getSystemColor(SWT.COLOR_DARK_RED);
        debugColor = display.getSystemColor(SWT.COLOR_DARK_GRAY);

        // Create main composite
        Composite composite = new Composite(parent, SWT.NONE);
        composite.setLayout(new GridLayout(1, false));

        // Create toolbar area
        Composite toolbar = new Composite(composite, SWT.NONE);
        toolbar.setLayout(new GridLayout(3, false));
        toolbar.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));

        Label headerLabel = new Label(toolbar, SWT.NONE);
        headerLabel.setText("Console Output:");
        headerLabel.setData("name", "consoleHeader");

        Label spacer = new Label(toolbar, SWT.NONE);
        spacer.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));

        clearButton = new Button(toolbar, SWT.PUSH);
        clearButton.setText("Clear");
        clearButton.setData("name", "clearButton");
        clearButton.addListener(SWT.Selection, event -> clear());

        // Create styled text for console output
        consoleText = new StyledText(composite,
            SWT.MULTI | SWT.H_SCROLL | SWT.V_SCROLL | SWT.BORDER | SWT.READ_ONLY);
        consoleText.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
        consoleText.setData("name", "consoleOutput");

        // Set monospaced font
        consoleText.setFont(
            new org.eclipse.swt.graphics.Font(display, "Courier New", 10, SWT.NORMAL));

        // Add initial log entries
        addInitialLogEntries();
    }

    private void addInitialLogEntries() {
        logInfo("Console initialized");
        logInfo("RCP Test Application started");
        logDebug("Loading workspace configuration...");
        logInfo("Workspace loaded successfully");
        logWarn("Sample warning message for testing");
        logDebug("Ready for automation testing");
    }

    /**
     * Log an info message.
     */
    public void logInfo(String message) {
        appendLog("INFO", message, infoColor);
    }

    /**
     * Log a warning message.
     */
    public void logWarn(String message) {
        appendLog("WARN", message, warnColor);
    }

    /**
     * Log an error message.
     */
    public void logError(String message) {
        appendLog("ERROR", message, errorColor);
    }

    /**
     * Log a debug message.
     */
    public void logDebug(String message) {
        appendLog("DEBUG", message, debugColor);
    }

    /**
     * Append a log message with timestamp.
     */
    private void appendLog(String level, String message, Color color) {
        if (consoleText == null || consoleText.isDisposed()) {
            return;
        }

        String timestamp = dateFormat.format(new Date());
        String logEntry = String.format("[%s] [%s] %s%n", timestamp, level, message);

        Display.getDefault().asyncExec(() -> {
            if (!consoleText.isDisposed()) {
                int start = consoleText.getCharCount();
                consoleText.append(logEntry);
                int end = consoleText.getCharCount();

                // Apply color styling
                org.eclipse.swt.custom.StyleRange style = new org.eclipse.swt.custom.StyleRange();
                style.start = start;
                style.length = end - start;
                style.foreground = color;
                consoleText.setStyleRange(style);

                // Scroll to bottom
                consoleText.setTopIndex(consoleText.getLineCount() - 1);
            }
        });
    }

    /**
     * Clear the console.
     */
    public void clear() {
        if (consoleText != null && !consoleText.isDisposed()) {
            consoleText.setText("");
            logInfo("Console cleared");
        }
    }

    /**
     * Get the console text content.
     */
    public String getText() {
        if (consoleText != null && !consoleText.isDisposed()) {
            return consoleText.getText();
        }
        return "";
    }

    /**
     * Get line count.
     */
    public int getLineCount() {
        if (consoleText != null && !consoleText.isDisposed()) {
            return consoleText.getLineCount();
        }
        return 0;
    }

    @Override
    public void setFocus() {
        if (consoleText != null && !consoleText.isDisposed()) {
            consoleText.setFocus();
        }
    }

    /**
     * Get the StyledText widget for testing purposes.
     */
    public StyledText getStyledText() {
        return consoleText;
    }
}
