use super::DataTrait;
use crate::shard::ShardInner;
use parking_lot::RwLockReadGuard;
use std::ops::Deref;

/// Guard object ensuring that data in the shard will not be modified while it exists
type ShardReadGuard<'a> = RwLockReadGuard<'a, ShardInner>;

/// Pointer to data. Contains a guard to prevent data modification in the shard.
///
/// **WARNING**: must be dropped after use, because while this object exists, the shard will remain locked.
pub struct GuardedDataPtr<'a, T> {
    data: T,
    _guard: ShardReadGuard<'a>,
}

impl<'a, T> GuardedDataPtr<'a, T>
where
    T: DataTrait,
{
    pub fn new(data: T, guard: ShardReadGuard<'a>) -> Self {
        Self {
            data,
            _guard: guard,
        }
    }
}

impl<'a, T> Deref for GuardedDataPtr<'a, T>
where
    T: DataTrait,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
