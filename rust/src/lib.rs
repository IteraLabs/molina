//! Molina, in honor to THE greatest Mexican researcher of all times, Mario Molina (1943-2020).
//!
//! A Rust and Python Synthetic Integration for an agentic-LLM approach to build a research agent
//! for local knowledge representation.

use crate::content::parse;
use pyo3::prelude::*;

/// Tools for content parsing and generation
pub mod content;

/// Tools for data accessing, I/O, compression.
pub mod data;

/// Tools for making an inference computation
pub mod inference;

/// Events, Custom Error Types, Logs
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
