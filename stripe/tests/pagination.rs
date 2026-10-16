//! Auto-pagination tests.

mod common;

use common::MockClient;
use serde::Deserialize;
use stripe::PagedItem;

#[derive(Debug, Deserialize)]
struct Customer {
    id: String,
}

impl PagedItem for Customer {
    fn id(&self) -> &str {
        &self.id
    }
}

#[tokio::test]
async fn walks_pages_and_uses_the_cursor() {
    let mock = MockClient::new();
    mock.push_json(
        200,
        r#"{"object":"list","url":"/v1/customers","has_more":true,"data":[{"id":"cus_1"},{"id":"cus_2"}]}"#,
    );
    mock.push_json(
        200,
        r#"{"object":"list","url":"/v1/customers","has_more":false,"data":[{"id":"cus_3"}]}"#,
    );

    let client = common::test_client(mock.clone());
    let mut pager = client.paginate::<Customer>("/v1/customers", "limit=2");

    let first = pager.next_page().await.expect("page 1").expect("some");
    assert_eq!(first.data.len(), 2);
    assert!(first.has_more);

    let second = pager.next_page().await.expect("page 2").expect("some");
    assert_eq!(second.data[0].id, "cus_3");
    assert!(!second.has_more);

    let third = pager.next_page().await.expect("end");
    assert!(third.is_none());
    assert!(pager.is_finished());

    let requests = mock.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].url, "https://api.stripe.test/v1/customers?limit=2");
    assert_eq!(
        requests[1].url,
        "https://api.stripe.test/v1/customers?limit=2&starting_after=cus_2"
    );
}

#[tokio::test]
async fn has_more_with_empty_page_is_an_error() {
    let mock = MockClient::new();
    mock.push_json(200, r#"{"object":"list","has_more":true,"data":[]}"#);
    let client = common::test_client(mock);
    let mut pager = client.paginate::<Customer>("/v1/customers", "");
    let error = pager.next_page().await.expect_err("protocol violation");
    assert!(error.to_string().contains("empty page"));
}

#[tokio::test]
async fn a_stuck_cursor_aborts_instead_of_looping_forever() {
    let mock = MockClient::new();
    // Both pages end with the same id and claim there is more.
    mock.push_json(200, r#"{"has_more":true,"data":[{"id":"cus_1"},{"id":"cus_2"}]}"#);
    mock.push_json(200, r#"{"has_more":true,"data":[{"id":"cus_2"}]}"#);
    let client = common::test_client(mock.clone());
    let mut pager = client.paginate::<Customer>("/v1/customers", "");
    pager.next_page().await.expect("page 1").expect("some");
    let error = pager.next_page().await.expect_err("stuck cursor");
    assert!(error.to_string().contains("cursor did not advance"));
    assert_eq!(mock.request_count(), 2);
}

#[tokio::test]
async fn starting_after_skips_forward() {
    let mock = MockClient::new();
    mock.push_json(200, r#"{"has_more":false,"data":[{"id":"cus_9"}]}"#);
    let client = common::test_client(mock.clone());
    let mut pager = client.paginate::<Customer>("/v1/customers", "").starting_after("cus_8");
    pager.next_page().await.expect("page");
    assert_eq!(
        mock.last_request().url,
        "https://api.stripe.test/v1/customers?starting_after=cus_8"
    );
}

#[tokio::test]
async fn cursor_values_are_percent_encoded() {
    let mock = MockClient::new();
    mock.push_json(200, r#"{"has_more":false,"data":[{"id":"x"}]}"#);
    let client = common::test_client(mock.clone());
    let mut pager = client.paginate::<Customer>("/v1/customers", "").starting_after("a b&c");
    pager.next_page().await.expect("page");
    assert_eq!(
        mock.last_request().url,
        "https://api.stripe.test/v1/customers?starting_after=a%20b%26c"
    );
}

#[tokio::test]
async fn v2_pages_follow_next_page_url() {
    let mock = MockClient::new();
    // A v2 list has no `has_more`; it pages by URL.
    mock.push_json(200, r#"{"data":[{"id":"cus_1"}],"next_page_url":"/v2/core/customers?page=2"}"#);
    mock.push_json(200, r#"{"data":[{"id":"cus_2"}],"next_page_url":null}"#);

    let client = common::test_client(mock.clone());
    let mut pager = client.paginate::<Customer>("/v2/core/customers", "limit=1");

    let first = pager.next_page().await.expect("page 1").expect("some");
    assert_eq!(first.data[0].id, "cus_1");
    assert!(!first.has_more, "v2 lists do not report has_more");
    assert!(!pager.is_finished(), "a next_page_url means more pages");

    let second = pager.next_page().await.expect("page 2").expect("some");
    assert_eq!(second.data[0].id, "cus_2");
    assert!(pager.is_finished());

    let requests = mock.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[1].url, "https://api.stripe.test/v2/core/customers?page=2");
}

#[tokio::test]
async fn v2_next_page_url_must_stay_on_the_configured_host() {
    let mock = MockClient::new();
    mock.push_json(
        200,
        r#"{"data":[{"id":"cus_1"}],"next_page_url":"https://evil.example/v2/core/customers"}"#,
    );
    let client = common::test_client(mock.clone());
    let mut pager = client.paginate::<Customer>("/v2/core/customers", "");
    pager.next_page().await.expect("page 1").expect("some");
    let error = pager.next_page().await.expect_err("must refuse");
    assert!(error.to_string().contains("outside the configured base URL"), "{error}");
    assert_eq!(mock.request_count(), 1, "the request must not be sent");
}

#[tokio::test]
async fn v2_next_page_url_must_advance() {
    let mock = MockClient::new();
    mock.push_json(200, r#"{"data":[{"id":"cus_1"}],"next_page_url":"/v2/core/customers?page=2"}"#);
    mock.push_json(200, r#"{"data":[{"id":"cus_2"}],"next_page_url":"/v2/core/customers?page=2"}"#);
    let client = common::test_client(mock.clone());
    let mut pager = client.paginate::<Customer>("/v2/core/customers", "");
    pager.next_page().await.expect("page 1").expect("some");
    let error = pager.next_page().await.expect_err("stuck url");
    assert!(error.to_string().contains("next_page_url did not advance"), "{error}");
    assert_eq!(mock.request_count(), 2);
}
