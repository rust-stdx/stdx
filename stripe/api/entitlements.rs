#[cfg(feature = "entitlements")]
#[doc = "Create a feature"]
pub async fn create_entitlement_feature(
    client: &crate::Client,
    params: &stripe_types_02::entitlements::CreateEntitlementFeatureParams,
) -> Result<stripe_types_06::entitlements::EntitlementsFeature, crate::Error> {
    let path = crate::params::format_path("/v1/entitlements/features", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "entitlements")]
#[doc = "Retrieve an active entitlement"]
pub async fn get_entitlement_active_entitlement(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_00::entitlements::GetEntitlementActiveEntitlementParams,
) -> Result<stripe_types_08::entitlements::EntitlementsActiveEntitlement, crate::Error> {
    let path = crate::params::format_path("/v1/entitlements/active_entitlements/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "entitlements")]
#[doc = "Retrieve a feature"]
pub async fn get_entitlement_feature(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::entitlements::GetEntitlementFeatureParams,
) -> Result<stripe_types_06::entitlements::EntitlementsFeature, crate::Error> {
    let path = crate::params::format_path("/v1/entitlements/features/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "entitlements")]
#[doc = "List all active entitlements"]
pub async fn list_entitlement_active_entitlements(
    client: &crate::Client,
    params: &stripe_types_01::entitlements::ListEntitlementActiveEntitlementsParams,
) -> Result<crate::List<stripe_types_08::entitlements::EntitlementsActiveEntitlement>, crate::Error> {
    let path = crate::params::format_path("/v1/entitlements/active_entitlements", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "entitlements")]
#[doc = "List all features"]
pub async fn list_entitlement_features(
    client: &crate::Client,
    params: &stripe_types_03::entitlements::ListEntitlementFeaturesParams,
) -> Result<crate::List<stripe_types_06::entitlements::EntitlementsFeature>, crate::Error> {
    let path = crate::params::format_path("/v1/entitlements/features", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "entitlements")]
#[doc = "Updates a feature"]
pub async fn update_entitlement_feature(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_01::entitlements::UpdateEntitlementFeatureParams,
) -> Result<stripe_types_06::entitlements::EntitlementsFeature, crate::Error> {
    let path = crate::params::format_path("/v1/entitlements/features/{id}", &[("id", id)]);
    client.post_form(&path, params).await
}
