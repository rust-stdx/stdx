#[cfg(feature = "identity")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIdentityVerificationSessionParamsOptionsDocument {
    #[doc = "Array of strings of allowed identity document types."]
    pub allowed_types:
        Option<Vec<stripe_types_05::identity::UpdateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem>>,
    #[doc = "Collect an ID number and perform an ID number check..."]
    pub require_id_number: Option<bool>,
    #[doc = "Disable image uploads, identity document images have to be captured using the device’s camera."]
    pub require_live_capture: Option<bool>,
    #[doc = "Capture a face image and perform a [selfie check](https://docs.stripe.com/identity/verification-checks?type=selfie) c..."]
    pub require_matching_selfie: Option<bool>,
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for UpdateIdentityVerificationSessionParamsOptionsDocument {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.allowed_types.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("allowed_types", index, item);
            }
        }
        form.field_opt("require_id_number", self.require_id_number.as_ref());
        form.field_opt("require_live_capture", self.require_live_capture.as_ref());
        form.field_opt("require_matching_selfie", self.require_matching_selfie.as_ref());
    }
}
