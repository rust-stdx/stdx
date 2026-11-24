#[cfg(any(feature = "balance_settings", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BalanceSettingsObject {
    BalanceSettings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
impl BalanceSettingsObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BalanceSettingsObject::BalanceSettings => "balance_settings",
            BalanceSettingsObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
impl Default for BalanceSettingsObject {
    fn default() -> Self {
        BalanceSettingsObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
impl<'json> json::Decode<'json> for BalanceSettingsObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BalanceSettingsObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balance_settings" => BalanceSettingsObject::BalanceSettings,
            other => BalanceSettingsObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BalanceSettingsResourceAutomaticTransferRuleType {
    TransferAll,
    TransferUpToAmount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
impl BalanceSettingsResourceAutomaticTransferRuleType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BalanceSettingsResourceAutomaticTransferRuleType::TransferAll => "transfer_all",
            BalanceSettingsResourceAutomaticTransferRuleType::TransferUpToAmount => "transfer_up_to_amount",
            BalanceSettingsResourceAutomaticTransferRuleType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
impl Default for BalanceSettingsResourceAutomaticTransferRuleType {
    fn default() -> Self {
        BalanceSettingsResourceAutomaticTransferRuleType::Unknown(String::new())
    }
}

#[cfg(any(feature = "balance_settings", feature = "events"))]
impl<'json> json::Decode<'json> for BalanceSettingsResourceAutomaticTransferRuleType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BalanceSettingsResourceAutomaticTransferRuleType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "transfer_all" => BalanceSettingsResourceAutomaticTransferRuleType::TransferAll,
            "transfer_up_to_amount" => BalanceSettingsResourceAutomaticTransferRuleType::TransferUpToAmount,
            other => BalanceSettingsResourceAutomaticTransferRuleType::Unknown(other.to_string()),
        })
    }
}
