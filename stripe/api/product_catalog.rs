#[cfg(feature = "product_catalog")]
#[doc = "Create a trial offer"]
pub async fn create_product_catalog_trial_offer(
    client: &crate::Client,
    params: &stripe_types_07::product_catalog::CreateProductCatalogTrialOfferParams,
) -> Result<stripe_types_13::product_catalog::ProductCatalogTrialOffer, crate::Error> {
    let path = crate::params::format_path("/v1/product_catalog/trial_offers", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "product_catalog")]
#[doc = "Retrieve a trial offer"]
pub async fn get_product_catalog_trial_offer(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_00::product_catalog::GetProductCatalogTrialOfferParams,
) -> Result<stripe_types_13::product_catalog::ProductCatalogTrialOffer, crate::Error> {
    let path = crate::params::format_path("/v1/product_catalog/trial_offers/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "product_catalog")]
#[doc = "List trial offers"]
pub async fn list_product_catalog_trial_offers(
    client: &crate::Client,
    params: &stripe_types_05::product_catalog::ListProductCatalogTrialOffersParams,
) -> Result<crate::List<stripe_types_13::product_catalog::ProductCatalogTrialOffer>, crate::Error> {
    let path = crate::params::format_path("/v1/product_catalog/trial_offers", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "product_catalog")]
#[doc = "Update a trial offer"]
pub async fn update_product_catalog_trial_offer(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_03::product_catalog::UpdateProductCatalogTrialOfferParams,
) -> Result<stripe_types_13::product_catalog::ProductCatalogTrialOffer, crate::Error> {
    let path = crate::params::format_path("/v1/product_catalog/trial_offers/{id}", &[("id", id)]);
    client.post_form(&path, params).await
}
