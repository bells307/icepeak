use chrono::{DateTime, Utc};

use data::Data;
pub use data::{ptr::GuardedDataPtr, DataBytes};

mod data;
mod shard;

#[cfg(test)]
mod tests;

use shard::Shard;
use smol_str::SmolStr;
use std::num::NonZeroUsize;
use tokio_util::sync::CancellationToken;

/// Key/value storage
///
/// Internally contains shards - a certain number of instances of `HashMap` wrapped in `RwLock` for
/// data synchronization. For each key, the shard index is calculated using the key's hash.
pub struct KeyValueStorage {
    shards: Vec<Shard>,
}

impl KeyValueStorage {
    pub fn new(shard_count: NonZeroUsize) -> Self {
        let shard_count = shard_count.get();

        let mut shards = Vec::with_capacity(shard_count);

        for _ in 0..shard_count {
            shards.push(Shard::new());
        }

        Self { shards }
    }

    /// Start shard cleaning
    pub fn run_active_cleaner(&self, ct: CancellationToken) {
        self.shards
            .iter()
            .for_each(|s| s.run_active_cleaner(ct.clone()))
    }
}

impl KeyValueStorage {
    /// Set data for the specified key. If data was previously set for this key,
    /// it will be removed and returned as the method's return value.
    pub fn set(
        &self,
        key: SmolStr,
        data: impl Data,
        expires: Option<DateTime<Utc>>,
    ) -> Option<DataBytes> {
        self.get_shard(&key).insert(key, data.into_data(), expires)
    }

    /// Retrieve data by key
    pub fn get<T>(&self, key: &str) -> Result<Option<GuardedDataPtr<T>>, T::Error>
    where
        T: Data,
    {
        self.get_shard(key).get(key)
    }

    /// Remove the value from the storage
    pub fn remove(&self, key: &str) {
        self.get_shard(key).remove(key);
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
