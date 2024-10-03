use chrono::{DateTime, Utc};
pub use data::Data;

mod data;
mod shard;

#[cfg(test)]
mod tests;

use data::DataPtr;
use shard::Shard;
use smol_str::SmolStr;
use std::num::NonZeroUsize;
use tokio_util::sync::CancellationToken;

const DEFAULT_SHARD_COUNT: usize = 4;

/// Key/value storage
///
/// Internally contains shards - a certain number of instances of `HashMap` wrapped in `RwLock` for
/// data synchronization. For each key, the shard index is calculated using the key's hash.
pub struct KeyValueStorage {
    shards: Vec<Shard>,
}

impl KeyValueStorage {
    pub fn new(ct: CancellationToken, shard_count: NonZeroUsize) -> Self {
        let shard_count = shard_count.get();

        let mut shards = Vec::with_capacity(shard_count);

        for _ in 0..shard_count {
            shards.push(Shard::new(ct.clone()));
        }

        Self { shards }
    }
}

impl KeyValueStorage {
    /// Set data for the specified key. If data was previously set for this key,
    /// it will be removed and returned as the method's return value.
    pub fn set(&self, key: SmolStr, data: Data, expires: Option<DateTime<Utc>>) -> Option<Data> {
        self.get_shard(&key).insert(key, data, expires)
    }

    /// Retrieve data by key
    pub fn get(&self, key: &str) -> Option<DataPtr> {
        self.get_shard(key).get(key)
    }

    /// Remove the value from the storage
    pub fn remove(&self, key: &str) -> Option<Data> {
        self.get_shard(key).remove(key)
    }

    /// Get the shard by the key name
    fn get_shard(&self, key: &str) -> &Shard {
        let shard_idx = key_hash(key) % self.shards.len();

        self.shards
            .get(shard_idx)
            .unwrap_or_else(|| panic!("shard with index {shard_idx} does not exist"))
    }
}

// djb2 hash
fn key_hash(input: &str) -> usize {
    let x = 33;
    let mut r = 5381_usize;

    for c in input.chars() {
        r = r.overflowing_mul(x).0 + (c as usize);
        r >>= 0;
    }

    r
}
