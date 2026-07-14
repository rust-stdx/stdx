# Design principles and lessons learned

`stdx` needs funding to thrive. Free work never has been and never will be sustainable.

The scope of an "extended standard library" is almost infinite. We need to focus on what really matters.

We need to build libraries that can scale from embedded systems where allocations can be fatal (e.g. heap fragmentation or even no heap at all) but inputs / outputs are controlled, to big servers where allocations are possible. `smallvec` and similar libraries are your best friends. See *[smallvec is probably one of the most underrated Rust crates](https://kerkour.com/smallvec-rust)*.


Some popular crates are very high-quality, are widely adopted and have very few or none transitive dependencies (e.g. `serde`, `bytes`). We should not re-implement them unless **strictly** necessary.
