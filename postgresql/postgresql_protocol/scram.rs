//! SCRAM-SHA-256 client (RFC 5802 / RFC 7677), used for PostgreSQL
//! password authentication.
//!
//! Channel binding (`SCRAM-SHA-256-PLUS`) is not implemented; the GS2 header
//! advertises no channel binding (`n,,`).
//!
//! The server is unauthenticated when `sslmode` is `require`, so every value
//! taken from the server is treated as hostile: iteration counts and salt
//! lengths are bounded and the nonce is validated before any key derivation
//! runs.
//!
//! The password is prepared with SASLprep (RFC 4013) before key derivation, so
//! it matches the form PostgreSQL stored the verifier for. See [`saslprep`]
//! for the implemented profile.

use base64::{Alphabet, decode as b64_decode, encode as b64_encode};
use crypto::{Hasher, hmac::Hmac, random, sha2::Sha256};

use crate::error::Error;

/// Default highest accepted PBKDF2 iteration count.
///
/// The server chooses this value and it is pure CPU work on the client, so an
/// unbounded count is a denial of service (a `u32::MAX` count would keep a
/// runtime thread busy for minutes). PostgreSQL itself uses 4096 by default;
/// 1,000,000 is ~244x that. It can be overridden per connection with
/// `Config::max_scram_iterations` / the `max_scram_iterations` URL parameter.
///
/// See also [`MAX_SALT_LEN`] and [`MAX_SERVER_FIRST_LEN`].
pub const DEFAULT_MAX_SCRAM_ITERATIONS: u32 = 1_000_000;

/// Highest accepted salt length, in bytes.
pub const MAX_SALT_LEN: usize = 1024;

/// Highest accepted length for a server-first or server-final message.
pub const MAX_SERVER_FIRST_LEN: usize = 8 * 1024;

fn hmac_sha256(key: &[u8], data: &[u8]) -> crypto::Hash {
    Hmac::<Sha256>::mac(key, data)
}

/// Applies SASLprep (RFC 4013) to a password before SCRAM key derivation.
///
/// PostgreSQL computes the stored SCRAM verifier from the SASLprep'd password,
/// so the client must prepare the password the same way or authentication fails
/// for any password SASLprep changes (for example one containing a non-ASCII
/// space, which `libpq` and every other client normalise).
///
/// This implements the mapping and prohibition steps: non-ASCII space
/// characters become ASCII spaces, the "commonly mapped to nothing" characters
/// are removed, and ASCII control characters are rejected. Full Unicode NFKC
/// normalisation and bidirectional checks are **not** implemented, so a password
/// whose meaning depends on them is unsupported (ASCII and ordinary passwords
/// are unaffected, and are returned unchanged).
fn saslprep(password: &str) -> std::result::Result<String, Error> {
    let mut out = String::with_capacity(password.len());
    for c in password.chars() {
        match c {
            // Prohibited ASCII control characters (RFC 3454, C.2.1).
            '\u{0000}'..='\u{001F}' | '\u{007F}' => {
                return Err(Error::Auth(
                    "the password contains a control character that SASLprep prohibits".into(),
                ));
            }
            // Non-ASCII space characters (Zs) map to ASCII space (C.1.2).
            '\u{00A0}' | '\u{1680}' | '\u{2000}'..='\u{200A}' | '\u{202F}' | '\u{205F}' | '\u{3000}' => {
                out.push(' ');
            }
            // "Commonly mapped to nothing" (B.1).
            '\u{00AD}'
            | '\u{034F}'
            | '\u{1806}'
            | '\u{180B}'..='\u{180D}'
            | '\u{200B}'..='\u{200D}'
            | '\u{2060}'
            | '\u{FE00}'..='\u{FE0F}'
            | '\u{FEFF}' => {}
            other => out.push(other),
        }
    }
    Ok(out)
}

/// `Hi()` from RFC 5802: PBKDF2-like salted password derivation.
///
/// # Errors
///
/// Returns [`Error::Auth`] when `iterations` is outside `1..=max_iterations`
/// or the salt is longer than [`MAX_SALT_LEN`].
fn hi(password: &[u8], salt: &[u8], iterations: u32, max_iterations: u32) -> std::result::Result<[u8; 32], Error> {
    if !(1..=max_iterations).contains(&iterations) {
        return Err(Error::Auth(format!(
            "server sent an unacceptable SCRAM iteration count ({iterations}, accepted range is 1..={max_iterations})"
        )));
    }
    if salt.len() > MAX_SALT_LEN {
        return Err(Error::Auth(format!(
            "server sent an oversized SCRAM salt ({} bytes)",
            salt.len()
        )));
    }
    let mut salt_input = Vec::with_capacity(salt.len() + 4);
    salt_input.extend_from_slice(salt);
    salt_input.extend_from_slice(&1u32.to_be_bytes());

    let mut u = hmac_sha256(password, &salt_input);
    let mut result = [0u8; 32];
    result.copy_from_slice(u.as_ref());

    for _ in 1..iterations {
        u = hmac_sha256(password, u.as_ref());
        for (a, b) in result.iter_mut().zip(u.as_ref()) {
            *a ^= b;
        }
    }
    Ok(result)
}

/// A SCRAM-SHA-256 exchange in progress.
pub struct ScramClient {
    password: String,
    client_nonce: String,
    client_first_bare: String,
    server_first: Option<String>,
    client_final_without_proof: Option<String>,
    salted_password: Option<[u8; 32]>,
    max_iterations: u32,
}

impl ScramClient {
    /// Starts a new exchange. `password` is the cleartext password.
    ///
    /// The startup message already carries the username, so it is not repeated
    /// in the SCRAM message (matching other PostgreSQL clients).
    pub fn new(password: &str) -> std::result::Result<Self, Error> {
        Self::with_max_iterations(password, DEFAULT_MAX_SCRAM_ITERATIONS)
    }

    /// Like [`ScramClient::new`], but caps the server-supplied PBKDF2
    /// iteration count at `max_iterations` instead of the default.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Auth`] when the password cannot be SASLprep'd (it
    /// contains a prohibited control character).
    pub fn with_max_iterations(password: &str, max_iterations: u32) -> std::result::Result<Self, Error> {
        let raw = random::bytes::<18>().map_err(|e| Error::Auth(format!("random: {e}")))?;
        let client_nonce = b64_encode(raw, Alphabet::StandardNoPadding);
        Self::with_nonce_and_max("", password, client_nonce, max_iterations)
    }

    #[cfg(test)]
    fn with_nonce(username: &str, password: &str, client_nonce: String) -> Self {
        Self::with_nonce_and_max(username, password, client_nonce, DEFAULT_MAX_SCRAM_ITERATIONS)
            .expect("test password is valid")
    }

    fn with_nonce_and_max(
        username: &str,
        password: &str,
        client_nonce: String,
        max_iterations: u32,
    ) -> std::result::Result<Self, Error> {
        let password = saslprep(password)?;
        let client_first_bare = format!("n={username},r={client_nonce}");
        Ok(ScramClient {
            password,
            client_nonce,
            client_first_bare,
            server_first: None,
            client_final_without_proof: None,
            salted_password: None,
            max_iterations,
        })
    }

    /// The full initial response: GS2 header `n,,` followed by the
    /// client-first-message-bare.
    pub fn client_first_message(&self) -> String {
        format!("n,,{}", self.client_first_bare)
    }

    /// Parses the server-first-message and derives the salted password.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Auth`] when the message is malformed, when the server
    /// nonce does not extend the client nonce, when the iteration count is
    /// outside `1..=MAX_SCRAM_ITERATIONS`, or when the salt is oversized.
    pub fn parse_server_first_message(&mut self, data: &[u8]) -> std::result::Result<(), Error> {
        if data.len() > MAX_SERVER_FIRST_LEN {
            return Err(Error::Auth("server-first message is too long".into()));
        }
        let msg = std::str::from_utf8(data).map_err(|_| Error::Auth("invalid UTF-8 in server-first".into()))?;
        self.server_first = Some(msg.to_string());

        let mut combined_nonce: Option<&str> = None;
        let mut salt_b64: Option<&str> = None;
        let mut iterations = None;

        for part in msg.split(',') {
            if let Some(v) = part.strip_prefix("r=") {
                if combined_nonce.replace(v).is_some() {
                    return Err(Error::Auth("duplicate nonce attribute".into()));
                }
            } else if let Some(v) = part.strip_prefix("s=") {
                if salt_b64.replace(v).is_some() {
                    return Err(Error::Auth("duplicate salt attribute".into()));
                }
            } else if let Some(v) = part.strip_prefix("i=") {
                if iterations.is_some() {
                    return Err(Error::Auth("duplicate iteration attribute".into()));
                }
                iterations = Some(
                    v.parse::<u32>()
                        .map_err(|_| Error::Auth("invalid iteration count".into()))?,
                );
            }
        }

        let combined_nonce = combined_nonce.ok_or_else(|| Error::Auth("missing nonce".into()))?;
        let salt_b64 = salt_b64.ok_or_else(|| Error::Auth("missing salt".into()))?;
        let iterations = iterations.ok_or_else(|| Error::Auth("missing iterations".into()))?;

        // The server appends its own nonce; echoing ours back unchanged is not
        // a valid challenge and would let a proxy replay the exchange.
        if !combined_nonce.starts_with(&self.client_nonce) || combined_nonce.len() <= self.client_nonce.len() {
            return Err(Error::Auth("server nonce does not extend the client nonce".into()));
        }

        let salt = b64_decode(salt_b64, Alphabet::Standard).map_err(|e| Error::Auth(format!("invalid salt: {e}")))?;

        self.salted_password = Some(hi(self.password.as_bytes(), &salt, iterations, self.max_iterations)?);
        self.client_final_without_proof = Some(format!("c=biws,r={combined_nonce}"));
        Ok(())
    }

    fn auth_message(&self) -> std::result::Result<String, Error> {
        let server_first = self
            .server_first
            .as_deref()
            .ok_or_else(|| Error::Auth("server-first message has not been parsed".into()))?;
        let client_final = self
            .client_final_without_proof
            .as_deref()
            .ok_or_else(|| Error::Auth("server-first message has not been parsed".into()))?;
        Ok(format!("{},{},{}", self.client_first_bare, server_first, client_final))
    }

    /// Builds the client-final-message.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Auth`] when called before
    /// [`parse_server_first_message`](Self::parse_server_first_message).
    pub fn build_client_final_message(&self) -> std::result::Result<Vec<u8>, Error> {
        let sp = self
            .salted_password
            .as_ref()
            .ok_or_else(|| Error::Auth("server-first message has not been parsed".into()))?;
        let client_key = hmac_sha256(sp, b"Client Key");
        let stored_key = Sha256::hash(client_key.as_ref());
        let client_signature = hmac_sha256(stored_key.as_ref(), self.auth_message()?.as_bytes());

        let mut proof = [0u8; 32];
        proof.copy_from_slice(client_key.as_ref());
        for (a, b) in proof.iter_mut().zip(client_signature.as_ref()) {
            *a ^= b;
        }

        Ok(format!(
            "{},p={}",
            self.client_final_without_proof
                .as_deref()
                .ok_or_else(|| Error::Auth("server-first message has not been parsed".into()))?,
            b64_encode(proof, Alphabet::Standard)
        )
        .into_bytes())
    }

    /// Verifies the server-final-message (`v=`) against the expected signature.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Auth`] when the server rejects the exchange (`e=`),
    /// when the signature is missing or malformed, or when it does not match
    /// the expected value (a proxy that does not know the password).
    pub fn parse_server_final_message(&self, data: &[u8]) -> std::result::Result<(), Error> {
        if data.len() > MAX_SERVER_FIRST_LEN {
            return Err(Error::Auth("server-final message is too long".into()));
        }
        let msg = std::str::from_utf8(data).map_err(|_| Error::Auth("invalid UTF-8 in server-final".into()))?;

        let mut server_sig_b64 = None;
        for part in msg.split(',') {
            if let Some(v) = part.strip_prefix("v=") {
                if server_sig_b64.replace(v).is_some() {
                    return Err(Error::Auth("duplicate server signature".into()));
                }
            } else if let Some(e) = part.strip_prefix("e=") {
                return Err(Error::Auth(format!("server rejected authentication: {e}")));
            }
        }

        let server_sig_b64 = server_sig_b64.ok_or_else(|| Error::Auth("missing server signature".into()))?;
        let server_sig = b64_decode(server_sig_b64, Alphabet::Standard)
            .map_err(|_| Error::Auth("server signature is not base64".into()))?;

        let sp = self
            .salted_password
            .as_ref()
            .ok_or_else(|| Error::Auth("server-first message has not been parsed".into()))?;
        let server_key = hmac_sha256(sp, b"Server Key");
        let expected = hmac_sha256(server_key.as_ref(), self.auth_message()?.as_bytes());

        // Constant-time: the signature covers the password-derived server key.
        if !constant_time_eq::constant_time_eq(expected.as_ref(), &server_sig) {
            return Err(Error::Auth("server signature mismatch".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // RFC 7677 section 3 test vector (SHA-256).
    #[test]
    fn rfc7677_vector() {
        let mut client = ScramClient::with_nonce("user", "pencil", "rOprNGfwEbeRWgbNEkqO".to_string());
        assert_eq!(client.client_first_message(), "n,,n=user,r=rOprNGfwEbeRWgbNEkqO");

        client
            .parse_server_first_message(
                b"r=rOprNGfwEbeRWgbNEkqO%hvYDpWUa2RaTCAfuxFIlj)hNlF$k0,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096",
            )
            .unwrap();

        let final_msg = client.build_client_final_message().unwrap();
        assert_eq!(
            String::from_utf8(final_msg).unwrap(),
            "c=biws,r=rOprNGfwEbeRWgbNEkqO%hvYDpWUa2RaTCAfuxFIlj)hNlF$k0,p=dHzbZapWIk4jUhN+Ute9ytag9zjfMHgsqmmiz7AndVQ="
        );

        client
            .parse_server_final_message(b"v=6rriTRBi23WpRR/wtup+mMhUZUn/dB5nLTJRsjl95G4=")
            .unwrap();
    }

    #[test]
    fn rejects_wrong_nonce() {
        let mut client = ScramClient::with_nonce("user", "pencil", "abc".to_string());
        let err = client
            .parse_server_first_message(b"r=xyz,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096")
            .unwrap_err();
        assert!(matches!(err, Error::Auth(_)));
    }

    #[test]
    fn rejects_unextended_nonce() {
        // A server that echoes the client nonce unchanged is not a valid peer.
        let mut client = ScramClient::with_nonce("user", "pencil", "abc".to_string());
        let err = client
            .parse_server_first_message(b"r=abc,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096")
            .unwrap_err();
        assert!(matches!(err, Error::Auth(_)));
    }

    #[test]
    fn rejects_unbounded_iterations() {
        for i in ["0", "2000001", "4294967295"] {
            let mut client = ScramClient::with_nonce("user", "pencil", "abc".to_string());
            let msg = format!("r=abcd,s=W22ZaJ0SNY7soEsUEjb6gQ==,i={i}");
            let err = client.parse_server_first_message(msg.as_bytes()).unwrap_err();
            assert!(matches!(err, Error::Auth(_)), "i={i} must be rejected");
        }
    }

    #[test]
    fn accepts_bounded_iterations() {
        for i in ["1", "4096", "2000000"] {
            let mut client = ScramClient::with_nonce("user", "pencil", "abc".to_string());
            let msg = format!("r=abcd,s=W22ZaJ0SNY7soEsUEjb6gQ==,i={i}");
            client.parse_server_first_message(msg.as_bytes()).unwrap();
        }
    }

    #[test]
    fn custom_iteration_limit_is_honoured() {
        // A low configured limit rejects a count the default would accept.
        let mut client = ScramClient::with_nonce_and_max("user", "pencil", "abc".to_string(), 1000).unwrap();
        let msg = "r=abcd,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096";
        assert!(client.parse_server_first_message(msg.as_bytes()).is_err());

        let mut client = ScramClient::with_nonce_and_max("user", "pencil", "abc".to_string(), 4096).unwrap();
        client.parse_server_first_message(msg.as_bytes()).unwrap();
    }

    #[test]
    fn rejects_oversized_salt() {
        let salt = "A".repeat(MAX_SALT_LEN * 2);
        let mut client = ScramClient::with_nonce("user", "pencil", "abc".to_string());
        let msg = format!("r=abcd,s={salt},i=4096");
        assert!(client.parse_server_first_message(msg.as_bytes()).is_err());
    }

    #[test]
    fn rejects_oversized_messages() {
        let mut client = ScramClient::with_nonce("user", "pencil", "abc".to_string());
        let msg = format!("r=abcd,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096,{}", "x".repeat(MAX_SERVER_FIRST_LEN));
        assert!(client.parse_server_first_message(msg.as_bytes()).is_err());
        assert!(
            client
                .parse_server_final_message(&vec![b'v'; MAX_SERVER_FIRST_LEN + 1])
                .is_err()
        );
    }

    #[test]
    fn rejects_duplicate_attributes() {
        let mut client = ScramClient::with_nonce("user", "pencil", "abc".to_string());
        assert!(
            client
                .parse_server_first_message(b"r=abcd,r=abcd,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096")
                .is_err()
        );
    }

    #[test]
    fn out_of_order_calls_error_instead_of_panicking() {
        let client = ScramClient::with_nonce("user", "pencil", "abc".to_string());
        assert!(client.build_client_final_message().is_err());
        assert!(client.parse_server_final_message(b"v=whatever").is_err());
    }

    #[test]
    fn rejects_bad_server_signature() {
        let mut client = ScramClient::with_nonce("user", "pencil", "abc".to_string());
        client
            .parse_server_first_message(b"r=abcdef,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096")
            .unwrap();
        let _ = client.build_client_final_message().unwrap();
        assert!(
            client
                .parse_server_final_message(b"v=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=")
                .is_err()
        );
        assert!(client.parse_server_final_message(b"v=not base64 !!").is_err());
        assert!(client.parse_server_final_message(b"e=server-error").is_err());
    }

    #[test]
    fn saslprep_maps_and_prohibits() {
        // ASCII is unchanged.
        assert_eq!(saslprep("pencil").unwrap(), "pencil");
        // Non-ASCII spaces map to ASCII space
        assert_eq!(saslprep("a\u{00A0}b").unwrap(), "a b");
        assert_eq!(saslprep("a\u{3000}b").unwrap(), "a b");
        // "Commonly mapped to nothing" characters are removed.
        assert_eq!(saslprep("a\u{00AD}b").unwrap(), "ab");
        assert_eq!(saslprep("a\u{200B}b").unwrap(), "ab");
        // Prohibited ASCII control characters are rejected.
        for c in ['\u{0000}', '\u{001F}', '\u{007F}'] {
            assert!(saslprep(&format!("a{c}b")).is_err(), "control {c:?} must be rejected");
        }
    }

    #[test]
    fn passwords_are_saslprep_before_derivation() {
        // A password with a non-ASCII space must derive the same proof as its
        // SASLprep'd (ASCII space) form, which is what the server stored.
        let mut with_nbsp = ScramClient::with_nonce("user", "pi\u{00A0}ncil", "abc".to_string());
        let mut ascii = ScramClient::with_nonce("user", "pi ncil", "abc".to_string());
        let server_first = b"r=abcd,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096";
        with_nbsp.parse_server_first_message(server_first).unwrap();
        ascii.parse_server_first_message(server_first).unwrap();
        assert_eq!(
            with_nbsp.build_client_final_message().unwrap(),
            ascii.build_client_final_message().unwrap()
        );
    }
}
