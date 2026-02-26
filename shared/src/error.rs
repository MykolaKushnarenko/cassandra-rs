//! Error types for the application.

#![allow(unused)]

/// Error variants that can occur during application execution.
#[derive(Debug)]
pub enum Error {
    /// Invalid request content.
    InvalidRequestContent,
    /// Failed to parse a message.
    ParseError,
    /// An I/O error occurred during connection handling.
    ConnectionError(Option<std::io::Error>),
    /// The received command is not recognized.
    UnknownCommand(String)
}

/// A specialized Result type for application operations.
pub type AppResult<T> = Result<T, Error>;