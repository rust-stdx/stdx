#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, Default)]
pub struct CreateThreeDSecureAuthenticationParamsAcquirerDetails {
    #[doc = "The Acquirer BIN (specific to the directory_server)."]
    pub acquirer_bin: String,
    #[doc = "The two-letter country code of the acquirer ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub acquirer_country: String,
    #[doc = "The Merchant ID (or Card Acceptor ID) that your acquirer assigned you (specific to the directory_server)."]
    pub acquirer_merchant_id: String,
    #[doc = "The [merchant category code](https://en.wikipedia.org/wiki/Merchant_category_code) as defined by each payment system ..."]
    pub mcc: Option<String>,
    #[doc = "The merchant name assigned by the acquirer or payment system."]
    pub merchant_name: Option<String>,
    #[doc = "Requestor ID if you’re enrolled in the card network’s 3DS program."]
    pub requestor_id: Option<String>,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsAcquirerDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("acquirer_bin", &self.acquirer_bin);
        form.field("acquirer_country", &self.acquirer_country);
        form.field("acquirer_merchant_id", &self.acquirer_merchant_id);
        form.field_opt("mcc", self.mcc.as_ref());
        form.field_opt("merchant_name", self.merchant_name.as_ref());
        form.field_opt("requestor_id", self.requestor_id.as_ref());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShareType {
    DsSpecific,
    EmvStandard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShareType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShareType::DsSpecific => "ds_specific",
            CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShareType::EmvStandard => "emv_standard",
            CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShareType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShareType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShareType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ds_specific" => CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShareType::DsSpecific,
            "emv_standard" => CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShareType::EmvStandard,
            other => CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShareType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShareType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentInterval {
    Day,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentInterval::Day => "day",
            CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentInterval::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentInterval::Day,
            other => CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, Default)]
pub struct CreateThreeDSecureAuthenticationParamsPaymentMethodDataBillingDetailsAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<String>,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: Option<String>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "Country subdivision code defined in ISO 3166-2."]
    pub state: Option<String>,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsPaymentMethodDataBillingDetailsAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsSubmit {
    Always,
    IfFingerprintingNotSupported,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsSubmit {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsSubmit::Always => "always",
            CreateThreeDSecureAuthenticationParamsSubmit::IfFingerprintingNotSupported => {
                "if_fingerprinting_not_supported"
            }
            CreateThreeDSecureAuthenticationParamsSubmit::Never => "never",
            CreateThreeDSecureAuthenticationParamsSubmit::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsSubmit {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsSubmit::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreateThreeDSecureAuthenticationParamsSubmit::Always,
            "if_fingerprinting_not_supported" => {
                CreateThreeDSecureAuthenticationParamsSubmit::IfFingerprintingNotSupported
            }
            "never" => CreateThreeDSecureAuthenticationParamsSubmit::Never,
            other => CreateThreeDSecureAuthenticationParamsSubmit::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsSubmit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, Default)]
pub struct SubmitThreeDSecureAuthenticationParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The fingerprinting result of the issuer fingerprinting step."]
    pub fingerprinting_result: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for SubmitThreeDSecureAuthenticationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("fingerprinting_result", self.fingerprinting_result.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureAuthenticationStatus {
    Canceled,
    Error,
    Failed,
    RequiresChallenge,
    RequiresSubmission,
    Succeeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureAuthenticationStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureAuthenticationStatus::Canceled => "canceled",
            ThreeDSecureAuthenticationStatus::Error => "error",
            ThreeDSecureAuthenticationStatus::Failed => "failed",
            ThreeDSecureAuthenticationStatus::RequiresChallenge => "requires_challenge",
            ThreeDSecureAuthenticationStatus::RequiresSubmission => "requires_submission",
            ThreeDSecureAuthenticationStatus::Succeeded => "succeeded",
            ThreeDSecureAuthenticationStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl Default for ThreeDSecureAuthenticationStatus {
    fn default() -> Self {
        ThreeDSecureAuthenticationStatus::Unknown(String::new())
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureAuthenticationStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureAuthenticationStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => ThreeDSecureAuthenticationStatus::Canceled,
            "error" => ThreeDSecureAuthenticationStatus::Error,
            "failed" => ThreeDSecureAuthenticationStatus::Failed,
            "requires_challenge" => ThreeDSecureAuthenticationStatus::RequiresChallenge,
            "requires_submission" => ThreeDSecureAuthenticationStatus::RequiresSubmission,
            "succeeded" => ThreeDSecureAuthenticationStatus::Succeeded,
            other => ThreeDSecureAuthenticationStatus::Unknown(other.to_string()),
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
pub enum ThreeDSecureDetailsResult {
    AttemptAcknowledged,
    Authenticated,
    DataShareOnly,
    Exempted,
    Failed,
    NotSupported,
    ProcessingError,
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
impl ThreeDSecureDetailsResult {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureDetailsResult::AttemptAcknowledged => "attempt_acknowledged",
            ThreeDSecureDetailsResult::Authenticated => "authenticated",
            ThreeDSecureDetailsResult::DataShareOnly => "data_share_only",
            ThreeDSecureDetailsResult::Exempted => "exempted",
            ThreeDSecureDetailsResult::Failed => "failed",
            ThreeDSecureDetailsResult::NotSupported => "not_supported",
            ThreeDSecureDetailsResult::ProcessingError => "processing_error",
            ThreeDSecureDetailsResult::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for ThreeDSecureDetailsResult {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureDetailsResult::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "attempt_acknowledged" => ThreeDSecureDetailsResult::AttemptAcknowledged,
            "authenticated" => ThreeDSecureDetailsResult::Authenticated,
            "data_share_only" => ThreeDSecureDetailsResult::DataShareOnly,
            "exempted" => ThreeDSecureDetailsResult::Exempted,
            "failed" => ThreeDSecureDetailsResult::Failed,
            "not_supported" => ThreeDSecureDetailsResult::NotSupported,
            "processing_error" => ThreeDSecureDetailsResult::ProcessingError,
            other => ThreeDSecureDetailsResult::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceBrowserDetails {
    #[doc = "The HTTP accept headers from the cardholder's browser."]
    #[json(default)]
    pub accept_header: String,
    #[doc = "The color depth of the cardholder’s screen."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub color_depth: Option<i64>,
    #[doc = "The IP address of the browser."]
    #[json(default)]
    pub ip_address: String,
    #[doc = "The cardholder browser’s ability to execute Java."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub java_enabled: Option<bool>,
    #[doc = "The cardholder browser’s ability to execute JavaScript."]
    #[json(default)]
    pub javascript_enabled: bool,
    #[doc = "An IETF BCP 47 language tag representing the browser language."]
    #[json(default)]
    pub language: String,
    #[doc = "The total height of the cardholder’s screen in pixels."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub screen_height: Option<i64>,
    #[doc = "The total width of the cardholder’s screen in pixels."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub screen_width: Option<i64>,
    #[doc = "The time difference between UTC time and the local time of the cardholder’s browser, in minutes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub timezone_offset: Option<i64>,
    #[doc = "The browser user agent."]
    #[json(default)]
    pub user_agent: String,
}
