use crate::shard::ShardMap;
use parking_lot::RwLockReadGuard;
use std::ops::Deref;

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

/// Guard object ensuring that data in the shard will not be modified while it exists
type ShardReadGuard<'a> = RwLockReadGuard<'a, ShardMap>;

/// Pointer to data. Contains a guard to prevent data modification in the shard.
///
/// **WARNING**: must be dropped after use, because while this object exists, the shard will remain locked.
pub struct DataPtr<'a> {
    value: *const [u8],
    _guard: ShardReadGuard<'a>,
}

impl<'a> DataPtr<'a> {
    pub fn new(value: *const [u8], _guard: ShardReadGuard<'a>) -> Self {
        Self { value, _guard }
    }
}

impl<'a> Deref for DataPtr<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        // SAFETY: the structure contains a read guard on the hashmap, ensuring that no one can
        // modify its values until `ValuePtr` is destroyed
        unsafe { &*self.value }
    }
}
