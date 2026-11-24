#[cfg(any(feature = "entitlements", feature = "events", feature = "products"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct EntitlementsFeature {
    #[doc = "Inactive features cannot be attached to new products and will not be returned from the features list endpoint."]
    #[json(default)]
    pub active: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "A unique key you provide as your own system identifier."]
    #[json(default)]
    pub lookup_key: String,
    #[doc = "Set of key-value pairs that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "The feature's name, for your own purpose, not meant to be displayable to the customer."]
    #[json(default)]
    pub name: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::entitlements::EntitlementsFeatureObject,
}

#[cfg(any(feature = "entitlements", feature = "events", feature = "products"))]
impl crate::PagedItem for EntitlementsFeature {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "entitlements")]
pub type GetEntitlementFeatureParams = stripe_types_00::entitlements::GetEntitlementActiveEntitlementParams;
