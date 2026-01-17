# SWT Connection Timeout Root Cause Analysis

**Date**: 2026-01-17
**Issue**: SWT connection hangs indefinitely while Swing connection works
**Severity**: HIGH - Blocks entire SWT test suite

## Executive Summary

The SWT connection timeout is caused by a **fundamental protocol mismatch** between the SWT agent server and the Rust client library. The SWT agent uses a **one-shot connection model** (closes socket after each request) while the Rust client expects a **persistent connection model** (reuses the same TCP stream for multiple RPC calls). This mismatch causes the client to hang when attempting to verify the connection with a ping request.

## Problem Statement

### Observed Behavior
- ✅ **Swing**: Connection succeeds, all 320+ tests pass
- ❌ **SWT**: Connection hangs at first test without timeout
- **SWT Agent**: Launches successfully, listens on port 5679
- **Client**: Establishes TCP connection but hangs on ping verification

### Test That Fails
```robot
# tests/robot/swt/01_connection.robot:31
Connect To SWT Application Successfully
    Connect To SWT Application    ${SWT_APP_NAME}    ${SWT_HOST}    ${SWT_PORT}
    # ← HANGS HERE
```

## Root Cause Analysis

### Critical Difference: Connection Lifecycle

#### Swing Agent (WORKS) - Persistent Connection Model
```
Client → Server: TCP Connect
Client → Server: {"jsonrpc":"2.0","method":"ping","id":1}
Client ← Server: {"jsonrpc":"2.0","result":"pong","id":1}
[Connection stays open]
Client → Server: {"jsonrpc":"2.0","method":"findWidgets","id":2}
Client ← Server: {"jsonrpc":"2.0","result":[...],"id":2}
[Connection stays open for entire test session]
```

#### SWT Agent (HANGS) - One-Shot Connection Model
```
Client → Server: TCP Connect
Server: Accept connection in handleClient()
Server: Read one request
Server: Send one response
Server: Close socket in finally block  ← PROBLEM!
Client: Try to read response
Client: Socket already closed → EOF/HANG
```

### Evidence from SWT Server Code

**File**: `/mnt/c/workspace/robotframework-swing/agent/src/main/java/com/robotframework/swt/SwtReflectionRpcServer.java`

```java
// Lines 55-84
private void handleClient(Socket socket) {
    try (BufferedReader reader = new BufferedReader(new InputStreamReader(socket.getInputStream(), StandardCharsets.UTF_8));
         PrintWriter writer = new PrintWriter(new OutputStreamWriter(socket.getOutputStream(), StandardCharsets.UTF_8), true)) {

        String line;
        StringBuilder requestBuilder = new StringBuilder();

        // Read ONE request
        while ((line = reader.readLine()) != null) {
            if (line.isEmpty()) {
                break;  // Stop at empty line
            }
            requestBuilder.append(line);
        }

        String request = requestBuilder.toString().trim();
        if (!request.isEmpty()) {
            String response = processRequest(request);
            writer.println(response);  // Send ONE response
        }

    } catch (IOException e) {
        System.err.println("[SwtAgent] Client handling error: " + e.getMessage());
    } finally {
        try {
            socket.close();  // ← CLOSES AFTER SINGLE REQUEST/RESPONSE
        } catch (IOException e) {
            // Ignore
        }
    }
}
```

**Problem**: The `try-with-resources` statement and explicit `finally` block close the socket after handling just one request.

### Evidence from Rust Client Code

**File**: `/mnt/c/workspace/robotframework-swing/src/python/swt_library.rs`

```rust
// Lines 191-243: connect_to_swt_application
pub fn connect_to_swt_application(&mut self, ...) -> PyResult<()> {
    // Establish TCP connection
    let stream = TcpStream::connect_timeout(&socket_addr, timeout_duration)?;
    stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(30))).ok();

    conn.stream = Some(stream);  // ← STORE for persistent reuse
    conn.connected = true;
    // ...
    drop(conn);

    // Verify connection with ping - EXPECTS TO REUSE STREAM
    let result = self.send_rpc_request("ping", serde_json::json!({}))?;
    if result.as_str() != Some("pong") {
        return Err(SwingError::connection("SWT agent did not respond to ping").into());
    }
    // ← HANGS HERE because socket was closed by server
}
```

```rust
// Lines 1386-1512: send_rpc_request
pub fn send_rpc_request(&self, method: &str, params: serde_json::Value) -> PyResult<serde_json::Value> {
    let stream = conn.stream.as_mut().ok_or_else(|| ...)?;  // ← REUSES stored stream

    writeln!(stream, "{}", request_str)?;  // Send request
    stream.flush()?;

    // Read response byte-by-byte with JSON depth tracking
    loop {
        match stream.read(&mut byte_buf) {
            Ok(0) => {
                // EOF - Server closed the connection!
                return Err(SwingError::connection("Connection closed by server").into());
            }
            // ...
        }
    }
}
```

**Problem**: Client expects to reuse the stored TCP stream for the ping request, but the server has already closed the socket.

## Secondary Issues Discovered

### Issue 2: Request Format Mismatch

**SWT Server Expects**:
```java
// Lines 62-69
while ((line = reader.readLine()) != null) {
    if (line.isEmpty()) {
        break;  // Expects empty line to signal end of request
    }
    requestBuilder.append(line);
}
```

**Rust Client Sends**:
```rust
writeln!(stream, "{}", request_str)?;  // Single-line JSON + newline
// NO empty line separator sent
```

The server waits for an empty line to know the request is complete, but the client only sends a newline after the JSON. This could cause the server to hang waiting for more input.

### Issue 3: Response Reading Strategy

**SWT Server Sends**:
```java
String response = processRequest(request);
writer.println(response);  // Single line response
```

**Rust Client Reads**:
```rust
// Byte-by-byte reading with JSON depth tracking
// Handles multi-line pretty-printed JSON
// Not optimized for single-line responses
```

The client's byte-by-byte reading is correct but may be slower than necessary for single-line responses.

## Why Swing Works

The Swing agent (not examined in detail) must implement a persistent connection model where:
1. Server accepts connection
2. Server enters a loop to handle multiple requests on the same socket
3. Socket remains open between requests
4. Socket only closes when client disconnects or error occurs

This matches the client's expectations perfectly.

## Solution Options

### Option 1: Fix SWT Agent (RECOMMENDED)

Refactor `SwtReflectionRpcServer` to use persistent connections like Swing:

```java
private void handleClient(Socket socket) {
    try {
        BufferedReader reader = new BufferedReader(new InputStreamReader(socket.getInputStream(), StandardCharsets.UTF_8));
        PrintWriter writer = new PrintWriter(new OutputStreamWriter(socket.getOutputStream(), StandardCharsets.UTF_8), true);

        // Keep connection open and handle multiple requests
        while (running.get() && !socket.isClosed()) {
            // Read request (line-delimited JSON)
            String line = reader.readLine();
            if (line == null || line.trim().isEmpty()) {
                break;  // Client disconnected or sent empty line
            }

            // Process and send response
            String response = processRequest(line);
            writer.println(response);
            writer.flush();
        }
    } catch (IOException e) {
        System.err.println("[SwtAgent] Client handling error: " + e.getMessage());
    } finally {
        try {
            socket.close();
        } catch (IOException e) {
            // Ignore
        }
    }
}
```

**Changes needed**:
1. Remove `try-with-resources` for readers/writers
2. Add loop to handle multiple requests
3. Read line-delimited JSON (one request per line)
4. Keep socket open until client disconnects or error
5. Handle empty line as request separator (optional)

### Option 2: Fix Rust Client for SWT Only

Create new TCP connection for each RPC request:

```rust
fn send_rpc_request(&self, method: &str, params: serde_json::Value) -> PyResult<serde_json::Value> {
    // Don't use stored stream - create new connection each time
    let conn = self.connection.read()?;
    let addr = format!("{}:{}", conn.host.as_ref().unwrap(), conn.port.unwrap());
    let mut stream = TcpStream::connect_timeout(&addr.to_socket_addrs()?.next().unwrap(), Duration::from_secs(30))?;

    // Send request
    writeln!(stream, "{}", request_str)?;
    stream.flush()?;

    // Read response
    // ... (same reading logic)

    // Connection automatically closed when stream drops
    Ok(result)
}
```

**Drawbacks**:
- Higher latency (new connection per request)
- More resource usage (TCP handshake overhead)
- Inconsistent with Swing implementation
- Not suitable for high-frequency operations

### Option 3: Protocol Negotiation

Implement protocol version detection to support both models:

**Not recommended** - adds complexity and doesn't solve the fundamental issue.

## Recommended Solution

**Fix the SWT agent to use persistent connections** (Option 1):

### Why:
1. **Consistency**: Matches Swing agent behavior
2. **Performance**: Better for rapid request sequences
3. **Simplicity**: No client-side changes needed
4. **Scalability**: Reduces connection overhead

### Implementation Steps:
1. Modify `handleClient()` to loop for multiple requests
2. Change request reading to handle line-delimited JSON
3. Remove automatic socket closure after one request
4. Add graceful disconnect detection (EOF on read)
5. Test with existing Rust client (should work immediately)

## Additional Fixes Required

### 1. Connection Timeout Handling

**Current State**: When server closes socket unexpectedly, client may hang instead of timing out cleanly.

**Fix**: Ensure timeout is properly enforced:
```rust
stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
```

This is already present but may need verification that it triggers on EOF.

### 2. Empty Line Protocol

**Decision needed**: Should requests be terminated with empty line?

- **Current Swing**: Single-line JSON (no empty line)
- **Current SWT**: Expects empty line separator
- **Recommendation**: Use newline-terminated single-line JSON (no empty line)

### 3. Error Messages

Improve error messages when connection fails:
```rust
Ok(0) => {
    return Err(SwingError::connection("Connection closed by server (possible protocol mismatch)").into());
}
```

## Testing Plan

After implementing the fix:

1. **Unit Test**: Create standalone test for SWT agent's multi-request handling
2. **Integration Test**: Run `tests/robot/swt/01_connection.robot`
3. **Full Suite**: Run entire SWT test suite
4. **Regression Test**: Verify Swing tests still pass (they should, no changes there)
5. **Performance Test**: Measure connection overhead vs one-shot model

## References

### Files Analyzed
- `/mnt/c/workspace/robotframework-swing/src/python/swt_library.rs` - SWT Rust client
- `/mnt/c/workspace/robotframework-swing/src/python/swing_library.rs` - Swing Rust client (comparison)
- `/mnt/c/workspace/robotframework-swing/agent/src/main/java/com/robotframework/swt/SwtReflectionRpcServer.java` - SWT Java server
- `/mnt/c/workspace/robotframework-swing/tests/robot/swt/01_connection.robot` - Failing test
- `/mnt/c/workspace/robotframework-swing/docs/TEST_EXECUTION_REPORT.md` - Issue report

### Related Issues
- **Issue 1**: Swing dialog test hangs (different issue, not connection-related)
- **Issue 3**: RCP mock app startup failure (different issue, application-level)

## Conclusion

The SWT connection timeout is caused by a **client-server protocol mismatch**:
- **Server**: One-shot connection (closes after one request)
- **Client**: Persistent connection (reuses stream for multiple requests)

The **recommended solution** is to refactor the SWT agent's `handleClient()` method to handle multiple requests on a single connection, matching the Swing agent's behavior and the client's expectations.

This is a **high-priority fix** as it blocks the entire SWT test suite (0 of 300+ tests can run).
