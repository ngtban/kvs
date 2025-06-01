use std::io;
use thiserror::Error;

/// Enum containing all errors that might be thrown by the library
#[derive(Debug, Error)]
pub enum KvsError {
    /// IO failure when opening command log file
    #[error("{0}")]
    Io(#[from] io::Error),
    /// Serialization failure from serde
    #[error("{0}")]
    Serialization(#[from] serde_json::Error),
    /// Error for when no key matching what the user gave exists
    #[error("key not found")]
    KeyNotFound,
    /// Error when the user attempts to execute an unknown command
    #[error("unknown command type")]
    UnknownCommand,
}

/// Shorthand for result types used by all functions in the library
pub type Result<T> = std::result::Result<T, KvsError>;
