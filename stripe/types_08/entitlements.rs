#[cfg(any(feature = "entitlements", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct EntitlementsActiveEntitlement {
    #[doc = "The [Feature](https://docs.stripe.com/api/entitlements/feature) that the customer is entitled to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<crate::Expandable<stripe_types_06::entitlements::EntitlementsFeature>>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "A unique key you provide as your own system identifier."]
    #[json(default)]
    pub lookup_key: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::entitlements::EntitlementsActiveEntitlementObject,
}

#[cfg(any(feature = "entitlements", feature = "events"))]
impl crate::PagedItem for EntitlementsActiveEntitlement {
    fn id(&self) -> &str {
        &self.id
    }
}
