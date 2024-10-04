mod clean;
mod value;

use crate::{Data, GuardedDataPtr};
use chrono::{DateTime, Utc};
use clean::ShardActiveCleaner;
use parking_lot::RwLock;
use smol_str::SmolStr;
use std::{collections::HashMap, mem, sync::Arc};
use tokio_util::sync::CancellationToken;
use value::ShardedValue;

/// Hashmap with shard data
pub type ShardMap = HashMap<SmolStr, ShardedValue>;

/// Container of keys and values
#[derive(Clone)]
pub struct Shard {
    inner: Arc<RwLock<ShardInner>>,
}

pub struct ShardInner {
    /// Hashmap with data
    map: ShardMap,
    /// Array of keys that exist in the hashmap (necessary for random key retrieval and active
    /// cleaning)
    keys: Vec<SmolStr>,
}

impl Shard {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(ShardInner {
                map: ShardMap::new(),
                keys: Vec::new(),
            })),
        }
    }

    /// Start shard cleaning
    pub fn run_active_cleaner(&self, ct: CancellationToken) {
        ShardActiveCleaner::run(ct, self.clone())
    }

    /// Set data for the specified key. If data was previously set for this key,
    /// it will be removed and returned as the method's return value.
    pub fn insert(
        &self,
        key: SmolStr,
        mut data: Data,
        expires: Option<DateTime<Utc>>,
    ) -> Option<Data> {
        let mut lock = self.inner.write();

        match lock.map.get_mut(&key) {
            Some(val) => {
                mem::swap(&mut val.data, &mut data);

                // Update expiration time only if it is explicitly specified
                if let Some(exp) = expires {
                    val.expires = Some(exp.timestamp_millis());
                }

                Some(data)
            }
            None => {
                // If the data was not previously added by key, save the key in the keys array and
                // record the key index in the hashmap
                lock.keys.push(key.clone());

                let idx = lock.keys.len() - 1;

                let expires = expires.map(|dt| dt.timestamp_millis());
                lock.map.insert(key, ShardedValue { data, idx, expires });

                None
            }
        }
    }

    /// Retrieve data by key
    pub fn get(&self, key: &str) -> Option<GuardedDataPtr> {
        let lock = self.inner.read();

        match lock.map.get(key) {
            Some(val) => {
                // Check if the key has expired. If it has, delete it
                if val.expired() {
                    drop(lock);
                    self.inner.write().map.remove(key);
                    None
                } else {
                    // Place the guard in the structure, which will ensure that the data cannot be modified
                    // until `DataPtr` is destroyed
                    Some(GuardedDataPtr::new(val.data.clone(), lock))
                }
            }
            None => None,
        }
    }

    /// Remove data from the shard
    pub fn remove(&self, key: &str) -> Option<Data> {
        let mut lock = self.inner.write();

        let ShardedValue { data, idx, .. } = lock.map.remove(key)?;
        lock.keys.remove(idx);
        Some(data)
    }
}
