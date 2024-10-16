use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContentError {
    #[error("Loading not possible, file not found {0}")]
    ContentNotFound(String),
    #[error("Unsuccessful extraction process: {0}")]
    UnsuccessfulExtraction(String),
}

