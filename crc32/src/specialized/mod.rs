#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod implementation {
    pub use super::pclmulqdq::State;
}

#[cfg(all(feature = "nightly", target_arch = "aarch64"))]
mod implementation {
    pub use super::aarch64::State;
}

#[cfg(not(any(
    any(target_arch = "x86", target_arch = "x86_64"),
    all(feature = "nightly", target_arch = "aarch64"),
)))]
mod implementation {
    #[derive(Clone)]
    pub enum State {}

    impl State {
        pub fn new(_: u32) -> Option<Self> {
            None
        }

        pub fn update(&mut self, _buf: &[u8]) {
            match *self {}
        }

        pub fn finalize(self) -> u32 {
            match self {}
        }

        pub fn reset(&mut self) {
            match *self {}
        }

        pub fn combine(&mut self, _other: u32, _amount: u64) {
            match *self {}
        }
    }
}

pub use self::implementation::State;
