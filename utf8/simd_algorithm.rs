#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_macros)]
//! Core SIMD UTF-8 validation algorithm adapted from simdutf8 (MIT OR Apache-2.0).

// Macros requires newtypes in scope:
// `SimdU8Value` - implementation of SIMD primitives
// `SimdInput` - which  holds 64 bytes of SIMD input
// `TempSimdChunk` - correctly aligned `TempSimdChunk`, either `TempSimdChunkA16` or `TempSimdChunkA32`

macro_rules! algorithm_simd {
    ($feat:expr) => {
        impl Utf8CheckAlgorithm<SimdU8Value> {
            #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
            #[inline]
            unsafe fn default() -> Self {
                Self {
                    prev: SimdU8Value::splat0(),
                    incomplete: SimdU8Value::splat0(),
                    error: SimdU8Value::splat0(),
                }
            }

            #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
            #[inline]
            unsafe fn check_incomplete_pending(&mut self) {
                self.error = self.error.or(self.incomplete);
            }

            #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
            #[inline]
            unsafe fn is_incomplete(input: SimdU8Value) -> SimdU8Value {
                input.saturating_sub(SimdU8Value::from_32_cut_off_leading(
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0xff,
                    0b1111_0000 - 1,
                    0b1110_0000 - 1,
                    0b1100_0000 - 1,
                ))
            }

            #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
            #[inline]
            #[allow(clippy::too_many_lines)]
            unsafe fn check_special_cases(input: SimdU8Value, prev1: SimdU8Value) -> SimdU8Value {
                const TOO_SHORT: u8 = 1 << 0;
                const TOO_LONG: u8 = 1 << 1;
                const OVERLONG_3: u8 = 1 << 2;
                const SURROGATE: u8 = 1 << 4;
                const OVERLONG_2: u8 = 1 << 5;
                const TWO_CONTS: u8 = 1 << 7;
                const TOO_LARGE: u8 = 1 << 3;
                const TOO_LARGE_1000: u8 = 1 << 6;
                const OVERLONG_4: u8 = 1 << 6;
                const CARRY: u8 = TOO_SHORT | TOO_LONG | TWO_CONTS;

                let byte_1_high = prev1.shr4().lookup_16(
                    TOO_LONG,
                    TOO_LONG,
                    TOO_LONG,
                    TOO_LONG,
                    TOO_LONG,
                    TOO_LONG,
                    TOO_LONG,
                    TOO_LONG,
                    TWO_CONTS,
                    TWO_CONTS,
                    TWO_CONTS,
                    TWO_CONTS,
                    TOO_SHORT | OVERLONG_2,
                    TOO_SHORT,
                    TOO_SHORT | OVERLONG_3 | SURROGATE,
                    TOO_SHORT | TOO_LARGE | TOO_LARGE_1000 | OVERLONG_4,
                );

                let byte_1_low = prev1.and(SimdU8Value::splat(0x0F)).lookup_16(
                    CARRY | OVERLONG_3 | OVERLONG_2 | OVERLONG_4,
                    CARRY | OVERLONG_2,
                    CARRY,
                    CARRY,
                    CARRY | TOO_LARGE,
                    CARRY | TOO_LARGE | TOO_LARGE_1000,
                    CARRY | TOO_LARGE | TOO_LARGE_1000,
                    CARRY | TOO_LARGE | TOO_LARGE_1000,
                    CARRY | TOO_LARGE | TOO_LARGE_1000,
                    CARRY | TOO_LARGE | TOO_LARGE_1000,
                    CARRY | TOO_LARGE | TOO_LARGE_1000,
                    CARRY | TOO_LARGE | TOO_LARGE_1000,
                    CARRY | TOO_LARGE | TOO_LARGE_1000,
                    CARRY | TOO_LARGE | TOO_LARGE_1000 | SURROGATE,
                    CARRY | TOO_LARGE | TOO_LARGE_1000,
                    CARRY | TOO_LARGE | TOO_LARGE_1000,
                );

                let byte_2_high = input.shr4().lookup_16(
                    TOO_SHORT,
                    TOO_SHORT,
                    TOO_SHORT,
                    TOO_SHORT,
                    TOO_SHORT,
                    TOO_SHORT,
                    TOO_SHORT,
                    TOO_SHORT,
                    TOO_LONG | OVERLONG_2 | TWO_CONTS | OVERLONG_3 | TOO_LARGE_1000 | OVERLONG_4,
                    TOO_LONG | OVERLONG_2 | TWO_CONTS | OVERLONG_3 | TOO_LARGE,
                    TOO_LONG | OVERLONG_2 | TWO_CONTS | SURROGATE | TOO_LARGE,
                    TOO_LONG | OVERLONG_2 | TWO_CONTS | SURROGATE | TOO_LARGE,
                    TOO_SHORT,
                    TOO_SHORT,
                    TOO_SHORT,
                    TOO_SHORT,
                );

                byte_1_high.and(byte_1_low).and(byte_2_high)
            }

            #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
            #[inline]
            unsafe fn check_multibyte_lengths(
                input: SimdU8Value,
                prev: SimdU8Value,
                special_cases: SimdU8Value,
            ) -> SimdU8Value {
                let prev2 = input.prev2(prev);
                let prev3 = input.prev3(prev);
                let must23 = Self::must_be_2_3_continuation(prev2, prev3);
                let must23_80 = must23.and(SimdU8Value::splat(0x80));
                must23_80.xor(special_cases)
            }

            #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
            #[inline]
            unsafe fn has_error(&self) -> bool {
                self.error.any_bit_set()
            }

            #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
            #[inline]
            unsafe fn check_bytes(&mut self, input: SimdU8Value) {
                let prev1 = input.prev1(self.prev);
                let sc = Self::check_special_cases(input, prev1);
                self.error = self.error.or(Self::check_multibyte_lengths(input, self.prev, sc));
                self.prev = input;
            }

            #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
            #[inline]
            unsafe fn check_utf8(&mut self, input: SimdInput) {
                if input.is_ascii() {
                    self.check_incomplete_pending();
                } else {
                    self.check_block(input);
                }
            }

            #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
            #[inline]
            unsafe fn check_block(&mut self, input: SimdInput) {
                // WORKAROUND
                // necessary because the for loop is not unrolled on ARM64
                if input.vals.len() == 2 {
                    self.check_bytes(*input.vals.get_unchecked(0));
                    self.check_bytes(*input.vals.get_unchecked(1));
                    self.incomplete = Self::is_incomplete(*input.vals.get_unchecked(1));
                } else if input.vals.len() == 4 {
                    self.check_bytes(*input.vals.get_unchecked(0));
                    self.check_bytes(*input.vals.get_unchecked(1));
                    self.check_bytes(*input.vals.get_unchecked(2));
                    self.check_bytes(*input.vals.get_unchecked(3));
                    self.incomplete = Self::is_incomplete(*input.vals.get_unchecked(3));
                } else {
                    panic!("Unsupported number of chunks");
                }
            }
        }

        /// Validate UTF-8 using the SIMD extension enabled for this module.
        ///
        /// Returns `false` when the input is not valid UTF-8.
        ///
        /// # Safety
        /// This function is inherently unsafe because it is compiled with SIMD extensions
        /// enabled. Make sure that the CPU supports it before calling.
        ///
        #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
        #[inline]
        pub unsafe fn validate_utf8_basic(input: &[u8]) -> bool {
            use crate::simd_helpers::SIMD_CHUNK_SIZE;
            let len = input.len();
            let mut algorithm = Utf8CheckAlgorithm::<SimdU8Value>::default();
            let mut idx: usize = 0;
            let iter_lim = len - (len % SIMD_CHUNK_SIZE);

            while idx < iter_lim {
                let simd_input = SimdInput::new(input.get_unchecked(idx as usize..));
                idx += SIMD_CHUNK_SIZE;
                if !simd_input.is_ascii() {
                    algorithm.check_block(simd_input);
                    break;
                }
            }

            while idx < iter_lim {
                if PREFETCH {
                    simd_prefetch(input.as_ptr().add(idx + SIMD_CHUNK_SIZE * 2));
                }
                let input = SimdInput::new(input.get_unchecked(idx as usize..));
                algorithm.check_utf8(input);
                idx += SIMD_CHUNK_SIZE;
            }

            if idx < len {
                let mut tmpbuf = TempSimdChunk::new();
                crate::simd_helpers::memcpy_unaligned_nonoverlapping_inline_opt_lt_64(
                    input.as_ptr().add(idx),
                    tmpbuf.0.as_mut_ptr(),
                    len - idx,
                );
                let simd_input = SimdInput::new(&tmpbuf.0);
                algorithm.check_utf8(simd_input);
            }
            algorithm.check_incomplete_pending();
            if algorithm.has_error() { false } else { true }
        }
    };
}

macro_rules! simd_input_128_bit {
    ($feat:expr) => {
        #[repr(C)]
        struct SimdInput {
            vals: [SimdU8Value; 4],
        }

        impl SimdInput {
            #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
            #[inline]
            #[allow(clippy::cast_ptr_alignment)]
            unsafe fn new(ptr: &[u8]) -> Self {
                Self {
                    vals: [
                        SimdU8Value::load_from(ptr.as_ptr()),
                        SimdU8Value::load_from(ptr.as_ptr().add(16)),
                        SimdU8Value::load_from(ptr.as_ptr().add(32)),
                        SimdU8Value::load_from(ptr.as_ptr().add(48)),
                    ],
                }
            }

            #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
            #[inline]
            unsafe fn is_ascii(&self) -> bool {
                let r1 = self.vals[0].or(self.vals[1]);
                let r2 = self.vals[2].or(self.vals[3]);
                let r = r1.or(r2);
                r.is_ascii()
            }
        }
    };
}

macro_rules! simd_input_256_bit {
    ($feat:expr) => {
        #[repr(C)]
        struct SimdInput {
            vals: [SimdU8Value; 2],
        }

        impl SimdInput {
            #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
            #[inline]
            #[allow(clippy::cast_ptr_alignment)]
            unsafe fn new(ptr: &[u8]) -> Self {
                Self {
                    vals: [
                        SimdU8Value::load_from(ptr.as_ptr()),
                        SimdU8Value::load_from(ptr.as_ptr().add(32)),
                    ],
                }
            }

            #[cfg_attr(not(target_arch="aarch64"), target_feature(enable = $feat))]
            #[inline]
            unsafe fn is_ascii(&self) -> bool {
                self.vals[0].or(self.vals[1]).is_ascii()
            }
        }
    };
}
