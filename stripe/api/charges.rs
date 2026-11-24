#[cfg(feature = "charges")]
#[doc = "Capture a charge"]
pub async fn capture_charge(
    client: &crate::Client,
    charge: &str,
    params: &stripe_types_05::charges::CaptureChargeParams,
) -> Result<stripe_types_14::charges::Charge, crate::Error> {
    let path = crate::params::format_path("/v1/charges/{charge}/capture", &[("charge", charge)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "charges")]
#[doc = "Create a charge"]
pub async fn create_charge(
    client: &crate::Client,
    params: &stripe_types_09::charges::CreateChargeParams,
) -> Result<stripe_types_14::charges::Charge, crate::Error> {
    let path = crate::params::format_path("/v1/charges", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "charges")]
#[doc = "Retrieve a charge"]
pub async fn get_charge(
    client: &crate::Client,
    charge: &str,
    params: &stripe_types_01::charges::GetChargeParams,
) -> Result<stripe_types_14::charges::Charge, crate::Error> {
    let path = crate::params::format_path("/v1/charges/{charge}", &[("charge", charge)]);
    client.get(&path, params).await
}

#[cfg(feature = "charges")]
#[doc = "<p>Retrieves the details of an existing refund.</p>"]
pub async fn get_charge_refund(
    client: &crate::Client,
    charge: &str,
    refund: &str,
    params: &stripe_types_05::charges::GetChargeRefundParams,
) -> Result<stripe_types_14::refunds::Refund, crate::Error> {
    let path = crate::params::format_path(
        "/v1/charges/{charge}/refunds/{refund}",
        &[("charge", charge), ("refund", refund)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "charges")]
#[doc = "List all refunds"]
pub async fn list_charge_refunds(
    client: &crate::Client,
    charge: &str,
    params: &stripe_types_01::charges::ListChargeRefundsParams,
) -> Result<crate::List<stripe_types_14::refunds::Refund>, crate::Error> {
    let path = crate::params::format_path("/v1/charges/{charge}/refunds", &[("charge", charge)]);
    client.get(&path, params).await
}

#[cfg(feature = "charges")]
#[doc = "List all charges"]
pub async fn list_charges(
    client: &crate::Client,
    params: &stripe_types_05::charges::ListChargesParams,
) -> Result<crate::List<stripe_types_14::charges::Charge>, crate::Error> {
    let path = crate::params::format_path("/v1/charges", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "charges")]
#[doc = "Search charges"]
pub async fn search_charges(
    client: &crate::Client,
    params: &stripe_types_00::charges::SearchChargesParams,
) -> Result<crate::List<stripe_types_14::charges::Charge>, crate::Error> {
    let path = crate::params::format_path("/v1/charges/search", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "charges")]
#[doc = "Update a charge"]
pub async fn update_charge(
    client: &crate::Client,
    charge: &str,
    params: &stripe_types_10::charges::UpdateChargeParams,
) -> Result<stripe_types_14::charges::Charge, crate::Error> {
    let path = crate::params::format_path("/v1/charges/{charge}", &[("charge", charge)]);
    client.post_form(&path, params).await
}
