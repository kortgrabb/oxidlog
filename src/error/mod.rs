use thiserror::Error;
#[derive(Error, Debug)]
pub enum JotError {
    #[error("Failed to initialize jot: {0}")]
    InitError(String),

    #[error("Failed to add entry: {0}")]
    AddError(String),

    #[error("Failed to remove entry: {0}")]
    RemoveError(String),

    #[error("Failed to edit entry: {0}")]
    EditError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("{0}")]
    Other(#[from] Box<dyn std::error::Error>),
}

pub type JotResult<T> = Result<T, JotError>;
