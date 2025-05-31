#![deny(missing_docs)]

//! Key-Value store

use std::collections::HashMap;

/// Struct holding stored key-value pairs.
/// Both keys and values are of `String` type.
/// Currently key-value pairs are stored in memory using a hash map.
///
/// Usage:
///
/// ```rust
/// use kvs::KvStore;
///
/// let mut store = KvStore::new();
///
/// store.set("hello".to_owned(), "world".to_owned());
///
/// let value = store.get("hello".to_owned());
///
/// assert_eq!(value, Some("world".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    inner: HashMap<String, String>,
}

/// Key-value store
impl KvStore {
    /// Creates a new key-value store
    pub fn new() -> Self {
        KvStore {
            inner: HashMap::new(),
        }
    }

    /// Gets a value corresponding to a string key
    pub fn get(&self, key: String) -> Option<String> {
        self.inner.get(&key).cloned()
    }

    /// Sets a value under a string key
    pub fn set(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }

    /// Removes a value under a string key
    pub fn remove(&mut self, _key: String) {
        self.inner.remove(&_key);
    }
}
