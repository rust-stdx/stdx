#[cfg(feature = "country_specs")]
#[doc = "Retrieve a Country Spec"]
pub async fn get_country_spec(
    client: &crate::Client,
    country: &str,
    params: &stripe_types_01::country_specs::GetCountrySpecParams,
) -> Result<stripe_types_07::country_specs::CountrySpec, crate::Error> {
    let path = crate::params::format_path("/v1/country_specs/{country}", &[("country", country)]);
    client.get(&path, params).await
}

#[cfg(feature = "country_specs")]
#[doc = "List Country Specs"]
pub async fn list_country_specs(
    client: &crate::Client,
    params: &stripe_types_00::country_specs::ListCountrySpecsParams,
) -> Result<crate::List<stripe_types_07::country_specs::CountrySpec>, crate::Error> {
    let path = crate::params::format_path("/v1/country_specs", &[]);
    client.get(&path, params).await
}
