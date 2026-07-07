//! Argon2id (RFC 9106) password hashing function.
//!
//! Argon2id is a memory-hard password hashing function that provides resistance
//! against both side-channel attacks and GPU/ASIC brute-force attacks.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{string::String, vec, vec::Vec};

use crate::{Hasher, blake2::Blake2b};

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

/// Argon2id parameters (RFC 9106).
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
///     tag_length: 32,
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
    /// Output tag length in bytes. Must be >= 4.
    pub tag_length: u32,
}

impl Default for Params {
    /// Default parameters: t=3, m=64 MiB, p=4, tag=32 bytes (SECOND RECOMMENDED option).
    fn default() -> Self {
        Params {
            iterations: 3,
            memory: 65536,
            parallelism: 4,
            tag_length: 32,
        }
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

/// A 1024-byte block used in Argon2's memory matrix.
#[derive(Clone)]
struct Block {
    v: [u64; 128],
}

impl Block {
    fn zero() -> Self {
        Block {
            v: [0u64; 128],
        }
    }

    fn xor_with(&mut self, other: &Block) {
        for i in 0..128 {
            self.v[i] ^= other.v[i];
        }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut block = Block::zero();
        for i in 0..128 {
            let offset = i * 8;
            block.v[i] = u64::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
                bytes[offset + 4],
                bytes[offset + 5],
                bytes[offset + 6],
                bytes[offset + 7],
            ]);
        }
        block
    }

    fn to_bytes(&self) -> [u8; BLOCK_SIZE] {
        let mut out = [0u8; BLOCK_SIZE];
        for i in 0..128 {
            let bytes = self.v[i].to_le_bytes();
            let offset = i * 8;
            out[offset..offset + 8].copy_from_slice(&bytes);
        }
        out
    }
}

// ============================================================
// Core Argon2 algorithm
// ============================================================

/// Derive a key using Argon2id (RFC 9106).
///
/// This is the main entry point for Argon2id key derivation.
///
/// # Arguments
/// * `password` - The password to hash
/// * `salt` - Salt (recommended 16 bytes)
/// * `secret` - Optional secret key (can be empty)
/// * `ad` - Optional associated data (can be empty)
/// * `params` - Argon2id parameters
///
/// # Returns
/// The derived key as a Vec<u8> of length `params.tag_length`.
///
/// # Example
///
/// ```ignore
/// use crypto::argon2::{derive_key, Params};
///
/// let tag = derive_key(
///     b"correct horse battery staple",
///     b"randomsalt123456",
///     &[],  // no secret
///     &[],  // no associated data
///     &Params { iterations: 3, memory: 65536, parallelism: 4, tag_length: 32 },
/// ).unwrap();
/// assert_eq!(tag.len(), 32);
/// ```
#[cfg(feature = "alloc")]
pub fn derive_key(
    password: &[u8],
    salt: &[u8],
    secret: &[u8],
    ad: &[u8],
    params: &Params,
) -> Result<Vec<u8>, Argon2Error> {
    argon2_core(ARGON2ID, password, salt, secret, ad, params)
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
) -> Result<Vec<u8>, Argon2Error> {
    // Validate parameters
    if params.iterations < 1 {
        return Err(Argon2Error::InvalidParams("iterations must be >= 1"));
    }
    if params.parallelism < 1 {
        return Err(Argon2Error::InvalidParams("parallelism must be >= 1"));
    }
    if params.tag_length < 4 {
        return Err(Argon2Error::InvalidParams("tag_length must be >= 4"));
    }
    if params.memory < 8 * params.parallelism {
        return Err(Argon2Error::InvalidParams("memory must be >= 8*parallelism"));
    }

    let p = params.parallelism;
    let t = params.iterations;
    let m = params.memory;
    let tag_length = params.tag_length;

    // Step 1: Compute H_0
    let h0 = compute_h0(argon_type, password, salt, secret, ad, p, tag_length, m, t);

    // Step 2: Determine actual memory size m' (rounded down to multiple of 4*p)
    let m_prime = 4 * p * (m / (4 * p));
    let q = m_prime / p; // columns per lane

    // Allocate memory as m' blocks
    let mut memory: Vec<Block> = vec![Block::zero(); m_prime as usize];

    // Step 3 & 4: Compute B[i][0] and B[i][1] for all lanes
    for i in 0..p {
        // B[i][0] = H'^(1024)(H_0 || LE32(0) || LE32(i))
        let mut input = Vec::with_capacity(72);
        input.extend_from_slice(&h0);
        input.extend_from_slice(&0u32.to_le_bytes());
        input.extend_from_slice(&i.to_le_bytes());
        let block_bytes = variable_length_hash(&input, BLOCK_SIZE as u32);
        memory[(i * q) as usize] = Block::from_bytes(&block_bytes);

        // B[i][1] = H'^(1024)(H_0 || LE32(1) || LE32(i))
        let mut input = Vec::with_capacity(72);
        input.extend_from_slice(&h0);
        input.extend_from_slice(&1u32.to_le_bytes());
        input.extend_from_slice(&i.to_le_bytes());
        let block_bytes = variable_length_hash(&input, BLOCK_SIZE as u32);
        memory[(i * q + 1) as usize] = Block::from_bytes(&block_bytes);
    }

    // Steps 5-6: Fill memory
    for pass in 0..t {
        for slice in 0..SYNC_POINTS {
            for lane in 0..p {
                fill_segment(&mut memory, argon_type, pass, lane, slice, p, q, t, m_prime);
            }
        }
    }

    // Step 7: Compute final block C = XOR of last column
    let mut final_block = memory[(q - 1) as usize].clone();
    for i in 1..p {
        let idx = (i * q + q - 1) as usize;
        final_block.xor_with(&memory[idx]);
    }

    // Step 8: Output tag = H'^T(C)
    let final_bytes = final_block.to_bytes();
    let tag = variable_length_hash(&final_bytes, tag_length);

    Ok(tag)
}

/// Fill a segment of the memory matrix.
#[cfg(feature = "alloc")]
fn fill_segment(
    memory: &mut [Block],
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

    // For Argon2i and Argon2id (first half of first pass), precompute pseudo-random values
    let mut pseudo_rands: Vec<u64> = Vec::new();
    let need_pseudo_rands = argon_type == ARGON2I || (argon_type == ARGON2ID && pass == 0 && slice < 2);

    if need_pseudo_rands {
        pseudo_rands = generate_addresses(pass, lane, slice, lanes, t, argon_type, m_prime, segment_length);
    }

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
            let val = pseudo_rands[s as usize];
            ((val & 0xFFFFFFFF) as u32, (val >> 32) as u32)
        } else {
            // Argon2d mode: use first 64 bits of previous block
            let prev = &memory[prev_index];
            (prev.v[0] as u32, (prev.v[0] >> 32) as u32)
        };

        // Map J1, J2 to reference block index
        let ref_lane = if pass == 0 && slice == 0 { lane } else { j2 % lanes };

        let ref_index = index_alpha(pass, slice, lanes, segment_length, s, q, ref_lane == lane, j1);
        let ref_block_index = (ref_lane * q + ref_index) as usize;

        // Compute new block
        let new_block = if pass == 0 {
            compress(&memory[prev_index], &memory[ref_block_index])
        } else {
            let mut new = compress(&memory[prev_index], &memory[ref_block_index]);
            new.xor_with(&memory[cur_index]);
            new
        };

        memory[cur_index] = new_block;
    }
}

/// Generate pseudo-random addresses for Argon2i/Argon2id data-independent addressing.
#[cfg(feature = "alloc")]
fn generate_addresses(
    pass: u32,
    lane: u32,
    slice: u32,
    _lanes: u32,
    t: u32,
    argon_type: u32,
    m_prime: u32,
    segment_length: u32,
) -> Vec<u64> {
    let mut pseudo_rands = Vec::with_capacity(segment_length as usize);

    // Build input block
    let mut input = Block::zero();
    input.v[0] = pass as u64;
    input.v[1] = lane as u64;
    input.v[2] = slice as u64;
    input.v[3] = m_prime as u64;
    input.v[4] = t as u64;
    input.v[5] = argon_type as u64;

    let zero_block = Block::zero();
    // Generate addresses in groups of 128 (each block gives 128 u64 values)
    let mut counter = 1u64;
    while pseudo_rands.len() < segment_length as usize {
        input.v[6] = counter;
        let tmp = compress(&zero_block, &input);
        let addr_block = compress(&zero_block, &tmp);

        for i in 0..128 {
            if pseudo_rands.len() >= segment_length as usize {
                break;
            }
            pseudo_rands.push(addr_block.v[i]);
        }
        counter += 1;
    }

    pseudo_rands
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
/// Uses Blake2b to produce output of arbitrary length.
#[cfg(feature = "alloc")]
fn variable_length_hash(input: &[u8], tag_length: u32) -> Vec<u8> {
    if tag_length <= 64 {
        // Short output: H'^T(A) = H^T(LE32(T)||A)
        let mut blake = Blake2b::new_keyed(&[], tag_length as usize);
        blake.update(&tag_length.to_le_bytes());
        blake.update(input);
        let hash = blake.sum();
        hash.as_ref()[..tag_length as usize].to_vec()
    } else {
        // Long output
        // r = ceil(T/32) - 2
        let r = ((tag_length + 31) / 32) - 2;

        let mut result = Vec::with_capacity(tag_length as usize);

        // V_1 = H^(64)(LE32(T)||A)
        let mut blake = Blake2b::new_keyed(&[], 64);
        blake.update(&tag_length.to_le_bytes());
        blake.update(input);
        let hash = blake.sum();
        let mut v_prev = hash.as_ref()[..64].to_vec();

        // W_1 = first 32 bytes of V_1
        result.extend_from_slice(&v_prev[..32]);

        // V_2 through V_r
        for _ in 2..=r {
            let mut blake = Blake2b::new_keyed(&[], 64);
            blake.update(&v_prev);
            let hash = blake.sum();
            v_prev = hash.as_ref()[..64].to_vec();
            result.extend_from_slice(&v_prev[..32]);
        }

        // V_{r+1} = H^(T-32*r)(V_r)
        let remaining = tag_length - 32 * r;
        let mut blake = Blake2b::new_keyed(&[], remaining as usize);
        blake.update(&v_prev);
        let hash = blake.sum();
        result.extend_from_slice(&hash.as_ref()[..remaining as usize]);

        result
    }
}

// ============================================================
// Compression function G and Permutation P
// ============================================================

/// Compression function G(X, Y) -> Z XOR R
///
/// Operates on two 1024-byte blocks.
fn compress(x: &Block, y: &Block) -> Block {
    // R = X XOR Y
    let mut r = Block::zero();
    for i in 0..128 {
        r.v[i] = x.v[i] ^ y.v[i];
    }

    let mut q = r.clone();

    // Apply P to each row of 8x8 matrix of 16-byte registers
    // Each row has 8 registers of 16 bytes = 8*2 u64 = 16 u64 values
    for row in 0..8 {
        let base = row * 16;
        permutation_p(&mut q.v[base..base + 16]);
    }

    // Apply P to each column
    // Columns: position i in each row
    for col in 0..8 {
        let mut buf = [0u64; 16];
        for row in 0..8 {
            let src = row * 16 + col * 2;
            buf[row * 2] = q.v[src];
            buf[row * 2 + 1] = q.v[src + 1];
        }
        permutation_p(&mut buf);
        for row in 0..8 {
            let dst = row * 16 + col * 2;
            q.v[dst] = buf[row * 2];
            q.v[dst + 1] = buf[row * 2 + 1];
        }
    }

    // Z XOR R
    for i in 0..128 {
        q.v[i] ^= r.v[i];
    }

    q
}

/// Permutation P based on the round function of BLAKE2b.
///
/// Operates on 128 bytes (16 u64 values) viewed as a 4x4 matrix.
fn permutation_p(v: &mut [u64]) {
    // Column-wise
    gb(v, 0, 4, 8, 12);
    gb(v, 1, 5, 9, 13);
    gb(v, 2, 6, 10, 14);
    gb(v, 3, 7, 11, 15);

    // Diagonal-wise
    gb(v, 0, 5, 10, 15);
    gb(v, 1, 6, 11, 12);
    gb(v, 2, 7, 8, 13);
    gb(v, 3, 4, 9, 14);
}

/// The GB mixing function for Argon2.
///
/// Unlike BLAKE2b's G, this uses multiplication for additional hardness.
#[inline(always)]
fn gb(v: &mut [u64], a: usize, b: usize, c: usize, d: usize) {
    v[a] = v[a]
        .wrapping_add(v[b])
        .wrapping_add(2u64.wrapping_mul((v[a] as u32 as u64).wrapping_mul(v[b] as u32 as u64)));
    v[d] = (v[d] ^ v[a]).rotate_right(32);
    v[c] = v[c]
        .wrapping_add(v[d])
        .wrapping_add(2u64.wrapping_mul((v[c] as u32 as u64).wrapping_mul(v[d] as u32 as u64)));
    v[b] = (v[b] ^ v[c]).rotate_right(24);

    v[a] = v[a]
        .wrapping_add(v[b])
        .wrapping_add(2u64.wrapping_mul((v[a] as u32 as u64).wrapping_mul(v[b] as u32 as u64)));
    v[d] = (v[d] ^ v[a]).rotate_right(16);
    v[c] = v[c]
        .wrapping_add(v[d])
        .wrapping_add(2u64.wrapping_mul((v[c] as u32 as u64).wrapping_mul(v[d] as u32 as u64)));
    v[b] = (v[b] ^ v[c]).rotate_right(63);
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
        tag_length: tag.len() as u32,
    };

    Ok((params, salt, tag))
}

/// Hash a password and return the PHC-encoded string.
///
/// # Example
///
/// ```ignore
/// use crypto::argon2::{hash_password, verify_password, Params};
///
/// let encoded = hash_password(
///     b"correct horse battery staple",
///     b"randomsalt123456",
///     &Params { iterations: 3, memory: 65536, parallelism: 4, tag_length: 32 },
/// ).unwrap();
///
/// assert!(verify_password(b"correct horse battery staple", &encoded).is_ok());
/// assert!(verify_password(b"wrong password", &encoded).is_err());
/// ```
#[cfg(feature = "alloc")]
pub fn hash_password(password: &[u8], salt: &[u8], params: &Params) -> Result<String, Argon2Error> {
    let tag = derive_key(password, salt, &[], &[], params)?;
    Ok(encode_phc(params, salt, &tag))
}

/// Verify a password against a PHC-encoded hash string.
///
/// See [`hash_password`] for an example.
#[cfg(feature = "alloc")]
pub fn verify_password(password: &[u8], encoded: &str) -> Result<(), Argon2Error> {
    let (params, salt, expected_tag) = decode_phc(encoded)?;
    let computed_tag = derive_key(password, &salt, &[], &[], &params)?;
    if constant_time_eq::constant_time_eq(&computed_tag, &expected_tag) {
        Ok(())
    } else {
        Err(Argon2Error::VerifyMismatch)
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
            tag_length: tag_length,
        };
        argon2_core(argon_type, password, salt, secret, ad, &params).unwrap()
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
            tag_length: 32,
        };
        let salt = b"somesalt12345678";
        let tag = vec![0xAB; 32];
        let encoded = encode_phc(&params, salt, &tag);
        assert!(encoded.starts_with("$argon2id$v=19$m=65536,t=3,p=4$"));
        let (dp, ds, dt) = decode_phc(&encoded).unwrap();
        assert_eq!(dp.iterations, 3);
        assert_eq!(dp.memory, 65536);
        assert_eq!(dp.parallelism, 4);
        assert_eq!(dp.tag_length, 32);
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
            tag_length: 32,
        };
        let encoded = hash_password(password, salt, &params).unwrap();
        assert!(verify_password(password, &encoded).is_ok());
        assert_eq!(verify_password(b"wrong password", &encoded), Err(Argon2Error::VerifyMismatch));
    }

    #[test]
    fn test_decode_phc_invalid() {
        assert!(decode_phc("").is_err());
        assert!(decode_phc("$argon2i$v=19$m=4096,t=3,p=1$salt$hash").is_err());
        assert!(decode_phc("$argon2id$v=16$m=4096,t=3,p=1$salt$hash").is_err());
        assert!(decode_phc("not a phc string").is_err());
    }

    #[test]
    fn test_invalid_params() {
        assert!(
            derive_key(
                b"password",
                b"salt",
                &[],
                &[],
                &Params {
                    iterations: 0,
                    memory: 64,
                    parallelism: 1,
                    tag_length: 32
                }
            )
            .is_err()
        );
        assert!(
            derive_key(
                b"password",
                b"salt",
                &[],
                &[],
                &Params {
                    iterations: 1,
                    memory: 4,
                    parallelism: 1,
                    tag_length: 32
                }
            )
            .is_err()
        );
        assert!(
            derive_key(
                b"password",
                b"salt",
                &[],
                &[],
                &Params {
                    iterations: 1,
                    memory: 64,
                    parallelism: 1,
                    tag_length: 3
                }
            )
            .is_err()
        );
    }

    #[test]
    fn test_variable_length_hash_short() {
        let input = b"test input";
        let r32 = variable_length_hash(input, 32);
        assert_eq!(r32.len(), 32);
        assert_eq!(variable_length_hash(input, 32), r32);
        let r48 = variable_length_hash(input, 48);
        assert_eq!(r48.len(), 48);
        assert_ne!(&r32[..], &r48[..32]);
    }

    #[test]
    fn test_variable_length_hash_long() {
        assert_eq!(variable_length_hash(b"test input for long hash", 128).len(), 128);
        assert_eq!(variable_length_hash(b"test input for long hash", 1024).len(), 1024);
    }

    #[test]
    fn test_argon2id_min_memory() {
        let result = derive_key(
            b"password",
            b"saltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 8,
                parallelism: 1,
                tag_length: 32,
            },
        )
        .unwrap();
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_argon2id_multiple_lanes() {
        assert_eq!(
            derive_key(
                b"password",
                b"saltsaltsaltsalt",
                &[],
                &[],
                &Params {
                    iterations: 1,
                    memory: 64,
                    parallelism: 4,
                    tag_length: 32
                }
            )
            .unwrap()
            .len(),
            32
        );
    }

    #[test]
    fn test_different_passwords() {
        let p = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
            tag_length: 32,
        };
        assert_ne!(
            derive_key(b"password1", b"saltsaltsaltsalt", &[], &[], &p).unwrap(),
            derive_key(b"password2", b"saltsaltsaltsalt", &[], &[], &p).unwrap()
        );
    }

    #[test]
    fn test_different_salts() {
        let p = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
            tag_length: 32,
        };
        assert_ne!(
            derive_key(b"password", b"salt1234salt1234", &[], &[], &p).unwrap(),
            derive_key(b"password", b"salt5678salt5678", &[], &[], &p).unwrap()
        );
    }

    #[test]
    fn test_long_tag() {
        let result = derive_key(
            b"password",
            b"saltsaltsaltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
                tag_length: 64,
            },
        )
        .unwrap();
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
            tag_length: 24,
        };
        let tag = derive_key(password, salt, &[], &[], &params).unwrap();
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
        let tag_length = params.tag_length;

        let h0 = compute_h0(argon_type, password, salt, secret, ad, p, tag_length, m, t);
        let m_prime = 4 * p * (m / (4 * p));
        let q = m_prime / p;

        let mut memory: Vec<Block> = vec![Block::zero(); m_prime as usize];

        for i in 0..p {
            let mut input = Vec::with_capacity(72);
            input.extend_from_slice(&h0);
            input.extend_from_slice(&0u32.to_le_bytes());
            input.extend_from_slice(&i.to_le_bytes());
            let block_bytes = variable_length_hash(&input, BLOCK_SIZE as u32);
            memory[(i * q) as usize] = Block::from_bytes(&block_bytes);

            let mut input = Vec::with_capacity(72);
            input.extend_from_slice(&h0);
            input.extend_from_slice(&1u32.to_le_bytes());
            input.extend_from_slice(&i.to_le_bytes());
            let block_bytes = variable_length_hash(&input, BLOCK_SIZE as u32);
            memory[(i * q + 1) as usize] = Block::from_bytes(&block_bytes);
        }

        let mut pass_snapshots = Vec::new();

        for pass in 0..t {
            for slice in 0..SYNC_POINTS {
                for lane in 0..p {
                    fill_segment(&mut memory, argon_type, pass, lane, slice, p, q, t, m_prime);
                }
            }
            pass_snapshots.push(memory.clone());
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
            tag_length: 32,
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
            tag_length: 32,
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
        let result = derive_key(
            b"password",
            b"saltsaltsaltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
                tag_length: 32,
            },
        )
        .unwrap();
        assert_eq!(result.len(), 32);
        let result2 = derive_key(
            b"password",
            b"saltsaltsaltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
                tag_length: 32,
            },
        )
        .unwrap();
        assert_eq!(result, result2);
    }

    #[test]
    fn test_argon2id_with_secret() {
        let p = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
            tag_length: 32,
        };
        let without_secret = derive_key(b"password", b"saltsaltsaltsalt", &[], &[], &p).unwrap();
        let with_secret = derive_key(b"password", b"saltsaltsaltsalt", b"secret", &[], &p).unwrap();
        assert_ne!(without_secret, with_secret);
    }

    #[test]
    fn test_argon2id_with_ad() {
        let p = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
            tag_length: 32,
        };
        let without_ad = derive_key(b"password", b"saltsaltsaltsalt", &[], &[], &p).unwrap();
        let with_ad = derive_key(b"password", b"saltsaltsaltsalt", &[], b"associated data", &p).unwrap();
        assert_ne!(without_ad, with_ad);
    }

    #[test]
    fn test_argon2id_tag_length_4() {
        let result = derive_key(
            b"password",
            b"saltsaltsaltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
                tag_length: 4,
            },
        )
        .unwrap();
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_argon2id_tag_length_128() {
        let result = derive_key(
            b"password",
            b"saltsaltsaltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
                tag_length: 128,
            },
        )
        .unwrap();
        assert_eq!(result.len(), 128);
    }

    #[test]
    fn test_argon2id_tag_length_256() {
        let result = derive_key(
            b"password",
            b"saltsaltsaltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
                tag_length: 256,
            },
        )
        .unwrap();
        assert_eq!(result.len(), 256);
    }

    #[test]
    fn test_argon2id_long_tag_consistency() {
        let p = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
            tag_length: 100,
        };
        let r1 = derive_key(b"password", b"saltsaltsaltsalt", &[], &[], &p).unwrap();
        let r2 = derive_key(b"password", b"saltsaltsaltsalt", &[], &[], &p).unwrap();
        assert_eq!(r1, r2);
        assert_eq!(r1.len(), 100);
    }

    #[test]
    fn test_argon2i_long_tag_consistency() {
        let params = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
            tag_length: 100,
        };
        let r1 = argon2_core(ARGON2I, b"password", b"saltsaltsaltsalt", &[], &[], &params).unwrap();
        let r2 = argon2_core(ARGON2I, b"password", b"saltsaltsaltsalt", &[], &[], &params).unwrap();
        assert_eq!(r1, r2);
        assert_eq!(r1.len(), 100);
    }

    #[test]
    fn test_argon2d_long_tag_consistency() {
        let params = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
            tag_length: 100,
        };
        let r1 = argon2_core(ARGON2D, b"password", b"saltsaltsaltsalt", &[], &[], &params).unwrap();
        let r2 = argon2_core(ARGON2D, b"password", b"saltsaltsaltsalt", &[], &[], &params).unwrap();
        assert_eq!(r1, r2);
        assert_eq!(r1.len(), 100);
    }

    #[test]
    fn test_argon2id_single_pass() {
        let result = derive_key(
            b"password",
            b"saltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 32,
                parallelism: 1,
                tag_length: 32,
            },
        )
        .unwrap();
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_argon2id_high_parallelism() {
        let result = derive_key(
            b"password",
            b"saltsaltsaltsalt",
            &[],
            &[],
            &Params {
                iterations: 1,
                memory: 64,
                parallelism: 8,
                tag_length: 32,
            },
        )
        .unwrap();
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
        let result = variable_length_hash(input, 64);
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn test_variable_length_hash_65_bytes() {
        let input = b"test";
        let result = variable_length_hash(input, 65);
        assert_eq!(result.len(), 65);
        let result2 = variable_length_hash(input, 65);
        assert_eq!(result, result2);
    }

    #[test]
    fn test_variable_length_hash_deterministic() {
        for len in [4, 16, 32, 48, 64, 65, 96, 128, 256, 512, 1024] {
            let r1 = variable_length_hash(b"determinism test", len);
            let r2 = variable_length_hash(b"determinism test", len);
            assert_eq!(r1, r2, "variable_length_hash not deterministic for len={}", len);
            assert_eq!(r1.len(), len as usize);
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
            tag_length: 32,
        };
        let p2 = Params {
            iterations: 2,
            memory: 64,
            parallelism: 1,
            tag_length: 32,
        };
        let r1 = derive_key(b"password", b"saltsaltsaltsalt", &[], &[], &p1).unwrap();
        let r2 = derive_key(b"password", b"saltsaltsaltsalt", &[], &[], &p2).unwrap();
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_argon2id_different_memory() {
        let p1 = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
            tag_length: 32,
        };
        let p2 = Params {
            iterations: 1,
            memory: 128,
            parallelism: 1,
            tag_length: 32,
        };
        let r1 = derive_key(b"password", b"saltsaltsaltsalt", &[], &[], &p1).unwrap();
        let r2 = derive_key(b"password", b"saltsaltsaltsalt", &[], &[], &p2).unwrap();
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_argon2id_different_parallelisms() {
        let p1 = Params {
            iterations: 1,
            memory: 64,
            parallelism: 1,
            tag_length: 32,
        };
        let p2 = Params {
            iterations: 1,
            memory: 64,
            parallelism: 2,
            tag_length: 32,
        };
        let r1 = derive_key(b"password", b"saltsaltsaltsalt", &[], &[], &p1).unwrap();
        let r2 = derive_key(b"password", b"saltsaltsaltsalt", &[], &[], &p2).unwrap();
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
            tag_length: 32,
        };
        let encoded = hash_password(password, salt, &params).unwrap();
        assert!(verify_password(password, &encoded).is_ok());
        assert_eq!(verify_password(b"wrong", &encoded), Err(Argon2Error::VerifyMismatch));
    }

    #[test]
    fn test_decode_phc_roundtrip_all_types() {
        for tag_len in [4, 16, 32, 64] {
            let params = Params {
                iterations: 1,
                memory: 64,
                parallelism: 1,
                tag_length: tag_len,
            };
            let salt = b"testsalt12345678";
            let tag = vec![0xAB; tag_len as usize];
            let encoded = encode_phc(&params, salt, &tag);
            let (dp, ds, dt) = decode_phc(&encoded).unwrap();
            assert_eq!(dp.memory, 64);
            assert_eq!(dp.iterations, 1);
            assert_eq!(dp.parallelism, 1);
            assert_eq!(dp.tag_length, tag_len);
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
    fn test_gb_known_values() {
        let mut v = [0u64; 16];
        v[0] = 1;
        v[1] = 2;
        v[2] = 3;
        v[3] = 4;
        gb(&mut v, 0, 1, 2, 3);
        assert_ne!(v[0], 1);
        assert_ne!(v[1], 2);
        assert_ne!(v[2], 3);
        assert_ne!(v[3], 4);
    }

    #[test]
    fn test_permutation_p_deterministic() {
        let mut v1: Vec<u64> = (0..16).collect();
        let mut v2: Vec<u64> = (0..16).collect();
        permutation_p(&mut v1);
        permutation_p(&mut v2);
        assert_eq!(v1, v2);
    }
}
