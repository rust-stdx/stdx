#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsBalancesFeatures {
    #[doc = "Whether Stripe user authentication is disabled."]
    pub disable_stripe_user_authentication: Option<bool>,
    #[doc = "Whether to allow payout schedule to be changed."]
    pub edit_payout_schedule: Option<bool>,
    #[doc = "Whether external account collection is enabled."]
    pub external_account_collection: Option<bool>,
    #[doc = "Whether instant payouts are enabled for this component."]
    pub instant_payouts: Option<bool>,
    #[doc = "Whether to allow creation of standard payouts."]
    pub standard_payouts: Option<bool>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsBalancesFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt(
            "disable_stripe_user_authentication",
            self.disable_stripe_user_authentication.as_ref(),
        );
        form.field_opt("edit_payout_schedule", self.edit_payout_schedule.as_ref());
        form.field_opt("external_account_collection", self.external_account_collection.as_ref());
        form.field_opt("instant_payouts", self.instant_payouts.as_ref());
        form.field_opt("standard_payouts", self.standard_payouts.as_ref());
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponentsIssuingCardsListFeatures {
    #[doc = "Whether to allow card management features."]
    pub card_management: Option<bool>,
    #[doc = "Whether to allow card spend dispute management features."]
    pub card_spend_dispute_management: Option<bool>,
    #[doc = "Whether to allow cardholder management features."]
    pub cardholder_management: Option<bool>,
    #[doc = "Whether Stripe user authentication is disabled."]
    pub disable_stripe_user_authentication: Option<bool>,
    #[doc = "Whether to allow spend control management features."]
    pub spend_control_management: Option<bool>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponentsIssuingCardsListFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("card_management", self.card_management.as_ref());
        form.field_opt("card_spend_dispute_management", self.card_spend_dispute_management.as_ref());
        form.field_opt("cardholder_management", self.cardholder_management.as_ref());
        form.field_opt(
            "disable_stripe_user_authentication",
            self.disable_stripe_user_authentication.as_ref(),
        );
        form.field_opt("spend_control_management", self.spend_control_management.as_ref());
    }
}
