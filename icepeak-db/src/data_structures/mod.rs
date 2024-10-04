mod primitives;

use crate::error::DbError;
use icepeak_kv::Data;

// Ability to convert a data type into the internal data representation in the storage
pub trait IntoData {
    fn into_data(self) -> Result<Data, DbError>;
}
