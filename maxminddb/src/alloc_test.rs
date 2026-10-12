//! Allocation regression tests.
//!
//! `alloc_counter` records allocation calls made by the current thread, so hot
//! paths can be asserted to not allocate without interference from parallel
//! tests.

use std::net::IpAddr;

use alloc_counter::measure;
use serde::Deserialize;

use super::{Reader, Value};

#[test]
fn lookup_of_a_missing_address_does_not_allocate() {
    let reader = Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
    let addr: IpAddr = "203.0.113.99".parse().unwrap();
    // Make sure the address really is a miss.
    assert!(reader.lookup::<Value>(addr).is_err());

    let (result, allocations) = measure(|| reader.lookup::<super::geoip2::City>(addr));
    assert!(result.is_err());
    assert_eq!(allocations.calls, 0, "a lookup miss should not allocate");
}

#[test]
fn borrowed_lookup_does_not_allocate() {
    #[derive(Deserialize)]
    struct Record<'a> {
        iso: &'a str,
    }

    // A record of borrowed data decodes without copying it.
    let mut data = Vec::new();
    super::test_support::enc_val(
        &super::test_support::Val::Map(vec![("iso", super::test_support::Val::Str("US"))]),
        &mut data,
    );
    let db = super::test_support::db_with_data(data, 4);
    let reader = Reader::from_source(db).unwrap();

    let (result, allocations) = measure(|| reader.lookup::<Record>("1.2.3.4".parse().unwrap()));
    assert_eq!(result.unwrap().iso, "US");
    assert_eq!(allocations.calls, 0, "decoding borrowed data should not allocate");
}
