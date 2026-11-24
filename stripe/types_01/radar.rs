#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct CreateRadarPaymentEvaluationParamsClientDeviceMetadataDetails {
    #[doc = "ID for the Radar Session to associate with the payment evaluation."]
    pub radar_session: String,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for CreateRadarPaymentEvaluationParamsClientDeviceMetadataDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("radar_session", &self.radar_session);
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType {
    OneOff,
    Recurring,
    SetupOneOff,
    SetupRecurring,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType::OneOff => "one_off",
            CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType::Recurring => {
                "recurring"
            }
            CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType::SetupOneOff => {
                "setup_one_off"
            }
            CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType::SetupRecurring => {
                "setup_recurring"
            }
            CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json>
    for CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "one_off" => CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType::OneOff,
            "recurring" => {
                CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType::Recurring
            }
            "setup_one_off" => {
                CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType::SetupOneOff
            }
            "setup_recurring" => {
                CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType::SetupRecurring
            }
            other => CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsMoneyMovementType {
    Card,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsMoneyMovementType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsMoneyMovementType::Card => "card",
            CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsMoneyMovementType::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json>
    for CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsMoneyMovementType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsMoneyMovementType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "card" => CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsMoneyMovementType::Card,
            other => CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsMoneyMovementType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode
    for CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsMoneyMovementType
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationCustomerDetails {
    #[doc = "The ID of the customer associated with the payment evaluation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[doc = "The ID of the Account representing the customer associated with the payment evaluation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "The customer's email address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The customer's full name or business name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The customer's phone number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationEventType {
    DisputeOpened,
    EarlyFraudWarningReceived,
    Refunded,
    UserInterventionRaised,
    UserInterventionResolved,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationEventType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationEventType::DisputeOpened => "dispute_opened",
            InsightsResourcesPaymentEvaluationEventType::EarlyFraudWarningReceived => "early_fraud_warning_received",
            InsightsResourcesPaymentEvaluationEventType::Refunded => "refunded",
            InsightsResourcesPaymentEvaluationEventType::UserInterventionRaised => "user_intervention_raised",
            InsightsResourcesPaymentEvaluationEventType::UserInterventionResolved => "user_intervention_resolved",
            InsightsResourcesPaymentEvaluationEventType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for InsightsResourcesPaymentEvaluationEventType {
    fn default() -> Self {
        InsightsResourcesPaymentEvaluationEventType::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationEventType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationEventType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "dispute_opened" => InsightsResourcesPaymentEvaluationEventType::DisputeOpened,
            "early_fraud_warning_received" => InsightsResourcesPaymentEvaluationEventType::EarlyFraudWarningReceived,
            "refunded" => InsightsResourcesPaymentEvaluationEventType::Refunded,
            "user_intervention_raised" => InsightsResourcesPaymentEvaluationEventType::UserInterventionRaised,
            "user_intervention_resolved" => InsightsResourcesPaymentEvaluationEventType::UserInterventionResolved,
            other => InsightsResourcesPaymentEvaluationEventType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence {
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence::OffSession => "off_session",
            InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence::OnSession => "on_session",
            InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence::OffSession,
            "on_session" => InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence::OnSession,
            other => InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom {
    #[doc = "Custom type of user intervention raised."]
    #[json(rename = "type", default)]
    pub r#type: String,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum RadarValueListItemObject {
    RadarValueListItem,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl RadarValueListItemObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            RadarValueListItemObject::RadarValueListItem => "radar.value_list_item",
            RadarValueListItemObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for RadarValueListItemObject {
    fn default() -> Self {
        RadarValueListItemObject::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for RadarValueListItemObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(RadarValueListItemObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "radar.value_list_item" => RadarValueListItemObject::RadarValueListItem,
            other => RadarValueListItemObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum RadarValueListItemType {
    Account,
    CardBin,
    CardFingerprint,
    CaseSensitiveString,
    Country,
    CryptoFingerprint,
    CustomerId,
    Email,
    IpAddress,
    SepaDebitFingerprint,
    String,
    UsBankAccountFingerprint,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl RadarValueListItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            RadarValueListItemType::Account => "account",
            RadarValueListItemType::CardBin => "card_bin",
            RadarValueListItemType::CardFingerprint => "card_fingerprint",
            RadarValueListItemType::CaseSensitiveString => "case_sensitive_string",
            RadarValueListItemType::Country => "country",
            RadarValueListItemType::CryptoFingerprint => "crypto_fingerprint",
            RadarValueListItemType::CustomerId => "customer_id",
            RadarValueListItemType::Email => "email",
            RadarValueListItemType::IpAddress => "ip_address",
            RadarValueListItemType::SepaDebitFingerprint => "sepa_debit_fingerprint",
            RadarValueListItemType::String => "string",
            RadarValueListItemType::UsBankAccountFingerprint => "us_bank_account_fingerprint",
            RadarValueListItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for RadarValueListItemType {
    fn default() -> Self {
        RadarValueListItemType::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for RadarValueListItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(RadarValueListItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => RadarValueListItemType::Account,
            "card_bin" => RadarValueListItemType::CardBin,
            "card_fingerprint" => RadarValueListItemType::CardFingerprint,
            "case_sensitive_string" => RadarValueListItemType::CaseSensitiveString,
            "country" => RadarValueListItemType::Country,
            "crypto_fingerprint" => RadarValueListItemType::CryptoFingerprint,
            "customer_id" => RadarValueListItemType::CustomerId,
            "email" => RadarValueListItemType::Email,
            "ip_address" => RadarValueListItemType::IpAddress,
            "sepa_debit_fingerprint" => RadarValueListItemType::SepaDebitFingerprint,
            "string" => RadarValueListItemType::String,
            "us_bank_account_fingerprint" => RadarValueListItemType::UsBankAccountFingerprint,
            other => RadarValueListItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct UpdateRadarValueListParams {
    #[doc = "The name of the value list for use in rules."]
    pub alias: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The human-readable name of the value list."]
    pub name: Option<String>,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for UpdateRadarValueListParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("alias", self.alias.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("name", self.name.as_ref());
    }
}
