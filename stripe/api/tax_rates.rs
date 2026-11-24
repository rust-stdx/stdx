#[cfg(feature = "tax_rates")]
#[doc = "Create a tax rate"]
pub async fn create_tax_rate(
    client: &crate::Client,
    params: &stripe_types_05::tax_rates::CreateTaxRateParams,
) -> Result<stripe_types_06::tax_rates::TaxRate, crate::Error> {
    let path = crate::params::format_path("/v1/tax_rates", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "tax_rates")]
#[doc = "Retrieve a tax rate"]
pub async fn get_tax_rate(
    client: &crate::Client,
    tax_rate: &str,
    params: &stripe_types_00::tax_rates::GetTaxRateParams,
) -> Result<stripe_types_06::tax_rates::TaxRate, crate::Error> {
    let path = crate::params::format_path("/v1/tax_rates/{tax_rate}", &[("tax_rate", tax_rate)]);
    client.get(&path, params).await
}

#[cfg(feature = "tax_rates")]
#[doc = "List all tax rates"]
pub async fn list_tax_rates(
    client: &crate::Client,
    params: &stripe_types_05::tax_rates::ListTaxRatesParams,
) -> Result<crate::List<stripe_types_06::tax_rates::TaxRate>, crate::Error> {
    let path = crate::params::format_path("/v1/tax_rates", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "tax_rates")]
#[doc = "Update a tax rate"]
pub async fn update_tax_rate(
    client: &crate::Client,
    tax_rate: &str,
    params: &stripe_types_07::tax_rates::UpdateTaxRateParams,
) -> Result<stripe_types_06::tax_rates::TaxRate, crate::Error> {
    let path = crate::params::format_path("/v1/tax_rates/{tax_rate}", &[("tax_rate", tax_rate)]);
    client.post_form(&path, params).await
}
