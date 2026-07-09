use serde::{Deserialize, Serialize};
use small_collections::SmallString;
use smallvec::SmallVec;

use crate::{
    Algorithm, Blake3Key, Ed25519PublicKey, Ed25519SecretKey, Error, HmacSha256Key, HmacSha512Key, P256PublicKey,
    P256SecretKey, RsaPublicKey, Signature, Signer, Verifier,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwks {
    pub keys: SmallVec<Jwk, 5>,
}

/// a JSON Web Key
/// https://www.rfc-editor.org/rfc/rfc7517
/// https://www.rfc-editor.org/rfc/rfc8037
/// Note: Jwk are not validated during deserialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwk {
    pub kid: SmallString<36>, // 36 = UUID length
    pub r#use: KeyUse,
    #[serde(rename = "alg")]
    pub algorithm: Algorithm,

    #[serde(flatten)]
    pub crypto: JwkCrypto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE", tag = "kty")]
pub enum JwkCrypto {
    /// EdDSA
    Okp {
        #[serde(rename = "crv")]
        curve: OkpCurve,
        #[serde(with = "base64_url_no_padding")]
        x: SmallVec<u8, 32>,
        #[serde(with = "base64_url_no_padding::option", skip_serializing_if = "Option::is_none")]
        d: Option<SmallVec<u8, 32>>,
    },
    /// ECDSA
    Ec {
        #[serde(rename = "crv")]
        curve: EcCurve,
        #[serde(with = "base64_url_no_padding")]
        x: SmallVec<u8, 32>,
        #[serde(with = "base64_url_no_padding")]
        y: SmallVec<u8, 32>,
        #[serde(with = "base64_url_no_padding::option", skip_serializing_if = "Option::is_none")]
        d: Option<SmallVec<u8, 32>>,
    },
    /// Static keys
    #[serde(rename = "oct")]
    Oct {
        #[serde(with = "base64_url_no_padding")]
        key: SmallVec<u8, 32>,
    },
    /// RSA public key
    #[serde(rename = "RSA")]
    Rsa {
        // Always heap-allocated. We use a `SmallVec` to avoid needing a separate implmentation for
        // serde's `base64_url_no_padding`.
        #[serde(with = "base64_url_no_padding")]
        n: SmallVec<u8, 0>,
        #[serde(with = "base64_url_no_padding")]
        e: SmallVec<u8, 4>,
    },
}

#[derive(Copy, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyUse {
    #[serde(rename = "sig")]
    Sign,
    #[serde(rename = "enc")]
    Encrypt,
}

// https://csrc.nist.gov/pubs/fips/186-5/final
// https://csrc.nist.gov/pubs/sp/800/186/final
// https://www.rfc-editor.org/rfc/rfc8032
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum OkpCurve {
    Ed25519,
}

impl core::str::FromStr for OkpCurve {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Ed25519" => Ok(OkpCurve::Ed25519),
            _ => Err(Error::InvalidCurve),
        }
    }
}

impl core::fmt::Display for OkpCurve {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum EcCurve {
    /// P-256 and SHA-256
    P256,

    /// P-384 and SHA-384
    P384,

    /// P-521 and SHA-512
    P521,
}

impl From<&Blake3Key> for Jwk {
    #[inline]
    fn from(key: &Blake3Key) -> Self {
        return Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::BLAKE3,
            crypto: JwkCrypto::Oct {
                key: key.as_bytes().into(),
            },
        };
    }
}

impl From<&HmacSha256Key> for Jwk {
    #[inline]
    fn from(key: &HmacSha256Key) -> Self {
        return Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::HS256,
            crypto: JwkCrypto::Oct {
                key: key.as_bytes().into(),
            },
        };
    }
}

impl From<&HmacSha512Key> for Jwk {
    #[inline]
    fn from(key: &HmacSha512Key) -> Self {
        return Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::HS512,
            crypto: JwkCrypto::Oct {
                key: key.as_bytes().into(),
            },
        };
    }
}

impl From<&Ed25519SecretKey> for Jwk {
    #[inline]
    fn from(key: &Ed25519SecretKey) -> Self {
        return Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::EdDSA,
            crypto: JwkCrypto::Okp {
                curve: OkpCurve::Ed25519,
                x: key.public_key().to_bytes().into(),
                d: Some(key.to_bytes().into()),
            },
        };
    }
}

impl From<&Ed25519PublicKey> for Jwk {
    #[inline]
    fn from(key: &Ed25519PublicKey) -> Self {
        return Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::EdDSA,
            crypto: JwkCrypto::Okp {
                curve: OkpCurve::Ed25519,
                x: key.to_bytes().into(),
                d: None,
            },
        };
    }
}

impl From<&P256SecretKey> for Jwk {
    #[inline]
    fn from(key: &P256SecretKey) -> Self {
        let public_key = key.public_key();
        let (x, y) = public_key.key.x_y();
        return Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::ES256,
            crypto: JwkCrypto::Ec {
                curve: EcCurve::P256,
                x: x.into(),
                y: y.into(),
                d: Some(key.to_bytes().into()),
            },
        };
    }
}

impl From<&P256PublicKey> for Jwk {
    #[inline]
    fn from(key: &P256PublicKey) -> Self {
        let (x, y) = key.key.x_y();
        return Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::ES256,
            crypto: JwkCrypto::Ec {
                curve: EcCurve::P256,
                x: x.into(),
                y: y.into(),
                d: None,
            },
        };
    }
}

impl From<&RsaPublicKey> for Jwk {
    fn from(key: &RsaPublicKey) -> Self {
        Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: key.algorithm(),
            crypto: JwkCrypto::Rsa {
                n: key.key.n_bytes().into(),
                e: key.key.e_bytes().into(),
            },
        }
    }
}

impl From<&Key> for Jwk {
    fn from(key: &Key) -> Self {
        match key {
            Key::Blake3(k) => Jwk::from(k),
            Key::HmacSha256(k) => Jwk::from(k),
            Key::HmacSha512(k) => Jwk::from(k),
            Key::Ed25519Secret(k) => Jwk::from(k),
            Key::Ed25519Public(k) => Jwk::from(k),
            Key::P256Secret(k) => Jwk::from(k),
            Key::P256Public(k) => Jwk::from(k),
            Key::RsaPublic(k) => Jwk::from(k.as_ref()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Key
////////////////////////////////////////////////////////////////////////////////////////////////////

/// A concrete cryptographic key extracted from and that can be converted to a [`Jwk`].
///
/// Use [`Key::try_from`]`(&Jwk)` to convert a [`Jwk`] (e.g. from a JWKS endpoint)
/// into a key without knowing the exact key type at compile time. The result
/// can be passed directly to [`crate::parse_and_verify`] or [`crate::sign`].
///
/// Use [`Key::into`]`(Jwk)` to convert a [`Key`] into a [`Jwk`].
///
/// `RsaPublicKey`s require heap-allocation to avoid bloating the enum, as RSA should now be deprecated.
pub enum Key {
    Blake3(Blake3Key),
    HmacSha256(HmacSha256Key),
    HmacSha512(HmacSha512Key),
    Ed25519Secret(Ed25519SecretKey),
    Ed25519Public(Ed25519PublicKey),
    P256Secret(P256SecretKey),
    P256Public(P256PublicKey),
    RsaPublic(alloc::boxed::Box<RsaPublicKey>),
}

impl TryFrom<&Jwk> for Key {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Error> {
        match &jwk.crypto {
            JwkCrypto::Okp {
                curve,
                x,
                d,
            } => match curve {
                OkpCurve::Ed25519 => match d {
                    Some(d_bytes) => {
                        let seed: [u8; 32] = d_bytes.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                        Ok(Key::Ed25519Secret(Ed25519SecretKey::from_bytes(&seed)?))
                    }
                    None => {
                        let pk: [u8; 32] = x.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                        Ok(Key::Ed25519Public(Ed25519PublicKey::from_bytes(&pk)?))
                    }
                },
            },
            JwkCrypto::Ec {
                curve,
                x,
                y,
                d,
            } => match curve {
                EcCurve::P256 => {
                    let x_arr: [u8; 32] = x.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                    let y_arr: [u8; 32] = y.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                    match d {
                        Some(d_bytes) => {
                            let key: [u8; 32] = d_bytes.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                            Ok(Key::P256Secret(P256SecretKey::from_bytes(&key)?))
                        }
                        None => Ok(Key::P256Public(P256PublicKey::from_x_y(&x_arr, &y_arr)?)),
                    }
                }
                EcCurve::P384 | EcCurve::P521 => Err(Error::InvalidEllipticCurve(alloc::format!("{curve:?}"))),
            },
            JwkCrypto::Oct {
                key,
            } => match jwk.algorithm {
                Algorithm::BLAKE3 => {
                    let arr: [u8; 32] = key.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                    Ok(Key::Blake3(Blake3Key::from_bytes(&arr)))
                }
                Algorithm::HS256 => Ok(Key::HmacSha256(HmacSha256Key::from_bytes(key)?)),
                Algorithm::HS512 => Ok(Key::HmacSha512(HmacSha512Key::from_bytes(key)?)),
                _ => Err(Error::InvalidJwk {
                    kid: alloc::format!("{}", jwk.kid),
                    err: alloc::format!("unsupported algorithm for oct key: {:?}", jwk.algorithm),
                }),
            },
            JwkCrypto::Rsa {
                n,
                e,
            } => {
                let key = RsaPublicKey::from_n_e(jwk.algorithm, n, e)?;
                Ok(Key::RsaPublic(alloc::boxed::Box::new(key)))
            }
        }
    }
}

impl Signer for Key {
    fn sign(&self, message: &[u8]) -> Result<Signature, Error> {
        match self {
            Key::Blake3(k) => k.sign(message),
            Key::HmacSha256(k) => k.sign(message),
            Key::HmacSha512(k) => k.sign(message),
            Key::Ed25519Secret(k) => k.sign(message),
            Key::P256Secret(k) => k.sign(message),
            Key::Ed25519Public(_) | Key::P256Public(_) | Key::RsaPublic(_) => Err(Error::InvalidKey),
        }
    }

    fn algorithm(&self) -> Algorithm {
        match self {
            Key::Blake3(k) => Signer::algorithm(k),
            Key::HmacSha256(k) => Signer::algorithm(k),
            Key::HmacSha512(k) => Signer::algorithm(k),
            Key::Ed25519Secret(k) => k.algorithm(),
            Key::P256Secret(k) => k.algorithm(),
            Key::Ed25519Public(k) => k.algorithm(),
            Key::P256Public(k) => k.algorithm(),
            Key::RsaPublic(k) => k.algorithm(),
        }
    }
}

impl Verifier for Key {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        match self {
            Key::Blake3(k) => k.verify(message, signature),
            Key::HmacSha256(k) => k.verify(message, signature),
            Key::HmacSha512(k) => k.verify(message, signature),
            Key::Ed25519Secret(k) => k.public_key().verify(message, signature),
            Key::Ed25519Public(k) => k.verify(message, signature),
            Key::P256Secret(k) => k.public_key().verify(message, signature),
            Key::P256Public(k) => k.verify(message, signature),
            Key::RsaPublic(k) => k.verify(message, signature),
        }
    }

    fn algorithm(&self) -> Algorithm {
        match self {
            Key::Blake3(k) => Verifier::algorithm(k),
            Key::HmacSha256(k) => Verifier::algorithm(k),
            Key::HmacSha512(k) => Verifier::algorithm(k),
            Key::Ed25519Secret(k) => k.algorithm(),
            Key::P256Secret(k) => k.algorithm(),
            Key::Ed25519Public(k) => k.algorithm(),
            Key::P256Public(k) => k.algorithm(),
            Key::RsaPublic(k) => k.algorithm(),
        }
    }
}

mod base64_url_no_padding {
    use base64::{Alphabet, decode, encode};
    use serde::{Deserializer, Serializer};

    use super::*;

    pub fn serialize<S: Serializer, const N: usize>(data: &SmallVec<u8, N>, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&encode(data, Alphabet::UrlNoPadding))
    }

    pub fn deserialize<'de, D: Deserializer<'de>, const N: usize>(
        deserializer: D,
    ) -> Result<SmallVec<u8, N>, D::Error> {
        let s = <&str>::deserialize(deserializer)?;
        let bytes = decode(s.as_bytes(), Alphabet::UrlNoPadding).map_err(serde::de::Error::custom)?;
        Ok(SmallVec::from(bytes))
    }

    pub(crate) mod option {
        use alloc::string::String;

        use super::*;

        pub fn serialize<S: Serializer, const N: usize>(
            data: &Option<SmallVec<u8, N>>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            match data {
                Some(val) => serializer.serialize_str(&encode(val, Alphabet::UrlNoPadding)),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D: Deserializer<'de>, const N: usize>(
            deserializer: D,
        ) -> Result<Option<SmallVec<u8, N>>, D::Error> {
            let opt: Option<String> = Option::deserialize(deserializer)?;
            match opt {
                Some(s) => {
                    let bytes = decode(s.as_bytes(), Alphabet::UrlNoPadding).map_err(serde::de::Error::custom)?;
                    Ok(Some(SmallVec::from(bytes)))
                }
                None => Ok(None),
            }
        }
    }
}
