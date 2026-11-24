#[cfg(feature = "tax_ids")]
#[doc = "Create a tax ID"]
pub async fn create_tax_id(
    client: &crate::Client,
    params: &stripe_types_11::tax_ids::CreateTaxIdParams,
) -> Result<stripe_types_14::tax_ids::TaxId, crate::Error> {
    let path = crate::params::format_path("/v1/tax_ids", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "tax_ids")]
#[doc = "Delete a tax ID"]
pub async fn delete_tax_id(
    client: &crate::Client,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_05::tax_ids::DeletedTaxId, crate::Error> {
    let path = crate::params::format_path("/v1/tax_ids/{id}", &[("id", id)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "tax_ids")]
#[doc = "Retrieve a tax ID"]
pub async fn get_tax_id(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_04::tax_ids::GetTaxIdParams,
) -> Result<stripe_types_14::tax_ids::TaxId, crate::Error> {
    let path = crate::params::format_path("/v1/tax_ids/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "tax_ids")]
#[doc = "List all tax IDs"]
pub async fn list_tax_ids(
    client: &crate::Client,
    params: &stripe_types_09::tax_ids::ListTaxIdsParams,
) -> Result<crate::List<stripe_types_14::tax_ids::TaxId>, crate::Error> {
    let path = crate::params::format_path("/v1/tax_ids", &[]);
    client.get(&path, params).await
}
