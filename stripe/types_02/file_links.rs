#[cfg(feature = "file_links")]
#[derive(Debug, Clone, Default)]
pub struct GetFileLinkParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "file_links")]
impl crate::params::FormEncode for GetFileLinkParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
