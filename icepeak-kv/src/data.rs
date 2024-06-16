use parking_lot::RwLockReadGuard;
use smol_str::SmolStr;
use std::{collections::HashMap, ops::Deref};

pub struct Data(Vec<u8>);

impl Data {
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

type ShardReadGuard<'a> = RwLockReadGuard<'a, HashMap<SmolStr, Data>>;

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
        // SAFETY: структура содержит guard на чтение из хешмапа, соответственно, никто не может
        // менять в нем значения до момента уничтожения `ValuePtr`
        unsafe { &*self.value }
    }
}
