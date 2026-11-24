#[cfg(feature = "application_fees")]
#[doc = "Create an application fee refund"]
pub async fn create_application_fee_refund(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_01::application_fees::CreateApplicationFeeRefundParams,
) -> Result<stripe_types_14::common::FeeRefund, crate::Error> {
    let path = crate::params::format_path("/v1/application_fees/{id}/refunds", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "application_fees")]
#[doc = "Retrieve an application fee"]
pub async fn get_application_fee(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::application_fees::GetApplicationFeeParams,
) -> Result<stripe_types_14::application_fees::ApplicationFee, crate::Error> {
    let path = crate::params::format_path("/v1/application_fees/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "application_fees")]
#[doc = "Retrieve an application fee refund"]
pub async fn get_application_fee_refund(
    client: &crate::Client,
    fee: &str,
    id: &str,
    params: &stripe_types_00::application_fees::GetApplicationFeeRefundParams,
) -> Result<stripe_types_14::common::FeeRefund, crate::Error> {
    let path = crate::params::format_path("/v1/application_fees/{fee}/refunds/{id}", &[("fee", fee), ("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "application_fees")]
#[doc = "List all application fee refunds"]
pub async fn list_application_fee_refunds(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_01::application_fees::ListApplicationFeeRefundsParams,
) -> Result<crate::List<stripe_types_14::common::FeeRefund>, crate::Error> {
    let path = crate::params::format_path("/v1/application_fees/{id}/refunds", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "application_fees")]
#[doc = "List all application fees"]
pub async fn list_application_fees(
    client: &crate::Client,
    params: &stripe_types_05::application_fees::ListApplicationFeesParams,
) -> Result<crate::List<stripe_types_14::application_fees::ApplicationFee>, crate::Error> {
    let path = crate::params::format_path("/v1/application_fees", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "application_fees")]
#[doc = "Update an application fee refund"]
pub async fn update_application_fee_refund(
    client: &crate::Client,
    fee: &str,
    id: &str,
    params: &stripe_types_03::application_fees::UpdateApplicationFeeRefundParams,
) -> Result<stripe_types_14::common::FeeRefund, crate::Error> {
    let path = crate::params::format_path("/v1/application_fees/{fee}/refunds/{id}", &[("fee", fee), ("id", id)]);
    client.post_form(&path, params).await
}
