#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, Default)]
pub struct CancelThreeDSecureAuthenticationParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CancelThreeDSecureAuthenticationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsChannelType {
    Browser,
    ThreeRI,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsChannelType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsChannelType::Browser => "browser",
            CreateThreeDSecureAuthenticationParamsChannelType::ThreeRI => "three_r_i",
            CreateThreeDSecureAuthenticationParamsChannelType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsChannelType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsChannelType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "browser" => CreateThreeDSecureAuthenticationParamsChannelType::Browser,
            "three_r_i" => CreateThreeDSecureAuthenticationParamsChannelType::ThreeRI,
            other => CreateThreeDSecureAuthenticationParamsChannelType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsChannelType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionlessType {
    LowRisk,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionlessType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionlessType::LowRisk => "low_risk",
            CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionlessType::None => "none",
            CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionlessType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionlessType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionlessType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "low_risk" => CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionlessType::LowRisk,
            "none" => CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionlessType::None,
            other => CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionlessType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionlessType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureAuthenticationMessageCategory {
    NonPaymentAuthentication,
    PaymentAuthentication,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureAuthenticationMessageCategory {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureAuthenticationMessageCategory::NonPaymentAuthentication => "non_payment_authentication",
            ThreeDSecureAuthenticationMessageCategory::PaymentAuthentication => "payment_authentication",
            ThreeDSecureAuthenticationMessageCategory::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl Default for ThreeDSecureAuthenticationMessageCategory {
    fn default() -> Self {
        ThreeDSecureAuthenticationMessageCategory::Unknown(String::new())
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureAuthenticationMessageCategory {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureAuthenticationMessageCategory::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "non_payment_authentication" => ThreeDSecureAuthenticationMessageCategory::NonPaymentAuthentication,
            "payment_authentication" => ThreeDSecureAuthenticationMessageCategory::PaymentAuthentication,
            other => ThreeDSecureAuthenticationMessageCategory::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureDetailsChargeExemptionIndicator {
    LowRisk,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl ThreeDSecureDetailsChargeExemptionIndicator {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureDetailsChargeExemptionIndicator::LowRisk => "low_risk",
            ThreeDSecureDetailsChargeExemptionIndicator::None => "none",
            ThreeDSecureDetailsChargeExemptionIndicator::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl Default for ThreeDSecureDetailsChargeExemptionIndicator {
    fn default() -> Self {
        ThreeDSecureDetailsChargeExemptionIndicator::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for ThreeDSecureDetailsChargeExemptionIndicator {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureDetailsChargeExemptionIndicator::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "low_risk" => ThreeDSecureDetailsChargeExemptionIndicator::LowRisk,
            "none" => ThreeDSecureDetailsChargeExemptionIndicator::None,
            other => ThreeDSecureDetailsChargeExemptionIndicator::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureDetailsVersion {
    N102,
    N210,
    N220,
    N230,
    N231,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl ThreeDSecureDetailsVersion {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureDetailsVersion::N102 => "1.0.2",
            ThreeDSecureDetailsVersion::N210 => "2.1.0",
            ThreeDSecureDetailsVersion::N220 => "2.2.0",
            ThreeDSecureDetailsVersion::N230 => "2.3.0",
            ThreeDSecureDetailsVersion::N231 => "2.3.1",
            ThreeDSecureDetailsVersion::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for ThreeDSecureDetailsVersion {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureDetailsVersion::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "1.0.2" => ThreeDSecureDetailsVersion::N102,
            "2.1.0" => ThreeDSecureDetailsVersion::N210,
            "2.2.0" => ThreeDSecureDetailsVersion::N220,
            "2.3.0" => ThreeDSecureDetailsVersion::N230,
            "2.3.1" => ThreeDSecureDetailsVersion::N231,
            other => ThreeDSecureDetailsVersion::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureStandaloneResourceChannelType {
    Browser,
    ThreeRI,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureStandaloneResourceChannelType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureStandaloneResourceChannelType::Browser => "browser",
            ThreeDSecureStandaloneResourceChannelType::ThreeRI => "three_r_i",
            ThreeDSecureStandaloneResourceChannelType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl Default for ThreeDSecureStandaloneResourceChannelType {
    fn default() -> Self {
        ThreeDSecureStandaloneResourceChannelType::Unknown(String::new())
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureStandaloneResourceChannelType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureStandaloneResourceChannelType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "browser" => ThreeDSecureStandaloneResourceChannelType::Browser,
            "three_r_i" => ThreeDSecureStandaloneResourceChannelType::ThreeRI,
            other => ThreeDSecureStandaloneResourceChannelType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureStandaloneResourceExpiryType {
    Date,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureStandaloneResourceExpiryType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureStandaloneResourceExpiryType::Date => "date",
            ThreeDSecureStandaloneResourceExpiryType::Never => "never",
            ThreeDSecureStandaloneResourceExpiryType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl Default for ThreeDSecureStandaloneResourceExpiryType {
    fn default() -> Self {
        ThreeDSecureStandaloneResourceExpiryType::Unknown(String::new())
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureStandaloneResourceExpiryType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureStandaloneResourceExpiryType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "date" => ThreeDSecureStandaloneResourceExpiryType::Date,
            "never" => ThreeDSecureStandaloneResourceExpiryType::Never,
            other => ThreeDSecureStandaloneResourceExpiryType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureStandaloneResourceFlowPreferenceType {
    Challenge,
    DataShare,
    Frictionless,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureStandaloneResourceFlowPreferenceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureStandaloneResourceFlowPreferenceType::Challenge => "challenge",
            ThreeDSecureStandaloneResourceFlowPreferenceType::DataShare => "data_share",
            ThreeDSecureStandaloneResourceFlowPreferenceType::Frictionless => "frictionless",
            ThreeDSecureStandaloneResourceFlowPreferenceType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl Default for ThreeDSecureStandaloneResourceFlowPreferenceType {
    fn default() -> Self {
        ThreeDSecureStandaloneResourceFlowPreferenceType::Unknown(String::new())
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureStandaloneResourceFlowPreferenceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureStandaloneResourceFlowPreferenceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "challenge" => ThreeDSecureStandaloneResourceFlowPreferenceType::Challenge,
            "data_share" => ThreeDSecureStandaloneResourceFlowPreferenceType::DataShare,
            "frictionless" => ThreeDSecureStandaloneResourceFlowPreferenceType::Frictionless,
            other => ThreeDSecureStandaloneResourceFlowPreferenceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion {
    N210,
    N220,
    N231,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion::N210 => "2.1.0",
            ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion::N220 => "2.2.0",
            ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion::N231 => "2.3.1",
            ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl Default for ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion {
    fn default() -> Self {
        ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion::Unknown(String::new())
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "2.1.0" => ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion::N210,
            "2.2.0" => ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion::N220,
            "2.3.1" => ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion::N231,
            other => ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion::Unknown(other.to_string()),
        })
    }
}
