mod clean;
mod value;

use crate::{Data, GuardedDataPtr};
use chrono::{DateTime, Utc};
use clean::ShardCleaner;
use parking_lot::RwLock;
use smol_str::SmolStr;
use std::{collections::HashMap, mem, sync::Arc};
use tokio_util::sync::CancellationToken;
use value::ShardedValue;

/// Hashmap with shard data
pub type ShardMap = HashMap<SmolStr, ShardedValue>;

/// Container of keys and values
#[derive(Default, Clone)]
pub struct Shard {
    /// Hashmap with data
    map: Arc<RwLock<ShardMap>>,
    /// Array of keys that exist in the hashmap (necessary for random key retrieval and active
    /// cleaning)
    keys: Arc<RwLock<Vec<SmolStr>>>,
}

impl Shard {
    pub fn new(ct: CancellationToken) -> Self {
        let shard = Self::default();
        // Start shard cleaning
        ShardCleaner::run(ct, shard.clone());
        shard
    }

    pub fn insert(
        &self,
        key: SmolStr,
        mut data: Data,
        expires: Option<DateTime<Utc>>,
    ) -> Option<Data> {
        let mut map_lock = self.map.write();

        match map_lock.get_mut(&key) {
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
                let mut keys_lock = self.keys.write();
                keys_lock.push(key.clone());

                let idx = keys_lock.len() - 1;

                let expires = expires.map(|dt| dt.timestamp_millis());
                map_lock.insert(key, ShardedValue { data, idx, expires });

                None
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<GuardedDataPtr> {
        let lock = self.map.read();

        match lock.get(key) {
            Some(val) => {
                // Check if the key has expired. If it has, delete it
                if val.expired() {
                    drop(lock);
                    self.map.write().remove(key);
                    None
                } else {
                    // Place the guard in the structure, which will ensure that the data cannot be modified
                    // until `DataPtr` is destroyed
                    Some(GuardedDataPtr::new(val.data.const_ptr(), lock))
                }
            }
            None => None,
        }
    }

    /// Remove data from the shard
    pub fn remove(&self, key: &str) -> Option<Data> {
        let mut map_lock = self.map.write();
        let mut keys_lock = self.keys.write();

        let ShardedValue { data, idx, .. } = map_lock.remove(key)?;
        keys_lock.remove(idx);
        Some(data)
    }
}
