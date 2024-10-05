use std::sync::Arc;

pub(crate) mod primitives;
pub(crate) mod ptr;

/// Data stored in the storage - stored as a set of bytes
#[derive(Clone)]
pub struct DataBytes(Arc<[u8]>);

/// Ability to convert a data type into the internal data representation in the storage
pub trait Data: Sized {
    type Error: std::error::Error;

    fn into_data(self) -> DataBytes;
    fn from_data(data: DataBytes) -> Result<Self, Self::Error>;
}

impl DataBytes {
    pub fn new(bytes: Arc<[u8]>) -> Self {
        Self(bytes)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl<T> From<T> for DataBytes
where
    T: Into<Arc<[u8]>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
