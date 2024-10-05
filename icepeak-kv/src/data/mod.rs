use std::sync::Arc;

pub(crate) mod primitives;
pub(crate) mod ptr;

/// Data stored in the storage - stored as a set of bytes
#[derive(Clone)]
pub struct Data(Arc<[u8]>);

/// Ability to convert a data type into the internal data representation in the storage
pub trait DataTrait: Sized {
    type Error: std::error::Error;

    fn into_data(self) -> Data;
    fn from_data(data: Data) -> Result<Self, Self::Error>;
}

impl Data {
    pub fn new(bytes: Arc<[u8]>) -> Self {
        Self(bytes)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl<T> From<T> for Data
where
    T: Into<Arc<[u8]>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
