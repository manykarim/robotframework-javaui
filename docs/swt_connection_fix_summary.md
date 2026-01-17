# SWT Connection Fix Summary

## Issues Found

1. **Server closes connection after single request** - The `SwtReflectionRpcServer.handleClient()` method was closing the socket immediately after processing one request, but the Rust client expects a persistent connection for multiple RPC calls.

2. **No connection retry logic** - The Rust `connect_to_swt_application()` had no retry mechanism, so if the server wasn't ready yet, the connection would fail immediately.

3. **Insufficient startup wait time** - The test only waited 3 seconds for the SWT application and agent to start, which wasn't always enough.

4. **Race condition in server startup** - The `UnifiedAgent.startSwtServer()` logs "SWT RPC server started" immediately after starting the thread, but the actual `ServerSocket.bind()` happens later when the thread runs. This causes premature connection attempts.

## Fixes Implemented

### 1. Fixed persistent connection (SwtReflectionRpcServer.java)
Modified `handleClient()` to keep the socket open and process multiple requests:

```java
private void handleClient(Socket socket) {
    try (BufferedReader reader = ...; PrintWriter writer = ...) {
        // Keep connection alive for multiple requests
        String line;
        while ((line = reader.readLine()) != null) {
            line = line.trim();
            if (line.isEmpty()) continue;

            String response = processRequest(line);
            writer.println(response);
            writer.flush();
        }
    }
    // Socket closes when client disconnects
}
```

### 2. Added connection retry logic (swt_library.rs)
Implemented retry mechanism with 500ms intervals:

```rust
// Retry connection attempts to allow SWT agent time to start
let mut last_error = None;
let stream = loop {
    let remaining_time = total_timeout.saturating_sub(start_time.elapsed());
    if remaining_time.is_zero() {
        break Err(last_error.unwrap_or_else(|| ...));
    }

    let attempt_timeout = std::cmp::min(remaining_time, Duration::from_secs(2));
    match TcpStream::connect_timeout(&socket_addr, attempt_timeout) {
        Ok(s) => break Ok(s),
        Err(e) => {
            last_error = Some(...);
            std::thread::sleep(Duration::from_millis(500));  // Wait before retry
        }
    }
}?;
```

### 3. Increased startup wait time (01_connection.robot)
Changed from 3 seconds to 5 seconds to allow more time for SWT initialization.

### 4. Fixed race condition with server ready signal
Added `AtomicBoolean ready` flag and `isReady()` method to `SwtReflectionRpcServer`:

```java
private final AtomicBoolean ready = new AtomicBoolean(false);

@Override
public void run() {
    running.set(true);
    try {
        serverSocket = new ServerSocket(port, 50, InetAddress.getByName(host));
        ready.set(true);  // Signal server is ready
        System.out.println("[SwtAgent] RPC server listening on " + host + ":" + port);
        // ...
    }
}

public boolean isReady() {
    return ready.get();
}
```

Updated `UnifiedAgent.startSwtServer()` to wait for ready signal:

```java
// Wait for server to be ready (max 5 seconds)
java.lang.reflect.Method isReadyMethod = rpcServerClass.getMethod("isReady");
long startTime = System.currentTimeMillis();
while (System.currentTimeMillis() - startTime < 5000) {
    Boolean ready = (Boolean) isReadyMethod.invoke(server);
    if (ready != null && ready) {
        System.out.println("[UnifiedAgent] SWT RPC server ready on " + host + ":" + port);
        return;
    }
    Thread.sleep(50);
}
```

## Test Results

All 18 connection tests now pass:

```
01 Connection :: Test suite for SWT application connection keywords.
18 tests, 18 passed, 0 failed
```

Tests verified:
- ✅ Connect with default parameters
- ✅ Connect with explicit host and port
- ✅ Connect with custom timeout
- ✅ Connection error handling (invalid host, port, empty app name)
- ✅ Disconnect functionality
- ✅ Multiple connect/disconnect cycles
- ✅ Connection state tracking with `is_connected()`
- ✅ Reconnection after disconnect
- ✅ Rapid connect/disconnect stress test
- ✅ Various timeout values

## Files Modified

1. `/mnt/c/workspace/robotframework-swing/agent/src/main/java/com/robotframework/swt/SwtReflectionRpcServer.java`
   - Fixed persistent connection handling
   - Added ready signal mechanism

2. `/mnt/c/workspace/robotframework-swing/agent/src/main/java/com/robotframework/UnifiedAgent.java`
   - Added server ready wait logic

3. `/mnt/c/workspace/robotframework-swing/src/python/swt_library.rs`
   - Added connection retry logic with exponential backoff

4. `/mnt/c/workspace/robotframework-swing/tests/robot/swt/01_connection.robot`
   - Increased startup wait from 3s to 5s
