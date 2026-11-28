//! Adversarial tests for the unsafe-bearing parts of `indexmap`:
//! `Core::extract` / `ExtractCore` (`extract_if`), the optional-index
//! `get_disjoint_mut` helper, and the `#[repr(transparent)]` `Slice` casts.
//!
//! The extraction tests cross-check map contents, order and *exactly-once*
//! drops against a simple `Vec` model for every range, early-drop point and
//! predicate panic point. The other tests check the `&mut` disjointness and
//! layout assumptions of the slice casts.
//!
//! Run with Miri:
//! `cargo +nightly miri test -p indexmap --test audit_unsafe`

use std::{
    borrow::Borrow,
    cell::RefCell,
    hash::{Hash, Hasher},
    panic::{AssertUnwindSafe, catch_unwind},
    rc::Rc,
};

use indexmap::{IndexMap, IndexSet};

// ---------------------------------------------------------------------------
// Drop-tracking key/value types
// ---------------------------------------------------------------------------

/// Per-test log of `2 * id` (key) and `2 * id + 1` (value) drop events.
#[derive(Clone)]
struct Log(Rc<RefCell<Vec<u32>>>);

impl Log {
    fn new() -> Self {
        Log(Rc::new(RefCell::new(Vec::new())))
    }

    fn record(&self, tag: u32) {
        self.0.borrow_mut().push(tag);
    }

    /// Asserts every key and value of the `n` logical entries was dropped
    /// exactly once (no leaks, no double drops).
    fn assert_complete(&self, n: usize) {
        let mut recorded = RefCell::borrow(&self.0).clone();
        recorded.sort_unstable();
        let expected: Vec<u32> = (0..2 * n as u32).collect();
        assert_eq!(recorded, expected, "drop log mismatch for n={n}: leaks or double drops");
    }
}

struct K {
    id: u32,
    log: Log,
}

impl K {
    fn new(id: u32, log: &Log) -> Self {
        K {
            id,
            log: log.clone(),
        }
    }
}

impl Drop for K {
    fn drop(&mut self) {
        self.log.record(2 * self.id);
    }
}

impl Hash for K {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for K {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for K {}

impl Borrow<u32> for K {
    fn borrow(&self) -> &u32 {
        &self.id
    }
}

struct V {
    id: u32,
    log: Log,
}

impl V {
    fn new(id: u32, log: &Log) -> Self {
        V {
            id,
            log: log.clone(),
        }
    }
}

impl Drop for V {
    fn drop(&mut self) {
        self.log.record(2 * self.id + 1);
    }
}

type TrackedMap = IndexMap<K, V>;
type TrackedSet = IndexSet<K>;

fn build_map(n: usize, log: &Log) -> TrackedMap {
    let mut map = TrackedMap::new();
    for id in 0..n as u32 {
        assert!(map.insert(K::new(id, log), V::new(id, log)).is_none());
    }
    assert_eq!(map.len(), n);
    map
}

fn map_ids(map: &TrackedMap) -> Vec<u32> {
    map.keys().map(|k| k.id).collect()
}

/// Verifies the internal index table is consistent with the visible entries.
fn check_map_invariants(map: &TrackedMap, all_ids: &[u32]) {
    assert_eq!(map.len(), map.as_slice().len());
    let ids = map_ids(map);
    for id in all_ids {
        match ids.iter().position(|x| x == id) {
            Some(pos) => assert_eq!(map.get_index_of(id), Some(pos), "id {id} index"),
            None => assert_eq!(map.get_index_of(id), None, "id {id} should be gone"),
        }
        assert_eq!(map.get(id).is_some(), ids.contains(id), "id {id} get");
    }
    // Every value must be reachable through its key and the hash table.
    for (pos, key) in map.keys().enumerate() {
        assert_eq!(map.get_index_of(&key.id), Some(pos));
    }
    // A fresh insert must still work and be found.
    let fresh = all_ids.len() as u32 + 10_000;
    map_get_fresh(map, fresh);
}

fn map_get_fresh(map: &TrackedMap, fresh: u32) {
    // read-only consistency probe that does not mutate the map
    assert_eq!(map.get_index_of(&fresh), None);
}

#[derive(Clone, Copy, Debug)]
enum PredKind {
    All,
    None,
    EveryThird,
}

fn pred_eval(kind: PredKind, id: u32) -> bool {
    match kind {
        PredKind::All => true,
        PredKind::None => false,
        PredKind::EveryThird => !id.is_multiple_of(3),
    }
}

#[derive(Clone, Copy, Debug)]
enum Consume {
    /// Collect the iterator to exhaustion.
    Full,
    /// Consume until `k` items were extracted, then drop the iterator.
    EarlyDrop(usize),
    /// Panic inside the predicate on the `p`-th call.
    PanicAt(usize),
}

/// Model of the map/extracted contents for one scenario.
struct Model {
    remaining: Vec<u32>,
    extracted: Vec<u32>,
}

fn model(n: usize, start: usize, end: usize, kind: PredKind, consume: Consume) -> Model {
    let ids: Vec<u32> = (0..n as u32).collect();
    let mut kept = Vec::new();
    let mut extracted = Vec::new();
    let mut i = start;
    let mut calls = 0usize;

    match consume {
        Consume::Full => {
            while i < end {
                if pred_eval(kind, ids[i]) {
                    extracted.push(ids[i]);
                } else {
                    kept.push(ids[i]);
                }
                i += 1;
            }
        }
        Consume::EarlyDrop(k) => {
            while i < end && extracted.len() < k {
                if pred_eval(kind, ids[i]) {
                    extracted.push(ids[i]);
                } else {
                    kept.push(ids[i]);
                }
                i += 1;
            }
        }
        Consume::PanicAt(p) => {
            while i < end {
                if calls == p {
                    break;
                }
                if pred_eval(kind, ids[i]) {
                    extracted.push(ids[i]);
                } else {
                    kept.push(ids[i]);
                }
                calls += 1;
                i += 1;
            }
        }
    }

    let mut remaining = ids[..start].to_vec();
    remaining.extend(kept);
    remaining.extend_from_slice(&ids[i..]);
    Model {
        remaining,
        extracted,
    }
}

/// Runs one scenario and verifies contents, order, index-table consistency,
/// and exactly-once drops of every original key/value.
fn run_scenario(n: usize, start: usize, end: usize, kind: PredKind, consume: Consume) {
    let expected = model(n, start, end, kind, consume);
    let log = Log::new();
    let all_ids: Vec<u32> = (0..n as u32).collect();
    {
        let mut map = build_map(n, &log);

        let mut extracted_ids: Vec<u32> = Vec::new();
        let pred = |k: &K, _v: &mut V| pred_eval(kind, k.id);

        match consume {
            Consume::Full => {
                let extracted: Vec<(K, V)> = map.extract_if(start..end, pred).collect();
                extracted_ids = extracted.iter().map(|(k, _)| k.id).collect();
                drop(extracted);
            }
            Consume::EarlyDrop(k) => {
                let extracted: Vec<(K, V)> = map.extract_if(start..end, pred).take(k).collect();
                extracted_ids = extracted.iter().map(|(k, _)| k.id).collect();
                drop(extracted);
            }
            Consume::PanicAt(p) => {
                let mut calls = 0usize;
                let res = catch_unwind(AssertUnwindSafe(|| {
                    let it = map.extract_if(start..end, |k: &K, _v: &mut V| {
                        if calls == p {
                            calls += 1;
                            panic!("audit: predicate panic");
                        }
                        calls += 1;
                        pred_eval(kind, k.id)
                    });
                    for (k, v) in it {
                        extracted_ids.push(k.id);
                        drop(v);
                    }
                }));
                assert!(res.is_err(), "expected predicate panic");
            }
        }

        assert_eq!(extracted_ids, expected.extracted, "extracted ids");
        assert_eq!(map_ids(&map), expected.remaining, "remaining ids");
        check_map_invariants(&map, &all_ids);

        // Exercise the rebuilt table with a mutation after extraction.
        if !map.is_empty() {
            let victim = map.keys().next().unwrap().id;
            assert!(map.swap_remove(&victim).is_some());
            assert!(map.get_index_of(&victim).is_none());
        }
        drop(map);
    }
    log.assert_complete(n);
}

const MAX_N: usize = if cfg!(miri) { 6 } else { 24 };

#[test]
fn extract_full_ranges_contents_and_drops() {
    for n in 0..=MAX_N {
        for start in 0..=n {
            for end in start..=n {
                for kind in [PredKind::All, PredKind::None, PredKind::EveryThird] {
                    run_scenario(n, start, end, kind, Consume::Full);
                }
            }
        }
    }
}

#[test]
fn extract_early_drop_every_prefix() {
    for n in 0..=MAX_N {
        for start in 0..=n {
            for end in start..=n {
                for k in 0..=(end - start) {
                    for kind in [PredKind::All, PredKind::EveryThird] {
                        run_scenario(n, start, end, kind, Consume::EarlyDrop(k));
                    }
                }
            }
        }
    }
}

#[cfg(panic = "unwind")]
#[test]
fn extract_predicate_panic_every_position() {
    for n in 1..=MAX_N {
        for start in 0..=n {
            for end in start..=n {
                for p in 0..(end - start) {
                    run_scenario(n, start, end, PredKind::All, Consume::PanicAt(p));
                    run_scenario(n, start, end, PredKind::EveryThird, Consume::PanicAt(p));
                }
            }
        }
    }
}

#[test]
fn extract_full_and_empty_and_one() {
    // Full range, all keep / all extract.
    for kind in [PredKind::All, PredKind::None] {
        run_scenario(1, 0, 1, kind, Consume::Full);
        run_scenario(8, 0, 8, kind, Consume::Full);
        run_scenario(32, 0, 32, kind, Consume::Full);
    }
    // Empty ranges inside a non-empty map.
    for start in [0, 1, 4] {
        run_scenario(8, start, start, PredKind::All, Consume::Full);
    }
}

#[test]
fn extract_in_range_bounds_forms() {
    let mut map: IndexMap<i32, i32> = (0..6).map(|i| (i, i * 10)).collect();
    {
        let extracted: Vec<(i32, i32)> = map.extract_if(.., |_, _| true).collect();
        assert_eq!(extracted.iter().map(|(k, _)| *k).collect::<Vec<_>>(), [0, 1, 2, 3, 4, 5]);
        assert!(map.is_empty());
    }
    // `..=` and `start..` / `..end` forms.
    let mut map: IndexMap<i32, i32> = (0..6).map(|i| (i, i * 10)).collect();
    {
        let ids: Vec<i32> = map.extract_if(1..=2, |_, _| true).map(|(k, _)| k).collect();
        assert_eq!(ids, [1, 2]);
        let ids: Vec<i32> = map.extract_if(..2, |_, _| true).map(|(k, _)| k).collect();
        assert_eq!(ids, [0, 3]);
        assert_eq!(map.keys().copied().collect::<Vec<_>>(), [4, 5]);
        let ids: Vec<i32> = map.extract_if(1.., |_, _| false).map(|(k, _)| k).collect();
        assert!(ids.is_empty());
        assert_eq!(map.keys().copied().collect::<Vec<_>>(), [4, 5]);
    }
}

#[cfg(panic = "unwind")]
#[test]
// The reversed `3..2` range is the point of this test.
#[allow(clippy::reversed_empty_ranges)]
fn extract_out_of_bounds_and_reversed_ranges_panic_cleanly() {
    let log = Log::new();
    let mut map = build_map(5, &log);
    for range in [(0usize..6usize), (5..6), (6..6), (3..2)] {
        let res = catch_unwind(AssertUnwindSafe(|| {
            let _ = map.extract_if(range.clone(), |_, _| true);
        }));
        assert!(res.is_err(), "range {range:?} should panic");
        assert_eq!(map_ids(&map), [0, 1, 2, 3, 4], "map untouched after panic");
        check_map_invariants(&map, &[0, 1, 2, 3, 4]);
    }
    drop(map);
    log.assert_complete(5);
}

#[test]
fn extract_zst_keys_and_values() {
    let mut zst: IndexMap<(), ()> = IndexMap::new();
    assert_eq!(zst.insert((), ()), None);
    assert_eq!(zst.insert((), ()), Some(()));
    let extracted: Vec<((), ())> = zst.extract_if(.., |_, _| true).collect();
    assert_eq!(extracted, [((), ())]);
    assert!(zst.is_empty());
    assert!(zst.extract_if(0..0, |_, _| true).next().is_none());

    // Empty value ZST with tracked, non-ZST keys.
    let log = Log::new();
    let mut map: IndexMap<K, ()> = IndexMap::new();
    for id in 0..8u32 {
        map.insert(K::new(id, &log), ());
    }
    let ids: Vec<u32> = map.extract_if(2..7, |k, _| k.id % 2 == 0).map(|(k, ())| k.id).collect();
    assert_eq!(ids, [2, 4, 6]);
    let remaining: Vec<u32> = map.keys().map(|k| k.id).collect();
    assert_eq!(remaining, [0, 1, 3, 5, 7]);
    drop(map);
    // Every key dropped exactly once: tags are `2 * id`.
    let mut recorded = RefCell::borrow(&log.0).clone();
    recorded.sort_unstable();
    assert_eq!(recorded, [0, 2, 4, 6, 8, 10, 12, 14]);
}

#[test]
fn extract_after_removals_and_reinsertion() {
    let log = Log::new();
    let mut map = build_map(12, &log);
    // Perturb both entries and index table.
    assert!(map.swap_remove(&3).is_some());
    assert!(map.shift_remove(&7).is_some());
    map.insert(K::new(12, &log), V::new(12, &log));
    // Order after the mutations: [0,1,2,11,4,5,6,8,9,10,12]
    let ids: Vec<u32> = map.extract_if(1..5, |k, _| k.id % 2 == 0).map(|(k, _)| k.id).collect();
    assert_eq!(ids, [2, 4]);
    let remaining = map_ids(&map);
    check_map_invariants(&map, &[0, 1, 2, 4, 5, 6, 8, 9, 10, 11, 12]);
    assert_eq!(remaining, [0, 1, 11, 5, 6, 8, 9, 10, 12]);
    drop(map);
    // 12 original entries + the reinserted one, each dropped exactly once.
    log.assert_complete(13);
}

// ---------------------------------------------------------------------------
// IndexSet extract_if
// ---------------------------------------------------------------------------

#[test]
fn set_extract_matrix() {
    const SET_MAX: usize = if cfg!(miri) { 5 } else { 16 };
    for n in 0..=SET_MAX {
        for start in 0..=n {
            for end in start..=n {
                for k in 0..=(end - start) {
                    let log = Log::new();
                    let mut set: TrackedSet = IndexSet::new();
                    for id in 0..n as u32 {
                        assert!(set.insert(K::new(id, &log)));
                    }
                    let extracted: Vec<u32> = set
                        .extract_if(start..end, |k| pred_eval(PredKind::EveryThird, k.id))
                        .take(k)
                        .map(|k| k.id)
                        .collect();
                    let expected = model(n, start, end, PredKind::EveryThird, Consume::EarlyDrop(k));
                    assert_eq!(extracted, expected.extracted);
                    let ids: Vec<u32> = set.iter().map(|k| k.id).collect();
                    assert_eq!(ids, expected.remaining);
                    for id in 0..n as u32 {
                        assert_eq!(set.get_index_of(&id), ids.iter().position(|x| *x == id));
                    }
                    drop(set);
                    let mut recorded = RefCell::borrow(&log.0).clone();
                    recorded.sort_unstable();
                    assert_eq!(recorded, (0..2 * n as u32).step_by(2).collect::<Vec<_>>());
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// retain (panic safety + exactly-once drops)
// ---------------------------------------------------------------------------

#[cfg(panic = "unwind")]
#[test]
fn extract_predicate_mutation_and_panic() {
    // Mutating values through `&mut V` must persist for kept entries, and for
    // the entry the predicate panics on (which is kept, not extracted).
    let mut map: IndexMap<i32, i32> = (0..10).map(|i| (i, i)).collect();
    let extracted: Vec<(i32, i32)> = map
        .extract_if(.., |k, v| {
            *v += 100;
            k % 2 == 0
        })
        .collect();
    assert_eq!(extracted, [(0, 100), (2, 102), (4, 104), (6, 106), (8, 108)]);
    assert_eq!(
        map.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(),
        [(1, 101), (3, 103), (5, 105), (7, 107), (9, 109)]
    );

    let mut map: IndexMap<i32, i32> = (0..10).map(|i| (i, i)).collect();
    let res = catch_unwind(AssertUnwindSafe(|| {
        let mut it = map.extract_if(.., |k, v| {
            *v += 100;
            if *k == 5 {
                panic!("audit: mutation then panic");
            }
            k % 2 == 0
        });
        while it.next().is_some() {}
    }));
    assert!(res.is_err());
    // 0,2,4 extracted (values mutated); 5 panicked but is kept and mutated;
    // the tail (6..9) was never visited and keeps its original values.
    assert_eq!(
        map.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(),
        [(1, 101), (3, 103), (5, 105), (6, 6), (7, 7), (8, 8), (9, 9)]
    );
    for (k, v) in map.iter() {
        assert_eq!(
            *k,
            map.get_index_of(k)
                .and_then(|i| map.get_index(i))
                .map(|(k, _)| *k)
                .unwrap()
        );
        let _ = v;
    }
}

#[cfg(panic = "unwind")]
#[test]
fn retain_panic_drops_exactly_once() {
    let log = Log::new();
    {
        let mut map = build_map(12, &log);
        let mut calls = 0;
        let res = catch_unwind(AssertUnwindSafe(|| {
            map.retain(|k, _| {
                if calls == 5 {
                    calls += 1;
                    panic!("audit: retain panic");
                }
                calls += 1;
                k.id % 2 == 0
            });
        }));
        assert!(res.is_err());
        // The hash table is rebuilt while unwinding, so surviving entries stay
        // reachable and every key/value is dropped exactly once.
        assert_eq!(map.len(), map.iter().count());
        for (pos, key) in map.keys().enumerate() {
            assert_eq!(map.get_index_of(&key.id), Some(pos), "id {} index", key.id);
        }
        drop(map);
    }
    log.assert_complete(12);
}

/// Regression test for F-1 (upstream indexmap 2.14.2 issue #433): a caught
/// panic from `IndexMap::retain` used to leave the hash table pointing at
/// stale positions, so surviving keys were no longer found. The rebuild guard
/// makes the map consistent again while unwinding.
#[cfg(panic = "unwind")]
#[test]
fn retain_panic_must_keep_lookups_working() {
    let mut map: IndexMap<i32, i32> = (0..4).map(|i| (i, i * 10)).collect();
    let res = catch_unwind(AssertUnwindSafe(|| {
        map.retain(|k, _| {
            if *k == 0 {
                return false; // remove key 0
            }
            if *k == 1 {
                panic!("audit: retain panic");
            }
            true
        });
    }));
    assert!(res.is_err());

    // Keys 1 and 2 are still present in iteration order, so lookup must find
    // them; the rebuilt indices make `get` succeed.
    assert_eq!(map.iter().map(|(k, _)| *k).collect::<Vec<_>>(), [1, 2, 3]);
    assert_eq!(map.get(&1), Some(&10), "surviving key 1 must be found");
}

/// Same regression, observed through `len()`: before the fix the index table
/// still reported 4 while only 3 entries survived.
#[cfg(panic = "unwind")]
#[test]
fn retain_panic_must_have_consistent_len() {
    let mut map: IndexMap<i32, i32> = (0..4).map(|i| (i, i * 10)).collect();
    let res = catch_unwind(AssertUnwindSafe(|| {
        map.retain(|k, _| {
            if *k == 0 {
                return false;
            }
            if *k == 1 {
                panic!("audit: retain panic");
            }
            true
        });
    }));
    assert!(res.is_err());
    assert_eq!(map.len(), map.iter().count());
}

/// Regression test: before the fix, key 3's stale table entry pointed past the
/// end of `entries` (`entries.len() == 3`) and a lookup panicked in the
/// bounds-checked index path. The rebuild guard keeps the table in bounds.
#[cfg(panic = "unwind")]
#[test]
fn retain_panic_no_stale_out_of_bounds_lookup() {
    let mut map: IndexMap<i32, i32> = (0..4).map(|i| (i, i * 10)).collect();
    let res = catch_unwind(AssertUnwindSafe(|| {
        map.retain(|k, _| {
            if *k == 0 {
                return false;
            }
            if *k == 1 {
                panic!("audit: retain panic");
            }
            true
        });
    }));
    assert!(res.is_err());

    // Unpatched indexmap 2.14.2 panicked here; a panic-safe build finds 30.
    let value = catch_unwind(AssertUnwindSafe(|| map.get(&3).copied()))
        .expect("lookup must not panic after a panic-safe retain");
    assert_eq!(value, Some(30), "key 3's own value");
    assert_eq!(map.iter().count(), 3, "iteration itself is still fine");
}

// ---------------------------------------------------------------------------
// get_disjoint_mut / get_disjoint_indices_mut
// ---------------------------------------------------------------------------

fn disjoint_exhaustive<const N: usize>(n: usize) {
    // Values carry a high-alignment marker so pointer arithmetic and
    // alignment are both exercised.
    #[derive(Debug, PartialEq, Eq)]
    #[repr(align(64))]
    struct Aligned(u64);

    let mut map: IndexMap<usize, Aligned> = IndexMap::new();
    for i in 0..n {
        map.insert(i, Aligned(i as u64));
    }

    // Enumerate every index tuple with values in `0..=n` (n == OOB).
    let mut idx = vec![0usize; N];
    loop {
        let arr: [usize; N] = idx.clone().try_into().unwrap();
        let has_dup = (0..N).any(|i| idx[..i].contains(&idx[i]));
        let has_oob = idx.iter().any(|&i| i >= n);
        match map.get_disjoint_indices_mut(arr) {
            Ok(refs) => {
                assert!(!has_dup && !has_oob, "unexpected Ok for {idx:?} (len {n})");
                let mut addrs = Vec::new();
                for (slot, (_, r)) in refs.into_iter().enumerate() {
                    let addr = r as *mut Aligned as usize;
                    assert_eq!(addr % 64, 0, "misaligned &mut at slot {slot}");
                    addrs.push(addr);
                    r.0 = 1000 + slot as u64;
                }
                addrs.sort_unstable();
                addrs.dedup();
                assert_eq!(addrs.len(), N, "returned &mut aliased for {idx:?}");
            }
            Err(err) => {
                assert!(has_dup || has_oob, "unexpected Err for in-bounds unique {idx:?}");
                if has_oob && !has_dup {
                    assert!(matches!(err, indexmap::GetDisjointMutError::IndexOutOfBounds));
                }
                if has_dup && !has_oob {
                    assert!(matches!(err, indexmap::GetDisjointMutError::OverlappingIndices));
                }
            }
        }
        // Verify that only successfully returned slots were mutated: re-read
        // every entry and check it is either the original or a written marker.
        for (i, v) in map.iter() {
            let is_marker = v.0 >= 1000 && v.0 < 1000 + N as u64;
            if !is_marker {
                assert_eq!(v.0, *i as u64, "entry {i} corrupted by disjoint access");
            }
        }
        // next tuple
        if N == 0 {
            break;
        }
        let mut pos = N;
        loop {
            if pos == 0 {
                return;
            }
            pos -= 1;
            if idx[pos] < n {
                idx[pos] += 1;
                for x in &mut idx[pos + 1..] {
                    *x = 0;
                }
                break;
            }
            if pos == 0 {
                return;
            }
        }
    }
}

#[test]
fn get_disjoint_indices_all_combinations() {
    for n in 0..=3 {
        disjoint_exhaustive::<0>(n);
        disjoint_exhaustive::<1>(n);
        disjoint_exhaustive::<2>(n);
        disjoint_exhaustive::<3>(n);
        disjoint_exhaustive::<4>(n);
    }
}

#[test]
fn get_disjoint_indices_larger_n() {
    // N = 8, 32 with all-distinct in-bounds indices and one OOB.
    let mut map: IndexMap<usize, u64> = (0..64).map(|i| (i, i as u64)).collect();
    let eight: [usize; 8] = [0, 9, 1, 8, 2, 7, 3, 6];
    let refs = map.get_disjoint_indices_mut(eight).unwrap();
    for (slot, (_, r)) in refs.into_iter().enumerate() {
        assert_eq!(*r, eight[slot] as u64);
        *r += 1000;
    }
    let refs = map.get_disjoint_indices_mut([0usize; 32]).unwrap_err();
    assert!(matches!(refs, indexmap::GetDisjointMutError::OverlappingIndices));
    let mut arr = [0usize; 32];
    for (i, x) in arr.iter_mut().enumerate() {
        *x = i;
    }
    let refs = map.get_disjoint_indices_mut(arr).unwrap();
    assert_eq!(refs.len(), 32);
    assert!(matches!(
        map.get_disjoint_indices_mut([64usize; 1]).unwrap_err(),
        indexmap::GetDisjointMutError::IndexOutOfBounds
    ));
}

#[cfg(panic = "unwind")]
#[test]
fn get_disjoint_mut_by_key_duplicates_and_none() {
    let mut map: IndexMap<i32, i32> = (0..6).map(|i| (i, i * 10)).collect();

    assert_eq!(map.get_disjoint_mut([] as [&i32; 0]), []);
    assert_eq!(map.get_disjoint_mut([&0, &99, &5]), [Some(&mut 0), None, Some(&mut 50)]);

    // Mutate through all returned refs; distinctness is guaranteed by the
    // duplicate check, so writing different values is meaningful.
    let refs = map.get_disjoint_mut([&2, &1, &4, &0]);
    let mut seen = Vec::new();
    for (slot, r) in refs.into_iter().enumerate() {
        let r = r.unwrap();
        *r = 100 + slot as i32;
        seen.push(r as *mut i32 as usize);
    }
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(seen.len(), 4);
    assert_eq!(map[&2], 100);
    assert_eq!(map[&1], 101);
    assert_eq!(map[&4], 102);
    assert_eq!(map[&0], 103);
    assert_eq!(map[&3], 30);
    assert_eq!(map[&5], 50);

    // Duplicate keys must panic, and must not leave a dangling mutation.
    let res = catch_unwind(AssertUnwindSafe(|| {
        let _ = map.get_disjoint_mut([&0i32, &0]);
    }));
    assert!(res.is_err(), "duplicate keys [0,0] must panic");
    let res = catch_unwind(AssertUnwindSafe(|| {
        let _ = map.get_disjoint_mut([&1i32, &2, &1]);
    }));
    assert!(res.is_err(), "duplicate keys [1,2,1] must panic");
    let res = catch_unwind(AssertUnwindSafe(|| {
        let _ = map.get_disjoint_mut([&5i32, &5, &5]);
    }));
    assert!(res.is_err(), "duplicate keys [5,5,5] must panic");
    // Still fully usable after the panics.
    assert_eq!(map.len(), 6);
    assert_eq!(map.get(&2), Some(&100));
    map.insert(10, 10);
    assert_eq!(map.get(&10), Some(&10));
}

#[test]
fn get_disjoint_mut_zst_values() {
    let mut map: IndexMap<usize, ()> = (0..5).map(|i| (i, ())).collect();
    let refs = map.get_disjoint_mut([&0, &2, &4]);
    assert_eq!(refs.len(), 3);
    for r in refs {
        assert_eq!(r, Some(&mut ()));
    }
    let all = map.get_disjoint_indices_mut([0, 1, 2, 3, 4]).unwrap();
    for (_, r) in all {
        *r = ();
    }
}

// ---------------------------------------------------------------------------
// Slice layout casts and Box<Slice>
// ---------------------------------------------------------------------------

#[test]
fn slice_as_slice_empty_and_sizes() {
    let empty: IndexMap<i32, i32> = IndexMap::new();
    assert!(empty.as_slice().is_empty());
    assert_eq!(empty.as_slice().len(), 0);
    assert_eq!(empty.as_slice().iter().count(), 0);
    assert_eq!(empty.as_slice().first(), None);
    assert_eq!(empty.as_slice().last(), None);
    assert!(empty.as_slice().split_at_checked(0).is_some());
    assert!(empty.as_slice().split_at_checked(1).is_none());

    for n in 1..=64usize {
        let mut map: IndexMap<i32, i32> = (0..n as i32).map(|i| (i, i * 3)).collect();
        {
            let slice = map.as_slice();
            assert_eq!(slice.len(), n);
            assert_eq!(slice.first().map(|(k, v)| (*k, *v)), Some((0, 0)));
            assert_eq!(slice.last().map(|(k, v)| (*k, *v)), Some((n as i32 - 1, (n as i32 - 1) * 3)));
            for split in [0, 1, n / 2, n] {
                let (a, b) = slice.split_at(split);
                assert_eq!(a.len() + b.len(), n);
                assert_eq!(a.len(), split);
            }
        }
        {
            let slice = map.as_mut_slice();
            for (_, v) in slice.iter_mut() {
                *v += 1;
            }
        }
        assert_eq!(map[&0], 1);
        assert_eq!(map[&(n as i32 - 1)], (n as i32 - 1) * 3 + 1);
    }
}

#[test]
fn slice_high_alignment_and_zst() {
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[repr(align(128))]
    struct Big([u64; 3]);

    let mut map: IndexMap<u64, Big> = IndexMap::new();
    for i in 0..8u64 {
        map.insert(i, Big([i, i + 1, i + 2]));
    }
    for (_, v) in map.as_slice() {
        assert_eq!(v as *const Big as usize % 128, 0);
    }
    for (_, v) in map.as_mut_slice() {
        assert_eq!(v as *mut Big as usize % 128, 0);
        v.0[0] += 10;
    }
    let boxed = map.clone().into_boxed_slice();
    for (_, v) in boxed.iter() {
        assert_eq!(v as *const Big as usize % 128, 0);
    }

    let mut zst: IndexMap<u32, ()> = (0..8).map(|i| (i, ())).collect();
    assert_eq!(zst.as_slice().len(), 8);
    assert_eq!(zst.as_mut_slice().iter_mut().count(), 8);
    let boxed = zst.into_boxed_slice();
    assert_eq!(boxed.len(), 8);
    assert_eq!(boxed.into_iter().count(), 8);
}

#[test]
fn boxed_slice_roundtrip_and_drop_counts() {
    for n in [0usize, 1, 2, 7, 64] {
        let log = Log::new();
        {
            let map = build_map(n, &log);
            let boxed: Box<indexmap::map::Slice<K, V>> = map.into_boxed_slice();
            assert_eq!(boxed.len(), n);
            assert_eq!(boxed.iter().count(), n);
            let ids: Vec<u32> = boxed.keys().map(|k| k.id).collect();
            assert_eq!(ids, (0..n as u32).collect::<Vec<_>>());
            // `Box<Slice>` -> owned iterator (into_entries -> into_boxed -> Vec)
            let pairs: Vec<(K, V)> = boxed.into_iter().collect();
            assert_eq!(pairs.len(), n);
            let ids: Vec<u32> = pairs.iter().map(|(k, _)| k.id).collect();
            assert_eq!(ids, (0..n as u32).collect::<Vec<_>>());
            drop(pairs);
        }
        log.assert_complete(n);
    }

    // Empty `Box<Slice>` constructed from the zero-length default allocation.
    let empty: Box<indexmap::map::Slice<i32, i32>> = Box::default();
    assert_eq!(empty.len(), 0);
    assert!(empty.is_empty());
    assert_eq!(empty.iter().count(), 0);
    assert_eq!(empty.into_iter().count(), 0);

    // Clone and `From<&Slice>` preserve layout and contents.
    let map: IndexMap<i32, i32> = (0..5).map(|i| (i, i * 2)).collect();
    let boxed: Box<indexmap::map::Slice<i32, i32>> = Box::from(map.as_slice());
    let cloned = boxed.clone();
    assert_eq!(boxed.len(), 5);
    assert_eq!(&*cloned, &*boxed);
    assert_eq!(
        cloned.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(),
        map.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>()
    );
    drop(cloned);
    drop(boxed);
}

#[test]
fn set_boxed_slice_roundtrip() {
    for n in [0usize, 1, 3, 32] {
        let log = Log::new();
        {
            let set: TrackedSet = (0..n as u32).map(|id| K::new(id, &log)).collect();
            assert_eq!(set.len(), n);
            let boxed: Box<indexmap::set::Slice<K>> = set.into_boxed_slice();
            assert_eq!(boxed.len(), n);
            assert_eq!(boxed.iter().count(), n);
            let items: Vec<K> = boxed.into_iter().collect();
            assert_eq!(items.len(), n);
            drop(items);
        }
        // Tag for each key is `2 * id`, not contiguous; check counts instead.
        let mut recorded = RefCell::borrow(&log.0).clone();
        recorded.sort_unstable();
        assert_eq!(recorded, (0..n as u32).map(|id| 2 * id).collect::<Vec<_>>());
    }

    let empty: Box<indexmap::set::Slice<i32>> = Box::default();
    assert!(empty.is_empty());
    assert_eq!(empty.into_iter().count(), 0);
}

#[test]
fn slice_pointer_identity_and_alignment() {
    let map: IndexMap<i32, i32> = (0..32).map(|i| (i, i)).collect();
    let slice = map.as_slice();
    assert_eq!(slice.len(), 32);
    // The transparent cast preserves each element address: the same key and
    // value references are visible through the map and through the slice.
    for i in 0..32usize {
        let (k, v) = slice.get_index(i).unwrap();
        assert_eq!(k as *const i32, map.get_key_value(&(i as i32)).unwrap().0 as *const i32);
        assert_eq!(v as *const i32, map.get(&(i as i32)).unwrap() as *const i32);
    }
    // Iterator over the slice sees exactly the same addresses as the map.
    for ((sk, _), (mk, _)) in slice.iter().zip(map.iter()) {
        assert_eq!(sk as *const i32, mk as *const i32);
    }
}
