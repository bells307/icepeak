use super::IntoData;
use crate::error::DbError;
use icepeak_kv::Data;
use std::sync::Arc;

impl IntoData for String {
    fn into_data(self) -> Result<Data, DbError> {
        Ok(Data::new(self.as_bytes().into()))
    }
}

impl IntoData for u32 {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}
