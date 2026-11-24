#[cfg(feature = "quotes")]
#[derive(Debug, Clone)]
pub struct UpdateQuoteParamsAutomaticTaxLiability {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_06::quotes::UpdateQuoteParamsAutomaticTaxLiabilityType,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for UpdateQuoteParamsAutomaticTaxLiability {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}
