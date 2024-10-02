
use crate::content::parse;
use pyo3::prelude::*;

pub mod content;
pub mod inference;

#[pyfunction]
fn read_pdf(_file_path: &str) -> PyResult<()> {

    let read_result = parse::pdf_generate();
    println!("temporal result {:?}", read_result);
    Ok(())
}

#[pymodule]
fn molina(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_pdf, m)?)?;
    Ok(())
}
