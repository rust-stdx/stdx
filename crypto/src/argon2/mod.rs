//! Argon2id (RFC 9106) password hashing function.
//!
//! Argon2id is a memory-hard password hashing function that provides resistance
//! against both side-channel attacks and GPU/ASIC brute-force attacks.
//!
//! The memory-filling phase automatically selects the fastest compression
//! kernel available on the running CPU: NEON (optionally with the SHA-3 `xar`
//! extension) on AArch64, AVX2 on x86-64, and a portable scalar implementation
//! everywhere else. When the `std` feature is enabled, the lanes are also
//! filled in parallel; the worker threads are scoped to each call and never
//! outlive it.
//!
//! The `m`, `t`, and `p` fields of a PHC-encoded hash are untrusted whenever
//! the encoded string is not fully trusted. [`verify_password`] therefore
//! accepts an optional [`Params`] upper bound so callers can cap the resources
//! a decoded hash may request.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{string::String, vec, vec::Vec};
use core::mem::MaybeUninit;

use crate::{Hasher, blake2::Blake2b};

mod fill;

#[cfg(target_arch = "x86_64")]
mod fill_avx2;

#[cfg(target_arch = "aarch64")]
mod fill_neon;

#[cfg(test)]
use fill::{compress, permutation_p};

/// Argon2 version 1.3 (0x13)
const VERSION: u32 = 0x13;

/// Number of synchronization points (slices per pass)
const SYNC_POINTS: u32 = 4;

/// Block size in bytes (1024 bytes = 128 u64 values)
const BLOCK_SIZE: usize = 1024;

/// Argon2 type constants
#[allow(dead_code)]
const ARGON2D: u32 = 0;
const ARGON2I: u32 = 1;
const ARGON2ID: u32 = 2;

/// Default output length (in bytes) used by [`hash_password`].
#[cfg(feature = "alloc")]
const DEFAULT_TAG_LENGTH: usize = 64;

/// Argon2id parameters (RFC 9106).
///
/// The output length is not part of the parameters: it is inferred from the
/// length of the output buffer passed to [`derive_key`].
///
/// # Example
///
/// ```ignore
/// use crypto::argon2::Params;
///
/// let params = Params {
///     iterations: 3,
///     memory: 65536,
///     parallelism: 4,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Params {
    /// Number of passes (iterations). Must be >= 1.
    pub iterations: u32,
    /// Memory size in KiB. Must be >= 8 * `parallelism`.
    pub memory: u32,
    /// Degree of parallelism (number of lanes). Must be >= 1.
    pub parallelism: u32,
}

impl Default for Params {
    /// Default parameters: t=3, m=64 MiB, p=4 (SECOND RECOMMENDED option).
    fn default() -> Self {
        Params {
            iterations: 3,
            memory: 65536,
            parallelism: 4,
        }
    }
}

#[cfg(feature = "alloc")]
impl Params {
    /// Check these parameters against the RFC 9106 bounds and the requested
    /// output length.
    ///
    /// Returns [`Argon2Error::InvalidParams`] if `iterations < 1`,
    /// `parallelism < 1`, `parallelism > 2^24 - 1`, `memory < 8 * parallelism`,
    /// `out_len < 4`, or if `out_len` does not fit in a `u32` (Argon2 encodes
    /// lengths as 32-bit little-endian values).
    fn validate(&self, out_len: usize) -> Result<(), Argon2Error> {
        if self.iterations < 1 {
            return Err(Argon2Error::InvalidParams("iterations must be >= 1"));
        }
        if self.parallelism < 1 {
            return Err(Argon2Error::InvalidParams("parallelism must be >= 1"));
        }
        if self.parallelism > (1 << 24) - 1 {
            return Err(Argon2Error::InvalidParams("parallelism must be <= 2^24 - 1"));
        }
        if out_len < 4 {
            return Err(Argon2Error::InvalidParams("output length must be >= 4"));
        }
        check_u32_len(out_len, "output length must be <= 2^32 - 1")?;
        // Checked after the parallelism bound above so the multiplication
        // cannot overflow.
        if self.memory < 8 * self.parallelism {
            return Err(Argon2Error::InvalidParams("memory must be >= 8*parallelism"));
        }
        Ok(())
    }
}

/// Return [`Argon2Error::InvalidParams`] if `len` does not fit in a `u32`.
///
/// Argon2 encodes every length as a 32-bit little-endian value, so inputs
/// larger than `u32::MAX` would be silently truncated.
#[cfg(feature = "alloc")]
#[inline(always)]
fn check_u32_len(len: usize, msg: &'static str) -> Result<(), Argon2Error> {
    if u32::try_from(len).is_err() {
        Err(Argon2Error::InvalidParams(msg))
    } else {
        Ok(())
    }
}

/// Argon2 error type.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(feature = "alloc")]
pub enum Argon2Error {
    /// Invalid parameter
    InvalidParams(&'static str),
    /// Invalid encoded string
    InvalidEncoding(&'static str),
    /// Password verification failed
    VerifyMismatch,
}

#[cfg(feature = "alloc")]
impl core::fmt::Display for Argon2Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Argon2Error::InvalidParams(msg) => write!(f, "argon2: invalid params: {}", msg),
            Argon2Error::InvalidEncoding(msg) => write!(f, "argon2: invalid encoding: {}", msg),
            Argon2Error::VerifyMismatch => write!(f, "argon2: verification failed"),
        }
    }
}

/// Derive a key using Argon2id (RFC 9106).
///
/// This is the main entry point for Argon2id key derivation. The derived key is
/// written into `out`; its length determines the Argon2 output length and must
/// be at least 4 bytes.
///
/// # Arguments
/// * `out` - Output buffer for the derived key. Its length is the key length.
/// * `password` - The password to hash
/// * `salt` - Salt (recommended 16 bytes)
/// * `secret` - Optional secret key (can be empty)
/// * `ad` - Optional associated data (can be empty)
/// * `params` - Argon2id parameters
///
/// # Errors
///
/// Returns [`Argon2Error::InvalidParams`] if `out` is shorter than 4 bytes or
/// longer than `2^32 - 1` bytes, if `params` is invalid, or if `password`,
/// `salt`, `secret`, or `ad` is longer than `2^32 - 1` bytes.
///
/// # Example
///
/// ```ignore
/// use crypto::argon2::{derive_key, Params};
///
/// let mut key = [0u8; 32];
/// derive_key(
///     &mut key,
///     b"correct horse battery staple",
///     b"randomsalt123456",
///     &[],  // no secret
///     &[],  // no associated data
///     &Params { iterations: 3, memory: 65536, parallelism: 4 },
/// ).unwrap();
/// assert_eq!(key.len(), 32);
/// ```
#[cfg(feature = "alloc")]
pub fn derive_key(
    out: &mut [u8],
    password: &[u8],
    salt: &[u8],
    secret: &[u8],
    ad: &[u8],
    params: &Params,
) -> Result<(), Argon2Error> {
    argon2_core(ARGON2ID, password, salt, secret, ad, params, out)
}

/// Hash a password and return the PHC-encoded string.
///
/// The output hash is 64 bytes long.
///
/// # Example
///
/// ```ignore
/// use crypto::argon2::{hash_password, verify_password, Params};
///
/// let encoded = hash_password(
///     b"correct horse battery staple",
///     b"randomsalt123456",
///     &Params { iterations: 3, memory: 65536, parallelism: 4 },
/// ).unwrap();
///
/// assert!(verify_password(b"correct horse battery staple", &encoded, None).is_ok());
/// assert!(verify_password(b"wrong password", &encoded, None).is_err());
/// ```
#[cfg(feature = "alloc")]
pub fn hash_password(password: &[u8], salt: &[u8], params: &Params) -> Result<String, Argon2Error> {
    let mut tag = [0u8; DEFAULT_TAG_LENGTH];
    derive_key(&mut tag, password, salt, &[], &[], params)?;
    Ok(encode_phc(params, salt, &tag))
}

/// Verify a password against a PHC-encoded hash string.
///
/// `limits` optionally caps the resource usage accepted from the encoded
/// string. The `m`, `t`, and `p` fields of a PHC hash are attacker-controlled
/// whenever the hash is not fully trusted (for example a user-supplied or
/// imported credential), and the decoder would otherwise happily honor a hash
/// requesting gigabytes of memory or billions of passes. When `limits` is
/// `Some`, verification fails with [`Argon2Error::InvalidParams`] as soon as
/// the decoded parameters exceed any of the given bounds, before any memory is
/// allocated. Pass `None` when the encoded string is trusted.
///
/// See [`hash_password`] for an example.
#[cfg(feature = "alloc")]
pub fn verify_password(password: &[u8], encoded: &str, limits: Option<&Params>) -> Result<(), Argon2Error> {
    let (params, salt, expected_tag) = decode_phc(encoded)?;

    if let Some(limits) = limits
        && (params.iterations > limits.iterations
            || params.memory > limits.memory
            || params.parallelism > limits.parallelism)
    {
        return Err(Argon2Error::InvalidParams("encoded parameters exceed the configured limits"));
    }

    let mut computed_tag = vec![0u8; expected_tag.len()];
    derive_key(&mut computed_tag, password, &salt, &[], &[], &params)?;

    constant_time_eq::constant_time_eq(&computed_tag, &expected_tag).ok_or(Argon2Error::VerifyMismatch)
}

// ============================================================
// PHC String Format encode/decode
// ============================================================

/// Encode an Argon2id hash in the PHC string format:
/// `$argon2id$v=19$m=<memory>,t=<iterations>,p=<parallelism>$<salt_b64>$<hash_b64>`
///
/// Uses base64 encoding without padding (standard alphabet with +/ replaced by the
/// PHC-standard base64 which is actually the standard base64 without padding).
#[cfg(feature = "alloc")]
pub fn encode_phc(params: &Params, salt: &[u8], tag: &[u8]) -> String {
    let salt_b64 = base64_encode_no_pad(salt);
    let tag_b64 = base64_encode_no_pad(tag);
    alloc::format!(
        "$argon2id$v=19$m={},t={},p={}${}${}",
        params.memory,
        params.iterations,
        params.parallelism,
        salt_b64,
        tag_b64
    )
}

/// Decode an Argon2id PHC string format into (params, salt, tag).
///
/// Expected format: `$argon2id$v=19$m=<m>,t=<t>,p=<p>$<salt_b64>$<hash_b64>`
///
/// # Errors
/// Returns [`Argon2Error::InvalidEncoding`] for a malformed string, and
/// [`Argon2Error::InvalidParams`] if the decoded parameters violate the
/// RFC 9106 bounds (`t >= 1`, `p` in `1..=2^24 - 1`, `m >= 8*p`) or if the
/// decoded tag is shorter than 4 bytes. Note that the salt is not validated.
#[cfg(feature = "alloc")]
pub fn decode_phc(encoded: &str) -> Result<(Params, Vec<u8>, Vec<u8>), Argon2Error> {
    let parts: Vec<&str> = encoded.split('$').collect();
    // Parts: ["", "argon2id", "v=19", "m=...,t=...,p=...", "<salt>", "<hash>"]
    if parts.len() != 6 {
        return Err(Argon2Error::InvalidEncoding("invalid PHC string format"));
    }
    if parts[0] != "" {
        return Err(Argon2Error::InvalidEncoding("must start with $"));
    }
    if parts[1] != "argon2id" {
        return Err(Argon2Error::InvalidEncoding("unsupported algorithm"));
    }
    if parts[2] != "v=19" {
        return Err(Argon2Error::InvalidEncoding("unsupported version"));
    }

    // Parse params
    let param_parts: Vec<&str> = parts[3].split(',').collect();
    if param_parts.len() != 3 {
        return Err(Argon2Error::InvalidEncoding("invalid parameters"));
    }

    let memory = parse_param(param_parts[0], "m=")?;
    let iterations = parse_param(param_parts[1], "t=")?;
    let parallelism = parse_param(param_parts[2], "p=")?;

    let salt = base64_decode_no_pad(parts[4]).map_err(|_| Argon2Error::InvalidEncoding("invalid base64 in salt"))?;
    let tag = base64_decode_no_pad(parts[5]).map_err(|_| Argon2Error::InvalidEncoding("invalid base64 in hash"))?;

    let params = Params {
        iterations,
        memory,
        parallelism,
    };

    // Reject parameters that `derive_key` would reject anyway, so callers can
    // rely on `decode_phc` returning only usable parameters.
    params.validate(tag.len())?;

    Ok((params, salt, tag))
}

// ============================================================
// Core Argon2 algorithm
// ============================================================

/// A 1024-byte block used in Argon2's memory matrix.
///
/// The 64-byte alignment keeps every block cache-line aligned, which lets the
/// SIMD backends use aligned loads/stores.
#[derive(Clone)]
#[repr(align(64))]
struct Block {
    v: [u64; 128],
}

impl Block {
    #[inline(always)]
    const fn zero() -> Self {
        Block {
            v: [0u64; 128],
        }
    }

    #[inline(always)]
    fn xor_with(&mut self, other: &Block) {
        for (dest, source) in self.v.iter_mut().zip(other.v.iter()) {
            *dest ^= *source;
        }
    }

    /// Build a block from its canonical little-endian byte representation.
    #[inline(always)]
    fn from_bytes(bytes: &[u8; BLOCK_SIZE]) -> Self {
        let mut v = [0u64; 128];
        for (word, chunk) in v.iter_mut().zip(bytes.as_chunks::<8>().0) {
            *word = u64::from_le_bytes(*chunk);
        }
        Block {
            v,
        }
    }

    /// Serialize the block to its canonical little-endian byte representation.
    #[inline(always)]
    fn to_bytes(&self) -> [u8; BLOCK_SIZE] {
        let mut out = [0u8; BLOCK_SIZE];
        for (chunk, word) in out.as_chunks_mut::<8>().0.iter_mut().zip(self.v.iter()) {
            *chunk = word.to_le_bytes();
        }
        out
    }
}

/// Internal function supporting all argon2 types (for testing).
#[cfg(feature = "alloc")]
fn argon2_core(
    argon_type: u32,
    password: &[u8],
    salt: &[u8],
    secret: &[u8],
    ad: &[u8],
    params: &Params,
    out: &mut [u8],
) -> Result<(), Argon2Error> {
    argon2_core_with_backend(argon_type, password, salt, secret, ad, params, out, detect_backend())
}

/// Like [`argon2_core`], but with an explicitly selected compression backend.
/// Used to run the test vectors against every available implementation.
#[cfg(feature = "alloc")]
#[allow(clippy::too_many_arguments)]
fn argon2_core_with_backend(
    argon_type: u32,
    password: &[u8],
    salt: &[u8],
    secret: &[u8],
    ad: &[u8],
    params: &Params,
    out: &mut [u8],
    backend: Backend,
) -> Result<(), Argon2Error> {
    // Validate parameters and lengths. Argon2 encodes every length as a 32-bit
    // little-endian value, so anything larger must be rejected rather than
    // silently truncated.
    params.validate(out.len())?;
    check_u32_len(password.len(), "password must be <= 2^32 - 1 bytes")?;
    check_u32_len(salt.len(), "salt must be <= 2^32 - 1 bytes")?;
    check_u32_len(secret.len(), "secret must be <= 2^32 - 1 bytes")?;
    check_u32_len(ad.len(), "associated data must be <= 2^32 - 1 bytes")?;

    let p = params.parallelism;
    let t = params.iterations;
    let m = params.memory;
    let tag_length = out.len() as u32;

    // Step 1: Compute H_0
    let h0 = compute_h0(argon_type, password, salt, secret, ad, p, tag_length, m, t);

    // Step 2: Determine actual memory size m' (rounded down to multiple of 4*p)
    let m_prime = 4 * p * (m / (4 * p));
    let q = m_prime / p; // columns per lane

    // Allocate memory as m' blocks. This is the only heap allocation performed
    // by the whole derivation; every other buffer lives on the stack. The arena
    // is left uninitialized (see `Memory`).
    let mem = Memory::uninit(m_prime as usize);

    // Step 3 & 4: Compute B[i][0] and B[i][1] for all lanes
    for i in 0..p {
        let mut input = [0u8; 72];
        input[..64].copy_from_slice(&h0);
        input[68..72].copy_from_slice(&i.to_le_bytes());

        let mut block_bytes = [0u8; BLOCK_SIZE];

        // B[i][0] = H'^(1024)(H_0 || LE32(0) || LE32(i))
        input[64..68].copy_from_slice(&0u32.to_le_bytes());
        variable_length_hash_into(&input, &mut block_bytes);
        mem.write((i * q) as usize, Block::from_bytes(&block_bytes));

        // B[i][1] = H'^(1024)(H_0 || LE32(1) || LE32(i))
        input[64..68].copy_from_slice(&1u32.to_le_bytes());
        variable_length_hash_into(&input, &mut block_bytes);
        mem.write((i * q + 1) as usize, Block::from_bytes(&block_bytes));
    }

    // Steps 5-6: Fill memory
    fill_memory(backend, &mem, argon_type, p, q, t, m_prime);

    // Step 7: Compute final block C = XOR of last column
    let mut final_block = mem.get((q - 1) as usize).clone();
    for i in 1..p {
        let idx = (i * q + q - 1) as usize;
        final_block.xor_with(mem.get(idx));
    }

    // Step 8: Output tag = H'^T(C)
    let final_bytes = final_block.to_bytes();
    variable_length_hash_into(&final_bytes, out);

    Ok(())
}

/// Fill the whole memory matrix.
///
/// When the `std` feature is enabled (and the target is not wasm32) and each
/// lane has enough work to amortize thread startup, every lane is filled by
/// its own thread. The threads are scoped to this call: they are spawned here
/// and joined before it returns, so no worker outlives the derivation. Lanes
/// synchronize on a barrier at every slice boundary, which is exactly where
/// Argon2 permits cross-lane reads.
#[cfg(feature = "alloc")]
fn fill_memory(backend: Backend, memory: &Memory, argon_type: u32, p: u32, q: u32, t: u32, m_prime: u32) {
    #[cfg(all(feature = "std", not(target_arch = "wasm32")))]
    {
        // Thread startup only pays off for reasonably large segments.
        const MIN_PARALLEL_SEGMENT_LENGTH: u32 = 64;
        let segment_length = q / SYNC_POINTS;
        if p > 1 && segment_length >= MIN_PARALLEL_SEGMENT_LENGTH {
            // The barrier lives in this frame, so it outlives the scope and its
            // worker threads. The workers themselves never outlive this call.
            let barrier = std::sync::Barrier::new(p as usize);
            std::thread::scope(|scope| {
                let barrier = &barrier;
                for lane in 0..p {
                    scope.spawn(move || {
                        for pass in 0..t {
                            for slice in 0..SYNC_POINTS {
                                fill_segment(backend, memory, argon_type, pass, lane, slice, p, q, t, m_prime);
                                barrier.wait();
                            }
                        }
                    });
                }
            });
            return;
        }
    }

    for pass in 0..t {
        for slice in 0..SYNC_POINTS {
            for lane in 0..p {
                fill_segment(backend, memory, argon_type, pass, lane, slice, p, q, t, m_prime);
            }
        }
    }
}

/// Selects the SIMD kernel used to fill the memory matrix.
///
/// The choice is resolved once per derivation (never per block) and cached for
/// the whole call.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)] // `Scalar` is unused when a SIMD backend is always selected.
enum Backend {
    /// Portable `u64` implementation. Always available.
    Scalar,
    /// AArch64 NEON, which is baseline on `aarch64`.
    #[cfg(target_arch = "aarch64")]
    Neon,
    /// AArch64 NEON with the `sha3` extension, which lets LLVM fuse the
    /// rotate-xor steps into a single `xar` instruction.
    #[cfg(target_arch = "aarch64")]
    NeonSha3,
    /// x86-64 AVX2.
    #[cfg(target_arch = "x86_64")]
    Avx2,
}

/// Detect the fastest compression kernel available on the running CPU.
///
/// Depending on the target architecture and feature set, any one of the
/// cfg-gated branches below is the terminal path, so they are written as
/// explicit returns rather than trailing expressions.
#[allow(clippy::needless_return)]
fn detect_backend() -> Backend {
    #[cfg(target_arch = "aarch64")]
    {
        #[cfg(feature = "std")]
        {
            if std::arch::is_aarch64_feature_detected!("sha3") {
                return Backend::NeonSha3;
            }
            return Backend::Neon;
        }

        #[cfg(all(not(feature = "std"), target_feature = "sha3"))]
        return Backend::NeonSha3;

        #[cfg(all(not(feature = "std"), not(target_feature = "sha3")))]
        return Backend::Neon;
    }

    #[cfg(target_arch = "x86_64")]
    {
        #[cfg(feature = "std")]
        {
            if std::arch::is_x86_feature_detected!("avx2") {
                return Backend::Avx2;
            }
            return Backend::Scalar;
        }

        #[cfg(all(not(feature = "std"), target_feature = "avx2"))]
        return Backend::Avx2;

        #[cfg(all(not(feature = "std"), not(target_feature = "avx2")))]
        return Backend::Scalar;
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    return Backend::Scalar;
}

/// Fill a segment of the memory matrix using the selected backend.
///
/// The compression kernel is called directly for the selected backend. The
/// backend is loop-invariant, so the `match` can be hoisted out of the hot
/// loop. The `#[target_feature]` kernels (`NeonSha3` and `Avx2`) remain
/// separate calls, since a function compiled with a target feature is never
/// inlined into a caller that does not enable it.
#[cfg(feature = "alloc")]
#[allow(clippy::too_many_arguments)]
fn fill_segment(
    backend: Backend,
    memory: &Memory,
    argon_type: u32,
    pass: u32,
    lane: u32,
    slice: u32,
    lanes: u32,
    q: u32,       // columns per lane
    t: u32,       // total passes
    m_prime: u32, // total blocks
) {
    let segment_length = q / SYNC_POINTS;

    // For Argon2i and Argon2id (first half of first pass), addresses are
    // derived from a pseudo-random stream. It is produced 128 words at a time
    // into a stack buffer, so no heap allocation is needed.
    let need_pseudo_rands = argon_type == ARGON2I || (argon_type == ARGON2ID && pass == 0 && slice < 2);
    let mut addr_words = [0u64; 128];
    let mut addr_chunk = u32::MAX;

    let start_index = if pass == 0 && slice == 0 { 2 } else { 0 };

    for s in start_index..segment_length {
        let j = slice * segment_length + s; // current column index in this lane
        let cur_index = (lane * q + j) as usize;

        // Previous block index
        let prev_index = if j == 0 {
            (lane * q + q - 1) as usize
        } else {
            (lane * q + j - 1) as usize
        };

        // Determine J1 and J2
        let (j1, j2) = if need_pseudo_rands {
            let chunk = s / 128;
            if chunk != addr_chunk {
                generate_address_block(pass, lane, slice, t, argon_type, m_prime, (chunk + 1) as u64, &mut addr_words);
                addr_chunk = chunk;
            }
            let val = addr_words[(s % 128) as usize];
            ((val & 0xFFFFFFFF) as u32, (val >> 32) as u32)
        } else {
            // Argon2d mode: use first 64 bits of previous block
            let word = memory.first_word(prev_index);
            (word as u32, (word >> 32) as u32)
        };

        // Map J1, J2 to reference block index
        let ref_lane = if pass == 0 && slice == 0 { lane } else { j2 % lanes };

        let ref_index = index_alpha(pass, slice, lanes, segment_length, s, q, ref_lane == lane, j1);
        let ref_block_index = (ref_lane * q + ref_index) as usize;

        // Argon2 never references the block currently being written, so
        // `cur_index` differs from both `prev_index` and `ref_block_index`.
        // Cross-lane reads target blocks finalized at this synchronization
        // point, so they never race with a concurrent write.
        //
        // SAFETY: the two shared borrows and the raw pointer target
        // pairwise-distinct blocks (see `Memory`), and `backend` is only ever
        // one that `detect_backend` selected for this CPU.
        unsafe {
            let prev = memory.get(prev_index);
            let reference = memory.get(ref_block_index);
            let cur = memory.get_mut(cur_index);
            match backend {
                Backend::Scalar => fill::fill_block(prev, reference, cur, pass != 0),
                #[cfg(target_arch = "aarch64")]
                Backend::Neon => fill_neon::fill_block(prev, reference, cur, pass != 0),
                #[cfg(target_arch = "aarch64")]
                Backend::NeonSha3 => fill_neon::fill_block_sha3(prev, reference, cur, pass != 0),
                #[cfg(target_arch = "x86_64")]
                Backend::Avx2 => fill_avx2::fill_block(prev, reference, cur, pass != 0),
            }
        }
    }
}

/// Fill `out` with one 128-word block of pseudo-random addresses for
/// Argon2i/Argon2id data-independent addressing.
#[cfg(feature = "alloc")]
fn generate_address_block(
    pass: u32,
    lane: u32,
    slice: u32,
    t: u32,
    argon_type: u32,
    m_prime: u32,
    counter: u64,
    out: &mut [u64; 128],
) {
    // Build input block
    let mut input = Block::zero();
    input.v[0] = pass as u64;
    input.v[1] = lane as u64;
    input.v[2] = slice as u64;
    input.v[3] = m_prime as u64;
    input.v[4] = t as u64;
    input.v[5] = argon_type as u64;
    input.v[6] = counter;

    let zero_block = Block::zero();
    let mut tmp = Block::zero();
    fill::fill_block_ref(&zero_block, &input, &mut tmp, false);
    let mut addr_block = Block::zero();
    fill::fill_block_ref(&zero_block, &tmp, &mut addr_block, false);
    out.copy_from_slice(&addr_block.v);
}

/// Map J1 to a reference block index within the available set W.
fn index_alpha(
    pass: u32,
    slice: u32,
    _lanes: u32,
    segment_length: u32,
    index_in_segment: u32,
    q: u32,
    same_lane: bool,
    j1: u32,
) -> u32 {
    // Determine reference area size
    let reference_area_size = if pass == 0 {
        // First pass: can only reference blocks already computed
        if slice == 0 {
            // Same lane, same slice, only previous blocks
            index_in_segment.saturating_sub(1)
        } else {
            if same_lane {
                slice * segment_length + index_in_segment - 1
            } else {
                slice * segment_length - if index_in_segment == 0 { 1 } else { 0 }
            }
        }
    } else {
        // Subsequent passes: all blocks except the current one
        if same_lane {
            q - segment_length + index_in_segment - 1
        } else {
            q - segment_length - if index_in_segment == 0 { 1 } else { 0 }
        }
    };

    if reference_area_size == 0 {
        return 0;
    }

    // Map J1 to an index with bias toward recent blocks
    let j1_64 = j1 as u64;
    let x = (j1_64 * j1_64) >> 32;
    let y = (reference_area_size as u64 * x) >> 32;
    let relative_position = (reference_area_size as u64 - 1 - y) as u32;

    // Compute starting position
    let start_position = if pass == 0 {
        0
    } else {
        if slice == SYNC_POINTS - 1 {
            0
        } else {
            (slice + 1) * segment_length
        }
    };

    (start_position + relative_position) % q
}

/// Compute H_0 as defined in the RFC.
#[cfg(feature = "alloc")]
fn compute_h0(
    argon_type: u32,
    password: &[u8],
    salt: &[u8],
    secret: &[u8],
    ad: &[u8],
    p: u32,
    tag_length: u32,
    m: u32,
    t: u32,
) -> [u8; 64] {
    let mut blake = Blake2b::new_keyed(&[], 64);

    blake.update(&p.to_le_bytes());
    blake.update(&tag_length.to_le_bytes());
    blake.update(&m.to_le_bytes());
    blake.update(&t.to_le_bytes());
    blake.update(&VERSION.to_le_bytes());
    blake.update(&argon_type.to_le_bytes());
    blake.update(&(password.len() as u32).to_le_bytes());
    blake.update(password);
    blake.update(&(salt.len() as u32).to_le_bytes());
    blake.update(salt);
    blake.update(&(secret.len() as u32).to_le_bytes());
    blake.update(secret);
    blake.update(&(ad.len() as u32).to_le_bytes());
    blake.update(ad);

    let hash = blake.sum();
    let mut result = [0u8; 64];
    result.copy_from_slice(&hash.as_ref()[..64]);
    result
}

/// Variable-length hash function H' as defined in RFC 9106 Section 3.3.
///
/// Uses Blake2b to fill `out` (its length is the tag length `T`) without
/// performing any heap allocation.
#[cfg(feature = "alloc")]
fn variable_length_hash_into(input: &[u8], out: &mut [u8]) {
    let tag_length = out.len();
    debug_assert!(u32::try_from(tag_length).is_ok(), "tag length must fit in a u32");

    if tag_length <= 64 {
        // Short output: H'^T(A) = H^T(LE32(T)||A)
        let mut blake = Blake2b::new_keyed(&[], tag_length);
        blake.update(&(tag_length as u32).to_le_bytes());
        blake.update(input);
        let hash = blake.sum();
        out.copy_from_slice(&hash.as_ref()[..tag_length]);
        return;
    }

    // Long output
    // r = ceil(T/32) - 2
    let r = tag_length.div_ceil(32) - 2;

    // V_1 = H^(64)(LE32(T)||A)
    let mut v = [0u8; 64];
    {
        let mut blake = Blake2b::new_keyed(&[], 64);
        blake.update(&(tag_length as u32).to_le_bytes());
        blake.update(input);
        let hash = blake.sum();
        v.copy_from_slice(&hash.as_ref()[..64]);
    }

    // W_1 = first 32 bytes of V_1
    out[..32].copy_from_slice(&v[..32]);

    // V_2 through V_r
    let mut offset = 32;
    for _ in 2..=r {
        let mut blake = Blake2b::new_keyed(&[], 64);
        blake.update(&v);
        let hash = blake.sum();
        v.copy_from_slice(&hash.as_ref()[..64]);
        out[offset..offset + 32].copy_from_slice(&v[..32]);
        offset += 32;
    }

    // V_{r+1} = H^(T-32*r)(V_r)
    let remaining = tag_length - 32 * r;
    let mut blake = Blake2b::new_keyed(&[], remaining);
    blake.update(&v);
    let hash = blake.sum();
    out[offset..offset + remaining].copy_from_slice(&hash.as_ref()[..remaining]);
}

// ============================================================
// Compression function G and Permutation P
// ============================================================

/// Argon2's block arena, and the only place where memory unsafety lives.
///
/// Argon2 fills disjoint blocks concurrently while reading blocks that were
/// finalized at an earlier synchronization point, an access pattern the borrow
/// checker cannot describe directly. All of that unsafety is confined here:
///
/// * the arena is allocated uninitialized and written through a shared
///   reference (every block is written before it is read);
/// * [`Memory::get`] and [`Memory::get_mut`] hand out two shared borrows and a
///   raw pointer to three pairwise-distinct blocks, so the kernel's reads and
///   write do not alias;
/// * lanes only ever write their own blocks, so concurrent calls from different
///   lanes touch disjoint blocks.
struct Memory {
    blocks: Vec<MaybeUninit<Block>>,
}

// SAFETY: concurrent access is always disjoint. Every write targets the calling
// lane's own current block, while reads only target blocks finalized at an
// earlier synchronization point or this lane's own previous block. See
// `fill_segment`.
unsafe impl Send for Memory {}
unsafe impl Sync for Memory {}

impl Memory {
    /// Allocate an uninitialized arena of `len` blocks.
    ///
    /// The contents are left uninitialized on purpose: Argon2 writes every
    /// block before reading it, so zero-filling the arena would be a wasted
    /// pass over the whole allocation.
    fn uninit(len: usize) -> Self {
        let mut blocks: Vec<MaybeUninit<Block>> = Vec::with_capacity(len);
        // SAFETY: `Block` is plain-old-data (an array of `u64`), so every bit
        // pattern is valid, and `MaybeUninit` tolerates uninitialized contents.
        // The length is set to the capacity that was just reserved.
        unsafe { blocks.set_len(len) };
        Memory {
            blocks,
        }
    }

    /// Base pointer of the arena.
    #[inline(always)]
    fn base_ptr(&self) -> *mut Block {
        self.blocks.as_ptr() as *mut Block
    }

    /// Return the number of blocks in the arena.
    #[inline(always)]
    fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Write `block` to `index`.
    ///
    /// The block at `index` may be uninitialized beforehand; its previous
    /// contents are not read.
    #[inline(always)]
    fn write(&self, index: usize, block: Block) {
        debug_assert!(index < self.len());
        // SAFETY: `index` is in bounds and no other access to that block is
        // live, so the write does not alias.
        unsafe { core::ptr::write(self.base_ptr().add(index), block) };
    }

    /// Read the block at `index`.
    ///
    /// Callers must only read blocks that have already been written and are not
    /// concurrently written.
    #[inline(always)]
    fn get(&self, index: usize) -> &Block {
        debug_assert!(index < self.len());
        // SAFETY: `index` is in bounds, the block has been initialized, and it
        // is not concurrently written (see the type-level invariant).
        unsafe { &*self.base_ptr().add(index) }
    }

    /// First word of the block at `index` (used by Argon2d addressing).
    #[inline(always)]
    fn first_word(&self, index: usize) -> u64 {
        self.get(index).v[0]
    }

    /// Raw pointer to the block at `index`, without dereferencing it.
    ///
    /// This may point at an uninitialized block (on the first pass) and is
    /// handed to the compression kernel, which is responsible for writing it.
    #[inline(always)]
    fn get_mut(&self, index: usize) -> *mut Block {
        debug_assert!(index < self.len());
        // SAFETY: `index` is in bounds, so the offset stays within the
        // allocation. The returned pointer is not dereferenced here.
        unsafe { self.base_ptr().add(index) }
    }
}

// ============================================================
// Base64 helpers (PHC format uses standard base64 without padding)
// ============================================================

#[cfg(feature = "alloc")]
fn base64_encode_no_pad(input: &[u8]) -> String {
    base64::encode(input, base64::Alphabet::StandardNoPadding)
}

#[cfg(feature = "alloc")]
fn base64_decode_no_pad(input: &str) -> Result<Vec<u8>, ()> {
    base64::decode(input.as_bytes(), base64::Alphabet::StandardNoPadding).map_err(|_| ())
}

#[cfg(feature = "alloc")]
fn parse_param(s: &str, prefix: &str) -> Result<u32, Argon2Error> {
    if !s.starts_with(prefix) {
        return Err(Argon2Error::InvalidEncoding("invalid parameter prefix"));
    }
    s[prefix.len()..]
        .parse::<u32>()
        .map_err(|_| Argon2Error::InvalidEncoding("invalid parameter value"))
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn derive_key_typed(
        argon_type: u32,
        password: &[u8],
        salt: &[u8],
        secret: &[u8],
        ad: &[u8],
        iterations: u32,
        memory: u32,
        parallelism: u32,
        tag_length: u32,
    ) -> Vec<u8> {
        let params = Params {
            iterations: iterations,
            memory: memory,
            parallelism: parallelism,
        };
        let mut out = vec![0u8; tag_length as usize];
        argon2_core(argon_type, password, salt, secret, ad, &params, &mut out).unwrap();
        out
    }

    /// Like `derive_key_typed`, but with an explicit backend.
    #[allow(clippy::too_many_arguments)]
    fn derive_key_typed_backend(
        backend: Backend,
        argon_type: u32,
        password: &[u8],
        salt: &[u8],
        secret: &[u8],
        ad: &[u8],
        iterations: u32,
        memory: u32,
        parallelism: u32,
        tag_length: u32,
    ) -> Vec<u8> {
        let params = Params {
            iterations,
            memory,
            parallelism,
        };
        let mut out = vec![0u8; tag_length as usize];
        argon2_core_with_backend(argon_type, password, salt, secret, ad, &params, &mut out, backend).unwrap();
        out
    }

    /// Every backend compiled into this build and actually supported by the
    /// running CPU.
    fn available_backends() -> Vec<Backend> {
        let mut backends = vec![Backend::Scalar];

        #[cfg(target_arch = "aarch64")]
        {
            backends.push(Backend::Neon);
            #[cfg(feature = "std")]
            if std::arch::is_aarch64_feature_detected!("sha3") {
                backends.push(Backend::NeonSha3);
            }
        }

        #[cfg(target_arch = "x86_64")]
        {
            #[cfg(feature = "std")]
            if std::arch::is_x86_feature_detected!("avx2") {
                backends.push(Backend::Avx2);
            }
        }

        backends
    }

    /// Every SIMD backend must reproduce the scalar kernel bit-for-bit, for
    /// all three Argon2 types, multiple passes and multiple lanes.
    #[test]
    fn test_backends_match_reference() {
        let password = b"password";
        let salt = b"somesalt";
        for backend in available_backends() {
            for v in GO_VECTORS.iter() {
                let expected = hex::decode(v.hash).unwrap();
                let result = derive_key_typed_backend(
                    backend,
                    v.mode,
                    password,
                    salt,
                    &[],
                    &[],
                    v.time,
                    v.memory,
                    v.threads,
                    expected.len() as u32,
                );
                assert_eq!(
                    result, expected,
                    "backend {:?} failed Go vector (mode={}, t={}, m={}, p={})",
                    backend, v.mode, v.time, v.memory, v.threads
                );
            }
        }
    }

    /// Convenience wrapper around `derive_key` that returns an allocated tag.
    fn derive_key_vec(
        tag_length: usize,
        password: &[u8],
        salt: &[u8],
        secret: &[u8],
        ad: &[u8],
        params: &Params,
    ) -> Vec<u8> {
        let mut out = vec![0u8; tag_length];
        derive_key(&mut out, password, salt, secret, ad, params).unwrap();
        out
    }

    /// Convenience wrapper around `variable_length_hash_into` returning a Vec.
    fn variable_length_hash_vec(input: &[u8], tag_length: usize) -> Vec<u8> {
        let mut out = vec![0u8; tag_length];
        variable_length_hash_into(input, &mut out);
        out
    }

    // ================================================================
    // RFC 9106 Section 5 test vectors
    // password = 0x01*32, salt = 0x02*16, secret = 0x03*8, ad = 0x04*12
    // t=3, m=32, p=4, tag=32
    // ================================================================

    #[test]
    fn test_rfc9106_argon2d() {
        let pwd = vec![0x01u8; 32];
        let salt = vec![0x02u8; 16];
        let secret = vec![0x03u8; 8];
        let ad = vec![0x04u8; 12];
        let expected = hex::decode("512b391b6f1162975371d30919734294f868e3be3984f3c1a13a4db9fabe4acb").unwrap();
        let result = derive_key_typed(ARGON2D, &pwd, &salt, &secret, &ad, 3, 32, 4, 32);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rfc9106_argon2i() {
        let pwd = vec![0x01u8; 32];
        let salt = vec![0x02u8; 16];
        let secret = vec![0x03u8; 8];
        let ad = vec![0x04u8; 12];
        let expected = hex::decode("c814d9d1dc7f37aa13f0d77f2494bda1c8de6b016dd388d29952a4c4672b6ce8").unwrap();
        let result = derive_key_typed(ARGON2I, &pwd, &salt, &secret, &ad, 3, 32, 4, 32);
        assert_eq!(result, expected);
    }

    // ================================================================
    // RFC 9106 H_0 pre-hashing digest tests for all types
    // ================================================================
    // Pre-hashing digest test (H0 from RFC 9106 Section 5.3)
    // ================================================================

    #[test]
    fn test_h0() {
        let pwd = vec![0x01u8; 32];
        let salt = vec![0x02u8; 16];
        let secret = vec![0x03u8; 8];
        let ad = vec![0x04u8; 12];
        let h0 = compute_h0(ARGON2ID, &pwd, &salt, &secret, &ad, 4, 32, 32, 3);
        let expected = "2889de487eb42ae500c0007ed9252f1069eadec40d5765b485de6dc2437a67b8546a2f0acc1a0882db8fcf74714b472e94df421a5da1112ffa11434370a1e997";
        assert_eq!(hex::encode(h0), expected);
    }

    // ================================================================
    // Test vectors from golang.org/x/crypto/argon2
    // password = "password", salt = "somesalt", no secret, no AD
    // ================================================================

    struct Vec3 {
        mode: u32,
        time: u32,
        memory: u32,
        threads: u32,
        hash: &'static str,
    }

    const GO_VECTORS: &[Vec3] = &[
        Vec3 {
            mode: ARGON2I,
            time: 1,
            memory: 64,
            threads: 1,
            hash: "b9c401d1844a67d50eae3967dc28870b22e508092e861a37",
        },
        Vec3 {
            mode: ARGON2D,
            time: 1,
            memory: 64,
            threads: 1,
            hash: "8727405fd07c32c78d64f547f24150d3f2e703a89f981a19",
        },
        Vec3 {
            mode: ARGON2ID,
            time: 1,
            memory: 64,
            threads: 1,
            hash: "655ad15eac652dc59f7170a7332bf49b8469be1fdb9c28bb",
        },
        Vec3 {
            mode: ARGON2I,
            time: 2,
            memory: 64,
            threads: 1,
            hash: "8cf3d8f76a6617afe35fac48eb0b7433a9a670ca4a07ed64",
        },
        Vec3 {
            mode: ARGON2D,
            time: 2,
            memory: 64,
            threads: 1,
            hash: "3be9ec79a69b75d3752acb59a1fbb8b295a46529c48fbb75",
        },
        Vec3 {
            mode: ARGON2ID,
            time: 2,
            memory: 64,
            threads: 1,
            hash: "068d62b26455936aa6ebe60060b0a65870dbfa3ddf8d41f7",
        },
        Vec3 {
            mode: ARGON2I,
            time: 2,
            memory: 64,
            threads: 2,
            hash: "2089f3e78a799720f80af806553128f29b132cafe40d059f",
        },
        Vec3 {
            mode: ARGON2D,
            time: 2,
            memory: 64,
            threads: 2,
            hash: "68e2462c98b8bc6bb60ec68db418ae2c9ed24fc6748a40e9",
        },
        Vec3 {
            mode: ARGON2ID,
            time: 2,
            memory: 64,
            threads: 2,
            hash: "350ac37222f436ccb5c0972f1ebd3bf6b958bf2071841362",
        },
        Vec3 {
            mode: ARGON2I,
            time: 3,
            memory: 256,
            threads: 2,
            hash: "f5bbf5d4c3836af13193053155b73ec7476a6a2eb93fd5e6",
        },
        Vec3 {
            mode: ARGON2D,
            time: 3,
            memory: 256,
            threads: 2,
            hash: "f4f0669218eaf3641f39cc97efb915721102f4b128211ef2",
        },
        Vec3 {
            mode: ARGON2ID,
            time: 3,
            memory: 256,
            threads: 2,
            hash: "4668d30ac4187e6878eedeacf0fd83c5a0a30db2cc16ef0b",
        },
        Vec3 {
            mode: ARGON2I,
            time: 4,
            memory: 4096,
            threads: 4,
            hash: "a11f7b7f3f93f02ad4bddb59ab62d121e278369288a0d0e7",
        },
        Vec3 {
            mode: ARGON2D,
            time: 4,
            memory: 4096,
            threads: 4,
            hash: "935598181aa8dc2b720914aa6435ac8d3e3a4210c5b0fb2d",
        },
        Vec3 {
            mode: ARGON2ID,
            time: 4,
            memory: 4096,
            threads: 4,
            hash: "145db9733a9f4ee43edf33c509be96b934d505a4efb33c5a",
        },
        Vec3 {
            mode: ARGON2I,
            time: 4,
            memory: 1024,
            threads: 8,
            hash: "0cdd3956aa35e6b475a7b0c63488822f774f15b43f6e6e17",
        },
        Vec3 {
            mode: ARGON2D,
            time: 4,
            memory: 1024,
            threads: 8,
            hash: "83604fc2ad0589b9d055578f4d3cc55bc616df3578a896e9",
        },
        Vec3 {
            mode: ARGON2ID,
            time: 4,
            memory: 1024,
            threads: 8,
            hash: "8dafa8e004f8ea96bf7c0f93eecf67a6047476143d15577f",
        },
        Vec3 {
            mode: ARGON2I,
            time: 2,
            memory: 64,
            threads: 3,
            hash: "5cab452fe6b8479c8661def8cd703b611a3905a6d5477fe6",
        },
        Vec3 {
            mode: ARGON2D,
            time: 2,
            memory: 64,
            threads: 3,
            hash: "22474a423bda2ccd36ec9afd5119e5c8949798cadf659f51",
        },
        Vec3 {
            mode: ARGON2ID,
            time: 2,
            memory: 64,
            threads: 3,
            hash: "4a15b31aec7c2590b87d1f520be7d96f56658172deaa3079",
        },
        Vec3 {
            mode: ARGON2I,
            time: 3,
            memory: 1024,
            threads: 6,
            hash: "d236b29c2b2a09babee842b0dec6aa1e83ccbdea8023dced",
        },
        Vec3 {
            mode: ARGON2D,
            time: 3,
            memory: 1024,
            threads: 6,
            hash: "a3351b0319a53229152023d9206902f4ef59661cdca89481",
        },
        Vec3 {
            mode: ARGON2ID,
            time: 3,
            memory: 1024,
            threads: 6,
            hash: "1640b932f4b60e272f5d2207b9a9c626ffa1bd88d2349016",
        },
    ];

    #[test]
    fn test_go_vectors() {
        let password = b"password";
        let salt = b"somesalt";
        for (i, v) in GO_VECTORS.iter().enumerate() {
            let expected = hex::decode(v.hash).unwrap();
            let result = derive_key_typed(
                v.mode,
                password,
                salt,
                &[],
                &[],
                v.time,
                v.memory,
                v.threads,
                expected.len() as u32,
            );
            assert_eq!(
                result, expected,
                "Go vector {} failed (mode={}, t={}, m={}, p={})",
                i, v.mode, v.time, v.memory, v.threads
            );
        }
    }

    // ================================================================
    // Test vectors from the C reference implementation (phc-winner-argon2)
    // https://github.com/P-H-C/phc-winner-argon2/blob/master/src/test.c
    // All use password="password", salt="somesalt" unless noted, v=19
    // ================================================================

    struct CVector {
        mode: u32,
        time: u32,
        memory: u32,
        threads: u32,
        hash: &'static str,
        pwd: &'static str,
        slt: &'static str,
    }

    const C_VECTORS: &[CVector] = &[
        CVector {
            mode: ARGON2I,
            time: 2,
            memory: 65536,
            threads: 1,
            hash: "c1628832147d9720c5bd1cfd61367078729f6dfb6f8fea9ff98158e0d7816ed0",
            pwd: "password",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2I,
            time: 2,
            memory: 262144,
            threads: 1,
            hash: "296dbae80b807cdceaad44ae741b506f14db0959267b183b118f9b24229bc7cb",
            pwd: "password",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2I,
            time: 2,
            memory: 256,
            threads: 1,
            hash: "89e9029f4637b295beb027056a7336c414fadd43f6b208645281cb214a56452f",
            pwd: "password",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2I,
            time: 2,
            memory: 256,
            threads: 2,
            hash: "4ff5ce2769a1d7f4c8a491df09d41a9fbe90e5eb02155a13e4c01e20cd4eab61",
            pwd: "password",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2I,
            time: 1,
            memory: 65536,
            threads: 1,
            hash: "d168075c4d985e13ebeae560cf8b94c3b5d8a16c51916b6f4ac2da3ac11bbecf",
            pwd: "password",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2I,
            time: 4,
            memory: 65536,
            threads: 1,
            hash: "aaa953d58af3706ce3df1aefd4a64a84e31d7f54175231f1285259f88174ce5b",
            pwd: "password",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2I,
            time: 2,
            memory: 65536,
            threads: 1,
            hash: "14ae8da01afea8700c2358dcef7c5358d9021282bd88663a4562f59fb74d22ee",
            pwd: "differentpassword",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2I,
            time: 2,
            memory: 65536,
            threads: 1,
            hash: "b0357cccfbef91f3860b0dba447b2348cbefecadaf990abfe9cc40726c521271",
            pwd: "password",
            slt: "diffsalt",
        },
        CVector {
            mode: ARGON2ID,
            time: 2,
            memory: 65536,
            threads: 1,
            hash: "09316115d5cf24ed5a15a31a3ba326e5cf32edc24702987c02b6566f61913cf7",
            pwd: "password",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2ID,
            time: 2,
            memory: 262144,
            threads: 1,
            hash: "78fe1ec91fb3aa5657d72e710854e4c3d9b9198c742f9616c2f085bed95b2e8c",
            pwd: "password",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2ID,
            time: 2,
            memory: 256,
            threads: 1,
            hash: "9dfeb910e80bad0311fee20f9c0e2b12c17987b4cac90c2ef54d5b3021c68bfe",
            pwd: "password",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2ID,
            time: 2,
            memory: 256,
            threads: 2,
            hash: "6d093c501fd5999645e0ea3bf620d7b8be7fd2db59c20d9fff9539da2bf57037",
            pwd: "password",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2ID,
            time: 1,
            memory: 65536,
            threads: 1,
            hash: "f6a5adc1ba723dddef9b5ac1d464e180fcd9dffc9d1cbf76cca2fed795d9ca98",
            pwd: "password",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2ID,
            time: 4,
            memory: 65536,
            threads: 1,
            hash: "9025d48e68ef7395cca9079da4c4ec3affb3c8911fe4f86d1a2520856f63172c",
            pwd: "password",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2ID,
            time: 2,
            memory: 65536,
            threads: 1,
            hash: "0b84d652cf6b0c4beaef0dfe278ba6a80df6696281d7e0d2891b817d8c458fde",
            pwd: "differentpassword",
            slt: "somesalt",
        },
        CVector {
            mode: ARGON2ID,
            time: 2,
            memory: 65536,
            threads: 1,
            hash: "bdf32b05ccc42eb15d58fd19b1f856b113da1e9a5874fdcc544308565aa8141c",
            pwd: "password",
            slt: "diffsalt",
        },
    ];

    #[test]
    fn test_c_reference_vectors() {
        for (i, v) in C_VECTORS.iter().enumerate() {
            let expected = hex::decode(v.hash).unwrap();
            let result = derive_key_typed(
                v.mode,
                v.pwd.as_bytes(),
                v.slt.as_bytes(),
                &[],
                &[],
                v.time,
                v.memory,
                v.threads,
                expected.len() as u32,
            );
            assert_eq!(
                result, expected,
                "C ref vector {} failed (mode={}, t={}, m={}, p={})",
                i, v.mode, v.time, v.memory, v.threads
            );
        }
    }

    // ================================================================
    // PHC string format tests
    // ================================================================

    #[test]
    fn test_phc_encode_decode() {
        let params = Params {
            iterations: 3,
            memory: 65536,
            parallelism: 4,
        };
        let salt = b"somesalt12345678";
        let tag = vec![0xAB; 32];
        let encoded = encode_phc(&params, salt, &tag);
        assert!(encoded.starts_with("$argon2id$v=19$m=65536,t=3,p=4$"));
        let (dp, ds, dt) = decode_phc(&encoded).unwrap();
        assert_eq!(dp.iterations, 3);
        assert_eq!(dp.memory, 65536);
        assert_eq!(dp.parallelism, 4);
        assert_eq!(ds, salt);
        assert_eq!(dt, tag);
    }

    #[test]
    fn test_hash_and_verify() {
        let password = b"correct horse battery staple";
        let salt = b"randomsalt123456";
        let params = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        let encoded = hash_password(password, salt, &params).unwrap();
        assert!(verify_password(password, &encoded, None).is_ok());
        assert_eq!(
            verify_password(b"wrong password", &encoded, None),
            Err(Argon2Error::VerifyMismatch)
        );
    }

    #[test]
    fn test_decode_phc_invalid() {
        assert!(decode_phc("").is_err());
        assert!(decode_phc("$argon2i$v=19$m=4096,t=3,p=1$salt$hash").is_err());
        assert!(decode_phc("$argon2id$v=16$m=4096,t=3,p=1$salt$hash").is_err());
        assert!(decode_phc("not a phc string").is_err());
    }

    #[test]
    fn test_decode_phc_rejects_invalid_params() {
        let salt = b"salt12345678";
        let tag = [0u8; 32];

        // iterations must be >= 1
        let enc = encode_phc(
            &Params {
                iterations: 0,
                memory: 64,
                parallelism: 1,
            },
            salt,
            &tag,
        );
        assert!(matches!(decode_phc(&enc), Err(Argon2Error::InvalidParams(_))));

        // parallelism must be >= 1
        let enc = encode_phc(
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 0,
            },
            salt,
            &tag,
        );
        assert!(matches!(decode_phc(&enc), Err(Argon2Error::InvalidParams(_))));

        // parallelism must be <= 2^24 - 1
        let enc = encode_phc(
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: u32::MAX,
            },
            salt,
            &tag,
        );
        assert!(matches!(decode_phc(&enc), Err(Argon2Error::InvalidParams(_))));

        // memory must be >= 8*parallelism
        let enc = encode_phc(
            &Params {
                iterations: 1,
                memory: 4,
                parallelism: 1,
            },
            salt,
            &tag,
        );
        assert!(matches!(decode_phc(&enc), Err(Argon2Error::InvalidParams(_))));

        // tag must be at least 4 bytes
        let enc = encode_phc(
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
            },
            salt,
            &[0u8; 3],
        );
        assert!(matches!(decode_phc(&enc), Err(Argon2Error::InvalidParams(_))));
    }

    #[test]
    fn test_decode_phc_huge_parallelism_does_not_panic() {
        let salt = b"salt12345678";
        let tag = [0u8; 32];
        // Before the fix, `8 * parallelism` overflowed in debug builds (panic)
        // and wrapped in release builds (multi-terabyte allocation).
        let enc = encode_phc(
            &Params {
                iterations: 1,
                memory: u32::MAX,
                parallelism: u32::MAX,
            },
            salt,
            &tag,
        );
        assert!(decode_phc(&enc).is_err());
        assert!(verify_password(b"password", &enc, None).is_err());
    }

    #[test]
    fn test_verify_password_limits() {
        let password = b"correct horse battery staple";
        let salt = b"randomsalt123456";
        let params = Params {
            iterations: 2,
            memory: 64,
            parallelism: 1,
        };
        let encoded = hash_password(password, salt, &params).unwrap();

        // No limits: normal verification.
        assert!(verify_password(password, &encoded, None).is_ok());

        // Limits exactly equal to the hash's parameters still verify.
        assert!(verify_password(password, &encoded, Some(&params)).is_ok());

        // Any tighter bound rejects the hash before deriving.
        for limits in [
            Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
            },
            Params {
                iterations: 2,
                memory: 32,
                parallelism: 1,
            },
            Params {
                iterations: 2,
                memory: 64,
                parallelism: 0,
            },
        ] {
            assert!(matches!(
                verify_password(password, &encoded, Some(&limits)),
                Err(Argon2Error::InvalidParams(_))
            ));
        }
    }

    #[test]
    fn test_verify_password_limits_reject_before_allocating() {
        let salt = b"salt12345678";
        let tag = [0u8; 32];
        // A hostile hash requesting ~4 GiB of memory. The limit check must
        // reject it before `derive_key` allocates anything.
        let hostile = encode_phc(
            &Params {
                iterations: 1,
                memory: u32::MAX,
                parallelism: 1,
            },
            salt,
            &tag,
        );
        let limits = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        assert!(matches!(
            verify_password(b"password", &hostile, Some(&limits)),
            Err(Argon2Error::InvalidParams(_))
        ));
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_check_u32_len() {
        assert!(check_u32_len(usize::MAX, "too long").is_err());
        assert!(check_u32_len(u32::MAX as usize, "ok").is_ok());
        assert!(check_u32_len(0, "ok").is_ok());
    }

    #[test]
    fn test_invalid_params() {
        let mut out = [0u8; 32];
        assert!(
            derive_key(
                &mut out,
                b"password",
                b"salt",
                &[],
                &[],
                &Params {
                    iterations: 0,
                    memory: 64,
                    parallelism: 1
                }
            )
            .is_err()
        );
        assert!(
            derive_key(
                &mut out,
                b"password",
                b"salt",
                &[],
                &[],
                &Params {
                    iterations: 1,
                    memory: 4,
                    parallelism: 1
                }
            )
            .is_err()
        );
        // The output buffer must be at least 4 bytes long.
        let mut short = [0u8; 3];
        assert!(
            derive_key(
                &mut short,
                b"password",
                b"salt",
                &[],
                &[],
                &Params {
                    iterations: 1,
                    memory: 64,
                    parallelism: 1
                }
            )
            .is_err()
        );
    }

    #[test]
    fn test_variable_length_hash_short() {
        let input = b"test input";
        let r32 = variable_length_hash_vec(input, 32);
        assert_eq!(r32.len(), 32);
        assert_eq!(variable_length_hash_vec(input, 32), r32);
        let r48 = variable_length_hash_vec(input, 48);
        assert_eq!(r48.len(), 48);
        assert_ne!(&r32[..], &r48[..32]);
    }

    #[test]
    fn test_variable_length_hash_long() {
        assert_eq!(variable_length_hash_vec(b"test input for long hash", 128).len(), 128);
        assert_eq!(variable_length_hash_vec(b"test input for long hash", 1024).len(), 1024);
    }

    #[test]
    fn test_argon2id_min_memory() {
        let result = derive_key_vec(
            32,
            b"password",
            b"saltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 8,
                parallelism: 1,
            },
        );
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_argon2id_multiple_lanes() {
        let result = derive_key_vec(
            32,
            b"password",
            b"saltsaltsaltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 4,
            },
        );
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_different_passwords() {
        let p = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        assert_ne!(
            derive_key_vec(32, b"password1", b"saltsaltsaltsalt", &[], &[], &p),
            derive_key_vec(32, b"password2", b"saltsaltsaltsalt", &[], &[], &p)
        );
    }

    #[test]
    fn test_different_salts() {
        let p = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        assert_ne!(
            derive_key_vec(32, b"password", b"salt1234salt1234", &[], &[], &p),
            derive_key_vec(32, b"password", b"salt5678salt5678", &[], &[], &p)
        );
    }

    #[test]
    fn test_long_tag() {
        let result = derive_key_vec(
            64,
            b"password",
            b"saltsaltsaltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
            },
        );
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn test_phc_roundtrip() {
        let password = b"password";
        let salt = b"somesalt";
        let params = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        let tag = derive_key_vec(24, password, salt, &[], &[], &params);
        let encoded = encode_phc(&params, salt, &tag);
        let (dp, ds, dt) = decode_phc(&encoded).unwrap();
        assert_eq!(dp.memory, params.memory);
        assert_eq!(dp.iterations, params.iterations);
        assert_eq!(dp.parallelism, params.parallelism);
        assert_eq!(ds, salt);
        assert_eq!(dt, tag);
    }

    // ================================================================
    // RFC 9106 intermediate block verification
    // Verifies Block 0000 and Block 0031 after each pass for all 3 types
    // Parameters: pwd=0x01*32, salt=0x02*16, secret=0x03*8, ad=0x04*12
    //             t=3, m=32, p=4, tag=32
    // ================================================================

    fn argon2_core_with_passes(
        argon_type: u32,
        password: &[u8],
        salt: &[u8],
        secret: &[u8],
        ad: &[u8],
        params: &Params,
    ) -> Vec<Vec<Block>> {
        let p = params.parallelism;
        let t = params.iterations;
        let m = params.memory;
        let tag_length = 32u32;

        let h0 = compute_h0(argon_type, password, salt, secret, ad, p, tag_length, m, t);
        let m_prime = 4 * p * (m / (4 * p));
        let q = m_prime / p;

        let mem = Memory::uninit(m_prime as usize);

        let mut input = [0u8; 72];
        input[..64].copy_from_slice(&h0);
        let mut block_bytes = [0u8; BLOCK_SIZE];
        for i in 0..p {
            input[68..72].copy_from_slice(&i.to_le_bytes());

            input[64..68].copy_from_slice(&0u32.to_le_bytes());
            variable_length_hash_into(&input, &mut block_bytes);
            mem.write((i * q) as usize, Block::from_bytes(&block_bytes));

            input[64..68].copy_from_slice(&1u32.to_le_bytes());
            variable_length_hash_into(&input, &mut block_bytes);
            mem.write((i * q + 1) as usize, Block::from_bytes(&block_bytes));
        }

        let mut pass_snapshots = Vec::new();
        for pass in 0..t {
            for slice in 0..SYNC_POINTS {
                for lane in 0..p {
                    fill_segment(Backend::Scalar, &mem, argon_type, pass, lane, slice, p, q, t, m_prime);
                }
            }
            pass_snapshots.push((0..mem.len()).map(|i| mem.get(i).clone()).collect());
        }

        pass_snapshots
    }

    fn block0_word(block: &Block, idx: usize) -> String {
        format!("{:016x}", block.v[idx])
    }

    fn block_last_word(block: &Block, idx: usize) -> String {
        format!("{:016x}", block.v[idx])
    }

    #[test]
    fn test_rfc9106_argon2d_intermediate_blocks() {
        let pwd = vec![0x01u8; 32];
        let salt = vec![0x02u8; 16];
        let secret = vec![0x03u8; 8];
        let ad = vec![0x04u8; 12];
        let params = Params {
            iterations: 3,
            memory: 32,
            parallelism: 4,
        };
        let passes = argon2_core_with_passes(ARGON2D, &pwd, &salt, &secret, &ad, &params);

        let p = params.parallelism;
        let q = (4 * p * (params.memory / (4 * p))) / p;
        let m_prime = p * q;

        assert_eq!(block0_word(&passes[0][0], 0), "db2fea6b2c6f5c8a");
        assert_eq!(block_last_word(&passes[0][(m_prime - 1) as usize], 127), "6a6c49d2cb75d5b6");

        assert_eq!(block0_word(&passes[1][0], 0), "d3801200410f8c0d");
        assert_eq!(block_last_word(&passes[1][(m_prime - 1) as usize], 127), "2dbfff23f31b5883");

        assert_eq!(block0_word(&passes[2][0], 0), "5f047b575c5ff4d2");
        assert_eq!(block_last_word(&passes[2][(m_prime - 1) as usize], 127), "c341b3ca45c10da5");
    }

    #[test]
    fn test_rfc9106_argon2i_intermediate_blocks() {
        let pwd = vec![0x01u8; 32];
        let salt = vec![0x02u8; 16];
        let secret = vec![0x03u8; 8];
        let ad = vec![0x04u8; 12];
        let params = Params {
            iterations: 3,
            memory: 32,
            parallelism: 4,
        };
        let passes = argon2_core_with_passes(ARGON2I, &pwd, &salt, &secret, &ad, &params);

        let p = params.parallelism;
        let q = (4 * p * (params.memory / (4 * p))) / p;
        let m_prime = p * q;

        assert_eq!(block0_word(&passes[0][0], 0), "f8f9e84545db08f6");
        assert_eq!(block_last_word(&passes[0][(m_prime - 1) as usize], 127), "c570f2ab2a86cf00");

        assert_eq!(block0_word(&passes[1][0], 0), "b2e4ddfcf76dc85a");
        assert_eq!(block_last_word(&passes[1][(m_prime - 1) as usize], 127), "421b3c6e9555b79d");

        assert_eq!(block0_word(&passes[2][0], 0), "af2a8bd8482c2f11");
        assert_eq!(block_last_word(&passes[2][(m_prime - 1) as usize], 127), "71e436f035f30ed0");
    }

    // ================================================================
    // RFC 9106 H_0 pre-hashing digest tests for all types
    // ================================================================

    #[test]
    fn test_h0_argon2d() {
        let pwd = vec![0x01u8; 32];
        let salt = vec![0x02u8; 16];
        let secret = vec![0x03u8; 8];
        let ad = vec![0x04u8; 12];
        let h0 = compute_h0(ARGON2D, &pwd, &salt, &secret, &ad, 4, 32, 32, 3);
        let expected = "b8819791a0359660bb7709c85fa48f04d5d82c05c5f215ccdb885491717cf757082c28b951be381410b5fc2eb7274033b9fdc7ae672bcaac5d179097a4af3109";
        assert_eq!(hex::encode(h0), expected);
    }

    #[test]
    fn test_h0_argon2i() {
        let pwd = vec![0x01u8; 32];
        let salt = vec![0x02u8; 16];
        let secret = vec![0x03u8; 8];
        let ad = vec![0x04u8; 12];
        let h0 = compute_h0(ARGON2I, &pwd, &salt, &secret, &ad, 4, 32, 32, 3);
        let expected = "c46065815276a0b3e731731c902f1fd80cf776907fbb7b6a5ca72e7b56011feeca446c86dd75b9469a5e6879dec4b72d0863fb939b982e5f397cc7d164fddaa9";
        assert_eq!(hex::encode(h0), expected);
    }

    // ================================================================
    // Additional test vectors from various sources
    // ================================================================

    #[test]
    fn test_argon2id_empty_secret_and_ad() {
        let params = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        let result = derive_key_vec(32, b"password", b"saltsaltsaltsalt", &[], &[], &params);
        assert_eq!(result.len(), 32);
        let result2 = derive_key_vec(32, b"password", b"saltsaltsaltsalt", &[], &[], &params);
        assert_eq!(result, result2);
    }

    #[test]
    fn test_argon2id_with_secret() {
        let p = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        let without_secret = derive_key_vec(32, b"password", b"saltsaltsaltsalt", &[], &[], &p);
        let with_secret = derive_key_vec(32, b"password", b"saltsaltsaltsalt", b"secret", &[], &p);
        assert_ne!(without_secret, with_secret);
    }

    #[test]
    fn test_argon2id_with_ad() {
        let p = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        let without_ad = derive_key_vec(32, b"password", b"saltsaltsaltsalt", &[], &[], &p);
        let with_ad = derive_key_vec(32, b"password", b"saltsaltsaltsalt", &[], b"associated data", &p);
        assert_ne!(without_ad, with_ad);
    }

    #[test]
    fn test_argon2id_tag_length_4() {
        let result = derive_key_vec(
            4,
            b"password",
            b"saltsaltsaltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
            },
        );
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_argon2id_tag_length_128() {
        let result = derive_key_vec(
            128,
            b"password",
            b"saltsaltsaltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
            },
        );
        assert_eq!(result.len(), 128);
    }

    #[test]
    fn test_argon2id_tag_length_256() {
        let result = derive_key_vec(
            256,
            b"password",
            b"saltsaltsaltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
            },
        );
        assert_eq!(result.len(), 256);
    }

    #[test]
    fn test_argon2id_long_tag_consistency() {
        let p = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        let r1 = derive_key_vec(100, b"password", b"saltsaltsaltsalt", &[], &[], &p);
        let r2 = derive_key_vec(100, b"password", b"saltsaltsaltsalt", &[], &[], &p);
        assert_eq!(r1, r2);
        assert_eq!(r1.len(), 100);
    }

    #[test]
    fn test_argon2i_long_tag_consistency() {
        let params = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        let mut r1 = [0u8; 100];
        let mut r2 = [0u8; 100];
        argon2_core(ARGON2I, b"password", b"saltsaltsaltsalt", &[], &[], &params, &mut r1).unwrap();
        argon2_core(ARGON2I, b"password", b"saltsaltsaltsalt", &[], &[], &params, &mut r2).unwrap();
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_argon2d_long_tag_consistency() {
        let params = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        let mut r1 = [0u8; 100];
        let mut r2 = [0u8; 100];
        argon2_core(ARGON2D, b"password", b"saltsaltsaltsalt", &[], &[], &params, &mut r1).unwrap();
        argon2_core(ARGON2D, b"password", b"saltsaltsaltsalt", &[], &[], &params, &mut r2).unwrap();
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_argon2id_single_pass() {
        let result = derive_key_vec(
            32,
            b"password",
            b"saltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 32,
                parallelism: 1,
            },
        );
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_argon2id_high_parallelism() {
        let result = derive_key_vec(
            32,
            b"password",
            b"saltsaltsaltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 8,
            },
        );
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_argon2d_rfc_h0() {
        let pwd = vec![0x01u8; 32];
        let salt = vec![0x02u8; 16];
        let secret = vec![0x03u8; 8];
        let ad = vec![0x04u8; 12];
        let h0 = compute_h0(ARGON2D, &pwd, &salt, &secret, &ad, 4, 32, 32, 3);
        assert_eq!(h0[0], 0xb8);
        assert_eq!(h0[1], 0x81);
        assert_eq!(h0[63], 0x09);
    }

    #[test]
    fn test_variable_length_hash_exact_64() {
        let input = b"test";
        let result = variable_length_hash_vec(input, 64);
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn test_variable_length_hash_65_bytes() {
        let input = b"test";
        let result = variable_length_hash_vec(input, 65);
        assert_eq!(result.len(), 65);
        let result2 = variable_length_hash_vec(input, 65);
        assert_eq!(result, result2);
    }

    #[test]
    fn test_variable_length_hash_deterministic() {
        for len in [4, 16, 32, 48, 64, 65, 96, 128, 256, 512, 1024] {
            let r1 = variable_length_hash_vec(b"determinism test", len);
            let r2 = variable_length_hash_vec(b"determinism test", len);
            assert_eq!(r1, r2, "variable_length_hash not deterministic for len={}", len);
            assert_eq!(r1.len(), len);
        }
    }

    #[test]
    fn test_compress_deterministic() {
        let a = Block::from_bytes(&[0xAA; BLOCK_SIZE]);
        let b = Block::from_bytes(&[0xBB; BLOCK_SIZE]);
        let c1 = compress(&a, &b);
        let c2 = compress(&a, &b);
        assert_eq!(c1.v, c2.v);
    }

    #[test]
    fn test_compress_xor_symmetry() {
        let a = Block::from_bytes(&[0x11; BLOCK_SIZE]);
        let b = Block::from_bytes(&[0x22; BLOCK_SIZE]);
        let c_ab = compress(&a, &b);
        let c_ba = compress(&b, &a);
        assert_eq!(c_ab.v, c_ba.v, "G(X,Y) should equal G(Y,X) since R = X XOR Y is symmetric");
    }

    #[test]
    fn test_block_from_bytes_roundtrip() {
        let original = [0x42u8; BLOCK_SIZE];
        let block = Block::from_bytes(&original);
        let recovered = block.to_bytes();
        assert_eq!(original, recovered);
    }

    #[test]
    fn test_argon2id_different_iterations() {
        let p1 = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        let p2 = Params {
            iterations: 2,
            memory: 64,
            parallelism: 1,
        };
        let r1 = derive_key_vec(32, b"password", b"saltsaltsaltsalt", &[], &[], &p1);
        let r2 = derive_key_vec(32, b"password", b"saltsaltsaltsalt", &[], &[], &p2);
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_argon2id_different_memory() {
        let p1 = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        let p2 = Params {
            iterations: 1,
            memory: 128,
            parallelism: 1,
        };
        let r1 = derive_key_vec(32, b"password", b"saltsaltsaltsalt", &[], &[], &p1);
        let r2 = derive_key_vec(32, b"password", b"saltsaltsaltsalt", &[], &[], &p2);
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_argon2id_different_parallelisms() {
        let p1 = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        let p2 = Params {
            iterations: 1,
            memory: 64,
            parallelism: 2,
        };
        let r1 = derive_key_vec(32, b"password", b"saltsaltsaltsalt", &[], &[], &p1);
        let r2 = derive_key_vec(32, b"password", b"saltsaltsaltsalt", &[], &[], &p2);
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_argon2i_rfc9106_tag() {
        let pwd = vec![0x01u8; 32];
        let salt = vec![0x02u8; 16];
        let secret = vec![0x03u8; 8];
        let ad = vec![0x04u8; 12];
        let expected = hex::decode("c814d9d1dc7f37aa13f0d77f2494bda1c8de6b016dd388d29952a4c4672b6ce8").unwrap();
        let result = derive_key_typed(ARGON2I, &pwd, &salt, &secret, &ad, 3, 32, 4, 32);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_argon2d_rfc9106_tag() {
        let pwd = vec![0x01u8; 32];
        let salt = vec![0x02u8; 16];
        let secret = vec![0x03u8; 8];
        let ad = vec![0x04u8; 12];
        let expected = hex::decode("512b391b6f1162975371d30919734294f868e3be3984f3c1a13a4db9fabe4acb").unwrap();
        let result = derive_key_typed(ARGON2D, &pwd, &salt, &secret, &ad, 3, 32, 4, 32);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_phc_verify_known() {
        let password = b"password";
        let salt = b"randomsalt123456";
        let params = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
        };
        let encoded = hash_password(password, salt, &params).unwrap();
        assert!(verify_password(password, &encoded, None).is_ok());
        assert_eq!(verify_password(b"wrong", &encoded, None), Err(Argon2Error::VerifyMismatch));
    }

    #[test]
    fn test_decode_phc_roundtrip_all_types() {
        for tag_len in [4, 16, 32, 64] {
            let params = Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
            };
            let salt = b"testsalt12345678";
            let tag = vec![0xAB; tag_len as usize];
            let encoded = encode_phc(&params, salt, &tag);
            let (dp, ds, dt) = decode_phc(&encoded).unwrap();
            assert_eq!(dp.memory, 64);
            assert_eq!(dp.iterations, 1);
            assert_eq!(dp.parallelism, 1);
            assert_eq!(ds, salt);
            assert_eq!(dt, tag);
        }
    }

    #[test]
    fn test_index_alpha_pass0_slice0() {
        let result = index_alpha(0, 0, 4, 2, 2, 8, true, 0xFFFFFFFF);
        assert!(result < 8);
    }

    #[test]
    fn test_index_alpha_reference_area_size_zero() {
        let result = index_alpha(0, 0, 4, 2, 0, 8, true, 0xFFFFFFFF);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_permutation_p_changes_values() {
        let mut v = [0u64; 16];
        v[0] = 1;
        v[1] = 2;
        v[2] = 3;
        v[3] = 4;
        permutation_p(&mut v);
        assert_ne!(v[0], 1);
        assert_ne!(v[1], 2);
        assert_ne!(v[2], 3);
        assert_ne!(v[3], 4);
    }

    #[test]
    fn test_permutation_p_deterministic() {
        let mut v1 = [0u64; 16];
        for (i, word) in v1.iter_mut().enumerate() {
            *word = i as u64;
        }
        let mut v2 = v1;
        permutation_p(&mut v1);
        permutation_p(&mut v2);
        assert_eq!(v1, v2);
    }
}
