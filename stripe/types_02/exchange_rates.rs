#[cfg(feature = "exchange_rates")]
#[derive(Debug, Clone, Default)]
pub struct GetExchangeRateParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "exchange_rates")]
impl crate::params::FormEncode for GetExchangeRateParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
