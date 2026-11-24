#[cfg(feature = "balance_settings")]
#[doc = "Update balance settings"]
pub async fn create_balance_setting(
    client: &crate::Client,
    params: &stripe_types_12::balance_settings::CreateBalanceSettingParams,
) -> Result<stripe_types_12::balance_settings::BalanceSettings, crate::Error> {
    let path = crate::params::format_path("/v1/balance_settings", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "balance_settings")]
#[doc = "Retrieve balance settings"]
pub async fn get_balance_setting(
    client: &crate::Client,
    params: &stripe_types_02::balance_settings::GetBalanceSettingParams,
) -> Result<stripe_types_12::balance_settings::BalanceSettings, crate::Error> {
    let path = crate::params::format_path("/v1/balance_settings", &[]);
    client.get(&path, params).await
}
