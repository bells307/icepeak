use std::{collections::HashMap, ops::Deref};

use parking_lot::RwLockReadGuard;

type ShardReadGuard<'a> = RwLockReadGuard<'a, HashMap<String, Value>>;

#[derive(Hash, Eq, PartialEq)]
pub enum Value {}

pub struct ValuePtr<'a> {
    value: *const Value,
    _guard: ShardReadGuard<'a>,
}

impl<'a> ValuePtr<'a> {
    pub fn new(value: *const Value, _guard: ShardReadGuard<'a>) -> Self {
        Self { value, _guard }
    }
}

impl<'a> Deref for ValuePtr<'a> {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        // SAFETY: структура содержит guard на чтение из хешмапа, соответственно, никто не может
        // менять в нем значения до момента уничтожения `ValueRef`
        unsafe { &*self.value }
    }
}
