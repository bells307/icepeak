use crate::KeyValueStorage;
use std::ops::Deref;
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn kv_basics() {
    let ct = CancellationToken::new();
    let shard_count = std::thread::available_parallelism().unwrap_or(8.try_into().unwrap());

    let kv = KeyValueStorage::new(ct, shard_count);

    let key = "key";
    let data = "123";

    let prev_data = kv.set(key.into(), data.into(), None);
    assert!(prev_data.is_none());

    let data_ptr = kv.get(key);
    assert!(data_ptr.is_some());

    let data_ptr = data_ptr.unwrap();
    let got_data = String::from_utf8_lossy(data_ptr.deref());
    assert_eq!(got_data, data);

    // Destroy `DataPtr`, otherwise we won't be able to acquire the write lock
    drop(data_ptr);

    let removed_data = kv.remove(key);
    assert!(removed_data.is_some());

    let data_ptr = removed_data.unwrap();
    let removed_data = String::from_utf8_lossy(data_ptr.as_slice());
    assert_eq!(removed_data, data);
}
