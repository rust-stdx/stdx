use cbor::Encoder;
use columnar::{Columnar, DecodeOptions, Error};

const ENC_ENUM: u64 = 2;
const ENC_PACKED: u64 = 3;
const ENC_FOR: u64 = 4;
const ENC_VARINT: u64 = 6;
const ENC_RLE: u64 = 7;
const ENC_DICTIONARY: u64 = 8;
const ENC_STRING_LENGTHS: u64 = 9;
const ENC_STRING_OFFSETS: u64 = 10;
const ENC_NULLABLE: u64 = 11;
const ENC_LIST_LENGTHS: u64 = 12;
const ENC_LIST_OFFSETS: u64 = 13;
const ENC_DELTA_RLE: u64 = 15;
const ENC_DELTA_OF_DELTA: u64 = 16;

#[derive(Columnar, Debug, PartialEq, Clone)]
struct WireRow {
    #[columnar(key = 1)]
    a: u64,
    #[columnar(key = 2)]
    b: String,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct OptRow {
    #[columnar(key = 1)]
    v: Option<u64>,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct ListRow {
    #[columnar(key = 1)]
    v: Vec<u64>,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct StringRow {
    #[columnar(key = 1)]
    v: String,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct DictRow {
    #[columnar(key = 1, encoding = "dictionary")]
    v: String,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct RleRow {
    #[columnar(key = 1, encoding = "rle")]
    v: u64,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct DeltaRleRow {
    #[columnar(key = 1, encoding = "delta_rle")]
    v: u64,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct DeltaRleByteRow {
    #[columnar(key = 1, encoding = "delta_rle")]
    v: u8,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct DeltaOfDeltaRow {
    #[columnar(key = 1, encoding = "delta_of_delta")]
    v: i64,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct EnumRow {
    #[columnar(key = 1)]
    v: Kind,
}

#[derive(Columnar, Debug, PartialEq, Clone)]
enum Kind {
    A,
    B(u64),
    C { x: u32, y: String },
}

fn encoder(entries: usize) -> Encoder<Vec<u8>> {
    let mut e = Encoder::new(Vec::new());
    e.map(entries).unwrap();
    e
}

fn header(e: &mut Encoder<Vec<u8>>, version: u64, rows: u64) {
    e.u64(0).unwrap();
    e.array(2).unwrap();
    e.u64(version).unwrap();
    e.u64(rows).unwrap();
}

fn column(e: &mut Encoder<Vec<u8>>, key: u64, encoding: u64, payload: impl FnOnce(&mut Encoder<Vec<u8>>)) {
    e.u64(key).unwrap();
    e.array(2).unwrap();
    e.u64(encoding).unwrap();
    payload(e);
}

fn raw_column(e: &mut Encoder<Vec<u8>>, encoding: u64, payload: impl FnOnce(&mut Encoder<Vec<u8>>)) {
    e.array(2).unwrap();
    e.u64(encoding).unwrap();
    payload(e);
}

fn empty_column(e: &mut Encoder<Vec<u8>>, key: u64, encoding: u64) {
    column(e, key, encoding, |e| {
        e.array(0).unwrap();
    });
}

fn valid_bytes() -> Vec<u8> {
    columnar::to_vec(&[
        WireRow {
            a: 1,
            b: "one".into(),
        },
        WireRow {
            a: 2,
            b: "two".into(),
        },
    ])
    .unwrap()
}

fn decode_wire(bytes: &[u8]) -> Result<Vec<WireRow>, Error> {
    columnar::from_slice(bytes)
}

#[test]
fn header_shape() {
    let bytes = valid_bytes();
    let mut d = cbor::Decoder::new(&bytes);
    let len = d.map().unwrap().unwrap();
    assert_eq!(len, 3);
    assert_eq!(d.u64().unwrap(), 0);
    let header_len = d.array().unwrap().unwrap();
    assert_eq!(header_len, 2);
    assert_eq!(d.u64().unwrap(), 1);
    assert_eq!(d.u64().unwrap(), 2);
    assert_eq!(d.u64().unwrap(), 1);
    assert_eq!(d.array().unwrap().unwrap(), 2);
    assert!(d.u64().unwrap() < 32);
    d.skip().unwrap();
    assert_eq!(d.u64().unwrap(), 2);
    assert_eq!(d.array().unwrap().unwrap(), 2);
    assert!(d.u64().unwrap() < 32);
    d.skip().unwrap();
    assert!(d.remaining() == 0);
}

#[test]
fn duplicate_field_key_is_rejected() {
    let mut e = encoder(4);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_FOR, |e| {
        e.array(3).unwrap();
        e.u64(1).unwrap();
        e.u64(0).unwrap();
        e.bytes(&[]).unwrap();
    });
    column(&mut e, 1, ENC_FOR, |e| {
        e.array(3).unwrap();
        e.u64(2).unwrap();
        e.u64(0).unwrap();
        e.bytes(&[]).unwrap();
    });
    column(&mut e, 2, ENC_STRING_LENGTHS, |e| {
        e.array(2).unwrap();
        e.bytes(&[]).unwrap();
        e.bytes(&[]).unwrap();
    });
    let bytes = e.into_inner();
    assert!(matches!(decode_wire(&bytes), Err(Error::DuplicateKey(1))));
}

#[test]
fn duplicate_header_is_rejected() {
    let mut e = encoder(3);
    header(&mut e, 1, 0);
    header(&mut e, 1, 0);
    empty_column(&mut e, 1, ENC_FOR);
    let bytes = e.into_inner();
    assert!(matches!(decode_wire(&bytes), Err(Error::DuplicateKey(0))));
}

#[test]
fn header_must_come_first() {
    let mut e = encoder(3);
    column(&mut e, 1, ENC_FOR, |e| {
        e.array(3).unwrap();
        e.u64(0).unwrap();
        e.u64(0).unwrap();
        e.bytes(&[]).unwrap();
    });
    header(&mut e, 1, 0);
    empty_column(&mut e, 2, ENC_STRING_LENGTHS);
    let bytes = e.into_inner();
    assert!(matches!(decode_wire(&bytes), Err(Error::MissingHeader)));
}

#[test]
fn missing_header_is_rejected() {
    let mut e = encoder(1);
    empty_column(&mut e, 1, ENC_FOR);
    let bytes = e.into_inner();
    assert!(matches!(decode_wire(&bytes), Err(Error::MissingHeader)));
}

#[test]
fn unsupported_version_is_rejected() {
    let mut e = encoder(1);
    header(&mut e, 2, 0);
    let bytes = e.into_inner();
    assert!(matches!(decode_wire(&bytes), Err(Error::UnsupportedVersion(2))));
}

#[test]
fn invalid_header_length_is_rejected() {
    let mut e = encoder(1);
    e.u64(0).unwrap();
    e.array(1).unwrap();
    e.u64(1).unwrap();
    assert!(matches!(decode_wire(&e.into_inner()), Err(Error::InvalidHeader)));

    let mut e = encoder(1);
    e.u64(0).unwrap();
    e.array(3).unwrap();
    e.u64(1).unwrap();
    e.u64(0).unwrap();
    e.u64(0).unwrap();
    assert!(matches!(decode_wire(&e.into_inner()), Err(Error::InvalidHeader)));
}

#[test]
fn trailing_data_is_rejected() {
    let mut bytes = valid_bytes();
    bytes.push(0);
    assert!(matches!(decode_wire(&bytes), Err(Error::TrailingData)));
}

#[test]
fn unknown_field_is_skipped() {
    let mut e = encoder(4);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_FOR, |e| {
        e.array(3).unwrap();
        e.u64(7).unwrap();
        e.u64(0).unwrap();
        e.bytes(&[]).unwrap();
    });
    column(&mut e, 2, ENC_STRING_LENGTHS, |e| {
        e.array(2).unwrap();
        e.bytes(&[1]).unwrap();
        e.bytes(b"x").unwrap();
    });
    column(&mut e, 99, ENC_VARINT, |e| {
        e.bytes(&[0]).unwrap();
    });
    let bytes = e.into_inner();
    assert_eq!(
        decode_wire(&bytes).unwrap(),
        vec![WireRow {
            a: 7,
            b: "x".into()
        }]
    );
}

#[test]
fn missing_field_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_FOR, |e| {
        e.array(3).unwrap();
        e.u64(7).unwrap();
        e.u64(0).unwrap();
        e.bytes(&[]).unwrap();
    });
    assert!(matches!(decode_wire(&e.into_inner()), Err(Error::MissingField("b"))));
}

#[test]
fn invalid_encoding_is_rejected() {
    let mut e = encoder(3);
    header(&mut e, 1, 1);
    empty_column(&mut e, 1, 999);
    empty_column(&mut e, 2, ENC_STRING_LENGTHS);
    assert!(matches!(decode_wire(&e.into_inner()), Err(Error::InvalidEncoding(999))));
}

#[test]
fn packed_length_mismatch_is_rejected() {
    let mut e = encoder(3);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_PACKED, |e| {
        e.bytes(&[0]).unwrap();
    });
    empty_column(&mut e, 2, ENC_STRING_LENGTHS);
    assert!(matches!(decode_wire(&e.into_inner()), Err(Error::LengthMismatch { .. })));
}

#[test]
fn for_width_is_validated() {
    let mut e = encoder(3);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_FOR, |e| {
        e.array(3).unwrap();
        e.u64(0).unwrap();
        e.u64(65).unwrap();
        e.bytes(&[]).unwrap();
    });
    empty_column(&mut e, 2, ENC_STRING_LENGTHS);
    assert!(matches!(decode_wire(&e.into_inner()), Err(Error::InvalidEncoding(65))));
}

#[test]
fn truncated_for_is_rejected() {
    let mut e = encoder(3);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_FOR, |e| {
        e.array(3).unwrap();
        e.u64(0).unwrap();
        e.u64(64).unwrap();
        e.bytes(&[]).unwrap();
    });
    empty_column(&mut e, 2, ENC_STRING_LENGTHS);
    assert!(matches!(decode_wire(&e.into_inner()), Err(Error::LengthMismatch { .. })));
}

#[test]
fn varint_trailing_bytes_are_rejected() {
    let mut e = encoder(3);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_VARINT, |e| {
        e.bytes(&[0, 1]).unwrap();
    });
    empty_column(&mut e, 2, ENC_STRING_LENGTHS);
    assert!(matches!(decode_wire(&e.into_inner()), Err(Error::LengthMismatch { .. })));
}

#[test]
fn invalid_utf8_is_rejected() {
    let mut e = encoder(3);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_FOR, |e| {
        e.array(3).unwrap();
        e.u64(0).unwrap();
        e.u64(0).unwrap();
        e.bytes(&[]).unwrap();
    });
    column(&mut e, 2, ENC_STRING_LENGTHS, |e| {
        e.array(2).unwrap();
        e.bytes(&[1]).unwrap();
        e.bytes(&[0xff]).unwrap();
    });
    assert!(matches!(decode_wire(&e.into_inner()), Err(Error::InvalidUtf8)));
}

#[test]
fn string_length_overrun_is_rejected() {
    let mut e = encoder(3);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_FOR, |e| {
        e.array(3).unwrap();
        e.u64(0).unwrap();
        e.u64(0).unwrap();
        e.bytes(&[]).unwrap();
    });
    column(&mut e, 2, ENC_STRING_LENGTHS, |e| {
        e.array(2).unwrap();
        e.bytes(&[5]).unwrap();
        e.bytes(b"a").unwrap();
    });
    assert!(matches!(decode_wire(&e.into_inner()), Err(Error::LengthMismatch { .. })));
}

#[test]
fn string_trailing_lengths_are_rejected() {
    let mut e = encoder(3);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_FOR, |e| {
        e.array(3).unwrap();
        e.u64(0).unwrap();
        e.u64(0).unwrap();
        e.bytes(&[]).unwrap();
    });
    column(&mut e, 2, ENC_STRING_LENGTHS, |e| {
        e.array(2).unwrap();
        e.bytes(&[1, 1]).unwrap();
        e.bytes(b"a").unwrap();
    });
    assert!(matches!(decode_wire(&e.into_inner()), Err(Error::LengthMismatch { .. })));
}

#[test]
fn option_bitmap_length_is_validated() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_NULLABLE, |e| {
        e.array(2).unwrap();
        e.bytes(&[1, 0]).unwrap();
        raw_column(e, ENC_FOR, |e| {
            e.array(3).unwrap();
            e.u64(0).unwrap();
            e.u64(0).unwrap();
            e.bytes(&[]).unwrap();
        });
    });
    // The inner "column" has a key 0, which is only valid at the top level;
    // the decoder should still reject the outer bitmap length first.
    assert!(matches!(
        columnar::from_slice::<OptRow>(&e.into_inner()),
        Err(Error::LengthMismatch { .. })
    ));
}

#[test]
fn option_missing_inner_values_are_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 2);
    column(&mut e, 1, ENC_NULLABLE, |e| {
        e.array(2).unwrap();
        e.bytes(&[0b11]).unwrap();
        raw_column(e, ENC_FOR, |e| {
            e.array(3).unwrap();
            e.u64(0).unwrap();
            e.u64(1).unwrap();
            e.bytes(&[]).unwrap();
        });
    });
    assert!(columnar::from_slice::<OptRow>(&e.into_inner()).is_err());
}

#[test]
fn list_length_mismatch_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_LIST_LENGTHS, |e| {
        e.array(2).unwrap();
        e.bytes(&[2]).unwrap();
        raw_column(e, ENC_FOR, |e| {
            e.array(3).unwrap();
            e.u64(0).unwrap();
            e.u64(1).unwrap();
            e.bytes(&[]).unwrap();
        });
    });
    assert!(columnar::from_slice::<ListRow>(&e.into_inner()).is_err());
}

#[test]
fn list_offsets_must_be_monotonic() {
    let mut e = encoder(2);
    header(&mut e, 1, 2);
    column(&mut e, 1, ENC_LIST_OFFSETS, |e| {
        e.array(3).unwrap();
        e.u64(4).unwrap();
        e.bytes(&[0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0]).unwrap();
        raw_column(e, ENC_FOR, |e| {
            e.array(3).unwrap();
            e.u64(0).unwrap();
            e.u64(0).unwrap();
            e.bytes(&[]).unwrap();
        });
    });
    assert!(matches!(
        columnar::from_slice::<ListRow>(&e.into_inner()),
        Err(Error::InvalidOffset)
    ));
}

#[test]
fn empty_dictionary_with_rows_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_DICTIONARY, |e| {
        e.array(3).unwrap();
        e.u64(0).unwrap();
        raw_column(e, ENC_STRING_LENGTHS, |e| {
            e.array(2).unwrap();
            e.bytes(&[]).unwrap();
            e.bytes(&[]).unwrap();
        });
        e.bytes(&[]).unwrap();
    });
    assert!(matches!(
        columnar::from_slice::<DictRow>(&e.into_inner()),
        Err(Error::DictionaryIndex { .. })
    ));
}

#[test]
fn rle_zero_run_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_RLE, |e| {
        e.array(2).unwrap();
        e.bytes(&[0]).unwrap();
        raw_column(e, ENC_FOR, |e| {
            e.array(3).unwrap();
            e.u64(0).unwrap();
            e.u64(0).unwrap();
            e.bytes(&[]).unwrap();
        });
    });
    assert!(matches!(columnar::from_slice::<RleRow>(&e.into_inner()), Err(Error::ZeroRun)));
}

#[test]
fn rle_sum_mismatch_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_RLE, |e| {
        e.array(2).unwrap();
        e.bytes(&[2]).unwrap();
        raw_column(e, ENC_FOR, |e| {
            e.array(3).unwrap();
            e.u64(0).unwrap();
            e.u64(0).unwrap();
            e.bytes(&[]).unwrap();
        });
    });
    assert!(matches!(
        columnar::from_slice::<RleRow>(&e.into_inner()),
        Err(Error::LengthMismatch { .. })
    ));
}

fn varint(mut value: u128) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            return out;
        }
    }
}

fn zigzag(value: i128) -> u128 {
    ((value << 1) ^ (value >> 127)) as u128
}

fn delta_varints(values: &[i128]) -> Vec<u8> {
    let mut out = Vec::new();
    for value in values {
        out.extend(varint(zigzag(*value)));
    }
    out
}

#[test]
fn delta_rle_wire_shape() {
    let rows = vec![
        DeltaRleRow {
            v: 1,
        },
        DeltaRleRow {
            v: 2,
        },
        DeltaRleRow {
            v: 3,
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let mut d = cbor::Decoder::new(&bytes);
    assert_eq!(d.map().unwrap().unwrap(), 2);
    assert_eq!(d.u64().unwrap(), 0);
    assert_eq!(d.array().unwrap().unwrap(), 2);
    assert_eq!(d.u64().unwrap(), 1);
    assert_eq!(d.u64().unwrap(), 3);
    assert_eq!(d.u64().unwrap(), 1);
    assert_eq!(d.array().unwrap().unwrap(), 2);
    assert_eq!(d.u64().unwrap(), ENC_DELTA_RLE);
    assert_eq!(d.array().unwrap().unwrap(), 2);
    assert_eq!(d.bytes().unwrap(), &[3]);
    assert_eq!(d.array().unwrap().unwrap(), 2);
    assert_eq!(d.u64().unwrap(), ENC_VARINT);
    assert_eq!(d.bytes().unwrap(), &[2]);
    assert!(d.remaining() == 0);
}

#[test]
fn delta_of_delta_wire_shape() {
    let rows = vec![
        DeltaOfDeltaRow {
            v: 1,
        },
        DeltaOfDeltaRow {
            v: 2,
        },
        DeltaOfDeltaRow {
            v: 3,
        },
    ];
    let bytes = columnar::to_vec(&rows).unwrap();
    let mut d = cbor::Decoder::new(&bytes);
    assert_eq!(d.map().unwrap().unwrap(), 2);
    assert_eq!(d.u64().unwrap(), 0);
    assert_eq!(d.array().unwrap().unwrap(), 2);
    assert_eq!(d.u64().unwrap(), 1);
    assert_eq!(d.u64().unwrap(), 3);
    assert_eq!(d.u64().unwrap(), 1);
    assert_eq!(d.array().unwrap().unwrap(), 2);
    assert_eq!(d.u64().unwrap(), ENC_DELTA_OF_DELTA);
    assert_eq!(d.array().unwrap().unwrap(), 2);
    assert_eq!(d.u64().unwrap(), ENC_VARINT);
    assert_eq!(d.bytes().unwrap(), &[2, 2, 0]);
    assert!(d.remaining() == 0);
}

#[test]
fn delta_rle_zero_run_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_DELTA_RLE, |e| {
        e.array(2).unwrap();
        e.bytes(&[0]).unwrap();
        raw_column(e, ENC_VARINT, |e| e.bytes(&[]).unwrap());
    });
    assert!(matches!(
        columnar::from_slice::<DeltaRleRow>(&e.into_inner()),
        Err(Error::ZeroRun)
    ));
}

#[test]
fn delta_rle_sum_mismatch_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_DELTA_RLE, |e| {
        e.array(2).unwrap();
        e.bytes(&[2]).unwrap();
        raw_column(e, ENC_VARINT, |e| e.bytes(&[]).unwrap());
    });
    assert!(matches!(
        columnar::from_slice::<DeltaRleRow>(&e.into_inner()),
        Err(Error::LengthMismatch { .. })
    ));
}

#[test]
fn delta_rle_requires_a_varint_delta_column() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_DELTA_RLE, |e| {
        e.array(2).unwrap();
        e.bytes(&[1]).unwrap();
        raw_column(e, ENC_PACKED, |e| e.bytes(&[0]).unwrap());
    });
    let result = columnar::from_slice::<DeltaRleRow>(&e.into_inner());
    assert!(matches!(result, Err(Error::InvalidEncoding(ENC_PACKED))), "{result:?}");
}

#[test]
fn delta_rle_trailing_delta_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_DELTA_RLE, |e| {
        e.array(2).unwrap();
        e.bytes(&[1]).unwrap();
        raw_column(e, ENC_VARINT, |e| e.bytes(&[2, 2]).unwrap());
    });
    assert!(matches!(
        columnar::from_slice::<DeltaRleRow>(&e.into_inner()),
        Err(Error::LengthMismatch { .. })
    ));
}

#[test]
fn delta_rle_truncated_delta_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_DELTA_RLE, |e| {
        e.array(2).unwrap();
        e.bytes(&[1]).unwrap();
        raw_column(e, ENC_VARINT, |e| e.bytes(&[0x80]).unwrap());
    });
    assert!(matches!(
        columnar::from_slice::<DeltaRleRow>(&e.into_inner()),
        Err(Error::InvalidVarint)
    ));
}

#[test]
fn delta_rle_out_of_range_value_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 2);
    column(&mut e, 1, ENC_DELTA_RLE, |e| {
        e.array(2).unwrap();
        e.bytes(&[2]).unwrap();
        raw_column(e, ENC_VARINT, |e| e.bytes(&delta_varints(&[300])).unwrap());
    });
    assert!(matches!(
        columnar::from_slice::<DeltaRleByteRow>(&e.into_inner()),
        Err(Error::DeltaOverflow)
    ));
}

#[test]
fn delta_rle_run_limit_is_enforced() {
    let mut e = encoder(2);
    header(&mut e, 1, 2);
    column(&mut e, 1, ENC_DELTA_RLE, |e| {
        e.array(2).unwrap();
        e.bytes(&[1, 1]).unwrap();
        raw_column(e, ENC_VARINT, |e| e.bytes(&[]).unwrap());
    });
    let bytes = e.into_inner();
    let options = DecodeOptions {
        max_list_len: 1,
        ..DecodeOptions::default()
    };
    assert!(matches!(
        columnar::from_slice_with_options::<DeltaRleRow>(&bytes, options),
        Err(Error::LimitExceeded("delta rle runs"))
    ));
}

#[test]
fn delta_of_delta_requires_a_varint_column() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_DELTA_OF_DELTA, |e| {
        raw_column(e, ENC_PACKED, |e| e.bytes(&[0]).unwrap());
    });
    let result = columnar::from_slice::<DeltaOfDeltaRow>(&e.into_inner());
    assert!(matches!(result, Err(Error::InvalidEncoding(ENC_PACKED))), "{result:?}");
}

#[test]
fn delta_of_delta_trailing_value_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_DELTA_OF_DELTA, |e| {
        raw_column(e, ENC_VARINT, |e| e.bytes(&delta_varints(&[1, 2])).unwrap());
    });
    assert!(matches!(
        columnar::from_slice::<DeltaOfDeltaRow>(&e.into_inner()),
        Err(Error::LengthMismatch { .. })
    ));
}

#[test]
fn delta_of_delta_out_of_range_value_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 2);
    column(&mut e, 1, ENC_DELTA_OF_DELTA, |e| {
        raw_column(e, ENC_VARINT, |e| e.bytes(&delta_varints(&[0, 1i128 << 63])).unwrap());
    });
    assert!(matches!(
        columnar::from_slice::<DeltaOfDeltaRow>(&e.into_inner()),
        Err(Error::DeltaOverflow)
    ));
}

#[test]
fn enum_tag_out_of_range_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_ENUM, |e| {
        e.array(4).unwrap();
        // One tag with value 3 (there are only 3 variants: 0, 1, 2).
        e.bytes(&[0b11]).unwrap();
        // The decoder validates tags before reading the children.
        e.array(2).unwrap();
        e.u64(ENC_FOR).unwrap();
        e.array(3).unwrap();
        e.u64(0).unwrap();
        e.u64(0).unwrap();
        e.bytes(&[]).unwrap();
        e.array(2).unwrap();
        e.u64(ENC_FOR).unwrap();
        e.array(3).unwrap();
        e.u64(0).unwrap();
        e.u64(0).unwrap();
        e.bytes(&[]).unwrap();
        e.array(2).unwrap();
        e.u64(ENC_STRING_LENGTHS).unwrap();
        e.array(2).unwrap();
        e.bytes(&[]).unwrap();
        e.bytes(&[]).unwrap();
    });
    assert!(matches!(
        columnar::from_slice::<EnumRow>(&e.into_inner()),
        Err(Error::InvalidBitpackedValue)
    ));
}

#[test]
fn enum_payload_length_is_validated() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_ENUM, |e| {
        e.array(2).unwrap();
        e.bytes(&[0]).unwrap();
        e.bytes(&[]).unwrap();
    });
    assert!(matches!(
        columnar::from_slice::<EnumRow>(&e.into_inner()),
        Err(Error::LengthMismatch { .. })
    ));
}

#[test]
fn every_prefix_of_a_valid_envelope_errors_without_panicking() {
    let bytes = valid_bytes();
    for len in 0..bytes.len() {
        let _ = decode_wire(&bytes[..len]);
    }
}

#[test]
fn every_single_byte_mutation_does_not_panic() {
    let bytes = valid_bytes();
    for index in 0..bytes.len() {
        for mask in [0x01u8, 0x80, 0xff] {
            let mut mutated = bytes.clone();
            mutated[index] ^= mask;
            let _ = decode_wire(&mutated);
        }
    }
}

#[test]
fn random_bytes_do_not_panic() {
    let mut state = 0x9e37_79b9_7f4a_7c15u64;
    for len in 0..128 {
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            bytes.push(state as u8);
        }
        let _ = decode_wire(&bytes);
    }
}

#[test]
fn rows_limit_is_enforced() {
    let bytes = valid_bytes();
    let options = DecodeOptions {
        max_rows: 1,
        ..DecodeOptions::default()
    };
    assert!(matches!(
        columnar::from_slice_with_options::<WireRow>(&bytes, options),
        Err(Error::LimitExceeded("rows"))
    ));
}

#[test]
fn non_map_envelope_is_rejected() {
    let mut e = Encoder::new(Vec::new());
    e.array(0).unwrap();
    assert!(decode_wire(&e.into_inner()).is_err());
}

#[test]
fn duplicate_map_keys_in_raw_cbor_are_rejected() {
    // CBOR itself may contain duplicate keys; our decoder detects them.
    let mut e = encoder(3);
    header(&mut e, 1, 1);
    for _ in 0..2 {
        column(&mut e, 1, ENC_FOR, |e| {
            e.array(3).unwrap();
            e.u64(0).unwrap();
            e.u64(0).unwrap();
            e.bytes(&[]).unwrap();
        });
    }
    assert!(matches!(decode_wire(&e.into_inner()), Err(Error::DuplicateKey(1))));
}

#[test]
fn string_offsets_are_validated() {
    let mut e = encoder(2);
    header(&mut e, 1, 2);
    column(&mut e, 1, ENC_STRING_OFFSETS, |e| {
        e.array(3).unwrap();
        e.u64(4).unwrap();
        // offsets 0, 2, 1: non-monotonic
        e.bytes(&[0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0]).unwrap();
        e.bytes(b"ab").unwrap();
    });
    assert!(matches!(
        columnar::from_slice::<StringRow>(&e.into_inner()),
        Err(Error::InvalidOffset)
    ));
}

#[test]
fn all_null_and_struct_encodings_are_accepted() {
    let bytes = columnar::to_vec(&[
        OptRow {
            v: None,
        },
        OptRow {
            v: Some(1),
        },
    ])
    .unwrap();
    let decoded: Vec<OptRow> = columnar::from_slice(&bytes).unwrap();
    assert_eq!(
        decoded,
        vec![
            OptRow {
                v: None
            },
            OptRow {
                v: Some(1)
            }
        ]
    );
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct CharRow {
    #[columnar(key = 1)]
    value: char,
}

#[test]
fn invalid_char_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_FOR, |e| {
        e.array(3).unwrap();
        e.u64(0xd800).unwrap();
        e.u64(0).unwrap();
        e.bytes(&[]).unwrap();
    });
    assert!(matches!(
        columnar::from_slice::<CharRow>(&e.into_inner()),
        Err(Error::InvalidChar(0xd800))
    ));
}

fn default_value() -> u64 {
    77
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct DefaultRow {
    #[columnar(key = 1)]
    a: u64,
    #[columnar(key = 2, default = "default_value")]
    b: u64,
}

#[test]
fn missing_field_uses_default() {
    let mut e = encoder(2);
    header(&mut e, 1, 1);
    column(&mut e, 1, ENC_FOR, |e| {
        e.array(3).unwrap();
        e.u64(5).unwrap();
        e.u64(0).unwrap();
        e.bytes(&[]).unwrap();
    });
    assert_eq!(
        columnar::from_slice::<DefaultRow>(&e.into_inner()).unwrap(),
        vec![DefaultRow {
            a: 5,
            b: 77
        }]
    );
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct Unsupported {
    #[columnar(key = 1, encoding = "for")]
    value: f64,
}

#[test]
fn unsupported_encoding_returns_error() {
    let rows = vec![Unsupported {
        value: 1.0,
    }];
    assert!(matches!(
        columnar::to_vec(&rows),
        Err(Error::UnsupportedEncoding(columnar::Encoding::ForBitpack))
    ));
}

#[derive(Columnar, Debug, PartialEq, Clone)]
struct UnsupportedList {
    #[columnar(key = 1, encoding = "rle")]
    value: Vec<u64>,
}

#[test]
fn unsupported_list_encoding_returns_error() {
    let rows = vec![UnsupportedList {
        value: vec![1],
    }];
    assert!(matches!(
        columnar::to_vec(&rows),
        Err(Error::UnsupportedEncoding(columnar::Encoding::Rle))
    ));
}

#[test]
fn list_limit_is_enforced() {
    let bytes = columnar::to_vec(&[ListRow {
        v: vec![1, 2, 3],
    }])
    .unwrap();
    let options = DecodeOptions {
        max_list_len: 2,
        ..DecodeOptions::default()
    };
    assert!(matches!(
        columnar::from_slice_with_options::<ListRow>(&bytes, options),
        Err(Error::LimitExceeded("list length"))
    ));
}

#[test]
fn unlimited_options_decode_valid_data() {
    let bytes = valid_bytes();
    assert!(columnar::from_slice_with_options::<WireRow>(&bytes, DecodeOptions::unlimited()).is_ok());
}

#[test]
fn string_offsets_width_8_are_supported() {
    let mut e = encoder(2);
    header(&mut e, 1, 2);
    column(&mut e, 1, ENC_STRING_OFFSETS, |e| {
        e.array(3).unwrap();
        e.u64(8).unwrap();
        e.bytes(&[0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0])
            .unwrap();
        e.bytes(b"ab").unwrap();
    });
    assert_eq!(
        columnar::from_slice::<StringRow>(&e.into_inner()).unwrap(),
        vec![
            StringRow {
                v: "a".into()
            },
            StringRow {
                v: "b".into()
            }
        ]
    );
}

#[test]
fn list_offsets_width_8_are_supported() {
    let mut e = encoder(2);
    header(&mut e, 1, 2);
    column(&mut e, 1, ENC_LIST_OFFSETS, |e| {
        e.array(3).unwrap();
        e.u64(8).unwrap();
        e.bytes(&[0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0])
            .unwrap();
        raw_column(e, ENC_FOR, |e| {
            e.array(3).unwrap();
            e.u64(0).unwrap();
            e.u64(0).unwrap();
            e.bytes(&[]).unwrap();
        });
    });
    assert_eq!(
        columnar::from_slice::<ListRow>(&e.into_inner()).unwrap(),
        vec![
            ListRow {
                v: vec![0]
            },
            ListRow {
                v: vec![0]
            }
        ]
    );
}

#[test]
fn header_row_count_mismatch_is_rejected() {
    let mut e = encoder(3);
    header(&mut e, 1, 3);
    column(&mut e, 1, ENC_FOR, |e| {
        e.array(3).unwrap();
        e.u64(0).unwrap();
        e.u64(1).unwrap();
        e.bytes(&[0]).unwrap();
    });
    column(&mut e, 2, ENC_STRING_LENGTHS, |e| {
        e.array(2).unwrap();
        e.bytes(&[1]).unwrap();
        e.bytes(b"a").unwrap();
    });
    assert!(decode_wire(&e.into_inner()).is_err());
}

#[test]
fn zero_rows_with_nonempty_column_is_rejected() {
    let mut e = encoder(2);
    header(&mut e, 1, 0);
    column(&mut e, 1, ENC_PACKED, |e| {
        e.bytes(&[1]).unwrap();
    });
    assert!(decode_wire(&e.into_inner()).is_err());
}
