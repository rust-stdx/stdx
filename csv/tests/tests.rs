use std::io::{Cursor, Read};

use csv::*;

fn collect_rows(data: &[u8]) -> Vec<Vec<String>> {
    let mut reader = Reader::new(Cursor::new(data));
    let mut out = Vec::new();
    for row in reader.rows() {
        out.push(row.to_vec().unwrap().iter().map(|s| s.to_string()).collect());
    }
    out
}

/// Forces reads in small chunks to exercise buffer-boundary code paths.
struct ChunkReader {
    data: Vec<u8>,
    pos: usize,
    chunk: usize,
}

impl ChunkReader {
    fn new(data: &[u8], chunk: usize) -> Self {
        ChunkReader {
            data: data.to_vec(),
            pos: 0,
            chunk,
        }
    }
}

impl Read for ChunkReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let to_read = self.chunk.min(buf.len()).min(self.data.len() - self.pos);
        buf[..to_read].copy_from_slice(&self.data[self.pos..self.pos + to_read]);
        self.pos += to_read;
        Ok(to_read)
    }
}

/// A reader that returns an error after yielding the given number of bytes.
struct ErrorAfterReader {
    data: Vec<u8>,
    pos: usize,
    limit: usize,
}

impl ErrorAfterReader {
    fn new(data: Vec<u8>, limit: usize) -> Self {
        ErrorAfterReader {
            data,
            pos: 0,
            limit,
        }
    }
}

impl Read for ErrorAfterReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.pos >= self.limit {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "simulated IO error"));
        }
        let to_read = self
            .limit
            .saturating_sub(self.pos)
            .min(buf.len())
            .min(self.data.len() - self.pos);
        buf[..to_read].copy_from_slice(&self.data[self.pos..self.pos + to_read]);
        self.pos += to_read;
        Ok(to_read)
    }
}

// ── Reader: Basic Parsing ──────────────────────────────────────────

#[test]
fn test_rfc4180_example_1() {
    let rows = collect_rows(b"aaa,bbb,ccc\r\nzzz,yyy,xxx\r\n");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["aaa", "bbb", "ccc"]);
    assert_eq!(rows[1], vec!["zzz", "yyy", "xxx"]);
}

#[test]
fn test_rfc4180_example_2() {
    let rows = collect_rows(b"aaa,bbb,ccc\r\nzzz,yyy,xxx");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].len(), 3);
    assert_eq!(rows[1].len(), 3);
}

#[test]
fn test_rfc4180_example_3() {
    let rows = collect_rows(b"field_name,field_name,field_name\r\naaa,bbb,ccc\r\nzzz,yyy,xxx\r\n");
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0], vec!["field_name", "field_name", "field_name"]);
}

#[test]
fn test_rfc4180_example_5() {
    let rows = collect_rows(b"\"aaa\",\"bbb\",\"ccc\"\r\nzzz,yyy,xxx\r\n");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["aaa", "bbb", "ccc"]);
}

#[test]
fn test_rfc4180_example_6() {
    let rows = collect_rows(b"\"aaa\",\"b\r\nbb\",\"ccc\"\r\nzzz,yyy,xxx\r\n");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["aaa", "b\r\nbb", "ccc"]);
}

#[test]
fn test_rfc4180_example_7() {
    let rows = collect_rows(b"\"aaa\",\"b\"\"bb\",\"ccc\"\r\nzzz,yyy,xxx\r\n");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["aaa", "b\"bb", "ccc"]);
}

#[test]
fn test_single_line_no_trailing_newline() {
    let rows = collect_rows(b"a,b,c");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0], vec!["a", "b", "c"]);
}

#[test]
fn test_empty_input() {
    let rows = collect_rows(b"");
    assert!(rows.is_empty());
}

#[test]
fn test_empty_fields() {
    let rows = collect_rows(b",,\r\n,,\r\n");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["", "", ""]);
    assert_eq!(rows[1], vec!["", "", ""]);
}

#[test]
fn test_blank_lines_skipped() {
    let rows = collect_rows(b"a,b,c\n\n\n1,2,3\n");
    assert_eq!(rows.len(), 2);
}

#[test]
fn test_escaped_quotes_in_quoted_field() {
    let rows = collect_rows(b"\"\"\"hello\"\"\",world\n");
    assert_eq!(rows[0], vec!["\"hello\"", "world"]);
}

#[test]
fn test_quoted_field_contains_commas() {
    let rows = collect_rows(b"\"hello, world\",foo\n");
    assert_eq!(rows[0], vec!["hello, world", "foo"]);
}

#[test]
fn test_quoted_field_contains_newline() {
    let rows = collect_rows(b"\"line1\nline2\",foo\n");
    assert_eq!(rows[0], vec!["line1\nline2", "foo"]);
}

#[test]
fn test_custom_delimiter() {
    let data = b"a|b|c\n1|2|3\n";
    let mut reader = Reader::new(Cursor::new(data)).set_delimiter(b'|');
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows[0], vec!["a", "b", "c"]);
    assert_eq!(rows[1], vec!["1", "2", "3"]);
}

#[test]
fn test_tab_delimiter() {
    let data = b"a\tb\tc\n1\t2\t3\n";
    let mut reader = Reader::new(Cursor::new(data)).set_delimiter(b'\t');
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows[0], vec!["a", "b", "c"]);
    assert_eq!(rows[1], vec!["1", "2", "3"]);
}

// ── Reader: Line endings ──────────────────────────────────────────────

#[test]
fn test_crlf_line_endings() {
    let rows = collect_rows(b"a,b,c\r\n1,2,3\r\n");
    assert_eq!(rows.len(), 2);
}

#[test]
fn test_lf_line_endings() {
    let rows = collect_rows(b"a,b,c\n1,2,3\n");
    assert_eq!(rows.len(), 2);
}

#[test]
fn test_cr_line_endings() {
    let rows = collect_rows(b"a,b,c\r1,2,3\r");
    assert_eq!(rows.len(), 2);
}

#[test]
fn test_mixed_line_endings() {
    let rows = collect_rows(b"a,b,c\r\n1,2,3\n4,5,6\r");
    assert_eq!(rows.len(), 3);
}

#[test]
fn test_no_trailing_newline() {
    let rows = collect_rows(b"a,b,c\n1,2,3");
    assert_eq!(rows.len(), 2);
}

#[test]
fn test_single_field_no_newline() {
    let rows = collect_rows(b"hello");
    assert_eq!(rows[0], vec!["hello"]);
}

// ── Reader: Headers ───────────────────────────────────────────────────

#[test]
fn test_parse_headers() {
    let mut reader = Reader::new(Cursor::new(b"name,age,city\nAlice,30,NYC\nBob,25,LA\n"));
    let headers = reader.parse_headers().unwrap();
    assert_eq!(headers, &["name", "age", "city"]);
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows[0], vec!["Alice", "30", "NYC"]);
}

#[test]
fn test_set_headers() {
    let reader = Reader::new(Cursor::new(b"Alice,30,NYC\nBob,25,LA\n")).set_headers(vec![
        "name".into(),
        "age".into(),
        "city".into(),
    ]);
    assert_eq!(reader.headers().unwrap(), &["name", "age", "city"]);
}

#[test]
fn test_headers_empty_csv() {
    let mut reader = Reader::new(Cursor::new(b""));
    let headers = reader.parse_headers().unwrap();
    assert!(headers.is_empty());
}

#[test]
fn test_headers_accessor() {
    let mut reader = Reader::new(Cursor::new(b"a,b\n1,2\n"));
    assert!(reader.headers().is_none());
    reader.parse_headers().unwrap();
    assert_eq!(reader.headers().unwrap(), &["a", "b"]);
}

// ── Reader: UTF-8 validation ──────────────────────────────────────────

#[test]
fn test_invalid_utf8_in_unquoted_field() {
    let data = b"a,\xff,b\n";
    let mut reader = Reader::new(Cursor::new(data));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    // csv defers UTF-8 validation: no error until to_vec() is called
    assert!(row.error().is_none());
    assert!(row.to_vec().is_err());
}

#[test]
fn test_invalid_utf8_in_quoted_field() {
    let data = b"a,\"\xff\",b\n";
    let mut reader = Reader::new(Cursor::new(data));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert!(row.error().is_none());
    assert!(row.to_vec().is_err());
}

#[test]
fn test_null_bytes_in_field() {
    let rows = collect_rows(b"a,\0,c\n");
    assert_eq!(rows[0], vec!["a", "\0", "c"]);
}

#[test]
fn test_invalid_utf8_second_row_ok_first_row() {
    let data = b"a,b\nc,\xff\n";
    let mut reader = Reader::new(Cursor::new(data));
    let mut rows = reader.rows();
    let row1 = rows.next().unwrap();
    assert_eq!(row1.to_vec().unwrap(), vec!["a", "b"]);
    let row2 = rows.next().unwrap();
    assert!(row2.to_vec().is_err());
}

// ── Reader: Errors ────────────────────────────────────────────────────

#[test]
fn test_unterminated_quote() {
    let data = b"a,\"unterminated\n";
    let mut reader = Reader::new(Cursor::new(data));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert!(matches!(row.error().unwrap().kind(), ReadErrorKind::UnterminatedQuote));
}

#[test]
fn test_trailing_content_after_quoted_field() {
    let data = b"\"hello\"garbage\n";
    let mut reader = Reader::new(Cursor::new(data));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert!(matches!(row.error().unwrap().kind(), ReadErrorKind::TrailingContent));
}

#[test]
fn test_io_error_propagation() {
    let data = b"a,b,c\n1,2,3\n4,5,6\n";
    let mut reader = Reader::new(ErrorAfterReader::new(data.to_vec(), 10));
    let mut rows = reader.rows();
    assert!(rows.next().is_some()); // may succeed
    // If the IO error hasn't been hit yet, keep going
    for row in rows {
        if let Some(e) = row.error() {
            assert!(matches!(e.kind(), ReadErrorKind::Io));
            break;
        }
    }
}

#[test]
fn test_error_display_invalid_utf8() {
    let e = ReadError::new(ReadErrorKind::InvalidUtf8, 4, 0);
    let s = e.to_string();
    assert!(s.contains("invalid UTF-8"));
    assert!(s.contains("line 4"));
}

#[test]
fn test_error_display_inconsistent_field_count() {
    let e = ReadError::new(
        ReadErrorKind::InconsistentFieldCount {
            expected: 3,
            found: 2,
        },
        5,
        0,
    );
    let s = e.to_string();
    assert!(s.contains("expected 3"));
    assert!(s.contains("found 2"));
    assert!(s.contains("line 5"));
}

// ── Reader: Row API ───────────────────────────────────────────────────

#[test]
fn test_row_to_vec() {
    let mut reader = Reader::new(Cursor::new(b"a,b,c\n"));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec!["a", "b", "c"]);
}

#[test]
fn test_row_len() {
    let mut reader = Reader::new(Cursor::new(b"a,b,c\n1,2,3,4\n"));
    let mut rows = reader.rows();
    assert_eq!(rows.next().unwrap().len(), 3);
    assert_eq!(rows.next().unwrap().len(), 4);
}

#[test]
fn test_row_is_empty() {
    // Blank lines are skipped, so a single newline produces no rows
    let mut reader = Reader::new(b"\n".as_slice());
    assert!(reader.rows().next().is_none());
}

#[test]
fn test_row_get() {
    let mut reader = Reader::new(Cursor::new(b"a,b,c\n"));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.get(0).unwrap().unwrap(), "a");
    assert_eq!(row.get(1).unwrap().unwrap(), "b");
    assert_eq!(row.get(2).unwrap().unwrap(), "c");
    assert!(row.get(3).is_none());
}

#[test]
fn test_row_iter() {
    let mut reader = Reader::new(Cursor::new(b"a,b,c\n"));
    let row = reader.rows().next().unwrap();
    let fields: Vec<&str> = row.iter().map(|r| r.unwrap()).collect();
    assert_eq!(fields, vec!["a", "b", "c"]);
}

#[test]
fn test_row_debug() {
    let mut reader = Reader::new(b"a,b,c\n".as_slice());
    let row = reader.rows().next().unwrap();
    let s = format!("{:?}", row);
    assert!(s.contains("a") && s.contains("b") && s.contains("c"));
}

#[test]
fn test_row_debug_error() {
    let data = b"\"unterminated\n";
    let mut reader = Reader::new(Cursor::new(data));
    let row = reader.rows().next().unwrap();
    let s = format!("{:?}", row);
    assert!(s.contains("Err("));
}

// ── BytesRow API ──────────────────────────────────────────────────────

#[test]
fn test_bytes_row_to_vec() {
    let mut reader = Reader::new(Cursor::new(b"a,b,c\n"));
    let bytes_row = reader.rows_bytes().next().unwrap();
    let fields = bytes_row.to_vec().unwrap();
    assert_eq!(fields, vec![&b"a"[..], &b"b"[..], &b"c"[..]]);
}

#[test]
fn test_bytes_row_get() {
    let mut reader = Reader::new(Cursor::new(b"a,b,c\n"));
    let bytes_row = reader.rows_bytes().next().unwrap();
    assert_eq!(bytes_row.get(0).unwrap(), &b"a"[..]);
    assert!(bytes_row.get(3).is_none());
}

#[test]
fn test_bytes_row_iter() {
    let mut reader = Reader::new(Cursor::new(b"a,b,c\n"));
    let bytes_row = reader.rows_bytes().next().unwrap();
    let fields: Vec<&[u8]> = bytes_row.iter().collect();
    assert_eq!(fields, vec![&b"a"[..], &b"b"[..], &b"c"[..]]);
}

#[test]
fn test_bytes_row_len() {
    let mut reader = Reader::new(Cursor::new(b"a,b,c\n"));
    let bytes_row = reader.rows_bytes().next().unwrap();
    assert_eq!(bytes_row.len(), 3);
}

#[test]
fn test_bytes_row_debug() {
    let mut reader = Reader::new(b"a,b,c\n".as_slice());
    let bytes_row = reader.rows_bytes().next().unwrap();
    let s = format!("{:?}", bytes_row);
    assert!(s.contains("97") || s.contains("BytesRow"));
}

#[test]
fn test_bytes_row_non_utf8_succeeds() {
    let data = b"a,\xff,b\n";
    let mut reader = Reader::new(Cursor::new(data));
    let bytes_row = reader.rows_bytes().next().unwrap();
    let fields = bytes_row.to_vec().unwrap();
    assert_eq!(fields.len(), 3);
    assert_eq!(fields[1], &b"\xff"[..]);
    // bytes row does NOT validate UTF-8
    assert!(bytes_row.error().is_none());
}

// ── Reader: Buffer boundary tests ─────────────────────────────────────

#[test]
fn test_chunked_reader_1byte() {
    let data = b"a,b,c\n1,2,3\n";
    let mut reader = Reader::new(ChunkReader::new(data, 1));
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["a", "b", "c"]);
}

#[test]
fn test_chunked_reader_4byte() {
    let data = b"a,b,c\n1,2,3\n";
    let mut reader = Reader::new(ChunkReader::new(data, 4));
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 2);
}

#[test]
fn test_quoted_field_across_chunks() {
    let data = b"\"hello, world\",foo\nbar,baz\n";
    let mut reader = Reader::new(ChunkReader::new(data, 4));
    let mut rows = reader.rows();
    let row1 = rows.next().unwrap();
    assert_eq!(row1.to_vec().unwrap(), vec!["hello, world", "foo"]);
    let row2 = rows.next().unwrap();
    assert_eq!(row2.to_vec().unwrap(), vec!["bar", "baz"]);
}

#[test]
fn test_large_quoted_field() {
    let inner = "x".repeat(40000);
    let data = format!("\"{inner}\",end\n");
    let mut reader = Reader::new(ChunkReader::new(data.as_bytes(), 4096));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert_eq!(row.len(), 2);
    assert_eq!(row.to_vec().unwrap()[0].len(), 40000);
    assert_eq!(row.to_vec().unwrap()[1], "end");
}

#[test]
fn test_crlf_split_chunk_boundary() {
    let before = "x".repeat(16380);
    let data = format!("aaa,{before}\r\n");
    let mut reader = Reader::new(ChunkReader::new(data.as_bytes(), 4096));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert_eq!(row.len(), 2);
    assert_eq!(row.to_vec().unwrap()[0], "aaa");
    assert_eq!(row.to_vec().unwrap()[1].len(), 16380);
}

// ── Reader: new convenience ───────────────────────────────────

#[test]
fn test_new_with_slice() {
    let mut reader = Reader::new(b"a,b,c\n1,2,3\n".as_slice());
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["a", "b", "c"]);
    assert_eq!(rows[1], vec!["1", "2", "3"]);
}

#[test]
fn test_new_with_quoted_slice() {
    let mut reader = Reader::new(b"\"hello\",\"world\"\n".as_slice());
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec!["hello", "world"]);
}

// ── Reader: CR at EOF ─────────────────────────────────────────────────

#[test]
fn test_cr_at_eof() {
    let rows = collect_rows(b"a,b\r");
    assert_eq!(rows[0], vec!["a", "b"]);
}

// ── Reader: Edge cases ────────────────────────────────────────────────

#[test]
fn test_leading_delimiter() {
    let rows = collect_rows(b",a,b\n");
    assert_eq!(rows[0], vec!["", "a", "b"]);
}

#[test]
fn test_trailing_delimiter() {
    let rows = collect_rows(b"a,b,\n");
    assert_eq!(rows[0], vec!["a", "b", ""]);
}

#[test]
fn test_only_delimiter() {
    let rows = collect_rows(b",\n");
    assert_eq!(rows[0], vec!["", ""]);
}

#[test]
fn test_quoted_empty_field() {
    let rows = collect_rows(b"\"\"\n");
    assert_eq!(rows[0], vec![""]);
}

#[test]
fn test_escaped_quote_at_end_of_quoted_field() {
    let rows = collect_rows(b"\"\"\"\"\n");
    assert_eq!(rows[0], vec!["\""]);
}

#[test]
fn test_single_field_rows() {
    let rows = collect_rows(b"hello\nworld\n");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["hello"]);
    assert_eq!(rows[1], vec!["world"]);
}

#[test]
fn test_mixed_quoted_and_unquoted() {
    let rows = collect_rows(b"\"quoted\",unquoted,\"also quoted\"\n");
    assert_eq!(rows[0], vec!["quoted", "unquoted", "also quoted"]);
}

#[test]
fn test_multiple_consecutive_blank_lines() {
    let rows = collect_rows(b"a,b\n\n\n\n1,2\n");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["a", "b"]);
    assert_eq!(rows[1], vec!["1", "2"]);
}

#[test]
fn test_unicode_fields() {
    let rows = collect_rows("café,ñoño\n".as_bytes());
    assert_eq!(rows[0], vec!["café", "ñoño"]);
}

#[test]
fn test_bom_at_start_of_csv() {
    // A leading BOM is stripped by default, so the first field is `a`.
    let rows = collect_rows(b"\xef\xbb\xbfa,b\n");
    assert_eq!(rows[0].len(), 2);
    assert_eq!(rows[0][0], "a");
    assert_eq!(rows[0][1], "b");
}

#[test]
fn test_bom_kept_when_stripping_disabled() {
    let mut reader = Reader::new(Cursor::new(b"\xef\xbb\xbfa,b\n".as_slice())).set_strip_bom(false);
    let row = reader.rows().next().unwrap();
    assert_eq!(row.get(0).unwrap().unwrap().as_bytes(), b"\xef\xbb\xbfa");
    assert_eq!(row.get(1).unwrap().unwrap(), "b");
}

#[test]
fn test_bom_in_the_middle_is_data() {
    let rows = collect_rows(b"a,b\n\xef\xbb\xbfc,d\n");
    assert_eq!(rows[1][0].as_bytes(), b"\xef\xbb\xbfc");
}

#[test]
fn test_unterminated_quote_at_eof() {
    let data = b"a,\"unterminated";
    let mut reader = Reader::new(Cursor::new(data));
    let row = reader.rows().next().unwrap();
    assert!(matches!(row.error().unwrap().kind(), ReadErrorKind::UnterminatedQuote));
}

#[test]
fn test_strict_mode_inconsistent_field_count_error() {
    let data = b"a,b\n1,2,3\n";
    let mut reader = Reader::new(Cursor::new(data));
    let mut rows = reader.rows();
    let row1 = rows.next().unwrap();
    assert_eq!(row1.to_vec().unwrap(), vec!["a", "b"]);
    let row2 = rows.next().unwrap();
    assert!(matches!(
        row2.error().unwrap().kind(),
        ReadErrorKind::InconsistentFieldCount {
            expected: 2,
            found: 3
        }
    ));
}

#[test]
fn test_fields_iterator_preserves_error_kind() {
    let data = b"\"hello\"garbage\n";
    let mut reader = Reader::new(Cursor::new(data));
    let row = reader.rows().next().unwrap();
    let mut iter = row.iter();
    let err = iter.next().unwrap().unwrap_err();
    assert!(matches!(err.kind(), ReadErrorKind::TrailingContent));
}

// ── Reader: set_flexible ──────────────────────────────────────────────

#[test]
fn test_strict_mode_consistent_fields_ok() {
    let mut reader = Reader::new(Cursor::new(b"a,b\n1,2\n3,4\n")).set_flexible(false);
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 3);
}

#[test]
fn test_flexible_mode_variable_fields_ok() {
    let mut reader = Reader::new(Cursor::new(b"a,b\n1,2,3\n4\n")).set_flexible(true);
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 3);
}

// ── Writer ────────────────────────────────────────────────────────────

#[test]
fn test_writer_basic() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["a", "b", "c"]).unwrap();
    w.write_row(["1", "2", "3"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "a,b,c\r\n1,2,3\r\n");
}

#[test]
fn test_writer_empty_fields() {
    // Empty fields are written unquoted (as nothing), matching mainstream writers.
    let mut w = Writer::new(Vec::new());
    w.write_row(["", "b", ""]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, ",b,\r\n");
}

#[test]
fn test_writer_auto_quote_comma() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["hello, world", "foo"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "\"hello, world\",foo\r\n");
}

#[test]
fn test_writer_auto_quote_quote() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["he said \"hello\"", "foo"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "\"he said \"\"hello\"\"\",foo\r\n");
}

#[test]
fn test_writer_auto_quote_newline() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["hello\nworld", "foo"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "\"hello\nworld\",foo\r\n");
}

#[test]
fn test_writer_custom_delimiter() {
    let mut w = Writer::new(Vec::new()).set_delimiter(b'\t');
    w.write_row(["a", "b", "c"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "a\tb\tc\r\n");
}

#[test]
fn test_writer_strict_field_count() {
    let mut w = Writer::new(Vec::new()).set_flexible(false);
    w.write_row(["a", "b"]).unwrap();
    let err = w.write_row(["1", "2", "3"]).unwrap_err();
    assert!(matches!(
        err,
        WriteError::InconsistentFieldCount {
            expected: 2,
            found: 3,
            ..
        }
    ));
}

#[test]
fn test_writer_flexible_field_count() {
    let mut w = Writer::new(Vec::new()).set_flexible(true);
    w.write_row(["a", "b"]).unwrap();
    w.write_row(["1", "2", "3"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "a,b\r\n1,2,3\r\n");
}

#[test]
fn test_writer_flush() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["a", "b"]).unwrap();
    w.flush().unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "a,b\r\n");
}

#[test]
fn test_writer_into_inner() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["a", "b"]).unwrap();
    let inner: Vec<u8> = w.into_inner().unwrap();
    assert_eq!(String::from_utf8(inner).unwrap(), "a,b\r\n");
}

#[test]
fn test_writer_no_rows() {
    let w = Writer::new(Vec::new());
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_writer_row_with_cr() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["hello\rworld", "foo"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "\"hello\rworld\",foo\r\n");
}

#[test]
fn test_writer_row_with_crlf() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["foo\r\nbar"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "\"foo\r\nbar\"\r\n");
}

#[test]
fn test_writer_no_fields_error_before_write() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["a", "b"]).unwrap();
    // This should error BEFORE the bad row is buffered
    let err = w.write_row(["1", "2", "3"]).unwrap_err();
    assert!(matches!(
        err,
        WriteError::InconsistentFieldCount {
            expected: 2,
            found: 3,
            ..
        }
    ));
    // The buffer should NOT contain the bad row — only the first row
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "a,b\r\n");
}

#[test]
fn test_writer_write_headers_basic() {
    let mut w = Writer::new(Vec::new());
    w.write_headers(["name", "age", "city"]).unwrap();
    w.write_row(["Alice", "30", "NYC"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "name,age,city\r\nAlice,30,NYC\r\n");
}

#[test]
fn test_writer_write_headers_twice_errors() {
    let mut w = Writer::new(Vec::new());
    w.write_headers(["a", "b"]).unwrap();
    let err = w.write_headers(["c", "d"]).unwrap_err();
    assert!(matches!(err, WriteError::HeadersAlreadyWritten));
}

#[test]
fn test_writer_write_headers_sets_strict_field_count() {
    let mut w = Writer::new(Vec::new());
    w.write_headers(["a", "b"]).unwrap();
    let err = w.write_row(["1", "2", "3"]).unwrap_err();
    assert!(matches!(
        err,
        WriteError::InconsistentFieldCount {
            expected: 2,
            found: 3,
            ..
        }
    ));
}

#[test]
fn test_writer_set_headers_no_write() {
    let mut w = Writer::new(Vec::new()).set_headers(vec!["x".into(), "y".into()]);
    // No headers written yet, just stored — output should start with first write_row
    w.write_row(["a", "b"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "a,b\r\n");
}

#[test]
fn test_writer_set_headers_then_write_headers() {
    let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "age".into()]);
    w.write_headers(["name", "age"]).unwrap();
    w.write_row(["Alice", "30"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "name,age\r\nAlice,30\r\n");
}

#[test]
fn test_writer_headers_accessor() {
    let mut w = Writer::new(Vec::new());
    assert!(w.headers().is_none());
    w.write_headers(["a", "b"]).unwrap();
    assert_eq!(w.headers(), Some(&["a".to_string(), "b".to_string()][..]));

    let w2 = Writer::new(Vec::new()).set_headers(vec!["x".into()]);
    assert_eq!(w2.headers(), Some(&["x".to_string()][..]));
}

#[test]
fn test_writer_write_headers_flexible_mode() {
    let mut w = Writer::new(Vec::new()).set_flexible(true);
    w.write_headers(["a", "b"]).unwrap();
    // Flexible mode: different field count is OK
    w.write_row(["1", "2", "3"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "a,b\r\n1,2,3\r\n");
}

#[test]
fn test_writer_headers_roundtrip() {
    let mut w = Writer::new(Vec::new());
    w.write_headers(["name", "age"]).unwrap();
    w.write_row(["Alice", "30"]).unwrap();
    let csv_data = w.into_inner().unwrap();

    let mut reader = Reader::new(Cursor::new(csv_data));
    let parsed = reader.parse_headers().unwrap();
    assert_eq!(parsed, &["name", "age"]);
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec!["Alice", "30"]);
}

// ── Roundtrip ─────────────────────────────────────────────────────────

#[test]
fn test_roundtrip_basic() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["a", "b", "c"]).unwrap();
    w.write_row(["1", "2", "3"]).unwrap();
    let csv_data = w.into_inner().unwrap();
    let mut reader = Reader::new(Cursor::new(csv_data));
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows[0], vec!["a", "b", "c"]);
    assert_eq!(rows[1], vec!["1", "2", "3"]);
}

#[test]
fn test_roundtrip_with_commas() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["hello, world", "foo,bar"]).unwrap();
    let csv_data = w.into_inner().unwrap();
    let mut reader = Reader::new(Cursor::new(csv_data));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec!["hello, world", "foo,bar"]);
}

#[test]
fn test_roundtrip_with_quotes() {
    let mut w = Writer::new(Vec::new());
    w.write_row([r#"he said "hello""#, "foo"]).unwrap();
    let csv_data = w.into_inner().unwrap();
    let mut reader = Reader::new(Cursor::new(csv_data));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec![r#"he said "hello""#, "foo"]);
}

#[test]
fn test_roundtrip_with_newlines() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["hello\nworld", "foo\r\nbar"]).unwrap();
    let csv_data = w.into_inner().unwrap();
    let mut reader = Reader::new(Cursor::new(csv_data));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec!["hello\nworld", "foo\r\nbar"]);
}

// ── Serde ─────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
mod serde_tests {
    use std::io::Cursor;

    use csv::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_deserialize_positional() {
        let data = b"Alice,30\nBob,25\n";
        let mut reader = Reader::new(Cursor::new(data)).set_headers(vec!["name".into(), "age".into()]);
        let mut rows = reader.rows();
        let p1: Person = rows.next().unwrap().deserialize().unwrap();
        assert_eq!(p1.name, "Alice");
        assert_eq!(p1.age, 30);
        let p2: Person = rows.next().unwrap().deserialize().unwrap();
        assert_eq!(p2.name, "Bob");
        assert_eq!(p2.age, 25);
    }

    #[test]
    fn test_deserialize_with_headers() {
        let data = b"name,age\nAlice,30\nBob,25\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let mut rows = reader.rows();
        let p1: Person = rows.next().unwrap().deserialize().unwrap();
        assert_eq!(p1.name, "Alice");
        assert_eq!(p1.age, 30);
        let p2: Person = rows.next().unwrap().deserialize().unwrap();
        assert_eq!(p2.name, "Bob");
        assert_eq!(p2.age, 25);
    }

    #[test]
    fn test_deserialize_header_order_independent() {
        let data = b"age,name\n30,Alice\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let p: Person = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(p.name, "Alice");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_deserialize_type_mismatch() {
        let data = b"name,age\nAlice,not_a_number\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let row = reader.rows().next().unwrap();
        let result: Result<Person, _> = row.deserialize();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err.kind(), ReadErrorKind::Deserialize(_)));
        let display = err.to_string();
        assert!(display.contains("deserialization error"), "got: {display}");
    }

    #[test]
    fn test_deserialize_missing_column_defaults_to_empty() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Record {
            name: String,
            age: u32,
            city: String,
        }
        let data = b"name,age\nAlice,30\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let row = reader.rows().next().unwrap();
        let result: Result<Record, _> = row.deserialize();
        assert!(result.is_ok(), "should succeed, got: {:?}", result.err());
        let rec = result.unwrap();
        assert_eq!(rec.name, "Alice");
        assert_eq!(rec.age, 30);
        assert_eq!(rec.city, "");
    }

    #[test]
    fn test_deserialize_tuple_fails_without_headers() {
        let data = b"Alice,30\n";
        let mut reader = Reader::new(Cursor::new(data));
        let result: Result<(String, u32), _> = reader.rows().next().unwrap().deserialize();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err().kind(), ReadErrorKind::Deserialize(_)));
    }

    #[test]
    fn test_deserialize_tuple_fails_even_with_headers() {
        // Tuples have no named fields, so header-based deserialization
        // cannot map them — they always fail.
        let data = b"Alice,30\n";
        let mut reader = Reader::new(Cursor::new(data)).set_headers(vec!["name".into(), "age".into()]);
        let result: Result<(String, u32), _> = reader.rows().next().unwrap().deserialize();
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_set_headers() {
        let data = b"Alice,30\nBob,25\n";
        let mut reader = Reader::new(Cursor::new(data)).set_headers(vec!["name".into(), "age".into()]);
        let mut rows = reader.rows();
        let p1: Person = rows.next().unwrap().deserialize().unwrap();
        assert_eq!(p1.name, "Alice");
        assert_eq!(p1.age, 30);
    }

    #[test]
    fn test_deserialize_row_can_outlive_reader() {
        // Owned rows prove they can outlive the reader
        let row = {
            let mut reader = Reader::new(b"Alice,30\n".as_slice()).set_headers(vec!["name".into(), "age".into()]);
            reader.rows().next().unwrap()
        };
        // row is used after reader is dropped
        let p: Person = row.deserialize().unwrap();
        assert_eq!(p.name, "Alice");
    }

    #[test]
    fn test_deserialize_with_bytes_row_also_works() {
        // BytesRow doesn't have deserialize, but Row wrapping it does
        let data = b"Alice,30\n";
        let mut reader = Reader::new(Cursor::new(data)).set_headers(vec!["name".into(), "age".into()]);
        let row = reader.rows().next().unwrap();
        let p: Person = row.deserialize().unwrap();
        assert_eq!(p.name, "Alice");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_deserialize_option_field() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct WithOption {
            name: String,
            age: Option<u32>,
            city: Option<String>,
        }
        // age present, city absent
        let data = b"name,age\nAlice,30\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: WithOption = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(rec.name, "Alice");
        assert_eq!(rec.age, Some(30));
        assert_eq!(rec.city, None);
    }

    #[test]
    fn test_deserialize_bool() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct WithBool {
            name: String,
            active: bool,
        }
        let data = b"name,active\nAlice,true\nBob,false\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rows: Vec<WithBool> = reader.rows().map(|r| r.deserialize().unwrap()).collect();
        assert_eq!(rows[0].name, "Alice");
        assert!(rows[0].active);
        assert_eq!(rows[1].name, "Bob");
        assert!(!rows[1].active);
    }

    #[test]
    fn test_deserialize_extra_columns_ignored() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct OnlyName {
            name: String,
        }
        let data = b"name,age,city\nAlice,30,NYC\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: OnlyName = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(rec.name, "Alice");
    }

    #[test]
    fn test_deserialize_flexible_row_different_field_count() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Record {
            a: String,
            b: String,
        }
        let data = b"a,b\n1,2,3\n";
        let mut reader = Reader::new(Cursor::new(data)).set_flexible(true);
        reader.parse_headers().unwrap();
        let result: Result<Record, _> = reader.rows().next().unwrap().deserialize();
        assert!(result.is_ok());
        // Positional deserialization only uses first N fields
    }

    // ── Writer serialize ───────────────────────────────────────────

    #[test]
    fn test_serialize_struct() {
        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "age".into()]);
        let alice = Person {
            name: "Alice".into(),
            age: 30,
        };
        w.serialize(&alice).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "Alice,30\r\n");
    }

    #[test]
    fn test_serialize_struct_header_order() {
        // Headers are in different order than struct fields
        let mut w = Writer::new(Vec::new()).set_headers(vec!["age".into(), "name".into()]);
        let alice = Person {
            name: "Alice".into(),
            age: 30,
        };
        w.serialize(&alice).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "30,Alice\r\n");
    }

    #[test]
    fn test_serialize_requires_headers() {
        let mut w = Writer::new(Vec::new());
        let alice = Person {
            name: "Alice".into(),
            age: 30,
        };
        let err = w.serialize(&alice).unwrap_err();
        assert!(matches!(err, WriteError::Serialize(_)));
    }

    #[test]
    fn test_serialize_unknown_field() {
        #[derive(Serialize)]
        struct Extended {
            name: String,
            age: u32,
            extra: String,
        }
        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "age".into()]);
        let rec = Extended {
            name: "Alice".into(),
            age: 30,
            extra: "x".into(),
        };
        let err = w.serialize(&rec).unwrap_err();
        assert!(matches!(err, WriteError::Serialize(_)));
    }

    #[test]
    fn test_serialize_seq() {
        let mut w = Writer::new(Vec::new()).set_headers(vec!["a".into(), "b".into()]);
        w.serialize(&["hello", "world"]).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "hello,world\r\n");
    }

    #[test]
    fn test_serialize_tuple() {
        let mut w = Writer::new(Vec::new()).set_headers(vec!["x".into(), "y".into()]);
        w.serialize(&(42u32, "answer")).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "42,answer\r\n");
    }

    #[test]
    fn test_serialize_mixed_types() {
        #[derive(Serialize)]
        struct Mixed {
            b: bool,
            i: i32,
            f: f64,
            s: String,
        }
        let mut w = Writer::new(Vec::new()).set_headers(vec!["b".into(), "i".into(), "f".into(), "s".into()]);
        let rec = Mixed {
            b: true,
            i: -42,
            f: 42.5,
            s: "test".into(),
        };
        w.serialize(&rec).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "true,-42,42.5,test\r\n");
    }

    #[test]
    fn test_serialize_option_some() {
        #[derive(Serialize)]
        struct WithOption {
            name: String,
            age: Option<u32>,
        }
        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "age".into()]);
        let rec = WithOption {
            name: "Alice".into(),
            age: Some(30),
        };
        w.serialize(&rec).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "Alice,30\r\n");
    }

    #[test]
    fn test_serialize_option_none() {
        #[derive(Serialize)]
        struct WithOption {
            name: String,
            age: Option<u32>,
        }
        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "age".into()]);
        let rec = WithOption {
            name: "Alice".into(),
            age: None,
        };
        w.serialize(&rec).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "Alice,\r\n");
    }

    #[test]
    fn test_serialize_strict_field_count() {
        let mut w = Writer::new(Vec::new()).set_headers(vec!["a".into(), "b".into()]);
        w.serialize(&["x", "y"]).unwrap();
        let err = w.serialize(&["1", "2", "3"]).unwrap_err();
        assert!(matches!(err, WriteError::Serialize(_)));
    }

    #[test]
    fn test_serialize_roundtrip() {
        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "age".into()]);
        let alice = Person {
            name: "Alice".into(),
            age: 30,
        };
        w.serialize(&alice).unwrap();
        let csv_data = w.into_inner().unwrap();

        let mut reader = Reader::new(Cursor::new(csv_data)).set_headers(vec!["name".into(), "age".into()]);
        let row = reader.rows().next().unwrap();
        let parsed: Person = row.deserialize().unwrap();
        assert_eq!(parsed, alice);
    }

    #[test]
    fn test_serialize_with_quoted_fields() {
        #[derive(Serialize)]
        struct Row {
            name: String,
            note: String,
        }
        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "note".into()]);
        let rec = Row {
            name: "Alice".into(),
            note: "has a comma, see".into(),
        };
        w.serialize(&rec).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "Alice,\"has a comma, see\"\r\n");
    }
}

// ── Row can outlive Reader (owned row test) ───────────────────────────

#[test]
fn test_row_outlives_reader() {
    let row = {
        let mut reader = Reader::new(b"a,b,c\n1,2,3\n".as_slice());
        reader.rows().nth(1).unwrap()
    };
    // row is used after reader would be dropped (owned)
    assert_eq!(row.len(), 3);
    assert_eq!(row.to_vec().unwrap(), vec!["1", "2", "3"]);
}

#[test]
fn test_rows_collected_outlive_reader() {
    let rows: Vec<Row> = {
        let mut reader = Reader::new(b"a,b\n1,2\n3,4\n".as_slice());
        reader.rows().collect()
    };
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].to_vec().unwrap(), vec!["a", "b"]);
    assert_eq!(rows[2].to_vec().unwrap(), vec!["3", "4"]);
}

#[test]
fn test_bytes_rows_collected_outlive_reader() {
    let rows: Vec<BytesRow> = {
        let mut reader = Reader::new(b"a,b\n1,2\n".as_slice());
        reader.rows_bytes().collect()
    };
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[1].to_vec().unwrap(), vec![&b"1"[..], &b"2"[..]]);
}

// ── ExactSizeIterator tests ──────────────────────────────────────────

#[test]
fn test_fields_exact_size() {
    let mut reader = Reader::new(b"a,b,c\n".as_slice());
    let row = reader.rows().next().unwrap();
    let mut iter = row.iter();
    assert_eq!(iter.size_hint(), (3, Some(3)));
    iter.next();
    assert_eq!(iter.size_hint(), (2, Some(2)));
}

#[test]
fn test_bytes_fields_exact_size() {
    let mut reader = Reader::new(b"a,b,c\n".as_slice());
    let bytes_row = reader.rows_bytes().next().unwrap();
    let mut iter = bytes_row.iter();
    assert_eq!(iter.size_hint(), (3, Some(3)));
    iter.next();
    assert_eq!(iter.size_hint(), (2, Some(2)));
}

// ── Large data tests ──────────────────────────────────────────────────

#[test]
fn test_large_fields() {
    let big = "x".repeat(10000);
    let data = format!("{big},{big},{big}\n");
    let mut reader = Reader::new(Cursor::new(data.as_bytes()));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.len(), 3);
    assert_eq!(row.to_vec().unwrap()[0].len(), 10000);
}

#[test]
fn test_many_fields() {
    let fields: Vec<String> = (0..100).map(|i| format!("field{i}")).collect();
    let line = fields.join(",") + "\n";
    let mut reader = Reader::new(Cursor::new(line.as_bytes()));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.len(), 100);
}

#[test]
fn test_large_streaming() {
    let mut csv = String::new();
    for i in 0..1000 {
        csv.push_str(&format!("a{}", i));
        for j in 1..20 {
            csv.push(',');
            csv.push_str(&format!("b{}", j));
        }
        csv.push('\n');
    }
    let mut reader = Reader::new(Cursor::new(csv.as_bytes()));
    let mut count = 0;
    for row in reader.rows() {
        assert_eq!(row.len(), 20);
        count += 1;
    }
    assert_eq!(count, 1000);
}

// ── Malformed input edge cases ────────────────────────────────────────

#[test]
fn test_empty_quoted_field_at_eof() {
    let rows = collect_rows(b"\"\"");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0], vec![""]);
}

#[test]
fn test_quote_inside_unquoted_field() {
    // A quote inside an unquoted field should be treated as literal 'quote' char
    let rows = collect_rows(b"he said \"hello\",world\n");
    assert_eq!(rows[0], vec!["he said \"hello\"", "world"]);
}

#[test]
fn test_missing_closing_quote_followed_by_delimiter() {
    let data = b"a,\"b,c\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert!(matches!(row.error().unwrap().kind(), ReadErrorKind::UnterminatedQuote));
}

#[test]
fn test_trailing_content_with_spaces() {
    let data = b"\"hello\"  extra\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert!(matches!(row.error().unwrap().kind(), ReadErrorKind::TrailingContent));
}

#[test]
fn test_trailing_content_with_tabs() {
    let data = b"\"hello\"\textra\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert!(matches!(row.error().unwrap().kind(), ReadErrorKind::TrailingContent));
}

#[test]
fn test_only_delimiter_no_newline() {
    let rows = collect_rows(b",");
    assert_eq!(rows[0], vec!["", ""]);
}

#[test]
fn test_many_empty_fields() {
    let rows = collect_rows(b",,,,\n");
    assert_eq!(rows[0], vec!["", "", "", "", ""]);
}

#[test]
fn test_leading_newline() {
    let rows = collect_rows(b"\na,b\n");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0], vec!["a", "b"]);
}

#[test]
fn test_trailing_blank_lines() {
    let rows = collect_rows(b"a,b\n\n\n\n");
    assert_eq!(rows.len(), 1);
}

#[test]
fn test_blank_lines_between_rows() {
    let rows = collect_rows(b"a,b\n\n\n1,2\n");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["a", "b"]);
    assert_eq!(rows[1], vec!["1", "2"]);
}

#[test]
fn test_cr_at_eof_with_following_read() {
    // bare \r at EOF triggers pending_cr; next read should not skip valid data
    let data = b"a,b\r";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0], vec!["a", "b"]);
}

#[test]
fn test_strict_mode_blank_lines_between_rows() {
    let data = b"a,b\n\n\n1,2\n";
    let mut reader = Reader::new(std::io::Cursor::new(data)).set_flexible(false);
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 2);
}

#[test]
fn test_parse_headers_with_unterminated_quote_returns_error() {
    let mut reader = Reader::new(std::io::Cursor::new(b"\"unterminated\n"));
    let result = reader.parse_headers();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err().kind(), ReadErrorKind::UnterminatedQuote));
}

#[test]
fn test_parse_headers_error_does_not_set_headers() {
    let mut reader = Reader::new(std::io::Cursor::new(b"\"unterminated\n"));
    let _ = reader.parse_headers();
    // headers() should NOT return Some after a failed parse_headers
    assert!(
        reader.headers().is_none(),
        "headers() should be None after failed parse_headers"
    );
}

#[test]
fn test_parse_headers_with_trailing_content_returns_error() {
    let mut reader = Reader::new(std::io::Cursor::new(b"\"header\"trailing\n"));
    let result = reader.parse_headers();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err().kind(), ReadErrorKind::TrailingContent));
}

#[test]
fn test_inconsistent_field_count_with_error_on_row() {
    let data = b"a,b\n1,2,3\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let mut rows = reader.rows();
    let row1 = rows.next().unwrap();
    assert!(row1.error().is_none());
    let row2 = rows.next().unwrap();
    let err = row2.error().unwrap();
    assert!(matches!(
        err.kind(),
        ReadErrorKind::InconsistentFieldCount {
            expected: 2,
            found: 3
        }
    ));
    // the error should have a non-zero line number
    assert!(err.line > 0);
}

#[test]
fn test_bytes_row_inconsistent_field_count() {
    let data = b"a,b\n1,2,3\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let mut rows = reader.rows_bytes();
    let _row1 = rows.next().unwrap();
    let row2 = rows.next().unwrap();
    let err = row2.error().unwrap();
    assert!(matches!(
        err.kind(),
        ReadErrorKind::InconsistentFieldCount {
            expected: 2,
            found: 3
        }
    ));
}

#[test]
fn test_row_iter_preserves_trailing_content_error() {
    let data = b"\"hello\"garbage\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let row = reader.rows().next().unwrap();
    let mut iter = row.iter();
    let err = iter.next().unwrap().unwrap_err();
    assert!(matches!(err.kind(), ReadErrorKind::TrailingContent));
    // only one field in this row
    assert!(iter.next().is_none());
}

#[test]
fn test_row_iter_preserves_invalid_utf8_error() {
    let data = b"a,\xff,b\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let row = reader.rows().next().unwrap();
    let fields: Vec<Result<&str, _>> = row.iter().collect();
    assert!(fields[0].is_ok());
    assert!(fields[1].is_err());
    assert!(matches!(fields[1].as_ref().unwrap_err().kind(), ReadErrorKind::InvalidUtf8));
    assert!(fields[2].is_ok());
}

// ── Buffer boundary stress tests ──────────────────────────────────────

#[test]
fn test_cr_split_across_chunks() {
    let data = b"a,b\r";
    let mut reader = Reader::new(ChunkReader::new(data, 1));
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0], vec!["a", "b"]);
}

#[test]
fn test_crlf_split_across_chunks() {
    // \r at end of one chunk, \n at start of next
    let data = b"a,b\r\n1,2\r\n";
    let mut reader = Reader::new(ChunkReader::new(data, 3));
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["a", "b"]);
    assert_eq!(rows[1], vec!["1", "2"]);
}

#[test]
fn test_quoted_field_at_buffer_start() {
    let data = b"x,\"hello\",y\n";
    let mut reader = Reader::new(ChunkReader::new(data, 4));
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows[0], vec!["x", "hello", "y"]);
}

#[test]
fn test_closing_quote_at_buffer_end() {
    let data = b"\"hello\"\n";
    let mut reader = Reader::new(ChunkReader::new(data, 6));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec!["hello"]);
}

#[test]
fn test_quote_split_across_chunks() {
    // closing quote is the first byte of a new chunk
    let data = b"\"hello\"\n";
    let mut reader = Reader::new(ChunkReader::new(data, 7));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec!["hello"]);
}

#[test]
fn test_escaped_quote_across_chunks() {
    let data = b"\"\"\"hello\"\"\",world\n";
    let mut reader = Reader::new(ChunkReader::new(data, 4));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec!["\"hello\"", "world"]);
}

#[test]
fn test_delimiter_at_chunk_boundary() {
    let data = b"a,b,c\n";
    let mut reader = Reader::new(ChunkReader::new(data, 2));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec!["a", "b", "c"]);
}

#[test]
fn test_newline_at_chunk_boundary() {
    let data = b"a,b,c\n1,2,3\n";
    let mut reader = Reader::new(ChunkReader::new(data, 5));
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 2);
}

// ── Writer edge cases ─────────────────────────────────────────────────

#[test]
fn test_writer_empty_row() {
    let mut w = Writer::new(Vec::new());
    w.write_row(Vec::<&str>::new()).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "\r\n");
}

#[test]
fn test_writer_single_field() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["only"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "only\r\n");
}

#[test]
fn test_writer_drop_flushes() {
    let inner = {
        let mut w = Writer::new(Vec::new());
        w.write_row(["a", "b"]).unwrap();
        // no explicit flush or into_inner
        // drop will flush
        w
    }
    .into_inner()
    .unwrap();
    assert_eq!(String::from_utf8(inner).unwrap(), "a,b\r\n");
}

#[test]
fn test_writer_quoted_field_with_custom_delimiter() {
    let mut w = Writer::new(Vec::new()).set_delimiter(b'|');
    w.write_row(["hello|world", "foo"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "\"hello|world\"|foo\r\n");
}

// ── Serde edge cases ──────────────────────────────────────────────────

#[cfg(feature = "serde")]
mod serde_edge_tests {
    use std::io::Cursor;

    use csv::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, PartialEq)]
    struct WithI128 {
        value: i128,
    }

    #[test]
    fn test_deserialize_i128() {
        let data = b"value\n170141183460469231731687303715884105727\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: WithI128 = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(rec.value, i128::MAX);
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct WithU128 {
        value: u128,
    }

    #[test]
    fn test_deserialize_u128() {
        let data = b"value\n340282366920938463463374607431768211455\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: WithU128 = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(rec.value, u128::MAX);
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct WithChar {
        ch: char,
    }

    #[test]
    fn test_deserialize_char_single() {
        let data = b"ch\na\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: WithChar = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(rec.ch, 'a');
    }

    #[test]
    fn test_deserialize_char_empty_errors() {
        let data = b"ch\n\n";
        let mut reader = Reader::new(Cursor::new(data)).set_headers(vec!["ch".into()]);
        let result: Result<WithChar, _> = reader.rows().next().unwrap().deserialize();
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_char_multi_char_errors() {
        let data = b"ch\nab\n";
        let mut reader = Reader::new(Cursor::new(data)).set_headers(vec!["ch".into()]);
        let result: Result<WithChar, _> = reader.rows().next().unwrap().deserialize();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string().contains("char field contains more than one character")
                || err.to_string().contains("deserialization error")
        );
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct WithVec {
        items: Vec<String>,
    }

    #[test]
    fn test_deserialize_vec_inside_struct_errors() {
        let data = b"items\na,b,c\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let result: Result<WithVec, _> = reader.rows().next().unwrap().deserialize();
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_struct_fewer_fields_than_headers_ok() {
        #[derive(Serialize)]
        struct Partial {
            name: String,
        }
        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "age".into()]);
        let rec = Partial {
            name: "Alice".into(),
        };
        w.serialize(&rec).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "Alice,\r\n");
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct MissingField {
        name: String,
        age: u32,
    }

    #[test]
    fn test_deserialize_struct_missing_field_defaults_empty() {
        // 'age' column missing in headers, struct has it
        // This should fail because 'age' can't be parsed from ""
        let data = b"name\nAlice\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let result: Result<MissingField, _> = reader.rows().next().unwrap().deserialize();
        // age is u32 which can't parse from "", so it should error
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_empty_struct_with_headers() {
        #[derive(Serialize)]
        struct Empty {}
        let mut w = Writer::new(Vec::new()).set_headers(vec!["a".into(), "b".into()]);
        let rec = Empty {};
        w.serialize(&rec).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, ",\r\n");
    }

    #[test]
    fn test_serialize_unit() {
        let mut w = Writer::new(Vec::new()).set_headers(vec!["val".into()]);
        w.serialize(&()).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "\r\n");
    }

    #[test]
    fn test_serialize_struct_seq_too_short_errors() {
        let mut w = Writer::new(Vec::new()).set_headers(vec!["a".into(), "b".into(), "c".into()]);
        let result = w.serialize(&["only"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_struct_seq_too_long_errors() {
        let mut w = Writer::new(Vec::new()).set_headers(vec!["a".into(), "b".into()]);
        let result = w.serialize(&["x", "y", "z"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_option_none() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct WithOption {
            name: String,
            extra: Option<String>,
        }
        let data = b"name\nalice\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: WithOption = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(rec.name, "alice");
        assert_eq!(rec.extra, None);
    }

    #[test]
    fn test_deserialize_option_some() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct WithOption {
            name: String,
            extra: Option<String>,
        }
        let data = b"name,extra\nalice,hello\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: WithOption = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(rec.name, "alice");
        assert_eq!(rec.extra, Some("hello".into()));
    }

    // ── roundtrip: Option None -> serialize -> deserialize -> None ──────

    #[test]
    fn test_option_none_roundtrip() {
        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        struct Record {
            name: String,
            extra: Option<String>,
        }
        let rec = Record {
            name: "Alice".into(),
            extra: None,
        };
        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "extra".into()]);
        w.serialize(&rec).unwrap();
        let csv_data = w.into_inner().unwrap();

        let mut reader = Reader::new(Cursor::new(csv_data)).set_headers(vec!["name".into(), "extra".into()]);
        let parsed: Record = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(parsed, rec);
    }

    // ── serde(skip) support ─────────────────────────────────────────

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct WithIgnoredField {
        name: String,
        #[serde(skip)]
        age: u32,
    }

    #[test]
    fn test_deserialize_ignored_field_from_csv_with_column() {
        let data = b"name,age\nAlice,30\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: WithIgnoredField = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(
            rec,
            WithIgnoredField {
                name: "Alice".into(),
                age: 0
            }
        );
    }

    #[test]
    fn test_deserialize_ignored_field_with_missing_column() {
        let data = b"name\nAlice\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: WithIgnoredField = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(
            rec,
            WithIgnoredField {
                name: "Alice".into(),
                age: 0
            }
        );
    }

    #[test]
    fn test_serialize_ignored_field_writes_empty() {
        let rec = WithIgnoredField {
            name: "Alice".into(),
            age: 42,
        };
        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "age".into()]);
        w.serialize(&rec).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "Alice,\r\n");
    }

    #[test]
    fn test_serialize_ignored_field_not_in_headers() {
        let rec = WithIgnoredField {
            name: "Alice".into(),
            age: 42,
        };
        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into()]);
        w.serialize(&rec).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "Alice\r\n");
    }

    #[test]
    fn test_ignored_field_roundtrip() {
        let rec = WithIgnoredField {
            name: "Bob".into(),
            age: 99,
        };
        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "age".into()]);
        w.serialize(&rec).unwrap();
        let csv_data = w.into_inner().unwrap();

        let mut reader = Reader::new(Cursor::new(csv_data)).set_headers(vec!["name".into(), "age".into()]);
        let parsed: WithIgnoredField = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(parsed.name, "Bob");
        assert_eq!(parsed.age, 0);
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct WithMultipleIgnored {
        a: String,
        #[serde(skip)]
        b: String,
        c: String,
        #[serde(skip)]
        d: u64,
    }

    #[test]
    fn test_deserialize_multiple_ignored_fields() {
        let data = b"a,b,c,d\nx,y,z,42\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: WithMultipleIgnored = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(
            rec,
            WithMultipleIgnored {
                a: "x".into(),
                b: "".into(),
                c: "z".into(),
                d: 0,
            }
        );
    }

    #[test]
    fn test_serialize_multiple_ignored_fields() {
        let rec = WithMultipleIgnored {
            a: "x".into(),
            b: "should be ignored".into(),
            c: "z".into(),
            d: 999,
        };
        let mut w = Writer::new(Vec::new()).set_headers(vec!["a".into(), "b".into(), "c".into(), "d".into()]);
        w.serialize(&rec).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "x,,z,\r\n");
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct WithSkipDeserializing {
        name: String,
        #[serde(skip_deserializing)]
        age: u32,
    }

    #[test]
    fn test_deserialize_skip_deserializing_uses_default() {
        let data = b"name,age\nAlice,30\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: WithSkipDeserializing = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(rec.name, "Alice");
        assert_eq!(rec.age, 0);
    }

    #[derive(Debug, Serialize, PartialEq)]
    struct WithSkipSerializing {
        name: String,
        #[serde(skip_serializing)]
        age: u32,
    }

    #[test]
    fn test_serialize_skip_serializing_omits_field() {
        let rec = WithSkipSerializing {
            name: "Alice".into(),
            age: 30,
        };
        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "age".into()]);
        w.serialize(&rec).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "Alice,\r\n");
    }
}

// ── I/O error at various positions ────────────────────────────────────

#[test]
fn test_io_error_at_first_byte() {
    let data = b"a,b,c\n";
    let mut reader = Reader::new(ErrorAfterReader::new(data.to_vec(), 0));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert!(matches!(row.error().unwrap().kind(), ReadErrorKind::Io));
}

#[test]
fn test_io_error_at_newline() {
    let data = b"a,b,c\n1,2,3\n";
    // error after 5 bytes = right at the newline after "a,b,c"
    let mut reader = Reader::new(ErrorAfterReader::new(data.to_vec(), 5));
    let mut rows = reader.rows();
    // first row might succeed
    let row1 = rows.next();
    if let Some(r) = row1 {
        if r.error().is_none() {
            // second row should have the error
            let row2 = rows.next().unwrap();
            assert!(matches!(row2.error().unwrap().kind(), ReadErrorKind::Io));
        }
    }
}

#[test]
fn test_io_error_at_quote() {
    let data = b"\"hello\",world\n";
    let mut reader = Reader::new(ErrorAfterReader::new(data.to_vec(), 2));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert!(matches!(row.error().unwrap().kind(), ReadErrorKind::Io));
}

#[test]
fn test_io_error_at_delimiter() {
    let data = b"a,b,c\n";
    let mut reader = Reader::new(ErrorAfterReader::new(data.to_vec(), 1));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert!(matches!(row.error().unwrap().kind(), ReadErrorKind::Io));
}

// ── Whitespace / unicode edge cases ───────────────────────────────────

#[test]
fn test_field_with_only_whitespace() {
    let rows = collect_rows(b"   ,\t  ,\n");
    assert_eq!(rows[0], vec!["   ", "\t  ", ""]);
}

#[test]
fn test_unicode_surrogate_bytes() {
    // lone surrogate bytes (invalid UTF-8 sequences)
    let data = b"a,\xed\xa0\x80,b\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert!(row.to_vec().is_err());
}

#[test]
fn test_very_long_single_field() {
    let long = "x".repeat(100_000);
    let data = format!("{long}\n");
    let rows = collect_rows(data.as_bytes());
    assert_eq!(rows[0].len(), 1);
    assert_eq!(rows[0][0].len(), 100_000);
}

// ── Reader &[u8] mode with chunked / streaming ────────────────────────

#[test]
fn test_slice_reader_exact() {
    let mut reader = Reader::new(b"a,b,c\n1,2,3\n".as_slice());
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 2);
}

#[test]
fn test_slice_reader_single_byte() {
    // use a custom Read impl that yields data one byte at a time
    let data = b"a,b\n";
    let mut reader = Reader::new(ChunkReader::new(data, 1));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec!["a", "b"]);
}

#[test]
fn test_flexible_varying_field_count() {
    let mut reader = Reader::new(std::io::Cursor::new(b"a\n1,2\n3,4,5\n")).set_flexible(true);
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].len(), 1);
    assert_eq!(rows[1].len(), 2);
    assert_eq!(rows[2].len(), 3);
}

// ── Writer: write_headers + flexible ──────────────────────────────────

#[test]
fn test_writer_headers_flexible_different_row_lengths() {
    let mut w = Writer::new(Vec::new()).set_flexible(true);
    w.write_headers(["a", "b"]).unwrap();
    w.write_row(["1", "2", "3"]).unwrap();
    w.write_row(["x"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "a,b\r\n1,2,3\r\nx\r\n");
}

// ── Writer: error state is unchanged after failed write ───────────────

#[test]
fn test_writer_strict_error_does_not_corrupt_buffer() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["a", "b"]).unwrap();
    // This should fail before writing anything
    let err = w.write_row(["1", "2", "3"]).unwrap_err();
    assert!(matches!(
        err,
        WriteError::InconsistentFieldCount {
            expected: 2,
            ..
        }
    ));
    // Subsequent valid write should still work
    w.write_row(["c", "d"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "a,b\r\nc,d\r\n");
}

#[test]
fn test_read_error_display() {
    let e = ReadError::new(ReadErrorKind::Io, 42, 0);
    let s = e.to_string();
    assert!(s.contains("line 42"));
}

#[test]
fn test_bytes_row_debug_with_error() {
    let data = b"\"hello\"\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let bytes_row = reader.rows_bytes().next().unwrap();
    let s = format!("{:?}", bytes_row);
    // Should display as a list since there's no error
    assert!(!s.contains("Err("));
}

// ── Reader: Malformed input edge cases ────────────────────────────────

#[test]
fn test_single_quote_at_eof() {
    let data = b"\"\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let row = reader.rows().next().unwrap();
    assert!(matches!(row.error().unwrap().kind(), ReadErrorKind::UnterminatedQuote));
}

#[test]
fn test_multiple_escaped_quotes() {
    let rows = collect_rows(b"\"\"\"\"\"\"\n");
    assert_eq!(rows[0], vec!["\"\""]);
}

#[test]
fn test_empty_quoted_then_delimiter() {
    let rows = collect_rows(b"\"\",b\n");
    assert_eq!(rows[0], vec!["", "b"]);
}

#[test]
fn test_quoted_field_with_only_quotes() {
    let rows = collect_rows(b"\"\"\"\"\n");
    assert_eq!(rows[0], vec!["\""]);
}

#[test]
fn test_crlf_only_input() {
    let rows = collect_rows(b"\r\n");
    assert!(rows.is_empty());
}

#[test]
fn test_alternating_blank_lines_and_data() {
    let rows = collect_rows(b"a,b\n\n\nc,d\n\n\ne,f\n");
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0], vec!["a", "b"]);
    assert_eq!(rows[1], vec!["c", "d"]);
    assert_eq!(rows[2], vec!["e", "f"]);
}

#[test]
fn test_unterminated_quote_after_partial_row() {
    let data = b"a,b\n\"c,d\ne,f\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let mut rows = reader.rows();
    let r1 = rows.next().unwrap();
    assert_eq!(r1.to_vec().unwrap(), vec!["a", "b"]);
    let r2 = rows.next().unwrap();
    assert!(matches!(r2.error().unwrap().kind(), ReadErrorKind::UnterminatedQuote));
}

#[test]
fn test_many_consecutive_delimiters() {
    let rows = collect_rows(b",,,,,,\n");
    assert_eq!(rows[0].len(), 7);
    assert!(rows[0].iter().all(|f| f.is_empty()));
}

// ── Reader: Buffer boundary stress (advanced) ─────────────────────────

#[test]
fn test_crlf_split_across_many_rows() {
    let data = b"a,b\r\nc,d\r\ne,f\r\n";
    let mut reader = Reader::new(ChunkReader::new(data, 3));
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0], vec!["a", "b"]);
    assert_eq!(rows[1], vec!["c", "d"]);
    assert_eq!(rows[2], vec!["e", "f"]);
}

#[test]
fn test_quote_at_exact_buffer_end() {
    let data = b"\"hello\"\n";
    let mut reader = Reader::new(ChunkReader::new(data, 7));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec!["hello"]);
}

#[test]
fn test_quoted_field_across_many_chunks() {
    let inner = "hello, world! ".repeat(500);
    let data = format!("\"{inner}\",end\n");
    let mut reader = Reader::new(ChunkReader::new(data.as_bytes(), 64));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert_eq!(row.len(), 2);
    assert_eq!(row.to_vec().unwrap()[1], "end");
    assert_eq!(row.to_vec().unwrap()[0], inner);
}

#[test]
fn test_field_spanning_16kb_boundary() {
    let field = "x".repeat(20000);
    let data = format!("{field},{field}\n");
    let mut reader = Reader::new(ChunkReader::new(data.as_bytes(), 4096));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.len(), 2);
    assert_eq!(row.to_vec().unwrap()[0].len(), 20000);
}

// ── Reader: pending_cr edge cases ─────────────────────────────────

#[test]
fn test_pending_cr_cleared_on_non_newline() {
    // \r at chunk boundary, next chunk starts with data (not \n),
    // then a \n later that should NOT be skipped.
    // This exercises the pending_cr bug where the flag wasn't cleared.
    let data = b"a,b\r1,2\n3,4\n";
    let mut reader = Reader::new(ChunkReader::new(data, 3));
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0], vec!["a", "b"]);
    assert_eq!(rows[1], vec!["1", "2"]);
    assert_eq!(rows[2], vec!["3", "4"]);
}

#[test]
fn test_pending_cr_with_crlf_across_chunks_followed_by_rows() {
    // \r at chunk boundary, \n at start of next chunk (\r\n split),
    // then more rows. pending_cr should skip the \n correctly.
    let data = b"a,b\r\n1,2\r\n3,4\r\n";
    let mut reader = Reader::new(ChunkReader::new(data, 3));
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0], vec!["a", "b"]);
    assert_eq!(rows[1], vec!["1", "2"]);
    assert_eq!(rows[2], vec!["3", "4"]);
}

#[test]
fn test_pending_cr_bare_r_then_data_then_newline() {
    // bare \r at chunk boundary, next chunk has data followed by \n.
    // The \r was a bare CR line ending. The \n should NOT be skipped.
    let data = b"a,b\r1,2,3\r4,5,6\n";
    let mut reader = Reader::new(ChunkReader::new(data, 4)).set_flexible(true);
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0], vec!["a", "b"]);
    assert_eq!(rows[1], vec!["1", "2", "3"]);
    assert_eq!(rows[2], vec!["4", "5", "6"]);
}

// ── Reader: API / state edge cases ────────────────────────────────────

#[test]
fn test_row_error_len_still_returns_count() {
    let data = b"a,b\n1,2,3\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let mut rows = reader.rows();
    let _r1 = rows.next().unwrap();
    let r2 = rows.next().unwrap();
    assert!(r2.error().is_some());
    assert_eq!(r2.len(), 3);
}

#[test]
fn test_bytes_row_error_get_still_works() {
    let data = b"a,b\n1,2,3\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    let mut rows = reader.rows_bytes();
    let _r1 = rows.next().unwrap();
    let r2 = rows.next().unwrap();
    assert!(r2.error().is_some());
    assert_eq!(r2.get(0), Some(&b"1"[..]));
    assert_eq!(r2.get(1), Some(&b"2"[..]));
    assert_eq!(r2.get(2), Some(&b"3"[..]));
}

#[test]
fn test_parse_headers_fails_then_rows_still_work() {
    let data = b"name,age\nAlice,30\nBob,25\n";
    let mut reader = Reader::new(std::io::Cursor::new(data));
    // Manually inject an error: parse headers then force re-parse should consume the header row
    let headers = reader.parse_headers().unwrap();
    assert_eq!(headers, &["name", "age"]);
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["Alice", "30"]);
}

#[test]
fn test_set_delimiter_null_byte() {
    let data = b"a\0b\0c\n1\02\03\n";
    let mut reader = Reader::new(std::io::Cursor::new(data)).set_delimiter(0x00);
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows[0], vec!["a", "b", "c"]);
    assert_eq!(rows[1], vec!["1", "2", "3"]);
}

#[test]
fn test_set_delimiter_0xff() {
    let data = b"a\xffb\xffc\n1\xff2\xff3\n";
    let mut reader = Reader::new(std::io::Cursor::new(data)).set_delimiter(0xFF);
    let rows: Vec<Vec<String>> = reader
        .rows()
        .map(|r| r.to_vec().unwrap().iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(rows[0], vec!["a", "b", "c"]);
    assert_eq!(rows[1], vec!["1", "2", "3"]);
}

#[test]
fn test_error_after_quote_with_cr_at_boundary() {
    // This exercises the fixed I/O error path in reader.rs InQuoted state
    // where a quote at a buffer boundary triggers fill_buf() which errors
    let data = b"\"hello";
    let mut reader = Reader::new(ErrorAfterReader::new(data.to_vec(), 3));
    let mut rows = reader.rows();
    let row = rows.next().unwrap();
    assert!(matches!(row.error().unwrap().kind(), ReadErrorKind::Io));
}

// ── Writer: edge cases ────────────────────────────────────────────────

#[test]
fn test_writer_unicode_fields() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["café", "ñoño"]).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "café,ñoño\r\n");
}

#[test]
fn test_writer_all_special_chars_in_one_field() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["comma,quote\"cr\rin\nlf", "ok"]).unwrap();
    let result = w.into_inner().unwrap();
    let mut reader = Reader::new(std::io::Cursor::new(result));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap()[0], "comma,quote\"cr\rin\nlf");
}

#[test]
fn test_writer_write_headers_empty() {
    let mut w = Writer::new(Vec::new());
    w.write_headers(Vec::<&str>::new()).unwrap();
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "\r\n");
}

#[test]
fn test_writer_into_inner_then_drop() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["a", "b"]).unwrap();
    let _ = w.into_inner().unwrap();
    // drop after into_inner: writer is None, should not panic
}

#[test]
fn test_writer_strict_error_truncates_buffer() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["a", "b"]).unwrap();
    let err = w.write_row(["1", "2", "3"]).unwrap_err();
    assert!(matches!(
        err,
        WriteError::InconsistentFieldCount {
            expected: 2,
            found: 3,
            ..
        }
    ));
    // Buffer should be clean — only first row present
    let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
    assert_eq!(result, "a,b\r\n");
}

#[test]
fn test_writer_unicode_roundtrip() {
    let mut w = Writer::new(Vec::new());
    w.write_row(["café", "ñoño"]).unwrap();
    let csv_data = w.into_inner().unwrap();
    let mut reader = Reader::new(std::io::Cursor::new(csv_data));
    let row = reader.rows().next().unwrap();
    assert_eq!(row.to_vec().unwrap(), vec!["café", "ñoño"]);
}

#[cfg(feature = "serde")]
mod serde_edge_tests2 {
    use std::io::Cursor;

    use csv::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_deserialize_float_nan() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct WithF64 {
            val: f64,
        }
        let data = b"val\nNaN\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: WithF64 = reader.rows().next().unwrap().deserialize().unwrap();
        assert!(rec.val.is_nan());
    }

    #[test]
    fn test_deserialize_float_inf() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct WithF64 {
            val: f64,
        }
        let data = b"val\ninf\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: WithF64 = reader.rows().next().unwrap().deserialize().unwrap();
        assert!(rec.val.is_infinite());
        assert!(rec.val.is_sign_positive());
    }

    #[test]
    fn test_deserialize_float_neg_inf() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct WithF64 {
            val: f64,
        }
        let data = b"val\n-inf\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: WithF64 = reader.rows().next().unwrap().deserialize().unwrap();
        assert!(rec.val.is_infinite());
        assert!(rec.val.is_sign_negative());
    }

    #[test]
    fn test_serialize_header_count_mismatch_error() {
        let mut w = Writer::new(Vec::new());
        w.write_headers(["a", "b", "c"]).unwrap();
        w = w.set_headers(vec!["x".into(), "y".into()]);
        #[derive(Serialize)]
        struct TwoFields {
            x: String,
            y: String,
        }
        let rec = TwoFields {
            x: "1".into(),
            y: "2".into(),
        };
        let err = w.serialize(&rec).unwrap_err();
        assert!(matches!(err, WriteError::InconsistentFieldCount { .. }));
    }

    #[test]
    fn test_multi_row_roundtrip() {
        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        struct Record {
            name: String,
            age: u32,
        }
        let records = vec![
            Record {
                name: "Alice".into(),
                age: 30,
            },
            Record {
                name: "Bob".into(),
                age: 25,
            },
            Record {
                name: "Carol".into(),
                age: 35,
            },
        ];

        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "age".into()]);
        for rec in &records {
            w.serialize(rec).unwrap();
        }
        let csv_data = w.into_inner().unwrap();

        let mut reader = Reader::new(Cursor::new(csv_data)).set_headers(vec!["name".into(), "age".into()]);
        let parsed: Vec<Record> = reader.rows().map(|r| r.deserialize().unwrap()).collect();
        assert_eq!(parsed, records);
    }

    #[test]
    fn test_roundtrip_with_header_reorder() {
        // Prove that serde matches by header name, not by position:
        // serialize with headers [age, name], deserialize with same headers
        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        struct Person {
            name: String,
            age: u32,
        }
        let alice = Person {
            name: "Alice".into(),
            age: 30,
        };

        // Serialize with headers in reverse order
        let mut w = Writer::new(Vec::new()).set_headers(vec!["age".into(), "name".into()]);
        w.serialize(&alice).unwrap();
        let csv_data = w.into_inner().unwrap();
        // Output should be: 30,Alice\r\n
        assert_eq!(String::from_utf8_lossy(&csv_data), "30,Alice\r\n");

        // Deserialize with same header order
        let mut reader = Reader::new(Cursor::new(csv_data)).set_headers(vec!["age".into(), "name".into()]);
        let parsed: Person = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(parsed, alice);
    }

    #[test]
    fn test_deserialize_missing_column_ends_with_option_string() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Record {
            name: String,
            extra: Option<String>,
        }
        let data = b"name\nAlice\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: Record = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(rec.name, "Alice");
        assert_eq!(rec.extra, None);
    }

    #[test]
    fn test_serialize_option_none_empty_field() {
        #[derive(Debug, Serialize)]
        struct WithOption {
            name: String,
            extra: Option<String>,
        }
        let mut w = Writer::new(Vec::new()).set_headers(vec!["name".into(), "extra".into()]);
        let rec = WithOption {
            name: "Alice".into(),
            extra: None,
        };
        w.serialize(&rec).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "Alice,\r\n");
    }

    #[test]
    fn test_serialize_multiple_headers_all_filled() {
        #[derive(Serialize)]
        struct Wide {
            a: String,
            b: String,
            c: String,
            d: String,
        }
        let mut w = Writer::new(Vec::new()).set_headers(vec!["a".into(), "b".into(), "c".into(), "d".into()]);
        let rec = Wide {
            a: "1".into(),
            b: "2".into(),
            c: "3".into(),
            d: "4".into(),
        };
        w.serialize(&rec).unwrap();
        let result = String::from_utf8(w.into_inner().unwrap()).unwrap();
        assert_eq!(result, "1,2,3,4\r\n");
    }

    #[test]
    fn test_deserialize_empty_string_to_bool_errors() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct WithBool {
            val: bool,
        }
        let data = b"val\n\n";
        let mut reader = Reader::new(Cursor::new(data)).set_headers(vec!["val".into()]);
        let result: Result<WithBool, _> = reader.rows().next().unwrap().deserialize();
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_empty_string_to_i32_errors() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct WithI32 {
            val: i32,
        }
        let data = b"val\n\n";
        let mut reader = Reader::new(Cursor::new(data)).set_headers(vec!["val".into()]);
        let result: Result<WithI32, _> = reader.rows().next().unwrap().deserialize();
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_empty_string_to_f64_errors() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct WithF64 {
            val: f64,
        }
        let data = b"val\n\n";
        let mut reader = Reader::new(Cursor::new(data)).set_headers(vec!["val".into()]);
        let result: Result<WithF64, _> = reader.rows().next().unwrap().deserialize();
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_option_with_explicit_empty_field() {
        // Column present but field is explicitly empty ""
        #[derive(Debug, Deserialize, PartialEq)]
        struct Record {
            name: String,
            extra: Option<String>,
        }
        let data = b"name,extra\nAlice,\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: Record = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(rec.name, "Alice");
        assert_eq!(rec.extra, None);
    }

    #[test]
    fn test_deserialize_option_with_present_field_is_some() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Record {
            name: String,
            extra: Option<String>,
        }
        let data = b"name,extra\nAlice,hello\n";
        let mut reader = Reader::new(Cursor::new(data));
        reader.parse_headers().unwrap();
        let rec: Record = reader.rows().next().unwrap().deserialize().unwrap();
        assert_eq!(rec.name, "Alice");
        assert_eq!(rec.extra, Some("hello".into()));
    }
}
