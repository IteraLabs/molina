//! # Molina
//! 
//! *In honor to THE greatest Mexican researcher of all times, Mario Molina (1943-2020).
//!
//! A Rust and Python Synthetic Integration for an agentic-LLM approach to build research agents
//! for local knowledge representation.
//!
//! - **Python**: A wrapper of core functionality in Rust, Interface to models.
//! - **Rust**: NLP tasks, Prompting, Routing, Content parsing.
//!

use crate::content::parse;
use pyo3::prelude::*;

/// Content parsing for PDFs and other tools.
pub mod content;

/// Data I/O processes.
pub mod data;

/// Model's inference engine and related computations.
pub mod inference;

/// Structs and logic for Events, Custom Error Types, Logs
pub mod messages;

#[pyfunction]
fn read_pdf(_file_path: &str) -> PyResult<()> {
    let read_result = parse::pdf_generate();
    println!("temporal result is this {:?}", read_result);
    Ok(())
}

#[pymodule]
fn molina(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_pdf, m)?)?;
    Ok(())
}
