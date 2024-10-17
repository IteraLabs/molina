//!
//! Process
//!
//! Convert to lower case
//! Remove all urls
//! Remove all emails
//! Remove extra spaces
//! Remove citations
//!

use crate::messages::errors;
use regex::Regex;

// --------------------------------------------------------------------------------- -- //
// --------------------------------------------------------------------------------- -- //
pub fn to_lowercase(text: &str) -> Result<String, errors::ContentError> {
    Ok(text.to_lowercase())
}

pub fn remove_emails(text: &str) -> Result<String, errors::ContentError> {
    let email_pattern = r"\{?\s*[\w\s,]+\s*\}?\s*@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}";
    let re = Regex::new(email_pattern).unwrap();

    Ok(re.replace_all(&text, "").to_string())
}

pub fn remove_urls(text: &str) -> Result<String, errors::ContentError> {
    let url_pattern = r"http[s]?:\s*//[^\s]+|www\.[^\s]+";
    let re = Regex::new(url_pattern).unwrap();

    Ok(re.replace_all(&text, "").to_string())
}

pub fn handle_emptyspaces(text: &str) -> Result<String, errors::ContentError> {
    // Trim leading and trailing whitespace
    let trimmed = text.trim();

    // Replace multiple spaces with a single space
    let space_pattern = r"\s+";
    let re = Regex::new(space_pattern).unwrap();

    // Replace all occurrences of multiple spaces with a single space
    Ok(re.replace_all(trimmed, " ").to_string())
}

pub fn handle_citations(text: &str) -> Result<String, errors::ContentError> {
    let citation_pattern = r"\(\s*[^()]*?\s*,\s*\d{4}\s*(;\s*[^()]*?\s*,\s*\d{4}\s*)*\)";
    let re = Regex::new(&citation_pattern).unwrap();

    Ok(re.replace_all(&text, "").to_string())
}

pub fn preprocess_text(text: &str) -> Result<String, errors::ContentError> {
    let lowercased = to_lowercase(text);
    let without_urls = remove_urls(&lowercased?);
    let without_emails = remove_emails(&without_urls?);
    let without_extra_spaces = handle_emptyspaces(&without_emails?);
    let without_citation = handle_citations(&without_extra_spaces?);
    Ok(without_citation?)
}
