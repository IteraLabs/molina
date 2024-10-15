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

use crate::messages::errors;
use lopdf::{Document, Object};
use std::path::Path;

static DEFAULT_FILTER: &[&str] = &["Length"];

#[derive(Clone)]
pub enum Source {
    Pdf,
}

// ------------------------------------------------------------------ FILTER_CONTENT -- //
// ------------------------------------------------------------------ -------------- -- //

fn filter_content(
    object_id: (u32, u16),
    object: &mut Object,
) -> Option<((u32, u16), Object)> {
    if DEFAULT_FILTER.contains(&object.type_name().unwrap_or_default()) {
        return None;
    }

    if let Ok(result) = object.as_dict_mut() {
        result.remove(b"Producer");
        result.remove(b"ModDate");
        result.remove(b"Creator");
        result.remove(b"ProcSet");
        result.remove(b"Procset");
        result.remove(b"XObject");
        result.remove(b"MediaBox");
        result.remove(b"Annots");

        if result.is_empty() {
            return None;
        }
    }
    Some((object_id, object.to_owned()))
}

// --------------------------------------------------------------- PUB : EXTRACT_PDF -- //
// --------------------------------------------------------------- ----------------- -- //

pub fn extract_pdf<P: AsRef<Path>>(path: P) -> Result<Document, errors::ContentError> {
    Document::load_filtered(path, filter_content)
        .map_err(|_e| errors::ContentError::ContentNotFound())
}

// ------------------------------------------------------------ PUB : FORMAT_CONTENT -- //
// ------------------------------------------------------------ -------------------- -- //

pub fn format_content() -> Result<(), errors::ContentError> {
    Ok(())
}
