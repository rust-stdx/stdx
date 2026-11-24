#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsAccount {
    #[doc = "The business type."]
    pub business_type: Option<stripe_types_02::tokens::CreateTokenParamsAccountBusinessType>,
    #[doc = "Information about the company or business."]
    pub company: Option<stripe_types_08::tokens::CreateTokenParamsAccountCompany>,
    #[doc = "Information about the person represented by the account."]
    pub individual: Option<stripe_types_10::tokens::CreateTokenParamsAccountIndividual>,
    #[doc = "Whether the user described by the data in the token has been shown the Stripe Connected Account Agreement..."]
    pub tos_shown_and_accepted: Option<bool>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("business_type", self.business_type.as_ref());
        form.field_opt("company", self.company.as_ref());
        form.field_opt("individual", self.individual.as_ref());
        form.field_opt("tos_shown_and_accepted", self.tos_shown_and_accepted.as_ref());
    }
}
