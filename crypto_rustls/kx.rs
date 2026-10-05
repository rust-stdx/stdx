//! Key exchange groups for this provider.
//!
//! The preferred group is `X25519MLKEM768`, a post-quantum hybrid combining
//! X25519 with ML-KEM-768. The classical `X25519`, `secp256r1` and
//! `secp384r1` groups are also offered so that handshakes with peers that do
//! not support the hybrid still succeed.

use crypto::{
    curve25519::x25519,
    mlkem::{CIPHERTEXT_SIZE_768, PUBLIC_KEY_SIZE_768, PublicKey768, SecretKey768, generate_keypair_768},
};
use rustls::{
    Error, NamedGroup, PeerMisbehaved, ProtocolVersion,
    crypto::{ActiveKeyExchange, CompletedKeyExchange, SharedSecret, SupportedKxGroup},
    ffdhe_groups::FfdheGroup,
};

mod hybrid;

const INVALID_KEY_SHARE: Error = Error::PeerMisbehaved(PeerMisbehaved::InvalidKeyShare);
const RANDOM_FAILURE: Error = Error::FailedToGetRandomBytes;

/// All key exchange groups supported by this provider, in preference order.
///
/// `X25519MLKEM768` comes first, so a key share for it is sent by default in
/// the client hello.
pub(crate) static KX_GROUPS: &[&dyn SupportedKxGroup] = &[X25519MLKEM768, &X25519, &SecP256R1, &SecP384R1];

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

macro_rules! nist_kx {
    ($group:ident, $active:ident, $named:expr, $secret:ty, $public:ty, $public_key_len:expr) => {
        #[derive(Debug)]
        pub(crate) struct $group;

        impl SupportedKxGroup for $group {
            fn start(&self) -> Result<Box<dyn ActiveKeyExchange>, Error> {
                let secret = <$secret>::generate().map_err(|_| RANDOM_FAILURE)?;
                let pub_key = secret.public_key().to_bytes();
                Ok(Box::new($active {
                    secret,
                    pub_key,
                }))
            }

            fn ffdhe_group(&self) -> Option<FfdheGroup<'static>> {
                None
            }

            fn name(&self) -> NamedGroup {
                $named
            }
        }

        struct $active {
            secret: $secret,
            pub_key: [u8; $public_key_len],
        }

        impl ActiveKeyExchange for $active {
            fn complete(self: Box<Self>, peer_pub_key: &[u8]) -> Result<SharedSecret, Error> {
                let peer = <$public>::from_bytes(peer_pub_key).map_err(|_| INVALID_KEY_SHARE)?;
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
                $named
            }
        }
    };
}

nist_kx!(
    SecP256R1,
    ActiveP256,
    NamedGroup::secp256r1,
    crypto::p256::SecretKey,
    crypto::p256::PublicKey,
    65
);
nist_kx!(
    SecP384R1,
    ActiveP384,
    NamedGroup::secp384r1,
    crypto::p384::SecretKey,
    crypto::p384::PublicKey,
    97
);

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
        let public = PublicKey768::from_bytes(&client_share);
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
pub(crate) static X25519MLKEM768: &dyn SupportedKxGroup = &hybrid::Hybrid {
    classical: &X25519,
    post_quantum: &MlKem768,
    name: NamedGroup::X25519MLKEM768,
    layout: hybrid::Layout {
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
        exercise(&SecP256R1);
        assert_eq!(SecP256R1.start().unwrap().pub_key().len(), 65);
    }

    #[test]
    fn secp384r1_round_trip() {
        exercise(&SecP384R1);
        assert_eq!(SecP384R1.start().unwrap().pub_key().len(), 97);
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
    fn groups_are_tls13_only_for_hybrid() {
        assert!(X25519MLKEM768.usable_for_version(ProtocolVersion::TLSv1_3));
        assert!(!X25519MLKEM768.usable_for_version(ProtocolVersion::TLSv1_2));
        assert!(X25519.usable_for_version(ProtocolVersion::TLSv1_2));
    }
}
