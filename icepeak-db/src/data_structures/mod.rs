mod primitives;

use crate::error::DbError;
use icepeak_kv::Data;

/// Возможность привидения типа данных к внутреннему представлению данных в хранилище
pub trait IntoData {
    fn into_data(self) -> Result<Data, DbError>;
}
