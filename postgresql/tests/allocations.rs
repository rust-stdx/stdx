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

fn data_row_8_int4() -> bytes::Bytes {
    let mut payload = BytesMut::new();
    payload.put_i16(8);
    for _ in 0..8 {
        payload.put_i32(4);
        payload.put_i32(42);
    }
    payload.freeze()
}

#[test]
fn parsing_a_row_allocates_once() {
    let columns = bench_columns(&[23; 8]);
    let payload = data_row_8_int4();
    let iterations = 1_000u32;
    let (_, counters) = measure(|| {
        for _ in 0..iterations {
            std::hint::black_box(bench_parse_row(columns.clone(), &payload).unwrap());
        }
    });
    assert_eq!(
        counters.calls, iterations as u64,
        "a row must allocate exactly its values vector, not one allocation per column"
    );
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

/// `execute` no longer allocates a row channel, and `fetch_one` allocates only
/// the fixed per-query machinery.
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

    let iterations = 100u32;
    let (_, counters) = measure(|| {
        for _ in 0..iterations {
            let _: i64 = rt
                .block_on(QueryAs::<i64>::new("SELECT 1::int8", &[]).fetch_one(&conn))
                .unwrap();
        }
    });
    assert!(
        counters.calls <= iterations as u64 * 8,
        "fetch_one allocated {} times for {iterations} queries",
        counters.calls
    );

    let (_, counters) = measure(|| {
        for _ in 0..iterations {
            let _ = rt.block_on(Query::new("SELECT 1::int8", &[]).execute(&conn)).unwrap();
        }
    });
    assert!(
        counters.calls <= iterations as u64 * 5,
        "execute allocated {} times for {iterations} queries (a discarded row channel?)",
        counters.calls
    );
}
