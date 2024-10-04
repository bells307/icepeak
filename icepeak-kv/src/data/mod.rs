use bytes::Bytes;

pub(crate) mod ptr;

/// Data stored in the storage - stored as a set of bytes
#[derive(Clone)]
pub struct Data(Bytes);

impl Data {
    pub fn new(bytes: Bytes) -> Self {
        Self(bytes)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl<T> From<T> for Data
where
    T: Into<Bytes>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
