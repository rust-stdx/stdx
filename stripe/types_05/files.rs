#[cfg(feature = "files")]
#[derive(Debug, Clone)]
pub struct CreateFileParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A file to upload."]
    pub file: String,
    #[doc = "Optional parameters that automatically create a [file link](https://docs.stripe.com/api#file_links) for the newly cre..."]
    pub file_link_data: Option<stripe_types_02::files::CreateFileParamsFileLinkData>,
    #[doc = "The [purpose](https://docs.stripe.com/file-upload#uploading-a-file) of the uploaded file."]
    pub purpose: stripe_types_01::files::CreateFileParamsPurpose,
}

#[cfg(feature = "files")]
impl crate::params::FormEncode for CreateFileParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("file", &self.file);
        form.field_opt("file_link_data", self.file_link_data.as_ref());
        form.field("purpose", &self.purpose);
    }
}
