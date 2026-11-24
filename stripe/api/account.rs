#[cfg(feature = "account")]
#[doc = "Retrieve account"]
pub async fn get_account(
    client: &crate::Client,
    params: &stripe_types_04::account::GetAccountParams,
) -> Result<stripe_types_14::account::Account, crate::Error> {
    let path = crate::params::format_path("/v1/account", &[]);
    client.get(&path, params).await
}
