#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationAddress {
    #[doc = "City, district, suburb, town, or village."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationMerchantBlockedReason {
    AuthenticationRequired,
    BlockedForFraud,
    InvalidPayment,
    Other,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationMerchantBlockedReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationMerchantBlockedReason::AuthenticationRequired => {
                "authentication_required"
            }
            InsightsResourcesPaymentEvaluationMerchantBlockedReason::BlockedForFraud => "blocked_for_fraud",
            InsightsResourcesPaymentEvaluationMerchantBlockedReason::InvalidPayment => "invalid_payment",
            InsightsResourcesPaymentEvaluationMerchantBlockedReason::Other => "other",
            InsightsResourcesPaymentEvaluationMerchantBlockedReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for InsightsResourcesPaymentEvaluationMerchantBlockedReason {
    fn default() -> Self {
        InsightsResourcesPaymentEvaluationMerchantBlockedReason::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationMerchantBlockedReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationMerchantBlockedReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "authentication_required" => {
                InsightsResourcesPaymentEvaluationMerchantBlockedReason::AuthenticationRequired
            }
            "blocked_for_fraud" => InsightsResourcesPaymentEvaluationMerchantBlockedReason::BlockedForFraud,
            "invalid_payment" => InsightsResourcesPaymentEvaluationMerchantBlockedReason::InvalidPayment,
            "other" => InsightsResourcesPaymentEvaluationMerchantBlockedReason::Other,
            other => InsightsResourcesPaymentEvaluationMerchantBlockedReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType {
    Card,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType::Card => "card",
            InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType {
    fn default() -> Self {
        InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "card" => InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType::Card,
            other => {
                InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check {
    Fail,
    Pass,
    Unavailable,
    Unchecked,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::Fail => "fail",
            InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::Pass => "pass",
            InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::Unavailable => "unavailable",
            InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::Unchecked => "unchecked",
            InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check {
    fn default() -> Self {
        InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fail" => InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::Fail,
            "pass" => InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::Pass,
            "unavailable" => InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::Unavailable,
            "unchecked" => InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::Unchecked,
            other => InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    Elevated,
    Highest,
    Low,
    Normal,
    NotAssessed,
    UnknownValue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationSignalV2RiskLevel::Elevated => "elevated",
            InsightsResourcesPaymentEvaluationSignalV2RiskLevel::Highest => "highest",
            InsightsResourcesPaymentEvaluationSignalV2RiskLevel::Low => "low",
            InsightsResourcesPaymentEvaluationSignalV2RiskLevel::Normal => "normal",
            InsightsResourcesPaymentEvaluationSignalV2RiskLevel::NotAssessed => "not_assessed",
            InsightsResourcesPaymentEvaluationSignalV2RiskLevel::UnknownValue => "unknown",
            InsightsResourcesPaymentEvaluationSignalV2RiskLevel::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    fn default() -> Self {
        InsightsResourcesPaymentEvaluationSignalV2RiskLevel::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationSignalV2RiskLevel {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationSignalV2RiskLevel::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "elevated" => InsightsResourcesPaymentEvaluationSignalV2RiskLevel::Elevated,
            "highest" => InsightsResourcesPaymentEvaluationSignalV2RiskLevel::Highest,
            "low" => InsightsResourcesPaymentEvaluationSignalV2RiskLevel::Low,
            "normal" => InsightsResourcesPaymentEvaluationSignalV2RiskLevel::Normal,
            "not_assessed" => InsightsResourcesPaymentEvaluationSignalV2RiskLevel::NotAssessed,
            "unknown" => InsightsResourcesPaymentEvaluationSignalV2RiskLevel::UnknownValue,
            other => InsightsResourcesPaymentEvaluationSignalV2RiskLevel::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum RadarValueListObject {
    RadarValueList,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl RadarValueListObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            RadarValueListObject::RadarValueList => "radar.value_list",
            RadarValueListObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for RadarValueListObject {
    fn default() -> Self {
        RadarValueListObject::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for RadarValueListObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(RadarValueListObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "radar.value_list" => RadarValueListObject::RadarValueList,
            other => RadarValueListObject::Unknown(other.to_string()),
        })
    }
}
