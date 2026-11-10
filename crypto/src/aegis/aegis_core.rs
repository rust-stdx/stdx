//! Shared, backend-independent AEGIS-256 core.
//!
//! The state is six 128-bit blocks, but a backend represents a *block* as one
//! or more SIMD lanes: AEGIS-256X2 stores two independent AEGIS-256 states in
//! the lanes of 256-bit registers, and AEGIS-256X4 stores four. All three
//! variants therefore use the same algorithm with a different [`BlockOps`]
//! implementation and parallelism degree `D`.
//!
//! Everything here is generic code; the only unsafe operations are the
//! backend [`BlockOps`] loads and stores. Backends must call these helpers from
//! a function carrying the matching `#[target_feature]` attribute; otherwise
//! LLVM cannot inline and schedule the AES round instructions, which costs up
//! to roughly a factor of eight during initialization.

use constant_time_eq::constant_time_eq;

/// Minimum message length for the split-state bulk loop. Measured on the
/// aarch64 backend: wins from 1 KiB upwards, neutral below it.
const BULK_THRESHOLD: usize = 1024;

/// AES round constants C0 and C1 (RFC 10032 §2).
pub(crate) const C0: [u8; 16] = [
    0x00, 0x01, 0x01, 0x02, 0x03, 0x05, 0x08, 0x0d, 0x15, 0x22, 0x37, 0x59, 0x90, 0xe9, 0x79, 0x62,
];
pub(crate) const C1: [u8; 16] = [
    0xdb, 0x3d, 0x18, 0x55, 0x6d, 0xc2, 0x2f, 0xf1, 0x20, 0x11, 0x31, 0x42, 0x73, 0xb5, 0x28, 0xdd,
];

/// A vector of `D` 128-bit AES blocks, held in a backend-native representation.
///
/// # Safety
///
/// `load` and `store` require a valid pointer to `D * 16` bytes. `ctx_all` and
/// `broadcast` must fill every lane; `fold` must combine all lanes with XOR.
pub(crate) trait BlockOps: Copy {
    /// Degree of parallelism: 1 for AEGIS-256, 2 for X2, 4 for X4.
    const D: usize;

    fn zero() -> Self;
    fn xor(self, rhs: Self) -> Self;
    fn and(self, rhs: Self) -> Self;

    /// One AES encryption round with a distinct round key per 128-bit lane.
    fn round(self, rk: Self) -> Self;

    /// Load `D * 16` bytes.
    ///
    /// # Safety
    ///
    /// `ptr` must be valid for reads of `D * 16` bytes.
    unsafe fn load(ptr: *const u8) -> Self;

    /// Store `D * 16` bytes.
    ///
    /// # Safety
    ///
    /// `ptr` must be valid for writes of `D * 16` bytes.
    unsafe fn store(self, ptr: *mut u8);

    /// Fill every lane with the same 128-bit block.
    fn broadcast(block: &[u8; 16]) -> Self;

    /// Per-lane initialization context: `ctx[i] = { i, D - 1, 0, ... }`.
    fn ctx_all() -> Self;

    /// `self ^ b ^ c`.
    ///
    /// Backends with a three-input XOR instruction override this.
    #[inline(always)]
    fn xor3(self, b: Self, c: Self) -> Self {
        self.xor(b).xor(c)
    }

    /// `AESRound(self, rk) ^ m`.
    ///
    /// This is the state-0 update of AEGIS. Backends with a three-input XOR
    /// instruction can fold the final XOR into the same instruction.
    #[inline(always)]
    fn round_xor(self, rk: Self, m: Self) -> Self {
        self.round(rk).xor(m)
    }

    /// Whether the split-state bulk loop is profitable for this backend.
    ///
    /// The loop keeps two state vectors live per AEGIS block. It is enabled
    /// only where the register file can hold them without spilling; current
    /// measurements show wins for the single-lane hardware backends and
    /// regressions for multi-lane NEON.
    const SPLIT_BULK: bool = false;

    /// AES round without the final AddRoundKey.
    #[inline(always)]
    fn round_nc(self) -> Self {
        self.round(Self::zero())
    }

    /// `round_nc(self ^ key)`, using the fused AddRoundKey input of the
    /// hardware AES instruction when available.
    #[inline(always)]
    fn round_nc_with(self, key: Self) -> Self {
        self.xor(key).round_nc()
    }

    /// XOR all 128-bit lanes together.
    #[inline(always)]
    fn fold(self) -> [u8; 16] {
        let mut raw = [0u8; 64];
        // SAFETY: `raw` has room for at least four lanes, and D <= 4.
        unsafe { self.store(raw.as_mut_ptr()) };
        let mut out = [0u8; 16];
        for lane in 0..Self::D {
            for k in 0..16 {
                out[k] ^= raw[lane * 16 + k];
            }
        }
        out
    }
}

#[inline(always)]
fn xor16(a: &[u8; 16], b: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = a[i] ^ b[i];
    }
    out
}

#[inline(always)]
pub(crate) fn length_block(a_bits: u64, b_bits: u64) -> [u8; 16] {
    let mut out = [0u8; 16];
    out[..8].copy_from_slice(&a_bits.to_le_bytes());
    out[8..].copy_from_slice(&b_bits.to_le_bytes());
    out
}

/// The six-block AEGIS state, parameterized by the backend's block type.
#[derive(Clone, Copy)]
pub(crate) struct Core<B: BlockOps> {
    pub(crate) v: [B; 6],
}

impl<B: BlockOps> Core<B> {
    /// `rate` is the number of message bytes absorbed or produced per update.
    pub(crate) const RATE: usize = 16 * B::D;

    /// `Init(key, nonce)` (RFC 10032 §4.4 and §5.5.2).
    ///
    /// `#[inline(always)]` is required: this function ends up in the
    /// `#[target_feature]` backend wrappers, and if it is not inlined the
    /// AES intrinsics compile to out-of-line calls (measured ~8x slower
    /// initialization).
    #[inline(always)]
    pub(crate) fn new(key: &[u8; 32], nonce: &[u8; 32]) -> Self {
        Self::new_inner(key, nonce, None)
    }

    /// One-lane initialization with an explicit context block.
    ///
    /// Used by the lane-major WebAssembly kernel, where each parallel AEGIS
    /// state is processed independently with `ctx[l] = { l, D - 1, 0, ... }`.
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    #[inline(always)]
    pub(crate) fn new_with_ctx(key: &[u8; 32], nonce: &[u8; 32], ctx: [u8; 16]) -> Self {
        Self::new_inner(key, nonce, Some(ctx))
    }

    #[inline(always)]
    fn new_inner(key: &[u8; 32], nonce: &[u8; 32], ctx: Option<[u8; 16]>) -> Self {
        let k0: [u8; 16] = key[..16].try_into().unwrap();
        let k1: [u8; 16] = key[16..].try_into().unwrap();
        let n0: [u8; 16] = nonce[..16].try_into().unwrap();
        let n1: [u8; 16] = nonce[16..].try_into().unwrap();

        let kn0 = xor16(&k0, &n0);
        let kn1 = xor16(&k1, &n1);
        let kc0 = xor16(&k0, &C0);
        let kc1 = xor16(&k1, &C1);

        let mut state = Core {
            v: [
                B::broadcast(&kn0),
                B::broadcast(&kn1),
                B::broadcast(&C1),
                B::broadcast(&C0),
                B::broadcast(&kc0),
                B::broadcast(&kc1),
            ],
        };

        let kb0 = B::broadcast(&k0);
        let kb1 = B::broadcast(&k1);
        let knb0 = B::broadcast(&kn0);
        let knb1 = B::broadcast(&kn1);
        for _ in 0..4 {
            state.xor_ctx_value(ctx);
            state.update(kb0);
            state.xor_ctx_value(ctx);
            state.update(kb1);
            state.xor_ctx_value(ctx);
            state.update(knb0);
            state.xor_ctx_value(ctx);
            state.update(knb1);
        }
        state
    }

    #[inline(always)]
    fn xor_ctx_value(&mut self, ctx: Option<[u8; 16]>) {
        let block = match ctx {
            Some(ctx) => B::broadcast(&ctx),
            None => B::ctx_all(),
        };
        self.v[3] = self.v[3].xor(block);
        self.v[5] = self.v[5].xor(block);
    }

    /// `Update(M)` (RFC 10032 §4.3 and §5.5.1).
    #[inline(always)]
    pub(crate) fn update(&mut self, m: B) {
        let tmp = self.v[5];
        self.v[5] = self.v[4].round(self.v[5]);
        self.v[4] = self.v[3].round(self.v[4]);
        self.v[3] = self.v[2].round(self.v[3]);
        self.v[2] = self.v[1].round(self.v[2]);
        self.v[1] = self.v[0].round(self.v[1]);
        self.v[0] = tmp.round_xor(self.v[0], m);
    }

    /// Keystream block `z = S1 ^ S4 ^ S5 ^ (S2 & S3)`, per lane.
    #[inline(always)]
    pub(crate) fn keystream_block(&self) -> B {
        self.v[1].xor3(self.v[4], self.v[5]).xor(self.v[2].and(self.v[3]))
    }

    /// Absorb associated data, zero-padding the final partial block.
    #[inline(always)]
    pub(crate) fn absorb(&mut self, ad: &[u8]) {
        let mut offset = 0;
        if B::SPLIT_BULK && ad.len() >= BULK_THRESHOLD {
            let full = ad.len() / Self::RATE * Self::RATE;
            self.absorb_split(&ad[..full]);
            offset = full;
        }
        while offset + Self::RATE <= ad.len() {
            // SAFETY: the slice contains at least `RATE` bytes.
            self.update(unsafe { B::load(ad[offset..].as_ptr()) });
            offset += Self::RATE;
        }
        if offset < ad.len() {
            let mut buf = [0u8; 64];
            buf[..ad.len() - offset].copy_from_slice(&ad[offset..]);
            // SAFETY: `buf` is at least `RATE` bytes (64 >= 16 * 4).
            self.update(unsafe { B::load(buf.as_ptr()) });
        }
    }

    /// Encrypt a message in place (`Enc` plus the final `DecPartial`-style
    /// partial block).
    #[inline(always)]
    pub(crate) fn encrypt_msg(&mut self, msg: &mut [u8]) {
        let mut offset = 0;
        if B::SPLIT_BULK && msg.len() >= BULK_THRESHOLD {
            let full = msg.len() / Self::RATE * Self::RATE;
            self.encrypt_msg_split(&mut msg[..full]);
            offset = full;
        }
        while offset + Self::RATE <= msg.len() {
            // SAFETY: the slice contains at least `RATE` bytes.
            let m = unsafe { B::load(msg[offset..].as_ptr()) };
            let c = m.xor(self.keystream_block());
            // SAFETY: the slice contains at least `RATE` bytes.
            unsafe { c.store(msg[offset..].as_mut_ptr()) };
            self.update(m);
            offset += Self::RATE;
        }
        if offset < msg.len() {
            let remaining = msg.len() - offset;
            let mut m = [0u8; 64];
            m[..remaining].copy_from_slice(&msg[offset..]);
            // SAFETY: `m` is at least `RATE` bytes.
            let m = unsafe { B::load(m.as_ptr()) };
            let z = self.keystream_block();
            let mut raw = [0u8; 64];
            // SAFETY: `raw` is at least `RATE` bytes.
            unsafe { z.store(raw.as_mut_ptr()) };
            for i in 0..remaining {
                msg[offset + i] ^= raw[i];
            }
            // The state absorbs the zero-padded plaintext, not the ciphertext.
            self.update(m);
        }
    }

    /// Decrypt a message in place.
    #[inline(always)]
    pub(crate) fn decrypt_msg(&mut self, msg: &mut [u8]) {
        let mut offset = 0;
        if B::SPLIT_BULK && msg.len() >= BULK_THRESHOLD {
            let full = msg.len() / Self::RATE * Self::RATE;
            self.decrypt_msg_split(&mut msg[..full]);
            offset = full;
        }
        while offset + Self::RATE <= msg.len() {
            // SAFETY: the slice contains at least `RATE` bytes.
            let c = unsafe { B::load(msg[offset..].as_ptr()) };
            let m = c.xor(self.keystream_block());
            // SAFETY: the slice contains at least `RATE` bytes.
            unsafe { m.store(msg[offset..].as_mut_ptr()) };
            self.update(m);
            offset += Self::RATE;
        }
        if offset < msg.len() {
            let remaining = msg.len() - offset;
            let mut c = [0u8; 64];
            c[..remaining].copy_from_slice(&msg[offset..]);
            // SAFETY: `c` is at least `RATE` bytes.
            let c = unsafe { B::load(c.as_ptr()) };
            let z = self.keystream_block();
            let mut raw = [0u8; 64];
            // SAFETY: `raw` is at least `RATE` bytes.
            unsafe { z.store(raw.as_mut_ptr()) };
            let c_raw = block_bytes(c);
            let mut v = [0u8; 64];
            for i in 0..remaining {
                let p = c_raw[i] ^ raw[i];
                v[i] = p;
                msg[offset + i] = p;
            }
            // SAFETY: `v` is at least `RATE` bytes.
            self.update(unsafe { B::load(v.as_ptr()) });
        }
    }

    /// Encrypt a full-block prefix using the split `x ^ y` representation.
    ///
    /// Keeping the state split lets the next iteration's AES instruction start
    /// as soon as the previous round key (`y`) is available, instead of waiting
    /// for the final AddRoundKey XOR. This is only used where
    /// [`BlockOps::SPLIT_BULK`] says the register file can hold both halves.
    #[inline(always)]
    fn encrypt_msg_split(&mut self, msg: &mut [u8]) {
        let rate = Self::RATE;
        debug_assert!(msg.len().is_multiple_of(rate));
        let (mut x0, mut x1, mut x2, mut x3, mut x4, mut x5) =
            (self.v[0], self.v[1], self.v[2], self.v[3], self.v[4], self.v[5]);
        let zero = B::zero();
        let (mut y0, mut y1, mut y2, mut y3, mut y4, mut y5) = (zero, zero, zero, zero, zero, zero);
        for chunk in msg.chunks_exact_mut(rate) {
            // SAFETY: the chunk contains exactly RATE bytes.
            let m = unsafe { B::load(chunk.as_ptr()) };
            let r0 = x5.round_nc_with(y5);
            let r1 = x0.round_nc_with(y0);
            let r2 = x1.round_nc_with(y1);
            let r3 = x2.round_nc_with(y2);
            let r4 = x3.round_nc_with(y3);
            let r5 = x4.round_nc_with(y4);
            let s0 = x0.xor(y0);
            let s1 = x1.xor(y1);
            let s2 = x2.xor(y2);
            let s3 = x3.xor(y3);
            let s4 = x4.xor(y4);
            let s5 = x5.xor(y5);
            let k = s1.xor3(s4, s5).xor(s2.and(s3));
            let c = m.xor(k);
            // SAFETY: the chunk contains exactly RATE bytes.
            unsafe { c.store(chunk.as_mut_ptr()) };
            x0 = s0.xor(m);
            x1 = s1;
            x2 = s2;
            x3 = s3;
            x4 = s4;
            x5 = s5;
            y0 = r0;
            y1 = r1;
            y2 = r2;
            y3 = r3;
            y4 = r4;
            y5 = r5;
        }
        self.v = [x0.xor(y0), x1.xor(y1), x2.xor(y2), x3.xor(y3), x4.xor(y4), x5.xor(y5)];
    }

    /// Decrypt a full-block prefix using the split representation.
    #[inline(always)]
    fn decrypt_msg_split(&mut self, msg: &mut [u8]) {
        let rate = Self::RATE;
        debug_assert!(msg.len().is_multiple_of(rate));
        let (mut x0, mut x1, mut x2, mut x3, mut x4, mut x5) =
            (self.v[0], self.v[1], self.v[2], self.v[3], self.v[4], self.v[5]);
        let zero = B::zero();
        let (mut y0, mut y1, mut y2, mut y3, mut y4, mut y5) = (zero, zero, zero, zero, zero, zero);
        for chunk in msg.chunks_exact_mut(rate) {
            // SAFETY: the chunk contains exactly RATE bytes.
            let c = unsafe { B::load(chunk.as_ptr()) };
            let r0 = x5.round_nc_with(y5);
            let r1 = x0.round_nc_with(y0);
            let r2 = x1.round_nc_with(y1);
            let r3 = x2.round_nc_with(y2);
            let r4 = x3.round_nc_with(y3);
            let r5 = x4.round_nc_with(y4);
            let s0 = x0.xor(y0);
            let s1 = x1.xor(y1);
            let s2 = x2.xor(y2);
            let s3 = x3.xor(y3);
            let s4 = x4.xor(y4);
            let s5 = x5.xor(y5);
            let k = s1.xor3(s4, s5).xor(s2.and(s3));
            let m = c.xor(k);
            // SAFETY: the chunk contains exactly RATE bytes.
            unsafe { m.store(chunk.as_mut_ptr()) };
            x0 = s0.xor(m);
            x1 = s1;
            x2 = s2;
            x3 = s3;
            x4 = s4;
            x5 = s5;
            y0 = r0;
            y1 = r1;
            y2 = r2;
            y3 = r3;
            y4 = r4;
            y5 = r5;
        }
        self.v = [x0.xor(y0), x1.xor(y1), x2.xor(y2), x3.xor(y3), x4.xor(y4), x5.xor(y5)];
    }

    /// Absorb a full-block prefix using the split representation.
    #[inline(always)]
    fn absorb_split(&mut self, ad: &[u8]) {
        let rate = Self::RATE;
        debug_assert!(ad.len().is_multiple_of(rate));
        let (mut x0, mut x1, mut x2, mut x3, mut x4, mut x5) =
            (self.v[0], self.v[1], self.v[2], self.v[3], self.v[4], self.v[5]);
        let zero = B::zero();
        let (mut y0, mut y1, mut y2, mut y3, mut y4, mut y5) = (zero, zero, zero, zero, zero, zero);
        for chunk in ad.chunks_exact(rate) {
            // SAFETY: the chunk contains exactly RATE bytes.
            let m = unsafe { B::load(chunk.as_ptr()) };
            let r0 = x5.round_nc_with(y5);
            let r1 = x0.round_nc_with(y0);
            let r2 = x1.round_nc_with(y1);
            let r3 = x2.round_nc_with(y2);
            let r4 = x3.round_nc_with(y3);
            let r5 = x4.round_nc_with(y4);
            x0 = x0.xor(y0).xor(m);
            x1 = x1.xor(y1);
            x2 = x2.xor(y2);
            x3 = x3.xor(y3);
            x4 = x4.xor(y4);
            x5 = x5.xor(y5);
            y0 = r0;
            y1 = r1;
            y2 = r2;
            y3 = r3;
            y4 = r4;
            y5 = r5;
        }
        self.v = [x0.xor(y0), x1.xor(y1), x2.xor(y2), x3.xor(y3), x4.xor(y4), x5.xor(y5)];
    }

    /// XOR a full-block keystream using the split representation.
    #[inline(always)]
    fn stream_xor_split(&mut self, in_out: &mut [u8]) {
        let rate = Self::RATE;
        debug_assert!(in_out.len().is_multiple_of(rate));
        let (mut x0, mut x1, mut x2, mut x3, mut x4, mut x5) =
            (self.v[0], self.v[1], self.v[2], self.v[3], self.v[4], self.v[5]);
        let zero = B::zero();
        let (mut y0, mut y1, mut y2, mut y3, mut y4, mut y5) = (zero, zero, zero, zero, zero, zero);
        for chunk in in_out.chunks_exact_mut(rate) {
            // SAFETY: the chunk contains exactly RATE bytes.
            let m = unsafe { B::load(chunk.as_ptr()) };
            let r0 = x5.round_nc_with(y5);
            let r1 = x0.round_nc_with(y0);
            let r2 = x1.round_nc_with(y1);
            let r3 = x2.round_nc_with(y2);
            let r4 = x3.round_nc_with(y3);
            let r5 = x4.round_nc_with(y4);
            let s0 = x0.xor(y0);
            let s1 = x1.xor(y1);
            let s2 = x2.xor(y2);
            let s3 = x3.xor(y3);
            let s4 = x4.xor(y4);
            let s5 = x5.xor(y5);
            let k = s1.xor3(s4, s5).xor(s2.and(s3));
            let c = m.xor(k);
            // SAFETY: the chunk contains exactly RATE bytes.
            unsafe { c.store(chunk.as_mut_ptr()) };
            x0 = s0;
            x1 = s1;
            x2 = s2;
            x3 = s3;
            x4 = s4;
            x5 = s5;
            y0 = r0;
            y1 = r1;
            y2 = r2;
            y3 = r3;
            y4 = r4;
            y5 = r5;
        }
        self.v = [x0.xor(y0), x1.xor(y1), x2.xor(y2), x3.xor(y3), x4.xor(y4), x5.xor(y5)];
    }

    /// Write a full-block keystream using the split representation.
    #[inline(always)]
    fn stream_fill_split(&mut self, out: &mut [u8]) {
        let rate = Self::RATE;
        debug_assert!(out.len().is_multiple_of(rate));
        let (mut x0, mut x1, mut x2, mut x3, mut x4, mut x5) =
            (self.v[0], self.v[1], self.v[2], self.v[3], self.v[4], self.v[5]);
        let zero = B::zero();
        let (mut y0, mut y1, mut y2, mut y3, mut y4, mut y5) = (zero, zero, zero, zero, zero, zero);
        for chunk in out.chunks_exact_mut(rate) {
            let r0 = x5.round_nc_with(y5);
            let r1 = x0.round_nc_with(y0);
            let r2 = x1.round_nc_with(y1);
            let r3 = x2.round_nc_with(y2);
            let r4 = x3.round_nc_with(y3);
            let r5 = x4.round_nc_with(y4);
            let s0 = x0.xor(y0);
            let s1 = x1.xor(y1);
            let s2 = x2.xor(y2);
            let s3 = x3.xor(y3);
            let s4 = x4.xor(y4);
            let s5 = x5.xor(y5);
            let k = s1.xor3(s4, s5).xor(s2.and(s3));
            // SAFETY: the chunk contains exactly RATE bytes.
            unsafe { k.store(chunk.as_mut_ptr()) };
            x0 = s0;
            x1 = s1;
            x2 = s2;
            x3 = s3;
            x4 = s4;
            x5 = s5;
            y0 = r0;
            y1 = r1;
            y2 = r2;
            y3 = r3;
            y4 = r4;
            y5 = r5;
        }
        self.v = [x0.xor(y0), x1.xor(y1), x2.xor(y2), x3.xor(y3), x4.xor(y4), x5.xor(y5)];
    }

    /// Write the authentication tag into `tag` (16 or 32 bytes).
    #[inline(always)]
    pub(crate) fn finalize(&mut self, ad_len: usize, msg_len: usize, tag: &mut [u8]) {
        let t = self.v[3].xor(B::broadcast(&length_block((ad_len as u64) * 8, (msg_len as u64) * 8)));
        for _ in 0..7 {
            self.update(t);
        }
        self.write_tag(tag);
    }

    /// `FinalizeMac` (RFC 10032 §8.3.2 and §8.4.2).
    #[inline(always)]
    pub(crate) fn finalize_mac(&mut self, data_len: u64, tag: &mut [u8]) {
        let tag_len_bits = (tag.len() as u64) * 8;
        let t = self.v[3].xor(B::broadcast(&length_block(data_len * 8, tag_len_bits)));
        for _ in 0..7 {
            self.update(t);
        }

        if B::D == 1 {
            self.write_tag(tag);
            return;
        }

        // Combine the per-lane tags, skipping state 0. RFC 10032 §8.4.2's
        // pseudocode shows `for i in 0..D` for 128-bit tags, but the A.8.5 /
        // A.8.6 `tags128` vectors contain only D-1 entries and the reference
        // implementation skips lane 0 for both tag sizes. Follow the vectors.
        let mut tags = [0u8; 96];
        let mut tags_len = 0;
        for lane in 1..B::D {
            if tag.len() == 16 {
                let mut t = [0u8; 16];
                for block in 0..6 {
                    let bytes = block_bytes(self.v[block]);
                    for k in 0..16 {
                        t[k] ^= bytes[lane * 16 + k];
                    }
                }
                tags[tags_len..tags_len + 16].copy_from_slice(&t);
                tags_len += 16;
            } else {
                let mut t0 = [0u8; 16];
                let mut t1 = [0u8; 16];
                for block in 0..3 {
                    let bytes = block_bytes(self.v[block]);
                    for k in 0..16 {
                        t0[k] ^= bytes[lane * 16 + k];
                    }
                }
                for block in 3..6 {
                    let bytes = block_bytes(self.v[block]);
                    for k in 0..16 {
                        t1[k] ^= bytes[lane * 16 + k];
                    }
                }
                tags[tags_len..tags_len + 16].copy_from_slice(&t0);
                tags[tags_len + 16..tags_len + 32].copy_from_slice(&t1);
                tags_len += 32;
            }
        }

        // Absorb each 128-bit tag chunk, zero-padded to the parallel rate.
        let mut offset = 0;
        while offset < tags_len {
            self.absorb(&tags[offset..offset + 16]);
            offset += 16;
        }

        // Finalize state 0 with `LE64(D) || LE64(tag_len_bits)`.
        let u = length_block(B::D as u64, tag_len_bits);
        let v3 = block_bytes(self.v[3]);
        let mut raw = [0u8; 64];
        for k in 0..16 {
            raw[k] = v3[k] ^ u[k];
        }
        // SAFETY: `raw` is at least `RATE` bytes.
        let t = unsafe { B::load(raw.as_ptr()) };
        for _ in 0..7 {
            self.update(t);
        }
        self.write_tag_lane0(tag);
    }

    #[inline(always)]
    fn write_tag(&self, tag: &mut [u8]) {
        let a = self.v[0].xor3(self.v[1], self.v[2]);
        let b = self.v[3].xor3(self.v[4], self.v[5]);
        if tag.len() == 16 {
            tag[..16].copy_from_slice(&a.xor(b).fold());
        } else {
            tag[..16].copy_from_slice(&a.fold());
            tag[16..32].copy_from_slice(&b.fold());
        }
    }

    #[inline(always)]
    fn write_tag_lane0(&self, tag: &mut [u8]) {
        let mut a = [0u8; 16];
        let mut b = [0u8; 16];
        for block in 0..3 {
            let bytes = block_bytes(self.v[block]);
            for k in 0..16 {
                a[k] ^= bytes[k];
            }
        }
        for block in 3..6 {
            let bytes = block_bytes(self.v[block]);
            for k in 0..16 {
                b[k] ^= bytes[k];
            }
        }
        if tag.len() == 16 {
            for k in 0..16 {
                a[k] ^= b[k];
            }
            tag[..16].copy_from_slice(&a);
        } else {
            tag[..16].copy_from_slice(&a);
            tag[16..32].copy_from_slice(&b);
        }
    }

    /// Generate up to `RATE` bytes of keystream and advance the state by
    /// absorbing a zero block (RFC 10032 §7).
    #[inline(always)]
    pub(crate) fn next_keystream(&mut self, out: &mut [u8]) {
        debug_assert!(out.len() <= Self::RATE);
        let z = self.keystream_block();
        let mut raw = [0u8; 64];
        // SAFETY: `raw` is at least `RATE` bytes.
        unsafe { z.store(raw.as_mut_ptr()) };
        out.copy_from_slice(&raw[..out.len()]);
        self.update(B::zero());
    }

    /// Serialize the six state blocks into the canonical lane layout.
    ///
    /// This is used by the incremental MAC and stream APIs, which keep their
    /// state in a backend-independent byte buffer so that the public types do
    /// not need a per-architecture state enum.
    #[inline(always)]
    pub(crate) fn to_bytes(&self, out: &mut [u8]) {
        assert_eq!(out.len(), 6 * Self::RATE, "AEGIS serialized state has the wrong size");
        for block in 0..6 {
            // SAFETY: `out` has exactly six `RATE`-byte slots.
            unsafe { self.v[block].store(out.as_mut_ptr().add(block * Self::RATE)) };
        }
    }

    /// Deserialize a state written by [`Self::to_bytes`].
    #[inline(always)]
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 6 * Self::RATE, "AEGIS serialized state has the wrong size");
        let mut v = [B::zero(); 6];
        for block in 0..6 {
            // SAFETY: `bytes` has exactly six `RATE`-byte slots.
            v[block] = unsafe { B::load(bytes.as_ptr().add(block * Self::RATE)) };
        }
        Self {
            v,
        }
    }
}

/// Build an initialized state and serialize it into `out`.
///
/// `out` must be exactly `6 * 16 * D` bytes; the public types size their state
/// buffers accordingly.
#[inline(always)]
pub(crate) fn init_state<B: BlockOps>(key: &[u8; 32], nonce: &[u8; 32], out: &mut [u8]) {
    Core::<B>::new(key, nonce).to_bytes(out);
}

/// Absorb data into a serialized state (MAC updates).
#[inline(always)]
pub(crate) fn absorb_state<B: BlockOps>(bytes: &mut [u8], ad: &[u8]) {
    let mut core = Core::<B>::from_bytes(bytes);
    core.absorb(ad);
    core.to_bytes(bytes);
}

/// Finalize a serialized MAC state. The state is consumed, so it is not written
/// back.
#[inline(always)]
pub(crate) fn finalize_mac_state<B: BlockOps>(bytes: &[u8], data_len: u64, tag: &mut [u8]) {
    let mut core = Core::<B>::from_bytes(bytes);
    core.finalize_mac(data_len, tag);
}

/// Generate one keystream block into `out` (`out.len() <= RATE`).
#[inline(always)]
pub(crate) fn next_keystream_state<B: BlockOps>(bytes: &mut [u8], out: &mut [u8]) {
    let mut core = Core::<B>::from_bytes(bytes);
    core.next_keystream(out);
    core.to_bytes(bytes);
}

/// XOR the stream keystream into `in_out`, which must contain whole `RATE`
/// blocks.
#[inline(always)]
pub(crate) fn stream_xor_state<B: BlockOps>(bytes: &mut [u8], in_out: &mut [u8]) {
    let rate = Core::<B>::RATE;
    debug_assert!(in_out.len().is_multiple_of(rate));
    let mut core = Core::<B>::from_bytes(bytes);
    let mut offset = 0;
    if B::SPLIT_BULK && in_out.len() >= BULK_THRESHOLD {
        let full = in_out.len() / rate * rate;
        core.stream_xor_split(&mut in_out[..full]);
        offset = full;
    }
    for chunk in in_out[offset..].chunks_exact_mut(rate) {
        // SAFETY: the chunk contains exactly `RATE` bytes.
        let x = unsafe { B::load(chunk.as_ptr()) };
        let c = x.xor(core.keystream_block());
        // SAFETY: the chunk contains exactly `RATE` bytes.
        unsafe { c.store(chunk.as_mut_ptr()) };
        core.update(B::zero());
    }
    core.to_bytes(bytes);
}

/// Write the stream keystream into `out`, which must contain whole `RATE`
/// blocks. Unlike [`stream_xor_state`], this overwrites the previous contents.
#[inline(always)]
pub(crate) fn stream_fill_state<B: BlockOps>(bytes: &mut [u8], out: &mut [u8]) {
    let rate = Core::<B>::RATE;
    debug_assert!(out.len().is_multiple_of(rate));
    let mut core = Core::<B>::from_bytes(bytes);
    let mut offset = 0;
    if B::SPLIT_BULK && out.len() >= BULK_THRESHOLD {
        let full = out.len() / rate * rate;
        core.stream_fill_split(&mut out[..full]);
        offset = full;
    }
    for chunk in out[offset..].chunks_exact_mut(rate) {
        let z = core.keystream_block();
        // SAFETY: the chunk contains exactly `RATE` bytes.
        unsafe { z.store(chunk.as_mut_ptr()) };
        core.update(B::zero());
    }
    core.to_bytes(bytes);
}

#[inline(always)]
fn block_bytes<B: BlockOps>(block: B) -> [u8; 64] {
    let mut raw = [0u8; 64];
    // SAFETY: `raw` is at least `RATE` bytes.
    unsafe { block.store(raw.as_mut_ptr()) };
    raw
}

/// One-shot AEAD encryption for a backend block type.
#[inline(always)]
pub(crate) fn encrypt<B: BlockOps>(key: &[u8; 32], nonce: &[u8; 32], ad: &[u8], msg: &mut [u8], tag: &mut [u8]) {
    encrypt_with_core(Core::<B>::new(key, nonce), ad, msg, tag);
}

/// One-shot AEAD encryption with an already initialized state.
///
/// Backend wrappers initialize the state in a separate `#[target_feature]`
/// function so the message loop is not compiled together with the (large)
/// initialization loop; the combined function spills state vectors under LTO.
#[inline(always)]
pub(crate) fn encrypt_with_core<B: BlockOps>(mut core: Core<B>, ad: &[u8], msg: &mut [u8], tag: &mut [u8]) {
    core.absorb(ad);
    core.encrypt_msg(msg);
    core.finalize(ad.len(), msg.len(), tag);
}

/// One-shot AEAD decryption for a backend block type.
///
/// Returns `true` when the tag authenticates. On failure the plaintext is
/// overwritten with zeros before returning, as required by RFC 10032 §4.2.
#[inline(always)]
pub(crate) fn decrypt<B: BlockOps>(key: &[u8; 32], nonce: &[u8; 32], ad: &[u8], msg: &mut [u8], tag: &[u8]) -> bool {
    decrypt_with_core(Core::<B>::new(key, nonce), ad, msg, tag)
}

/// One-shot AEAD decryption with an already initialized state.
#[inline(always)]
pub(crate) fn decrypt_with_core<B: BlockOps>(mut core: Core<B>, ad: &[u8], msg: &mut [u8], tag: &[u8]) -> bool {
    core.absorb(ad);
    core.decrypt_msg(msg);
    let mut expected = [0u8; 32];
    core.finalize(ad.len(), msg.len(), &mut expected[..tag.len()]);
    let ok = constant_time_eq(tag, &expected[..tag.len()]);
    if !ok {
        msg.fill(0);
    }
    ok
}
