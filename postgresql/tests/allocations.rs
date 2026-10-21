//! Allocation budgets for the hot paths.
//!
//! `alloc_counter` installs a per-thread counting allocator, so these
//! measurements are exact for the thread running the code. Database tests run
//! on a current-thread runtime so the actor's allocations are included too.

use alloc_counter::measure;
use bytes::{BufMut, BytesMut};
use postgresql::{
    __private::{bench_columns, bench_parse_row},
    Connection, FromSql, Query, QueryAs,
};

fn data_row(int4_columns: usize) -> bytes::Bytes {
    let mut payload = BytesMut::new();
    payload.put_i16(int4_columns as i16);
    for _ in 0..int4_columns {
        payload.put_i32(4);
        payload.put_i32(42);
    }
    payload.freeze()
}

/// `Row` stores up to four offsets inline, so narrow rows must not allocate at
/// all when parsed or cloned.
#[test]
fn narrow_rows_parse_and_clone_without_allocating() {
    for columns in 1..=4 {
        let fields = bench_columns(&vec![23u32; columns]);
        let payload = data_row(columns);
        let iterations = 1_000u64;
        let (row, counters) = measure(|| bench_parse_row(fields, &payload).unwrap());
        assert_eq!(counters.calls, 0, "{columns} column(s): parse allocated");
        let (_, counters) = measure(|| {
            for _ in 0..iterations {
                std::hint::black_box(row.clone());
            }
        });
        assert_eq!(
            counters.calls, 0,
            "{columns} column(s): clone allocated {} times",
            counters.calls
        );
    }
}

/// Rows wider than four columns spill the offsets to the heap exactly once.
#[test]
fn wide_rows_spill_at_most_one_allocation() {
    for columns in [5usize, 8, 16] {
        let fields = bench_columns(&vec![23u32; columns]);
        let payload = data_row(columns);
        let iterations = 1_000u64;
        let row = bench_parse_row(fields.clone(), &payload).unwrap();
        let (_, counters) = measure(|| {
            for _ in 0..iterations {
                std::hint::black_box(bench_parse_row(fields.clone(), &payload).unwrap());
            }
        });
        assert_eq!(
            counters.calls, iterations,
            "{columns} column(s): parse must allocate exactly the offsets"
        );
        let (_, counters) = measure(|| {
            for _ in 0..iterations {
                std::hint::black_box(row.clone());
            }
        });
        assert_eq!(
            counters.calls, iterations,
            "{columns} column(s): clone must allocate exactly the offsets"
        );
    }
}

#[test]
fn decoding_an_array_allocates_once() {
    let values: Vec<i32> = (0..1000).collect();
    let mut encoded = BytesMut::new();
    postgresql::ToSql::encode(&values, &mut encoded).unwrap();
    let (decoded, counters) = measure(|| Vec::<i32>::from_sql(1007, &encoded).unwrap());
    assert_eq!(decoded.len(), 1000);
    assert_eq!(counters.calls, 1, "the output vector is the only allocation");
}

/// `fetch_one` uses a one-shot instead of a row channel, `execute` has no row
/// sink at all, and a large `fetch_all` allocates only the decoded values.
#[test]
fn query_paths_stay_within_their_allocation_budget() {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("skipping: DATABASE_URL is not set");
        return;
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let conn = rt.block_on(Connection::connect(&url)).unwrap();

    // Warm the statement cache.
    let _: i64 = rt
        .block_on(QueryAs::<i64>::new("SELECT 1::int8", &[]).fetch_one(&conn))
        .unwrap();
    let _ = rt.block_on(Query::new("SELECT 1::int8", &[]).execute(&conn)).unwrap();
    let sql = "SELECT g::int4, 'row-' || g::text FROM generate_series(1, 1000) g";
    let _: Vec<(i32, String)> = rt.block_on(QueryAs::new(sql, &[]).fetch_all(&conn)).unwrap();

    let iterations = 100u32;
    let (_, counters) = measure(|| {
        for _ in 0..iterations {
            let _: i64 = rt
                .block_on(QueryAs::<i64>::new("SELECT 1::int8", &[]).fetch_one(&conn))
                .unwrap();
        }
    });
    assert!(
        counters.calls <= iterations as u64 * 6,
        "fetch_one allocated {} times for {iterations} queries (a row channel?)",
        counters.calls
    );
    assert!(
        counters.bytes <= iterations as u64 * 1_500,
        "fetch_one allocated {} bytes for {iterations} queries (a 32-slot channel block is 2592 B)",
        counters.bytes
    );

    let (_, counters) = measure(|| {
        for _ in 0..iterations {
            let _ = rt.block_on(Query::new("SELECT 1::int8", &[]).execute(&conn)).unwrap();
        }
    });
    assert!(
        counters.calls <= iterations as u64 * 5,
        "execute allocated {} times for {iterations} queries",
        counters.calls
    );

    let rounds = 10u32;
    let (_, counters) = measure(|| {
        for _ in 0..rounds {
            let _: Vec<(i32, String)> = rt.block_on(QueryAs::new(sql, &[]).fetch_all(&conn)).unwrap();
        }
    });
    assert!(
        counters.calls <= rounds as u64 * 1_100,
        "fetch_all(1000) allocated {} times for {rounds} queries",
        counters.calls
    );
    assert!(
        counters.bytes <= rounds as u64 * 210_000,
        "fetch_all(1000) allocated {} bytes for {rounds} queries",
        counters.bytes
    );
}
