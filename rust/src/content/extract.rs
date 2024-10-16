//! 
//! Extractor of Content
//!
//! # Design Pattern
//!
//! ## Strategy
//! An entity that does a specific job,
//! but uses of different algorithms to execute it.
//!
//! ## Elements
//!
//! Context: Transcript struct
//! Strategy Interface: Parser trait
//! Concrete Strategies: string, table, image, latex, code
//!

// use crate::content::filter::filter_content;
use crate::messages::errors;
use crate::content::filter;
use lopdf::Document;
use std::path::Path;

// --------------------------------------------------------------- PUB : EXTRACT_PDF -- //
// --------------------------------------------------------------- ----------------- -- //

pub fn extract_text<P: AsRef<Path>>(path: P) -> Result<String, errors::ContentError> {
    
    // Attempt to Load Document
    let r_load = Document::load_filtered(path, filter::filter_content)
        .map_err(|_| errors::ContentError::ContentNotFound(String::from("During Attempt to Load Document")))?;
    
    // Attempt to Extract text
    let r_extract = r_load.extract_text(&vec![1])
        .map_err(|_| errors::ContentError::UnsuccessfulExtraction(String::from("During Attempt to Extract Text")));

    r_extract
}

