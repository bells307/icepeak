use super::IntoData;
use crate::error::DbError;
use bytes::{Bytes, BytesMut};
use icepeak_kv::Data;

impl IntoData for String {
    fn into_data(self) -> Result<Data, DbError> {
        Ok(Data::new(self.into()))
    }
}

impl IntoData for u32 {
    fn into_data(self) -> Result<Data, DbError> {
        let mut buf = BytesMut::with_capacity(4);

        buf.extend_from_slice(&self.to_le_bytes());

        // let bytes = self.to_be_bytes();
        // Ok(Data::new(Bytes::copy_from_slice(&bytes)))
    }
}

// impl IntoData for u32 {
//     fn into_data(self) -> Result<Data, DbError> {
//         let bytes = self.to_be_bytes();
//         Ok(Data::new(Bytes::copy_from_slice(&bytes)))
//     }
// }
