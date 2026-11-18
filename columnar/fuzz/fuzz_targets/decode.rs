#![no_main]

use libfuzzer_sys::fuzz_target;

#[derive(columnar::Columnar, Debug, PartialEq)]
#[columnar(iterable)]
struct Row {
    #[columnar(key = 1)]
    id: u64,
    #[columnar(key = 2)]
    name: String,
    #[columnar(key = 3)]
    score: Option<i64>,
    #[columnar(key = 4)]
    tags: Vec<u32>,
    #[columnar(key = 5)]
    kind: Kind,
    #[columnar(key = 6, encoding = "delta_rle")]
    delta: u64,
    #[columnar(key = 7, encoding = "delta_of_delta")]
    delta_of_delta: i64,
    #[columnar(key = 8, encoding = "dictionary")]
    dictionary: String,
    #[columnar(key = 9, encoding = "offsets")]
    offsets: String,
    #[columnar(key = 10, encoding = "packed")]
    packed: u64,
    #[columnar(key = 11, encoding = "varint")]
    varint: i64,
    #[columnar(key = 12, encoding = "rle")]
    rle: bool,
    #[columnar(key = 13)]
    optional_list: Option<Vec<u16>>,
}

#[derive(columnar::Columnar, Debug, PartialEq)]
#[columnar(iterable)]
enum Kind {
    A,
    B(u64),
    C { x: i32, y: String },
}

fuzz_target!(|data: &[u8]| {
    let _ = columnar::from_slice::<Row>(data);
    if let Ok(rows) = columnar::iter_from_slice::<Row>(data) {
        for row in rows {
            let _ = row;
        }
    }
    // Tight limits exercise the budget and cap paths.
    let options = columnar::DecodeOptions {
        max_rows: 1 << 8,
        max_string_len: 1 << 12,
        max_list_len: 1 << 12,
        max_depth: 16,
        max_total_bytes: 1 << 16,
        max_decoded_bytes: 1 << 16,
    };
    let _ = columnar::from_slice_with_options::<Row>(data, options);
    let _ = columnar::iter_from_slice_with_options::<Row>(data, options).map(|rows| rows.count());
});
