#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSessionResourceComponentsResourceActiveEntitlements {
    #[doc = "Whether the active entitlements is enabled."]
    #[json(default)]
    pub enabled: bool,
}
