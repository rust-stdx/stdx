use alloc::sync::Arc;

use heapless::Vec;

use crate::crypto::{CipherSuite, CryptoProvider, MAX_HASH_OUTPUT};

fn derive_secret(
    secret: &[u8],
    label: &[u8],
    transcript_hash: &[u8],
    provider: &Arc<dyn CryptoProvider>,
    suite: CipherSuite,
) -> Vec<u8, MAX_HASH_OUTPUT> {
    let hash_size = suite.hash_size();
    provider.hkdf_expand_label(suite, secret, label, transcript_hash, hash_size)
}

#[derive(Clone)]
pub struct TlsKeys {
    pub suite: CipherSuite,
    pub client_handshake_key: Vec<u8, MAX_HASH_OUTPUT>,
    pub client_handshake_iv: [u8; 12],
    pub server_handshake_key: Vec<u8, MAX_HASH_OUTPUT>,
    pub server_handshake_iv: [u8; 12],
    pub server_finished_key: Vec<u8, MAX_HASH_OUTPUT>,
    pub client_finished_key: Vec<u8, MAX_HASH_OUTPUT>,
    pub client_application_key: Vec<u8, MAX_HASH_OUTPUT>,
    pub client_application_iv: [u8; 12],
    pub server_application_key: Vec<u8, MAX_HASH_OUTPUT>,
    pub server_application_iv: [u8; 12],
    pub resumption_master_secret: Vec<u8, MAX_HASH_OUTPUT>,
    pub client_handshake_traffic_secret: Vec<u8, MAX_HASH_OUTPUT>,
    pub server_handshake_traffic_secret: Vec<u8, MAX_HASH_OUTPUT>,
    pub client_application_traffic_secret: Vec<u8, MAX_HASH_OUTPUT>,
    pub server_application_traffic_secret: Vec<u8, MAX_HASH_OUTPUT>,
    pub client_early_traffic_secret: Vec<u8, MAX_HASH_OUTPUT>,
    pub binder_key: Vec<u8, MAX_HASH_OUTPUT>,
    pub exporter_master_secret: Vec<u8, MAX_HASH_OUTPUT>,
}

pub struct KeySchedule {
    suite: CipherSuite,
    provider: Arc<dyn CryptoProvider>,
    early_secret: Vec<u8, MAX_HASH_OUTPUT>,
    handshake_secret: Option<Vec<u8, MAX_HASH_OUTPUT>>,
    master_secret: Option<Vec<u8, MAX_HASH_OUTPUT>>,
    client_early_traffic_secret: Vec<u8, MAX_HASH_OUTPUT>,
    zero_hash: Vec<u8, MAX_HASH_OUTPUT>,
}

impl KeySchedule {
    pub fn new(suite: CipherSuite, provider: Arc<dyn CryptoProvider>, psk: Option<&[u8]>) -> Self {
        let hash_size = suite.hash_size();
        let zero_hash = provider.hash(suite, &[]);

        let zeros = vec![0u8; hash_size];
        let early_secret = if let Some(psk) = psk {
            provider.hkdf_extract(suite, &zeros, psk)
        } else {
            provider.hkdf_extract(suite, &zeros, &zeros)
        };

        let binder_key = if psk.is_some() {
            provider.hkdf_expand_label(suite, &early_secret, b"tls13 res binder", &[], hash_size)
        } else {
            Vec::new()
        };

        let client_early_traffic_secret = if psk.is_some() {
            derive_secret(&early_secret, b"tls13 c e traffic", &zero_hash, &provider, suite)
        } else {
            Vec::new()
        };

        let _ = binder_key;

        KeySchedule {
            suite,
            provider,
            early_secret,
            handshake_secret: None,
            master_secret: None,
            client_early_traffic_secret,
            zero_hash,
        }
    }

    pub fn add_shared_secret(&mut self, shared_secret: &[u8]) {
        let hash_size = self.suite.hash_size();

        let derived = derive_secret(
            &self.early_secret,
            b"tls13 derived",
            &self.zero_hash,
            &self.provider,
            self.suite,
        );
        let handshake_secret = self
            .provider
            .hkdf_extract(self.suite, &derived[..hash_size], shared_secret);

        let zeros = vec![0u8; hash_size];
        let handshake_derived =
            derive_secret(&handshake_secret, b"tls13 derived", &self.zero_hash, &self.provider, self.suite);
        let master_secret = self
            .provider
            .hkdf_extract(self.suite, &handshake_derived[..hash_size], &zeros);

        self.handshake_secret = Some(handshake_secret);
        self.master_secret = Some(master_secret);
    }

    pub fn derive_keys(
        &self,
        server_hello_transcript: &[u8],
        server_finished_transcript: &[u8],
        client_finished_transcript: &[u8],
    ) -> TlsKeys {
        let hs = self.handshake_secret.as_ref().expect("add_shared_secret not called");
        let ms = self.master_secret.as_ref().expect("add_shared_secret not called");

        let key_size = self.suite.key_size();

        let c_hs_traffic =
            derive_secret(hs, b"tls13 c hs traffic", server_hello_transcript, &self.provider, self.suite);
        let s_hs_traffic =
            derive_secret(hs, b"tls13 s hs traffic", server_hello_transcript, &self.provider, self.suite);

        let client_handshake_key =
            self.provider
                .hkdf_expand_label(self.suite, &c_hs_traffic, b"tls13 key", &[], key_size);
        let client_handshake_iv: [u8; 12] = self
            .provider
            .hkdf_expand_label(self.suite, &c_hs_traffic, b"tls13 iv", &[], 12)
            .as_slice()
            .try_into()
            .unwrap();
        let server_handshake_key =
            self.provider
                .hkdf_expand_label(self.suite, &s_hs_traffic, b"tls13 key", &[], key_size);
        let server_handshake_iv: [u8; 12] = self
            .provider
            .hkdf_expand_label(self.suite, &s_hs_traffic, b"tls13 iv", &[], 12)
            .as_slice()
            .try_into()
            .unwrap();

        let server_finished_key =
            self.provider
                .hkdf_expand_label(self.suite, &s_hs_traffic, b"tls13 finished", &[], self.suite.hash_size());
        let client_finished_key =
            self.provider
                .hkdf_expand_label(self.suite, &c_hs_traffic, b"tls13 finished", &[], self.suite.hash_size());

        let c_ap_traffic = derive_secret(
            ms,
            b"tls13 c ap traffic",
            server_finished_transcript,
            &self.provider,
            self.suite,
        );
        let s_ap_traffic = derive_secret(
            ms,
            b"tls13 s ap traffic",
            server_finished_transcript,
            &self.provider,
            self.suite,
        );

        let client_application_key =
            self.provider
                .hkdf_expand_label(self.suite, &c_ap_traffic, b"tls13 key", &[], key_size);
        let client_application_iv: [u8; 12] = self
            .provider
            .hkdf_expand_label(self.suite, &c_ap_traffic, b"tls13 iv", &[], 12)
            .as_slice()
            .try_into()
            .unwrap();
        let server_application_key =
            self.provider
                .hkdf_expand_label(self.suite, &s_ap_traffic, b"tls13 key", &[], key_size);
        let server_application_iv: [u8; 12] = self
            .provider
            .hkdf_expand_label(self.suite, &s_ap_traffic, b"tls13 iv", &[], 12)
            .as_slice()
            .try_into()
            .unwrap();

        let res_master = derive_secret(ms, b"tls13 res master", client_finished_transcript, &self.provider, self.suite);
        let exporter = derive_secret(ms, b"tls13 exp master", server_finished_transcript, &self.provider, self.suite);

        let binder_key = self.provider.hkdf_expand_label(
            self.suite,
            &self.early_secret,
            b"tls13 res binder",
            &self.zero_hash,
            self.suite.hash_size(),
        );

        TlsKeys {
            suite: self.suite,
            client_handshake_key,
            client_handshake_iv,
            server_handshake_key,
            server_handshake_iv,
            server_finished_key,
            client_finished_key,
            client_application_key,
            client_application_iv,
            server_application_key,
            server_application_iv,
            resumption_master_secret: res_master,
            client_handshake_traffic_secret: c_hs_traffic,
            server_handshake_traffic_secret: s_hs_traffic,
            client_application_traffic_secret: c_ap_traffic,
            server_application_traffic_secret: s_ap_traffic,
            client_early_traffic_secret: self.client_early_traffic_secret.clone(),
            binder_key,
            exporter_master_secret: exporter,
        }
    }

    pub fn compute_binder_key(&self) -> Vec<u8, MAX_HASH_OUTPUT> {
        self.provider.hkdf_expand_label(
            self.suite,
            &self.early_secret,
            b"tls13 res binder",
            &self.zero_hash,
            self.suite.hash_size(),
        )
    }

    pub fn compute_finished(&self, finished_key: &[u8], transcript_hash: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT> {
        self.provider.hmac(self.suite, finished_key, transcript_hash)
    }

    pub fn verify_finished(
        &self,
        finished_key: &[u8],
        transcript_hash: &[u8],
        verify_data: &[u8],
    ) -> Result<(), crate::Error> {
        let expected = self.compute_finished(finished_key, transcript_hash);
        if constant_time_eq::constant_time_eq(&expected, verify_data) {
            Ok(())
        } else {
            Err(crate::Error::HandshakeFailed("finished verification failed".into()))
        }
    }

    pub fn hash_size(&self) -> usize {
        self.suite.hash_size()
    }

    pub fn early_secret(&self) -> &[u8] {
        &self.early_secret
    }

    pub fn handshake_secret(&self) -> Option<&[u8]> {
        self.handshake_secret.as_deref()
    }

    pub fn client_handshake_traffic_secret(&self, transcript: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT> {
        let hs = self.handshake_secret.as_ref().expect("add_shared_secret not called");
        derive_secret(hs, b"tls13 c hs traffic", transcript, &self.provider, self.suite)
    }

    pub fn server_handshake_traffic_secret(&self, transcript: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT> {
        let hs = self.handshake_secret.as_ref().expect("add_shared_secret not called");
        derive_secret(hs, b"tls13 s hs traffic", transcript, &self.provider, self.suite)
    }

    pub fn client_application_traffic_secret(&self, transcript: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT> {
        let ms = self.master_secret.as_ref().expect("add_shared_secret not called");
        derive_secret(ms, b"tls13 c ap traffic", transcript, &self.provider, self.suite)
    }

    pub fn server_application_traffic_secret(&self, transcript: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT> {
        let ms = self.master_secret.as_ref().expect("add_shared_secret not called");
        derive_secret(ms, b"tls13 s ap traffic", transcript, &self.provider, self.suite)
    }

    pub fn exporter_master_secret(&self, transcript: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT> {
        let ms = self.master_secret.as_ref().expect("add_shared_secret not called");
        derive_secret(ms, b"tls13 exp master", transcript, &self.provider, self.suite)
    }
}
