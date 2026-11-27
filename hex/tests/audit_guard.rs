//! Guard-page tests for the NEON hex kernels (aarch64 Linux only).

#![cfg(all(target_os = "linux", target_arch = "aarch64"))]

use hex::{Alphabet, decode_into, decode_into_constant_time, encode_into, encode_into_constant_time};

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
                in("x8") 222u64,
                inout("x0") 0u64 => ret,
                in("x1") len as u64,
                in("x2") 3u64,
                in("x3") 0x22u64,
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
                in("x8") 226u64,
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

const ALPHABETS: [Alphabet; 2] = [Alphabet::Lower, Alphabet::Upper];
const SIZES: [usize; 16] = [16, 17, 31, 32, 33, 47, 48, 63, 64, 65, 95, 96, 127, 128, 255, 300];

#[test]
fn encode_input_reads_stop_at_slice_end() {
    use page_guard::Guarded;
    for alphabet in ALPHABETS {
        for &len in &SIZES {
            let data = pattern(len, len as u64 + 1);
            let mut expected = vec![0u8; len * 2];
            encode_into_constant_time(&mut expected, &data, alphabet).unwrap();
            let mut guard = Guarded::new(len);
            guard.as_mut_slice().copy_from_slice(&data);
            let mut out = vec![0u8; len * 2];
            encode_into(&mut out, guard.as_slice(), alphabet).unwrap();
            assert_eq!(out, expected, "guarded encode alphabet={alphabet:?} len={len}");
        }
    }
}

#[test]
fn encode_output_writes_stop_at_slice_end() {
    use page_guard::Guarded;
    for alphabet in ALPHABETS {
        for &len in &SIZES {
            let data = pattern(len, len as u64 + 2);
            let mut expected = vec![0u8; len * 2];
            encode_into_constant_time(&mut expected, &data, alphabet).unwrap();
            let mut guard = Guarded::new(len * 2);
            encode_into(guard.as_mut_slice(), &data, alphabet).unwrap();
            assert_eq!(guard.as_slice(), &expected[..], "guarded encode out len={len}");
        }
    }
}

#[test]
fn decode_input_reads_stop_at_slice_end() {
    use page_guard::Guarded;
    for &raw_len in &SIZES {
        let raw = pattern(raw_len, raw_len as u64 + 3);
        let mut encoded = vec![0u8; raw_len * 2];
        encode_into_constant_time(&mut encoded, &raw, Alphabet::Lower).unwrap();
        let mut guard = Guarded::new(encoded.len());
        guard.as_mut_slice().copy_from_slice(&encoded);
        let mut out = vec![0u8; raw_len];
        decode_into(&mut out, guard.as_slice()).unwrap();
        assert_eq!(out, raw, "guarded decode raw_len={raw_len}");
    }
}

#[test]
fn decode_output_writes_stop_at_slice_end() {
    use page_guard::Guarded;
    for &raw_len in &SIZES {
        let raw = pattern(raw_len, raw_len as u64 + 4);
        let mut encoded = vec![0u8; raw_len * 2];
        encode_into_constant_time(&mut encoded, &raw, Alphabet::Upper).unwrap();
        let mut guard = Guarded::new(raw_len);
        decode_into(guard.as_mut_slice(), &encoded).unwrap();
        assert_eq!(guard.as_slice(), &raw[..], "guarded decode out raw_len={raw_len}");

        let mut guard = Guarded::new(raw_len);
        decode_into_constant_time(guard.as_mut_slice(), &encoded).unwrap();
        assert_eq!(guard.as_slice(), &raw[..], "guarded ct decode out raw_len={raw_len}");
    }
}
