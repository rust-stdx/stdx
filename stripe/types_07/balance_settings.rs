#[cfg(feature = "balance_settings")]
#[derive(Debug, Clone, Default)]
pub struct CreateBalanceSettingParamsPaymentsPayouts {
    #[doc = "Configures per-currency rules for automatically transferring funds from the payments balance to a FinancialAccount."]
    pub automatic_transfer_rules_by_currency: Option<std::collections::HashMap<String, json::OwnedValue>>,
    #[doc = "The minimum balance amount to retain per currency after automatic payouts."]
    pub minimum_balance_by_currency: Option<std::collections::HashMap<String, json::OwnedValue>>,
    #[doc = "Details on when funds from charges are available, and when they are paid out to an external account."]
    pub schedule: Option<stripe_types_05::balance_settings::CreateBalanceSettingParamsPaymentsPayoutsSchedule>,
    #[doc = "The text that appears on the bank account statement for payouts."]
    pub statement_descriptor: Option<String>,
}

#[cfg(feature = "balance_settings")]
impl crate::params::FormEncode for CreateBalanceSettingParamsPaymentsPayouts {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("schedule", self.schedule.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
    }
}
