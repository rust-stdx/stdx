#[cfg(feature = "account_links")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountLinkParamsCollectionOptions {
    #[doc = "Specifies whether the platform collects only currently_due requirements (`currently_due`) or both currently_due and e..."]
    pub fields: Option<stripe_types_06::account_links::CreateAccountLinkParamsCollectionOptionsFields>,
    #[doc = "Specifies whether the platform collects future_requirements in addition to requirements in Connect Onboarding."]
    pub future_requirements:
        Option<stripe_types_01::account_links::CreateAccountLinkParamsCollectionOptionsFutureRequirements>,
}

#[cfg(feature = "account_links")]
impl crate::params::FormEncode for CreateAccountLinkParamsCollectionOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("fields", self.fields.as_ref());
        form.field_opt("future_requirements", self.future_requirements.as_ref());
    }
}
