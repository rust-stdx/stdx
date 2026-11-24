#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParams {
    #[doc = "Information for the account this token represents."]
    pub account: Option<stripe_types_12::tokens::CreateTokenParamsAccount>,
    #[doc = "The bank account this token will represent."]
    pub bank_account: Option<stripe_types_06::tokens::CreateTokenParamsBankAccount>,
    #[doc = "The card this token will represent."]
    pub card: Option<stripe_types_08::tokens::CreateTokenParamsCard>,
    #[doc = "Create a token for the customer, which is owned by the application's account."]
    pub customer: Option<String>,
    #[doc = "The updated CVC value this token represents."]
    pub cvc_update: Option<stripe_types_02::tokens::CreateTokenParamsCvcUpdate>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Information for the person this token represents."]
    pub person: Option<stripe_types_11::tokens::CreateTokenParamsPerson>,
    #[doc = "The PII this token represents."]
    pub pii: Option<stripe_types_00::tokens::CreateTokenParamsPii>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field_opt("bank_account", self.bank_account.as_ref());
        form.field_opt("card", self.card.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("cvc_update", self.cvc_update.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("person", self.person.as_ref());
        form.field_opt("pii", self.pii.as_ref());
    }
}
