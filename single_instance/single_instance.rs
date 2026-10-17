//! A rust library for single instance application.
//!
//! single-instance provides a single API to check if there are any other running instance.
//!
//! ## Detail
//! On Linux and Android, [`SingleInstance::new`] binds an abstract unix domain socket
//! with the given name. On Apple platforms (macOS, iOS, tvOS, watchOS), it creates or
//! opens the file at the given path and takes an exclusive advisory lock on it.
//!
//! ### Examples
//! ```rust
//! extern crate single_instance;
//!
//! use std::thread;
//! use single_instance::SingleInstance;
//!
//! fn main() {
//!     let instance = SingleInstance::new("whatever").unwrap();
//!     assert!(instance.is_single());
//! }
//! ```
//!
//! #### Keeping the instance alive for the process lifetime
//!
//! The [`SingleInstance`] must stay alive for as long as the exclusivity
//! guarantee is needed, because dropping it releases the lock. Store it for the
//! lifetime of the process, for example by leaking it:
//!
//! ```rust
//! use single_instance::SingleInstance;
//!
//! fn ensure_single_instance(unique_id: &str) -> bool {
//!     let instance = Box::new(SingleInstance::new(unique_id).unwrap());
//!     if instance.is_single() {
//!         Box::leak(instance);
//!         true
//!     } else {
//!         false
//!     }
//! }
//! ```

/// The error returned when a [`SingleInstance`] cannot be created.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The underlying operating system operation failed.
    ///
    /// Binding the abstract socket (Linux/Android) or creating and locking the
    /// file (Apple platforms) returned an error.
    #[error("single instance error")]
    Io(#[from] std::io::Error),
}

pub use self::inner::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod inner {
    #[cfg(target_os = "android")]
    use std::os::android::net::SocketAddrExt;
    #[cfg(target_os = "linux")]
    use std::os::linux::net::SocketAddrExt;
    use std::{
        io,
        os::unix::net::{SocketAddr, UnixListener},
    };

    use super::Error;

    /// A struct representing one running instance.
    ///
    /// The abstract socket is owned by the instance and released on drop, at
    /// which point another process may claim the same name.
    pub struct SingleInstance {
        maybe_listener: Option<UnixListener>,
    }

    impl SingleInstance {
        /// Returns a new SingleInstance object.
        ///
        /// Returns an error if the abstract socket address cannot be built or if
        /// binding it fails for a reason other than another instance already
        /// holding the name.
        pub fn new(name: &str) -> Result<Self, Error> {
            let addr = SocketAddr::from_abstract_name(name.as_bytes())?;

            let maybe_listener = match UnixListener::bind_addr(&addr) {
                Ok(listener) => Some(listener),
                // Another instance already bound this abstract address.
                Err(e) if e.kind() == io::ErrorKind::AddrInUse => None,
                Err(e) => return Err(e.into()),
            };

            Ok(Self {
                maybe_listener,
            })
        }

        /// Returns whether this instance is single.
        pub fn is_single(&self) -> bool {
            self.maybe_listener.is_some()
        }
    }
}

#[cfg(target_vendor = "apple")]
mod inner {
    use std::{
        fs::{File, TryLockError},
        path::Path,
    };

    use super::Error;

    /// A struct representing one running instance.
    ///
    /// The file is kept open for the lifetime of the instance; the advisory lock
    /// is released on drop.
    pub struct SingleInstance {
        _file: File,
        is_single: bool,
    }

    impl SingleInstance {
        /// Returns a new SingleInstance object.
        ///
        /// Returns an error if the file cannot be created or opened, or if locking
        /// it fails for a reason other than the lock already being held by another
        /// instance.
        pub fn new(name: &str) -> Result<Self, Error> {
            let path = Path::new(name);
            let file = if path.exists() {
                File::open(path)?
            } else {
                File::create(path)?
            };

            let is_single = match file.try_lock() {
                Ok(()) => true,
                Err(TryLockError::WouldBlock) => false,
                Err(TryLockError::Error(e)) => return Err(e.into()),
            };

            Ok(Self {
                _file: file,
                is_single,
            })
        }

        /// Returns whether this instance is single.
        pub fn is_single(&self) -> bool {
            self.is_single
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static UNIQ_ID: &str = "aa2d0258-ffe9-11e7-ba89-0ed5f89f718b";
    #[test]
    fn test_single_instance() {
        {
            let instance_a = SingleInstance::new(UNIQ_ID).unwrap();
            assert!(instance_a.is_single());
            let instance_b = SingleInstance::new(UNIQ_ID).unwrap();
            assert!(!instance_b.is_single());
        }
        let instance_c = SingleInstance::new(UNIQ_ID).unwrap();
        assert!(instance_c.is_single());
    }

    #[test]
    fn test_create_error() {
        // On Linux and Android the name is an abstract socket name, so any
        // string is a valid identifier. The file backends fail when the path
        // cannot be created.
        let result = SingleInstance::new("/nonexistent-stdx-dir/single_instance.lock");
        #[cfg(target_vendor = "apple")]
        assert!(result.is_err());
        let _ = result;
    }
}
