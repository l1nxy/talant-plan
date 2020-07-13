use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        let v = self.map.get(&key);
        match v {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
