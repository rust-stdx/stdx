#[cfg(feature = "ephemeral_keys")]
#[derive(Debug, Clone, Default)]
pub struct DeleteEphemeralKeyParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "ephemeral_keys")]
impl crate::params::FormEncode for DeleteEphemeralKeyParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
