use std::hint::black_box;

use cbor_benchmarks::*;
use criterion::{Criterion, criterion_group, criterion_main};
use serde::{Serialize, de::DeserializeOwned};

fn log_benches(c: &mut Criterion) {
    let logs = logs(128);
    let one = &logs[0];

    let cbor_bytes = cbor::to_vec(&logs);
    let mut ciborium_bytes = Vec::new();
    ciborium::into_writer(&logs, &mut ciborium_bytes).unwrap();
    let cbor2_bytes = cbor2::to_vec(&logs).unwrap();
    let minicbor_bytes = minicbor::to_vec(&logs).unwrap();

    println!(
        "log sizes: cbor={} ciborium={} cbor2={} minicbor={}",
        cbor_bytes.len(),
        ciborium_bytes.len(),
        cbor2_bytes.len(),
        minicbor_bytes.len()
    );

    let mut group = c.benchmark_group("encode/log");
    group.bench_function("cbor_typed", |b| b.iter(|| cbor::to_vec(black_box(&logs))));
    group.bench_function("ciborium", |b| {
        b.iter(|| {
            let mut out = Vec::new();
            ciborium::into_writer(black_box(&logs), &mut out).unwrap();
            out
        })
    });
    group.bench_function("cbor2", |b| b.iter(|| cbor2::to_vec(black_box(&logs)).unwrap()));
    group.bench_function("minicbor", |b| b.iter(|| minicbor::to_vec(black_box(&logs)).unwrap()));
    group.finish();

    let mut group = c.benchmark_group("decode/log");
    group.bench_function("cbor_typed", |b| {
        b.iter(|| cbor::from_slice::<Vec<LogEntry>>(black_box(&cbor_bytes)).unwrap())
    });
    group.bench_function("ciborium", |b| {
        b.iter(|| ciborium::from_reader::<Vec<LogEntry>, _>(black_box(ciborium_bytes.as_slice())).unwrap())
    });
    group.bench_function("cbor2", |b| {
        b.iter(|| cbor2::from_slice::<Vec<LogEntry>>(black_box(&cbor2_bytes)).unwrap())
    });
    group.bench_function("minicbor", |b| {
        b.iter(|| minicbor::decode::<Vec<LogEntry>>(black_box(&minicbor_bytes)).unwrap())
    });
    group.finish();

    let manual: Vec<ManualLog> = logs.iter().cloned().map(ManualLog).collect();
    let manual_bytes = cbor::to_vec(&manual);
    let mut group = c.benchmark_group("encode/log_manual");
    group.bench_function("cbor_manual", |b| b.iter(|| cbor::to_vec(black_box(&manual))));
    group.finish();
    let mut group = c.benchmark_group("decode/log_manual");
    group.bench_function("cbor_manual", |b| {
        b.iter(|| cbor::from_slice::<Vec<ManualLog>>(black_box(&manual_bytes)).unwrap())
    });
    group.finish();

    let mut group = c.benchmark_group("encode/log_fixed");
    let mut buffer = [0u8; 4096];
    group.bench_function("cbor_typed", |b| {
        b.iter(|| {
            let used = cbor::to_slice(black_box(one), &mut buffer).unwrap().len();
            black_box(&buffer[..used]);
            used
        })
    });
    group.finish();

    let mut group = c.benchmark_group("encode/log_capacity");
    group.bench_function("to_vec", |b| b.iter(|| cbor::to_vec(black_box(&logs))));
    group.bench_function("to_vec_with_capacity", |b| {
        b.iter(|| {
            let len = cbor::encoded_len(black_box(&logs));
            cbor::to_vec_with_capacity(black_box(&logs), len)
        })
    });
    group.bench_function("to_vec_into_reused", |b| {
        let mut buffer = Vec::new();
        b.iter(|| {
            cbor::to_vec_into(black_box(&logs), &mut buffer);
            black_box(buffer.len())
        })
    });
    group.bench_function("to_vec_with_capacity_512", |b| {
        b.iter(|| cbor::to_vec_with_capacity(black_box(&logs), 512))
    });
    group.bench_function("to_vec_with_capacity_4096", |b| {
        b.iter(|| cbor::to_vec_with_capacity(black_box(&logs), 4096))
    });
    group.finish();

    let mut group = c.benchmark_group("encode/one_capacity");
    group.bench_function("to_vec", |b| b.iter(|| cbor::to_vec(black_box(one))));
    group.bench_function("to_vec_with_capacity_256", |b| {
        b.iter(|| cbor::to_vec_with_capacity(black_box(one), 256))
    });
    group.bench_function("to_vec_with_capacity_512", |b| {
        b.iter(|| cbor::to_vec_with_capacity(black_box(one), 512))
    });
    group.finish();
}

fn array_benches(c: &mut Criterion) {
    let logs = logs(128);
    let entries = array_entries(&logs);
    let one = &entries[0];

    let cbor_bytes = cbor::to_vec(&entries);
    let mut ciborium_bytes = Vec::new();
    ciborium::into_writer(&entries, &mut ciborium_bytes).unwrap();
    let cbor2_bytes = cbor2::to_vec(&entries).unwrap();

    println!(
        "array sizes: cbor={} ciborium={} cbor2={}",
        cbor_bytes.len(),
        ciborium_bytes.len(),
        cbor2_bytes.len()
    );

    let mut group = c.benchmark_group("encode/array");
    group.bench_function("cbor_typed", |b| b.iter(|| cbor::to_vec(black_box(&entries))));
    group.bench_function("ciborium", |b| {
        b.iter(|| {
            let mut out = Vec::new();
            ciborium::into_writer(black_box(&entries), &mut out).unwrap();
            out
        })
    });
    group.bench_function("cbor2", |b| b.iter(|| cbor2::to_vec(black_box(&entries)).unwrap()));
    group.finish();

    let mut group = c.benchmark_group("decode/array");
    group.bench_function("cbor_typed", |b| {
        b.iter(|| cbor::from_slice::<Vec<ArrayEntry>>(black_box(&cbor_bytes)).unwrap())
    });
    group.bench_function("ciborium", |b| {
        b.iter(|| ciborium::from_reader::<Vec<ArrayEntry>, _>(black_box(ciborium_bytes.as_slice())).unwrap())
    });
    group.bench_function("cbor2", |b| {
        b.iter(|| cbor2::from_slice::<Vec<ArrayEntry>>(black_box(&cbor2_bytes)).unwrap())
    });
    group.finish();

    let mut group = c.benchmark_group("encode/array_fixed");
    let mut buffer = [0u8; 4096];
    group.bench_function("cbor_typed", |b| {
        b.iter(|| {
            let used = cbor::to_slice(black_box(one), &mut buffer).unwrap().len();
            black_box(&buffer[..used]);
            used
        })
    });
    group.finish();
}

fn int_benches(c: &mut Criterion) {
    let entries = int_entries(128);
    let one = &entries[0];
    let cbor_bytes = cbor::to_vec(&entries);
    let minicbor_bytes = minicbor::to_vec(&entries).unwrap();
    println!("int sizes: cbor={} minicbor={}", cbor_bytes.len(), minicbor_bytes.len());

    let mut group = c.benchmark_group("encode/int_keys");
    group.bench_function("cbor_typed", |b| b.iter(|| cbor::to_vec(black_box(&entries))));
    group.bench_function("minicbor", |b| b.iter(|| minicbor::to_vec(black_box(&entries)).unwrap()));
    group.finish();

    let mut group = c.benchmark_group("decode/int_keys");
    group.bench_function("cbor_typed", |b| {
        b.iter(|| cbor::from_slice::<Vec<IntEntry>>(black_box(&cbor_bytes)).unwrap())
    });
    group.bench_function("minicbor", |b| {
        b.iter(|| minicbor::decode::<Vec<IntEntry>>(black_box(&minicbor_bytes)).unwrap())
    });
    group.finish();

    let manual: Vec<ManualInt> = entries.iter().cloned().map(ManualInt).collect();
    let manual_bytes = cbor::to_vec(&manual);
    let mut group = c.benchmark_group("encode/int_keys_manual");
    group.bench_function("cbor_manual", |b| b.iter(|| cbor::to_vec(black_box(&manual))));
    group.finish();
    let mut group = c.benchmark_group("decode/int_keys_manual");
    group.bench_function("cbor_manual", |b| {
        b.iter(|| cbor::from_slice::<Vec<ManualInt>>(black_box(&manual_bytes)).unwrap())
    });
    group.finish();

    let mut group = c.benchmark_group("encode/int_keys_fixed");
    let mut buffer = [0u8; 4096];
    group.bench_function("cbor_typed", |b| {
        b.iter(|| {
            let used = cbor::to_slice(black_box(one), &mut buffer).unwrap().len();
            black_box(&buffer[..used]);
            used
        })
    });
    group.finish();
}

fn blob_benches(c: &mut Criterion) {
    let blob = blob();
    let cbor_blob = bytes::Bytes::from(blob.clone());
    let cbor_bytes = cbor::to_vec(&cbor_blob);
    let serde_blob = serde_bytes::ByteBuf::from(blob.clone());
    let mut ciborium_bytes = Vec::new();
    ciborium::into_writer(&serde_blob, &mut ciborium_bytes).unwrap();
    let cbor2_bytes = cbor2::to_vec(&serde_blob).unwrap();
    let minicbor_blob = minicbor::bytes::ByteVec::from(blob.clone());
    let minicbor_bytes = minicbor::to_vec(&minicbor_blob).unwrap();
    println!(
        "blob sizes: cbor={} ciborium={} cbor2={} minicbor={}",
        cbor_bytes.len(),
        ciborium_bytes.len(),
        cbor2_bytes.len(),
        minicbor_bytes.len()
    );

    let mut group = c.benchmark_group("encode/blob");
    group.bench_function("cbor_typed", |b| b.iter(|| cbor::to_vec(black_box(&cbor_blob))));
    group.bench_function("ciborium", |b| {
        b.iter(|| {
            let mut out = Vec::new();
            ciborium::into_writer(black_box(&serde_blob), &mut out).unwrap();
            out
        })
    });
    group.bench_function("cbor2", |b| b.iter(|| cbor2::to_vec(black_box(&serde_blob)).unwrap()));
    group.bench_function("minicbor", |b| b.iter(|| minicbor::to_vec(black_box(&minicbor_blob)).unwrap()));
    group.finish();

    let mut group = c.benchmark_group("decode/blob");
    group.bench_function("cbor_typed", |b| {
        b.iter(|| cbor::from_slice::<bytes::Bytes>(black_box(&cbor_bytes)).unwrap())
    });
    group.bench_function("ciborium", |b| {
        b.iter(|| ciborium::from_reader::<serde_bytes::ByteBuf, _>(black_box(ciborium_bytes.as_slice())).unwrap())
    });
    group.bench_function("cbor2", |b| {
        b.iter(|| cbor2::from_slice::<serde_bytes::ByteBuf>(black_box(&cbor2_bytes)).unwrap())
    });
    group.bench_function("minicbor", |b| {
        b.iter(|| minicbor::decode::<minicbor::bytes::ByteVec>(black_box(&minicbor_bytes)).unwrap())
    });
    group.finish();
}

fn text_benches(c: &mut Criterion) {
    let ascii: String = std::iter::repeat_n('a', 64 * 1024).collect();
    let non_ascii: String = std::iter::repeat_n('€', 32 * 1024).collect();

    for (name, text) in [("ascii", &ascii), ("non_ascii", &non_ascii)] {
        let bytes = cbor::to_vec(text);
        let mut group = c.benchmark_group(format!("text/{name}"));
        group.bench_function("cbor_validate", |b| b.iter(|| cbor::validate(black_box(&bytes))));
        group.bench_function("std_validate", |b| {
            b.iter(|| core::str::from_utf8(black_box(text.as_bytes())).is_ok())
        });
        group.bench_function("cbor_from_slice_str", |b| {
            b.iter(|| cbor::from_slice::<&str>(black_box(&bytes)).unwrap())
        });
        group.bench_function("std_from_utf8", |b| {
            b.iter(|| core::str::from_utf8(black_box(text.as_bytes())).unwrap())
        });
        group.bench_function("cbor_from_slice_string", |b| {
            b.iter(|| cbor::from_slice::<String>(black_box(&bytes)).unwrap())
        });
        group.bench_function("to_owned", |b| b.iter(|| black_box(text).to_owned()));
        group.finish();
    }
}

fn stream_benches(c: &mut Criterion) {
    use std::io::{BufReader, Cursor};

    let logs = logs(128);
    let bytes = cbor::to_vec(&logs[0]);

    let mut group = c.benchmark_group("stream/read_item");
    group.bench_function("read_item", |b| {
        b.iter(|| {
            let mut reader = BufReader::new(Cursor::new(black_box(bytes.as_slice())));
            cbor::reader::read_item(&mut reader).unwrap()
        })
    });
    group.bench_function("read_item_into", |b| {
        let mut buffer = Vec::new();
        b.iter(|| {
            let mut reader = BufReader::new(Cursor::new(black_box(bytes.as_slice())));
            let used = cbor::reader::read_item_into(&mut reader, &mut buffer).unwrap();
            black_box(used)
        })
    });
    group.bench_function("from_slice", |b| {
        b.iter(|| cbor::from_slice::<LogEntry>(black_box(&bytes)).unwrap())
    });
    group.finish();
}

fn _assert_traits<T: Serialize + DeserializeOwned>() {}

criterion_group!(
    benches,
    log_benches,
    array_benches,
    int_benches,
    blob_benches,
    text_benches,
    stream_benches
);
criterion_main!(benches, micro);

fn micro_benches(c: &mut Criterion) {
    let strings: Vec<String> = (0..1000).map(|i| format!("string-{i:04}")).collect();
    let ints: Vec<u64> = (0..1000).map(|i| i * 37).collect();
    let cbor_strings = cbor::to_vec(&strings);
    let mini_strings = minicbor::to_vec(&strings).unwrap();
    let cbor_ints = cbor::to_vec(&ints);
    let mini_ints = minicbor::to_vec(&ints).unwrap();
    let floats: Vec<f64> = (0..1000).map(|i| i as f64 / 1024.0).collect();
    let cbor_floats = cbor::to_vec(&floats);
    let mini_floats = minicbor::to_vec(&floats).unwrap();

    let mut group = c.benchmark_group("micro/strings");
    group.bench_function("cbor", |b| {
        b.iter(|| cbor::from_slice::<Vec<String>>(black_box(&cbor_strings)).unwrap())
    });
    group.bench_function("minicbor", |b| {
        b.iter(|| minicbor::decode::<Vec<String>>(black_box(&mini_strings)).unwrap())
    });
    group.finish();

    let mut skip_group = c.benchmark_group("skip/ints");
    skip_group.bench_function("skip", |b| {
        b.iter(|| {
            let mut decoder = cbor::Decoder::new(black_box(&cbor_ints));
            decoder.skip().unwrap()
        })
    });
    skip_group.bench_function("validate", |b| b.iter(|| cbor::validate(black_box(&cbor_ints)).unwrap()));
    skip_group.bench_function("decode", |b| {
        b.iter(|| cbor::from_slice::<Vec<u64>>(black_box(&cbor_ints)).unwrap())
    });
    skip_group.finish();

    let mut group = c.benchmark_group("micro/ints");
    group.bench_function("cbor", |b| {
        b.iter(|| cbor::from_slice::<Vec<u64>>(black_box(&cbor_ints)).unwrap())
    });
    group.bench_function("minicbor", |b| {
        b.iter(|| minicbor::decode::<Vec<u64>>(black_box(&mini_ints)).unwrap())
    });
    group.finish();

    let mut group = c.benchmark_group("micro/floats");
    group.bench_function("cbor", |b| {
        b.iter(|| cbor::from_slice::<Vec<f64>>(black_box(&cbor_floats)).unwrap())
    });
    group.bench_function("minicbor", |b| {
        b.iter(|| minicbor::decode::<Vec<f64>>(black_box(&mini_floats)).unwrap())
    });
    group.finish();
}

criterion_group!(micro, micro_benches);
