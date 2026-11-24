#[cfg(feature = "shipping_rates")]
#[doc = "Create a shipping rate"]
pub async fn create_shipping_rate(
    client: &crate::Client,
    params: &stripe_types_12::shipping_rates::CreateShippingRateParams,
) -> Result<stripe_types_11::shipping_rates::ShippingRate, crate::Error> {
    let path = crate::params::format_path("/v1/shipping_rates", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "shipping_rates")]
#[doc = "Retrieve a shipping rate"]
pub async fn get_shipping_rate(
    client: &crate::Client,
    shipping_rate_token: &str,
    params: &stripe_types_01::shipping_rates::GetShippingRateParams,
) -> Result<stripe_types_11::shipping_rates::ShippingRate, crate::Error> {
    let path = crate::params::format_path(
        "/v1/shipping_rates/{shipping_rate_token}",
        &[("shipping_rate_token", shipping_rate_token)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "shipping_rates")]
#[doc = "List all shipping rates"]
pub async fn list_shipping_rates(
    client: &crate::Client,
    params: &stripe_types_05::shipping_rates::ListShippingRatesParams,
) -> Result<crate::List<stripe_types_11::shipping_rates::ShippingRate>, crate::Error> {
    let path = crate::params::format_path("/v1/shipping_rates", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "shipping_rates")]
#[doc = "Update a shipping rate"]
pub async fn update_shipping_rate(
    client: &crate::Client,
    shipping_rate_token: &str,
    params: &stripe_types_12::shipping_rates::UpdateShippingRateParams,
) -> Result<stripe_types_11::shipping_rates::ShippingRate, crate::Error> {
    let path = crate::params::format_path(
        "/v1/shipping_rates/{shipping_rate_token}",
        &[("shipping_rate_token", shipping_rate_token)],
    );
    client.post_form(&path, params).await
}
