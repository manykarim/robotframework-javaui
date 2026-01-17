# Robot Framework Java GUI Library - Troubleshooting Guide

**Last Updated**: 2026-01-17
**Version**: 1.0.0

## Table of Contents
1. [Connection Issues](#connection-issues)
2. [Element Finding Issues](#element-finding-issues)
3. [Test Execution Issues](#test-execution-issues)
4. [Environment Issues](#environment-issues)
5. [Common Error Messages](#common-error-messages)

---

## Connection Issues

### Issue: "Connection refused" Error

**Symptoms**:
```
SwingConnectionError: Failed to connect to localhost:5678: Connection refused (os error 111)
```

**Common Causes**:

1. **Application not running**
   ```bash
   # Verify application is running
   ps aux | grep java
   ```

2. **Wrong port number**
   ```robot
   # Check port matches application
   Connect To Application    MyApp    localhost    5678
   ```

3. **Agent not loaded**
   ```bash
   # Correct way to start with agent:
   java -javaagent:agent/target/robotframework-swing-agent-1.0.0-all.jar=port=5678 -jar myapp.jar

   # Common mistake (wrong agent name):
   java -javaagent:robotframework-swt-agent-1.0.0-all.jar=port=5678 -jar myapp.jar
   ```

**Solutions**:

✅ **Verify agent JAR name**:
```bash
# Check what agent JAR actually exists:
ls -l agent/target/*.jar

# Should see:
# robotframework-swing-agent-1.0.0-all.jar  (CORRECT)
# NOT: robotframework-swt-agent-1.0.0-all.jar (WRONG)
```

✅ **Check application output** for agent initialization:
```
[UnifiedAgent] Initializing with host=127.0.0.1, port=5678, toolkit=auto
[UnifiedAgent] Detected Swing via javax.swing.SwingUtilities
[UnifiedAgent] Using toolkit: swing
[UnifiedAgent] RPC server started on 127.0.0.1:5678
```

✅ **Verify port is not in use**:
```bash
# Linux/macOS:
netstat -tuln | grep 5678

# If port in use, choose different port
```

---

### Issue: "Connection timeout" Error

**Symptoms**:
```
SwingConnectionError: Connection timeout after 30000ms
```

**Common Causes**:

1. **Application takes too long to start**
2. **Firewall blocking connection**
3. **Application crashes during startup**

**Solutions**:

✅ **Increase connection timeout**:
```robot
# Default timeout is 30 seconds
Connect To Application    MyApp    localhost    5678    timeout=60

# Or set library timeout
Library    SwingLibrary    timeout=60
```

✅ **Check application logs**:
```bash
# Look for crashes or exceptions during startup
tail -f /tmp/app.log
```

✅ **Test connection manually**:
```bash
# Try telnet to verify port is accessible
telnet localhost 5678

# Or use nc (netcat)
nc -zv localhost 5678
```

---

### Issue: "Not connected to any application" Error

**Symptoms**:
```
NotConnectedError: Not connected to any application. Use 'Connect To Application' first.
```

**Cause**: Attempting to interact with application before establishing connection.

**Solution**:

✅ **Always connect first**:
```robot
*** Test Cases ***
My Test
    Connect To Application    MyApp    localhost    5678
    Click Button    name:submitButton    # Now this will work
    [Teardown]    Disconnect From Application
```

✅ **Verify connection status**:
```robot
*** Test Cases ***
Check Connection
    ${connected}=    Is Connected
    Should Be True    ${connected}    Not connected!
```

---

## Element Finding Issues

### Issue: "No element found matching the given locator"

**Symptoms**:
```
ElementNotFoundError: No element found matching locator 'name:submitButton' after 10.0s
```

**Common Causes**:

1. **Incorrect locator syntax**
2. **Element not visible/loaded yet**
3. **Typo in element name**
4. **Wrong locator strategy**

**Solutions**:

✅ **Inspect UI tree**:
```robot
*** Test Cases ***
Debug Element Finding
    Connect To Application    MyApp    localhost    5678
    Log UI Tree    # Prints entire component hierarchy
    Log UI Tree    name:mainPanel    # Print specific subtree
```

✅ **Try different locator strategies**:
```robot
# By name (preferred)
Click Button    name:submitButton

# By text
Click Button    text:Submit

# By type
Click Button    type:JButton

# By index
Click Button    index:0

# By CSS-like selector
Click Button    button[name='submitButton']
```

✅ **Add wait time**:
```robot
# Wait for element to appear
Wait Until Element Exists    name:submitButton    timeout=30

# Then interact with it
Click Button    name:submitButton
```

✅ **Check similar elements** (new feature):
```
ElementNotFoundError: No element found matching locator 'name:submitButon'

Similar elements found:
  • name:submitButton (similarity: 0.95)
  • name:cancelButton (similarity: 0.65)

Suggestion: Did you mean 'name:submitButton'?
```

---

### Issue: "Multiple elements found when only one was expected"

**Symptoms**:
```
MultipleElementsFoundError: Found 3 elements matching 'type:JButton', expected 1
```

**Cause**: Locator is too generic.

**Solutions**:

✅ **Make locator more specific**:
```robot
# Too generic (finds all buttons):
Click Button    type:JButton

# More specific (finds button by name):
Click Button    name:submitButton

# Very specific (combined attributes):
Click Button    button[name='submit'][enabled='true']
```

✅ **Use index if intentional**:
```robot
# Click first button
Click Button    type:JButton    index=0

# Click second button
Click Button    type:JButton    index=1
```

✅ **Use parent-child relationship**:
```robot
# Find button within specific panel
Click Button    panel[name='loginPanel'] > button[name='submit']
```

---

### Issue: "Element not interactable"

**Symptoms**:
```
ElementNotInteractableError: Element is not interactable (disabled or not visible)
```

**Common Causes**:

1. **Element is disabled**
2. **Element is not visible**
3. **Element is obscured by another component**

**Solutions**:

✅ **Check element state**:
```robot
*** Test Cases ***
Verify Element State
    ${enabled}=    Element Should Be Enabled    name:submitButton
    ${visible}=    Element Should Be Visible    name:submitButton
```

✅ **Wait for element to be enabled**:
```robot
Wait Until Element Is Enabled    name:submitButton    timeout=10
Click Button    name:submitButton
```

✅ **Force interaction** (use with caution):
```robot
# This may bypass UI validation - use only for testing
${element}=    Get Element    name:submitButton
${element}.force_click()    # Direct method call
```

---

## Test Execution Issues

### Issue: Tests hang indefinitely

**Symptoms**:
- Test execution stops responding
- No output for extended period (>5 minutes)
- CPU usage remains high

**Common Causes**:

1. **Modal dialog waiting for input**
2. **Infinite loop in application**
3. **Deadlock in test or application**

**Solutions**:

✅ **Add global timeout**:
```robot
*** Settings ***
Library    SwingLibrary    timeout=30    # 30 second default
Test Timeout    5 minutes    # Max time per test
```

✅ **Use test timeouts**:
```robot
*** Test Cases ***
My Test
    [Timeout]    2 minutes
    # Test content
```

✅ **Handle modal dialogs**:
```robot
*** Test Cases ***
Test With Dialog
    Click Button    name:openDialog

    # Handle the dialog (don't let it block)
    ${dialog}=    Wait Until Dialog Exists    timeout=5
    Click Dialog Button    ${dialog}    OK
```

---

### Issue: Tests fail in headless environment

**Symptoms**:
```
Exception in thread "main" java.awt.HeadlessException
```

**Cause**: X11 display not available (Linux servers, CI/CD).

**Solutions**:

✅ **Use xvfb-run**:
```bash
# Run tests with virtual X server
xvfb-run -a uv run robot tests/

# Or with explicit display
xvfb-run -a -s "-screen 0 1920x1080x24" uv run robot tests/
```

✅ **Set DISPLAY environment variable**:
```bash
export DISPLAY=:99
Xvfb :99 -screen 0 1920x1080x24 &
uv run robot tests/
```

✅ **CI/CD Configuration** (GitHub Actions example):
```yaml
- name: Run Robot Framework Tests
  run: |
    xvfb-run -a uv run robot --outputdir output tests/
```

---

## Environment Issues

### Issue: Agent JAR not found

**Symptoms**:
```
Error: Unable to access jarfile agent/target/robotframework-swing-agent-1.0.0-all.jar
```

**Solutions**:

✅ **Build the agent**:
```bash
cd agent
mvn clean package
cd ..

# Verify JAR exists:
ls -lh agent/target/*-all.jar
```

✅ **Check Maven version**:
```bash
mvn --version
# Requires Maven 3.6+
```

---

### Issue: Python import errors

**Symptoms**:
```
ImportError: No module named 'swing_library'
ModuleNotFoundError: No module named '_swing_library'
```

**Solutions**:

✅ **Build Rust Python module**:
```bash
# Using maturin (recommended)
maturin develop --release

# Or using cargo
cargo build --release

# Verify import works
python3 -c "import swing_library; print('OK')"
```

✅ **Check Python version**:
```bash
python3 --version
# Requires Python 3.8+
```

✅ **Verify virtual environment**:
```bash
# Ensure virtual env is activated
which python3
# Should show venv path

# Check installed packages
pip list | grep swing
```

---

## Common Error Messages

### Error: "Toolkit detection failed"

**Full Error**:
```
TechnologyError: Unable to detect GUI toolkit (Swing/SWT/RCP)
```

**Meaning**: Agent couldn't determine if application uses Swing, SWT, or RCP.

**Solution**:
```bash
# Explicitly specify toolkit:
java -javaagent:agent.jar=port=5678,toolkit=swing -jar app.jar

# Available toolkits: swing, swt, rcp, auto (default)
```

---

### Error: "RCP Workbench not initialized"

**Full Error**:
```
RcpWorkbenchError: RCP Workbench not available or not initialized
```

**Meaning**: Attempting RCP-specific operations on non-RCP application.

**Solution**:
```robot
# Use RcpLibrary for RCP apps
Library    RcpLibrary

# Not SwingLibrary or SwtLibrary
```

---

### Error: "Invalid locator syntax"

**Full Error**:
```
InvalidLocatorSyntaxError: Invalid locator syntax 'name=submitButton'
```

**Meaning**: Wrong syntax in locator expression.

**Solution**:
```robot
# WRONG (using = instead of :)
Click Button    name=submitButton

# CORRECT (using :)
Click Button    name:submitButton

# Also correct (CSS-style)
Click Button    [name='submitButton']
```

---

## Getting Help

### Debug Information to Collect

When reporting issues, include:

1. **Library version**:
   ```bash
   cargo --version
   mvn --version
   python3 --version
   ```

2. **Test output**:
   ```bash
   robot --loglevel DEBUG tests/
   # Provides detailed logs
   ```

3. **UI tree dump**:
   ```robot
   Log UI Tree    # Include output in bug report
   ```

4. **Application startup logs**:
   ```bash
   java -javaagent:agent.jar=port=5678 -jar app.jar > app.log 2>&1
   # Include app.log in bug report
   ```

5. **Connection test**:
   ```bash
   telnet localhost 5678
   # Include connection result
   ```

### Where to Get Help

- **Documentation**: `/docs/user-guide/`
- **Examples**: `/tests/robot/` (working test examples)
- **Issues**: GitHub issue tracker
- **Logs**: Check `/tmp/*_test*.log` for recent execution logs

---

## Quick Reference

### Common Locator Patterns

```robot
# By name (most reliable)
name:componentName

# By text (visible text)
text:Button Label

# By type (class name)
type:JButton
type:javax.swing.JButton

# By index (position)
index:0

# By CSS-style selector
button[name='submit']
panel[name='main'] > button[enabled='true']

# By XPath-like
//button[@name='submit']
//panel[@name='main']//button
```

### Connection Patterns

```robot
*** Settings ***
Library    SwingLibrary    timeout=30    # For Swing apps
Library    SwtLibrary      timeout=30    # For SWT apps
Library    RcpLibrary      timeout=30    # For RCP apps

*** Test Cases ***
My Test
    # Connect
    Connect To Application    AppName    localhost    5678    timeout=60

    # Verify connection
    ${connected}=    Is Connected
    Should Be True    ${connected}

    # Test actions here

    # Cleanup
    [Teardown]    Disconnect From Application
```

### Timeout Patterns

```robot
*** Settings ***
Library    SwingLibrary    timeout=30    # Library-wide default

*** Test Cases ***
Test With Timeouts
    [Timeout]    5 minutes    # Test timeout

    # Keyword timeout
    Wait Until Element Exists    name:button    timeout=60

    # Connection timeout
    Connect To Application    App    localhost    5678    timeout=90
```

---

## Troubleshooting Checklist

Before asking for help, check:

- [ ] Application is running with agent JAR loaded
- [ ] Agent JAR name is correct (`robotframework-swing-agent-1.0.0-all.jar`)
- [ ] Port number matches between app and tests
- [ ] Display/X11 is available (or using xvfb-run)
- [ ] Python module is built and importable
- [ ] Locator syntax is correct (using `:` not `=`)
- [ ] Connection timeout is sufficient for app startup
- [ ] Element actually exists in UI (checked with Log UI Tree)
- [ ] No firewall blocking localhost connections
- [ ] Test timeout is sufficient for operation

If all items checked and issue persists, collect debug information and seek help.
