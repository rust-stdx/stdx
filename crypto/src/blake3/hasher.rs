//! The BLAKE3 hashing engine: chunk state, subtree (CV) stack, tree logic,
//! extendable output, and the three modes (hash, keyed hash, derive key).
//!
//! This implementation is based on the BLAKE3 specification (see `blake3.pdf` in docs folder),
//! including the "lazy merging" CV-stack strategy and the wide subtree compression that lets the
//! SIMD kernels hash several chunks at once.

use core::cmp;

use super::platform::{
    BLOCK_LEN, CHUNK_END, CHUNK_LEN, CHUNK_START, DERIVE_KEY_CONTEXT, DERIVE_KEY_MATERIAL, IV, IncrementCounter,
    KEY_LEN, KEYED_HASH, MAX_DEPTH, MAX_SIMD_DEGREE, MAX_SIMD_DEGREE_OR_2, OUT_LEN, PARENT, Platform, ROOT,
};

#[inline(always)]
fn words_from_le_bytes_32(bytes: &[u8; KEY_LEN]) -> [u32; 8] {
    let mut words = [0u32; 8];
    for (word, chunk) in words.iter_mut().zip(bytes.as_chunks::<4>().0) {
        *word = u32::from_le_bytes(*chunk);
    }
    words
}

#[inline(always)]
fn le_bytes_from_words_32(words: &[u32; 8]) -> [u8; OUT_LEN] {
    let mut out = [0u8; OUT_LEN];
    for (word, chunk) in words.iter().zip(out.as_chunks_mut::<4>().0) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
    out
}

/// A 64-byte block buffer aligned to a cache line.
#[derive(Clone, Copy)]
#[cfg_attr(target_pointer_width = "64", repr(align(64)))]
#[cfg_attr(target_pointer_width = "32", repr(align(32)))]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
pub(super) struct Aligned64([u8; BLOCK_LEN]);

/// The state of the chunk currently being hashed.
///
/// The final block of a chunk is only compressed once the caller has supplied
/// at least one byte of the *next* block, because until then it might turn out
/// to be the last block of the whole input and would need the `CHUNK_END` (and
/// possibly `ROOT`) flag.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
struct ChunkState {
    cv: [u32; 8],
    chunk_counter: u64,
    buf: Aligned64,
    buf_len: u8,
    blocks_compressed: u8,
    flags: u8,
    platform: Platform,
}

impl ChunkState {
    fn new(key: &[u32; 8], chunk_counter: u64, flags: u8, platform: Platform) -> Self {
        Self {
            cv: *key,
            chunk_counter,
            buf: Aligned64([0; BLOCK_LEN]),
            buf_len: 0,
            blocks_compressed: 0,
            flags,
            platform,
        }
    }

    fn count(&self) -> usize {
        BLOCK_LEN * self.blocks_compressed as usize + self.buf_len as usize
    }

    fn fill_buf(&mut self, input: &mut &[u8]) {
        let want = BLOCK_LEN - self.buf_len as usize;
        let take = cmp::min(want, input.len());
        self.buf.0[self.buf_len as usize..][..take].copy_from_slice(&input[..take]);
        self.buf_len += take as u8;
        *input = &input[take..];
    }

    fn start_flag(&self) -> u8 {
        if self.blocks_compressed == 0 { CHUNK_START } else { 0 }
    }

    /// Absorb input, compressing full blocks directly from the slice.
    fn update(&mut self, mut input: &[u8]) {
        if self.buf_len > 0 {
            self.fill_buf(&mut input);
            if !input.is_empty() {
                debug_assert_eq!(self.buf_len as usize, BLOCK_LEN);
                let block_flags = self.flags | self.start_flag();
                self.platform.compress_in_place(
                    &mut self.cv,
                    &self.buf.0,
                    BLOCK_LEN as u8,
                    self.chunk_counter,
                    block_flags,
                );
                self.buf_len = 0;
                self.buf = Aligned64([0; BLOCK_LEN]);
                self.blocks_compressed += 1;
            }
        }

        while input.len() > BLOCK_LEN {
            debug_assert_eq!(self.buf_len, 0);
            let block_flags = self.flags | self.start_flag();
            let block: &[u8; BLOCK_LEN] = input[..BLOCK_LEN].try_into().unwrap();
            self.platform
                .compress_in_place(&mut self.cv, block, BLOCK_LEN as u8, self.chunk_counter, block_flags);
            self.blocks_compressed += 1;
            input = &input[BLOCK_LEN..];
        }

        self.fill_buf(&mut input);
        debug_assert!(input.is_empty());
        debug_assert!(self.count() <= CHUNK_LEN);
    }

    fn output(&self) -> Output {
        let block_flags = self.flags | self.start_flag() | CHUNK_END;
        Output {
            input_chaining_value: self.cv,
            block: self.buf,
            block_len: self.buf_len,
            counter: self.chunk_counter,
            flags: block_flags,
            platform: self.platform,
        }
    }

    /// Test-only: whether the fields selected for zeroization have been cleared.
    #[cfg(all(test, feature = "zeroize"))]
    fn debug_is_zeroized(&self) -> bool {
        self.cv == [0u32; 8]
            && self.chunk_counter == 0
            && self.buf.0 == [0u8; BLOCK_LEN]
            && self.buf_len == 0
            && self.blocks_compressed == 0
            && self.flags == 0
    }
}

/// The state just before choosing between a chaining value and final output
/// bytes. Keeping this around is what makes the `ROOT` flag and extendable
/// output work.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
struct Output {
    input_chaining_value: [u32; 8],
    block: Aligned64,
    block_len: u8,
    counter: u64,
    flags: u8,
    platform: Platform,
}

impl Output {
    fn chaining_value(&self) -> [u8; OUT_LEN] {
        let mut cv = self.input_chaining_value;
        self.platform
            .compress_in_place(&mut cv, &self.block.0, self.block_len, self.counter, self.flags);
        le_bytes_from_words_32(&cv)
    }

    fn root_hash(&self) -> [u8; OUT_LEN] {
        debug_assert_eq!(self.counter, 0, "the root node always has counter 0");
        let mut cv = self.input_chaining_value;
        self.platform
            .compress_in_place(&mut cv, &self.block.0, self.block_len, 0, self.flags | ROOT);
        le_bytes_from_words_32(&cv)
    }

    fn root_output_block(&self) -> [u8; 2 * OUT_LEN] {
        self.platform.compress_xof(
            &self.input_chaining_value,
            &self.block.0,
            self.block_len,
            self.counter,
            self.flags | ROOT,
        )
    }

    /// Write several consecutive root output blocks at once.
    ///
    /// `out` must be a whole number of 64-byte blocks; an empty slice is a
    /// no-op. Block `i` is produced with counter `self.counter + i`. This is
    /// the wide counterpart of [`Output::root_output_block`], letting the SIMD
    /// backends compress several output blocks in parallel.
    fn output_many(&self, out: &mut [u8]) {
        debug_assert_eq!(out.len() % BLOCK_LEN, 0, "whole output blocks only");
        self.platform.xof_many(
            &self.input_chaining_value,
            &self.block.0,
            self.block_len,
            self.counter,
            self.flags | ROOT,
            out,
        );
    }

    /// Test-only: whether the fields selected for zeroization have been cleared.
    #[cfg(all(test, feature = "zeroize"))]
    fn debug_is_zeroized(&self) -> bool {
        self.input_chaining_value == [0u32; 8]
            && self.block.0 == [0u8; BLOCK_LEN]
            && self.block_len == 0
            && self.counter == 0
            && self.flags == 0
    }
}

fn parent_node_output(
    left_child: &[u8; OUT_LEN],
    right_child: &[u8; OUT_LEN],
    key: &[u32; 8],
    flags: u8,
    platform: Platform,
) -> Output {
    let mut block = Aligned64([0; BLOCK_LEN]);
    block.0[..OUT_LEN].copy_from_slice(left_child);
    block.0[OUT_LEN..].copy_from_slice(right_child);
    Output {
        input_chaining_value: *key,
        block,
        block_len: BLOCK_LEN as u8,
        counter: 0, // Always 0 for parent nodes.
        flags: flags | PARENT,
        platform,
    }
}

/// The largest power of two less than or equal to `n`.
///
/// `n == 0` maps to `1` (used by `Hasher::update`, which never calls it with
/// an input that cannot hold at least one whole chunk).
#[inline]
pub(super) fn largest_power_of_two_leq(n: usize) -> usize {
    ((n / 2) + 1).next_power_of_two()
}

/// Hash up to `MAX_SIMD_DEGREE` whole chunks in parallel, writing one chaining
/// value per chunk and returning how many were written.
///
/// The chunks are never the root and never empty; those cases use a different
/// code path.
fn compress_chunks_parallel(
    input: &[u8],
    key: &[u32; 8],
    chunk_counter: u64,
    flags: u8,
    platform: Platform,
    out: &mut [u8],
) -> usize {
    debug_assert!(!input.is_empty(), "empty chunks below the root");
    debug_assert!(input.len() <= MAX_SIMD_DEGREE * CHUNK_LEN);

    let (chunks, remainder) = input.as_chunks::<CHUNK_LEN>();
    let mut chunks_array: [&[u8]; MAX_SIMD_DEGREE] = [&[] as &[u8]; MAX_SIMD_DEGREE];
    let mut num_chunks = 0;
    for chunk in chunks {
        chunks_array[num_chunks] = chunk;
        num_chunks += 1;
    }
    platform.hash_many(
        &chunks_array[..num_chunks],
        key,
        chunk_counter,
        IncrementCounter::Yes,
        flags,
        CHUNK_START,
        CHUNK_END,
        out,
    );

    // Hash the trailing partial chunk, if any. (The empty input is a different
    // code path.)
    if !remainder.is_empty() {
        let counter = chunk_counter.wrapping_add(num_chunks as u64);
        let mut chunk_state = ChunkState::new(key, counter, flags, platform);
        chunk_state.update(remainder);
        out[num_chunks * OUT_LEN..][..OUT_LEN].copy_from_slice(&chunk_state.output().chaining_value());
        num_chunks + 1
    } else {
        num_chunks
    }
}

/// Hash up to `MAX_SIMD_DEGREE_OR_2` parent nodes in parallel, writing one
/// chaining value per pair of children and returning how many were written. A
/// leftover odd child is passed through unchanged.
fn compress_parents_parallel(
    child_chaining_values: &[u8],
    key: &[u32; 8],
    flags: u8,
    platform: Platform,
    out: &mut [u8],
) -> usize {
    debug_assert_eq!(child_chaining_values.len() % OUT_LEN, 0, "wacky hash bytes");
    let num_children = child_chaining_values.len() / OUT_LEN;
    debug_assert!(num_children >= 2, "not enough children");
    debug_assert!(num_children <= 2 * MAX_SIMD_DEGREE_OR_2, "too many");

    let (parents, remainder) = child_chaining_values.as_chunks::<BLOCK_LEN>();
    let mut parents_array: [&[u8]; MAX_SIMD_DEGREE_OR_2] = [&[] as &[u8]; MAX_SIMD_DEGREE_OR_2];
    let mut num_parents = 0;
    for parent in parents {
        parents_array[num_parents] = parent;
        num_parents += 1;
    }
    platform.hash_many(
        &parents_array[..num_parents],
        key,
        0, // Parents always use counter 0.
        IncrementCounter::No,
        flags | PARENT,
        0, // Parents have no start flag.
        0, // Parents have no end flag.
        out,
    );

    if !remainder.is_empty() {
        out[num_parents * OUT_LEN..][..OUT_LEN].copy_from_slice(remainder);
        num_parents + 1
    } else {
        num_parents
    }
}

/// Hash a subtree with SIMD parallelism across chunks, returning the number of
/// chaining values written to `out` (at most the SIMD degree, at least 2).
///
/// This never performs the root compression: when the SIMD degree is 1 it
/// still returns two chaining values, so the caller can apply `ROOT` itself.
fn compress_subtree_wide(
    input: &[u8],
    key: &[u32; 8],
    chunk_counter: u64,
    flags: u8,
    platform: Platform,
    out: &mut [u8],
) -> usize {
    if input.len() <= platform.simd_degree() * CHUNK_LEN {
        return compress_chunks_parallel(input, key, chunk_counter, flags, platform, out);
    }

    // More than a SIMD degree of chunks: split into two subtrees and recurse.
    // The split is only optimal while the SIMD degree is a power of two.
    debug_assert!(platform.simd_degree().is_power_of_two(), "power of 2");
    let left_len = hazmat_left_subtree_len(input.len() as u64) as usize;
    let (left, right) = input.split_at(left_len);
    let right_chunk_counter = chunk_counter.wrapping_add((left.len() / CHUNK_LEN) as u64);

    let mut cv_array = [0u8; 2 * MAX_SIMD_DEGREE_OR_2 * OUT_LEN];
    let degree = if left.len() == CHUNK_LEN {
        // The `simd_degree == 1` leaf case.
        debug_assert_eq!(platform.simd_degree(), 1);
        1
    } else {
        cmp::max(platform.simd_degree(), 2)
    };
    let (left_out, right_out) = cv_array.split_at_mut(degree * OUT_LEN);

    let left_n = compress_subtree_wide(left, key, chunk_counter, flags, platform, left_out);
    let right_n = compress_subtree_wide(right, key, right_chunk_counter, flags, platform, right_out);

    debug_assert_eq!(left_n, degree);
    debug_assert!(right_n >= 1 && right_n <= left_n);

    if left_n == 1 {
        out[..2 * OUT_LEN].copy_from_slice(&cv_array[..2 * OUT_LEN]);
        return 2;
    }

    let num_children = left_n + right_n;
    compress_parents_parallel(&cv_array[..num_children * OUT_LEN], key, flags, platform, out)
}

/// Compress a subtree down to its topmost parent *node* (the 64-byte message,
/// not the chaining value), so that the caller can still apply `ROOT`.
fn compress_subtree_to_parent_node(
    input: &[u8],
    key: &[u32; 8],
    chunk_counter: u64,
    flags: u8,
    platform: Platform,
) -> [u8; BLOCK_LEN] {
    debug_assert!(input.len() > CHUNK_LEN);
    let mut cv_array = [0u8; MAX_SIMD_DEGREE_OR_2 * OUT_LEN];
    let mut num_cvs = compress_subtree_wide(input, key, chunk_counter, flags, platform, &mut cv_array);
    debug_assert!(num_cvs >= 2);

    let mut out_array = [0u8; MAX_SIMD_DEGREE_OR_2 * OUT_LEN / 2];
    while num_cvs > 2 {
        let cv_slice = &cv_array[..num_cvs * OUT_LEN];
        num_cvs = compress_parents_parallel(cv_slice, key, flags, platform, &mut out_array);
        cv_array[..num_cvs * OUT_LEN].copy_from_slice(&out_array[..num_cvs * OUT_LEN]);
    }
    cv_array[..BLOCK_LEN].try_into().unwrap()
}

/// The left/right split point used by [`compress_subtree_wide`]: the largest
/// power-of-two number of chunks that leaves at least one byte on the right.
///
/// Only valid for `content_len > CHUNK_LEN`.
#[inline]
pub(super) fn hazmat_left_subtree_len(content_len: u64) -> u64 {
    debug_assert!(content_len > CHUNK_LEN as u64);
    // A power-of-two number of full chunks, excluding the last chunk.
    let full_chunks = (content_len - 1) / CHUNK_LEN as u64;
    largest_power_of_two_leq_u64(full_chunks) * CHUNK_LEN as u64
}

#[inline]
fn largest_power_of_two_leq_u64(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    1u64 << (63 - n.leading_zeros())
}

/// Hash an entire input at once. Unlike the subtree helpers this handles the
/// one-chunk case, which is also the only case where the chunk itself is root.
fn hash_all_at_once(input: &[u8], key: &[u32; 8], flags: u8, platform: Platform) -> Output {
    if input.len() <= CHUNK_LEN {
        let mut chunk_state = ChunkState::new(key, 0, flags, platform);
        chunk_state.update(input);
        return chunk_state.output();
    }

    Output {
        input_chaining_value: *key,
        block: Aligned64(compress_subtree_to_parent_node(input, key, 0, flags, platform)),
        block_len: BLOCK_LEN as u8,
        counter: 0,
        flags: flags | PARENT,
        platform,
    }
}

/// One-shot BLAKE3 hash. Returns the default 32-byte digest.
pub(crate) fn hash(input: &[u8]) -> [u8; OUT_LEN] {
    let platform = Platform::detect();
    hash_all_at_once(input, &IV, 0, platform).root_hash()
}

/// One-shot keyed hash (MAC).
pub(crate) fn keyed_hash(key: &[u8; KEY_LEN], input: &[u8]) -> [u8; OUT_LEN] {
    let key_words = words_from_le_bytes_32(key);
    let platform = Platform::detect();
    hash_all_at_once(input, &key_words, KEYED_HASH, platform).root_hash()
}

/// Hash the context string of the key derivation mode.
fn hash_derive_key_context(context: &str) -> [u8; OUT_LEN] {
    let platform = Platform::detect();
    hash_all_at_once(context.as_bytes(), &IV, DERIVE_KEY_CONTEXT, platform).root_hash()
}

/// One-shot key derivation. The context string must be hardcoded, globally
/// unique and application-specific.
pub(crate) fn derive_key(context: &str, key_material: &[u8]) -> [u8; OUT_LEN] {
    let context_key = hash_derive_key_context(context);
    let key_words = words_from_le_bytes_32(&context_key);
    let platform = Platform::detect();
    hash_all_at_once(key_material, &key_words, DERIVE_KEY_MATERIAL, platform).root_hash()
}

/// Incremental BLAKE3 hasher.
///
/// The internal state is about 1.9 KiB and lives entirely on the stack: hashing
/// never allocates. Compressing a wide subtree uses a few KiB of additional
/// stack scratch inside `update`. Cloning a hasher clones its state.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
pub(crate) struct Hasher {
    key: [u32; 8],
    chunk_state: ChunkState,
    // The stack is `MAX_DEPTH + 1` entries because merging is lazy: with 7
    // chunks there are 3 entries, and an 8th chunk needs a 4th rather than
    // collapsing everything to 1, since more input might still be coming.
    cv_stack: [[u8; OUT_LEN]; MAX_DEPTH + 1],
    cv_stack_len: u8,
}

impl Hasher {
    fn new_internal(key: &[u32; 8], flags: u8) -> Self {
        Self {
            key: *key,
            chunk_state: ChunkState::new(key, 0, flags, Platform::detect()),
            cv_stack: [[0; OUT_LEN]; MAX_DEPTH + 1],
            cv_stack_len: 0,
        }
    }

    /// Create a hasher for the regular hash function.
    pub(crate) fn new() -> Self {
        Self::new_internal(&IV, 0)
    }

    /// Create a hasher for the keyed hash function.
    pub(crate) fn new_keyed(key: &[u8; KEY_LEN]) -> Self {
        let key_words = words_from_le_bytes_32(key);
        Self::new_internal(&key_words, KEYED_HASH)
    }

    /// Create a hasher for the key derivation function.
    pub(crate) fn new_derive_key(context: &str) -> Self {
        let context_key = hash_derive_key_context(context);
        let key_words = words_from_le_bytes_32(&context_key);
        Self::new_internal(&key_words, DERIVE_KEY_MATERIAL)
    }

    fn push_stack(&mut self, cv: [u8; OUT_LEN]) {
        self.cv_stack[self.cv_stack_len as usize] = cv;
        self.cv_stack_len += 1;
    }

    fn pop_stack(&mut self) -> [u8; OUT_LEN] {
        self.cv_stack_len -= 1;
        self.cv_stack[self.cv_stack_len as usize]
    }

    /// Merge the top of the stack until it has as many entries as there are
    /// set bits in `chunk_counter`.
    ///
    /// This keeps unmerged subtrees on the stack so that the root
    /// compression is never performed early; each stack entry corresponds to a
    /// 1-bit in the number of chunks hashed so far.
    fn merge_cv_stack(&mut self, chunk_counter: u64) {
        let post_merge_stack_len = chunk_counter.count_ones() as usize;
        while self.cv_stack_len as usize > post_merge_stack_len {
            let right_child = self.pop_stack();
            let left_child = self.pop_stack();
            let parent = parent_node_output(
                &left_child,
                &right_child,
                &self.key,
                self.chunk_state.flags,
                self.chunk_state.platform,
            );
            self.push_stack(parent.chaining_value());
        }
    }

    fn push_cv(&mut self, new_cv: [u8; OUT_LEN], chunk_counter: u64) {
        self.merge_cv_stack(chunk_counter);
        self.push_stack(new_cv);
    }

    /// Add input bytes. May be called any number of times; an empty slice is a
    /// no-op.
    pub(crate) fn update(&mut self, mut input: &[u8]) {
        if input.is_empty() {
            return;
        }

        // Finish the partial chunk first, if any.
        if self.chunk_state.count() > 0 {
            let want = CHUNK_LEN - self.chunk_state.count();
            let take = cmp::min(want, input.len());
            self.chunk_state.update(&input[..take]);
            input = &input[take..];
            if input.is_empty() {
                return;
            }
            // The chunk is full and more input is coming, so it is not root.
            debug_assert_eq!(self.chunk_state.count(), CHUNK_LEN);
            let chunk_cv = self.chunk_state.output().chaining_value();
            self.push_cv(chunk_cv, self.chunk_state.chunk_counter);
            self.chunk_state = ChunkState::new(
                &self.key,
                self.chunk_state.chunk_counter.wrapping_add(1),
                self.chunk_state.flags,
                self.chunk_state.platform,
            );
        }

        // Hash the largest whole power-of-two subtree we can, with full SIMD
        // parallelism. Two restrictions:
        // - the subtree must be a power-of-two number of chunks (only right-edge
        //   subtrees may be incomplete, and we don't know where the edge is yet);
        // - it must evenly divide the chunks hashed so far, otherwise we could
        //   never graduate to a wider SIMD degree (BLAKE3 issue #69).
        while input.len() > CHUNK_LEN {
            debug_assert_eq!(self.chunk_state.count(), 0, "no partial chunk data");
            let mut subtree_len = largest_power_of_two_leq(input.len());
            let count_so_far = self.chunk_state.chunk_counter.wrapping_mul(CHUNK_LEN as u64);
            while (subtree_len - 1) as u64 & count_so_far != 0 {
                subtree_len /= 2;
            }

            let subtree_chunks = (subtree_len / CHUNK_LEN) as u64;
            if subtree_len <= CHUNK_LEN {
                debug_assert_eq!(subtree_len, CHUNK_LEN);
                let mut chunk_state = ChunkState::new(
                    &self.key,
                    self.chunk_state.chunk_counter,
                    self.chunk_state.flags,
                    self.chunk_state.platform,
                );
                chunk_state.update(&input[..subtree_len]);
                self.push_cv(chunk_state.output().chaining_value(), self.chunk_state.chunk_counter);
            } else {
                let cv_pair = compress_subtree_to_parent_node(
                    &input[..subtree_len],
                    &self.key,
                    self.chunk_state.chunk_counter,
                    self.chunk_state.flags,
                    self.chunk_state.platform,
                );
                let left_cv: [u8; OUT_LEN] = cv_pair[..OUT_LEN].try_into().unwrap();
                let right_cv: [u8; OUT_LEN] = cv_pair[OUT_LEN..].try_into().unwrap();
                // Push both halves in order; because merging is lazy the second
                // one stays unmerged, so the root is never compressed early.
                self.push_cv(left_cv, self.chunk_state.chunk_counter);
                self.push_cv(right_cv, self.chunk_state.chunk_counter.wrapping_add(subtree_chunks / 2));
            }
            self.chunk_state.chunk_counter = self.chunk_state.chunk_counter.wrapping_add(subtree_chunks);
            input = &input[subtree_len..];
        }

        // What remains is one chunk or less.
        debug_assert!(input.len() <= CHUNK_LEN);
        if !input.is_empty() {
            self.chunk_state.update(input);
            // Now that the chunk state holds bytes, the stack cannot become the
            // root, so we can do an extra merge to simplify `final_output`.
            self.merge_cv_stack(self.chunk_state.chunk_counter);
        }
    }

    fn final_output(&self) -> Output {
        // A single chunk is its own root.
        if self.cv_stack_len == 0 {
            debug_assert_eq!(self.chunk_state.chunk_counter, 0);
            return self.chunk_state.output();
        }

        let mut output;
        let mut num_cvs_remaining = self.cv_stack_len as usize;
        if self.chunk_state.count() > 0 {
            debug_assert_eq!(
                self.cv_stack_len as usize,
                self.chunk_state.chunk_counter.count_ones() as usize,
                "cv stack does not need a merge",
            );
            output = self.chunk_state.output();
        } else {
            debug_assert!(self.cv_stack_len >= 2);
            output = parent_node_output(
                &self.cv_stack[num_cvs_remaining - 2],
                &self.cv_stack[num_cvs_remaining - 1],
                &self.key,
                self.chunk_state.flags,
                self.chunk_state.platform,
            );
            num_cvs_remaining -= 2;
        }
        while num_cvs_remaining > 0 {
            output = parent_node_output(
                &self.cv_stack[num_cvs_remaining - 1],
                &output.chaining_value(),
                &self.key,
                self.chunk_state.flags,
                self.chunk_state.platform,
            );
            num_cvs_remaining -= 1;
        }
        output
    }

    /// Finalize and return the 32-byte digest. Idempotent; more input may be
    /// added afterwards.
    pub(crate) fn finalize(&self) -> [u8; OUT_LEN] {
        self.final_output().root_hash()
    }

    /// Finalize and return an extendable-output reader.
    pub(crate) fn finalize_xof(&self) -> OutputReader {
        OutputReader::new(self.final_output())
    }

    /// Test-only: whether the fields selected for zeroization have been cleared.
    #[cfg(all(test, feature = "zeroize"))]
    pub(crate) fn debug_is_zeroized(&self) -> bool {
        self.key == [0u32; 8]
            && self.cv_stack_len == 0
            && self.chunk_state.debug_is_zeroized()
            && self.cv_stack.iter().all(|cv| *cv == [0u8; OUT_LEN])
    }
}

/// Extendable-output reader produced by [`Hasher::finalize_xof`].
///
/// Output is generated on demand from the root node; the reader keeps the
/// position so that repeated calls to [`OutputReader::fill`] continue the
/// stream. The unread part of the current output block is cached, so a partial
/// read never recomputes an output block it has already started.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize))]
pub(crate) struct OutputReader {
    inner: Output,
    /// The current 64-byte output block.
    ///
    /// Bytes at and after `position_within_block` have not been handed out yet.
    /// When `position_within_block` is zero the cache is stale: the next block
    /// is computed on demand.
    block: [u8; BLOCK_LEN],
    /// How many bytes of `block` have already been handed out.
    position_within_block: u8,
}

impl OutputReader {
    fn new(inner: Output) -> Self {
        Self {
            inner,
            block: [0u8; BLOCK_LEN],
            position_within_block: 0,
        }
    }

    /// Write output bytes into `buf`, advancing the reader. An empty `buf` is a
    /// no-op.
    ///
    /// Output is buffered: the unread remainder of the current 64-byte block is
    /// kept, so reading in small pieces costs at most one compression per 64
    /// bytes instead of one per call. Supplying several whole blocks at once
    /// (up to `MAX_SIMD_DEGREE` blocks per compression on SIMD backends) remains
    /// the fastest way to read a long stream.
    pub(crate) fn fill(&mut self, mut buf: &mut [u8]) {
        if buf.is_empty() {
            return;
        }

        // Serve the unread remainder of the cached block first.
        if self.position_within_block != 0 {
            let start = self.position_within_block as usize;
            let take = cmp::min(buf.len(), BLOCK_LEN - start);
            buf[..take].copy_from_slice(&self.block[start..start + take]);
            self.position_within_block += take as u8;
            if self.position_within_block as usize == BLOCK_LEN {
                // Wrapping on purpose: reaching 2^64 bytes of output is
                // impossible, but a debug-build panic here would be a denial
                // of service.
                self.inner.counter = self.inner.counter.wrapping_add(1);
                self.position_within_block = 0;
            }
            buf = &mut buf[take..];
            if buf.is_empty() {
                return;
            }
        }

        // Write whole blocks straight into the caller's buffer, keeping the
        // wide, parallel SIMD output path available.
        let full_len = (buf.len() / BLOCK_LEN) * BLOCK_LEN;
        if full_len > 0 {
            debug_assert_eq!(self.position_within_block, 0);
            self.inner.output_many(&mut buf[..full_len]);
            self.inner.counter = self.inner.counter.wrapping_add((full_len / BLOCK_LEN) as u64);
            buf = &mut buf[full_len..];
        }

        // Cache the final partial block rather than recomputing it on the next
        // call.
        if !buf.is_empty() {
            debug_assert_eq!(self.position_within_block, 0);
            debug_assert!(buf.len() < BLOCK_LEN);
            self.block = self.inner.root_output_block();
            let take = buf.len();
            buf.copy_from_slice(&self.block[..take]);
            self.position_within_block = take as u8;
        }
    }

    /// The current position, in bytes, within the output stream.
    #[cfg(test)]
    pub(crate) fn position(&self) -> u64 {
        self.inner
            .counter
            .wrapping_mul(BLOCK_LEN as u64)
            .wrapping_add(self.position_within_block as u64)
    }

    /// Seek to `position` bytes into the output stream.
    #[cfg(test)]
    pub(crate) fn set_position(&mut self, position: u64) {
        self.position_within_block = (position % BLOCK_LEN as u64) as u8;
        self.inner.counter = position / BLOCK_LEN as u64;
        if self.position_within_block != 0 {
            // Refresh the cache so it matches the new position.
            self.block = self.inner.root_output_block();
        }
    }

    /// Test-only: whether the fields selected for zeroization have been cleared.
    #[cfg(all(test, feature = "zeroize"))]
    pub(crate) fn debug_is_zeroized(&self) -> bool {
        self.inner.debug_is_zeroized() && self.position_within_block == 0 && self.block.iter().all(|b| *b == 0)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Test-only entry points that force a specific backend
////////////////////////////////////////////////////////////////////////////////

/// One-shot hash with an explicitly chosen backend, so tests can exercise the
/// portable (scalar) path on a machine where a SIMD backend is available.
#[cfg(test)]
pub(crate) fn debug_hash_with(platform: Platform, input: &[u8]) -> [u8; OUT_LEN] {
    hash_all_at_once(input, &IV, 0, platform).root_hash()
}

/// Keyed hash with an explicitly chosen backend.
#[cfg(test)]
pub(crate) fn debug_keyed_hash_with(platform: Platform, key: &[u8; KEY_LEN], input: &[u8]) -> [u8; OUT_LEN] {
    let key_words = words_from_le_bytes_32(key);
    hash_all_at_once(input, &key_words, KEYED_HASH, platform).root_hash()
}

/// Key derivation with an explicitly chosen backend.
#[cfg(test)]
pub(crate) fn debug_derive_key_with(platform: Platform, context: &str, material: &[u8]) -> [u8; OUT_LEN] {
    let context_key = hash_all_at_once(context.as_bytes(), &IV, DERIVE_KEY_CONTEXT, platform).root_hash();
    let key_words = words_from_le_bytes_32(&context_key);
    hash_all_at_once(material, &key_words, DERIVE_KEY_MATERIAL, platform).root_hash()
}

/// Extended output (from offset 0) with an explicitly chosen backend.
#[cfg(test)]
pub(crate) fn debug_xof_with(platform: Platform, input: &[u8], out: &mut [u8]) {
    OutputReader::new(hash_all_at_once(input, &IV, 0, platform)).fill(out);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic test bytes.
    fn pattern(len: usize, seed: u8) -> Vec<u8> {
        (0..len)
            .map(|i| ((i as u8).wrapping_mul(31).wrapping_add(seed)) % 251)
            .collect()
    }

    /// The CV stack never exceeds `MAX_DEPTH + 1` entries, and when the chunk
    /// buffer is empty the stack is either empty or has at least two entries
    /// (the invariant `final_output` relies on).
    #[test]
    fn cv_stack_invariants_hold_while_hashing() {
        assert_eq!(MAX_DEPTH, 54, "MAX_DEPTH encodes 2^54 chunks of 2^10 bytes");
        assert!(MAX_DEPTH < u8::MAX as usize, "stack length must fit in u8");

        let data = pattern(20_000, 7);
        for &len in &[
            0usize, 1, 63, 64, 65, 1023, 1024, 1025, 2047, 2048, 2049, 3072, 4095, 4096, 4097, 8191, 8192, 8193, 16384,
            20_000,
        ] {
            let mut hasher = Hasher::new();
            hasher.update(&data[..len]);
            assert!(
                hasher.cv_stack_len as usize <= MAX_DEPTH + 1,
                "stack overflow at len {len}: {}",
                hasher.cv_stack_len
            );
            if hasher.chunk_state.count() == 0 {
                assert!(
                    hasher.cv_stack_len == 0 || hasher.cv_stack_len >= 2,
                    "a non-empty stack with an empty chunk buffer must have >= 2 entries at len {len}"
                );
            }
            // Any number of extra empty updates must not change the state.
            let before = hasher.cv_stack_len;
            hasher.update(&[]);
            assert_eq!(before, hasher.cv_stack_len);
        }
    }

    /// The stack cannot grow past `MAX_DEPTH + 1` even at the counter with the
    /// maximum possible number of set bits (54 ones, i.e. `2^54 - 1` chunks).
    #[test]
    fn cv_stack_bounds_at_maximum_popcount_counter() {
        let max_counter = (1u64 << 54) - 1;
        assert_eq!(max_counter.count_ones() as usize, MAX_DEPTH);

        let mut hasher = Hasher::new();
        // Pretend 54 subtrees are already on the stack; merging for a counter
        // with 54 ones keeps all of them, and the push adds the 55th slot.
        hasher.cv_stack_len = MAX_DEPTH as u8;
        hasher.push_cv([0xAAu8; OUT_LEN], max_counter);
        assert_eq!(hasher.cv_stack_len as usize, MAX_DEPTH + 1);
        assert_eq!(hasher.cv_stack[MAX_DEPTH], [0xAAu8; OUT_LEN]);
        assert!(hasher.cv_stack[..MAX_DEPTH].iter().all(|cv| *cv == [0u8; OUT_LEN]));

        // A counter with one set bit collapses the whole stack before pushing.
        hasher.push_cv([0xBBu8; OUT_LEN], 1u64 << 54);
        assert_eq!(hasher.cv_stack_len, 2);
        assert_eq!(hasher.cv_stack[1], [0xBBu8; OUT_LEN]);

        // `merge_cv_stack` is monotone: it never leaves more entries than the
        // target popcount (or the previous length, whichever is smaller). The
        // lengths used here are the ones a real hasher can produce: either
        // `ones(counter)` or the extra un-merged slot above it.
        for &counter in &[
            1u64,
            2,
            3,
            7,
            8,
            u32::MAX as u64,
            (1u64 << 32) + 1,
            (1u64 << 54) - 1,
            u64::MAX,
        ] {
            let target = counter.count_ones() as usize;
            for extra in 0..=1usize {
                let mut hasher = Hasher::new();
                let before = core::cmp::min(target + extra, MAX_DEPTH + 1);
                hasher.cv_stack_len = before as u8;
                hasher.merge_cv_stack(counter);
                assert_eq!(
                    hasher.cv_stack_len as usize,
                    core::cmp::min(before, target),
                    "merge_cv_stack({counter}) produced the wrong length"
                );
            }
        }
    }

    /// `final_output` walks a maximum-depth stack (55 entries) without
    /// panicking, and the merging loop is in bounds for every length.
    #[test]
    fn final_output_at_maximum_depth() {
        let mut hasher = Hasher::new();
        hasher.chunk_state.chunk_counter = (1u64 << 54) - 1;
        // A full-size lazy stack with an empty chunk buffer.
        hasher.cv_stack_len = (MAX_DEPTH + 1) as u8;
        for (i, cv) in hasher.cv_stack.iter_mut().enumerate() {
            *cv = [i as u8; OUT_LEN];
        }
        let output = hasher.final_output();
        // The output must be stable and must never apply ROOT early.
        assert_eq!(output.root_hash(), hasher.finalize());

        // A counter with the maximum popcount and a non-empty chunk buffer.
        let mut hasher = Hasher::new();
        hasher.chunk_state.chunk_counter = (1u64 << 54) - 1;
        hasher.cv_stack_len = MAX_DEPTH as u8;
        hasher.chunk_state.update(b"x");
        let _ = hasher.finalize();
    }

    /// `update` at the maximum *documented* chunk counter (`2^54`, i.e. 2^64
    /// bytes of input) must keep the stack bounded and must not panic.
    /// Counters beyond that are outside the documented input limit (the digest
    /// is unspecified there); see the report.
    #[test]
    fn update_at_maximum_documented_chunk_counter() {
        let data = pattern(3 * CHUNK_LEN + 1, 11);
        for &start in &[(1u64 << 54) - 4, (1u64 << 54) - 3, (1u64 << 54) - 1] {
            let mut hasher = Hasher::new();
            hasher.chunk_state.chunk_counter = start;
            hasher.update(&data);
            // Three whole chunks were consumed; the trailing byte sits in chunk 3.
            assert_eq!(hasher.chunk_state.chunk_counter, start.wrapping_add(3));
            assert_eq!(hasher.chunk_state.count(), 1);
            assert!(hasher.cv_stack_len as usize <= MAX_DEPTH + 1);
        }
    }

    // The `u64` chunk counter is documented as wrapping, but the lazy CV
    // stack only has `MAX_DEPTH + 1` slots, so a stream long enough to push
    // the chunk counter past `2^54` chunks is outside the documented input
    // limit and can panic (`pop_stack`/`push_stack`). No test constructs such
    // a stream: it would need more than 2^64 bytes of input, which cannot be
    // supplied on any current target.

    /// `compress_chunks_parallel` must give each lane `chunk_counter + i`,
    /// including when that wraps past `u64::MAX`, and must agree with hashing
    /// each chunk separately.
    #[test]
    fn compress_chunks_parallel_counter_wrap() {
        for platform in [Platform::Portable, Platform::detect()] {
            let degree = platform.simd_degree();
            let input = pattern(degree * CHUNK_LEN, 3);
            let start = u64::MAX - (degree as u64) / 2;

            let mut out = vec![0u8; (degree + 1) * OUT_LEN];
            let written = compress_chunks_parallel(&input, &IV, start, 0, platform, &mut out);
            assert_eq!(written, degree, "platform {platform:?}");

            for i in 0..degree {
                let counter = start.wrapping_add(i as u64);
                let mut chunk = ChunkState::new(&IV, counter, 0, Platform::Portable);
                chunk.update(&input[i * CHUNK_LEN..(i + 1) * CHUNK_LEN]);
                assert_eq!(
                    &out[i * OUT_LEN..(i + 1) * OUT_LEN],
                    &chunk.output().chaining_value()[..],
                    "platform {platform:?}, lane {i}, counter {counter}"
                );
            }
        }
    }

    /// A trailing partial chunk gets `chunk_counter + num_full_chunks`, also
    /// with wrapping.
    #[test]
    fn compress_chunks_parallel_partial_counter_wrap() {
        for platform in [Platform::Portable, Platform::detect()] {
            let degree = platform.simd_degree();
            if degree < 2 {
                continue;
            }
            let full = degree - 1;
            let input = pattern(full * CHUNK_LEN + 5, 9);
            let start = u64::MAX - (full as u64);

            let mut out = vec![0u8; (full + 2) * OUT_LEN];
            let written = compress_chunks_parallel(&input, &IV, start, 0, platform, &mut out);
            assert_eq!(written, full + 1, "platform {platform:?}");

            let counter = start.wrapping_add(full as u64);
            let mut chunk = ChunkState::new(&IV, counter, 0, Platform::Portable);
            chunk.update(&input[full * CHUNK_LEN..]);
            assert_eq!(
                &out[full * OUT_LEN..(full + 1) * OUT_LEN],
                &chunk.output().chaining_value()[..],
                "platform {platform:?}, partial counter {counter}"
            );
        }
    }
}
