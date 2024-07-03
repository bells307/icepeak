use super::IntoData;
use crate::error::DbError;
use icepeak_kv::Data;

impl IntoData for String {
    fn into_data(self) -> Result<Data, DbError> {
        Ok(Data::from_bytes(self.into_bytes()))
    }
}

impl IntoData for u32 {
    fn into_data(self) -> Result<Data, DbError> {
        Ok(Data::from_bytes(self.to_be_bytes().into()))
    }
}
