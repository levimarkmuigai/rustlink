use thiserror::Error;

#[derive(Debug, Error)]
pub enum LinkError {
    #[error("Any Link must have an id")]
    LinkIdNotFound,

    #[error("Link hashed code not empty")]
    EmptyHashedCode,

    #[error("Failed to generate code")]
    CodeGenerationFailure,

    #[error("User url is empty")]
    EmptyURL,

    #[error("Persistence error: {0}")]
    PersistenceError(String),
}
