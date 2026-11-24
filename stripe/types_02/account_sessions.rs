#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, PartialEq)]
pub enum AccountSessionObject {
    AccountSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "account_sessions")]
impl AccountSessionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            AccountSessionObject::AccountSession => "account_session",
            AccountSessionObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "account_sessions")]
impl Default for AccountSessionObject {
    fn default() -> Self {
        AccountSessionObject::Unknown(String::new())
    }
}

#[cfg(feature = "account_sessions")]
impl<'json> json::Decode<'json> for AccountSessionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(AccountSessionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_session" => AccountSessionObject::AccountSession,
            other => AccountSessionObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsAccountManagementFeatures {
    #[doc = "Whether Stripe user authentication is disabled."]
    pub disable_stripe_user_authentication: Option<bool>,
    #[doc = "Whether external account collection is enabled."]
    pub external_account_collection: Option<bool>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsAccountManagementFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt(
            "disable_stripe_user_authentication",
            self.disable_stripe_user_authentication.as_ref(),
        );
        form.field_opt("external_account_collection", self.external_account_collection.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsDisputesListFeatures {
    #[doc = "Whether to allow capturing and cancelling payment intents."]
    pub capture_payments: Option<bool>,
    #[doc = "Whether connected accounts can manage destination charges that are created on behalf of them."]
    pub destination_on_behalf_of_charge_management: Option<bool>,
    #[doc = "Whether responding to disputes is enabled, including submitting evidence and accepting disputes."]
    pub dispute_management: Option<bool>,
    #[doc = "Whether sending refunds is enabled."]
    pub refund_management: Option<bool>,
    #[doc = "Whether to allow connected accounts to submit disputes using Smart Disputes."]
    pub smart_disputes_management: Option<bool>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsDisputesListFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_payments", self.capture_payments.as_ref());
        form.field_opt(
            "destination_on_behalf_of_charge_management",
            self.destination_on_behalf_of_charge_management.as_ref(),
        );
        form.field_opt("dispute_management", self.dispute_management.as_ref());
        form.field_opt("refund_management", self.refund_management.as_ref());
        form.field_opt("smart_disputes_management", self.smart_disputes_management.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsIssuingCardFeatures {
    #[doc = "Whether to allow card management features."]
    pub card_management: Option<bool>,
    #[doc = "Whether to allow card spend dispute management features."]
    pub card_spend_dispute_management: Option<bool>,
    #[doc = "Whether to allow cardholder management features."]
    pub cardholder_management: Option<bool>,
    #[doc = "Whether to allow spend control management features."]
    pub spend_control_management: Option<bool>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsIssuingCardFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("card_management", self.card_management.as_ref());
        form.field_opt("card_spend_dispute_management", self.card_spend_dispute_management.as_ref());
        form.field_opt("cardholder_management", self.cardholder_management.as_ref());
        form.field_opt("spend_control_management", self.spend_control_management.as_ref());
    }
}
