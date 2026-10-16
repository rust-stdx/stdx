//! A pure-Rust client for the [Stripe](https://stripe.com) API.
//!
//! `stripe` talks to Stripe's v1 (form-encoded) and v2 (JSON) HTTP APIs. Its
//! request and response types are generated from Stripe's official OpenAPI
//! specification, so they track the documented API surface, and the transport is
//! pluggable through the [`HttpClient`] trait.
//!
//! # Layout
//!
//! Every part of the API is a module named after the resource or API area it
//! belongs to: [`customers`], [`payment_intents`], [`invoices`], [`checkout`],
//! [`issuing`], ... Each module holds the response types, the request parameter
//! types and the asynchronous methods for that area, so everything about
//! customers is in [`customers`]:
//!
//! ```no_run
//! # async fn example(client: &stripe::Client) -> Result<(), stripe::Error> {
//! use stripe::{checkout, customers};
//!
//! let params = customers::ListCustomersParams { email: Some("jenny@example.com".into()), ..Default::default() };
//! let customers_page = customers::list_customers(client, &params).await?;
//!
//! let params = customers::GetCustomerParams { expand: None };
//! let customer = customers::get_customer(client, "cus_123", &params).await?;
//!
//! let params = checkout::CreateCheckoutSessionParams { ..Default::default() };
//! let session = checkout::create_checkout_session(client, &params).await?;
//! # let _ = (customers_page, customer, session);
//! # Ok(())
//! # }
//! ```
//!
//! Methods are grouped by path, so sub-resources live with their parent:
//! `customers::list_customer_sources`, `checkout::list_checkout_session_line_items`,
//! `invoices::pay_invoice`, `payment_intents::confirm_payment_intent`. Shared
//! shapes (`Address`, `Currency`, `Metadata`, ...) live in [`common`], and the
//! typed webhook payloads live in `events` (behind the `events` feature: no
//! endpoint returns them, so they are not compiled unless asked for).
//!
//! # Pagination
//!
//! List methods return [`List`]; hand it to [`Client::paginate`] to walk every
//! page. v1 lists page with a cursor, v2 lists with a `next_page_url`, and the
//! pager handles both:
//!
//! ```no_run
//! # async fn example(client: &stripe::Client) -> Result<(), stripe::Error> {
//! # use stripe::customers::{Customer, ListCustomersParams};
//! let params = ListCustomersParams { limit: Some(3), ..Default::default() };
//! let mut pager = client.paginate::<Customer>("/v1/customers", stripe::encode(&params));
//! while let Some(page) = pager.next_page().await? {
//!     for customer in page.data {
//!         println!("{}", customer.id);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Retries and idempotency
//!
//! `GET` and `DELETE` requests are retried on transport errors and on the
//! statuses Stripe documents as retryable (`409`, `429`, `5xx`, or anything
//! Stripe marks with `Stripe-Should-Retry: true`). A `POST` is only retried when
//! it carries an `Idempotency-Key`, which the client generates automatically, so
//! a retry cannot create a second object. Pass your own key with
//! [`RequestOptions`] when you want your *own* retries to be safe too:
//!
//! ```no_run
//! # async fn example(client: &stripe::Client) -> Result<(), stripe::Error> {
//! # use stripe::customers::{CreateCustomerParams, Customer};
//! let options = stripe::RequestOptions::new().with_idempotency_key("order-6735");
//! let params = CreateCustomerParams::default();
//! let customer: Customer = client
//!     .post_form_with_options("/v1/customers", &params, &options)
//!     .await?;
//! # let _ = customer;
//! # Ok(())
//! # }
//! ```
//!
//! # Errors
//!
//! Everything falls out of [`Error`]: transport failures, [`Error::Api`] for a
//! non-2xx response (with the parsed error object and the `Request-Id` for
//! Stripe support), [`Error::Decode`] when a success response does not match the
//! expected type, and [`Error::ResponseTooLarge`] when a body exceeds the
//! configured limit.
//!
//! # Cargo features
//!
//! Each module has a cargo feature of the same name, and enables the request
//! builders and methods in that module: enable `issuing`, `terminal`,
//! `treasury`, and so on to compile the corresponding endpoints. The default set
//! is `reqwest` (the built-in HTTP transport) plus `core`, an umbrella over the
//! resources most integrations need.
//!
//! Response types are gated by reachability rather than by where they live: a
//! type is compiled when an enabled module can return it, so enabling
//! `customers` compiles the customer types and the types they refer to (the
//! objects behind `expand`, for example) and leaves `issuing`, `treasury` and
//! the rest out of the build. A small set of types shared by most of the API is
//! always compiled. `events` is the exception in the other direction: no
//! endpoint returns its webhook payloads, so they are compiled only with the
//! `events` feature.
//!
//! Everything is generated from Stripe's official OpenAPI specification; see
//! [`SPEC_VERSION`].
//!
//! # Default HTTP client
//!
//! With the default `reqwest` feature, `Client::new("sk_test_…")` builds a
//! client using the bundled transport (which pools connections and installs the
//! stdx rustls provider). Pass any [`HttpClient`] implementation to
//! [`Client::with_http_client`] to use a different transport:
//!
//! ```no_run
//! # use stripe::{Client, HttpClient, HttpRequest, HttpResponseData, HttpError};
//! # struct MyHttp;
//! # impl HttpClient for MyHttp {
//! #     async fn send(&self, _request: HttpRequest) -> Result<HttpResponseData, HttpError> { todo!() }
//! # }
//! # fn example() -> Result<(), stripe::Error> {
//! let client = Client::with_http_client("sk_test_...", MyHttp)?;
//! # let _ = client;
//! # Ok(())
//! # }
//! ```
//!
//! # Regenerating from the specification
//!
//! The `api/` tree and the module list below are produced by the `stripe_codegen`
//! binary from the vendored specification at `stripe/spec/openapi.spec3.sdk.json`:
//!
//! ```text
//! make stripe_codegen         # regenerate in place
//! make stripe_codegen_check   # fail if the committed code is stale
//! ```
//!
//! Regeneration needs no network access; pass `--fetch` to `stripe_codegen` to
//! download the latest specification first.
//!
//! ## Naming rules
//!
//! Names are derived from the API paths, not from Stripe's internal operation
//! ids, so they stay stable and readable:
//!
//! * the module is the first segment after the version (`/v1/customers/…` →
//!   `customers`, `/v2/core/events` → `v2_core`);
//! * the resource chain is every non-placeholder segment, singularized
//!   (`/v1/customers/{customer}/sources/{id}` → `customer_source`);
//! * the verb is `get`, `list`, `search`, `create`, `update` or `delete`
//!   (`list` for a list-shaped response, `create` for a new sub-resource), or the
//!   trailing action segment for an endpoint like `/confirm`;
//! * a single-word action is prefixed to the chain (`expire_checkout_session`),
//!   a multi-word one reads as a whole operation (`confirm_payment_intent`);
//! * parameter types are `{Verb}{Chain}Params`, so
//!   `GET /v1/customers/{customer}` takes `GetCustomerParams`.
//!
//! `tests/api_surface.rs` pins a representative sample of these names.
//!
//! # Integration tests
//!
//! Integration tests in `stripe/tests/integration.rs` talk to the real Stripe
//! API but are **opt-in and off by default**. They run only when both variables
//! are set:
//!
//! * `STRIPE_INTEGRATION_TESTS=1` (or `true`)
//! * `STRIPE_API_KEY`, which must be a **test** key (`sk_test_…` or `rk_test_…`)
//!
//! Without them every test prints `skipping …` and returns, so `cargo test`
//! stays offline. If a key is present but is not a test key the tests fail
//! loudly instead of touching a live account, and the account's mode is
//! double-checked at runtime with `GET /v1/balance` (`livemode` must be false).
//! Every object a test creates is deleted afterwards, even when an assertion
//! fails; a resource left behind by a panic is reported on stderr. A restricted
//! test key needs read/write access to balance, customers, products, prices,
//! payment intents and webhook endpoints.
//!
//! ```text
//! STRIPE_INTEGRATION_TESTS=1 STRIPE_API_KEY=sk_test_… \
//!     cargo test -p stripe --test integration -- --nocapture --test-threads=1
//! # or
//! STRIPE_INTEGRATION_TESTS=1 STRIPE_API_KEY=sk_test_… make stripe_integration_test
//! ```
//!
//! # Security and robustness
//!
//! * **No panics on hostile input.** Malformed JSON, oversized bodies, absurd
//!   integers and unknown enum values are reported as errors, never as panics.
//! * **Bounded resources.** Response bodies are capped (see
//!   [`Client::with_max_response_size`]) and retries are bounded.
//! * **Forward compatible.** Unknown response fields are ignored, unknown enum
//!   values are preserved, and a field the response omits — including most
//!   fields of an object returned for a deleted resource — decodes to its
//!   default instead of failing the whole response. `null` is accepted wherever
//!   the specification allows it (nullable fields, enums); a `null` in a field
//!   that requires a value is reported as a decoding error, like a wrongly typed
//!   value, and so is a key that appears twice in the same object (which would
//!   otherwise silently lose one of the two values).
//! * **Safe retries.** A `POST` that may be retried always carries a stable
//!   `Idempotency-Key`, so a retry cannot create a duplicate object.
//! * **Constant-time webhook verification.** See [`Webhook`].
//! * **Secrets stay secret.** The API key is zeroized on drop and never
//!   rendered by [`Client`]'s `Debug` implementation.
//! * **No request injection.** Path segments, query keys and form keys are
//!   percent-encoded, and header values are rejected if they contain control
//!   characters.
//!
//! # Known limitations
//!
//! * Endpoints whose request body is `multipart/form-data` (currently
//!   `POST /v1/files`) return [`Error::UnsupportedEncoding`] instead of sending
//!   a malformed request.
//! * `POST /v1/files` and other multipart endpoints therefore cannot be used
//!   yet.
//!
//! # Webhooks
//!
//! Verify a webhook using the raw request body:
//!
//! ```
//! # use stripe::{Webhook, WebhookError};
//! # let payload = br#"{"id":"evt_1"}"#;
//! # let secret = "whsec_test";
//! # fn header(t: i64, payload: &[u8], secret: &str) -> String {
//! #     let mut mac = crypto::hmac::Hmac::<crypto::sha2::Sha256>::new(secret.as_bytes());
//! #     use crypto::Hasher as _;
//! #     mac.update(t.to_string().as_bytes());
//! #     mac.update(b".");
//! #     mac.update(payload);
//! #     let tag = mac.finalize();
//! #     let hex: String = tag.as_ref().iter().map(|b| format!("{b:02x}")).collect();
//! #     format!("t={t},v1={hex}")
//! # }
//! # let header = header(1_700_000_000, payload, secret);
//! // `tolerance = 0` disables the recency check; use a real window in production.
//! Webhook::verify_header(payload, &header, secret, 0).unwrap();
//! ```

#![deny(missing_docs)]
#![deny(clippy::undocumented_unsafe_blocks)]

mod client;
pub mod error;
mod expandable;
pub mod http;
pub mod pagination;
pub mod params;
mod request;
pub mod webhooks;

pub use client::{
    Client, DEFAULT_API_VERSION, DEFAULT_BASE_URL, DEFAULT_MAX_RESPONSE_SIZE, DEFAULT_MAX_RETRIES, RequestOptions,
};
pub use error::{ApiErrorBody, ApiResponseMeta, Error, HttpError, WebhookError};
pub use expandable::Expandable;
#[cfg(feature = "reqwest")]
pub use http::ReqwestHttpClient;
pub use http::{ByteStream, HttpClient, HttpMethod, HttpRequest, HttpResponseData, byte_stream, chunk_stream};
pub use pagination::{List, PagedItem, Pager};
pub use params::{FormBuilder, FormEncode, encode, percent_encode_into};
pub use webhooks::{DEFAULT_TOLERANCE_SECONDS, Webhook, verify_signature};

/// The Stripe API version this crate was generated from.
pub const SPEC_VERSION: &str = "2026-09-30.endive";

// region: generated modules (do not edit; `make stripe_codegen`)
//
// Every module is also a cargo feature of the same name: it gates the request
// builders and methods for that part of the API. Response types are gated by
// reachability instead: a type is compiled only when one of the features on its
// `#[cfg]` (the modules whose methods can reach it) is enabled. The modules in
// `GATED_MODULES` only application code can reach, so they are gated as a whole.
#[path = "api/account.rs"]
pub mod account;
#[path = "api/account_links.rs"]
pub mod account_links;
#[path = "api/account_sessions.rs"]
pub mod account_sessions;
#[path = "api/accounts.rs"]
pub mod accounts;
#[path = "api/apple_pay.rs"]
pub mod apple_pay;
#[path = "api/application_fees.rs"]
pub mod application_fees;
#[path = "api/apps.rs"]
pub mod apps;
#[path = "api/balance.rs"]
pub mod balance;
#[path = "api/balance_settings.rs"]
pub mod balance_settings;
#[path = "api/balance_transactions.rs"]
pub mod balance_transactions;
#[path = "api/billing.rs"]
pub mod billing;
#[path = "api/billing_portal.rs"]
pub mod billing_portal;
#[path = "api/charges.rs"]
pub mod charges;
#[path = "api/checkout.rs"]
pub mod checkout;
#[path = "api/climate.rs"]
pub mod climate;
#[path = "api/common.rs"]
pub mod common;
#[path = "api/confirmation_tokens.rs"]
pub mod confirmation_tokens;
#[path = "api/country_specs.rs"]
pub mod country_specs;
#[path = "api/coupons.rs"]
pub mod coupons;
#[path = "api/credit_notes.rs"]
pub mod credit_notes;
#[path = "api/customer_sessions.rs"]
pub mod customer_sessions;
#[path = "api/customers.rs"]
pub mod customers;
#[path = "api/disputes.rs"]
pub mod disputes;
#[path = "api/entitlements.rs"]
pub mod entitlements;
#[path = "api/ephemeral_keys.rs"]
pub mod ephemeral_keys;
#[cfg(feature = "events")]
#[path = "api/events.rs"]
pub mod events;
#[path = "api/exchange_rates.rs"]
pub mod exchange_rates;
#[path = "api/file_links.rs"]
pub mod file_links;
#[path = "api/files.rs"]
pub mod files;
#[path = "api/financial_connections.rs"]
pub mod financial_connections;
#[path = "api/forwarding.rs"]
pub mod forwarding;
#[path = "api/identity.rs"]
pub mod identity;
#[path = "api/invoice_payments.rs"]
pub mod invoice_payments;
#[path = "api/invoice_rendering_templates.rs"]
pub mod invoice_rendering_templates;
#[path = "api/invoiceitems.rs"]
pub mod invoiceitems;
#[path = "api/invoices.rs"]
pub mod invoices;
#[path = "api/issuing.rs"]
pub mod issuing;
#[path = "api/mandates.rs"]
pub mod mandates;
#[path = "api/payment_attempt_records.rs"]
pub mod payment_attempt_records;
#[path = "api/payment_intents.rs"]
pub mod payment_intents;
#[path = "api/payment_links.rs"]
pub mod payment_links;
#[path = "api/payment_method_configurations.rs"]
pub mod payment_method_configurations;
#[path = "api/payment_method_domains.rs"]
pub mod payment_method_domains;
#[path = "api/payment_methods.rs"]
pub mod payment_methods;
#[path = "api/payment_records.rs"]
pub mod payment_records;
#[path = "api/payouts.rs"]
pub mod payouts;
#[path = "api/plans.rs"]
pub mod plans;
#[path = "api/prices.rs"]
pub mod prices;
#[path = "api/product_catalog.rs"]
pub mod product_catalog;
#[path = "api/products.rs"]
pub mod products;
#[path = "api/promotion_codes.rs"]
pub mod promotion_codes;
#[path = "api/quotes.rs"]
pub mod quotes;
#[path = "api/radar.rs"]
pub mod radar;
#[path = "api/refunds.rs"]
pub mod refunds;
#[path = "api/reporting.rs"]
pub mod reporting;
#[path = "api/reviews.rs"]
pub mod reviews;
#[path = "api/setup_attempts.rs"]
pub mod setup_attempts;
#[path = "api/setup_intents.rs"]
pub mod setup_intents;
#[path = "api/shipping_rates.rs"]
pub mod shipping_rates;
#[path = "api/sigma.rs"]
pub mod sigma;
#[path = "api/sources.rs"]
pub mod sources;
#[path = "api/subscription_items.rs"]
pub mod subscription_items;
#[path = "api/subscription_schedules.rs"]
pub mod subscription_schedules;
#[path = "api/subscriptions.rs"]
pub mod subscriptions;
#[path = "api/tax.rs"]
pub mod tax;
#[path = "api/tax_codes.rs"]
pub mod tax_codes;
#[path = "api/tax_ids.rs"]
pub mod tax_ids;
#[path = "api/tax_rates.rs"]
pub mod tax_rates;
#[path = "api/terminal.rs"]
pub mod terminal;
#[path = "api/test_helpers.rs"]
pub mod test_helpers;
#[path = "api/three_d_secure.rs"]
pub mod three_d_secure;
#[path = "api/tokens.rs"]
pub mod tokens;
#[path = "api/topups.rs"]
pub mod topups;
#[path = "api/transfers.rs"]
pub mod transfers;
#[path = "api/treasury.rs"]
pub mod treasury;
#[path = "api/v2_billing.rs"]
pub mod v2_billing;
#[path = "api/v2_commerce.rs"]
pub mod v2_commerce;
#[path = "api/v2_core.rs"]
pub mod v2_core;
#[path = "api/webhook_endpoints.rs"]
pub mod webhook_endpoints;
// endregion: generated modules
