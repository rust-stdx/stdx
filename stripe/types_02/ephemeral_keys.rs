#[cfg(feature = "ephemeral_keys")]
#[derive(Debug, Clone, Default)]
pub struct CreateEphemeralKeyParams {
    #[doc = "The ID of the Customer you'd like to modify using the resulting ephemeral key."]
    pub customer: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The ID of the Issuing Card you'd like to access using the resulting ephemeral key."]
    pub issuing_card: Option<String>,
    #[doc = "A single-use token, created by Stripe.js, used for creating ephemeral keys for Issuing Cards without exchanging sensi..."]
    pub nonce: Option<String>,
    #[doc = "The ID of the Identity VerificationSession you'd like to access using the resulting ephemeral key"]
    pub verification_session: Option<String>,
}

#[cfg(feature = "ephemeral_keys")]
impl crate::params::FormEncode for CreateEphemeralKeyParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("customer", self.customer.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("issuing_card", self.issuing_card.as_ref());
        form.field_opt("nonce", self.nonce.as_ref());
        form.field_opt("verification_session", self.verification_session.as_ref());
    }
}
