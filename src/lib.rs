//! robotframework-javagui: High-performance Robot Framework library for Java GUI automation
//!
//! This crate provides a high-performance Robot Framework library for automating
//! Java GUI desktop applications including Swing, SWT, and Eclipse RCP. It features:
//!
//! - CSS/XPath-like locator syntax for finding UI elements
//! - Rust-based implementation for optimal performance
//! - Python bindings via PyO3 for Robot Framework integration
//! - Java agent for JVM injection and component inspection
//! - Support for Swing, SWT (Standard Widget Toolkit), and Eclipse RCP applications

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
    // Add library classes
    m.add_class::<python::swing_library::SwingLibrary>()?;
    m.add_class::<python::element::SwingElement>()?;

    // Register exception types
    python::exceptions::register_exceptions(py, m)?;

    // Add version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", "High-performance Robot Framework library for Java GUI automation (Swing, SWT, RCP)")?;

    Ok(())
}
