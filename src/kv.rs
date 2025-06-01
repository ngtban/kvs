use crate::Result;
use std::collections::HashMap;
use std::path::Path;

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
/// let set_result = store.set("hello".to_owned(), "world".to_owned());
///
/// let value = store.get("hello".to_owned())?;
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
    pub fn get(&self, key: String) -> Result<Option<String>> {
        if let Some(value) = self.inner.get(&key) {
            Ok(Some(value.to_owned()))
        } else {
            Ok(None)
        }
    }

    /// Sets a value under a string key
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.inner.insert(key, value);

        Ok(())
    }

    /// Removes a value under a string key
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.inner.remove(&key);

        Ok(())
    }

    /// Use a file to store command history
    pub fn open(_path: &Path) -> Result<Self> {
        Ok(Self::default())
    }
}
