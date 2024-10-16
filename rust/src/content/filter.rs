//! 
//! Filtering functions
//!

use lopdf::Object;

pub static SIMPLE_FILTER: &[&str] = &[
    "Length",
];

pub static DEFAULT_FILTER: &[&str] = &[
    "Length",
    "BBox",
    "FormType",
    "Matrix",
    "Type",
    "XObject",
    "Subtype",
    "Filter",
    "ColorSpace",
    "Width",
    "Height",
    "BitsPerComponent",
    "Length1",
    "Length2",
    "Length3",
    "PTEX.FileName",
    "PTEX.PageNumber",
    "PTEX.InfoDict",
    "FontDescriptor",
    "ExtGState",
    "MediaBox",
    "Annot",
];

// ------------------------------------------------------------------ FILTER_CONTENT -- //
// ------------------------------------------------------------------ -------------- -- //

pub fn filter_content(
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

