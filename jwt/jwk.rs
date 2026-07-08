use serde::{Deserialize, Serialize};
use small_collections::SmallString;
use smallvec::SmallVec;

use crate::{Algorithm, Blake3Key, Ed25519PublicKey, Error, HmacSha256Key, HmacSha512Key, P256PublicKey};

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
    Okp {
        #[serde(rename = "crv")]
        curve: OkpCurve,
        #[serde(with = "base64_url_no_padding")]
        x: SmallVec<u8, 32>,
        #[serde(with = "base64_url_no_padding::option", skip_serializing_if = "Option::is_none")]
        d: Option<SmallVec<u8, 32>>,
    },
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
    #[serde(rename = "oct")]
    Oct {
        #[serde(with = "base64_url_no_padding")]
        key: SmallVec<u8, 32>,
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl From<&P256PublicKey> for Jwk {
    #[inline]
    fn from(key: &P256PublicKey) -> Self {
        let (x, y) = key.key.to_x_y();
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
