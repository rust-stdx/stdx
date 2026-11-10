//! WebAssembly `simd128` backend.
//!
//! WebAssembly has no AES instruction. This backend implements the AES round
//! with `i8x16.swizzle`-based S-box tables. Indices are derived from the data,
//! but they are only used as *lane selectors* inside a register: there is no
//! secret-dependent memory access, so the round remains constant-time, unlike a
//! table-based software AES.

use core::arch::wasm32::*;

use constant_time_eq::constant_time_eq;

use crate::aegis::aegis_core::{BlockOps, Core, length_block};

const IPT_LO: [u8; 16] = [
    0x00, 0x70, 0x2a, 0x5a, 0x98, 0xe8, 0xb2, 0xc2, 0x08, 0x78, 0x22, 0x52, 0x90, 0xe0, 0xba, 0xca,
];
const IPT_HI: [u8; 16] = [
    0x00, 0x4d, 0x7c, 0x31, 0x7d, 0x30, 0x01, 0x4c, 0x81, 0xcc, 0xfd, 0xb0, 0xfc, 0xb1, 0x80, 0xcd,
];
const INV: [u8; 16] = [
    0x80, 0x01, 0x08, 0x0d, 0x0f, 0x06, 0x05, 0x0e, 0x02, 0x0c, 0x0b, 0x0a, 0x09, 0x03, 0x07, 0x04,
];
const INVA: [u8; 16] = [
    0x80, 0x07, 0x0b, 0x0f, 0x06, 0x0a, 0x04, 0x01, 0x09, 0x08, 0x05, 0x02, 0x0c, 0x0e, 0x0d, 0x03,
];
const SBO_U: [u8; 16] = [
    0x00, 0xc7, 0xbd, 0x6f, 0x17, 0x6d, 0xd2, 0xd0, 0x78, 0xa8, 0x02, 0xc5, 0x7a, 0xbf, 0xaa, 0x15,
];
const SBO_T: [u8; 16] = [
    0x00, 0x6a, 0xbb, 0x5f, 0xa5, 0x74, 0xe4, 0xcf, 0xfa, 0x35, 0x2b, 0x41, 0xd1, 0x90, 0x1e, 0x8e,
];
const SBO2_U: [u8; 16] = [
    0x00, 0x95, 0x61, 0xde, 0x2e, 0xda, 0xbf, 0xbb, 0xf0, 0x4b, 0x04, 0x91, 0xf4, 0x65, 0x4f, 0x2a,
];
const SBO2_T: [u8; 16] = [
    0x00, 0xd4, 0x6d, 0xbe, 0x51, 0xe8, 0xd3, 0x85, 0xef, 0x6a, 0x56, 0x82, 0xb9, 0x3b, 0x3c, 0x07,
];
const MC0: [u8; 16] = [
    0x00, 0x05, 0x0a, 0x0f, 0x04, 0x09, 0x0e, 0x03, 0x08, 0x0d, 0x02, 0x07, 0x0c, 0x01, 0x06, 0x0b,
];
const MC1: [u8; 16] = [
    0x05, 0x0a, 0x0f, 0x00, 0x09, 0x0e, 0x03, 0x04, 0x0d, 0x02, 0x07, 0x08, 0x01, 0x06, 0x0b, 0x0c,
];
const MC2: [u8; 16] = [
    0x0a, 0x0f, 0x00, 0x05, 0x0e, 0x03, 0x04, 0x09, 0x02, 0x07, 0x08, 0x0d, 0x06, 0x0b, 0x0c, 0x01,
];
const MC3: [u8; 16] = [
    0x0f, 0x00, 0x05, 0x0a, 0x03, 0x04, 0x09, 0x0e, 0x07, 0x08, 0x0d, 0x02, 0x0b, 0x0c, 0x01, 0x06,
];
const S0F: [u8; 16] = [0x0f; 16];

#[inline(always)]
fn ld(t: &[u8; 16]) -> v128 {
    // SAFETY: `t` is a 16-byte array.
    unsafe { v128_load(t.as_ptr().cast()) }
}

/// AES round without the final `AddRoundKey` and without the S-box affine
/// constant, which is added by the caller. This is `SubBytes + ShiftRows +
/// MixColumns` with the constant folded into the byte permutation tables.
#[inline(always)]
fn aesl_nc(x: v128) -> v128 {
    let ipt_lo = ld(&IPT_LO);
    let ipt_hi = ld(&IPT_HI);
    let inv = ld(&INV);
    let inva = ld(&INVA);
    let sbo_u = ld(&SBO_U);
    let sbo_t = ld(&SBO_T);
    let sbo2_u = ld(&SBO2_U);
    let sbo2_t = ld(&SBO2_T);
    let mc0 = ld(&MC0);
    let mc1 = ld(&MC1);
    let mc2 = ld(&MC2);
    let mc3 = ld(&MC3);
    let s0f = ld(&S0F);

    let lo = v128_and(x, s0f);
    let hi = v128_and(u16x8_shr(x, 4), s0f);
    let t = v128_xor(i8x16_swizzle(ipt_lo, lo), i8x16_swizzle(ipt_hi, hi));

    let k = v128_and(t, s0f);
    let i = v128_and(u16x8_shr(t, 4), s0f);
    let j = v128_xor(k, i);
    let ak = i8x16_swizzle(inva, k);
    let iak = v128_xor(i8x16_swizzle(inv, i), ak);
    let jak = v128_xor(i8x16_swizzle(inv, j), ak);
    let io = v128_xor(i8x16_swizzle(inv, iak), j);
    let jo = v128_xor(i8x16_swizzle(inv, jak), i);

    let a = v128_xor(i8x16_swizzle(sbo_u, io), i8x16_swizzle(sbo_t, jo));
    let a2 = v128_xor(i8x16_swizzle(sbo2_u, io), i8x16_swizzle(sbo2_t, jo));

    let m0 = i8x16_swizzle(a2, mc0);
    let m1 = i8x16_swizzle(v128_xor(a, a2), mc1);
    let m2 = i8x16_swizzle(a, mc2);
    let m3 = i8x16_swizzle(a, mc3);

    v128_xor(v128_xor(m0, m1), v128_xor(m2, m3))
}

#[inline(always)]
fn aes_round(x: v128, rk: v128) -> v128 {
    v128_xor(v128_xor(aesl_nc(x), i8x16_splat(0x63)), rk)
}

/// A vector of `D` 128-bit AES lanes.
#[derive(Clone, Copy)]
pub(crate) struct Wasm<const D: usize> {
    v: [v128; D],
}

impl<const D: usize> BlockOps for Wasm<D> {
    const D: usize = D;

    #[inline(always)]
    fn zero() -> Self {
        Self {
            v: [i8x16_splat(0); D],
        }
    }

    #[inline(always)]
    fn xor(self, rhs: Self) -> Self {
        let mut v = [i8x16_splat(0); D];
        for lane in 0..D {
            v[lane] = v128_xor(self.v[lane], rhs.v[lane]);
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn and(self, rhs: Self) -> Self {
        let mut v = [i8x16_splat(0); D];
        for lane in 0..D {
            v[lane] = v128_and(self.v[lane], rhs.v[lane]);
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn round(self, rk: Self) -> Self {
        let mut v = [i8x16_splat(0); D];
        for lane in 0..D {
            v[lane] = aes_round(self.v[lane], rk.v[lane]);
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn load(ptr: *const u8) -> Self {
        let mut v = [i8x16_splat(0); D];
        for lane in 0..D {
            // SAFETY: the caller guarantees `D * 16` readable bytes.
            v[lane] = unsafe { v128_load(ptr.add(lane * 16).cast()) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn store(self, ptr: *mut u8) {
        for lane in 0..D {
            // SAFETY: the caller guarantees `D * 16` writable bytes.
            unsafe { v128_store(ptr.add(lane * 16).cast(), self.v[lane]) };
        }
    }

    #[inline(always)]
    fn broadcast(block: &[u8; 16]) -> Self {
        let b = ld(block);
        Self {
            v: [b; D],
        }
    }

    #[inline(always)]
    fn ctx_all() -> Self {
        let mut v = [i8x16_splat(0); D];
        for lane in 0..D {
            let mut ctx = [0u8; 16];
            ctx[0] = lane as u8;
            ctx[1] = (D - 1) as u8;
            v[lane] = ld(&ctx);
        }
        Self {
            v,
        }
    }
}

pub(crate) unsafe fn init_state<const D: usize>(key: &[u8; 32], nonce: &[u8; 32], out: &mut [u8]) {
    crate::aegis::aegis_core::init_state::<Wasm<D>>(key, nonce, out)
}

pub(crate) unsafe fn encrypt<const D: usize>(
    key: &[u8; 32],
    nonce: &[u8; 32],
    ad: &[u8],
    msg: &mut [u8],
    tag: &mut [u8],
) {
    crate::aegis::aegis_core::encrypt::<Wasm<D>>(key, nonce, ad, msg, tag)
}

pub(crate) unsafe fn decrypt<const D: usize>(
    key: &[u8; 32],
    nonce: &[u8; 32],
    ad: &[u8],
    msg: &mut [u8],
    tag: &[u8],
) -> bool {
    crate::aegis::aegis_core::decrypt::<Wasm<D>>(key, nonce, ad, msg, tag)
}

pub(crate) unsafe fn absorb_state<const D: usize>(bytes: &mut [u8], ad: &[u8]) {
    crate::aegis::aegis_core::absorb_state::<Wasm<D>>(bytes, ad)
}

pub(crate) unsafe fn finalize_mac_state<const D: usize>(bytes: &[u8], data_len: u64, tag: &mut [u8]) {
    crate::aegis::aegis_core::finalize_mac_state::<Wasm<D>>(bytes, data_len, tag)
}

pub(crate) unsafe fn next_keystream_state<const D: usize>(bytes: &mut [u8], out: &mut [u8]) {
    crate::aegis::aegis_core::next_keystream_state::<Wasm<D>>(bytes, out)
}

pub(crate) unsafe fn stream_state<const D: usize>(bytes: &mut [u8], in_out: &mut [u8]) {
    crate::aegis::aegis_core::stream_xor_state::<Wasm<D>>(bytes, in_out)
}

pub(crate) unsafe fn fill_state<const D: usize>(bytes: &mut [u8], out: &mut [u8]) {
    crate::aegis::aegis_core::stream_fill_state::<Wasm<D>>(bytes, out)
}

// ── Lane-major kernel for the parallel modes ────────────────────────────────
//
// Each AEGIS-256X lane is an independent AEGIS-256 state. The block-major
// implementation keeps all `D` lanes live at once as `[v128; D]` blocks. On
// wasm that is 12 vectors for X2 and 24 for X4, far beyond the 16 architectural
// `v128` registers, and the state spills to linear memory.
//
// The lane-major kernel keeps only one lane's six blocks in registers at a
// time. It processes the D independent states sequentially and writes the
// interleaved ciphertext layout expected by the RFC. This does not change any
// output; it only reduces register pressure.

fn block_bytes(block: Wasm<1>) -> [u8; 16] {
    let mut out = [0u8; 16];
    // SAFETY: a single lane writes 16 bytes.
    unsafe { block.store(out.as_mut_ptr()) };
    out
}

struct LaneState<const D: usize> {
    lanes: [Core<Wasm<1>>; D],
}

impl<const D: usize> LaneState<D> {
    fn new(key: &[u8; 32], nonce: &[u8; 32]) -> Self {
        let lanes = core::array::from_fn(|lane| {
            let mut ctx = [0u8; 16];
            ctx[0] = lane as u8;
            ctx[1] = (D - 1) as u8;
            Core::<Wasm<1>>::new_with_ctx(key, nonce, ctx)
        });
        Self {
            lanes,
        }
    }

    fn to_bytes(&self, out: &mut [u8]) {
        assert_eq!(out.len(), 96 * D, "AEGIS lane state has the wrong size");
        for lane in 0..D {
            for block in 0..6 {
                // SAFETY: lane-major layout reserves one 16-byte slot per block.
                unsafe {
                    self.lanes[lane].v[block].store(out.as_mut_ptr().add(lane * 96 + block * 16));
                }
            }
        }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 96 * D, "AEGIS lane state has the wrong size");
        let lanes = core::array::from_fn(|lane| {
            let mut v = [Wasm::<1>::zero(); 6];
            for block in 0..6 {
                // SAFETY: lane-major layout reserves one 16-byte slot per block.
                v[block] = unsafe { Wasm::<1>::load(bytes.as_ptr().add(lane * 96 + block * 16)) };
            }
            Core {
                v,
            }
        });
        Self {
            lanes,
        }
    }

    fn absorb(&mut self, ad: &[u8]) {
        let rate = 16 * D;
        let mut offset = 0;
        while offset < ad.len() {
            let n = (ad.len() - offset).min(rate);
            let mut block = [0u8; 64];
            block[..n].copy_from_slice(&ad[offset..offset + n]);
            for lane in 0..D {
                self.lanes[lane].absorb(&block[lane * 16..lane * 16 + 16]);
            }
            offset += n;
        }
    }

    fn encrypt_msg(&mut self, msg: &mut [u8]) {
        let rate = 16 * D;
        let mut offset = 0;
        while offset < msg.len() {
            let n = (msg.len() - offset).min(rate);
            for lane in 0..D {
                let start = offset + lane * 16;
                if start >= msg.len() {
                    self.lanes[lane].absorb(&[0u8; 16]);
                } else {
                    let end = (start + 16).min(msg.len());
                    self.lanes[lane].encrypt_msg(&mut msg[start..end]);
                }
            }
            offset += n;
        }
    }

    fn decrypt_msg(&mut self, msg: &mut [u8]) {
        let rate = 16 * D;
        let mut offset = 0;
        while offset < msg.len() {
            let n = (msg.len() - offset).min(rate);
            for lane in 0..D {
                let start = offset + lane * 16;
                if start >= msg.len() {
                    self.lanes[lane].absorb(&[0u8; 16]);
                } else {
                    let end = (start + 16).min(msg.len());
                    self.lanes[lane].decrypt_msg(&mut msg[start..end]);
                }
            }
            offset += n;
        }
    }

    fn finalize(&mut self, ad_len: usize, msg_len: usize, tag: &mut [u8]) {
        let u = length_block((ad_len as u64) * 8, (msg_len as u64) * 8);
        for lane in 0..D {
            let t = self.lanes[lane].v[3].xor(Wasm::<1>::broadcast(&u));
            for _ in 0..7 {
                self.lanes[lane].update(t);
            }
        }
        let mut a = [0u8; 16];
        let mut b = [0u8; 16];
        for lane in 0..D {
            for block in 0..3 {
                let x = block_bytes(self.lanes[lane].v[block]);
                for k in 0..16 {
                    a[k] ^= x[k];
                }
            }
            for block in 3..6 {
                let x = block_bytes(self.lanes[lane].v[block]);
                for k in 0..16 {
                    b[k] ^= x[k];
                }
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

    fn finalize_mac(&mut self, data_len: u64, tag: &mut [u8]) {
        if D == 1 {
            self.lanes[0].finalize_mac(data_len, tag);
            return;
        }
        let tag_len_bits = (tag.len() as u64) * 8;
        let u = length_block(data_len * 8, tag_len_bits);
        for lane in 0..D {
            let t = self.lanes[lane].v[3].xor(Wasm::<1>::broadcast(&u));
            for _ in 0..7 {
                self.lanes[lane].update(t);
            }
        }

        let mut tags = [0u8; 96];
        let mut tags_len = 0;
        for lane in 1..D {
            if tag.len() == 16 {
                let mut t = [0u8; 16];
                for block in 0..6 {
                    let x = block_bytes(self.lanes[lane].v[block]);
                    for k in 0..16 {
                        t[k] ^= x[k];
                    }
                }
                tags[tags_len..tags_len + 16].copy_from_slice(&t);
                tags_len += 16;
            } else {
                let mut t0 = [0u8; 16];
                let mut t1 = [0u8; 16];
                for block in 0..3 {
                    let x = block_bytes(self.lanes[lane].v[block]);
                    for k in 0..16 {
                        t0[k] ^= x[k];
                    }
                }
                for block in 3..6 {
                    let x = block_bytes(self.lanes[lane].v[block]);
                    for k in 0..16 {
                        t1[k] ^= x[k];
                    }
                }
                tags[tags_len..tags_len + 16].copy_from_slice(&t0);
                tags[tags_len + 16..tags_len + 32].copy_from_slice(&t1);
                tags_len += 32;
            }
        }

        // Absorb and re-finalize lane 0 only: lanes 1..D are no longer used
        // for the output tag.
        for chunk in tags[..tags_len].chunks(16) {
            self.lanes[0].absorb(chunk);
        }

        let u2 = length_block(D as u64, tag_len_bits);
        let t2 = Wasm::<1>::broadcast(&u2);
        let t = self.lanes[0].v[3].xor(t2);
        for _ in 0..7 {
            self.lanes[0].update(t);
        }

        let mut a = [0u8; 16];
        let mut b = [0u8; 16];
        for block in 0..3 {
            let x = block_bytes(self.lanes[0].v[block]);
            for k in 0..16 {
                a[k] ^= x[k];
            }
        }
        for block in 3..6 {
            let x = block_bytes(self.lanes[0].v[block]);
            for k in 0..16 {
                b[k] ^= x[k];
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

    fn next_keystream(&mut self, out: &mut [u8]) {
        let rate = 16 * D;
        debug_assert!(out.len() <= rate);
        for lane in 0..D {
            let z = self.lanes[lane].keystream_block();
            let raw = block_bytes(z);
            let start = lane * 16;
            if start < out.len() {
                let n = (out.len() - start).min(16);
                out[start..start + n].copy_from_slice(&raw[..n]);
            }
            self.lanes[lane].update(Wasm::<1>::zero());
        }
    }

    fn stream_xor(&mut self, in_out: &mut [u8]) {
        let rate = 16 * D;
        debug_assert!(in_out.len().is_multiple_of(rate));
        for chunk in in_out.chunks_exact_mut(rate) {
            for lane in 0..D {
                let block = &mut chunk[lane * 16..lane * 16 + 16];
                // SAFETY: the chunk is exactly 16 bytes.
                let x = unsafe { Wasm::<1>::load(block.as_ptr()) };
                let y = x.xor(self.lanes[lane].keystream_block());
                // SAFETY: the chunk is exactly 16 bytes.
                unsafe { y.store(block.as_mut_ptr()) };
                self.lanes[lane].update(Wasm::<1>::zero());
            }
        }
    }

    fn fill(&mut self, out: &mut [u8]) {
        let rate = 16 * D;
        debug_assert!(out.len().is_multiple_of(rate));
        for chunk in out.chunks_exact_mut(rate) {
            for lane in 0..D {
                let z = self.lanes[lane].keystream_block();
                let raw = block_bytes(z);
                let block = &mut chunk[lane * 16..lane * 16 + 16];
                block.copy_from_slice(&raw);
                self.lanes[lane].update(Wasm::<1>::zero());
            }
        }
    }
}

pub(crate) unsafe fn init_state_lane<const D: usize>(key: &[u8; 32], nonce: &[u8; 32], out: &mut [u8]) {
    LaneState::<D>::new(key, nonce).to_bytes(out);
}

pub(crate) unsafe fn encrypt_lane<const D: usize>(
    key: &[u8; 32],
    nonce: &[u8; 32],
    ad: &[u8],
    msg: &mut [u8],
    tag: &mut [u8],
) {
    let mut state = LaneState::<D>::new(key, nonce);
    state.absorb(ad);
    state.encrypt_msg(msg);
    state.finalize(ad.len(), msg.len(), tag);
}

pub(crate) unsafe fn decrypt_lane<const D: usize>(
    key: &[u8; 32],
    nonce: &[u8; 32],
    ad: &[u8],
    msg: &mut [u8],
    tag: &[u8],
) -> bool {
    let mut state = LaneState::<D>::new(key, nonce);
    state.absorb(ad);
    state.decrypt_msg(msg);
    let mut expected = [0u8; 32];
    state.finalize(ad.len(), msg.len(), &mut expected[..tag.len()]);
    let ok = constant_time_eq(tag, &expected[..tag.len()]);
    if !ok {
        msg.fill(0);
    }
    ok
}

pub(crate) unsafe fn absorb_state_lane<const D: usize>(bytes: &mut [u8], ad: &[u8]) {
    let mut state = LaneState::<D>::from_bytes(bytes);
    state.absorb(ad);
    state.to_bytes(bytes);
}

pub(crate) unsafe fn finalize_mac_state_lane<const D: usize>(bytes: &[u8], data_len: u64, tag: &mut [u8]) {
    let mut state = LaneState::<D>::from_bytes(bytes);
    state.finalize_mac(data_len, tag);
}

pub(crate) unsafe fn next_keystream_state_lane<const D: usize>(bytes: &mut [u8], out: &mut [u8]) {
    let mut state = LaneState::<D>::from_bytes(bytes);
    state.next_keystream(out);
    state.to_bytes(bytes);
}

pub(crate) unsafe fn stream_state_lane<const D: usize>(bytes: &mut [u8], in_out: &mut [u8]) {
    let mut state = LaneState::<D>::from_bytes(bytes);
    state.stream_xor(in_out);
    state.to_bytes(bytes);
}

pub(crate) unsafe fn fill_state_lane<const D: usize>(bytes: &mut [u8], out: &mut [u8]) {
    let mut state = LaneState::<D>::from_bytes(bytes);
    state.fill(out);
    state.to_bytes(bytes);
}
