//! Python bindings module for Robot Framework Swing/SWT/RCP library
//!
//! This module provides PyO3-based Python bindings for the Swing, SWT, and RCP automation libraries,
//! enabling usage from Robot Framework and direct Python scripts.

pub mod element;
pub mod exceptions;
pub mod swing_library;
pub mod swt_element;
pub mod swt_library;
pub mod rcp_library;

pub use element::SwingElement;
pub use exceptions::*;
pub use swing_library::SwingLibrary;
pub use swt_element::SwtElement;
pub use swt_library::SwtLibrary;
pub use rcp_library::RcpLibrary;
