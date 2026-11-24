#[cfg(feature = "billing_portal")]
#[doc = "Create a portal configuration"]
pub async fn create_billing_portal_configuration(
    client: &crate::Client,
    params: &stripe_types_13::billing_portal::CreateBillingPortalConfigurationParams,
) -> Result<stripe_types_13::billing_portal::BillingPortalConfiguration, crate::Error> {
    let path = crate::params::format_path("/v1/billing_portal/configurations", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing_portal")]
#[doc = "Create a portal session"]
pub async fn create_billing_portal_session(
    client: &crate::Client,
    params: &stripe_types_12::billing_portal::CreateBillingPortalSessionParams,
) -> Result<stripe_types_13::billing_portal::BillingPortalSession, crate::Error> {
    let path = crate::params::format_path("/v1/billing_portal/sessions", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing_portal")]
#[doc = "Retrieve a portal configuration"]
pub async fn get_billing_portal_configuration(
    client: &crate::Client,
    configuration: &str,
    params: &stripe_types_04::billing_portal::GetBillingPortalConfigurationParams,
) -> Result<stripe_types_13::billing_portal::BillingPortalConfiguration, crate::Error> {
    let path = crate::params::format_path(
        "/v1/billing_portal/configurations/{configuration}",
        &[("configuration", configuration)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "billing_portal")]
#[doc = "List portal configurations"]
pub async fn list_billing_portal_configurations(
    client: &crate::Client,
    params: &stripe_types_04::billing_portal::ListBillingPortalConfigurationsParams,
) -> Result<crate::List<stripe_types_13::billing_portal::BillingPortalConfiguration>, crate::Error> {
    let path = crate::params::format_path("/v1/billing_portal/configurations", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "billing_portal")]
#[doc = "Update a portal configuration"]
pub async fn update_billing_portal_configuration(
    client: &crate::Client,
    configuration: &str,
    params: &stripe_types_12::billing_portal::UpdateBillingPortalConfigurationParams,
) -> Result<stripe_types_13::billing_portal::BillingPortalConfiguration, crate::Error> {
    let path = crate::params::format_path(
        "/v1/billing_portal/configurations/{configuration}",
        &[("configuration", configuration)],
    );
    client.post_form(&path, params).await
}
