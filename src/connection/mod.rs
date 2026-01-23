//! Connection management for Java Swing applications
//!
//! This module handles:
//! - JVM discovery (by PID, window title, or main class)
//! - Java agent injection via Attach API
//! - Communication channel management

use crate::error::{SwingError, SwingResult};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Connection to a Java Swing application
#[derive(Debug)]
pub struct SwingConnection {
    /// Target JVM process ID
    pub pid: u32,
    /// Connection stream
    stream: Arc<Mutex<TcpStream>>,
    /// Port the agent is listening on
    pub port: u16,
    /// Whether connected
    connected: bool,
    /// Request ID counter
    request_id: u64,
}

/// JVM process information
#[derive(Debug, Clone)]
pub struct JvmInfo {
    /// Process ID
    pub pid: u32,
    /// Main class name
    pub main_class: String,
    /// Command line arguments
    pub args: Vec<String>,
    /// Window titles (if any)
    pub window_titles: Vec<String>,
}

impl SwingConnection {
    /// Connect to a running JVM by PID
    pub fn connect_by_pid(pid: u32, timeout: Duration) -> SwingResult<Self> {
        // Extract agent JAR
        let agent_path = Self::extract_agent()?;

        // Inject agent into target JVM
        let port = Self::inject_agent(pid, &agent_path)?;

        // Connect to agent
        Self::connect_to_agent(pid, port, timeout)
    }

    /// Connect to a JVM by window title pattern
    pub fn connect_by_title(title_pattern: &str, timeout: Duration) -> SwingResult<Self> {
        let jvms = Self::list_jvms()?;

        for jvm in jvms {
            for title in &jvm.window_titles {
                if Self::matches_pattern(title, title_pattern) {
                    return Self::connect_by_pid(jvm.pid, timeout);
                }
            }
        }

        Err(SwingError::JvmNotFound {
            identifier: title_pattern.to_string(),
        })
    }

    /// Connect to a JVM by main class name
    pub fn connect_by_main_class(class_name: &str, timeout: Duration) -> SwingResult<Self> {
        let jvms = Self::list_jvms()?;

        for jvm in jvms {
            if jvm.main_class.contains(class_name) || jvm.main_class.ends_with(class_name) {
                return Self::connect_by_pid(jvm.pid, timeout);
            }
        }

        Err(SwingError::JvmNotFound {
            identifier: class_name.to_string(),
        })
    }

    /// List all running JVMs
    pub fn list_jvms() -> SwingResult<Vec<JvmInfo>> {
        let mut jvms = Vec::new();

        // Use jps to list Java processes
        let output = Command::new("jps")
            .arg("-l")
            .output()
            .map_err(|e| SwingError::Internal {
                message: format!("Failed to run jps: {}", e),
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() >= 1 {
                if let Ok(pid) = parts[0].parse::<u32>() {
                    let main_class = parts.get(1).unwrap_or(&"").to_string();
                    jvms.push(JvmInfo {
                        pid,
                        main_class,
                        args: Vec::new(),
                        window_titles: Vec::new(),
                    });
                }
            }
        }

        Ok(jvms)
    }

    /// Extract embedded agent JAR to temp directory
    fn extract_agent() -> SwingResult<PathBuf> {
        // In a real implementation, this would extract the embedded JAR
        // For now, return a placeholder path
        let cache_dir = std::env::temp_dir().join("robotframework-swing");
        std::fs::create_dir_all(&cache_dir).map_err(|e| SwingError::Internal {
            message: format!("Failed to create cache dir: {}", e),
        })?;

        let agent_path = cache_dir.join("swing-agent.jar");

        // TODO: Write embedded agent bytes to file
        // const AGENT_BYTES: &[u8] = include_bytes!("../../agent/target/swing-agent.jar");
        // std::fs::write(&agent_path, AGENT_BYTES)?;

        Ok(agent_path)
    }

    /// Inject agent into target JVM
    fn inject_agent(pid: u32, agent_path: &PathBuf) -> SwingResult<u16> {
        // Find available port
        let port = Self::find_available_port()?;

        // Use jattach or native attach API
        let output = Command::new("jattach")
            .arg(pid.to_string())
            .arg("load")
            .arg("instrument")
            .arg("false")
            .arg(format!("{}={}", agent_path.display(), port))
            .output()
            .or_else(|_| {
                // Fallback: try using Java attach API via a helper class
                Command::new("java")
                    .arg("-cp")
                    .arg(agent_path)
                    .arg("com.robotframework.swing.AttachHelper")
                    .arg(pid.to_string())
                    .arg(port.to_string())
                    .output()
            })
            .map_err(|e| SwingError::AgentInjectionFailed {
                reason: format!("Failed to attach: {}", e),
            })?;

        if !output.status.success() {
            return Err(SwingError::AgentInjectionFailed {
                reason: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        Ok(port)
    }

    /// Find an available port
    fn find_available_port() -> SwingResult<u16> {
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| SwingError::Internal {
            message: format!("Failed to find available port: {}", e),
        })?;

        let port = listener.local_addr().map_err(|e| SwingError::Internal {
            message: format!("Failed to get port: {}", e),
        })?.port();

        Ok(port)
    }

    /// Connect to the injected agent
    fn connect_to_agent(pid: u32, port: u16, timeout: Duration) -> SwingResult<Self> {
        let addr = format!("127.0.0.1:{}", port);

        let stream = TcpStream::connect_timeout(
            &addr.parse().unwrap(),
            timeout,
        ).map_err(|_e| SwingError::ConnectionTimeout {
            timeout_ms: timeout.as_millis() as u64,
        })?;

        stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
        stream.set_write_timeout(Some(Duration::from_secs(30))).ok();

        Ok(Self {
            pid,
            stream: Arc::new(Mutex::new(stream)),
            port,
            connected: true,
            request_id: 0,
        })
    }

    /// Check if pattern matches (supports * wildcards)
    fn matches_pattern(text: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        if pattern.starts_with('*') && pattern.ends_with('*') {
            let inner = &pattern[1..pattern.len()-1];
            return text.contains(inner);
        }

        if pattern.starts_with('*') {
            let suffix = &pattern[1..];
            return text.ends_with(suffix);
        }

        if pattern.ends_with('*') {
            let prefix = &pattern[..pattern.len()-1];
            return text.starts_with(prefix);
        }

        text == pattern
    }

    /// Send a JSON-RPC request and get response
    pub fn send_request(&mut self, method: &str, params: serde_json::Value) -> SwingResult<serde_json::Value> {
        if !self.connected {
            return Err(SwingError::NotConnected);
        }

        self.request_id += 1;

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": self.request_id
        });

        let request_str = serde_json::to_string(&request).map_err(|e| SwingError::SerializationError {
            message: e.to_string(),
        })?;

        let mut stream = self.stream.lock().map_err(|_| SwingError::Internal {
            message: "Failed to lock stream".to_string(),
        })?;

        // Send request
        writeln!(stream, "{}", request_str).map_err(|e| SwingError::ProtocolError {
            message: format!("Failed to send request: {}", e),
        })?;
        stream.flush().map_err(|e| SwingError::ProtocolError {
            message: format!("Failed to flush: {}", e),
        })?;

        // Read response
        let mut reader = BufReader::new(&*stream);
        let mut response_str = String::new();
        reader.read_line(&mut response_str).map_err(|e| SwingError::ProtocolError {
            message: format!("Failed to read response: {}", e),
        })?;

        let response: serde_json::Value = serde_json::from_str(&response_str).map_err(|e| SwingError::SerializationError {
            message: format!("Failed to parse response: {}", e),
        })?;

        // Check for error
        if let Some(error) = response.get("error") {
            let code = error.get("code").and_then(|c| c.as_i64()).unwrap_or(-1) as i32;
            let message = error.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error").to_string();
            return Err(SwingError::RpcError { code, message });
        }

        // Return result
        Ok(response.get("result").cloned().unwrap_or(serde_json::Value::Null))
    }

    /// Disconnect from the application
    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    /// Check if still connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Ping the agent to check connectivity
    pub fn ping(&mut self) -> SwingResult<bool> {
        let result = self.send_request("ping", serde_json::json!({}))?;
        Ok(result.as_str() == Some("pong"))
    }
}

/// Connection manager for multiple application connections
#[derive(Default)]
pub struct ConnectionManager {
    connections: HashMap<u32, SwingConnection>,
    active_pid: Option<u32>,
}

impl ConnectionManager {
    /// Create a new connection manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the active connection
    pub fn active(&mut self) -> SwingResult<&mut SwingConnection> {
        let pid = self.active_pid.ok_or(SwingError::NotConnected)?;
        self.connections.get_mut(&pid).ok_or(SwingError::NotConnected)
    }

    /// Connect and set as active
    pub fn connect(&mut self, pid: u32, timeout: Duration) -> SwingResult<()> {
        let conn = SwingConnection::connect_by_pid(pid, timeout)?;
        self.connections.insert(pid, conn);
        self.active_pid = Some(pid);
        Ok(())
    }

    /// Disconnect the active connection
    pub fn disconnect(&mut self) -> SwingResult<()> {
        if let Some(pid) = self.active_pid.take() {
            if let Some(conn) = self.connections.get_mut(&pid) {
                conn.disconnect();
            }
            self.connections.remove(&pid);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_matching() {
        assert!(SwingConnection::matches_pattern("MyApp - Main Window", "*Main*"));
        assert!(SwingConnection::matches_pattern("MyApp", "MyApp"));
        assert!(SwingConnection::matches_pattern("MyApp v1.0", "MyApp*"));
        assert!(SwingConnection::matches_pattern("App - Main", "*Main"));
        assert!(!SwingConnection::matches_pattern("Other", "MyApp"));
    }
}
