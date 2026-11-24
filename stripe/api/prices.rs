#[cfg(feature = "prices")]
#[doc = "Create a price"]
pub async fn create_price(
    client: &crate::Client,
    params: &stripe_types_11::prices::CreatePriceParams,
) -> Result<stripe_types_11::prices::Price, crate::Error> {
    let path = crate::params::format_path("/v1/prices", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "prices")]
#[doc = "Retrieve a price"]
pub async fn get_price(
    client: &crate::Client,
    price: &str,
    params: &stripe_types_03::prices::GetPriceParams,
) -> Result<stripe_types_11::prices::Price, crate::Error> {
    let path = crate::params::format_path("/v1/prices/{price}", &[("price", price)]);
    client.get(&path, params).await
}

#[cfg(feature = "prices")]
#[doc = "List all prices"]
pub async fn list_prices(
    client: &crate::Client,
    params: &stripe_types_09::prices::ListPricesParams,
) -> Result<crate::List<stripe_types_11::prices::Price>, crate::Error> {
    let path = crate::params::format_path("/v1/prices", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "prices")]
#[doc = "Search prices"]
pub async fn search_prices(
    client: &crate::Client,
    params: &stripe_types_02::prices::SearchPricesParams,
) -> Result<crate::List<stripe_types_11::prices::Price>, crate::Error> {
    let path = crate::params::format_path("/v1/prices/search", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "prices")]
#[doc = "Update a price"]
pub async fn update_price(
    client: &crate::Client,
    price: &str,
    params: &stripe_types_10::prices::UpdatePriceParams,
) -> Result<stripe_types_11::prices::Price, crate::Error> {
    let path = crate::params::format_path("/v1/prices/{price}", &[("price", price)]);
    client.post_form(&path, params).await
}
