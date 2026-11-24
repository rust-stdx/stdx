#[cfg(feature = "exchange_rates")]
#[doc = "Retrieve an exchange rate"]
pub async fn get_exchange_rate(
    client: &crate::Client,
    rate_id: &str,
    params: &stripe_types_02::exchange_rates::GetExchangeRateParams,
) -> Result<stripe_types_05::exchange_rates::ExchangeRate, crate::Error> {
    let path = crate::params::format_path("/v1/exchange_rates/{rate_id}", &[("rate_id", rate_id)]);
    client.get(&path, params).await
}

#[cfg(feature = "exchange_rates")]
#[doc = "List all exchange rates"]
pub async fn list_exchange_rates(
    client: &crate::Client,
    params: &stripe_types_01::exchange_rates::ListExchangeRatesParams,
) -> Result<crate::List<stripe_types_05::exchange_rates::ExchangeRate>, crate::Error> {
    let path = crate::params::format_path("/v1/exchange_rates", &[]);
    client.get(&path, params).await
}
