//! Allocation and retained-memory regression tests.
//!
//! Everything lives in a single `#[test]` on purpose: the tests observe a
//! process-wide counting allocator, so they must not run concurrently with any
//! other test in this binary.

use std::{
    alloc::{GlobalAlloc, Layout, System},
    io::Cursor,
    sync::atomic::{AtomicUsize, Ordering},
};

use csv::*;

struct Counting;

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);
static LIVE: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        LIVE.fetch_add(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(new_size, Ordering::Relaxed);
        if new_size >= layout.size() {
            LIVE.fetch_add(new_size - layout.size(), Ordering::Relaxed);
        } else {
            LIVE.fetch_sub(layout.size() - new_size, Ordering::Relaxed);
        }
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// Runs `f`, returning `(allocations, bytes)` made during the call.
fn measure<R>(f: impl FnOnce() -> R) -> (usize, usize, R) {
    let allocs = ALLOCS.load(Ordering::Relaxed);
    let bytes = BYTES.load(Ordering::Relaxed);
    let value = f();
    (
        ALLOCS.load(Ordering::Relaxed) - allocs,
        BYTES.load(Ordering::Relaxed) - bytes,
        value,
    )
}

fn live() -> usize {
    LIVE.load(Ordering::Relaxed)
}

#[test]
fn allocation_bounds() {
    read_path_allocates_a_bounded_amount_per_row();
    a_huge_record_does_not_inflate_later_rows();
    a_wide_header_does_not_inflate_later_rows();
    a_record_limit_bounds_memory();
    a_field_limit_bounds_memory();
    the_reader_releases_memory_after_a_huge_record();
    #[cfg(feature = "serde")]
    serializing_does_not_allocate_per_field();
}

fn read_path_allocates_a_bounded_amount_per_row() {
    let mut data = Vec::new();
    for i in 0..2000 {
        data.extend_from_slice(format!("row{i},value{i},3rd{i}\n").as_bytes());
    }

    let mut reader = Reader::new(Cursor::new(&data[..]));
    let mut rows = reader.rows();
    for _ in 0..64 {
        assert!(rows.next().is_some());
    }

    let (allocs, bytes, n) = measure(|| rows.count());
    assert_eq!(n, 2000 - 64);

    let bytes_per_row = bytes / n;
    let allocs_per_row = allocs as f64 / n as f64;
    assert!(
        bytes_per_row < 256,
        "{bytes_per_row} bytes allocated per row ({bytes} for {n} rows)"
    );
    assert!(
        allocs_per_row < 4.0,
        "{allocs_per_row:.2} allocations per row ({allocs} for {n} rows)"
    );
}

fn a_huge_record_does_not_inflate_later_rows() {
    // Regression: `row_size_hint` was the running maximum row size and was used
    // to pre-size every later row's buffer, so one 4 MiB record made each
    // following row of empty fields allocate 4 MiB.
    let mut data = Vec::new();
    data.extend_from_slice(b"x,y,z\n");
    data.extend(std::iter::repeat_n(b'A', 4 * 1024 * 1024));
    data.extend_from_slice(b",2,3\n");
    for _ in 0..2000 {
        data.extend_from_slice(b",,\n");
    }

    // 1 header-ish record + 1 huge record + 2000 records of empty fields.
    const TOTAL: usize = 2002;
    let mut reader = Reader::new(Cursor::new(&data[..]));
    let mut rows = reader.rows();
    for _ in 0..3 {
        assert!(rows.next().is_some());
    }
    // Let the reader's buffers settle back to their steady state.
    for _ in 0..64 {
        assert!(rows.next().is_some());
    }

    let (_, bytes, n) = measure(|| {
        let mut n = 0;
        for row in &mut rows {
            assert_eq!(row.len(), 3);
            n += 1;
        }
        n
    });
    assert_eq!(n, TOTAL - 67);

    let bytes_per_row = bytes / n;
    assert!(
        bytes_per_row < 256,
        "the 4 MiB record inflated later rows to {bytes_per_row} bytes each"
    );
}

fn a_wide_header_does_not_inflate_later_rows() {
    // Regression: the per-row field-range vector was pre-sized to the header
    // column count, so a 200k-column header made every later 2-field row
    // allocate ~3.2 MB.
    let mut data = Vec::new();
    for i in 0..200_000 {
        if i > 0 {
            data.push(b',');
        }
        data.push(b'c');
    }
    data.push(b'\n');
    for _ in 0..2000 {
        data.extend_from_slice(b"1,2\n");
    }

    let mut reader = Reader::new(Cursor::new(&data[..]));
    assert_eq!(reader.parse_headers().unwrap().len(), 200_000);

    let mut rows = reader.rows();
    for _ in 0..64 {
        assert!(rows.next().is_some());
    }

    let (_, bytes, n) = measure(|| rows.count());
    // `parse_headers` already consumed the header row.
    assert_eq!(n, 2000 - 64);

    let bytes_per_row = bytes / n;
    assert!(
        bytes_per_row < 256,
        "the wide header inflated later rows to {bytes_per_row} bytes each"
    );
}

fn a_record_limit_bounds_memory() {
    // An 8 MiB single-field record must be rejected without buffering it all.
    let mut data = Vec::new();
    data.extend(std::iter::repeat_n(b'A', 8 * 1024 * 1024));
    data.push(b'\n');

    let mut reader = Reader::new(Cursor::new(&data[..])).set_max_record_bytes(1024);
    let (_, bytes, rows) = measure(|| reader.rows().collect::<Vec<Row>>());

    assert_eq!(rows.len(), 1);
    assert!(matches!(
        rows[0].error().map(|e| e.kind()),
        Some(ReadErrorKind::RecordTooLarge {
            limit: 1024
        })
    ));
    assert!(bytes < 512 * 1024, "the record limit did not bound allocations: {bytes} bytes");
}

fn a_field_limit_bounds_memory() {
    // An 8 MiB quoted field with a 1 KiB field limit must not be copied into
    // the scratch buffer, nor into the row buffer.
    let mut data = Vec::new();
    data.push(b'"');
    data.extend(std::iter::repeat_n(b'A', 8 * 1024 * 1024));
    data.extend_from_slice(b"\",b\n");

    let mut reader = Reader::new(Cursor::new(&data[..])).set_max_field_bytes(1024);
    let (_, bytes, rows) = measure(|| reader.rows().collect::<Vec<Row>>());

    assert_eq!(rows.len(), 1);
    assert!(matches!(
        rows[0].error().map(|e| e.kind()),
        Some(ReadErrorKind::FieldTooLarge {
            limit: 1024
        })
    ));
    assert!(bytes < 512 * 1024, "the field limit did not bound allocations: {bytes} bytes");
}

fn the_reader_releases_memory_after_a_huge_record() {
    let mut data = Vec::new();
    data.extend(std::iter::repeat_n(b'A', 4 * 1024 * 1024));
    data.push(b'\n');
    for _ in 0..2000 {
        data.extend_from_slice(b"x,y,z\n");
    }

    let before = live();
    {
        let mut reader = Reader::new(Cursor::new(&data[..]));
        for _ in reader.rows() {}
    }
    let retained = live() - before;

    assert!(
        retained < 1024 * 1024,
        "reader retained {retained} bytes after the huge record was consumed"
    );
}

#[cfg(feature = "serde")]
fn serializing_does_not_allocate_per_field() {
    #[derive(serde::Serialize)]
    struct Record {
        a: String,
        b: String,
        c: String,
    }

    let values: Vec<Record> = (0..2000)
        .map(|i| Record {
            a: format!("row{i}"),
            b: "value".into(),
            c: "3".into(),
        })
        .collect();

    let mut writer = Writer::new(Vec::new()).set_headers(vec!["a".into(), "b".into(), "c".into()]);
    for value in &values {
        writer.serialize(value).unwrap();
    }
    writer.flush().unwrap();

    let (allocs, bytes, ()) = measure(|| {
        for value in &values {
            writer.serialize(value).unwrap();
        }
    });

    // Regression: this used to allocate once per field (3 per row).
    assert!(
        allocs <= 4,
        "serializing allocated {allocs} times for {} rows ({bytes} bytes)",
        values.len()
    );
}
