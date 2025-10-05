use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinkError {
    #[error("Long url cannot be empty.")]
    EmptyLongUrl,

    #[error("Short code cannot be empty.")]
    EmptyShortCode,

    #[error("Delete key hash cannot be empty.")]
    EmptyDeleteKeyHash,

    #[error("Short code already exists.")]
    CodeAlreadyExists,

    #[error("Failed to generate short code.")]
    CodeGenerationFailure,

    #[error("Entity not found")]
    NotFound,
}
