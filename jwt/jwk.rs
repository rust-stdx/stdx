use alloc::{borrow::Cow, string::ToString};

use crypto::{
    curve25519::ed25519,
    mldsa::{
        MlDsa44PublicKey, MlDsa44SecretKey, MlDsa65PublicKey, MlDsa65SecretKey, MlDsa87PublicKey, MlDsa87SecretKey,
    },
    p256, p384, p521,
};
use json::{Decode, Decoder, Encode, Encoder, ErrorKind, Result, Write};
use small_collections::{SmallString, SmallVec};

use crate::{Algorithm, Error, RsaPublicKey, SecretKey};

#[derive(Debug, Clone, json::Encode, json::Decode)]
pub struct Jwks {
    pub keys: SmallVec<Jwk, 5>,
}

/// a JSON Web Key
/// https://www.rfc-editor.org/rfc/rfc7517
/// https://www.rfc-editor.org/rfc/rfc8037
/// https://www.ietf.org/archive/id/draft-ietf-jose-pqc-02.html
/// Note: Jwk are not validated during deserialization
#[derive(Debug, Clone)]
pub struct Jwk {
    pub kid: SmallString<36>, // 36 = UUID length
    pub r#use: KeyUse,
    pub algorithm: Algorithm,

    pub crypto: JwkCrypto,
}

impl Encode for Jwk {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        let mut map = json::Map::new();
        map.push(Cow::Borrowed("kid"), json::Value::from(self.kid.as_str()));
        map.push(
            Cow::Borrowed("use"),
            json::Value::from(match self.r#use {
                KeyUse::Sign => "sig",
                KeyUse::Encrypt => "enc",
            }),
        );
        map.push(Cow::Borrowed("alg"), json::Value::from(self.algorithm.to_string()));
        match &self.crypto {
            JwkCrypto::Okp {
                curve,
                x,
                d,
            } => {
                map.push(Cow::Borrowed("kty"), json::Value::from("OKP"));
                map.push(Cow::Borrowed("crv"), json::Value::from(curve.to_string()));
                map.push(
                    Cow::Borrowed("x"),
                    json::Value::from(base64::encode(x, base64::Alphabet::UrlNoPadding)),
                );
                if let Some(d) = d {
                    map.push(
                        Cow::Borrowed("d"),
                        json::Value::from(base64::encode(d, base64::Alphabet::UrlNoPadding)),
                    );
                }
            }
            JwkCrypto::Ec {
                curve,
                x,
                y,
                d,
            } => {
                map.push(Cow::Borrowed("kty"), json::Value::from("EC"));
                map.push(Cow::Borrowed("crv"), json::Value::from(curve.to_string()));
                map.push(
                    Cow::Borrowed("x"),
                    json::Value::from(base64::encode(x, base64::Alphabet::UrlNoPadding)),
                );
                map.push(
                    Cow::Borrowed("y"),
                    json::Value::from(base64::encode(y, base64::Alphabet::UrlNoPadding)),
                );
                if let Some(d) = d {
                    map.push(
                        Cow::Borrowed("d"),
                        json::Value::from(base64::encode(d, base64::Alphabet::UrlNoPadding)),
                    );
                }
            }
            JwkCrypto::Oct {
                key,
            } => {
                map.push(Cow::Borrowed("kty"), json::Value::from("oct"));
                map.push(
                    Cow::Borrowed("key"),
                    json::Value::from(base64::encode(key, base64::Alphabet::UrlNoPadding)),
                );
            }
            JwkCrypto::Rsa {
                n,
                e,
            } => {
                map.push(Cow::Borrowed("kty"), json::Value::from("RSA"));
                map.push(
                    Cow::Borrowed("n"),
                    json::Value::from(base64::encode(n, base64::Alphabet::UrlNoPadding)),
                );
                map.push(
                    Cow::Borrowed("e"),
                    json::Value::from(base64::encode(e, base64::Alphabet::UrlNoPadding)),
                );
            }
            JwkCrypto::Akp {
                pub_key,
                private_key,
            } => {
                map.push(Cow::Borrowed("kty"), json::Value::from("AKP"));
                map.push(
                    Cow::Borrowed("pub"),
                    json::Value::from(base64::encode(pub_key, base64::Alphabet::UrlNoPadding)),
                );
                if let Some(private) = private_key {
                    map.push(
                        Cow::Borrowed("priv"),
                        json::Value::from(base64::encode(private, base64::Alphabet::UrlNoPadding)),
                    );
                }
            }
        }
        json::Value::Object(map).encode(encoder)
    }
}

impl<'a> Decode<'a> for Jwk {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        let value = decoder.decode::<json::Value<'a>>()?;
        let object = value
            .as_object()
            .ok_or_else(|| decoder.error(ErrorKind::Message("JWK must be an object")))?;

        let kid = field_str(object, "kid")?;
        let key_use = match field_str(object, "use")? {
            "sig" => KeyUse::Sign,
            "enc" => KeyUse::Encrypt,
            _ => return Err(decoder.error(ErrorKind::Message("invalid JWK use"))),
        };
        let algorithm = field_str(object, "alg")?
            .parse::<Algorithm>()
            .map_err(|_| decoder.error(ErrorKind::Message("invalid JWK algorithm")))?;

        let crypto = match field_str(object, "kty")? {
            "OKP" => JwkCrypto::Okp {
                curve: field_str(object, "crv")?
                    .parse::<OkpCurve>()
                    .map_err(|_| decoder.error(ErrorKind::Message("invalid OKP curve")))?,
                x: field_b64(object, "x")?,
                d: field_b64_opt(object, "d")?,
            },
            "EC" => JwkCrypto::Ec {
                curve: field_str(object, "crv")?
                    .parse::<EcCurve>()
                    .map_err(|_| decoder.error(ErrorKind::Message("invalid EC curve")))?,
                x: field_b64(object, "x")?,
                y: field_b64(object, "y")?,
                d: field_b64_opt(object, "d")?,
            },
            "oct" => JwkCrypto::Oct {
                key: field_b64(object, "key")?,
            },
            "RSA" => JwkCrypto::Rsa {
                n: field_b64(object, "n")?,
                e: field_b64(object, "e")?,
            },
            "AKP" => JwkCrypto::Akp {
                pub_key: field_b64(object, "pub")?,
                private_key: field_b64_opt(object, "priv")?,
            },
            _ => return Err(decoder.error(ErrorKind::Message("invalid JWK key type"))),
        };

        Ok(Jwk {
            kid: SmallString::from_str(kid),
            r#use: key_use,
            algorithm,
            crypto,
        })
    }
}

fn field_str<'a, 'v>(object: &'v json::Map<'a>, name: &'static str) -> Result<&'v str> {
    object
        .get(name)
        .and_then(json::Value::as_str)
        .ok_or_else(|| json::Error::message(name, 0))
}

fn field_b64<'a, 'v, const N: usize>(object: &'v json::Map<'a>, name: &'static str) -> Result<SmallVec<u8, N>> {
    let text = field_str(object, name)?;
    base64::decode(text.as_bytes(), base64::Alphabet::UrlNoPadding)
        .map(SmallVec::from)
        .map_err(|_| json::Error::message(name, 0))
}

fn field_b64_opt<'a, 'v, const N: usize>(
    object: &'v json::Map<'a>,
    name: &'static str,
) -> Result<Option<SmallVec<u8, N>>> {
    match object.get(name) {
        None => Ok(None),
        Some(value) => {
            let text = value.as_str().ok_or_else(|| json::Error::message(name, 0))?;
            base64::decode(text.as_bytes(), base64::Alphabet::UrlNoPadding)
                .map(SmallVec::from)
                .map(Some)
                .map_err(|_| json::Error::message(name, 0))
        }
    }
}

#[derive(Debug, Clone, json::Encode, json::Decode)]
#[json(rename_all = "UPPERCASE", tag = "kty")]
pub enum JwkCrypto {
    /// EdDSA
    Okp {
        #[json(rename = "crv")]
        curve: OkpCurve,
        #[json(with = "base64_url_no_padding")]
        x: SmallVec<u8, 32>,
        #[json(with = "base64_url_no_padding::option", skip_serializing_if = "Option::is_none")]
        d: Option<SmallVec<u8, 32>>,
    },
    /// ECDSA
    Ec {
        #[json(rename = "crv")]
        curve: EcCurve,
        #[json(with = "base64_url_no_padding")]
        x: SmallVec<u8, 32>,
        #[json(with = "base64_url_no_padding")]
        y: SmallVec<u8, 32>,
        #[json(with = "base64_url_no_padding::option", skip_serializing_if = "Option::is_none")]
        d: Option<SmallVec<u8, 32>>,
    },
    /// Static keys
    #[json(rename = "oct")]
    Oct {
        #[json(with = "base64_url_no_padding")]
        key: SmallVec<u8, 32>,
    },
    /// RSA public key
    #[json(rename = "RSA")]
    Rsa {
        // Always heap-allocated. We use a `SmallVec` to avoid needing a separate implmentation for
        // serde's `base64_url_no_padding`.
        #[json(with = "base64_url_no_padding")]
        n: SmallVec<u8, 0>,
        #[json(with = "base64_url_no_padding")]
        e: SmallVec<u8, 4>,
    },
    /// ML-DSA public key (draft "AKP" key type)
    ///
    /// `pub` holds the encoded public key and the optional `priv` holds the 32-byte seed the
    /// signing key was generated from.
    #[json(rename = "AKP")]
    Akp {
        #[json(rename = "pub", with = "base64_url_no_padding")]
        pub_key: SmallVec<u8, 0>,
        #[json(
            rename = "priv",
            with = "base64_url_no_padding::option",
            skip_serializing_if = "Option::is_none"
        )]
        private_key: Option<SmallVec<u8, 0>>,
    },
}

#[derive(Copy, Debug, Clone, json::Encode, json::Decode, PartialEq, Eq)]
pub enum KeyUse {
    #[json(rename = "sig")]
    Sign,
    #[json(rename = "enc")]
    Encrypt,
}

// https://csrc.nist.gov/pubs/fips/186-5/final
// https://csrc.nist.gov/pubs/sp/800/186/final
// https://www.rfc-editor.org/rfc/rfc8032
#[derive(Copy, Clone, Debug, json::Decode, json::Encode, PartialEq, Eq)]
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

#[derive(Copy, Clone, Debug, json::Decode, json::Encode, PartialEq, Eq)]
pub enum EcCurve {
    /// P-256 and SHA-256
    #[json(rename = "P-256")]
    P256,

    /// P-384 and SHA-384
    #[json(rename = "P-384")]
    P384,

    /// P-521 and SHA-512
    #[json(rename = "P-521")]
    P521,
}

impl core::str::FromStr for EcCurve {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "P-256" => Ok(EcCurve::P256),
            "P-384" => Ok(EcCurve::P384),
            "P-521" => Ok(EcCurve::P521),
            _ => Err(Error::InvalidCurve),
        }
    }
}

impl core::fmt::Display for EcCurve {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        f.write_str(match self {
            EcCurve::P256 => "P-256",
            EcCurve::P384 => "P-384",
            EcCurve::P521 => "P-521",
        })
    }
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
                e: key.key.e_bytes(),
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
    use json::{Decoder, Encoder, ErrorKind, Write};

    use super::*;

    pub fn serialize<W: Write, const N: usize>(
        data: &SmallVec<u8, N>,
        encoder: &mut Encoder<W>,
    ) -> core::result::Result<(), W::Error> {
        encoder.string(&encode(data, Alphabet::UrlNoPadding))
    }

    pub fn deserialize<'a, const N: usize>(decoder: &mut Decoder<'a>) -> Result<SmallVec<u8, N>> {
        let text = decoder.decode_str()?;
        decode(text.as_bytes(), Alphabet::UrlNoPadding)
            .map(SmallVec::from)
            .map_err(|_| decoder.error(ErrorKind::Message("invalid base64")))
    }

    pub(crate) mod option {
        use super::*;

        pub fn serialize<W: Write, const N: usize>(
            data: &Option<SmallVec<u8, N>>,
            encoder: &mut Encoder<W>,
        ) -> core::result::Result<(), W::Error> {
            match data {
                Some(value) => encoder.string(&encode(value, Alphabet::UrlNoPadding)),
                None => encoder.null(),
            }
        }

        pub fn deserialize<'a, const N: usize>(decoder: &mut Decoder<'a>) -> Result<Option<SmallVec<u8, N>>> {
            decoder.skip_ws()?;
            if decoder.peek_byte() == Some(b'n') {
                decoder.decode_null()?;
                Ok(None)
            } else {
                let text = decoder.decode_str()?;
                decode(text.as_bytes(), Alphabet::UrlNoPadding)
                    .map(SmallVec::from)
                    .map(Some)
                    .map_err(|_| decoder.error(ErrorKind::Message("invalid base64")))
            }
        }
    }
}
