#[cfg(any(feature = "balance_settings", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem {
    Friday,
    Monday,
    Thursday,
    Tuesday,
    Wednesday,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
impl BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem::Friday => "friday",
            BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem::Monday => "monday",
            BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem::Thursday => "thursday",
            BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem::Tuesday => "tuesday",
            BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem::Wednesday => "wednesday",
            BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
impl<'json> json::Decode<'json> for BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "friday" => BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem::Friday,
            "monday" => BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem::Monday,
            "thursday" => BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem::Thursday,
            "tuesday" => BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem::Tuesday,
            "wednesday" => BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem::Wednesday,
            other => BalanceSettingsResourcePayoutScheduleWeeklyPayoutDaysItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "balance_settings")]
#[derive(Debug, Clone, Default)]
pub struct CreateBalanceSettingParamsPaymentsSettlementTimingStartOfDay {
    #[doc = "Hour at which the customized start of day begins according to the given timezone."]
    pub hour: Option<i64>,
    #[doc = "Minutes at which the customized start of day begins according to the given timezone."]
    pub minutes: Option<i64>,
    #[doc = "Timezone for the customized start of day."]
    pub timezone: Option<String>,
}

#[cfg(feature = "balance_settings")]
impl crate::params::FormEncode for CreateBalanceSettingParamsPaymentsSettlementTimingStartOfDay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("hour", self.hour.as_ref());
        form.field_opt("minutes", self.minutes.as_ref());
        form.field_opt("timezone", self.timezone.as_ref());
    }
}
