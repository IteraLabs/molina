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
use crate::content::{filter, process};
use crate::messages::errors;
use lopdf::Document;
use std::collections::BTreeMap;
use std::path::Path;

// --------------------------------------------------------------- PUB : EXTRACT_PDF -- //
// --------------------------------------------------------------- ----------------- -- //

pub fn extract_text<P: AsRef<Path>>(
    path: P,
) -> Result<BTreeMap<u32, String>, errors::ContentError> {
    // Attempt to Load Document
    let r_load = Document::load_filtered(path, filter::filter_content).map_err(|_| {
        errors::ContentError::ContentNotFound(String::from(
            "During Attempt to Load Document",
        ))
    })?;

    // Attempt to Extract text
    let mut b_extract = BTreeMap::new();
    let size_document = r_load.get_pages().len() as u32;

    for i in 1..=size_document {
        let i_text = r_load.extract_text(&vec![i]).map_err(|_| {
            errors::ContentError::UnsuccessfulExtraction(String::from(
                "During Attempt to Extract Text",
            ))
        });
        // pre-process extracted text before inserting into BTreeMap

        // to lower case
        let r0_text = process::preprocess_text(&i_text?);

        b_extract.insert(i, r0_text?);
    }

    Ok(b_extract)
}
