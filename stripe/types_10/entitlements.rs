#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct EntitlementsActiveEntitlementSummary {
    #[doc = "The customer that is entitled to this feature."]
    #[json(default)]
    pub customer: String,
    #[doc = "The list of entitlements this customer has."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<crate::List<stripe_types_08::entitlements::EntitlementsActiveEntitlement>>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::entitlements::EntitlementsActiveEntitlementSummaryObject,
}
