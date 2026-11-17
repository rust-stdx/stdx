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
});
