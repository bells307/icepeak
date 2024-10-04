pub(crate) mod ptr;

/// Data stored in the storage - stored as a set of bytes
pub struct Data(Vec<u8>);

impl Data {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn const_ptr(&self) -> *const [u8] {
        self.0.as_ref() as *const _
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl<T> From<T> for Data
where
    T: Into<Vec<u8>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
