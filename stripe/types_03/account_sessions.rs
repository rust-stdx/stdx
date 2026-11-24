#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsBalanceReportFeatures {}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsBalanceReportFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsFinancialAccountTransactionsFeatures {
    #[doc = "Whether to allow card spend dispute management features."]
    pub card_spend_dispute_management: Option<bool>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsFinancialAccountTransactionsFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("card_spend_dispute_management", self.card_spend_dispute_management.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsInstantPayoutsPromotionFeatures {
    #[doc = "Whether Stripe user authentication is disabled."]
    pub disable_stripe_user_authentication: Option<bool>,
    #[doc = "Whether external account collection is enabled."]
    pub external_account_collection: Option<bool>,
    #[doc = "Whether instant payouts are enabled for this component."]
    pub instant_payouts: Option<bool>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsInstantPayoutsPromotionFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt(
            "disable_stripe_user_authentication",
            self.disable_stripe_user_authentication.as_ref(),
        );
        form.field_opt("external_account_collection", self.external_account_collection.as_ref());
        form.field_opt("instant_payouts", self.instant_payouts.as_ref());
    }
}
