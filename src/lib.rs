//! robotframework-javagui: High-performance Robot Framework library for Java GUI automation (Swing, SWT, RCP)
//!
//! This crate provides a high-performance Robot Framework library for automating
//! Java Swing, Eclipse SWT, and Eclipse RCP desktop applications. It features:
//!
//! - CSS/XPath-like locator syntax for finding UI elements
//! - Rust-based implementation for optimal performance
//! - Python bindings via PyO3 for Robot Framework integration
//! - Java agent for JVM injection and component inspection
//! - Support for Swing, SWT, and RCP widget toolkits
//!
//! # Architecture
//!
//! The library is organized into the following modules:
//!
//! - `core`: Unified abstractions (Backend trait, configuration, unified elements)
//! - `locator`: Locator parsing and matching
//! - `model`: Data models for UI components
//! - `protocol`: JSON-RPC communication protocol
//! - `connection`: Connection management
//! - `python`: PyO3 bindings for Robot Framework
//!
//! # Supported Toolkits
//!
//! - **Swing**: Standard Java GUI toolkit (javax.swing)
//! - **SWT**: Eclipse Standard Widget Toolkit (org.eclipse.swt)
//! - **RCP**: Eclipse Rich Client Platform (org.eclipse.ui)
//!
//! # Exception Hierarchy
//!
//! The library uses a unified, technology-agnostic exception hierarchy:
//!
//! ```text
//! JavaGuiError (base)
//! +-- ConnectionError
//! |   +-- ConnectionRefusedError
//! |   +-- ConnectionTimeoutError
//! |   +-- NotConnectedError
//! +-- ElementError
//! |   +-- ElementNotFoundError
//! |   +-- MultipleElementsFoundError
//! |   +-- ElementNotInteractableError
//! |   +-- StaleElementError
//! +-- LocatorError
//! |   +-- LocatorParseError
//! |   +-- InvalidLocatorSyntaxError
//! +-- ActionError
//! |   +-- ActionFailedError
//! |   +-- ActionTimeoutError
//! |   +-- ActionNotSupportedError
//! +-- TechnologyError
//! |   +-- ModeNotSupportedError
//! |   +-- RcpWorkbenchError
//! |   +-- SwtShellError
//! +-- InternalError
//! ```
//!
//! Legacy exception names (e.g., `SwingConnectionError`, `SwingTimeoutError`)
//! are maintained as aliases for backwards compatibility.
//!
//! # Feature Flags
//!
//! - `swing` - Enable Swing UI toolkit support
//! - `swt` - Enable Eclipse SWT widget toolkit support
//! - `rcp` - Enable Eclipse RCP support (requires SWT)
//! - `all-toolkits` - Enable all UI toolkits

// Core abstractions module (unified Backend, Config, Element)
pub mod core;

pub mod error;
pub mod locator;
pub mod model;
pub mod protocol;
pub mod connection;

// Python bindings module
pub mod python;

use pyo3::prelude::*;

/// Python module entry point for the JavaGui automation library.
///
/// This function is called when the module is imported in Python as JavaGui._core.
/// It registers all Python-accessible classes and exceptions.
#[pymodule]
fn _core(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Add unified element class (works across all toolkits)
    m.add_class::<core::element::JavaGuiElement>()?;

    // Add unified JavaGuiLibrary (recommended for new projects)
    m.add_class::<python::base_library::JavaGuiLibrary>()?;

    // Add Swing library classes (backwards-compatible wrapper)
    m.add_class::<python::swing_library::SwingLibrary>()?;
    m.add_class::<python::element::SwingElement>()?;

    // Add SWT library classes (backwards-compatible wrapper)
    m.add_class::<python::swt_library::SwtLibrary>()?;
    m.add_class::<python::swt_element::SwtElement>()?;

    // Add RCP library classes (backwards-compatible wrapper, extends SWT)
    m.add_class::<python::rcp_library::RcpLibrary>()?;

    // Register exception types
    python::exceptions::register_exceptions(py, m)?;

    // Add version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", "High-performance Robot Framework library for Java Swing/SWT/RCP automation")?;

    Ok(())
}
