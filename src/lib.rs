pub struct KvStore {}

impl KvStore {
    pub fn new() -> Self {
        KvStore {}
    }

    pub fn get(&self, _key: String) -> Option<String> {
        None
    }

    pub fn set(&mut self, _key: String, __value: String) {}

    pub fn remove(&mut self, _key: String) {}
}
