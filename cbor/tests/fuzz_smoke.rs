#![cfg(feature = "alloc")]

use cbor::Value;
use rand::{Rng, RngExt, SeedableRng, rngs::StdRng};

#[test]
fn random_bytes_never_panic() {
    let mut rng = StdRng::seed_from_u64(0xDEAD_BEEF_CAFE_F00D);
    for _ in 0..20_000 {
        let len = (rng.next_u64() as usize) % 64;
        let mut bytes = vec![0u8; len];
        rng.fill(&mut bytes);
        let _ = cbor::validate(&bytes);
        let _ = cbor::validate_canonical(&bytes);
        let _ = cbor::from_slice::<Value>(&bytes);
        let mut decoder = cbor::Decoder::new(&bytes);
        let _ = decoder.skip();
        let _ = cbor::to_diagnostic_string(&bytes);
    }
}

#[test]
fn mutated_valid_items_never_panic() {
    let seeds: &[&[u8]] = &[
        &[0xa2, 0x61, b'a', 0x01, 0x61, b'b', 0x82, 0x02, 0x03],
        &[0x9f, 0x01, 0x02, 0xff],
        &[0x5f, 0x41, 0x01, 0x41, 0x02, 0xff],
        &[0xc1, 0x1a, 0x65, 0x53, 0xf1, 0x00],
        &[0x83, 0xf9, 0x3e, 0x00, 0x61, b'x', 0xf6],
    ];
    let mut rng = StdRng::seed_from_u64(0x1234_5678_9abc_def0);
    for seed in seeds {
        for _ in 0..10_000 {
            let mut bytes = seed.to_vec();
            let mutations = 1 + rng.next_u64() as usize % 4;
            for _ in 0..mutations {
                let position = rng.next_u64() as usize % bytes.len();
                bytes[position] = rng.next_u64() as u8;
            }
            let _ = cbor::validate(&bytes);
            let _ = cbor::validate_canonical(&bytes);
            let _ = cbor::from_slice::<Value>(&bytes);
            let mut decoder = cbor::Decoder::new(&bytes);
            let _ = decoder.skip();
        }
    }
}

#[test]
fn value_roundtrip_is_stable() {
    let mut rng = StdRng::seed_from_u64(0x0BAD_C0DE_1234_5678);
    for _ in 0..1_000 {
        let len = (rng.next_u64() as usize) % 32;
        let mut bytes = vec![0u8; len];
        rng.fill(&mut bytes);
        if let Ok(value) = cbor::from_slice::<Value>(&bytes) {
            let encoded = cbor::to_vec(&value);
            let decoded: Value = cbor::from_slice(&encoded).unwrap();
            // NaN is not equal to itself; compare the stable re-encoding.
            assert_eq!(cbor::to_vec(&decoded), encoded);
        }
    }
}
