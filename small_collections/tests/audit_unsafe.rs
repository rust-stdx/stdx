//! Adversarial tests for the unsafe-bearing parts of `small_collections`:
//!
//! * `bytes::BufMut for SmallVec<u8, N>` (`advance_mut` -> `set_len`,
//!   `chunk_mut` -> `UninitSlice::from_raw_parts_mut`, inline <-> heap
//!   transitions), cross-checked byte-for-byte against `Vec<u8>`;
//! * `SmallString::as_bytes_mut` and the UTF-8 invariant of the safe
//!   `String`-like API at char and capacity boundaries.
//!
//! Run with Miri:
//! `cargo +nightly miri test -p small_collections --all-features --test audit_unsafe`

use small_collections::{SmallString, SmallVec};

/// Small deterministic LCG so failures are reproducible without extra deps.
#[cfg(feature = "bytes")]
struct Lcg(u64);

#[cfg(feature = "bytes")]
impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg(seed.wrapping_mul(6364136223846793005).wrapping_add(1))
    }

    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }

    fn below(&mut self, n: u64) -> u64 {
        if n == 0 { 0 } else { self.next() % n }
    }
}

// ===========================================================================
// bytes::BufMut
// ===========================================================================

#[cfg(feature = "bytes")]
mod bufmut {
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use bytes::{Buf, BufMut, Bytes};
    use small_collections::SmallVec;

    use super::Lcg;

    fn check<const N: usize>(v: &SmallVec<u8, N>, model: &[u8], what: &str) {
        assert_eq!(v.as_slice(), model, "{what}: contents");
        assert_eq!(v.len(), model.len(), "{what}: len");
        assert!(v.len() <= v.capacity(), "{what}: len {} > capacity {}", v.len(), v.capacity());
        assert!(
            v.remaining_mut() >= v.capacity().saturating_sub(v.len()),
            "{what}: remaining_mut contract"
        );
    }

    #[test]
    fn put_slice_growth_matches_vec() {
        fn one<const N: usize>() {
            for len in 0..=40usize {
                let model: Vec<u8> = (0..len as u8).collect();
                let mut v: SmallVec<u8, N> = SmallVec::new();
                for chunk in model.chunks(3) {
                    v.put_slice(chunk);
                }
                check::<N>(&v, &model, &format!("N={N} len={len}"));
                assert_eq!(v.is_inline(), len <= N, "N={N} len={len} storage");
            }
        }
        one::<0>();
        one::<1>();
        one::<2>();
        one::<3>();
        one::<4>();
        one::<8>();
        one::<16>();
        one::<64>();
    }

    #[test]
    fn mixed_bufmut_ops_match_vec_model() {
        let mut rng = Lcg::new(0xA11CE);
        fn one<const N: usize>(rng: &mut Lcg) {
            let mut v: SmallVec<u8, N> = SmallVec::new();
            let mut model: Vec<u8> = Vec::new();
            for step in 0..200 {
                match rng.below(10) {
                    0 => {
                        let b = rng.next() as u8;
                        v.put_u8(b);
                        model.push(b);
                    }
                    1 => {
                        // Multi-chunk `put` (uses chunk_mut/advance_mut).
                        let a = rng.next() as u8;
                        let b = rng.next() as u8;
                        let src = Bytes::from(vec![a, b]).chain(Bytes::from_static(&[0xAA, 0xBB]));
                        v.put(src);
                        model.extend_from_slice(&[a, b, 0xAA, 0xBB]);
                    }
                    2 | 3 => {
                        let len = rng.below(9) as usize;
                        let bytes: Vec<u8> = (0..len).map(|i| (i as u8) ^ (step as u8)).collect();
                        v.put_slice(&bytes);
                        model.extend_from_slice(&bytes);
                    }
                    4 => {
                        let cnt = rng.below(12) as usize;
                        let val = rng.next() as u8;
                        v.put_bytes(val, cnt);
                        model.extend(std::iter::repeat_n(val, cnt));
                    }
                    5 => {
                        let x = rng.next() as u16;
                        v.put_u16(x);
                        model.extend_from_slice(&x.to_be_bytes());
                    }
                    6 => {
                        let x = rng.next() as u32;
                        v.put_u32(x);
                        model.extend_from_slice(&x.to_be_bytes());
                    }
                    7 => {
                        let cap_before = v.capacity();
                        let len_before = v.len();
                        let want = rng.below(9) as usize;
                        let byte = rng.next() as u8;
                        let m = {
                            let chunk = v.chunk_mut();
                            assert!(chunk.len() != 0, "chunk_mut must be non-empty");
                            assert_eq!(
                                chunk.len(),
                                cap_before - len_before,
                                "N={N}: chunk must cover the spare capacity"
                            );
                            let m = want.min(chunk.len());
                            for i in 0..m {
                                chunk.write_byte(i, byte);
                            }
                            m
                        };
                        // SAFETY: exactly `m` bytes of the chunk were initialized.
                        unsafe { v.advance_mut(m) };
                        model.extend(std::iter::repeat_n(byte, m));
                    }
                    8 => {
                        let new_len = rng.below(model.len() as u64 + 1) as usize;
                        v.truncate(new_len);
                        model.truncate(new_len);
                    }
                    9 => {
                        if rng.below(8) == 0 {
                            v.clear();
                            model.clear();
                        } else {
                            let extra = rng.below(20) as usize;
                            v.reserve(extra);
                            assert!(v.capacity() >= v.len() + extra, "reserve contract");
                        }
                    }
                    _ => unreachable!(),
                }
                check::<N>(&v, &model, &format!("N={N} step={step}"));
            }
        }
        one::<0>(&mut rng);
        one::<1>(&mut rng);
        one::<2>(&mut rng);
        one::<3>(&mut rng);
        one::<4>(&mut rng);
        one::<8>(&mut rng);
        one::<16>(&mut rng);
        one::<64>(&mut rng);
    }

    #[test]
    fn chunk_mut_then_partial_advance() {
        fn one<const N: usize>() {
            for initial in 0..=(N + 9) {
                let bytes: Vec<u8> = (0..initial).map(|i| i as u8).collect();
                let mut v: SmallVec<u8, N> = SmallVec::new();
                v.put_slice(&bytes);
                // `usize::MAX` means "the whole chunk"; other values are
                // clamped to the chunk length.
                for want in [0usize, 1, 3, usize::MAX] {
                    let mut v2 = v.clone();
                    let mut model = bytes.clone();
                    let m = {
                        let chunk = v2.chunk_mut();
                        assert!(chunk.len() != 0, "N={N} initial={initial}");
                        let m = want.min(chunk.len());
                        for i in 0..m {
                            chunk.write_byte(i, 0xC0 | (i as u8 & 0x3F));
                        }
                        m
                    };
                    // SAFETY: exactly `m` bytes of the chunk were initialized.
                    unsafe { v2.advance_mut(m) };
                    for i in 0..m {
                        model.push(0xC0 | (i as u8 & 0x3F));
                    }
                    check::<N>(&v2, &model, &format!("N={N} initial={initial} m={m}"));
                }
            }
        }
        one::<0>();
        one::<1>();
        one::<2>();
        one::<3>();
        one::<4>();
        one::<8>();
        one::<16>();
        one::<64>();
    }

    #[test]
    fn advance_mut_past_capacity_panics_without_mutation() {
        fn one<const N: usize>() {
            for initial in 0..=(N + 5) {
                let bytes: Vec<u8> = (0..initial).map(|i| (i as u8).wrapping_add(1)).collect();
                let mut v: SmallVec<u8, N> = SmallVec::new();
                v.put_slice(&bytes);
                let before = v.as_slice().to_vec();
                let len_before = v.len();
                let spare = v.capacity() - v.len();
                // Deliberately violate the `BufMut` precondition by one byte;
                // the implementation must reject it before `set_len`.
                let res = catch_unwind(AssertUnwindSafe(|| unsafe { v.advance_mut(spare + 1) }));
                assert!(res.is_err(), "N={N} initial={initial}: over-advance must panic");
                assert_eq!(v.len(), len_before, "len unchanged after rejected advance");
                assert_eq!(v.as_slice(), &before[..], "bytes unchanged after rejected advance");
                // Still usable.
                v.put_u8(0xEE);
                assert_eq!(v.last(), Some(&0xEE));
            }
        }
        one::<0>();
        one::<1>();
        one::<4>();
        one::<8>();
    }

    #[test]
    fn put_bytes_edges() {
        fn one<const N: usize>() {
            for cnt in [0usize, 1, N, N + 1, 2 * N + 3, 100] {
                let mut v: SmallVec<u8, N> = SmallVec::new();
                v.put_bytes(0x5A, cnt);
                assert_eq!(v.len(), cnt, "N={N} cnt={cnt}");
                assert!(v.iter().all(|&b| b == 0x5A), "N={N} cnt={cnt}");
                assert_eq!(v.is_inline(), cnt <= N, "N={N} cnt={cnt} storage");
                v.put_bytes(0xA5, 3);
                assert_eq!(&v[..cnt], &vec![0x5A; cnt][..]);
                assert_eq!(&v[cnt..], &[0xA5, 0xA5, 0xA5]);
            }
        }
        one::<0>();
        one::<1>();
        one::<2>();
        one::<4>();
        one::<8>();
        one::<16>();
    }

    #[test]
    fn chunk_mut_on_full_spills_and_preserves_bytes() {
        // Inline, exactly full.
        let mut v: SmallVec<u8, 4> = SmallVec::new();
        v.put_slice(b"abcd");
        assert!(v.is_inline());
        let cap_before = v.capacity();
        let (m, spare) = {
            let chunk = v.chunk_mut();
            assert!(chunk.len() != 0, "full inline vector must grow");
            let m = chunk.len().min(4);
            for i in 0..m {
                chunk.write_byte(i, b'x');
            }
            (m, chunk.len())
        };
        assert!(!v.is_inline(), "chunk_mut on a full inline vector spills");
        assert!(v.capacity() >= cap_before);
        assert!(spare > 0);
        // SAFETY: `m` bytes were initialized above.
        unsafe { v.advance_mut(m) };
        assert_eq!(&v[..4], b"abcd");
        assert_eq!(&v[4..], &vec![b'x'; m][..]);

        // Heap, exactly full.
        let mut v: SmallVec<u8, 4> = SmallVec::new();
        v.put_bytes(0x11, 13);
        assert!(!v.is_inline());
        let len = v.len();
        while v.len() < v.capacity() {
            v.put_u8(0x22);
        }
        assert_eq!(v.len(), v.capacity());
        let cap_before = v.capacity();
        let chunk_len = {
            let chunk = v.chunk_mut();
            assert!(chunk.len() != 0);
            chunk.len()
        };
        assert!(v.capacity() > cap_before);
        assert!(chunk_len > 0);
        assert_eq!(&v[..len], &vec![0x11; len][..]);
    }

    #[test]
    fn put_u32_put_u64_layout() {
        let mut v: SmallVec<u8, 8> = SmallVec::new();
        v.put_u32(0x0102_0304);
        v.put_u64(0x0102_0304_0506_0708);
        let mut model = Vec::new();
        model.extend_from_slice(&0x0102_0304u32.to_be_bytes());
        model.extend_from_slice(&0x0102_0304_0506_0708u64.to_be_bytes());
        assert_eq!(v.as_slice(), &model[..]);
        assert!(!v.is_inline());
    }

    #[test]
    fn put_reader_roundtrip() {
        let mut v: SmallVec<u8, 3> = SmallVec::new();
        v.put_slice(b"abcdef");
        let mut src = Bytes::from(v.as_slice().to_vec());
        let mut out = Vec::new();
        while src.has_remaining() {
            let chunk = src.chunk();
            let len = chunk.len();
            out.extend_from_slice(chunk);
            src.advance(len);
        }
        assert_eq!(out, b"abcdef");
        assert_eq!(v.as_slice(), b"abcdef");
    }
}

// ===========================================================================
// SmallString: UTF-8 invariant and boundary handling
// ===========================================================================

const STRING_SAMPLES: [&str; 7] = [
    "",
    "a",
    "é", // 2 bytes
    "€", // 3 bytes
    "𝄞", // 4 bytes
    "aé€𝄞z",
    "hello world, this is a longer ascii string",
];

fn string_push_chars<const N: usize>(s: &str) {
    let mut v: SmallString<N> = SmallString::new();
    let mut expected = String::new();
    let mut ever_spilled = false;
    for ch in s.chars() {
        v.push(ch);
        expected.push(ch);
        assert_eq!(v.as_str(), expected.as_str(), "N={N} sample={s:?}");
        assert_eq!(v.len(), expected.len());
        ever_spilled |= expected.len() > N;
        assert_eq!(
            v.is_inline(),
            !ever_spilled,
            "N={N} sample={s:?} after {ch:?}, len {}",
            expected.len()
        );
    }
    assert_eq!(v.as_str(), s, "N={N} sample={s:?}");
    if !ever_spilled {
        assert_eq!(v.capacity(), N);
    }
    // Every char boundary is a valid truncation point.
    let full = v.clone();
    let mut acc = 0;
    for ch in s.chars() {
        let mut t = full.clone();
        t.truncate(acc);
        assert_eq!(t.as_str(), &s[..acc], "N={N} truncate to {acc}");
        acc += ch.len_utf8();
    }
    let mut t = full.clone();
    t.truncate(acc);
    assert_eq!(t.as_str(), s, "N={N} truncate to full length");
    v.clear();
    assert!(v.is_empty());
}

#[test]
fn small_string_push_char_at_capacity_boundaries() {
    for &s in &STRING_SAMPLES {
        string_push_chars::<0>(s);
        string_push_chars::<1>(s);
        string_push_chars::<2>(s);
        string_push_chars::<3>(s);
        string_push_chars::<4>(s);
        string_push_chars::<5>(s);
        string_push_chars::<6>(s);
        string_push_chars::<7>(s);
        string_push_chars::<8>(s);
        string_push_chars::<16>(s);
        string_push_chars::<64>(s);
    }
}

fn string_push_str_insert<const N: usize>() {
    let mut v: SmallString<N> = SmallString::new();
    for chunk in ["a", "é", "€", "𝄞", "bc", "xyz", "0123456789"] {
        v.push_str(chunk);
        assert_eq!(v.as_str().len(), v.len());
        assert!(std::str::from_utf8(v.as_bytes()).is_ok());
    }
    let expected: String = ["a", "é", "€", "𝄞", "bc", "xyz", "0123456789"].concat();
    assert_eq!(v.as_str(), expected, "N={N}");

    // `push_str` keeps UTF-8 valid across every capacity boundary; `pop`
    // reverses it one char at a time.
    let mut remaining = expected.chars().rev();
    while let Some(ch) = v.pop() {
        assert_eq!(Some(ch), remaining.next(), "N={N} pop order");
        assert!(std::str::from_utf8(v.as_bytes()).is_ok());
    }
    assert_eq!(remaining.next(), None, "N={N}: all chars popped");
    assert!(v.is_empty());
}

#[test]
fn small_string_push_str_and_insert() {
    string_push_str_insert::<0>();
    string_push_str_insert::<1>();
    string_push_str_insert::<2>();
    string_push_str_insert::<3>();
    string_push_str_insert::<4>();
    string_push_str_insert::<5>();
    string_push_str_insert::<7>();
    string_push_str_insert::<8>();
    string_push_str_insert::<16>();
}

fn string_truncate_non_boundary<const N: usize>() {
    let mut v: SmallString<N> = SmallString::new();
    v.push_str("aé");
    assert_eq!(v.len(), 3);
    assert_eq!(v.is_inline(), N >= 3);
    let before = v.as_str().to_owned();
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        v.truncate(2); // inside the 2-byte 'é'
    }));
    assert!(res.is_err(), "truncate at non-char boundary must panic (N={N})");
    assert_eq!(v.as_str(), before, "string unchanged after panicking truncate");
}

#[test]
fn small_string_truncate_non_boundary_panics_cleanly() {
    // Inline and heap variants.
    string_truncate_non_boundary::<16>();
    string_truncate_non_boundary::<2>();
}

fn string_bytes_mut<const N: usize>() {
    // ASCII in-place mutation keeps the UTF-8 invariant.
    let mut v: SmallString<N> = SmallString::from_str("hello");
    assert_eq!(v.is_inline(), N >= 5);
    {
        // SAFETY: replacing ASCII bytes with ASCII bytes preserves UTF-8.
        let bytes = unsafe { v.as_bytes_mut() };
        for b in bytes.iter_mut() {
            *b = b.to_ascii_uppercase();
        }
    }
    assert_eq!(v.as_str(), "HELLO");
    // Mutating part of a multi-byte-containing string while preserving
    // validity is allowed; here we only touch the ASCII separator.
    let mut v: SmallString<N> = SmallString::from_str("é a é");
    {
        // SAFETY: the space (1 byte) is replaced by '_' (1 byte), so the
        // multi-byte sequences and total UTF-8 remain valid.
        let bytes = unsafe { v.as_bytes_mut() };
        for b in bytes.iter_mut() {
            if *b == b' ' {
                *b = b'_';
            }
        }
    }
    assert_eq!(v.as_str(), "é_a_é");
    assert!(std::str::from_utf8(v.as_bytes()).is_ok());
}

#[test]
fn small_string_as_bytes_mut_documented_contract() {
    string_bytes_mut::<16>();
    string_bytes_mut::<2>();
}

#[test]
fn small_string_lossy_and_utf8_validation() {
    let valid = "hello é € 𝄞";
    assert_eq!(
        SmallString::<8>::from_utf8_lossy(valid.as_bytes()).as_str(),
        String::from_utf8_lossy(valid.as_bytes())
    );
    let invalid: &[u8] = &[0xF0, 0x28, 0x8C, 0x28, b'a', 0xFF, 0xFE];
    assert_eq!(
        SmallString::<8>::from_utf8_lossy(invalid).as_str(),
        String::from_utf8_lossy(invalid)
    );
    assert!(SmallString::<8>::from_utf8_slice(invalid).is_err());
    assert!(SmallString::<8>::from_utf8(invalid.to_vec()).is_err());
    assert_eq!(SmallString::<8>::from_utf8(valid.as_bytes().to_vec()).unwrap().as_str(), valid);
}

#[test]
fn small_string_shrink_moves_back_inline() {
    let mut v: SmallString<8> = SmallString::from_str("hello world");
    assert!(!v.is_inline());
    v.truncate(5);
    v.shrink_to_fit();
    assert!(v.is_inline());
    assert_eq!(v.as_str(), "hello");
    v.push_str("!!!");
    assert!(v.is_inline());
    v.push_str("!!!!");
    assert!(!v.is_inline());
    assert_eq!(v.as_str(), "hello!!!!!!!");
}

// ===========================================================================
// SmallVec non-BufMut edge cases: ZST / capacity / transitions
// ===========================================================================

#[test]
fn small_vec_zst_operations() {
    fn one<const N: usize>() {
        let mut v: SmallVec<(), N> = SmallVec::new();
        for _ in 0..(N + 5) {
            v.push(());
        }
        assert_eq!(v.len(), N + 5);
        assert_eq!(v.iter().count(), N + 5);
        assert_eq!(v.pop(), Some(()));
        v.truncate(N);
        assert_eq!(v.len(), N);
        let drained: Vec<()> = v.drain(..).collect();
        assert_eq!(drained.len(), N);
        assert!(v.is_empty());
        v.extend(std::iter::repeat_n((), N + 3));
        let collected: Vec<()> = v.into_iter().collect();
        assert_eq!(collected.len(), N + 3);
    }
    one::<0>();
    one::<1>();
    one::<4>();
    one::<8>();
}

#[test]
fn small_vec_inline_heap_transitions() {
    let mut v: SmallVec<u32, 4> = SmallVec::new();
    for i in 0..20u32 {
        v.push(i);
        assert_eq!(v.len(), i as usize + 1);
        assert!(v.len() <= v.capacity());
        assert_eq!(v.is_inline(), v.len() <= 4);
    }
    assert_eq!(&v[..], &(0..20).collect::<Vec<_>>()[..]);
    // Shrinking below N moves back inline and preserves the prefix.
    v.truncate(4);
    v.shrink_to_fit();
    assert!(v.is_inline());
    assert_eq!(v.capacity(), 4);
    assert_eq!(&v[..], &[0, 1, 2, 3]);
    v.reserve(100);
    assert!(!v.is_inline());
    assert!(v.capacity() >= 104);
    assert_eq!(&v[..], &[0, 1, 2, 3]);
}
