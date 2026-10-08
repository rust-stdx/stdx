/// Create an [`IndexMap`][crate::IndexMap] from a list of key-value pairs
/// and a [`BuildHasherDefault`][core::hash::BuildHasherDefault]-wrapped custom hasher.
///
/// ## Example
///
/// ```
/// use indexmap::indexmap_with_default;
/// use std::collections::hash_map::DefaultHasher;
///
/// let map = indexmap_with_default!{
///     DefaultHasher;
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
///
/// // "a" is the first key
/// assert_eq!(map.keys().next(), Some(&"a"));
/// ```
///
/// This can also be initialized in `const` contexts:
///
/// ```
/// use indexmap::{IndexMap, indexmap_with_default};
/// use std::collections::hash_map::DefaultHasher;
/// use std::hash::BuildHasherDefault;
/// use std::sync::Mutex;
///
/// type DefaultHashBuilder = BuildHasherDefault<DefaultHasher>;
///
/// static GLOBAL: Mutex<IndexMap<String, i32, DefaultHashBuilder>> =
///     Mutex::new(indexmap_with_default!());
///
/// if let Ok(mut map) = GLOBAL.lock() {
///     map.insert("a".into(), 1);
///     map.insert("b".into(), 2);
/// }
///
/// assert_eq!(GLOBAL.lock().unwrap()["a"], 1);
/// assert_eq!(GLOBAL.lock().unwrap()["b"], 2);
/// ```
#[macro_export]
macro_rules! indexmap_with_default {
    () => { const {
        $crate::IndexMap::with_hasher(
            // Let type inference figure out the hasher:
            ::core::hash::BuildHasherDefault::new(),
        )
    }};
    ($H:ty $(;)?) => { const {
        $crate::IndexMap::with_hasher(
            // Specify your custom `H` (must implement Default + Hasher) as the hasher:
            ::core::hash::BuildHasherDefault::<$H>::new(),
        )
    }};
    ($H:ty; $($key:expr => $value:expr),+ $(,)?) => {{
        let mut map = $crate::IndexMap::with_capacity_and_hasher(
            // Note: `stringify!($key)` is just here to consume the repetition,
            // but we throw away that string literal during constant evaluation.
            const { <[()]>::len(&[$({ stringify!($key); }),*]) },
            // Specify your custom `H` (must implement Default + Hasher) as the hasher:
            ::core::hash::BuildHasherDefault::<$H>::new(),
        );
        $(
            map.insert($key, $value);
        )+
        map
    }};
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[macro_export]
/// Create an [`IndexMap`][crate::IndexMap] from a list of key-value pairs
///
/// ## Example
///
/// ```
/// use indexmap::indexmap;
///
/// let map = indexmap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
///
/// // "a" is the first key
/// assert_eq!(map.keys().next(), Some(&"a"));
/// ```
macro_rules! indexmap {
    () => { $crate::IndexMap::new() };
    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut map = $crate::IndexMap::with_capacity(
            // Note: `stringify!($key)` is just here to consume the repetition,
            // but we throw away that string literal during constant evaluation.
            const { <[()]>::len(&[$({ stringify!($key); }),*]) },
        );
        $(
            map.insert($key, $value);
        )+
        map
    }};
}

/// Create an [`IndexSet`][crate::IndexSet] from a list of values
/// and a [`BuildHasherDefault`][core::hash::BuildHasherDefault]-wrapped custom hasher.
///
/// ## Example
///
/// ```
/// use indexmap::indexset_with_default;
/// use std::collections::hash_map::DefaultHasher;
///
/// let set = indexset_with_default!{
///     DefaultHasher;
///     "a",
///     "b",
/// };
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
///
/// // "a" is the first value
/// assert_eq!(set.iter().next(), Some(&"a"));
/// ```
///
/// This can also be initialized in `const` contexts:
///
/// ```
/// use indexmap::{IndexSet, indexset_with_default};
/// use std::collections::hash_map::DefaultHasher;
/// use std::hash::BuildHasherDefault;
/// use std::sync::Mutex;
///
/// type DefaultHashBuilder = BuildHasherDefault<DefaultHasher>;
///
/// static INTERN: Mutex<IndexSet<String, DefaultHashBuilder>> =
///     Mutex::new(indexset_with_default!());
///
/// if let Ok(mut set) = INTERN.lock() {
///     set.insert("a".into());
///     set.insert("b".into());
///     set.insert("c".into());
/// }
///
/// assert!(INTERN.lock().unwrap().contains("a"));
/// assert!(INTERN.lock().unwrap().contains("b"));
/// assert!(INTERN.lock().unwrap().contains("c"));
/// ```
#[macro_export]
macro_rules! indexset_with_default {
    () => { const {
        $crate::IndexSet::with_hasher(
            // Let type inference figure out the hasher:
            ::core::hash::BuildHasherDefault::new(),
        )
    }};
    ($H:ty $(;)?) => { const {
        $crate::IndexSet::with_hasher(
            // Specify your custom `H` (must implement Default + Hasher) as the hasher:
            ::core::hash::BuildHasherDefault::<$H>::new(),
        )
    }};
    ($H:ty; $($value:expr),+ $(,)?) => {{
        let mut set = $crate::IndexSet::with_capacity_and_hasher(
            // Note: `stringify!($value)` is just here to consume the repetition,
            // but we throw away that string literal during constant evaluation.
            const { <[()]>::len(&[$({ stringify!($value); }),*]) },
            // Specify your custom `H` (must implement Default + Hasher) as the hasher:
            ::core::hash::BuildHasherDefault::<$H>::new(),
        );
        $(
            set.insert($value);
        )+
        set
    }};
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[macro_export]
/// Create an [`IndexSet`][crate::IndexSet] from a list of values
///
/// ## Example
///
/// ```
/// use indexmap::indexset;
///
/// let set = indexset!{
///     "a",
///     "b",
/// };
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
///
/// // "a" is the first value
/// assert_eq!(set.iter().next(), Some(&"a"));
/// ```
macro_rules! indexset {
    () => { $crate::IndexSet::new() };
    ($($value:expr),+ $(,)?) => {{
        let mut set = $crate::IndexSet::with_capacity(
            // Note: `stringify!($value)` is just here to consume the repetition,
            // but we throw away that string literal during constant evaluation.
            const { <[()]>::len(&[$({ stringify!($value); }),*]) },
        );
        $(
            set.insert($value);
        )+
        set
    }};
}

// generate all the Iterator methods by just forwarding to the underlying
// self.iter and mapping its element.
macro_rules! iterator_methods {
    // $map_elt is the mapping function from the underlying iterator's element
    // same mapping function for both options and iterators
    ($map_elt:expr) => {
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map($map_elt)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }

        fn count(self) -> usize {
            self.iter.len()
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.iter.nth(n).map($map_elt)
        }

        fn last(mut self) -> Option<Self::Item> {
            self.next_back()
        }

        fn collect<C>(self) -> C
        where
            C: FromIterator<Self::Item>,
        {
            // NB: forwarding this directly to standard iterators will
            // allow it to leverage unstable traits like `TrustedLen`.
            self.iter.map($map_elt).collect()
        }
    };
}

macro_rules! double_ended_iterator_methods {
    // $map_elt is the mapping function from the underlying iterator's element
    // same mapping function for both options and iterators
    ($map_elt:expr) => {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.iter.next_back().map($map_elt)
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            self.iter.nth_back(n).map($map_elt)
        }
    };
}
