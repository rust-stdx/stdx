#![cfg(feature = "std")]
#![allow(clippy::extra_unused_type_parameters)]

use anyerr::Error;

#[test]
fn test_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Error>();
}

#[test]
fn test_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<Error>();
}
