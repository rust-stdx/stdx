use columnar::{Columnar, DecodeOptions, Encoding, Error};

#[derive(Columnar, Debug, PartialEq, Clone)]
struct AllPrimitives {
    #[columnar(key = 1)]
    bool_value: bool,
    #[columnar(key = 2)]
    u8_value: u8,
    #[columnar(key = 3)]
    u16_value: u16,
    #[columnar(key = 4)]
    u32_value: u32,
    #[columnar(key = 5)]
    u64_value: u64,
    #[columnar(key = 6)]
    u128_value: u128,
    #[columnar(key = 7)]
    i8_value: i8,
    #[columnar(key = 8)]
    i16_value: i16,
    #[columnar(key = 9)]
    i32_value: i32,
    #[columnar(key = 10)]
    i64_value: i64,
    #[columnar(key = 11)]
    i128_value: i128,
    #[columnar(key = 12)]
    usize_value: usize,
    #[columnar(key = 13)]
    isize_value: isize,
    #[columnar(key = 14)]
    f32_value: f32,
    #[columnar(key = 15)]
    f64_value: f64,
    #[columnar(key = 16)]
    char_value: char,
    #[columnar(key = 17)]
    string_value: String,
    #[columnar(key = 18)]
    option_value: Option<u64>,
    #[columnar(key = 19)]
    nested_option: Option<Option<i64>>,
    #[columnar(key = 20)]
    list_value: Vec<u32>,
    #[columnar(key = 21)]
    string_list: Vec<String>,
    #[columnar(key = 22)]
    byte_list: Vec<u8>,
    #[columnar(key = 23)]
    unit_value: (),
}

fn primitive_rows() -> Vec<AllPrimitives> {
    vec![
        AllPrimitives {
            bool_value: false,
            u8_value: 0,
            u16_value: 0,
            u32_value: 0,
            u64_value: 0,
            u128_value: 0,
            i8_value: i8::MIN,
            i16_value: i16::MIN,
            i32_value: i32::MIN,
            i64_value: i64::MIN,
            i128_value: i128::MIN,
            usize_value: 0,
            isize_value: isize::MIN,
            f32_value: -0.0,
            f64_value: -0.0,
            char_value: '\0',
            string_value: String::new(),
            option_value: None,
            nested_option: None,
            list_value: vec![],
            string_list: vec![],
            byte_list: vec![],
            unit_value: (),
        },
        AllPrimitives {
            bool_value: true,
            u8_value: u8::MAX,
            u16_value: u16::MAX,
            u32_value: u32::MAX,
            u64_value: u64::MAX,
            u128_value: u128::MAX,
            i8_value: i8::MAX,
            i16_value: i16::MAX,
            i32_value: i32::MAX,
            i64_value: i64::MAX,
            i128_value: i128::MAX,
            usize_value: usize::MAX,
            isize_value: isize::MAX,
            f32_value: f32::INFINITY,
            f64_value: f64::NEG_INFINITY,
            char_value: '\u{10ffff}',
            string_value: "héllo 🌍".into(),
            option_value: Some(u64::MAX),
            nested_option: Some(None),
            list_value: vec![0, 1, u32::MAX],
            string_list: vec!["".into(), "a".into(), "longer string".into()],
            byte_list: vec![0, 1, 255],
            unit_value: (),
        },
        AllPrimitives {
            bool_value: true,
            u8_value: 42,
            u16_value: 4242,
            u32_value: 424242,
            u64_value: 42424242,
            u128_value: 42,
            i8_value: -1,
            i16_value: -2,
            i32_value: -3,
            i64_value: -4,
            i128_value: -5,
            usize_value: 7,
            isize_value: -7,
            f32_value: 1.5,
            f64_value: 2.25,
            char_value: '中',
            string_value: "Ω".into(),
            option_value: Some(0),
            nested_option: Some(Some(i64::MIN)),
            list_value: vec![7; 33],
            string_list: vec!["repeat".into(); 3],
            byte_list: vec![1, 2, 3, 4, 5],
            unit_value: (),
        },
        AllPrimitives {
            bool_value: false,
            u8_value: 1,
            u16_value: 2,
            u32_value: 3,
            u64_value: 4,
            u128_value: 5,
            i8_value: 0,
            i16_value: 0,
            i32_value: 0,
            i64_value: 0,
            i128_value: 0,
            usize_value: 1,
            isize_value: 0,
            f32_value: f32::MIN,
            f64_value: f64::MAX,
            char_value: 'é',
            string_value: "x".into(),
            option_value: None,
            nested_option: Some(Some(0)),
            list_value: vec![1, 2],
            string_list: vec!["a".into(), "b".into()],
            byte_list: vec![9],
            unit_value: (),
        },
    ]
}

#[test]
fn all_primitives_roundtrip() {
    let rows = primitive_rows();
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<AllPrimitives> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded.len(), rows.len());
    for (a, b) in decoded.iter().zip(rows.iter()) {
        assert_eq!(a, b);
        assert_eq!(a.f32_value.to_bits(), b.f32_value.to_bits());
        assert_eq!(a.f64_value.to_bits(), b.f64_value.to_bits());
    }
}

#[test]
fn empty_rows_roundtrip() {
    let rows: Vec<AllPrimitives> = Vec::new();
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<AllPrimitives> = columnar::from_slice(&bytes).unwrap();
    assert!(decoded.is_empty());
}

#[test]
fn single_row_roundtrip() {
    let rows = vec![primitive_rows().remove(2)];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<AllPrimitives> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

#[test]
fn many_rows_roundtrip() {
    let mut state = 0x1234_5678_9abc_def0u64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut rows = Vec::new();
    for _ in 0..10_000 {
        rows.push(AllPrimitives {
            bool_value: next() & 1 == 0,
            u8_value: next() as u8,
            u16_value: next() as u16,
            u32_value: next() as u32,
            u64_value: next(),
            u128_value: u128::from(next()) << 64 | u128::from(next()),
            i8_value: next() as i8,
            i16_value: next() as i16,
            i32_value: next() as i32,
            i64_value: next() as i64,
            i128_value: (u128::from(next()) << 64 | u128::from(next())) as i128,
            usize_value: next() as usize,
            isize_value: next() as isize,
            f32_value: f32::from_bits(next() as u32),
            f64_value: f64::from_bits(next()),
            char_value: char::from_u32((next() % 0x11_0000) as u32).unwrap_or('?'),
            string_value: format!("row-{}-{}", next() % 10, next() % 3),
            option_value: (next() % 5 != 0).then_some(next()),
            nested_option: match next() % 4 {
                0 => None,
                1 => Some(None),
                _ => Some(Some(next() as i64)),
            },
            list_value: (0..(next() % 4)).map(|_| next() as u32).collect(),
            string_list: (0..(next() % 3)).map(|_| format!("s{}", next() % 5)).collect(),
            byte_list: (0..(next() % 5)).map(|_| next() as u8).collect(),
            unit_value: (),
        });
    }
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<AllPrimitives> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded.len(), rows.len());
    for (a, b) in decoded.iter().zip(rows.iter()) {
        assert_eq!(a.bool_value, b.bool_value);
        assert_eq!(a.u128_value, b.u128_value);
        assert_eq!(a.i128_value, b.i128_value);
        assert_eq!(a.string_value, b.string_value);
        assert_eq!(a.option_value, b.option_value);
        assert_eq!(a.nested_option, b.nested_option);
        assert_eq!(a.list_value, b.list_value);
        assert_eq!(a.string_list, b.string_list);
        assert_eq!(a.byte_list, b.byte_list);
        assert_eq!(a.f32_value.to_bits(), b.f32_value.to_bits());
        assert_eq!(a.f64_value.to_bits(), b.f64_value.to_bits());
    }
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct Nested {
    #[columnar(key = 1)]
    inner: Inner,
    #[columnar(key = 2)]
    optional_inner: Option<Inner>,
    #[columnar(key = 3)]
    list: Vec<Inner>,
    #[columnar(key = 4)]
    optional_list: Option<Vec<Inner>>,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct Inner {
    #[columnar(key = 1)]
    a: u32,
    #[columnar(key = 2)]
    b: String,
}

#[test]
fn nested_structs_roundtrip() {
    let rows = vec![
        Nested {
            inner: Inner {
                a: 1,
                b: "one".into(),
            },
            optional_inner: None,
            list: vec![],
            optional_list: Some(vec![]),
        },
        Nested {
            inner: Inner {
                a: 2,
                b: "two".into(),
            },
            optional_inner: Some(Inner {
                a: 3,
                b: "three".into(),
            }),
            list: vec![Inner {
                a: 4,
                b: "four".into(),
            }],
            optional_list: None,
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Nested> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct TupleStruct(u32, String, Option<u8>);

#[derive(Columnar, Debug, PartialEq, Clone)]
struct UnitStruct;

#[derive(Columnar, Debug, PartialEq, Clone)]
enum Variants {
    Unit,
    Newtype(u64),
    Tuple(i32, String),
    Named { a: bool, b: Option<u64>, c: Vec<String> },
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct EnumHolder {
    #[columnar(key = 1)]
    value: Variants,
    #[columnar(key = 2)]
    optional: Option<Variants>,
    #[columnar(key = 3)]
    list: Vec<Variants>,
}

#[test]
fn tuple_and_unit_structs_roundtrip() {
    let rows = vec![
        TupleStruct(0, String::new(), None),
        TupleStruct(u32::MAX, "x".into(), Some(255)),
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<TupleStruct> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);

    let units = vec![UnitStruct, UnitStruct, UnitStruct];
    let bytes = columnar::to_vec(&units).unwrap();
    let decoded: Vec<UnitStruct> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded.len(), 3);
}

#[test]
fn enums_roundtrip() {
    let rows = vec![
        EnumHolder {
            value: Variants::Unit,
            optional: None,
            list: vec![],
        },
        EnumHolder {
            value: Variants::Newtype(u64::MAX),
            optional: Some(Variants::Tuple(-1, "neg".into())),
            list: vec![Variants::Unit],
        },
        EnumHolder {
            value: Variants::Tuple(0, "zero".into()),
            optional: Some(Variants::Named {
                a: true,
                b: None,
                c: vec!["a".into(), "b".into()],
            }),
            list: vec![
                Variants::Named {
                    a: false,
                    b: Some(1),
                    c: vec![],
                },
                Variants::Newtype(2),
            ],
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<EnumHolder> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);

    let top = vec![Variants::Unit, Variants::Newtype(1)];
    let bytes = columnar::to_vec(&top).unwrap();
    let decoded: Vec<Variants> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, top);
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct Generic<T> {
    #[columnar(key = 1)]
    value: T,
    #[columnar(key = 2)]
    values: Vec<T>,
}

#[test]
fn generic_struct_roundtrip() {
    let rows = vec![
        Generic {
            value: 1u64,
            values: vec![2, 3],
        },
        Generic {
            value: 4u64,
            values: vec![],
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Generic<u64>> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);

    let rows = vec![Generic {
        value: "x".to_string(),
        values: vec!["y".to_string()],
    }];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Generic<String>> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

fn default_skipped() -> u64 {
    99
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct Overrides {
    #[columnar(key = 1, encoding = "packed")]
    packed: u64,
    #[columnar(key = 2, encoding = "for")]
    for_value: i64,
    #[columnar(key = 3, encoding = "varint")]
    varint: i64,
    #[columnar(key = 4, encoding = "rle")]
    rle: u32,
    #[columnar(key = 5, encoding = "dictionary")]
    dictionary: String,
    #[columnar(key = 6, encoding = "lengths")]
    lengths: String,
    #[columnar(key = 7, encoding = "offsets")]
    offsets: String,
    #[columnar(key = 8, encoding = "bitpack")]
    bitpack: bool,
    #[columnar(key = 9, encoding = "list_offsets")]
    list_offsets: Vec<u32>,
    #[columnar(key = 10, encoding = "rle")]
    float_rle: f64,
    #[columnar(key = 11, encoding = "lengths")]
    list_lengths: Vec<u32>,
    #[columnar(key = 12, encoding = "offsets")]
    list_offsets_alias: Vec<u32>,
    #[columnar(skip, default = "default_skipped")]
    skipped: u64,
}

#[test]
fn encoding_overrides_roundtrip() {
    let rows = vec![
        Overrides {
            packed: 0,
            for_value: i64::MIN,
            varint: -1,
            rle: 7,
            dictionary: "eu".into(),
            lengths: "a".into(),
            offsets: "b".into(),
            bitpack: true,
            list_offsets: vec![1, 2, 3],
            float_rle: 1.5,
            list_lengths: vec![1, 2],
            list_offsets_alias: vec![3],
            skipped: 1,
        },
        Overrides {
            packed: u64::MAX,
            for_value: i64::MAX,
            varint: i64::MAX,
            rle: 7,
            dictionary: "us".into(),
            lengths: String::new(),
            offsets: "longer".into(),
            bitpack: false,
            list_offsets: vec![],
            float_rle: 1.5,
            list_lengths: vec![],
            list_offsets_alias: vec![],
            skipped: 2,
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Overrides> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded.len(), 2);
    assert_eq!(decoded[0].packed, 0);
    assert_eq!(decoded[1].packed, u64::MAX);
    assert_eq!(decoded[0].for_value, i64::MIN);
    assert_eq!(decoded[1].for_value, i64::MAX);
    assert_eq!(decoded[0].varint, -1);
    assert_eq!(decoded[1].varint, i64::MAX);
    assert_eq!(decoded[0].rle, 7);
    assert_eq!(decoded[1].rle, 7);
    assert_eq!(decoded[0].dictionary, "eu");
    assert_eq!(decoded[1].dictionary, "us");
    assert_eq!(decoded[0].lengths, "a");
    assert_eq!(decoded[1].lengths, "");
    assert_eq!(decoded[0].offsets, "b");
    assert_eq!(decoded[1].offsets, "longer");
    assert!(decoded[0].bitpack);
    assert!(!decoded[1].bitpack);
    assert_eq!(decoded[0].list_offsets, vec![1, 2, 3]);
    assert_eq!(decoded[1].list_offsets, vec![]);
    assert_eq!(decoded[0].float_rle.to_bits(), 1.5f64.to_bits());
    assert_eq!(decoded[1].float_rle.to_bits(), 1.5f64.to_bits());
    assert_eq!(decoded[0].list_lengths, vec![1, 2]);
    assert_eq!(decoded[1].list_lengths, vec![]);
    assert_eq!(decoded[0].list_offsets_alias, vec![3]);
    assert_eq!(decoded[1].list_offsets_alias, vec![]);
    assert_eq!(decoded[0].skipped, 99);
    assert_eq!(decoded[1].skipped, 99);
}

#[test]
fn limits_are_applied() {
    let rows = vec![AllPrimitives {
        string_value: "hello".into(),
        ..primitive_rows().remove(0)
    }];
    let bytes = columnar::to_vec(&rows).unwrap();
    let options = DecodeOptions {
        max_rows: 0,
        ..DecodeOptions::default()
    };
    assert!(columnar::from_slice_with_options::<AllPrimitives>(&bytes, options).is_err());

    let options = DecodeOptions {
        max_total_bytes: 1,
        ..DecodeOptions::default()
    };
    assert!(columnar::from_slice_with_options::<AllPrimitives>(&bytes, options).is_err());

    let options = DecodeOptions {
        max_string_len: 1,
        ..DecodeOptions::default()
    };
    assert!(columnar::from_slice_with_options::<AllPrimitives>(&bytes, options).is_err());
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct Floats {
    #[columnar(key = 1)]
    a: f32,
    #[columnar(key = 2)]
    b: f64,
}

#[test]
fn float_bits_are_preserved() {
    let rows = vec![
        Floats {
            a: -0.0,
            b: -0.0,
        },
        Floats {
            a: f32::from_bits(0x7fc0_0001),
            b: f64::from_bits(0x7ff8_0000_0000_0001),
        },
        Floats {
            a: f32::from_bits(0x0000_0001),
            b: f64::from_bits(0x0000_0000_0000_0001),
        },
        Floats {
            a: f32::INFINITY,
            b: f64::NEG_INFINITY,
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Floats> = columnar::from_slice(&bytes).unwrap();
    for (a, b) in decoded.iter().zip(rows.iter()) {
        assert_eq!(a.a.to_bits(), b.a.to_bits());
        assert_eq!(a.b.to_bits(), b.b.to_bits());
    }
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct CharRow {
    #[columnar(key = 1)]
    value: char,
}

#[test]
fn char_edge_values_roundtrip() {
    let rows = vec![
        CharRow {
            value: '\0',
        },
        CharRow {
            value: 'é',
        },
        CharRow {
            value: '中',
        },
        CharRow {
            value: '\u{10ffff}',
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<CharRow> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

#[test]
fn encoding_is_deterministic() {
    let rows = primitive_rows();
    let a = columnar::to_vec(&rows).unwrap();
    let b = columnar::to_vec(&rows).unwrap();
    assert_eq!(a, b);
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct NestedContainers {
    #[columnar(key = 1)]
    list_of_lists: Vec<Vec<u64>>,
    #[columnar(key = 2)]
    list_of_options: Vec<Option<u64>>,
    #[columnar(key = 3)]
    optional_list: Option<Vec<Option<u64>>>,
    #[columnar(key = 4)]
    list_of_strings: Vec<Vec<String>>,
}

#[test]
fn nested_containers_roundtrip() {
    let rows = vec![
        NestedContainers {
            list_of_lists: vec![],
            list_of_options: vec![],
            optional_list: None,
            list_of_strings: vec![],
        },
        NestedContainers {
            list_of_lists: vec![vec![]],
            list_of_options: vec![None],
            optional_list: Some(vec![]),
            list_of_strings: vec![vec![]],
        },
        NestedContainers {
            list_of_lists: vec![vec![1, 2], vec![], vec![3]],
            list_of_options: vec![Some(1), None, Some(3)],
            optional_list: Some(vec![None, Some(0), None]),
            list_of_strings: vec![vec!["a".into()], vec![], vec!["b".into(), "c".into()]],
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<NestedContainers> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct OptionRows {
    #[columnar(key = 1)]
    value: Option<u64>,
}

#[test]
fn option_all_none_all_some_and_mixed() {
    let cases = vec![
        vec![
            OptionRows {
                value: None,
            },
            OptionRows {
                value: None,
            },
        ],
        vec![
            OptionRows {
                value: Some(1),
            },
            OptionRows {
                value: Some(2),
            },
        ],
        vec![
            OptionRows {
                value: None,
            },
            OptionRows {
                value: Some(2),
            },
        ],
        vec![],
    ];
    for rows in cases {
        let bytes = columnar::to_vec(&rows).unwrap();
        let decoded: Vec<OptionRows> = columnar::from_slice(&bytes).unwrap();
        assert_eq!(decoded, rows);
    }
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct StringVariants {
    #[columnar(key = 1, encoding = "dictionary")]
    dictionary: String,
    #[columnar(key = 2, encoding = "rle")]
    rle: String,
    #[columnar(key = 3, encoding = "offsets")]
    offsets: String,
}

#[test]
fn string_encoding_edge_cases() {
    let rows = vec![
        StringVariants {
            dictionary: String::new(),
            rle: String::new(),
            offsets: String::new(),
        },
        StringVariants {
            dictionary: "a".into(),
            rle: "a".into(),
            offsets: "a".into(),
        },
        StringVariants {
            dictionary: "a".into(),
            rle: "a".into(),
            offsets: "a".into(),
        },
        StringVariants {
            dictionary: "b".into(),
            rle: "b".into(),
            offsets: "b".into(),
        },
        StringVariants {
            dictionary: "b".into(),
            rle: "b".into(),
            offsets: "b".into(),
        },
        StringVariants {
            dictionary: String::new(),
            rle: String::new(),
            offsets: String::new(),
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<StringVariants> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct SkipDefault {
    #[columnar(key = 1)]
    value: u64,
    #[columnar(skip)]
    skipped: u64,
}

#[test]
fn skipped_field_uses_default() {
    let rows = vec![SkipDefault {
        value: 1,
        skipped: 42,
    }];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<SkipDefault> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(
        decoded,
        vec![SkipDefault {
            value: 1,
            skipped: 0
        }]
    );
}

#[derive(Columnar, Debug, PartialEq, Clone)]
enum SkippedVariantField {
    A {
        #[columnar(skip)]
        skipped: u64,
        kept: u64,
    },
    B(u64),
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct EnumSkipHolder {
    #[columnar(key = 1)]
    value: SkippedVariantField,
}

#[test]
fn skipped_enum_field_uses_default() {
    let rows = vec![EnumSkipHolder {
        value: SkippedVariantField::A {
            skipped: 1,
            kept: 2,
        },
    }];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<EnumSkipHolder> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(
        decoded,
        vec![EnumSkipHolder {
            value: SkippedVariantField::A {
                skipped: 0,
                kept: 2
            }
        }]
    );
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct MapRow {
    #[columnar(key = 1)]
    value: std::collections::BTreeMap<String, u64>,
    #[columnar(key = 2)]
    list: Vec<std::collections::BTreeMap<u32, String>>,
}

#[test]
fn btree_maps_roundtrip() {
    use std::collections::BTreeMap;
    let mut a = BTreeMap::new();
    a.insert("a".to_string(), 1);
    a.insert("b".to_string(), 2);
    let mut b = BTreeMap::new();
    b.insert(1u32, "one".to_string());
    b.insert(2u32, "two".to_string());
    let rows = vec![
        MapRow {
            value: BTreeMap::new(),
            list: vec![],
        },
        MapRow {
            value: a,
            list: vec![BTreeMap::new(), b],
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<MapRow> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq, Clone)]
#[columnar(crate = "columnar")]
struct CustomCratePath {
    #[columnar(key = 1)]
    value: u64,
}

#[test]
fn custom_crate_path_attribute() {
    let rows = vec![CustomCratePath {
        value: 7,
    }];
    let bytes = columnar::to_vec(&rows).unwrap();
    assert_eq!(columnar::from_slice::<CustomCratePath>(&bytes).unwrap(), rows);
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct ByteRow {
    #[columnar(key = 1)]
    unsigned: u8,
    #[columnar(key = 2)]
    signed: i8,
    #[columnar(key = 3)]
    boolean: bool,
}

#[test]
fn exhaustive_u8_i8_bool_values() {
    let rows: Vec<ByteRow> = (0..=255u8)
        .map(|value| ByteRow {
            unsigned: value,
            signed: value as i8,
            boolean: value & 1 == 0,
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<ByteRow> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct DeltaEncodings {
    #[columnar(key = 1, encoding = "delta_rle")]
    delta_rle: u64,
    #[columnar(key = 2, encoding = "delta_of_delta")]
    delta_of_delta: i64,
    #[columnar(key = 3, encoding = "delta_rle")]
    narrow: u8,
    #[columnar(key = 4, encoding = "delta_of_delta")]
    optional: Option<u32>,
}

fn delta_row(delta_rle: u64, delta_of_delta: i64, narrow: u8, optional: Option<u32>) -> DeltaEncodings {
    DeltaEncodings {
        delta_rle,
        delta_of_delta,
        narrow,
        optional,
    }
}

#[test]
fn delta_encodings_roundtrip() {
    let increasing: Vec<_> = (0..32u64)
        .map(|index| {
            delta_row(
                1_000_000 + index,
                -1_000_000 + index as i64 * 7,
                index as u8,
                Some(index as u32),
            )
        })
        .collect();
    let decreasing: Vec<_> = (0..32u64)
        .map(|index| delta_row(1_000_000 - index, 1_000_000 - index as i64 * 3, 255 - index as u8, None))
        .collect();
    let jittered: Vec<_> = (0..64u64)
        .map(|index| {
            let delta = if index % 5 == 0 {
                1001
            } else if index % 7 == 0 {
                999
            } else {
                1000
            };
            delta_row(
                1_700_000_000_000 + index * delta,
                index as i64 * 1000 + (index % 3) as i64,
                (index % 251) as u8,
                Some(1_000 - index as u32),
            )
        })
        .collect();
    let cases = vec![
        vec![],
        vec![delta_row(0, 0, 0, None)],
        vec![delta_row(7, 7, 7, Some(7))],
        increasing,
        decreasing,
        jittered,
        vec![
            delta_row(u64::MAX, i64::MIN, 255, Some(u32::MAX)),
            delta_row(0, i64::MAX, 0, Some(0)),
            delta_row(u64::MAX, 0, 1, None),
            delta_row(1, i64::MIN, 254, Some(1)),
        ],
    ];
    for rows in cases {
        let bytes = columnar::to_vec(&rows).unwrap();
        let decoded: Vec<DeltaEncodings> = columnar::from_slice(&bytes).unwrap();
        assert_eq!(decoded, rows);
    }
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct DeltaOfDeltaU64 {
    #[columnar(key = 1, encoding = "delta_of_delta")]
    value: u64,
}

#[test]
fn delta_of_delta_u64_extremes_roundtrip() {
    let rows = vec![
        DeltaOfDeltaU64 {
            value: u64::MAX,
        },
        DeltaOfDeltaU64 {
            value: 0,
        },
        DeltaOfDeltaU64 {
            value: u64::MAX,
        },
        DeltaOfDeltaU64 {
            value: 1,
        },
        DeltaOfDeltaU64 {
            value: 1,
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<DeltaOfDeltaU64> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct DeltaDelegated {
    #[columnar(key = 1, encoding = "delta_rle")]
    index: usize,
    #[columnar(key = 2, encoding = "delta_of_delta")]
    offset: isize,
    #[columnar(key = 3, encoding = "delta_rle")]
    letter: char,
    #[columnar(key = 4, encoding = "delta_of_delta")]
    codepoint: char,
}

#[test]
fn delta_encodings_support_delegated_types() {
    let rows = vec![
        DeltaDelegated {
            index: 0,
            offset: -1_000,
            letter: 'a',
            codepoint: '\u{10ffff}',
        },
        DeltaDelegated {
            index: 1,
            offset: -999,
            letter: 'b',
            codepoint: '\u{10fffe}',
        },
        DeltaDelegated {
            index: 2,
            offset: -998,
            letter: 'c',
            codepoint: '\u{10fffd}',
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<DeltaDelegated> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct DeltaRleWide {
    #[columnar(key = 1, encoding = "delta_rle")]
    value: u128,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct DeltaOfDeltaWide {
    #[columnar(key = 1, encoding = "delta_of_delta")]
    value: i128,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct DeltaOfDeltaFloat {
    #[columnar(key = 1, encoding = "delta_of_delta")]
    value: f64,
}

#[test]
fn delta_encodings_reject_wide_integers_and_floats() {
    assert!(matches!(
        columnar::to_vec(&[DeltaRleWide {
            value: 1,
        }]),
        Err(Error::UnsupportedEncoding(Encoding::DeltaRle))
    ));
    assert!(matches!(
        columnar::to_vec(&[DeltaOfDeltaWide {
            value: 1,
        }]),
        Err(Error::UnsupportedEncoding(Encoding::DeltaOfDelta))
    ));
    assert!(matches!(
        columnar::to_vec(&[DeltaOfDeltaFloat {
            value: 1.0,
        }]),
        Err(Error::UnsupportedEncoding(Encoding::DeltaOfDelta))
    ));
}

macro_rules! auto_packed_case {
    ($module:ident, $auto:ident, $packed:ident, $for_rows:ident, $ty:ty, $full_a:expr, $full_b:expr, $narrow_a:expr, $narrow_b:expr) => {
        mod $module {
            use super::*;

            #[derive(Columnar, Debug, PartialEq)]
            struct $auto {
                #[columnar(key = 1)]
                value: $ty,
            }

            #[derive(Columnar, Debug, PartialEq)]
            struct $packed {
                #[columnar(key = 1, encoding = "packed")]
                value: $ty,
            }

            #[derive(Columnar, Debug, PartialEq)]
            struct $for_rows {
                #[columnar(key = 1, encoding = "for")]
                value: $ty,
            }

            #[test]
            fn auto_uses_packed_for_full_width_values() {
                let auto = vec![
                    $auto {
                        value: $full_a,
                    },
                    $auto {
                        value: $full_b,
                    },
                ];
                let packed = vec![
                    $packed {
                        value: $full_a,
                    },
                    $packed {
                        value: $full_b,
                    },
                ];
                let auto_bytes = columnar::to_vec(&auto).unwrap();
                assert_eq!(auto_bytes, columnar::to_vec(&packed).unwrap());
                let decoded: Vec<$auto> = columnar::from_slice(&auto_bytes).unwrap();
                assert_eq!(decoded, auto);
            }

            #[test]
            fn auto_uses_for_for_narrow_values() {
                let auto = vec![
                    $auto {
                        value: $narrow_a,
                    },
                    $auto {
                        value: $narrow_b,
                    },
                ];
                let for_rows = vec![
                    $for_rows {
                        value: $narrow_a,
                    },
                    $for_rows {
                        value: $narrow_b,
                    },
                ];
                assert_eq!(columnar::to_vec(&auto).unwrap(), columnar::to_vec(&for_rows).unwrap());
            }

            #[test]
            fn old_for_full_width_data_still_decodes() {
                let for_rows = vec![
                    $for_rows {
                        value: $full_a,
                    },
                    $for_rows {
                        value: $full_b,
                    },
                ];
                let bytes = columnar::to_vec(&for_rows).unwrap();
                let decoded: Vec<$auto> = columnar::from_slice(&bytes).unwrap();
                assert_eq!(decoded[0].value, $full_a);
                assert_eq!(decoded[1].value, $full_b);
            }
        }
    };
}

auto_packed_case!(auto_packed_u8, AutoU8, PackedU8, ForU8, u8, 0, u8::MAX, 1, 2);
auto_packed_case!(auto_packed_u16, AutoU16, PackedU16, ForU16, u16, 0, u16::MAX, 100, 200);
auto_packed_case!(auto_packed_u32, AutoU32, PackedU32, ForU32, u32, 0, u32::MAX, 100, 200);
auto_packed_case!(auto_packed_u64, AutoU64, PackedU64, ForU64, u64, 0, u64::MAX, 100, 200);
auto_packed_case!(auto_packed_i8, AutoI8, PackedI8, ForI8, i8, i8::MIN, i8::MAX, -1, 1);
auto_packed_case!(auto_packed_i16, AutoI16, PackedI16, ForI16, i16, i16::MIN, i16::MAX, -100, 100);
auto_packed_case!(auto_packed_i32, AutoI32, PackedI32, ForI32, i32, i32::MIN, i32::MAX, -100, 100);
auto_packed_case!(auto_packed_i64, AutoI64, PackedI64, ForI64, i64, i64::MIN, i64::MAX, -100, 100);

#[derive(Columnar, Debug, PartialEq, Clone)]
struct ListEdges {
    #[columnar(key = 1)]
    lengths: Vec<u64>,
    #[columnar(key = 2, encoding = "list_offsets")]
    offsets: Vec<u64>,
}

#[test]
fn list_empty_row_edges_roundtrip() {
    let cases = vec![
        vec![],
        vec![ListEdges {
            lengths: vec![],
            offsets: vec![],
        }],
        vec![
            ListEdges {
                lengths: vec![],
                offsets: vec![],
            },
            ListEdges {
                lengths: vec![],
                offsets: vec![],
            },
            ListEdges {
                lengths: vec![1, 2],
                offsets: vec![3, 4],
            },
            ListEdges {
                lengths: vec![],
                offsets: vec![],
            },
            ListEdges {
                lengths: vec![5],
                offsets: vec![6],
            },
            ListEdges {
                lengths: vec![],
                offsets: vec![],
            },
        ],
    ];
    for rows in cases {
        let bytes = columnar::to_vec(&rows).unwrap();
        let decoded: Vec<ListEdges> = columnar::from_slice(&bytes).unwrap();
        assert_eq!(decoded, rows);
    }
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct StringListEdges {
    #[columnar(key = 1)]
    values: Vec<String>,
}

#[test]
fn list_string_empty_values_roundtrip() {
    let rows = vec![
        StringListEdges {
            values: vec![],
        },
        StringListEdges {
            values: vec![String::new()],
        },
        StringListEdges {
            values: vec![String::new(), "a".into(), String::new()],
        },
        StringListEdges {
            values: vec![],
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<StringListEdges> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}
