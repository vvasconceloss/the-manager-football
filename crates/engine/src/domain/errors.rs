#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("The input is invalid: {0}")]
    InputError(String),
    #[error("The resource was not found: {0}")]
    NotFoundError(String),
    #[error("An unexpected error has occurred")]
    UnexpectedError,
}
