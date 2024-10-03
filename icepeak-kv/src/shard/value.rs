use crate::Data;
use chrono::Utc;

/// Value in the hashmap with data
pub struct ShardedValue {
    /// The actual data
    pub(super) data: Data,
    /// Index of the key in the keys array
    pub(super) idx: usize,
    /// Time when the value expires (ms)
    pub(super) expires: Option<i64>,
}

impl ShardedValue {
    /// Check if the value has expired
    pub fn expired(&self) -> bool {
        match self.expires {
            Some(exp) => Utc::now().timestamp_millis() > exp,
            None => false,
        }
    }
}
