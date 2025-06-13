use std::io;
use thiserror::Error;

/// Enum containing all errors that might be thrown by the library
#[derive(Debug, Error)]
pub enum KvsError {
    /// IO failure when opening command log file
    #[error("{0}")]
    Io(#[from] io::Error),
    /// Decoding failure from bincode
    #[error("{0}")]
    Decode(#[from] bincode::error::DecodeError),
    /// Encoding failure from bincode
    #[error("{0}")]
    Encode(#[from] bincode::error::EncodeError),
    /// Error for when no key matching what the user gave exists
    #[error("key not found")]
    KeyNotFound,
    /// Error when the user attempts to execute an unknown command
    #[error("unknown command type")]
    UnknownCommand,
    #[error("unexpected command")]
    /// Error when retrieving a value from the log and the command we found is not a Set command
    UnexpectedCommand,
}

/// Shorthand for result types used by all functions in the library
pub type Result<T> = std::result::Result<T, KvsError>;
