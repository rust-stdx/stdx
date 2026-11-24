#[cfg(feature = "refunds")]
#[doc = "Cancel a refund"]
pub async fn cancel_refund(
    client: &crate::Client,
    refund: &str,
    params: &stripe_types_06::refunds::CancelRefundParams,
) -> Result<stripe_types_14::refunds::Refund, crate::Error> {
    let path = crate::params::format_path("/v1/refunds/{refund}/cancel", &[("refund", refund)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "refunds")]
#[doc = "Create a refund"]
pub async fn create_refund(
    client: &crate::Client,
    params: &stripe_types_06::refunds::CreateRefundParams,
) -> Result<stripe_types_14::refunds::Refund, crate::Error> {
    let path = crate::params::format_path("/v1/refunds", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "refunds")]
#[doc = "Retrieve a refund"]
pub async fn get_refund(
    client: &crate::Client,
    refund: &str,
    params: &stripe_types_03::refunds::GetRefundParams,
) -> Result<stripe_types_14::refunds::Refund, crate::Error> {
    let path = crate::params::format_path("/v1/refunds/{refund}", &[("refund", refund)]);
    client.get(&path, params).await
}

#[cfg(feature = "refunds")]
#[doc = "List all refunds"]
pub async fn list_refunds(
    client: &crate::Client,
    params: &stripe_types_05::refunds::ListRefundsParams,
) -> Result<crate::List<stripe_types_14::refunds::Refund>, crate::Error> {
    let path = crate::params::format_path("/v1/refunds", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "refunds")]
#[doc = "Update a refund"]
pub async fn update_refund(
    client: &crate::Client,
    refund: &str,
    params: &stripe_types_03::refunds::UpdateRefundParams,
) -> Result<stripe_types_14::refunds::Refund, crate::Error> {
    let path = crate::params::format_path("/v1/refunds/{refund}", &[("refund", refund)]);
    client.post_form(&path, params).await
}
