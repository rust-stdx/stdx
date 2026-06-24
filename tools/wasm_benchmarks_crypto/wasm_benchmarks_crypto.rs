// wasm32 crypto benchmark suite (hash, mac, stream cipher, AEAD).
//
// Compile (with simd): RUSTFLAGS="-C target-feature=+simd128" cargo build --target=wasm32-wasip1 -p wasm_benchmarks_crypto --release
// Run:              node tools/wasm_runner/wasm_runner.ts target/wasm32-wasip1/release/wasm_benchmarks_crypto.wasm

use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use crypto::{
    Aead, Hasher, StreamCipher,
    aes::{Aes256Ctr, Aes256Gcm},
    ascon::{AsconAead128, AsconHash256},
    blake3::Blake3,
    chacha::{ChaCha8Djb, ChaCha12Blake3, ChaCha12Djb, ChaCha20Blake3, ChaCha20Djb, ChaCha20Poly1305},
    hmac::Hmac,
    poly1305::Poly1305,
    sha2::{Sha256, Sha512},
    sha3::{Kmac256, Sha3_256, Sha3_512, Shake256},
};

const DATA_SIZES: &[usize] = &[64, 1024, 16 * 1024, 64 * 1024, 1024 * 1024];

const KEY: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52,
    0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,
];

const KEY_16: [u8; 16] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F,
];

const NONCE_8: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
const NONCE_12: [u8; 12] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C];
const NONCE_16: [u8; 16] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
];
const NONCE_32: [u8; 32] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13,
    0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20,
];

const WARMUP_MS: u64 = 5000;
const BENCH_MS: u64 = 5000;

fn main() {
    let mut results = Vec::new();

    bench_hashes(&mut results);
    bench_macs(&mut results);
    bench_stream_ciphers(&mut results);
    bench_aead(&mut results);

    print_results(&results);
}

fn section(title: &str) {
    eprintln!("\n////////////////////////////////////////////////////////////////////////////////");
    eprintln!("// {title}");
    eprintln!("////////////////////////////////////////////////////////////////////////////////\n");
}

fn format_size(s: usize) -> String {
    if s >= 1024 * 1024 {
        format!("{}MiB", s / (1024 * 1024))
    } else if s >= 1024 {
        format!("{}KiB", s / 1024)
    } else {
        format!("{}B", s)
    }
}

fn benchmark<F>(name: &str, size: usize, mut f: F) -> f64
where
    F: FnMut(),
{
    eprint!("  {:<16} {:>8} ", name, format_size(size));

    let warmup_end = Instant::now() + Duration::from_millis(WARMUP_MS);
    while Instant::now() < warmup_end {
        f();
    }

    let mut elapsed = Duration::ZERO;
    let mut iters: u64 = 0;
    let bench_end = Instant::now() + Duration::from_millis(BENCH_MS);

    while Instant::now() < bench_end {
        let start = Instant::now();
        f();
        elapsed += start.elapsed();
        iters += 1;
    }

    let total_bytes = size as u64 * iters;
    let secs = elapsed.as_secs_f64();
    let mbs = if secs > 0.0 {
        total_bytes as f64 / secs / 1_048_576.0
    } else {
        0.0
    };

    eprintln!("{:>8.1} MB/s", mbs);
    mbs
}

fn bench_stream_ciphers(results: &mut Vec<(&str, usize, &str, f64)>) {
    section("STREAM CIPHERS");

    for &size in DATA_SIZES {
        let mbs = benchmark("AES-256-CTR", size, || {
            let mut cipher = Aes256Ctr::new(&KEY);
            let mut buf = vec![0xA5u8; size];
            cipher.xor_keystream(black_box(&mut buf));
        });
        results.push(("stream", size, "AES-256-CTR", mbs));

        let mbs = benchmark("ChaCha8", size, || {
            let mut cipher = ChaCha8Djb::new(&KEY, &NONCE_8);
            let mut buf = vec![0xA5u8; size];
            cipher.xor_keystream(black_box(&mut buf));
        });
        results.push(("stream", size, "ChaCha8", mbs));

        let mbs = benchmark("ChaCha12", size, || {
            let mut cipher = ChaCha12Djb::new(&KEY, &NONCE_8);
            let mut buf = vec![0xA5u8; size];
            cipher.xor_keystream(black_box(&mut buf));
        });
        results.push(("stream", size, "ChaCha12", mbs));

        let mbs = benchmark("ChaCha20", size, || {
            let mut cipher = ChaCha20Djb::new(&KEY, &NONCE_8);
            let mut buf = vec![0xA5u8; size];
            cipher.xor_keystream(black_box(&mut buf));
        });
        results.push(("stream", size, "ChaCha20", mbs));

        eprintln!();
    }
}

fn bench_hashes(results: &mut Vec<(&str, usize, &str, f64)>) {
    section("HASH FUNCTIONS");

    for &size in DATA_SIZES {
        let data = vec![0xA5u8; size];
        let data2 = data.clone();
        let mbs = benchmark("SHA-256", size, || {
            let _ = Sha256::hash(black_box(&data2));
        });
        results.push(("hash", size, "SHA-256", mbs));

        let data2 = data.clone();
        let mbs = benchmark("SHA-512", size, || {
            let _ = Sha512::hash(black_box(&data2));
        });
        results.push(("hash", size, "SHA-512", mbs));

        let data2 = data.clone();
        let mbs = benchmark("SHA3-256", size, || {
            let _ = Sha3_256::hash(black_box(&data2));
        });
        results.push(("hash", size, "SHA3-256", mbs));

        let data2 = data.clone();
        let mbs = benchmark("SHA3-512", size, || {
            let _ = Sha3_512::hash(black_box(&data2));
        });
        results.push(("hash", size, "SHA3-512", mbs));

        let data2 = data.clone();
        let mbs = benchmark("SHAKE256", size, || {
            let _ = <Shake256 as Hasher>::hash(black_box(&data2));
        });
        results.push(("hash", size, "SHAKE256", mbs));

        let data2 = data.clone();
        let mbs = benchmark("BLAKE3", size, || {
            let _ = Blake3::hash(black_box(&data2));
        });
        results.push(("hash", size, "BLAKE3", mbs));

        let data2 = data.clone();
        let mbs = benchmark("Ascon-Hash256", size, || {
            let _ = AsconHash256::hash(black_box(&data2));
        });
        results.push(("hash", size, "Ascon-Hash256", mbs));

        eprintln!();
    }
}

fn bench_macs(results: &mut Vec<(&str, usize, &str, f64)>) {
    section("MACs");

    let hmac_key = b"rust-stdx-crypto-bench-key";
    let customization = b"rust-stdx";

    for &size in DATA_SIZES {
        let data = vec![0xA3u8; size];

        let data2 = data.clone();
        let mbs = benchmark("HMAC-SHA256", size, || {
            let mut hmac = Hmac::<Sha256>::new(black_box(hmac_key));
            hmac.update(black_box(&data2));
            let _ = hmac.finalize();
        });
        results.push(("mac", size, "HMAC-SHA256", mbs));

        let data2 = data.clone();
        let mbs = benchmark("HMAC-SHA512", size, || {
            let mut hmac = Hmac::<Sha512>::new(black_box(hmac_key));
            hmac.update(black_box(&data2));
            let _ = hmac.finalize();
        });
        results.push(("mac", size, "HMAC-SHA512", mbs));

        let data2 = data.clone();
        let mbs = benchmark("KMAC256", size, || {
            let mut kmac = Kmac256::new(black_box(&KEY), black_box(customization));
            kmac.update(black_box(&data2));
            let mut out = [0u8; 32];
            kmac.finalize_into(&mut out);
            black_box(out);
        });
        results.push(("mac", size, "KMAC256", mbs));

        let data2 = data.clone();
        let mbs = benchmark("Poly1305", size, || {
            let out = Poly1305::mac(&KEY, &data2);
            black_box(out);
        });
        results.push(("mac", size, "Poly1305", mbs));

        let data2 = data.clone();
        let mbs = benchmark("BLAKE3-keyed", size, || {
            let out = Blake3::keyed_hash(black_box(&KEY), black_box(&data2));
            black_box(out);
        });
        results.push(("mac", size, "BLAKE3-keyed", mbs));

        eprintln!();
    }
}

fn bench_aead(results: &mut Vec<(&str, usize, &str, f64)>) {
    section("AEADs");

    let aes = Aes256Gcm::new(&KEY);
    let chacha = ChaCha20Poly1305::new(&KEY);
    let chacha_blake3 = ChaCha20Blake3::new(&KEY);
    let chacha12_blake3 = ChaCha12Blake3::new(&KEY);
    let ascon_aead = AsconAead128::new(&KEY_16);

    for &size in DATA_SIZES {
        let mbs = benchmark("AES-256-GCM-encrypt", size, || {
            let mut buf = vec![0xA5u8; size];
            let _tag = aes.encrypt_in_place(&mut buf, &NONCE_12[..], &[]);
        });
        results.push(("aead", size, "AES-256-GCM-encrypt", mbs));

        let mut data = vec![0xA5u8; size];
        let tag = aes.encrypt_in_place(&mut data, &NONCE_12[..], &[]);
        let mbs = benchmark("AES-256-GCM-decrypt", size, || {
            let mut buf = data.clone();
            let _ = aes.decrypt_in_place(&mut buf, &NONCE_12[..], &[], tag.as_ref());
        });
        results.push(("aead", size, "AES-256-GCM-decrypt", mbs));

        let mbs = benchmark("ChaCha20-P1305-encrypt", size, || {
            let mut buf = vec![0xA5u8; size];
            let _tag = chacha.encrypt_in_place(&mut buf, &NONCE_12[..], &[]);
        });
        results.push(("aead", size, "ChaCha20-P1305-encrypt", mbs));

        let mut data = vec![0xA5u8; size];
        let tag = chacha.encrypt_in_place(&mut data, &NONCE_12[..], &[]);
        let mbs = benchmark("ChaCha20-P1305-decrypt", size, || {
            let mut buf = data.clone();
            let _ = chacha.decrypt_in_place(&mut buf, &NONCE_12[..], &[], tag.as_ref());
        });
        results.push(("aead", size, "ChaCha20-P1305-decrypt", mbs));

        let mbs = benchmark("ChaCha20-BLAKE3-encrypt", size, || {
            let mut buf = vec![0xA5u8; size];
            let _tag = chacha_blake3.encrypt_in_place(&mut buf, &NONCE_32[..], &[]);
        });
        results.push(("aead", size, "ChaCha20-B3-encrypt", mbs));

        let mut data = vec![0xA5u8; size];
        let tag = chacha_blake3.encrypt_in_place(&mut data, &NONCE_32[..], &[]);
        let mbs = benchmark("ChaCha20-BLAKE3-decrypt", size, || {
            let mut buf = data.clone();
            let _ = chacha_blake3.decrypt_in_place(&mut buf, &NONCE_32[..], &[], tag.as_ref());
        });
        results.push(("aead", size, "ChaCha20-B3-decrypt", mbs));

        let mbs = benchmark("ChaCha12-BLAKE3-encrypt", size, || {
            let mut buf = vec![0xA5u8; size];
            let _tag = chacha12_blake3.encrypt_in_place(&mut buf, &NONCE_32[..], &[]);
        });
        results.push(("aead", size, "ChaCha12-B3-encrypt", mbs));

        let mut data = vec![0xA5u8; size];
        let tag = chacha12_blake3.encrypt_in_place(&mut data, &NONCE_32[..], &[]);
        let mbs = benchmark("ChaCha12-BLAKE3-decrypt", size, || {
            let mut buf = data.clone();
            let _ = chacha12_blake3.decrypt_in_place(&mut buf, &NONCE_32[..], &[], tag.as_ref());
        });
        results.push(("aead", size, "ChaCha12-B3-decrypt", mbs));

        let mbs = benchmark("Ascon-AEAD128-encrypt", size, || {
            let mut buf = vec![0xA5u8; size];
            let _tag = ascon_aead.encrypt_in_place(&mut buf, &NONCE_16[..], &[]);
        });
        results.push(("aead", size, "Ascon-AEAD128-encrypt", mbs));

        let mut data = vec![0xA5u8; size];
        let tag = ascon_aead.encrypt_in_place(&mut data, &NONCE_16[..], &[]);
        let mbs = benchmark("Ascon-AEAD128-decrypt", size, || {
            let mut buf = data.clone();
            let _ = ascon_aead.decrypt_in_place(&mut buf, &NONCE_16[..], &[], tag.as_ref());
        });
        results.push(("aead", size, "Ascon-AEAD128-decrypt", mbs));

        eprintln!();
    }
}

fn print_results(results: &[(&str, usize, &str, f64)]) {
    println!("\n\n========== SUMMARY ==========");
    println!("{:<24} {:>10} {:>16}", "Algorithm", "Size (B)", "Throughput");
    println!("{:-<24} {:-<10} {:-<16}", "", "", "");

    for &(_cat, size, name, mbs) in results {
        println!("{:<24} {:>10} {:>10.1} MB/s", name, size, mbs);
    }
}
