pub mod error;

mod data_structures;

use data_structures::IntoData;
use error::DbError;
use icepeak_kv::{Data, KeyValueStorage};
use smol_str::SmolStr;

pub struct Database {
    kv: KeyValueStorage,
}

impl Database {
    pub fn new(kv: KeyValueStorage) -> Self {
        Self { kv }
    }

    /// Добавить значение в базу данных
    pub fn insert(&self, key: SmolStr, data: impl IntoData) -> Result<Option<Data>, DbError> {
        let data = data.into_data()?;
        Ok(self.kv.set(key, data))
    }
}
