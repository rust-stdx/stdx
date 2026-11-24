#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerSessionParamsComponentsMobilePaymentElement {
    #[doc = "Whether the mobile payment element is enabled."]
    pub enabled: bool,
    #[doc = "This hash defines whether the mobile payment element supports certain features."]
    pub features:
        Option<stripe_types_09::customer_sessions::CreateCustomerSessionParamsComponentsMobilePaymentElementFeatures>,
}

#[cfg(feature = "customer_sessions")]
impl crate::params::FormEncode for CreateCustomerSessionParamsComponentsMobilePaymentElement {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("features", self.features.as_ref());
    }
}
