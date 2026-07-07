//! X-Wing: hybrid post-quantum key encapsulation mechanism (KEM) algorithm (ML-KEM-768 with X25519).

use crate::{
    curve25519::x25519,
    mlkem::{self, MlKemError},
    sha3::{Sha3_256, Shake256},
};

/// Size of the X-Wing secret key (32 bytes).
pub const SECRET_KEY_SIZE: usize = 32;
/// Size of the X-Wing public key in bytes (ML-KEM-768 pk + X25519 pk = 1216).
pub const PUBLIC_KEY_SIZE: usize = mlkem::PUBLIC_KEY_SIZE_768 + x25519::KEY_SIZE; // 1216
/// Size of the X-Wing ciphertext in bytes (ML-KEM-768 ct + X25519 shared secret = 1120).
pub const CIPHERTEXT_SIZE: usize = mlkem::CIPHERTEXT_SIZE_768 + x25519::SHARED_SECRET_SIZE; // 1120
/// Size of the X-Wing shared secret (32 bytes).
pub const SHARED_SECRET_SIZE: usize = 32;

const XWING_LABEL: &[u8; 6] = b"\\.//^\\";

/// X-Wing error type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XWingError {
    MlKem(MlKemError),
}

impl From<MlKemError> for XWingError {
    fn from(err: MlKemError) -> Self {
        XWingError::MlKem(err)
    }
}

impl core::fmt::Display for XWingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            XWingError::MlKem(err) => write!(f, "ML-KEM error: {err}"),
        }
    }
}

/// X-Wing hybrid KEM decapsulation (secret) key.
///
/// Combines an ML-KEM-768 and an X25519 secret key as specified in the
/// X-Wing draft. The shared secret is derived via a combiner that hashes
/// both component secrets together.
///
/// # Example
///
/// ```ignore
/// use crypto::xwing::{generate_keypair, SecretKey, PublicKey};
///
/// let (secret_key, public_key) = generate_keypair();
/// let (shared_secret, ciphertext) = public_key.encapsulate();
/// let decapsulated = secret_key.decapsulate(&ciphertext).unwrap();
/// assert_eq!(shared_secret, decapsulated);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SecretKey {
    bytes: [u8; SECRET_KEY_SIZE],
    x25519_secret_key: x25519::SecretKey,
    x25519_public_key_bytes: [u8; x25519::KEY_SIZE],
    mlkem_secret_key: mlkem::SecretKey768,
}

impl SecretKey {
    pub fn to_bytes(&self) -> [u8; SECRET_KEY_SIZE] {
        self.bytes
    }

    pub fn decapsulate(&self, ct: &[u8; CIPHERTEXT_SIZE]) -> Result<[u8; SHARED_SECRET_SIZE], XWingError> {
        let ct_m = &ct[..mlkem::CIPHERTEXT_SIZE_768].try_into().unwrap();
        let ct_x = x25519::PublicKey::from_bytes(&ct[mlkem::CIPHERTEXT_SIZE_768..].try_into().unwrap());

        let ss_m = self.mlkem_secret_key.decapsulate(&ct_m)?;
        let ss_x = self.x25519_secret_key.ecdh(&ct_x);

        Ok(combiner(&ss_m, &ss_x, &ct_x.to_bytes(), &self.x25519_public_key_bytes))
    }
}

/// X-Wing hybrid KEM encapsulation (public) key.
///
/// See [`SecretKey`] for a full usage example.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicKey {
    mlkem_public_key: mlkem::PublicKey768,
    x25519_public_key: x25519::PublicKey,
}

impl PublicKey {
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_SIZE] {
        let mut bytes = [0u8; PUBLIC_KEY_SIZE];
        bytes[..mlkem::PUBLIC_KEY_SIZE_768].copy_from_slice(&self.mlkem_public_key.to_bytes());
        bytes[mlkem::PUBLIC_KEY_SIZE_768..].copy_from_slice(&self.x25519_public_key.to_bytes());
        bytes
    }

    #[cfg(feature = "random")]
    pub fn encapsulate(&self) -> ([u8; SHARED_SECRET_SIZE], [u8; CIPHERTEXT_SIZE]) {
        let eseed: [u8; 64] = crate::random::random_bytes();
        self.encapsulate_derand(&eseed)
    }

    fn encapsulate_derand(&self, eseed: &[u8; 64]) -> ([u8; SHARED_SECRET_SIZE], [u8; CIPHERTEXT_SIZE]) {
        let ek_x = x25519::SecretKey::from_bytes(&eseed[32..64].try_into().unwrap());
        let ct_x = ek_x.public_key();
        let ss_x = ek_x.ecdh(&self.x25519_public_key);

        let m = &eseed[..32].try_into().unwrap();
        let (ct_m, ss_m) = self.mlkem_public_key.encapsulate_derand(&m);

        let ss = combiner(&ss_m, &ss_x, &ct_x.to_bytes(), &self.x25519_public_key.to_bytes());

        let mut ct = [0u8; CIPHERTEXT_SIZE];
        ct[..mlkem::CIPHERTEXT_SIZE_768].copy_from_slice(&ct_m);
        ct[mlkem::CIPHERTEXT_SIZE_768..].copy_from_slice(&ct_x.to_bytes());

        (ss, ct)
    }
}

/// Generate an X-Wing keypair.
///
/// This is a convenience wrapper around [`SecretKey`] generation.
///
/// See [`SecretKey`] for a usage example.
#[cfg(feature = "random")]
pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let seed: [u8; SECRET_KEY_SIZE] = crate::random::random_bytes();
    generate_keypair_derand(&seed)
}

/// Generate a deterministic keypair from the given seed (for testing).
fn generate_keypair_derand(secret_key: &[u8; SECRET_KEY_SIZE]) -> (SecretKey, PublicKey) {
    let (mlkem_sk, x25519_sk, mlkem_pk, x25519_pk) = expand_decapsulation_key(secret_key);

    let secret_key = SecretKey {
        bytes: *secret_key,
        x25519_secret_key: x25519_sk,
        x25519_public_key_bytes: x25519_pk.to_bytes(),
        mlkem_secret_key: mlkem_sk,
    };

    let public_key = PublicKey {
        mlkem_public_key: mlkem_pk,
        x25519_public_key: x25519_pk,
    };

    (secret_key, public_key)
}

fn expand_decapsulation_key(
    secret_key: &[u8; 32],
) -> (mlkem::SecretKey768, x25519::SecretKey, mlkem::PublicKey768, x25519::PublicKey) {
    let mut expanded_secret_key = [0u8; 96];
    Shake256::hash(secret_key, &mut expanded_secret_key);

    let (sk_m, pk_m) = derive_mlkeem_keys(&expanded_secret_key);

    let sk_x = x25519::SecretKey::from_bytes(&expanded_secret_key[64..96].try_into().unwrap());
    let pk_x = sk_x.public_key();

    (sk_m, sk_x, pk_m, pk_x)
}

fn derive_mlkeem_keys(expnded_secret_key: &[u8; 96]) -> (mlkem::SecretKey768, mlkem::PublicKey768) {
    mlkem::generate_keypair_768_derand(&expnded_secret_key[..64].try_into().unwrap())
}

fn combiner(
    ss_m: &[u8; mlkem::SHARED_SECRET_SIZE],
    ss_x: &[u8; x25519::KEY_SIZE],
    ct_x: &[u8; x25519::KEY_SIZE],
    pk_x: &[u8; x25519::KEY_SIZE],
) -> [u8; SHARED_SECRET_SIZE] {
    use crate::Hasher;
    let mut hasher = Sha3_256::new();
    hasher.update(ss_m);
    hasher.update(ss_x);
    hasher.update(ct_x);
    hasher.update(pk_x);
    hasher.update(XWING_LABEL);
    hasher.sum().as_ref().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex_to_array<const N: usize>(hex_str: &str) -> [u8; N] {
        let bytes = hex::decode(hex_str).unwrap();
        return bytes.try_into().unwrap();
    }

    #[test]
    fn constants() {
        assert!(PUBLIC_KEY_SIZE == 1216);
        assert!(CIPHERTEXT_SIZE == 1120);
    }

    struct TestVector {
        seed: &'static str,
        eseed: &'static str,
        ss: &'static str,
    }

    const TEST_VECTORS: [TestVector; 3] = [
        TestVector {
            seed: "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26",
            eseed: "3cb1eea988004b93103cfb0aeefd2a686e01fa4a58e8a3639ca8a1e3f9ae57e235b8cc873c23dc62b8d260169afa2f75ab916a58d974918835d25e6a435085b2",
            ss: "d2df0522128f09dd8e2c92b1e905c793d8f57a54c3da25861f10bf4ca613e384",
        },
        TestVector {
            seed: "badfd6dfaac359a5efbb7bcc4b59d538df9a04302e10c8bc1cbf1a0b3a5120ea",
            eseed: "17cda7cfad765f5623474d368ccca8af0007cd9f5e4c849f167a580b14aabdefaee7eef47cb0fca9767be1fda69419dfb927e9df07348b196691abaeb580b32d",
            ss: "f2e86241c64d60f6649fbc6c5b7d17180b780a3f34355e64a85749949c45f150",
        },
        TestVector {
            seed: "ef58538b8d23f87732ea63b02b4fa0f4873360e2841928cd60dd4cee8cc0d4c9",
            eseed: "22a96188d032675c8ac850933c7aff1533b94c834adbb69c6115bad4692d8619f90b0cdf8a7b9c264029ac185b70b83f2801f2f4b3f70c593ea3aeeb613a7f1b",
            ss: "953f7f4e8c5b5049bdc771d1dffada0dd961477d1a2ae0988baa7ea6898d893f",
        },
    ];

    #[test]
    fn test_vectors_from_draft() {
        for (i, tv) in TEST_VECTORS.iter().enumerate() {
            let seed: [u8; 32] = hex_to_array(tv.seed);
            let eseed: [u8; 64] = hex_to_array(tv.eseed);
            let expected_ss: [u8; 32] = hex_to_array(tv.ss);

            let (secret_key, pk) = generate_keypair_derand(&seed);
            assert_eq!(secret_key.to_bytes(), seed, "vector {i}: sk mismatch");

            let (ss, ct) = pk.encapsulate_derand(&eseed);
            assert_eq!(ss, expected_ss, "vector {i}: encaps ss mismatch");

            let decapsulated_ss = secret_key.decapsulate(&ct).unwrap();
            assert_eq!(decapsulated_ss, expected_ss, "vector {i}: decaps ss mismatch");
        }
    }

    #[test]
    fn round_trip() {
        let (secret_key, public_key) = generate_keypair();
        let (ss, ct) = public_key.encapsulate();
        let decapsulated = secret_key.decapsulate(&ct).unwrap();
        assert_eq!(ss, decapsulated);
    }

    #[test]
    fn round_trip_many() {
        for _ in 0..10 {
            let (secret_key, public_key) = generate_keypair();
            let (ss, ct) = public_key.encapsulate();
            let decapsulated = secret_key.decapsulate(&ct).unwrap();
            assert_eq!(ss, decapsulated);
        }
    }

    #[test]
    fn decapsulation_with_wrong_key_produces_different_secret() {
        let (_, pk_a) = generate_keypair();
        let (sk_b, _) = generate_keypair();

        let (ss_a, ct) = pk_a.encapsulate();
        let ss_b = sk_b.decapsulate(&ct).unwrap();
        assert_ne!(ss_a, ss_b);
    }

    #[test]
    fn tampered_ciphertext_produces_different_secret() {
        let (secret_key, public_key) = generate_keypair();
        let (ss, mut ct) = public_key.encapsulate();

        ct[0] ^= 0x80;

        let tampered_ss = secret_key.decapsulate(&ct).unwrap();
        assert_ne!(ss, tampered_ss);
    }

    #[test]
    fn derandomized_keygen_is_deterministic() {
        let seed: [u8; 32] = hex_to_array("7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26");
        let (sk1, pk1) = generate_keypair_derand(&seed);
        let (sk2, pk2) = generate_keypair_derand(&seed);
        assert_eq!(sk1.to_bytes(), sk2.to_bytes());
        assert_eq!(pk1.to_bytes(), pk2.to_bytes());
    }

    #[test]
    fn derandomized_encaps_is_deterministic() {
        let seed: [u8; 32] = hex_to_array("7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26");
        let eseed: [u8; 64] = hex_to_array(
            "3cb1eea988004b93103cfb0aeefd2a686e01fa4a58e8a3639ca8a1e3f9ae57e235b8cc873c23dc62b8d260169afa2f75ab916a58d974918835d25e6a435085b2",
        );
        let (_, pk) = generate_keypair_derand(&seed);

        let (ss1, ct1) = pk.encapsulate_derand(&eseed);
        let (ss2, ct2) = pk.encapsulate_derand(&eseed);
        assert_eq!(ct1, ct2);
        assert_eq!(ss1, ss2);
    }

    #[test]
    fn xwing_label_is_correct() {
        assert_eq!(XWING_LABEL.len(), 6);
        assert_eq!(hex::encode(XWING_LABEL), "5c2e2f2f5e5c");
    }

    #[test]
    fn expand_decapsulation_key_is_deterministic() {
        let seed: [u8; 32] = hex_to_array("7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26");

        let (sk_m1, sk_x1, pk_m1, pk_x1) = expand_decapsulation_key(&seed);
        let (sk_m2, sk_x2, pk_m2, pk_x2) = expand_decapsulation_key(&seed);
        assert_eq!(sk_m1, sk_m2);
        assert_eq!(sk_x1, sk_x2);
        assert_eq!(pk_m1, pk_m2);
        assert_eq!(pk_x1, pk_x2);
    }

    #[test]
    fn combiner_is_deterministic() {
        let ss_m = [0x01u8; 32];
        let ss_x = [0x02u8; 32];
        let ct_x = [0x03u8; 32];
        let pk_x = [0x04u8; 32];

        let result1 = combiner(&ss_m, &ss_x, &ct_x, &pk_x);
        let result2 = combiner(&ss_m, &ss_x, &ct_x, &pk_x);
        assert_eq!(result1, result2);
    }
}
