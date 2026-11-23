//! Tests for `base64/serde.rs`.
//!
//! These tests exercise the serde helpers through a real human-readable
//! deserializer (`serde_json`), a token-based binary deserializer
//! (`serde_test`), and the public `FromEncoded` trait. They compare every
//! acceptance decision against the crate's direct `decode`/`encode` API.
//!
//! Run with:
//! `cargo test -p base64 --features serde,alloc --test audit_serde`

#![allow(clippy::needless_range_loop)]
#![cfg(feature = "serde")]

use base64::FromEncoded;
#[cfg(feature = "alloc")]
use base64::{Alphabet, DecodeError, decode, encode};
use rand::{RngExt, SeedableRng, rngs::StdRng};
use serde_json::value::Serializer as JsonSerializer;
use serde_test::{Configure, Token, assert_de_tokens, assert_ser_tokens};

/// `Serialize` adapter that routes a byte slice through `base64::serialize`.
#[derive(Debug)]
struct Bin<'a>(&'a [u8]);

impl serde::Serialize for Bin<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        base64::serialize(self.0, serializer)
    }
}

/// `Deserialize` adapter that routes through `base64::deserialize`.
#[derive(Debug, PartialEq)]
struct OwnedVec(Vec<u8>);

impl<'de> serde::Deserialize<'de> for OwnedVec {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[cfg(feature = "alloc")]
        {
            base64::deserialize::<_, Vec<u8>>(deserializer).map(OwnedVec)
        }
        #[cfg(not(feature = "alloc"))]
        {
            let bytes = base64::deserialize::<_, [u8; 12]>(deserializer)?;
            Ok(OwnedVec(bytes.to_vec()))
        }
    }
}

#[cfg(feature = "alloc")]
const ALPHABETS: [Alphabet; 4] = [
    Alphabet::Standard,
    Alphabet::StandardNoPadding,
    Alphabet::Url,
    Alphabet::UrlNoPadding,
];

/// Deterministic seeded `StdRng` generator.
fn json_serialize(data: &[u8]) -> serde_json::Value {
    base64::serialize(data, JsonSerializer).expect("human-readable serialization failed")
}

/// Independent RFC 4648 standard padded encoder, used where the crate's
/// `encode` is unavailable (`alloc` disabled).
fn ref_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut acc: u32 = 0;
    let mut nbits = 0u32;
    for &byte in data {
        acc = (acc << 8) | u32::from(byte);
        nbits += 8;
        while nbits >= 6 {
            nbits -= 6;
            out.push(TABLE[((acc >> nbits) & 63) as usize] as char);
        }
        acc &= (1u32 << nbits) - 1;
    }
    if nbits > 0 {
        out.push(TABLE[((acc << (6 - nbits)) & 63) as usize] as char);
    }
    while !out.len().is_multiple_of(4) {
        out.push('=');
    }
    out
}

#[cfg(feature = "alloc")]
fn json_deserialize_vec(text: &str) -> Result<Vec<u8>, String> {
    let mut de = serde_json::Deserializer::from_str(text);
    let value = base64::deserialize::<_, Vec<u8>>(&mut de).map_err(|e| e.to_string())?;
    de.end().map_err(|e| e.to_string())?;
    Ok(value)
}

fn json_deserialize_array<const N: usize>(text: &str) -> Result<[u8; N], String> {
    let mut de = serde_json::Deserializer::from_str(text);
    let value = base64::deserialize::<_, [u8; N]>(&mut de).map_err(|e| e.to_string())?;
    de.end().map_err(|e| e.to_string())?;
    Ok(value)
}

#[test]
fn json_roundtrip_every_length_0_to_512() {
    let mut rng = StdRng::seed_from_u64(0x1234_5678_9ABC_DEF1);
    for len in 0..=512usize {
        let data = {
            let mut v = vec![0u8; len];
            rng.fill(&mut v);
            v
        };
        let value = json_serialize(&data);
        let expected = serde_json::Value::String(ref_encode(&data));
        assert_eq!(value, expected, "serialized form differs at len {len}");

        let text = serde_json::to_string(&value).unwrap();
        #[cfg(feature = "alloc")]
        assert_eq!(
            json_deserialize_vec(&text).unwrap(),
            data,
            "Vec round-trip differs at len {len}"
        );
        assert_eq!(
            json_deserialize_array::<0>(&text).err().is_some(),
            len != 0,
            "wrong array acceptance at len {len}"
        );
    }
}

#[test]
fn json_array_roundtrip_fixed_sizes() {
    // Fixed sizes around common block and stack boundaries.
    fn check<const N: usize>(rng: &mut StdRng) {
        let data: [u8; N] = core::array::from_fn(|_| rng.random::<u8>());
        let value = json_serialize(&data);
        assert_eq!(value, serde_json::Value::String(ref_encode(&data)));
        let text = serde_json::to_string(&value).unwrap();
        assert_eq!(json_deserialize_array::<N>(&text).unwrap(), data, "array N={N}");
        #[cfg(feature = "alloc")]
        assert_eq!(json_deserialize_vec(&text).unwrap(), data);
    }

    let mut rng = StdRng::seed_from_u64(0xDEAD_BEEF_CAFE_1234);
    check::<0>(&mut rng);
    check::<1>(&mut rng);
    check::<2>(&mut rng);
    check::<3>(&mut rng);
    check::<4>(&mut rng);
    check::<5>(&mut rng);
    check::<7>(&mut rng);
    check::<8>(&mut rng);
    check::<15>(&mut rng);
    check::<16>(&mut rng);
    check::<17>(&mut rng);
    check::<20>(&mut rng);
    check::<24>(&mut rng);
    check::<31>(&mut rng);
    check::<32>(&mut rng);
    check::<33>(&mut rng);
    check::<40>(&mut rng);
    check::<48>(&mut rng);
    check::<63>(&mut rng);
    check::<64>(&mut rng);
    check::<65>(&mut rng);
    check::<127>(&mut rng);
    check::<128>(&mut rng);
    check::<129>(&mut rng);
    check::<189>(&mut rng);
    check::<190>(&mut rng);
    check::<191>(&mut rng);
    check::<192>(&mut rng);
    check::<193>(&mut rng);
    check::<255>(&mut rng);
    check::<256>(&mut rng);
    check::<257>(&mut rng);
    check::<512>(&mut rng);
    check::<513>(&mut rng);
    check::<1024>(&mut rng);
}

#[cfg(feature = "alloc")]
#[test]
fn direct_roundtrip_all_alphabets_0_to_512() {
    // The serde helpers only wire `Alphabet::Standard`; the other alphabets
    // are checked directly to show the shared decoder is sound and to record
    // that serde cannot select them.
    let mut rng = StdRng::seed_from_u64(0x0BAD_F00D_1234_5678);
    for alphabet in ALPHABETS {
        for len in 0..=512usize {
            let data = {
                let mut v = vec![0u8; len];
                rng.fill(&mut v);
                v
            };
            let text = encode(&data, alphabet);
            let decoded = decode(text.as_bytes(), alphabet).unwrap();
            assert_eq!(decoded, data, "round-trip differs for {alphabet:?} at len {len}");
        }
    }
}

#[cfg(feature = "alloc")]
#[test]
fn from_encoded_matches_direct_decode_for_padded_standard() {
    // Base corpus: every representable short string plus padding variants.
    let corpus: &[&[u8]] = &[
        b"",
        b"=",
        b"==",
        b"===",
        b"====",
        b"A",
        b"AA",
        b"AAA",
        b"AAAA",
        b"AAAAA",
        b"Zg",
        b"Zg=",
        b"Zg==",
        b"Zg===",
        b"Zm8",
        b"Zm8=",
        b"Zm8==",
        b"Zm9v",
        b"Zm9vYg==",
        b"Zm9vYmE=",
        b"Zm9vYmFy",
        b"AA=A",
        b"A=AA",
        b"AA==AA==",
        b"AAAA====",
        b"Zh==", // non-canonical trailing bits
        b"Zm9=", // non-canonical trailing bits
        b"Zm9v\n",
        b" Zm9v",
        b"Zm 9v",
        b"Zm9v!",
        b"_-wgVQA=", // URL-safe alphabet must be rejected by Standard
        b"-_-_",
        b"____",
        b"",
    ];

    for &input in corpus {
        let direct = decode(input, Alphabet::Standard);
        let serde = <Vec<u8> as FromEncoded>::from_encoded(input);
        match (&direct, &serde) {
            (Ok(a), Ok(b)) => assert_eq!(a, b, "output differs for {input:?}"),
            (Err(_), Err(_)) => {}
            (a, b) => panic!("acceptance mismatch for {input:?}: direct={a:?} serde={b:?}"),
        }
    }
}

#[cfg(feature = "alloc")]
#[test]
fn from_encoded_matches_direct_decode_all_256_byte_mutations() {
    // Base strings around every branch in padding/trailing-bit validation.
    let bases: &[&[u8]] = &[b"", b"A", b"AA", b"AAA", b"AAAA", b"Zg==", b"Zm8=", b"Zm9vYmFy"];

    for base in bases {
        // Mutation at every position.
        for pos in 0..base.len() {
            for byte in 0..=255u8 {
                let mut mutated = base.to_vec();
                mutated[pos] = byte;
                check_serde_vs_direct(&mutated);
            }
        }
        // Insertion at every position.
        for pos in 0..=base.len() {
            for byte in 0..=255u8 {
                let mut mutated = base.to_vec();
                mutated.insert(pos, byte);
                check_serde_vs_direct(&mutated);
            }
        }
        // Deletion at every position.
        for pos in 0..base.len() {
            let mut mutated = base.to_vec();
            mutated.remove(pos);
            check_serde_vs_direct(&mutated);
        }
    }
}

#[cfg(feature = "alloc")]
fn check_serde_vs_direct(input: &[u8]) {
    let direct = decode(input, Alphabet::Standard);
    let serde = <Vec<u8> as FromEncoded>::from_encoded(input);
    match (&direct, &serde) {
        (Ok(a), Ok(b)) => assert_eq!(a, b, "output differs for {input:?}"),
        (Err(_), Err(_)) => {
            // A decode error must not be bypassable through an array target.
            macro_rules! reject {
                ($($n:literal),*) => {$(
                    assert!(
                        <[u8; $n] as FromEncoded>::from_encoded(input).is_err(),
                        "array[$n] accepted input that direct decode rejected: {input:?}"
                    );
                )*};
            }
            reject!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
        }
        (a, b) => panic!("acceptance mismatch for {input:?}: direct={a:?} serde={b:?}"),
    }

    // Array targets must agree with the decoded length, never truncate or
    // zero-pad behind the caller's back.
    if let Ok(expected) = &direct {
        let n = expected.len();
        macro_rules! check_array {
            ($($n:literal),*) => {$(
                let got = <[u8; $n] as FromEncoded>::from_encoded(input);
                if n == $n {
                    match got {
                        Ok(array) => assert_eq!(array.as_slice(), expected.as_slice(), "array[$n] for {input:?}"),
                        Err(err) => panic!("array[$n] rejected valid input {input:?}: {err:?}"),
                    }
                } else {
                    assert!(got.is_err(), "array[$n] accepted wrong length for {input:?}");
                }
            )*};
        }
        check_array!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    }
}

#[test]
fn deterministic_fuzz_no_panics_and_consistent_acceptance() {
    let mut rng = StdRng::seed_from_u64(0xFEED_FACE_1357_9BDF);
    for len in 0..=96usize {
        for _ in 0..64 {
            let input: Vec<u8> = (0..len).map(|_| rng.random::<u8>()).collect();
            #[cfg(feature = "alloc")]
            check_serde_vs_direct(&input);

            // Array probes at fixed sizes must never panic either.
            let _ = <[u8; 0] as FromEncoded>::from_encoded(&input);
            let _ = <[u8; 1] as FromEncoded>::from_encoded(&input);
            let _ = <[u8; 8] as FromEncoded>::from_encoded(&input);
            let _ = <[u8; 32] as FromEncoded>::from_encoded(&input);
        }
    }
}

/// Known bug, shared with the core decoder (unit E1): `Alphabet::Standard`
/// claims to expect padding but accepts unpadded input whose length leaves a
/// remainder of 2 or 3. Run with `--ignored` to reproduce.
#[cfg(feature = "alloc")]
#[test]
fn strict_standard_padding_rejects_unpadded_input() {
    assert!(
        <[u8; 1] as FromEncoded>::from_encoded(b"Zg").is_err(),
        "serde array accepted unpadded 1-byte form"
    );
    assert!(
        <Vec<u8> as FromEncoded>::from_encoded(b"Zg").is_err(),
        "serde Vec accepted unpadded 1-byte form"
    );
    assert!(decode(b"Zg", Alphabet::Standard).is_err(), "core accepted unpadded 1-byte form");
    assert!(
        decode(b"Zm8", Alphabet::Standard).is_err(),
        "core accepted unpadded 2-byte form"
    );
    assert!(
        decode(b"AAAAAA", Alphabet::Standard).is_err(),
        "core accepted unpadded 4-byte form"
    );
}

#[cfg(feature = "alloc")]
#[test]
fn long_unpadded_standard_is_rejected() {
    for len in 1..=256usize {
        let data: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(43)).collect();
        let mut encoded = encode(&data, Alphabet::Standard).into_bytes();
        while encoded.last() == Some(&b'=') {
            encoded.pop();
        }
        let result = <Vec<u8> as FromEncoded>::from_encoded(&encoded);
        if data.len().is_multiple_of(3) {
            assert_eq!(result.unwrap(), data, "len={len}");
        } else {
            assert!(result.is_err(), "len={len} accepted unpadded {encoded:?}");
        }
    }
}

#[cfg(feature = "alloc")]
#[test]
fn serde_uses_only_the_standard_alphabet() {
    // URL-safe characters are not valid Standard base64.
    let corpus: &[&[u8]] = &[b"_-wgVQA=", b"-_-_", b"____"];
    for &input in corpus {
        assert!(decode(input, Alphabet::Standard).is_err(), "accepted {input:?}");
        assert!(
            <Vec<u8> as FromEncoded>::from_encoded(input).is_err(),
            "serde accepted {input:?}"
        );
    }
    // The Standard special characters still decode.
    assert!(decode(b"++//", Alphabet::Standard).is_ok());
}

#[cfg(feature = "alloc")]
#[test]
fn malformed_padding_is_rejected() {
    // Padding, when present, is strictly validated. The specific error
    // variant differs: a total length not divisible by 4 is reported as
    // InvalidInputLength before the padding-count check runs.
    assert_eq!(decode(b"Zg=", Alphabet::Standard), Err(DecodeError::InvalidInputLength));
    assert_eq!(decode(b"Zg===", Alphabet::Standard), Err(DecodeError::InvalidInputLength));
    assert_eq!(decode(b"A===", Alphabet::Standard), Err(DecodeError::InvalidPadding));
    assert!(decode(b"Z", Alphabet::Standard).is_err());
    for input in [&b"=AAA"[..], b"AA=A", b"AAAA====", b"===="] {
        assert!(decode(input, Alphabet::Standard).is_err(), "accepted {input:?}");
        assert!(
            <[u8; 3] as FromEncoded>::from_encoded(input).is_err(),
            "serde accepted {input:?}"
        );
    }
}

#[cfg(feature = "alloc")]
#[test]
fn non_canonical_trailing_bits_are_rejected() {
    for input in [&b"Zh=="[..], b"Zm9=", b"Zh", b"Zm9"] {
        assert!(decode(input, Alphabet::Standard).is_err(), "accepted {input:?}");
        assert!(
            <Vec<u8> as FromEncoded>::from_encoded(input).is_err(),
            "serde accepted {input:?}"
        );
    }
}

#[test]
fn non_canonical_trailing_bits_are_rejected_arrays() {
    for input in [&b"Zh=="[..], b"Zh"] {
        assert!(
            <[u8; 1] as FromEncoded>::from_encoded(input).is_err(),
            "serde accepted {input:?}"
        );
    }
    for input in [&b"Zm9="[..], b"Zm9"] {
        assert!(
            <[u8; 2] as FromEncoded>::from_encoded(input).is_err(),
            "serde accepted {input:?}"
        );
    }
    assert_eq!(<[u8; 1] as FromEncoded>::from_encoded(b"Zg==").unwrap(), *b"f");
    assert_eq!(<[u8; 2] as FromEncoded>::from_encoded(b"Zm8=").unwrap(), *b"fo");
}

#[cfg(feature = "alloc")]
#[test]
fn json_rejects_invalid_and_odd_inputs() {
    for text in [
        "\"\"",
        "\"=\"",
        "\"==\"",
        "\"A\"",
        "\"Z\"",
        "\"Zg=\"",
        "\"Zg===\"",
        "\"Zm 9v\"",
        "\"Zm9v!\"",
        "\"Zm9v\\n\"",
        "\"===A\"",
    ] {
        let decoded_text: String = serde_json::from_str(text).unwrap();
        let direct = decode(decoded_text.as_bytes(), Alphabet::Standard);
        let serde = json_deserialize_vec(text);
        assert_eq!(
            serde.is_ok(),
            direct.is_ok(),
            "acceptance differs for {text}: {serde:?} vs {direct:?}"
        );
        if let (Ok(s), Ok(d)) = (serde, direct) {
            assert_eq!(s, d);
        }
    }
}

#[test]
fn exact_array_length_is_enforced() {
    // "Zm9v" decodes to 3 bytes; only [u8; 3] may accept it.
    let text = "\"Zm9v\"";
    assert_eq!(json_deserialize_array::<3>(text).unwrap(), *b"foo");
    assert!(json_deserialize_array::<0>(text).is_err());
    assert!(json_deserialize_array::<2>(text).is_err());
    assert!(json_deserialize_array::<4>(text).is_err());
    assert!(json_deserialize_array::<32>(text).is_err());
    assert!(<[u8; 3] as FromEncoded>::from_decoded(b"foo").is_ok());
    assert!(<[u8; 3] as FromEncoded>::from_decoded(b"fo").is_err());
    assert!(<[u8; 3] as FromEncoded>::from_decoded(b"food").is_err());
}

#[test]
fn binary_roundtrip_uses_raw_bytes() {
    let data = b"hello binary";
    assert_ser_tokens(&Bin(data).compact(), &[Token::Bytes(data)]);
    assert_de_tokens(&OwnedVec(data.to_vec()).compact(), &[Token::Bytes(data)]);

    // Borrowed bytes go through `from_borrowed`; owned containers copy.
    assert_de_tokens(&OwnedVec(data.to_vec()).compact(), &[Token::BorrowedBytes(data)]);

    // Large binary payloads must bypass the text encoder entirely.
    #[cfg(feature = "alloc")]
    {
        static LARGE: [u8; 4096] = [0xAB; 4096];
        assert_ser_tokens(&Bin(&LARGE).compact(), &[Token::Bytes(&LARGE)]);
        assert_de_tokens(&OwnedVec(LARGE.to_vec()).compact(), &[Token::Bytes(&LARGE)]);
    }
}

#[test]
fn human_readable_visitor_only_accepts_strings() {
    // `deserialize` asks for a string in human-readable mode; a raw-bytes
    // token makes serde_json fail, which is the expected contract.
    let value = json_serialize(b"abc");
    assert_eq!(value, serde_json::Value::String("YWJj".into()));

    // A JSON array is not a string and must be rejected, not treated as raw
    // decoded bytes (no `visit_seq` implementation).
    let mut de = serde_json::Deserializer::from_str("[1,2,3]");
    let err = base64::deserialize::<_, [u8; 3]>(&mut de).unwrap_err();
    assert!(err.to_string().contains("invalid type"), "{err}");
}

#[test]
fn borrowed_slice_contract() {
    let bytes = b"abc";
    let borrowed = <&[u8] as FromEncoded>::from_borrowed(bytes).unwrap();
    assert_eq!(borrowed.as_ptr(), bytes.as_ptr());
    assert_eq!(borrowed, &bytes[..]);
    assert!(<&[u8] as FromEncoded>::from_encoded(b"YWJj").is_err());
    assert!(<&[u8] as FromEncoded>::from_decoded(bytes).is_err());

    // Human-readable formats cannot lend decoded bytes out of the text.
    let mut de = serde_json::Deserializer::from_str("\"YWJj\"");
    assert!(base64::deserialize::<_, &[u8]>(&mut de).is_err());

    #[derive(Debug, PartialEq)]
    struct Borrowed<'a>(&'a [u8]);
    impl<'de> serde::Deserialize<'de> for Borrowed<'de> {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            base64::deserialize::<_, &'de [u8]>(deserializer).map(Borrowed)
        }
    }
    assert_de_tokens(&Borrowed(b"abcd").compact(), &[Token::BorrowedBytes(b"abcd")]);
}

#[cfg(feature = "alloc")]
#[test]
fn cow_contract() {
    use std::borrow::Cow;

    let bytes = b"abc";
    let owned = <Cow<'_, [u8]> as FromEncoded>::from_encoded(b"YWJj").unwrap();
    assert!(matches!(owned, Cow::Owned(_)));
    assert_eq!(&*owned, bytes);

    let owned = <Cow<'_, [u8]> as FromEncoded>::from_decoded(bytes).unwrap();
    assert!(matches!(owned, Cow::Owned(_)));
    assert_eq!(&*owned, bytes);

    let borrowed = <Cow<'_, [u8]> as FromEncoded>::from_borrowed(bytes).unwrap();
    assert!(matches!(borrowed, Cow::Borrowed(_)));
    assert_eq!(borrowed.as_ptr(), bytes.as_ptr());

    let mut de = serde_json::Deserializer::from_str("\"YWJj\"");
    let cow: Cow<'_, [u8]> = base64::deserialize(&mut de).unwrap();
    assert!(matches!(cow, Cow::Owned(_)));
    assert_eq!(&*cow, bytes);
}

#[cfg(feature = "small_collections")]
#[test]
fn smallvec_roundtrip_inline_and_heap() {
    use small_collections::SmallVec;

    let mut rng = StdRng::seed_from_u64(0x7E57_1A11_0C01_0110);
    for len in 0..=64usize {
        let data = {
            let mut v = vec![0u8; len];
            rng.fill(&mut v);
            v
        };
        let text = ref_encode(&data);

        let out = <SmallVec<u8, 16> as FromEncoded>::from_encoded(text.as_bytes()).unwrap();
        assert_eq!(&*out, data.as_slice(), "SmallVec<16> from_encoded len {len}");
        assert_eq!(out.is_inline(), len <= 16, "inline state at len {len}");

        let out = <SmallVec<u8, 16> as FromEncoded>::from_decoded(&data).unwrap();
        assert_eq!(&*out, data.as_slice(), "SmallVec<16> from_decoded len {len}");

        // Heap target smaller than the payload must still spill correctly.
        let out = <SmallVec<u8, 1> as FromEncoded>::from_encoded(text.as_bytes()).unwrap();
        assert_eq!(&*out, data.as_slice(), "SmallVec<1> len {len}");
    }
}

#[cfg(feature = "small_collections")]
#[test]
fn smallvec_json_roundtrip() {
    use small_collections::SmallVec;

    let data: [u8; 40] = core::array::from_fn(|i| (i * 7) as u8);
    let text = serde_json::to_string(&json_serialize(&data)).unwrap();
    let mut de = serde_json::Deserializer::from_str(&text);
    let out = base64::deserialize::<_, SmallVec<u8, 8>>(&mut de).unwrap();
    assert_eq!(&*out, &data);
}

#[cfg(feature = "alloc")]
#[test]
fn allocation_is_bounded_by_input_length() {
    use alloc_counter::measure;

    // A valid 48 KiB base64 string decodes to 36 KiB. The decode helper must
    // not allocate dramatically more than the input (no length-amplification
    // from a hostile length).
    let mut rng = StdRng::seed_from_u64(0xA5A5_5A5A_1234_5678);
    let data = {
        let mut v = vec![0u8; 48 * 1024];
        rng.fill(&mut v);
        v
    };
    let text = encode(&data, Alphabet::Standard);
    assert_eq!(text.len(), 64 * 1024);

    let (decoded, counters) = measure(|| decode(text.as_bytes(), Alphabet::Standard).unwrap());
    assert_eq!(decoded, data);
    assert!(
        counters.bytes <= text.len() as u64 + 4096,
        "decode allocated {} bytes for a {} byte input",
        counters.bytes,
        text.len()
    );
}
