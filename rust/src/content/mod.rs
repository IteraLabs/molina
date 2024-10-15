//!
//! Extractor of content
//!
pub mod extract;
use std::collections::BTreeMap;

// A collection of Records
pub struct Transcript<T> {
    records: Vec<Record<T>>,
}

// An atomic level representation of content and parsing log (message)
pub struct Record<T> {
    // The actual content
    pub content: T,
    // Errors and Warnings during parsing
    pub messages: Vec<String>,
}

// To be a Parser is to implement parse
trait Parser {
    fn parse<T>(&self, content_document: &Transcript<T>) -> BTreeMap<u32, Record<T>>;
}
