//! COSE_Key and COSE_KeySet support.

use core::fmt;

use cbor::{Decoder, Encoder, Type, Write};
use heapless::Vec as HeaplessVec;

use crate::{
    Algorithm, Curve, Error, KeyOp, KeyType, Label, Result, RsaPublicKey, SymmetricKey,
    bytes::Bytes,
    codec::{
        Cursor, MAX_MAP_ENTRIES, cbor_len_array, cbor_len_bytes, cbor_len_int, cbor_len_uint, decode_bytes, decode_i64,
        decoder, find_value, label_len, options, take_raw, validate_array, validate_map, value_len, write_label,
        write_value,
    },
    header::HeaderValue,
};

/// A COSE_Key object.
///
/// The key is kept in its encoded form. This is necessary because several COSE
/// key labels have different meanings depending on `kty` (`-1` is `crv` for
/// EC2/OKP, `k` for Symmetric, `n` for RSA, and `pub` for AKP).
#[derive(Clone)]
pub struct CoseKey<'a> {
    raw: Bytes<'a>,
}

impl fmt::Debug for CoseKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CoseKey")
            .field("kty", &self.kty().ok())
            .field("kid", &self.kid())
            .field("alg", &self.alg())
            .finish()
    }
}

/// A two-coordinate elliptic-curve public/private key view.
pub struct Ec2Key<'a> {
    /// The curve.
    pub crv: Curve,
    /// The x-coordinate, when present.
    pub x: Option<&'a [u8]>,
    /// The y-coordinate or compressed-point parity, when present.
    pub y: Option<YCoordinate<'a>>,
    /// The private scalar, when present.
    pub d: Option<&'a [u8]>,
}

/// The y-coordinate of an EC2 key.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum YCoordinate<'a> {
    /// A full big-endian coordinate.
    Bytes(&'a [u8]),
    /// A compressed-point parity flag.
    Sign(bool),
}

/// An Octet Key Pair view.
pub struct OkpKey<'a> {
    /// The curve.
    pub crv: Curve,
    /// The public key, when present.
    pub x: Option<&'a [u8]>,
    /// The private key/seed, when present.
    pub d: Option<&'a [u8]>,
}

/// An RSA key view.
pub struct RsaKey<'a> {
    /// The modulus.
    pub n: &'a [u8],
    /// The public exponent.
    pub e: &'a [u8],
    /// The private exponent, when present.
    pub d: Option<&'a [u8]>,
}

/// An Algorithm Key Pair view (RFC 9964).
pub struct AkpKey<'a> {
    /// The algorithm this key is restricted to.
    pub alg: Algorithm,
    /// The public key, when present.
    pub public: Option<&'a [u8]>,
    /// The private key/seed, when present.
    pub private: Option<&'a [u8]>,
}

impl<'a> CoseKey<'a> {
    /// Parses and structurally validates a COSE_Key map.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidStructure`] for an empty slice,
    /// [`Error::Decode`] for malformed CBOR, [`Error::IndefiniteLength`] for
    /// an indefinite-length map, [`Error::DuplicateLabel`] for duplicate
    /// labels, [`Error::TooManyEntries`] for more than 32 entries, and
    /// [`Error::TrailingBytes`] if extra bytes follow the map.
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self> {
        if bytes.is_empty() {
            return Err(Error::InvalidStructure);
        }
        validate_map(bytes)?;
        Ok(Self {
            raw: Bytes::borrowed(bytes),
        })
    }

    /// Parses a COSE_Key map and takes ownership of its bytes.
    #[cfg(feature = "alloc")]
    pub fn from_owned(bytes: alloc::vec::Vec<u8>) -> Result<Self> {
        if bytes.is_empty() {
            return Err(Error::InvalidStructure);
        }
        validate_map(&bytes)?;
        Ok(Self {
            raw: Bytes::owned(bytes),
        })
    }

    /// Starts building a COSE_Key with the given key type.
    #[inline]
    pub fn builder<'b>(kty: KeyType) -> KeyBuilder<'b> {
        KeyBuilder::new(kty)
    }

    /// Returns the encoded COSE_Key map.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.raw.as_slice()
    }

    /// Copies the encoded COSE_Key map into a new `Vec`.
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> alloc::vec::Vec<u8> {
        self.as_bytes().to_vec()
    }

    /// Returns the required `kty` parameter.
    pub fn kty(&self) -> Result<KeyType> {
        let raw = self.raw(Label::Int(1)).ok_or(Error::InvalidKey)?;
        Ok(KeyType::from_i64(decode_i64(raw)?))
    }

    /// Returns the optional `kid` parameter.
    pub fn kid(&self) -> Option<&[u8]> {
        let raw = self.raw(Label::Int(2))?;
        decode_bytes(raw).ok()
    }

    /// Returns the optional `alg` parameter.
    pub fn alg(&self) -> Option<Algorithm> {
        let raw = self.raw(Label::Int(3))?;
        decode_i64(raw).ok().map(Algorithm::from_i64)
    }

    /// Returns the optional `key_ops` array.
    pub fn key_ops(&self) -> Result<KeyOps<'_>> {
        let Some(raw) = self.raw(Label::Int(4)) else {
            return Ok(KeyOps {
                d: decoder(&[]),
                remaining: 0,
            });
        };
        let mut d = decoder(raw);
        let remaining = d.array()?.ok_or(Error::IndefiniteLength)?;
        if remaining > MAX_MAP_ENTRIES {
            return Err(Error::TooManyEntries);
        }
        Ok(KeyOps {
            d,
            remaining,
        })
    }

    /// Returns the raw encoding of a key parameter value.
    pub fn raw<'b>(&'b self, label: impl Into<Label<'b>>) -> Option<&'b [u8]> {
        find_value(self.as_bytes(), label.into()).ok().flatten()
    }

    /// Decodes a key parameter with `cbor`.
    ///
    /// Nested maps inside private parameter values are decoded with
    /// duplicate keys allowed; the application owns their interpretation.
    pub fn decode<'b, T>(&'b self, label: impl Into<Label<'b>>) -> Result<Option<T>>
    where
        T: cbor::Decode<'b>,
    {
        let Some(raw) = self.raw(label) else {
            return Ok(None);
        };
        Ok(Some(cbor::from_slice_with_options(raw, options())?))
    }

    /// Iterates over all key parameters.
    pub fn iter(&self) -> KeyIter<'_> {
        let input = self.as_bytes();
        let mut d = decoder(input);
        let remaining = match d.map() {
            Ok(Some(len)) => len,
            _ => 0,
        };
        KeyIter {
            input,
            d,
            remaining,
        }
    }

    /// Returns the EC2 key parameters.
    pub fn ec2(&self) -> Result<Ec2Key<'_>> {
        if self.kty()? != KeyType::Ec2 {
            return Err(Error::InvalidKey);
        }

        let crv = Curve::from_i64(decode_i64(self.raw(Label::Int(-1)).ok_or(Error::InvalidKey)?)?);
        let x = self.raw(Label::Int(-2)).map(decode_bytes).transpose()?;
        let y = match self.raw(Label::Int(-3)) {
            Some(raw) => {
                let mut d = decoder(raw);
                let value = match d.datatype()? {
                    Type::Bytes => YCoordinate::Bytes(d.bytes()?),
                    Type::Bool => YCoordinate::Sign(d.bool()?),
                    _ => return Err(Error::InvalidKey),
                };
                if d.position() != raw.len() {
                    return Err(Error::TrailingBytes);
                }
                Some(value)
            }
            None => None,
        };
        let d = self.raw(Label::Int(-4)).map(decode_bytes).transpose()?;

        Ok(Ec2Key {
            crv,
            x,
            y,
            d,
        })
    }

    /// Returns the OKP key parameters.
    pub fn okp(&self) -> Result<OkpKey<'_>> {
        if self.kty()? != KeyType::Okp {
            return Err(Error::InvalidKey);
        }

        let crv = Curve::from_i64(decode_i64(self.raw(Label::Int(-1)).ok_or(Error::InvalidKey)?)?);
        let x = self.raw(Label::Int(-2)).map(decode_bytes).transpose()?;
        let d = self.raw(Label::Int(-4)).map(decode_bytes).transpose()?;

        Ok(OkpKey {
            crv,
            x,
            d,
        })
    }

    /// Returns the symmetric key value (`k`).
    pub fn symmetric(&self) -> Result<&[u8]> {
        if self.kty()? != KeyType::Symmetric {
            return Err(Error::InvalidKey);
        }
        decode_bytes(self.raw(Label::Int(-1)).ok_or(Error::InvalidKey)?)
    }

    /// Returns the RSA key parameters.
    pub fn rsa(&self) -> Result<RsaKey<'_>> {
        if self.kty()? != KeyType::Rsa {
            return Err(Error::InvalidKey);
        }

        let n = decode_bytes(self.raw(Label::Int(-1)).ok_or(Error::InvalidKey)?)?;
        let e = decode_bytes(self.raw(Label::Int(-2)).ok_or(Error::InvalidKey)?)?;
        let d = self.raw(Label::Int(-3)).map(decode_bytes).transpose()?;

        Ok(RsaKey {
            n,
            e,
            d,
        })
    }

    /// Returns the AKP key parameters.
    pub fn akp(&self) -> Result<AkpKey<'_>> {
        if self.kty()? != KeyType::Akp {
            return Err(Error::InvalidKey);
        }

        let alg = self.alg().ok_or(Error::InvalidKey)?;
        let public = self.raw(Label::Int(-1)).map(decode_bytes).transpose()?;
        let private = self.raw(Label::Int(-2)).map(decode_bytes).transpose()?;

        Ok(AkpKey {
            alg,
            public,
            private,
        })
    }

    /// Validates the key's structure, parameter types, and key material.
    ///
    /// For the key types implemented by this crate, the RSA modulus/exponent,
    /// EC point/scalar, and Ed25519 public key are parsed by the `crypto`
    /// crate. This means a structurally well-formed but cryptographically
    /// invalid key is rejected here, not only when it is converted.
    ///
    /// Unknown key types and unknown algorithms are rejected. Unknown or
    /// private-use *parameters* are preserved and ignored.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidKey`] for malformed or missing parameters and
    /// invalid key material, [`Error::UnsupportedKeyType`] for an unknown
    /// `kty`, [`Error::UnsupportedCurve`] for an unknown curve, and
    /// [`Error::UnsupportedAlgorithm`] when `alg` is incompatible with the
    /// key type or curve.
    pub fn validate(&self) -> Result<()> {
        validate_view(&self.view()?)
    }

    /// Validates the key for a specific algorithm and operation.
    ///
    /// In addition to [`CoseKey::validate`], this checks `key_ops`, algorithm
    /// compatibility, and that the key contains the material required by the
    /// operation (for example, a private scalar for signing).
    ///
    /// # Errors
    ///
    /// Propagates [`CoseKey::validate`] errors, and returns
    /// [`Error::UnsupportedKeyOperation`] when the operation is incompatible
    /// with the key type, algorithm, or `key_ops` list.
    pub fn validate_for(&self, alg: Algorithm, op: KeyOp) -> Result<()> {
        let view = self.view()?;
        validate_view(&view)?;

        match op {
            KeyOp::Sign | KeyOp::Verify => {
                if !alg.is_signature() {
                    return Err(Error::UnsupportedKeyOperation(op));
                }
            }
            KeyOp::MacCreate | KeyOp::MacVerify => {
                if !alg.is_mac() {
                    return Err(Error::UnsupportedKeyOperation(op));
                }
            }
            KeyOp::Unknown(_) => return Err(Error::UnsupportedKeyOperation(op)),
        }

        if !alg.is_compatible_key(view.kty, view.curve()) {
            return Err(Error::InvalidKey);
        }
        if let Some(key_alg) = view.alg
            && !key_alg.is_compatible(alg)
        {
            return Err(Error::InvalidKey);
        }
        if alg.is_mac() {
            match &view.material {
                Material::Symmetric {
                    k,
                } if k.len() >= 16 => {}
                _ => return Err(Error::InvalidKey),
            }
        }

        if !view.key_ops.is_empty() && !view.key_ops.contains(&op) {
            return Err(Error::UnsupportedKeyOperation(op));
        }

        match (op, view.kty) {
            (KeyOp::Sign, KeyType::Okp) => match &view.material {
                Material::Okp {
                    d, ..
                } if d.is_some() => {}
                _ => return Err(Error::InvalidKey),
            },
            (KeyOp::Sign, KeyType::Ec2) => match &view.material {
                Material::Ec2 {
                    d, ..
                } if d.is_some() => {}
                _ => return Err(Error::InvalidKey),
            },
            (KeyOp::Sign, KeyType::Akp) => match &view.material {
                Material::Akp {
                    private: Some(_), ..
                } => {}
                _ => return Err(Error::InvalidKey),
            },
            (KeyOp::Sign, KeyType::Rsa) => {
                return Err(Error::UnsupportedKeyOperation(op));
            }
            (KeyOp::Verify, KeyType::Symmetric) => {
                return Err(Error::UnsupportedKeyOperation(op));
            }
            (KeyOp::MacCreate | KeyOp::MacVerify, KeyType::Symmetric) => {}
            (KeyOp::MacCreate | KeyOp::MacVerify, _) => {
                return Err(Error::UnsupportedKeyOperation(op));
            }
            _ => {}
        }

        Ok(())
    }

    /// Parses the encoded key into a single-pass view used by validation.
    fn view(&self) -> Result<KeyView<'_>> {
        let bytes = self.as_bytes();
        let mut d = decoder(bytes);
        let len = d.map()?.ok_or(Error::IndefiniteLength)?;
        if len > MAX_MAP_ENTRIES {
            return Err(Error::TooManyEntries);
        }

        let mut kty = None;
        let mut alg = None;
        let mut key_ops: HeaplessVec<KeyOp, MAX_MAP_ENTRIES> = HeaplessVec::new();
        let mut field1 = None;
        let mut field2 = None;
        let mut field3 = None;
        let mut field4 = None;

        for _ in 0..len {
            let label = crate::codec::decode_label(&mut d)?;
            let start = d.position();
            d.skip()?;
            let raw = &bytes[start..d.position()];

            match label {
                Label::Int(1) => {
                    kty = Some(KeyType::from_i64(decode_i64(raw).map_err(|_| Error::InvalidKey)?));
                }
                Label::Int(2) => {
                    let value = decode_bytes(raw).map_err(|_| Error::InvalidKey)?;
                    if value.is_empty() {
                        return Err(Error::InvalidKey);
                    }
                }
                Label::Int(3) => {
                    alg = Some(Algorithm::from_i64(decode_i64(raw).map_err(|_| Error::InvalidKey)?));
                }
                Label::Int(4) => {
                    let mut ops = decoder(raw);
                    let count = ops
                        .array()
                        .map_err(|_| Error::InvalidKey)?
                        .ok_or(Error::IndefiniteLength)?;
                    if count > MAX_MAP_ENTRIES {
                        return Err(Error::TooManyEntries);
                    }
                    for _ in 0..count {
                        let value = ops.i64().map_err(|_| Error::InvalidKey)?;
                        key_ops
                            .push(KeyOp::from_i64(value))
                            .map_err(|_| Error::TooManyEntries)?;
                    }
                    if ops.position() != raw.len() {
                        return Err(Error::TrailingBytes);
                    }
                }
                Label::Int(-1) => field1 = Some(raw),
                Label::Int(-2) => field2 = Some(raw),
                Label::Int(-3) => field3 = Some(raw),
                Label::Int(-4) => field4 = Some(raw),
                _ => {}
            }
        }

        if d.position() != bytes.len() {
            return Err(Error::TrailingBytes);
        }

        let kty = kty.ok_or(Error::InvalidKey)?;
        let material = match kty {
            KeyType::Ec2 => {
                let raw = field1.ok_or(Error::InvalidKey)?;
                let crv = Curve::from_i64(decode_i64(raw).map_err(|_| Error::InvalidKey)?);
                let x = field2.map(decode_bytes).transpose().map_err(|_| Error::InvalidKey)?;
                let y = match field3 {
                    Some(raw) => {
                        let mut value = decoder(raw);
                        let y = match value.datatype()? {
                            Type::Bytes => YCoordinate::Bytes(value.bytes()?),
                            Type::Bool => YCoordinate::Sign(value.bool()?),
                            _ => return Err(Error::InvalidKey),
                        };
                        if value.position() != raw.len() {
                            return Err(Error::TrailingBytes);
                        }
                        Some(y)
                    }
                    None => None,
                };
                let d = field4.map(decode_bytes).transpose().map_err(|_| Error::InvalidKey)?;
                Material::Ec2 {
                    crv,
                    x,
                    y,
                    d,
                }
            }
            KeyType::Okp => {
                let raw = field1.ok_or(Error::InvalidKey)?;
                let crv = Curve::from_i64(decode_i64(raw).map_err(|_| Error::InvalidKey)?);
                let x = field2.map(decode_bytes).transpose().map_err(|_| Error::InvalidKey)?;
                let d = field4.map(decode_bytes).transpose().map_err(|_| Error::InvalidKey)?;
                Material::Okp {
                    crv,
                    x,
                    d,
                }
            }
            KeyType::Symmetric => {
                let k = decode_bytes(field1.ok_or(Error::InvalidKey)?).map_err(|_| Error::InvalidKey)?;
                Material::Symmetric {
                    k,
                }
            }
            KeyType::Rsa => {
                let n = decode_bytes(field1.ok_or(Error::InvalidKey)?).map_err(|_| Error::InvalidKey)?;
                let e = decode_bytes(field2.ok_or(Error::InvalidKey)?).map_err(|_| Error::InvalidKey)?;
                if let Some(raw) = field3 {
                    decode_bytes(raw).map_err(|_| Error::InvalidKey)?;
                }
                Material::Rsa {
                    n,
                    e,
                }
            }
            KeyType::Akp => {
                let public = field1.map(decode_bytes).transpose().map_err(|_| Error::InvalidKey)?;
                let private = field2.map(decode_bytes).transpose().map_err(|_| Error::InvalidKey)?;
                Material::Akp {
                    public,
                    private,
                }
            }
            KeyType::Unknown(value) => return Err(Error::UnsupportedKeyType(KeyType::Unknown(value))),
        };

        Ok(KeyView {
            kty,
            alg,
            key_ops,
            material,
        })
    }

    /// Returns an HMAC key after validating the algorithm and operation.
    pub fn symmetric_key(&self, alg: Algorithm, op: KeyOp) -> Result<SymmetricKey<'_>> {
        self.validate_for(alg, op)?;
        SymmetricKey::new(alg, self.symmetric()?)
    }

    /// Returns an RSA verification key after validating the algorithm and
    /// operation.
    pub fn rsa_public_key(&self, alg: Algorithm, op: KeyOp) -> Result<RsaPublicKey> {
        self.validate_for(alg, op)?;
        let key = self.rsa()?;
        RsaPublicKey::from_n_e(alg, key.n, key.e)
    }
}

struct KeyView<'a> {
    kty: KeyType,
    alg: Option<Algorithm>,
    key_ops: HeaplessVec<KeyOp, MAX_MAP_ENTRIES>,
    material: Material<'a>,
}

impl KeyView<'_> {
    fn curve(&self) -> Option<Curve> {
        match &self.material {
            Material::Ec2 {
                crv, ..
            }
            | Material::Okp {
                crv, ..
            } => Some(*crv),
            _ => None,
        }
    }
}

fn validate_view(view: &KeyView<'_>) -> Result<()> {
    if let Some(alg) = view.alg
        && !alg.is_compatible_key(view.kty, view.curve())
    {
        return Err(Error::UnsupportedAlgorithm(alg));
    }

    match &view.material {
        Material::Okp {
            crv,
            x,
            d,
        } => {
            if *crv != Curve::Ed25519 {
                return Err(Error::UnsupportedCurve(*crv));
            }
            match (x, d) {
                (None, None) => return Err(Error::InvalidKey),
                (Some(x), _) if x.len() != 32 => return Err(Error::InvalidKey),
                (_, Some(d)) if d.len() != 32 => return Err(Error::InvalidKey),
                _ => {}
            }
            if let Some(x) = x {
                let x: [u8; 32] = (*x).try_into().map_err(|_| Error::InvalidKey)?;
                crypto::curve25519::ed25519::PublicKey::from_bytes(&x).map_err(|_| Error::InvalidKey)?;
            }
        }
        Material::Ec2 {
            crv,
            x,
            y,
            d,
        } => {
            let size = match crv {
                Curve::P256 => 32,
                Curve::P384 => 48,
                Curve::P521 => 66,
                other => return Err(Error::UnsupportedCurve(*other)),
            };
            if let Some(x) = x
                && x.len() != size
            {
                return Err(Error::InvalidKey);
            }
            if let Some(YCoordinate::Bytes(y)) = y
                && y.len() != size
            {
                return Err(Error::InvalidKey);
            }
            if let Some(d) = d
                && d.len() != size
            {
                return Err(Error::InvalidKey);
            }
            match (x, y, d) {
                (Some(_), Some(_), _) => {}
                (_, _, Some(_)) => {}
                _ => return Err(Error::InvalidKey),
            }
            validate_ec2_material(*crv, *x, *y, *d)?;
        }
        Material::Symmetric {
            k,
        } => {
            if k.is_empty() {
                return Err(Error::InvalidKey);
            }
            if let Some(alg) = view.alg
                && alg.is_mac()
                && k.len() < 16
            {
                return Err(Error::InvalidKey);
            }
        }
        Material::Rsa {
            n,
            e,
        } => {
            if n.is_empty() || e.is_empty() {
                return Err(Error::InvalidKey);
            }
            if let Some(alg) = view.alg
                && !alg.is_signature()
            {
                return Err(Error::UnsupportedAlgorithm(alg));
            }
            crypto::rsa::PublicKey::from_n_e(n, e).map_err(|_| Error::InvalidKey)?;
        }
        Material::Akp {
            public,
            private,
        } => {
            let alg = view.alg.ok_or(Error::InvalidKey)?;
            let public_size = match alg {
                Algorithm::MlDsa44 => crypto::mldsa::ML_DSA_44_PUBLIC_KEY_SIZE,
                Algorithm::MlDsa65 => crypto::mldsa::ML_DSA_65_PUBLIC_KEY_SIZE,
                Algorithm::MlDsa87 => crypto::mldsa::ML_DSA_87_PUBLIC_KEY_SIZE,
                _ => return Err(Error::UnsupportedAlgorithm(alg)),
            };
            match (public, private) {
                (None, None) => return Err(Error::InvalidKey),
                (Some(public), _) if public.len() != public_size => {
                    return Err(Error::InvalidKey);
                }
                (_, Some(private)) if private.len() != 32 => return Err(Error::InvalidKey),
                _ => {}
            }
        }
        Material::None => return Err(Error::InvalidKey),
    }

    Ok(())
}

fn validate_ec2_material(crv: Curve, x: Option<&[u8]>, y: Option<YCoordinate<'_>>, d: Option<&[u8]>) -> Result<()> {
    macro_rules! check {
        ($public:ty, $secret:ty, $size:literal) => {{
            if let Some(x) = x {
                let x: [u8; $size] = x.try_into().map_err(|_| Error::InvalidKey)?;
                match y {
                    Some(YCoordinate::Bytes(y)) => {
                        let y: [u8; $size] = y.try_into().map_err(|_| Error::InvalidKey)?;
                        <$public>::from_x_y(&x, &y).map_err(|_| Error::InvalidKey)?;
                    }
                    Some(YCoordinate::Sign(sign)) => {
                        let mut sec1 = [0u8; $size + 1];
                        sec1[0] = if sign { 0x03 } else { 0x02 };
                        sec1[1..].copy_from_slice(&x);
                        <$public>::from_bytes(&sec1).map_err(|_| Error::InvalidKey)?;
                    }
                    None => {}
                }
            }
            if let Some(d) = d {
                let d: [u8; $size] = d.try_into().map_err(|_| Error::InvalidKey)?;
                <$secret>::from_bytes(&d).map_err(|_| Error::InvalidKey)?;
            }
        }};
    }

    match crv {
        Curve::P256 => check!(crypto::p256::PublicKey, crypto::p256::SecretKey, 32),
        Curve::P384 => check!(crypto::p384::PublicKey, crypto::p384::SecretKey, 48),
        Curve::P521 => check!(crypto::p521::PublicKey, crypto::p521::SecretKey, 66),
        other => return Err(Error::UnsupportedCurve(other)),
    }
    Ok(())
}

/// Iterator over a `key_ops` array.
pub struct KeyOps<'a> {
    d: Decoder<'a>,
    remaining: usize,
}

impl<'a> Iterator for KeyOps<'a> {
    type Item = Result<KeyOp>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let result = self.d.i64().map(KeyOp::from_i64);
        if result.is_err() {
            self.remaining = 0;
        }
        Some(result.map_err(Error::Decode))
    }
}

/// Iterator over a COSE_Key map.
pub struct KeyIter<'a> {
    input: &'a [u8],
    d: Decoder<'a>,
    remaining: usize,
}

impl<'a> Iterator for KeyIter<'a> {
    type Item = Result<(Label<'a>, HeaderValue<'a>)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;

        let result = (|| -> Result<(Label<'a>, HeaderValue<'a>)> {
            let label = crate::codec::decode_label(&mut self.d)?;
            let value = match self.d.datatype()? {
                Type::U8 | Type::U16 | Type::U32 | Type::U64 => HeaderValue::Uint(self.d.u64()?),
                Type::I8 | Type::I16 | Type::I32 | Type::I64 => HeaderValue::Int(self.d.i64()?),
                Type::Bytes => HeaderValue::Bytes(self.d.bytes()?),
                Type::Text => HeaderValue::Text(self.d.text()?),
                Type::Bool => HeaderValue::Bool(self.d.bool()?),
                Type::Null => {
                    self.d.null()?;
                    HeaderValue::Null
                }
                _ => HeaderValue::Raw(take_raw(self.input, &mut self.d)?),
            };
            Ok((label, value))
        })();

        match result {
            Ok(item) => Some(Ok(item)),
            Err(err) => {
                self.remaining = 0;
                Some(Err(err))
            }
        }
    }
}

enum Material<'a> {
    None,
    Ec2 {
        crv: Curve,
        x: Option<&'a [u8]>,
        y: Option<YCoordinate<'a>>,
        d: Option<&'a [u8]>,
    },
    Okp {
        crv: Curve,
        x: Option<&'a [u8]>,
        d: Option<&'a [u8]>,
    },
    Symmetric {
        k: &'a [u8],
    },
    Rsa {
        n: &'a [u8],
        e: &'a [u8],
    },
    Akp {
        public: Option<&'a [u8]>,
        private: Option<&'a [u8]>,
    },
}

/// Builder for a COSE_Key.
///
/// The builder stores borrowed key material and never allocates. It supports
/// up to four `key_ops` entries and four additional parameters. Exceeding
/// those limits makes `build`/`build_into` return [`Error::TooManyEntries`].
pub struct KeyBuilder<'a> {
    kty: KeyType,
    alg: Option<Algorithm>,
    kid: Option<&'a [u8]>,
    key_ops: HeaplessVec<KeyOp, 4>,
    material: Material<'a>,
    rest: HeaplessVec<(Label<'a>, HeaderValue<'a>), 4>,
    overflow: bool,
}

impl<'a> KeyBuilder<'a> {
    /// Creates a builder for `kty`.
    #[inline]
    pub const fn new(kty: KeyType) -> Self {
        Self {
            kty,
            alg: None,
            kid: None,
            key_ops: HeaplessVec::new(),
            material: Material::None,
            rest: HeaplessVec::new(),
            overflow: false,
        }
    }

    /// Sets the `alg` parameter.
    #[inline]
    pub fn algorithm(mut self, alg: Algorithm) -> Self {
        self.alg = Some(alg);
        self
    }

    /// Sets the `kid` parameter.
    #[inline]
    pub fn key_id(mut self, kid: &'a [u8]) -> Self {
        self.kid = Some(kid);
        self
    }

    /// Adds a `key_ops` entry.
    pub fn key_op(mut self, op: KeyOp) -> Self {
        if self.key_ops.push(op).is_err() {
            self.overflow = true;
        }
        self
    }

    /// Sets EC2 public key material.
    #[inline]
    pub fn ec2(mut self, crv: Curve, x: &'a [u8], y: YCoordinate<'a>) -> Self {
        self.material = Material::Ec2 {
            crv,
            x: Some(x),
            y: Some(y),
            d: None,
        };
        self
    }

    /// Sets EC2 private key material (`d` only; the public key is derived when
    /// the key is used).
    #[inline]
    pub fn ec2_private(mut self, crv: Curve, d: &'a [u8]) -> Self {
        self.material = Material::Ec2 {
            crv,
            x: None,
            y: None,
            d: Some(d),
        };
        self
    }

    /// Sets OKP public key material.
    #[inline]
    pub fn okp(mut self, crv: Curve, x: &'a [u8]) -> Self {
        self.material = Material::Okp {
            crv,
            x: Some(x),
            d: None,
        };
        self
    }

    /// Sets OKP private key material (`d` only; the public key is derived when
    /// the key is used).
    #[inline]
    pub fn okp_private(mut self, crv: Curve, d: &'a [u8]) -> Self {
        self.material = Material::Okp {
            crv,
            x: None,
            d: Some(d),
        };
        self
    }

    /// Sets symmetric key material.
    #[inline]
    pub fn symmetric(mut self, k: &'a [u8]) -> Self {
        self.material = Material::Symmetric {
            k,
        };
        self
    }

    /// Sets RSA public key material.
    #[inline]
    pub fn rsa(mut self, n: &'a [u8], e: &'a [u8]) -> Self {
        self.material = Material::Rsa {
            n,
            e,
        };
        self
    }

    /// Sets AKP public key material and the required `alg`.
    #[inline]
    pub fn akp(mut self, alg: Algorithm, public: &'a [u8]) -> Self {
        self.alg = Some(alg);
        self.material = Material::Akp {
            public: Some(public),
            private: None,
        };
        self
    }

    /// Sets AKP private key material (the 32-byte ML-DSA seed) and `alg`.
    #[inline]
    pub fn akp_private(mut self, alg: Algorithm, seed: &'a [u8]) -> Self {
        self.alg = Some(alg);
        self.material = Material::Akp {
            public: None,
            private: Some(seed),
        };
        self
    }

    /// Adds an arbitrary key parameter.
    pub fn value(mut self, label: Label<'a>, value: HeaderValue<'a>) -> Self {
        if self.rest.push((label, value)).is_err() {
            self.overflow = true;
        }
        self
    }

    /// Adds an arbitrary key parameter whose value is already CBOR-encoded.
    #[inline]
    pub fn raw_value(self, label: Label<'a>, encoded: &'a [u8]) -> Self {
        self.value(label, HeaderValue::Raw(encoded))
    }

    /// Encodes the key into `out` and returns a view borrowing `out`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooManyEntries`] if the builder overflowed,
    /// [`Error::BufferTooSmall`] if `out` is too small, and any validation
    /// error from [`CoseKey::validate`].
    pub fn build_into<'b>(&self, out: &'b mut [u8]) -> Result<CoseKey<'b>> {
        if self.overflow {
            return Err(Error::TooManyEntries);
        }

        let required = self.encoded_len();
        if out.len() < required {
            return Err(Error::BufferTooSmall {
                required,
                available: out.len(),
            });
        }

        let mut cursor = Cursor::new(&mut out[..required]);
        {
            let mut e = Encoder::new(&mut cursor);
            e.map(self.entry_count() as usize)?;
            e.i64(1)?;
            e.i64(self.kty.as_i64())?;

            if let Some(alg) = self.alg {
                e.i64(3)?;
                e.i64(alg.as_i64())?;
            }
            if let Some(kid) = self.kid {
                e.i64(2)?;
                e.bytes(kid)?;
            }
            if !self.key_ops.is_empty() {
                e.i64(4)?;
                e.array(self.key_ops.len())?;
                for op in &self.key_ops {
                    e.i64(op.as_i64())?;
                }
            }

            match &self.material {
                Material::None => {}
                Material::Ec2 {
                    crv,
                    x,
                    y,
                    d,
                } => {
                    e.i64(-1)?;
                    e.i64(crv.as_i64())?;
                    if let Some(x) = x {
                        e.i64(-2)?;
                        e.bytes(x)?;
                    }
                    if let Some(y) = y {
                        e.i64(-3)?;
                        match y {
                            YCoordinate::Bytes(bytes) => {
                                e.bytes(bytes)?;
                            }
                            YCoordinate::Sign(sign) => {
                                e.bool(*sign)?;
                            }
                        }
                    }
                    if let Some(d) = d {
                        e.i64(-4)?;
                        e.bytes(d)?;
                    }
                }
                Material::Okp {
                    crv,
                    x,
                    d,
                } => {
                    e.i64(-1)?;
                    e.i64(crv.as_i64())?;
                    if let Some(x) = x {
                        e.i64(-2)?;
                        e.bytes(x)?;
                    }
                    if let Some(d) = d {
                        e.i64(-4)?;
                        e.bytes(d)?;
                    }
                }
                Material::Symmetric {
                    k,
                } => {
                    e.i64(-1)?;
                    e.bytes(k)?;
                }
                Material::Rsa {
                    n,
                    e: exponent,
                } => {
                    e.i64(-1)?;
                    e.bytes(n)?;
                    e.i64(-2)?;
                    e.bytes(exponent)?;
                }
                Material::Akp {
                    public,
                    private,
                    ..
                } => {
                    if let Some(public) = public {
                        e.i64(-1)?;
                        e.bytes(public)?;
                    }
                    if let Some(private) = private {
                        e.i64(-2)?;
                        e.bytes(private)?;
                    }
                }
            }

            for (label, value) in &self.rest {
                write_label(&mut e, *label)?;
                write_value(&mut e, *value)?;
            }
        }

        if cursor.position() != required {
            return Err(Error::Encode);
        }

        let key = CoseKey::from_slice(&out[..required])?;
        key.validate()?;
        Ok(key)
    }

    /// Encodes the key into a new owned value.
    #[cfg(feature = "alloc")]
    pub fn build(&self) -> Result<CoseKey<'static>> {
        let required = self.encoded_len();
        let mut bytes = alloc::vec![0u8; required];
        self.build_into(&mut bytes)?;
        CoseKey::from_owned(bytes)
    }

    fn entry_count(&self) -> u64 {
        let mut count = 1; // kty
        count += self.alg.is_some() as u64;
        count += self.kid.is_some() as u64;
        count += (!self.key_ops.is_empty()) as u64;
        count += match &self.material {
            Material::None => 0,
            Material::Ec2 {
                x,
                y,
                d,
                ..
            } => 1 + x.is_some() as u64 + y.is_some() as u64 + d.is_some() as u64,
            Material::Okp {
                x,
                d,
                ..
            } => 1 + x.is_some() as u64 + d.is_some() as u64,
            Material::Symmetric {
                ..
            } => 1,
            Material::Rsa {
                ..
            } => 2,
            Material::Akp {
                public,
                private,
                ..
            } => public.is_some() as u64 + private.is_some() as u64,
        };
        count + self.rest.len() as u64
    }

    fn encoded_len(&self) -> usize {
        let mut len = cbor_len_uint(self.entry_count());
        len += 1 + cbor_len_int(self.kty.as_i64());

        if let Some(alg) = self.alg {
            len += 1 + cbor_len_int(alg.as_i64());
        }
        if let Some(kid) = self.kid {
            len += 1 + cbor_len_bytes(kid.len());
        }
        if !self.key_ops.is_empty() {
            len += 1 + cbor_len_array(self.key_ops.len() as u64);
            for op in &self.key_ops {
                len += cbor_len_int(op.as_i64());
            }
        }

        match &self.material {
            Material::None => {}
            Material::Ec2 {
                crv,
                x,
                y,
                d,
            } => {
                len += 1 + cbor_len_int(crv.as_i64());
                if let Some(x) = x {
                    len += 1 + cbor_len_bytes(x.len());
                }
                if let Some(y) = y {
                    len += 1 + match y {
                        YCoordinate::Bytes(bytes) => cbor_len_bytes(bytes.len()),
                        YCoordinate::Sign(_) => 1,
                    };
                }
                if let Some(d) = d {
                    len += 1 + cbor_len_bytes(d.len());
                }
            }
            Material::Okp {
                crv,
                x,
                d,
            } => {
                len += 1 + cbor_len_int(crv.as_i64());
                if let Some(x) = x {
                    len += 1 + cbor_len_bytes(x.len());
                }
                if let Some(d) = d {
                    len += 1 + cbor_len_bytes(d.len());
                }
            }
            Material::Symmetric {
                k,
            } => {
                len += 1 + cbor_len_bytes(k.len());
            }
            Material::Rsa {
                n,
                e,
            } => {
                len += 1 + cbor_len_bytes(n.len());
                len += 1 + cbor_len_bytes(e.len());
            }
            Material::Akp {
                public,
                private,
                ..
            } => {
                if let Some(public) = public {
                    len += 1 + cbor_len_bytes(public.len());
                }
                if let Some(private) = private {
                    len += 1 + cbor_len_bytes(private.len());
                }
            }
        }

        for (label, value) in &self.rest {
            len += label_len(*label) + value_len(*value);
        }
        len
    }
}

/// A COSE_KeySet (a non-empty CBOR array of COSE_Key maps).
///
/// Parsing accepts between 1 and 1024 keys. Individual keys are validated
/// lazily: [`CoseKeySet::iter`] yields each key, but call
/// [`CoseKey::validate`] before trusting it.
#[derive(Clone)]
pub struct CoseKeySet<'a> {
    raw: Bytes<'a>,
}

impl fmt::Debug for CoseKeySet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CoseKeySet").finish()
    }
}

impl<'a> CoseKeySet<'a> {
    /// Parses a COSE_KeySet.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidStructure`] if the array is empty or has more
    /// than 1024 elements, [`Error::IndefiniteLength`] for an indefinite
    /// array, [`Error::Decode`] for malformed CBOR, and
    /// [`Error::TrailingBytes`] if extra bytes follow the array.
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self> {
        validate_array(bytes, 1, 1024)?;
        Ok(Self {
            raw: Bytes::borrowed(bytes),
        })
    }

    /// Parses a COSE_KeySet and takes ownership of its bytes.
    #[cfg(feature = "alloc")]
    pub fn from_owned(bytes: alloc::vec::Vec<u8>) -> Result<Self> {
        validate_array(&bytes, 1, 1024)?;
        Ok(Self {
            raw: Bytes::owned(bytes),
        })
    }

    /// Returns the encoded COSE_KeySet array.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.raw.as_slice()
    }

    /// Returns the number of keys in the set.
    ///
    /// The value comes from the validated array header.
    pub fn len(&self) -> usize {
        let mut d = decoder(self.raw.as_slice());
        d.array().ok().flatten().unwrap_or(0)
    }

    /// Returns `true` if the set contains no keys.
    ///
    /// A parsed `CoseKeySet` is always non-empty, so this is normally `false`.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterates over the keys.
    pub fn iter(&self) -> CoseKeySetIter<'_> {
        let input = self.raw.as_slice();
        let mut d = decoder(input);
        let remaining = match d.array() {
            Ok(Some(len)) => len,
            _ => 0,
        };
        CoseKeySetIter {
            input,
            d,
            remaining,
        }
    }

    /// Returns the first key whose `kid` matches.
    pub fn find_by_kid(&self, kid: &[u8]) -> Option<CoseKey<'_>> {
        self.iter().flatten().find(|key| key.kid() == Some(kid))
    }

    /// Returns the first key matching the optional `kid` and algorithm filters.
    pub fn find(&self, kid: Option<&[u8]>, alg: Option<Algorithm>) -> Option<CoseKey<'_>> {
        for key in self.iter() {
            let Ok(key) = key else {
                continue;
            };
            if let Some(kid) = kid
                && key.kid() != Some(kid)
            {
                continue;
            }
            if let Some(alg) = alg {
                match key.alg() {
                    Some(key_alg) if !key_alg.is_compatible(alg) => continue,
                    _ => {}
                }
            }
            return Some(key);
        }
        None
    }

    /// Copies the encoded key set into a new `Vec`.
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> alloc::vec::Vec<u8> {
        self.raw.as_slice().to_vec()
    }
}

impl CoseKeySet<'static> {
    /// Encodes an array of keys into `out`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::BufferTooSmall`] if `out` is too small, and
    /// [`Error::InvalidStructure`] if the resulting set is empty or contains
    /// more than 1024 keys. The keys themselves are copied verbatim; call
    /// [`CoseKey::validate`] first if they are untrusted.
    pub fn encode_into<'b>(out: &'b mut [u8], keys: &[CoseKey<'_>]) -> Result<CoseKeySet<'b>> {
        let mut required = cbor_len_array(keys.len() as u64);
        for key in keys {
            required += key.as_bytes().len();
        }
        if out.len() < required {
            return Err(Error::BufferTooSmall {
                required,
                available: out.len(),
            });
        }

        let mut cursor = Cursor::new(&mut out[..required]);
        {
            let mut e = Encoder::new(&mut cursor);
            e.array(keys.len())?;
            for key in keys {
                e.writer_mut().write_all(key.as_bytes()).map_err(|_| Error::Encode)?;
            }
        }
        if cursor.position() != required {
            return Err(Error::Encode);
        }
        CoseKeySet::from_slice(&out[..required])
    }
}

/// Iterator over a COSE_KeySet.
pub struct CoseKeySetIter<'a> {
    input: &'a [u8],
    d: Decoder<'a>,
    remaining: usize,
}

impl<'a> Iterator for CoseKeySetIter<'a> {
    type Item = Result<CoseKey<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let raw = match take_raw(self.input, &mut self.d) {
            Ok(raw) => raw,
            Err(err) => {
                self.remaining = 0;
                return Some(Err(err));
            }
        };
        Some(CoseKey::from_slice(raw))
    }
}

/// Owned builder for a COSE_KeySet.
#[cfg(feature = "alloc")]
pub struct KeySetBuilder {
    keys: alloc::vec::Vec<alloc::vec::Vec<u8>>,
}

#[cfg(feature = "alloc")]
impl KeySetBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self {
            keys: alloc::vec::Vec::new(),
        }
    }

    /// Adds a key. The key is copied into the builder, so it may borrow.
    pub fn push(mut self, key: CoseKey<'_>) -> Self {
        self.keys.push(key.as_bytes().to_vec());
        self
    }

    /// Builds an owned key set.
    ///
    /// Keys are copied verbatim; validate untrusted keys before adding them.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidStructure`] for an empty set or more than 1024
    /// keys, and [`Error::Encode`] if the computed buffer is inconsistent.
    pub fn build(self) -> Result<CoseKeySet<'static>> {
        let mut required = cbor_len_array(self.keys.len() as u64);
        for key in &self.keys {
            required += key.len();
        }
        let mut bytes = alloc::vec![0u8; required];
        {
            let mut cursor = Cursor::new(&mut bytes[..required]);
            let mut e = Encoder::new(&mut cursor);
            e.array(self.keys.len())?;
            for key in &self.keys {
                e.writer_mut().write_all(key).map_err(|_| Error::Encode)?;
            }
            if cursor.position() != required {
                return Err(Error::Encode);
            }
        }
        CoseKeySet::from_owned(bytes)
    }
}

#[cfg(feature = "alloc")]
impl Default for KeySetBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(feature = "alloc", feature = "zeroize"))]
impl Drop for KeySetBuilder {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        for key in &mut self.keys {
            key.zeroize();
        }
    }
}

// ---------------------------------------------------------------------------
// Conversions from crypto key types into owned COSE_Key values.
// ---------------------------------------------------------------------------

#[cfg(feature = "alloc")]
impl From<&crypto::curve25519::ed25519::PublicKey> for CoseKey<'static> {
    fn from(key: &crypto::curve25519::ed25519::PublicKey) -> Self {
        let bytes = key.to_bytes();
        KeyBuilder::new(KeyType::Okp)
            .algorithm(Algorithm::Ed25519)
            .okp(Curve::Ed25519, &bytes)
            .build()
            .expect("valid Ed25519 public key")
    }
}

#[cfg(feature = "alloc")]
impl From<&crypto::curve25519::ed25519::SecretKey> for CoseKey<'static> {
    fn from(key: &crypto::curve25519::ed25519::SecretKey) -> Self {
        let public = key.public_key().to_bytes();
        let private = key.to_bytes();
        KeyBuilder::new(KeyType::Okp)
            .algorithm(Algorithm::Ed25519)
            .okp(Curve::Ed25519, &public)
            .value(Label::Int(-4), HeaderValue::Bytes(&private))
            .build()
            .expect("valid Ed25519 secret key")
    }
}

#[cfg(feature = "alloc")]
impl From<&crypto::p256::PublicKey> for CoseKey<'static> {
    fn from(key: &crypto::p256::PublicKey) -> Self {
        let (x, y) = key.x_y();
        KeyBuilder::new(KeyType::Ec2)
            .algorithm(Algorithm::ESP256)
            .ec2(Curve::P256, &x, YCoordinate::Bytes(&y))
            .build()
            .expect("valid P-256 public key")
    }
}

#[cfg(feature = "alloc")]
impl From<&crypto::p256::SecretKey> for CoseKey<'static> {
    fn from(key: &crypto::p256::SecretKey) -> Self {
        let (x, y) = key.public_key().x_y();
        let d = key.to_bytes();
        KeyBuilder::new(KeyType::Ec2)
            .algorithm(Algorithm::ESP256)
            .ec2(Curve::P256, &x, YCoordinate::Bytes(&y))
            .value(Label::Int(-4), HeaderValue::Bytes(&d))
            .build()
            .expect("valid P-256 secret key")
    }
}

#[cfg(feature = "alloc")]
impl From<&crypto::p384::PublicKey> for CoseKey<'static> {
    fn from(key: &crypto::p384::PublicKey) -> Self {
        let (x, y) = key.x_y();
        KeyBuilder::new(KeyType::Ec2)
            .algorithm(Algorithm::ESP384)
            .ec2(Curve::P384, &x, YCoordinate::Bytes(&y))
            .build()
            .expect("valid P-384 public key")
    }
}

#[cfg(feature = "alloc")]
impl From<&crypto::p384::SecretKey> for CoseKey<'static> {
    fn from(key: &crypto::p384::SecretKey) -> Self {
        let (x, y) = key.public_key().x_y();
        let d = key.to_bytes();
        KeyBuilder::new(KeyType::Ec2)
            .algorithm(Algorithm::ESP384)
            .ec2(Curve::P384, &x, YCoordinate::Bytes(&y))
            .value(Label::Int(-4), HeaderValue::Bytes(&d))
            .build()
            .expect("valid P-384 secret key")
    }
}

#[cfg(feature = "alloc")]
impl From<&crypto::p521::PublicKey> for CoseKey<'static> {
    fn from(key: &crypto::p521::PublicKey) -> Self {
        let (x, y) = key.x_y();
        KeyBuilder::new(KeyType::Ec2)
            .algorithm(Algorithm::ESP512)
            .ec2(Curve::P521, &x, YCoordinate::Bytes(&y))
            .build()
            .expect("valid P-521 public key")
    }
}

#[cfg(feature = "alloc")]
impl From<&crypto::p521::SecretKey> for CoseKey<'static> {
    fn from(key: &crypto::p521::SecretKey) -> Self {
        let (x, y) = key.public_key().x_y();
        let d = key.to_bytes();
        KeyBuilder::new(KeyType::Ec2)
            .algorithm(Algorithm::ESP512)
            .ec2(Curve::P521, &x, YCoordinate::Bytes(&y))
            .value(Label::Int(-4), HeaderValue::Bytes(&d))
            .build()
            .expect("valid P-521 secret key")
    }
}

#[cfg(feature = "alloc")]
impl From<&crypto::mldsa::MlDsa44PublicKey> for CoseKey<'static> {
    fn from(key: &crypto::mldsa::MlDsa44PublicKey) -> Self {
        let public = key.to_bytes();
        KeyBuilder::new(KeyType::Akp)
            .akp(Algorithm::MlDsa44, &public)
            .build()
            .expect("valid ML-DSA-44 public key")
    }
}

#[cfg(feature = "alloc")]
impl From<&crypto::mldsa::MlDsa44SecretKey> for CoseKey<'static> {
    fn from(key: &crypto::mldsa::MlDsa44SecretKey) -> Self {
        let public = key.public_key().to_bytes();
        let seed = key.seed();
        KeyBuilder::new(KeyType::Akp)
            .akp(Algorithm::MlDsa44, &public)
            .value(Label::Int(-2), HeaderValue::Bytes(seed))
            .build()
            .expect("valid ML-DSA-44 secret key")
    }
}

#[cfg(feature = "alloc")]
impl From<&crypto::mldsa::MlDsa65PublicKey> for CoseKey<'static> {
    fn from(key: &crypto::mldsa::MlDsa65PublicKey) -> Self {
        let public = key.to_bytes();
        KeyBuilder::new(KeyType::Akp)
            .akp(Algorithm::MlDsa65, &public)
            .build()
            .expect("valid ML-DSA-65 public key")
    }
}

#[cfg(feature = "alloc")]
impl From<&crypto::mldsa::MlDsa65SecretKey> for CoseKey<'static> {
    fn from(key: &crypto::mldsa::MlDsa65SecretKey) -> Self {
        let public = key.public_key().to_bytes();
        let seed = key.seed();
        KeyBuilder::new(KeyType::Akp)
            .akp(Algorithm::MlDsa65, &public)
            .value(Label::Int(-2), HeaderValue::Bytes(seed))
            .build()
            .expect("valid ML-DSA-65 secret key")
    }
}

#[cfg(feature = "alloc")]
impl From<&crypto::mldsa::MlDsa87PublicKey> for CoseKey<'static> {
    fn from(key: &crypto::mldsa::MlDsa87PublicKey) -> Self {
        let public = key.to_bytes();
        KeyBuilder::new(KeyType::Akp)
            .akp(Algorithm::MlDsa87, &public)
            .build()
            .expect("valid ML-DSA-87 public key")
    }
}

#[cfg(feature = "alloc")]
impl From<&crypto::mldsa::MlDsa87SecretKey> for CoseKey<'static> {
    fn from(key: &crypto::mldsa::MlDsa87SecretKey) -> Self {
        let public = key.public_key().to_bytes();
        let seed = key.seed();
        KeyBuilder::new(KeyType::Akp)
            .akp(Algorithm::MlDsa87, &public)
            .value(Label::Int(-2), HeaderValue::Bytes(seed))
            .build()
            .expect("valid ML-DSA-87 secret key")
    }
}

#[cfg(feature = "alloc")]
impl From<&RsaPublicKey> for CoseKey<'static> {
    fn from(key: &RsaPublicKey) -> Self {
        let n = key.n_bytes();
        let e = key.e_bytes();
        KeyBuilder::new(KeyType::Rsa)
            .algorithm(key.algorithm())
            .rsa(&n, &e)
            .build()
            .expect("valid RSA public key")
    }
}

#[cfg(feature = "alloc")]
impl From<&SymmetricKey<'_>> for CoseKey<'static> {
    fn from(key: &SymmetricKey<'_>) -> Self {
        KeyBuilder::new(KeyType::Symmetric)
            .algorithm(key.algorithm())
            .symmetric(key.as_bytes())
            .build()
            .expect("valid symmetric key")
    }
}

// ---------------------------------------------------------------------------
// Conversions from COSE_Key into crypto key types.
// ---------------------------------------------------------------------------

fn fixed_32(bytes: &[u8]) -> Result<[u8; 32]> {
    bytes.try_into().map_err(|_| Error::InvalidKey)
}

impl TryFrom<&CoseKey<'_>> for crypto::curve25519::ed25519::PublicKey {
    type Error = Error;

    fn try_from(key: &CoseKey<'_>) -> Result<Self> {
        key.validate_for(Algorithm::Ed25519, KeyOp::Verify)?;
        let okp = key.okp()?;
        if let Some(x) = okp.x {
            return crypto::curve25519::ed25519::PublicKey::from_bytes(&fixed_32(x)?).map_err(|_| Error::InvalidKey);
        }
        if let Some(d) = okp.d {
            return Ok(crypto::curve25519::ed25519::SecretKey::from_bytes(&fixed_32(d)?).public_key());
        }
        Err(Error::InvalidKey)
    }
}

impl TryFrom<&CoseKey<'_>> for crypto::curve25519::ed25519::SecretKey {
    type Error = Error;

    fn try_from(key: &CoseKey<'_>) -> Result<Self> {
        key.validate_for(Algorithm::Ed25519, KeyOp::Sign)?;
        let okp = key.okp()?;
        let d = okp.d.ok_or(Error::InvalidKey)?;
        Ok(crypto::curve25519::ed25519::SecretKey::from_bytes(&fixed_32(d)?))
    }
}

macro_rules! ec2_public_conversion {
    ($public:ty, $secret:ty, $variant:ident, $size:literal, $alg:expr) => {
        impl TryFrom<&CoseKey<'_>> for $public {
            type Error = Error;

            fn try_from(key: &CoseKey<'_>) -> Result<Self> {
                key.validate_for($alg, KeyOp::Verify)?;
                let ec2 = key.ec2()?;
                match (ec2.x, ec2.y, ec2.d) {
                    (Some(x), Some(YCoordinate::Bytes(y)), _) => {
                        let x: [u8; $size] = x.try_into().map_err(|_| Error::InvalidKey)?;
                        let y: [u8; $size] = y.try_into().map_err(|_| Error::InvalidKey)?;
                        <$public>::from_x_y(&x, &y).map_err(|_| Error::InvalidKey)
                    }
                    (Some(x), Some(YCoordinate::Sign(sign)), _) => {
                        let mut sec1 = [0u8; $size + 1];
                        sec1[0] = if sign { 0x03 } else { 0x02 };
                        sec1[1..].copy_from_slice(x);
                        <$public>::from_bytes(&sec1).map_err(|_| Error::InvalidKey)
                    }
                    (_, _, Some(d)) => {
                        let d: [u8; $size] = d.try_into().map_err(|_| Error::InvalidKey)?;
                        let secret = <$secret>::from_bytes(&d).map_err(|_| Error::InvalidKey)?;
                        Ok(secret.public_key())
                    }
                    _ => Err(Error::InvalidKey),
                }
            }
        }

        impl TryFrom<&CoseKey<'_>> for $secret {
            type Error = Error;

            fn try_from(key: &CoseKey<'_>) -> Result<Self> {
                key.validate_for($alg, KeyOp::Sign)?;
                let ec2 = key.ec2()?;
                let d = ec2.d.ok_or(Error::InvalidKey)?;
                let d: [u8; $size] = d.try_into().map_err(|_| Error::InvalidKey)?;
                <$secret>::from_bytes(&d).map_err(|_| Error::InvalidKey)
            }
        }
    };
}

ec2_public_conversion!(crypto::p256::PublicKey, crypto::p256::SecretKey, P256, 32, Algorithm::ESP256);
ec2_public_conversion!(crypto::p384::PublicKey, crypto::p384::SecretKey, P384, 48, Algorithm::ESP384);
ec2_public_conversion!(crypto::p521::PublicKey, crypto::p521::SecretKey, P521, 66, Algorithm::ESP512);

fn akp_private<'a>(key: &'a CoseKey<'a>, alg: Algorithm) -> Result<&'a [u8]> {
    let akp = key.akp()?;
    if akp.alg != alg {
        return Err(Error::InvalidKey);
    }
    akp.private.ok_or(Error::InvalidKey)
}

impl TryFrom<&CoseKey<'_>> for crypto::mldsa::MlDsa44PublicKey {
    type Error = Error;
    fn try_from(key: &CoseKey<'_>) -> Result<Self> {
        key.validate_for(Algorithm::MlDsa44, KeyOp::Verify)?;
        let akp = key.akp()?;
        if let Some(public) = akp.public {
            let public: &[u8; crypto::mldsa::ML_DSA_44_PUBLIC_KEY_SIZE] =
                public.try_into().map_err(|_| Error::InvalidKey)?;
            return Ok(Self::from_bytes(public));
        }
        if let Some(private) = akp.private {
            let seed: &[u8; 32] = private.try_into().map_err(|_| Error::InvalidKey)?;
            return Ok(crypto::mldsa::MlDsa44SecretKey::new(seed).public_key());
        }
        Err(Error::InvalidKey)
    }
}

impl TryFrom<&CoseKey<'_>> for crypto::mldsa::MlDsa44SecretKey {
    type Error = Error;
    fn try_from(key: &CoseKey<'_>) -> Result<Self> {
        key.validate_for(Algorithm::MlDsa44, KeyOp::Sign)?;
        let private = akp_private(key, Algorithm::MlDsa44)?;
        let seed: &[u8; 32] = private.try_into().map_err(|_| Error::InvalidKey)?;
        Ok(Self::new(seed))
    }
}

impl TryFrom<&CoseKey<'_>> for crypto::mldsa::MlDsa65PublicKey {
    type Error = Error;
    fn try_from(key: &CoseKey<'_>) -> Result<Self> {
        key.validate_for(Algorithm::MlDsa65, KeyOp::Verify)?;
        let akp = key.akp()?;
        if let Some(public) = akp.public {
            let public: &[u8; crypto::mldsa::ML_DSA_65_PUBLIC_KEY_SIZE] =
                public.try_into().map_err(|_| Error::InvalidKey)?;
            return Ok(Self::from_bytes(public));
        }
        if let Some(private) = akp.private {
            let seed: &[u8; 32] = private.try_into().map_err(|_| Error::InvalidKey)?;
            return Ok(crypto::mldsa::MlDsa65SecretKey::new(seed).public_key());
        }
        Err(Error::InvalidKey)
    }
}

impl TryFrom<&CoseKey<'_>> for crypto::mldsa::MlDsa65SecretKey {
    type Error = Error;
    fn try_from(key: &CoseKey<'_>) -> Result<Self> {
        key.validate_for(Algorithm::MlDsa65, KeyOp::Sign)?;
        let private = akp_private(key, Algorithm::MlDsa65)?;
        let seed: &[u8; 32] = private.try_into().map_err(|_| Error::InvalidKey)?;
        Ok(Self::new(seed))
    }
}

impl TryFrom<&CoseKey<'_>> for crypto::mldsa::MlDsa87PublicKey {
    type Error = Error;
    fn try_from(key: &CoseKey<'_>) -> Result<Self> {
        key.validate_for(Algorithm::MlDsa87, KeyOp::Verify)?;
        let akp = key.akp()?;
        if let Some(public) = akp.public {
            let public: &[u8; crypto::mldsa::ML_DSA_87_PUBLIC_KEY_SIZE] =
                public.try_into().map_err(|_| Error::InvalidKey)?;
            return Ok(Self::from_bytes(public));
        }
        if let Some(private) = akp.private {
            let seed: &[u8; 32] = private.try_into().map_err(|_| Error::InvalidKey)?;
            return Ok(crypto::mldsa::MlDsa87SecretKey::new(seed).public_key());
        }
        Err(Error::InvalidKey)
    }
}

impl TryFrom<&CoseKey<'_>> for crypto::mldsa::MlDsa87SecretKey {
    type Error = Error;
    fn try_from(key: &CoseKey<'_>) -> Result<Self> {
        key.validate_for(Algorithm::MlDsa87, KeyOp::Sign)?;
        let private = akp_private(key, Algorithm::MlDsa87)?;
        let seed: &[u8; 32] = private.try_into().map_err(|_| Error::InvalidKey)?;
        Ok(Self::new(seed))
    }
}

impl TryFrom<&CoseKey<'_>> for RsaPublicKey {
    type Error = Error;

    fn try_from(key: &CoseKey<'_>) -> Result<Self> {
        let alg = key.alg().ok_or(Error::InvalidKey)?;
        key.validate_for(alg, KeyOp::Verify)?;
        let rsa = key.rsa()?;
        RsaPublicKey::from_n_e(alg, rsa.n, rsa.e)
    }
}
