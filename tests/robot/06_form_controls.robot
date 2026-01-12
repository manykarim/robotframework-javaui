*** Settings ***
Documentation     Test form controls - combo boxes, radio buttons, etc.
Resource          resources/common.resource
Suite Setup       Start Demo Application
Suite Teardown    Stop Demo Application

*** Test Cases ***
Find ComboBox By Name
    [Documentation]    Find combo box using name selector
    Element Should Exist    JComboBox[name='countryCombo']

Find Radio Buttons
    [Documentation]    Find radio buttons using name selector
    Element Should Exist    JRadioButton[name='optionA']
    Element Should Exist    JRadioButton[name='optionB']

Find Slider By Name
    [Documentation]    Find slider component
    Element Should Exist    JSlider[name='volumeSlider']

Find Spinner By Name
    [Documentation]    Find spinner component
    Element Should Exist    JSpinner[name='quantitySpinner']

Find Text Area By Name
    [Documentation]    Find text area component
    Element Should Exist    JTextArea[name='notesArea']

Find Progress Bar By Name
    [Documentation]    Find progress bar component
    Element Should Exist    JProgressBar[name='progressBar']
