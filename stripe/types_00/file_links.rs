#[cfg(feature = "file_links")]
#[derive(Debug, Clone, Default)]
pub struct CreateFileLinkParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The link isn't usable after this future timestamp."]
    pub expires_at: Option<i64>,
    #[doc = "The ID of the file."]
    pub file: String,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "file_links")]
impl crate::params::FormEncode for CreateFileLinkParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("expires_at", self.expires_at.as_ref());
        form.field("file", &self.file);
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}

#[cfg(feature = "file_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdateFileLinkParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A future timestamp after which the link will no longer be usable, or `now` to expire the link immediately."]
    pub expires_at: Option<json::OwnedValue>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "file_links")]
impl crate::params::FormEncode for UpdateFileLinkParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}
