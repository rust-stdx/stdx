use reqwest::{
    Method,
    header::{self, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub struct Client {
    pub http_client: reqwest::Client,
    pub api_base_url: &'static str,
    pub account_api_token: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiError {
    pub error_code: i64,
    pub message: String,
}

pub struct SendRequestInput<B: Serialize> {
    pub method: Method,
    pub url: String,
    pub body: B,
    pub server_token: Option<String>,
}

impl Client {
    /// Builds a client for the Postmark API.
    ///
    /// Installs the stdx rustls `CryptoProvider` as the process default (if
    /// none is installed yet) before creating the underlying `reqwest` client,
    /// because `reqwest` is compiled with the `rustls-no-provider` feature and
    /// would otherwise panic for lack of a cryptography provider.
    pub fn new(account_api_token: Option<String>) -> Client {
        crypto_rustls::install_default_rustls_provider();
        let http_client = reqwest::Client::new();

        return Client {
            http_client,
            api_base_url: "https://api.postmarkapp.com",
            account_api_token,
        };
    }

    pub(crate) async fn send_request<B: Serialize, R: DeserializeOwned>(
        &self,
        input: SendRequestInput<B>,
    ) -> Result<R, ApiError> {
        // it's okay to unwrap here because we are sure that the headers won't contain any invalid
        // characters
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::new();
        headers.insert(header::ACCEPT, HeaderValue::from_str("application/json").unwrap());
        headers.insert(header::CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());

        if let Some(server_api_token) = input.server_token {
            headers.insert("X-Postmark-Server-Token", HeaderValue::from_str(&server_api_token).unwrap());
        } else if let Some(account_api_token) = &self.account_api_token {
            headers.insert(
                "X-Postmark-Account-Token",
                HeaderValue::from_str(account_api_token.as_str()).unwrap(),
            );
        }

        let url = format!("{}{}", &self.api_base_url, input.url);
        let res = self
            .http_client
            .request(input.method, url)
            .headers(headers)
            .json(&input.body)
            .send()
            .await
            .map_err(|err| ApiError {
                error_code: 0,
                message: format!("postmark: error sending request: {err}"),
            })?;

        if res.status().as_u16() > 399 {
            let err: ApiError = res.json().await.map_err(|err| ApiError {
                error_code: 0,
                message: format!("postmark: error parsing error response: {err}"),
            })?;
            return Err(err);
        }

        let res: R = res.json().await.map_err(|err| ApiError {
            error_code: 0,
            message: format!("postmark: error parsing response: {err}"),
        })?;

        return Ok(res);
    }
}
