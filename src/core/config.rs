//! Configuration management for the unified JavaGui library
//!
//! Provides unified configuration handling for library settings,
//! connection parameters, and environment variable support.
//!
//! # Overview
//!
//! This module provides two main configuration structures:
//!
//! - [`LibraryConfig`] - Library-wide settings for timeouts, screenshots, logging, etc.
//! - [`ConnectionConfig`] - Connection-specific settings for connecting to Java applications
//!
//! Both support builder patterns for fluent construction and can be initialized from
//! environment variables for containerized deployments.
//!
//! # Example
//!
//! ```ignore
//! use crate::core::config::{LibraryConfig, ConnectionConfig, GuiMode};
//! use std::time::Duration;
//!
//! let lib_config = LibraryConfig::new()
//!     .with_timeout_secs(30.0)
//!     .with_mode(GuiMode::Swing)
//!     .with_screenshot_on_failure(true);
//!
//! let conn_config = ConnectionConfig::swing("myapp.jar")
//!     .with_host("localhost")
//!     .with_port(5678);
//! ```

use super::backend::ToolkitType;
use std::fmt;
use std::path::PathBuf;
use std::time::Duration;

/// GUI toolkit mode for automatic detection or explicit selection
///
/// This enum determines which GUI toolkit the library should target.
/// In `Auto` mode, the library will attempt to detect the toolkit
/// automatically based on the connected application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GuiMode {
    /// Java Swing applications (javax.swing)
    Swing,
    /// Eclipse SWT applications (org.eclipse.swt)
    Swt,
    /// Eclipse RCP applications (org.eclipse.ui)
    Rcp,
    /// Automatic detection based on application classes
    #[default]
    Auto,
}

impl GuiMode {
    /// Parse from string (case insensitive)
    ///
    /// # Arguments
    /// * `s` - String to parse ("swing", "swt", "rcp", or "auto")
    ///
    /// # Returns
    /// `Some(GuiMode)` if valid, `None` otherwise
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "swing" => Some(GuiMode::Swing),
            "swt" => Some(GuiMode::Swt),
            "rcp" => Some(GuiMode::Rcp),
            "auto" => Some(GuiMode::Auto),
            _ => None,
        }
    }

    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            GuiMode::Swing => "swing",
            GuiMode::Swt => "swt",
            GuiMode::Rcp => "rcp",
            GuiMode::Auto => "auto",
        }
    }

    /// Convert to toolkit type if explicitly set
    ///
    /// Returns `None` for `Auto` mode since the toolkit hasn't been determined yet.
    pub fn to_toolkit_type(&self) -> Option<ToolkitType> {
        match self {
            GuiMode::Swing => Some(ToolkitType::Swing),
            GuiMode::Swt => Some(ToolkitType::Swt),
            GuiMode::Rcp => Some(ToolkitType::Rcp),
            GuiMode::Auto => None,
        }
    }

    /// Create from toolkit type
    pub fn from_toolkit_type(toolkit: ToolkitType) -> Self {
        match toolkit {
            ToolkitType::Swing => GuiMode::Swing,
            ToolkitType::Swt => GuiMode::Swt,
            ToolkitType::Rcp => GuiMode::Rcp,
        }
    }
}

impl fmt::Display for GuiMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Log level configuration
///
/// Controls the verbosity of library logging output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogLevel {
    /// Detailed debug information for troubleshooting
    Debug,
    /// General information messages (default)
    #[default]
    Info,
    /// Warning messages for potential issues
    Warning,
    /// Error messages only
    Error,
}

impl LogLevel {
    /// Parse from string (case insensitive)
    ///
    /// # Arguments
    /// * `s` - String to parse ("debug", "info", "warning"/"warn", or "error")
    ///
    /// # Returns
    /// `Some(LogLevel)` if valid, `None` otherwise
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "debug" => Some(LogLevel::Debug),
            "info" => Some(LogLevel::Info),
            "warning" | "warn" => Some(LogLevel::Warning),
            "error" => Some(LogLevel::Error),
            _ => None,
        }
    }

    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warning => "warning",
            LogLevel::Error => "error",
        }
    }

    /// Check if this level should log at the given level
    ///
    /// Returns true if `other` level is at or above this level's threshold.
    pub fn should_log(&self, other: LogLevel) -> bool {
        let self_level = match self {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Warning => 2,
            LogLevel::Error => 3,
        };
        let other_level = match other {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Warning => 2,
            LogLevel::Error => 3,
        };
        other_level >= self_level
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Library configuration
///
/// Contains all settings for the JavaGui library that can be
/// configured via constructor arguments or environment variables.
///
/// # Builder Pattern
///
/// ```ignore
/// use crate::core::config::{LibraryConfig, GuiMode, LogLevel};
///
/// let config = LibraryConfig::new()
///     .with_timeout_secs(30.0)
///     .with_poll_interval_secs(0.5)
///     .with_mode(GuiMode::Swing)
///     .with_log_level(LogLevel::Debug)
///     .with_screenshot_on_failure(true);
/// ```
///
/// # Environment Variables
///
/// Configuration can also be loaded from environment variables using [`LibraryConfig::from_env()`]:
///
/// - `JAVAGUI_TIMEOUT` - Default timeout in seconds
/// - `JAVAGUI_POLL_INTERVAL` - Poll interval in milliseconds
/// - `JAVAGUI_MODE` - GUI mode (swing/swt/rcp/auto)
/// - `JAVAGUI_SCREENSHOT_DIR` - Screenshot directory
/// - `JAVAGUI_SCREENSHOT_ON_FAILURE` - Whether to screenshot on failure (true/false)
/// - `JAVAGUI_LOG_LEVEL` - Log level (debug/info/warning/error)
#[derive(Debug, Clone)]
pub struct LibraryConfig {
    /// Default timeout for wait operations
    pub timeout: Duration,
    /// Polling interval for wait operations
    pub poll_interval: Duration,
    /// Whether to take screenshots on failure
    pub screenshot_on_failure: bool,
    /// Directory for screenshots
    pub screenshot_directory: PathBuf,
    /// Screenshot format (png, jpg)
    pub screenshot_format: String,
    /// Log level
    pub log_level: LogLevel,
    /// GUI mode (Swing, SWT, RCP, or Auto-detect)
    pub mode: GuiMode,
    /// Whether to cache elements
    pub enable_element_cache: bool,
    /// Element cache TTL
    pub cache_ttl: Duration,
    /// Whether to log actions
    pub log_actions: bool,
    /// Maximum retry attempts for recoverable errors
    pub max_retries: u32,
    /// Delay between retries
    pub retry_delay: Duration,
}

impl Default for LibraryConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            poll_interval: Duration::from_millis(500),
            screenshot_on_failure: true,
            screenshot_directory: PathBuf::from("."),
            screenshot_format: "png".to_string(),
            log_level: LogLevel::Info,
            mode: GuiMode::Auto,
            enable_element_cache: true,
            cache_ttl: Duration::from_secs(5),
            log_actions: true,
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
        }
    }
}

impl LibraryConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create configuration from environment variables
    ///
    /// Supported environment variables:
    /// - `JAVAGUI_TIMEOUT`: Default timeout in seconds
    /// - `JAVAGUI_POLL_INTERVAL`: Poll interval in milliseconds
    /// - `JAVAGUI_MODE`: GUI mode (swing/swt/rcp/auto)
    /// - `JAVAGUI_SCREENSHOT_DIR`: Screenshot directory
    /// - `JAVAGUI_SCREENSHOT_ON_FAILURE`: Whether to screenshot on failure (true/false)
    /// - `JAVAGUI_SCREENSHOT_FORMAT`: Screenshot format (png/jpg)
    /// - `JAVAGUI_LOG_LEVEL`: Log level (debug/info/warning/error)
    /// - `JAVAGUI_CACHE_ENABLED`: Enable element cache (true/false)
    /// - `JAVAGUI_CACHE_TTL`: Cache TTL in seconds
    /// - `JAVAGUI_MAX_RETRIES`: Maximum retry attempts
    /// - `JAVAGUI_RETRY_DELAY`: Delay between retries in milliseconds
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(timeout) = std::env::var("JAVAGUI_TIMEOUT") {
            if let Ok(secs) = timeout.parse::<u64>() {
                config.timeout = Duration::from_secs(secs);
            }
        }

        if let Ok(poll) = std::env::var("JAVAGUI_POLL_INTERVAL") {
            if let Ok(ms) = poll.parse::<u64>() {
                config.poll_interval = Duration::from_millis(ms);
            }
        }

        if let Ok(mode) = std::env::var("JAVAGUI_MODE") {
            if let Some(gui_mode) = GuiMode::from_str(&mode) {
                config.mode = gui_mode;
            }
        }

        if let Ok(dir) = std::env::var("JAVAGUI_SCREENSHOT_DIR") {
            config.screenshot_directory = PathBuf::from(dir);
        }

        if let Ok(val) = std::env::var("JAVAGUI_SCREENSHOT_ON_FAILURE") {
            config.screenshot_on_failure = val.to_lowercase() == "true";
        }

        if let Ok(format) = std::env::var("JAVAGUI_SCREENSHOT_FORMAT") {
            if format == "png" || format == "jpg" || format == "jpeg" {
                config.screenshot_format = format;
            }
        }

        if let Ok(level) = std::env::var("JAVAGUI_LOG_LEVEL") {
            if let Some(log_level) = LogLevel::from_str(&level) {
                config.log_level = log_level;
            }
        }

        if let Ok(val) = std::env::var("JAVAGUI_CACHE_ENABLED") {
            config.enable_element_cache = val.to_lowercase() == "true";
        }

        if let Ok(ttl) = std::env::var("JAVAGUI_CACHE_TTL") {
            if let Ok(secs) = ttl.parse::<u64>() {
                config.cache_ttl = Duration::from_secs(secs);
            }
        }

        if let Ok(retries) = std::env::var("JAVAGUI_MAX_RETRIES") {
            if let Ok(n) = retries.parse::<u32>() {
                config.max_retries = n;
            }
        }

        if let Ok(delay) = std::env::var("JAVAGUI_RETRY_DELAY") {
            if let Ok(ms) = delay.parse::<u64>() {
                config.retry_delay = Duration::from_millis(ms);
            }
        }

        config
    }

    /// Builder method: set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Builder method: set timeout from seconds
    pub fn with_timeout_secs(mut self, secs: f64) -> Self {
        self.timeout = Duration::from_secs_f64(secs);
        self
    }

    /// Builder method: set poll interval
    pub fn with_poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Builder method: set poll interval from seconds
    pub fn with_poll_interval_secs(mut self, secs: f64) -> Self {
        self.poll_interval = Duration::from_secs_f64(secs);
        self
    }

    /// Builder method: set screenshot directory
    pub fn with_screenshot_directory<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.screenshot_directory = dir.into();
        self
    }

    /// Builder method: enable/disable screenshots on failure
    pub fn with_screenshot_on_failure(mut self, enabled: bool) -> Self {
        self.screenshot_on_failure = enabled;
        self
    }

    /// Builder method: set log level
    pub fn with_log_level(mut self, level: LogLevel) -> Self {
        self.log_level = level;
        self
    }

    /// Builder method: enable/disable element cache
    pub fn with_element_cache(mut self, enabled: bool) -> Self {
        self.enable_element_cache = enabled;
        self
    }

    /// Builder method: set cache TTL
    pub fn with_cache_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = ttl;
        self
    }

    /// Builder method: set GUI mode
    pub fn with_mode(mut self, mode: GuiMode) -> Self {
        self.mode = mode;
        self
    }

    /// Builder method: set maximum retries
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    /// Builder method: set retry delay
    pub fn with_retry_delay(mut self, delay: Duration) -> Self {
        self.retry_delay = delay;
        self
    }

    /// Builder method: enable/disable action logging
    pub fn with_log_actions(mut self, enabled: bool) -> Self {
        self.log_actions = enabled;
        self
    }

    /// Validate the configuration
    ///
    /// Returns a list of validation errors, or an empty vector if valid.
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.timeout.is_zero() {
            errors.push("Timeout must be greater than zero".to_string());
        }

        if self.poll_interval.is_zero() {
            errors.push("Poll interval must be greater than zero".to_string());
        }

        if self.poll_interval > self.timeout {
            errors.push("Poll interval should not be greater than timeout".to_string());
        }

        if !["png", "jpg", "jpeg"].contains(&self.screenshot_format.as_str()) {
            errors.push(format!(
                "Invalid screenshot format '{}'. Must be 'png', 'jpg', or 'jpeg'",
                self.screenshot_format
            ));
        }

        errors
    }
}

/// Connection configuration
///
/// Specifies how to connect to a Java application.
///
/// # Builder Pattern
///
/// ```ignore
/// use crate::core::config::ConnectionConfig;
///
/// let config = ConnectionConfig::swing("myapp.jar")
///     .with_host("localhost")
///     .with_port(5678)
///     .with_timeout_secs(30.0)
///     .with_retry_count(3);
/// ```
///
/// # Toolkit-Specific Constructors
///
/// - [`ConnectionConfig::swing()`] - For Swing applications (port 5678)
/// - [`ConnectionConfig::swt()`] - For SWT applications (port 5679)
/// - [`ConnectionConfig::rcp()`] - For RCP applications (port 5679)
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// Host address
    pub host: String,
    /// Port number
    pub port: u16,
    /// Connection timeout
    pub timeout: Duration,
    /// Number of connection retry attempts
    pub retry_count: u32,
    /// Application identifier (JAR path, class name, or PID)
    pub application: String,
    /// Target toolkit type
    pub toolkit: ToolkitType,
    /// Whether to auto-reconnect on connection loss
    pub auto_reconnect: bool,
    /// Delay between reconnection attempts
    pub reconnect_delay: Duration,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5678,
            timeout: Duration::from_secs(30),
            retry_count: 3,
            application: String::new(),
            toolkit: ToolkitType::Swing,
            auto_reconnect: true,
            reconnect_delay: Duration::from_secs(1),
        }
    }
}

impl ConnectionConfig {
    /// Create a new connection config with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Create configuration for Swing applications
    ///
    /// # Arguments
    /// * `app` - Application identifier (JAR path, class name, or PID)
    pub fn swing(app: &str) -> Self {
        Self {
            application: app.to_string(),
            port: ToolkitType::Swing.default_port(),
            toolkit: ToolkitType::Swing,
            ..Default::default()
        }
    }

    /// Create configuration for SWT applications
    ///
    /// # Arguments
    /// * `app` - Application identifier
    pub fn swt(app: &str) -> Self {
        Self {
            application: app.to_string(),
            port: ToolkitType::Swt.default_port(),
            toolkit: ToolkitType::Swt,
            ..Default::default()
        }
    }

    /// Create configuration for RCP applications
    ///
    /// # Arguments
    /// * `app` - Application identifier
    pub fn rcp(app: &str) -> Self {
        Self {
            application: app.to_string(),
            port: ToolkitType::Rcp.default_port(),
            toolkit: ToolkitType::Rcp,
            ..Default::default()
        }
    }

    /// Create from environment variables
    ///
    /// Supported environment variables:
    /// - `JAVAGUI_HOST`: Host address
    /// - `JAVAGUI_PORT`: Port number
    /// - `JAVAGUI_TIMEOUT`: Connection timeout in seconds
    /// - `JAVAGUI_TOOLKIT`: Toolkit type (swing/swt/rcp)
    /// - `JAVAGUI_RETRY_COUNT`: Number of connection retry attempts
    /// - `JAVAGUI_AUTO_RECONNECT`: Enable auto-reconnect (true/false)
    /// - `JAVAGUI_RECONNECT_DELAY`: Delay between reconnection attempts in milliseconds
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(host) = std::env::var("JAVAGUI_HOST") {
            config.host = host;
        }

        if let Ok(port) = std::env::var("JAVAGUI_PORT") {
            if let Ok(p) = port.parse::<u16>() {
                config.port = p;
            }
        }

        if let Ok(timeout) = std::env::var("JAVAGUI_TIMEOUT") {
            if let Ok(secs) = timeout.parse::<u64>() {
                config.timeout = Duration::from_secs(secs);
            }
        }

        if let Ok(toolkit) = std::env::var("JAVAGUI_TOOLKIT") {
            if let Some(t) = ToolkitType::from_str(&toolkit) {
                config.toolkit = t;
                config.port = t.default_port();
            }
        }

        if let Ok(retry) = std::env::var("JAVAGUI_RETRY_COUNT") {
            if let Ok(n) = retry.parse::<u32>() {
                config.retry_count = n;
            }
        }

        if let Ok(val) = std::env::var("JAVAGUI_AUTO_RECONNECT") {
            config.auto_reconnect = val.to_lowercase() == "true";
        }

        if let Ok(delay) = std::env::var("JAVAGUI_RECONNECT_DELAY") {
            if let Ok(ms) = delay.parse::<u64>() {
                config.reconnect_delay = Duration::from_millis(ms);
            }
        }

        config
    }

    /// Builder method: set host
    pub fn with_host<S: Into<String>>(mut self, host: S) -> Self {
        self.host = host.into();
        self
    }

    /// Builder method: set port
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Builder method: set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Builder method: set timeout from seconds
    pub fn with_timeout_secs(mut self, secs: f64) -> Self {
        self.timeout = Duration::from_secs_f64(secs);
        self
    }

    /// Builder method: set application identifier
    pub fn with_application<S: Into<String>>(mut self, app: S) -> Self {
        self.application = app.into();
        self
    }

    /// Builder method: set toolkit type
    pub fn with_toolkit(mut self, toolkit: ToolkitType) -> Self {
        self.toolkit = toolkit;
        // Update port to toolkit default if not explicitly set
        if self.port == ToolkitType::Swing.default_port() || self.port == ToolkitType::Swt.default_port() {
            self.port = toolkit.default_port();
        }
        self
    }

    /// Builder method: set retry count
    pub fn with_retry_count(mut self, count: u32) -> Self {
        self.retry_count = count;
        self
    }

    /// Builder method: enable/disable auto-reconnect
    pub fn with_auto_reconnect(mut self, enabled: bool) -> Self {
        self.auto_reconnect = enabled;
        self
    }

    /// Builder method: set reconnect delay
    pub fn with_reconnect_delay(mut self, delay: Duration) -> Self {
        self.reconnect_delay = delay;
        self
    }

    /// Get socket address string
    pub fn socket_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Validate the configuration
    ///
    /// Returns a list of validation errors, or an empty vector if valid.
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.host.is_empty() {
            errors.push("Host cannot be empty".to_string());
        }

        if self.port == 0 {
            errors.push("Port must be greater than zero".to_string());
        }

        if self.timeout.is_zero() {
            errors.push("Timeout must be greater than zero".to_string());
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_config_defaults() {
        let config = LibraryConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(10));
        assert_eq!(config.poll_interval, Duration::from_millis(500));
        assert!(config.screenshot_on_failure);
        assert!(config.enable_element_cache);
        assert_eq!(config.mode, GuiMode::Auto);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_library_config_builder() {
        let config = LibraryConfig::new()
            .with_timeout_secs(30.0)
            .with_poll_interval_secs(1.0)
            .with_screenshot_on_failure(false)
            .with_log_level(LogLevel::Debug)
            .with_mode(GuiMode::Swing)
            .with_max_retries(5);

        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.poll_interval, Duration::from_secs(1));
        assert!(!config.screenshot_on_failure);
        assert_eq!(config.log_level, LogLevel::Debug);
        assert_eq!(config.mode, GuiMode::Swing);
        assert_eq!(config.max_retries, 5);
    }

    #[test]
    fn test_library_config_validation() {
        let valid_config = LibraryConfig::default();
        assert!(valid_config.validate().is_empty());

        let invalid_config = LibraryConfig::new()
            .with_timeout(Duration::ZERO);
        let errors = invalid_config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("Timeout")));
    }

    #[test]
    fn test_connection_config_swing() {
        let config = ConnectionConfig::swing("myapp.jar");
        assert_eq!(config.application, "myapp.jar");
        assert_eq!(config.port, 5678);
        assert_eq!(config.toolkit, ToolkitType::Swing);
        assert_eq!(config.retry_count, 3);
    }

    #[test]
    fn test_connection_config_swt() {
        let config = ConnectionConfig::swt("com.example.App");
        assert_eq!(config.application, "com.example.App");
        assert_eq!(config.port, 5679);
        assert_eq!(config.toolkit, ToolkitType::Swt);
    }

    #[test]
    fn test_connection_config_rcp() {
        let config = ConnectionConfig::rcp("eclipse");
        assert_eq!(config.application, "eclipse");
        assert_eq!(config.port, 5679);
        assert_eq!(config.toolkit, ToolkitType::Rcp);
    }

    #[test]
    fn test_connection_config_builder() {
        let config = ConnectionConfig::new()
            .with_host("192.168.1.100")
            .with_port(9999)
            .with_timeout_secs(60.0)
            .with_application("test")
            .with_toolkit(ToolkitType::Swt)
            .with_retry_count(5)
            .with_auto_reconnect(false);

        assert_eq!(config.host, "192.168.1.100");
        assert_eq!(config.port, 9999);
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.application, "test");
        assert_eq!(config.toolkit, ToolkitType::Swt);
        assert_eq!(config.retry_count, 5);
        assert!(!config.auto_reconnect);
    }

    #[test]
    fn test_connection_config_socket_addr() {
        let config = ConnectionConfig::new()
            .with_host("localhost")
            .with_port(5678);
        assert_eq!(config.socket_addr(), "localhost:5678");
    }

    #[test]
    fn test_connection_config_validation() {
        let valid_config = ConnectionConfig::default();
        assert!(valid_config.validate().is_empty());

        let invalid_config = ConnectionConfig::new()
            .with_host("");
        let errors = invalid_config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("Host")));
    }

    #[test]
    fn test_log_level_from_str() {
        assert_eq!(LogLevel::from_str("debug"), Some(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("INFO"), Some(LogLevel::Info));
        assert_eq!(LogLevel::from_str("WARNING"), Some(LogLevel::Warning));
        assert_eq!(LogLevel::from_str("warn"), Some(LogLevel::Warning));
        assert_eq!(LogLevel::from_str("error"), Some(LogLevel::Error));
        assert_eq!(LogLevel::from_str("unknown"), None);
    }

    #[test]
    fn test_log_level_as_str() {
        assert_eq!(LogLevel::Debug.as_str(), "debug");
        assert_eq!(LogLevel::Info.as_str(), "info");
        assert_eq!(LogLevel::Warning.as_str(), "warning");
        assert_eq!(LogLevel::Error.as_str(), "error");
    }

    #[test]
    fn test_log_level_should_log() {
        assert!(LogLevel::Debug.should_log(LogLevel::Debug));
        assert!(LogLevel::Debug.should_log(LogLevel::Info));
        assert!(LogLevel::Debug.should_log(LogLevel::Error));
        assert!(!LogLevel::Error.should_log(LogLevel::Debug));
        assert!(!LogLevel::Error.should_log(LogLevel::Warning));
        assert!(LogLevel::Error.should_log(LogLevel::Error));
    }

    #[test]
    fn test_gui_mode_from_str() {
        assert_eq!(GuiMode::from_str("swing"), Some(GuiMode::Swing));
        assert_eq!(GuiMode::from_str("SWT"), Some(GuiMode::Swt));
        assert_eq!(GuiMode::from_str("RCP"), Some(GuiMode::Rcp));
        assert_eq!(GuiMode::from_str("auto"), Some(GuiMode::Auto));
        assert_eq!(GuiMode::from_str("unknown"), None);
    }

    #[test]
    fn test_gui_mode_to_toolkit_type() {
        assert_eq!(GuiMode::Swing.to_toolkit_type(), Some(ToolkitType::Swing));
        assert_eq!(GuiMode::Swt.to_toolkit_type(), Some(ToolkitType::Swt));
        assert_eq!(GuiMode::Rcp.to_toolkit_type(), Some(ToolkitType::Rcp));
        assert_eq!(GuiMode::Auto.to_toolkit_type(), None);
    }

    #[test]
    fn test_gui_mode_display() {
        assert_eq!(format!("{}", GuiMode::Swing), "swing");
        assert_eq!(format!("{}", GuiMode::Auto), "auto");
    }
}
