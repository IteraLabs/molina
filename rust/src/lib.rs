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

use std::collections::BTreeMap;

// Bridge functions across Python and Rust
use pyo3::prelude::*;

/// Content parsing for PDFs and other tools.
pub mod content;

/// Data I/O processes.
pub mod data;

/// Model's inference engine and related computations.
pub mod inference;

/// Structs and logic for Events, Custom Error Types, Logs
pub mod messages;

// use lopdf::Document;
// ### extract_content
// Extract's the PDFs content as indicated with the params.
#[pyfunction]
fn extract_content(input_file: &str) -> PyResult<BTreeMap<u32, String>> {
    // Attempt to extract text
    let text = content::extract::extract_text(input_file).map_err(|e| {
        match e {
            messages::errors::ContentError::ContentNotFound(msg) => {
                // Raise a FileNotFoundError for ContentNotFound errors
                pyo3::exceptions::PyFileNotFoundError::new_err(msg)
            }
            messages::errors::ContentError::UnsuccessfulExtraction(msg) => {
                // Raise a RuntimeError for UnsuccessfulExtraction error
                pyo3::exceptions::PyRuntimeError::new_err(msg)
            }
        }
    })?;

    // Return the extracted text
    Ok(text)
}

// ### separate_content
// sepparation into chunks with size as selected
// {sub-character, character, sentence, paragraph, section, page}
#[pyfunction]
fn split_content() -> PyResult<()> {
    // println!("split_content call");
    Ok(())
}

#[pymodule]
fn molina(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract_content, m)?)?;
    m.add_function(wrap_pyfunction!(split_content, m)?)?;
    Ok(())
}
