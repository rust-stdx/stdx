#[cfg(any(feature = "balance_settings", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BalanceSettingsResourcePayoutScheduleInterval {
    Daily,
    Manual,
    Monthly,
    Weekly,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
impl BalanceSettingsResourcePayoutScheduleInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BalanceSettingsResourcePayoutScheduleInterval::Daily => "daily",
            BalanceSettingsResourcePayoutScheduleInterval::Manual => "manual",
            BalanceSettingsResourcePayoutScheduleInterval::Monthly => "monthly",
            BalanceSettingsResourcePayoutScheduleInterval::Weekly => "weekly",
            BalanceSettingsResourcePayoutScheduleInterval::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
impl<'json> json::Decode<'json> for BalanceSettingsResourcePayoutScheduleInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BalanceSettingsResourcePayoutScheduleInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "daily" => BalanceSettingsResourcePayoutScheduleInterval::Daily,
            "manual" => BalanceSettingsResourcePayoutScheduleInterval::Manual,
            "monthly" => BalanceSettingsResourcePayoutScheduleInterval::Monthly,
            "weekly" => BalanceSettingsResourcePayoutScheduleInterval::Weekly,
            other => BalanceSettingsResourcePayoutScheduleInterval::Unknown(other.to_string()),
        })
    }
}
