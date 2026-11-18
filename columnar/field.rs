//! Implementations of the column encodings for Rust types.
//!
//! Every type that can appear as a struct field implements [`Field`]. The
//! derive macro uses these implementations to turn `&[Struct]` into columns and
//! back.

use alloc::{
    borrow::Cow,
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use core::marker::PhantomData;

use cbor::{Decoder, Encoder};

use crate::{
    DecodeOptions, Encoding, Error, Result,
    bitpack::{BitReader, BitWriter, bits_needed, count_valid, packed_len, unpack_into},
    format::*,
};

/// A type that can be encoded as a column.
///
/// Decoding is provided separately by [`FieldDecode`] so that borrowed types
/// can tie their lifetime to the input buffer. Most users do not implement
/// either trait by hand; `#[derive(Columnar)]` generates them for structs and
/// enums.
pub trait Field: Sized {
    /// Encode a sequence of values as one column.
    ///
    /// `hint` is [`Encoding::Auto`] unless the field has a
    /// `#[columnar(encoding = "...")]` attribute.
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a;
}

/// A field that can be decoded from a column, borrowing from `d` when needed.
///
/// Owned types implement this for every `'de`. Borrowed types such as
/// `Cow<'de, [u8]>` and `Cow<'de, str>` implement it for their matching
/// lifetime.
pub trait FieldDecode<'de>: Sized {
    /// Decode one column with exactly `rows` logical values.
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>>;

    /// Decode exactly `rows` values in order, passing each one to `sink`.
    ///
    /// The default implementation decodes a vector and forwards its values.
    /// Types that can decode incrementally override this method so callers can
    /// avoid an intermediate vector, for example when decoding list elements.
    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        for value in Self::decode_column(d, rows, options)? {
            sink(value)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Shared payload helpers
// ---------------------------------------------------------------------------

pub(crate) fn packed_bytes<'a>(d: &mut Decoder<'a>, rows: usize, width: usize) -> Result<&'a [u8]> {
    let data = d.bytes()?;
    let expected = rows.checked_mul(width).ok_or(Error::LengthOverflow)?;
    if data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: data.len(),
        });
    }
    Ok(data)
}

/// Validate UTF-8, using the SIMD-accelerated workspace crate for long slices.
#[inline]
fn validated_str(bytes: &[u8]) -> Result<&str> {
    if bytes.len() >= 64 {
        utf8::from_utf8(bytes).map_err(|_| Error::InvalidUtf8)
    } else {
        core::str::from_utf8(bytes).map_err(|_| Error::InvalidUtf8)
    }
}

/// Check the declared output size and return an allocation hint.
fn decode_capacity<T>(rows: usize, options: &DecodeOptions) -> Result<usize> {
    options.output_capacity::<T>(rows)
}

/// Reserve the output capacity on the first decoded value.
///
/// Encodings validate their metadata before producing a value, so reaching this
/// helper proves that the declared row count is backed by a well-formed payload.
fn push_decoded<T>(out: &mut Vec<T>, value: T, rows: usize, options: &DecodeOptions) -> Result<()> {
    if out.capacity() == 0 {
        out.reserve(decode_capacity::<T>(rows, options)?);
    }
    out.push(value);
    Ok(())
}

/// Statistics used by [`Encoding::Auto`] for integer columns.
///
/// The pass is linear and allocation-free. It records the value range, run
/// lengths, and first/second differences so Auto can pick run-length or delta
/// encodings when they are substantially smaller than packed/FOR data.
struct RangeStats {
    rows: usize,
    min: i128,
    max: i128,
    runs: usize,
    run_len: usize,
    max_run_len: usize,
}

impl RangeStats {
    fn width(&self) -> u8 {
        if self.rows == 0 {
            0
        } else {
            bits_needed((self.max - self.min) as u128)
        }
    }

    fn simple(&self, type_bits: u8) -> AutoChoice {
        if self.width() == type_bits {
            AutoChoice::Packed
        } else {
            AutoChoice::For
        }
    }

    fn simple_bytes(&self, type_bits: u8, type_bytes: usize) -> usize {
        let width = self.width();
        if width == type_bits {
            self.rows.saturating_mul(type_bytes)
        } else {
            packed_len(self.rows, width).unwrap_or(usize::MAX)
        }
    }

    fn rle_bytes(&self, type_bits: u8, type_bytes: usize) -> usize {
        let width = self.width();
        let run_value_bytes = if width == type_bits {
            self.runs.saturating_mul(type_bytes)
        } else {
            packed_len(self.runs, width).unwrap_or(usize::MAX)
        };
        run_value_bytes
            .saturating_add(self.runs.saturating_mul(varint_len(self.max_run_len as u128)))
            .saturating_add(16)
    }
}

fn range_stats<'a, T, I>(values: I, to_i128: fn(&T) -> i128) -> RangeStats
where
    T: 'a,
    I: Iterator<Item = &'a T>,
{
    let mut stats = RangeStats {
        rows: 0,
        min: i128::MAX,
        max: i128::MIN,
        runs: 0,
        run_len: 0,
        max_run_len: 0,
    };
    let mut last = 0i128;
    let mut have_last = false;
    for value in values {
        let value = to_i128(value);
        stats.rows += 1;
        stats.min = stats.min.min(value);
        stats.max = stats.max.max(value);
        if have_last && value == last {
            stats.run_len += 1;
        } else {
            stats.runs += 1;
            stats.run_len = 1;
        }
        stats.max_run_len = stats.max_run_len.max(stats.run_len);
        last = value;
        have_last = true;
    }
    stats
}

struct DeltaStats {
    rows: usize,
    delta_runs: usize,
    max_delta_run_len: usize,
    first_value_abs: u128,
    max_delta_abs_rest: u128,
    second_dod_abs: u128,
    max_dod_abs_rest: u128,
}

fn delta_stats<'a, T, I>(values: I, to_i128: fn(&T) -> i128) -> DeltaStats
where
    T: 'a,
    I: Iterator<Item = &'a T>,
{
    let mut rows = 0usize;
    let mut delta_runs = 0usize;
    let mut delta_run_len = 0usize;
    let mut max_delta_run_len = 0usize;
    let mut first_value_abs = 0u128;
    let mut max_delta_abs_rest = 0u128;
    let mut second_dod_abs = 0u128;
    let mut max_dod_abs_rest = 0u128;
    let mut previous = 0i128;
    let mut previous_delta = 0i128;
    let mut last_delta = 0i128;
    let mut have_previous = false;
    let mut have_delta = false;
    for value in values {
        let value = to_i128(value);
        rows += 1;
        let delta = if have_previous { value - previous } else { value };
        if have_delta && delta == last_delta {
            delta_run_len += 1;
        } else {
            delta_runs += 1;
            delta_run_len = 1;
            if have_previous {
                max_delta_abs_rest = max_delta_abs_rest.max(delta.unsigned_abs());
            } else {
                first_value_abs = value.unsigned_abs();
            }
        }
        max_delta_run_len = max_delta_run_len.max(delta_run_len);
        let dod = if have_previous { delta - previous_delta } else { value };
        if have_previous {
            if rows == 2 {
                second_dod_abs = dod.unsigned_abs();
            } else {
                max_dod_abs_rest = max_dod_abs_rest.max(dod.unsigned_abs());
            }
        }
        previous = value;
        previous_delta = delta;
        last_delta = delta;
        have_previous = true;
        have_delta = true;
    }
    DeltaStats {
        rows,
        delta_runs,
        max_delta_run_len,
        first_value_abs,
        max_delta_abs_rest,
        second_dod_abs,
        max_dod_abs_rest,
    }
}

const DELTA_SAMPLE_LIMIT: usize = 4096;

/// Estimate delta statistics on at most [`DELTA_SAMPLE_LIMIT`] samples.
///
/// The full range/run scan already ran, so this pass only needs to recognise
/// regular first differences and small second differences. Sampling keeps the
/// encoder fast on large columns; the estimated run counts are scaled back to
/// the full row count. Sampling every `stride`-th value preserves first
/// differences (scaled by `stride`) and, importantly, second differences.
fn sampled_delta_stats<'a, T, I>(range: &RangeStats, values: I, to_i128: fn(&T) -> i128) -> DeltaStats
where
    T: 'a,
    I: Iterator<Item = &'a T> + Clone,
{
    let stride = (range.rows / DELTA_SAMPLE_LIMIT).max(1);
    let mut delta = delta_stats(values.step_by(stride), to_i128);
    delta.rows = range.rows;
    delta.delta_runs = delta.delta_runs.saturating_mul(stride).min(range.rows.max(1));
    delta
}

fn choose_integer(range: &RangeStats, delta: &DeltaStats, type_bits: u8, type_bytes: usize) -> AutoChoice {
    let simple = range.simple(type_bits);
    let mut best = simple;
    let mut best_bytes = range.simple_bytes(type_bits, type_bytes);
    if range.rows < 32 {
        return best;
    }

    let rle_bytes = range.rle_bytes(type_bits, type_bytes);
    if rle_bytes.saturating_mul(100) < best_bytes.saturating_mul(95) {
        best = AutoChoice::Rle;
        best_bytes = rle_bytes;
    }

    let first_bytes = varint_len(zigzag(delta.first_value_abs as i128));
    let rest_deltas = delta.delta_runs.saturating_sub(1);
    let delta_bytes =
        first_bytes.saturating_add(rest_deltas.saturating_mul(varint_len(zigzag(delta.max_delta_abs_rest as i128))));
    let delta_length_bytes = delta
        .delta_runs
        .saturating_mul(varint_len(delta.max_delta_run_len as u128));
    let delta_rle_bytes = delta_bytes.saturating_add(delta_length_bytes).saturating_add(16);

    let second_bytes = if delta.rows >= 2 {
        varint_len(zigzag(delta.second_dod_abs as i128))
    } else {
        0
    };
    let dod_bytes = first_bytes
        .saturating_add(second_bytes)
        .saturating_add(
            delta
                .rows
                .saturating_sub(2)
                .saturating_mul(varint_len(zigzag(delta.max_dod_abs_rest as i128))),
        )
        .saturating_add(16);

    for (choice, cost) in [
        (AutoChoice::DeltaRle, delta_rle_bytes),
        (AutoChoice::DeltaOfDelta, dod_bytes),
    ] {
        if cost.saturating_mul(100) < best_bytes.saturating_mul(95) {
            best = choice;
            best_bytes = cost;
        }
    }
    best
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AutoChoice {
    Packed,
    For,
    Rle,
    DeltaRle,
    DeltaOfDelta,
}

/// Run-length statistics for boolean, float, string and byte columns.
struct RunStats {
    rows: usize,
    runs: usize,
    run_len: usize,
    max_run_len: usize,
}

impl RunStats {
    fn new() -> Self {
        RunStats {
            rows: 0,
            runs: 0,
            run_len: 0,
            max_run_len: 0,
        }
    }

    #[inline]
    fn observe(&mut self, equal: bool) {
        self.rows += 1;
        if equal {
            self.run_len += 1;
        } else {
            self.runs += 1;
            self.run_len = 1;
        }
        self.max_run_len = self.max_run_len.max(self.run_len);
    }

    /// Estimated RLE payload size: run values plus run lengths.
    fn rle_bytes(&self, value_bytes: usize) -> usize {
        self.runs
            .saturating_mul(value_bytes)
            .saturating_add(self.runs.saturating_mul(varint_len(self.max_run_len as u128)))
            .saturating_add(16)
    }
}

fn exact_len<I: Iterator + Clone>(values: &I) -> usize {
    let (lower, upper) = values.size_hint();
    match upper {
        Some(upper) if upper == lower => lower,
        _ => values.clone().count(),
    }
}

fn encode_packed_u64<I>(values: I, width: usize, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = u64> + Clone,
{
    let rows = exact_len(&values);
    let bytes = rows.checked_mul(width).ok_or(Error::LengthOverflow)?;
    write_column(out, ENC_PACKED)?;
    out.bytes_len(bytes as u64)?;
    let buf = out.writer_mut();
    buf.reserve(bytes);
    for value in values {
        buf.extend_from_slice(&value.to_le_bytes()[..width]);
    }
    Ok(())
}

fn encode_packed_u128<I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = u128> + Clone,
{
    let rows = exact_len(&values);
    let bytes = rows.checked_mul(16).ok_or(Error::LengthOverflow)?;
    write_column(out, ENC_PACKED)?;
    out.bytes_len(bytes as u64)?;
    let buf = out.writer_mut();
    buf.reserve(bytes);
    for value in values {
        buf.extend_from_slice(&value.to_le_bytes());
    }
    Ok(())
}

fn encode_for_u64<I>(values: I, base: u64, width: u8, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = u64> + Clone,
{
    let rows = exact_len(&values);
    let bytes = packed_len(rows, width)?;
    write_column(out, ENC_FOR)?;
    out.array(3)?;
    out.u64(base)?;
    out.u64(u64::from(width))?;
    out.bytes_len(bytes as u64)?;
    let mut writer = BitWriter::new(out.writer_mut());
    for value in values {
        writer.write(value.wrapping_sub(base), width);
    }
    writer.finish();
    Ok(())
}

fn encode_for_i64<I>(values: I, base: i64, width: u8, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = i64> + Clone,
{
    let rows = exact_len(&values);
    let bytes = packed_len(rows, width)?;
    write_column(out, ENC_FOR)?;
    out.array(3)?;
    out.i64(base)?;
    out.u64(u64::from(width))?;
    out.bytes_len(bytes as u64)?;
    let mut writer = BitWriter::new(out.writer_mut());
    for value in values {
        writer.write((value as i128 - base as i128) as u64, width);
    }
    writer.finish();
    Ok(())
}

fn decode_for_unsigned_into<'de, T, F>(d: &mut Decoder<'de>, rows: usize, mut sink: F) -> Result<()>
where
    T: TryFrom<u128>,
    F: FnMut(T) -> Result<()>,
{
    read_array_len(d, 3)?;
    let base = d.u64()?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    if width > 64 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let data = d.bytes()?;
    let expected = packed_len(rows, width as u8)?;
    if data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: data.len(),
        });
    }
    let mut buffer = [0u64; 64];
    let mut bit_pos = 0usize;
    let mut remaining = rows;
    while remaining > 0 {
        let take = remaining.min(buffer.len());
        unpack_into(data, &mut bit_pos, width as u8, &mut buffer[..take])?;
        for &offset in &buffer[..take] {
            let value = u128::from(base).wrapping_add(u128::from(offset));
            sink(T::try_from(value).map_err(|_| Error::LengthOverflow)?)?;
        }
        remaining -= take;
    }
    Ok(())
}

fn decode_for_signed_into<'de, T, F>(d: &mut Decoder<'de>, rows: usize, mut sink: F) -> Result<()>
where
    T: TryFrom<i128>,
    F: FnMut(T) -> Result<()>,
{
    read_array_len(d, 3)?;
    let base = d.i64()?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    if width > 64 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let data = d.bytes()?;
    let expected = packed_len(rows, width as u8)?;
    if data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: data.len(),
        });
    }
    let mut buffer = [0u64; 64];
    let mut bit_pos = 0usize;
    let mut remaining = rows;
    while remaining > 0 {
        let take = remaining.min(buffer.len());
        unpack_into(data, &mut bit_pos, width as u8, &mut buffer[..take])?;
        for &offset in &buffer[..take] {
            let value = base as i128 + offset as i128;
            sink(T::try_from(value).map_err(|_| Error::LengthOverflow)?)?;
        }
        remaining -= take;
    }
    Ok(())
}

fn encode_varint_unsigned<I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = u128> + Clone,
{
    let bytes: usize = values.clone().map(varint_len).sum();
    write_column(out, ENC_VARINT)?;
    out.bytes_len(bytes as u64)?;
    let buf = out.writer_mut();
    buf.reserve(bytes);
    for value in values {
        write_varint(buf, value);
    }
    Ok(())
}

fn encode_varint_signed<I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = i128> + Clone,
{
    let bytes: usize = values.clone().map(|value| varint_len(zigzag(value))).sum();
    write_column(out, ENC_VARINT)?;
    out.bytes_len(bytes as u64)?;
    let buf = out.writer_mut();
    buf.reserve(bytes);
    for value in values {
        write_varint(buf, zigzag(value));
    }
    Ok(())
}

fn decode_varint_unsigned_into<'de, T, F>(d: &mut Decoder<'de>, rows: usize, mut sink: F) -> Result<()>
where
    T: TryFrom<u128>,
    F: FnMut(T) -> Result<()>,
{
    let data = d.bytes()?;
    let mut pos = 0;
    for _ in 0..rows {
        let value = read_varint(data, &mut pos)?;
        sink(T::try_from(value).map_err(|_| Error::LengthOverflow)?)?;
    }
    if pos != data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: data.len(),
        });
    }
    Ok(())
}

fn decode_varint_signed_into<'de, T, F>(d: &mut Decoder<'de>, rows: usize, mut sink: F) -> Result<()>
where
    T: TryFrom<i128>,
    F: FnMut(T) -> Result<()>,
{
    let data = d.bytes()?;
    let mut pos = 0;
    for _ in 0..rows {
        let value = unzigzag(read_varint(data, &mut pos)?);
        sink(T::try_from(value).map_err(|_| Error::LengthOverflow)?)?;
    }
    if pos != data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: data.len(),
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// RLE and dictionary, shared by leaf types
// ---------------------------------------------------------------------------

fn encode_rle<'a, T, I>(values: I, out: &mut Encoder<Vec<u8>>, equal: impl Fn(&T, &T) -> bool) -> Result<()>
where
    T: Field + 'a,
    I: Iterator<Item = &'a T> + Clone,
{
    let (lower, _) = values.size_hint();
    let mut run_values: Vec<&'a T> = Vec::with_capacity(lower.min(4096));
    let mut lengths: Vec<u32> = Vec::with_capacity(lower.min(4096));
    for value in values {
        if let Some(last) = run_values.last()
            && equal(last, value)
        {
            let last_len = lengths.last_mut().expect("run length exists");
            *last_len = last_len.checked_add(1).ok_or(Error::LengthOverflow)?;
            continue;
        }
        run_values.push(value);
        lengths.push(1);
    }
    let length_bytes: usize = lengths.iter().map(|length| varint_len(u128::from(*length))).sum();
    write_column(out, ENC_RLE)?;
    out.array(2)?;
    out.bytes_len(length_bytes as u64)?;
    {
        let buf = out.writer_mut();
        buf.reserve(length_bytes);
        for length in &lengths {
            write_varint(buf, u128::from(*length));
        }
    }
    T::encode_column(run_values.iter().copied(), out, Encoding::Auto)?;
    Ok(())
}

fn decode_rle_into<'de, T, F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
where
    T: FieldDecode<'de> + Clone,
    F: FnMut(T) -> Result<()>,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let mut pos = 0;
    let mut total = 0usize;
    let mut run_count = 0usize;
    while pos < length_data.len() {
        if run_count >= options.max_list_len {
            return Err(Error::LimitExceeded("rle runs"));
        }
        let length = u32::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        if length == 0 {
            return Err(Error::ZeroRun);
        }
        total = total.checked_add(length as usize).ok_or(Error::LengthOverflow)?;
        if total > rows {
            return Err(Error::LengthMismatch {
                expected: rows,
                actual: total,
            });
        }
        run_count += 1;
    }
    if total != rows {
        return Err(Error::LengthMismatch {
            expected: rows,
            actual: total,
        });
    }
    let mut pos = 0;
    for value in T::decode_column(d, run_count, options)? {
        let length = u32::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        for _ in 0..length {
            sink(value.clone())?;
        }
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Delta RLE and delta-of-delta, shared by integer types
// ---------------------------------------------------------------------------

fn encode_delta_rle<I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = i128> + Clone,
{
    let (lower, _) = values.size_hint();
    let mut lengths: Vec<u32> = Vec::with_capacity(lower.min(4096));
    let mut delta_bytes = 0usize;
    let mut previous = 0i128;
    let mut last_delta = None;
    for value in values.clone() {
        let delta = value.checked_sub(previous).ok_or(Error::DeltaOverflow)?;
        previous = value;
        if last_delta == Some(delta) {
            let last_length = lengths.last_mut().expect("delta run length exists");
            *last_length = last_length.checked_add(1).ok_or(Error::LengthOverflow)?;
            continue;
        }
        last_delta = Some(delta);
        lengths.push(1);
        delta_bytes += varint_len(zigzag(delta));
    }
    let length_bytes: usize = lengths.iter().map(|length| varint_len(u128::from(*length))).sum();
    write_column(out, ENC_DELTA_RLE)?;
    out.array(2)?;
    out.bytes_len(length_bytes as u64)?;
    {
        let buf = out.writer_mut();
        buf.reserve(length_bytes);
        for length in &lengths {
            write_varint(buf, u128::from(*length));
        }
    }
    write_column(out, ENC_VARINT)?;
    out.bytes_len(delta_bytes as u64)?;
    {
        let buf = out.writer_mut();
        buf.reserve(delta_bytes);
        let mut previous = 0i128;
        let mut last_delta = None;
        for value in values {
            let delta = value.checked_sub(previous).ok_or(Error::DeltaOverflow)?;
            previous = value;
            if last_delta == Some(delta) {
                continue;
            }
            last_delta = Some(delta);
            write_varint(buf, zigzag(delta));
        }
    }
    Ok(())
}

fn decode_delta_rle_into<'de, T, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    T: TryFrom<i128>,
    F: FnMut(T) -> Result<()>,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let mut pos = 0;
    let mut total = 0usize;
    let mut run_count = 0usize;
    while pos < length_data.len() {
        if run_count >= options.max_list_len {
            return Err(Error::LimitExceeded("delta rle runs"));
        }
        let length = u32::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        if length == 0 {
            return Err(Error::ZeroRun);
        }
        total = total.checked_add(length as usize).ok_or(Error::LengthOverflow)?;
        if total > rows {
            return Err(Error::LengthMismatch {
                expected: rows,
                actual: total,
            });
        }
        run_count += 1;
    }
    if total != rows {
        return Err(Error::LengthMismatch {
            expected: rows,
            actual: total,
        });
    }
    let encoding = read_column(d)?;
    if encoding != ENC_VARINT {
        return Err(Error::InvalidEncoding(encoding));
    }
    let delta_data = d.bytes()?;
    let mut pos = 0;
    let mut delta_pos = 0;
    let mut absolute = 0i128;
    for _ in 0..run_count {
        let length = u32::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        let delta = unzigzag(read_varint(delta_data, &mut delta_pos)?);
        for _ in 0..length {
            absolute = absolute.checked_add(delta).ok_or(Error::DeltaOverflow)?;
            sink(T::try_from(absolute).map_err(|_| Error::DeltaOverflow)?)?;
        }
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    if delta_pos != delta_data.len() {
        return Err(Error::LengthMismatch {
            expected: delta_pos,
            actual: delta_data.len(),
        });
    }
    Ok(())
}

fn encode_delta_of_delta<I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = i128> + Clone,
{
    let mut bytes = 0usize;
    {
        let mut previous_value = None;
        let mut previous_delta = 0i128;
        for value in values.clone() {
            let encoded = if let Some(previous) = previous_value {
                let delta = value.checked_sub(previous).ok_or(Error::DeltaOverflow)?;
                let delta_of_delta = delta.checked_sub(previous_delta).ok_or(Error::DeltaOverflow)?;
                previous_delta = delta;
                delta_of_delta
            } else {
                value
            };
            previous_value = Some(value);
            bytes += varint_len(zigzag(encoded));
        }
    }
    write_column(out, ENC_DELTA_OF_DELTA)?;
    write_column(out, ENC_VARINT)?;
    out.bytes_len(bytes as u64)?;
    {
        let buf = out.writer_mut();
        buf.reserve(bytes);
        let mut previous_value = None;
        let mut previous_delta = 0i128;
        for value in values {
            let encoded = if let Some(previous) = previous_value {
                let delta = value.checked_sub(previous).ok_or(Error::DeltaOverflow)?;
                let delta_of_delta = delta.checked_sub(previous_delta).ok_or(Error::DeltaOverflow)?;
                previous_delta = delta;
                delta_of_delta
            } else {
                value
            };
            previous_value = Some(value);
            write_varint(buf, zigzag(encoded));
        }
    }
    Ok(())
}

fn decode_delta_of_delta_into<'de, T, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    _options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    T: TryFrom<i128>,
    F: FnMut(T) -> Result<()>,
{
    let encoding = read_column(d)?;
    if encoding != ENC_VARINT {
        return Err(Error::InvalidEncoding(encoding));
    }
    let data = d.bytes()?;
    let mut pos = 0;
    let mut previous_value = 0i128;
    let mut previous_delta = 0i128;
    for index in 0..rows {
        let encoded = unzigzag(read_varint(data, &mut pos)?);
        let value = if index == 0 {
            previous_value = encoded;
            encoded
        } else {
            previous_delta = previous_delta.checked_add(encoded).ok_or(Error::DeltaOverflow)?;
            previous_value = previous_value.checked_add(previous_delta).ok_or(Error::DeltaOverflow)?;
            previous_value
        };
        sink(T::try_from(value).map_err(|_| Error::DeltaOverflow)?)?;
    }
    if pos != data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: data.len(),
        });
    }
    Ok(())
}

fn encode_dictionary<'a, T, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    T: Field + Ord + 'a,
    I: Iterator<Item = &'a T> + Clone,
{
    let rows = exact_len(&values);
    let mut dict: Vec<&T> = Vec::with_capacity(rows);
    dict.extend(values.clone());
    dict.sort_unstable();
    dict.dedup();
    let width = bits_needed(u128::from(dict.len().saturating_sub(1) as u64));
    let index_bytes = packed_len(rows, width)?;
    write_column(out, ENC_DICTIONARY)?;
    out.array(3)?;
    out.u64(dict.len() as u64)?;
    T::encode_column(dict.iter().copied(), out, Encoding::Auto)?;
    out.bytes_len(index_bytes as u64)?;
    {
        let mut writer = BitWriter::new(out.writer_mut());
        for value in values {
            let index = dict.binary_search(&value).map_err(|_| Error::InvalidBitpackedValue)?;
            writer.write(index as u64, width);
        }
        writer.finish();
    }
    Ok(())
}

fn decode_dictionary_into<'de, T, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    T: FieldDecode<'de> + Clone,
    F: FnMut(T) -> Result<()>,
{
    read_array_len(d, 3)?;
    let dict_len = d.u64()?;
    let dict_len = usize::try_from(dict_len).map_err(|_| Error::LengthOverflow)?;
    if dict_len > options.max_list_len {
        return Err(Error::LimitExceeded("dictionary length"));
    }
    options.check_output_bytes::<T>(dict_len)?;
    if dict_len == 0 && rows != 0 {
        return Err(Error::DictionaryIndex {
            index: 0,
            len: 0,
        });
    }
    let dict = T::decode_column(d, dict_len, options)?;
    let indices = d.bytes()?;
    let width = bits_needed(u128::from(dict_len.saturating_sub(1) as u64));
    let expected = packed_len(rows, width)?;
    if indices.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: indices.len(),
        });
    }
    let mut reader = BitReader::new(indices);
    for _ in 0..rows {
        let index = u32::try_from(reader.read(width)?).map_err(|_| Error::InvalidBitpackedValue)?;
        let value = dict.get(index as usize).ok_or(Error::DictionaryIndex {
            index,
            len: dict.len(),
        })?;
        sink(value.clone())?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Integer macros
// ---------------------------------------------------------------------------

fn encode_unsigned_for<'a, T, I>(
    values: I,
    out: &mut Encoder<Vec<u8>>,
    to_u64: fn(&T) -> u64,
    to_i128: fn(&T) -> i128,
) -> Result<()>
where
    T: Field + PartialEq + 'a,
    I: Iterator<Item = &'a T> + Clone,
{
    let range = range_stats(values.clone(), to_i128);
    let width = range.width();
    let base = if range.rows == 0 { 0 } else { range.min as u64 };
    encode_for_u64(values.map(to_u64), base, width, out)
}

fn encode_unsigned_auto<'a, T, I>(
    values: I,
    out: &mut Encoder<Vec<u8>>,
    type_bytes: usize,
    type_bits: u8,
    to_u64: fn(&T) -> u64,
    to_i128: fn(&T) -> i128,
) -> Result<()>
where
    T: Field + PartialEq + 'a,
    I: Iterator<Item = &'a T> + Clone,
{
    let range = range_stats(values.clone(), to_i128);
    let width = range.width();
    let simple = range.simple(type_bits);
    let simple_bytes = range.simple_bytes(type_bits, type_bytes);
    let rle_bytes = range.rle_bytes(type_bits, type_bytes);
    if range.rows < 32 || (width == type_bits && range.runs.saturating_mul(64) >= range.rows.saturating_mul(63)) {
        return match simple {
            AutoChoice::Packed => encode_packed_u64(values.map(to_u64), type_bytes, out),
            _ => encode_for_u64(values.map(to_u64), range.min as u64, width, out),
        };
    }
    if width == 0 || rle_bytes.saturating_mul(100) < simple_bytes.saturating_mul(10) {
        return encode_rle(values, out, |a, b| a == b);
    }
    let delta = sampled_delta_stats(&range, values.clone(), to_i128);
    match choose_integer(&range, &delta, type_bits, type_bytes) {
        AutoChoice::Packed => encode_packed_u64(values.map(to_u64), type_bytes, out),
        AutoChoice::For => encode_for_u64(values.map(to_u64), range.min as u64, width, out),
        AutoChoice::Rle => encode_rle(values, out, |a, b| a == b),
        AutoChoice::DeltaRle => encode_delta_rle(values.map(to_i128), out),
        AutoChoice::DeltaOfDelta => encode_delta_of_delta(values.map(to_i128), out),
    }
}

fn encode_signed_for<'a, T, I>(
    values: I,
    out: &mut Encoder<Vec<u8>>,
    to_i64: fn(&T) -> i64,
    to_i128: fn(&T) -> i128,
) -> Result<()>
where
    T: Field + PartialEq + 'a,
    I: Iterator<Item = &'a T> + Clone,
{
    let range = range_stats(values.clone(), to_i128);
    let width = range.width();
    let base = if range.rows == 0 { 0 } else { range.min as i64 };
    encode_for_i64(values.map(to_i64), base, width, out)
}

fn encode_signed_auto<'a, T, I>(
    values: I,
    out: &mut Encoder<Vec<u8>>,
    type_bytes: usize,
    type_bits: u8,
    to_u64: fn(&T) -> u64,
    to_i64: fn(&T) -> i64,
    to_i128: fn(&T) -> i128,
) -> Result<()>
where
    T: Field + PartialEq + 'a,
    I: Iterator<Item = &'a T> + Clone,
{
    let range = range_stats(values.clone(), to_i128);
    let width = range.width();
    let simple = range.simple(type_bits);
    let simple_bytes = range.simple_bytes(type_bits, type_bytes);
    let rle_bytes = range.rle_bytes(type_bits, type_bytes);
    if range.rows < 32 || (width == type_bits && range.runs.saturating_mul(64) >= range.rows.saturating_mul(63)) {
        return match simple {
            AutoChoice::Packed => encode_packed_u64(values.map(to_u64), type_bytes, out),
            _ => encode_for_i64(values.map(to_i64), range.min as i64, width, out),
        };
    }
    if width == 0 || rle_bytes.saturating_mul(100) < simple_bytes.saturating_mul(10) {
        return encode_rle(values, out, |a, b| a == b);
    }
    let delta = sampled_delta_stats(&range, values.clone(), to_i128);
    match choose_integer(&range, &delta, type_bits, type_bytes) {
        AutoChoice::Packed => encode_packed_u64(values.map(to_u64), type_bytes, out),
        AutoChoice::For => encode_for_i64(values.map(to_i64), range.min as i64, width, out),
        AutoChoice::Rle => encode_rle(values, out, |a, b| a == b),
        AutoChoice::DeltaRle => encode_delta_rle(values.map(to_i128), out),
        AutoChoice::DeltaOfDelta => encode_delta_of_delta(values.map(to_i128), out),
    }
}

macro_rules! impl_unsigned_int {
    ($ty:ty, $bytes:expr, $bits:expr) => {
        impl Field for $ty {
            fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
            where
                I: Iterator<Item = &'a Self> + Clone,
                Self: 'a,
            {
                match hint {
                    Encoding::Auto => encode_unsigned_auto(
                        values,
                        out,
                        $bytes,
                        $bits,
                        |value| u64::from(*value),
                        |value| i128::from(*value),
                    ),
                    Encoding::ForBitpack => {
                        encode_unsigned_for(values, out, |value| u64::from(*value), |value| i128::from(*value))
                    }
                    Encoding::Packed => encode_packed_u64(values.map(|value| u64::from(*value)), $bytes, out),
                    Encoding::Varint => encode_varint_unsigned(values.map(|value| u128::from(*value)), out),
                    Encoding::Rle => encode_rle(values, out, |a, b| a == b),
                    Encoding::DeltaRle => encode_delta_rle(values.map(|value| i128::from(*value)), out),
                    Encoding::DeltaOfDelta => encode_delta_of_delta(values.map(|value| i128::from(*value)), out),
                    Encoding::Dictionary => encode_dictionary(values, out),
                    other => Err(Error::UnsupportedEncoding(other)),
                }
            }
        }

        impl<'de> FieldDecode<'de> for $ty {
            fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
                let position = d.position();
                let encoding = read_column(d)?;
                if encoding == ENC_PACKED {
                    let data = packed_bytes(d, rows, $bytes)?;
                    options.check_output_bytes::<Self>(rows)?;
                    return Ok(data
                        .as_chunks::<$bytes>()
                        .0
                        .iter()
                        .map(|chunk| <$ty>::from_le_bytes(*chunk))
                        .collect());
                }
                d.set_position(position);
                let mut out = Vec::new();
                Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
                Ok(out)
            }

            fn decode_column_into<F>(
                d: &mut Decoder<'de>,
                rows: usize,
                options: &DecodeOptions,
                mut sink: F,
            ) -> Result<()>
            where
                F: FnMut(Self) -> Result<()>,
            {
                let encoding = read_column(d)?;
                match encoding {
                    ENC_PACKED => {
                        let data = packed_bytes(d, rows, $bytes)?;
                        for chunk in data.as_chunks::<$bytes>().0 {
                            sink(<$ty>::from_le_bytes(*chunk))?;
                        }
                        Ok(())
                    }
                    ENC_FOR => decode_for_unsigned_into::<$ty, _>(d, rows, sink),
                    ENC_VARINT => decode_varint_unsigned_into::<$ty, _>(d, rows, sink),
                    ENC_RLE => decode_rle_into::<$ty, _>(d, rows, options, sink),
                    ENC_DELTA_RLE => decode_delta_rle_into::<$ty, _>(d, rows, options, sink),
                    ENC_DELTA_OF_DELTA => decode_delta_of_delta_into::<$ty, _>(d, rows, options, sink),
                    ENC_DICTIONARY => decode_dictionary_into::<$ty, _>(d, rows, options, sink),
                    other => Err(Error::InvalidEncoding(other)),
                }
            }
        }
    };
}

impl_unsigned_int!(u8, 1, 8);
impl_unsigned_int!(u16, 2, 16);
impl_unsigned_int!(u32, 4, 32);
impl_unsigned_int!(u64, 8, 64);

macro_rules! impl_signed_int {
    ($ty:ty, $bytes:expr, $bits:expr) => {
        impl Field for $ty {
            fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
            where
                I: Iterator<Item = &'a Self> + Clone,
                Self: 'a,
            {
                match hint {
                    Encoding::Auto => encode_signed_auto(
                        values,
                        out,
                        $bytes,
                        $bits,
                        |value| *value as u64,
                        |value| i64::from(*value),
                        |value| i128::from(*value),
                    ),
                    Encoding::ForBitpack => {
                        encode_signed_for(values, out, |value| i64::from(*value), |value| i128::from(*value))
                    }
                    Encoding::Packed => encode_packed_u64(values.map(|value| *value as u64), $bytes, out),
                    Encoding::Varint => encode_varint_signed(values.map(|value| i128::from(*value)), out),
                    Encoding::Rle => encode_rle(values, out, |a, b| a == b),
                    Encoding::DeltaRle => encode_delta_rle(values.map(|value| i128::from(*value)), out),
                    Encoding::DeltaOfDelta => encode_delta_of_delta(values.map(|value| i128::from(*value)), out),
                    Encoding::Dictionary => encode_dictionary(values, out),
                    other => Err(Error::UnsupportedEncoding(other)),
                }
            }
        }

        impl<'de> FieldDecode<'de> for $ty {
            fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
                let position = d.position();
                let encoding = read_column(d)?;
                if encoding == ENC_PACKED {
                    let data = packed_bytes(d, rows, $bytes)?;
                    options.check_output_bytes::<Self>(rows)?;
                    return Ok(data
                        .as_chunks::<$bytes>()
                        .0
                        .iter()
                        .map(|chunk| <$ty>::from_le_bytes(*chunk))
                        .collect());
                }
                d.set_position(position);
                let mut out = Vec::new();
                Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
                Ok(out)
            }

            fn decode_column_into<F>(
                d: &mut Decoder<'de>,
                rows: usize,
                options: &DecodeOptions,
                mut sink: F,
            ) -> Result<()>
            where
                F: FnMut(Self) -> Result<()>,
            {
                let encoding = read_column(d)?;
                match encoding {
                    ENC_PACKED => {
                        let data = packed_bytes(d, rows, $bytes)?;
                        for chunk in data.as_chunks::<$bytes>().0 {
                            sink(<$ty>::from_le_bytes(*chunk))?;
                        }
                        Ok(())
                    }
                    ENC_FOR => decode_for_signed_into::<$ty, _>(d, rows, sink),
                    ENC_VARINT => decode_varint_signed_into::<$ty, _>(d, rows, sink),
                    ENC_RLE => decode_rle_into::<$ty, _>(d, rows, options, sink),
                    ENC_DELTA_RLE => decode_delta_rle_into::<$ty, _>(d, rows, options, sink),
                    ENC_DELTA_OF_DELTA => decode_delta_of_delta_into::<$ty, _>(d, rows, options, sink),
                    ENC_DICTIONARY => decode_dictionary_into::<$ty, _>(d, rows, options, sink),
                    other => Err(Error::InvalidEncoding(other)),
                }
            }
        }
    };
}

impl_signed_int!(i8, 1, 8);
impl_signed_int!(i16, 2, 16);
impl_signed_int!(i32, 4, 32);
impl_signed_int!(i64, 8, 64);

// usize/isize are encoded as u64/i64 so the format is portable.
impl Field for usize {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto => encode_unsigned_auto(values, out, 8, 64, |value| *value as u64, |value| *value as i128),
            Encoding::ForBitpack => encode_unsigned_for(values, out, |value| *value as u64, |value| *value as i128),
            Encoding::Packed => encode_packed_u64(values.map(|value| *value as u64), 8, out),
            Encoding::Varint => encode_varint_unsigned(values.map(|value| u128::from(*value as u64)), out),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            Encoding::DeltaRle => encode_delta_rle(values.map(|value| i128::from(*value as u64)), out),
            Encoding::DeltaOfDelta => encode_delta_of_delta(values.map(|value| i128::from(*value as u64)), out),
            Encoding::Dictionary => encode_dictionary(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for usize {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::new();
        Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        u64::decode_column_into(d, rows, options, |value| {
            sink(usize::try_from(value).map_err(|_| Error::LengthOverflow)?)
        })
    }
}

impl Field for isize {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto => encode_signed_auto(
                values,
                out,
                8,
                64,
                |value| *value as u64,
                |value| *value as i64,
                |value| i128::from(*value as i64),
            ),
            Encoding::ForBitpack => {
                encode_signed_for(values, out, |value| *value as i64, |value| i128::from(*value as i64))
            }
            Encoding::Packed => encode_packed_u64(values.map(|value| *value as u64), 8, out),
            Encoding::Varint => encode_varint_signed(values.map(|value| i128::from(*value as i64)), out),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            Encoding::DeltaRle => encode_delta_rle(values.map(|value| i128::from(*value as i64)), out),
            Encoding::DeltaOfDelta => encode_delta_of_delta(values.map(|value| i128::from(*value as i64)), out),
            Encoding::Dictionary => encode_dictionary(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for isize {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::new();
        Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        i64::decode_column_into(d, rows, options, |value| {
            sink(isize::try_from(value).map_err(|_| Error::LengthOverflow)?)
        })
    }
}

fn wide_runs(values: impl Iterator<Item = u128>) -> RunStats {
    let mut stats = RunStats::new();
    let mut first = true;
    let mut last = 0u128;
    for value in values {
        stats.observe(!first && value == last);
        first = false;
        last = value;
    }
    stats
}

impl Field for u128 {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto => {
                let stats = wide_runs(values.clone().copied());
                let packed_bytes = stats.rows.saturating_mul(16);
                let rle_bytes = stats.rle_bytes(16);
                if stats.rows >= 32 && rle_bytes.saturating_mul(100) < packed_bytes.saturating_mul(95) {
                    encode_rle(values, out, |a, b| a == b)
                } else {
                    encode_packed_u128(values.copied(), out)
                }
            }
            Encoding::Packed => encode_packed_u128(values.copied(), out),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            Encoding::Dictionary => encode_dictionary(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for u128 {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let position = d.position();
        let encoding = read_column(d)?;
        if encoding == ENC_PACKED {
            let data = packed_bytes(d, rows, 16)?;
            options.check_output_bytes::<Self>(rows)?;
            return Ok(data
                .as_chunks::<16>()
                .0
                .iter()
                .map(|chunk| u128::from_le_bytes(*chunk))
                .collect());
        }
        d.set_position(position);
        let mut out = Vec::new();
        Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_PACKED => {
                let data = packed_bytes(d, rows, 16)?;
                for chunk in data.as_chunks::<16>().0 {
                    sink(u128::from_le_bytes(*chunk))?;
                }
                Ok(())
            }
            ENC_RLE => decode_rle_into::<u128, _>(d, rows, options, sink),
            ENC_DICTIONARY => decode_dictionary_into::<u128, _>(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

impl Field for i128 {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto => {
                let stats = wide_runs(values.clone().map(|value| *value as u128));
                let packed_bytes = stats.rows.saturating_mul(16);
                let rle_bytes = stats.rle_bytes(16);
                if stats.rows >= 32 && rle_bytes.saturating_mul(100) < packed_bytes.saturating_mul(95) {
                    encode_rle(values, out, |a, b| a == b)
                } else {
                    encode_packed_u128(values.map(|value| *value as u128), out)
                }
            }
            Encoding::Packed => encode_packed_u128(values.map(|value| *value as u128), out),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            Encoding::Dictionary => encode_dictionary(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for i128 {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let position = d.position();
        let encoding = read_column(d)?;
        if encoding == ENC_PACKED {
            let data = packed_bytes(d, rows, 16)?;
            options.check_output_bytes::<Self>(rows)?;
            return Ok(data
                .as_chunks::<16>()
                .0
                .iter()
                .map(|chunk| i128::from_le_bytes(*chunk))
                .collect());
        }
        d.set_position(position);
        let mut out = Vec::new();
        Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_PACKED => {
                let data = packed_bytes(d, rows, 16)?;
                for chunk in data.as_chunks::<16>().0 {
                    sink(i128::from_le_bytes(*chunk))?;
                }
                Ok(())
            }
            ENC_RLE => decode_rle_into::<i128, _>(d, rows, options, sink),
            ENC_DICTIONARY => decode_dictionary_into::<i128, _>(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// bool, floats, char
// ---------------------------------------------------------------------------

impl Field for bool {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        let mut bitpack = |values: I| -> Result<()> {
            let rows = exact_len(&values);
            let bytes = packed_len(rows, 1)?;
            write_column(out, ENC_BITPACK)?;
            out.bytes_len(bytes as u64)?;
            let mut writer = BitWriter::new(out.writer_mut());
            for value in values {
                writer.write(u64::from(*value), 1);
            }
            writer.finish();
            Ok(())
        };
        match hint {
            Encoding::Auto => {
                let stats = {
                    let mut stats = RunStats::new();
                    let mut first = true;
                    let mut last = false;
                    for value in values.clone() {
                        stats.observe(!first && *value == last);
                        first = false;
                        last = *value;
                    }
                    stats
                };
                let packed_bytes = packed_len(stats.rows, 1)?;
                let run_value_bytes = stats.runs.div_ceil(8);
                let rle_bytes = run_value_bytes
                    .saturating_add(stats.runs.saturating_mul(varint_len(stats.max_run_len as u128)))
                    .saturating_add(16);
                if stats.rows >= 32 && rle_bytes.saturating_mul(100) < packed_bytes.saturating_mul(95) {
                    encode_rle(values, out, |a, b| a == b)
                } else {
                    bitpack(values)
                }
            }
            Encoding::Bitpack => bitpack(values),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for bool {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::new();
        Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_BITPACK => {
                let data = d.bytes()?;
                let expected = packed_len(rows, 1)?;
                if data.len() != expected {
                    return Err(Error::LengthMismatch {
                        expected,
                        actual: data.len(),
                    });
                }
                let mut buffer = [0u64; 64];
                let mut bit_pos = 0usize;
                let mut remaining = rows;
                while remaining > 0 {
                    let take = remaining.min(buffer.len());
                    unpack_into(data, &mut bit_pos, 1, &mut buffer[..take])?;
                    for &value in &buffer[..take] {
                        if value > 1 {
                            return Err(Error::InvalidBitpackedValue);
                        }
                        sink(value == 1)?;
                    }
                    remaining -= take;
                }
                Ok(())
            }
            ENC_RLE => decode_rle_into::<bool, _>(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

macro_rules! impl_float {
    ($ty:ty, $bits:ty, $width:expr) => {
        impl Field for $ty {
            fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
            where
                I: Iterator<Item = &'a Self> + Clone,
                Self: 'a,
            {
                let mut packed = |values: I| -> Result<()> {
                    let rows = exact_len(&values);
                    let bytes = rows.checked_mul($width).ok_or(Error::LengthOverflow)?;
                    write_column(out, ENC_PACKED)?;
                    out.bytes_len(bytes as u64)?;
                    let buf = out.writer_mut();
                    buf.reserve(bytes);
                    for value in values {
                        buf.extend_from_slice(&value.to_bits().to_le_bytes());
                    }
                    Ok(())
                };
                match hint {
                    Encoding::Auto => {
                        let stats = {
                            let mut stats = RunStats::new();
                            let mut first = true;
                            let mut last = 0u64;
                            for value in values.clone() {
                                let bits = u64::from(value.to_bits());
                                stats.observe(!first && bits == last);
                                first = false;
                                last = bits;
                            }
                            stats
                        };
                        let packed_bytes = stats.rows.saturating_mul($width);
                        let rle_bytes = stats.rle_bytes($width);
                        if stats.rows >= 32 && rle_bytes.saturating_mul(100) < packed_bytes.saturating_mul(95) {
                            encode_rle(values, out, |a, b| a.to_bits() == b.to_bits())
                        } else {
                            packed(values)
                        }
                    }
                    Encoding::Packed => packed(values),
                    Encoding::Rle => encode_rle(values, out, |a, b| a.to_bits() == b.to_bits()),
                    other => Err(Error::UnsupportedEncoding(other)),
                }
            }
        }

        impl<'de> FieldDecode<'de> for $ty {
            fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
                let position = d.position();
                let encoding = read_column(d)?;
                if encoding == ENC_PACKED {
                    let data = packed_bytes(d, rows, $width)?;
                    options.check_output_bytes::<Self>(rows)?;
                    return Ok(data
                        .as_chunks::<$width>()
                        .0
                        .iter()
                        .map(|chunk| <$ty>::from_bits(<$bits>::from_le_bytes(*chunk)))
                        .collect());
                }
                d.set_position(position);
                let mut out = Vec::new();
                Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
                Ok(out)
            }

            fn decode_column_into<F>(
                d: &mut Decoder<'de>,
                rows: usize,
                options: &DecodeOptions,
                mut sink: F,
            ) -> Result<()>
            where
                F: FnMut(Self) -> Result<()>,
            {
                let encoding = read_column(d)?;
                match encoding {
                    ENC_PACKED => {
                        let data = packed_bytes(d, rows, $width)?;
                        for chunk in data.as_chunks::<$width>().0 {
                            let bits = <$bits>::from_le_bytes(*chunk);
                            sink(<$ty>::from_bits(bits))?;
                        }
                        Ok(())
                    }
                    ENC_RLE => decode_rle_into::<$ty, _>(d, rows, options, sink),
                    other => Err(Error::InvalidEncoding(other)),
                }
            }
        }
    };
}

impl_float!(f32, u32, 4);
impl_float!(f64, u64, 8);

impl Field for char {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto => encode_unsigned_auto(
                values,
                out,
                4,
                32,
                |value| u64::from(u32::from(*value)),
                |value| i128::from(u32::from(*value)),
            ),
            Encoding::ForBitpack => encode_unsigned_for(
                values,
                out,
                |value| u64::from(u32::from(*value)),
                |value| i128::from(u32::from(*value)),
            ),
            Encoding::Packed => encode_packed_u64(values.map(|value| u64::from(u32::from(*value))), 4, out),
            Encoding::Varint => encode_varint_unsigned(values.map(|value| u128::from(u32::from(*value))), out),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            Encoding::DeltaRle => encode_delta_rle(values.map(|value| i128::from(u32::from(*value))), out),
            Encoding::DeltaOfDelta => encode_delta_of_delta(values.map(|value| i128::from(u32::from(*value))), out),
            Encoding::Dictionary => encode_dictionary(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for char {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::new();
        Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        u32::decode_column_into(d, rows, options, |value| {
            sink(char::from_u32(value).ok_or(Error::InvalidChar(value))?)
        })
    }
}

// ---------------------------------------------------------------------------
// Strings
// ---------------------------------------------------------------------------

#[derive(Default)]
struct StringStats {
    rows: usize,
    total: usize,
    length_bytes: usize,
    runs: usize,
    run_len: usize,
    max_run_len: usize,
    run_value_bytes: usize,
}

fn string_stats<'a, I>(values: I) -> Result<StringStats>
where
    I: Iterator<Item = &'a str> + Clone,
{
    let mut stats = StringStats::default();
    let mut last: Option<&str> = None;
    for value in values {
        stats.rows += 1;
        stats.total = stats.total.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
        let varint = varint_len(value.len() as u128);
        stats.length_bytes += varint;
        if last == Some(value) {
            stats.run_len += 1;
        } else {
            stats.runs += 1;
            stats.run_len = 1;
            stats.run_value_bytes += value.len() + varint;
        }
        stats.max_run_len = stats.max_run_len.max(stats.run_len);
        last = Some(value);
    }
    Ok(stats)
}

fn encode_strings<'a, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = &'a str> + Clone,
{
    let stats = string_stats(values.clone())?;
    write_column(out, ENC_STRING_LENGTHS)?;
    out.array(2)?;
    out.bytes_len(stats.length_bytes as u64)?;
    {
        let buf = out.writer_mut();
        buf.reserve(stats.length_bytes);
        for value in values.clone() {
            write_varint(buf, value.len() as u128);
        }
    }
    out.bytes_len(stats.total as u64)?;
    let buf = out.writer_mut();
    buf.reserve(stats.total);
    for value in values {
        buf.extend_from_slice(value.as_bytes());
    }
    Ok(())
}

fn encode_strings_auto<'a, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = &'a str> + Clone,
{
    let stats = string_stats(values.clone())?;
    let lengths_bytes = stats.total.saturating_add(stats.length_bytes).saturating_add(16);
    let rle_bytes = stats
        .run_value_bytes
        .saturating_add(stats.runs.saturating_mul(varint_len(stats.max_run_len as u128)))
        .saturating_add(16);
    if stats.rows >= 32 && rle_bytes.saturating_mul(100) < lengths_bytes.saturating_mul(95) {
        encode_str_rle(values, out)
    } else {
        encode_strings(values, out)
    }
}

fn encode_str_rle<'a, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = &'a str> + Clone,
{
    let (lower, _) = values.size_hint();
    let mut run_values: Vec<&'a str> = Vec::with_capacity(lower.min(4096));
    let mut lengths: Vec<u32> = Vec::with_capacity(lower.min(4096));
    for value in values {
        if run_values.last() == Some(&value) {
            let last_len = lengths.last_mut().expect("run length exists");
            *last_len = last_len.checked_add(1).ok_or(Error::LengthOverflow)?;
            continue;
        }
        run_values.push(value);
        lengths.push(1);
    }
    let length_bytes: usize = lengths.iter().map(|length| varint_len(u128::from(*length))).sum();
    write_column(out, ENC_RLE)?;
    out.array(2)?;
    out.bytes_len(length_bytes as u64)?;
    {
        let buf = out.writer_mut();
        buf.reserve(length_bytes);
        for length in &lengths {
            write_varint(buf, u128::from(*length));
        }
    }
    encode_strings(run_values.iter().copied(), out)
}

fn decode_strings_into<'de, F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
where
    F: FnMut(String) -> Result<()>,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("string payload"));
    }
    let mut pos = 0;
    let mut data_pos = 0usize;
    for _ in 0..rows {
        let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        if len > options.max_string_len {
            return Err(Error::LimitExceeded("string length"));
        }
        let end = data_pos.checked_add(len).ok_or(Error::LengthOverflow)?;
        let slice = data.get(data_pos..end).ok_or(Error::LengthMismatch {
            expected: end,
            actual: data.len(),
        })?;
        sink(validated_str(slice)?.to_string())?;
        data_pos = end;
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    if data_pos != data.len() {
        return Err(Error::LengthMismatch {
            expected: data_pos,
            actual: data.len(),
        });
    }
    Ok(())
}

fn encode_string_offsets<'a, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = &'a str> + Clone,
{
    let mut rows = 0usize;
    let mut total = 0usize;
    for value in values.clone() {
        rows += 1;
        total = total.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
    }
    let width = if total <= u32::MAX as usize { 4usize } else { 8usize };
    let offset_bytes = rows
        .checked_add(1)
        .and_then(|count| count.checked_mul(width))
        .ok_or(Error::LengthOverflow)?;
    write_column(out, ENC_STRING_OFFSETS)?;
    out.array(3)?;
    out.u64(width as u64)?;
    out.bytes_len(offset_bytes as u64)?;
    {
        let buf = out.writer_mut();
        buf.reserve(offset_bytes);
        let mut offset = 0usize;
        if width == 4 {
            buf.extend_from_slice(&0u32.to_le_bytes());
        } else {
            buf.extend_from_slice(&0u64.to_le_bytes());
        }
        for value in values.clone() {
            offset = offset.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
            if width == 4 {
                buf.extend_from_slice(&(offset as u32).to_le_bytes());
            } else {
                buf.extend_from_slice(&(offset as u64).to_le_bytes());
            }
        }
    }
    out.bytes_len(total as u64)?;
    let buf = out.writer_mut();
    buf.reserve(total);
    for value in values {
        buf.extend_from_slice(value.as_bytes());
    }
    Ok(())
}

pub(crate) fn offset_at(data: &[u8], width: usize, index: usize) -> Result<u64> {
    if width != 4 && width != 8 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let start = index.checked_mul(width).ok_or(Error::LengthOverflow)?;
    let end = start.checked_add(width).ok_or(Error::LengthOverflow)?;
    let slice = data.get(start..end).ok_or(Error::LengthMismatch {
        expected: end,
        actual: data.len(),
    })?;
    Ok(if width == 4 {
        u64::from(u32::from_le_bytes(slice.try_into().unwrap()))
    } else {
        u64::from_le_bytes(slice.try_into().unwrap())
    })
}

fn decode_string_offsets_into<'de, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    F: FnMut(String) -> Result<()>,
{
    read_array_len(d, 3)?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    let offsets_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("string payload"));
    }
    if width != 4 && width != 8 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let expected = rows
        .checked_add(1)
        .and_then(|count| count.checked_mul(width))
        .ok_or(Error::LengthOverflow)?;
    if offsets_data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: offsets_data.len(),
        });
    }
    let mut start = offset_at(offsets_data, width, 0)?;
    if start != 0 {
        return Err(Error::InvalidOffset);
    }
    for index in 0..rows {
        let end = offset_at(offsets_data, width, index + 1)?;
        if end < start || end > data.len() as u64 {
            return Err(Error::InvalidOffset);
        }
        let start_usize = start as usize;
        let end_usize = end as usize;
        if end_usize - start_usize > options.max_string_len {
            return Err(Error::LimitExceeded("string length"));
        }
        sink(validated_str(&data[start_usize..end_usize])?.to_string())?;
        start = end;
    }
    if start != data.len() as u64 {
        return Err(Error::InvalidOffset);
    }
    Ok(())
}

impl Field for String {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto => encode_strings_auto(values.map(|value| value.as_str()), out),
            Encoding::StringLengths => encode_strings(values.map(|value| value.as_str()), out),
            Encoding::StringOffsets => encode_string_offsets(values.map(|value| value.as_str()), out),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            Encoding::Dictionary => encode_dictionary(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for String {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::new();
        Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_STRING_LENGTHS => decode_strings_into(d, rows, options, sink),
            ENC_STRING_OFFSETS => decode_string_offsets_into(d, rows, options, sink),
            ENC_RLE => decode_rle_into::<String, _>(d, rows, options, sink),
            ENC_DICTIONARY => decode_dictionary_into::<String, _>(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// Cow<[u8]>
// ---------------------------------------------------------------------------

fn encode_bytes<'a, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = &'a [u8]> + Clone,
{
    let mut stats = StringStats::default();
    let mut last: Option<&[u8]> = None;
    for value in values.clone() {
        stats.rows += 1;
        stats.total = stats.total.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
        let varint = varint_len(value.len() as u128);
        stats.length_bytes += varint;
        if last == Some(value) {
            stats.run_len += 1;
        } else {
            stats.runs += 1;
            stats.run_len = 1;
            stats.run_value_bytes += value.len() + varint;
        }
        stats.max_run_len = stats.max_run_len.max(stats.run_len);
        last = Some(value);
    }
    write_column(out, ENC_STRING_LENGTHS)?;
    out.array(2)?;
    out.bytes_len(stats.length_bytes as u64)?;
    {
        let buf = out.writer_mut();
        buf.reserve(stats.length_bytes);
        for value in values.clone() {
            write_varint(buf, value.len() as u128);
        }
    }
    out.bytes_len(stats.total as u64)?;
    let buf = out.writer_mut();
    buf.reserve(stats.total);
    for value in values {
        buf.extend_from_slice(value);
    }
    Ok(())
}

fn encode_bytes_auto<'a, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = &'a [u8]> + Clone,
{
    let mut stats = StringStats::default();
    let mut last: Option<&[u8]> = None;
    for value in values.clone() {
        stats.rows += 1;
        stats.total = stats.total.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
        let varint = varint_len(value.len() as u128);
        stats.length_bytes += varint;
        if last == Some(value) {
            stats.run_len += 1;
        } else {
            stats.runs += 1;
            stats.run_len = 1;
            stats.run_value_bytes += value.len() + varint;
        }
        stats.max_run_len = stats.max_run_len.max(stats.run_len);
        last = Some(value);
    }
    let lengths_bytes = stats.total.saturating_add(stats.length_bytes).saturating_add(16);
    let rle_bytes = stats
        .run_value_bytes
        .saturating_add(stats.runs.saturating_mul(varint_len(stats.max_run_len as u128)))
        .saturating_add(16);
    if stats.rows >= 32 && rle_bytes.saturating_mul(100) < lengths_bytes.saturating_mul(95) {
        encode_bytes_rle(values, out)
    } else {
        encode_bytes(values, out)
    }
}

fn encode_bytes_rle<'a, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = &'a [u8]> + Clone,
{
    let (lower, _) = values.size_hint();
    let mut run_values: Vec<&'a [u8]> = Vec::with_capacity(lower.min(4096));
    let mut lengths: Vec<u32> = Vec::with_capacity(lower.min(4096));
    for value in values {
        if run_values.last() == Some(&value) {
            let last_len = lengths.last_mut().expect("run length exists");
            *last_len = last_len.checked_add(1).ok_or(Error::LengthOverflow)?;
            continue;
        }
        run_values.push(value);
        lengths.push(1);
    }
    let length_bytes: usize = lengths.iter().map(|length| varint_len(u128::from(*length))).sum();
    write_column(out, ENC_RLE)?;
    out.array(2)?;
    out.bytes_len(length_bytes as u64)?;
    {
        let buf = out.writer_mut();
        buf.reserve(length_bytes);
        for length in &lengths {
            write_varint(buf, u128::from(*length));
        }
    }
    encode_bytes(run_values.iter().copied(), out)
}

fn decode_bytes_into<'de, F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
where
    F: FnMut(Cow<'de, [u8]>) -> Result<()>,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("byte payload"));
    }
    let mut pos = 0;
    let mut data_pos = 0usize;
    for _ in 0..rows {
        let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        if len > options.max_string_len {
            return Err(Error::LimitExceeded("byte length"));
        }
        let end = data_pos.checked_add(len).ok_or(Error::LengthOverflow)?;
        let slice = data.get(data_pos..end).ok_or(Error::LengthMismatch {
            expected: end,
            actual: data.len(),
        })?;
        sink(Cow::Borrowed(slice))?;
        data_pos = end;
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    if data_pos != data.len() {
        return Err(Error::LengthMismatch {
            expected: data_pos,
            actual: data.len(),
        });
    }
    Ok(())
}

fn encode_byte_offsets<'a, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = &'a [u8]> + Clone,
{
    let mut rows = 0usize;
    let mut total = 0usize;
    for value in values.clone() {
        rows += 1;
        total = total.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
    }
    let width = if total <= u32::MAX as usize { 4usize } else { 8usize };
    let offset_bytes = rows
        .checked_add(1)
        .and_then(|count| count.checked_mul(width))
        .ok_or(Error::LengthOverflow)?;
    write_column(out, ENC_STRING_OFFSETS)?;
    out.array(3)?;
    out.u64(width as u64)?;
    out.bytes_len(offset_bytes as u64)?;
    {
        let buf = out.writer_mut();
        buf.reserve(offset_bytes);
        let mut offset = 0usize;
        if width == 4 {
            buf.extend_from_slice(&0u32.to_le_bytes());
        } else {
            buf.extend_from_slice(&0u64.to_le_bytes());
        }
        for value in values.clone() {
            offset = offset.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
            if width == 4 {
                buf.extend_from_slice(&(offset as u32).to_le_bytes());
            } else {
                buf.extend_from_slice(&(offset as u64).to_le_bytes());
            }
        }
    }
    out.bytes_len(total as u64)?;
    let buf = out.writer_mut();
    buf.reserve(total);
    for value in values {
        buf.extend_from_slice(value);
    }
    Ok(())
}

fn decode_byte_offsets_into<'de, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    F: FnMut(Cow<'de, [u8]>) -> Result<()>,
{
    read_array_len(d, 3)?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    let offsets_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("byte payload"));
    }
    if width != 4 && width != 8 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let expected = rows
        .checked_add(1)
        .and_then(|count| count.checked_mul(width))
        .ok_or(Error::LengthOverflow)?;
    if offsets_data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: offsets_data.len(),
        });
    }
    let mut start = offset_at(offsets_data, width, 0)?;
    if start != 0 {
        return Err(Error::InvalidOffset);
    }
    for index in 0..rows {
        let end = offset_at(offsets_data, width, index + 1)?;
        if end < start || end > data.len() as u64 {
            return Err(Error::InvalidOffset);
        }
        let start_usize = start as usize;
        let end_usize = end as usize;
        if end_usize - start_usize > options.max_string_len {
            return Err(Error::LimitExceeded("byte length"));
        }
        sink(Cow::Borrowed(&data[start_usize..end_usize]))?;
        start = end;
    }
    if start != data.len() as u64 {
        return Err(Error::InvalidOffset);
    }
    Ok(())
}

impl<'a> Field for Cow<'a, [u8]> {
    fn encode_column<'b, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'b Self> + Clone,
        Self: 'b,
    {
        match hint {
            Encoding::Auto => encode_bytes_auto(values.map(|value| value.as_ref()), out),
            Encoding::StringLengths => encode_bytes(values.map(|value| value.as_ref()), out),
            Encoding::StringOffsets => encode_byte_offsets(values.map(|value| value.as_ref()), out),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            Encoding::Dictionary => encode_dictionary(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for Cow<'de, [u8]> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::new();
        Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_STRING_LENGTHS => decode_bytes_into(d, rows, options, sink),
            ENC_STRING_OFFSETS => decode_byte_offsets_into(d, rows, options, sink),
            ENC_RLE => decode_rle_into::<Cow<'de, [u8]>, _>(d, rows, options, sink),
            ENC_DICTIONARY => decode_dictionary_into::<Cow<'de, [u8]>, _>(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// Cow<str>
// ---------------------------------------------------------------------------

fn decode_str_lengths_into<'de, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    F: FnMut(Cow<'de, str>) -> Result<()>,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("string payload"));
    }
    let mut pos = 0;
    let mut data_pos = 0usize;
    for _ in 0..rows {
        let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        if len > options.max_string_len {
            return Err(Error::LimitExceeded("string length"));
        }
        let end = data_pos.checked_add(len).ok_or(Error::LengthOverflow)?;
        let slice = data.get(data_pos..end).ok_or(Error::LengthMismatch {
            expected: end,
            actual: data.len(),
        })?;
        sink(Cow::Borrowed(validated_str(slice)?))?;
        data_pos = end;
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    if data_pos != data.len() {
        return Err(Error::LengthMismatch {
            expected: data_pos,
            actual: data.len(),
        });
    }
    Ok(())
}

fn decode_str_offsets_into<'de, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    F: FnMut(Cow<'de, str>) -> Result<()>,
{
    read_array_len(d, 3)?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    let offsets_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("string payload"));
    }
    if width != 4 && width != 8 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let expected = rows
        .checked_add(1)
        .and_then(|count| count.checked_mul(width))
        .ok_or(Error::LengthOverflow)?;
    if offsets_data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: offsets_data.len(),
        });
    }
    let mut start = offset_at(offsets_data, width, 0)?;
    if start != 0 {
        return Err(Error::InvalidOffset);
    }
    for index in 0..rows {
        let end = offset_at(offsets_data, width, index + 1)?;
        if end < start || end > data.len() as u64 {
            return Err(Error::InvalidOffset);
        }
        let start_usize = start as usize;
        let end_usize = end as usize;
        if end_usize - start_usize > options.max_string_len {
            return Err(Error::LimitExceeded("string length"));
        }
        sink(Cow::Borrowed(validated_str(&data[start_usize..end_usize])?))?;
        start = end;
    }
    if start != data.len() as u64 {
        return Err(Error::InvalidOffset);
    }
    Ok(())
}

impl<'a> Field for Cow<'a, str> {
    fn encode_column<'b, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'b Self> + Clone,
        Self: 'b,
    {
        match hint {
            Encoding::Auto => encode_strings_auto(values.map(|value| value.as_ref()), out),
            Encoding::StringLengths => encode_strings(values.map(|value| value.as_ref()), out),
            Encoding::StringOffsets => encode_string_offsets(values.map(|value| value.as_ref()), out),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            Encoding::Dictionary => encode_dictionary(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for Cow<'de, str> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::new();
        Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_STRING_LENGTHS => decode_str_lengths_into(d, rows, options, sink),
            ENC_STRING_OFFSETS => decode_str_offsets_into(d, rows, options, sink),
            ENC_RLE => decode_rle_into::<Cow<'de, str>, _>(d, rows, options, sink),
            ENC_DICTIONARY => decode_dictionary_into::<Cow<'de, str>, _>(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// Option
// ---------------------------------------------------------------------------

impl<T: Field> Field for Option<T> {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        let inner_hint = match hint {
            Encoding::Auto | Encoding::Nullable => Encoding::Auto,
            other => other,
        };
        let mut present = 0usize;
        for value in values.clone() {
            if value.is_some() {
                present += 1;
            }
        }
        if present == 0 {
            write_column(out, ENC_ALL_NULL)?;
            out.array(0)?;
            return Ok(());
        }
        write_column(out, ENC_NULLABLE)?;
        out.array(2)?;
        let rows = exact_len(&values);
        let validity_bytes = rows.div_ceil(8);
        out.bytes_len(validity_bytes as u64)?;
        {
            let mut writer = BitWriter::new(out.writer_mut());
            for value in values.clone() {
                writer.write(u64::from(value.is_some()), 1);
            }
            writer.finish();
        }
        T::encode_column(values.filter_map(|value| value.as_ref()), out, inner_hint)
    }
}

impl<'de, T: FieldDecode<'de>> FieldDecode<'de> for Option<T> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        options.check_output_bytes::<Self>(rows)?;
        let mut out = Vec::new();
        Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_ALL_NULL => {
                read_array_len(d, 0)?;
                for _ in 0..rows {
                    sink(None)?;
                }
                Ok(())
            }
            ENC_NULLABLE => {
                read_array_len(d, 2)?;
                let validity = d.bytes()?;
                let expected = rows.div_ceil(8);
                if validity.len() != expected {
                    return Err(Error::LengthMismatch {
                        expected,
                        actual: validity.len(),
                    });
                }
                let valid = count_valid(validity, rows);
                let mut index = 0usize;
                let mut emitted = 0usize;
                T::decode_column_into(d, valid, options, |value| {
                    while index < rows && validity[index / 8] & (1 << (index % 8)) == 0 {
                        sink(None)?;
                        index += 1;
                    }
                    if index >= rows {
                        return Err(Error::LengthMismatch {
                            expected: valid,
                            actual: emitted,
                        });
                    }
                    sink(Some(value))?;
                    index += 1;
                    emitted += 1;
                    Ok(())
                })?;
                while index < rows {
                    if validity[index / 8] & (1 << (index % 8)) != 0 {
                        return Err(Error::LengthMismatch {
                            expected: valid,
                            actual: emitted,
                        });
                    }
                    sink(None)?;
                    index += 1;
                }
                if emitted != valid {
                    return Err(Error::LengthMismatch {
                        expected: valid,
                        actual: emitted,
                    });
                }
                Ok(())
            }
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// Vec
// ---------------------------------------------------------------------------

fn encode_list_lengths<'a, T, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    T: Field + 'a,
    I: Iterator<Item = &'a Vec<T>> + Clone,
{
    let mut length_bytes = 0usize;
    let mut count = 0usize;
    for value in values.clone() {
        let len = u32::try_from(value.len()).map_err(|_| Error::LengthOverflow)?;
        length_bytes += varint_len(u128::from(len));
        count = count.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
    }
    if count > u32::MAX as usize {
        return Err(Error::LengthOverflow);
    }
    write_column(out, ENC_LIST_LENGTHS)?;
    out.array(2)?;
    out.bytes_len(length_bytes as u64)?;
    {
        let buf = out.writer_mut();
        buf.reserve(length_bytes);
        for value in values.clone() {
            write_varint(buf, value.len() as u128);
        }
    }
    T::encode_column(values.flat_map(|value| value.iter()), out, Encoding::Auto)
}

fn decode_list_lengths_into<'de, T, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    T: FieldDecode<'de>,
    F: FnMut(Vec<T>) -> Result<()>,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let mut pos = 0;
    let mut total = 0usize;
    for _ in 0..rows {
        let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        total = total.checked_add(len).ok_or(Error::LengthOverflow)?;
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    if total > options.max_list_len {
        return Err(Error::LimitExceeded("list length"));
    }
    options.check_output_bytes::<T>(total)?;
    options.check_output_bytes::<Vec<T>>(rows)?;

    let mut pos = 0;
    let mut row_index = 0usize;
    let mut remaining = 0usize;
    let mut row = Vec::new();
    if rows > 0 {
        let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        remaining = len;
        row = Vec::with_capacity(options.output_capacity::<T>(len)?);
    }
    let mut produced = 0usize;
    T::decode_column_into(d, total, options, |value| {
        produced += 1;
        while remaining == 0 {
            sink(core::mem::take(&mut row))?;
            row_index += 1;
            if row_index >= rows {
                return Err(Error::LengthMismatch {
                    expected: total,
                    actual: produced,
                });
            }
            let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
            remaining = len;
            row = Vec::with_capacity(options.output_capacity::<T>(len)?);
        }
        row.push(value);
        remaining -= 1;
        Ok(())
    })?;
    if produced != total {
        return Err(Error::LengthMismatch {
            expected: total,
            actual: produced,
        });
    }
    while row_index < rows {
        if remaining != 0 {
            return Err(Error::LengthMismatch {
                expected: total,
                actual: produced,
            });
        }
        sink(core::mem::take(&mut row))?;
        row_index += 1;
        if row_index < rows {
            let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
            remaining = len;
            row = Vec::with_capacity(options.output_capacity::<T>(len)?);
        }
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    Ok(())
}

fn encode_list_offsets<'a, T, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    T: Field + 'a,
    I: Iterator<Item = &'a Vec<T>> + Clone,
{
    let mut total = 0usize;
    let mut count = 0usize;
    for value in values.clone() {
        total = total.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
        count += 1;
    }
    let width = if total <= u32::MAX as usize { 4usize } else { 8usize };
    let offset_bytes = count
        .checked_add(1)
        .and_then(|count| count.checked_mul(width))
        .ok_or(Error::LengthOverflow)?;
    write_column(out, ENC_LIST_OFFSETS)?;
    out.array(3)?;
    out.u64(width as u64)?;
    out.bytes_len(offset_bytes as u64)?;
    {
        let buf = out.writer_mut();
        buf.reserve(offset_bytes);
        if width == 4 {
            buf.extend_from_slice(&0u32.to_le_bytes());
        } else {
            buf.extend_from_slice(&0u64.to_le_bytes());
        }
        let mut offset = 0usize;
        for value in values.clone() {
            offset = offset.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
            if width == 4 {
                buf.extend_from_slice(&(offset as u32).to_le_bytes());
            } else {
                buf.extend_from_slice(&(offset as u64).to_le_bytes());
            }
        }
    }
    T::encode_column(values.flat_map(|value| value.iter()), out, Encoding::Auto)
}

fn decode_list_offsets_into<'de, T, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    T: FieldDecode<'de>,
    F: FnMut(Vec<T>) -> Result<()>,
{
    read_array_len(d, 3)?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    if width != 4 && width != 8 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let offsets_data = d.bytes()?;
    let expected = rows
        .checked_add(1)
        .and_then(|count| count.checked_mul(width))
        .ok_or(Error::LengthOverflow)?;
    if offsets_data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: offsets_data.len(),
        });
    }
    if offset_at(offsets_data, width, 0)? != 0 {
        return Err(Error::InvalidOffset);
    }
    let mut previous = 0u64;
    for index in 0..rows {
        let next = offset_at(offsets_data, width, index + 1)?;
        if next < previous {
            return Err(Error::InvalidOffset);
        }
        previous = next;
    }
    let total = usize::try_from(previous).map_err(|_| Error::LimitExceeded("list length"))?;
    if total > options.max_list_len {
        return Err(Error::LimitExceeded("list length"));
    }
    options.check_output_bytes::<T>(total)?;
    options.check_output_bytes::<Vec<T>>(rows)?;

    let mut row_index = 0usize;
    let mut start = 0usize;
    let mut end = if rows > 0 {
        usize::try_from(offset_at(offsets_data, width, 1)?).map_err(|_| Error::LimitExceeded("list length"))?
    } else {
        0
    };
    let mut remaining = end - start;
    let mut row = Vec::with_capacity(options.output_capacity::<T>(remaining)?);
    let mut produced = 0usize;
    T::decode_column_into(d, total, options, |value| {
        produced += 1;
        while remaining == 0 {
            sink(core::mem::take(&mut row))?;
            row_index += 1;
            if row_index >= rows {
                return Err(Error::LengthMismatch {
                    expected: total,
                    actual: produced,
                });
            }
            start = usize::try_from(offset_at(offsets_data, width, row_index)?)
                .map_err(|_| Error::LimitExceeded("list length"))?;
            end = usize::try_from(offset_at(offsets_data, width, row_index + 1)?)
                .map_err(|_| Error::LimitExceeded("list length"))?;
            if end < start {
                return Err(Error::InvalidOffset);
            }
            remaining = end - start;
            row = Vec::with_capacity(options.output_capacity::<T>(remaining)?);
        }
        row.push(value);
        remaining -= 1;
        Ok(())
    })?;
    if produced != total {
        return Err(Error::LengthMismatch {
            expected: total,
            actual: produced,
        });
    }
    while row_index < rows {
        if remaining != 0 {
            return Err(Error::LengthMismatch {
                expected: total,
                actual: produced,
            });
        }
        sink(core::mem::take(&mut row))?;
        row_index += 1;
        if row_index < rows {
            start = usize::try_from(offset_at(offsets_data, width, row_index)?)
                .map_err(|_| Error::LimitExceeded("list length"))?;
            end = usize::try_from(offset_at(offsets_data, width, row_index + 1)?)
                .map_err(|_| Error::LimitExceeded("list length"))?;
            if end < start {
                return Err(Error::InvalidOffset);
            }
            remaining = end - start;
            row = Vec::with_capacity(options.output_capacity::<T>(remaining)?);
        }
    }
    Ok(())
}

impl<T: Field> Field for Vec<T> {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto | Encoding::ListLengths | Encoding::StringLengths => encode_list_lengths(values, out),
            Encoding::ListOffsets | Encoding::StringOffsets => encode_list_offsets(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de, T: FieldDecode<'de>> FieldDecode<'de> for Vec<T> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::new();
        Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_LIST_LENGTHS => decode_list_lengths_into(d, rows, options, sink),
            ENC_LIST_OFFSETS => decode_list_offsets_into(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// BTreeMap
// ---------------------------------------------------------------------------

impl<K: Field + Ord, V: Field> Field for BTreeMap<K, V> {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto | Encoding::ListLengths | Encoding::StringLengths => {}
            other => return Err(Error::UnsupportedEncoding(other)),
        }
        let mut length_bytes = 0usize;
        for map in values.clone() {
            let len = u32::try_from(map.len()).map_err(|_| Error::LengthOverflow)?;
            length_bytes += varint_len(u128::from(len));
        }
        write_column(out, ENC_LIST_LENGTHS)?;
        out.array(2)?;
        out.bytes_len(length_bytes as u64)?;
        {
            let buf = out.writer_mut();
            buf.reserve(length_bytes);
            for map in values.clone() {
                write_varint(buf, map.len() as u128);
            }
        }
        write_column(out, ENC_STRUCT)?;
        out.array(2)?;
        K::encode_column(values.clone().flat_map(|map| map.keys()), out, Encoding::Auto)?;
        V::encode_column(values.flat_map(|map| map.values()), out, Encoding::Auto)?;
        Ok(())
    }
}

impl<'de, K: FieldDecode<'de> + Ord, V: FieldDecode<'de>> FieldDecode<'de> for BTreeMap<K, V> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        options.check_output_bytes::<Self>(rows)?;
        let mut out = Vec::with_capacity(decode_capacity::<Self>(rows, options)?);
        let encoding = read_column(d)?;
        if encoding != ENC_LIST_LENGTHS {
            return Err(Error::InvalidEncoding(encoding));
        }
        read_array_len(d, 2)?;
        let length_data = d.bytes()?;
        let mut pos = 0;
        let mut lengths = Vec::with_capacity(decode_capacity::<usize>(rows, options)?);
        let mut total = 0usize;
        for _ in 0..rows {
            let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
            total = total.checked_add(len).ok_or(Error::LengthOverflow)?;
            lengths.push(len);
        }
        if pos != length_data.len() {
            return Err(Error::LengthMismatch {
                expected: pos,
                actual: length_data.len(),
            });
        }
        if total > options.max_list_len {
            return Err(Error::LimitExceeded("map length"));
        }
        options.check_output_bytes::<K>(total)?;
        options.check_output_bytes::<V>(total)?;
        let child = read_column(d)?;
        if child != ENC_STRUCT {
            return Err(Error::InvalidEncoding(child));
        }
        read_array_len(d, 2)?;
        let keys = K::decode_column(d, total, options)?;
        let values = V::decode_column(d, total, options)?;
        let mut keys = keys.into_iter();
        let mut values = values.into_iter();
        for len in lengths {
            let mut map = BTreeMap::new();
            for _ in 0..len {
                let key = keys.next().ok_or(Error::LengthMismatch {
                    expected: total,
                    actual: 0,
                })?;
                let value = values.next().ok_or(Error::LengthMismatch {
                    expected: total,
                    actual: 0,
                })?;
                if map.insert(key, value).is_some() {
                    return Err(Error::DuplicateMapKey);
                }
            }
            out.push(map);
        }
        Ok(out)
    }
}

// ---------------------------------------------------------------------------
// Box and unit
// ---------------------------------------------------------------------------

impl<T: Field> Field for Box<T> {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        T::encode_column(values.map(|value| value.as_ref()), out, hint)
    }
}

impl<'de, T: FieldDecode<'de>> FieldDecode<'de> for Box<T> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::new();
        Self::decode_column_into(d, rows, options, |value| push_decoded(&mut out, value, rows, options))?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        T::decode_column_into(d, rows, options, |value| sink(Box::new(value)))
    }
}

impl Field for () {
    fn encode_column<'a, I>(_values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        if !matches!(hint, Encoding::Auto | Encoding::Struct) {
            return Err(Error::UnsupportedEncoding(hint));
        }
        write_column(out, ENC_STRUCT)?;
        out.array(0)?;
        Ok(())
    }
}

impl<'de> FieldDecode<'de> for () {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        options.check_output_bytes::<Self>(rows)?;
        let encoding = read_column(d)?;
        if encoding != ENC_STRUCT {
            return Err(Error::InvalidEncoding(encoding));
        }
        read_array_len(d, 0)?;
        Ok(core::iter::repeat_with(|| ()).take(rows).collect())
    }
}

impl<T> Field for PhantomData<T> {
    fn encode_column<'a, I>(_values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        if !matches!(hint, Encoding::Auto | Encoding::Struct) {
            return Err(Error::UnsupportedEncoding(hint));
        }
        write_column(out, ENC_STRUCT)?;
        out.array(0)?;
        Ok(())
    }
}

impl<'de, T> FieldDecode<'de> for PhantomData<T> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        options.check_output_bytes::<Self>(rows)?;
        let encoding = read_column(d)?;
        if encoding != ENC_STRUCT {
            return Err(Error::InvalidEncoding(encoding));
        }
        read_array_len(d, 0)?;
        Ok(core::iter::repeat_with(|| PhantomData).take(rows).collect())
    }
}
