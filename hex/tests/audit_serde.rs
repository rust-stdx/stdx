//! Tests for `hex/serde.rs`.
//!
//! Run with:
//! `cargo test -p hex --features serde,alloc --test audit_serde`
//! `cargo test -p hex --features small_collections --test audit_serde`
//! `cargo test -p hex --no-default-features --features serde --test audit_serde`

#![allow(clippy::needless_range_loop)]
#![cfg(feature = "serde")]

#[cfg(feature = "alloc")]
use hex::Alphabet;
use hex::FromEncoded;
#[cfg(feature = "alloc")]
use hex::decode;
use rand::{RngExt, SeedableRng, rngs::StdRng};
use serde_json::value::Serializer as JsonSerializer;
use serde_test::{Configure, Token, assert_de_tokens, assert_ser_tokens};

fn json_serialize(data: &[u8]) -> serde_json::Value {
    hex::serialize(data, JsonSerializer).expect("human-readable serialization failed")
}

/// Independent lowercase hex encoder (serde always emits lowercase).
fn ref_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(data.len() * 2);
    for &byte in data {
        out.push(TABLE[(byte >> 4) as usize] as char);
        out.push(TABLE[(byte & 0x0F) as usize] as char);
    }
    out
}

#[cfg(feature = "alloc")]
fn json_deserialize_vec(text: &str) -> Result<Vec<u8>, String> {
    let mut de = serde_json::Deserializer::from_str(text);
    let value = hex::deserialize::<_, Vec<u8>>(&mut de).map_err(|e| e.to_string())?;
    de.end().map_err(|e| e.to_string())?;
    Ok(value)
}

fn json_deserialize_array<const N: usize>(text: &str) -> Result<[u8; N], String> {
    let mut de = serde_json::Deserializer::from_str(text);
    let value = hex::deserialize::<_, [u8; N]>(&mut de).map_err(|e| e.to_string())?;
    de.end().map_err(|e| e.to_string())?;
    Ok(value)
}

#[test]
fn json_roundtrip_every_length_0_to_512() {
    let mut rng = StdRng::seed_from_u64(0x0123_4567_89AB_CDEF);
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

    let mut rng = StdRng::seed_from_u64(0xFEDC_BA98_7654_3210);
    check::<0>(&mut rng);
    check::<1>(&mut rng);
    check::<2>(&mut rng);
    check::<3>(&mut rng);
    check::<4>(&mut rng);
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
    check::<255>(&mut rng);
    check::<256>(&mut rng);
    check::<257>(&mut rng);
    check::<512>(&mut rng);
    check::<513>(&mut rng);
    check::<1024>(&mut rng);
}

#[cfg(feature = "alloc")]
#[test]
fn direct_roundtrip_both_alphabets_0_to_512() {
    let mut rng = StdRng::seed_from_u64(0x1111_2222_3333_4444);
    for alphabet in [Alphabet::Lower, Alphabet::Upper] {
        for len in 0..=512usize {
            let data = {
                let mut v = vec![0u8; len];
                rng.fill(&mut v);
                v
            };
            let text = hex::encode_with_alphabet(&data, alphabet);
            assert_eq!(text.len(), len * 2);
            match alphabet {
                Alphabet::Lower => assert!(text.bytes().all(|b| !b.is_ascii_uppercase()), "{text}"),
                Alphabet::Upper => assert!(text.bytes().all(|b| !b.is_ascii_lowercase()), "{text}"),
            }
            // Uppercase output must also decode; the decoder is
            // case-insensitive by design.
            let decoded = decode(text.as_bytes()).unwrap();
            assert_eq!(decoded, data, "round-trip differs for {alphabet:?} at len {len}");
        }
    }
}

#[cfg(feature = "alloc")]
#[test]
fn from_encoded_matches_direct_decode() {
    let corpus: &[&[u8]] = &[
        b"",
        b"0",
        b"00",
        b"0g",
        b"g0",
        b"gg",
        b"GG",
        b"aA",
        b"Aa",
        b"aabbccDD",
        b"aabbccd",
        b"aabbccdde",
        b" 00",
        b"00 ",
        b"0 0",
        b"00\n",
        b"0x00",
        b"00 11",
        b"0011 ",
        b"=",
        b"==",
        b"++",
        b"--",
        b"ZZ",
    ];

    for &input in corpus {
        let direct = decode(input);
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
    let bases: &[&[u8]] = &[b"", b"0", b"00", b"0f", b"FF", b"deadbeef", b"00ff00ff"];

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
    let direct = decode(input);
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
    let mut rng = StdRng::seed_from_u64(0x5555_AAAA_1234_9876);
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

#[cfg(feature = "alloc")]
#[test]
fn odd_length_is_rejected() {
    assert!(decode(b"0").is_err());
    assert!(decode(b"000").is_err());
    assert!(decode(b"deadbee").is_err());
    assert!(<Vec<u8> as FromEncoded>::from_encoded(b"0").is_err());
    assert!(<Vec<u8> as FromEncoded>::from_encoded(b"000").is_err());
}

#[test]
fn odd_length_is_rejected_arrays() {
    assert!(<[u8; 0] as FromEncoded>::from_encoded(b"0").is_err());
    assert!(<[u8; 1] as FromEncoded>::from_encoded(b"000").is_err());
    assert!(<[u8; 1] as FromEncoded>::from_encoded(b"00").is_ok());
}

#[test]
fn exact_array_length_is_enforced() {
    let text = "\"00ff\"";
    assert_eq!(json_deserialize_array::<2>(text).unwrap(), [0x00, 0xFF]);
    assert!(json_deserialize_array::<0>(text).is_err());
    assert!(json_deserialize_array::<1>(text).is_err());
    assert!(json_deserialize_array::<3>(text).is_err());
    assert!(json_deserialize_array::<32>(text).is_err());
    assert!(<[u8; 2] as FromEncoded>::from_decoded(&[0, 1]).is_ok());
    assert!(<[u8; 2] as FromEncoded>::from_decoded(&[0]).is_err());
    assert!(<[u8; 2] as FromEncoded>::from_decoded(&[0, 1, 2]).is_err());
}

#[cfg(feature = "alloc")]
#[test]
fn whitespace_and_uppercase_handling() {
    // Serde emits lowercase; the decoder accepts mixed case but no
    // whitespace or prefixes.
    assert_eq!(decode(b"DEADBEEF").unwrap(), [0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(decode(b"DeAdBeEf").unwrap(), [0xDE, 0xAD, 0xBE, 0xEF]);
    for bad in [&b" DEADBEEF"[..], b"DEADBEEF ", b"DEAD BEEF", b"0xdead", b"dead\n"] {
        assert!(decode(bad).is_err(), "accepted {bad:?}");
    }
    // JSON string with a newline escape: rejected, not silently trimmed.
    let mut de = serde_json::Deserializer::from_str("\"de\\nad\"");
    assert!(hex::deserialize::<_, [u8; 2]>(&mut de).is_err());
}

#[test]
fn whitespace_and_uppercase_handling_arrays() {
    assert_eq!(
        <[u8; 4] as FromEncoded>::from_encoded(b"DEADBEEF").unwrap(),
        [0xDE, 0xAD, 0xBE, 0xEF]
    );
    for bad in [&b" DEADBEEF"[..], b"DEADBEEF ", b"DEAD BEEF", b"0xdead", b"dead\n"] {
        assert!(<[u8; 4] as FromEncoded>::from_encoded(bad).is_err(), "accepted {bad:?}");
    }
    let mut de = serde_json::Deserializer::from_str("\"de\\nad\"");
    assert!(hex::deserialize::<_, [u8; 2]>(&mut de).is_err());
}

#[test]
fn binary_roundtrip_uses_raw_bytes() {
    #[derive(Debug)]
    struct Bin<'a>(&'a [u8]);
    impl serde::Serialize for Bin<'_> {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            hex::serialize(self.0, serializer)
        }
    }

    #[derive(Debug, PartialEq)]
    struct OwnedVec(Vec<u8>);
    impl<'de> serde::Deserialize<'de> for OwnedVec {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            #[cfg(feature = "alloc")]
            {
                hex::deserialize::<_, Vec<u8>>(deserializer).map(OwnedVec)
            }
            #[cfg(not(feature = "alloc"))]
            {
                let bytes = hex::deserialize::<_, [u8; 12]>(deserializer)?;
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
    assert_eq!(value, serde_json::Value::String("616263".into()));

    let mut de = serde_json::Deserializer::from_str("[1,2,3]");
    let err = hex::deserialize::<_, [u8; 3]>(&mut de).unwrap_err();
    assert!(err.to_string().contains("invalid type"), "{err}");
}

#[test]
fn borrowed_slice_contract() {
    let bytes = b"abc";
    let borrowed = <&[u8] as FromEncoded>::from_borrowed(bytes).unwrap();
    assert_eq!(borrowed.as_ptr(), bytes.as_ptr());
    assert_eq!(borrowed, &bytes[..]);
    assert!(<&[u8] as FromEncoded>::from_encoded(b"616263").is_err());
    assert!(<&[u8] as FromEncoded>::from_decoded(bytes).is_err());

    let mut de = serde_json::Deserializer::from_str("\"616263\"");
    assert!(hex::deserialize::<_, &[u8]>(&mut de).is_err());

    #[derive(Debug, PartialEq)]
    struct Borrowed<'a>(&'a [u8]);
    impl<'de> serde::Deserialize<'de> for Borrowed<'de> {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            hex::deserialize::<_, &'de [u8]>(deserializer).map(Borrowed)
        }
    }
    assert_de_tokens(&Borrowed(b"abcd").compact(), &[Token::BorrowedBytes(b"abcd")]);
}

#[cfg(feature = "alloc")]
#[test]
fn cow_contract() {
    use std::borrow::Cow;

    let bytes = b"abc";
    let owned = <Cow<'_, [u8]> as FromEncoded>::from_encoded(b"616263").unwrap();
    assert!(matches!(owned, Cow::Owned(_)));
    assert_eq!(&*owned, bytes);

    let owned = <Cow<'_, [u8]> as FromEncoded>::from_decoded(bytes).unwrap();
    assert!(matches!(owned, Cow::Owned(_)));
    assert_eq!(&*owned, bytes);

    let borrowed = <Cow<'_, [u8]> as FromEncoded>::from_borrowed(bytes).unwrap();
    assert!(matches!(borrowed, Cow::Borrowed(_)));
    assert_eq!(borrowed.as_ptr(), bytes.as_ptr());

    let mut de = serde_json::Deserializer::from_str("\"616263\"");
    let cow: Cow<'_, [u8]> = hex::deserialize(&mut de).unwrap();
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
    let out = hex::deserialize::<_, SmallVec<u8, 8>>(&mut de).unwrap();
    assert_eq!(&*out, &data);
}

#[cfg(feature = "alloc")]
#[test]
fn allocation_is_bounded_by_input_length() {
    use alloc_counter::measure;

    let mut rng = StdRng::seed_from_u64(0xABCD_1234_5678_EF90);
    let data = {
        let mut v = vec![0u8; 64 * 1024];
        rng.fill(&mut v);
        v
    };
    let text = ref_encode(&data);
    assert_eq!(text.len(), 2 * data.len());

    let (decoded, counters) = measure(|| decode(text.as_bytes()).unwrap());
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
