//! Async pure-Rust ACME (RFC 8555) client.

#![warn(unreachable_pub)]
#![warn(missing_docs)]

use std::{fmt, sync::Arc};

use crypto::{Hasher, encoding::pkcs8, hmac::Hmac, p256::SecretKey, sha2::Sha256};
use reqwest::{
    Method, Response, StatusCode,
    header::{CONTENT_TYPE, LOCATION},
};
use serde::{Serialize, de::DeserializeOwned};

mod types;
pub use types::{
    AccountCredentials,
    Authorization,
    AuthorizationStatus,
    Challenge,
    ChallengeType,
    Error,
    Identifier,
    LetsEncrypt,
    NewAccount,
    NewOrder,
    OrderState,
    OrderStatus,
    Problem,
    RevocationReason,
    ZeroSsl, // RevocationRequest
};
use types::{
    DirectoryUrls, Empty, FinalizeRequest, Header, JoseJson, Jwk, KeyOrKeyId, NewAccountPayload, Signer,
    SigningAlgorithm,
};

/// An ACME order as described in RFC 8555 (section 7.1.3)
///
/// An order is created from an [`Account`] by calling [`Account::new_order()`]. The `Order`
/// type represents the stable identity of an order, while the [`Order::state()`] method
/// gives you access to the current state of the order according to the server.
///
/// <https://datatracker.ietf.org/doc/html/rfc8555#section-7.1.3>
pub struct Order {
    account: Arc<AccountInner>,
    nonce: Option<String>,
    url: String,
    state: OrderState,
}

impl Order {
    /// Retrieve the authorizations for this order
    ///
    /// An order will contain one authorization to complete per identifier in the order.
    /// After creating an order, you'll need to retrieve the authorizations so that
    /// you can set up a challenge response for each authorization.
    ///
    /// For each authorization, you'll need to:
    ///
    /// * Select which [`ChallengeType`] you want to complete
    /// * Create a [`KeyAuthorization`] for that [`Challenge`]
    /// * Call [`Order::set_challenge_ready()`] for that challenge
    ///
    /// After the challenges have been set up, check the [`Order::state()`] to see
    /// if the order is ready to be finalized (or becomes invalid). Once it is
    /// ready, call `Order::finalize()` to get the certificate.
    pub async fn authorizations(&mut self) -> Result<Vec<Authorization>, Error> {
        let mut authorizations = Vec::with_capacity(self.state.authorizations.len());
        for url in &self.state.authorizations {
            authorizations.push(self.account.get(&mut self.nonce, url).await?);
        }
        Ok(authorizations)
    }

    /// Create a [`KeyAuthorization`] for the given [`Challenge`]
    ///
    /// Signs the challenge's token with the account's private key and use the
    /// value from [`KeyAuthorization::as_str()`] as the challenge response.
    pub fn key_authorization(&self, challenge: &Challenge) -> KeyAuthorization {
        KeyAuthorization::new(challenge, &self.account.key)
    }

    /// Request a certificate from the given Certificate Signing Request (CSR)
    ///
    /// Creating a CSR is outside of the scope of instant-acme. Make sure you pass in a
    /// DER representation of the CSR in `csr_der`. Call `certificate()` to retrieve the
    /// certificate chain once the order is in the appropriate state.
    pub async fn finalize(&mut self, csr_der: &[u8]) -> Result<(), Error> {
        let rsp = self
            .account
            .post(Some(&FinalizeRequest::new(csr_der)), self.nonce.take(), &self.state.finalize)
            .await?;

        self.nonce = nonce_from_response(&rsp);
        self.state = Problem::check::<OrderState>(rsp).await?;
        Ok(())
    }

    /// Get the certificate for this order
    ///
    /// If the cached order state is in `ready` or `processing` state, this will poll the server
    /// for the latest state. If the order is still in `processing` state after that, this will
    /// return `Ok(None)`. If the order is in `valid` state, this will attempt to retrieve
    /// the certificate from the server and return it as a `String`. If the order contains
    /// an error or ends up in any state other than `valid` or `processing`, return an error.
    pub async fn certificate(&mut self) -> Result<Option<String>, Error> {
        if matches!(self.state.status, OrderStatus::Processing) {
            let rsp = self.account.post(None::<&Empty>, self.nonce.take(), &self.url).await?;
            self.nonce = nonce_from_response(&rsp);
            self.state = Problem::check::<OrderState>(rsp).await?;
        }

        if let Some(error) = &self.state.error {
            return Err(Error::Api(error.clone()));
        } else if self.state.status == OrderStatus::Processing {
            return Ok(None);
        } else if self.state.status != OrderStatus::Valid {
            return Err(Error::Str("invalid order state"));
        }

        let cert_url = match &self.state.certificate {
            Some(cert_url) => cert_url,
            None => return Err(Error::Str("no certificate URL found")),
        };

        let rsp = self.account.post(None::<&Empty>, self.nonce.take(), cert_url).await?;

        // let body = rsp.bytes().await?;
        let body = Problem::from_response(rsp).await?;
        Ok(Some(
            String::from_utf8(body.to_vec()).map_err(|_| "unable to decode certificate as UTF-8")?,
        ))
    }

    /// Notify the server that the given challenge is ready to be completed
    ///
    /// `challenge_url` should be the `Challenge::url` field.
    pub async fn set_challenge_ready(&mut self, challenge_url: &str) -> Result<(), Error> {
        let rsp = self
            .account
            .post(Some(&Empty {}), self.nonce.take(), challenge_url)
            .await?;

        self.nonce = nonce_from_response(&rsp);
        let _ = Problem::check::<Challenge>(rsp).await?;
        Ok(())
    }

    /// Get the current state of the given challenge
    pub async fn challenge(&mut self, challenge_url: &str) -> Result<Challenge, Error> {
        self.account.get(&mut self.nonce, challenge_url).await
    }

    /// Refresh the current state of the order
    pub async fn refresh(&mut self) -> Result<&OrderState, Error> {
        let rsp = self.account.post(None::<&Empty>, self.nonce.take(), &self.url).await?;

        self.nonce = nonce_from_response(&rsp);
        self.state = Problem::check::<OrderState>(rsp).await?;
        Ok(&self.state)
    }

    /// Get the last known state of the order
    ///
    /// Call `refresh()` to get the latest state from the server.
    pub fn state(&mut self) -> &OrderState {
        &self.state
    }

    /// Get the URL of the order
    pub fn url(&self) -> &str {
        &self.url
    }
}

/// An ACME account as described in RFC 8555 (section 7.1.2)
///
/// Create an [`Account`] with [`Account::create()`] or restore it from serialized data
/// by passing deserialized [`AccountCredentials`] to [`Account::from_credentials()`].
///
/// The [`Account`] type is cheap to clone.
///
/// <https://datatracker.ietf.org/doc/html/rfc8555#section-7.1.2>
#[derive(Clone)]
pub struct Account {
    inner: Arc<AccountInner>,
}

impl Account {
    /// Restore an existing account from the given credentials
    ///
    /// The [`AccountCredentials`] type is opaque, but supports deserialization.
    // #[cfg(feature = "hyper-rustls")]
    pub async fn from_credentials(credentials: AccountCredentials) -> Result<Self, Error> {
        Ok(Self {
            inner: Arc::new(AccountInner::from_credentials(credentials, reqwest::Client::new()).await?),
        })
    }

    /// Restore an existing account from the given credentials and HTTP client
    ///
    /// The [`AccountCredentials`] type is opaque, but supports deserialization.
    pub async fn from_credentials_and_http(
        credentials: AccountCredentials,
        http: reqwest::Client,
    ) -> Result<Self, Error> {
        Ok(Self {
            inner: Arc::new(AccountInner::from_credentials(credentials, http).await?),
        })
    }

    /// Restore an existing account from the given ID, private key, server URL and HTTP client
    ///
    /// The key must be provided in DER-encoded PKCS#8. This is usually how ECDSA keys are
    /// encoded in PEM files. Use a crate like rustls-pemfile to decode from PEM to DER.
    pub async fn from_parts(
        id: String,
        key_pkcs8_der: &[u8],
        directory_url: &str,
        http: reqwest::Client,
    ) -> Result<Self, Error> {
        Ok(Self {
            inner: Arc::new(AccountInner {
                id,
                key: Key::from_pkcs8_der(key_pkcs8_der)?,
                client: Client::new(directory_url, http).await?,
            }),
        })
    }

    /// Create a new account on the `server_url` with the information in [`NewAccount`]
    ///
    /// The returned [`AccountCredentials`] can be serialized and stored for later use.
    /// Use [`Account::from_credentials()`] to restore the account from the credentials.
    #[cfg(feature = "hyper-rustls")]
    pub async fn create(
        account: &NewAccount<'_>,
        server_url: &str,
        external_account: Option<&ExternalAccountKey>,
    ) -> Result<(Account, AccountCredentials), Error> {
        Self::create_inner(
            account,
            external_account,
            Client::new(server_url, Box::<DefaultClient>::default()).await?,
            server_url,
        )
        .await
    }

    /// Create a new account with a custom HTTP client
    ///
    /// The returned [`AccountCredentials`] can be serialized and stored for later use.
    /// Use [`Account::from_credentials()`] to restore the account from the credentials.
    pub async fn create_with_http(
        account: &NewAccount<'_>,
        server_url: &str,
        external_account: Option<&ExternalAccountKey>,
        http: reqwest::Client,
    ) -> Result<(Account, AccountCredentials), Error> {
        Self::create_inner(account, external_account, Client::new(server_url, http).await?, server_url).await
    }

    async fn create_inner(
        account: &NewAccount<'_>,
        external_account: Option<&ExternalAccountKey>,
        client: Client,
        server_url: &str,
    ) -> Result<(Account, AccountCredentials), Error> {
        let (key, key_pkcs8) = Key::generate()?;
        let payload = NewAccountPayload {
            new_account: account,
            external_account_binding: external_account
                .map(|eak| {
                    JoseJson::new(
                        Some(&Jwk::new(&key.inner.public_key())),
                        eak.header(None, &client.urls.new_account),
                        eak,
                    )
                })
                .transpose()?,
        };

        let rsp = client
            .post(Some(&payload), None, &key, &client.urls.new_account)
            .await?;

        let account_url = rsp
            .headers()
            .get(LOCATION)
            .and_then(|hv| hv.to_str().ok())
            .map(|s| s.to_owned());

        // The response redirects, we don't need the body
        let _ = Problem::from_response(rsp).await?;
        let id = account_url.ok_or("failed to get account URL")?;
        let credentials = AccountCredentials {
            id: id.clone(),
            key_pkcs8: key_pkcs8.to_vec(),
            directory: Some(server_url.to_owned()),
            // We support deserializing URLs for compatibility with versions pre 0.4,
            // but we prefer to get fresh URLs from the `server_url` for newer credentials.
            urls: None,
        };

        let account = AccountInner {
            client,
            key,
            id: id.clone(),
        };

        Ok((
            Self {
                inner: Arc::new(account),
            },
            credentials,
        ))
    }

    /// Create a new order based on the given [`NewOrder`]
    ///
    /// Returns an [`Order`] instance. Use the [`Order::state()`] method to inspect its state.
    pub async fn new_order<'a>(&'a self, order: &NewOrder<'_>) -> Result<Order, Error> {
        let rsp = self
            .inner
            .post(Some(order), None, &self.inner.client.urls.new_order)
            .await?;

        let nonce = nonce_from_response(&rsp);
        let order_url = rsp
            .headers()
            .get(LOCATION)
            .and_then(|hv| hv.to_str().ok())
            .map(|s| s.to_owned());

        Ok(Order {
            account: self.inner.clone(),
            nonce,
            // Order of fields matters! We return errors from Problem::check
            // before emitting an error if there is no order url. Or the
            // simple no url error hides the causing error in `Problem::check`.
            state: Problem::check::<OrderState>(rsp).await?,
            url: order_url.ok_or("no order URL found")?,
        })
    }

    // /// Revokes a previously issued certificate
    // pub async fn revoke<'a>(&'a self, payload: &RevocationRequest<'a>) -> Result<(), Error> {
    //     let revoke_url = match self.inner.client.urls.revoke_cert.as_deref() {
    //         Some(url) => url,
    //         // This happens because the current account credentials were deserialized from an
    //         // older version which only serialized a subset of the directory URLs. You should
    //         // make sure the account credentials include a `directory` field containing a
    //         // string with the server's directory URL.
    //         None => return Err("no revokeCert URL found".into()),
    //     };

    //     let rsp = self.inner.post(Some(payload), None, revoke_url).await?;
    //     // The body is empty if the request was successful
    //     let _ = Problem::from_response(rsp).await?;
    //     Ok(())
    // }
}

struct AccountInner {
    client: Client,
    key: Key,
    id: String,
}

impl AccountInner {
    async fn from_credentials(credentials: AccountCredentials, http: reqwest::Client) -> Result<Self, Error> {
        Ok(Self {
            id: credentials.id,
            key: Key::from_pkcs8_der(credentials.key_pkcs8.as_ref())?,
            client: match (credentials.directory, credentials.urls) {
                (Some(server_url), _) => Client::new(&server_url, http).await?,
                (None, Some(urls)) => Client {
                    http,
                    urls,
                },
                (None, None) => return Err("no server URLs found".into()),
            },
        })
    }

    async fn get<T: DeserializeOwned>(&self, nonce: &mut Option<String>, url: &str) -> Result<T, Error> {
        let rsp = self.post(None::<&Empty>, nonce.take(), url).await?;
        *nonce = nonce_from_response(&rsp);
        Problem::check(rsp).await
    }

    async fn post(
        &self,
        payload: Option<&impl Serialize>,
        nonce: Option<String>,
        url: &str,
    ) -> Result<Response, Error> {
        self.client.post(payload, nonce, self, url).await
    }
}

impl Signer for AccountInner {
    type Signature = <Key as Signer>::Signature;

    fn header<'n, 'u: 'n, 's: 'u>(&'s self, nonce: Option<&'n str>, url: &'u str) -> Header<'n> {
        debug_assert!(nonce.is_some());
        Header {
            alg: self.key.signing_algorithm,
            key: KeyOrKeyId::KeyId(&self.id),
            nonce,
            url,
        }
    }

    fn sign(&self, payload: &[u8]) -> Result<Self::Signature, Error> {
        self.key.sign(payload)
    }
}

struct Client {
    http: reqwest::Client,
    urls: DirectoryUrls,
}

impl Client {
    async fn new(server_url: &str, http: reqwest::Client) -> Result<Self, Error> {
        // let req = Request::new(Method::GET, )
        //     .body(Body::empty())
        //     .unwrap();
        let res = http.get(server_url).send().await?;

        // .request(req).await?;
        let urls = res.json().await?;
        Ok(Client {
            http,
            urls,
        })
    }

    async fn post(
        &self,
        payload: Option<&impl Serialize>,
        nonce: Option<String>,
        signer: &impl Signer,
        url: &str,
    ) -> Result<Response, Error> {
        let nonce = self.nonce(nonce).await?;
        let body = JoseJson::new(payload, signer.header(Some(&nonce), url), signer)?;

        let request = self
            .http
            .request(Method::POST, url)
            .header(CONTENT_TYPE, JOSE_JSON)
            .json(&body)
            .build()?;
        // let request = RequestBuilder::()
        //     .method(Method::POST)
        //     .uri(url)
        //     .header(CONTENT_TYPE, JOSE_JSON)
        //     .body(Body::from(serde_json::to_vec(&body)?))
        //     .unwrap();

        Ok(self.http.execute(request).await?)
    }

    async fn nonce(&self, nonce: Option<String>) -> Result<String, Error> {
        if let Some(nonce) = nonce {
            return Ok(nonce);
        }

        let rsp = self.http.head(&self.urls.new_nonce).send().await?;

        // https://datatracker.ietf.org/doc/html/rfc8555#section-7.2
        // "The server's response MUST include a Replay-Nonce header field containing a fresh
        // nonce and SHOULD have status code 200 (OK)."
        if rsp.status() != StatusCode::OK {
            return Err("error response from newNonce resource".into());
        }

        match nonce_from_response(&rsp) {
            Some(nonce) => Ok(nonce),
            None => Err("no nonce found in newNonce response".into()),
        }
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("client", &"..")
            .field("urls", &self.urls)
            .finish()
    }
}

struct Key {
    signing_algorithm: SigningAlgorithm,
    inner: SecretKey,
    thumb: String,
}

impl Key {
    fn generate() -> Result<(Self, [u8; 138]), Error> {
        let inner = SecretKey::generate().map_err(|_| Error::Crypto)?;
        let pkcs8_der = pkcs8::encode_p256_pkcs8_der(&inner).map_err(|_| Error::Crypto)?;
        let thumb = base64::encode(&Jwk::thumb_sha256(&inner.public_key())?, base64::Alphabet::UrlNoPadding);

        Ok((
            Self {
                signing_algorithm: SigningAlgorithm::Es256,
                inner,
                thumb,
            },
            pkcs8_der,
        ))
    }

    fn from_pkcs8_der(pkcs8_der: &[u8]) -> Result<Self, Error> {
        let inner = pkcs8::decode_p256_pkcs8_der(pkcs8_der).map_err(|_| Error::CryptoKey)?;
        let thumb = base64::encode(&Jwk::thumb_sha256(&inner.public_key())?, base64::Alphabet::UrlNoPadding);

        Ok(Self {
            signing_algorithm: SigningAlgorithm::Es256,
            inner,
            thumb,
        })
    }
}

impl Signer for Key {
    type Signature = [u8; 64];

    fn header<'n, 'u: 'n, 's: 'u>(&'s self, nonce: Option<&'n str>, url: &'u str) -> Header<'n> {
        debug_assert!(nonce.is_some());
        Header {
            alg: self.signing_algorithm,
            key: KeyOrKeyId::from_key(&self.inner.public_key()),
            nonce,
            url,
        }
    }

    fn sign(&self, payload: &[u8]) -> Result<Self::Signature, Error> {
        self.inner.sign(payload).map_err(|_| Error::Crypto)
    }
}

/// The response value to use for challenge responses
///
/// Refer to the methods below to see which encoding to use for your challenge type.
///
/// <https://datatracker.ietf.org/doc/html/rfc8555#section-8.1>
pub struct KeyAuthorization(String);

impl KeyAuthorization {
    fn new(challenge: &Challenge, key: &Key) -> Self {
        Self(format!("{}.{}", challenge.token, &key.thumb))
    }

    /// Get the key authorization value
    ///
    /// This can be used for HTTP-01 challenge responses.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the SHA-256 digest of the key authorization
    ///
    /// This can be used for TLS-ALPN-01 challenge responses.
    ///
    /// <https://datatracker.ietf.org/doc/html/rfc8737#section-3>
    pub fn digest(&self) -> impl AsRef<[u8]> {
        let hash = Sha256::hash(self.0.as_bytes());
        let mut out = [0u8; 32];
        out.copy_from_slice(&hash.as_ref()[..32]);
        out
    }

    /// Get the base64-encoded SHA256 digest of the key authorization
    ///
    /// This can be used for DNS-01 challenge responses.
    pub fn dns_value(&self) -> String {
        base64::encode(self.digest().as_ref(), base64::Alphabet::UrlNoPadding)
    }
}

impl fmt::Debug for KeyAuthorization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("KeyAuthorization").finish()
    }
}

/// A HMAC key used to link account creation requests to an external account
///
/// See RFC 8555 section 7.3.4 for more information.
pub struct ExternalAccountKey {
    id: String,
    key_value: Vec<u8>,
}

impl ExternalAccountKey {
    /// Create a new external account key
    pub fn new(id: String, key_value: &[u8]) -> Self {
        Self {
            id,
            key_value: key_value.to_vec(),
        }
    }
}

impl Signer for ExternalAccountKey {
    type Signature = [u8; 32];

    fn header<'n, 'u: 'n, 's: 'u>(&'s self, nonce: Option<&'n str>, url: &'u str) -> Header<'n> {
        debug_assert_eq!(nonce, None);
        Header {
            alg: SigningAlgorithm::Hs256,
            key: KeyOrKeyId::KeyId(&self.id),
            nonce,
            url,
        }
    }

    fn sign(&self, payload: &[u8]) -> Result<Self::Signature, Error> {
        let h = Hmac::<Sha256>::mac(&self.key_value, payload);
        let mut out = [0u8; 32];
        out.copy_from_slice(&h.as_ref()[..32]);
        Ok(out)
    }
}

// fn nonce_from_response(rsp: &Response<Body>) -> Option<String> {
//     rsp.headers()
//         .get(REPLAY_NONCE)
//         .and_then(|hv| String::from_utf8(hv.as_ref().to_vec()).ok())
// }

fn nonce_from_response(res: &Response) -> Option<String> {
    res.headers()
        .get(REPLAY_NONCE)
        .map(|header| header.to_str().unwrap_or_default().to_string())
}

#[cfg(feature = "hyper-rustls")]
struct DefaultClient(hyper::Client<hyper_rustls::HttpsConnector<HttpConnector>>);

#[cfg(feature = "hyper-rustls")]
impl HttpClient for DefaultClient {
    fn request(&self, req: Request<Body>) -> Pin<Box<dyn Future<Output = hyper::Result<Response<Body>>> + Send>> {
        Box::pin(self.0.request(req))
    }
}

#[cfg(feature = "hyper-rustls")]
impl Default for DefaultClient {
    fn default() -> Self {
        Self(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_only()
                    .enable_http1()
                    .enable_http2()
                    .build(),
            ),
        )
    }
}

// /// A HTTP client based on [`hyper::Client`]
// pub trait HttpClient: Send + Sync + 'static {
//     /// Send the given request and return the response
//     fn request(
//         &self,
//         req: Request<Body>,
//     ) -> Pin<Box<dyn Future<Output = hyper::Result<Response<Body>>> + Send>>;
// }

// impl<C> HttpClient for hyper::Client<C>
// where
//     C: Connect + Clone + Send + Sync + 'static,
// {
//     fn request(
//         &self,
//         req: Request<Body>,
//     ) -> Pin<Box<dyn Future<Output = hyper::Result<Response<Body>>> + Send>> {
//         Box::pin(<hyper::Client<C>>::request(self, req))
//     }
// }

const JOSE_JSON: &str = "application/jose+json";
const REPLAY_NONCE: &str = "Replay-Nonce";

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn deserialize_old_credentials() -> Result<(), Error> {
        const CREDENTIALS: &str = r#"{"id":"id","key_pkcs8":"MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgJVWC_QzOTCS5vtsJp2IG-UDc8cdDfeoKtxSZxaznM-mhRANCAAQenCPoGgPFTdPJ7VLLKt56RxPlYT1wNXnHc54PEyBg3LxKaH0-sJkX0mL8LyPEdsfL_Oz4TxHkWLJGrXVtNhfH","urls":{"newNonce":"new-nonce","newAccount":"new-acct","newOrder":"new-order", "revokeCert": "revoke-cert"}}"#;
        Account::from_credentials(serde_json::from_str::<AccountCredentials>(CREDENTIALS)?).await?;
        Ok(())
    }

    #[tokio::test]
    async fn deserialize_new_credentials() -> Result<(), Error> {
        const CREDENTIALS: &str = r#"{"id":"id","key_pkcs8":"MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgJVWC_QzOTCS5vtsJp2IG-UDc8cdDfeoKtxSZxaznM-mhRANCAAQenCPoGgPFTdPJ7VLLKt56RxPlYT1wNXnHc54PEyBg3LxKaH0-sJkX0mL8LyPEdsfL_Oz4TxHkWLJGrXVtNhfH","directory":"https://acme-staging-v02.api.letsencrypt.org/directory"}"#;
        Account::from_credentials(serde_json::from_str::<AccountCredentials>(CREDENTIALS)?).await?;
        Ok(())
    }
}
