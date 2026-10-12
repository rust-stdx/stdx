#![allow(unsafe_code)]

/// Like `slice::get_disjoint_mut` but with optional indices,
/// allowing for absent keys from the user's original request.
#[track_caller]
pub(super) fn get_disjoint_opt_mut<T, const N: usize>(
    entries: &mut [T],
    indices: [Option<usize>; N],
) -> [Option<&mut T>; N] {
    // SAFETY: Can't allow duplicate indices as we would return mutable refs to the same data.
    let len = entries.len();
    for i in 0..N {
        if let Some(idx) = indices[i] {
            if idx >= len {
                unreachable!("`get_index_of` returned an out-of-bounds index");
            } else if indices[..i].contains(&Some(idx)) {
                panic!("duplicate keys found");
            }
        }
    }

    let entries_ptr = entries.as_mut_ptr();
    indices.map(move |idx_opt| {
        match idx_opt {
            Some(idx) => {
                // SAFETY: The base pointer is valid as it comes from a slice and the reference is always
                // in-bounds & unique as we've already checked the indices above.
                Some(unsafe { &mut *entries_ptr.add(idx) })
            }
            None => None,
        }
    })
}
