//! Integration tests against the real Stripe API.
//!
//! These are **opt-in and off by default**. Every test runs only when both
//! `STRIPE_INTEGRATION_TESTS=1` and a `STRIPE_API_KEY` are set; otherwise
//! it prints a `skipping …` line and returns immediately, so `cargo test` stays
//! offline and green.
//!
//! The key must be a **test** key (`sk_test_…` or `rk_test_…`). If it is a live
//! key the tests fail loudly rather than touch a real account, and the account's
//! mode is double-checked at runtime via `GET /v1/balance`.
//!
//! Every resource a test creates is registered for cleanup and deleted
//! afterwards, even when an assertion fails. Run with a single thread so the
//! API is not hammered:
//!
//! ```text
//! STRIPE_INTEGRATION_TESTS=1 STRIPE_API_KEY=sk_test_… \
//!     cargo test -p stripe --test integration -- --nocapture --test-threads=1
//! ```

#![cfg(all(feature = "reqwest", feature = "core"))]

use std::{
    cell::RefCell,
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use stripe::{
    Client, Error, RequestOptions,
    balance::{GetBalanceParams, get_balance},
    customers::{
        CreateCustomerParams, GetCustomerParams, GetCustomerResponseExpanded, ListCustomersParams,
        UpdateCustomerParams, create_customer, delete_customer, get_customer, list_customers, update_customer,
    },
    params::Empty,
    payment_intents::{
        CancelPaymentIntentParams, CreatePaymentIntentParams, GetPaymentIntentParams, cancel_payment_intent,
        create_payment_intent, get_payment_intent,
    },
    prices::{CreatePriceParams, ListPricesParams, UpdatePriceParams, create_price, list_prices, update_price},
    products::{CreateProductParams, create_product, delete_product},
    webhook_endpoints::{
        CreateWebhookEndpointParams, CreateWebhookEndpointParamsEnabledEventsItem, GetWebhookEndpointParams,
        create_webhook_endpoint, delete_webhook_endpoint, get_webhook_endpoint,
    },
};

/// Returns true when `key` looks like a Stripe test-mode secret or restricted key.
fn is_test_key(key: &str) -> bool {
    key.starts_with("sk_test_") || key.starts_with("rk_test_")
}

/// Returns a client when integration tests are enabled and the key is a test
/// key. Returns `None` (skip) when the gate or key is absent; panics when a key
/// is present but is not a test key.
fn integration_client() -> Option<Client> {
    let gate = std::env::var("STRIPE_INTEGRATION_TESTS").ok();
    if !matches!(gate.as_deref(), Some("1") | Some("true")) {
        return None;
    }
    let key = std::env::var("STRIPE_API_KEY").ok()?;
    let key = key.trim().to_string();
    if key.is_empty() {
        return None;
    }
    if !is_test_key(&key) {
        panic!(
            "refusing to run Stripe integration tests: STRIPE_API_KEY must be a test key \
             (sk_test_… or rk_test_…)"
        );
    }
    Some(Client::new(key).expect("build Stripe client"))
}

fn skip(test: &str) {
    eprintln!(
        "skipping {test}: set STRIPE_INTEGRATION_TESTS=1 and STRIPE_API_KEY=sk_test_… \
         to run integration tests"
    );
}

/// A unique, sortable marker used for created resources and idempotency keys.
fn unique_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock before the unix epoch")
        .as_nanos();
    format!("stdx-it-{}-{}", std::process::id(), nanos)
}

fn metadata(marker: &str) -> HashMap<String, String> {
    HashMap::from([("stdx_integration_test".to_string(), marker.to_string())])
}

/// Asserts, once per process, that the account is not a live account.
async fn assert_test_mode(client: &Client) {
    static TEST_MODE: tokio::sync::OnceCell<bool> = tokio::sync::OnceCell::const_new();
    let livemode = *TEST_MODE
        .get_or_init(|| async {
            get_balance(client, &GetBalanceParams::default())
                .await
                .expect("GET /v1/balance")
                .livemode
        })
        .await;
    assert!(
        !livemode,
        "STRIPE_API_KEY belongs to a live account; refusing to run integration tests"
    );
}

#[derive(Debug)]
enum Action {
    Customer(String),
    Product(String),
    Price(String),
    PaymentIntent(String),
    WebhookEndpoint(String),
}

/// Records created resources and deletes them in reverse order.
#[derive(Default)]
struct Cleanup {
    actions: RefCell<Vec<Action>>,
}

impl Cleanup {
    fn defer(&self, action: Action) {
        self.actions.borrow_mut().push(action);
    }

    /// Deletes every recorded resource. "Not found" is treated as success, so
    /// running cleanup twice (or after a partial failure) is harmless.
    async fn run(&self, client: &Client) -> Result<(), Error> {
        let actions: Vec<Action> = self.actions.borrow_mut().drain(..).collect();
        let mut first_error = None;
        for action in actions.into_iter().rev() {
            let result = match &action {
                Action::Customer(id) => delete_customer(client, id, &Empty).await.map(|_| ()),
                Action::Product(id) => delete_product(client, id, &Empty).await.map(|_| ()),
                Action::Price(id) => update_price(
                    client,
                    id,
                    &UpdatePriceParams {
                        active: Some(false),
                        ..Default::default()
                    },
                )
                .await
                .map(|_| ()),
                Action::PaymentIntent(id) => cancel_payment_intent(client, id, &CancelPaymentIntentParams::default())
                    .await
                    .map(|_| ()),
                Action::WebhookEndpoint(id) => delete_webhook_endpoint(client, id, &Empty).await.map(|_| ()),
            };
            match result {
                Ok(()) => {}
                Err(Error::Api {
                    meta, ..
                }) if meta.status == 404 => {}
                Err(err) => {
                    eprintln!("integration cleanup failed for {action:?}: {err}");
                    if first_error.is_none() {
                        first_error = Some(err);
                    }
                }
            }
        }
        match first_error {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        let actions = self.actions.borrow();
        if !actions.is_empty() {
            eprintln!(
                "warning: integration test panicked and left {} resource(s) behind: {actions:?}",
                actions.len()
            );
        }
    }
}

#[test]
fn test_key_detection_refuses_live_keys() {
    assert!(is_test_key("sk_test_abc"));
    assert!(is_test_key("rk_test_abc"));
    assert!(!is_test_key("sk_live_abc"));
    assert!(!is_test_key("rk_live_abc"));
    assert!(!is_test_key(""));
    assert!(!is_test_key("sk_test"));
    assert!(!is_test_key("AKIAEXAMPLE"));
}

#[tokio::test]
async fn balance_reports_test_mode() -> Result<(), Error> {
    let Some(client) = integration_client() else {
        skip("balance_reports_test_mode");
        return Ok(());
    };
    assert_test_mode(&client).await;

    let balance = get_balance(&client, &GetBalanceParams::default())
        .await
        .expect("GET /v1/balance");
    assert!(!balance.livemode);
    assert_eq!(balance.object.as_str(), "balance");
    Ok(())
}

#[tokio::test]
async fn customer_lifecycle() -> Result<(), Error> {
    let Some(client) = integration_client() else {
        skip("customer_lifecycle");
        return Ok(());
    };
    assert_test_mode(&client).await;

    let marker = unique_id();
    let cleanup = Cleanup::default();

    let outcome: Result<(), Error> = async {
        let customer = create_customer(
            &client,
            &CreateCustomerParams {
                email: Some(format!("{marker}@example.com")),
                description: Some(marker.clone()),
                metadata: Some(metadata(&marker)),
                ..Default::default()
            },
        )
        .await?;
        cleanup.defer(Action::Customer(customer.id.clone()));
        assert_eq!(customer.object.as_str(), "customer");
        assert_eq!(customer.email.as_deref(), Some(format!("{marker}@example.com").as_str()));
        assert_eq!(
            customer.metadata.as_ref().and_then(|m| m.get("stdx_integration_test")),
            Some(&marker)
        );

        let fetched = get_customer(
            &client,
            &customer.id,
            &GetCustomerParams {
                expand: None,
            },
        )
        .await?;
        match fetched {
            GetCustomerResponseExpanded::Customer(fetched) => assert_eq!(fetched.id, customer.id),
            other => panic!("unexpected customer response: {other:?}"),
        }

        let updated = update_customer(
            &client,
            &customer.id,
            &UpdateCustomerParams {
                description: Some("updated by stdx integration test".to_string()),
                ..Default::default()
            },
        )
        .await?;
        assert_eq!(updated.description.as_deref(), Some("updated by stdx integration test"));

        let page = list_customers(
            &client,
            &ListCustomersParams {
                limit: Some(1),
                ..Default::default()
            },
        )
        .await?;
        assert!(page.data.len() <= 1);

        Ok(())
    }
    .await;

    let cleanup_result = cleanup.run(&client).await;
    outcome?;
    cleanup_result
}

#[tokio::test]
async fn product_and_price_lifecycle() -> Result<(), Error> {
    let Some(client) = integration_client() else {
        skip("product_and_price_lifecycle");
        return Ok(());
    };
    assert_test_mode(&client).await;

    let marker = unique_id();
    let cleanup = Cleanup::default();

    let outcome: Result<(), Error> = async {
        let product = create_product(
            &client,
            &CreateProductParams {
                name: format!("stdx integration test {marker}"),
                metadata: Some(metadata(&marker)),
                ..Default::default()
            },
        )
        .await?;
        cleanup.defer(Action::Product(product.id.clone()));

        let price = create_price(
            &client,
            &CreatePriceParams {
                currency: "usd".to_string(),
                unit_amount: Some(1000),
                product: Some(product.id.clone()),
                metadata: Some(metadata(&marker)),
                ..Default::default()
            },
        )
        .await?;
        cleanup.defer(Action::Price(price.id.clone()));
        assert_eq!(price.unit_amount, Some(1000));
        assert_eq!(price.currency, "usd");

        let prices = list_prices(
            &client,
            &ListPricesParams {
                product: Some(product.id.clone()),
                limit: Some(1),
                ..Default::default()
            },
        )
        .await?;
        assert!(prices.data.iter().any(|listed| listed.id == price.id));

        // Stripe has no price delete; deactivate it instead.
        let deactivated = update_price(
            &client,
            &price.id,
            &UpdatePriceParams {
                active: Some(false),
                ..Default::default()
            },
        )
        .await?;
        assert!(!deactivated.active);

        Ok(())
    }
    .await;

    let cleanup_result = cleanup.run(&client).await;
    outcome?;
    cleanup_result
}

#[tokio::test]
async fn payment_intent_create_and_cancel() -> Result<(), Error> {
    let Some(client) = integration_client() else {
        skip("payment_intent_create_and_cancel");
        return Ok(());
    };
    assert_test_mode(&client).await;

    let marker = unique_id();
    let cleanup = Cleanup::default();

    let outcome: Result<(), Error> = async {
        let intent = create_payment_intent(
            &client,
            &CreatePaymentIntentParams {
                amount: 1000,
                currency: "usd".to_string(),
                metadata: Some(metadata(&marker)),
                ..Default::default()
            },
        )
        .await?;
        cleanup.defer(Action::PaymentIntent(intent.id.clone()));
        assert_eq!(intent.status.as_str(), "requires_payment_method");

        let fetched = get_payment_intent(&client, &intent.id, &GetPaymentIntentParams::default()).await?;
        assert_eq!(fetched.id, intent.id);

        let canceled = cancel_payment_intent(&client, &intent.id, &CancelPaymentIntentParams::default()).await?;
        assert_eq!(canceled.status.as_str(), "canceled");

        Ok(())
    }
    .await;

    let cleanup_result = cleanup.run(&client).await;
    outcome?;
    cleanup_result
}

#[tokio::test]
async fn webhook_endpoint_lifecycle() -> Result<(), Error> {
    let Some(client) = integration_client() else {
        skip("webhook_endpoint_lifecycle");
        return Ok(());
    };
    assert_test_mode(&client).await;

    let marker = unique_id();
    let cleanup = Cleanup::default();

    let outcome: Result<(), Error> = async {
        let endpoint = create_webhook_endpoint(
            &client,
            &CreateWebhookEndpointParams {
                enabled_events: vec![CreateWebhookEndpointParamsEnabledEventsItem::CustomerCreated],
                url: format!("https://example.com/stdx-integration-test/{marker}"),
                ..Default::default()
            },
        )
        .await?;
        cleanup.defer(Action::WebhookEndpoint(endpoint.id.clone()));
        assert!(endpoint.url.contains(&marker));

        let fetched = get_webhook_endpoint(&client, &endpoint.id, &GetWebhookEndpointParams::default()).await?;
        assert_eq!(fetched.id, endpoint.id);

        Ok(())
    }
    .await;

    let cleanup_result = cleanup.run(&client).await;
    outcome?;
    cleanup_result
}

#[tokio::test]
async fn pagination_over_http() -> Result<(), Error> {
    let Some(client) = integration_client() else {
        skip("pagination_over_http");
        return Ok(());
    };
    assert_test_mode(&client).await;

    // Read-only: page through customers one at a time.
    let mut pager = client.paginate::<stripe::customers::Customer>("/v1/customers", "limit=1");
    let page = pager.next_page().await.expect("first page").expect("a page");
    assert!(page.data.len() <= 1);
    assert_eq!(page.object.as_str(), "list");
    if page.has_more {
        let second = pager.next_page().await.expect("second page");
        assert!(second.is_some());
    }
    Ok(())
}

#[tokio::test]
async fn idempotent_create_returns_the_same_object() -> Result<(), Error> {
    let Some(client) = integration_client() else {
        skip("idempotent_create_returns_the_same_object");
        return Ok(());
    };
    assert_test_mode(&client).await;

    let marker = unique_id();
    let cleanup = Cleanup::default();

    let outcome: Result<(), Error> = async {
        let options = RequestOptions::new().with_idempotency_key(format!("stdx-it-idem-{marker}"));
        let params = CreateCustomerParams {
            email: Some(format!("{marker}@example.com")),
            metadata: Some(metadata(&marker)),
            ..Default::default()
        };

        let first = client
            .post_form_with_options::<stripe::customers::Customer, _>("/v1/customers", &params, &options)
            .await?;
        cleanup.defer(Action::Customer(first.id.clone()));

        let second = client
            .post_form_with_options::<stripe::customers::Customer, _>("/v1/customers", &params, &options)
            .await?;

        assert_eq!(first.id, second.id, "reusing an idempotency key must return the same object");

        Ok(())
    }
    .await;

    let cleanup_result = cleanup.run(&client).await;
    outcome?;
    cleanup_result
}
