#[cfg(feature = "balance_settings")]
#[derive(Debug, Clone, Default)]
pub struct CreateBalanceSettingParamsPayments {
    #[doc = "A Boolean indicating whether Stripe should try to reclaim negative balances from an attached bank account."]
    pub debit_negative_balances: Option<bool>,
    #[doc = "Settings specific to the account's payouts."]
    pub payouts: Option<stripe_types_07::balance_settings::CreateBalanceSettingParamsPaymentsPayouts>,
    #[doc = "Settings related to the account's balance settlement timing."]
    pub settlement_timing:
        Option<stripe_types_06::balance_settings::CreateBalanceSettingParamsPaymentsSettlementTiming>,
}

#[cfg(feature = "balance_settings")]
impl crate::params::FormEncode for CreateBalanceSettingParamsPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("debit_negative_balances", self.debit_negative_balances.as_ref());
        form.field_opt("payouts", self.payouts.as_ref());
        form.field_opt("settlement_timing", self.settlement_timing.as_ref());
    }
}
