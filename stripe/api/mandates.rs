#[cfg(feature = "mandates")]
#[doc = "Retrieve a Mandate"]
pub async fn get_mandate(
    client: &crate::Client,
    mandate: &str,
    params: &stripe_types_03::mandates::GetMandateParams,
) -> Result<stripe_types_14::mandates::Mandate, crate::Error> {
    let path = crate::params::format_path("/v1/mandates/{mandate}", &[("mandate", mandate)]);
    client.get(&path, params).await
}
