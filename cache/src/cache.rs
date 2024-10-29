use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, RwLock};

use crate::error::{CacheErrorExt, CacheResult};

pub struct Cache<K, V> {
    storage: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> Clone for Cache<K, V> {
    fn clone(&self) -> Self {
        Self {
            storage: Arc::clone(&self.storage),
        }
    }
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone + Display,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn set(&self, k: K, v: V) -> CacheResult<()> {
        self.storage
            .write()
            .map_err(|e| anyhow::anyhow!("Lock error {}", e))
            .map(|mut guard| guard.insert(k, v))?;

        Ok(())
    }

    pub async fn get(&self, k: K) -> CacheResult<V> {
        let storage = self
            .storage
            .read()
            .map_err(|e| anyhow::anyhow!("Lock error {}", e))?;

        storage
            .get(&k)
            .cloned()
            .cache_err(&format!("Key not found {}", k))
    }

    pub async fn delete(&self, k: K) -> CacheResult<bool> {
        let mut storage = self
            .storage
            .write()
            .map_err(|e| anyhow::anyhow!("Lock error {}", e))?;

        Ok(storage.remove(&k).is_some())
    }

    pub async fn clear(&self) -> CacheResult<()> {
        let mut storage = self
            .storage
            .write()
            .map_err(|e| anyhow::anyhow!("Lock error {}", e))?;

        storage.clear();
        Ok(())
    }
}
