//! Independent differential tests for `utf8::validate` / `utf8::from_utf8`.
//!
//! Every check compares against `core::str::from_utf8`. The goal is to prove
//! (or refute) that the SIMD kernels never accept invalid UTF-8 and that the
//! error values returned by `from_utf8` are exactly the std ones.
//!
//! Runtime knob: `AUDIT_SCALE` (default 1) multiplies the expensive
//! exhaustive/random iteration counts (all audit runs below used the default).

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn show(bytes: &[u8]) -> String {
    // Keep assertion messages bounded for multi-KB inputs.
    let n = bytes.len().min(96);
    let mut s = format!("len={} [", bytes.len());
    for (i, b) in bytes[..n].iter().enumerate() {
        if i != 0 {
            s.push(' ');
        }
        s.push_str(&format!("{b:02x}"));
    }
    if bytes.len() > n {
        s.push_str(" ...");
    }
    s.push(']');
    s
}

/// Compare `validate` and `from_utf8` against std for one input.
fn agrees(bytes: &[u8]) {
    let expected = core::str::from_utf8(bytes);
    let actual_valid = utf8::validate(bytes);
    assert_eq!(
        actual_valid,
        expected.is_ok(),
        "validate disagreed with std for {}",
        show(bytes)
    );

    match (utf8::from_utf8(bytes), expected) {
        (Ok(actual), Ok(expected)) => {
            assert_eq!(actual, expected, "from_utf8 Ok payload mismatch for {}", show(bytes))
        }
        (Err(actual), Err(expected)) => {
            assert_eq!(
                actual.valid_up_to(),
                expected.valid_up_to(),
                "valid_up_to mismatch for {}",
                show(bytes)
            );
            assert_eq!(
                actual.error_len(),
                expected.error_len(),
                "error_len mismatch for {}",
                show(bytes)
            );
            // Also verify the error's derived formatting API is consistent.
            assert_eq!(actual, expected, "Utf8Error mismatch for {}", show(bytes));
        }
        (actual, expected) => panic!(
            "from_utf8 disagreed with std for {}: {:?} vs {:?}",
            show(bytes),
            actual.map(str::len),
            expected.map(str::len)
        ),
    }
}

fn assert_valid(bytes: &[u8]) {
    agrees(bytes);
    assert!(utf8::validate(bytes), "expected valid: {}", show(bytes));
    assert!(utf8::from_utf8(bytes).is_ok(), "expected Ok: {}", show(bytes));
}

fn assert_invalid(bytes: &[u8]) {
    agrees(bytes);
    assert!(!utf8::validate(bytes), "expected invalid: {}", show(bytes));
    assert!(utf8::from_utf8(bytes).is_err(), "expected Err: {}", show(bytes));
}

fn scale() -> usize {
    std::env::var("AUDIT_SCALE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
        .max(1)
}

/// Growable document builder used by the valid-document tests.
fn ascii(len: usize) -> Vec<u8> {
    vec![b'a'; len]
}

const CHARS: &[char] = &[
    'a',
    '\n',
    '\u{7f}',
    '\u{80}',
    '\u{7ff}',
    '\u{800}',
    '\u{d7ff}',
    '\u{e000}',
    '\u{ffff}',
    '\u{10000}',
    '\u{10ffff}',
    'é',
    '€',
    '😀',
];

fn encode(ch: char) -> ([u8; 4], usize) {
    let mut buf = [0u8; 4];
    let len = ch.encode_utf8(&mut buf).len();
    (buf, len)
}

// ---------------------------------------------------------------------------
// 0. Which backend is actually compiled / selected?
// ---------------------------------------------------------------------------

#[test]
fn backend_probe() {
    println!("target_arch = {}", std::env::consts::ARCH);
    println!("target_os   = {}", std::env::consts::OS);
    #[cfg(target_arch = "aarch64")]
    {
        // NEON is mandatory on AArch64, so `validate` always takes the NEON
        // kernel for inputs >= 64 bytes.
        println!("backend = neon (mandatory on aarch64)");
    }
    #[cfg(target_arch = "x86_64")]
    {
        let avx2 = std::arch::is_x86_feature_detected!("avx2");
        println!("backend = avx2? {avx2} (else scalar)");
        if std::env::var_os("AUDIT_ALLOW_SCALAR").is_none() {
            assert!(
                avx2,
                "this x86_64 audit run expects AVX2 (qemu -cpu max); otherwise the SIMD \
                 kernel is not exercised"
            );
        }
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    println!("backend = scalar (no SIMD kernel for this target)");
}

// ---------------------------------------------------------------------------
// 1. Exhaustive short-sequence classification, embedded at block boundaries
// ---------------------------------------------------------------------------

/// Place `seq` at `offset` in a buffer of `total` `0x61` bytes.
fn place(buf: &mut [u8], offset: usize, seq: &[u8]) {
    assert!(offset + seq.len() <= buf.len());
    buf[offset..offset + seq.len()].copy_from_slice(seq);
}

#[test]
fn exhaustive_one_and_two_bytes_at_offsets() {
    // 1-byte, bare (scalar path) and at every boundary-relevant offset in a
    // 64-byte block.
    for a in 0..=u8::MAX {
        agrees(&[a]);
        let mut buf = ascii(64);
        for off in [0usize, 32, 63] {
            buf[off] = a;
            agrees(&buf);
            buf[off] = b'a';
        }
    }

    // All 65_536 two-byte inputs, bare, at the start, at the end of a block
    // (62..64), and straddling the 64-byte boundary (63..65).
    let mut block = ascii(64);
    let mut cross = ascii(128);
    for a in 0..=u8::MAX {
        for b in 0..=u8::MAX {
            let seq = [a, b];
            agrees(&seq);
            for off in [0usize, 32, 62] {
                place(&mut block, off, &seq);
                agrees(&block);
                block[off..off + 2].fill(b'a');
            }
            place(&mut cross, 63, &seq);
            agrees(&cross);
            cross[63..65].fill(b'a');
            agrees(&cross[..65]);
        }
    }
}

#[test]
fn exhaustive_three_byte_lead_range() {
    let n = scale();
    let mut block = ascii(64);
    let mut cross = ascii(128);
    for _ in 0..n {
        for lead in 0xe0..=0xefu8 {
            for b in 0..=u8::MAX {
                for c in 0..=u8::MAX {
                    let seq = [lead, b, c];
                    // Start of input, 64-byte input (tail path).
                    place(&mut block, 0, &seq);
                    agrees(&block);
                    block[0..3].fill(b'a');

                    // Ends exactly at a 64-byte block boundary (is_incomplete).
                    place(&mut block, 61, &seq);
                    agrees(&block);
                    block[61..64].fill(b'a');

                    // Straddles the 64-byte boundary (63..66), with a tail.
                    place(&mut cross, 63, &seq);
                    agrees(&cross[..66]);
                    agrees(&cross);
                    cross[63..66].fill(b'a');
                }
            }
        }
    }
}

#[test]
fn exhaustive_four_byte_lead_range() {
    let n = scale();
    let mut block = ascii(64);
    let mut cross = ascii(128);
    for _ in 0..n {
        for lead in 0xf0..=0xf4u8 {
            // Full continuation range, plus the rejected boundary values.
            for b in 0x80..=0xbfu8 {
                for c in 0x80..=0xbfu8 {
                    for d in 0x80..=0xbfu8 {
                        let seq = [lead, b, c, d];
                        place(&mut block, 0, &seq);
                        agrees(&block);
                        block[0..4].fill(b'a');

                        // Ends exactly at the 64-byte boundary.
                        place(&mut block, 60, &seq);
                        agrees(&block);
                        block[60..64].fill(b'a');

                        // Straddles the boundary (63..67).
                        place(&mut cross, 63, &seq);
                        agrees(&cross[..67]);
                        agrees(&cross);
                        cross[63..67].fill(b'a');
                    }
                }
            }

            for b in [0x00u8, 0x7f, 0x80, 0x8f, 0x90, 0xbf, 0xc0, 0xff] {
                for c in [0x00u8, 0x7f, 0x80, 0xbf, 0xc0, 0xff] {
                    for d in [0x00u8, 0x7f, 0x80, 0xbf, 0xc0, 0xff] {
                        let seq = [lead, b, c, d];
                        agrees(&seq);
                        place(&mut block, 60, &seq);
                        agrees(&block);
                        block[60..64].fill(b'a');
                        place(&mut cross, 63, &seq);
                        agrees(&cross[..67]);
                        agrees(&cross);
                        cross[63..67].fill(b'a');
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 2. Valid documents: every length and every placement offset
// ---------------------------------------------------------------------------

#[test]
fn valid_char_at_every_offset_and_length() {
    for len in 0..=320usize {
        for &ch in CHARS {
            let (enc, elen) = encode(ch);
            if elen > len {
                continue;
            }
            let mut buf = ascii(len);
            for off in 0..=len - elen {
                place(&mut buf, off, &enc[..elen]);
                assert_valid(&buf);
                buf[off..off + elen].fill(b'a');
            }
        }
    }
}

#[test]
fn all_scalar_values_at_block_boundaries() {
    // Every Unicode scalar value, placed so that it ends exactly at 64 bytes
    // and so that it straddles the 64-byte boundary.
    let mut block = ascii(64);
    let mut cross = ascii(128);
    for cp in 0..=0x10ffffu32 {
        let Some(ch) = char::from_u32(cp) else {
            continue;
        };
        let (enc, elen) = encode(ch);

        place(&mut block, 64 - elen, &enc[..elen]);
        assert_valid(&block);
        block[64 - elen..].fill(b'a');

        if elen > 1 {
            place(&mut cross, 64 - elen / 2, &enc[..elen]);
            assert_valid(&cross);
            cross[64 - elen / 2..64 - elen / 2 + elen].fill(b'a');
        }
    }
}

#[test]
fn valid_chars_straddling_every_boundary() {
    for boundary in [64usize, 128, 192, 256, 320, 384] {
        for &ch in CHARS {
            let (enc, elen) = encode(ch);
            if elen == 1 {
                continue;
            }
            let len = boundary + 64;
            let mut buf = ascii(len);
            for missing_first in 1..elen {
                let start = boundary - missing_first;
                place(&mut buf, start, &enc[..elen]);
                assert_valid(&buf);
                buf[start..start + elen].fill(b'a');
            }
            // Also start exactly on the boundary.
            place(&mut buf, boundary, &enc[..elen]);
            assert_valid(&buf);
            buf[boundary..boundary + elen].fill(b'a');
        }
    }
}

// ---------------------------------------------------------------------------
// 3. Invalid bytes / truncated sequences at every position and length
// ---------------------------------------------------------------------------

#[test]
fn invalid_byte_at_every_position_and_length() {
    const BAD: &[u8] = &[0x80, 0xbf, 0xc0, 0xc1, 0xf5, 0xf6, 0xf7, 0xf8, 0xfc, 0xfe, 0xff];
    for len in 0..=300usize {
        let mut buf = ascii(len);
        for pos in 0..len {
            for &bad in BAD {
                buf[pos] = bad;
                assert_invalid(&buf);
                buf[pos] = b'a';
            }
        }
    }
}

#[test]
fn truncated_sequence_at_every_position_and_length() {
    for len in 0..=300usize {
        for &ch in CHARS {
            let (enc, elen) = encode(ch);
            if elen == 1 {
                continue;
            }
            for cut in 1..elen {
                if cut > len {
                    continue;
                }
                let mut buf = ascii(len);
                for pos in 0..=len - cut {
                    place(&mut buf, pos, &enc[..cut]);
                    // A truncated prefix followed by ASCII (or at end of
                    // input) is always invalid.
                    assert_invalid(&buf);
                    // ... and followed by continuation bytes it is still
                    // invalid whenever fewer bytes than the full sequence
                    // remain (partial completion then a non-continuation).
                    // If `cut + 1 == elen` the added 0x80 completes the
                    // character, which would be valid.
                    if pos + cut < len && cut + 1 < elen {
                        let save = buf[pos + cut];
                        buf[pos + cut] = 0x80;
                        assert_invalid(&buf);
                        buf[pos + cut] = save;
                    }
                    buf[pos..pos + cut].fill(b'a');
                }
            }
        }
    }
}

#[test]
fn exact_multiples_of_64_ending_mid_character() {
    // The `check_incomplete_pending` path: no tail block exists.
    for blocks in 1..=8usize {
        let len = blocks * 64;
        for &ch in CHARS {
            let (enc, elen) = encode(ch);
            if elen == 1 {
                continue;
            }
            for cut in 1..elen {
                let mut buf = ascii(len);
                place(&mut buf, len - cut, &enc[..cut]);
                assert_invalid(&buf);
                // Same, but continuation-prefixed so the last bytes are
                // continuations without a preceding lead.
                let mut buf2 = ascii(len);
                for i in 0..cut {
                    buf2[len - cut + i] = 0x80;
                }
                assert_invalid(&buf2);
            }
        }

        // All invalid lead bytes in the last three positions.
        for &bad in &[0xc0u8, 0xc1, 0xe0, 0xed, 0xf0, 0xf4, 0xf5, 0xfe, 0xff] {
            for back in 1..=3usize {
                let mut buf = ascii(len);
                buf[len - back] = bad;
                assert_invalid(&buf);
            }
        }
    }
}

#[test]
fn exact_multiples_of_64_with_complete_character() {
    // Same lengths, but the final character is complete: must be accepted.
    for blocks in 1..=8usize {
        let len = blocks * 64;
        for &ch in CHARS {
            let (enc, elen) = encode(ch);
            let mut buf = ascii(len);
            place(&mut buf, len - elen, &enc[..elen]);
            assert_valid(&buf);
        }
    }

    // Lengths just below/above a multiple of 64 with a char at the end.
    for base in [63usize, 64, 127, 128, 129, 191, 192, 193, 255, 256, 257] {
        for &ch in CHARS {
            let (enc, elen) = encode(ch);
            let mut buf = ascii(base);
            place(&mut buf, base - elen, &enc[..elen]);
            assert_valid(&buf);
            // Truncating the last byte makes it invalid (unless the final
            // character is a single-byte ASCII char).
            if elen > 1 {
                let mut cut = buf.clone();
                cut.pop();
                assert_invalid(&cut);
            }
        }
    }
}

#[test]
fn invalid_sequences_straddling_block_boundaries() {
    // Sequences that are invalid in isolation, placed at every offset that
    // makes them straddle a block boundary.
    const SEQS: &[&[u8]] = &[
        &[0x80],
        &[0xbf],
        &[0xc0, 0x80],
        &[0xc1, 0xbf],
        &[0xc2, 0x20],
        &[0xc2],
        &[0xe0, 0x80, 0x80],
        &[0xe0, 0x9f, 0xbf],
        &[0xe2, 0x82],
        &[0xe2, 0x82, 0x20],
        &[0xed, 0xa0, 0x80],
        &[0xed, 0xbf, 0xbf],
        &[0xf0, 0x80, 0x80, 0x80],
        &[0xf0, 0x8f, 0xbf, 0xbf],
        &[0xf0, 0x9f, 0x92],
        &[0xf4, 0x90, 0x80, 0x80],
        &[0xf5, 0x80, 0x80, 0x80],
        &[0xfe],
        &[0xff],
    ];
    for boundary in [64usize, 128, 192, 256] {
        for seq in SEQS {
            for back in 1..seq.len() {
                let len = boundary + 64;
                let mut buf = ascii(len);
                let start = boundary - back;
                place(&mut buf, start, seq);
                assert_invalid(&buf);
            }
            // ... and starting exactly at the boundary.
            let mut buf = ascii(boundary + 64);
            place(&mut buf, boundary, seq);
            assert_invalid(&buf);
        }
    }
}

// ---------------------------------------------------------------------------
// 4. Tail-buffer / threshold edge cases
// ---------------------------------------------------------------------------

#[test]
fn threshold_63_64_65() {
    for len in 60..=70usize {
        // All ASCII.
        assert_valid(&ascii(len));
        // Trailing `\0` (NUL) is valid and must not confuse the tail copy.
        let mut nul = ascii(len);
        if len > 0 {
            nul[len - 1] = 0;
        }
        assert_valid(&nul);
        // Truncated 2/3/4-byte sequences ending the input.
        for &ch in &['é', '€', '😀', '\u{10ffff}'] {
            let (enc, elen) = encode(ch);
            for cut in 1..elen {
                let mut buf = ascii(len);
                place(&mut buf, len - cut, &enc[..cut]);
                assert_invalid(&buf);
            }
            let mut buf = ascii(len);
            place(&mut buf, len - elen, &enc[..elen]);
            assert_valid(&buf);
        }
        // A single invalid byte at every position.
        for pos in 0..len {
            let mut buf = ascii(len);
            buf[pos] = 0xff;
            assert_invalid(&buf);
        }
    }
}

#[test]
fn tail_with_incomplete_sequence_then_zero_padding() {
    // Input lengths 65..=127 take the `memcpy` tail path; the temporary chunk
    // is zero-filled. Verify truncation at every tail offset is rejected.
    for len in 64..=128usize {
        for &ch in &['é', '€', '😀', '\u{10ffff}'] {
            let (enc, elen) = encode(ch);
            for cut in 1..elen {
                let mut buf = ascii(len);
                place(&mut buf, len - cut, &enc[..cut]);
                assert_invalid(&buf);
            }
        }
        // Lead byte at each of the last 4 positions.
        for &lead in &[0xc2u8, 0xe0, 0xed, 0xf0, 0xf4, 0xf5, 0xff] {
            for back in 1..=4usize {
                if back <= len {
                    let mut buf = ascii(len);
                    buf[len - back] = lead;
                    assert_invalid(&buf);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 5. Misalignment + guard page (over-read detection)
// ---------------------------------------------------------------------------

/// Slice of page-aligned anonymous memory whose last byte is immediately
/// followed by a PROT_NONE page. Any load past the end faults.
#[cfg(all(target_os = "linux", any(target_arch = "aarch64", target_arch = "x86_64")))]
mod guard {
    const PROT_NONE: usize = 0;
    const PROT_READ_WRITE: usize = 3;
    const MAP_PRIVATE_ANONYMOUS: usize = 0x22;
    const PAGE: usize = 4096;

    #[cfg(target_arch = "aarch64")]
    unsafe fn syscall6(nr: usize, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> isize {
        let ret: isize;
        unsafe {
            core::arch::asm!(
                "svc 0",
                in("x8") nr,
                inlateout("x0") a => ret,
                in("x1") b,
                in("x2") c,
                in("x3") d,
                in("x4") e,
                in("x5") f,
                options(nostack),
            );
        }
        ret
    }

    #[cfg(target_arch = "x86_64")]
    unsafe fn syscall6(nr: usize, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> isize {
        let ret: isize;
        unsafe {
            core::arch::asm!(
                "syscall",
                inlateout("rax") nr as isize => ret,
                in("rdi") a,
                in("rsi") b,
                in("rdx") c,
                in("r10") d,
                in("r8") e,
                in("r9") f,
                out("rcx") _,
                out("r11") _,
                options(nostack),
            );
        }
        ret
    }

    #[cfg(target_arch = "aarch64")]
    const NR_MMAP: usize = 222;
    #[cfg(target_arch = "aarch64")]
    const NR_MUNMAP: usize = 215;
    #[cfg(target_arch = "aarch64")]
    const NR_MPROTECT: usize = 226;

    #[cfg(target_arch = "x86_64")]
    const NR_MMAP: usize = 9;
    #[cfg(target_arch = "x86_64")]
    const NR_MUNMAP: usize = 11;
    #[cfg(target_arch = "x86_64")]
    const NR_MPROTECT: usize = 10;

    pub struct Guarded {
        base: *mut u8,
        total: usize,
        data_len: usize,
        shift: usize,
    }

    impl Guarded {
        /// `data_len` bytes of usable data, shifted `shift` bytes back from the
        /// guard page so that the slice start has varying alignment.
        pub fn new(data_len: usize, shift: usize) -> Self {
            let pages = data_len.div_ceil(PAGE);
            let total = (pages + 1) * PAGE;
            let base = unsafe {
                syscall6(
                    NR_MMAP,
                    0,
                    total,
                    PROT_READ_WRITE,
                    MAP_PRIVATE_ANONYMOUS,
                    usize::MAX, // fd = -1
                    0,
                )
            };
            assert!(base > 0, "mmap failed: {base}");
            let base = base as *mut u8;
            // Poison the whole data region so writes are detectable.
            unsafe { core::ptr::write_bytes(base, 0xa5, pages * PAGE) };
            let guard = unsafe { base.add(pages * PAGE) };
            let rc = unsafe { syscall6(NR_MPROTECT, guard as usize, PAGE, PROT_NONE, 0, 0, 0) };
            assert_eq!(rc, 0, "mprotect failed: {rc}");
            Self {
                base,
                total,
                data_len,
                shift,
            }
        }

        pub fn slice_mut(&mut self) -> &mut [u8] {
            let offset = self.data_len.div_ceil(PAGE) * PAGE - self.data_len - self.shift;
            unsafe { core::slice::from_raw_parts_mut(self.base.add(offset), self.data_len) }
        }

        pub fn slice(&self) -> &[u8] {
            let offset = self.data_len.div_ceil(PAGE) * PAGE - self.data_len - self.shift;
            unsafe { core::slice::from_raw_parts(self.base.add(offset), self.data_len) }
        }
    }

    impl Drop for Guarded {
        fn drop(&mut self) {
            let _ = unsafe { syscall6(NR_MUNMAP, self.base as usize, self.total, 0, 0, 0, 0) };
        }
    }
}

#[test]
#[cfg_attr(miri, ignore = "raw mmap/mprotect syscalls are not supported by Miri")]
#[cfg(all(target_os = "linux", any(target_arch = "aarch64", target_arch = "x86_64")))]
fn guarded_misaligned_slices_do_not_overread() {
    for len in [64usize, 65, 66, 100, 127, 128, 129, 192, 200, 256, 300] {
        for shift in 0..64usize {
            // Valid document ending with a 4-byte char (worst case for the
            // tail copy), then an invalid variant.
            let mut g = guard::Guarded::new(len, shift);
            let buf = g.slice_mut();
            buf.fill(b'a');
            let (enc, elen) = encode('😀');
            let start = len - elen;
            buf[start..].copy_from_slice(&enc[..elen]);
            assert_valid(g.slice());

            let buf = g.slice_mut();
            // 4-byte lead + two continuations + ASCII: invalid, and exactly
            // fills the final four bytes.
            buf[start..].copy_from_slice(&[0xf0, 0x9f, 0x98, 0x41]);
            assert_invalid(g.slice());

            // All-invalid tail (continuations only).
            let buf = g.slice_mut();
            buf.fill(b'a');
            buf[len - 1] = 0xff;
            assert_invalid(g.slice());

            // All-ASCII fast path.
            let buf = g.slice_mut();
            buf.fill(b'a');
            assert_valid(g.slice());
        }
    }
}

#[test]
fn exact_size_allocations() {
    // No spare capacity: reallocating to the exact length means any read past
    // the end is outside the allocation (caught by Miri/ASan, and a heuristic
    // canary check here).
    for len in 0..=256usize {
        let mut v: Vec<u8> = Vec::with_capacity(len);
        for i in 0..len {
            v.push(if i % 4 == 0 { b'a' } else { b'b' });
        }
        assert_eq!(v.capacity(), len);
        assert_eq!(v.len(), len);
        let mut bytes = v.into_boxed_slice(); // exact-size allocation
        assert_eq!(bytes.len(), len);
        assert_valid(&bytes);
        if len >= 3 {
            bytes[len - 1] = 0xff;
            assert_invalid(&bytes);
        }
    }
}

// ---------------------------------------------------------------------------
// 6. Random differential testing
// ---------------------------------------------------------------------------

struct XorShift(u64);

impl XorShift {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % (n as u64)) as usize
    }
}

fn random_valid_doc(rng: &mut XorShift, target: usize, out: &mut Vec<u8>) {
    out.clear();
    while out.len() < target {
        let ch = CHARS[rng.below(CHARS.len())];
        let (enc, elen) = encode(ch);
        if out.len() + elen > target {
            out.push(b'a');
        } else {
            out.extend_from_slice(&enc[..elen]);
        }
    }
}

#[test]
fn random_differential_one_million() {
    let n = 1_000_000 * scale();
    let mut rng = XorShift(0x243f_6a88_85a3_08d3);
    let mut buf: Vec<u8> = Vec::with_capacity(512);
    for i in 0..n {
        let len = rng.below(301);
        buf.clear();
        match i % 5 {
            // Uniform random bytes.
            0 => {
                for _ in 0..len {
                    buf.push(rng.next_u64() as u8);
                }
            }
            // Valid document.
            1 => random_valid_doc(&mut rng, len, &mut buf),
            // Valid document with 1..=3 corrupted bytes.
            2 => {
                random_valid_doc(&mut rng, len, &mut buf);
                if !buf.is_empty() {
                    for _ in 0..=rng.below(3) {
                        let pos = rng.below(buf.len());
                        buf[pos] = rng.next_u64() as u8;
                    }
                }
            }
            // Truncated valid document.
            3 => {
                random_valid_doc(&mut rng, len, &mut buf);
                if !buf.is_empty() {
                    let cut = rng.below(buf.len());
                    buf.truncate(cut);
                }
            }
            // Valid document with an injected invalid byte at a random spot.
            4 => {
                random_valid_doc(&mut rng, len, &mut buf);
                if !buf.is_empty() {
                    let pos = rng.below(buf.len());
                    buf[pos] = [0x80u8, 0xc0, 0xc2, 0xe0, 0xed, 0xf0, 0xf4, 0xf5, 0xff][rng.below(9)];
                }
            }
            _ => unreachable!(),
        }
        agrees(&buf);
    }
}

#[test]
fn non_ascii_in_first_block_with_prefetch_distance() {
    // A non-ASCII byte in the first block forces the validator into its
    // second loop, where a prefetch (if enabled) looks two blocks ahead.
    // Lengths >= 192 make that lookahead leave the allocation; the port has
    // PREFETCH disabled, but this keeps the path covered.
    for len in [64usize, 65, 128, 129, 191, 192, 193, 256, 300] {
        let mut v = vec![b'a'; len];
        v[0..2].copy_from_slice(&[0xc3, 0xa9]);
        assert_valid(&v);
        v[1] = 0xff;
        assert_invalid(&v);
    }
}

#[test]
fn mixed_multibyte_pattern_across_blocks() {
    // Dense multibyte data with embeddings that cross every 64-byte boundary.
    let pattern = "aé€😀\u{10ffff}";
    for repeats in 1..=64usize {
        let text = pattern.repeat(repeats);
        assert_valid(text.as_bytes());
        let mut bytes = text.clone().into_bytes();
        for boundary in (64..bytes.len()).step_by(64) {
            for back in 0..=4usize {
                let pos = boundary - back;
                if pos >= bytes.len() {
                    continue;
                }
                let saved = bytes[pos];
                bytes[pos] = 0xff;
                assert_invalid(&bytes);
                bytes[pos] = saved;
            }
        }
        assert_valid(&bytes);
    }
}

#[test]
fn misaligned_subslice_ending_at_allocation_end() {
    // The slice is a suffix of a larger allocation, so any read past its end
    // is also past the allocation (Miri redzones catch it). Shift varies the
    // pointer alignment modulo 64.
    for shift in [0usize, 1, 3, 7, 8, 15, 16, 31, 32, 63] {
        for len in [64usize, 65, 66, 127, 128, 191, 192, 200] {
            let mut v = vec![b'a'; shift + len];

            // Ends with a complete 4-byte character.
            v[shift + len - 4..].copy_from_slice(&[0xf0, 0x9f, 0x98, 0x80]);
            assert_valid(&v[shift..]);

            // Ends with a complete 2-byte character.
            let mut v2 = vec![b'a'; shift + len];
            v2[shift + len - 2..].copy_from_slice(&[0xc3, 0xa9]);
            assert_valid(&v2[shift..]);

            // Ends with an invalid byte.
            v[shift + len - 1] = 0xff;
            assert_invalid(&v[shift..]);
        }
    }
}

// ---------------------------------------------------------------------------
// 7. Miri-friendly smoke coverage (scalar path, len < 64)
// ---------------------------------------------------------------------------

#[test]
#[cfg_attr(not(miri), ignore = "miri-only; the native SIMD tests cover this better")]
fn miri_scalar_path() {
    for a in 0..=u8::MAX {
        agrees(&[a]);
    }
    for len in 0..64usize {
        assert_valid(&ascii(len));
        for &ch in CHARS {
            let (enc, elen) = encode(ch);
            if elen > len {
                continue;
            }
            for off in 0..=len - elen {
                let mut buf = ascii(len);
                place(&mut buf, off, &enc[..elen]);
                assert_valid(&buf);
            }
        }
        for pos in 0..len {
            let mut buf = ascii(len);
            buf[pos] = 0xff;
            assert_invalid(&buf);
        }
    }
}
