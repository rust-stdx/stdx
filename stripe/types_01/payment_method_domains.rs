#[cfg(feature = "payment_method_domains")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodDomainParams {
    #[doc = "Whether this payment method domain is enabled."]
    pub enabled: Option<bool>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "payment_method_domains")]
impl crate::params::FormEncode for UpdatePaymentMethodDomainParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enabled", self.enabled.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
