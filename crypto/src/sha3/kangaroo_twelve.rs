//! KangarooTwelve-256 (KT256) and HopMAC256, as defined in RFC 9861.
//!
//! KangarooTwelve is a tree hash built on TurboSHAKE. It splits
//! `S = M || C || length_encode(|C|)` into 8192-byte chunks, hashes every chunk
//! after the first independently into a 64-byte chaining value, and absorbs the
//! first chunk together with those chaining values into a final TurboSHAKE256
//! call. The chunks are independent, so this module hashes several of them at a
//! time with the batched Keccak core used by
//! [`ParallelHash256`](super::ParallelHash256).
//!
//! # Parallelism
//!
//! | Target        | Backend              | Chunks per permutation |
//! |---------------|----------------------|------------------------|
//! | `x86_64`      | AVX-512F (runtime)   | 8                      |
//! | `x86_64`      | AVX2 (runtime)       | 4                      |
//! | `aarch64`     | Armv8.4-A SHA-3      | 2                      |
//! | `aarch64`     | NEON                 | 2                      |
//! | `wasm32`      | `simd128` (compile-time) | 2                  |
//! | anything else | portable scalar      | 2                      |
//!
//! WebAssembly has no runtime feature detection, so `simd128` is used only
//! when it is enabled at build time (`-C target-feature=+simd128`).
//!
//! The first chunk is always absorbed serially and the final node is a single
//! sponge, so messages of at most 8192 bytes see no tree parallelism at all.
//! For longer messages the leaf phase scales with the backend width, minus one
//! 64-byte chaining value absorbed per chunk.
//!
//! # Allocation-free
//!
//! A [`Kt256`] keeps a fixed-size Keccak state and a fixed-size batched leaf
//! sponge. It does not allocate, including on the streaming and XOF paths, and
//! works without the `alloc` feature.
//!
//! # Customization string
//!
//! `C` is appended after the message, so [`Kt256::new`] borrows it for the
//! lifetime of the hasher instead of copying it. This keeps the type
//! allocation-free and lets a streaming caller choose `C` at construction. The
//! empty customization string is the default used by [`Default`].
//!
//! # Output
//!
//! KT256 is an eXtendable-Output Function: the output length is not bound into
//! the hash, and a shorter output is always a prefix of a longer one for the
//! same message and customization string. [`Kt256::finalize_xof`] returns a
//! reader for incremental output; [`Kt256::finalize_into`] writes a one-shot
//! output of any length.
//!
//! # HopMAC256
//!
//! [`HopMac256`] is the hash-then-MAC construction recommended by RFC 9861,
//! section 4:
//!
//! `HopMAC256(Key, M, C, L) = KT256(Key, KT256(M, C, 64), L)`.
//!
//! The inner call is keyless and compresses the message and customization
//! string to a 64-byte digest; the outer call authenticates that digest with
//! the key. Only the outer call processes the key, so a side-channel-protected
//! implementation only has to protect that part.
//!
//! # Example
//!
//! ```ignore
//! use crypto::sha3::Kt256;
//!
//! let mut output = [0u8; 64];
//! Kt256::hash(b"hello world", b"application", &mut output);
//!
//! let mut hasher = Kt256::new(b"application");
//! hasher.update(b"hello ");
//! hasher.update(b"world");
//! let mut reader = hasher.finalize_xof();
//! reader.squeeze(&mut output);
//! ```

use core::cmp::min;

use super::{
    keccak_batch::{Backend, Leaves, MAX_BATCH},
    turboshake::{TURBOSHAKE256_RATE, TurboShake256},
};
use crate::{Bytes, Xof};

/// Size of a KangarooTwelve chunk, in bytes, as specified by RFC 9861.
pub const KT256_CHUNK_SIZE: usize = 8192;

/// Chaining-value size of KT256, in bytes (512 bits).
const KT256_CV_LEN: usize = 64;

/// Rate of the TurboSHAKE256 sponge used by every KT256 node, in bytes.
const KT256_RATE: usize = TURBOSHAKE256_RATE;

/// Domain-separation byte of a single (non-tree) node.
const SINGLE_PAD: u8 = 0x07;
/// Domain-separation byte of an intermediate (leaf) node.
const INTERMEDIATE_PAD: u8 = 0x0b;
/// Domain-separation byte of the final node.
const FINAL_PAD: u8 = 0x06;

/// Sakura hop marker that separates the first chunk from the chaining values.
const HOP: [u8; 8] = [0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
/// Sakura end marker of the final node.
const END: [u8; 2] = [0xff, 0xff];

/// Reader returned by [`Kt256::finalize_xof`].
///
/// Produces the KT256 output stream; [`squeeze`](Self::squeeze) may be called
/// any number of times.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Kt256Reader {
    inner: TurboShake256,
}

impl Kt256Reader {
    /// Squeezes the next `out.len()` bytes of the output stream.
    #[inline]
    pub fn squeeze(&mut self, out: &mut [u8]) {
        self.inner.squeeze(out);
    }
}

/// KangarooTwelve-256 (KT256), as defined in RFC 9861.
///
/// `customization` is the optional customization string `C`. It may be empty;
/// [`Default`] uses the empty string. The string is borrowed, not copied, and
/// is read when the hasher is finalized, so it must stay alive (and unchanged)
/// until then.
///
/// This type does not allocate and keeps no `N * 8192` staging buffer. A single
/// [`update`](Self::update) call spanning at least two whole chunks hashes them
/// in batched permutations (see the module-level documentation); a lone chunk
/// and the final partial chunk fall back to sequential TurboSHAKE256. How many
/// chunks share one permutation depends on the CPU, see
/// [`kt256_backend_name`].
///
/// # Example
///
/// ```ignore
/// use crypto::sha3::Kt256;
///
/// let mut hasher = Kt256::new(b"domain");
/// hasher.update(b"message");
/// let mut output = [0u8; 64];
/// hasher.finalize_into(&mut output);
/// ```
#[derive(Clone)]
pub struct Kt256<'a> {
    customization: &'a [u8],
    /// First chunk (`S_0`), later the final node. It starts with the
    /// single-node domain separator and is relabelled when the input grows past
    /// one chunk.
    first: TurboShake256,
    /// Number of bytes absorbed into `first`; at most `KT256_CHUNK_SIZE`.
    first_len: usize,
    /// Whether the input has grown beyond the first chunk.
    tree: bool,
    /// Current partial leaf chunk, if any. Created lazily once tree mode is
    /// entered, so single-node messages do not pay for zeroing it.
    queue: Option<TurboShake256>,
    queue_len: usize,
    /// Batched leaf sponge, created lazily in tree mode.
    leaves: Option<Leaves<12>>,
    /// Number of chaining values absorbed into `first`.
    chunks: usize,
}

impl<'a> Kt256<'a> {
    /// Creates a hasher with the given customization string `C`.
    ///
    /// The customization string is optional; pass `&[]` for the default.
    #[inline]
    pub fn new(customization: &'a [u8]) -> Self {
        return Kt256 {
            customization,
            first: TurboShake256::new(SINGLE_PAD),
            first_len: 0,
            tree: false,
            queue: None,
            queue_len: 0,
            leaves: None,
            chunks: 0,
        };
    }

    /// Hashes `message` with the customization string `customization`, filling
    /// `output`.
    ///
    /// `output` may have any length, including zero. As with every XOF, the
    /// output length does not change the hash: a shorter output is a prefix of
    /// a longer one.
    #[inline]
    pub fn hash(message: &[u8], customization: &[u8], output: &mut [u8]) {
        let mut hasher = Kt256::new(customization);
        hasher.update(message);
        hasher.finalize_into(output);
    }

    /// Absorbs message bytes.
    ///
    /// Chunks are taken straight from `data`; only the current partial chunk is
    /// buffered inside a sponge state.
    pub fn update(&mut self, mut data: &[u8]) {
        if data.is_empty() {
            return;
        }

        if !self.tree {
            let take = min(KT256_CHUNK_SIZE - self.first_len, data.len());
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

    /// Finalizes the hasher and writes `output.len()` bytes.
    ///
    /// The output length is arbitrary; see [`Self::finalize_xof`] for an
    /// incremental reader. `output` may be empty.
    pub fn finalize_into(self, output: &mut [u8]) {
        let mut final_node = self.finish();
        final_node.squeeze(output);
    }

    /// Finalizes the hasher and returns an incremental output reader.
    pub fn finalize_xof(self) -> Kt256Reader {
        return Kt256Reader {
            inner: self.finish(),
        };
    }

    /// Switches from the single-node layout to the tree layout.
    fn begin_tree(&mut self) {
        debug_assert_eq!(self.first_len, KT256_CHUNK_SIZE);
        debug_assert!(!self.tree);

        self.tree = true;
        self.first.set_domain_separator(FINAL_PAD);
        self.first.absorb(&HOP);
        self.leaves = Some(Leaves::new(KT256_RATE, INTERMEDIATE_PAD, Backend::detect()));
    }

    /// Absorbs bytes that belong to leaf chunks (`S_1` and later).
    fn absorb_leaves(&mut self, mut data: &[u8]) {
        // Finish the partial leaf chunk first.
        if self.queue_len > 0 {
            let take = min(KT256_CHUNK_SIZE - self.queue_len, data.len());
            self.queue
                .as_mut()
                .expect("the leaf queue exists while it holds bytes")
                .absorb(&data[..take]);
            self.queue_len += take;
            data = &data[take..];

            if self.queue_len == KT256_CHUNK_SIZE {
                self.flush_queue();
            }
        }

        // Whole batches, straight from the caller's slice.
        let width = self.leaves.as_ref().expect("tree mode initializes the leaves").width();
        while data.len() >= KT256_CHUNK_SIZE * width {
            let mut blocks: [&[u8]; MAX_BATCH] = [&[]; MAX_BATCH];
            let mut rest = data;
            for slot in blocks[..width].iter_mut() {
                let (block, tail) = rest.split_at(KT256_CHUNK_SIZE);
                *slot = block;
                rest = tail;
            }
            self.hash_batch(&blocks[..width], width);
            data = rest;
        }

        // Fill a partial batch of `2..width` whole chunks with the unused lanes
        // aliased to a real chunk. Only the first `whole` chaining values are
        // absorbed, so the dummy lanes cannot affect the output.
        let whole = data.len() / KT256_CHUNK_SIZE;
        if whole >= 2 {
            let mut blocks: [&[u8]; MAX_BATCH] = [&[]; MAX_BATCH];
            for (j, slot) in blocks[..width].iter_mut().enumerate() {
                let idx = j.min(whole - 1);
                *slot = &data[idx * KT256_CHUNK_SIZE..(idx + 1) * KT256_CHUNK_SIZE];
            }
            self.hash_batch(&blocks[..width], whole);
            data = &data[whole * KT256_CHUNK_SIZE..];
        }

        // At most one whole chunk remains; a wide permutation would not be
        // cheaper than one sequential TurboSHAKE256 call.
        if data.len() >= KT256_CHUNK_SIZE {
            let (block, rest) = data.split_at(KT256_CHUNK_SIZE);
            self.hash_one(block);
            data = rest;
        }

        // Buffer the partial tail inside the queue sponge.
        if !data.is_empty() {
            debug_assert_eq!(self.queue_len, 0);
            let queue = self.queue.get_or_insert_with(|| TurboShake256::new(INTERMEDIATE_PAD));
            queue.absorb(data);
            self.queue_len = data.len();
        }
    }

    /// Finalizes the tree and returns the final-node sponge.
    #[inline]
    fn finish(mut self) -> TurboShake256 {
        // `S = M || C || length_encode(|C|)`; the message was absorbed as it
        // arrived, so only the customization string and its length remain.
        self.update(self.customization);
        let encoded = length_encode(self.customization.len());
        self.update(encoded.as_ref());

        if self.tree {
            if self.queue_len > 0 {
                self.flush_queue();
            }

            let count = length_encode(self.chunks);
            self.first.absorb(count.as_ref());
            self.first.absorb(&END);
        }

        // `Kt256` may implement `Drop`, so move the finished sponge out instead
        // of returning a field.
        return core::mem::take(&mut self.first);
    }

    fn flush_queue(&mut self) {
        debug_assert!(self.queue_len > 0);

        let mut cv = [0u8; KT256_CV_LEN];
        self.queue
            .as_mut()
            .expect("the leaf queue holds the bytes being flushed")
            .squeeze(&mut cv);
        self.first.absorb(&cv);
        self.chunks += 1;

        self.queue = None;
        self.queue_len = 0;
    }

    fn hash_one(&mut self, chunk: &[u8]) {
        let mut cv = [0u8; KT256_CV_LEN];
        let mut leaf = TurboShake256::new(INTERMEDIATE_PAD);
        leaf.absorb(chunk);
        leaf.squeeze(&mut cv);

        self.first.absorb(&cv);
        self.chunks += 1;
    }

    fn hash_batch(&mut self, chunks: &[&[u8]], active: usize) {
        let mut cvs = [[0u8; KT256_CV_LEN]; MAX_BATCH];
        {
            let leaves = self.leaves.as_mut().expect("tree mode initializes the leaves");
            debug_assert_eq!(chunks.len(), leaves.width());
            debug_assert!((1..=chunks.len()).contains(&active));

            leaves.reset();
            leaves.absorb_batch(chunks);
            leaves.squeeze_cvs::<KT256_CV_LEN>(&mut cvs[..chunks.len()]);
        }

        for cv in &cvs[..active] {
            self.first.absorb(cv);
        }
        self.chunks += active;
    }
}

impl<'a> Default for Kt256<'a> {
    /// Creates a hasher with an empty customization string.
    fn default() -> Self {
        return Kt256::new(&[]);
    }
}

#[cfg(feature = "zeroize")]
impl zeroize::Zeroize for Kt256<'_> {
    fn zeroize(&mut self) {
        zeroize::Zeroize::zeroize(&mut self.first);
        if let Some(queue) = &mut self.queue {
            zeroize::Zeroize::zeroize(queue);
        }
        if let Some(leaves) = &mut self.leaves {
            zeroize::Zeroize::zeroize(leaves);
        }
        self.first_len = 0;
        self.queue_len = 0;
        self.chunks = 0;
        // `customization` is borrowed from the caller and must not be wiped.
    }
}

#[cfg(feature = "zeroize")]
impl zeroize::ZeroizeOnDrop for Kt256<'_> {}

#[cfg(feature = "zeroize")]
impl Drop for Kt256<'_> {
    fn drop(&mut self) {
        zeroize::Zeroize::zeroize(self);
    }
}

/// HopMAC256, as recommended by RFC 9861, section 4.
///
/// `HopMAC256(Key, M, C, L) = KT256(Key, KT256(M, C, 64), L)`: the message `M`
/// and customization string `C` are first compressed by a keyless KT256 call,
/// and the resulting 64-byte digest becomes the customization string of an
/// outer KT256 call whose message is `Key`.
///
/// `key` and `customization` are borrowed, not copied, and must stay alive and
/// unchanged until the hasher is finalized.
///
/// # One-shot API
///
/// ```ignore
/// use crypto::sha3::HopMac256;
///
/// let key = b"super secret key";
/// let mut tag = [0u8; 64];
/// HopMac256::mac(key, b"message", b"application", &mut tag);
/// ```
///
/// # Incremental API
///
/// ```ignore
/// use crypto::sha3::HopMac256;
///
/// let key = b"super secret key";
/// let mut mac = HopMac256::new(key, b"application");
/// mac.update(b"hello ");
/// mac.update(b"world");
/// let mut tag = [0u8; 64];
/// mac.finalize_into(&mut tag);
/// ```
///
/// # Output
///
/// Like KT256, the tag length is not bound into the computation: a shorter tag
/// is a prefix of a longer one for the same key, message, and customization
/// string. [`finalize_xof`](Self::finalize_xof) returns an incremental reader
/// for long or streamed output.
///
/// # Security
///
/// Unlike HMAC, the inner KT256 call is keyless, so only the outer call needs
/// protection against side-channel attacks when the key is secret.
#[derive(Clone)]
pub struct HopMac256<'a> {
    key: &'a [u8],
    inner: Kt256<'a>,
}

impl<'a> HopMac256<'a> {
    /// Creates a HopMAC256 hasher with the given `key` and customization
    /// string `C`.
    ///
    /// Pass `&[]` for `customization` when no domain separation is needed.
    #[inline]
    pub fn new(key: &'a [u8], customization: &'a [u8]) -> Self {
        return HopMac256 {
            key,
            inner: Kt256::new(customization),
        };
    }

    /// Absorbs message bytes.
    #[inline]
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Finalizes the MAC and writes `output.len()` bytes.
    ///
    /// `output` may have any length, including zero. As with every XOF, the
    /// output length does not change the tag: a shorter output is a prefix of
    /// a longer one. See [`Self::finalize_xof`] for an incremental reader.
    pub fn finalize_into(self, output: &mut [u8]) {
        let mut digest = [0u8; KT256_CV_LEN];
        let HopMac256 {
            key,
            inner,
        } = self;
        inner.finalize_into(&mut digest);
        Kt256::hash(key, &digest, output);
    }

    /// Finalizes the MAC and returns an incremental output reader.
    pub fn finalize_xof(self) -> Kt256Reader {
        let mut digest = [0u8; KT256_CV_LEN];
        let HopMac256 {
            key,
            inner,
        } = self;
        inner.finalize_into(&mut digest);

        // The reader owns the finalized sponge, so borrowing the local digest
        // as the outer customization string is safe.
        let mut outer = Kt256::new(&digest);
        outer.update(key);
        return outer.finalize_xof();
    }

    /// Computes `HopMAC256(key, message, customization, output)` in one call.
    ///
    /// `output` may have any length, including zero.
    pub fn mac(key: &[u8], message: &[u8], customization: &[u8], output: &mut [u8]) {
        let mut hasher = HopMac256::new(key, customization);
        hasher.update(message);
        hasher.finalize_into(output);
    }
}

#[cfg(feature = "zeroize")]
impl zeroize::Zeroize for HopMac256<'_> {
    fn zeroize(&mut self) {
        zeroize::Zeroize::zeroize(&mut self.inner);
        // `key` is borrowed from the caller and must not be wiped.
    }
}

/// Returns the name of the batched Keccak backend KT256 uses on this CPU.
///
/// Useful in benchmarks and CI logs. Detection is cached by the standard
/// library, so calling this repeatedly is cheap.
///
/// ```
/// use crypto::sha3::kt256_backend_name;
/// println!("KT256 backend: {}", kt256_backend_name());
/// ```
pub fn kt256_backend_name() -> &'static str {
    return Backend::detect().name();
}

/// RFC 9861 `length_encode`: the minimal big-endian bytes of `x` followed by
/// the number of those bytes. In particular `length_encode(0) == [0x00]`.
#[inline]
pub(crate) fn length_encode(mut x: usize) -> Bytes<9> {
    let mut digits = [0u8; 8];
    let mut n = 0;

    while x > 0 {
        digits[digits.len() - 1 - n] = (x & 0xff) as u8;
        x >>= 8;
        n += 1;
    }

    let mut out = Bytes::<9>::new();
    out.append(&digits[digits.len() - n..]);
    out.push(n as u8);
    return out;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RFC 9861, section 5: `ptn(n)` repeats `00 01 02 .. F9 FA` (251-byte
    /// cycle) as many times as needed, truncated to `n` bytes.
    fn ptn(n: usize) -> Vec<u8> {
        return (0..n).map(|i| (i % 251) as u8).collect();
    }

    #[test]
    fn length_encode_matches_rfc_examples() {
        assert_eq!(length_encode(0).as_ref(), &[0x00]);
        assert_eq!(length_encode(12).as_ref(), &[0x0c, 0x01]);
        assert_eq!(length_encode(255).as_ref(), &[0xff, 0x01]);
        assert_eq!(length_encode(256).as_ref(), &[0x01, 0x00, 0x02]);
        assert_eq!(length_encode(65538).as_ref(), &[0x01, 0x00, 0x02, 0x03]);
        assert_eq!(length_encode(usize::MAX).as_ref().len(), usize::BITS as usize / 8 + 1);
    }

    #[test]
    fn default_uses_empty_customization() {
        let mut from_default = [0u8; 64];
        let mut hasher = Kt256::default();
        hasher.update(b"default");
        hasher.finalize_into(&mut from_default);

        let mut explicit = [0u8; 64];
        Kt256::hash(b"default", &[], &mut explicit);
        assert_eq!(from_default, explicit);
    }

    /// Known-answer vectors for HopMAC256.
    ///
    /// RFC 9861 defines HopMAC but does not publish vectors for it. These tags
    /// were produced with XKCP/K12's independent Python KT256 implementation,
    /// composed as `KT256(Key, KT256(M, C, 64), L)`, and cross-checked against
    /// this module's `Kt256`.
    #[test]
    fn hopmac256_known_answer_vectors() {
        struct Vector {
            key: Vec<u8>,
            message: Vec<u8>,
            customization: Vec<u8>,
            output_len: usize,
            expected: &'static str,
        }

        let vectors = [
            Vector {
                key: ptn(32),
                message: vec![],
                customization: vec![],
                output_len: 32,
                expected: "137d48ca2fc05854533ddfce3c5e0acb\
                           1888488f651cbe5940b0f1e87321c52c",
            },
            Vector {
                key: ptn(32),
                message: vec![],
                customization: vec![],
                output_len: 64,
                expected: "137d48ca2fc05854533ddfce3c5e0acb\
                           1888488f651cbe5940b0f1e87321c52c\
                           85f7ce0a325d9864f75fa9d07dcaa834\
                           642fe922c504856a92049f3790508bb1",
            },
            Vector {
                key: ptn(32),
                message: ptn(17),
                customization: vec![],
                output_len: 64,
                expected: "398819a1b1a2bc5d809713fa22c21284\
                           e360562a6ee17840816e66be84bc7dbd\
                           d028886d493a57a33f38d6596afc5cf5\
                           7ad7224650374af90d9fff65506af127",
            },
            Vector {
                key: ptn(32),
                message: vec![],
                customization: ptn(1),
                output_len: 64,
                expected: "58a613d2dd1638a04cd8fd62b198a794\
                           9531310eb51bab6cc7ac710d537618bc\
                           df4bc9b2470f10378cf491922871c7ee\
                           b16ea48103925faf40abdbbac7fa06a9",
            },
            Vector {
                key: ptn(32),
                message: ptn(8191),
                customization: vec![],
                output_len: 64,
                expected: "fba827592baf98f827290d970299f3ee\
                           26f4eac7287b2a9fb222eb15ec0a04d8\
                           c7b347da65ead88064b92d86c69ec16c\
                           529aa6ff53ad2aca3b28512005d05e86",
            },
            Vector {
                key: ptn(32),
                message: ptn(8192),
                customization: vec![],
                output_len: 64,
                expected: "55bfce4e56df0f1c8289793c69205f71\
                           fed60653d81daf4ff9e54ce36b45b26f\
                           d86c40691e39cc6f6ad5fcfc6ebbe69b\
                           93d42afbe0b85bfe6064bea71ee14279",
            },
            Vector {
                key: ptn(32),
                message: ptn(8192),
                customization: ptn(8189),
                output_len: 64,
                expected: "0f76ea0a8b1d77c67785473e501acf80\
                           a6e3eb0c406943863c47f5cb23146112\
                           17b45144f1a1d697603d695d016ce46e\
                           bb31986495b75540ec00a144c888bb4d",
            },
            Vector {
                key: ptn(32),
                message: ptn(8192),
                customization: ptn(8190),
                output_len: 64,
                expected: "80b774368a06069cf715dbdec65ef9ed\
                           c27c467ca8efeb15d5facb89027b3002\
                           0c1ed1af1d7b46daf9066cffbccda4b2\
                           f4872cca204a5b5899d3c2d365fe8bd0",
            },
            Vector {
                key: ptn(131),
                message: ptn(17),
                customization: ptn(41),
                output_len: 64,
                expected: "90f208c103e2b2445be8bae68b8ecc2e\
                           44af85fb30ede8cc9d0c6c407cafe25b\
                           867d1c34ffd92d220e4058e11e90fabd\
                           9553ca84938b7caef41ee818d210fcbf",
            },
            Vector {
                key: ptn(200),
                message: ptn(17usize.pow(3)),
                customization: ptn(41usize.pow(2)),
                output_len: 64,
                expected: "bfaea92418ed6b3c2d14ae2a518f9703\
                           d20170cabd5b290829385a469510340f\
                           a4dcad1383c587f29eefe12e402bc2a2\
                           085cfc5c4327c4eb5d6ec554257a30c5",
            },
        ];

        for (i, vector) in vectors.iter().enumerate() {
            let mut output = vec![0u8; vector.output_len];
            HopMac256::mac(&vector.key, &vector.message, &vector.customization, &mut output);
            assert_eq!(
                hex::encode(&output),
                vector.expected.replace(['\n', ' '], ""),
                "vector {i}: key {}, message {}, customization {}",
                vector.key.len(),
                vector.message.len(),
                vector.customization.len()
            );
        }
    }

    #[test]
    fn hopmac256_long_output_is_a_prefix_stream() {
        const OUTPUT_LEN: usize = 10064;
        let key = ptn(32);

        let mut output = vec![0u8; OUTPUT_LEN];
        HopMac256::mac(&key, &[], &[], &mut output);
        assert_eq!(
            hex::encode(&output[OUTPUT_LEN - 64..]),
            "92122d35b17f6ade0790f24437ddc1c3\
             eee008b7f736331ef8fc259574b3f513\
             70f9d8db8baa2fc8231aec4805be00cb\
             dc9951043ae80aab609a70b1368ed622"
        );

        let mut short = [0u8; 32];
        HopMac256::mac(&key, &[], &[], &mut short);
        assert_eq!(short, output[..32]);
    }

    #[test]
    fn hopmac256_matches_spec_composition() {
        let cases = [
            (0usize, 0usize, 0usize, 32usize),
            (1, 1, 1, 64),
            (69, 8191, 0, 64),
            (131, 8192, 41, 64),
            (200, 16385, 8189, 32),
        ];

        for (key_len, message_len, customization_len, output_len) in cases {
            let key = ptn(key_len);
            let message = ptn(message_len);
            let customization = ptn(customization_len);

            let inner = spec_kt256(&message, &customization, KT256_CV_LEN);
            let expected = spec_kt256(&key, &inner, output_len);

            let mut output = vec![0u8; output_len];
            HopMac256::mac(&key, &message, &customization, &mut output);
            assert_eq!(
                output, expected,
                "key length {key_len}, message length {message_len}, \
                 customization length {customization_len}"
            );
        }
    }

    #[test]
    fn hopmac256_streaming_matches_one_shot() {
        let key = ptn(32);
        let customization = ptn(41);

        for &len in &[0usize, 1, 8191, 8192, 8193, 2 * 8192, 2 * 8192 + 1] {
            let data = ptn(len);

            let mut one_shot = [0u8; 64];
            HopMac256::mac(&key, &data, &customization, &mut one_shot);

            for &chunk_size in &[1usize, 3, 7, 136, 8192, 2 * 8192 + 1] {
                let mut hasher = HopMac256::new(&key, &customization);
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
    fn hopmac256_xof_squeeze_is_chunk_independent() {
        let key = ptn(32);
        let customization = ptn(5);
        let data = ptn(2 * KT256_CHUNK_SIZE + 11);

        let mut one_shot = [0u8; 1000];
        HopMac256::mac(&key, &data, &customization, &mut one_shot);

        let mut hasher = HopMac256::new(&key, &customization);
        hasher.update(&data);
        let mut reader = hasher.finalize_xof();

        let mut streamed = [0u8; 1000];
        let mut offset = 0;
        for chunk in [1usize, 7, 63, 135, 137, 1, 500] {
            let n = chunk.min(streamed.len() - offset);
            reader.squeeze(&mut streamed[offset..offset + n]);
            offset += n;
        }
        reader.squeeze(&mut streamed[offset..]);

        assert_eq!(one_shot, streamed);
    }

    #[test]
    fn hopmac256_clone_mid_stream_matches_linear() {
        let key = ptn(32);
        let customization = ptn(13);
        let data = ptn(3 * KT256_CHUNK_SIZE + 7);

        let mut expected = [0u8; 64];
        HopMac256::mac(&key, &data, &customization, &mut expected);

        for split in [0usize, 8191, 8192, 8193, 2 * 8192 + 5] {
            let mut hasher = HopMac256::new(&key, &customization);
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
    fn hopmac256_binds_key_and_customization() {
        let message = ptn(3 * KT256_CHUNK_SIZE + 1);

        let mut a = [0u8; 64];
        let mut b = [0u8; 64];
        let mut c = [0u8; 64];
        HopMac256::mac(b"key one", &message, b"custom", &mut a);
        HopMac256::mac(b"key two", &message, b"custom", &mut b);
        HopMac256::mac(b"key one", &message, b"other", &mut c);

        assert_ne!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn hopmac256_accepts_empty_key_and_empty_output() {
        let mut empty_output = [0u8; 0];
        HopMac256::mac(&[], b"data", b"", &mut empty_output);

        let mut output = [0u8; 32];
        HopMac256::mac(&[], b"data", b"", &mut output);
        assert_ne!(output, [0u8; 32]);
    }

    #[test]
    fn empty_input_matches_rfc() {
        let mut out = [0u8; 64];
        Kt256::hash(&[], &[], &mut out);
        assert_eq!(
            hex::encode(out),
            "b23d2e9cea9f4904e02bec06817fc10c\
             e38ce8e93ef4c89e6537076af8646404\
             e3e8b68107b8833a5d30490aa3348235\
             3fd4adc7148ecb782855003aaebde4a9"
        );
    }

    #[test]
    fn prefix_property_and_long_output() {
        let mut short = [0u8; 64];
        let mut long = [0u8; 128];
        Kt256::hash(&[], &[], &mut short);
        Kt256::hash(&[], &[], &mut long);
        assert_eq!(&long[..64], &short);

        const OUTPUT_LEN: usize = 10064;
        let mut output = vec![0u8; OUTPUT_LEN];
        Kt256::hash(&[], &[], &mut output);
        assert_eq!(&output[..64], &short);
        assert_eq!(
            hex::encode(&output[OUTPUT_LEN - 64..]),
            "ad4a1d718cf950506709a4c33396139b\
             4449041fc79a05d68da35f1e453522e0\
             56c64fe94958e7085f2964888259b993\
             2752f3ccd855288efee5fcbb8b563069"
        );
    }

    #[test]
    fn rfc_short_message_vectors() {
        struct Vector {
            message: Vec<u8>,
            customization: Vec<u8>,
            expected: &'static str,
        }

        let vectors = [
            Vector {
                message: ptn(1),
                customization: vec![],
                expected: "0d005a194085360217128cf17f91e1f7\
                           1314efa5564539d444912e3437efa17f\
                           82db6f6ffe76e781eaa068bce01f2bbf\
                           81eacb983d7230f2fb02834a21b1ddd0",
            },
            Vector {
                message: ptn(17),
                customization: vec![],
                expected: "1ba3c02b1fc514474f06c8979978a905\
                           6c8483f4a1b63d0dccefe3a28a2f323e\
                           1cdcca40ebf006ac76ef039715234683\
                           7b1277d3e7faa9c9653b19075098527b",
            },
            Vector {
                message: ptn(17usize.pow(2)),
                customization: vec![],
                expected: "de8ccbc63e0f133ebb4416814d4c66f6\
                           91bbf8b6a61ec0a7700f836b086cb029\
                           d54f12ac7159472c72db118c35b4e6aa\
                           213c6562caaa9dcc518959e69b10f3ba",
            },
            Vector {
                message: ptn(17usize.pow(3)),
                customization: vec![],
                expected: "647efb49fe9d717500171b41e7f11bd4\
                           91544443209997ce1c2530d15eb1ffbb\
                           598935ef954528ffc152b1e4d731ee26\
                           83680674365cd191d562bae753b84aa5",
            },
            Vector {
                message: ptn(17usize.pow(4)),
                customization: vec![],
                expected: "b06275d284cd1cf205bcbe57dccd3ec1\
                           ff6686e3ed15776383e1f2fa3c6ac8f0\
                           8bf8a162829db1a44b2a43ff83dd89c3\
                           cf1ceb61ede659766d5ccf817a62ba8d",
            },
            Vector {
                message: ptn(17usize.pow(5)),
                customization: vec![],
                expected: "9473831d76a4c7bf77ace45b59f1458b\
                           1673d64bcd877a7c66b2664aa6dd149e\
                           60eab71b5c2bab858c074ded81ddce2b\
                           4022b5215935c0d4d19bf511aeeb0772",
            },
            Vector {
                message: ptn(17usize.pow(6)),
                customization: vec![],
                expected: "0652b740d78c5e1f7c8dcc1777097382\
                           768b7ff38f9a7a20f29f413bb1b3045b\
                           31a5578f568f911e09cf44746da84224\
                           a5266e96a4a535e871324e4f9c7004da",
            },
            Vector {
                message: vec![],
                customization: ptn(1),
                expected: "9280f5cc39b54a5a594ec63de0bb9937\
                           1e4609d44bf845c2f5b8c316d72b1598\
                           11f748f23e3fabbe5c3226ec96c62186\
                           df2d33e9df74c5069ceecbb4dd10eff6",
            },
            Vector {
                message: vec![0xff],
                customization: ptn(41),
                expected: "47ef96dd616f200937aa7847e34ec2fe\
                           ae8087e3761dc0f8c1a154f51dc9ccf8\
                           45d7adbce57ff64b639722c6a1672e3b\
                           f5372d87e00aff89be97240756998853",
            },
            Vector {
                message: vec![0xff; 3],
                customization: ptn(41usize.pow(2)),
                expected: "3b48667a5051c5966c53c5d42b95de45\
                           1e05584e7806e2fb765eda959074172c\
                           b438a9e91dde337c98e9c41bed94c4e0\
                           aef431d0b64ef2324f7932caa6f54969",
            },
            Vector {
                message: vec![0xff; 7],
                customization: ptn(41usize.pow(3)),
                expected: "e0911cc00025e1540831e266d94add9b\
                           98712142b80d2629e643aac4efaf5a3a\
                           30a88cbf4ac2a91a2432743054fbcc98\
                           97670e86ba8cec2fc2ace9c966369724",
            },
            Vector {
                message: ptn(8191),
                customization: vec![],
                expected: "3081434d93a4108d8d8a3305b89682ce\
                           bedc7ca4ea8a3ce869fbb73cbe4a58ee\
                           f6f24de38ffc170514c70e7ab2d01f03\
                           812616e863d769afb3753193ba045b20",
            },
            Vector {
                message: ptn(8192),
                customization: vec![],
                expected: "c6ee8e2ad3200c018ac87aaa031cdac2\
                           2121b412d07dc6e0dccbb53423747e9a\
                           1c18834d99df596cf0cf4b8dfafb7bf0\
                           2d139d0c9035725adc1a01b7230a41fa",
            },
            Vector {
                message: ptn(8192),
                customization: ptn(8189),
                expected: "74e47879f10a9c5d11bd2da7e194fe57\
                           e86378bf3c3f7448eff3c576a0f18c5c\
                           aae0999979512090a7f348af4260d4de\
                           3c37f1ecaf8d2c2c96c1d16c64b12496",
            },
            Vector {
                message: ptn(8192),
                customization: ptn(8190),
                expected: "f4b5908b929ffe01e0f79ec2f21243d4\
                           1a396b2e7303a6af1d6399cd6c7a0a2d\
                           d7c4f607e8277f9c9b1cb4ab9ddc59d4\
                           b92d1fc7558441f1832c3279a4241b8b",
            },
        ];

        for (i, vector) in vectors.iter().enumerate() {
            let mut output = [0u8; 64];
            Kt256::hash(&vector.message, &vector.customization, &mut output);
            assert_eq!(
                hex::encode(output),
                vector.expected.replace(['\n', ' '], ""),
                "vector {i}: message length {}, customization length {}",
                vector.message.len(),
                vector.customization.len()
            );
        }
    }

    /// Slow, literal transcription of RFC 9861, appendix A.5.
    fn spec_kt256(message: &[u8], customization: &[u8], out_len: usize) -> Vec<u8> {
        let mut s = message.to_vec();
        s.extend_from_slice(customization);
        s.extend_from_slice(length_encode(customization.len()).as_ref());

        if s.len() <= KT256_CHUNK_SIZE {
            let mut out = vec![0u8; out_len];
            crate::sha3::keccak::test_util::sponge::<12>(&s, KT256_RATE, SINGLE_PAD, &mut out);
            return out;
        }

        let mut final_node = s[..KT256_CHUNK_SIZE].to_vec();
        final_node.extend_from_slice(&HOP);

        let mut chunks = 0;
        for chunk in s[KT256_CHUNK_SIZE..].chunks(KT256_CHUNK_SIZE) {
            let mut cv = [0u8; KT256_CV_LEN];
            crate::sha3::keccak::test_util::sponge::<12>(chunk, KT256_RATE, INTERMEDIATE_PAD, &mut cv);
            final_node.extend_from_slice(&cv);
            chunks += 1;
        }

        final_node.extend_from_slice(length_encode(chunks).as_ref());
        final_node.extend_from_slice(&END);

        let mut out = vec![0u8; out_len];
        crate::sha3::keccak::test_util::sponge::<12>(&final_node, KT256_RATE, FINAL_PAD, &mut out);
        return out;
    }

    #[test]
    fn matches_spec_around_chunk_boundaries() {
        let widths = Leaves::<12>::new(KT256_RATE, INTERMEDIATE_PAD, Backend::detect()).width();
        let lengths = [
            0,
            1,
            135,
            136,
            137,
            8191,
            8192,
            8193,
            16383,
            16384,
            16385,
            widths * KT256_CHUNK_SIZE - 1,
            widths * KT256_CHUNK_SIZE,
            widths * KT256_CHUNK_SIZE + 1,
        ];

        for len in lengths {
            for customization_len in [0usize, 1, 3, 41, 8189, 8190, 8191, 8192, 20000] {
                let message = ptn(len);
                let customization = ptn(customization_len);
                let expected = spec_kt256(&message, &customization, 64);
                let mut output = [0u8; 64];
                Kt256::hash(&message, &customization, &mut output);
                assert_eq!(
                    output,
                    expected[..],
                    "message length {len}, customization length {customization_len}"
                );
            }
        }
    }

    #[test]
    fn streaming_matches_one_shot() {
        let batch = Leaves::<12>::new(KT256_RATE, INTERMEDIATE_PAD, Backend::detect()).width();

        for &len in &[
            0usize,
            1,
            8191,
            8192,
            8193,
            2 * 8192,
            2 * 8192 + 1,
            batch * KT256_CHUNK_SIZE,
            batch * KT256_CHUNK_SIZE + 7,
        ] {
            let data = ptn(len);
            let customization = ptn(41);

            let mut one_shot = [0u8; 64];
            Kt256::hash(&data, &customization, &mut one_shot);

            for &chunk_size in &[1usize, 3, 7, 136, 8192, 2 * 8192 + 1, batch * 8192 + 1] {
                let mut hasher = Kt256::new(&customization);
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
        let data = ptn(3 * KT256_CHUNK_SIZE + 17);
        let customization = ptn(7);

        let mut batched = [0u8; 64];
        Kt256::hash(&data, &customization, &mut batched);

        // Feeding one chunk at a time never leaves two whole chunks in a single
        // update, so the leaves are hashed sequentially.
        let mut sequential = [0u8; 64];
        let mut hasher = Kt256::new(&customization);
        for chunk in data.chunks(KT256_CHUNK_SIZE) {
            hasher.update(chunk);
        }
        hasher.finalize_into(&mut sequential);

        assert_eq!(batched, sequential);
    }

    #[test]
    fn xof_squeeze_is_chunk_independent() {
        let data = ptn(2 * KT256_CHUNK_SIZE + 11);
        let customization = ptn(5);

        let mut one_shot = [0u8; 1000];
        Kt256::hash(&data, &customization, &mut one_shot);

        let mut hasher = Kt256::new(&customization);
        hasher.update(&data);
        let mut reader = hasher.finalize_xof();

        let mut streamed = [0u8; 1000];
        let mut offset = 0;
        for chunk in [1usize, 7, 63, 135, 137, 1, 500] {
            let n = chunk.min(streamed.len() - offset);
            reader.squeeze(&mut streamed[offset..offset + n]);
            offset += n;
        }
        reader.squeeze(&mut streamed[offset..]);

        assert_eq!(one_shot, streamed);
    }

    #[test]
    fn customization_is_bound_into_the_hash() {
        let data = ptn(3 * KT256_CHUNK_SIZE + 1);
        let mut a = [0u8; 64];
        let mut b = [0u8; 64];
        Kt256::hash(&data, b"one", &mut a);
        Kt256::hash(&data, b"two", &mut b);
        assert_ne!(a, b);
    }

    #[test]
    fn zero_length_output_is_allowed() {
        let mut empty = [0u8; 0];
        Kt256::hash(b"data", b"", &mut empty);

        let mut one = [0u8; 1];
        Kt256::hash(b"data", b"", &mut one);
        assert_ne!(one, [0u8]);
    }

    #[test]
    fn clone_mid_stream_matches_linear() {
        let data = ptn(3 * KT256_CHUNK_SIZE + 7);
        let customization = ptn(13);

        let mut expected = [0u8; 64];
        Kt256::hash(&data, &customization, &mut expected);

        for split in [0usize, 8191, 8192, 8193, 2 * 8192 + 5] {
            let mut hasher = Kt256::new(&customization);
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

    /// Relabelling the first chunk from `0x07` to `0x06` before finalization
    /// must be equivalent to absorbing the same bytes with `0x06` from the
    /// start.
    #[test]
    fn domain_separator_switch_matches_fresh_sponge() {
        let data = ptn(KT256_CHUNK_SIZE);

        let mut switched = TurboShake256::new(SINGLE_PAD);
        switched.absorb(&data);
        switched.set_domain_separator(FINAL_PAD);
        switched.absorb(&HOP);

        let mut fresh = TurboShake256::new(FINAL_PAD);
        fresh.absorb(&data);
        fresh.absorb(&HOP);

        let mut a = [0u8; 64];
        let mut b = [0u8; 64];
        switched.squeeze(&mut a);
        fresh.squeeze(&mut b);
        assert_eq!(a, b);
    }
}
