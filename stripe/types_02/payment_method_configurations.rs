#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct GetPaymentMethodConfigurationParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for GetPaymentMethodConfigurationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
