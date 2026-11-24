#[cfg(any(feature = "balance_settings", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceSettingsResourceAutomaticTransferRule {
    #[doc = "The ID of the FinancialAccount that funds will be transferred to during automatic transfers."]
    #[json(default)]
    pub payout_method: String,
    #[doc = "The maximum amount in minor units to transfer to the FinancialAccount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transfer_up_to_amount: Option<i64>,
    #[doc = "The type of automatic transfer rule."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::balance_settings::BalanceSettingsResourceAutomaticTransferRuleType,
}

#[cfg(feature = "balance_settings")]
#[derive(Debug, Clone, Default)]
pub struct CreateBalanceSettingParamsPaymentsPayoutsSchedule {
    #[doc = "How frequently available funds are paid out."]
    pub interval: Option<stripe_types_03::balance_settings::CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval>,
    #[doc = "The days of the month when available funds are paid out, specified as an array of numbers between 1--31."]
    pub monthly_payout_days: Option<Vec<i64>>,
    #[doc = "The days of the week when available funds are paid out, specified as an array, e.g., `monday`, `tuesday`."]
    pub weekly_payout_days: Option<
        Vec<stripe_types_02::balance_settings::CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem>,
    >,
}

#[cfg(feature = "balance_settings")]
impl crate::params::FormEncode for CreateBalanceSettingParamsPaymentsPayoutsSchedule {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("interval", self.interval.as_ref());
        if let Some(values) = self.monthly_payout_days.as_ref() {
            form.field_array("monthly_payout_days", values.as_slice());
        }
        if let Some(values) = self.weekly_payout_days.as_ref() {
            form.field_array("weekly_payout_days", values.as_slice());
        }
    }
}
