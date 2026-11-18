use columnar::Columnar;

#[derive(Columnar, Debug, PartialEq, Clone)]
struct Basic {
    #[columnar(key = 1)]
    id: u64,
    #[columnar(key = 2)]
    name: String,
    #[columnar(key = 3)]
    active: bool,
    #[columnar(key = 4)]
    score: Option<u64>,
    #[columnar(key = 5)]
    tags: Vec<u64>,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct Tuple(u32, String);

#[derive(Columnar, Debug, PartialEq, Clone)]
enum Kind {
    Unit,
    Number(u64),
    Pair { left: i32, right: String },
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct Nested {
    #[columnar(key = 1)]
    basic: Basic,
    #[columnar(key = 2)]
    kind: Kind,
    #[columnar(key = 3)]
    maybe: Option<Option<u64>>,
}

#[test]
fn basic_roundtrip() {
    let rows = vec![
        Basic {
            id: 1,
            name: "a".into(),
            active: true,
            score: Some(10),
            tags: vec![1, 2],
        },
        Basic {
            id: 2,
            name: "b".into(),
            active: false,
            score: None,
            tags: vec![],
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Basic> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

#[test]
fn tuple_roundtrip() {
    let rows = vec![Tuple(1, "x".into()), Tuple(2, "y".into())];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Tuple> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

#[test]
fn enum_roundtrip() {
    let rows = vec![
        Kind::Unit,
        Kind::Number(7),
        Kind::Pair {
            left: -1,
            right: "z".into(),
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Kind> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

#[test]
fn nested_roundtrip() {
    let rows = vec![Nested {
        basic: Basic {
            id: 1,
            name: "a".into(),
            active: true,
            score: None,
            tags: vec![1],
        },
        kind: Kind::Number(5),
        maybe: Some(None),
    }];
    let bytes = columnar::to_vec(&rows).unwrap();
    let decoded: Vec<Nested> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(decoded, rows);
}

#[test]
fn to_vec_into_replaces_the_buffer_and_reuses_capacity() {
    use columnar::Columnar;

    #[derive(Columnar, Debug, PartialEq)]
    struct Row {
        #[columnar(key = 1)]
        value: u64,
    }

    let rows: Vec<Row> = (0..1_000)
        .map(|value| Row {
            value: value * 7,
        })
        .collect();
    let expected = columnar::to_vec(&rows).unwrap();

    let mut buffer = vec![0xAA; 32];
    columnar::to_vec_into(&rows, &mut buffer).unwrap();
    assert_eq!(buffer, expected);

    let capacity = buffer.capacity();
    columnar::to_vec_into(&rows, &mut buffer).unwrap();
    assert_eq!(buffer, expected);
    assert!(buffer.capacity() >= capacity);
}
