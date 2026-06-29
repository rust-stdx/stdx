mod mlkem;
mod mlkem1024;
mod mlkem768;

pub use mlkem::{MlKemError, SHARED_SECRET_SIZE};
#[cfg(feature = "random")]
pub use mlkem768::generate_keypair_768;
pub(crate) use mlkem768::generate_keypair_768_derand;
pub use mlkem768::{CIPHERTEXT_SIZE_768, PUBLIC_KEY_SIZE_768, PublicKey768, SECRET_KEY_SIZE_768, SecretKey768};
#[cfg(feature = "random")]
pub use mlkem1024::generate_keypair_1024;
pub use mlkem1024::{CIPHERTEXT_SIZE_1024, PUBLIC_KEY_SIZE_1024, PublicKey1024, SECRET_KEY_SIZE_1024, SecretKey1024};
