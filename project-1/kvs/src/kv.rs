use std::collections::HashMap;
///The `KvStore` store key-value pairs in memory.
///This is a Key-Value database store in memery.
///```rust
/// use kvs::KvStore;
/// let mut kv = KvStore::new();
/// kv.set("key1".to_string(),"value1".to_string());
/// assert_eq!(kv.get("key1".to_string()),Some("value1".to_string()));
/// ```
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    ///Create a `KvStore`.
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }
    ///Insert a key-value pair to `KvStore`.
    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    ///Get a value by key from `KvStore`.
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    ///Remove a key-value pair from `KvStore`.
    pub fn remove(&mut self, key: String) -> Option<String> {
        self.map.remove(&key)
    }
}
