//! Resource budgets: heap allocations on hot paths and stack usage.
//!
//! Verification, signing and the modular arithmetic must not allocate, and the
//! deepest path (RSA-8192 verification) must fit in a modest thread stack.
//! These guard against regressions such as reintroducing intermediate `Vec`s
//! or oversized stack temporaries.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use big_number::Uint;

/// Wraps the system allocator so a test can count allocations.
struct CountingAllocator;

static COUNTING: AtomicBool = AtomicBool::new(false);
static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);

/// Serialises the tests in this file so allocation counting is not polluted by
/// a test running concurrently in another thread.
static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

/// Runs `f` with allocation counting enabled and returns the number of
/// allocations it performed.
fn count_allocations(f: impl FnOnce()) -> u64 {
    ALLOCATIONS.store(0, Ordering::SeqCst);
    COUNTING.store(true, Ordering::SeqCst);
    f();
    COUNTING.store(false, Ordering::SeqCst);
    ALLOCATIONS.load(Ordering::SeqCst)
}

fn odd_modulus_bytes(bytes: usize) -> Vec<u8> {
    let mut n = vec![0x5Au8; bytes];
    n[0] |= 0x80; // full width
    n[bytes - 1] |= 1; // odd
    n
}

#[test]
fn hot_paths_do_not_allocate() {
    let _guard = TEST_LOCK.lock().unwrap();
    type U256 = Uint<256, 4>;
    let a = U256::from_limbs([1, 2, 3, 4]);
    let m = U256::from_limbs([u64::MAX, u64::MAX, u64::MAX, 1 << 63]);
    let mu = m.compute_mu_for_barrett();

    assert_eq!(
        count_allocations(|| {
            for _ in 0..1_000 {
                std::hint::black_box(a.mul_mod(&a, &m));
                std::hint::black_box(a.mul_mod_barrett(&a, &m, &mu));
            }
        }),
        0,
        "modular multiplication must not allocate"
    );

    assert_eq!(
        count_allocations(|| {
            std::hint::black_box(U256::from_str_radix("123456789012345678901234567890", 10).unwrap());
        }),
        0,
        "from_str_radix must not allocate"
    );

    assert!(
        count_allocations(|| {
            std::hint::black_box(a.to_be_bytes());
        }) <= 1,
        "to_be_bytes allocates once"
    );

    assert!(
        count_allocations(|| {
            std::hint::black_box(a.to_string_radix(10));
        }) <= 1,
        "to_string_radix should allocate a single String"
    );

    // RSA and ECDSA hot paths.
    let n = odd_modulus_bytes(256);
    let e = [0x01u8, 0x00, 0x01];
    let key = crypto::rsa::PublicKey::from_n_e(&n, &e).unwrap();
    let sig = vec![0xA5u8; 256];
    assert_eq!(
        count_allocations(|| {
            let _ = std::hint::black_box(key.verify_pkcs1_sha256(&sig, b"message"));
            let _ = std::hint::black_box(key.verify_pss_sha256(&sig, b"message"));
        }),
        0,
        "RSA verification must not allocate"
    );

    let sk = crypto::p256::SecretKey::generate().unwrap();
    let pk = sk.public_key();
    let signature = sk.sign(b"message").unwrap();
    assert_eq!(
        count_allocations(|| {
            let _ = std::hint::black_box(sk.sign(b"message"));
            let _ = std::hint::black_box(pk.verify(b"message", &signature));
        }),
        0,
        "ECDSA sign/verify must not allocate"
    );
}

#[test]
fn rsa_8192_verification_fits_a_small_stack() {
    let _guard = TEST_LOCK.lock().unwrap();
    let n = odd_modulus_bytes(1024); // 8192-bit modulus
    let e = [0x01u8, 0x00, 0x01];
    let key = crypto::rsa::PublicKey::from_n_e(&n, &e).unwrap();
    let sig = vec![0xA5u8; 1024];

    // 256 KiB is comfortably above the observed footprint but far below the
    // default 8 MiB main-thread stack, so a regression in temporary sizing is
    // caught here.
    let handle = std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(move || {
            assert!(key.verify_pkcs1_sha256(&sig, b"message").is_err());
        })
        .unwrap();
    handle.join().unwrap();
}
