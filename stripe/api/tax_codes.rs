#[cfg(feature = "tax_codes")]
#[doc = "Retrieve a tax code"]
pub async fn get_tax_code(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_04::tax_codes::GetTaxCodeParams,
) -> Result<stripe_types_08::tax_codes::TaxCode, crate::Error> {
    let path = crate::params::format_path("/v1/tax_codes/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "tax_codes")]
#[doc = "List all tax codes"]
pub async fn list_tax_codes(
    client: &crate::Client,
    params: &stripe_types_03::tax_codes::ListTaxCodesParams,
) -> Result<crate::List<stripe_types_08::tax_codes::TaxCode>, crate::Error> {
    let path = crate::params::format_path("/v1/tax_codes", &[]);
    client.get(&path, params).await
}
