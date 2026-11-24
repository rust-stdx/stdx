#[cfg(feature = "tax")]
#[doc = "Create a Transaction from a Calculation"]
pub async fn create_from_calculation(
    client: &crate::Client,
    params: &stripe_types_02::tax::CreateFromCalculationParams,
) -> Result<stripe_types_12::tax::TaxTransaction, crate::Error> {
    let path = crate::params::format_path("/v1/tax/transactions/create_from_calculation", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Create a reversal Transaction"]
pub async fn create_reversal(
    client: &crate::Client,
    params: &stripe_types_05::tax::CreateReversalParams,
) -> Result<stripe_types_12::tax::TaxTransaction, crate::Error> {
    let path = crate::params::format_path("/v1/tax/transactions/create_reversal", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Create a Calculation"]
pub async fn create_tax_calculation(
    client: &crate::Client,
    params: &stripe_types_10::tax::CreateTaxCalculationParams,
) -> Result<stripe_types_12::tax::TaxCalculation, crate::Error> {
    let path = crate::params::format_path("/v1/tax/calculations", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Create a tax location"]
pub async fn create_tax_location(
    client: &crate::Client,
    params: &stripe_types_07::tax::CreateTaxLocationParams,
) -> Result<stripe_types_06::tax::TaxLocation, crate::Error> {
    let path = crate::params::format_path("/v1/tax/locations", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Create a registration"]
pub async fn create_tax_registration(
    client: &crate::Client,
    params: &stripe_types_13::tax::CreateTaxRegistrationParams,
) -> Result<stripe_types_13::tax::TaxRegistration, crate::Error> {
    let path = crate::params::format_path("/v1/tax/registrations", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Update settings"]
pub async fn create_tax_setting(
    client: &crate::Client,
    params: &stripe_types_08::tax::CreateTaxSettingParams,
) -> Result<stripe_types_08::tax::TaxSettings, crate::Error> {
    let path = crate::params::format_path("/v1/tax/settings", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Find a Tax Association"]
pub async fn get_tax_association_find(
    client: &crate::Client,
    params: &stripe_types_02::tax::GetTaxAssociationFindParams,
) -> Result<stripe_types_11::tax::TaxAssociation, crate::Error> {
    let path = crate::params::format_path("/v1/tax/associations/find", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Retrieve a Calculation"]
pub async fn get_tax_calculation(
    client: &crate::Client,
    calculation: &str,
    params: &stripe_types_01::tax::GetTaxCalculationParams,
) -> Result<stripe_types_12::tax::TaxCalculation, crate::Error> {
    let path = crate::params::format_path("/v1/tax/calculations/{calculation}", &[("calculation", calculation)]);
    client.get(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Retrieve a tax location"]
pub async fn get_tax_location(
    client: &crate::Client,
    location: &str,
    params: &stripe_types_06::tax::GetTaxLocationParams,
) -> Result<stripe_types_06::tax::TaxLocation, crate::Error> {
    let path = crate::params::format_path("/v1/tax/locations/{location}", &[("location", location)]);
    client.get(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Retrieve a registration"]
pub async fn get_tax_registration(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_05::tax::GetTaxRegistrationParams,
) -> Result<stripe_types_13::tax::TaxRegistration, crate::Error> {
    let path = crate::params::format_path("/v1/tax/registrations/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Retrieve settings"]
pub async fn get_tax_setting(
    client: &crate::Client,
    params: &stripe_types_06::tax::GetTaxSettingParams,
) -> Result<stripe_types_08::tax::TaxSettings, crate::Error> {
    let path = crate::params::format_path("/v1/tax/settings", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Retrieve a Transaction"]
pub async fn get_tax_transaction(
    client: &crate::Client,
    transaction: &str,
    params: &stripe_types_06::tax::GetTaxTransactionParams,
) -> Result<stripe_types_12::tax::TaxTransaction, crate::Error> {
    let path = crate::params::format_path("/v1/tax/transactions/{transaction}", &[("transaction", transaction)]);
    client.get(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Retrieve a Calculation's line items"]
pub async fn list_tax_calculation_line_items(
    client: &crate::Client,
    calculation: &str,
    params: &stripe_types_02::tax::ListTaxCalculationLineItemsParams,
) -> Result<crate::List<stripe_types_11::tax::TaxCalculationLineItem>, crate::Error> {
    let path =
        crate::params::format_path("/v1/tax/calculations/{calculation}/line_items", &[("calculation", calculation)]);
    client.get(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "List tax locations"]
pub async fn list_tax_locations(
    client: &crate::Client,
    params: &stripe_types_06::tax::ListTaxLocationsParams,
) -> Result<crate::List<stripe_types_06::tax::TaxLocation>, crate::Error> {
    let path = crate::params::format_path("/v1/tax/locations", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "List registrations"]
pub async fn list_tax_registrations(
    client: &crate::Client,
    params: &stripe_types_05::tax::ListTaxRegistrationsParams,
) -> Result<crate::List<stripe_types_13::tax::TaxRegistration>, crate::Error> {
    let path = crate::params::format_path("/v1/tax/registrations", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Retrieve a Transaction's line items"]
pub async fn list_tax_transaction_line_items(
    client: &crate::Client,
    transaction: &str,
    params: &stripe_types_06::tax::ListTaxTransactionLineItemsParams,
) -> Result<crate::List<stripe_types_05::tax::TaxTransactionLineItem>, crate::Error> {
    let path =
        crate::params::format_path("/v1/tax/transactions/{transaction}/line_items", &[("transaction", transaction)]);
    client.get(&path, params).await
}

#[cfg(feature = "tax")]
#[doc = "Update a registration"]
pub async fn update_tax_registration(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_02::tax::UpdateTaxRegistrationParams,
) -> Result<stripe_types_13::tax::TaxRegistration, crate::Error> {
    let path = crate::params::format_path("/v1/tax/registrations/{id}", &[("id", id)]);
    client.post_form(&path, params).await
}
