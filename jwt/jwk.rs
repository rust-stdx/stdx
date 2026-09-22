use crypto::{
    curve25519::ed25519,
    mldsa::{
        MlDsa44PublicKey, MlDsa44SecretKey, MlDsa65PublicKey, MlDsa65SecretKey, MlDsa87PublicKey, MlDsa87SecretKey,
    },
    p256, p384, p521,
};
use serde::{Deserialize, Serialize};
use small_collections::SmallString;
use smallvec::SmallVec;

use crate::{Algorithm, Error, RsaPublicKey, SecretKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwks {
    pub keys: SmallVec<Jwk, 5>,
}

/// a JSON Web Key
/// https://www.rfc-editor.org/rfc/rfc7517
/// https://www.rfc-editor.org/rfc/rfc8037
/// https://www.ietf.org/archive/id/draft-ietf-jose-pqc-02.html
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
    /// ML-DSA public key (draft "AKP" key type)
    ///
    /// `pub` holds the encoded public key and the optional `priv` holds the 32-byte seed the
    /// signing key was generated from.
    #[serde(rename = "AKP")]
    Akp {
        #[serde(rename = "pub", with = "base64_url_no_padding")]
        pub_key: SmallVec<u8, 0>,
        #[serde(
            rename = "priv",
            with = "base64_url_no_padding::option",
            skip_serializing_if = "Option::is_none"
        )]
        private_key: Option<SmallVec<u8, 0>>,
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
    #[serde(rename = "P-256")]
    P256,

    /// P-384 and SHA-384
    #[serde(rename = "P-384")]
    P384,

    /// P-521 and SHA-512
    #[serde(rename = "P-521")]
    P521,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Secret key (BLAKE3 / HMAC)
////////////////////////////////////////////////////////////////////////////////////////////////////

impl From<&SecretKey<'_>> for Jwk {
    #[inline]
    fn from(key: &SecretKey<'_>) -> Self {
        return Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: key.algorithm,
            crypto: JwkCrypto::Oct {
                key: key.key.into(),
            },
        };
    }
}

impl<'a> TryFrom<&'a Jwk> for SecretKey<'a> {
    type Error = Error;

    #[inline]
    fn try_from(jwk: &'a Jwk) -> Result<Self, Self::Error> {
        if !matches!(
            jwk.algorithm,
            Algorithm::BLAKE3 | Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512
        ) {
            return Err(Error::InvalidKey);
        }

        match &jwk.crypto {
            JwkCrypto::Oct {
                key,
            } => Ok(SecretKey::new(jwk.algorithm, key.as_slice())),
            _ => Err(Error::InvalidKey),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ed25519
////////////////////////////////////////////////////////////////////////////////////////////////////

impl From<&ed25519::SecretKey> for Jwk {
    #[inline]
    fn from(key: &ed25519::SecretKey) -> Self {
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

impl From<&ed25519::PublicKey> for Jwk {
    #[inline]
    fn from(key: &ed25519::PublicKey) -> Self {
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

impl TryFrom<&Jwk> for ed25519::SecretKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        match &jwk.crypto {
            JwkCrypto::Okp {
                curve: OkpCurve::Ed25519,
                d: Some(d_bytes),
                ..
            } => {
                let seed: [u8; 32] = d_bytes.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                Ok(ed25519::SecretKey::from_bytes(&seed))
            }
            _ => Err(Error::InvalidKey),
        }
    }
}

impl TryFrom<&Jwk> for ed25519::PublicKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        match &jwk.crypto {
            JwkCrypto::Okp {
                curve: OkpCurve::Ed25519,
                x,
                ..
            } => {
                let public_key: [u8; 32] = x.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                ed25519::PublicKey::from_bytes(&public_key).map_err(|_| Error::InvalidKey)
            }
            _ => Err(Error::InvalidKey),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// P-256
////////////////////////////////////////////////////////////////////////////////////////////////////

impl From<&p256::SecretKey> for Jwk {
    #[inline]
    fn from(key: &p256::SecretKey) -> Self {
        let (x, y) = key.public_key().x_y();
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

impl From<&p256::PublicKey> for Jwk {
    #[inline]
    fn from(key: &p256::PublicKey) -> Self {
        let (x, y) = key.x_y();
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

impl TryFrom<&Jwk> for p256::SecretKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        match &jwk.crypto {
            JwkCrypto::Ec {
                curve: EcCurve::P256,
                d: Some(d_bytes),
                ..
            } => {
                let key: [u8; 32] = d_bytes.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                p256::SecretKey::from_bytes(&key).map_err(|_| Error::InvalidKey)
            }
            _ => Err(Error::InvalidKey),
        }
    }
}

impl TryFrom<&Jwk> for p256::PublicKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        match &jwk.crypto {
            JwkCrypto::Ec {
                curve: EcCurve::P256,
                x,
                y,
                ..
            } => {
                let x: [u8; 32] = x.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                let y: [u8; 32] = y.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                p256::PublicKey::from_x_y(&x, &y).map_err(|_| Error::InvalidKey)
            }
            _ => Err(Error::InvalidKey),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// P-384
////////////////////////////////////////////////////////////////////////////////////////////////////

impl From<&p384::PublicKey> for Jwk {
    #[inline]
    fn from(key: &p384::PublicKey) -> Self {
        let (x, y) = key.x_y();
        return Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::ES384,
            crypto: JwkCrypto::Ec {
                curve: EcCurve::P384,
                x: x.into(),
                y: y.into(),
                d: None,
            },
        };
    }
}

impl TryFrom<&Jwk> for p384::PublicKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        match &jwk.crypto {
            JwkCrypto::Ec {
                curve: EcCurve::P384,
                x,
                y,
                ..
            } => {
                let x: [u8; 48] = x.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                let y: [u8; 48] = y.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                p384::PublicKey::from_x_y(&x, &y).map_err(|_| Error::InvalidKey)
            }
            _ => Err(Error::InvalidKey),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// P-521
////////////////////////////////////////////////////////////////////////////////////////////////////

impl From<&p521::SecretKey> for Jwk {
    #[inline]
    fn from(key: &p521::SecretKey) -> Self {
        let (x, y) = key.public_key().x_y();
        return Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::ES512,
            crypto: JwkCrypto::Ec {
                curve: EcCurve::P521,
                x: x.into(),
                y: y.into(),
                d: Some(key.to_bytes().into()),
            },
        };
    }
}

impl From<&p521::PublicKey> for Jwk {
    #[inline]
    fn from(key: &p521::PublicKey) -> Self {
        let (x, y) = key.x_y();
        return Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::ES512,
            crypto: JwkCrypto::Ec {
                curve: EcCurve::P521,
                x: x.into(),
                y: y.into(),
                d: None,
            },
        };
    }
}

impl TryFrom<&Jwk> for p521::SecretKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        match &jwk.crypto {
            JwkCrypto::Ec {
                curve: EcCurve::P521,
                d: Some(d_bytes),
                ..
            } => {
                let key: [u8; 66] = d_bytes.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                p521::SecretKey::from_bytes(&key).map_err(|_| Error::InvalidKey)
            }
            _ => Err(Error::InvalidKey),
        }
    }
}

impl TryFrom<&Jwk> for p521::PublicKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        match &jwk.crypto {
            JwkCrypto::Ec {
                curve: EcCurve::P521,
                x,
                y,
                ..
            } => {
                let x: [u8; 66] = x.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                let y: [u8; 66] = y.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                p521::PublicKey::from_x_y(&x, &y).map_err(|_| Error::InvalidKey)
            }
            _ => Err(Error::InvalidKey),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// RSA
////////////////////////////////////////////////////////////////////////////////////////////////////

impl From<&RsaPublicKey> for Jwk {
    fn from(key: &RsaPublicKey) -> Self {
        Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: key.alg,
            crypto: JwkCrypto::Rsa {
                n: key.key.n_bytes().into(),
                e: key.key.e_bytes().into(),
            },
        }
    }
}

impl TryFrom<&Jwk> for RsaPublicKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        match &jwk.crypto {
            JwkCrypto::Rsa {
                n,
                e,
            } => RsaPublicKey::from_n_e(jwk.algorithm, n, e),
            _ => Err(Error::InvalidKey),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ML-DSA-44
////////////////////////////////////////////////////////////////////////////////////////////////////

impl From<&MlDsa44PublicKey> for Jwk {
    fn from(key: &MlDsa44PublicKey) -> Self {
        Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::MlDsa44,
            crypto: JwkCrypto::Akp {
                pub_key: key.to_bytes().into(),
                private_key: None,
            },
        }
    }
}

impl From<&MlDsa44SecretKey> for Jwk {
    fn from(key: &MlDsa44SecretKey) -> Self {
        Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::MlDsa44,
            crypto: JwkCrypto::Akp {
                pub_key: key.public_key().to_bytes().into(),
                private_key: Some(key.seed().as_slice().into()),
            },
        }
    }
}

impl TryFrom<&Jwk> for MlDsa44PublicKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        if jwk.algorithm != Algorithm::MlDsa44 {
            return Err(Error::InvalidKey);
        }

        match &jwk.crypto {
            JwkCrypto::Akp {
                pub_key, ..
            } => MlDsa44PublicKey::try_from(pub_key.as_slice()).map_err(|_| Error::InvalidKey),
            _ => Err(Error::InvalidKey),
        }
    }
}

impl TryFrom<&Jwk> for MlDsa44SecretKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        if jwk.algorithm != Algorithm::MlDsa44 {
            return Err(Error::InvalidKey);
        }

        match &jwk.crypto {
            JwkCrypto::Akp {
                private_key: Some(seed),
                ..
            } => {
                let seed: [u8; 32] = seed.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                Ok(MlDsa44SecretKey::new(&seed))
            }
            _ => Err(Error::InvalidKey),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ML-DSA-65
////////////////////////////////////////////////////////////////////////////////////////////////////

impl From<&MlDsa65PublicKey> for Jwk {
    fn from(key: &MlDsa65PublicKey) -> Self {
        Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::MlDsa65,
            crypto: JwkCrypto::Akp {
                pub_key: key.to_bytes().into(),
                private_key: None,
            },
        }
    }
}

impl From<&MlDsa65SecretKey> for Jwk {
    fn from(key: &MlDsa65SecretKey) -> Self {
        Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::MlDsa65,
            crypto: JwkCrypto::Akp {
                pub_key: key.public_key().to_bytes().into(),
                private_key: Some(key.seed().as_slice().into()),
            },
        }
    }
}

impl TryFrom<&Jwk> for MlDsa65PublicKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        if jwk.algorithm != Algorithm::MlDsa65 {
            return Err(Error::InvalidKey);
        }

        match &jwk.crypto {
            JwkCrypto::Akp {
                pub_key, ..
            } => MlDsa65PublicKey::try_from(pub_key.as_slice()).map_err(|_| Error::InvalidKey),
            _ => Err(Error::InvalidKey),
        }
    }
}

impl TryFrom<&Jwk> for MlDsa65SecretKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        if jwk.algorithm != Algorithm::MlDsa65 {
            return Err(Error::InvalidKey);
        }

        match &jwk.crypto {
            JwkCrypto::Akp {
                private_key: Some(seed),
                ..
            } => {
                let seed: [u8; 32] = seed.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                Ok(MlDsa65SecretKey::new(&seed))
            }
            _ => Err(Error::InvalidKey),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ML-DSA-87
////////////////////////////////////////////////////////////////////////////////////////////////////

impl From<&MlDsa87PublicKey> for Jwk {
    fn from(key: &MlDsa87PublicKey) -> Self {
        Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::MlDsa87,
            crypto: JwkCrypto::Akp {
                pub_key: key.to_bytes().into(),
                private_key: None,
            },
        }
    }
}

impl From<&MlDsa87SecretKey> for Jwk {
    fn from(key: &MlDsa87SecretKey) -> Self {
        Jwk {
            kid: SmallString::new(),
            r#use: KeyUse::Sign,
            algorithm: Algorithm::MlDsa87,
            crypto: JwkCrypto::Akp {
                pub_key: key.public_key().to_bytes().into(),
                private_key: Some(key.seed().as_slice().into()),
            },
        }
    }
}

impl TryFrom<&Jwk> for MlDsa87PublicKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        if jwk.algorithm != Algorithm::MlDsa87 {
            return Err(Error::InvalidKey);
        }

        match &jwk.crypto {
            JwkCrypto::Akp {
                pub_key, ..
            } => MlDsa87PublicKey::try_from(pub_key.as_slice()).map_err(|_| Error::InvalidKey),
            _ => Err(Error::InvalidKey),
        }
    }
}

impl TryFrom<&Jwk> for MlDsa87SecretKey {
    type Error = Error;

    fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
        if jwk.algorithm != Algorithm::MlDsa87 {
            return Err(Error::InvalidKey);
        }

        match &jwk.crypto {
            JwkCrypto::Akp {
                private_key: Some(seed),
                ..
            } => {
                let seed: [u8; 32] = seed.as_slice().try_into().map_err(|_| Error::InvalidKey)?;
                Ok(MlDsa87SecretKey::new(&seed))
            }
            _ => Err(Error::InvalidKey),
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
