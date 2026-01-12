//! Python bindings module for Robot Framework Swing library
//!
//! This module provides PyO3-based Python bindings for the Swing automation library,
//! enabling usage from Robot Framework and direct Python scripts.

pub mod element;
pub mod exceptions;
pub mod swing_library;

pub use element::SwingElement;
pub use exceptions::*;
pub use swing_library::SwingLibrary;
