use constant_time_eq::constant_time_eq;

/// A fixed-capacity, stack-allocated bytes buffer of capacity `N`.
/// Use [`Self::as_ref`] to get the bytes as a `&[u8]` and [`Self::as_mut`] to get the bytes as a `&mut [u8]`.
/// Comparing `Bytes` is a constant-time operation.
#[derive(Copy, Clone)]
pub(crate) struct Bytes<const N: usize> {
    pub(crate) bytes: [u8; N],
    pub(crate) length: u16,
}

impl<const N: usize> Bytes<N> {
    #[inline]
    pub(crate) fn new() -> Bytes<N> {
        assert!(N <= u16::MAX as usize);
        return Bytes {
            bytes: [0u8; N],
            length: 0,
        };
    }

    #[inline]
    pub fn len(&self) -> usize {
        return self.length as usize;
    }

    #[inline]
    pub(crate) fn with_length(length: usize) -> Bytes<N> {
        assert!(N <= u16::MAX as usize && length <= u16::MAX as usize);
        assert!(length <= N, "length exceeds capacity");
        return Bytes {
            bytes: [0u8; N],
            length: length as u16,
        };
    }

    #[inline]
    pub(crate) fn push(&mut self, byte: u8) {
        assert!(self.length as usize + 1 <= N);
        self.bytes[self.length as usize] = byte;
        self.length += 1;
    }

    #[inline]
    pub(crate) fn append(&mut self, data: &[u8]) {
        assert!(self.length as usize + data.len() <= N);
        self.bytes[self.length as usize..data.len() + self.length as usize].copy_from_slice(data);
        self.length += data.len() as u16;
    }
}

impl<const N: usize, const L: usize> From<[u8; L]> for Bytes<N> {
    #[inline]
    fn from(data: [u8; L]) -> Self {
        const {
            assert!(L <= N);
        }

        let mut bytes = [0u8; N];
        bytes[..L].copy_from_slice(&data);

        Bytes {
            bytes,
            length: L as u16,
        }
    }
}

impl<const N: usize, const L: usize> From<&[u8; L]> for Bytes<N> {
    #[inline]
    fn from(data: &[u8; L]) -> Self {
        const {
            assert!(L <= N);
        }

        let mut bytes = [0u8; N];
        bytes[..L].copy_from_slice(data);

        Bytes {
            bytes,
            length: L as u16,
        }
    }
}

impl<const N: usize> PartialEq for Bytes<N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        constant_time_eq(&self, &other)
    }
}

impl<const N: usize> Eq for Bytes<N> {}

impl<const N: usize> core::ops::Deref for Bytes<N> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.bytes[..self.length as usize]
    }
}

impl<const N: usize> AsMut<[u8]> for Bytes<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.bytes[..self.length as usize]
    }
}

/// A stack-allocated bytes buffer.
/// Use [`Self::as_ref`] to get the bytes as a `&[u8]` and [`Self::as_mut`] to get the bytes as a `&mut [u8]`.
/// Comparing `Hash` is a constant-time operation.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Hash(pub(crate) Bytes<64>);

/// implement the required public methods for `Type` to be used as a bytes buffer.
macro_rules! impl_bytes {
    ($name:ident($inner:ty)) => {
        impl $name {
            #[inline]
            pub fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl AsRef<[u8]> for $name {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }

        impl core::ops::Deref for $name {
            type Target = [u8];
            fn deref(&self) -> &[u8] {
                &self.0
            }
        }

        impl AsMut<[u8]> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut [u8] {
                self.0.as_mut()
            }
        }

        impl PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl Eq for $name {}
    };
}

impl_bytes!(Hash(Bytes<64>));
