use std::collections::HashMap;

pub struct KvStore {
    inner: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            inner: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.inner.get(&key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }

    pub fn remove(&mut self, _key: String) {
        self.inner.remove(&_key);
    }
}
