#[cfg(feature = "disputes")]
#[doc = "Close a dispute"]
pub async fn close_dispute(
    client: &crate::Client,
    dispute: &str,
    params: &stripe_types_06::disputes::CloseDisputeParams,
) -> Result<stripe_types_14::disputes::Dispute, crate::Error> {
    let path = crate::params::format_path("/v1/disputes/{dispute}/close", &[("dispute", dispute)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "disputes")]
#[doc = "Retrieve a dispute"]
pub async fn get_dispute(
    client: &crate::Client,
    dispute: &str,
    params: &stripe_types_01::disputes::GetDisputeParams,
) -> Result<stripe_types_14::disputes::Dispute, crate::Error> {
    let path = crate::params::format_path("/v1/disputes/{dispute}", &[("dispute", dispute)]);
    client.get(&path, params).await
}

#[cfg(feature = "disputes")]
#[doc = "List all disputes"]
pub async fn list_disputes(
    client: &crate::Client,
    params: &stripe_types_05::disputes::ListDisputesParams,
) -> Result<crate::List<stripe_types_14::disputes::Dispute>, crate::Error> {
    let path = crate::params::format_path("/v1/disputes", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "disputes")]
#[doc = "Update a dispute"]
pub async fn update_dispute(
    client: &crate::Client,
    dispute: &str,
    params: &stripe_types_13::disputes::UpdateDisputeParams,
) -> Result<stripe_types_14::disputes::Dispute, crate::Error> {
    let path = crate::params::format_path("/v1/disputes/{dispute}", &[("dispute", dispute)]);
    client.post_form(&path, params).await
}
