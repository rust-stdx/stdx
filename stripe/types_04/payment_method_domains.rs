#[cfg(feature = "payment_method_domains")]
#[derive(Debug, Clone, Default)]
pub struct GetPaymentMethodDomainParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "payment_method_domains")]
impl crate::params::FormEncode for GetPaymentMethodDomainParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
