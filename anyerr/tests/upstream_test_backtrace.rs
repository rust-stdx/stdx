#![cfg(feature = "std")]
#![allow(clippy::let_underscore_untyped)]

#[test]
fn test_backtrace() {
    use anyerr::anyhow;

    let error = anyhow!("oh no!");
    let _ = error.backtrace();
}
