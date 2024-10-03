pub use data::Data;

mod data;

#[cfg(test)]
mod tests;

use data::DataPtr;
use parking_lot::RwLock;
use smol_str::SmolStr;
use std::{collections::HashMap, num::NonZeroUsize};

/// Key/value storage
///
/// Internally contains shards - a certain number of instances of `HashMap` wrapped in `RwLock` for
/// data synchronization. For each key, the shard index is calculated using the key's hash.
pub struct KeyValueStorage {
    shards: Vec<Shard>,
}

impl Default for KeyValueStorage {
    fn default() -> Self {
        let shard_count =
            (std::thread::available_parallelism().map_or(1, usize::from) * 4).next_power_of_two();

        Self::new(unsafe { NonZeroUsize::new_unchecked(shard_count) })
    }
}

type Shard = RwLock<HashMap<SmolStr, Data>>;

impl KeyValueStorage {
    pub fn new(shard_count: NonZeroUsize) -> Self {
        let shard_count = shard_count.get();

        let mut shards = Vec::with_capacity(shard_count);

        for _ in 0..shard_count {
            shards.push(RwLock::new(HashMap::new()));
        }

        Self { shards }
    }
}

impl KeyValueStorage {
    /// Set data for the specified key. If data was previously set for this key,
    /// it will be removed and returned as the method's return value.
    pub fn set(&self, key: SmolStr, data: Data) -> Option<Data> {
        self.get_shard(&key).write().insert(key, data)
    }

    /// Retrieve data by key
    pub fn get(&self, key: &str) -> Option<DataPtr> {
        // Acquire read lock on the shard
        let guard = self.get_shard(key).read();
        let data = (*guard).get(key)?;

        // Place the guard in the structure, which will ensure that the data cannot be modified
        // until `ValuePtr` is destroyed
        Some(DataPtr::new(data.const_ptr(), guard))
    }

    /// Remove the value from the storage
    pub fn remove(&self, key: &str) -> Option<Data> {
        self.get_shard(key).write().remove(key)
    }

    /// Get the shard by the key name
    fn get_shard(&self, key: &str) -> &Shard {
        let hash = key_hash(key);
        let shard_idx = hash % self.shards.len();

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
