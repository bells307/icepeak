mod value;

use parking_lot::RwLock;
use std::{collections::HashMap, num::NonZeroUsize};
use value::{Value, ValuePtr};

#[derive(Default)]
pub struct Database {
    shards: Vec<Shard>,
    shift: usize,
}

type Shard = RwLock<HashMap<String, Value>>;

impl Database {
    pub fn new(shard_count: NonZeroUsize) -> Self {
        let shard_count = shard_count.get();

        let mut shards = Vec::with_capacity(shard_count);

        for _ in 0..shard_count {
            shards.push(RwLock::new(HashMap::new()));
        }

        let shift = calc_shift(shard_count);

        Self { shards, shift }
    }
}

impl Database {
    pub fn set(&self, key: String, value: Value) -> Option<Value> {
        self.get_shard(&key).write().insert(key, value)
    }

    pub fn get(&self, key: &str) -> Option<ValuePtr> {
        // Берем лок шарда на чтение
        let guard = self.get_shard(key).read();
        let val = (*guard).get(key)?;

        // Помещаем guard в структуру, что будет гарантировать нам невозможность изменения
        // данных до уничтожения `ValuePtr`
        Some(ValuePtr::new(val, guard))
    }

    pub fn remove(&self, key: &str) -> Option<Value> {
        self.get_shard(key).write().remove(key)
    }

    fn get_shard(&self, key: &str) -> &Shard {
        let shard_idx = self.key_shard_idx(key);

        self.shards
            .get(shard_idx)
            .unwrap_or_else(|| panic!("shard with index {shard_idx} does not exist"))
    }

    fn key_shard_idx(&self, value: &str) -> usize {
        let hash = key_hash(value);
        (hash << 7) >> self.shift
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

fn calc_shift(shard_count: usize) -> usize {
    (std::mem::size_of::<usize>() * 8) - (shard_count.trailing_zeros() as usize)
}
