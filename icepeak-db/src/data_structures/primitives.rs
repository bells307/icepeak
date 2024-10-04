use super::IntoData;
use crate::error::DbError;
use icepeak_kv::Data;
use std::sync::Arc;

impl IntoData for String {
    fn into_data(self) -> Result<Data, DbError> {
        Ok(Data::new(self.as_bytes().into()))
    }
}

impl IntoData for &str {
    fn into_data(self) -> Result<Data, DbError> {
        Ok(Data::new(self.as_bytes().into()))
    }
}

impl IntoData for i8 {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for i16 {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for i32 {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for i64 {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for i128 {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for isize {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for u8 {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for u16 {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for u32 {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for u64 {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for u128 {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for usize {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for f32 {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for f64 {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = self.to_be_bytes();
        Ok(Data::new(Arc::new(bytes)))
    }
}

impl IntoData for bool {
    fn into_data(self) -> Result<Data, DbError> {
        let bytes = [self as u8];
        Ok(Data::new(Arc::new(bytes)))
    }
}
