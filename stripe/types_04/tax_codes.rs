#[cfg(feature = "tax_codes")]
#[derive(Debug, Clone, Default)]
pub struct GetTaxCodeParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "tax_codes")]
impl crate::params::FormEncode for GetTaxCodeParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
