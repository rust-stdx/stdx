//! Key exchange groups for this provider.
//!
//! The preferred group is `X25519MLKEM768`, a post-quantum hybrid combining
//! X25519 with ML-KEM-768. The classical `X25519`, `secp256r1` and
//! `secp384r1` groups are also offered so that handshakes with peers that do
//! not support the hybrid still succeed.

use core::marker::PhantomData;

use crypto::{
    curve25519::x25519,
    mlkem::{CIPHERTEXT_SIZE_768, PUBLIC_KEY_SIZE_768, PublicKey768, SecretKey768, generate_keypair_768},
};
use rustls::{
    Error, NamedGroup, PeerMisbehaved, ProtocolVersion,
    crypto::{ActiveKeyExchange, CompletedKeyExchange, SharedSecret, SupportedKxGroup},
    ffdhe_groups::FfdheGroup,
};

const INVALID_KEY_SHARE: Error = Error::PeerMisbehaved(PeerMisbehaved::InvalidKeyShare);
const RANDOM_FAILURE: Error = Error::FailedToGetRandomBytes;

/// All key exchange groups supported by this provider, in preference order.
///
/// `X25519MLKEM768` comes first, so a key share for it is sent by default in
/// the client hello.
pub(crate) static KX_GROUPS: &[&dyn SupportedKxGroup] = &[X25519MLKEM768, &X25519, &SECP256R1, &SECP384R1];

/// X25519 (RFC 7748) Diffie-Hellman key exchange.
#[derive(Debug)]
pub(crate) struct X25519;

impl SupportedKxGroup for X25519 {
    fn start(&self) -> Result<Box<dyn ActiveKeyExchange>, Error> {
        let secret = x25519::SecretKey::generate().map_err(|_| RANDOM_FAILURE)?;
        let pub_key = secret.public_key().to_bytes();
        Ok(Box::new(ActiveX25519 {
            secret,
            pub_key,
        }))
    }

    fn ffdhe_group(&self) -> Option<FfdheGroup<'static>> {
        None
    }

    fn name(&self) -> NamedGroup {
        NamedGroup::X25519
    }
}

struct ActiveX25519 {
    secret: x25519::SecretKey,
    pub_key: [u8; 32],
}

impl ActiveKeyExchange for ActiveX25519 {
    fn complete(self: Box<Self>, peer_pub_key: &[u8]) -> Result<SharedSecret, Error> {
        let peer_pub_key: [u8; 32] = peer_pub_key.try_into().map_err(|_| INVALID_KEY_SHARE)?;
        let peer = x25519::PublicKey::from_bytes(&peer_pub_key);
        let secret = self.secret.ecdh(&peer).map_err(|_| INVALID_KEY_SHARE)?;
        Ok(SharedSecret::from(&secret[..]))
    }

    fn pub_key(&self) -> &[u8] {
        &self.pub_key
    }

    fn ffdhe_group(&self) -> Option<FfdheGroup<'static>> {
        None
    }

    fn group(&self) -> NamedGroup {
        NamedGroup::X25519
    }
}

/// A NIST (short Weierstrass) curve usable as an ECDHE group.
///
/// This is a local trait, so it can be implemented directly on the (foreign)
/// `crypto` secret-key types; [`NistGroup`] is the rustls `SupportedKxGroup`.
pub(crate) trait NistCurve: Send + Sync + Sized + 'static {
    /// The parsed peer public key.
    type PublicKey;
    /// The uncompressed SEC1 public-key encoding.
    type PublicBytes: AsRef<[u8]> + Send + Sync + 'static;
    /// A raw ECDH shared secret.
    type SharedSecret: AsRef<[u8]> + Send + Sync + 'static;

    /// The TLS `NamedGroup`.
    const NAME: NamedGroup;
    /// Length of the uncompressed SEC1 public key (`65` or `97`).
    const PUBLIC_KEY_LEN: usize;

    /// Generates a fresh ephemeral secret key.
    fn generate() -> Result<Self, Error>;

    /// Returns the uncompressed SEC1 public-key encoding.
    fn public_bytes(&self) -> Self::PublicBytes;

    /// Parses a peer's SEC1 public key.
    fn parse_public(bytes: &[u8]) -> Result<Self::PublicKey, Error>;

    /// Computes the ECDH shared secret with `peer`.
    fn ecdh(&self, peer: &Self::PublicKey) -> Result<Self::SharedSecret, Error>;
}

impl NistCurve for crypto::p256::SecretKey {
    type PublicKey = crypto::p256::PublicKey;
    type PublicBytes = [u8; 65];
    type SharedSecret = [u8; 32];

    const NAME: NamedGroup = NamedGroup::secp256r1;
    const PUBLIC_KEY_LEN: usize = 65;

    fn generate() -> Result<Self, Error> {
        crypto::p256::SecretKey::generate().map_err(|_| RANDOM_FAILURE)
    }

    fn public_bytes(&self) -> [u8; 65] {
        self.public_key().to_bytes()
    }

    fn parse_public(bytes: &[u8]) -> Result<Self::PublicKey, Error> {
        crypto::p256::PublicKey::from_bytes(bytes).map_err(|_| INVALID_KEY_SHARE)
    }

    fn ecdh(&self, peer: &Self::PublicKey) -> Result<[u8; 32], Error> {
        crypto::p256::SecretKey::ecdh(self, peer).map_err(|_| INVALID_KEY_SHARE)
    }
}

impl NistCurve for crypto::p384::SecretKey {
    type PublicKey = crypto::p384::PublicKey;
    type PublicBytes = [u8; 97];
    type SharedSecret = [u8; 48];

    const NAME: NamedGroup = NamedGroup::secp384r1;
    const PUBLIC_KEY_LEN: usize = 97;

    fn generate() -> Result<Self, Error> {
        crypto::p384::SecretKey::generate().map_err(|_| RANDOM_FAILURE)
    }

    fn public_bytes(&self) -> [u8; 97] {
        self.public_key().to_bytes()
    }

    fn parse_public(bytes: &[u8]) -> Result<Self::PublicKey, Error> {
        crypto::p384::PublicKey::from_bytes(bytes).map_err(|_| INVALID_KEY_SHARE)
    }

    fn ecdh(&self, peer: &Self::PublicKey) -> Result<[u8; 48], Error> {
        crypto::p384::SecretKey::ecdh(self, peer).map_err(|_| INVALID_KEY_SHARE)
    }
}

/// A NIST ECDHE `SupportedKxGroup` for a curve `K`.
pub(crate) struct NistGroup<K: NistCurve>(PhantomData<K>);

impl<K: NistCurve> SupportedKxGroup for NistGroup<K> {
    fn start(&self) -> Result<Box<dyn ActiveKeyExchange>, Error> {
        let secret = K::generate()?;
        let pub_key = secret.public_bytes();
        Ok(Box::new(ActiveNist {
            secret,
            pub_key,
        }))
    }

    fn ffdhe_group(&self) -> Option<FfdheGroup<'static>> {
        None
    }

    fn name(&self) -> NamedGroup {
        K::NAME
    }
}

impl<K: NistCurve> core::fmt::Debug for NistGroup<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::any::type_name::<K>())
    }
}

struct ActiveNist<K: NistCurve> {
    secret: K,
    pub_key: K::PublicBytes,
}

impl<K: NistCurve> ActiveKeyExchange for ActiveNist<K> {
    fn complete(self: Box<Self>, peer_pub_key: &[u8]) -> Result<SharedSecret, Error> {
        // TLS 1.3 (RFC 8446 section 4.2.8.2) requires the uncompressed
        // SEC1 point encoding for NIST curves. The underlying parser
        // also accepts the compressed form, so reject it explicitly to
        // match rustls' own providers.
        if peer_pub_key.len() != K::PUBLIC_KEY_LEN || peer_pub_key.first() != Some(&0x04) {
            return Err(INVALID_KEY_SHARE);
        }
        let peer = K::parse_public(peer_pub_key)?;
        let secret = self.secret.ecdh(&peer)?;
        Ok(SharedSecret::from(secret.as_ref()))
    }

    fn pub_key(&self) -> &[u8] {
        self.pub_key.as_ref()
    }

    fn ffdhe_group(&self) -> Option<FfdheGroup<'static>> {
        None
    }

    fn group(&self) -> NamedGroup {
        K::NAME
    }
}

/// secp256r1 (NIST P-256) ECDHE.
pub(crate) static SECP256R1: NistGroup<crypto::p256::SecretKey> = NistGroup(PhantomData);
/// secp384r1 (NIST P-384) ECDHE.
pub(crate) static SECP384R1: NistGroup<crypto::p384::SecretKey> = NistGroup(PhantomData);

/// ML-KEM-768 (FIPS 203) as a key-encapsulation component.
///
/// This is not offered as a standalone TLS group; it is only used as the
/// post-quantum half of [`X25519MLKEM768`].
#[derive(Debug)]
struct MlKem768;

impl SupportedKxGroup for MlKem768 {
    fn start(&self) -> Result<Box<dyn ActiveKeyExchange>, Error> {
        let (secret, public) = generate_keypair_768().map_err(|_| RANDOM_FAILURE)?;
        let pub_key = public.to_bytes();
        Ok(Box::new(ActiveMlKem768 {
            secret,
            pub_key,
        }))
    }

    fn start_and_complete(&self, client_share: &[u8]) -> Result<CompletedKeyExchange, Error> {
        let client_share: [u8; PUBLIC_KEY_SIZE_768] = client_share.try_into().map_err(|_| INVALID_KEY_SHARE)?;
        // FIPS 203 section 7.2: an encapsulation key whose `t̂` is not canonical is a peer misbehaviour.
        let public = PublicKey768::from_bytes(&client_share).map_err(|_| INVALID_KEY_SHARE)?;
        let (secret, ciphertext) = public.encapsulate().map_err(|_| RANDOM_FAILURE)?;

        Ok(CompletedKeyExchange {
            group: self.name(),
            pub_key: ciphertext.to_vec(),
            secret: SharedSecret::from(&secret[..]),
        })
    }

    fn ffdhe_group(&self) -> Option<FfdheGroup<'static>> {
        None
    }

    fn name(&self) -> NamedGroup {
        NamedGroup::MLKEM768
    }

    fn usable_for_version(&self, version: ProtocolVersion) -> bool {
        version == ProtocolVersion::TLSv1_3
    }
}

struct ActiveMlKem768 {
    secret: SecretKey768,
    pub_key: [u8; PUBLIC_KEY_SIZE_768],
}

impl ActiveKeyExchange for ActiveMlKem768 {
    fn complete(self: Box<Self>, peer_pub_key: &[u8]) -> Result<SharedSecret, Error> {
        let ciphertext: [u8; CIPHERTEXT_SIZE_768] = peer_pub_key.try_into().map_err(|_| INVALID_KEY_SHARE)?;
        let secret = self.secret.decapsulate(&ciphertext).map_err(|_| INVALID_KEY_SHARE)?;
        Ok(SharedSecret::from(&secret[..]))
    }

    fn pub_key(&self) -> &[u8] {
        &self.pub_key
    }

    fn group(&self) -> NamedGroup {
        NamedGroup::MLKEM768
    }
}

/// The `X25519MLKEM768` post-quantum hybrid group.
///
/// The client share is `ML-KEM-768 encapsulation key || X25519 public key`
/// and the server share is `ML-KEM-768 ciphertext || X25519 public key`. The
/// combined shared secret is `ML-KEM-768 shared secret || X25519 shared
/// secret`.
pub(crate) static X25519MLKEM768: &dyn SupportedKxGroup = &crate::hybrid_key_exchange::Hybrid {
    classical: &X25519,
    post_quantum: &MlKem768,
    name: NamedGroup::X25519MLKEM768,
    layout: crate::hybrid_key_exchange::Layout {
        classical_share_len: 32,
        post_quantum_client_share_len: PUBLIC_KEY_SIZE_768,
        post_quantum_server_share_len: CIPHERTEXT_SIZE_768,
        post_quantum_first: true,
    },
};

#[cfg(test)]
mod tests {
    use super::*;

    fn exercise(group: &'static dyn SupportedKxGroup) {
        let client = group.start().unwrap();
        let client_pub = client.pub_key().to_vec();

        let server = group.start_and_complete(&client_pub).unwrap();
        let client_secret = client.complete(&server.pub_key).unwrap();

        assert_eq!(client_secret.secret_bytes(), server.secret.secret_bytes());
        assert_eq!(group.name(), server.group);
    }

    #[test]
    fn x25519_round_trip() {
        exercise(&X25519);
        assert_eq!(X25519.start().unwrap().pub_key().len(), 32);
    }

    #[test]
    fn secp256r1_round_trip() {
        exercise(&SECP256R1);
        assert_eq!(SECP256R1.start().unwrap().pub_key().len(), 65);
    }

    #[test]
    fn secp384r1_round_trip() {
        exercise(&SECP384R1);
        assert_eq!(SECP384R1.start().unwrap().pub_key().len(), 97);
    }

    #[test]
    fn nist_groups_reject_compressed_points() {
        // TLS 1.3 requires the uncompressed point encoding. The underlying
        // parser also accepts compressed points, so the key exchange must
        // reject them explicitly.
        let active = SECP256R1.start().unwrap();
        let public = crypto::p256::PublicKey::from_bytes(active.pub_key()).unwrap();
        let compressed = public.to_compressed_bytes();
        assert!(active.complete(&compressed).is_err());

        // A correctly sized share with the wrong SEC1 prefix is also rejected.
        let active = SECP256R1.start().unwrap();
        let mut wrong_prefix = active.pub_key().to_vec();
        wrong_prefix[0] = 0x02;
        assert!(active.complete(&wrong_prefix).is_err());

        let active = SECP384R1.start().unwrap();
        let public = crypto::p384::PublicKey::from_bytes(active.pub_key()).unwrap();
        let compressed = public.to_compressed_bytes();
        assert!(active.complete(&compressed).is_err());
    }

    #[test]
    fn x25519_mlkem768_round_trip() {
        exercise(X25519MLKEM768);

        // Client share: ML-KEM-768 encapsulation key (1184) + X25519 public key (32).
        let client = X25519MLKEM768.start().unwrap();
        assert_eq!(client.pub_key().len(), 1216);

        // Server share: ML-KEM-768 ciphertext (1088) + X25519 public key (32).
        let server = X25519MLKEM768.start_and_complete(client.pub_key()).unwrap();
        assert_eq!(server.pub_key.len(), 1120);
    }

    #[test]
    fn invalid_shares_are_rejected() {
        for group in KX_GROUPS {
            let active = group.start().unwrap();
            assert!(active.complete(&[0u8]).is_err(), "{:?}", group.name());
            assert!(group.start_and_complete(&[0u8]).is_err(), "{:?}", group.name());
        }
    }

    #[test]
    fn mlkem_rejects_non_canonical_encapsulation_key() {
        // A 12-bit coefficient equal to q (3329) is not a canonical `t̂` and
        // must be reported as peer misbehaviour, not accepted.
        let mut share = [0u8; PUBLIC_KEY_SIZE_768];
        share[0] = 0x01;
        share[1] = 0x0d; // first coefficient = 0x0d01 = 3329
        assert!(matches!(
            MlKem768.start_and_complete(&share),
            Err(Error::PeerMisbehaved(PeerMisbehaved::InvalidKeyShare))
        ));
    }

    #[test]
    fn hybrid_rejects_non_canonical_mlkem_share() {
        let active = X25519MLKEM768.start().unwrap();
        let mut combined = active.pub_key().to_vec();
        assert_eq!(combined.len(), 1216);
        combined[0] = 0x01;
        combined[1] = (combined[1] & 0xf0) | 0x0d;
        assert!(matches!(
            X25519MLKEM768.start_and_complete(&combined),
            Err(Error::PeerMisbehaved(PeerMisbehaved::InvalidKeyShare))
        ));
    }

    #[test]
    fn groups_are_tls13_only_for_hybrid() {
        assert!(X25519MLKEM768.usable_for_version(ProtocolVersion::TLSv1_3));
        assert!(!X25519MLKEM768.usable_for_version(ProtocolVersion::TLSv1_2));
        assert!(X25519.usable_for_version(ProtocolVersion::TLSv1_2));
    }
}
