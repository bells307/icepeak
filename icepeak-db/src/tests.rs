use crate::Database;
use std::ops::Deref;

#[test]
fn get_set() {
    let db = Database::default();

    let key = "key";
    let data = "123";

    let prev_data = db.set(key.into(), data.into());
    assert!(prev_data.is_none());

    let data_ptr = db.get(key);
    assert!(data_ptr.is_some());

    let data_ptr = data_ptr.unwrap();
    let got_data = String::from_utf8_lossy(data_ptr.deref());
    assert_eq!(got_data, data)
}
