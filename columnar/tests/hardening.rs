//! Adversarial envelopes: declared lengths must not translate into large
//! allocations, extreme row counts must not panic, and every iterator must
//! terminate after an error.

use alloc_counter::measure;
use columnar::{
    __private::{Decoder, Encoder, encoder, finish, read_array_len, read_column, write_column},
    Columnar, ColumnarDecode, DecodeOptions, Encoding, Error, Field, FieldDecode, FieldIter, Result,
};

#[derive(Columnar, Debug, PartialEq)]
struct One {
    #[columnar(key = 1)]
    a: u64,
}

#[derive(Columnar, Debug, PartialEq)]
struct ListOne {
    #[columnar(key = 1)]
    a: Vec<u64>,
}

#[derive(Columnar, Debug, PartialEq)]
struct TextOne {
    #[columnar(key = 1)]
    a: String,
}

fn header(e: &mut Encoder<Vec<u8>>, rows: u64) {
    e.array(2).unwrap();
    e.u64(1).unwrap();
    e.u64(rows).unwrap();
}

fn envelope(rows: u64, entries: impl FnOnce(&mut Encoder<Vec<u8>>)) -> Vec<u8> {
    let mut e = encoder();
    e.map(2).unwrap();
    e.u64(0).unwrap();
    header(&mut e, rows);
    entries(&mut e);
    finish(e)
}

fn decode_one(bytes: &[u8]) -> Result<Vec<One>> {
    columnar::from_slice::<One>(bytes)
}

fn decode_list(bytes: &[u8], options: DecodeOptions) -> Result<Vec<ListOne>> {
    columnar::from_slice_with_options::<ListOne>(bytes, options)
}

#[test]
fn huge_row_count_with_truncated_column_does_not_allocate() {
    let bytes = envelope(16_000_000, |e| {
        e.u64(1).unwrap();
        e.array(2).unwrap();
        e.u64(3).unwrap(); // packed
        e.bytes(&[]).unwrap();
    });
    let (result, counters) = measure(|| decode_one(&bytes));
    assert!(result.is_err());
    assert!(counters.bytes < 1 << 20, "13-byte envelope allocated {} bytes", counters.bytes);
}

#[test]
fn huge_dictionary_length_does_not_allocate() {
    let bytes = envelope(0, |e| {
        e.u64(1).unwrap();
        e.array(2).unwrap();
        e.u64(8).unwrap(); // dictionary
        e.array(3).unwrap();
        e.u64(1 << 26).unwrap(); // dictionary length
        e.array(2).unwrap();
        e.u64(3).unwrap();
        e.bytes(&[]).unwrap(); // dictionary values
        e.bytes(&[]).unwrap(); // indices
    });
    let (result, counters) = measure(|| decode_one(&bytes));
    assert!(result.is_err());
    assert!(counters.bytes < 1 << 20, "18-byte envelope allocated {} bytes", counters.bytes);
}

#[test]
fn list_length_is_bounded_by_the_decoded_byte_budget() {
    // A single list of 2^28 zero values is valid according to max_list_len but
    // would need 2^31 bytes for u64, above the default 1 GiB output budget.
    let mut lengths = Vec::new();
    let mut value = 1u128 << 28;
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        lengths.push(byte);
        if value == 0 {
            break;
        }
    }
    let bytes = envelope(1, |e| {
        e.u64(1).unwrap();
        e.array(2).unwrap();
        e.u64(12).unwrap(); // list lengths
        e.array(2).unwrap();
        e.bytes(&lengths).unwrap();
        e.array(2).unwrap();
        e.u64(4).unwrap(); // FOR with width 0: no value bytes
        e.array(3).unwrap();
        e.u64(0).unwrap();
        e.u64(0).unwrap();
        e.bytes(&[]).unwrap();
    });
    let (result, counters) = measure(|| decode_list(&bytes, DecodeOptions::default()));
    assert!(
        matches!(result, Err(Error::LimitExceeded("decoded bytes"))),
        "unexpected result: {result:?}"
    );
    assert!(
        counters.bytes < 1 << 20,
        "list envelope allocated {} bytes before the limit check",
        counters.bytes
    );
}

#[test]
fn extreme_row_count_with_unlimited_options_returns_an_error() {
    let bytes = envelope(u64::MAX, |e| {
        e.u64(1).unwrap();
        e.array(2).unwrap();
        e.u64(10).unwrap(); // string offsets
        e.array(3).unwrap();
        e.u64(4).unwrap();
        e.bytes(&[]).unwrap();
        e.bytes(&[]).unwrap();
    });
    let result =
        std::panic::catch_unwind(|| TextOne::decode_rows(&bytes, DecodeOptions::unlimited()).map(|rows| rows.len()));
    let result = result.expect("decoding must not panic with unlimited options");
    assert!(result.is_err());
}

#[test]
fn decoded_byte_budget_rejects_an_otherwise_valid_column() {
    let rows: Vec<One> = (0..1_000)
        .map(|a| One {
            a,
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();
    let options = DecodeOptions {
        max_decoded_bytes: 1_024,
        ..DecodeOptions::default()
    };
    assert!(matches!(
        columnar::from_slice_with_options::<One>(&bytes, options),
        Err(Error::LimitExceeded("decoded bytes"))
    ));
    // A budget large enough for the output accepts the same bytes.
    let options = DecodeOptions {
        max_decoded_bytes: 1 << 20,
        ..DecodeOptions::default()
    };
    assert_eq!(columnar::from_slice_with_options::<One>(&bytes, options).unwrap(), rows);
}

#[test]
fn extreme_row_count_all_null_does_not_panic() {
    // `ENC_ALL_NULL` has no payload, so the declared row count is the only
    // bound. Unlimited options must still report an error for an impossible
    // vector size instead of panicking in `Vec::reserve`.
    let bytes = envelope(u64::MAX, |e| {
        e.u64(1).unwrap();
        e.array(2).unwrap();
        e.u64(14).unwrap(); // all null
        e.array(0).unwrap();
    });
    let result = std::panic::catch_unwind(|| {
        #[derive(Columnar, Debug, PartialEq)]
        struct Maybe {
            #[columnar(key = 1)]
            value: Option<u64>,
        }
        columnar::from_slice_with_options::<Maybe>(&bytes, DecodeOptions::unlimited()).map(|rows| rows.len())
    });
    let result = result.expect("decoding must not panic");
    assert!(result.is_err());
}

#[test]
fn overlong_varint_with_truncated_high_bits_is_rejected() {
    // 19 bytes: the final byte sits at shift 126, so only two bits fit in a
    // u128. 0x04 would silently decode as zero with the old reader.
    let mut lengths = vec![0x80u8; 18];
    lengths.push(0x04);
    let bytes = envelope(1, |e| {
        e.u64(1).unwrap();
        e.array(2).unwrap();
        e.u64(9).unwrap(); // string lengths
        e.array(2).unwrap();
        e.bytes(&lengths).unwrap();
        e.bytes(&[]).unwrap();
    });
    assert!(matches!(decode_text(&bytes), Err(Error::InvalidVarint)));
}

fn decode_text(bytes: &[u8]) -> Result<Vec<TextOne>> {
    columnar::from_slice::<TextOne>(bytes)
}

#[test]
fn overlong_varint_of_zero_is_rejected() {
    let mut lengths = vec![0x80u8; 18];
    lengths.push(0x00);
    let bytes = envelope(1, |e| {
        e.u64(1).unwrap();
        e.array(2).unwrap();
        e.u64(9).unwrap();
        e.array(2).unwrap();
        e.bytes(&lengths).unwrap();
        e.bytes(&[]).unwrap();
    });
    assert!(matches!(decode_text(&bytes), Err(Error::InvalidVarint)));
}

#[test]
fn direct_field_iterators_terminate_after_an_error() {
    // delta_rle with a valid run-length vector but no delta bytes: the first
    // read fails.
    let bytes = envelope(1, |e| {
        e.u64(1).unwrap();
        e.array(2).unwrap();
        e.u64(15).unwrap(); // delta rle
        e.array(2).unwrap();
        e.bytes(&[1]).unwrap(); // one run of length 1
        e.array(2).unwrap();
        e.u64(6).unwrap(); // varint
        e.bytes(&[]).unwrap(); // no deltas
    });

    let mut d = columnar::__private::decoder(&bytes, DecodeOptions::default());
    assert_eq!(d.map().unwrap(), Some(2));
    assert_eq!(d.u64().unwrap(), 0);
    read_array_len(&mut d, 2).unwrap();
    d.u64().unwrap();
    d.u64().unwrap();
    assert_eq!(d.u64().unwrap(), 1);
    let mut iter = <u64 as FieldIter>::decode_column_iter(&mut d, 1, &DecodeOptions::default()).unwrap();
    assert!(matches!(iter.next(), Some(Err(_))));
    assert!(iter.next().is_none(), "iterator yielded more than one error");
    assert_eq!(iter.len(), 0);
}

#[test]
fn composite_field_iterators_terminate_after_an_inner_error() {
    // Nullable outer column whose inner delta_rle payload is truncated.
    let bytes = envelope(1, |e| {
        e.u64(1).unwrap();
        e.array(2).unwrap();
        e.u64(11).unwrap(); // nullable
        e.array(2).unwrap();
        e.bytes(&[1]).unwrap(); // validity: one present value
        e.array(2).unwrap();
        e.u64(15).unwrap(); // delta rle
        e.array(2).unwrap();
        e.bytes(&[1]).unwrap();
        e.array(2).unwrap();
        e.u64(6).unwrap();
        e.bytes(&[]).unwrap();
    });
    let mut d = columnar::__private::decoder(&bytes, DecodeOptions::default());
    assert_eq!(d.map().unwrap(), Some(2));
    d.u64().unwrap();
    read_array_len(&mut d, 2).unwrap();
    d.u64().unwrap();
    d.u64().unwrap();
    d.u64().unwrap();
    let mut iter = <Option<u64> as FieldIter>::decode_column_iter(&mut d, 1, &DecodeOptions::default()).unwrap();
    assert!(matches!(iter.next(), Some(Err(_))));
    assert!(iter.next().is_none());

    // List whose child delta_rle payload is truncated.
    let bytes = envelope(1, |e| {
        e.u64(1).unwrap();
        e.array(2).unwrap();
        e.u64(12).unwrap(); // list lengths
        e.array(2).unwrap();
        e.bytes(&[1]).unwrap();
        e.array(2).unwrap();
        e.u64(15).unwrap();
        e.array(2).unwrap();
        e.bytes(&[1]).unwrap();
        e.array(2).unwrap();
        e.u64(6).unwrap();
        e.bytes(&[]).unwrap();
    });
    let mut d = columnar::__private::decoder(&bytes, DecodeOptions::default());
    assert_eq!(d.map().unwrap(), Some(2));
    d.u64().unwrap();
    read_array_len(&mut d, 2).unwrap();
    d.u64().unwrap();
    d.u64().unwrap();
    d.u64().unwrap();
    let mut iter = <Vec<u64> as FieldIter>::decode_column_iter(&mut d, 1, &DecodeOptions::default()).unwrap();
    assert!(matches!(iter.next(), Some(Err(_))));
    assert!(iter.next().is_none());
}

/// A user-provided `FieldDecode` that violates the documented "exactly rows
/// values" contract must be reported, not panic the derive-generated code.
#[derive(Debug, PartialEq)]
struct Short;

impl Field for Short {
    fn encode_column<'a, I>(_values: I, out: &mut Encoder<Vec<u8>>, _hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        write_column(out, columnar::__private::ENC_STRUCT)?;
        out.array(0)?;
        Ok(())
    }
}

impl<'de> FieldDecode<'de> for Short {
    fn decode_column(d: &mut Decoder<'de>, _rows: usize, _options: &DecodeOptions) -> Result<Vec<Self>> {
        let encoding = read_column(d)?;
        if encoding != columnar::__private::ENC_STRUCT {
            return Err(Error::InvalidEncoding(encoding));
        }
        read_array_len(d, 0)?;
        Ok(Vec::new())
    }
}

#[derive(Columnar, Debug, PartialEq)]
struct HasShort {
    #[columnar(key = 1)]
    a: Short,
}

#[test]
fn short_user_column_returns_an_error_instead_of_panicking() {
    let bytes = envelope(2, |e| {
        e.u64(1).unwrap();
        e.array(2).unwrap();
        e.u64(columnar::__private::ENC_STRUCT).unwrap();
        e.array(0).unwrap();
    });
    let result =
        std::panic::catch_unwind(|| HasShort::decode_rows(&bytes, DecodeOptions::default()).map(|rows| rows.len()));
    let result = result.expect("decoding must not panic");
    assert!(matches!(result, Err(Error::LengthMismatch { .. })), "{result:?}");
}
