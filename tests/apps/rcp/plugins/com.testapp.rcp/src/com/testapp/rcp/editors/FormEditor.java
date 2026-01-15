package com.testapp.rcp.editors;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.swt.SWT;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Button;
import org.eclipse.swt.widgets.Combo;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Label;
import org.eclipse.swt.widgets.Text;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IEditorSite;
import org.eclipse.ui.PartInitException;
import org.eclipse.ui.forms.widgets.FormToolkit;
import org.eclipse.ui.forms.widgets.ScrolledForm;
import org.eclipse.ui.forms.widgets.Section;
import org.eclipse.ui.part.EditorPart;

/**
 * Form Editor - a form-based editor with multiple sections.
 *
 * Editor ID: com.testapp.rcp.editors.form
 * File extensions: form
 *
 * Features:
 * - Eclipse Forms UI
 * - Multiple sections with various widgets
 * - Text fields, combos, checkboxes, buttons
 * - Dirty state tracking
 */
public class FormEditor extends EditorPart {

    /** Editor ID */
    public static final String ID = "com.testapp.rcp.editors.form";

    /** Form toolkit */
    private FormToolkit toolkit;

    /** Scrolled form */
    private ScrolledForm form;

    /** Dirty state */
    private boolean dirty = false;

    // Form fields
    private Text nameText;
    private Text descriptionText;
    private Combo typeCombo;
    private Combo priorityCombo;
    private Button activeCheckbox;
    private Button enabledCheckbox;
    private Text tagsText;

    @Override
    public void init(IEditorSite site, IEditorInput input) throws PartInitException {
        setSite(site);
        setInput(input);
        setPartName(input.getName());
        setTitleToolTip(input.getToolTipText());

        System.out.println("[FormEditor] Initializing editor for: " + input.getName());
    }

    @Override
    public void createPartControl(Composite parent) {
        toolkit = new FormToolkit(parent.getDisplay());
        form = toolkit.createScrolledForm(parent);
        form.setText("Form Editor - " + getEditorInput().getName());
        form.getBody().setLayout(new GridLayout(1, false));

        createGeneralSection(form.getBody());
        createDetailsSection(form.getBody());
        createActionsSection(form.getBody());

        // Set initial values
        initializeFormValues();

        System.out.println("[FormEditor] Form editor created");
    }

    private void createGeneralSection(Composite parent) {
        Section section = toolkit.createSection(parent,
            Section.TITLE_BAR | Section.DESCRIPTION | Section.EXPANDED);
        section.setText("General Information");
        section.setDescription("Enter the basic information for this item.");
        section.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, false));

        Composite client = toolkit.createComposite(section);
        client.setLayout(new GridLayout(2, false));
        section.setClient(client);

        // Name field
        toolkit.createLabel(client, "Name:");
        nameText = toolkit.createText(client, "", SWT.BORDER);
        nameText.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        nameText.setData("name", "formNameField");
        nameText.addModifyListener(e -> markDirty());

        // Type combo
        toolkit.createLabel(client, "Type:");
        typeCombo = new Combo(client, SWT.READ_ONLY);
        typeCombo.setItems("Feature", "Bug", "Enhancement", "Task", "Story");
        typeCombo.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        typeCombo.setData("name", "formTypeCombo");
        typeCombo.addSelectionListener(new org.eclipse.swt.events.SelectionAdapter() {
            @Override
            public void widgetSelected(org.eclipse.swt.events.SelectionEvent e) {
                markDirty();
            }
        });
        toolkit.adapt(typeCombo);

        // Priority combo
        toolkit.createLabel(client, "Priority:");
        priorityCombo = new Combo(client, SWT.READ_ONLY);
        priorityCombo.setItems("Critical", "High", "Medium", "Low");
        priorityCombo.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        priorityCombo.setData("name", "formPriorityCombo");
        priorityCombo.addSelectionListener(new org.eclipse.swt.events.SelectionAdapter() {
            @Override
            public void widgetSelected(org.eclipse.swt.events.SelectionEvent e) {
                markDirty();
            }
        });
        toolkit.adapt(priorityCombo);
    }

    private void createDetailsSection(Composite parent) {
        Section section = toolkit.createSection(parent,
            Section.TITLE_BAR | Section.TWISTIE | Section.EXPANDED);
        section.setText("Details");
        section.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));

        Composite client = toolkit.createComposite(section);
        client.setLayout(new GridLayout(2, false));
        section.setClient(client);

        // Description field (multi-line)
        toolkit.createLabel(client, "Description:");
        descriptionText = toolkit.createText(client, "",
            SWT.BORDER | SWT.MULTI | SWT.V_SCROLL | SWT.WRAP);
        GridData descGD = new GridData(SWT.FILL, SWT.FILL, true, true);
        descGD.heightHint = 100;
        descriptionText.setLayoutData(descGD);
        descriptionText.setData("name", "formDescriptionField");
        descriptionText.addModifyListener(e -> markDirty());

        // Tags field
        toolkit.createLabel(client, "Tags:");
        tagsText = toolkit.createText(client, "", SWT.BORDER);
        tagsText.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        tagsText.setData("name", "formTagsField");
        tagsText.addModifyListener(e -> markDirty());

        // Active checkbox
        Label spacer = toolkit.createLabel(client, "");
        activeCheckbox = toolkit.createButton(client, "Active", SWT.CHECK);
        activeCheckbox.setData("name", "formActiveCheckbox");
        activeCheckbox.addSelectionListener(new org.eclipse.swt.events.SelectionAdapter() {
            @Override
            public void widgetSelected(org.eclipse.swt.events.SelectionEvent e) {
                markDirty();
            }
        });

        // Enabled checkbox
        toolkit.createLabel(client, "");
        enabledCheckbox = toolkit.createButton(client, "Enabled", SWT.CHECK);
        enabledCheckbox.setData("name", "formEnabledCheckbox");
        enabledCheckbox.addSelectionListener(new org.eclipse.swt.events.SelectionAdapter() {
            @Override
            public void widgetSelected(org.eclipse.swt.events.SelectionEvent e) {
                markDirty();
            }
        });
    }

    private void createActionsSection(Composite parent) {
        Section section = toolkit.createSection(parent,
            Section.TITLE_BAR | Section.TWISTIE | Section.EXPANDED);
        section.setText("Actions");
        section.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));

        Composite client = toolkit.createComposite(section);
        client.setLayout(new GridLayout(4, true));
        section.setClient(client);

        Button saveButton = toolkit.createButton(client, "Save", SWT.PUSH);
        saveButton.setData("name", "formSaveButton");
        saveButton.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        saveButton.addListener(SWT.Selection, e -> doSave(null));

        Button resetButton = toolkit.createButton(client, "Reset", SWT.PUSH);
        resetButton.setData("name", "formResetButton");
        resetButton.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        resetButton.addListener(SWT.Selection, e -> initializeFormValues());

        Button validateButton = toolkit.createButton(client, "Validate", SWT.PUSH);
        validateButton.setData("name", "formValidateButton");
        validateButton.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        validateButton.addListener(SWT.Selection, e -> validateForm());

        Button cancelButton = toolkit.createButton(client, "Cancel", SWT.PUSH);
        cancelButton.setData("name", "formCancelButton");
        cancelButton.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
        cancelButton.addListener(SWT.Selection, e -> {
            dirty = false;
            firePropertyChange(PROP_DIRTY);
            getSite().getPage().closeEditor(this, false);
        });
    }

    private void initializeFormValues() {
        nameText.setText("Sample Item");
        typeCombo.select(0); // Feature
        priorityCombo.select(2); // Medium
        descriptionText.setText("This is a sample form item for testing.\n" +
            "You can modify the fields to test form functionality.");
        tagsText.setText("test, sample, demo");
        activeCheckbox.setSelection(true);
        enabledCheckbox.setSelection(true);

        dirty = false;
        firePropertyChange(PROP_DIRTY);
        System.out.println("[FormEditor] Form values initialized");
    }

    private void validateForm() {
        StringBuilder errors = new StringBuilder();

        if (nameText.getText().trim().isEmpty()) {
            errors.append("- Name is required\n");
        }
        if (typeCombo.getSelectionIndex() < 0) {
            errors.append("- Type must be selected\n");
        }
        if (priorityCombo.getSelectionIndex() < 0) {
            errors.append("- Priority must be selected\n");
        }

        if (errors.length() > 0) {
            System.out.println("[FormEditor] Validation failed:\n" + errors);
        } else {
            System.out.println("[FormEditor] Validation passed");
        }
    }

    private void markDirty() {
        if (!dirty) {
            dirty = true;
            firePropertyChange(PROP_DIRTY);
            System.out.println("[FormEditor] Form modified, dirty=true");
        }
    }

    @Override
    public void doSave(IProgressMonitor monitor) {
        System.out.println("[FormEditor] Saving form: " + getEditorInput().getName());
        System.out.println("  Name: " + nameText.getText());
        System.out.println("  Type: " + typeCombo.getText());
        System.out.println("  Priority: " + priorityCombo.getText());
        dirty = false;
        firePropertyChange(PROP_DIRTY);
        System.out.println("[FormEditor] Form saved successfully");
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
        if (nameText != null && !nameText.isDisposed()) {
            nameText.setFocus();
        }
    }

    @Override
    public void dispose() {
        if (toolkit != null) {
            toolkit.dispose();
        }
        super.dispose();
    }

    // Getter methods for testing
    public Text getNameText() { return nameText; }
    public Text getDescriptionText() { return descriptionText; }
    public Combo getTypeCombo() { return typeCombo; }
    public Combo getPriorityCombo() { return priorityCombo; }
    public Button getActiveCheckbox() { return activeCheckbox; }
    public Button getEnabledCheckbox() { return enabledCheckbox; }
}
