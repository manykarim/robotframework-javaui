package com.testapp.rcp.editors;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.jface.text.Document;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.source.SourceViewer;
import org.eclipse.jface.text.source.VerticalRuler;
import org.eclipse.swt.SWT;
import org.eclipse.swt.layout.FillLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IEditorSite;
import org.eclipse.ui.PartInitException;
import org.eclipse.ui.part.EditorPart;

/**
 * Simple Text Editor - a basic text editor implementation.
 *
 * Editor ID: com.testapp.rcp.editors.text
 * File extensions: txt, log, cfg
 *
 * Features:
 * - Source viewer with line numbers
 * - Dirty state tracking
 * - Save functionality
 */
public class SimpleTextEditor extends EditorPart {

    /** Editor ID */
    public static final String ID = "com.testapp.rcp.editors.text";

    /** Source viewer for text editing */
    private SourceViewer sourceViewer;

    /** Document */
    private IDocument document;

    /** Dirty state */
    private boolean dirty = false;

    @Override
    public void init(IEditorSite site, IEditorInput input) throws PartInitException {
        setSite(site);
        setInput(input);
        setPartName(input.getName());
        setTitleToolTip(input.getToolTipText());

        System.out.println("[SimpleTextEditor] Initializing editor for: " + input.getName());
    }

    @Override
    public void createPartControl(Composite parent) {
        parent.setLayout(new FillLayout());

        // Create source viewer with vertical ruler for line numbers
        VerticalRuler ruler = new VerticalRuler(12);
        sourceViewer = new SourceViewer(parent, ruler, SWT.H_SCROLL | SWT.V_SCROLL | SWT.BORDER);
        sourceViewer.getTextWidget().setData("name", "textEditorContent");

        // Create document with sample content
        document = new Document(getSampleContent());
        sourceViewer.setDocument(document);

        // Track modifications for dirty state
        document.addDocumentListener(new org.eclipse.jface.text.IDocumentListener() {
            @Override
            public void documentAboutToBeChanged(org.eclipse.jface.text.DocumentEvent event) {
            }

            @Override
            public void documentChanged(org.eclipse.jface.text.DocumentEvent event) {
                if (!dirty) {
                    dirty = true;
                    firePropertyChange(PROP_DIRTY);
                    System.out.println("[SimpleTextEditor] Document modified, dirty=true");
                }
            }
        });

        System.out.println("[SimpleTextEditor] Editor created for: " + getEditorInput().getName());
    }

    private String getSampleContent() {
        String name = getEditorInput().getName();
        if (name.endsWith(".txt")) {
            return "Sample text file content.\n\n" +
                   "This is a test file for Robot Framework automation.\n" +
                   "You can edit this content to test editor functionality.\n\n" +
                   "Features to test:\n" +
                   "- Text input\n" +
                   "- Save operations\n" +
                   "- Dirty state tracking\n" +
                   "- Multiple editors\n";
        } else if (name.endsWith(".log")) {
            return "[INFO] Application started\n" +
                   "[DEBUG] Loading configuration\n" +
                   "[INFO] Configuration loaded successfully\n" +
                   "[WARN] Using default settings for missing values\n" +
                   "[INFO] Ready for operations\n";
        } else if (name.endsWith(".cfg")) {
            return "# Configuration file\n\n" +
                   "app.name=RcpTestApp\n" +
                   "app.version=1.0.0\n" +
                   "debug.enabled=true\n" +
                   "log.level=INFO\n" +
                   "timeout.seconds=30\n";
        }
        return "Default content for: " + name;
    }

    @Override
    public void doSave(IProgressMonitor monitor) {
        System.out.println("[SimpleTextEditor] Saving: " + getEditorInput().getName());
        dirty = false;
        firePropertyChange(PROP_DIRTY);
        System.out.println("[SimpleTextEditor] Saved successfully");
    }

    @Override
    public void doSaveAs() {
        System.out.println("[SimpleTextEditor] Save As requested");
        // For simplicity, just do regular save
        doSave(null);
    }

    @Override
    public boolean isDirty() {
        return dirty;
    }

    @Override
    public boolean isSaveAsAllowed() {
        return true;
    }

    @Override
    public void setFocus() {
        if (sourceViewer != null && sourceViewer.getTextWidget() != null) {
            sourceViewer.getTextWidget().setFocus();
        }
    }

    /**
     * Get the source viewer for testing.
     */
    public SourceViewer getSourceViewer() {
        return sourceViewer;
    }

    /**
     * Get the document content.
     */
    public String getText() {
        return document != null ? document.get() : "";
    }

    /**
     * Set the document content.
     */
    public void setText(String text) {
        if (document != null) {
            document.set(text);
        }
    }

    /**
     * Insert text at current cursor position.
     */
    public void insertText(String text) {
        if (sourceViewer != null && sourceViewer.getTextWidget() != null) {
            sourceViewer.getTextWidget().insert(text);
        }
    }
}
