use columnar::Columnar;

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct Sale {
    #[columnar(key = 1)]
    id: u64,
    #[columnar(key = 2)]
    amount: f64,
    #[columnar(key = 3)]
    region: String,
}

#[test]
fn struct_rows_iterate() {
    let rows = vec![
        Sale {
            id: 1,
            amount: 1.5,
            region: "eu".into(),
        },
        Sale {
            id: 2,
            amount: 2.5,
            region: "us".into(),
        },
        Sale {
            id: 3,
            amount: 3.5,
            region: "".into(),
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let iter = columnar::iter_from_slice::<Sale>(&bytes).unwrap();
    assert_eq!(iter.len(), rows.len());
    let decoded: Vec<Sale> = iter.map(|row| row.unwrap()).collect();
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
enum Kind {
    None,
    Count(u64),
    Named { name: String, id: u64 },
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct EnumRow {
    #[columnar(key = 1)]
    kind: Kind,
    #[columnar(key = 2)]
    id: u64,
}

#[test]
fn enum_rows_iterate() {
    let rows = vec![
        EnumRow {
            kind: Kind::None,
            id: 0,
        },
        EnumRow {
            kind: Kind::Count(7),
            id: 1,
        },
        EnumRow {
            kind: Kind::Named {
                name: "hello".into(),
                id: 9,
            },
            id: 2,
        },
        EnumRow {
            kind: Kind::Count(7),
            id: 3,
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<EnumRow> = columnar::iter_from_slice::<EnumRow>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(decoded, rows);
}

use std::{borrow::Cow, collections::BTreeMap};

use alloc_counter::measure;

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct Inner {
    #[columnar(key = 1)]
    id: u64,
    #[columnar(key = 2)]
    name: String,
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct Outer {
    #[columnar(key = 1)]
    id: u64,
    #[columnar(key = 2)]
    inner: Inner,
    #[columnar(key = 3)]
    active: bool,
}

#[test]
fn nested_struct_rows_iterate() {
    let rows = vec![
        Outer {
            id: 1,
            inner: Inner {
                id: 10,
                name: "a".into(),
            },
            active: true,
        },
        Outer {
            id: 2,
            inner: Inner {
                id: 11,
                name: "".into(),
            },
            active: false,
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Outer> = columnar::iter_from_slice::<Outer>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct Containers<'a> {
    #[columnar(key = 1)]
    maybe: Option<u64>,
    #[columnar(key = 2)]
    list: Vec<u64>,
    #[columnar(key = 3)]
    text: Cow<'a, str>,
    #[columnar(key = 4)]
    bytes: Cow<'a, [u8]>,
    #[columnar(key = 5)]
    map: BTreeMap<u32, String>,
    #[columnar(key = 6, encoding = "list_offsets")]
    offsets: Vec<u64>,
}

#[test]
fn container_rows_iterate() {
    let rows = vec![
        Containers {
            maybe: None,
            list: vec![],
            text: Cow::Borrowed(""),
            bytes: Cow::Borrowed(&[]),
            map: BTreeMap::new(),
            offsets: vec![],
        },
        Containers {
            maybe: Some(7),
            list: vec![1, 2, 3],
            text: Cow::Borrowed("hello"),
            bytes: Cow::Borrowed(&[1, 2, 3]),
            map: BTreeMap::from([(1, "one".into()), (2, "two".into())]),
            offsets: vec![4, 5],
        },
        Containers {
            maybe: Some(0),
            list: vec![9],
            text: Cow::Borrowed("β"),
            bytes: Cow::Borrowed(&[0xff]),
            map: BTreeMap::from([(9, "nine".into())]),
            offsets: vec![6],
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Containers<'_>> = columnar::iter_from_slice::<Containers<'_>>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(decoded.len(), rows.len());
    for (decoded, expected) in decoded.iter().zip(rows.iter()) {
        assert_eq!(decoded, expected);
        assert!(matches!(decoded.text, Cow::Borrowed(_)));
        assert!(matches!(decoded.bytes, Cow::Borrowed(_)));
    }
}

fn default_value() -> u64 {
    77
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct Defaults {
    #[columnar(key = 1)]
    a: u64,
    #[columnar(key = 2, default = "default_value")]
    b: u64,
    #[columnar(skip)]
    c: u64,
}

#[test]
fn default_and_skipped_fields_iterate() {
    let rows = vec![
        Defaults {
            a: 1,
            b: 2,
            c: 3,
        },
        Defaults {
            a: 4,
            b: 5,
            c: 6,
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let eager: Vec<Defaults> = columnar::from_slice(&bytes).unwrap();
    let lazy: Vec<Defaults> = columnar::iter_from_slice::<Defaults>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(eager, lazy);
    assert_eq!(
        lazy[0],
        Defaults {
            a: 1,
            b: 2,
            c: 0
        }
    );
}

#[test]
fn missing_default_field_iterates() {
    // { 0: [1, 1], 1: [4, [5, 0, bytes([])]] }
    let mut e = cbor::Encoder::new(Vec::new());
    e.map(2).unwrap();
    e.u64(0).unwrap();
    e.array(2).unwrap();
    e.u64(1).unwrap();
    e.u64(1).unwrap();
    e.u64(1).unwrap();
    e.array(2).unwrap();
    e.u64(4).unwrap();
    e.array(3).unwrap();
    e.u64(5).unwrap();
    e.u64(0).unwrap();
    e.bytes(&[]).unwrap();
    let bytes = e.into_inner();
    let decoded: Vec<Defaults> = columnar::iter_from_slice::<Defaults>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(
        decoded,
        vec![Defaults {
            a: 5,
            b: 77,
            c: 0
        }]
    );
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct Generic<T> {
    #[columnar(key = 1)]
    value: T,
    #[columnar(key = 2)]
    list: Vec<T>,
}

#[test]
fn generic_rows_iterate() {
    let rows = vec![
        Generic {
            value: 1u64,
            list: vec![2, 3],
        },
        Generic {
            value: 4u64,
            list: vec![],
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Generic<u64>> = columnar::iter_from_slice::<Generic<u64>>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable, borrow = "'a")]
struct TwoLifetimes<'a, 'b> {
    #[columnar(key = 1)]
    text: Cow<'a, str>,
    #[columnar(key = 2)]
    marker: std::marker::PhantomData<&'b ()>,
}

#[test]
fn multiple_lifetime_rows_iterate() {
    let rows = vec![TwoLifetimes::<'_, 'static> {
        text: Cow::Borrowed("hello"),
        marker: std::marker::PhantomData,
    }];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<TwoLifetimes<'_, 'static>> = columnar::iter_from_slice::<TwoLifetimes<'_, 'static>>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(decoded.len(), 1);
    assert!(matches!(decoded[0].text, Cow::Borrowed(_)));
}

#[test]
fn empty_batch_iterates() {
    let rows: Vec<Sale> = vec![];
    let bytes = columnar::to_vec(&rows).unwrap();
    let iter = columnar::iter_from_slice::<Sale>(&bytes).unwrap();
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.count(), 0);
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct Numeric {
    #[columnar(key = 1)]
    a: u64,
    #[columnar(key = 2)]
    b: i64,
    #[columnar(key = 3)]
    c: u32,
    #[columnar(key = 4)]
    d: i32,
    #[columnar(key = 5)]
    e: f64,
    #[columnar(key = 6)]
    f: bool,
    #[columnar(key = 7)]
    g: u16,
    #[columnar(key = 8)]
    h: i16,
}

fn numeric_rows(count: usize) -> Vec<Numeric> {
    (0..count)
        .map(|index| Numeric {
            a: index as u64,
            b: -(index as i64),
            c: index as u32,
            d: -(index as i32),
            e: index as f64,
            f: index % 2 == 0,
            g: index as u16,
            h: -(index as i16),
        })
        .collect()
}

#[test]
fn numeric_iter_allocates_only_the_row() {
    let rows = numeric_rows(1_000);
    let bytes = columnar::to_vec(&rows).unwrap();
    let (_, counters) = measure(|| columnar::iter_from_slice::<Numeric>(&bytes).unwrap().take(1).count());
    assert!(counters.calls <= 1, "lazy numeric iteration allocated {} times", counters.calls);
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct TextRow<'a> {
    #[columnar(key = 1)]
    first: Cow<'a, str>,
    #[columnar(key = 2)]
    second: Cow<'a, str>,
}

#[test]
fn borrowed_text_iteration_does_not_allocate() {
    let rows: Vec<TextRow<'_>> = (0..1_000)
        .map(|_| TextRow {
            first: Cow::Borrowed("constant"),
            second: Cow::Borrowed("another constant"),
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();
    let (_, counters) = measure(|| {
        columnar::iter_from_slice::<TextRow<'_>>(&bytes)
            .unwrap()
            .take(1)
            .count()
    });
    assert!(
        counters.calls == 0,
        "borrowed text iteration allocated {} times",
        counters.calls
    );
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct OwnedTextRow {
    #[columnar(key = 1)]
    first: String,
    #[columnar(key = 2)]
    second: String,
}

#[test]
fn owned_text_iteration_only_allocates_consumed_rows() {
    let rows: Vec<OwnedTextRow> = (0..1_000)
        .map(|_| OwnedTextRow {
            first: "constant".into(),
            second: "another constant".into(),
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();
    let (_, counters) = measure(|| {
        columnar::iter_from_slice::<OwnedTextRow>(&bytes)
            .unwrap()
            .take(1)
            .count()
    });
    assert!(
        counters.calls <= 3,
        "owned text iteration allocated {} times for one row",
        counters.calls
    );
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct StructList {
    #[columnar(key = 1)]
    items: Vec<Inner>,
    #[columnar(key = 2)]
    maybe: Option<Inner>,
}

#[test]
fn lists_and_options_of_structs_stream() {
    let rows = vec![
        StructList {
            items: vec![],
            maybe: None,
        },
        StructList {
            items: vec![
                Inner {
                    id: 1,
                    name: "a".into(),
                },
                Inner {
                    id: 2,
                    name: "b".into(),
                },
            ],
            maybe: Some(Inner {
                id: 3,
                name: "c".into(),
            }),
        },
        StructList {
            items: vec![Inner {
                id: 4,
                name: "".into(),
            }],
            maybe: Some(Inner {
                id: 5,
                name: "e".into(),
            }),
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<StructList> = columnar::iter_from_slice::<StructList>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
enum SkippedEnum {
    A {
        #[columnar(skip)]
        skipped: u64,
        kept: u64,
    },
    B(u64),
    C,
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct SkippedEnumRow {
    #[columnar(key = 1)]
    value: SkippedEnum,
}

#[test]
fn enum_skipped_field_streams() {
    let rows = vec![
        SkippedEnumRow {
            value: SkippedEnum::A {
                skipped: 1,
                kept: 2,
            },
        },
        SkippedEnumRow {
            value: SkippedEnum::B(3),
        },
        SkippedEnumRow {
            value: SkippedEnum::C,
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<SkippedEnumRow> = columnar::iter_from_slice::<SkippedEnumRow>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(
        decoded[0].value,
        SkippedEnum::A {
            skipped: 0,
            kept: 2
        }
    );
    assert_eq!(decoded[1].value, SkippedEnum::B(3));
    assert_eq!(decoded[2].value, SkippedEnum::C);
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct VarintRow {
    #[columnar(key = 1, encoding = "varint")]
    value: u64,
}

#[test]
fn varint_trailing_data_is_rejected_at_construction() {
    // { 0: [1, 2], 1: [6, bytes([1])] } has one varint for two rows.
    let mut e = cbor::Encoder::new(Vec::new());
    e.map(2).unwrap();
    e.u64(0).unwrap();
    e.array(2).unwrap();
    e.u64(1).unwrap();
    e.u64(2).unwrap();
    e.u64(1).unwrap();
    e.array(2).unwrap();
    e.u64(6).unwrap();
    e.bytes(&[1]).unwrap();
    let bytes = e.into_inner();
    assert!(columnar::iter_from_slice::<VarintRow>(&bytes).is_err());
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct OneString {
    #[columnar(key = 1)]
    value: String,
}

#[test]
fn late_utf8_errors_are_items() {
    // { 0: [1, 2], 1: [9, [bytes([1, 1]), bytes([0x61, 0xff])]] }
    let mut e = cbor::Encoder::new(Vec::new());
    e.map(2).unwrap();
    e.u64(0).unwrap();
    e.array(2).unwrap();
    e.u64(1).unwrap();
    e.u64(2).unwrap();
    e.u64(1).unwrap();
    e.array(2).unwrap();
    e.u64(9).unwrap();
    e.array(2).unwrap();
    e.bytes(&[1, 1]).unwrap();
    e.bytes(&[0x61, 0xff]).unwrap();
    let bytes = e.into_inner();
    let mut rows = columnar::iter_from_slice::<OneString>(&bytes).unwrap();
    assert_eq!(
        rows.next().unwrap().unwrap(),
        OneString {
            value: "a".into()
        }
    );
    assert!(rows.next().unwrap().is_err());
}

#[test]
fn malformed_envelopes_are_rejected_at_construction() {
    // Missing header: { 1: [4, [0, 0, bytes([])]] }
    let mut e = cbor::Encoder::new(Vec::new());
    e.map(1).unwrap();
    e.u64(1).unwrap();
    e.array(2).unwrap();
    e.u64(4).unwrap();
    e.array(3).unwrap();
    e.u64(0).unwrap();
    e.u64(0).unwrap();
    e.bytes(&[]).unwrap();
    assert!(columnar::iter_from_slice::<VarintRow>(&e.into_inner()).is_err());

    // Duplicate header key.
    let mut e = cbor::Encoder::new(Vec::new());
    e.map(2).unwrap();
    e.u64(0).unwrap();
    e.array(2).unwrap();
    e.u64(1).unwrap();
    e.u64(0).unwrap();
    e.u64(0).unwrap();
    e.array(2).unwrap();
    e.u64(1).unwrap();
    e.u64(0).unwrap();
    assert!(columnar::iter_from_slice::<Sale>(&e.into_inner()).is_err());
}

#[test]
fn full_lazy_iteration_avoids_column_vectors() {
    let rows = numeric_rows(10_000);
    let bytes = columnar::to_vec(&rows).unwrap();
    let (eager_rows, eager) = measure(|| columnar::from_slice::<Numeric>(&bytes).unwrap());
    let (lazy_rows, lazy) = measure(|| {
        columnar::iter_from_slice::<Numeric>(&bytes)
            .unwrap()
            .map(|row| row.unwrap())
            .collect::<Vec<_>>()
    });
    assert_eq!(eager_rows, lazy_rows);
    assert!(
        lazy.calls < eager.calls,
        "lazy iteration used {} allocations, eager used {}",
        lazy.calls,
        eager.calls
    );
    assert!(
        lazy.bytes < eager.bytes,
        "lazy iteration used {} bytes, eager used {}",
        lazy.bytes,
        eager.bytes
    );
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct ListRow {
    #[columnar(key = 1)]
    values: Vec<u64>,
}

#[test]
fn list_iteration_only_builds_consumed_rows() {
    let rows: Vec<ListRow> = (0..1_000)
        .map(|_| ListRow {
            values: (0..10).collect(),
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();
    let (_, counters) = measure(|| columnar::iter_from_slice::<ListRow>(&bytes).unwrap().take(1).count());
    assert!(counters.calls <= 2, "one list row allocated {} times", counters.calls);
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct UnitStruct;

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct TupleStruct(u64, String, bool);

#[test]
fn tuple_and_unit_structs_iterate() {
    let rows = vec![UnitStruct, UnitStruct, UnitStruct];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<UnitStruct> = columnar::iter_from_slice::<UnitStruct>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(decoded, rows);

    let rows = vec![TupleStruct(1, "a".into(), true), TupleStruct(2, "".into(), false)];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<TupleStruct> = columnar::iter_from_slice::<TupleStruct>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
enum UnitEnum {
    A,
    B,
    C,
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct UnitEnumRow {
    #[columnar(key = 1)]
    value: UnitEnum,
}

#[test]
fn unit_only_enum_iterates() {
    let rows = vec![
        UnitEnumRow {
            value: UnitEnum::A,
        },
        UnitEnumRow {
            value: UnitEnum::C,
        },
        UnitEnumRow {
            value: UnitEnum::B,
        },
        UnitEnumRow {
            value: UnitEnum::A,
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<UnitEnumRow> = columnar::iter_from_slice::<UnitEnumRow>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(decoded, rows);
}

#[test]
fn top_level_enum_iterates() {
    let rows = vec![
        Kind::None,
        Kind::Count(7),
        Kind::Named {
            name: "hello".into(),
            id: 9,
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Kind> = columnar::iter_from_slice::<Kind>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(decoded, rows);
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct NestedContainers<'a> {
    #[columnar(key = 1)]
    list_of_lists: Vec<Vec<u64>>,
    #[columnar(key = 2)]
    list_of_options: Vec<Option<u64>>,
    #[columnar(key = 3)]
    optional_list: Option<Vec<Option<u64>>>,
    #[columnar(key = 4)]
    list_of_strings: Vec<Vec<Cow<'a, str>>>,
}

#[test]
fn nested_containers_stream() {
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
            list_of_strings: vec![
                vec![Cow::Borrowed("a")],
                vec![],
                vec![Cow::Borrowed("b"), Cow::Borrowed("c")],
            ],
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<NestedContainers<'_>> = columnar::iter_from_slice::<NestedContainers<'_>>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(decoded.len(), rows.len());
    for (decoded, expected) in decoded.iter().zip(rows.iter()) {
        assert_eq!(decoded, expected);
        for list in &decoded.list_of_strings {
            for value in list {
                assert!(matches!(value, Cow::Borrowed(_)));
            }
        }
    }
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct Encoded {
    #[columnar(key = 1, encoding = "rle")]
    rle: u64,
    #[columnar(key = 2, encoding = "delta_rle")]
    delta_rle: u64,
    #[columnar(key = 3, encoding = "delta_of_delta")]
    delta_of_delta: i64,
    #[columnar(key = 4, encoding = "dictionary")]
    dictionary: String,
    #[columnar(key = 5, encoding = "offsets")]
    offsets: String,
    #[columnar(key = 6, encoding = "varint")]
    varint: i64,
    #[columnar(key = 7, encoding = "bitpack")]
    bitpack: bool,
    #[columnar(key = 8, encoding = "packed")]
    packed: u64,
    #[columnar(key = 9, encoding = "for")]
    for_value: i64,
    #[columnar(key = 10, encoding = "list_offsets")]
    list: Vec<u64>,
    #[columnar(key = 11, encoding = "rle")]
    float_rle: f64,
}

#[test]
fn all_encoding_variants_stream() {
    let rows: Vec<Encoded> = (0..64u64)
        .map(|index| Encoded {
            rle: index / 8,
            delta_rle: 1_000 + index,
            delta_of_delta: -1_000 + index as i64 * 7,
            dictionary: ["a", "b", "longer", "a"][index as usize % 4].into(),
            offsets: format!("value-{index}"),
            varint: -(index as i64) * 3,
            bitpack: index % 2 == 0,
            packed: u64::MAX - index,
            for_value: i64::MIN + index as i64,
            list: (0..index % 4).collect(),
            float_rle: (index / 8) as f64,
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();
    let eager: Vec<Encoded> = columnar::from_slice(&bytes).unwrap();
    let lazy: Vec<Encoded> = columnar::iter_from_slice::<Encoded>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(eager, rows);
    assert_eq!(lazy, rows);
}

#[derive(Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct Scalars {
    #[columnar(key = 1)]
    u128_value: u128,
    #[columnar(key = 2)]
    i128_value: i128,
    #[columnar(key = 3)]
    usize_value: usize,
    #[columnar(key = 4)]
    isize_value: isize,
    #[columnar(key = 5)]
    char_value: char,
    #[columnar(key = 6)]
    u8_value: u8,
    #[columnar(key = 7)]
    bool_value: bool,
    #[columnar(key = 8)]
    f32_value: f32,
    #[columnar(key = 9)]
    f64_value: f64,
}

#[test]
fn delegated_scalars_stream() {
    let rows = vec![
        Scalars {
            u128_value: 0,
            i128_value: i128::MIN,
            usize_value: 0,
            isize_value: isize::MIN,
            char_value: '\0',
            u8_value: 0,
            bool_value: false,
            f32_value: -0.0,
            f64_value: 0.0,
        },
        Scalars {
            u128_value: u128::MAX,
            i128_value: i128::MAX,
            usize_value: usize::MAX,
            isize_value: isize::MAX,
            char_value: '\u{10ffff}',
            u8_value: u8::MAX,
            bool_value: true,
            f32_value: f32::INFINITY,
            f64_value: f64::NEG_INFINITY,
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let eager: Vec<Scalars> = columnar::from_slice(&bytes).unwrap();
    let lazy: Vec<Scalars> = columnar::iter_from_slice::<Scalars>(&bytes)
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    assert_eq!(eager, rows);
    assert_eq!(lazy, rows);
}
