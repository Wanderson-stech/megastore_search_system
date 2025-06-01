use std::collections::HashMap;

pub struct Cache {
    store: HashMap<String, Vec<u32>>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<u32>> {
        self.store.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: Vec<u32>) {
        self.store.insert(key, value);
    }
}
