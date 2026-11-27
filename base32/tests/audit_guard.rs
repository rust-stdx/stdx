//! Guard-page tests for the NEON base32 kernels: the input slice ends exactly
//! at an unmapped page, so any read past the logical end faults. Output slices
//! are likewise placed against a guard page so any store past the exact
//! expected length faults. These tests run on aarch64 Linux only.

#![cfg(all(target_os = "linux", target_arch = "aarch64"))]

use base32::{Alphabet, decode_into, decode_into_constant_time, encode_into, encode_into_constant_time};

mod page_guard {
    pub const PAGE: usize = 4096;

    pub struct Guarded {
        data: *mut u8,
        len: usize,
    }

    unsafe fn sys_mmap(len: usize) -> *mut u8 {
        let ret: u64;
        unsafe {
            core::arch::asm!(
                "svc 0",
                in("x8") 222u64, // mmap
                inout("x0") 0u64 => ret,
                in("x1") len as u64,
                in("x2") 3u64,      // PROT_READ | PROT_WRITE
                in("x3") 0x22u64,   // MAP_PRIVATE | MAP_ANONYMOUS
                in("x4") (-1i64) as u64,
                in("x5") 0u64,
                options(nostack),
            );
        }
        let signed = ret as i64;
        assert!(signed > 0, "mmap failed: {signed}");
        ret as *mut u8
    }

    unsafe fn sys_mprotect(addr: *mut u8, len: usize, prot: u64) {
        let ret: u64;
        unsafe {
            core::arch::asm!(
                "svc 0",
                in("x8") 226u64, // mprotect
                inout("x0") addr as u64 => ret,
                in("x1") len as u64,
                in("x2") prot,
                options(nostack),
            );
        }
        assert_eq!(ret as i64, 0, "mprotect failed");
    }

    impl Guarded {
        pub fn new(len: usize) -> Self {
            assert!(len <= PAGE);
            unsafe {
                let base = sys_mmap(2 * PAGE);
                sys_mprotect(base.add(PAGE), PAGE, 0);
                Self {
                    data: base.add(PAGE - len),
                    len,
                }
            }
        }

        pub fn as_slice(&self) -> &[u8] {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }

        pub fn as_mut_slice(&mut self) -> &mut [u8] {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

fn pattern(len: usize, salt: u64) -> Vec<u8> {
    let mut x = salt ^ 0x9E37_79B9_7F4A_7C15;
    let mut v = Vec::with_capacity(len);
    for _ in 0..len {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        v.push((x >> 24) as u8);
    }
    v
}

fn padded(a: Alphabet) -> bool {
    matches!(
        a,
        Alphabet::Rfc4648 | Alphabet::Rfc4648Lower | Alphabet::Rfc4648Hex | Alphabet::Rfc4648HexLower
    )
}

const ALPHABETS: &[Alphabet] = &[
    Alphabet::Rfc4648,
    Alphabet::Rfc4648NoPadding,
    Alphabet::Rfc4648Lower,
    Alphabet::Rfc4648Hex,
    Alphabet::Crockford,
];

#[test]
fn encode_input_reads_stop_at_slice_end() {
    use page_guard::Guarded;
    for &a in ALPHABETS {
        for &len in &[40usize, 41, 49, 50, 59, 60, 61, 79, 80, 81, 99, 100, 101, 127, 128, 300] {
            let data = pattern(len, len as u64 + 1);
            let mut guard = Guarded::new(len);
            guard.as_mut_slice().copy_from_slice(&data);
            let mut expected = vec![0u8; base32::encoded_length(len, padded(a)).unwrap()];
            encode_into_constant_time(&mut expected, &data, a).unwrap();
            let mut out = vec![0u8; expected.len()];
            encode_into(&mut out, guard.as_slice(), a).unwrap();
            assert_eq!(out, expected, "guarded encode alphabet={a:?} len={len}");
        }
    }
}

#[test]
fn encode_output_writes_stop_at_slice_end() {
    use page_guard::Guarded;
    for &a in ALPHABETS {
        for &len in &[40usize, 41, 49, 50, 59, 60, 61, 79, 80, 81, 99, 100, 101, 127, 128, 300] {
            let data = pattern(len, len as u64 + 2);
            let mut expected = vec![0u8; base32::encoded_length(len, padded(a)).unwrap()];
            encode_into_constant_time(&mut expected, &data, a).unwrap();
            let mut guard = Guarded::new(expected.len());
            encode_into(guard.as_mut_slice(), &data, a).unwrap();
            assert_eq!(guard.as_slice(), &expected[..], "guarded encode out alphabet={a:?} len={len}");
        }
    }
}

#[test]
fn decode_input_reads_stop_at_slice_end() {
    use page_guard::Guarded;
    for &a in ALPHABETS {
        for &raw_len in &[40usize, 41, 49, 50, 63, 64, 65, 79, 80, 99, 100, 127, 128, 199, 200] {
            let raw = pattern(raw_len, raw_len as u64 + 3);
            let mut encoded = vec![0u8; base32::encoded_length(raw_len, padded(a)).unwrap()];
            encode_into_constant_time(&mut encoded, &raw, a).unwrap();
            if encoded.len() > page_guard::PAGE {
                continue;
            }
            let mut guard = Guarded::new(encoded.len());
            guard.as_mut_slice().copy_from_slice(&encoded);
            let mut out = vec![0u8; raw_len];
            decode_into(&mut out, guard.as_slice(), a).unwrap();
            assert_eq!(out, raw, "guarded decode alphabet={a:?} raw_len={raw_len}");
        }
    }
}

#[test]
fn decode_output_writes_stop_at_slice_end() {
    use page_guard::Guarded;
    for &a in ALPHABETS {
        for &raw_len in &[40usize, 41, 49, 50, 63, 64, 65, 79, 80, 99, 100, 127, 128, 199, 200] {
            let raw = pattern(raw_len, raw_len as u64 + 4);
            let mut encoded = vec![0u8; base32::encoded_length(raw_len, padded(a)).unwrap()];
            encode_into_constant_time(&mut encoded, &raw, a).unwrap();
            let mut guard = Guarded::new(raw_len);
            decode_into(guard.as_mut_slice(), &encoded, a).unwrap();
            assert_eq!(
                guard.as_slice(),
                &raw[..],
                "guarded decode out alphabet={a:?} raw_len={raw_len}"
            );

            // Also drive the constant-time path into a guarded exact buffer.
            let mut guard = Guarded::new(raw_len);
            decode_into_constant_time(guard.as_mut_slice(), &encoded, a).unwrap();
            assert_eq!(
                guard.as_slice(),
                &raw[..],
                "guarded ct decode out alphabet={a:?} raw_len={raw_len}"
            );
        }
    }
}
