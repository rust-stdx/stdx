#[cfg(feature = "forwarding")]
#[doc = "Create a ForwardingRequest"]
pub async fn create_forwarding_request(
    client: &crate::Client,
    params: &stripe_types_09::forwarding::CreateForwardingRequestParams,
) -> Result<stripe_types_08::forwarding::ForwardingRequest, crate::Error> {
    let path = crate::params::format_path("/v1/forwarding/requests", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "forwarding")]
#[doc = "Retrieve a ForwardingRequest"]
pub async fn get_forwarding_request(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_03::forwarding::GetForwardingRequestParams,
) -> Result<stripe_types_08::forwarding::ForwardingRequest, crate::Error> {
    let path = crate::params::format_path("/v1/forwarding/requests/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "forwarding")]
#[doc = "List all ForwardingRequests"]
pub async fn list_forwarding_requests(
    client: &crate::Client,
    params: &stripe_types_05::forwarding::ListForwardingRequestsParams,
) -> Result<crate::List<stripe_types_08::forwarding::ForwardingRequest>, crate::Error> {
    let path = crate::params::format_path("/v1/forwarding/requests", &[]);
    client.get(&path, params).await
}
