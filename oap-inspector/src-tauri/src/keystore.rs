use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct KeyStore {
    // Map of Public Key / Key ID -> Private Key (PEM or Multibase)
    keys: Arc<Mutex<HashMap<String, String>>>,
}

impl KeyStore {
    pub fn new() -> Self {
        Self {
            keys: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_key(&self, key_id: String, private_key: String) {
        let mut k = self.keys.lock().unwrap();
        k.insert(key_id, private_key);
    }

    pub fn remove_key(&self, key_id: &str) {
        let mut k = self.keys.lock().unwrap();
        k.remove(key_id);
    }

    pub fn list_keys(&self) -> Vec<String> {
        let k = self.keys.lock().unwrap();
        k.keys().cloned().collect()
    }
    
    pub fn get_key(&self, key_id: &str) -> Option<String> {
        let k = self.keys.lock().unwrap();
        k.get(key_id).cloned()
    }

    pub fn get_all_secrets(&self) -> Vec<String> {
        let k = self.keys.lock().unwrap();
        k.values().cloned().collect()
    }
}
