//! Tests for the optional `serde` feature, replacing the upstream
//! `test-serde` crate with `serde_json` (an existing workspace dependency).
#![cfg(feature = "serde")]

use indexmap::{IndexMap, IndexSet, indexmap, indexset};
use serde::{Deserialize, Serialize};

#[test]
fn map_roundtrip_preserves_order() {
    let map: IndexMap<String, i32> = indexmap! {
        "b".to_string() => 2,
        "a".to_string() => 1,
        "c".to_string() => 3,
    };

    let json = serde_json::to_string(&map).unwrap();
    assert_eq!(json, r#"{"b":2,"a":1,"c":3}"#);

    let back: IndexMap<String, i32> = serde_json::from_str(&json).unwrap();
    assert!(back.keys().eq(map.keys()));
    assert_eq!(back, map);
}

#[test]
fn set_roundtrip_preserves_order() {
    let set = indexset! { 3, 1, 2 };

    let json = serde_json::to_string(&set).unwrap();
    assert_eq!(json, "[3,1,2]");

    let back: IndexSet<i32> = serde_json::from_str(&json).unwrap();
    assert!(back.iter().eq(set.iter()));
    assert_eq!(back, set);
}

#[test]
fn serde_seq_roundtrip() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct Wrapper {
        #[serde(with = "indexmap::map::serde_seq")]
        map: IndexMap<String, i32>,
    }

    let wrapper = Wrapper {
        map: indexmap! {
            "b".to_string() => 2,
            "a".to_string() => 1,
        },
    };

    // `serde_seq` serializes as an ordered sequence of `(key, value)` pairs.
    let json = serde_json::to_string(&wrapper).unwrap();
    assert_eq!(json, r#"{"map":[["b",2],["a",1]]}"#);

    let back: Wrapper = serde_json::from_str(&json).unwrap();
    assert_eq!(back, wrapper);
    assert!(back.map.keys().eq(wrapper.map.keys()));
}
