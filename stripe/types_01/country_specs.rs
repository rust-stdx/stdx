#[cfg(feature = "country_specs")]
#[derive(Debug, Clone, Default)]
pub struct GetCountrySpecParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "country_specs")]
impl crate::params::FormEncode for GetCountrySpecParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
