#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsFinancialAccountFeatures {
    #[doc = "Whether Stripe user authentication is disabled."]
    pub disable_stripe_user_authentication: Option<bool>,
    #[doc = "Whether external account collection is enabled."]
    pub external_account_collection: Option<bool>,
    #[doc = "Whether to allow sending money."]
    pub send_money: Option<bool>,
    #[doc = "Whether to allow transferring balance."]
    pub transfer_balance: Option<bool>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsFinancialAccountFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt(
            "disable_stripe_user_authentication",
            self.disable_stripe_user_authentication.as_ref(),
        );
        form.field_opt("external_account_collection", self.external_account_collection.as_ref());
        form.field_opt("send_money", self.send_money.as_ref());
        form.field_opt("transfer_balance", self.transfer_balance.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsPaymentDisputesFeatures {
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
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsPaymentDisputesFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
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
pub struct CreateAccountSessionParamsComponentsPaymentMethodSettingsFeatures {
    #[doc = "Whether Stripe user authentication is disabled."]
    pub disable_stripe_user_authentication: Option<bool>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsPaymentMethodSettingsFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt(
            "disable_stripe_user_authentication",
            self.disable_stripe_user_authentication.as_ref(),
        );
    }
}
