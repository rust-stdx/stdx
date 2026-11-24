#[cfg(feature = "balance")]
#[doc = "Retrieve balance"]
pub async fn get_balance(
    client: &crate::Client,
    params: &stripe_types_04::balance::GetBalanceParams,
) -> Result<stripe_types_11::balance::Balance, crate::Error> {
    let path = crate::params::format_path("/v1/balance", &[]);
    client.get(&path, params).await
}
