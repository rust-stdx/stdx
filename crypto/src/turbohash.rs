//! TurboHash, a custom tree hash built on a 16-round Keccak-p permutation.
//!
//! TurboHash follows the tree-hash layout of KangarooTwelve (RFC 9861) but
//! uses `Keccak-p[1600, n_r=16]` instead of the last twelve rounds, a 4096-byte
//! chunk size instead of 8192, and a context string that is prepended to the
//! message. It is a 256-bit construction: the sponge uses a 136-byte rate and
//! a 512-bit capacity.
//!
//! # Warning
//!
//! TurboHash is **not** a standardized construction. It is not compatible with
//! RFC 9861, its 16-round permutation is not one of the standard Keccak-p
//! instances, and it has not been analyzed or audited by a third party. Do not
//! use it as a replacement for a standardized hash function without an
//! independent review.
//!
//! # Construction
//!
//! The context string `C` is absorbed when the hasher is created, so the type
//! never borrows it. Message bytes are absorbed after it. Conceptually the
//! input of the tree is
//!
//! ```text
//! S = left_encode(|C|) || C || M
//! ```
//!
//! `left_encode` is the SP 800-185 length prefix, so the boundary between the
//! context and the message is unambiguous. In particular, `left_encode(0)`
//! is `[0x01, 0x00]`.
//!
//! If `S` fits in one 4096-byte chunk, it becomes the final node directly and
//! is hashed with the single-node domain byte. This avoids the extra inner
//! hash that ParallelHash performs for small messages. Otherwise every chunk,
//! including the first, is hashed independently into a 64-byte chaining value,
//! and the chaining values are absorbed into a small final node:
//!
//! ```text
//! final node = CV_0 || ... || CV_(n-1)
//!              || length_encode(n) || END || length_encode(L)
//! ```
//!
//! The first chunk is hashed serially as `CV_0` when tree mode begins. It is
//! not buffered, so single-node messages stay small and cheap.
//!
//! The requested output length `L` is bound into the hash with
//! `length_encode(L)`. Consequently a 32-byte output is not a prefix of a
//! 64-byte output. [`TurboHash::finalize_xof`] uses `L = 0` and returns a reader
//! whose successive outputs form one prefix-consistent stream.
//!
//! # Keyed mode
//!
//! [`TurboHash::new_keyed`] implements the HopMAC-shaped construction
//!
//! ```text
//! TurboHash(Key, M, C, L) =
//!     TurboHash(Key, KEYED_DOMAIN || TurboHash(M, C, 64), L)
//! ```
//!
//! The inner call compresses the message and context to 64 bytes; the outer
//! call authenticates that digest with the key. `KEYED_DOMAIN` is the ASCII
//! string `TurboHash-keyed`; it separates the outer keyed call from an unkeyed
//! call that happens to use the inner digest as its context. The key is a
//! 256-bit value owned by the hasher and zeroized on drop. Key material with a
//! different length should be derived into 32 bytes, for example with HKDF,
//! rather than truncated.
//!
//! # Allocation
//!
//! TurboHash does not allocate, including on the streaming and XOF paths, and
//! works without the `alloc` feature. Long inputs are hashed in parallel with
//! the batched Keccak backend; [`backend_name`] reports which one is used.
//!
//! # Example
//!
//! ```
//! use crypto::turbohash::TurboHash;
//!
//! let mut output = [0u8; 32];
//! TurboHash::hash(b"application", b"hello world", &mut output);
//!
//! let mut hasher = TurboHash::new(b"application");
//! hasher.update(b"hello ");
//! hasher.update(b"world");
//! hasher.finalize_into(&mut output);
//!
//! let key = [0x42; 32];
//! let mut tag = [0u8; 32];
//! TurboHash::mac(key, b"application", b"hello world", &mut tag);
//! ```

use core::cmp::min;

use crate::sha3::{
    keccak::KeccakSponge,
    keccak_batch::{Backend, Leaves, MAX_BATCH},
    left_encode, length_encode,
};

/// Size of a TurboHash chunk, in bytes.
pub const TURBOHASH_CHUNK_SIZE: usize = 4096;

/// Rate of the TurboHash sponge, in bytes (1088 bits).
const TURBOHASH_RATE: usize = 136;

/// Chaining-value size of TurboHash, in bytes (512 bits).
const TURBOHASH_CV_LEN: usize = 64;

/// Number of `Keccak-p[1600]` rounds used by TurboHash.
const TURBOHASH_ROUNDS: usize = 16;

/// Domain-separation byte of a single (non-tree) node.
const SINGLE_PAD: u8 = 0x07;
/// Domain-separation byte of an intermediate (leaf) node.
const INTERMEDIATE_PAD: u8 = 0x0b;
/// Domain-separation byte of the final node.
const FINAL_PAD: u8 = 0x06;

/// Domain prefix of the outer, keyed call.
const KEYED_DOMAIN: &[u8] = b"TurboHash-keyed";
/// Length of the outer keyed context: the domain prefix plus the inner digest.
const KEYED_CONTEXT_LEN: usize = KEYED_DOMAIN.len() + TURBOHASH_CV_LEN;

/// Sakura end marker of the final node.
const END: [u8; 2] = [0xff, 0xff];

/// Reader returned by [`TurboHash::finalize_xof`].
///
/// Produces the TurboHash output stream for `L = 0`; [`squeeze`](Self::squeeze)
/// may be called any number of times.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct TurboHashReader {
    inner: KeccakSponge<TURBOHASH_ROUNDS>,
}

impl TurboHashReader {
    /// Squeezes the next `out.len()` bytes of the output stream.
    #[inline]
    pub fn squeeze(&mut self, out: &mut [u8]) {
        self.inner.squeeze(out);
    }
}

/// Owned 256-bit key used by the keyed hasher.
///
/// Wrapped in a private type so the key can be zeroized on drop even though the
/// public API accepts a plain `[u8; 32]`.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
struct Key([u8; 32]);

/// TurboHash256 hasher.
///
/// `context` is the optional context string `C`; it is absorbed when the
/// hasher is created and is not borrowed afterwards. [`Default`] uses the empty
/// context string. In keyed mode, the 256-bit key is owned and zeroized on
/// drop.
///
/// This type does not allocate and keeps no staging buffer. A single
/// [`update`](Self::update) call spanning at least two whole chunks hashes them
/// in batched permutations; the first chunk, a lone chunk, and the final
/// partial chunk fall back to sequential 16-round Keccak. How many chunks share
/// one permutation depends on the CPU, see [`backend_name`].
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct TurboHash {
    key: Option<Key>,
    core: Option<TurboHashCore>,
}

impl TurboHash {
    /// Creates a hasher with the given context string `C`.
    ///
    /// Pass `&[]` for no context. The string is absorbed immediately, so the
    /// caller may drop it as soon as this function returns.
    #[inline]
    pub fn new(context: &[u8]) -> Self {
        return TurboHash {
            key: None,
            core: Some(TurboHashCore::new(context)),
        };
    }

    /// Creates a keyed hasher with the given 256-bit `key` and context string
    /// `C`.
    ///
    /// The key is copied into the hasher and zeroized on drop. Key material
    /// with a different length should be derived into exactly 32 bytes, for
    /// example with HKDF, rather than truncated.
    #[inline]
    pub fn new_keyed(key: &[u8; 32], context: &[u8]) -> Self {
        return TurboHash {
            key: Some(Key(*key)),
            core: Some(TurboHashCore::new(context)),
        };
    }

    /// Absorbs message bytes.
    #[inline]
    pub fn update(&mut self, data: &[u8]) {
        self.core
            .as_mut()
            .expect("the hasher cannot be updated after finalization")
            .update(data);
    }

    /// Finalizes the hasher and writes `output.len()` bytes.
    ///
    /// The output length is bound into the hash, so two calls with different
    /// output lengths produce unrelated outputs. `output` may be empty; an
    /// empty output uses the same `L = 0` marker as the XOF mode and writes
    /// nothing.
    pub fn finalize_into(mut self, output: &mut [u8]) {
        let key = self.key.take();
        let core = self.core.take().expect("the hasher is finalized once");

        match key {
            None => core.finalize_into(output),
            Some(key) => {
                let mut digest = [0u8; TURBOHASH_CV_LEN];
                core.finalize_into(&mut digest);

                let outer_context = keyed_context(&digest);
                let mut outer = TurboHashCore::new(&outer_context);
                outer.update(&key.0);
                outer.finalize_into(output);
            }
        }
    }

    /// Finalizes the hasher in XOF mode (`L = 0`) and returns a reader.
    ///
    /// The reader produces the prefix-consistent TurboHash stream. Fixed-size
    /// outputs use [`Self::finalize_into`] instead and are unrelated to it.
    pub fn finalize_xof(mut self) -> TurboHashReader {
        let key = self.key.take();
        let core = self.core.take().expect("the hasher is finalized once");

        return match key {
            None => core.finalize_xof(),
            Some(key) => {
                let mut digest = [0u8; TURBOHASH_CV_LEN];
                core.finalize_into(&mut digest);

                let outer_context = keyed_context(&digest);
                let mut outer = TurboHashCore::new(&outer_context);
                outer.update(&key.0);
                outer.finalize_xof()
            }
        };
    }

    /// Hashes `message` under `context` and writes `output.len()` bytes.
    ///
    /// The output length is bound into the hash. Use [`Self::xof`] for the XOF
    /// variant.
    #[inline]
    pub fn hash(context: &[u8], message: &[u8], output: &mut [u8]) {
        let mut hasher = TurboHash::new(context);
        hasher.update(message);
        hasher.finalize_into(output);
    }

    /// Hashes `message` under `context` in XOF mode and writes `output.len()`
    /// bytes.
    ///
    /// The requested length does not change the stream: the bytes written are a
    /// prefix of the bytes [`Self::finalize_xof`] would produce.
    #[inline]
    pub fn xof(context: &[u8], message: &[u8], output: &mut [u8]) {
        let mut hasher = TurboHash::new(context);
        hasher.update(message);
        hasher.finalize_xof().squeeze(output);
    }

    /// Computes `TurboHash(Key, M, C, output.len())` in one call.
    ///
    /// The key is the first argument; `context` precedes the message. This is
    /// the HopMAC-shaped construction described in the module-level
    /// documentation.
    #[inline]
    pub fn mac(key: &[u8; 32], context: &[u8], message: &[u8], output: &mut [u8]) {
        let mut hasher = TurboHash::new_keyed(key, context);
        hasher.update(message);
        hasher.finalize_into(output);
    }

    /// Computes the keyed XOF variant in one call, writing `output.len()`
    /// bytes.
    #[inline]
    pub fn mac_xof(key: &[u8; 32], context: &[u8], message: &[u8], output: &mut [u8]) {
        let mut hasher = TurboHash::new_keyed(key, context);
        hasher.update(message);
        hasher.finalize_xof().squeeze(output);
    }
}

impl Default for TurboHash {
    /// Creates an unkeyed hasher with an empty context string.
    #[inline]
    fn default() -> Self {
        return TurboHash::new(&[]);
    }
}

/// Builds the outer context of the keyed construction.
fn keyed_context(digest: &[u8; TURBOHASH_CV_LEN]) -> [u8; KEYED_CONTEXT_LEN] {
    let mut context = [0u8; KEYED_CONTEXT_LEN];
    context[..KEYED_DOMAIN.len()].copy_from_slice(KEYED_DOMAIN);
    context[KEYED_DOMAIN.len()..].copy_from_slice(digest);
    return context;
}

/// The unkeyed tree state shared by the plain and keyed hashers.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
struct TurboHashCore {
    /// Single-node sponge until the input grows past one chunk, then the final
    /// node sponge.
    first: KeccakSponge<TURBOHASH_ROUNDS>,
    /// Number of bytes absorbed into `first` while it holds the single node.
    first_len: usize,
    /// Whether the input has grown beyond the first chunk.
    tree: bool,
    /// Current partial leaf chunk, if any. Created lazily once tree mode is
    /// entered, so single-node messages do not pay for zeroing it.
    queue: Option<KeccakSponge<TURBOHASH_ROUNDS>>,
    queue_len: usize,
    /// Batched leaf sponge, created lazily in tree mode.
    leaves: Option<Leaves<TURBOHASH_ROUNDS>>,
    /// Number of chaining values absorbed into the final node.
    chunks: usize,
}

impl TurboHashCore {
    /// Creates an unkeyed core and absorbs `left_encode(|C|) || C`.
    fn new(context: &[u8]) -> Self {
        let mut core = TurboHashCore {
            first: KeccakSponge::new(TURBOHASH_RATE, SINGLE_PAD),
            first_len: 0,
            tree: false,
            queue: None,
            queue_len: 0,
            leaves: None,
            chunks: 0,
        };

        // Absorb the length-prefixed context immediately. This may already
        // cross the first chunk boundary when the context is large.
        core.update(left_encode(context.len()).as_ref());
        core.update(context);

        return core;
    }

    /// Absorbs message bytes.
    fn update(&mut self, mut data: &[u8]) {
        if data.is_empty() {
            return;
        }

        if !self.tree {
            let take = min(TURBOHASH_CHUNK_SIZE - self.first_len, data.len());
            self.first.absorb(&data[..take]);
            self.first_len += take;
            data = &data[take..];

            if data.is_empty() {
                return;
            }

            self.begin_tree();
        }

        self.absorb_leaves(data);
    }

    /// Finalizes the core and writes `output.len()` bytes.
    fn finalize_into(mut self, output: &mut [u8]) {
        let mut sponge = self.finish(output.len());
        sponge.squeeze(output);
    }

    /// Finalizes the core in XOF mode and returns a reader.
    fn finalize_xof(mut self) -> TurboHashReader {
        return TurboHashReader {
            inner: self.finish(0),
        };
    }

    /// Switches from the single-node layout to the tree layout.
    fn begin_tree(&mut self) {
        debug_assert_eq!(self.first_len, TURBOHASH_CHUNK_SIZE);
        debug_assert!(!self.tree);

        self.tree = true;

        // The first chunk is still in `first`. Re-label it as a leaf, squeeze
        // its chaining value, and start a fresh final node.
        self.first.set_delimiter(INTERMEDIATE_PAD);
        let mut cv = [0u8; TURBOHASH_CV_LEN];
        self.first.squeeze(&mut cv);

        let mut final_node = KeccakSponge::new(TURBOHASH_RATE, FINAL_PAD);
        final_node.absorb(&cv);
        self.first = final_node;
        self.first_len = 0;
        self.chunks = 1;
        self.leaves = Some(Leaves::new(TURBOHASH_RATE, INTERMEDIATE_PAD, Backend::detect()));
    }

    /// Absorbs bytes that belong to leaf chunks.
    fn absorb_leaves(&mut self, mut data: &[u8]) {
        // Finish the partial leaf chunk first.
        if self.queue_len > 0 {
            let take = min(TURBOHASH_CHUNK_SIZE - self.queue_len, data.len());
            self.queue
                .as_mut()
                .expect("the leaf queue exists while it holds bytes")
                .absorb(&data[..take]);
            self.queue_len += take;
            data = &data[take..];

            if self.queue_len == TURBOHASH_CHUNK_SIZE {
                self.flush_queue();
            }
        }

        // Whole batches, straight from the caller's slice.
        let width = self.leaves.as_ref().expect("tree mode initializes the leaves").width();
        while data.len() >= TURBOHASH_CHUNK_SIZE * width {
            let mut blocks: [&[u8]; MAX_BATCH] = [&[]; MAX_BATCH];
            let mut rest = data;
            for slot in blocks[..width].iter_mut() {
                let (block, tail) = rest.split_at(TURBOHASH_CHUNK_SIZE);
                *slot = block;
                rest = tail;
            }
            hash_batch_into(
                &mut self.first,
                self.leaves.as_mut().expect("tree mode initializes the leaves"),
                &mut self.chunks,
                &blocks[..width],
                width,
            );
            data = rest;
        }

        // Fill a partial batch of `2..width` whole chunks with the unused lanes
        // aliased to a real chunk. Only the first `whole` chaining values are
        // absorbed, so the dummy lanes cannot affect the output.
        let whole = data.len() / TURBOHASH_CHUNK_SIZE;
        if whole >= 2 {
            let mut blocks: [&[u8]; MAX_BATCH] = [&[]; MAX_BATCH];
            for (j, slot) in blocks[..width].iter_mut().enumerate() {
                let idx = j.min(whole - 1);
                *slot = &data[idx * TURBOHASH_CHUNK_SIZE..(idx + 1) * TURBOHASH_CHUNK_SIZE];
            }
            hash_batch_into(
                &mut self.first,
                self.leaves.as_mut().expect("tree mode initializes the leaves"),
                &mut self.chunks,
                &blocks[..width],
                whole,
            );
            data = &data[whole * TURBOHASH_CHUNK_SIZE..];
        }

        // At most one whole chunk remains; a wide permutation would not be
        // cheaper than one sequential 16-round Keccak call.
        if data.len() >= TURBOHASH_CHUNK_SIZE {
            let (block, rest) = data.split_at(TURBOHASH_CHUNK_SIZE);
            hash_one_into(&mut self.first, &mut self.chunks, block);
            data = rest;
        }

        // Buffer the partial tail inside the queue sponge.
        if !data.is_empty() {
            debug_assert_eq!(self.queue_len, 0);
            let queue = self
                .queue
                .get_or_insert_with(|| KeccakSponge::new(TURBOHASH_RATE, INTERMEDIATE_PAD));
            queue.absorb(data);
            self.queue_len = data.len();
        }
    }

    /// Finalizes the tree and returns the final-node sponge.
    fn finish(&mut self, output_len: usize) -> KeccakSponge<TURBOHASH_ROUNDS> {
        if self.tree {
            if self.queue_len > 0 {
                self.flush_queue();
            }

            let count = length_encode(self.chunks);
            self.first.absorb(count.as_ref());
            self.first.absorb(&END);
        }

        // Bind the requested output length (0 for XOF) into the final node.
        self.first.absorb(length_encode(output_len).as_ref());

        // `TurboHashCore` implements `Drop` when `zeroize` is on, so move the
        // finished sponge out instead of returning the field.
        return core::mem::replace(&mut self.first, KeccakSponge::new(TURBOHASH_RATE, SINGLE_PAD));
    }

    fn flush_queue(&mut self) {
        debug_assert!(self.queue_len > 0);

        let mut cv = [0u8; TURBOHASH_CV_LEN];
        self.queue
            .as_mut()
            .expect("the leaf queue holds the bytes being flushed")
            .squeeze(&mut cv);
        self.first.absorb(&cv);
        self.chunks += 1;

        self.queue = None;
        self.queue_len = 0;
    }
}

fn hash_one_into(final_node: &mut KeccakSponge<TURBOHASH_ROUNDS>, chunks: &mut usize, chunk: &[u8]) {
    let mut cv = [0u8; TURBOHASH_CV_LEN];
    let mut leaf: KeccakSponge<TURBOHASH_ROUNDS> = KeccakSponge::new(TURBOHASH_RATE, INTERMEDIATE_PAD);
    leaf.absorb(chunk);
    leaf.squeeze(&mut cv);
    final_node.absorb(&cv);
    *chunks += 1;
}

fn hash_batch_into(
    final_node: &mut KeccakSponge<TURBOHASH_ROUNDS>,
    leaves: &mut Leaves<TURBOHASH_ROUNDS>,
    chunks: &mut usize,
    blocks: &[&[u8]],
    active: usize,
) {
    let mut cvs = [[0u8; TURBOHASH_CV_LEN]; MAX_BATCH];
    debug_assert_eq!(blocks.len(), leaves.width());
    debug_assert!((1..=blocks.len()).contains(&active));

    leaves.reset();
    leaves.absorb_batch(blocks);
    leaves.squeeze_cvs::<TURBOHASH_CV_LEN>(&mut cvs[..blocks.len()]);

    for cv in &cvs[..active] {
        final_node.absorb(cv);
    }
    *chunks += active;
}

/// Returns the name of the batched Keccak backend TurboHash uses on this CPU.
///
/// Useful in benchmarks and CI logs. Detection is cached by the standard
/// library, so calling this repeatedly is cheap.
///
/// ```
/// use crypto::turbohash::backend_name;
/// println!("TurboHash backend: {}", backend_name());
/// ```
pub fn backend_name() -> &'static str {
    return Backend::detect().name();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sha3::keccak::test_util::sponge;

    /// RFC 9861, section 5: `ptn(n)` repeats the 251-byte cycle `00..=FA` as
    /// many times as needed, truncated to `n` bytes.
    fn ptn(n: usize) -> Vec<u8> {
        return (0..n).map(|i| (i % 251) as u8).collect();
    }

    fn key32() -> [u8; 32] {
        let mut key = [0u8; 32];
        key.copy_from_slice(&ptn(32));
        return key;
    }

    /// Literal transcription of the construction defined in the module docs.
    fn spec_turbohash(message: &[u8], context: &[u8], output_len: usize, xof: bool) -> Vec<u8> {
        let mut s = Vec::with_capacity(context.len() + 9 + message.len());
        s.extend_from_slice(left_encode(context.len()).as_ref());
        s.extend_from_slice(context);
        s.extend_from_slice(message);

        let bound_len = if xof { 0 } else { output_len };
        let encoded_len = length_encode(bound_len);

        let mut out = vec![0u8; output_len];
        if s.len() <= TURBOHASH_CHUNK_SIZE {
            let mut final_node = s;
            final_node.extend_from_slice(encoded_len.as_ref());
            sponge::<TURBOHASH_ROUNDS>(&final_node, TURBOHASH_RATE, SINGLE_PAD, &mut out);
            return out;
        }

        let mut final_node = Vec::new();
        let mut chunks = 0;
        for chunk in s.chunks(TURBOHASH_CHUNK_SIZE) {
            let mut cv = [0u8; TURBOHASH_CV_LEN];
            sponge::<TURBOHASH_ROUNDS>(chunk, TURBOHASH_RATE, INTERMEDIATE_PAD, &mut cv);
            final_node.extend_from_slice(&cv);
            chunks += 1;
        }

        final_node.extend_from_slice(length_encode(chunks).as_ref());
        final_node.extend_from_slice(&END);
        final_node.extend_from_slice(encoded_len.as_ref());

        sponge::<TURBOHASH_ROUNDS>(&final_node, TURBOHASH_RATE, FINAL_PAD, &mut out);
        return out;
    }

    /// Known-answer vectors produced by an independent Python reference during
    /// implementation. The reference implements `Keccak-p[1600, 16]` directly
    /// from FIPS 202 and was checked against the XKCP `Keccak-f[1600]`
    /// all-zero-state vector before generating these.
    struct UnkeyedVector {
        message_len: usize,
        context_len: usize,
        output_len: usize,
        expected: &'static str,
    }

    fn unkeyed_fixed_vectors() -> Vec<UnkeyedVector> {
        return vec![
            UnkeyedVector {
                message_len: 0,
                context_len: 0,
                output_len: 32,
                expected: "b7444b55c104041b12a01d222e6929b1aef8dd5bc08f989ea6c89e08730feba3",
            },
            UnkeyedVector {
                message_len: 0,
                context_len: 0,
                output_len: 64,
                expected: "4d5432523db98706fc026fe509e6096061863db436f0a72a294f5d877e15bf86\
                           561ab705f35d14e3814f5633378ee7a6439fe8e4052e1c834d99d33c1faa171c",
            },
            UnkeyedVector {
                message_len: 1,
                context_len: 0,
                output_len: 32,
                expected: "cceb535a81dc4db651cb557030afd365031c9819691efc9624a720cbf53a71cc",
            },
            UnkeyedVector {
                message_len: 17,
                context_len: 0,
                output_len: 64,
                expected: "ba0d15248180646f74638d58f8de0cc36a77af54791731f76e68bf74ba16cf32\
                           00376be53d0eb8ae2a1e1d3111ccaf2ffa694f1b62efd2d045fc8ed416f00c36",
            },
            UnkeyedVector {
                message_len: 136,
                context_len: 1,
                output_len: 32,
                expected: "639fdb5995590485a6cf9c2789f23184d4a5110e90229a8115eec2277f71bc92",
            },
            UnkeyedVector {
                message_len: 4095,
                context_len: 41,
                output_len: 32,
                expected: "3d60aa4e4af8cff1dab1b735fa09706a7c929ba08fea33d430a19031d7515e47",
            },
            UnkeyedVector {
                message_len: 4096,
                context_len: 0,
                output_len: 64,
                expected: "9b2fea4ef7072ab7cbf182f6a637dcb3bdb2aa75d4256f6ee8cab0560a0537b0\
                           d2cdea4297310e028c2747dc53e40c762dabc433df82f22e24bdbdcf29d9a4bc",
            },
            UnkeyedVector {
                message_len: 4097,
                context_len: 0,
                output_len: 64,
                expected: "e9e0077e222541ccb037674aebcb40671d08bb42e23069ef495a2628568a0a2e\
                           4fc3997cae2f80d8ff2cff41461e6ad2b1b1afb6bd957b6aabece83a174a0451",
            },
            UnkeyedVector {
                message_len: 8192,
                context_len: 41,
                output_len: 32,
                expected: "b64ea92ebcb6fbe729862b387bfad0db0b2cac1fb80e8b675b04cd26c06e99c5",
            },
            UnkeyedVector {
                message_len: 8193,
                context_len: 3,
                output_len: 64,
                expected: "11fecd4b6d6f7f65350fc5276a7c5b33aa1a345aa9d21abfbee18f7d3bde8dcb\
                           8c3fde7e1c578dfaa4cefcb3a774e0bdc3cdeb79e5409a326cb786fdb68663ec",
            },
            UnkeyedVector {
                message_len: 3 * 4096 + 17,
                context_len: 41,
                output_len: 32,
                expected: "f2f3186f57bd20748bfeb30cbaec5529443a479a01c1614e57528dd27e707359",
            },
            UnkeyedVector {
                message_len: 3 * 4096 + 17,
                context_len: 41,
                output_len: 64,
                expected: "9a44e065646b729a1dfc463598c4da412398d4e654e4845c7cc868fb2b05fd82\
                           16c2c9931754b99773e76929cd581e8d37b5bad531e22cffd9bc798f00e03e39",
            },
        ];
    }

    #[test]
    fn known_answer_vectors() {
        for (i, vector) in unkeyed_fixed_vectors().iter().enumerate() {
            let message = ptn(vector.message_len);
            let context = ptn(vector.context_len);
            let mut output = vec![0u8; vector.output_len];
            TurboHash::hash(&context, &message, &mut output);
            assert_eq!(
                hex::encode(&output),
                vector.expected.replace(['\n', ' '], ""),
                "fixed vector {i}: message {}, context {}, output {}",
                vector.message_len,
                vector.context_len,
                vector.output_len
            );
        }

        struct XofVector {
            message_len: usize,
            context_len: usize,
            output_len: usize,
            expected: &'static str,
        }

        let xof_vectors = [
            XofVector {
                message_len: 0,
                context_len: 0,
                output_len: 64,
                expected: "c5b0efd6eef842ca7d6c5c21196675e5f054d9c33b8560ef283e743fb9ed5fe7\
                           fddb5fcc7c4bafdb05c41ee8956114fc2990e1d8827774fb4aaf5cc35b97f487",
            },
            XofVector {
                message_len: 4095,
                context_len: 0,
                output_len: 64,
                expected: "1bb8909bf84c018a586cac770e2ad50bfbdfedae064a57cd3dc0580edb69cbaf\
                           62844fec3d2080672cae671f28b253a06fd30572781e06a839ba798d2763a951",
            },
            XofVector {
                message_len: 4096,
                context_len: 0,
                output_len: 64,
                expected: "d6dc7f34bdac98339eb9d09b94812756d0162c8371b203fe94f2a4a424df9318\
                           450b6c8e42471ddd02563702a4423b847acb62fc718c4339f16f8f9cf6e534a9",
            },
            XofVector {
                message_len: 4097,
                context_len: 41,
                output_len: 64,
                expected: "0b8c5e23c4235a18d9de93fdd013eb38b8dd16a4378b9efbecb079448e9f8ffa\
                           13c85646453771096668a0791136af70dd81ecfa94bdeb04c08a5d84dfd9a2d4",
            },
        ];

        for (i, vector) in xof_vectors.iter().enumerate() {
            let message = ptn(vector.message_len);
            let context = ptn(vector.context_len);
            let mut output = vec![0u8; vector.output_len];
            TurboHash::xof(&context, &message, &mut output);
            assert_eq!(
                hex::encode(&output),
                vector.expected.replace(['\n', ' '], ""),
                "xof vector {i}: message {}, context {}, output {}",
                vector.message_len,
                vector.context_len,
                vector.output_len
            );
        }

        struct KeyedVector {
            message_len: usize,
            context_len: usize,
            output_len: usize,
            expected: &'static str,
        }

        let keyed_vectors = [
            KeyedVector {
                message_len: 0,
                context_len: 0,
                output_len: 32,
                expected: "4ec74e6dc86cb187bffaa8da5f648b6f729752c635141272fb98f553dfa7fb9e",
            },
            KeyedVector {
                message_len: 0,
                context_len: 0,
                output_len: 64,
                expected: "d2ab37cf448e4b2a7786d7c97da05d1bd34ea10cb6fc43ef4021014be51290b9\
                           7ab20db245a0606f37ec8b653b691f05c83d3998298caff5ae2a7a56e9bc2a99",
            },
            KeyedVector {
                message_len: 4096,
                context_len: 0,
                output_len: 64,
                expected: "b0f11c5649ec61c6e3eb25bb398aebd5224ce7e5049093c352c71d3be5e64d9d\
                           7d78da3b3011d8f937dc0f1417f2aed449bb7b5a54a8b749bf188ef7d6efbf3a",
            },
            KeyedVector {
                message_len: 4097,
                context_len: 41,
                output_len: 64,
                expected: "1080cb0efb73d67ed93b180fdbb2e1c4dc0d53aeb7dee695e2e80a073f572739\
                           283454f210e9195c1b793eb2563359721513f3db574de85d7454218834bfb101",
            },
            KeyedVector {
                message_len: 3 * 4096,
                context_len: 8189,
                output_len: 32,
                expected: "17ac8283a2517a578357c6c65d7e76a3e171a76ddf25701cf036cd8e9e6c8de8",
            },
            KeyedVector {
                message_len: 3 * 4096 + 17,
                context_len: 41,
                output_len: 32,
                expected: "210d8f06b27d7c7ed0eeb42f5c9147f3e764e76cf293d4474c4511d5cc0dd29a",
            },
        ];

        let key = key32();
        for (i, vector) in keyed_vectors.iter().enumerate() {
            let message = ptn(vector.message_len);
            let context = ptn(vector.context_len);
            let mut output = vec![0u8; vector.output_len];
            TurboHash::mac(&key, &context, &message, &mut output);
            assert_eq!(
                hex::encode(&output),
                vector.expected.replace(['\n', ' '], ""),
                "keyed vector {i}: message {}, context {}, output {}",
                vector.message_len,
                vector.context_len,
                vector.output_len
            );
        }

        let keyed_xof_vectors = [
            KeyedVector {
                message_len: 0,
                context_len: 0,
                output_len: 64,
                expected: "db6b14b6cc4b82fafc3c80c4735f86e08f81ff9a5db54ff1a4321cbb42effd7e\
                           303bccca0b3a38498fe95c56dfef480240811adbf12e8387fdea2ef35c13d3b8",
            },
            KeyedVector {
                message_len: 4096,
                context_len: 0,
                output_len: 64,
                expected: "064614ea50ccdda6196f434f70ead1fdd1acf0e2fec1f5128e384b7d02c336b7\
                           23ed0e8cbf2dbf19a249ca4deb93b9b1673eaf4052e652e44da5da12b1a2addb",
            },
            KeyedVector {
                message_len: 4097,
                context_len: 41,
                output_len: 64,
                expected: "9242f90eb3832c668a8df3d493442dc6ba6b09513aa925e091f900c1e86fb333\
                           88d87ea98994cdf6ce94e4de023c2d295adf31fb6a05da12ad7f361f17c0a3d8",
            },
        ];

        for (i, vector) in keyed_xof_vectors.iter().enumerate() {
            let message = ptn(vector.message_len);
            let context = ptn(vector.context_len);
            let mut output = vec![0u8; vector.output_len];
            TurboHash::mac_xof(&key, &context, &message, &mut output);
            assert_eq!(
                hex::encode(&output),
                vector.expected.replace(['\n', ' '], ""),
                "keyed xof vector {i}: message {}, context {}, output {}",
                vector.message_len,
                vector.context_len,
                vector.output_len
            );
        }
    }

    #[test]
    fn default_uses_empty_context() {
        let mut from_default = [0u8; 64];
        let mut hasher = TurboHash::default();
        hasher.update(b"default");
        hasher.finalize_into(&mut from_default);

        let mut explicit = [0u8; 64];
        TurboHash::hash(&[], b"default", &mut explicit);
        assert_eq!(from_default, explicit);
    }

    #[test]
    fn matches_literal_spec_around_chunk_boundaries() {
        let widths = Leaves::<TURBOHASH_ROUNDS>::new(TURBOHASH_RATE, INTERMEDIATE_PAD, Backend::detect()).width();
        let lengths = [
            0,
            1,
            135,
            136,
            137,
            4095,
            4096,
            4097,
            8191,
            8192,
            8193,
            widths * TURBOHASH_CHUNK_SIZE - 1,
            widths * TURBOHASH_CHUNK_SIZE,
            widths * TURBOHASH_CHUNK_SIZE + 1,
        ];

        for len in lengths {
            for context_len in [0usize, 1, 3, 41, 4095, 4096, 4097, 20000] {
                for out_len in [1usize, 32, 64, 136, 1000] {
                    let message = ptn(len);
                    let context = ptn(context_len);
                    let expected = spec_turbohash(&message, &context, out_len, false);
                    let mut actual = vec![0u8; out_len];
                    TurboHash::hash(&context, &message, &mut actual);
                    assert_eq!(
                        actual, expected,
                        "message length {len}, context length {context_len}, output length {out_len}"
                    );
                }
            }
        }
    }

    #[test]
    fn xof_matches_literal_spec() {
        for len in [0usize, 1, 4095, 4096, 4097, 2 * 4096 + 11] {
            for context_len in [0usize, 1, 41, 4096] {
                let message = ptn(len);
                let context = ptn(context_len);
                let expected = spec_turbohash(&message, &context, 1000, true);

                let mut one_shot = [0u8; 1000];
                TurboHash::xof(&context, &message, &mut one_shot);
                assert_eq!(one_shot, expected[..], "message {len}, context {context_len}");

                let mut hasher = TurboHash::new(&context);
                hasher.update(&message);
                let mut reader = hasher.finalize_xof();
                let mut streamed = [0u8; 1000];
                let mut offset = 0;
                for chunk in [1usize, 7, 63, 135, 137, 1, 500] {
                    let n = chunk.min(streamed.len() - offset);
                    reader.squeeze(&mut streamed[offset..offset + n]);
                    offset += n;
                }
                reader.squeeze(&mut streamed[offset..]);
                assert_eq!(streamed, expected[..]);
            }
        }
    }

    #[test]
    fn fixed_output_lengths_are_unrelated() {
        let message = ptn(3 * TURBOHASH_CHUNK_SIZE + 17);
        let context = ptn(41);

        let mut short = [0u8; 32];
        let mut long = [0u8; 64];
        TurboHash::hash(&context, &message, &mut short);
        TurboHash::hash(&context, &message, &mut long);
        assert_ne!(short[..], long[..32]);

        let mut xof = [0u8; 64];
        TurboHash::xof(&context, &message, &mut xof);
        assert_ne!(xof[..32], short[..]);
        assert_ne!(xof, long);

        let mut short_xof = [0u8; 32];
        TurboHash::xof(&context, &message, &mut short_xof);
        assert_eq!(short_xof, xof[..32]);
    }

    #[test]
    fn context_does_not_need_to_outlive_the_hasher() {
        let mut hasher = {
            let context = ptn(41);
            TurboHash::new(&context)
        };
        hasher.update(b"message");

        let mut output = [0u8; 64];
        hasher.finalize_into(&mut output);

        let mut expected = [0u8; 64];
        TurboHash::hash(&ptn(41), b"message", &mut expected);
        assert_eq!(output, expected);
    }

    #[test]
    fn context_framing_is_unambiguous() {
        let mut a = [0u8; 32];
        let mut b = [0u8; 32];
        let mut c = [0u8; 32];
        let mut d = [0u8; 32];

        // Ambiguous under `C || length_encode(|C|) || M`: both inputs would
        // have produced `00 01`.
        TurboHash::hash(b"", &[0x01], &mut a);
        TurboHash::hash(&[0x00], &[], &mut b);
        assert_ne!(a, b);

        // Ambiguous under plain `C || M`: both inputs would have produced
        // `61`.
        TurboHash::hash(b"", b"a", &mut c);
        TurboHash::hash(b"a", b"", &mut d);
        assert_ne!(c, d);
    }

    #[test]
    fn context_is_bound_into_the_hash() {
        let message = ptn(3 * TURBOHASH_CHUNK_SIZE + 1);
        let mut a = [0u8; 64];
        let mut b = [0u8; 64];
        let mut c = [0u8; 64];
        TurboHash::hash(b"one", &message, &mut a);
        TurboHash::hash(b"two", &message, &mut b);
        TurboHash::hash(b"one\0", &message, &mut c);
        assert_ne!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn streaming_matches_one_shot() {
        let widths = Leaves::<TURBOHASH_ROUNDS>::new(TURBOHASH_RATE, INTERMEDIATE_PAD, Backend::detect()).width();

        for &len in &[
            0usize,
            1,
            4095,
            4096,
            4097,
            2 * 4096,
            2 * 4096 + 1,
            widths * TURBOHASH_CHUNK_SIZE,
            widths * TURBOHASH_CHUNK_SIZE + 7,
        ] {
            let data = ptn(len);
            let context = ptn(41);

            let mut one_shot = [0u8; 64];
            TurboHash::hash(&context, &data, &mut one_shot);

            for &chunk_size in &[1usize, 3, 7, 136, 4096, 2 * 4096 + 1, widths * 4096 + 1] {
                let mut hasher = TurboHash::new(&context);
                for chunk in data.chunks(chunk_size) {
                    hasher.update(chunk);
                }
                let mut streamed = [0u8; 64];
                hasher.finalize_into(&mut streamed);
                assert_eq!(one_shot, streamed, "len {len}, chunk {chunk_size}");
            }
        }
    }

    #[test]
    fn batched_and_sequential_leaves_agree() {
        let data = ptn(3 * TURBOHASH_CHUNK_SIZE + 17);
        let context = ptn(7);

        let mut batched = [0u8; 64];
        TurboHash::hash(&context, &data, &mut batched);

        // Feeding one chunk at a time never leaves two whole chunks in a single
        // update, so the leaves are hashed sequentially.
        let mut sequential = [0u8; 64];
        let mut hasher = TurboHash::new(&context);
        for chunk in data.chunks(TURBOHASH_CHUNK_SIZE) {
            hasher.update(chunk);
        }
        hasher.finalize_into(&mut sequential);

        assert_eq!(batched, sequential);
    }

    #[test]
    fn clone_mid_stream_matches_linear() {
        let data = ptn(3 * TURBOHASH_CHUNK_SIZE + 7);
        let context = ptn(13);

        let mut expected = [0u8; 64];
        TurboHash::hash(&context, &data, &mut expected);

        for split in [0usize, 4095, 4096, 4097, 2 * 4096 + 5] {
            let mut hasher = TurboHash::new(&context);
            hasher.update(&data[..split]);
            let mut clone = hasher.clone();
            hasher.update(&data[split..]);
            clone.update(&data[split..]);

            let mut from_original = [0u8; 64];
            let mut from_clone = [0u8; 64];
            hasher.finalize_into(&mut from_original);
            clone.finalize_into(&mut from_clone);

            assert_eq!(from_original, expected, "split {split}");
            assert_eq!(from_clone, expected, "split {split}");
        }
    }

    #[test]
    fn zero_length_output_is_allowed() {
        let mut empty = [0u8; 0];
        TurboHash::hash(b"", b"data", &mut empty);
        TurboHash::xof(b"", b"data", &mut empty);

        let mut one = [0u8; 1];
        TurboHash::hash(b"", b"data", &mut one);
        assert_ne!(one, [0u8]);
    }

    #[test]
    fn keyed_mode_matches_hopmac_composition() {
        let cases = [
            (0usize, 0usize, 1usize),
            (1, 1, 32),
            (4095, 0, 32),
            (4096, 41, 64),
            (4097, 41, 64),
            (3 * 4096, 8189, 32),
            (2 * 4096 + 1, 20000, 64),
        ];
        let key = key32();

        for (message_len, context_len, output_len) in cases {
            let message = ptn(message_len);
            let context = ptn(context_len);

            let mut inner = [0u8; TURBOHASH_CV_LEN];
            TurboHash::hash(&context, &message, &mut inner);
            let outer_context = keyed_context(&inner);
            let expected = spec_turbohash(&key, &outer_context, output_len, false);

            let mut output = vec![0u8; output_len];
            TurboHash::mac(&key, &context, &message, &mut output);
            assert_eq!(
                output, expected,
                "message {message_len}, context {context_len}, output {output_len}"
            );

            let expected_xof = spec_turbohash(&key, &outer_context, 1000, true);
            let mut xof = [0u8; 1000];
            TurboHash::mac_xof(&key, &context, &message, &mut xof);
            assert_eq!(xof, expected_xof[..]);
        }
    }

    #[test]
    fn keyed_mode_binds_key_message_and_context() {
        let message = ptn(3 * TURBOHASH_CHUNK_SIZE + 1);
        let key_a = [0x11u8; 32];
        let key_b = [0x22u8; 32];

        let mut a = [0u8; 64];
        let mut b = [0u8; 64];
        let mut c = [0u8; 64];
        TurboHash::mac(&key_a, b"custom", &message, &mut a);
        TurboHash::mac(&key_b, b"custom", &message, &mut b);
        TurboHash::mac(&key_a, b"other", &message, &mut c);

        assert_ne!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn keyed_mode_has_an_explicit_domain_separator() {
        let key = key32();
        let context = b"domain separation";
        let message = ptn(3 * TURBOHASH_CHUNK_SIZE + 1);

        let mut tag = [0u8; 64];
        TurboHash::mac(&key, context, &message, &mut tag);

        let mut inner = [0u8; TURBOHASH_CV_LEN];
        TurboHash::hash(context, &message, &mut inner);

        // Without the keyed domain, the outer call would be an unkeyed hash
        // with the inner digest as its context.
        let mut unkeyed = [0u8; 64];
        TurboHash::hash(&inner, &key, &mut unkeyed);
        assert_ne!(tag, unkeyed);

        // With the domain prefix it is exactly the documented composition.
        let outer_context = keyed_context(&inner);
        let mut expected = [0u8; 64];
        TurboHash::hash(&outer_context, &key, &mut expected);
        assert_eq!(tag, expected);
    }

    #[test]
    fn keyed_streaming_and_clone_match_one_shot() {
        let key = key32();
        let context = ptn(41);
        let data = ptn(3 * TURBOHASH_CHUNK_SIZE + 7);

        let mut expected = [0u8; 64];
        TurboHash::mac(&key, &context, &data, &mut expected);

        let mut hasher = TurboHash::new_keyed(&key, &context);
        hasher.update(&data[..4097]);
        let mut clone = hasher.clone();
        hasher.update(&data[4097..]);
        clone.update(&data[4097..]);

        let mut from_original = [0u8; 64];
        let mut from_clone = [0u8; 64];
        hasher.finalize_into(&mut from_original);
        clone.finalize_into(&mut from_clone);
        assert_eq!(from_original, expected);
        assert_eq!(from_clone, expected);

        let mut hasher = TurboHash::new_keyed(&key, &context);
        for chunk in data.chunks(7) {
            hasher.update(chunk);
        }
        let mut streamed = [0u8; 64];
        hasher.finalize_into(&mut streamed);
        assert_eq!(streamed, expected);
    }

    #[test]
    fn keyed_accepts_empty_output() {
        let key = [0u8; 32];

        let mut empty_output = [0u8; 0];
        TurboHash::mac(&key, b"", b"data", &mut empty_output);

        let mut output = [0u8; 32];
        TurboHash::mac(&key, b"", b"data", &mut output);
        assert_ne!(output, [0u8; 32]);
    }
}
