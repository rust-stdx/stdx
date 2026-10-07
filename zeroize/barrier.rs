/// Observe the referenced data and prevent the compiler from removing previous writes to it.
///
/// This function acts like [`core::hint::black_box`] but takes a reference and
/// does not return the passed value.
///
/// It's implemented using the [`core::arch::asm!`] macro on target arches where `asm!` is stable,
/// i.e. `aarch64`, `arm`, `arm64ec`, `loongarch64`, `riscv32`, `riscv64`, `s390x`, `x86`, and
/// `x86_64`.
///
/// On all other targets it's implemented using [`core::hint::black_box`] and custom `black_box`
/// implemented using `#[inline(never)]` and `read_volatile`.
///
/// # Examples
/// ```
/// use core::num::NonZeroU32;
///
/// # type ThirdPartyType = u32;
///
/// struct DataToZeroize {
///     buf: [u8; 32],
///     // `ThirdPartyType` can be a type with private fields
///     // defined in a third-party crate and which does not implement
///     // `Zeroize` or zeroization on drop.
///     data: ThirdPartyType,
///     pos: NonZeroU32,
/// }
///
/// struct SomeMoreFlatData(u64);
///
/// impl Drop for DataToZeroize {
///     fn drop(&mut self) {
///         self.buf = [0u8; 32];
///         self.data = ThirdPartyType::default();
///         self.pos = NonZeroU32::new(32).unwrap();
///         zeroize::optimization_barrier(self);
///     }
/// }
///
/// impl zeroize::ZeroizeOnDrop for DataToZeroize {}
///
/// let mut data = DataToZeroize {
///     buf: [3u8; 32],
///     data: ThirdPartyType::default(),
///     pos: NonZeroU32::new(32).unwrap(),
/// };
///
/// // data gets zeroized when dropped
/// ```
///
/// Note that erasure of `ThirdPartyType` demonstrated above can be fragile if it contains
/// `MaybeUninit` or `union` data. It also does not perform erasure of types like `Box` or `Vec`.
pub fn optimization_barrier<T: ?Sized>(val: &T) {
    #[cfg(all(
        not(miri),
        any(
            target_arch = "aarch64",
            target_arch = "arm",
            target_arch = "arm64ec",
            target_arch = "loongarch64",
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "s390x",
            target_arch = "x86",
            target_arch = "x86_64",
        )
    ))]
    // Safety:
    //
    // The asm block only passes the address of `val` in a register and does not read or write
    // any memory itself (`readonly`), so it has no additional requirements. Not specifying
    // `nomem` is intentional: it makes the compiler assume the block may read memory, which is
    // what prevents earlier writes to `val` from being optimized away.
    unsafe {
        core::arch::asm!(
            "# {}",
            in(reg) core::ptr::from_ref::<T>(val).cast::<()>(),
            options(readonly, preserves_flags, nostack),
        );
    }
    #[cfg(not(all(
        not(miri),
        any(
            target_arch = "aarch64",
            target_arch = "arm",
            target_arch = "arm64ec",
            target_arch = "loongarch64",
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "s390x",
            target_arch = "x86",
            target_arch = "x86_64",
        )
    )))]
    {
        /// Custom version of `core::hint::black_box` implemented using
        /// `#[inline(never)]` and `read_volatile`.
        #[inline(never)]
        fn custom_black_box(p: *const u8) {
            use core::mem::MaybeUninit;

            // Safety:
            //
            // Callers only pass a non-null pointer to a live value (`custom_black_box` is only
            // called when `size_of_val(val) > 0`), and `MaybeUninit<u8>` accepts any byte, so
            // the volatile read is valid even for uninitialized padding.
            let _: MaybeUninit<u8> = unsafe { core::ptr::read_volatile(p.cast()) };
        }

        core::hint::black_box(val);
        if size_of_val(val) > 0 {
            custom_black_box(core::ptr::from_ref(val).cast::<u8>());
        }
    }
}
