#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerSessionParamsComponentsActiveEntitlements {
    #[doc = "Whether the active entitlements is enabled."]
    pub enabled: bool,
}

#[cfg(feature = "customer_sessions")]
impl crate::params::FormEncode for CreateCustomerSessionParamsComponentsActiveEntitlements {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
    }
}

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSessionResourceComponentsResourceCustomerPortal {
    #[doc = "Whether the customer portal is enabled."]
    #[json(default)]
    pub enabled: bool,
}
