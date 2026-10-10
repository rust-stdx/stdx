//! Encoding/decoding microbenchmarks (no database required).

use std::hint::black_box;

use bytes::{BufMut, BytesMut};
use criterion::{Criterion, criterion_group, criterion_main};
use postgresql::{
    __private::{bench_columns, bench_parse_row},
    FromSql, ToSql, array,
};

fn encode_scalar_params(c: &mut Criterion) {
    c.bench_function("encode 3 scalar params", |b| {
        b.iter(|| {
            let mut buf = BytesMut::with_capacity(64);
            black_box(&42i32).encode(&mut buf).unwrap();
            black_box(&"hello").encode(&mut buf).unwrap();
            black_box(&true).encode(&mut buf).unwrap();
            buf
        })
    });
}

fn encode_vec_array(c: &mut Criterion) {
    let values: Vec<i32> = (0..1000).collect();
    c.bench_function("encode Vec<i32> array (1000)", |b| {
        b.iter(|| {
            let mut buf = BytesMut::with_capacity(4100);
            black_box(&values).encode(&mut buf).unwrap();
            buf
        })
    });
}

fn encode_iter_array(c: &mut Criterion) {
    c.bench_function("encode lazy iterator array (1000)", |b| {
        b.iter(|| {
            let mut buf = BytesMut::with_capacity(4100);
            array(0..1000i32).encode(&mut buf).unwrap();
            buf
        })
    });
}

fn encode_jsonb(c: &mut Criterion) {
    let value = serde_json::json!({
        "name": "carol",
        "admin": true,
        "roles": ["reader", "writer", "admin"],
        "meta": { "created": 1_700_000_000, "score": 12.5 }
    });
    c.bench_function("encode jsonb value", |b| {
        b.iter(|| {
            let mut buf = BytesMut::with_capacity(256);
            black_box(&value).encode(&mut buf).unwrap();
            buf
        })
    });
}

fn decode_array(c: &mut Criterion) {
    let values: Vec<i32> = (0..1000).collect();
    let mut encoded = BytesMut::new();
    values.encode(&mut encoded).unwrap();
    c.bench_function("decode int4[] (1000)", |b| {
        b.iter(|| black_box(Vec::<i32>::from_sql(1007, &encoded).unwrap()))
    });
}

fn decode_scalar(c: &mut Criterion) {
    let bytes = 42i32.to_be_bytes();
    c.bench_function("decode int4", |b| b.iter(|| black_box(i32::from_sql(23, &bytes).unwrap())));
}

fn decode_text(c: &mut Criterion) {
    let bytes = b"the quick brown fox jumps over the lazy dog";
    c.bench_function("decode text (43 bytes)", |b| {
        b.iter(|| black_box(String::from_sql(25, bytes).unwrap()))
    });
}

/// Parses a `DataRow` payload of eight `int4` columns, the shape a wide result
/// table produces row after row.
fn parse_row(c: &mut Criterion) {
    let columns = bench_columns(&[23; 8]);
    let mut payload = BytesMut::new();
    payload.put_i16(8);
    for _ in 0..8 {
        payload.put_i32(4);
        payload.put_i32(42);
    }
    let payload = payload.freeze();
    c.bench_function("parse DataRow (8 int4 columns)", |b| {
        b.iter(|| black_box(bench_parse_row(columns.clone(), &payload).unwrap()))
    });
}

criterion_group!(
    benches,
    encode_scalar_params,
    encode_vec_array,
    encode_iter_array,
    encode_jsonb,
    decode_array,
    decode_scalar,
    decode_text,
    parse_row
);
criterion_main!(benches);
