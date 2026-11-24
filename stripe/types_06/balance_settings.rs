#[cfg(any(feature = "balance_settings", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceSettingsResourcePayoutSchedule {
    #[doc = "How frequently funds will be paid out."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<stripe_types_01::balance_settings::BalanceSettingsResourcePayoutScheduleInterval>,
    #[doc = "The day of the month funds will be paid out."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub monthly_payout_days: Option<Vec<i64>>,
    #[doc = "The days of the week when available funds are paid out, specified as an array, for example, `monday`, `tuesday`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub weekly_payout_days:
        Option<Vec<stripe_types_04::balance_settings::BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem>>,
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceSettingsResourceSettlementTiming {
    #[doc = "The number of days charge funds are held before becoming available."]
    #[json(default)]
    pub delay_days: i64,
    #[doc = "The number of days charge funds are held before becoming available."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub delay_days_override: Option<i64>,
    #[doc = "Customized start of day configuration for automatic payouts to group and send payments in local timezones with a cust..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub start_of_day: Option<stripe_types_02::balance_settings::BalanceSettingsResourceStartOfDay>,
}

#[cfg(feature = "balance_settings")]
#[derive(Debug, Clone, Default)]
pub struct CreateBalanceSettingParamsPaymentsSettlementTiming {
    #[doc = "Change `delay_days` for this account, which determines the number of days charge funds are held before becoming avail..."]
    pub delay_days_override: Option<json::OwnedValue>,
    #[doc = "Customized start of day configuration for automatic payouts to group and send payments in local timezones with a cust..."]
    pub start_of_day:
        Option<stripe_types_04::balance_settings::CreateBalanceSettingParamsPaymentsSettlementTimingStartOfDay>,
}

#[cfg(feature = "balance_settings")]
impl crate::params::FormEncode for CreateBalanceSettingParamsPaymentsSettlementTiming {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("start_of_day", self.start_of_day.as_ref());
    }
}
