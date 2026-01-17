//! Backend trait and technology-specific communication abstraction
//!
//! The `Backend` trait defines the interface for toolkit-specific communication
//! with Java applications. Each toolkit (Swing, SWT, RCP) implements this trait
//! to handle its specific protocol requirements.

use serde_json::Value;
use std::fmt;
use std::time::Duration;
use thiserror::Error;

/// Result type for backend operations
pub type BackendResult<T> = Result<T, BackendError>;

/// Errors that can occur in backend operations
#[derive(Error, Debug)]
pub enum BackendError {
    /// Connection-related errors
    #[error("Connection error: {message}")]
    Connection { message: String },

    /// Connection not established
    #[error("Not connected to any application")]
    NotConnected,

    /// Connection timeout
    #[error("Connection timeout after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },

    /// RPC protocol error
    #[error("RPC error (code {code}): {message}")]
    Rpc { code: i32, message: String },

    /// Protocol error
    #[error("Protocol error: {message}")]
    Protocol { message: String },

    /// Serialization error
    #[error("Serialization error: {message}")]
    Serialization { message: String },

    /// Element not found
    #[error("Element not found: {locator}")]
    ElementNotFound { locator: String },

    /// Multiple elements found when expecting one
    #[error("Multiple elements found for locator '{locator}' (found {count})")]
    MultipleElements { locator: String, count: usize },

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Internal error
    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl BackendError {
    /// Create a connection error
    pub fn connection<S: Into<String>>(message: S) -> Self {
        Self::Connection { message: message.into() }
    }

    /// Create a protocol error
    pub fn protocol<S: Into<String>>(message: S) -> Self {
        Self::Protocol { message: message.into() }
    }

    /// Create an internal error
    pub fn internal<S: Into<String>>(message: S) -> Self {
        Self::Internal { message: message.into() }
    }

    /// Check if this error is recoverable (can be retried)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            BackendError::Timeout { .. }
                | BackendError::ElementNotFound { .. }
        )
    }

    /// Check if this error indicates a connection problem
    pub fn is_connection_error(&self) -> bool {
        matches!(
            self,
            BackendError::Connection { .. }
                | BackendError::NotConnected
                | BackendError::Timeout { .. }
        )
    }
}

/// Supported toolkit types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolkitType {
    /// Java Swing (javax.swing)
    Swing,
    /// Eclipse Standard Widget Toolkit (org.eclipse.swt)
    Swt,
    /// Eclipse Rich Client Platform (org.eclipse.ui)
    Rcp,
}

impl ToolkitType {
    /// Get the default port for this toolkit
    pub fn default_port(&self) -> u16 {
        match self {
            ToolkitType::Swing => 5678,
            ToolkitType::Swt | ToolkitType::Rcp => 5679,
        }
    }

    /// Get the human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            ToolkitType::Swing => "swing",
            ToolkitType::Swt => "swt",
            ToolkitType::Rcp => "rcp",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "swing" => Some(ToolkitType::Swing),
            "swt" => Some(ToolkitType::Swt),
            "rcp" => Some(ToolkitType::Rcp),
            _ => None,
        }
    }
}

impl fmt::Display for ToolkitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Element conditions for wait operations
#[derive(Debug, Clone)]
pub enum ElementCondition {
    /// Element exists in the UI tree
    Exists,
    /// Element does not exist in the UI tree
    NotExists,
    /// Element is visible
    Visible,
    /// Element is not visible (hidden)
    NotVisible,
    /// Element is enabled (can receive input)
    Enabled,
    /// Element is disabled
    Disabled,
    /// Element has specific text content
    HasText(String),
    /// Element text contains substring
    TextContains(String),
    /// Element is focused
    Focused,
    /// Custom condition with name for error messages
    Custom(String),
}

impl ElementCondition {
    /// Get a description of this condition for error messages
    pub fn description(&self) -> String {
        match self {
            ElementCondition::Exists => "exists".to_string(),
            ElementCondition::NotExists => "does not exist".to_string(),
            ElementCondition::Visible => "is visible".to_string(),
            ElementCondition::NotVisible => "is not visible".to_string(),
            ElementCondition::Enabled => "is enabled".to_string(),
            ElementCondition::Disabled => "is disabled".to_string(),
            ElementCondition::HasText(text) => format!("has text '{}'", text),
            ElementCondition::TextContains(text) => format!("text contains '{}'", text),
            ElementCondition::Focused => "is focused".to_string(),
            ElementCondition::Custom(name) => name.clone(),
        }
    }

    /// Create a custom condition with a name
    pub fn custom<S: Into<String>>(name: S) -> Self {
        ElementCondition::Custom(name.into())
    }
}

/// Backend trait - defines technology-specific communication
///
/// Each toolkit implements this trait to provide its specific protocol
/// for communicating with Java applications.
///
/// # Example Implementation
///
/// ```ignore
/// pub struct SwingBackend {
///     stream: Option<TcpStream>,
///     config: Option<ConnectionConfig>,
/// }
///
/// impl Backend for SwingBackend {
///     fn toolkit_type(&self) -> ToolkitType {
///         ToolkitType::Swing
///     }
///     // ... other methods
/// }
/// ```
pub trait Backend: Send + Sync {
    /// Get the toolkit type this backend supports
    fn toolkit_type(&self) -> ToolkitType;

    /// Connect to the application
    ///
    /// # Arguments
    /// * `host` - Host address (e.g., "localhost")
    /// * `port` - Port number
    /// * `timeout` - Connection timeout
    fn connect(&mut self, host: &str, port: u16, timeout: Duration) -> BackendResult<()>;

    /// Disconnect from the application
    fn disconnect(&mut self) -> BackendResult<()>;

    /// Check if connected to an application
    fn is_connected(&self) -> bool;

    /// Send an RPC request and receive a response
    ///
    /// # Arguments
    /// * `method` - RPC method name
    /// * `params` - Method parameters as JSON
    ///
    /// # Returns
    /// JSON response from the agent
    fn send_request(&mut self, method: &str, params: Value) -> BackendResult<Value>;

    /// Ping the agent to verify connection
    fn ping(&mut self) -> BackendResult<bool> {
        match self.send_request("ping", Value::Object(Default::default())) {
            Ok(response) => Ok(response.as_str() == Some("pong")),
            Err(_) => Ok(false),
        }
    }

    /// Get the default port for this backend
    fn default_port(&self) -> u16 {
        self.toolkit_type().default_port()
    }

    /// Get connection info for debugging
    fn connection_info(&self) -> Option<ConnectionInfo> {
        None
    }
}

/// Connection information for debugging
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub host: String,
    pub port: u16,
    pub connected: bool,
    pub toolkit: ToolkitType,
    pub application: Option<String>,
}

/// Factory for creating toolkit-specific backends
pub struct BackendFactory;

impl BackendFactory {
    /// Create a backend for the specified toolkit type
    ///
    /// # Arguments
    /// * `toolkit` - The toolkit type to create a backend for
    ///
    /// # Returns
    /// A boxed Backend implementation
    pub fn create(toolkit: ToolkitType) -> Box<dyn Backend> {
        match toolkit {
            ToolkitType::Swing => Box::new(GenericBackend::new(ToolkitType::Swing)),
            ToolkitType::Swt => Box::new(GenericBackend::new(ToolkitType::Swt)),
            ToolkitType::Rcp => Box::new(GenericBackend::new(ToolkitType::Rcp)),
        }
    }
}

/// Generic backend implementation using JSON-RPC over TCP
///
/// This provides a base implementation that all toolkits can use.
/// Toolkit-specific backends can wrap this or implement Backend directly.
pub struct GenericBackend {
    toolkit: ToolkitType,
    stream: Option<std::net::TcpStream>,
    host: Option<String>,
    port: Option<u16>,
    request_id: u64,
}

impl GenericBackend {
    /// Create a new generic backend for the specified toolkit
    pub fn new(toolkit: ToolkitType) -> Self {
        Self {
            toolkit,
            stream: None,
            host: None,
            port: None,
            request_id: 0,
        }
    }

    /// Get the next request ID
    fn next_request_id(&mut self) -> u64 {
        self.request_id += 1;
        self.request_id
    }
}

impl Backend for GenericBackend {
    fn toolkit_type(&self) -> ToolkitType {
        self.toolkit
    }

    fn connect(&mut self, host: &str, port: u16, timeout: Duration) -> BackendResult<()> {
        use std::net::{TcpStream, ToSocketAddrs};

        let addr = format!("{}:{}", host, port);
        let socket_addr = addr
            .to_socket_addrs()
            .map_err(|e| BackendError::connection(format!("Failed to resolve '{}': {}", addr, e)))?
            .next()
            .ok_or_else(|| BackendError::connection(format!("No addresses found for '{}'", addr)))?;

        let stream = TcpStream::connect_timeout(&socket_addr, timeout)
            .map_err(|e| BackendError::connection(format!("Failed to connect to {}: {}", addr, e)))?;

        stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
        stream.set_write_timeout(Some(Duration::from_secs(30))).ok();

        self.stream = Some(stream);
        self.host = Some(host.to_string());
        self.port = Some(port);
        self.request_id = 0;

        Ok(())
    }

    fn disconnect(&mut self) -> BackendResult<()> {
        self.stream = None;
        self.host = None;
        self.port = None;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    fn send_request(&mut self, method: &str, params: Value) -> BackendResult<Value> {
        use std::io::{BufRead, BufReader, Write};

        // Get next request ID before borrowing stream
        self.request_id += 1;
        let id = self.request_id;

        let stream = self.stream.as_mut().ok_or(BackendError::NotConnected)?;

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id
        });

        let request_str = serde_json::to_string(&request)
            .map_err(|e| BackendError::Serialization { message: e.to_string() })?;

        writeln!(stream, "{}", request_str)
            .map_err(|e| BackendError::protocol(format!("Failed to send request: {}", e)))?;
        stream.flush()
            .map_err(|e| BackendError::protocol(format!("Failed to flush: {}", e)))?;

        // Clone the stream for reading (TcpStream implements Clone via try_clone)
        let read_stream = stream.try_clone()
            .map_err(|e| BackendError::protocol(format!("Failed to clone stream: {}", e)))?;
        let mut reader = BufReader::new(read_stream);
        let mut response_str = String::new();
        reader.read_line(&mut response_str)
            .map_err(|e| BackendError::protocol(format!("Failed to read response: {}", e)))?;

        let response: Value = serde_json::from_str(&response_str)
            .map_err(|e| BackendError::Serialization { message: format!("Failed to parse response: {}", e) })?;

        // Check for error
        if let Some(error) = response.get("error") {
            let code = error.get("code").and_then(|c| c.as_i64()).unwrap_or(-1) as i32;
            let message = error.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error").to_string();
            return Err(BackendError::Rpc { code, message });
        }

        // Return result
        Ok(response.get("result").cloned().unwrap_or(Value::Null))
    }

    fn connection_info(&self) -> Option<ConnectionInfo> {
        if self.is_connected() {
            Some(ConnectionInfo {
                host: self.host.clone().unwrap_or_default(),
                port: self.port.unwrap_or(0),
                connected: true,
                toolkit: self.toolkit,
                application: None,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolkit_type_default_ports() {
        assert_eq!(ToolkitType::Swing.default_port(), 5678);
        assert_eq!(ToolkitType::Swt.default_port(), 5679);
        assert_eq!(ToolkitType::Rcp.default_port(), 5679);
    }

    #[test]
    fn test_toolkit_type_names() {
        assert_eq!(ToolkitType::Swing.name(), "swing");
        assert_eq!(ToolkitType::Swt.name(), "swt");
        assert_eq!(ToolkitType::Rcp.name(), "rcp");
    }

    #[test]
    fn test_toolkit_type_from_str() {
        assert_eq!(ToolkitType::from_str("swing"), Some(ToolkitType::Swing));
        assert_eq!(ToolkitType::from_str("SWT"), Some(ToolkitType::Swt));
        assert_eq!(ToolkitType::from_str("RCP"), Some(ToolkitType::Rcp));
        assert_eq!(ToolkitType::from_str("unknown"), None);
    }

    #[test]
    fn test_backend_error_is_recoverable() {
        assert!(BackendError::Timeout { timeout_ms: 1000 }.is_recoverable());
        assert!(BackendError::ElementNotFound { locator: "test".to_string() }.is_recoverable());
        assert!(!BackendError::NotConnected.is_recoverable());
    }

    #[test]
    fn test_backend_error_is_connection_error() {
        assert!(BackendError::NotConnected.is_connection_error());
        assert!(BackendError::Timeout { timeout_ms: 1000 }.is_connection_error());
        assert!(!BackendError::ElementNotFound { locator: "test".to_string() }.is_connection_error());
    }

    #[test]
    fn test_element_condition_descriptions() {
        assert_eq!(ElementCondition::Exists.description(), "exists");
        assert_eq!(ElementCondition::HasText("hello".to_string()).description(), "has text 'hello'");
    }

    #[test]
    fn test_generic_backend_creation() {
        let backend = GenericBackend::new(ToolkitType::Swing);
        assert_eq!(backend.toolkit_type(), ToolkitType::Swing);
        assert!(!backend.is_connected());
    }

    #[test]
    fn test_backend_factory() {
        let backend = BackendFactory::create(ToolkitType::Swt);
        assert_eq!(backend.toolkit_type(), ToolkitType::Swt);
    }
}
