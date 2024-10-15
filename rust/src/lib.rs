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

// ### extract_content
// Extract's the PDFs content as indicated with the params.
#[pyfunction]
fn extract_content(_input_path: &str) -> PyResult<()> {
    // distinguish the content type
    // match action according to an Enum::variant

    // input the file path and name
    // load contents with lopdf function filtered

    // output a json-like struct inside a string
    // transform the contents into a JSON-like structure

    Ok(())
}

// fn extract_content(input_path: &str) -> PyResult<Document> {
//    let r_content = match content::extract::extract_pdf(input_path) {
//        Ok(v) => v,
//        Err(e) => return Err(PyIOError::new_err(format!("Error occurred: {}", e))),
//    };
//    Ok(r_content)
//}

// ### separate_content
// sepparation into chunks with size as selected
// {sub-character, character, sentence, paragraph, section, page}
#[pyfunction]
fn split_content() -> PyResult<()> {
    Ok(())
}

#[pymodule]
fn molina(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract_content, m)?)?;
    m.add_function(wrap_pyfunction!(split_content, m)?)?;
    Ok(())
}
