#![deny(missing_docs)]

//! Key-Value store

pub use error::{KvsError, Result};
pub use kv::KvStore;

mod error;
mod kv;
