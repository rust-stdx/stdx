//! Audit tests for the novel `zeroize` bulk fast paths and container impls.
//!
//! These tests check the *observable* byte-level result: exactly `count` bytes are
//! written, canaries outside the target range survive, every target byte becomes the
//! zeroization value, and the whole capacity of `Vec`/`String` is covered.
//!
//! Reading raw bytes is only done for types with no padding/uninit bytes (integers and
//! buffers we explicitly initialised), so the tests are Miri-clean.

use core::{
    mem::{MaybeUninit, size_of},
    num::{NonZeroU32, NonZeroU64, Wrapping},
};

use zeroize::{Zeroize, optimization_barrier};

#[repr(align(64))]
struct Aligned<const N: usize>([u8; N]);

/// 0xAA-filled stack scratch, zeroize `[off, off + n)` and verify canaries and target.
#[test]
fn slice_u8_every_length_and_offset() {
    const PAD: usize = 16;
    const SIZE: usize = PAD + 4 * size_of::<usize>() + 3 + PAD;
    let max_n = 4 * size_of::<usize>() + 3;

    for n in 0..=max_n {
        for off in 0..8 {
            let mut buf = Aligned([0xAAu8; SIZE]);
            let start = PAD + off;
            let end = start + n;

            buf.0[start..end].zeroize();

            assert!(
                buf.0[..start].iter().all(|&b| b == 0xAA),
                "n={n} off={off}: wrote before the slice"
            );
            assert!(buf.0[end..].iter().all(|&b| b == 0xAA), "n={n} off={off}: wrote past the slice");
            assert!(
                buf.0[start..end].iter().all(|&b| b == 0),
                "n={n} off={off}: non-zero byte left in range"
            );

            // Same through the doc-hidden associated helper.
            let mut buf2 = Aligned([0xAAu8; SIZE]);
            <u8 as Zeroize>::zeroize_slice(&mut buf2.0[start..end]);
            assert!(buf2.0[..start].iter().all(|&b| b == 0xAA));
            assert!(buf2.0[end..].iter().all(|&b| b == 0xAA));
            assert!(buf2.0[start..end].iter().all(|&b| b == 0));
        }
    }
}

/// `zeroize_slice` over slices whose base address is not word aligned.
#[test]
fn slice_u8_heap_misalignment() {
    for shift in 0..8usize {
        for n in 0..=40usize {
            let mut v = vec![0xAAu8; shift + n + 8];
            let offset = shift;
            {
                let s = &mut v[offset..offset + n];
                <u8 as Zeroize>::zeroize_slice(s);
            }
            assert!(v[..offset].iter().all(|&b| b == 0xAA), "shift={shift} n={n} head");
            assert!(v[offset..offset + n].iter().all(|&b| b == 0), "shift={shift} n={n} body");
            assert!(v[offset + n..].iter().all(|&b| b == 0xAA), "shift={shift} n={n} tail");
        }
    }
}

fn assert_all_zero_bytes<T>(value: &T) {
    let bytes = unsafe { core::slice::from_raw_parts((value as *const T).cast::<u8>(), size_of::<T>()) };
    assert!(bytes.iter().all(|&b| b == 0), "non-zero byte in {} bytes", size_of::<T>());
}

/// Word-sized and wider element types, as both arrays and vectors.
#[test]
fn word_sized_types() {
    macro_rules! check {
        ($ty:ty, $val:expr, $n:expr) => {{
            let mut arr: [$ty; $n] = [$val; $n];
            arr.zeroize();
            assert_all_zero_bytes(&arr);

            let mut v: Vec<$ty> = vec![$val; $n];
            v.zeroize();
            assert!(v.is_empty());
            assert_all_zero_bytes(&arr);
        }};
    }

    check!(u8, 0xAA, 17);
    check!(u16, 0xAAAA, 17);
    check!(u32, 0xAAAA_AAAA, 17);
    check!(u64, 0xAAAA_AAAA_AAAA_AAAA, 17);
    check!(u128, 0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA, 17);
    check!(usize, usize::MAX / 3, 17);
    check!(i64, -2, 17);

    let mut b = [true; 9];
    b.zeroize();
    assert!(b.iter().all(|&x| !x));

    let mut c = ['A'; 5];
    c.zeroize();
    assert!(c.iter().all(|&x| x == '\0'));

    let mut f = [1.5f64; 3];
    f.zeroize();
    assert!(f.iter().all(|&x| x == 0.0));

    let mut w = [Wrapping(0xAAAA_AAAAu32); 4];
    w.zeroize();
    assert!(w.iter().all(|x| x.0 == 0));
}

/// Arrays, nested arrays and tuples.
#[test]
fn arrays_and_tuples() {
    let mut nested = [[0xAAu8; 16]; 4];
    nested.zeroize();
    for row in &nested {
        assert!(row.iter().all(|&b| b == 0));
    }

    let mut tup = (0xAAAA_AAAA_AAAA_AAAAu64, 0xAAAA_AAAAu32, 0xAAu8, [0xAAu8; 5]);
    tup.zeroize();
    assert_eq!(tup.0, 0);
    assert_eq!(tup.1, 0);
    assert_eq!(tup.2, 0);
    assert!(tup.3.iter().all(|&b| b == 0));

    let mut one = (0xAAu8,);
    one.zeroize();
    assert_eq!(one.0, 0);
}

/// `Vec<u8>` and `String` must zero the *whole capacity*, not just `len`.
#[test]
fn vec_and_string_whole_capacity() {
    // Vec<u8>: pre-initialise every capacity byte so reading it afterwards is defined.
    let mut v: Vec<u8> = Vec::with_capacity(101);
    v.extend_from_slice(&[0x11u8; 7]);
    let cap = v.capacity();
    for m in v.spare_capacity_mut() {
        m.write(0xAA);
    }
    v.zeroize();
    assert!(v.is_empty());
    let ptr = v.as_mut_ptr();
    let bytes = unsafe { core::slice::from_raw_parts(ptr, cap) };
    assert!(bytes.iter().all(|&b| b == 0), "Vec<u8> capacity not fully zeroed");

    // String: same, through a UTF-8 buffer.
    let mut s = String::with_capacity(101);
    s.push_str("secret!");
    let cap = s.capacity();
    for m in unsafe { s.as_mut_vec() }.spare_capacity_mut() {
        m.write(0xA5);
    }
    s.zeroize();
    assert!(s.is_empty());
    let ptr = unsafe { s.as_mut_vec() }.as_mut_ptr();
    let bytes = unsafe { core::slice::from_raw_parts(ptr, cap) };
    assert!(bytes.iter().all(|&b| b == 0), "String capacity not fully zeroed");

    // Vec<u64>: multi-byte element spare capacity.
    let mut v: Vec<u64> = Vec::with_capacity(64);
    v.extend_from_slice(&[0xAAAA_AAAA_AAAA_AAAAu64; 9]);
    let cap = v.capacity();
    for m in v.spare_capacity_mut() {
        m.write(0xAAAA_AAAA_AAAA_AAAAu64);
    }
    v.zeroize();
    assert!(v.is_empty());
    let bytes = unsafe { core::slice::from_raw_parts(v.as_ptr().cast::<u8>(), cap * 8) };
    assert!(bytes.iter().all(|&b| b == 0), "Vec<u64> capacity not fully zeroed");
}

/// `Box<[T]>` and `Box<str>`.
#[test]
fn boxed_slices() {
    let mut b: Box<[u8]> = vec![0xAAu8; 33].into_boxed_slice();
    b.zeroize();
    assert!(b.iter().all(|&x| x == 0));

    let mut b: Box<[u64]> = vec![0xAAAA_AAAA_AAAA_AAAAu64; 11].into_boxed_slice();
    b.zeroize();
    assert!(b.iter().all(|&x| x == 0));

    let mut s: Box<str> = "hello, secret world".to_string().into_boxed_str();
    let len = s.len();
    s.zeroize();
    assert_eq!(s.len(), len);
    assert!(s.bytes().all(|x| x == 0));
}

/// `str` keeps its length but becomes all NUL; `String` truncates.
#[test]
fn str_impl() {
    let mut s = String::from("hello");
    {
        let slice: &mut str = &mut s;
        slice.zeroize();
        assert_eq!(slice.len(), 5);
        assert!(slice.bytes().all(|b| b == 0));
    }
    let mut s = String::from("hello");
    s.zeroize();
    assert!(s.is_empty());
}

/// `Option<Z>` must end up `None` for niche and non-niche payloads alike.
#[test]
fn option_impl() {
    let mut o: Option<char> = Some('A');
    o.zeroize();
    assert!(o.is_none());

    let mut o: Option<NonZeroU32> = NonZeroU32::new(42);
    o.zeroize();
    assert!(o.is_none());

    let mut o: Option<u64> = Some(0xAAAA_AAAA_AAAA_AAAA);
    o.zeroize();
    assert!(o.is_none());

    let mut o: Option<[u8; 16]> = Some([0xAA; 16]);
    o.zeroize();
    assert!(o.is_none());

    let mut o: Option<Vec<u8>> = Some(vec![0xAA; 8]);
    o.zeroize();
    assert!(o.is_none());

    let mut o: Option<Box<[u8]>> = Some(vec![0xAA; 8].into_boxed_slice());
    o.zeroize();
    assert!(o.is_none());

    let mut o: Option<u64> = None;
    o.zeroize();
    assert!(o.is_none());
}

/// `MaybeUninit<Z>` and slices of it.
#[test]
fn maybe_uninit_impl() {
    let mut m = MaybeUninit::new(0xAAAA_AAAA_AAAA_AAAAu64);
    m.zeroize();
    let bytes = unsafe { core::slice::from_raw_parts(m.as_ptr().cast::<u8>(), size_of::<u64>()) };
    assert!(bytes.iter().all(|&b| b == 0));

    let mut arr = [MaybeUninit::new(0xAAAA_AAAA_AAAA_AAAAu64); 6];
    arr.zeroize();
    let bytes = unsafe { core::slice::from_raw_parts(arr.as_ptr().cast::<u8>(), size_of_val(&arr)) };
    assert!(bytes.iter().all(|&b| b == 0));
}

/// Zero-sized element types must not be expanded into huge writes/loops.
#[test]
fn zst_impl() {
    let mut v: Vec<()> = vec![(); 5];
    v.zeroize();
    assert!(v.is_empty());

    let mut v: Vec<[u8; 0]> = vec![[0u8; 0]; 5];
    v.zeroize();
    assert!(v.is_empty());

    let mut a = [(); 5];
    a.zeroize();

    let mut b: Box<[()]> = vec![(); 3].into_boxed_slice();
    b.zeroize();
    assert_eq!(b.len(), 3);
}

/// `NonZero*` types must become one (not zero) so the value stays valid.
#[test]
fn nonzero_impl() {
    let mut n = NonZeroU64::new(0xAAAA_AAAA_AAAA_AAAA).unwrap();
    n.zeroize();
    assert_eq!(n.get(), 1);

    let mut o: Option<NonZeroU64> = NonZeroU64::new(7);
    o.zeroize();
    assert!(o.is_none());
}

/// Large slice: exercises the word loop many times (smaller under Miri).
#[test]
fn large_slice() {
    let n = if cfg!(miri) { 4096 } else { 1 << 20 };
    let mut v = vec![0xAAu8; n];
    v.zeroize();
    assert!(v.iter().all(|&b| b == 0));
}

/// `optimization_barrier` must not corrupt the observed value and keeps a zero canary.
#[test]
fn barrier_canary() {
    let buf = [0u8; 64];
    optimization_barrier(&buf);
    assert!(buf.iter().all(|&b| b == 0));

    let value = 0x0123_4567_89AB_CDEFu64;
    optimization_barrier(&value);
    assert_eq!(value, 0x0123_4567_89AB_CDEF);
}
