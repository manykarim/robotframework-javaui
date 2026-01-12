package com.example.demo;

import javax.swing.*;
import javax.swing.border.*;
import javax.swing.table.*;
import javax.swing.tree.*;
import java.awt.*;
import java.awt.event.*;
import java.util.ArrayList;
import java.util.List;

/**
 * Demo Swing application for testing Robot Framework Swing Library.
 * Contains various Swing components for comprehensive testing.
 */
public class SwingDemoApp extends JFrame {

    // Components accessible for testing
    private JTextField usernameField;
    private JPasswordField passwordField;
    private JButton loginButton;
    private JButton clearButton;
    private JLabel statusLabel;
    private JTabbedPane tabbedPane;
    private JTable dataTable;
    private JTree fileTree;
    private JList<String> itemList;
    private JComboBox<String> countryCombo;
    private JCheckBox rememberCheckbox;
    private JRadioButton optionA;
    private JRadioButton optionB;
    private JSlider volumeSlider;
    private JSpinner quantitySpinner;
    private JProgressBar progressBar;
    private JTextArea notesArea;
    private JMenuBar menuBar;

    public SwingDemoApp() {
        super("Swing Demo Application");
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        initComponents();
        pack();
        setLocationRelativeTo(null);
    }

    private void initComponents() {
        setLayout(new BorderLayout(10, 10));

        // Create menu bar
        createMenuBar();

        // Create tabbed pane with different panels
        tabbedPane = new JTabbedPane();
        tabbedPane.setName("mainTabs");

        tabbedPane.addTab("Login", createLoginPanel());
        tabbedPane.addTab("Data Table", createTablePanel());
        tabbedPane.addTab("Tree View", createTreePanel());
        tabbedPane.addTab("Form Controls", createFormPanel());
        tabbedPane.addTab("Lists", createListPanel());

        add(tabbedPane, BorderLayout.CENTER);

        // Status bar
        JPanel statusBar = new JPanel(new FlowLayout(FlowLayout.LEFT));
        statusBar.setBorder(new BevelBorder(BevelBorder.LOWERED));
        statusLabel = new JLabel("Ready");
        statusLabel.setName("statusLabel");
        statusBar.add(statusLabel);
        add(statusBar, BorderLayout.SOUTH);
    }

    private void createMenuBar() {
        menuBar = new JMenuBar();

        // File menu
        JMenu fileMenu = new JMenu("File");
        fileMenu.setMnemonic('F');

        JMenuItem newItem = new JMenuItem("New", 'N');
        newItem.setName("menuNew");
        newItem.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_N, InputEvent.CTRL_DOWN_MASK));
        newItem.addActionListener(e -> statusLabel.setText("New clicked"));

        JMenuItem openItem = new JMenuItem("Open", 'O');
        openItem.setName("menuOpen");
        openItem.addActionListener(e -> statusLabel.setText("Open clicked"));

        JMenuItem saveItem = new JMenuItem("Save", 'S');
        saveItem.setName("menuSave");
        saveItem.addActionListener(e -> statusLabel.setText("Save clicked"));

        JMenuItem exitItem = new JMenuItem("Exit", 'x');
        exitItem.setName("menuExit");
        exitItem.addActionListener(e -> System.exit(0));

        fileMenu.add(newItem);
        fileMenu.add(openItem);
        fileMenu.add(saveItem);
        fileMenu.addSeparator();
        fileMenu.add(exitItem);

        // Edit menu
        JMenu editMenu = new JMenu("Edit");
        editMenu.setMnemonic('E');

        JMenuItem cutItem = new JMenuItem("Cut");
        cutItem.setName("menuCut");
        JMenuItem copyItem = new JMenuItem("Copy");
        copyItem.setName("menuCopy");
        JMenuItem pasteItem = new JMenuItem("Paste");
        pasteItem.setName("menuPaste");

        editMenu.add(cutItem);
        editMenu.add(copyItem);
        editMenu.add(pasteItem);

        // Help menu
        JMenu helpMenu = new JMenu("Help");
        helpMenu.setMnemonic('H');

        JMenuItem aboutItem = new JMenuItem("About");
        aboutItem.setName("menuAbout");
        aboutItem.addActionListener(e -> {
            JOptionPane.showMessageDialog(this,
                "Swing Demo Application v1.0\nFor Robot Framework Testing",
                "About",
                JOptionPane.INFORMATION_MESSAGE);
        });

        helpMenu.add(aboutItem);

        menuBar.add(fileMenu);
        menuBar.add(editMenu);
        menuBar.add(helpMenu);

        setJMenuBar(menuBar);
    }

    private JPanel createLoginPanel() {
        JPanel panel = new JPanel(new GridBagLayout());
        panel.setName("loginPanel");
        panel.setBorder(new EmptyBorder(20, 20, 20, 20));

        GridBagConstraints gbc = new GridBagConstraints();
        gbc.insets = new Insets(5, 5, 5, 5);
        gbc.fill = GridBagConstraints.HORIZONTAL;

        // Title
        JLabel titleLabel = new JLabel("User Login");
        titleLabel.setName("loginTitle");
        titleLabel.setFont(new Font("Arial", Font.BOLD, 18));
        gbc.gridx = 0;
        gbc.gridy = 0;
        gbc.gridwidth = 2;
        gbc.anchor = GridBagConstraints.CENTER;
        panel.add(titleLabel, gbc);

        // Username
        JLabel userLabel = new JLabel("Username:");
        userLabel.setName("usernameLabel");
        gbc.gridx = 0;
        gbc.gridy = 1;
        gbc.gridwidth = 1;
        gbc.anchor = GridBagConstraints.EAST;
        panel.add(userLabel, gbc);

        usernameField = new JTextField(20);
        usernameField.setName("username");
        usernameField.setToolTipText("Enter your username");
        gbc.gridx = 1;
        gbc.anchor = GridBagConstraints.WEST;
        panel.add(usernameField, gbc);

        // Password
        JLabel passLabel = new JLabel("Password:");
        passLabel.setName("passwordLabel");
        gbc.gridx = 0;
        gbc.gridy = 2;
        gbc.anchor = GridBagConstraints.EAST;
        panel.add(passLabel, gbc);

        passwordField = new JPasswordField(20);
        passwordField.setName("password");
        passwordField.setToolTipText("Enter your password");
        gbc.gridx = 1;
        gbc.anchor = GridBagConstraints.WEST;
        panel.add(passwordField, gbc);

        // Remember me
        rememberCheckbox = new JCheckBox("Remember me");
        rememberCheckbox.setName("rememberMe");
        gbc.gridx = 1;
        gbc.gridy = 3;
        panel.add(rememberCheckbox, gbc);

        // Buttons
        JPanel buttonPanel = new JPanel(new FlowLayout(FlowLayout.CENTER, 10, 0));

        loginButton = new JButton("Login");
        loginButton.setName("loginBtn");
        loginButton.setToolTipText("Click to login");
        loginButton.addActionListener(e -> handleLogin());

        clearButton = new JButton("Clear");
        clearButton.setName("clearBtn");
        clearButton.addActionListener(e -> {
            usernameField.setText("");
            passwordField.setText("");
            rememberCheckbox.setSelected(false);
            statusLabel.setText("Form cleared");
        });

        buttonPanel.add(loginButton);
        buttonPanel.add(clearButton);

        gbc.gridx = 0;
        gbc.gridy = 4;
        gbc.gridwidth = 2;
        gbc.anchor = GridBagConstraints.CENTER;
        panel.add(buttonPanel, gbc);

        // Result label
        JLabel resultLabel = new JLabel(" ");
        resultLabel.setName("loginResult");
        resultLabel.setForeground(Color.BLUE);
        gbc.gridy = 5;
        panel.add(resultLabel, gbc);

        return panel;
    }

    private void handleLogin() {
        String username = usernameField.getText();
        String password = new String(passwordField.getPassword());

        if (username.isEmpty() || password.isEmpty()) {
            statusLabel.setText("Please enter username and password");
            JOptionPane.showMessageDialog(this,
                "Username and password are required!",
                "Validation Error",
                JOptionPane.ERROR_MESSAGE);
            return;
        }

        if ("admin".equals(username) && "password123".equals(password)) {
            statusLabel.setText("Login successful!");
            JOptionPane.showMessageDialog(this,
                "Welcome, " + username + "!",
                "Login Success",
                JOptionPane.INFORMATION_MESSAGE);
        } else {
            statusLabel.setText("Login failed!");
            JOptionPane.showMessageDialog(this,
                "Invalid username or password!",
                "Login Failed",
                JOptionPane.ERROR_MESSAGE);
        }
    }

    private JPanel createTablePanel() {
        JPanel panel = new JPanel(new BorderLayout(10, 10));
        panel.setName("tablePanel");
        panel.setBorder(new EmptyBorder(10, 10, 10, 10));

        // Create table model
        String[] columns = {"ID", "Name", "Email", "Role", "Status"};
        Object[][] data = {
            {1, "John Doe", "john@example.com", "Admin", "Active"},
            {2, "Jane Smith", "jane@example.com", "User", "Active"},
            {3, "Bob Wilson", "bob@example.com", "User", "Inactive"},
            {4, "Alice Brown", "alice@example.com", "Manager", "Active"},
            {5, "Charlie Davis", "charlie@example.com", "User", "Active"},
        };

        DefaultTableModel model = new DefaultTableModel(data, columns) {
            @Override
            public boolean isCellEditable(int row, int column) {
                return column > 0; // ID not editable
            }
        };

        dataTable = new JTable(model);
        dataTable.setName("dataTable");
        dataTable.setSelectionMode(ListSelectionModel.SINGLE_SELECTION);
        dataTable.setRowHeight(25);
        dataTable.getTableHeader().setReorderingAllowed(false);

        // Selection listener
        dataTable.getSelectionModel().addListSelectionListener(e -> {
            if (!e.getValueIsAdjusting()) {
                int row = dataTable.getSelectedRow();
                if (row >= 0) {
                    statusLabel.setText("Selected: " + dataTable.getValueAt(row, 1));
                }
            }
        });

        JScrollPane scrollPane = new JScrollPane(dataTable);
        scrollPane.setPreferredSize(new Dimension(500, 200));
        panel.add(scrollPane, BorderLayout.CENTER);

        // Button panel
        JPanel buttonPanel = new JPanel(new FlowLayout(FlowLayout.LEFT));

        JButton addButton = new JButton("Add Row");
        addButton.setName("addRowBtn");
        addButton.addActionListener(e -> {
            int newId = model.getRowCount() + 1;
            model.addRow(new Object[]{newId, "New User", "new@example.com", "User", "Active"});
            statusLabel.setText("Row added");
        });

        JButton deleteButton = new JButton("Delete Row");
        deleteButton.setName("deleteRowBtn");
        deleteButton.addActionListener(e -> {
            int row = dataTable.getSelectedRow();
            if (row >= 0) {
                model.removeRow(row);
                statusLabel.setText("Row deleted");
            }
        });

        JButton refreshButton = new JButton("Refresh");
        refreshButton.setName("refreshBtn");
        refreshButton.addActionListener(e -> {
            dataTable.repaint();
            statusLabel.setText("Table refreshed");
        });

        buttonPanel.add(addButton);
        buttonPanel.add(deleteButton);
        buttonPanel.add(refreshButton);

        panel.add(buttonPanel, BorderLayout.SOUTH);

        return panel;
    }

    private JPanel createTreePanel() {
        JPanel panel = new JPanel(new BorderLayout(10, 10));
        panel.setName("treePanel");
        panel.setBorder(new EmptyBorder(10, 10, 10, 10));

        // Create tree model
        DefaultMutableTreeNode root = new DefaultMutableTreeNode("Root");

        DefaultMutableTreeNode documents = new DefaultMutableTreeNode("Documents");
        documents.add(new DefaultMutableTreeNode("Reports"));
        documents.add(new DefaultMutableTreeNode("Presentations"));
        documents.add(new DefaultMutableTreeNode("Spreadsheets"));

        DefaultMutableTreeNode images = new DefaultMutableTreeNode("Images");
        images.add(new DefaultMutableTreeNode("Photos"));
        images.add(new DefaultMutableTreeNode("Screenshots"));
        images.add(new DefaultMutableTreeNode("Icons"));

        DefaultMutableTreeNode music = new DefaultMutableTreeNode("Music");
        music.add(new DefaultMutableTreeNode("Rock"));
        music.add(new DefaultMutableTreeNode("Jazz"));
        music.add(new DefaultMutableTreeNode("Classical"));

        DefaultMutableTreeNode videos = new DefaultMutableTreeNode("Videos");
        videos.add(new DefaultMutableTreeNode("Movies"));
        videos.add(new DefaultMutableTreeNode("Tutorials"));

        root.add(documents);
        root.add(images);
        root.add(music);
        root.add(videos);

        fileTree = new JTree(root);
        fileTree.setName("fileTree");
        fileTree.setRootVisible(true);
        fileTree.setShowsRootHandles(true);

        // Selection listener
        fileTree.addTreeSelectionListener(e -> {
            DefaultMutableTreeNode node = (DefaultMutableTreeNode) fileTree.getLastSelectedPathComponent();
            if (node != null) {
                statusLabel.setText("Selected: " + node.getUserObject());
            }
        });

        JScrollPane scrollPane = new JScrollPane(fileTree);
        scrollPane.setPreferredSize(new Dimension(300, 300));
        panel.add(scrollPane, BorderLayout.CENTER);

        // Button panel
        JPanel buttonPanel = new JPanel(new FlowLayout(FlowLayout.LEFT));

        JButton expandAllBtn = new JButton("Expand All");
        expandAllBtn.setName("expandAllBtn");
        expandAllBtn.addActionListener(e -> expandAllNodes(fileTree, 0, fileTree.getRowCount()));

        JButton collapseAllBtn = new JButton("Collapse All");
        collapseAllBtn.setName("collapseAllBtn");
        collapseAllBtn.addActionListener(e -> {
            for (int i = fileTree.getRowCount() - 1; i >= 1; i--) {
                fileTree.collapseRow(i);
            }
        });

        buttonPanel.add(expandAllBtn);
        buttonPanel.add(collapseAllBtn);

        panel.add(buttonPanel, BorderLayout.SOUTH);

        return panel;
    }

    private void expandAllNodes(JTree tree, int startRow, int rowCount) {
        for (int i = startRow; i < rowCount; i++) {
            tree.expandRow(i);
        }
        if (tree.getRowCount() != rowCount) {
            expandAllNodes(tree, rowCount, tree.getRowCount());
        }
    }

    private JPanel createFormPanel() {
        JPanel panel = new JPanel(new GridBagLayout());
        panel.setName("formPanel");
        panel.setBorder(new EmptyBorder(20, 20, 20, 20));

        GridBagConstraints gbc = new GridBagConstraints();
        gbc.insets = new Insets(8, 8, 8, 8);
        gbc.fill = GridBagConstraints.HORIZONTAL;
        gbc.anchor = GridBagConstraints.WEST;

        int row = 0;

        // Country combo box
        gbc.gridx = 0;
        gbc.gridy = row;
        panel.add(new JLabel("Country:"), gbc);

        String[] countries = {"United States", "Canada", "United Kingdom", "Germany", "France", "Japan", "Australia"};
        countryCombo = new JComboBox<>(countries);
        countryCombo.setName("countryCombo");
        countryCombo.addActionListener(e -> statusLabel.setText("Country: " + countryCombo.getSelectedItem()));
        gbc.gridx = 1;
        panel.add(countryCombo, gbc);

        row++;

        // Radio buttons
        gbc.gridx = 0;
        gbc.gridy = row;
        panel.add(new JLabel("Option:"), gbc);

        JPanel radioPanel = new JPanel(new FlowLayout(FlowLayout.LEFT, 10, 0));
        ButtonGroup optionGroup = new ButtonGroup();

        optionA = new JRadioButton("Option A");
        optionA.setName("optionA");
        optionA.setSelected(true);
        optionB = new JRadioButton("Option B");
        optionB.setName("optionB");

        optionGroup.add(optionA);
        optionGroup.add(optionB);
        radioPanel.add(optionA);
        radioPanel.add(optionB);

        gbc.gridx = 1;
        panel.add(radioPanel, gbc);

        row++;

        // Volume slider
        gbc.gridx = 0;
        gbc.gridy = row;
        panel.add(new JLabel("Volume:"), gbc);

        volumeSlider = new JSlider(0, 100, 50);
        volumeSlider.setName("volumeSlider");
        volumeSlider.setMajorTickSpacing(25);
        volumeSlider.setMinorTickSpacing(5);
        volumeSlider.setPaintTicks(true);
        volumeSlider.setPaintLabels(true);
        volumeSlider.addChangeListener(e -> statusLabel.setText("Volume: " + volumeSlider.getValue()));
        gbc.gridx = 1;
        panel.add(volumeSlider, gbc);

        row++;

        // Quantity spinner
        gbc.gridx = 0;
        gbc.gridy = row;
        panel.add(new JLabel("Quantity:"), gbc);

        SpinnerNumberModel spinnerModel = new SpinnerNumberModel(1, 1, 100, 1);
        quantitySpinner = new JSpinner(spinnerModel);
        quantitySpinner.setName("quantitySpinner");
        quantitySpinner.addChangeListener(e -> statusLabel.setText("Quantity: " + quantitySpinner.getValue()));
        gbc.gridx = 1;
        panel.add(quantitySpinner, gbc);

        row++;

        // Progress bar
        gbc.gridx = 0;
        gbc.gridy = row;
        panel.add(new JLabel("Progress:"), gbc);

        progressBar = new JProgressBar(0, 100);
        progressBar.setName("progressBar");
        progressBar.setValue(0);
        progressBar.setStringPainted(true);
        gbc.gridx = 1;
        panel.add(progressBar, gbc);

        row++;

        // Progress buttons
        gbc.gridx = 1;
        gbc.gridy = row;
        JPanel progressButtons = new JPanel(new FlowLayout(FlowLayout.LEFT, 5, 0));

        JButton startProgressBtn = new JButton("Start");
        startProgressBtn.setName("startProgressBtn");
        startProgressBtn.addActionListener(e -> startProgress());

        JButton resetProgressBtn = new JButton("Reset");
        resetProgressBtn.setName("resetProgressBtn");
        resetProgressBtn.addActionListener(e -> progressBar.setValue(0));

        progressButtons.add(startProgressBtn);
        progressButtons.add(resetProgressBtn);
        panel.add(progressButtons, gbc);

        row++;

        // Notes text area
        gbc.gridx = 0;
        gbc.gridy = row;
        gbc.anchor = GridBagConstraints.NORTHWEST;
        panel.add(new JLabel("Notes:"), gbc);

        notesArea = new JTextArea(4, 30);
        notesArea.setName("notesArea");
        notesArea.setLineWrap(true);
        notesArea.setWrapStyleWord(true);
        JScrollPane notesScroll = new JScrollPane(notesArea);
        gbc.gridx = 1;
        gbc.anchor = GridBagConstraints.WEST;
        panel.add(notesScroll, gbc);

        return panel;
    }

    private void startProgress() {
        progressBar.setValue(0);
        Timer timer = new Timer(50, null);
        timer.addActionListener(e -> {
            int value = progressBar.getValue();
            if (value < 100) {
                progressBar.setValue(value + 1);
            } else {
                ((Timer) e.getSource()).stop();
                statusLabel.setText("Progress complete!");
            }
        });
        timer.start();
    }

    private JPanel createListPanel() {
        JPanel panel = new JPanel(new BorderLayout(10, 10));
        panel.setName("listPanel");
        panel.setBorder(new EmptyBorder(10, 10, 10, 10));

        // Create list model
        DefaultListModel<String> listModel = new DefaultListModel<>();
        listModel.addElement("Apple");
        listModel.addElement("Banana");
        listModel.addElement("Cherry");
        listModel.addElement("Date");
        listModel.addElement("Elderberry");
        listModel.addElement("Fig");
        listModel.addElement("Grape");
        listModel.addElement("Honeydew");

        itemList = new JList<>(listModel);
        itemList.setName("itemList");
        itemList.setSelectionMode(ListSelectionModel.MULTIPLE_INTERVAL_SELECTION);
        itemList.setVisibleRowCount(8);

        itemList.addListSelectionListener(e -> {
            if (!e.getValueIsAdjusting()) {
                List<String> selected = itemList.getSelectedValuesList();
                statusLabel.setText("Selected: " + String.join(", ", selected));
            }
        });

        JScrollPane scrollPane = new JScrollPane(itemList);
        scrollPane.setPreferredSize(new Dimension(200, 200));

        // Input panel
        JPanel inputPanel = new JPanel(new FlowLayout(FlowLayout.LEFT));
        JTextField newItemField = new JTextField(15);
        newItemField.setName("newItemField");

        JButton addItemBtn = new JButton("Add Item");
        addItemBtn.setName("addItemBtn");
        addItemBtn.addActionListener(e -> {
            String item = newItemField.getText().trim();
            if (!item.isEmpty()) {
                listModel.addElement(item);
                newItemField.setText("");
                statusLabel.setText("Added: " + item);
            }
        });

        JButton removeItemBtn = new JButton("Remove Selected");
        removeItemBtn.setName("removeItemBtn");
        removeItemBtn.addActionListener(e -> {
            int[] indices = itemList.getSelectedIndices();
            for (int i = indices.length - 1; i >= 0; i--) {
                listModel.remove(indices[i]);
            }
            statusLabel.setText("Items removed");
        });

        inputPanel.add(new JLabel("New Item:"));
        inputPanel.add(newItemField);
        inputPanel.add(addItemBtn);
        inputPanel.add(removeItemBtn);

        panel.add(scrollPane, BorderLayout.CENTER);
        panel.add(inputPanel, BorderLayout.SOUTH);

        return panel;
    }

    public static void main(String[] args) {
        // Set look and feel
        try {
            UIManager.setLookAndFeel(UIManager.getSystemLookAndFeelClassName());
        } catch (Exception e) {
            e.printStackTrace();
        }

        // Create and show GUI
        SwingUtilities.invokeLater(() -> {
            SwingDemoApp app = new SwingDemoApp();
            app.setVisible(true);
        });
    }
}
