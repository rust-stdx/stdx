//! ML-KEM post-quantum key encapsulation mechanism (KEM) standardized in FIPS 203.

mod mlkem;
mod mlkem1024;
mod mlkem768;

pub use mlkem::{MlKemError, SHARED_SECRET_SIZE};
pub use mlkem768::{CIPHERTEXT_SIZE_768, PUBLIC_KEY_SIZE_768, PublicKey768, SECRET_KEY_SIZE_768, SecretKey768};
pub use mlkem1024::{CIPHERTEXT_SIZE_1024, PUBLIC_KEY_SIZE_1024, PublicKey1024, SECRET_KEY_SIZE_1024, SecretKey1024};
