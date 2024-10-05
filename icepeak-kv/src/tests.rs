use crate::{data::primitives::arc_str::ArcString, KeyValueStorage};
use chrono::Utc;
use std::time::Duration;

#[tokio::test]
async fn kv_basics() {
    let shard_count = std::thread::available_parallelism().unwrap_or(8.try_into().unwrap());
    let kv = KeyValueStorage::new(shard_count);

    let key = "key";
    let data = ArcString::from("123");

    let prev_data = kv.set(key.into(), data.clone(), None);
    assert!(prev_data.is_none());

    let data_ptr = kv.get::<ArcString>(key).unwrap();
    assert!(data_ptr.is_some());

    let data_ptr = data_ptr.unwrap();
    assert_eq!(*data_ptr, data);

    // Destroy `DataPtr`, otherwise we won't be able to acquire the write lock
    drop(data_ptr);

    kv.remove(key);

    let data_ptr = kv.get::<ArcString>(key).unwrap();
    assert!(data_ptr.is_none());
}

#[tokio::test]
async fn key_expires() {
    let shard_count = std::thread::available_parallelism().unwrap_or(8.try_into().unwrap());
    let kv = KeyValueStorage::new(shard_count);

    let key = "key";
    let data = ArcString::from("123");

    let expires = Utc::now() + chrono::Duration::milliseconds(100);
    let prev_data = kv.set(key.into(), data.clone(), Some(expires));
    assert!(prev_data.is_none());

    let data_ptr = kv.get::<ArcString>(key).unwrap();
    assert!(data_ptr.is_some());

    let data_ptr = data_ptr.unwrap();
    assert_eq!(*data_ptr, data);

    drop(data_ptr);

    // Wait ...
    tokio::time::sleep(Duration::from_millis(200)).await;

    let data_ptr = kv.get::<ArcString>(key).unwrap();
    assert!(data_ptr.is_none());
}
