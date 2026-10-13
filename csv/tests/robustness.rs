//! Robustness regression tests: panic-freedom, resource bounds, error recovery,
//! line numbers and the opt-in resource limits.
//!
//! These complement `tests.rs`, which covers the functional behaviour.
#![cfg(feature = "std")]

use std::{error::Error as _, io::Cursor};

use csv::*;

// ── Read sources used by the tests ────────────────────────────────────

/// A source that repeatedly claims more data is available without appending
/// any, which the `Read` contract forbids.
struct NoProgressSource {
    data: Cursor<Vec<u8>>,
    calls: usize,
    stall_every: usize,
}

impl Read for NoProgressSource {
    fn read(&mut self, buf: &mut Vec<u8>) -> Result<bool, ReadError> {
        self.calls += 1;
        if self.calls.is_multiple_of(self.stall_every) {
            return Ok(true);
        }
        let mut tmp = [0u8; 4];
        let n = std::io::Read::read(&mut self.data, &mut tmp).unwrap();
        if n == 0 {
            return Ok(false);
        }
        buf.extend_from_slice(&tmp[..n]);
        Ok(true)
    }
}

/// Delivers `data` in fixed-size chunks, to exercise buffer-boundary paths.
struct Chunked {
    data: Vec<u8>,
    pos: usize,
    chunk: usize,
}

impl Chunked {
    fn new(data: &[u8], chunk: usize) -> Self {
        Chunked {
            data: data.to_vec(),
            pos: 0,
            chunk,
        }
    }
}

impl std::io::Read for Chunked {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.chunk.min(buf.len()).min(self.data.len() - self.pos);
        buf[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
        self.pos += n;
        Ok(n)
    }
}

// ── Panic freedom ─────────────────────────────────────────────────────

#[test]
fn read_source_without_progress_reports_error_instead_of_panicking() {
    // Before this was fixed the reader indexed out of bounds and panicked.
    let mut reader = Reader::new(NoProgressSource {
        data: Cursor::new(b"a,b\nc,d\n".to_vec()),
        calls: 0,
        stall_every: 2,
    });
    let rows: Vec<Row> = reader.rows().collect();
    assert!(
        rows.iter()
            .any(|r| matches!(r.error().map(|e| e.kind()), Some(ReadErrorKind::NoProgress))),
        "expected a NoProgress error, got {rows:?}"
    );
}

#[test]
fn read_source_without_progress_in_quoted_field_does_not_panic() {
    let mut reader = Reader::new(NoProgressSource {
        data: Cursor::new(b"a,\"b\"\"c\",d\n".to_vec()),
        calls: 0,
        stall_every: 3,
    });
    let rows: Vec<Row> = reader.rows().collect();
    assert!(
        rows.iter()
            .any(|r| matches!(r.error().map(|e| e.kind()), Some(ReadErrorKind::NoProgress))),
        "expected a NoProgress error, got {rows:?}"
    );
}

/// A deterministic xorshift PRNG, so the property test needs no dependencies.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

#[test]
fn parser_never_panics_on_arbitrary_bytes() {
    // A small alphabet biased towards the bytes that drive the state machine,
    // including the three bytes of the UTF-8 BOM so a leading mark can form.
    const ALPHABET: &[u8] = b"ab,\"\r\n\x00\xff\xef\xbb\xbf";

    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for case in 0..2000 {
        let len = rng.below(64);
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(ALPHABET[rng.below(ALPHABET.len())]);
        }

        // Every chunk size from 1 to 5 exercises the cross-buffer state.
        for chunk in 1..=5 {
            let mut reader = Reader::new(Chunked::new(&data, chunk));
            let mut rows = 0usize;
            for row in reader.rows_bytes() {
                // Touch every accessor: none of them may panic.
                let _ = row.error();
                let _ = row.len();
                let _ = row.is_empty();
                let _ = row.to_vec();
                for field in row.iter() {
                    let _ = field.len();
                }
                rows += 1;
                assert!(rows <= data.len() + 1, "iteration did not terminate for {data:?}");
            }
        }

        // Same input, with every opt-in limit enabled.
        for chunk in 1..=3 {
            let mut reader = Reader::new(Chunked::new(&data, chunk))
                .set_flexible(true)
                .set_max_record_bytes(32)
                .set_max_field_bytes(8)
                .set_max_columns(4);
            let mut rows = 0usize;
            for row in reader.rows_bytes() {
                let _ = row.error();
                let _ = row.to_vec();
                rows += 1;
                assert!(rows <= data.len() + 1, "iteration did not terminate for {data:?}");
            }
        }

        assert!(case < 2000);
    }
}

// ── Resource bounds ───────────────────────────────────────────────────

// ── Error recovery ────────────────────────────────────────────────────

#[test]
fn malformed_record_does_not_create_a_phantom_row() {
    // `"abc"XX,rest` is malformed. The junk must be skipped rather than being
    // re-parsed as a record of its own.
    let data = b"\"abc\"XX,rest\nnext,row\nmore,row\n";
    let mut reader = Reader::new(Cursor::new(&data[..])).set_flexible(true);

    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|row| row.iter().map(|f| f.unwrap_or("<err>").to_string()).collect())
        .collect();

    assert_eq!(rows.len(), 3);
    assert!(rows[0][0].starts_with("<err>"));
    assert_eq!(rows[1], vec!["next", "row"]);
    assert_eq!(rows[2], vec!["more", "row"]);
}

#[test]
fn malformed_record_does_not_poison_the_field_count() {
    // Regression: a malformed first record used to establish `num_fields`, so
    // every later well-formed record failed with `InconsistentFieldCount`.
    let data = b"\"abc\"XX,rest\nnext,row\nmore,row\n";
    let mut reader = Reader::new(Cursor::new(&data[..]));

    let rows: Vec<Row> = reader.rows().collect();
    assert_eq!(rows.len(), 3);
    assert!(rows[0].error().is_some());
    for row in &rows[1..] {
        assert!(row.error().is_none(), "unexpected error: {:?}", row.error());
        assert_eq!(row.len(), 2);
    }
}

// ── Line numbers ──────────────────────────────────────────────────────

fn error_lines(data: &[u8]) -> Vec<usize> {
    let mut reader = Reader::new(Cursor::new(data));
    let mut lines = Vec::new();
    for row in reader.rows() {
        for field in row.iter() {
            if let Err(e) = field {
                lines.push(e.line);
            }
        }
    }
    lines
}

#[test]
fn line_numbers_are_exact_for_lf() {
    // The invalid byte is on the third line.
    assert_eq!(error_lines(b"a\nb\nBAD\xff\nd\n"), vec![3]);
}

#[test]
fn line_numbers_are_exact_for_crlf() {
    assert_eq!(error_lines(b"a\r\nb\r\nBAD\xff\r\nd\r\n"), vec![3]);
}

#[test]
fn line_numbers_are_exact_for_cr_only() {
    assert_eq!(error_lines(b"a\rb\rBAD\xff\rd\r"), vec![3]);
}

#[test]
fn line_numbers_account_for_newlines_inside_quoted_fields() {
    // The record spans lines 2 and 3, so the next record is on line 4.
    assert_eq!(error_lines(b"a\n\"x\ny\"\nBAD\xff\n"), vec![4]);
}

#[test]
fn line_numbers_are_exact_across_buffer_boundaries() {
    let data = b"aaaa\nbbbb\ncccc\nBAD\xff\ndddd\n";
    for chunk in 1..=6 {
        let mut reader = Reader::new(Chunked::new(data, chunk));
        let mut lines = Vec::new();
        for row in reader.rows() {
            for field in row.iter() {
                if let Err(e) = field {
                    lines.push(e.line);
                }
            }
        }
        assert_eq!(lines, vec![4], "chunk size {chunk}");
    }
}

#[test]
fn line_numbers_with_crlf_inside_quoted_fields_across_chunks() {
    // The quoted field spans lines 1 and 2, so `BAD` is on line 3. Every chunk
    // size splits the `\r\n` pair differently.
    let data = b"\"one\r\ntwo\"\nBAD\xff\n";
    for chunk in 1..=10 {
        let mut reader = Reader::new(Chunked::new(data, chunk));
        let mut lines = Vec::new();
        for row in reader.rows() {
            for field in row.iter() {
                if let Err(e) = field {
                    lines.push(e.line);
                }
            }
        }
        assert_eq!(lines, vec![3], "chunk size {chunk}");
    }
}

#[test]
fn quoted_field_content_is_preserved_byte_for_byte() {
    // Line counting must not rewrite quoted content, including CRLF pairs that
    // straddle a refill.
    let data = b"\"a\r\nb\rc\nd\",x\n";
    for chunk in 1..=12 {
        let mut reader = Reader::new(Chunked::new(data, chunk));
        let row = reader.rows().next().unwrap();
        assert_eq!(row.to_vec().unwrap(), vec!["a\r\nb\rc\nd", "x"], "chunk size {chunk}");
    }
}

#[test]
fn record_error_reports_the_record_start_line() {
    let data = b"a,b\n\"unterminated\n";
    let mut reader = Reader::new(Cursor::new(&data[..]));
    let rows: Vec<Row> = reader.rows().collect();
    assert_eq!(rows[1].error().unwrap().line, 2);
}

// ── Error sources ─────────────────────────────────────────────────────

struct FailingReader;

impl std::io::Read for FailingReader {
    fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
        Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "underlying cause"))
    }
}

#[test]
fn io_error_source_survives_cloning_and_accessors() {
    let mut reader = Reader::new(FailingReader);
    let row = reader.rows().next().unwrap();

    // Direct access.
    let direct = row.error().unwrap().source().map(|s| s.to_string());
    assert_eq!(direct.as_deref(), Some("underlying cause"));

    // Through the deferred-error accessor, which clones the error.
    let via_to_vec = row.to_vec().unwrap_err();
    assert_eq!(via_to_vec.line, 0);
    assert_eq!(
        via_to_vec.into_source().map(|s| s.to_string()).as_deref(),
        Some("underlying cause")
    );

    // And the cause is shown by Display.
    assert!(row.error().unwrap().to_string().contains("underlying cause"));
}

// ── API consistency ───────────────────────────────────────────────────

#[test]
fn set_headers_pins_the_expected_field_count() {
    // `set_headers` establishes the expected column count, exactly like
    // `parse_headers`, so a ragged record is reported.
    let mut reader = Reader::new(Cursor::new(&b"x,y\n1,2,3\n"[..])).set_headers(vec!["a".into(), "b".into()]);
    let rows: Vec<Row> = reader.rows().collect();
    assert_eq!(rows.len(), 2);
    assert!(rows[0].error().is_none());
    assert!(matches!(
        rows[1].error().map(|e| e.kind()),
        Some(ReadErrorKind::InconsistentFieldCount {
            expected: 2,
            found: 3
        })
    ));
}

#[test]
fn row_accessors_agree_on_errored_rows() {
    let mut reader = Reader::new(Cursor::new(&b"\"hello\"garbage\n"[..]));
    let row = reader.rows().next().unwrap();

    assert!(row.error().is_some());
    assert!(row.to_vec().is_err());
    assert!(row.iter().next().unwrap().is_err());
    assert!(row.get(0).unwrap().is_err());
    assert!(row.get(1).is_none());
}

// ── Configuration validation ──────────────────────────────────────────

#[test]
#[should_panic(expected = "invalid CSV delimiter")]
fn reader_rejects_newline_delimiter() {
    let _ = Reader::new(Cursor::new(&b"a,b\n"[..])).set_delimiter(b'\n');
}

#[test]
#[should_panic(expected = "invalid CSV delimiter")]
fn reader_rejects_carriage_return_delimiter() {
    let _ = Reader::new(Cursor::new(&b"a,b\n"[..])).set_delimiter(b'\r');
}

#[test]
#[should_panic(expected = "invalid CSV delimiter")]
fn reader_rejects_quote_delimiter() {
    let _ = Reader::new(Cursor::new(&b"a,b\n"[..])).set_delimiter(b'"');
}

#[test]
#[should_panic(expected = "invalid CSV delimiter")]
fn writer_rejects_newline_delimiter() {
    let _ = Writer::new(Vec::new()).set_delimiter(b'\n');
}

// ── Writer ────────────────────────────────────────────────────────────

#[test]
fn writer_does_not_quote_empty_fields() {
    let mut writer = Writer::new(Vec::new()).set_flexible(true);
    writer.write_row(["a", "", "b"]).unwrap();
    writer.write_row(["", ""]).unwrap();
    let out = String::from_utf8(writer.into_inner().unwrap()).unwrap();
    assert_eq!(out, "a,,b\r\n,\r\n");
}

#[test]
fn writer_empty_field_output_stays_small() {
    let mut writer = Writer::new(Vec::new()).set_flexible(true);
    writer.write_row(vec![""; 100_000]).unwrap();
    let out = writer.into_inner().unwrap();
    assert_eq!(out.len(), 100_001, "empty fields should cost one byte each");
}

#[test]
fn write_headers_validates_against_stored_headers() {
    let mut writer = Writer::new(Vec::new()).set_headers(vec!["a".into(), "b".into()]);
    let err = writer.write_headers(["x", "y", "z"]).unwrap_err();
    assert!(matches!(
        err,
        WriteError::InconsistentFieldCount {
            expected: 2,
            found: 3,
            ..
        }
    ));
    // Nothing was written, so the output is still empty.
    assert_eq!(writer.into_inner().unwrap(), Vec::<u8>::new());
}

#[test]
fn write_headers_validates_against_prior_rows() {
    let mut writer = Writer::new(Vec::new());
    writer.write_row(["1", "2"]).unwrap();
    let err = writer.write_headers(["x", "y", "z"]).unwrap_err();
    assert!(matches!(
        err,
        WriteError::InconsistentFieldCount {
            expected: 2,
            found: 3,
            ..
        }
    ));
    // The header row must not be appended after the data row.
    let out = String::from_utf8(writer.into_inner().unwrap()).unwrap();
    assert_eq!(out, "1,2\r\n");
}

// ── Resource limits ───────────────────────────────────────────────────

#[test]
fn max_record_bytes_rejects_oversized_records() {
    let mut reader = Reader::new(Cursor::new(&b"a,b\ncccccccccccccccccccccccccccc,d\nx,y\n"[..]))
        .set_flexible(true)
        .set_max_record_bytes(10);

    let rows: Vec<Row> = reader.rows().collect();
    assert_eq!(rows.len(), 2);
    assert!(rows[0].error().is_none());
    assert!(matches!(
        rows[1].error().map(|e| e.kind()),
        Some(ReadErrorKind::RecordTooLarge {
            limit: 10
        })
    ));
}

#[test]
fn max_field_bytes_rejects_oversized_unquoted_fields() {
    let mut reader = Reader::new(Cursor::new(&b"a,bbbbbbbbbbbb,c\nx,y,z\n"[..]))
        .set_flexible(true)
        .set_max_field_bytes(4);

    // Reading stops at the offending record: it cannot be skipped without
    // reading all of it.
    let rows: Vec<Row> = reader.rows().collect();
    assert_eq!(rows.len(), 1);
    assert!(matches!(
        rows[0].error().map(|e| e.kind()),
        Some(ReadErrorKind::FieldTooLarge {
            limit: 4
        })
    ));
}

#[test]
fn max_field_bytes_rejects_oversized_quoted_fields() {
    let mut reader = Reader::new(Cursor::new(&b"\"aaaaaaaaaaaaaaaaaaaa\",b\nx,y\n"[..]))
        .set_flexible(true)
        .set_max_field_bytes(4);

    // Reading stops at the offending record: it cannot be skipped without
    // reading all of it.
    let rows: Vec<Row> = reader.rows().collect();
    assert_eq!(rows.len(), 1);
    assert!(matches!(
        rows[0].error().map(|e| e.kind()),
        Some(ReadErrorKind::FieldTooLarge {
            limit: 4
        })
    ));
}

#[test]
fn max_columns_rejects_wide_records() {
    let mut reader = Reader::new(Cursor::new(&b"a,b,c,d,e,f\nx,y\n"[..]))
        .set_flexible(true)
        .set_max_columns(3);

    // Reading stops at the offending record: it cannot be skipped without
    // reading all of it.
    let rows: Vec<Row> = reader.rows().collect();
    assert_eq!(rows.len(), 1);
    assert!(matches!(
        rows[0].error().map(|e| e.kind()),
        Some(ReadErrorKind::TooManyColumns {
            limit: 3
        })
    ));
}

#[test]
fn limits_are_off_by_default() {
    // A 64 KiB field parses fine when no limit is configured.
    let mut data = Vec::new();
    data.extend(std::iter::repeat_n(b'A', 64 * 1024));
    data.push(b'\n');
    let mut reader = Reader::new(Cursor::new(&data[..]));
    let row = reader.rows().next().unwrap();
    assert!(row.error().is_none());
    assert_eq!(row.get(0).unwrap().unwrap().len(), 64 * 1024);
}

// ── Byte-order mark ───────────────────────────────────────────────────

#[test]
fn bom_is_stripped_across_all_chunk_sizes() {
    // The three BOM bytes must be recognised even when a refill splits them.
    let data = b"\xef\xbb\xbfname,age\nAlice,30\n";
    for chunk in 1..=6 {
        let mut reader = Reader::new(Chunked::new(data, chunk));
        let headers = reader.parse_headers().unwrap().to_vec();
        assert_eq!(headers, vec!["name", "age"], "chunk size {chunk}");
        let row = reader.rows().next().unwrap();
        assert_eq!(row.to_vec().unwrap(), vec!["Alice", "30"], "chunk size {chunk}");
    }
}

#[test]
fn bom_only_input_yields_no_rows() {
    for chunk in 1..=4 {
        let mut reader = Reader::new(Chunked::new(b"\xef\xbb\xbf", chunk));
        assert_eq!(reader.rows().count(), 0, "chunk size {chunk}");
        assert_eq!(reader.rows_bytes().count(), 0, "chunk size {chunk}");
    }
}

#[test]
fn bom_before_a_quoted_field_still_parses() {
    // The trap Go hits (#33887): the BOM precedes the `"`, so unless it is
    // removed the quote is not seen at the start of a field.
    let data = b"\xef\xbb\xbf\"a,b\",c\n";
    for chunk in 1..=5 {
        let mut reader = Reader::new(Chunked::new(data, chunk));
        let row = reader.rows().next().unwrap();
        assert_eq!(row.to_vec().unwrap(), vec!["a,b", "c"], "chunk size {chunk}");
    }
}

#[test]
fn bom_is_stripped_for_rows_bytes_too() {
    let mut reader = Reader::new(Cursor::new(&b"\xef\xbb\xbfa,b\n"[..]));
    let row = reader.rows_bytes().next().unwrap();
    assert_eq!(row.get(0), Some(&b"a"[..]));
}

#[test]
fn bom_is_not_counted_against_the_record_limit() {
    // `a,b` is exactly three bytes; if the BOM were charged to the record the
    // limit would trip.
    let mut reader = Reader::new(Cursor::new(&b"\xef\xbb\xbfa,b\n"[..])).set_max_record_bytes(3);
    let rows: Vec<Row> = reader.rows().collect();
    assert_eq!(rows.len(), 1);
    assert!(rows[0].error().is_none(), "unexpected error: {:?}", rows[0].error());
}

#[test]
fn partial_bom_prefixes_are_data() {
    // Fewer than three leading bytes cannot be a BOM, so they are field data.
    for chunk in 1..=2 {
        let mut reader = Reader::new(Chunked::new(b"\xef", chunk));
        let row = reader.rows_bytes().next().unwrap();
        assert_eq!(row.get(0), Some(&b"\xef"[..]), "chunk size {chunk}");

        let mut reader = Reader::new(Chunked::new(b"\xef\xbb", chunk));
        let row = reader.rows_bytes().next().unwrap();
        assert_eq!(row.get(0), Some(&b"\xef\xbb"[..]), "chunk size {chunk}");
    }
}

#[test]
fn bom_is_stripped_only_once() {
    // A second BOM is data even when it immediately follows the first.
    let mut reader = Reader::new(Cursor::new(&b"\xef\xbb\xbf\xef\xbb\xbfa,b\n"[..]));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.get(0).unwrap().unwrap().as_bytes(), b"\xef\xbb\xbfa");
}

#[test]
fn writer_bom_is_written_once_before_everything() {
    let mut writer = Writer::new(Vec::new()).set_write_bom(true);
    writer.write_headers(["name", "age"]).unwrap();
    writer.write_row(["Alice", "30"]).unwrap();
    writer.write_row(["Bob", "25"]).unwrap();
    let out = writer.into_inner().unwrap();

    assert_eq!(
        out,
        b"\xef\xbb\xbfname,age\r\nAlice,30\r\nBob,25\r\n",
        "got {:?}",
        String::from_utf8_lossy(&out)
    );
    assert_eq!(out.iter().filter(|&&b| b == 0xEF).count(), 1);
}

#[test]
fn writer_bom_defaults_to_off() {
    let mut writer = Writer::new(Vec::new());
    writer.write_row(["a", "b"]).unwrap();
    assert_eq!(writer.into_inner().unwrap(), b"a,b\r\n");
}

#[test]
fn writer_bom_round_trips_through_the_reader() {
    let mut writer = Writer::new(Vec::new()).set_write_bom(true);
    writer.write_headers(["name", "age"]).unwrap();
    writer.write_row(["Alice", "30"]).unwrap();
    let out = writer.into_inner().unwrap();

    let mut reader = Reader::new(Cursor::new(&out[..]));
    assert_eq!(reader.parse_headers().unwrap(), &["name", "age"]);
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec!["Alice", "30"]);
}

#[test]
fn writer_bom_is_not_lost_when_a_row_is_rejected() {
    // A strict-mode rejection rolls the buffer back; the BOM must survive
    // because it is not part of the rejected row.
    let mut writer = Writer::new(Vec::new()).set_write_bom(true);
    writer.write_row(["a", "b"]).unwrap();
    assert!(writer.write_row(["1", "2", "3"]).is_err());
    let out = writer.into_inner().unwrap();
    assert_eq!(out, b"\xef\xbb\xbfa,b\r\n");
}

// ── serde robustness ──────────────────────────────────────────────────

#[cfg(feature = "serde")]
mod serde_robustness {
    use super::*;

    #[derive(serde::Deserialize, Debug)]
    #[allow(dead_code)]
    struct Pair {
        a: String,
        b: String,
    }

    #[test]
    fn short_row_is_an_error_not_an_empty_string() {
        let mut reader = Reader::new(Cursor::new(&b"a,b\n1\n"[..])).set_flexible(true);
        reader.parse_headers().unwrap();
        let row = reader.rows().next().unwrap();
        let result: Result<Pair, _> = row.deserialize();
        assert!(result.is_err(), "expected an error, got {result:?}");
    }

    #[test]
    fn invalid_utf8_is_an_error_not_an_empty_string() {
        let mut reader = Reader::new(Cursor::new(&b"a,b\n\xff,2\n"[..])).set_flexible(true);
        reader.parse_headers().unwrap();
        let row = reader.rows().next().unwrap();
        let result: Result<Pair, _> = row.deserialize();
        assert!(result.is_err(), "expected an error, got {result:?}");
    }

    #[test]
    fn missing_column_still_defaults_to_empty() {
        // A struct field with no matching header is unaffected by the above.
        #[derive(serde::Deserialize, Debug)]
        struct WithCity {
            a: String,
            city: String,
        }
        let mut reader = Reader::new(Cursor::new(&b"a,b\n1,2\n"[..]));
        reader.parse_headers().unwrap();
        let row = reader.rows().next().unwrap();
        let record: WithCity = row.deserialize().unwrap();
        assert_eq!(record.a, "1");
        assert_eq!(record.city, "");
    }

    #[test]
    fn duplicate_headers_are_rejected() {
        let mut reader = Reader::new(Cursor::new(&b"a,a\n1,2\n"[..]));
        reader.parse_headers().unwrap();
        let row = reader.rows().next().unwrap();
        let result: Result<Pair, _> = row.deserialize();
        assert!(result.is_err(), "ambiguous headers must not silently drop a column");
    }

    #[test]
    fn bom_does_not_break_header_matching() {
        // The motivation for stripping: an Excel-style file whose first header
        // would otherwise be "\u{feff}a" and match no struct field.
        let data = b"\xef\xbb\xbfa,b\n1,2\n";
        let mut reader = Reader::new(Cursor::new(&data[..]));
        assert_eq!(reader.parse_headers().unwrap(), &["a", "b"]);
        let row = reader.rows().next().unwrap();
        let record: Pair = row.deserialize().unwrap();
        assert_eq!(record.a, "1");
        assert_eq!(record.b, "2");
    }

    #[test]
    fn deserialize_reports_the_row_line() {
        let mut reader = Reader::new(Cursor::new(&b"a,b\nx,y\n\xff,2\n"[..]));
        reader.parse_headers().unwrap();
        let rows: Vec<Row> = reader.rows().collect();
        let err = rows[1].deserialize::<Pair>().unwrap_err();
        assert_eq!(err.line, 3);
    }
}

// ── Round-trip property ───────────────────────────────────────────────

#[test]
fn writer_output_round_trips() {
    const ALPHABET: &[char] = &[
        'a', 'b', 'Z', ',', '"', '\n', '\r', ' ', '\t', 'é', '日', '\\', '\'', '=', '+', '@',
    ];

    let mut rng = Rng(0x1234_5678_9ABC_DEF0);
    let mut writer = Writer::new(Vec::new()).set_flexible(true);
    let mut expected: Vec<Vec<String>> = Vec::new();

    for _ in 0..500 {
        // At least two fields: a record consisting of a single empty field is
        // written as a blank line, which every CSV reader (including this one)
        // skips, so it cannot round-trip.
        let fields = 2 + rng.below(5);
        let mut row = Vec::with_capacity(fields);
        for _ in 0..fields {
            let len = rng.below(8);
            let field: String = (0..len).map(|_| ALPHABET[rng.below(ALPHABET.len())]).collect();
            row.push(field);
        }
        writer.write_row(row.iter().map(|s| s.as_str())).unwrap();
        expected.push(row);
    }

    let out = writer.into_inner().unwrap();
    let mut reader = Reader::new(Cursor::new(&out[..])).set_flexible(true);
    let actual: Vec<Vec<String>> = reader
        .rows()
        .map(|row| row.to_vec().unwrap().into_iter().map(|s| s.to_string()).collect())
        .collect();

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual, expected);
}

#[test]
fn reader_accepts_output_from_any_chunk_size() {
    let mut writer = Writer::new(Vec::new()).set_flexible(true);
    let rows: Vec<Vec<&str>> = vec![
        vec!["a", "", "b"],
        vec!["with,comma", "with\"quote", "with\nnewline"],
        vec!["é", "日", " plain "],
        vec!["", "x"],
    ];
    for row in &rows {
        writer.write_row(row.iter().copied()).unwrap();
    }
    let out = writer.into_inner().unwrap();

    let expected: Vec<Vec<String>> = rows.iter().map(|r| r.iter().map(|s| s.to_string()).collect()).collect();

    for chunk in 1..=17 {
        let mut reader = Reader::new(Chunked::new(&out, chunk)).set_flexible(true);
        let actual: Vec<Vec<String>> = reader
            .rows()
            .map(|row| row.to_vec().unwrap().into_iter().map(|s| s.to_string()).collect())
            .collect();
        assert_eq!(actual, expected, "chunk size {chunk}");
    }
}
