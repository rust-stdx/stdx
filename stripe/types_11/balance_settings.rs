#[cfg(any(feature = "balance_settings", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceSettingsResourcePayments {
    #[doc = "A Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub debit_negative_balances: Option<bool>,
    #[doc = "Settings specific to the account's payouts."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payouts: Option<stripe_types_08::balance_settings::BalanceSettingsResourcePayouts>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub settlement_timing: Option<stripe_types_06::balance_settings::BalanceSettingsResourceSettlementTiming>,
}
