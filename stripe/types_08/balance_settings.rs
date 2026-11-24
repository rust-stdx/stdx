#[cfg(any(feature = "balance_settings", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceSettingsResourcePayouts {
    #[doc = "Configures per-currency rules for automatically transferring funds from the payments balance to a FinancialAccount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub automatic_transfer_rules_by_currency: Option<
        std::collections::HashMap<
            String,
            Vec<stripe_types_05::balance_settings::BalanceSettingsResourceAutomaticTransferRule>,
        >,
    >,
    #[doc = "The minimum balance amount to retain per currency after automatic payouts."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub minimum_balance_by_currency: Option<std::collections::HashMap<String, i64>>,
    #[doc = "Details on when funds from charges are available, and when they are paid out to an external account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<stripe_types_06::balance_settings::BalanceSettingsResourcePayoutSchedule>,
    #[doc = "The text that appears on the bank account statement for payouts."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[doc = "Whether the funds in this account can be paid out."]
    #[json(default)]
    pub status: stripe_types_03::balance_settings::BalanceSettingsResourcePayoutsStatus,
}
