#[cfg(feature = "payouts")]
#[doc = "Cancel a payout"]
pub async fn cancel_payout(
    client: &crate::Client,
    payout: &str,
    params: &stripe_types_05::payouts::CancelPayoutParams,
) -> Result<stripe_types_14::payouts::Payout, crate::Error> {
    let path = crate::params::format_path("/v1/payouts/{payout}/cancel", &[("payout", payout)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payouts")]
#[doc = "Create a payout"]
pub async fn create_payout(
    client: &crate::Client,
    params: &stripe_types_05::payouts::CreatePayoutParams,
) -> Result<stripe_types_14::payouts::Payout, crate::Error> {
    let path = crate::params::format_path("/v1/payouts", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payouts")]
#[doc = "Retrieve a payout"]
pub async fn get_payout(
    client: &crate::Client,
    payout: &str,
    params: &stripe_types_02::payouts::GetPayoutParams,
) -> Result<stripe_types_14::payouts::Payout, crate::Error> {
    let path = crate::params::format_path("/v1/payouts/{payout}", &[("payout", payout)]);
    client.get(&path, params).await
}

#[cfg(feature = "payouts")]
#[doc = "List all payouts"]
pub async fn list_payouts(
    client: &crate::Client,
    params: &stripe_types_07::payouts::ListPayoutsParams,
) -> Result<crate::List<stripe_types_14::payouts::Payout>, crate::Error> {
    let path = crate::params::format_path("/v1/payouts", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "payouts")]
#[doc = "Reverse a payout"]
pub async fn reverse_payout(
    client: &crate::Client,
    payout: &str,
    params: &stripe_types_05::payouts::ReversePayoutParams,
) -> Result<stripe_types_14::payouts::Payout, crate::Error> {
    let path = crate::params::format_path("/v1/payouts/{payout}/reverse", &[("payout", payout)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payouts")]
#[doc = "Update a payout"]
pub async fn update_payout(
    client: &crate::Client,
    payout: &str,
    params: &stripe_types_04::payouts::UpdatePayoutParams,
) -> Result<stripe_types_14::payouts::Payout, crate::Error> {
    let path = crate::params::format_path("/v1/payouts/{payout}", &[("payout", payout)]);
    client.post_form(&path, params).await
}
