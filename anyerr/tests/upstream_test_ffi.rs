#![cfg(feature = "std")]
#![deny(improper_ctypes, improper_ctypes_definitions)]

use anyerr::anyhow;

#[unsafe(no_mangle)]
pub extern "C" fn anyhow1(err: anyerr::Error) {
    println!("{:?}", err);
}

#[unsafe(no_mangle)]
pub extern "C" fn anyhow2(err: &mut Option<anyerr::Error>) {
    *err = Some(anyhow!("ffi error"));
}

#[unsafe(no_mangle)]
pub extern "C" fn anyhow3() -> Option<anyerr::Error> {
    Some(anyhow!("ffi error"))
}
