#![deny(missing_docs)]

//! Key-Value store

pub use command::Commands;
pub use error::{KvsError, Result};
pub use kv::KvStore;

mod command;
mod error;
mod kv;
