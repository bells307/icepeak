use std::sync::Arc;

pub(crate) mod ptr;

/// Data stored in the storage - stored as a set of bytes
#[derive(Clone)]
pub struct Data(Arc<[u8]>);

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
