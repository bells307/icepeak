use crate::shard::ShardMap;
use parking_lot::RwLockReadGuard;
use std::ops::Deref;

/// Guard object ensuring that data in the shard will not be modified while it exists
type ShardReadGuard<'a> = RwLockReadGuard<'a, ShardMap>;

/// Pointer to data. Contains a guard to prevent data modification in the shard.
///
/// **WARNING**: must be dropped after use, because while this object exists, the shard will remain locked.
pub struct GuardedDataPtr<'a> {
    value: *const [u8],
    _guard: ShardReadGuard<'a>,
}

impl<'a> GuardedDataPtr<'a> {
    pub fn new(value: *const [u8], guard: ShardReadGuard<'a>) -> Self {
        Self {
            value,
            _guard: guard,
        }
    }
}

impl<'a> Deref for GuardedDataPtr<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        // SAFETY: the structure contains a read guard on the hashmap, ensuring that no one can
        // modify its values until `ValuePtr` is destroyed
        unsafe { &*self.value }
    }
}
