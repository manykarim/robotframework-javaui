*** Settings ***
Documentation     Java Web Start (JNLP) attach proof — OPT-IN, self-skipping.
...
...               This suite serves the JNLP harness in tests/apps/jnlp/ over 127.0.0.1,
...               launches it with `Launch Web Start Application`, and asserts an HONEST
...               outcome: EITHER the library connected and can drive the live app, OR the
...               launch failed with the *clear, expected* SecurityManager AttachError.
...
...               Why "either/or": under IcedTea-Web, a JNLP app installs a JNLPSecurityManager
...               that STRUCTURALLY blocks runtime dynamic attach — it cannot classify the
...               attach-loaded agent's foreign code. This was verified even for a *signed
...               all-permissions* JNLP: still blocked. It is independent of the app's
...               permission level. So a fully-green attach REQUIRES a launcher/JDK WITHOUT the
...               legacy SecurityManager: modern OpenWebStart, or JDK 24+ (where the
...               SecurityManager is gone). Point JAVAGUI_JAVAWS at such an image to get a green
...               connect; on a legacy IcedTea-Web `javaws`, the suite instead proves that the
...               failure is the documented SecurityManager block (not some other breakage).
...
...               The suite SELF-SKIPS unless a launcher is available (JAVAGUI_JAVAWS env or
...               `javaws` on PATH) and a DISPLAY is present (run under xvfb). It also skips if
...               the Swing test app jar has not been built. This keeps default CI green when no
...               Web Start launcher is installed. See tests/apps/jnlp/README.md.

Library           JavaGui.SwingLibrary
Library           Process
Library           OperatingSystem
Library           Collections

Suite Setup       Start Web Start Harness
Suite Teardown    Stop Web Start Harness

Force Tags        jnlp    webstart    opt-in

*** Variables ***
${HOST}             127.0.0.1
${SERVE_PORT}       8099
${JNLP_FILE}        app.jnlp
${JNLP_URL}         http://${HOST}:${SERVE_PORT}/${JNLP_FILE}
# Harness sources (paths are relative to THIS suite file, so run location doesn't matter).
${SWING_JAR}        ${CURDIR}${/}..${/}..${/}apps${/}swing${/}target${/}swing-test-app-1.0.0.jar
${JNLP_SRC}         ${CURDIR}${/}..${/}..${/}apps${/}jnlp${/}${JNLP_FILE}
# Timing.
${SETTLE}           8
${LAUNCH_TIMEOUT}   60

*** Test Cases ***
Web Start Launch Connects Or Reports The SecurityManager Block
    [Documentation]    Launch the JNLP harness and require an honest result:
    ...                a live connection, or the known SecurityManager AttachError.
    ...                Never asserts success unconditionally.
    # No port= => the library picks a free agent RPC port (must differ from the http port).
    ${status}    ${error}=    Run Keyword And Ignore Error
    ...    Launch Web Start Application    ${JNLP_URL}
    ...        host=${HOST}    toolkit=auto
    ...        settle=${SETTLE}    timeout=${LAUNCH_TIMEOUT}
    IF    '${status}' == 'PASS'
        Web Start Attach Drove The Live App
    ELSE
        Failure Is The Documented SecurityManager Block    ${error}
    END

*** Keywords ***
Start Web Start Harness
    [Documentation]    Self-skip guard + stage a serve dir + start a local http.server on
    ...                127.0.0.1. Skips (does not fail) when the environment can't run the test.
    ${launcher}=    Evaluate
    ...    os.environ.get('JAVAGUI_JAVAWS') or __import__('shutil').which('javaws')    modules=os
    Skip If    not $launcher
    ...    No Web Start launcher: set JAVAGUI_JAVAWS to a launcher/IcedTea-Web image, or put 'javaws' on PATH (opt-in suite).
    ${display}=    Get Environment Variable    DISPLAY    ${EMPTY}
    Skip If    '${display}' == '${EMPTY}'    No DISPLAY — run under xvfb (opt-in suite).
    ${has_jar}=    Run Keyword And Return Status    File Should Exist    ${SWING_JAR}
    Skip If    not ${has_jar}
    ...    Swing test app not built: run 'mvn -f tests/apps/swing/pom.xml package' (opt-in suite).

    # Stage app.jnlp + the swing jar into one served directory.
    ${serve_dir}=    Set Variable    ${OUTPUT DIR}${/}jnlp-serve
    Create Directory    ${serve_dir}
    Copy File    ${JNLP_SRC}    ${serve_dir}${/}${JNLP_FILE}
    Copy File    ${SWING_JAR}    ${serve_dir}${/}swing-test-app-1.0.0.jar
    Set Suite Variable    ${SERVE_DIR}    ${serve_dir}

    # Serve it on 127.0.0.1 only (never 0.0.0.0).
    Start Process    python3    -m    http.server    ${SERVE_PORT}    --bind    ${HOST}
    ...    cwd=${serve_dir}    alias=httpd
    ...    stdout=${OUTPUT DIR}${/}httpd-stdout.log    stderr=${OUTPUT DIR}${/}httpd-stderr.log
    Wait Until Keyword Succeeds    15x    0.5s    Http Server Is Up

Http Server Is Up
    ${sock}=    Evaluate    __import__('socket').create_connection(('${HOST}', ${SERVE_PORT}), 1)
    Call Method    ${sock}    close

Web Start Attach Drove The Live App
    [Documentation]    A no-SecurityManager launcher connected — prove it's really driving the app.
    ${connected}=    Is Connected
    Should Be True    ${connected}    Launch reported success but the library is not connected.
    ${buttons}=    Find Elements    JButton
    Should Be True    ${{len($buttons)}} >= 1
    ...    Connected, but no JButton was locatable — the app tree did not load.
    Log    Web Start attach succeeded: connected and located ${{len($buttons)}} JButton(s) live.

Failure Is The Documented SecurityManager Block
    [Documentation]    On a legacy IcedTea-Web launcher, the ONLY acceptable failure is the
    ...                clear SecurityManager AttachError. Any other error is a real defect.
    [Arguments]    ${error}
    Should Contain    ${error}    SecurityManager
    ...    msg=Web Start attach failed for a reason OTHER than the documented SecurityManager block. A green attach needs a no-SecurityManager launcher (modern OpenWebStart / JDK 24+). Actual error: ${error}
    Log    Confirmed the documented IcedTea-Web SecurityManager block (attach structurally denied): ${error}

Stop Web Start Harness
    [Documentation]    Best-effort teardown: disconnect, terminate the Web Start launcher/app
    ...                JVM, then stop the http server. Uses a pattern unique to the JNLP so it
    ...                never matches this suite's own process or the http.server.
    Run Keyword And Ignore Error    Disconnect
    # 'app.jnlp' appears in the launcher's (and forked app JVM's) command line, but not in the
    # python http.server command line nor in this suite's command line.
    Run Keyword And Ignore Error    Run Process    pkill    -f    ${JNLP_FILE}
    Run Keyword And Ignore Error    Terminate Process    httpd
