#[cfg(feature = "tax_ids")]
#[derive(Debug, Clone, Default)]
pub struct GetTaxIdParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "tax_ids")]
impl crate::params::FormEncode for GetTaxIdParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
