use crate::{shard::ShardInner, Data};
use parking_lot::RwLockReadGuard;
use std::ops::Deref;

/// Guard object ensuring that data in the shard will not be modified while it exists
type ShardReadGuard<'a> = RwLockReadGuard<'a, ShardInner>;

/// Pointer to data. Contains a guard to prevent data modification in the shard.
///
/// **WARNING**: must be dropped after use, because while this object exists, the shard will remain locked.
pub struct GuardedDataPtr<'a> {
    data: Data,
    _guard: ShardReadGuard<'a>,
}

impl<'a> GuardedDataPtr<'a> {
    pub fn new(data: Data, guard: ShardReadGuard<'a>) -> Self {
        Self {
            data,
            _guard: guard,
        }
    }
}

impl<'a> Deref for GuardedDataPtr<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.data.0.deref()
    }
}
