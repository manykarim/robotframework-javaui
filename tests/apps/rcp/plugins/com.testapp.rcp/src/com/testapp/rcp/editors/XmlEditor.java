package com.testapp.rcp.editors;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.jface.text.Document;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.TextAttribute;
import org.eclipse.jface.text.presentation.PresentationReconciler;
import org.eclipse.jface.text.rules.DefaultDamagerRepairer;
import org.eclipse.jface.text.rules.IRule;
import org.eclipse.jface.text.rules.IToken;
import org.eclipse.jface.text.rules.RuleBasedScanner;
import org.eclipse.jface.text.rules.SingleLineRule;
import org.eclipse.jface.text.rules.Token;
import org.eclipse.jface.text.source.ISourceViewer;
import org.eclipse.jface.text.source.SourceViewer;
import org.eclipse.jface.text.source.SourceViewerConfiguration;
import org.eclipse.jface.text.source.VerticalRuler;
import org.eclipse.swt.SWT;
import org.eclipse.swt.graphics.Color;
import org.eclipse.swt.graphics.RGB;
import org.eclipse.swt.layout.FillLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IEditorSite;
import org.eclipse.ui.PartInitException;
import org.eclipse.ui.part.EditorPart;

/**
 * XML Editor - a text editor with basic XML syntax highlighting.
 *
 * Editor ID: com.testapp.rcp.editors.xml
 * File extensions: xml
 *
 * Features:
 * - Source viewer with line numbers
 * - Basic XML syntax coloring (tags, attributes, strings)
 * - Dirty state tracking
 */
public class XmlEditor extends EditorPart {

    /** Editor ID */
    public static final String ID = "com.testapp.rcp.editors.xml";

    /** Source viewer */
    private SourceViewer sourceViewer;

    /** Document */
    private IDocument document;

    /** Dirty state */
    private boolean dirty = false;

    // Colors for syntax highlighting
    private Color tagColor;
    private Color stringColor;
    private Color commentColor;

    @Override
    public void init(IEditorSite site, IEditorInput input) throws PartInitException {
        setSite(site);
        setInput(input);
        setPartName(input.getName());
        setTitleToolTip(input.getToolTipText());

        System.out.println("[XmlEditor] Initializing editor for: " + input.getName());
    }

    @Override
    public void createPartControl(Composite parent) {
        parent.setLayout(new FillLayout());

        // Initialize colors
        Display display = parent.getDisplay();
        tagColor = new Color(display, new RGB(0, 0, 128)); // Blue
        stringColor = new Color(display, new RGB(0, 128, 0)); // Green
        commentColor = new Color(display, new RGB(128, 128, 128)); // Gray

        // Create source viewer
        VerticalRuler ruler = new VerticalRuler(12);
        sourceViewer = new SourceViewer(parent, ruler,
            SWT.H_SCROLL | SWT.V_SCROLL | SWT.BORDER);
        sourceViewer.getTextWidget().setData("name", "xmlEditorContent");

        // Configure source viewer
        sourceViewer.configure(new XmlSourceViewerConfiguration());

        // Create document
        document = new Document(getSampleXmlContent());
        sourceViewer.setDocument(document);

        // Track modifications
        document.addDocumentListener(new org.eclipse.jface.text.IDocumentListener() {
            @Override
            public void documentAboutToBeChanged(org.eclipse.jface.text.DocumentEvent event) {
            }

            @Override
            public void documentChanged(org.eclipse.jface.text.DocumentEvent event) {
                if (!dirty) {
                    dirty = true;
                    firePropertyChange(PROP_DIRTY);
                    System.out.println("[XmlEditor] Document modified, dirty=true");
                }
            }
        });

        System.out.println("[XmlEditor] XML editor created");
    }

    private String getSampleXmlContent() {
        return "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n" +
               "<!-- Sample XML file for testing -->\n" +
               "<project name=\"RcpTestApp\" version=\"1.0\">\n" +
               "    <description>Test application for Robot Framework</description>\n" +
               "    \n" +
               "    <configuration>\n" +
               "        <property name=\"debug\" value=\"true\"/>\n" +
               "        <property name=\"timeout\" value=\"30\"/>\n" +
               "        <property name=\"logLevel\" value=\"INFO\"/>\n" +
               "    </configuration>\n" +
               "    \n" +
               "    <modules>\n" +
               "        <module id=\"core\" enabled=\"true\">\n" +
               "            <dependency>utils</dependency>\n" +
               "        </module>\n" +
               "        <module id=\"utils\" enabled=\"true\"/>\n" +
               "        <module id=\"ui\" enabled=\"false\">\n" +
               "            <dependency>core</dependency>\n" +
               "            <dependency>utils</dependency>\n" +
               "        </module>\n" +
               "    </modules>\n" +
               "</project>\n";
    }

    @Override
    public void doSave(IProgressMonitor monitor) {
        System.out.println("[XmlEditor] Saving: " + getEditorInput().getName());
        dirty = false;
        firePropertyChange(PROP_DIRTY);
    }

    @Override
    public void doSaveAs() {
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
        if (sourceViewer != null) {
            sourceViewer.getTextWidget().setFocus();
        }
    }

    @Override
    public void dispose() {
        if (tagColor != null) tagColor.dispose();
        if (stringColor != null) stringColor.dispose();
        if (commentColor != null) commentColor.dispose();
        super.dispose();
    }

    public SourceViewer getSourceViewer() {
        return sourceViewer;
    }

    public String getText() {
        return document != null ? document.get() : "";
    }

    // ========== Inner Classes ==========

    /**
     * Source viewer configuration for XML.
     */
    private class XmlSourceViewerConfiguration extends SourceViewerConfiguration {
        @Override
        public PresentationReconciler getPresentationReconciler(ISourceViewer viewer) {
            PresentationReconciler reconciler = new PresentationReconciler();

            // Create scanner with XML rules
            RuleBasedScanner scanner = new RuleBasedScanner();

            IToken tagToken = new Token(new TextAttribute(tagColor, null, SWT.BOLD));
            IToken stringToken = new Token(new TextAttribute(stringColor));
            IToken commentToken = new Token(new TextAttribute(commentColor, null, SWT.ITALIC));

            IRule[] rules = new IRule[] {
                new SingleLineRule("\"", "\"", stringToken, '\\'),
                new SingleLineRule("'", "'", stringToken, '\\'),
                new SingleLineRule("<!--", "-->", commentToken),
                new SingleLineRule("<", ">", tagToken),
            };
            scanner.setRules(rules);

            DefaultDamagerRepairer dr = new DefaultDamagerRepairer(scanner);
            reconciler.setDamager(dr, IDocument.DEFAULT_CONTENT_TYPE);
            reconciler.setRepairer(dr, IDocument.DEFAULT_CONTENT_TYPE);

            return reconciler;
        }
    }
}
