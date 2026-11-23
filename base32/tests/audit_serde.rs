//! Tests for `base32/serde.rs`.
//!
//! Run with:
//! `cargo test -p base32 --features serde,alloc --test audit_serde`
//! `cargo test -p base32 --features small_collections --test audit_serde`
//! `cargo test -p base32 --no-default-features --features serde --test audit_serde`

#![allow(clippy::needless_range_loop)]
#![cfg(feature = "serde")]

use base32::FromEncoded;
#[cfg(feature = "alloc")]
use base32::{Alphabet, decode, encode};
use rand::{RngExt, SeedableRng, rngs::StdRng};
use serde_json::value::Serializer as JsonSerializer;
use serde_test::{Configure, Token, assert_de_tokens, assert_ser_tokens};

#[cfg(feature = "alloc")]
const ALPHABETS: [Alphabet; 10] = [
    Alphabet::Crockford,
    Alphabet::Rfc4648,
    Alphabet::Rfc4648NoPadding,
    Alphabet::Rfc4648Lower,
    Alphabet::Rfc4648LowerNoPadding,
    Alphabet::Rfc4648Hex,
    Alphabet::Rfc4648HexNoPadding,
    Alphabet::Rfc4648HexLower,
    Alphabet::Rfc4648HexLowerNoPadding,
    Alphabet::Z32,
];

fn json_serialize(data: &[u8]) -> serde_json::Value {
    base32::serialize(data, JsonSerializer).expect("human-readable serialization failed")
}

/// Local, independent RFC 4648 padded base32 encoder used to check the
/// serializer without depending on the crate's own `encode` (which is
/// unavailable when the `alloc` feature is off).
fn ref_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut out = String::new();
    let mut acc: u64 = 0;
    let mut nbits = 0u32;
    for &byte in data {
        acc = (acc << 8) | u64::from(byte);
        nbits += 8;
        while nbits >= 5 {
            nbits -= 5;
            out.push(TABLE[((acc >> nbits) & 31) as usize] as char);
        }
        acc &= (1u64 << nbits) - 1;
    }
    if nbits > 0 {
        out.push(TABLE[((acc << (5 - nbits)) & 31) as usize] as char);
    }
    while !out.len().is_multiple_of(8) {
        out.push('=');
    }
    out
}

#[cfg(feature = "alloc")]
fn json_deserialize_vec(text: &str) -> Result<Vec<u8>, String> {
    let mut de = serde_json::Deserializer::from_str(text);
    let value = base32::deserialize::<_, Vec<u8>>(&mut de).map_err(|e| e.to_string())?;
    de.end().map_err(|e| e.to_string())?;
    Ok(value)
}

fn json_deserialize_array<const N: usize>(text: &str) -> Result<[u8; N], String> {
    let mut de = serde_json::Deserializer::from_str(text);
    let value = base32::deserialize::<_, [u8; N]>(&mut de).map_err(|e| e.to_string())?;
    de.end().map_err(|e| e.to_string())?;
    Ok(value)
}

#[test]
fn json_roundtrip_every_length_0_to_512() {
    let mut rng = StdRng::seed_from_u64(0x1020_3040_5060_7080);
    for len in 0..=512usize {
        let data = {
            let mut v = vec![0u8; len];
            rng.fill(&mut v);
            v
        };
        let value = json_serialize(&data);
        assert_eq!(
            value,
            serde_json::Value::String(ref_encode(&data)),
            "serialized form differs at len {len}"
        );

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
    fn check<const N: usize>(rng: &mut StdRng) {
        let data: [u8; N] = core::array::from_fn(|_| rng.random::<u8>());
        let value = json_serialize(&data);
        assert_eq!(value, serde_json::Value::String(ref_encode(&data)));
        let text = serde_json::to_string(&value).unwrap();
        assert_eq!(json_deserialize_array::<N>(&text).unwrap(), data, "array N={N}");
        #[cfg(feature = "alloc")]
        assert_eq!(json_deserialize_vec(&text).unwrap(), data);
    }

    let mut rng = StdRng::seed_from_u64(0xABCD_EF01_2345_6789);
    check::<0>(&mut rng);
    check::<1>(&mut rng);
    check::<2>(&mut rng);
    check::<3>(&mut rng);
    check::<4>(&mut rng);
    check::<5>(&mut rng);
    check::<6>(&mut rng);
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
    check::<158>(&mut rng);
    check::<159>(&mut rng);
    check::<160>(&mut rng);
    check::<161>(&mut rng);
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
    let mut rng = StdRng::seed_from_u64(0x3141_5926_5358_9793);
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
fn from_encoded_matches_direct_decode_for_rfc4648() {
    let corpus: &[&[u8]] = &[
        b"",
        b"=",
        b"========",
        b"A",
        b"AA",
        b"AAA",
        b"AAAA",
        b"AAAAA",
        b"AAAAAAAA",
        b"AA======",
        b"AA=====",
        b"AA=====",
        b"NA",
        b"NA======",
        b"NBSQ",
        b"NBSQ====",
        b"NBSWY",
        b"NBSWY===",
        b"NBSWY3A",
        b"NBSWY3A=",
        b"NBSWY3DP",
        b"MZXW6===",
        b"MZXW6YQ=",
        b"AA==AAAA",
        b"AA======AA======",
        b"AAAAAAAA========",
        // non-canonical trailing bits: "NA" with the low 2 bits of the
        // second quintet set becomes "NB".."ND" style mutations.
        b"NB",
        b"NC",
        b"ND",
        b"NBSR",
        b"NBSWZ",
        b"nbs w",
        b"NB SWY3DP",
        b"NBSWY3DP\n",
        b"NBSWY3DP!",
        b"nbswy3dp", // lowercase alphabet must be rejected by Rfc4648
        b"D1IMOR3F", // extended-hex alphabet must be rejected by Rfc4648
        b"0189",     // digits outside 2-7
        b"ABCDEFG1", // 1 is invalid
    ];

    for &input in corpus {
        let direct = decode(input, Alphabet::Rfc4648);
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
    let bases: &[&[u8]] = &[
        b"",
        b"A",
        b"AA",
        b"NA",
        b"NBSQ",
        b"NBSWY",
        b"NBSWY3A",
        b"NBSWY3DP",
        b"AA======",
    ];

    for base in bases {
        for pos in 0..base.len() {
            for byte in 0..=255u8 {
                let mut mutated = base.to_vec();
                mutated[pos] = byte;
                check_serde_vs_direct(&mutated);
            }
        }
        for pos in 0..=base.len() {
            for byte in 0..=255u8 {
                let mut mutated = base.to_vec();
                mutated.insert(pos, byte);
                check_serde_vs_direct(&mutated);
            }
        }
        for pos in 0..base.len() {
            let mut mutated = base.to_vec();
            mutated.remove(pos);
            check_serde_vs_direct(&mutated);
        }
    }
}

#[cfg(feature = "alloc")]
fn check_serde_vs_direct(input: &[u8]) {
    let direct = decode(input, Alphabet::Rfc4648);
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
fn deterministic_fuzz_no_panics() {
    let mut rng = StdRng::seed_from_u64(0x2718_2818_2845_9045);
    for len in 0..=96usize {
        for _ in 0..64 {
            let input: Vec<u8> = (0..len).map(|_| rng.random::<u8>()).collect();
            #[cfg(feature = "alloc")]
            check_serde_vs_direct(&input);
            let _ = <[u8; 0] as FromEncoded>::from_encoded(&input);
            let _ = <[u8; 1] as FromEncoded>::from_encoded(&input);
            let _ = <[u8; 8] as FromEncoded>::from_encoded(&input);
            let _ = <[u8; 32] as FromEncoded>::from_encoded(&input);
        }
    }
}

/// Known bug, shared with the core decoder (unit E4): padded `Rfc4648`
/// claims to expect padding but accepts unpadded input of every valid
/// remainder (2, 4, 5, 7). Run with `--ignored` to reproduce.
#[cfg(feature = "alloc")]
#[test]
fn strict_rfc4648_padding_rejects_unpadded_input() {
    assert!(
        <[u8; 1] as FromEncoded>::from_encoded(b"NA").is_err(),
        "serde array accepted unpadded 1-byte form"
    );
    assert!(
        <Vec<u8> as FromEncoded>::from_encoded(b"NA").is_err(),
        "serde Vec accepted unpadded 1-byte form"
    );
    assert!(decode(b"NA", Alphabet::Rfc4648).is_err(), "core accepted unpadded 1-byte form");
    assert!(
        decode(b"NBSQ", Alphabet::Rfc4648).is_err(),
        "core accepted unpadded 2-byte form"
    );
    assert!(
        decode(b"NBSWY", Alphabet::Rfc4648).is_err(),
        "core accepted unpadded 3-byte form"
    );
    assert!(
        decode(b"NBSWY3A", Alphabet::Rfc4648).is_err(),
        "core accepted unpadded 4-byte form"
    );
}

#[cfg(feature = "alloc")]
#[test]
fn long_unpadded_rfc4648_is_rejected() {
    for len in 1..=256usize {
        let data: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(47)).collect();
        let mut encoded = encode(&data, Alphabet::Rfc4648).into_bytes();
        while encoded.last() == Some(&b'=') {
            encoded.pop();
        }
        let result = <Vec<u8> as FromEncoded>::from_encoded(&encoded);
        if data.len().is_multiple_of(5) {
            assert_eq!(result.unwrap(), data, "len={len}");
        } else {
            assert!(result.is_err(), "len={len} accepted unpadded {encoded:?}");
        }
    }
}

#[cfg(feature = "alloc")]
#[test]
fn malformed_padding_is_rejected() {
    assert!(decode(b"AA=====", Alphabet::Rfc4648).is_err());
    assert!(decode(b"AA==========", Alphabet::Rfc4648).is_err());
    assert!(decode(b"AAA=====", Alphabet::Rfc4648).is_err());
    assert!(decode(b"A=======", Alphabet::Rfc4648).is_err());
    for input in [&b"A======="[..], b"AA======", b"NBSWY3D="] {
        assert!(
            <[u8; 5] as FromEncoded>::from_encoded(input).is_err(),
            "serde accepted {input:?}"
        );
    }
}

#[cfg(feature = "alloc")]
#[test]
fn non_canonical_trailing_bits_are_rejected() {
    // 10 bits carry one byte; the low 2 bits of the second quintet must be 0.
    // 'A'=0, 'B'=1 so "NB" has non-zero trailing bits, "NA" is canonical.
    assert!(decode(b"NB", Alphabet::Rfc4648NoPadding).is_err());
    assert!(decode(b"NC", Alphabet::Rfc4648NoPadding).is_err());
    assert!(decode(b"ND", Alphabet::Rfc4648NoPadding).is_err());
    assert_eq!(decode(b"NA", Alphabet::Rfc4648NoPadding).unwrap(), b"h");
    assert!(<Vec<u8> as FromEncoded>::from_encoded(b"NB======").is_err());
    assert!(<Vec<u8> as FromEncoded>::from_encoded(b"NC======").is_err());
    assert!(<Vec<u8> as FromEncoded>::from_encoded(b"ND======").is_err());
}

#[test]
fn non_canonical_trailing_bits_are_rejected_arrays() {
    assert!(<[u8; 1] as FromEncoded>::from_encoded(b"NB======").is_err());
    assert!(<[u8; 1] as FromEncoded>::from_encoded(b"NC======").is_err());
    assert!(<[u8; 1] as FromEncoded>::from_encoded(b"ND======").is_err());
    assert_eq!(<[u8; 1] as FromEncoded>::from_encoded(b"NA======").unwrap(), *b"h");
}

#[test]
fn exact_array_length_is_enforced() {
    // "NBSWY3DP" decodes to "hello".
    let text = "\"NBSWY3DP\"";
    assert_eq!(json_deserialize_array::<5>(text).unwrap(), *b"hello");
    assert!(json_deserialize_array::<0>(text).is_err());
    assert!(json_deserialize_array::<4>(text).is_err());
    assert!(json_deserialize_array::<6>(text).is_err());
    assert!(json_deserialize_array::<32>(text).is_err());
    assert!(<[u8; 5] as FromEncoded>::from_decoded(b"hello").is_ok());
    assert!(<[u8; 5] as FromEncoded>::from_decoded(b"hell").is_err());
    assert!(<[u8; 5] as FromEncoded>::from_decoded(b"helloo").is_err());
}

#[test]
fn binary_roundtrip_uses_raw_bytes() {
    #[derive(Debug)]
    struct Bin<'a>(&'a [u8]);
    impl serde::Serialize for Bin<'_> {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            base32::serialize(self.0, serializer)
        }
    }

    #[derive(Debug, PartialEq)]
    struct OwnedVec(Vec<u8>);
    impl<'de> serde::Deserialize<'de> for OwnedVec {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            #[cfg(feature = "alloc")]
            {
                base32::deserialize::<_, Vec<u8>>(deserializer).map(OwnedVec)
            }
            #[cfg(not(feature = "alloc"))]
            {
                // No Vec target without `alloc`; map raw bytes through a
                // fixed-size array of the same length for the test payload.
                let bytes = base32::deserialize::<_, [u8; 12]>(deserializer)?;
                Ok(OwnedVec(bytes.to_vec()))
            }
        }
    }

    let data = b"hello binary";
    assert_ser_tokens(&Bin(data).compact(), &[Token::Bytes(data)]);
    assert_de_tokens(&OwnedVec(data.to_vec()).compact(), &[Token::Bytes(data)]);
    assert_de_tokens(&OwnedVec(data.to_vec()).compact(), &[Token::BorrowedBytes(data)]);

    #[cfg(feature = "alloc")]
    {
        static LARGE: [u8; 4096] = [0xAB; 4096];
        assert_ser_tokens(&Bin(&LARGE).compact(), &[Token::Bytes(&LARGE)]);
        assert_de_tokens(&OwnedVec(LARGE.to_vec()).compact(), &[Token::Bytes(&LARGE)]);
    }
}

#[test]
fn human_readable_visitor_only_accepts_strings() {
    let value = json_serialize(b"abc");
    assert_eq!(value, serde_json::Value::String("MFRGG===".into()));

    let mut de = serde_json::Deserializer::from_str("[1,2,3]");
    let err = base32::deserialize::<_, [u8; 3]>(&mut de).unwrap_err();
    assert!(err.to_string().contains("invalid type"), "{err}");
}

#[test]
fn borrowed_slice_contract() {
    let bytes = b"abc";
    let borrowed = <&[u8] as FromEncoded>::from_borrowed(bytes).unwrap();
    assert_eq!(borrowed.as_ptr(), bytes.as_ptr());
    assert_eq!(borrowed, &bytes[..]);
    assert!(<&[u8] as FromEncoded>::from_encoded(b"MFRGG===").is_err());
    assert!(<&[u8] as FromEncoded>::from_decoded(bytes).is_err());

    let mut de = serde_json::Deserializer::from_str("\"MFRGG===\"");
    assert!(base32::deserialize::<_, &[u8]>(&mut de).is_err());

    #[derive(Debug, PartialEq)]
    struct Borrowed<'a>(&'a [u8]);
    impl<'de> serde::Deserialize<'de> for Borrowed<'de> {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            base32::deserialize::<_, &'de [u8]>(deserializer).map(Borrowed)
        }
    }
    assert_de_tokens(&Borrowed(b"abcd").compact(), &[Token::BorrowedBytes(b"abcd")]);
}

#[cfg(feature = "alloc")]
#[test]
fn cow_contract() {
    use std::borrow::Cow;

    let bytes = b"abc";
    let owned = <Cow<'_, [u8]> as FromEncoded>::from_encoded(b"MFRGG===").unwrap();
    assert!(matches!(owned, Cow::Owned(_)));
    assert_eq!(&*owned, bytes);

    let owned = <Cow<'_, [u8]> as FromEncoded>::from_decoded(bytes).unwrap();
    assert!(matches!(owned, Cow::Owned(_)));
    assert_eq!(&*owned, bytes);

    let borrowed = <Cow<'_, [u8]> as FromEncoded>::from_borrowed(bytes).unwrap();
    assert!(matches!(borrowed, Cow::Borrowed(_)));
    assert_eq!(borrowed.as_ptr(), bytes.as_ptr());

    let mut de = serde_json::Deserializer::from_str("\"MFRGG===\"");
    let cow: Cow<'_, [u8]> = base32::deserialize(&mut de).unwrap();
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
    let out = base32::deserialize::<_, SmallVec<u8, 8>>(&mut de).unwrap();
    assert_eq!(&*out, &data);
}

#[cfg(feature = "alloc")]
#[test]
fn allocation_is_bounded_by_input_length() {
    use alloc_counter::measure;

    let mut rng = StdRng::seed_from_u64(0x9999_8888_7777_6666);
    let data = {
        let mut v = vec![0u8; 48 * 1024];
        rng.fill(&mut v);
        v
    };
    let text = encode(&data, Alphabet::Rfc4648);
    // Padded encoding: ceil(len / 5) * 8.
    assert_eq!(text.len(), data.len().div_ceil(5) * 8);

    let (decoded, counters) = measure(|| decode(text.as_bytes(), Alphabet::Rfc4648).unwrap());
    assert_eq!(decoded, data);
    assert!(
        counters.bytes <= text.len() as u64 + 4096,
        "decode allocated {} bytes for a {} byte input",
        counters.bytes,
        text.len()
    );

    let (decoded, counters) = measure(|| <Vec<u8> as FromEncoded>::from_encoded(text.as_bytes()).unwrap());
    assert_eq!(decoded, data);
    assert!(
        counters.bytes <= text.len() as u64 + 4096,
        "from_encoded allocated {} bytes for a {} byte input",
        counters.bytes,
        text.len()
    );
}
