#[cfg(feature = "webhook_endpoints")]
#[doc = "Create a webhook endpoint"]
pub async fn create_webhook_endpoint(
    client: &crate::Client,
    params: &stripe_types_06::webhook_endpoints::CreateWebhookEndpointParams,
) -> Result<stripe_types_06::webhook_endpoints::WebhookEndpoint, crate::Error> {
    let path = crate::params::format_path("/v1/webhook_endpoints", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "webhook_endpoints")]
#[doc = "Delete a webhook endpoint"]
pub async fn delete_webhook_endpoint(
    client: &crate::Client,
    webhook_endpoint: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_05::webhook_endpoints::DeletedWebhookEndpoint, crate::Error> {
    let path = crate::params::format_path(
        "/v1/webhook_endpoints/{webhook_endpoint}",
        &[("webhook_endpoint", webhook_endpoint)],
    );
    client.delete_form(&path, params).await
}

#[cfg(feature = "webhook_endpoints")]
#[doc = "Retrieve a webhook endpoint"]
pub async fn get_webhook_endpoint(
    client: &crate::Client,
    webhook_endpoint: &str,
    params: &stripe_types_02::webhook_endpoints::GetWebhookEndpointParams,
) -> Result<stripe_types_06::webhook_endpoints::WebhookEndpoint, crate::Error> {
    let path = crate::params::format_path(
        "/v1/webhook_endpoints/{webhook_endpoint}",
        &[("webhook_endpoint", webhook_endpoint)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "webhook_endpoints")]
#[doc = "List all webhook endpoints"]
pub async fn list_webhook_endpoints(
    client: &crate::Client,
    params: &stripe_types_03::webhook_endpoints::ListWebhookEndpointsParams,
) -> Result<crate::List<stripe_types_06::webhook_endpoints::WebhookEndpoint>, crate::Error> {
    let path = crate::params::format_path("/v1/webhook_endpoints", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "webhook_endpoints")]
#[doc = "Update a webhook endpoint"]
pub async fn update_webhook_endpoint(
    client: &crate::Client,
    webhook_endpoint: &str,
    params: &stripe_types_08::webhook_endpoints::UpdateWebhookEndpointParams,
) -> Result<stripe_types_06::webhook_endpoints::WebhookEndpoint, crate::Error> {
    let path = crate::params::format_path(
        "/v1/webhook_endpoints/{webhook_endpoint}",
        &[("webhook_endpoint", webhook_endpoint)],
    );
    client.post_form(&path, params).await
}
