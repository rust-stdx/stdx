#[cfg(feature = "customer_sessions")]
#[doc = "Create a Customer Session"]
pub async fn create_customer_session(
    client: &crate::Client,
    params: &stripe_types_13::customer_sessions::CreateCustomerSessionParams,
) -> Result<stripe_types_15::customer_sessions::CustomerSession, crate::Error> {
    let path = crate::params::format_path("/v1/customer_sessions", &[]);
    client.post_form(&path, params).await
}
