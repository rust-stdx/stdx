#[cfg(any(feature = "balance_settings", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BalanceSettingsResourcePayoutsStatus {
    Disabled,
    Enabled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
impl BalanceSettingsResourcePayoutsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BalanceSettingsResourcePayoutsStatus::Disabled => "disabled",
            BalanceSettingsResourcePayoutsStatus::Enabled => "enabled",
            BalanceSettingsResourcePayoutsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
impl Default for BalanceSettingsResourcePayoutsStatus {
    fn default() -> Self {
        BalanceSettingsResourcePayoutsStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
impl<'json> json::Decode<'json> for BalanceSettingsResourcePayoutsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BalanceSettingsResourcePayoutsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "disabled" => BalanceSettingsResourcePayoutsStatus::Disabled,
            "enabled" => BalanceSettingsResourcePayoutsStatus::Enabled,
            other => BalanceSettingsResourcePayoutsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "balance_settings")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval {
    Daily,
    Manual,
    Monthly,
    Weekly,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "balance_settings")]
impl CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval::Daily => "daily",
            CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval::Manual => "manual",
            CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval::Monthly => "monthly",
            CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval::Weekly => "weekly",
            CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "balance_settings")]
impl<'json> json::Decode<'json> for CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "daily" => CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval::Daily,
            "manual" => CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval::Manual,
            "monthly" => CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval::Monthly,
            "weekly" => CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval::Weekly,
            other => CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "balance_settings")]
impl crate::params::FormEncode for CreateBalanceSettingParamsPaymentsPayoutsScheduleInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
