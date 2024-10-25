use thiserror::Error;

/// Errors that can occur when working with YDKE URLs.
#[derive(Error, Debug)]
pub enum YdkeError {
    /// The URL is not a valid YDKE URL
    #[error("Invalid URL format: The URL must start with 'ydke://'")]
    NotYdkeUrl,
    /// The URL is missing required exclamation marks
    #[error("Invalid URL format: The URL must contain exactly three '!' separators to divide main, extra, and side decks")]
    MissingDelimiters,
    /// Failed to decode base64 data
    #[error("Base64 decode error: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
}