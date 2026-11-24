#[cfg(any(feature = "balance_settings", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceSettingsResourceStartOfDay {
    #[doc = "Hour at which the customized start of day begins according to the given timezone."]
    #[json(default)]
    pub hour: i64,
    #[doc = "Minutes at which the customized start of day begins according to the given timezone."]
    #[json(default)]
    pub minutes: i64,
    #[doc = "Timezone for the customized start of day."]
    #[json(default)]
    pub timezone: String,
}

#[cfg(feature = "balance_settings")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem {
    Friday,
    Monday,
    Thursday,
    Tuesday,
    Wednesday,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "balance_settings")]
impl CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem::Friday => "friday",
            CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem::Monday => "monday",
            CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem::Thursday => "thursday",
            CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem::Tuesday => "tuesday",
            CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem::Wednesday => "wednesday",
            CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "balance_settings")]
impl<'json> json::Decode<'json> for CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "friday" => CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem::Friday,
            "monday" => CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem::Monday,
            "thursday" => CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem::Thursday,
            "tuesday" => CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem::Tuesday,
            "wednesday" => CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem::Wednesday,
            other => CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "balance_settings")]
impl crate::params::FormEncode for CreateBalanceSettingParamsPaymentsPayoutsScheduleWeeklyPayoutDaysItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "balance_settings")]
#[derive(Debug, Clone, Default)]
pub struct GetBalanceSettingParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "balance_settings")]
impl crate::params::FormEncode for GetBalanceSettingParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
