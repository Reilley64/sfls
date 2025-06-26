use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    ValidationError(String),
    #[error("Internal error")]
    AnyhowError(anyhow::Error),
    #[error("Standard error")]
    Error(std::io::Error),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::AnyhowError(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Error(err)
    }   
}
