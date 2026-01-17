//! Python bindings module for Robot Framework Swing/SWT/RCP library
//!
//! This module provides PyO3-based Python bindings for the Swing, SWT, and RCP automation libraries,
//! enabling usage from Robot Framework and direct Python scripts.
//!
//! # Library Classes
//!
//! - `JavaGuiLibrary` - Unified library supporting all toolkits (recommended for new projects)
//! - `SwingLibrary` - Backwards-compatible wrapper for Swing automation
//! - `SwtLibrary` - Backwards-compatible wrapper for SWT automation
//! - `RcpLibrary` - Backwards-compatible wrapper for RCP automation
//!
//! # Migration
//!
//! For existing users of SwingLibrary, SwtLibrary, or RcpLibrary:
//! - All existing keywords continue to work unchanged
//! - New unified keywords are also available
//! - Consider migrating to JavaGuiLibrary for new test suites
//!
//! # Exception Hierarchy
//!
//! The library uses a unified exception hierarchy (see `unified_exceptions` module):
//!
//! - `JavaGuiError` - Base exception for all library errors
//! - `ConnectionError` - Connection-related errors
//! - `ElementError` - Element-related errors
//! - `LocatorError` - Locator parsing errors
//! - `ActionError` - Action execution errors
//! - `TechnologyError` - Technology-specific errors (RCP, SWT)

pub mod element;
pub mod exceptions;
pub mod unified_exceptions;
pub mod base_library;
pub mod swing_library;
pub mod swt_element;
pub mod swt_library;
pub mod rcp_library;

pub use element::SwingElement;
pub use exceptions::*;
pub use unified_exceptions::*;
pub use base_library::JavaGuiLibrary;
pub use swing_library::SwingLibrary;
pub use swt_element::SwtElement;
pub use swt_library::SwtLibrary;
pub use rcp_library::RcpLibrary;
