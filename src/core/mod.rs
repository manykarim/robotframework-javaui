//! Core abstractions for the unified JavaGui library
//!
//! This module provides shared infrastructure for Swing, SWT, and RCP automation:
//!
//! - `Backend` trait: Technology-specific communication abstraction
//! - `LibraryConfig`: Unified configuration management
//! - `JavaGuiElement`: Unified element representation
//! - `ToolkitType`: Enumeration of supported toolkits

pub mod backend;
pub mod config;
pub mod element;

// Re-export main types
pub use backend::{Backend, BackendError, BackendResult, ToolkitType, ElementCondition};
pub use config::{LibraryConfig, ConnectionConfig, LogLevel};
pub use element::{JavaGuiElement, ElementType};
