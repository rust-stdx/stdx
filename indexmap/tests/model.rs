//! Model-based randomized tests, replacing the upstream `quickcheck` suite
//! without any external dependency.
//!
//! A tiny seeded `ChaChaRng` drives random operation sequences against an
//! [`IndexMap`]/[`IndexSet`] and a reference `HashMap`/`HashSet`, asserting
//! that the observable state stays equivalent after every operation.

use std::collections::{HashMap, HashSet};

use indexmap::{IndexMap, IndexSet, map::Entry};
use random::{Rng, RngExt, SeedableRng, rngs::ChaChaRng};

/// Minimal deterministic seeded `ChaChaRng` generator.
fn assert_map_equivalent(map: &IndexMap<u64, u64>, reference: &HashMap<u64, u64>) {
    assert_eq!(map.len(), reference.len());
    for (k, v) in reference {
        assert_eq!(map.get(k), Some(v), "missing or wrong value for key {k}");
    }
    for (k, v) in map {
        assert_eq!(reference.get(k), Some(v), "unexpected key {k}");
    }
    // The dense side must agree with the lookup table.
    for (k, v) in map.as_slice() {
        assert_eq!(reference.get(k), Some(v));
    }
}

#[test]
fn map_model_swap_remove() {
    for seed in 0..128u64 {
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let mut map: IndexMap<u64, u64> = IndexMap::new();
        let mut reference: HashMap<u64, u64> = HashMap::new();

        for _ in 0..500 {
            let key = rng.random_range(0u64..64);
            let value = rng.next_u64();
            match rng.random_range(0..4) {
                0 => {
                    map.insert(key, value);
                    reference.insert(key, value);
                }
                1 => {
                    map.entry(key).or_insert_with(|| value);
                    reference.entry(key).or_insert_with(|| value);
                }
                2 => {
                    map.swap_remove(&key);
                    reference.remove(&key);
                }
                _ => {
                    if let Entry::Occupied(entry) = map.entry(key) {
                        entry.swap_remove_entry();
                    }
                    reference.remove(&key);
                }
            }
            assert_map_equivalent(&map, &reference);
        }
    }
}

#[test]
fn map_model_shift_remove_preserves_order() {
    for seed in 0..32u64 {
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let mut map: IndexMap<u64, u64> = IndexMap::new();
        let mut order: Vec<u64> = Vec::new();

        for _ in 0..500 {
            let key = rng.random_range(0u64..32);
            match rng.random_range(0..3) {
                0 => {
                    if map.insert(key, key).is_none() {
                        order.push(key);
                    }
                }
                1 => {
                    if map.shift_remove(&key).is_some() {
                        order.retain(|&k| k != key);
                    }
                }
                _ => {
                    map.insert(key, key);
                    if !order.contains(&key) {
                        order.push(key);
                    }
                }
            }
            assert_eq!(map.keys().copied().collect::<Vec<_>>(), order);
        }
    }
}

#[test]
fn set_model_swap_remove() {
    for seed in 0..128u64 {
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let mut set: IndexSet<u64> = IndexSet::new();
        let mut reference: HashSet<u64> = HashSet::new();

        for _ in 0..500 {
            let value = rng.random_range(0u64..64);
            match rng.random_range(0..4) {
                0 => {
                    set.insert(value);
                    reference.insert(value);
                }
                1 => {
                    set.replace(value);
                    reference.insert(value);
                }
                2 => {
                    set.swap_remove(&value);
                    reference.remove(&value);
                }
                _ => {
                    set.swap_take(&value);
                    reference.remove(&value);
                }
            }
            assert_eq!(set.len(), reference.len());
            for v in &reference {
                assert!(set.contains(v), "missing value {v}");
            }
            for v in &set {
                assert!(reference.contains(v), "unexpected value {v}");
            }
        }
    }
}

#[test]
fn set_model_shift_remove_preserves_order() {
    for seed in 0..32u64 {
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let mut set: IndexSet<u64> = IndexSet::new();
        let mut order: Vec<u64> = Vec::new();

        for _ in 0..500 {
            let value = rng.random_range(0u64..32);
            match rng.random_range(0..2) {
                0 => {
                    if set.insert(value) {
                        order.push(value);
                    }
                }
                _ => {
                    if set.shift_remove(&value) {
                        order.retain(|&v| v != value);
                    }
                }
            }
            assert_eq!(set.iter().copied().collect::<Vec<_>>(), order);
        }
    }
}
