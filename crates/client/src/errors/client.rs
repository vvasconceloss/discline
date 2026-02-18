use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Unauthorized: Invalid Discord token")]
    Unauthorized,

    #[error("Forbidden: Missing permissions to access this resource")]
    Forbidden,

    #[error("Not found: {resource_type} with ID {resource_id} doesn't exist")]
    NotFound {
        resource_type: String,
        resource_id: String,
    },

    #[error("Rate limited: Retry after {retry_after} seconds")]
    RateLimited { retry_after: u64 },

    #[error("Discord API error: {status} - {message}")]
    ApiError { status: u16, message: String },

    #[error("Failed to parse response: {0}")]
    ParseError(String),
}
