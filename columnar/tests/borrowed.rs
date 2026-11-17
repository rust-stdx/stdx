use std::{borrow::Cow, marker::PhantomData};

use alloc_counter::measure;
use columnar::Columnar;

#[derive(Columnar, Debug, PartialEq)]
struct Blob<'a> {
    #[columnar(key = 1)]
    id: u64,
    #[columnar(key = 2)]
    data: Cow<'a, [u8]>,
    #[columnar(key = 3)]
    maybe: Option<Cow<'a, [u8]>>,
    #[columnar(key = 4)]
    list: Vec<Cow<'a, [u8]>>,
}

fn source() -> Vec<Vec<u8>> {
    vec![vec![], vec![0], vec![0xff, 0x00, 0x80], (0..=255).collect()]
}

#[test]
fn cow_bytes_decode_borrowed() {
    let source = source();
    let rows: Vec<Blob<'_>> = source
        .iter()
        .enumerate()
        .map(|(index, data)| Blob {
            id: index as u64,
            data: Cow::Borrowed(data.as_slice()),
            maybe: if index % 2 == 0 {
                Some(Cow::Borrowed(data.as_slice()))
            } else {
                None
            },
            list: vec![Cow::Borrowed(data.as_slice())],
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Blob<'_>> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded.len(), rows.len());

    let start = bytes.as_ptr() as usize;
    let end = start + bytes.len();
    for (decoded, expected) in decoded.iter().zip(rows.iter()) {
        assert_eq!(decoded.id, expected.id);
        assert!(matches!(decoded.data, Cow::Borrowed(_)));
        assert_eq!(decoded.data.as_ref(), expected.data.as_ref());
        if let Cow::Borrowed(slice) = &decoded.data
            && !slice.is_empty()
        {
            let ptr = slice.as_ptr() as usize;
            assert!(ptr >= start && ptr + slice.len() <= end);
        }
        match (&decoded.maybe, &expected.maybe) {
            (Some(decoded), Some(expected)) => {
                assert!(matches!(decoded, Cow::Borrowed(_)));
                assert_eq!(decoded.as_ref(), expected.as_ref());
            }
            (None, None) => {}
            other => panic!("option mismatch: {other:?}"),
        }
        assert_eq!(decoded.list.len(), expected.list.len());
        for (decoded, expected) in decoded.list.iter().zip(expected.list.iter()) {
            assert!(matches!(decoded, Cow::Borrowed(_)));
            assert_eq!(decoded.as_ref(), expected.as_ref());
        }
    }
}

#[test]
fn cow_bytes_encode_owned_and_decode_borrowed() {
    let source = source();
    let rows: Vec<Blob<'_>> = source
        .iter()
        .enumerate()
        .map(|(index, data)| Blob {
            id: index as u64,
            data: Cow::Owned(data.clone()),
            maybe: Some(Cow::Owned(data.clone())),
            list: vec![Cow::Owned(data.clone())],
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Blob<'_>> = columnar::from_slice(&bytes).unwrap();
    for (decoded, expected) in decoded.iter().zip(rows.iter()) {
        assert!(matches!(decoded.data, Cow::Borrowed(_)));
        assert_eq!(decoded.data.as_ref(), expected.data.as_ref());
        assert!(matches!(decoded.maybe, Some(Cow::Borrowed(_))));
        assert!(matches!(&decoded.list[0], Cow::Borrowed(_)));
    }
}

#[test]
fn empty_cow_rows() {
    let rows: Vec<Blob<'_>> = Vec::new();
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Blob<'_>> = columnar::from_slice(&bytes).unwrap();
    assert!(decoded.is_empty());
}

#[derive(Columnar, Debug, PartialEq)]
struct OffsetCow<'a> {
    #[columnar(key = 1, encoding = "offsets")]
    data: Cow<'a, [u8]>,
}

#[test]
fn cow_bytes_offsets_are_borrowed() {
    let source = source();
    let rows: Vec<OffsetCow<'_>> = source
        .iter()
        .map(|data| OffsetCow {
            data: Cow::Borrowed(data.as_slice()),
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<OffsetCow<'_>> = columnar::from_slice(&bytes).unwrap();
    for (decoded, expected) in decoded.iter().zip(rows.iter()) {
        assert!(matches!(decoded.data, Cow::Borrowed(_)));
        assert_eq!(decoded.data.as_ref(), expected.data.as_ref());
    }
}

#[derive(Columnar, Debug, PartialEq)]
struct RleCow<'a> {
    #[columnar(key = 1, encoding = "rle")]
    data: Cow<'a, [u8]>,
}

#[test]
fn cow_bytes_rle_roundtrip() {
    let source = source();
    let mut rows: Vec<RleCow<'_>> = Vec::new();
    for data in &source {
        rows.push(RleCow {
            data: Cow::Borrowed(data.as_slice()),
        });
        rows.push(RleCow {
            data: Cow::Borrowed(data.as_slice()),
        });
    }
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<RleCow<'_>> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded.len(), rows.len());
    for (decoded, expected) in decoded.iter().zip(rows.iter()) {
        assert!(matches!(decoded.data, Cow::Borrowed(_)));
        assert_eq!(decoded.data.as_ref(), expected.data.as_ref());
    }
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(borrow = "'a")]
struct TwoLifetimes<'a, 'b> {
    #[columnar(key = 1)]
    data: Cow<'a, [u8]>,
    #[columnar(key = 2)]
    marker: PhantomData<&'b ()>,
}

#[test]
fn explicit_borrow_lifetime() {
    let data = b"hello";
    let rows = vec![TwoLifetimes::<'_, 'static> {
        data: Cow::Borrowed(data.as_slice()),
        marker: PhantomData,
    }];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<TwoLifetimes<'_, 'static>> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded.len(), 1);
    assert!(matches!(decoded[0].data, Cow::Borrowed(_)));
    assert_eq!(decoded[0].data.as_ref(), b"hello");
}

#[derive(Columnar, Debug, PartialEq)]
struct OnlyCow<'a> {
    #[columnar(key = 1)]
    data: Cow<'a, [u8]>,
}

#[test]
fn borrowed_decode_does_not_copy_payloads() {
    const ROWS: usize = 1_000;
    const LEN: usize = 64;
    let payload = vec![7u8; LEN];
    let rows: Vec<OnlyCow<'_>> = (0..ROWS)
        .map(|_| OnlyCow {
            data: Cow::Borrowed(payload.as_slice()),
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();

    let (decoded, counters) = measure(|| columnar::from_slice::<OnlyCow<'_>>(&bytes).unwrap());
    assert_eq!(decoded.len(), ROWS);
    assert!(decoded.iter().all(|row| matches!(row.data, Cow::Borrowed(_))));
    assert!(
        counters.calls <= 4,
        "borrowed decode should only allocate a few vectors, got {}",
        counters.calls
    );
    assert!(
        counters.bytes < (ROWS * LEN) as u64,
        "borrowed decode copied payload bytes: {} >= {}",
        counters.bytes,
        ROWS * LEN
    );
}

#[test]
fn malformed_cow_lengths_are_rejected() {
    // [0: [1, 1], 1: [9, [[5], [1]]]]
    let mut e = cbor::Encoder::new(Vec::new());
    e.map(2).unwrap();
    e.u64(0).unwrap();
    e.array(2).unwrap();
    e.u64(1).unwrap();
    e.u64(1).unwrap();
    e.u64(1).unwrap();
    e.array(2).unwrap();
    e.u64(9).unwrap();
    e.array(2).unwrap();
    e.bytes(&[5]).unwrap();
    e.bytes(&[1]).unwrap();
    let bytes = e.into_inner();
    assert!(columnar::from_slice::<OnlyCow<'_>>(&bytes).is_err());
}

#[test]
fn cow_length_limit_is_enforced() {
    let rows = vec![OnlyCow {
        data: Cow::Borrowed(b"abcdef".as_slice()),
    }];
    let bytes = columnar::to_vec(&rows).unwrap();
    let options = columnar::DecodeOptions {
        max_string_len: 3,
        ..columnar::DecodeOptions::default()
    };
    assert!(columnar::from_slice_with_options::<OnlyCow<'_>>(&bytes, options).is_err());
}

#[derive(Columnar, Debug, PartialEq)]
enum CowEnum<'a> {
    None,
    Data(Cow<'a, [u8]>),
    Named { data: Cow<'a, [u8]>, id: u64 },
}

#[derive(Columnar, Debug, PartialEq)]
struct CowEnumHolder<'a> {
    #[columnar(key = 1)]
    value: CowEnum<'a>,
}

#[test]
fn cow_bytes_in_enum_variants_are_borrowed() {
    let payload = b"enum payload".to_vec();
    let rows = vec![
        CowEnumHolder {
            value: CowEnum::None,
        },
        CowEnumHolder {
            value: CowEnum::Data(Cow::Borrowed(payload.as_slice())),
        },
        CowEnumHolder {
            value: CowEnum::Named {
                data: Cow::Borrowed(payload.as_slice()),
                id: 7,
            },
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<CowEnumHolder<'_>> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded.len(), rows.len());
    match &decoded[1].value {
        CowEnum::Data(data) => {
            assert!(matches!(data, Cow::Borrowed(_)));
            assert_eq!(data.as_ref(), payload.as_slice());
        }
        other => panic!("unexpected variant: {other:?}"),
    }
    match &decoded[2].value {
        CowEnum::Named {
            data,
            id,
        } => {
            assert!(matches!(data, Cow::Borrowed(_)));
            assert_eq!(*id, 7);
        }
        other => panic!("unexpected variant: {other:?}"),
    }
}

#[derive(Columnar, Debug, PartialEq)]
struct DictCow<'a> {
    #[columnar(key = 1, encoding = "dictionary")]
    data: Cow<'a, [u8]>,
}

#[test]
fn cow_bytes_dictionary_roundtrip() {
    let source = source();
    let mut rows: Vec<DictCow<'_>> = Vec::new();
    for data in &source {
        for _ in 0..3 {
            rows.push(DictCow {
                data: Cow::Borrowed(data.as_slice()),
            });
        }
    }
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<DictCow<'_>> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded.len(), rows.len());
    for (decoded, expected) in decoded.iter().zip(rows.iter()) {
        assert!(matches!(decoded.data, Cow::Borrowed(_)));
        assert_eq!(decoded.data.as_ref(), expected.data.as_ref());
    }
}

// ---------------------------------------------------------------------------
// Cow<str>
// ---------------------------------------------------------------------------

#[derive(Columnar, Debug, PartialEq)]
struct TextOwned {
    #[columnar(key = 1)]
    id: u64,
    #[columnar(key = 2)]
    region: String,
    #[columnar(key = 3)]
    name: String,
    #[columnar(key = 4)]
    maybe: Option<String>,
    #[columnar(key = 5)]
    list: Vec<String>,
}

#[derive(Columnar, Debug, PartialEq)]
struct TextBorrowed<'a> {
    #[columnar(key = 1)]
    id: u64,
    #[columnar(key = 2)]
    region: Cow<'a, str>,
    #[columnar(key = 3)]
    name: Cow<'a, str>,
    #[columnar(key = 4)]
    maybe: Option<Cow<'a, str>>,
    #[columnar(key = 5)]
    list: Vec<Cow<'a, str>>,
}

fn text_owned_rows() -> Vec<TextOwned> {
    vec![
        TextOwned {
            id: 0,
            region: String::new(),
            name: "alpha".into(),
            maybe: None,
            list: vec![],
        },
        TextOwned {
            id: 1,
            region: "eu-west".into(),
            name: "βeta".into(),
            maybe: Some(String::new()),
            list: vec!["x".into()],
        },
        TextOwned {
            id: 2,
            region: "us-east".into(),
            name: "gamma".into(),
            maybe: Some("δ".into()),
            list: vec!["one".into(), "two".into()],
        },
    ]
}

#[test]
fn cow_str_decodes_borrowed_from_string_bytes() {
    let rows = text_owned_rows();
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<TextBorrowed<'_>> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded.len(), rows.len());

    let start = bytes.as_ptr() as usize;
    let end = start + bytes.len();
    for (decoded, expected) in decoded.iter().zip(rows.iter()) {
        assert_eq!(decoded.id, expected.id);
        assert!(matches!(decoded.region, Cow::Borrowed(_)));
        assert!(matches!(decoded.name, Cow::Borrowed(_)));
        assert_eq!(decoded.region, expected.region);
        assert_eq!(decoded.name, expected.name);
        if let Cow::Borrowed(value) = &decoded.name
            && !value.is_empty()
        {
            let ptr = value.as_ptr() as usize;
            assert!(ptr >= start && ptr + value.len() <= end);
        }
        match (&decoded.maybe, &expected.maybe) {
            (Some(decoded), Some(expected)) => {
                assert!(matches!(decoded, Cow::Borrowed(_)));
                assert_eq!(decoded, expected);
            }
            (None, None) => {}
            other => panic!("option mismatch: {other:?}"),
        }
        assert_eq!(decoded.list.len(), expected.list.len());
        for (decoded, expected) in decoded.list.iter().zip(expected.list.iter()) {
            assert!(matches!(decoded, Cow::Borrowed(_)));
            assert_eq!(decoded, expected);
        }
    }
}

#[test]
fn cow_str_encodes_owned_and_decodes_borrowed() {
    let rows: Vec<TextBorrowed<'_>> = text_owned_rows()
        .iter()
        .map(|row| TextBorrowed {
            id: row.id,
            region: Cow::Owned(row.region.clone()),
            name: Cow::Owned(row.name.clone()),
            maybe: row.maybe.as_ref().map(|value| Cow::Owned(value.clone())),
            list: row.list.iter().map(|value| Cow::Owned(value.clone())).collect(),
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<TextBorrowed<'_>> = columnar::from_slice(&bytes).unwrap();
    assert!(decoded.iter().all(|row| matches!(row.region, Cow::Borrowed(_))));
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq)]
struct TextOffsets<'a> {
    #[columnar(key = 1, encoding = "offsets")]
    value: Cow<'a, str>,
}

#[derive(Columnar, Debug, PartialEq)]
struct TextRle<'a> {
    #[columnar(key = 1, encoding = "rle")]
    value: Cow<'a, str>,
}

#[derive(Columnar, Debug, PartialEq)]
struct TextDictionary<'a> {
    #[columnar(key = 1, encoding = "dictionary")]
    value: Cow<'a, str>,
}

#[test]
fn cow_str_offsets_rle_and_dictionary() {
    let values = ["a", "", "longer value", "a", "a", "β"];

    let offsets: Vec<TextOffsets<'_>> = values
        .iter()
        .map(|value| TextOffsets {
            value: Cow::Borrowed(value),
        })
        .collect();
    let bytes = columnar::to_vec(&offsets).unwrap();
    let decoded: Vec<TextOffsets<'_>> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, offsets);

    let rle: Vec<TextRle<'_>> = values
        .iter()
        .map(|value| TextRle {
            value: Cow::Borrowed(value),
        })
        .collect();
    let bytes = columnar::to_vec(&rle).unwrap();
    let decoded: Vec<TextRle<'_>> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rle);

    let dictionary: Vec<TextDictionary<'_>> = values
        .iter()
        .map(|value| TextDictionary {
            value: Cow::Borrowed(value),
        })
        .collect();
    let bytes = columnar::to_vec(&dictionary).unwrap();
    let decoded: Vec<TextDictionary<'_>> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, dictionary);
}

#[derive(Columnar, Debug, PartialEq)]
struct OneText<'a> {
    #[columnar(key = 1)]
    value: Cow<'a, str>,
}

#[test]
fn malformed_cow_str_utf8_is_rejected() {
    // { 0: [1, 1], 1: [9, [bytes([1]), bytes([0xff])]] }
    let mut e = cbor::Encoder::new(Vec::new());
    e.map(2).unwrap();
    e.u64(0).unwrap();
    e.array(2).unwrap();
    e.u64(1).unwrap();
    e.u64(1).unwrap();
    e.u64(1).unwrap();
    e.array(2).unwrap();
    e.u64(9).unwrap();
    e.array(2).unwrap();
    e.bytes(&[1]).unwrap();
    e.bytes(&[0xff]).unwrap();
    let bytes = e.into_inner();
    let result = columnar::from_slice::<OneText<'_>>(&bytes);
    assert!(matches!(result, Err(columnar::Error::InvalidUtf8)), "{result:?}");
}

#[test]
fn cow_str_limits_are_enforced() {
    let rows = vec![TextBorrowed {
        id: 1,
        region: Cow::Borrowed("abcdef"),
        name: Cow::Borrowed("ghijkl"),
        maybe: None,
        list: vec![],
    }];
    let bytes = columnar::to_vec(&rows).unwrap();
    let options = columnar::DecodeOptions {
        max_string_len: 3,
        ..columnar::DecodeOptions::default()
    };
    assert!(columnar::from_slice_with_options::<TextBorrowed<'_>>(&bytes, options).is_err());

    let options = columnar::DecodeOptions {
        max_total_bytes: 4,
        ..columnar::DecodeOptions::default()
    };
    assert!(columnar::from_slice_with_options::<TextBorrowed<'_>>(&bytes, options).is_err());
}

#[derive(Columnar, Debug, PartialEq)]
struct TwoStrings<'a> {
    #[columnar(key = 1)]
    first: Cow<'a, str>,
    #[columnar(key = 2)]
    second: Cow<'a, str>,
}

#[test]
fn cow_str_decode_does_not_allocate_per_value() {
    const ROWS: usize = 1_000;
    let rows: Vec<TwoStrings<'_>> = (0..ROWS)
        .map(|_| TwoStrings {
            first: Cow::Borrowed("constant"),
            second: Cow::Borrowed("another constant"),
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();
    let (decoded, counters) = measure(|| columnar::from_slice::<TwoStrings<'_>>(&bytes).unwrap());
    assert_eq!(decoded.len(), ROWS);
    assert!(decoded.iter().all(|row| matches!(row.first, Cow::Borrowed(_))));
    assert!(counters.calls <= 4, "borrowed string decode allocated {} times", counters.calls);
}

#[derive(Columnar, Debug, PartialEq)]
enum TextEnum<'a> {
    Empty,
    Word(Cow<'a, str>),
    Named { value: Cow<'a, str>, id: u64 },
}

#[derive(Columnar, Debug, PartialEq)]
struct TextEnumRow<'a> {
    #[columnar(key = 1)]
    value: TextEnum<'a>,
}

#[test]
fn cow_str_in_enum_variants_is_borrowed() {
    let rows = vec![
        TextEnumRow {
            value: TextEnum::Empty,
        },
        TextEnumRow {
            value: TextEnum::Word(Cow::Borrowed("hello")),
        },
        TextEnumRow {
            value: TextEnum::Named {
                value: Cow::Borrowed("world"),
                id: 7,
            },
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<TextEnumRow<'_>> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
    assert!(matches!(&decoded[1].value, TextEnum::Word(value) if matches!(value, Cow::Borrowed(_))));
    assert!(matches!(&decoded[2].value, TextEnum::Named { value, .. } if matches!(value, Cow::Borrowed(_))));
}
