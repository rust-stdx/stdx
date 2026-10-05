//! Hybrid key exchange support.
//!
//! A [`Hybrid`] combines a classical (elliptic-curve) key exchange with a
//! post-quantum key encapsulation mechanism. The two component key shares are
//! concatenated into a single TLS key share, and the two shared secrets are
//! concatenated (in the order dictated by [`Layout`]) into the input keying
//! material for the TLS 1.3 key schedule.
//!
//! This mirrors the construction standardized for `X25519MLKEM768` in
//! draft-ietf-tls-ecdhe-mlkem.

use rustls::{
    Error, NamedGroup, PeerMisbehaved, ProtocolVersion,
    crypto::{ActiveKeyExchange, CompletedKeyExchange, SharedSecret, SupportedKxGroup},
    ffdhe_groups::FfdheGroup,
};

const INVALID_KEY_SHARE: Error = Error::PeerMisbehaved(PeerMisbehaved::InvalidKeyShare);

/// A hybrid key exchange composed of a classical and a post-quantum group.
#[derive(Debug)]
pub(crate) struct Hybrid {
    /// The classical (elliptic-curve) component.
    pub(crate) classical: &'static dyn SupportedKxGroup,
    /// The post-quantum key-encapsulation component.
    pub(crate) post_quantum: &'static dyn SupportedKxGroup,
    /// The TLS `NamedGroup` for the combined exchange.
    pub(crate) name: NamedGroup,
    /// How the component shares and secrets are laid out.
    pub(crate) layout: Layout,
}

impl SupportedKxGroup for Hybrid {
    fn start(&self) -> Result<Box<dyn ActiveKeyExchange>, Error> {
        let classical = self.classical.start()?;
        let post_quantum = self.post_quantum.start()?;

        let combined_pub_key = self.layout.concat(post_quantum.pub_key(), classical.pub_key());

        Ok(Box::new(ActiveHybrid {
            classical,
            post_quantum,
            name: self.name,
            layout: self.layout,
            combined_pub_key,
        }))
    }

    fn start_and_complete(&self, client_share: &[u8]) -> Result<CompletedKeyExchange, Error> {
        let (post_quantum_share, classical_share) = self
            .layout
            .split_received_client_share(client_share)
            .ok_or(INVALID_KEY_SHARE)?;

        let classical = self.classical.start_and_complete(classical_share)?;
        let post_quantum = self.post_quantum.start_and_complete(post_quantum_share)?;

        let combined_pub_key = self.layout.concat(&post_quantum.pub_key, &classical.pub_key);
        let secret = self
            .layout
            .concat(post_quantum.secret.secret_bytes(), classical.secret.secret_bytes());

        Ok(CompletedKeyExchange {
            group: self.name,
            pub_key: combined_pub_key,
            secret: SharedSecret::from(&secret[..]),
        })
    }

    fn ffdhe_group(&self) -> Option<FfdheGroup<'static>> {
        None
    }

    fn name(&self) -> NamedGroup {
        self.name
    }

    fn usable_for_version(&self, version: ProtocolVersion) -> bool {
        version == ProtocolVersion::TLSv1_3
    }
}

struct ActiveHybrid {
    classical: Box<dyn ActiveKeyExchange>,
    post_quantum: Box<dyn ActiveKeyExchange>,
    name: NamedGroup,
    layout: Layout,
    combined_pub_key: Vec<u8>,
}

impl ActiveKeyExchange for ActiveHybrid {
    fn complete(self: Box<Self>, peer_pub_key: &[u8]) -> Result<SharedSecret, Error> {
        let Self {
            classical,
            post_quantum,
            layout,
            ..
        } = *self;

        let (post_quantum_share, classical_share) = layout
            .split_received_server_share(peer_pub_key)
            .ok_or(INVALID_KEY_SHARE)?;

        let classical = classical.complete(classical_share)?;
        let post_quantum = post_quantum.complete(post_quantum_share)?;

        let secret = layout.concat(post_quantum.secret_bytes(), classical.secret_bytes());
        Ok(SharedSecret::from(&secret[..]))
    }

    fn hybrid_component(&self) -> Option<(NamedGroup, &[u8])> {
        Some((self.classical.group(), self.classical.pub_key()))
    }

    fn complete_hybrid_component(self: Box<Self>, peer_pub_key: &[u8]) -> Result<SharedSecret, Error> {
        self.classical.complete(peer_pub_key)
    }

    fn pub_key(&self) -> &[u8] {
        &self.combined_pub_key
    }

    fn ffdhe_group(&self) -> Option<FfdheGroup<'static>> {
        None
    }

    fn group(&self) -> NamedGroup {
        self.name
    }
}

/// Layout of a hybrid key share and shared secret.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Layout {
    /// Length of the classical component's key share.
    pub(crate) classical_share_len: usize,
    /// Length of the post-quantum component's share sent by the client
    /// (an encapsulation key).
    pub(crate) post_quantum_client_share_len: usize,
    /// Length of the post-quantum component's share sent by the server
    /// (a ciphertext).
    pub(crate) post_quantum_server_share_len: usize,
    /// Whether the post-quantum component comes first in shares and secrets.
    ///
    /// `X25519MLKEM768` places the post-quantum component first;
    /// `SECP256R1MLKEM768` places the classical component first.
    pub(crate) post_quantum_first: bool,
}

impl Layout {
    fn split_received_client_share<'a>(&self, share: &'a [u8]) -> Option<(&'a [u8], &'a [u8])> {
        self.split(share, self.post_quantum_client_share_len)
    }

    fn split_received_server_share<'a>(&self, share: &'a [u8]) -> Option<(&'a [u8], &'a [u8])> {
        self.split(share, self.post_quantum_server_share_len)
    }

    /// Splits a key share into its post-quantum and classical components.
    fn split<'a>(&self, share: &'a [u8], post_quantum_len: usize) -> Option<(&'a [u8], &'a [u8])> {
        if share.len() != self.classical_share_len + post_quantum_len {
            return None;
        }

        Some(match self.post_quantum_first {
            true => {
                let (post_quantum, classical) = share.split_at(post_quantum_len);
                (post_quantum, classical)
            }
            false => {
                let (classical, post_quantum) = share.split_at(self.classical_share_len);
                (post_quantum, classical)
            }
        })
    }

    fn concat(&self, post_quantum: &[u8], classical: &[u8]) -> Vec<u8> {
        match self.post_quantum_first {
            true => [post_quantum, classical].concat(),
            false => [classical, post_quantum].concat(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_post_quantum_first() {
        let layout = Layout {
            classical_share_len: 2,
            post_quantum_client_share_len: 3,
            post_quantum_server_share_len: 4,
            post_quantum_first: true,
        };
        let share = [1, 2, 3, 4, 5];
        let (pq, cl) = layout.split(&share, 3).unwrap();
        assert_eq!(pq, &[1, 2, 3]);
        assert_eq!(cl, &[4, 5]);
        assert_eq!(layout.concat(pq, cl), share);
    }

    #[test]
    fn layout_classical_first() {
        let layout = Layout {
            classical_share_len: 2,
            post_quantum_client_share_len: 3,
            post_quantum_server_share_len: 4,
            post_quantum_first: false,
        };
        let share = [1, 2, 3, 4, 5];
        let (pq, cl) = layout.split(&share, 3).unwrap();
        assert_eq!(cl, &[1, 2]);
        assert_eq!(pq, &[3, 4, 5]);
        assert_eq!(layout.concat(pq, cl), share);
    }

    #[test]
    fn layout_rejects_wrong_length() {
        let layout = Layout {
            classical_share_len: 2,
            post_quantum_client_share_len: 3,
            post_quantum_server_share_len: 4,
            post_quantum_first: true,
        };
        assert!(layout.split(&[0u8; 4], 3).is_none());
        assert!(layout.split(&[0u8; 6], 3).is_none());
    }
}
