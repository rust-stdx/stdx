#[cfg(feature = "topups")]
#[doc = "Cancel a top-up"]
pub async fn cancel_topup(
    client: &crate::Client,
    topup: &str,
    params: &stripe_types_06::topups::CancelTopupParams,
) -> Result<stripe_types_14::topups::Topup, crate::Error> {
    let path = crate::params::format_path("/v1/topups/{topup}/cancel", &[("topup", topup)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "topups")]
#[doc = "Create a top-up"]
pub async fn create_topup(
    client: &crate::Client,
    params: &stripe_types_10::topups::CreateTopupParams,
) -> Result<stripe_types_14::topups::Topup, crate::Error> {
    let path = crate::params::format_path("/v1/topups", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "topups")]
#[doc = "Retrieve a top-up"]
pub async fn get_topup(
    client: &crate::Client,
    topup: &str,
    params: &stripe_types_01::topups::GetTopupParams,
) -> Result<stripe_types_14::topups::Topup, crate::Error> {
    let path = crate::params::format_path("/v1/topups/{topup}", &[("topup", topup)]);
    client.get(&path, params).await
}

#[cfg(feature = "topups")]
#[doc = "List all top-ups"]
pub async fn list_topups(
    client: &crate::Client,
    params: &stripe_types_07::topups::ListTopupsParams,
) -> Result<crate::List<stripe_types_14::topups::Topup>, crate::Error> {
    let path = crate::params::format_path("/v1/topups", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "topups")]
#[doc = "Update a top-up"]
pub async fn update_topup(
    client: &crate::Client,
    topup: &str,
    params: &stripe_types_00::topups::UpdateTopupParams,
) -> Result<stripe_types_14::topups::Topup, crate::Error> {
    let path = crate::params::format_path("/v1/topups/{topup}", &[("topup", topup)]);
    client.post_form(&path, params).await
}
