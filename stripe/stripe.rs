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
//! The generated types live in internal `stripe_types_*` crates and are
//! re-exported here, so the public paths are still `stripe::customers::...`.
//!
//! ```no_run
//! # #[cfg(all(feature = "customers", feature = "checkout"))]
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
//! Examples are gated on the features they use; enable `core` (or individual
//! modules) and, for the bundled transport, `reqwest`. The crate enables
//! nothing by default, so a build only contains the API areas it asks for.
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
//! # #[cfg(feature = "customers")]
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
//! # #[cfg(feature = "customers")]
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
//! builders and methods in that module. No feature is enabled by default:
//! enable `reqwest` for the built-in HTTP transport and `core`, an umbrella over
//! the resources a SaaS or online store needs. `core` covers:
//!
//! * selling and the customer portal: [`checkout`], [`billing_portal`],
//!   [`prices`], [`products`], invoices with
//!   [`payment_intents`]/[`payment_methods`]/[`setup_intents`];
//! * subscriptions, including metered usage: [`subscriptions`],
//!   [`subscription_items`], [`subscription_schedules`], and the Meters API in
//!   [`billing`] (`create_billing_meter`, `create_billing_meter_event`, ...).
//!   Usage-based prices are created through [`prices`] with
//!   `recurring.usage_type = metered` and `recurring.meter`;
//! * billing: [`invoices`], [`invoiceitems`], [`credit_notes`], [`tax_rates`],
//!   [`tax_ids`], [`tax_codes`];
//! * payments: [`customers`], [`charges`], [`refunds`], [`disputes`],
//!   [`mandates`], [`confirmation_tokens`], [`coupons`], [`promotion_codes`];
//! * webhooks: [`webhook_endpoints`].
//!
//! Add what your product needs on top of that:
//!
//! * **`events`** — typed webhook payloads (`Event`, `customer.subscription.*`,
//!   `invoice.*`, `checkout.session.*`, ...). This is the largest optional
//!   feature: it adds roughly 10 seconds to a `cargo check`. If you handle only
//!   a few event types, decoding them with your own types (or
//!   [`Webhook::parse_event`] into `json::OwnedValue`) is cheaper.
//! * `entitlements` — plan and feature gating for the account's customers.
//! * `test_helpers` — Stripe's test-mode helpers (test clocks, funding, ...).
//! * `v2_billing` — the v2 meter event stream, for usage reported at high volume.
//! * `shipping_rates` and `tax` — physical goods and Stripe Tax.
//! * `accounts`, `account_links`, `account_sessions`, `payouts`, `transfers`,
//!   `application_fees` — Connect platforms and marketplaces.
//! * `apple_pay`, `customer_sessions`, `ephemeral_keys` — mobile SDK flows.
//! * `plans`, `quotes`, `payment_links`, `sources`, `tokens`, `three_d_secure`,
//!   `files`, the `balance*` and `payment_record*` APIs — legacy, sales,
//!   reconciliation and specialised APIs kept out of `core`.
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
//! With the `reqwest` feature enabled, `Client::new_reqwest(reqwest::Client::new(),
//! "sk_test_…")` builds a client using the bundled transport (which pools
//! connections and installs the stdx rustls provider). Pass any [`HttpClient`]
//! implementation to [`Client::with_http_client`] to use a different transport:
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
//! The `types/` and `api/` trees, their manifests, and the module list below are produced by the `stripe_codegen`
//! binary from the vendored specification at `stripe/spec/openapi.spec3.sdk.json`:
//!
//! ```text
//! make -C stripe codegen         # regenerate in place
//! make -C stripe codegen_check   # fail if the committed code is stale
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
//! double-checked at runtime with `GET /v1/balance` (`livemode` must be false),
//! which is why the test target requires the `balance` feature. Every object a
//! test creates is deleted afterwards, even when an assertion fails; a resource
//! left behind by a panic is reported on stderr. A restricted test key needs
//! read/write access to balance, customers, products, prices, payment intents
//! and webhook endpoints.
//!
//! ```text
//! STRIPE_INTEGRATION_TESTS=1 STRIPE_API_KEY=sk_test_… \
//!     cargo test -p stripe --features balance --test integration -- --nocapture --test-threads=1
//! # or
//! STRIPE_INTEGRATION_TESTS=1 STRIPE_API_KEY=sk_test_… make -C stripe integration_test
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

// region: generated modules (do not edit; `make -C stripe codegen`)

pub use stripe_core::*;

pub mod account {
    #[allow(unused_imports)]
    pub use stripe_api::account::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::account::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::account::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::account::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::account::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::account::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::account::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::account::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::account::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::account::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::account::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::account::*;
}

pub mod account_links {
    #[allow(unused_imports)]
    pub use stripe_api::account_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::account_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::account_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::account_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::account_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::account_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::account_links::*;
}

pub mod account_sessions {
    #[allow(unused_imports)]
    pub use stripe_api::account_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::account_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::account_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::account_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::account_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::account_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::account_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::account_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::account_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::account_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::account_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::account_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::account_sessions::*;
}

pub mod accounts {
    #[allow(unused_imports)]
    pub use stripe_api::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::accounts::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::accounts::*;
}

pub mod apple_pay {
    #[allow(unused_imports)]
    pub use stripe_api::apple_pay::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::apple_pay::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::apple_pay::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::apple_pay::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::apple_pay::*;
}

pub mod application_fees {
    #[allow(unused_imports)]
    pub use stripe_api::application_fees::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::application_fees::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::application_fees::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::application_fees::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::application_fees::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::application_fees::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::application_fees::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::application_fees::*;
}

pub mod apps {
    #[allow(unused_imports)]
    pub use stripe_api::apps::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::apps::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::apps::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::apps::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::apps::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::apps::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::apps::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::apps::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::apps::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::apps::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::apps::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::apps::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::apps::*;
}

pub mod balance {
    #[allow(unused_imports)]
    pub use stripe_api::balance::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::balance::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::balance::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::balance::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::balance::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::balance::*;
}

pub mod balance_settings {
    #[allow(unused_imports)]
    pub use stripe_api::balance_settings::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::balance_settings::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::balance_settings::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::balance_settings::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::balance_settings::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::balance_settings::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::balance_settings::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::balance_settings::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::balance_settings::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::balance_settings::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::balance_settings::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::balance_settings::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::balance_settings::*;
}

pub mod balance_transactions {
    #[allow(unused_imports)]
    pub use stripe_api::balance_transactions::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::balance_transactions::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::balance_transactions::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::balance_transactions::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::balance_transactions::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::balance_transactions::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::balance_transactions::*;
}

pub mod billing {
    #[allow(unused_imports)]
    pub use stripe_api::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::billing::*;
}

pub mod billing_portal {
    #[allow(unused_imports)]
    pub use stripe_api::billing_portal::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::billing_portal::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::billing_portal::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::billing_portal::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::billing_portal::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::billing_portal::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::billing_portal::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::billing_portal::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::billing_portal::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::billing_portal::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::billing_portal::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::billing_portal::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::billing_portal::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::billing_portal::*;
}

pub mod charges {
    #[allow(unused_imports)]
    pub use stripe_api::charges::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::charges::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::charges::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::charges::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::charges::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::charges::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::charges::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::charges::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::charges::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::charges::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::charges::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::charges::*;
}

pub mod checkout {
    #[allow(unused_imports)]
    pub use stripe_api::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::checkout::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::checkout::*;
}

pub mod climate {
    #[allow(unused_imports)]
    pub use stripe_api::climate::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::climate::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::climate::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::climate::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::climate::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::climate::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::climate::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::climate::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::climate::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::climate::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::climate::*;
}

pub mod common {
    #[allow(unused_imports)]
    pub use stripe_types_00::common::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::common::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::common::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::common::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::common::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::common::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::common::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::common::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::common::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::common::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::common::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::common::*;
}

pub mod confirmation_tokens {
    #[allow(unused_imports)]
    pub use stripe_api::confirmation_tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::confirmation_tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::confirmation_tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::confirmation_tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::confirmation_tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::confirmation_tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::confirmation_tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::confirmation_tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::confirmation_tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::confirmation_tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::confirmation_tokens::*;
}

pub mod country_specs {
    #[allow(unused_imports)]
    pub use stripe_api::country_specs::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::country_specs::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::country_specs::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::country_specs::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::country_specs::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::country_specs::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::country_specs::*;
}

pub mod coupons {
    #[allow(unused_imports)]
    pub use stripe_api::coupons::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::coupons::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::coupons::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::coupons::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::coupons::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::coupons::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::coupons::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::coupons::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::coupons::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::coupons::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::coupons::*;
}

pub mod credit_notes {
    #[allow(unused_imports)]
    pub use stripe_api::credit_notes::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::credit_notes::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::credit_notes::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::credit_notes::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::credit_notes::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::credit_notes::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::credit_notes::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::credit_notes::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::credit_notes::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::credit_notes::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::credit_notes::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::credit_notes::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::credit_notes::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::credit_notes::*;
}

pub mod customer_sessions {
    #[allow(unused_imports)]
    pub use stripe_api::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::customer_sessions::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::customer_sessions::*;
}

pub mod customers {
    #[allow(unused_imports)]
    pub use stripe_api::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::customers::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::customers::*;
}

pub mod disputes {
    #[allow(unused_imports)]
    pub use stripe_api::disputes::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::disputes::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::disputes::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::disputes::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::disputes::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::disputes::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::disputes::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::disputes::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::disputes::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::disputes::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::disputes::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::disputes::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::disputes::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::disputes::*;
}

pub mod entitlements {
    #[allow(unused_imports)]
    pub use stripe_api::entitlements::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::entitlements::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::entitlements::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::entitlements::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::entitlements::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::entitlements::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::entitlements::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::entitlements::*;
}

pub mod ephemeral_keys {
    #[allow(unused_imports)]
    pub use stripe_api::ephemeral_keys::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::ephemeral_keys::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::ephemeral_keys::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::ephemeral_keys::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::ephemeral_keys::*;
}

#[cfg(feature = "events")]
pub mod events {
    #[allow(unused_imports)]
    pub use stripe_api::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::events::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::events::*;
}

pub mod exchange_rates {
    #[allow(unused_imports)]
    pub use stripe_api::exchange_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::exchange_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::exchange_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::exchange_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::exchange_rates::*;
}

pub mod file_links {
    #[allow(unused_imports)]
    pub use stripe_api::file_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::file_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::file_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::file_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::file_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::file_links::*;
}

pub mod files {
    #[allow(unused_imports)]
    pub use stripe_api::files::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::files::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::files::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::files::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::files::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::files::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::files::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::files::*;
}

pub mod financial_connections {
    #[allow(unused_imports)]
    pub use stripe_api::financial_connections::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::financial_connections::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::financial_connections::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::financial_connections::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::financial_connections::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::financial_connections::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::financial_connections::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::financial_connections::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::financial_connections::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::financial_connections::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::financial_connections::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::financial_connections::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::financial_connections::*;
}

pub mod forwarding {
    #[allow(unused_imports)]
    pub use stripe_api::forwarding::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::forwarding::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::forwarding::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::forwarding::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::forwarding::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::forwarding::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::forwarding::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::forwarding::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::forwarding::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::forwarding::*;
}

pub mod identity {
    #[allow(unused_imports)]
    pub use stripe_api::identity::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::identity::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::identity::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::identity::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::identity::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::identity::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::identity::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::identity::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::identity::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::identity::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::identity::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::identity::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::identity::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::identity::*;
}

pub mod invoice_payments {
    #[allow(unused_imports)]
    pub use stripe_api::invoice_payments::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::invoice_payments::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::invoice_payments::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::invoice_payments::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::invoice_payments::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::invoice_payments::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::invoice_payments::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::invoice_payments::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::invoice_payments::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::invoice_payments::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::invoice_payments::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::invoice_payments::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::invoice_payments::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::invoice_payments::*;
}

pub mod invoice_rendering_templates {
    #[allow(unused_imports)]
    pub use stripe_api::invoice_rendering_templates::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::invoice_rendering_templates::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::invoice_rendering_templates::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::invoice_rendering_templates::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::invoice_rendering_templates::*;
}

pub mod invoiceitems {
    #[allow(unused_imports)]
    pub use stripe_api::invoiceitems::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::invoiceitems::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::invoiceitems::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::invoiceitems::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::invoiceitems::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::invoiceitems::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::invoiceitems::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::invoiceitems::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::invoiceitems::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::invoiceitems::*;
}

pub mod invoices {
    #[allow(unused_imports)]
    pub use stripe_api::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::invoices::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::invoices::*;
}

pub mod issuing {
    #[allow(unused_imports)]
    pub use stripe_api::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::issuing::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::issuing::*;
}

pub mod mandates {
    #[allow(unused_imports)]
    pub use stripe_api::mandates::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::mandates::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::mandates::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::mandates::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::mandates::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::mandates::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::mandates::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::mandates::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::mandates::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::mandates::*;
}

pub mod payment_attempt_records {
    #[allow(unused_imports)]
    pub use stripe_api::payment_attempt_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::payment_attempt_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::payment_attempt_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::payment_attempt_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::payment_attempt_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::payment_attempt_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::payment_attempt_records::*;
}

pub mod payment_intents {
    #[allow(unused_imports)]
    pub use stripe_api::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::payment_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::payment_intents::*;
}

pub mod payment_links {
    #[allow(unused_imports)]
    pub use stripe_api::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::payment_links::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::payment_links::*;
}

pub mod payment_method_configurations {
    #[allow(unused_imports)]
    pub use stripe_api::payment_method_configurations::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::payment_method_configurations::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::payment_method_configurations::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::payment_method_configurations::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::payment_method_configurations::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::payment_method_configurations::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::payment_method_configurations::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::payment_method_configurations::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::payment_method_configurations::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::payment_method_configurations::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::payment_method_configurations::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::payment_method_configurations::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::payment_method_configurations::*;
}

pub mod payment_method_domains {
    #[allow(unused_imports)]
    pub use stripe_api::payment_method_domains::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::payment_method_domains::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::payment_method_domains::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::payment_method_domains::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::payment_method_domains::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::payment_method_domains::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::payment_method_domains::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::payment_method_domains::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::payment_method_domains::*;
}

pub mod payment_methods {
    #[allow(unused_imports)]
    pub use stripe_api::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::payment_methods::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::payment_methods::*;
}

pub mod payment_records {
    #[allow(unused_imports)]
    pub use stripe_api::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::payment_records::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::payment_records::*;
}

pub mod payouts {
    #[allow(unused_imports)]
    pub use stripe_api::payouts::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::payouts::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::payouts::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::payouts::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::payouts::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::payouts::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::payouts::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::payouts::*;
}

pub mod plans {
    #[allow(unused_imports)]
    pub use stripe_api::plans::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::plans::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::plans::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::plans::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::plans::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::plans::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::plans::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::plans::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::plans::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::plans::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::plans::*;
}

pub mod prices {
    #[allow(unused_imports)]
    pub use stripe_api::prices::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::prices::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::prices::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::prices::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::prices::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::prices::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::prices::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::prices::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::prices::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::prices::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::prices::*;
}

pub mod product_catalog {
    #[allow(unused_imports)]
    pub use stripe_api::product_catalog::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::product_catalog::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::product_catalog::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::product_catalog::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::product_catalog::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::product_catalog::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::product_catalog::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::product_catalog::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::product_catalog::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::product_catalog::*;
}

pub mod products {
    #[allow(unused_imports)]
    pub use stripe_api::products::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::products::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::products::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::products::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::products::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::products::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::products::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::products::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::products::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::products::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::products::*;
}

pub mod promotion_codes {
    #[allow(unused_imports)]
    pub use stripe_api::promotion_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::promotion_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::promotion_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::promotion_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::promotion_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::promotion_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::promotion_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::promotion_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::promotion_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::promotion_codes::*;
}

pub mod quotes {
    #[allow(unused_imports)]
    pub use stripe_api::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::quotes::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::quotes::*;
}

pub mod radar {
    #[allow(unused_imports)]
    pub use stripe_api::radar::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::radar::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::radar::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::radar::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::radar::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::radar::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::radar::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::radar::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::radar::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::radar::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::radar::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::radar::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::radar::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::radar::*;
}

pub mod refunds {
    #[allow(unused_imports)]
    pub use stripe_api::refunds::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::refunds::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::refunds::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::refunds::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::refunds::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::refunds::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::refunds::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::refunds::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::refunds::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::refunds::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::refunds::*;
}

pub mod reporting {
    #[allow(unused_imports)]
    pub use stripe_api::reporting::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::reporting::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::reporting::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::reporting::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::reporting::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::reporting::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::reporting::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::reporting::*;
}

pub mod reviews {
    #[allow(unused_imports)]
    pub use stripe_api::reviews::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::reviews::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::reviews::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::reviews::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::reviews::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::reviews::*;
}

pub mod setup_attempts {
    #[allow(unused_imports)]
    pub use stripe_api::setup_attempts::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::setup_attempts::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::setup_attempts::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::setup_attempts::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::setup_attempts::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::setup_attempts::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::setup_attempts::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::setup_attempts::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::setup_attempts::*;
}

pub mod setup_intents {
    #[allow(unused_imports)]
    pub use stripe_api::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::setup_intents::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::setup_intents::*;
}

pub mod shipping_rates {
    #[allow(unused_imports)]
    pub use stripe_api::shipping_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::shipping_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::shipping_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::shipping_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::shipping_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::shipping_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::shipping_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::shipping_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::shipping_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::shipping_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::shipping_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::shipping_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::shipping_rates::*;
}

pub mod sigma {
    #[allow(unused_imports)]
    pub use stripe_api::sigma::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::sigma::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::sigma::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::sigma::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::sigma::*;
}

pub mod sources {
    #[allow(unused_imports)]
    pub use stripe_api::sources::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::sources::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::sources::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::sources::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::sources::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::sources::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::sources::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::sources::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::sources::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::sources::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::sources::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::sources::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::sources::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::sources::*;
}

pub mod subscription_items {
    #[allow(unused_imports)]
    pub use stripe_api::subscription_items::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::subscription_items::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::subscription_items::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::subscription_items::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::subscription_items::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::subscription_items::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::subscription_items::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::subscription_items::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::subscription_items::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::subscription_items::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::subscription_items::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::subscription_items::*;
}

pub mod subscription_schedules {
    #[allow(unused_imports)]
    pub use stripe_api::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::subscription_schedules::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::subscription_schedules::*;
}

pub mod subscriptions {
    #[allow(unused_imports)]
    pub use stripe_api::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::subscriptions::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::subscriptions::*;
}

pub mod tax {
    #[allow(unused_imports)]
    pub use stripe_api::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::tax::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::tax::*;
}

pub mod tax_codes {
    #[allow(unused_imports)]
    pub use stripe_api::tax_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::tax_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::tax_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::tax_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::tax_codes::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::tax_codes::*;
}

pub mod tax_ids {
    #[allow(unused_imports)]
    pub use stripe_api::tax_ids::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::tax_ids::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::tax_ids::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::tax_ids::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::tax_ids::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::tax_ids::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::tax_ids::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::tax_ids::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::tax_ids::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::tax_ids::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::tax_ids::*;
}

pub mod tax_rates {
    #[allow(unused_imports)]
    pub use stripe_api::tax_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::tax_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::tax_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::tax_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::tax_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::tax_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::tax_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::tax_rates::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::tax_rates::*;
}

pub mod terminal {
    #[allow(unused_imports)]
    pub use stripe_api::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::terminal::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::terminal::*;
}

pub mod test_helpers {
    #[allow(unused_imports)]
    pub use stripe_api::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::test_helpers::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::test_helpers::*;
}

pub mod three_d_secure {
    #[allow(unused_imports)]
    pub use stripe_api::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::three_d_secure::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::three_d_secure::*;
}

pub mod tokens {
    #[allow(unused_imports)]
    pub use stripe_api::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::tokens::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::tokens::*;
}

pub mod topups {
    #[allow(unused_imports)]
    pub use stripe_api::topups::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::topups::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::topups::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::topups::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::topups::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::topups::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::topups::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::topups::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::topups::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::topups::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::topups::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::topups::*;
}

pub mod transfers {
    #[allow(unused_imports)]
    pub use stripe_api::transfers::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::transfers::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::transfers::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::transfers::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::transfers::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::transfers::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::transfers::*;
    #[allow(unused_imports)]
    pub use stripe_types_14::transfers::*;
}

pub mod treasury {
    #[allow(unused_imports)]
    pub use stripe_api::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::treasury::*;
    #[allow(unused_imports)]
    pub use stripe_types_15::treasury::*;
}

pub mod v2_billing {
    #[allow(unused_imports)]
    pub use stripe_api::v2_billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::v2_billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::v2_billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::v2_billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::v2_billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::v2_billing::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::v2_billing::*;
}

pub mod v2_commerce {
    #[allow(unused_imports)]
    pub use stripe_api::v2_commerce::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::v2_commerce::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::v2_commerce::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::v2_commerce::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::v2_commerce::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::v2_commerce::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::v2_commerce::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::v2_commerce::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::v2_commerce::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::v2_commerce::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::v2_commerce::*;
}

pub mod v2_core {
    #[allow(unused_imports)]
    pub use stripe_api::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_00::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_07::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_09::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_10::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_11::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_12::v2_core::*;
    #[allow(unused_imports)]
    pub use stripe_types_13::v2_core::*;
}

pub mod webhook_endpoints {
    #[allow(unused_imports)]
    pub use stripe_api::webhook_endpoints::*;
    #[allow(unused_imports)]
    pub use stripe_types_01::webhook_endpoints::*;
    #[allow(unused_imports)]
    pub use stripe_types_02::webhook_endpoints::*;
    #[allow(unused_imports)]
    pub use stripe_types_03::webhook_endpoints::*;
    #[allow(unused_imports)]
    pub use stripe_types_04::webhook_endpoints::*;
    #[allow(unused_imports)]
    pub use stripe_types_05::webhook_endpoints::*;
    #[allow(unused_imports)]
    pub use stripe_types_06::webhook_endpoints::*;
    #[allow(unused_imports)]
    pub use stripe_types_08::webhook_endpoints::*;
}

// endregion: generated modules
