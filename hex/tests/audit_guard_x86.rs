//! Guard-page tests for the AVX2 hex kernels (x86_64 Linux), built with
//! `-C target-feature=+avx2`.

#![cfg(all(target_os = "linux", target_arch = "x86_64", target_feature = "avx2"))]

use hex::{Alphabet, decode_into, decode_into_constant_time, encode_into, encode_into_constant_time};

const PAGE: usize = 4096;

unsafe fn sys_mmap(len: usize) -> *mut u8 {
    let minus_one: isize = -1;
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") 9isize => ret,
            in("rdi") 0usize,
            in("rsi") len,
            in("rdx") 0b011usize,
            in("r10") 0x22usize,
            in("r8") minus_one,
            in("r9") 0usize,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack)
        );
    }
    assert!(ret > 0, "mmap failed: {ret}");
    ret as *mut u8
}

unsafe fn sys_mprotect(addr: *mut u8, len: usize, prot: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") 10isize => ret,
            in("rdi") addr,
            in("rsi") len,
            in("rdx") prot,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack)
        );
    }
    ret
}

struct Guarded {
    base: *mut u8,
    data: *mut u8,
    len: usize,
}

impl Guarded {
    fn new(len: usize) -> Self {
        assert!(len <= PAGE);
        unsafe {
            let base = sys_mmap(2 * PAGE);
            assert_eq!(sys_mprotect(base.add(PAGE), PAGE, 0), 0, "mprotect failed");
            Self {
                base,
                data: base.add(PAGE - len),
                len,
            }
        }
    }

    fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.data, self.len) }
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
    }
}

impl Drop for Guarded {
    fn drop(&mut self) {
        unsafe {
            let _ = sys_mprotect(self.base, PAGE, 0b011);
            let _ = sys_mprotect(self.base, 2 * PAGE, 0);
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
    for alphabet in ALPHABETS {
        for &len in &SIZES {
            let data = pattern(len, len as u64 + 1);
            let mut expected = vec![0u8; len * 2];
            encode_into_constant_time(&mut expected, &data, alphabet).unwrap();
            let mut guard = Guarded::new(len);
            guard.as_mut_slice().copy_from_slice(&data);
            let mut out = vec![0u8; len * 2];
            encode_into(&mut out, guard.as_slice(), alphabet).unwrap();
            assert_eq!(out, expected, "guard encode alphabet={alphabet:?} len={len}");
        }
    }
}

#[test]
fn encode_output_writes_stop_at_slice_end() {
    for alphabet in ALPHABETS {
        for &len in &SIZES {
            let data = pattern(len, len as u64 + 2);
            let mut expected = vec![0u8; len * 2];
            encode_into_constant_time(&mut expected, &data, alphabet).unwrap();
            let mut guard = Guarded::new(len * 2);
            encode_into(guard.as_mut_slice(), &data, alphabet).unwrap();
            assert_eq!(guard.as_slice(), &expected[..], "guard encode out len={len}");
        }
    }
}

#[test]
fn decode_input_reads_stop_at_slice_end() {
    for &raw_len in &SIZES {
        let raw = pattern(raw_len, raw_len as u64 + 3);
        let mut encoded = vec![0u8; raw_len * 2];
        encode_into_constant_time(&mut encoded, &raw, Alphabet::Lower).unwrap();
        let mut guard = Guarded::new(encoded.len());
        guard.as_mut_slice().copy_from_slice(&encoded);
        let mut out = vec![0u8; raw_len];
        decode_into(&mut out, guard.as_slice()).unwrap();
        assert_eq!(out, raw, "guard decode raw_len={raw_len}");
    }
}

#[test]
fn decode_output_writes_stop_at_slice_end() {
    for &raw_len in &SIZES {
        let raw = pattern(raw_len, raw_len as u64 + 4);
        let mut encoded = vec![0u8; raw_len * 2];
        encode_into_constant_time(&mut encoded, &raw, Alphabet::Upper).unwrap();
        let mut guard = Guarded::new(raw_len);
        decode_into(guard.as_mut_slice(), &encoded).unwrap();
        assert_eq!(guard.as_slice(), &raw[..], "guard decode out raw_len={raw_len}");

        let mut guard = Guarded::new(raw_len);
        decode_into_constant_time(guard.as_mut_slice(), &encoded).unwrap();
        assert_eq!(guard.as_slice(), &raw[..], "guard ct out raw_len={raw_len}");
    }
}
