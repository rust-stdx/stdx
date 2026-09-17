use std::cmp::Ordering;

include!("mime_types.rs");
include!(env!("MIME_TYPES_GENERATED_PATH"));

#[cfg(feature = "rev-mappings")]
#[derive(Copy, Clone)]
struct TopLevelExts {
    start: usize,
    end: usize,
    subs: &'static [(&'static str, (usize, usize))],
}

pub fn get_mime_types(ext: &str) -> Option<&'static [&'static str]> {
    map_lookup(MIME_TYPES, &ext)
}

#[cfg(feature = "rev-mappings")]
pub fn get_extensions(toplevel: &str, sublevel: &str) -> Option<&'static [&'static str]> {
    if toplevel == "*" {
        return Some(EXTS);
    }

    let top = map_lookup(REV_MAPPINGS, toplevel)?;

    if sublevel == "*" {
        return Some(&EXTS[top.start..top.end]);
    }

    let sub = map_lookup(&top.subs, sublevel)?;
    Some(&EXTS[sub.0..sub.1])
}

/// Looks up `key` in `map`, comparing ASCII letters case-insensitively.
///
/// `map` must be sorted by its keys in ascending byte order; stored keys are
/// lowercase ASCII, so folding only the query keeps the ordering valid. No
/// allocation is performed.
fn map_lookup<K, V>(map: &'static [(K, V)], key: &str) -> Option<V>
where
    K: Copy + Into<&'static str>,
    V: Copy,
{
    let mut left = 0;
    let mut right = map.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match cmp_ignore_ascii_case(map[mid].0.into(), key) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
            Ordering::Equal => return Some(map[mid].1),
        }
    }

    None
}

/// Compares two strings byte-wise, folding ASCII letters to lowercase.
fn cmp_ignore_ascii_case(a: &str, b: &str) -> Ordering {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let common = a.len().min(b.len());

    for i in 0..common {
        let (ca, cb) = (a[i].to_ascii_lowercase(), b[i].to_ascii_lowercase());
        if ca != cb {
            return ca.cmp(&cb);
        }
    }

    a.len().cmp(&b.len())
}
