use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContentError {
    #[error("Loading not possible, file not found")]
    ContentNotFound(),
}
