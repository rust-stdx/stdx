// Copyright 2015 Joe Neeman
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!
A crate for substring searching, with functionality similar to the `memmem`
function in C.

The main trait is [`Searcher`], which finds fixed byte-strings in longer
byte-strings. The only implementer is [`TwoWaySearcher`], a copy of the two-way
search implementation from Rust's standard library adapted to search `&[u8]`.

# Example

```rust
use memmem::{Searcher, TwoWaySearcher};
let search = TwoWaySearcher::new("dog".as_bytes());
assert_eq!(search.search_in("The quick brown fox jumped over the lazy dog.".as_bytes()), Some(41));
```
*/

mod two_way;

pub use two_way::TwoWaySearcher;

/// A trait that searches for patterns in byte-strings.
pub trait Searcher {
    /// Search for something in a byte-string. Returns the starting index of the match, if found.
    fn search_in(&self, haystack: &[u8]) -> Option<usize>;
}
