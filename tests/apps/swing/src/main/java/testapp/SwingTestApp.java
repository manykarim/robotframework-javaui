package testapp;

import javax.swing.*;
import javax.swing.event.*;
import javax.swing.table.DefaultTableModel;
import javax.swing.tree.DefaultMutableTreeNode;
import javax.swing.tree.DefaultTreeModel;
import java.awt.*;
import java.awt.event.*;

/**
 * Comprehensive Swing Test Application for testing SwingLibrary Robot Framework keywords.
 * This application includes all major Swing components with meaningful names for testing.
 */
public class SwingTestApp extends JFrame {

    // Component declarations
    private JMenuBar menuBar;
    private JToolBar toolBar;
    private JTabbedPane tabbedPane;
    private JSplitPane mainSplitPane;
    private JTable dataTable;
    private JTree fileTree;
    private JProgressBar progressBar;
    private JLabel statusLabel;

    // Text components
    private JTextField nameTextField;
    private JTextField emailTextField;
    private JPasswordField passwordField;
    private JTextArea descriptionTextArea;

    // Selection components
    private JComboBox<String> categoryComboBox;
    private JList<String> itemList;
    private JCheckBox enabledCheckBox;
    private JCheckBox notificationsCheckBox;
    private JCheckBox autoSaveCheckBox;
    private JRadioButton optionARadioButton;
    private JRadioButton optionBRadioButton;
    private JRadioButton optionCRadioButton;
    private ButtonGroup optionButtonGroup;

    // Numeric components
    private JSpinner quantitySpinner;
    private JSlider volumeSlider;

    // Buttons
    private JButton submitButton;
    private JButton clearButton;
    private JButton openDialogButton;
    private JButton openModalDialogButton;

    // Dialogs
    private JDialog settingsDialog;
    private JDialog aboutDialog;

    // Popup menus
    private JPopupMenu tablePopupMenu;
    private JPopupMenu treePopupMenu;

    public SwingTestApp() {
        initComponents();
        layoutComponents();
        setupEventHandlers();
        finalizeFrame();
    }

    private void initComponents() {
        // Initialize menu bar
        initMenuBar();

        // Initialize toolbar
        initToolBar();

        // Initialize tree (needed before tabbed pane)
        initTree();

        // Initialize table (needed before tabbed pane - used in Data View tab)
        initTable();

        // Initialize tabbed pane with tabs (uses tree and table)
        initTabbedPane();

        // Initialize popup menus
        initPopupMenus();

        // Initialize dialogs
        initDialogs();

        // Initialize progress bar and status
        progressBar = new JProgressBar(0, 100);
        progressBar.setName("progressBar");
        progressBar.setValue(0);
        progressBar.setStringPainted(true);

        statusLabel = new JLabel("Ready");
        statusLabel.setName("statusLabel");
    }

    private void initMenuBar() {
        menuBar = new JMenuBar();
        menuBar.setName("menuBar");

        // File menu
        JMenu fileMenu = new JMenu("File");
        fileMenu.setName("fileMenu");
        fileMenu.setMnemonic(KeyEvent.VK_F);

        JMenuItem newMenuItem = new JMenuItem("New", KeyEvent.VK_N);
        newMenuItem.setName("newMenuItem");
        newMenuItem.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_N, InputEvent.CTRL_DOWN_MASK));

        JMenuItem openMenuItem = new JMenuItem("Open...", KeyEvent.VK_O);
        openMenuItem.setName("openMenuItem");
        openMenuItem.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_O, InputEvent.CTRL_DOWN_MASK));

        JMenuItem saveMenuItem = new JMenuItem("Save", KeyEvent.VK_S);
        saveMenuItem.setName("saveMenuItem");
        saveMenuItem.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_S, InputEvent.CTRL_DOWN_MASK));

        JMenuItem saveAsMenuItem = new JMenuItem("Save As...");
        saveAsMenuItem.setName("saveAsMenuItem");

        JMenuItem exitMenuItem = new JMenuItem("Exit", KeyEvent.VK_X);
        exitMenuItem.setName("exitMenuItem");
        exitMenuItem.addActionListener(e -> System.exit(0));

        fileMenu.add(newMenuItem);
        fileMenu.add(openMenuItem);
        fileMenu.addSeparator();
        fileMenu.add(saveMenuItem);
        fileMenu.add(saveAsMenuItem);
        fileMenu.addSeparator();
        fileMenu.add(exitMenuItem);

        // Edit menu
        JMenu editMenu = new JMenu("Edit");
        editMenu.setName("editMenu");
        editMenu.setMnemonic(KeyEvent.VK_E);

        JMenuItem cutMenuItem = new JMenuItem("Cut", KeyEvent.VK_T);
        cutMenuItem.setName("cutMenuItem");
        cutMenuItem.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_X, InputEvent.CTRL_DOWN_MASK));

        JMenuItem copyMenuItem = new JMenuItem("Copy", KeyEvent.VK_C);
        copyMenuItem.setName("copyMenuItem");
        copyMenuItem.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_C, InputEvent.CTRL_DOWN_MASK));

        JMenuItem pasteMenuItem = new JMenuItem("Paste", KeyEvent.VK_P);
        pasteMenuItem.setName("pasteMenuItem");
        pasteMenuItem.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_V, InputEvent.CTRL_DOWN_MASK));

        JMenuItem selectAllMenuItem = new JMenuItem("Select All", KeyEvent.VK_A);
        selectAllMenuItem.setName("selectAllMenuItem");
        selectAllMenuItem.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_A, InputEvent.CTRL_DOWN_MASK));

        // Edit submenu
        JMenu findSubmenu = new JMenu("Find");
        findSubmenu.setName("findSubmenu");

        JMenuItem findMenuItem = new JMenuItem("Find...");
        findMenuItem.setName("findMenuItem");
        findMenuItem.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_F, InputEvent.CTRL_DOWN_MASK));

        JMenuItem findNextMenuItem = new JMenuItem("Find Next");
        findNextMenuItem.setName("findNextMenuItem");
        findNextMenuItem.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_F3, 0));

        JMenuItem replaceMenuItem = new JMenuItem("Replace...");
        replaceMenuItem.setName("replaceMenuItem");
        replaceMenuItem.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_H, InputEvent.CTRL_DOWN_MASK));

        findSubmenu.add(findMenuItem);
        findSubmenu.add(findNextMenuItem);
        findSubmenu.add(replaceMenuItem);

        editMenu.add(cutMenuItem);
        editMenu.add(copyMenuItem);
        editMenu.add(pasteMenuItem);
        editMenu.addSeparator();
        editMenu.add(selectAllMenuItem);
        editMenu.addSeparator();
        editMenu.add(findSubmenu);

        // View menu
        JMenu viewMenu = new JMenu("View");
        viewMenu.setName("viewMenu");
        viewMenu.setMnemonic(KeyEvent.VK_V);

        JCheckBoxMenuItem toolbarMenuItem = new JCheckBoxMenuItem("Toolbar", true);
        toolbarMenuItem.setName("toolbarMenuItem");

        JCheckBoxMenuItem statusBarMenuItem = new JCheckBoxMenuItem("Status Bar", true);
        statusBarMenuItem.setName("statusBarMenuItem");

        JRadioButtonMenuItem normalViewMenuItem = new JRadioButtonMenuItem("Normal View", true);
        normalViewMenuItem.setName("normalViewMenuItem");

        JRadioButtonMenuItem compactViewMenuItem = new JRadioButtonMenuItem("Compact View");
        compactViewMenuItem.setName("compactViewMenuItem");

        ButtonGroup viewGroup = new ButtonGroup();
        viewGroup.add(normalViewMenuItem);
        viewGroup.add(compactViewMenuItem);

        viewMenu.add(toolbarMenuItem);
        viewMenu.add(statusBarMenuItem);
        viewMenu.addSeparator();
        viewMenu.add(normalViewMenuItem);
        viewMenu.add(compactViewMenuItem);

        // Help menu
        JMenu helpMenu = new JMenu("Help");
        helpMenu.setName("helpMenu");
        helpMenu.setMnemonic(KeyEvent.VK_H);

        JMenuItem helpContentsMenuItem = new JMenuItem("Help Contents", KeyEvent.VK_H);
        helpContentsMenuItem.setName("helpContentsMenuItem");
        helpContentsMenuItem.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_F1, 0));

        JMenuItem aboutMenuItem = new JMenuItem("About", KeyEvent.VK_A);
        aboutMenuItem.setName("aboutMenuItem");
        aboutMenuItem.addActionListener(e -> showAboutDialog());

        helpMenu.add(helpContentsMenuItem);
        helpMenu.addSeparator();
        helpMenu.add(aboutMenuItem);

        menuBar.add(fileMenu);
        menuBar.add(editMenu);
        menuBar.add(viewMenu);
        menuBar.add(helpMenu);
    }

    private void initToolBar() {
        toolBar = new JToolBar("Main Toolbar");
        toolBar.setName("mainToolBar");
        toolBar.setFloatable(true);

        JButton newButton = new JButton("New");
        newButton.setName("toolbarNewButton");
        newButton.setToolTipText("Create new document");

        JButton openButton = new JButton("Open");
        openButton.setName("toolbarOpenButton");
        openButton.setToolTipText("Open existing document");

        JButton saveButton = new JButton("Save");
        saveButton.setName("toolbarSaveButton");
        saveButton.setToolTipText("Save current document");

        JButton cutButton = new JButton("Cut");
        cutButton.setName("toolbarCutButton");
        cutButton.setToolTipText("Cut selection to clipboard");

        JButton copyButton = new JButton("Copy");
        copyButton.setName("toolbarCopyButton");
        copyButton.setToolTipText("Copy selection to clipboard");

        JButton pasteButton = new JButton("Paste");
        pasteButton.setName("toolbarPasteButton");
        pasteButton.setToolTipText("Paste from clipboard");

        JButton refreshButton = new JButton("Refresh");
        refreshButton.setName("toolbarRefreshButton");
        refreshButton.setToolTipText("Refresh the view");

        JButton settingsButton = new JButton("Settings");
        settingsButton.setName("toolbarSettingsButton");
        settingsButton.setToolTipText("Open settings dialog");
        settingsButton.addActionListener(e -> showSettingsDialog());

        toolBar.add(newButton);
        toolBar.add(openButton);
        toolBar.add(saveButton);
        toolBar.addSeparator();
        toolBar.add(cutButton);
        toolBar.add(copyButton);
        toolBar.add(pasteButton);
        toolBar.addSeparator();
        toolBar.add(refreshButton);
        toolBar.add(settingsButton);
    }

    private void initTabbedPane() {
        tabbedPane = new JTabbedPane();
        tabbedPane.setName("mainTabbedPane");

        // Tab 1: Form Input
        JPanel formPanel = createFormPanel();
        tabbedPane.addTab("Form Input", null, formPanel, "Input form fields");

        // Tab 2: Selections
        JPanel selectionsPanel = createSelectionsPanel();
        tabbedPane.addTab("Selections", null, selectionsPanel, "Selection controls");

        // Tab 3: Data View
        JPanel dataPanel = createDataPanel();
        tabbedPane.addTab("Data View", null, dataPanel, "Data tables and lists");

        // Tab 4: Settings
        JPanel settingsPanel = createSettingsTabPanel();
        tabbedPane.addTab("Settings", null, settingsPanel, "Application settings");
    }

    private JPanel createFormPanel() {
        JPanel panel = new JPanel(new GridBagLayout());
        panel.setName("formPanel");
        GridBagConstraints gbc = new GridBagConstraints();
        gbc.insets = new Insets(5, 5, 5, 5);
        gbc.fill = GridBagConstraints.HORIZONTAL;

        // Name field
        JLabel nameLabel = new JLabel("Name:");
        nameLabel.setName("nameLabel");
        nameTextField = new JTextField(20);
        nameTextField.setName("nameTextField");
        nameTextField.setToolTipText("Enter your full name");

        gbc.gridx = 0; gbc.gridy = 0;
        panel.add(nameLabel, gbc);
        gbc.gridx = 1;
        panel.add(nameTextField, gbc);

        // Email field
        JLabel emailLabel = new JLabel("Email:");
        emailLabel.setName("emailLabel");
        emailTextField = new JTextField(20);
        emailTextField.setName("emailTextField");
        emailTextField.setToolTipText("Enter your email address");

        gbc.gridx = 0; gbc.gridy = 1;
        panel.add(emailLabel, gbc);
        gbc.gridx = 1;
        panel.add(emailTextField, gbc);

        // Password field
        JLabel passwordLabel = new JLabel("Password:");
        passwordLabel.setName("passwordLabel");
        passwordField = new JPasswordField(20);
        passwordField.setName("passwordField");
        passwordField.setToolTipText("Enter your password");

        gbc.gridx = 0; gbc.gridy = 2;
        panel.add(passwordLabel, gbc);
        gbc.gridx = 1;
        panel.add(passwordField, gbc);

        // Description text area
        JLabel descriptionLabel = new JLabel("Description:");
        descriptionLabel.setName("descriptionLabel");
        descriptionTextArea = new JTextArea(5, 20);
        descriptionTextArea.setName("descriptionTextArea");
        descriptionTextArea.setLineWrap(true);
        descriptionTextArea.setWrapStyleWord(true);
        descriptionTextArea.setToolTipText("Enter a description");
        JScrollPane descScrollPane = new JScrollPane(descriptionTextArea);
        descScrollPane.setName("descriptionScrollPane");

        gbc.gridx = 0; gbc.gridy = 3;
        gbc.anchor = GridBagConstraints.NORTHWEST;
        panel.add(descriptionLabel, gbc);
        gbc.gridx = 1;
        gbc.fill = GridBagConstraints.BOTH;
        gbc.weightx = 1.0;
        gbc.weighty = 1.0;
        panel.add(descScrollPane, gbc);

        // Buttons panel
        JPanel buttonPanel = new JPanel(new FlowLayout(FlowLayout.RIGHT));
        buttonPanel.setName("formButtonPanel");

        submitButton = new JButton("Submit");
        submitButton.setName("submitButton");
        submitButton.setToolTipText("Submit the form");

        clearButton = new JButton("Clear");
        clearButton.setName("clearButton");
        clearButton.setToolTipText("Clear all fields");
        clearButton.addActionListener(e -> clearFormFields());

        buttonPanel.add(clearButton);
        buttonPanel.add(submitButton);

        gbc.gridx = 0; gbc.gridy = 4;
        gbc.gridwidth = 2;
        gbc.fill = GridBagConstraints.HORIZONTAL;
        gbc.weightx = 0;
        gbc.weighty = 0;
        panel.add(buttonPanel, gbc);

        return panel;
    }

    private JPanel createSelectionsPanel() {
        JPanel panel = new JPanel(new GridBagLayout());
        panel.setName("selectionsPanel");
        GridBagConstraints gbc = new GridBagConstraints();
        gbc.insets = new Insets(5, 5, 5, 5);
        gbc.fill = GridBagConstraints.HORIZONTAL;
        gbc.anchor = GridBagConstraints.WEST;

        // ComboBox
        JLabel categoryLabel = new JLabel("Category:");
        categoryLabel.setName("categoryLabel");
        categoryComboBox = new JComboBox<>(new String[]{
            "Electronics", "Clothing", "Books", "Home & Garden", "Sports", "Toys"
        });
        categoryComboBox.setName("categoryComboBox");
        categoryComboBox.setEditable(true);
        categoryComboBox.setToolTipText("Select or enter a category");

        gbc.gridx = 0; gbc.gridy = 0;
        panel.add(categoryLabel, gbc);
        gbc.gridx = 1;
        panel.add(categoryComboBox, gbc);

        // Spinner
        JLabel quantityLabel = new JLabel("Quantity:");
        quantityLabel.setName("quantityLabel");
        SpinnerNumberModel spinnerModel = new SpinnerNumberModel(1, 0, 100, 1);
        quantitySpinner = new JSpinner(spinnerModel);
        quantitySpinner.setName("quantitySpinner");
        quantitySpinner.setToolTipText("Select quantity");

        gbc.gridx = 0; gbc.gridy = 1;
        panel.add(quantityLabel, gbc);
        gbc.gridx = 1;
        panel.add(quantitySpinner, gbc);

        // Slider
        JLabel volumeLabel = new JLabel("Volume:");
        volumeLabel.setName("volumeLabel");
        volumeSlider = new JSlider(JSlider.HORIZONTAL, 0, 100, 50);
        volumeSlider.setName("volumeSlider");
        volumeSlider.setMajorTickSpacing(25);
        volumeSlider.setMinorTickSpacing(5);
        volumeSlider.setPaintTicks(true);
        volumeSlider.setPaintLabels(true);
        volumeSlider.setToolTipText("Adjust volume level");

        gbc.gridx = 0; gbc.gridy = 2;
        panel.add(volumeLabel, gbc);
        gbc.gridx = 1;
        gbc.fill = GridBagConstraints.HORIZONTAL;
        panel.add(volumeSlider, gbc);

        // Checkboxes
        JLabel optionsLabel = new JLabel("Options:");
        optionsLabel.setName("optionsLabel");

        JPanel checkboxPanel = new JPanel(new GridLayout(3, 1));
        checkboxPanel.setName("checkboxPanel");

        enabledCheckBox = new JCheckBox("Enabled");
        enabledCheckBox.setName("enabledCheckBox");
        enabledCheckBox.setSelected(true);
        enabledCheckBox.setToolTipText("Enable or disable the feature");

        notificationsCheckBox = new JCheckBox("Enable Notifications");
        notificationsCheckBox.setName("notificationsCheckBox");
        notificationsCheckBox.setToolTipText("Enable email notifications");

        autoSaveCheckBox = new JCheckBox("Auto-save");
        autoSaveCheckBox.setName("autoSaveCheckBox");
        autoSaveCheckBox.setToolTipText("Enable automatic saving");

        checkboxPanel.add(enabledCheckBox);
        checkboxPanel.add(notificationsCheckBox);
        checkboxPanel.add(autoSaveCheckBox);

        gbc.gridx = 0; gbc.gridy = 3;
        gbc.fill = GridBagConstraints.NONE;
        panel.add(optionsLabel, gbc);
        gbc.gridx = 1;
        panel.add(checkboxPanel, gbc);

        // Radio buttons
        JLabel priorityLabel = new JLabel("Priority:");
        priorityLabel.setName("priorityLabel");

        JPanel radioPanel = new JPanel(new GridLayout(3, 1));
        radioPanel.setName("radioPanel");

        optionARadioButton = new JRadioButton("High Priority");
        optionARadioButton.setName("highPriorityRadioButton");
        optionARadioButton.setToolTipText("Set high priority");

        optionBRadioButton = new JRadioButton("Normal Priority");
        optionBRadioButton.setName("normalPriorityRadioButton");
        optionBRadioButton.setSelected(true);
        optionBRadioButton.setToolTipText("Set normal priority");

        optionCRadioButton = new JRadioButton("Low Priority");
        optionCRadioButton.setName("lowPriorityRadioButton");
        optionCRadioButton.setToolTipText("Set low priority");

        optionButtonGroup = new ButtonGroup();
        optionButtonGroup.add(optionARadioButton);
        optionButtonGroup.add(optionBRadioButton);
        optionButtonGroup.add(optionCRadioButton);

        radioPanel.add(optionARadioButton);
        radioPanel.add(optionBRadioButton);
        radioPanel.add(optionCRadioButton);

        gbc.gridx = 0; gbc.gridy = 4;
        panel.add(priorityLabel, gbc);
        gbc.gridx = 1;
        panel.add(radioPanel, gbc);

        // List
        JLabel itemsLabel = new JLabel("Items:");
        itemsLabel.setName("itemsLabel");

        DefaultListModel<String> listModel = new DefaultListModel<>();
        listModel.addElement("Item 1 - Apple");
        listModel.addElement("Item 2 - Banana");
        listModel.addElement("Item 3 - Cherry");
        listModel.addElement("Item 4 - Date");
        listModel.addElement("Item 5 - Elderberry");
        listModel.addElement("Item 6 - Fig");
        listModel.addElement("Item 7 - Grape");

        itemList = new JList<>(listModel);
        itemList.setName("itemList");
        itemList.setSelectionMode(ListSelectionModel.MULTIPLE_INTERVAL_SELECTION);
        itemList.setVisibleRowCount(5);
        itemList.setToolTipText("Select items from the list");
        JScrollPane listScrollPane = new JScrollPane(itemList);
        listScrollPane.setName("itemListScrollPane");

        gbc.gridx = 0; gbc.gridy = 5;
        gbc.anchor = GridBagConstraints.NORTHWEST;
        panel.add(itemsLabel, gbc);
        gbc.gridx = 1;
        gbc.fill = GridBagConstraints.BOTH;
        gbc.weightx = 1.0;
        gbc.weighty = 1.0;
        panel.add(listScrollPane, gbc);

        return panel;
    }

    private JPanel createDataPanel() {
        JPanel panel = new JPanel(new BorderLayout(5, 5));
        panel.setName("dataPanel");
        panel.setBorder(BorderFactory.createEmptyBorder(5, 5, 5, 5));

        // Table at the top
        JScrollPane tableScrollPane = new JScrollPane(dataTable);
        tableScrollPane.setName("dataTableScrollPane");
        tableScrollPane.setPreferredSize(new Dimension(400, 200));

        panel.add(tableScrollPane, BorderLayout.CENTER);

        // Buttons below table
        JPanel tableButtonPanel = new JPanel(new FlowLayout(FlowLayout.LEFT));
        tableButtonPanel.setName("tableButtonPanel");

        JButton addRowButton = new JButton("Add Row");
        addRowButton.setName("addRowButton");
        addRowButton.setToolTipText("Add a new row to the table");
        addRowButton.addActionListener(e -> addTableRow());

        JButton deleteRowButton = new JButton("Delete Row");
        deleteRowButton.setName("deleteRowButton");
        deleteRowButton.setToolTipText("Delete selected row from the table");
        deleteRowButton.addActionListener(e -> deleteTableRow());

        JButton editRowButton = new JButton("Edit Row");
        editRowButton.setName("editRowButton");
        editRowButton.setToolTipText("Edit selected row");

        tableButtonPanel.add(addRowButton);
        tableButtonPanel.add(deleteRowButton);
        tableButtonPanel.add(editRowButton);

        panel.add(tableButtonPanel, BorderLayout.SOUTH);

        return panel;
    }

    private JPanel createSettingsTabPanel() {
        JPanel panel = new JPanel(new BorderLayout(5, 5));
        panel.setName("settingsTabPanel");
        panel.setBorder(BorderFactory.createEmptyBorder(10, 10, 10, 10));

        // Settings form
        JPanel settingsForm = new JPanel(new GridBagLayout());
        settingsForm.setName("settingsFormPanel");
        GridBagConstraints gbc = new GridBagConstraints();
        gbc.insets = new Insets(5, 5, 5, 5);
        gbc.anchor = GridBagConstraints.WEST;

        // Theme selection
        JLabel themeLabel = new JLabel("Theme:");
        themeLabel.setName("themeLabel");
        JComboBox<String> themeComboBox = new JComboBox<>(new String[]{
            "Light", "Dark", "System Default", "High Contrast"
        });
        themeComboBox.setName("themeComboBox");

        gbc.gridx = 0; gbc.gridy = 0;
        settingsForm.add(themeLabel, gbc);
        gbc.gridx = 1;
        settingsForm.add(themeComboBox, gbc);

        // Font size
        JLabel fontSizeLabel = new JLabel("Font Size:");
        fontSizeLabel.setName("fontSizeLabel");
        JSpinner fontSizeSpinner = new JSpinner(new SpinnerNumberModel(12, 8, 24, 1));
        fontSizeSpinner.setName("fontSizeSpinner");

        gbc.gridx = 0; gbc.gridy = 1;
        settingsForm.add(fontSizeLabel, gbc);
        gbc.gridx = 1;
        settingsForm.add(fontSizeSpinner, gbc);

        // Language
        JLabel languageLabel = new JLabel("Language:");
        languageLabel.setName("languageLabel");
        JComboBox<String> languageComboBox = new JComboBox<>(new String[]{
            "English", "Spanish", "French", "German", "Japanese", "Chinese"
        });
        languageComboBox.setName("languageComboBox");

        gbc.gridx = 0; gbc.gridy = 2;
        settingsForm.add(languageLabel, gbc);
        gbc.gridx = 1;
        settingsForm.add(languageComboBox, gbc);

        // Checkboxes for various settings
        JCheckBox showLineNumbersCheckBox = new JCheckBox("Show Line Numbers");
        showLineNumbersCheckBox.setName("showLineNumbersCheckBox");
        showLineNumbersCheckBox.setSelected(true);

        JCheckBox wordWrapCheckBox = new JCheckBox("Word Wrap");
        wordWrapCheckBox.setName("wordWrapCheckBox");

        JCheckBox highlightCurrentLineCheckBox = new JCheckBox("Highlight Current Line");
        highlightCurrentLineCheckBox.setName("highlightCurrentLineCheckBox");
        highlightCurrentLineCheckBox.setSelected(true);

        gbc.gridx = 0; gbc.gridy = 3; gbc.gridwidth = 2;
        settingsForm.add(showLineNumbersCheckBox, gbc);
        gbc.gridy = 4;
        settingsForm.add(wordWrapCheckBox, gbc);
        gbc.gridy = 5;
        settingsForm.add(highlightCurrentLineCheckBox, gbc);

        panel.add(settingsForm, BorderLayout.NORTH);

        // Buttons
        JPanel buttonPanel = new JPanel(new FlowLayout(FlowLayout.RIGHT));
        buttonPanel.setName("settingsButtonPanel");

        JButton applyButton = new JButton("Apply");
        applyButton.setName("applySettingsButton");
        applyButton.setToolTipText("Apply settings");

        JButton resetButton = new JButton("Reset to Defaults");
        resetButton.setName("resetSettingsButton");
        resetButton.setToolTipText("Reset all settings to defaults");

        buttonPanel.add(resetButton);
        buttonPanel.add(applyButton);

        panel.add(buttonPanel, BorderLayout.SOUTH);

        return panel;
    }

    private void initTree() {
        // Create root node
        DefaultMutableTreeNode root = new DefaultMutableTreeNode("Project Root");

        // Level 1: Main categories
        DefaultMutableTreeNode sourcesNode = new DefaultMutableTreeNode("Sources");
        DefaultMutableTreeNode resourcesNode = new DefaultMutableTreeNode("Resources");
        DefaultMutableTreeNode testsNode = new DefaultMutableTreeNode("Tests");

        // Level 2: Under Sources
        DefaultMutableTreeNode mainPackage = new DefaultMutableTreeNode("com.example.main");
        DefaultMutableTreeNode utilPackage = new DefaultMutableTreeNode("com.example.util");
        DefaultMutableTreeNode modelPackage = new DefaultMutableTreeNode("com.example.model");

        // Level 3: Under main package
        mainPackage.add(new DefaultMutableTreeNode("Application.java"));
        mainPackage.add(new DefaultMutableTreeNode("MainController.java"));
        mainPackage.add(new DefaultMutableTreeNode("MainView.java"));

        // Level 3: Under util package
        utilPackage.add(new DefaultMutableTreeNode("StringUtils.java"));
        utilPackage.add(new DefaultMutableTreeNode("FileUtils.java"));
        utilPackage.add(new DefaultMutableTreeNode("DateUtils.java"));

        // Level 3: Under model package
        modelPackage.add(new DefaultMutableTreeNode("User.java"));
        modelPackage.add(new DefaultMutableTreeNode("Product.java"));
        modelPackage.add(new DefaultMutableTreeNode("Order.java"));

        sourcesNode.add(mainPackage);
        sourcesNode.add(utilPackage);
        sourcesNode.add(modelPackage);

        // Level 2: Under Resources
        DefaultMutableTreeNode imagesNode = new DefaultMutableTreeNode("images");
        DefaultMutableTreeNode configNode = new DefaultMutableTreeNode("config");

        imagesNode.add(new DefaultMutableTreeNode("logo.png"));
        imagesNode.add(new DefaultMutableTreeNode("icon.png"));
        imagesNode.add(new DefaultMutableTreeNode("banner.jpg"));

        configNode.add(new DefaultMutableTreeNode("application.properties"));
        configNode.add(new DefaultMutableTreeNode("database.properties"));
        configNode.add(new DefaultMutableTreeNode("logging.xml"));

        resourcesNode.add(imagesNode);
        resourcesNode.add(configNode);

        // Level 2: Under Tests
        DefaultMutableTreeNode unitTests = new DefaultMutableTreeNode("unit");
        DefaultMutableTreeNode integrationTests = new DefaultMutableTreeNode("integration");

        unitTests.add(new DefaultMutableTreeNode("ApplicationTest.java"));
        unitTests.add(new DefaultMutableTreeNode("UserTest.java"));
        unitTests.add(new DefaultMutableTreeNode("ProductTest.java"));

        integrationTests.add(new DefaultMutableTreeNode("DatabaseIntegrationTest.java"));
        integrationTests.add(new DefaultMutableTreeNode("APIIntegrationTest.java"));

        testsNode.add(unitTests);
        testsNode.add(integrationTests);

        // Build tree structure
        root.add(sourcesNode);
        root.add(resourcesNode);
        root.add(testsNode);

        // Create tree
        fileTree = new JTree(root);
        fileTree.setName("fileTree");
        fileTree.setRootVisible(true);
        fileTree.setShowsRootHandles(true);
        fileTree.setToolTipText("Project file structure");

        // Expand first level
        fileTree.expandRow(0);
        fileTree.expandRow(1);
    }

    private void initTable() {
        // Create table model with columns
        String[] columnNames = {"ID", "Name", "Category", "Price", "In Stock"};
        Object[][] data = {
            {1, "Laptop", "Electronics", 999.99, true},
            {2, "T-Shirt", "Clothing", 29.99, true},
            {3, "Java Programming Book", "Books", 49.99, false},
            {4, "Garden Hose", "Home & Garden", 34.99, true},
            {5, "Basketball", "Sports", 24.99, true},
            {6, "Action Figure", "Toys", 19.99, true},
            {7, "Smartphone", "Electronics", 699.99, false},
            {8, "Running Shoes", "Sports", 89.99, true}
        };

        DefaultTableModel tableModel = new DefaultTableModel(data, columnNames) {
            @Override
            public Class<?> getColumnClass(int columnIndex) {
                if (columnIndex == 0) return Integer.class;
                if (columnIndex == 3) return Double.class;
                if (columnIndex == 4) return Boolean.class;
                return String.class;
            }

            @Override
            public boolean isCellEditable(int row, int column) {
                return column != 0; // ID is not editable
            }
        };

        dataTable = new JTable(tableModel);
        dataTable.setName("dataTable");
        dataTable.setSelectionMode(ListSelectionModel.SINGLE_SELECTION);
        dataTable.setAutoCreateRowSorter(true);
        dataTable.setRowHeight(25);
        dataTable.getTableHeader().setReorderingAllowed(true);
        dataTable.setToolTipText("Product data table");

        // Set column widths
        dataTable.getColumnModel().getColumn(0).setPreferredWidth(50);
        dataTable.getColumnModel().getColumn(1).setPreferredWidth(150);
        dataTable.getColumnModel().getColumn(2).setPreferredWidth(100);
        dataTable.getColumnModel().getColumn(3).setPreferredWidth(80);
        dataTable.getColumnModel().getColumn(4).setPreferredWidth(70);
    }

    private void initPopupMenus() {
        // Table popup menu
        tablePopupMenu = new JPopupMenu();
        tablePopupMenu.setName("tablePopupMenu");

        JMenuItem viewDetailsMenuItem = new JMenuItem("View Details");
        viewDetailsMenuItem.setName("viewDetailsMenuItem");

        JMenuItem editItemMenuItem = new JMenuItem("Edit Item");
        editItemMenuItem.setName("editItemMenuItem");

        JMenuItem deleteItemMenuItem = new JMenuItem("Delete Item");
        deleteItemMenuItem.setName("deleteItemMenuItem");

        JMenuItem copyToClipboardMenuItem = new JMenuItem("Copy to Clipboard");
        copyToClipboardMenuItem.setName("copyToClipboardMenuItem");

        tablePopupMenu.add(viewDetailsMenuItem);
        tablePopupMenu.add(editItemMenuItem);
        tablePopupMenu.addSeparator();
        tablePopupMenu.add(deleteItemMenuItem);
        tablePopupMenu.addSeparator();
        tablePopupMenu.add(copyToClipboardMenuItem);

        // Tree popup menu
        treePopupMenu = new JPopupMenu();
        treePopupMenu.setName("treePopupMenu");

        JMenuItem openFileMenuItem = new JMenuItem("Open");
        openFileMenuItem.setName("openFileMenuItem");

        JMenuItem renameMenuItem = new JMenuItem("Rename");
        renameMenuItem.setName("renameMenuItem");

        JMenuItem deleteFileMenuItem = new JMenuItem("Delete");
        deleteFileMenuItem.setName("deleteFileMenuItem");

        JMenuItem newFolderMenuItem = new JMenuItem("New Folder");
        newFolderMenuItem.setName("newFolderMenuItem");

        JMenuItem newFileMenuItem = new JMenuItem("New File");
        newFileMenuItem.setName("newFileMenuItem");

        JMenuItem refreshTreeMenuItem = new JMenuItem("Refresh");
        refreshTreeMenuItem.setName("refreshTreeMenuItem");

        treePopupMenu.add(openFileMenuItem);
        treePopupMenu.add(renameMenuItem);
        treePopupMenu.add(deleteFileMenuItem);
        treePopupMenu.addSeparator();
        treePopupMenu.add(newFolderMenuItem);
        treePopupMenu.add(newFileMenuItem);
        treePopupMenu.addSeparator();
        treePopupMenu.add(refreshTreeMenuItem);
    }

    private void initDialogs() {
        // Settings dialog (modeless)
        settingsDialog = new JDialog(this, "Settings", false);
        settingsDialog.setName("settingsDialog");
        settingsDialog.setSize(400, 300);
        settingsDialog.setLocationRelativeTo(this);

        JPanel dialogPanel = new JPanel(new BorderLayout(10, 10));
        dialogPanel.setBorder(BorderFactory.createEmptyBorder(10, 10, 10, 10));

        JLabel dialogLabel = new JLabel("Settings Dialog Content");
        dialogLabel.setName("settingsDialogLabel");
        dialogPanel.add(dialogLabel, BorderLayout.CENTER);

        JPanel dialogButtonPanel = new JPanel(new FlowLayout(FlowLayout.RIGHT));
        JButton dialogOkButton = new JButton("OK");
        dialogOkButton.setName("settingsDialogOkButton");
        dialogOkButton.addActionListener(e -> settingsDialog.setVisible(false));

        JButton dialogCancelButton = new JButton("Cancel");
        dialogCancelButton.setName("settingsDialogCancelButton");
        dialogCancelButton.addActionListener(e -> settingsDialog.setVisible(false));

        dialogButtonPanel.add(dialogCancelButton);
        dialogButtonPanel.add(dialogOkButton);
        dialogPanel.add(dialogButtonPanel, BorderLayout.SOUTH);

        settingsDialog.add(dialogPanel);

        // About dialog (modal)
        aboutDialog = new JDialog(this, "About SwingTestApp", true);
        aboutDialog.setName("aboutDialog");
        aboutDialog.setSize(350, 200);
        aboutDialog.setLocationRelativeTo(this);
        aboutDialog.setResizable(false);

        JPanel aboutPanel = new JPanel(new BorderLayout(10, 10));
        aboutPanel.setBorder(BorderFactory.createEmptyBorder(20, 20, 20, 20));

        JPanel infoPanel = new JPanel(new GridLayout(4, 1, 5, 5));
        JLabel appNameLabel = new JLabel("SwingTestApp", SwingConstants.CENTER);
        appNameLabel.setName("aboutAppNameLabel");
        appNameLabel.setFont(appNameLabel.getFont().deriveFont(Font.BOLD, 16f));

        JLabel versionLabel = new JLabel("Version 1.0.0", SwingConstants.CENTER);
        versionLabel.setName("aboutVersionLabel");

        JLabel copyrightLabel = new JLabel("Copyright 2024", SwingConstants.CENTER);
        copyrightLabel.setName("aboutCopyrightLabel");

        JLabel descLabel = new JLabel("A comprehensive Swing test application", SwingConstants.CENTER);
        descLabel.setName("aboutDescLabel");

        infoPanel.add(appNameLabel);
        infoPanel.add(versionLabel);
        infoPanel.add(copyrightLabel);
        infoPanel.add(descLabel);

        aboutPanel.add(infoPanel, BorderLayout.CENTER);

        JButton aboutCloseButton = new JButton("Close");
        aboutCloseButton.setName("aboutCloseButton");
        aboutCloseButton.addActionListener(e -> aboutDialog.setVisible(false));

        JPanel aboutButtonPanel = new JPanel(new FlowLayout(FlowLayout.CENTER));
        aboutButtonPanel.add(aboutCloseButton);
        aboutPanel.add(aboutButtonPanel, BorderLayout.SOUTH);

        aboutDialog.add(aboutPanel);
    }

    private void layoutComponents() {
        setLayout(new BorderLayout());

        // Add menu bar
        setJMenuBar(menuBar);

        // Add toolbar at top
        add(toolBar, BorderLayout.NORTH);

        // Create main split pane (tree on left, tabbed pane on right)
        JScrollPane treeScrollPane = new JScrollPane(fileTree);
        treeScrollPane.setName("treeScrollPane");
        treeScrollPane.setPreferredSize(new Dimension(250, 400));

        mainSplitPane = new JSplitPane(JSplitPane.HORIZONTAL_SPLIT, treeScrollPane, tabbedPane);
        mainSplitPane.setName("mainSplitPane");
        mainSplitPane.setDividerLocation(250);
        mainSplitPane.setOneTouchExpandable(true);

        add(mainSplitPane, BorderLayout.CENTER);

        // Status panel at bottom
        JPanel statusPanel = new JPanel(new BorderLayout(5, 0));
        statusPanel.setName("statusPanel");
        statusPanel.setBorder(BorderFactory.createEmptyBorder(2, 5, 2, 5));

        statusPanel.add(statusLabel, BorderLayout.WEST);
        statusPanel.add(progressBar, BorderLayout.EAST);

        // Add dialog buttons to status panel
        JPanel dialogButtonsPanel = new JPanel(new FlowLayout(FlowLayout.CENTER));
        dialogButtonsPanel.setName("dialogButtonsPanel");

        openDialogButton = new JButton("Open Dialog");
        openDialogButton.setName("openDialogButton");
        openDialogButton.setToolTipText("Open a modeless dialog");
        openDialogButton.addActionListener(e -> showSettingsDialog());

        openModalDialogButton = new JButton("Open Modal Dialog");
        openModalDialogButton.setName("openModalDialogButton");
        openModalDialogButton.setToolTipText("Open a modal dialog");
        openModalDialogButton.addActionListener(e -> showAboutDialog());

        JButton startProgressButton = new JButton("Start Progress");
        startProgressButton.setName("startProgressButton");
        startProgressButton.setToolTipText("Start progress bar animation");
        startProgressButton.addActionListener(e -> startProgress());

        dialogButtonsPanel.add(openDialogButton);
        dialogButtonsPanel.add(openModalDialogButton);
        dialogButtonsPanel.add(startProgressButton);

        statusPanel.add(dialogButtonsPanel, BorderLayout.CENTER);

        add(statusPanel, BorderLayout.SOUTH);
    }

    private void setupEventHandlers() {
        // Table right-click handler
        dataTable.addMouseListener(new MouseAdapter() {
            @Override
            public void mousePressed(MouseEvent e) {
                showPopupIfNeeded(e);
            }

            @Override
            public void mouseReleased(MouseEvent e) {
                showPopupIfNeeded(e);
            }

            private void showPopupIfNeeded(MouseEvent e) {
                if (e.isPopupTrigger()) {
                    int row = dataTable.rowAtPoint(e.getPoint());
                    if (row >= 0 && row < dataTable.getRowCount()) {
                        dataTable.setRowSelectionInterval(row, row);
                    }
                    tablePopupMenu.show(e.getComponent(), e.getX(), e.getY());
                }
            }
        });

        // Tree right-click handler
        fileTree.addMouseListener(new MouseAdapter() {
            @Override
            public void mousePressed(MouseEvent e) {
                showPopupIfNeeded(e);
            }

            @Override
            public void mouseReleased(MouseEvent e) {
                showPopupIfNeeded(e);
            }

            private void showPopupIfNeeded(MouseEvent e) {
                if (e.isPopupTrigger()) {
                    int row = fileTree.getClosestRowForLocation(e.getX(), e.getY());
                    if (row >= 0) {
                        fileTree.setSelectionRow(row);
                    }
                    treePopupMenu.show(e.getComponent(), e.getX(), e.getY());
                }
            }
        });

        // Tree selection listener
        fileTree.addTreeSelectionListener(e -> {
            DefaultMutableTreeNode node = (DefaultMutableTreeNode) fileTree.getLastSelectedPathComponent();
            if (node != null) {
                statusLabel.setText("Selected: " + node.getUserObject().toString());
            }
        });

        // Table selection listener
        dataTable.getSelectionModel().addListSelectionListener(e -> {
            if (!e.getValueIsAdjusting()) {
                int selectedRow = dataTable.getSelectedRow();
                if (selectedRow >= 0) {
                    Object name = dataTable.getValueAt(selectedRow, 1);
                    statusLabel.setText("Selected: " + name);
                }
            }
        });

        // Submit button handler
        submitButton.addActionListener(e -> {
            String name = nameTextField.getText();
            String email = emailTextField.getText();
            if (name.isEmpty() || email.isEmpty()) {
                JOptionPane.showMessageDialog(this,
                    "Please fill in all required fields",
                    "Validation Error",
                    JOptionPane.WARNING_MESSAGE);
            } else {
                JOptionPane.showMessageDialog(this,
                    "Form submitted successfully!\nName: " + name + "\nEmail: " + email,
                    "Success",
                    JOptionPane.INFORMATION_MESSAGE);
            }
        });
    }

    private void finalizeFrame() {
        setTitle("SwingTestApp - Comprehensive Swing Component Test");
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        setSize(1024, 768);
        setMinimumSize(new Dimension(800, 600));
        setLocationRelativeTo(null);
        setName("swingTestAppFrame");
    }

    // Action methods
    private void clearFormFields() {
        nameTextField.setText("");
        emailTextField.setText("");
        passwordField.setText("");
        descriptionTextArea.setText("");
        statusLabel.setText("Form cleared");
    }

    private void showSettingsDialog() {
        settingsDialog.setVisible(true);
    }

    private void showAboutDialog() {
        aboutDialog.setVisible(true);
    }

    private void addTableRow() {
        DefaultTableModel model = (DefaultTableModel) dataTable.getModel();
        int newId = model.getRowCount() + 1;
        model.addRow(new Object[]{newId, "New Item", "Category", 0.00, true});
        statusLabel.setText("Row added");
    }

    private void deleteTableRow() {
        int selectedRow = dataTable.getSelectedRow();
        if (selectedRow >= 0) {
            DefaultTableModel model = (DefaultTableModel) dataTable.getModel();
            model.removeRow(selectedRow);
            statusLabel.setText("Row deleted");
        } else {
            JOptionPane.showMessageDialog(this,
                "Please select a row to delete",
                "No Selection",
                JOptionPane.WARNING_MESSAGE);
        }
    }

    private void startProgress() {
        progressBar.setValue(0);
        Timer timer = new Timer(50, null);
        timer.addActionListener(e -> {
            int value = progressBar.getValue();
            if (value < 100) {
                progressBar.setValue(value + 2);
            } else {
                ((Timer) e.getSource()).stop();
                statusLabel.setText("Progress complete");
            }
        });
        timer.start();
        statusLabel.setText("Progress started...");
    }

    public static void main(String[] args) {
        // Set look and feel to system default
        try {
            UIManager.setLookAndFeel(UIManager.getSystemLookAndFeelClassName());
        } catch (Exception e) {
            // Fall back to default look and feel
        }

        // Create and show the application
        SwingUtilities.invokeLater(() -> {
            SwingTestApp app = new SwingTestApp();
            app.setVisible(true);
        });
    }
}
