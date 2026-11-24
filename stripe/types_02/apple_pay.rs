#[cfg(feature = "apple_pay")]
#[derive(Debug, Clone, Default)]
pub struct CreateApplePayDomainParams {
    pub domain_name: String,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "apple_pay")]
impl crate::params::FormEncode for CreateApplePayDomainParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("domain_name", &self.domain_name);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "apple_pay")]
#[derive(Debug, Clone, Default)]
pub struct GetApplePayDomainParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "apple_pay")]
impl crate::params::FormEncode for GetApplePayDomainParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
