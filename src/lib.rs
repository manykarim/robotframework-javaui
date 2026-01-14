//! robotframework-swing: High-performance Robot Framework library for Java Swing/SWT/RCP automation
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
//! # Supported Toolkits
//!
//! - **Swing**: Standard Java GUI toolkit (javax.swing)
//! - **SWT**: Eclipse Standard Widget Toolkit (org.eclipse.swt)
//! - **RCP**: Eclipse Rich Client Platform (org.eclipse.ui)
//!
//! # Feature Flags
//!
//! - `swing` - Enable Swing UI toolkit support
//! - `swt` - Enable Eclipse SWT widget toolkit support
//! - `rcp` - Enable Eclipse RCP support (requires SWT)
//! - `all-toolkits` - Enable all UI toolkits

pub mod error;
pub mod locator;
pub mod model;
pub mod protocol;
pub mod connection;

// Python bindings module
pub mod python;

use pyo3::prelude::*;

/// Python module entry point for the Swing/SWT/RCP automation library.
///
/// This function is called when the module is imported in Python.
/// It registers all Python-accessible classes and exceptions.
#[pymodule]
fn _core(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Add Swing library classes
    m.add_class::<python::swing_library::SwingLibrary>()?;
    m.add_class::<python::element::SwingElement>()?;

    // Add SWT library classes
    m.add_class::<python::swt_library::SwtLibrary>()?;
    m.add_class::<python::swt_element::SwtElement>()?;

    // Add RCP library classes (extends SWT with workbench/perspective/view/editor keywords)
    m.add_class::<python::rcp_library::RcpLibrary>()?;

    // Register exception types
    python::exceptions::register_exceptions(py, m)?;

    // Add version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", "High-performance Robot Framework library for Java Swing/SWT/RCP automation")?;

    Ok(())
}
