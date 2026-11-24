#[cfg(feature = "identity")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIdentityVerificationSessionParamsOptions {
    #[doc = "Options that apply to the [document check](https://docs.stripe.com/identity/verification-checks?type=document)."]
    pub document: Option<stripe_types_08::identity::UpdateIdentityVerificationSessionParamsOptionsDocument>,
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for UpdateIdentityVerificationSessionParamsOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("document", self.document.as_ref());
    }
}
