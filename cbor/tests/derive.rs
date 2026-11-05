#![cfg(all(feature = "alloc", feature = "derive"))]

use cbor::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct User {
    id: u64,
    name: String,
    active: bool,
}

#[test]
fn default_struct_is_a_text_key_map() {
    let user = User {
        id: 7,
        name: "ada".into(),
        active: true,
    };
    let bytes = cbor::to_vec(&user);
    let value: cbor::Value = cbor::from_slice(&bytes).unwrap();
    let map = value.as_map().unwrap();
    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&"id".into()).unwrap().as_u64(), Some(7));
    assert_eq!(map.get(&"name".into()).unwrap().as_str(), Some("ada"));
    assert_eq!(map.get(&"active".into()).unwrap().as_bool(), Some(true));
    assert_eq!(cbor::from_slice::<User>(&bytes).unwrap(), user);
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(array)]
struct Point(f64, f64);

#[test]
fn array_struct() {
    let point = Point(1.5, -2.0);
    let bytes = cbor::to_vec(&point);
    assert_eq!(bytes[0], 0x82);
    assert_eq!(cbor::from_slice::<Point>(&bytes).unwrap(), point);
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Header {
    #[cbor(key = 1)]
    alg: i64,
    #[cbor(key = 5)]
    iv: Vec<u8>,
    #[cbor(key = 10)]
    kid: String,
}

#[test]
fn integer_keyed_struct() {
    let header = Header {
        alg: -7,
        iv: vec![1, 2, 3],
        kid: "key".into(),
    };
    let bytes = cbor::to_vec(&header);
    let value: cbor::Value = cbor::from_slice(&bytes).unwrap();
    let map = value.as_map().unwrap();
    assert_eq!(map.get(&1i64.into()).unwrap().as_i64(), Some(-7));
    assert_eq!(cbor::from_slice::<Header>(&bytes).unwrap(), header);
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Message {
    Ping,
    Data(u8),
    Pair(u8, u8),
    Named { x: u8 },
}

#[test]
fn external_enum() {
    let cases = [
        Message::Ping,
        Message::Data(7),
        Message::Pair(1, 2),
        Message::Named {
            x: 9,
        },
    ];
    for value in cases {
        let bytes = cbor::to_vec(&value);
        assert_eq!(cbor::from_slice::<Message>(&bytes).unwrap(), value);
    }
    assert_eq!(cbor::to_vec(&Message::Ping), vec![0x64, b'P', b'i', b'n', b'g']);
    assert_eq!(cbor::to_vec(&Message::Data(7)), vec![0xa1, 0x64, b'D', b'a', b't', b'a', 0x07]);
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(array)]
enum Compact {
    A,
    B(u8),
    C(u8, u8),
}

#[test]
fn array_enum() {
    let cases = [Compact::A, Compact::B(1), Compact::C(2, 3)];
    for value in cases {
        let bytes = cbor::to_vec(&value);
        assert_eq!(cbor::from_slice::<Compact>(&bytes).unwrap(), value);
    }
    assert_eq!(cbor::to_vec(&Compact::A), vec![0x81, 0x00]);
    assert_eq!(cbor::to_vec(&Compact::B(1)), vec![0x82, 0x01, 0x01]);
    assert_eq!(cbor::to_vec(&Compact::C(2, 3)), vec![0x83, 0x02, 0x02, 0x03]);
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(index_only)]
enum Code {
    Ok,
    Error,
}

#[test]
fn index_only_enum() {
    assert_eq!(cbor::to_vec(&Code::Ok), vec![0x00]);
    assert_eq!(cbor::to_vec(&Code::Error), vec![0x01]);
    assert_eq!(cbor::from_slice::<Code>(&[0x01]).unwrap(), Code::Error);
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(tag = 37)]
struct Uuid {
    #[cbor(bytes)]
    bytes: Vec<u8>,
}

#[test]
fn tagged_struct() {
    let uuid = Uuid {
        bytes: vec![7; 16],
    };
    let bytes = cbor::to_vec(&uuid);
    assert_eq!(bytes[0], 0xd8);
    assert_eq!(bytes[1], 0x25);
    assert_eq!(cbor::from_slice::<Uuid>(&bytes).unwrap(), uuid);
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Options {
    required: u8,
    optional: Option<u8>,
    #[cbor(default)]
    defaulted: u8,
    #[cbor(skip)]
    skipped: u8,
}

#[test]
fn option_default_and_skip() {
    let value = Options {
        required: 1,
        optional: None,
        defaulted: 2,
        skipped: 3,
    };
    let bytes = cbor::to_vec(&value);
    let back: Options = cbor::from_slice(&bytes).unwrap();
    assert_eq!(
        back,
        Options {
            required: 1,
            optional: None,
            defaulted: 2,
            skipped: 0
        }
    );

    let bytes = cbor::to_vec(&Options {
        required: 1,
        optional: Some(4),
        defaulted: 2,
        skipped: 3,
    });
    assert_eq!(cbor::from_slice::<Options>(&bytes).unwrap().optional, Some(4));
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Borrowed<'a> {
    #[cbor(borrow)]
    name: &'a str,
    #[cbor(bytes)]
    data: &'a [u8],
}

#[test]
fn borrowed_fields() {
    let bytes = cbor::to_vec(&Borrowed {
        name: "hi",
        data: b"abc",
    });
    let value: Borrowed<'_> = cbor::from_slice(&bytes).unwrap();
    assert_eq!(
        value,
        Borrowed {
            name: "hi",
            data: b"abc"
        }
    );
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BytesField {
    #[cbor(bytes)]
    payload: Vec<u8>,
}

#[test]
fn bytes_field() {
    let value = BytesField {
        payload: vec![0xde, 0xad],
    };
    assert_eq!(
        cbor::to_vec(&value),
        vec![0xa1, 0x67, b'p', b'a', b'y', b'l', b'o', b'a', b'd', 0x42, 0xde, 0xad]
    );
    assert_eq!(cbor::from_slice::<BytesField>(&cbor::to_vec(&value)).unwrap(), value);
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(rename_all = "camelCase")]
struct Renamed {
    first_name: String,
    #[cbor(rename = "ID")]
    id: u64,
}

#[test]
fn renames() {
    let value = Renamed {
        first_name: "a".into(),
        id: 1,
    };
    let bytes = cbor::to_vec(&value);
    let decoded: cbor::Value = cbor::from_slice(&bytes).unwrap();
    let map = decoded.as_map().unwrap();
    assert!(map.get(&"firstName".into()).is_some());
    assert!(map.get(&"ID".into()).is_some());
    assert_eq!(cbor::from_slice::<Renamed>(&bytes).unwrap(), value);
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum IntEnum {
    #[cbor(key = 1)]
    A,
    #[cbor(key = 2)]
    B(u8),
    #[cbor(key = 3)]
    C { x: u8 },
}

#[test]
fn integer_keyed_enum() {
    for value in [
        IntEnum::A,
        IntEnum::B(7),
        IntEnum::C {
            x: 9,
        },
    ] {
        let bytes = cbor::to_vec(&value);
        assert_eq!(cbor::from_slice::<IntEnum>(&bytes).unwrap(), value);
    }
    assert_eq!(cbor::to_vec(&IntEnum::A), vec![0xa1, 0x01, 0xf6]);
    assert_eq!(cbor::to_vec(&IntEnum::B(7)), vec![0xa1, 0x02, 0x07]);
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(tag = 1, tag_optional)]
struct Timestamp {
    value: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct UntaggedTimestamp {
    value: u64,
}

#[test]
fn optional_tag() {
    let value = Timestamp {
        value: 42,
    };
    let tagged = cbor::to_vec(&value);
    assert_eq!(tagged[0], 0xc1);
    assert_eq!(cbor::from_slice::<Timestamp>(&tagged).unwrap(), value);
    let untagged = cbor::to_vec(&UntaggedTimestamp {
        value: 42,
    });
    assert_eq!(cbor::from_slice::<Timestamp>(&untagged).unwrap(), value);
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(transparent)]
struct Wrapper(String);

#[test]
fn transparent_newtype() {
    let value = Wrapper("hello".into());
    let bytes = cbor::to_vec(&value);
    assert_eq!(bytes, cbor::to_vec("hello"));
    assert_eq!(cbor::from_slice::<Wrapper>(&bytes).unwrap(), value);
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(array)]
struct Gaps {
    #[cbor(index = 0)]
    a: u8,
    #[cbor(index = 2)]
    b: u8,
}

#[test]
fn array_index_gaps() {
    let value = Gaps {
        a: 1,
        b: 3,
    };
    let bytes = cbor::to_vec(&value);
    assert_eq!(bytes, vec![0x83, 0x01, 0xf6, 0x03]);
    assert_eq!(cbor::from_slice::<Gaps>(&bytes).unwrap(), value);
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(array)]
struct ArraySkip {
    a: u8,
    #[cbor(skip)]
    b: u8,
    c: u8,
}

#[test]
fn array_skip_keeps_positions() {
    let value = ArraySkip {
        a: 1,
        b: 2,
        c: 3,
    };
    let bytes = cbor::to_vec(&value);
    assert_eq!(bytes, vec![0x83, 0x01, 0xf6, 0x03]);
    assert_eq!(
        cbor::from_slice::<ArraySkip>(&bytes).unwrap(),
        ArraySkip {
            a: 1,
            b: 0,
            c: 3
        }
    );
}

fn is_zero(value: &u8) -> bool {
    *value == 0
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipIf {
    #[cbor(skip_if = "is_zero")]
    value: u8,
}

#[test]
fn skip_if_omits_and_defaults() {
    let bytes = cbor::to_vec(&SkipIf {
        value: 0,
    });
    assert_eq!(bytes, vec![0xa0]);
    assert_eq!(
        cbor::from_slice::<SkipIf>(&bytes).unwrap(),
        SkipIf {
            value: 0
        }
    );
    let bytes = cbor::to_vec(&SkipIf {
        value: 7,
    });
    assert_eq!(
        cbor::from_slice::<SkipIf>(&bytes).unwrap(),
        SkipIf {
            value: 7
        }
    );
}

mod as_byte_string {
    pub fn encode<W: cbor::Write>(value: &[u8], e: &mut cbor::Encoder<W>) -> Result<(), W::Error> {
        e.bytes(value)
    }
    pub fn decode<'b>(d: &mut cbor::Decoder<'b>) -> cbor::Result<Vec<u8>> {
        d.bytes_owned()
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithModule {
    #[cbor(with = "as_byte_string")]
    payload: Vec<u8>,
}

#[test]
fn with_module() {
    let value = WithModule {
        payload: vec![1, 2, 3],
    };
    let bytes = cbor::to_vec(&value);
    assert_eq!(bytes, vec![0xa1, 0x67, b'p', b'a', b'y', b'l', b'o', b'a', b'd', 0x43, 1, 2, 3]);
    assert_eq!(cbor::from_slice::<WithModule>(&bytes).unwrap(), value);
}

fn encode_plus_one<W: cbor::Write>(value: &u64, e: &mut cbor::Encoder<W>) -> Result<(), W::Error> {
    e.u64(value + 1)
}

fn decode_minus_one<'b>(d: &mut cbor::Decoder<'b>) -> cbor::Result<u64> {
    Ok(d.u64()? - 1)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithFunctions {
    #[cbor(encode_with = "encode_plus_one", decode_with = "decode_minus_one")]
    value: u64,
}

#[test]
fn encode_decode_with_functions() {
    let value = WithFunctions {
        value: 41,
    };
    let bytes = cbor::to_vec(&value);
    assert_eq!(cbor::from_slice::<WithFunctions>(&bytes).unwrap(), value);
    assert_eq!(cbor::from_slice::<WithFunctions>(&bytes).unwrap().value, 41);
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Generic<T> {
    value: T,
}

#[test]
fn generic_struct() {
    let value = Generic {
        value: 7u8,
    };
    let bytes = cbor::to_vec(&value);
    assert_eq!(cbor::from_slice::<Generic<u8>>(&bytes).unwrap(), value);
}

fn default_seven() -> u8 {
    7
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DefaultPath {
    #[cbor(default = "default_seven")]
    value: u8,
}

#[test]
fn default_path() {
    let bytes = cbor::to_vec(&std::collections::BTreeMap::<u8, u8>::new());
    assert_eq!(
        cbor::from_slice::<DefaultPath>(&bytes).unwrap(),
        DefaultPath {
            value: 7
        }
    );
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SortedFields {
    z: u8,
    a: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(declaration_order)]
struct DeclarationFields {
    z: u8,
    a: u8,
}

#[test]
fn declaration_order() {
    let sorted = cbor::to_vec(&SortedFields {
        z: 1,
        a: 2,
    });
    // "a" (0x61) sorts before "z" (0x61? no: both length 1, 0x61 0x61 < 0x61 0x7a)
    assert!(sorted[1] == 0x61 && sorted[2] == b'a');
    let declared = cbor::to_vec(&DeclarationFields {
        z: 1,
        a: 2,
    });
    assert!(declared[1] == 0x61 && declared[2] == b'z');
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(array)]
enum ArraySkipEnum {
    V(u8, #[cbor(skip)] u8, u8),
}

#[test]
fn enum_array_skip_keeps_positions() {
    let bytes = cbor::to_vec(&ArraySkipEnum::V(1, 2, 3));
    assert_eq!(bytes, vec![0x84, 0x00, 0x01, 0xf6, 0x03]);
    assert_eq!(cbor::from_slice::<ArraySkipEnum>(&bytes).unwrap(), ArraySkipEnum::V(1, 0, 3));
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NegativeKey {
    #[cbor(key = -1)]
    value: u8,
}

#[test]
fn negative_integer_key() {
    let value = NegativeKey {
        value: 7,
    };
    let bytes = cbor::to_vec(&value);
    assert_eq!(bytes, vec![0xa1, 0x20, 0x07]);
    assert_eq!(cbor::from_slice::<NegativeKey>(&bytes).unwrap(), value);
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(tagged)]
enum Tagged {
    #[cbor(index = 0)]
    Complete {
        #[cbor(key = 0)]
        user: u8,
        #[cbor(key = 1)]
        #[cbor(skip_if_none)]
        vault: Option<u8>,
        #[cbor(key = 2)]
        #[cbor(bytes)]
        #[cbor(skip_if_none)]
        blob: Option<Vec<u8>>,
    },
    #[cbor(index = 1)]
    Absent,
}

#[test]
fn tagged_enum_uses_index_and_field_map() {
    let bytes = cbor::to_vec(&Tagged::Complete {
        user: 7,
        vault: Some(9),
        blob: None,
    });
    assert_eq!(bytes, vec![0x82, 0x00, 0xa2, 0x00, 0x07, 0x01, 0x09]);
    assert_eq!(
        cbor::from_slice::<Tagged>(&bytes).unwrap(),
        Tagged::Complete {
            user: 7,
            vault: Some(9),
            blob: None,
        }
    );

    let bytes = cbor::to_vec(&Tagged::Complete {
        user: 1,
        vault: None,
        blob: Some(vec![2, 3]),
    });
    assert_eq!(bytes, vec![0x82, 0x00, 0xa2, 0x00, 0x01, 0x02, 0x42, 0x02, 0x03]);
    assert_eq!(
        cbor::from_slice::<Tagged>(&bytes).unwrap(),
        Tagged::Complete {
            user: 1,
            vault: None,
            blob: Some(vec![2, 3]),
        }
    );
}

#[test]
fn tagged_enum_unit_variant_is_an_empty_map() {
    assert_eq!(cbor::to_vec(&Tagged::Absent), vec![0x82, 0x01, 0xa0]);
    assert_eq!(cbor::from_slice::<Tagged>(&[0x82, 0x01, 0xa0]).unwrap(), Tagged::Absent);
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalBytes {
    #[cbor(key = 0)]
    #[cbor(bytes)]
    data: Option<Vec<u8>>,
}

#[test]
fn optional_bytes_encode_null_or_byte_string() {
    assert_eq!(
        cbor::to_vec(&OptionalBytes {
            data: None
        }),
        vec![0xa1, 0x00, 0xf6]
    );
    assert_eq!(
        cbor::to_vec(&OptionalBytes {
            data: Some(vec![1, 2])
        }),
        vec![0xa1, 0x00, 0x42, 0x01, 0x02]
    );
    assert_eq!(
        cbor::from_slice::<OptionalBytes>(&[0xa1, 0x00, 0xf6]).unwrap(),
        OptionalBytes {
            data: None
        }
    );
    assert_eq!(
        cbor::from_slice::<OptionalBytes>(&[0xa1, 0x00, 0x42, 0x01, 0x02]).unwrap(),
        OptionalBytes {
            data: Some(vec![1, 2])
        }
    );
    assert_eq!(
        cbor::from_slice::<OptionalBytes>(&[0xa1, 0x00, 0xf7]).unwrap(),
        OptionalBytes {
            data: None
        }
    );
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkippedOptionalBytes {
    #[cbor(key = 0)]
    #[cbor(bytes)]
    #[cbor(skip_if_none)]
    data: Option<Vec<u8>>,
    #[cbor(key = 1)]
    present: bool,
}

#[test]
fn skipped_optional_bytes_omit_the_key() {
    assert_eq!(
        cbor::to_vec(&SkippedOptionalBytes {
            data: None,
            present: true,
        }),
        vec![0xa1, 0x01, 0xf5]
    );
    assert_eq!(
        cbor::to_vec(&SkippedOptionalBytes {
            data: Some(vec![7]),
            present: false,
        }),
        vec![0xa2, 0x00, 0x41, 0x07, 0x01, 0xf4]
    );
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(index_only)]
enum ExplicitIndex {
    #[cbor(index = 3)]
    A,
    #[cbor(index = 7)]
    B,
}

#[test]
fn index_only_honours_explicit_indices() {
    assert_eq!(cbor::to_vec(&ExplicitIndex::A), vec![0x03]);
    assert_eq!(cbor::to_vec(&ExplicitIndex::B), vec![0x07]);
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MixedKeys {
    #[cbor(key = -1)]
    a: u8,
    #[cbor(key = 1)]
    b: u8,
}

#[test]
fn mixed_sign_integer_keys() {
    let value = MixedKeys {
        a: 1,
        b: 2,
    };
    let bytes = cbor::to_vec(&value);
    assert_eq!(cbor::from_slice::<MixedKeys>(&bytes).unwrap(), value);
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MixedEnum {
    #[cbor(key = -1)]
    A(u8),
    #[cbor(key = 1)]
    B(u8),
}

#[test]
fn mixed_sign_enum_keys() {
    for value in [MixedEnum::A(1), MixedEnum::B(2)] {
        let bytes = cbor::to_vec(&value);
        assert_eq!(cbor::from_slice::<MixedEnum>(&bytes).unwrap(), value);
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[cbor(tagged)]
enum MixedTagged {
    V {
        #[cbor(key = -1)]
        a: u8,
        #[cbor(key = 1)]
        b: u8,
    },
}

#[test]
fn mixed_sign_tagged_field_keys() {
    let value = MixedTagged::V {
        a: 1,
        b: 2,
    };
    let bytes = cbor::to_vec(&value);
    assert_eq!(cbor::from_slice::<MixedTagged>(&bytes).unwrap(), value);
}

#[derive(Debug, PartialEq, Decode)]
#[cbor(transparent)]
struct BorrowedRaw<'a> {
    #[cbor(borrow)]
    raw: cbor::RawValue<'a>,
}

#[test]
fn borrowed_raw_value_field() {
    let bytes = cbor::to_vec(&(1u8, 2u8));
    let value: BorrowedRaw<'_> = cbor::from_slice(&bytes).unwrap();
    assert_eq!(value.raw.as_bytes(), &bytes[..]);
}
