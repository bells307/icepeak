pub use data::Data;

mod data;

#[cfg(test)]
mod tests;

use data::DataPtr;
use parking_lot::RwLock;
use smol_str::SmolStr;
use std::{collections::HashMap, num::NonZeroUsize};

/// Key/value хранилище
///
/// Внутри себя имеет шарды - определенное количество экземпляров `HashMap`, обернутых в `RwLock` для
/// синхронизации данных. Для каждого ключа с помощью хэша вычисляется индекс шарда.
pub struct KeyValueStorage {
    shards: Vec<Shard>,
    shift: usize,
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

        let shift = calc_shift(shard_count);

        Self { shards, shift }
    }
}

impl KeyValueStorage {
    /// Установка данных по указанному ключу. Если по такому ключу данные были ранее установлены,
    /// то они будут удалены и переданы в качестве возвращаемого значения метода.
    pub fn set(&self, key: SmolStr, data: Data) -> Option<Data> {
        self.get_shard(&key).write().insert(key, data)
    }

    pub fn get(&self, key: &str) -> Option<DataPtr> {
        // Берем лок шарда на чтение
        let guard = self.get_shard(key).read();
        let data = (*guard).get(key)?;

        // Помещаем guard в структуру, что будет гарантировать нам невозможность изменения
        // данных до уничтожения `ValuePtr`
        Some(DataPtr::new(data.const_ptr(), guard))
    }

    /// Удалить значение из хранилища
    pub fn remove(&self, key: &str) -> Option<Data> {
        self.get_shard(key).write().remove(key)
    }

    /// Получить шард по имени ключа
    fn get_shard(&self, key: &str) -> &Shard {
        let shard_idx = self.key_shard_idx(key);

        self.shards
            .get(shard_idx)
            .unwrap_or_else(|| panic!("shard with index {shard_idx} does not exist"))
    }

    /// Вычисление индекса шарда
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
