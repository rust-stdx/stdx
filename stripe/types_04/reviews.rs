#[cfg(feature = "reviews")]
#[derive(Debug, Clone, Default)]
pub struct GetReviewParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "reviews")]
impl crate::params::FormEncode for GetReviewParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
