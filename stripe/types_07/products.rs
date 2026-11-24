#[cfg(feature = "products")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ProductFeature {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub entitlement_feature: Option<stripe_types_06::entitlements::EntitlementsFeature>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::products::ProductFeatureObject,
}

#[cfg(feature = "products")]
impl crate::PagedItem for ProductFeature {
    fn id(&self) -> &str {
        &self.id
    }
}
