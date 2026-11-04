#![cfg(feature = "alloc")]

// RFC 8949 Appendix A vectors from the cbor/test-vectors repository.
// `f818` is intentionally excluded: RFC 8949 §3.3 makes two-byte
// simple values below 32 not well-formed, even though older test
// vectors list simple(24).

const VECTORS: &[(&str, bool, Option<&str>)] = &[
    ("00", true, None),
    ("01", true, None),
    ("0a", true, None),
    ("17", true, None),
    ("1818", true, None),
    ("1819", true, None),
    ("1864", true, None),
    ("1903e8", true, None),
    ("1a000f4240", true, None),
    ("1b000000e8d4a51000", true, None),
    ("1bffffffffffffffff", true, None),
    ("c249010000000000000000", true, None),
    ("3bffffffffffffffff", true, None),
    ("c349010000000000000000", true, None),
    ("20", true, None),
    ("29", true, None),
    ("3863", true, None),
    ("3903e7", true, None),
    ("f90000", true, None),
    ("f98000", true, None),
    ("f93c00", true, None),
    ("fb3ff199999999999a", true, None),
    ("f93e00", true, None),
    ("f97bff", true, None),
    ("fa47c35000", true, None),
    ("fa7f7fffff", true, None),
    ("fb7e37e43c8800759c", true, None),
    ("f90001", true, None),
    ("f90400", true, None),
    ("f9c400", true, None),
    ("fbc010666666666666", true, None),
    ("f97c00", true, Some("Infinity")),
    ("f97e00", true, Some("NaN")),
    ("f9fc00", true, Some("-Infinity")),
    ("fa7f800000", false, Some("Infinity")),
    ("fa7fc00000", false, Some("NaN")),
    ("faff800000", false, Some("-Infinity")),
    ("fb7ff0000000000000", false, Some("Infinity")),
    ("fb7ff8000000000000", false, Some("NaN")),
    ("fbfff0000000000000", false, Some("-Infinity")),
    ("f4", true, None),
    ("f5", true, None),
    ("f6", true, None),
    ("f7", true, Some("undefined")),
    ("f0", true, Some("simple(16)")),
    ("f8ff", true, Some("simple(255)")),
    (
        "c074323031332d30332d32315432303a30343a30305a",
        true,
        Some("0(\"2013-03-21T20:04:00Z\")"),
    ),
    ("c11a514b67b0", true, Some("1(1363896240)")),
    ("c1fb41d452d9ec200000", true, Some("1(1363896240.5)")),
    ("d74401020304", true, Some("23(h'01020304')")),
    ("d818456449455446", true, Some("24(h'6449455446')")),
    (
        "d82076687474703a2f2f7777772e6578616d706c652e636f6d",
        true,
        Some("32(\"http://www.example.com\")"),
    ),
    ("40", true, Some("h''")),
    ("4401020304", true, Some("h'01020304'")),
    ("60", true, None),
    ("6161", true, None),
    ("6449455446", true, None),
    ("62225c", true, None),
    ("62c3bc", true, None),
    ("63e6b0b4", true, None),
    ("64f0908591", true, None),
    ("80", true, None),
    ("83010203", true, None),
    ("8301820203820405", true, None),
    ("98190102030405060708090a0b0c0d0e0f101112131415161718181819", true, None),
    ("a0", true, None),
    ("a201020304", true, Some("{1: 2, 3: 4}")),
    ("a26161016162820203", true, None),
    ("826161a161626163", true, None),
    ("a56161614161626142616361436164614461656145", true, None),
    ("5f42010243030405ff", false, Some("(_ h'0102', h'030405')")),
    ("7f657374726561646d696e67ff", false, None),
    ("9fff", false, None),
    ("9f018202039f0405ffff", false, None),
    ("9f01820203820405ff", false, None),
    ("83018202039f0405ff", false, None),
    ("83019f0203ff820405", false, None),
    ("9f0102030405060708090a0b0c0d0e0f101112131415161718181819ff", false, None),
    ("bf61610161629f0203ffff", false, None),
    ("826161bf61626163ff", false, None),
    ("bf6346756ef563416d7421ff", false, None),
];

fn hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

#[test]
fn appendix_a_vectors_roundtrip() {
    for &(encoded, roundtrip, _) in VECTORS {
        let bytes = hex(encoded);
        assert!(cbor::validate(&bytes).is_ok(), "validate {encoded}");
        let value: cbor::Value = cbor::from_slice(&bytes).unwrap_or_else(|e| panic!("decode {encoded}: {e}"));
        if roundtrip {
            let reencoded = cbor::to_vec(&value);
            assert_eq!(hex(encoded), reencoded, "roundtrip {encoded}");
        }
    }
}

#[test]
fn appendix_a_diagnostic_vectors() {
    for &(encoded, _, diagnostic) in VECTORS {
        let Some(expected) = diagnostic else { continue };
        let bytes = hex(encoded);
        // The test-vectors file omits the space after the indefinite marker;
        // RFC 8949 §8 uses `(_ h'...')`.
        let expected = expected.replace("(_h", "(_ h").replace("(_\"", "(_ \"");
        let actual = cbor::to_diagnostic_string(&bytes);
        assert_eq!(expected, actual, "diagnostic {encoded}");
    }
}
