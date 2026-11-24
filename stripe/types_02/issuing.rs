#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct ApproveIssuingAuthorizationParams {
    #[doc = "If the authorization's `pending_request.is_amount_controllable` property is `true`, you may provide this value to con..."]
    pub amount: Option<i64>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ApproveIssuingAuthorizationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
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

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateIssuingCardParamsReplacementReason {
    Damaged,
    Expired,
    Lost,
    Stolen,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "issuing")]
impl CreateIssuingCardParamsReplacementReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateIssuingCardParamsReplacementReason::Damaged => "damaged",
            CreateIssuingCardParamsReplacementReason::Expired => "expired",
            CreateIssuingCardParamsReplacementReason::Lost => "lost",
            CreateIssuingCardParamsReplacementReason::Stolen => "stolen",
            CreateIssuingCardParamsReplacementReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "issuing")]
impl<'json> json::Decode<'json> for CreateIssuingCardParamsReplacementReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateIssuingCardParamsReplacementReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "damaged" => CreateIssuingCardParamsReplacementReason::Damaged,
            "expired" => CreateIssuingCardParamsReplacementReason::Expired,
            "lost" => CreateIssuingCardParamsReplacementReason::Lost,
            "stolen" => CreateIssuingCardParamsReplacementReason::Stolen,
            other => CreateIssuingCardParamsReplacementReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardParamsReplacementReason {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateIssuingCardParamsShippingAddressValidationMode {
    Disabled,
    NormalizationOnly,
    ValidationAndNormalization,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "issuing")]
impl CreateIssuingCardParamsShippingAddressValidationMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateIssuingCardParamsShippingAddressValidationMode::Disabled => "disabled",
            CreateIssuingCardParamsShippingAddressValidationMode::NormalizationOnly => "normalization_only",
            CreateIssuingCardParamsShippingAddressValidationMode::ValidationAndNormalization => {
                "validation_and_normalization"
            }
            CreateIssuingCardParamsShippingAddressValidationMode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "issuing")]
impl<'json> json::Decode<'json> for CreateIssuingCardParamsShippingAddressValidationMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateIssuingCardParamsShippingAddressValidationMode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "disabled" => CreateIssuingCardParamsShippingAddressValidationMode::Disabled,
            "normalization_only" => CreateIssuingCardParamsShippingAddressValidationMode::NormalizationOnly,
            "validation_and_normalization" => {
                CreateIssuingCardParamsShippingAddressValidationMode::ValidationAndNormalization
            }
            other => CreateIssuingCardParamsShippingAddressValidationMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardParamsShippingAddressValidationMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateIssuingCardParamsShippingService {
    Express,
    Priority,
    Standard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "issuing")]
impl CreateIssuingCardParamsShippingService {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateIssuingCardParamsShippingService::Express => "express",
            CreateIssuingCardParamsShippingService::Priority => "priority",
            CreateIssuingCardParamsShippingService::Standard => "standard",
            CreateIssuingCardParamsShippingService::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "issuing")]
impl<'json> json::Decode<'json> for CreateIssuingCardParamsShippingService {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateIssuingCardParamsShippingService::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "express" => CreateIssuingCardParamsShippingService::Express,
            "priority" => CreateIssuingCardParamsShippingService::Priority,
            "standard" => CreateIssuingCardParamsShippingService::Standard,
            other => CreateIssuingCardParamsShippingService::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardParamsShippingService {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateIssuingCardParamsShippingType {
    Bulk,
    Individual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "issuing")]
impl CreateIssuingCardParamsShippingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateIssuingCardParamsShippingType::Bulk => "bulk",
            CreateIssuingCardParamsShippingType::Individual => "individual",
            CreateIssuingCardParamsShippingType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "issuing")]
impl<'json> json::Decode<'json> for CreateIssuingCardParamsShippingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateIssuingCardParamsShippingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bulk" => CreateIssuingCardParamsShippingType::Bulk,
            "individual" => CreateIssuingCardParamsShippingType::Individual,
            other => CreateIssuingCardParamsShippingType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardParamsShippingType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingCardholderParamsIndividualCardIssuingUserTermsAcceptance {
    #[doc = "The Unix timestamp marking when the cardholder accepted the Authorized User Terms."]
    pub date: Option<i64>,
    #[doc = "The IP address from which the cardholder accepted the Authorized User Terms."]
    pub ip: Option<String>,
    #[doc = "The user agent of the browser from which the cardholder accepted the Authorized User Terms."]
    pub user_agent: Option<json::OwnedValue>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardholderParamsIndividualCardIssuingUserTermsAcceptance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("date", self.date.as_ref());
        form.field_opt("ip", self.ip.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingCardholderParamsIndividualDob {
    #[doc = "The day of birth, between 1 and 31."]
    pub day: i64,
    #[doc = "The month of birth, between 1 and 12."]
    pub month: i64,
    #[doc = "The four-digit year of birth."]
    pub year: i64,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardholderParamsIndividualDob {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("day", &self.day);
        form.field("month", &self.month);
        form.field("year", &self.year);
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingCardholderParamsIndividualVerificationDocument {
    #[doc = "The back of an ID returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value of `ide..."]
    pub back: Option<String>,
    #[doc = "The front of an ID returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value of `id..."]
    pub front: Option<String>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardholderParamsIndividualVerificationDocument {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("back", self.back.as_ref());
        form.field_opt("front", self.front.as_ref());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingDisputeParamsEvidenceFraudulent {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    pub additional_documentation: Option<json::OwnedValue>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    pub explanation: Option<json::OwnedValue>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingDisputeParamsEvidenceFraudulent {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateIssuingDisputeParamsEvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NoValidAuthorization,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "issuing")]
impl CreateIssuingDisputeParamsEvidenceReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateIssuingDisputeParamsEvidenceReason::Canceled => "canceled",
            CreateIssuingDisputeParamsEvidenceReason::Duplicate => "duplicate",
            CreateIssuingDisputeParamsEvidenceReason::Fraudulent => "fraudulent",
            CreateIssuingDisputeParamsEvidenceReason::MerchandiseNotAsDescribed => "merchandise_not_as_described",
            CreateIssuingDisputeParamsEvidenceReason::NoValidAuthorization => "no_valid_authorization",
            CreateIssuingDisputeParamsEvidenceReason::NotReceived => "not_received",
            CreateIssuingDisputeParamsEvidenceReason::Other => "other",
            CreateIssuingDisputeParamsEvidenceReason::ServiceNotAsDescribed => "service_not_as_described",
            CreateIssuingDisputeParamsEvidenceReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "issuing")]
impl<'json> json::Decode<'json> for CreateIssuingDisputeParamsEvidenceReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateIssuingDisputeParamsEvidenceReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => CreateIssuingDisputeParamsEvidenceReason::Canceled,
            "duplicate" => CreateIssuingDisputeParamsEvidenceReason::Duplicate,
            "fraudulent" => CreateIssuingDisputeParamsEvidenceReason::Fraudulent,
            "merchandise_not_as_described" => CreateIssuingDisputeParamsEvidenceReason::MerchandiseNotAsDescribed,
            "no_valid_authorization" => CreateIssuingDisputeParamsEvidenceReason::NoValidAuthorization,
            "not_received" => CreateIssuingDisputeParamsEvidenceReason::NotReceived,
            "other" => CreateIssuingDisputeParamsEvidenceReason::Other,
            "service_not_as_described" => CreateIssuingDisputeParamsEvidenceReason::ServiceNotAsDescribed,
            other => CreateIssuingDisputeParamsEvidenceReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingDisputeParamsEvidenceReason {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingDisputeParamsTreasury {
    #[doc = "The ID of the ReceivedDebit to initiate an Issuings dispute for."]
    pub received_debit: String,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingDisputeParamsTreasury {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("received_debit", &self.received_debit);
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
pub enum IssuingAuthorizationAuthenticationExemptionClaimedBy {
    Acquirer,
    Issuer,
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
impl IssuingAuthorizationAuthenticationExemptionClaimedBy {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingAuthorizationAuthenticationExemptionClaimedBy::Acquirer => "acquirer",
            IssuingAuthorizationAuthenticationExemptionClaimedBy::Issuer => "issuer",
            IssuingAuthorizationAuthenticationExemptionClaimedBy::Unknown(value) => value,
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
impl Default for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn default() -> Self {
        IssuingAuthorizationAuthenticationExemptionClaimedBy::Unknown(String::new())
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
impl<'json> json::Decode<'json> for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingAuthorizationAuthenticationExemptionClaimedBy::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acquirer" => IssuingAuthorizationAuthenticationExemptionClaimedBy::Acquirer,
            "issuer" => IssuingAuthorizationAuthenticationExemptionClaimedBy::Issuer,
            other => IssuingAuthorizationAuthenticationExemptionClaimedBy::Unknown(other.to_string()),
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingAuthorizationFleetCardholderPromptData {
    #[doc = "Deprecated An alphanumeric ID, though typical point of sales only support numeric entry."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alphanumeric_id: Option<String>,
    #[doc = "Driver ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub driver_id: Option<String>,
    #[doc = "Odometer reading."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub odometer: Option<i64>,
    #[doc = "An alphanumeric ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unspecified_id: Option<String>,
    #[doc = "User ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "Vehicle number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub vehicle_number: Option<String>,
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
pub enum IssuingAuthorizationFleetDataServiceType {
    FullService,
    NonFuelTransaction,
    SelfService,
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
impl IssuingAuthorizationFleetDataServiceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingAuthorizationFleetDataServiceType::FullService => "full_service",
            IssuingAuthorizationFleetDataServiceType::NonFuelTransaction => "non_fuel_transaction",
            IssuingAuthorizationFleetDataServiceType::SelfService => "self_service",
            IssuingAuthorizationFleetDataServiceType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for IssuingAuthorizationFleetDataServiceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingAuthorizationFleetDataServiceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "full_service" => IssuingAuthorizationFleetDataServiceType::FullService,
            "non_fuel_transaction" => IssuingAuthorizationFleetDataServiceType::NonFuelTransaction,
            "self_service" => IssuingAuthorizationFleetDataServiceType::SelfService,
            other => IssuingAuthorizationFleetDataServiceType::Unknown(other.to_string()),
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
pub enum IssuingAuthorizationFraudChallengeUndeliverableReason {
    NoPhoneNumber,
    UnsupportedPhoneNumber,
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
impl IssuingAuthorizationFraudChallengeUndeliverableReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingAuthorizationFraudChallengeUndeliverableReason::NoPhoneNumber => "no_phone_number",
            IssuingAuthorizationFraudChallengeUndeliverableReason::UnsupportedPhoneNumber => "unsupported_phone_number",
            IssuingAuthorizationFraudChallengeUndeliverableReason::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for IssuingAuthorizationFraudChallengeUndeliverableReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingAuthorizationFraudChallengeUndeliverableReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "no_phone_number" => IssuingAuthorizationFraudChallengeUndeliverableReason::NoPhoneNumber,
            "unsupported_phone_number" => IssuingAuthorizationFraudChallengeUndeliverableReason::UnsupportedPhoneNumber,
            other => IssuingAuthorizationFraudChallengeUndeliverableReason::Unknown(other.to_string()),
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingAuthorizationMerchantData {
    #[doc = "A categorization of the seller's type of business."]
    #[json(default)]
    pub category: String,
    #[doc = "The merchant category code for the seller’s business"]
    #[json(default)]
    pub category_code: String,
    #[doc = "City where the seller is located"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Country where the seller is located"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Name of the seller"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Identifier assigned to the seller by the card network."]
    #[json(default)]
    pub network_id: String,
    #[doc = "Postal code where the seller is located"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "State where the seller is located"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The seller's tax identification number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[doc = "An ID assigned by the seller to the location of the sale."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub terminal_id: Option<String>,
    #[doc = "URL provided by the merchant on a 3DS request"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingAuthorizationTreasury {
    #[doc = "The array of [ReceivedCredits](https://docs.stripe.com/api/treasury/received_credits) associated with this authorization"]
    #[json(default)]
    pub received_credits: Vec<String>,
    #[doc = "The array of [ReceivedDebits](https://docs.stripe.com/api/treasury/received_debits) associated with this authorization"]
    #[json(default)]
    pub received_debits: Vec<String>,
    #[doc = "The Treasury [Transaction](https://docs.stripe.com/api/treasury/transactions) associated with this authorization"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,
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
pub enum IssuingAuthorizationVerificationDataAddressLine1Check {
    Match,
    Mismatch,
    NotProvided,
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
impl IssuingAuthorizationVerificationDataAddressLine1Check {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingAuthorizationVerificationDataAddressLine1Check::Match => "match",
            IssuingAuthorizationVerificationDataAddressLine1Check::Mismatch => "mismatch",
            IssuingAuthorizationVerificationDataAddressLine1Check::NotProvided => "not_provided",
            IssuingAuthorizationVerificationDataAddressLine1Check::Unknown(value) => value,
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
impl Default for IssuingAuthorizationVerificationDataAddressLine1Check {
    fn default() -> Self {
        IssuingAuthorizationVerificationDataAddressLine1Check::Unknown(String::new())
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
impl<'json> json::Decode<'json> for IssuingAuthorizationVerificationDataAddressLine1Check {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingAuthorizationVerificationDataAddressLine1Check::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "match" => IssuingAuthorizationVerificationDataAddressLine1Check::Match,
            "mismatch" => IssuingAuthorizationVerificationDataAddressLine1Check::Mismatch,
            "not_provided" => IssuingAuthorizationVerificationDataAddressLine1Check::NotProvided,
            other => IssuingAuthorizationVerificationDataAddressLine1Check::Unknown(other.to_string()),
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
pub enum IssuingCardReplacementReason {
    Damaged,
    Expired,
    FulfillmentError,
    Lost,
    Stolen,
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
impl IssuingCardReplacementReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingCardReplacementReason::Damaged => "damaged",
            IssuingCardReplacementReason::Expired => "expired",
            IssuingCardReplacementReason::FulfillmentError => "fulfillment_error",
            IssuingCardReplacementReason::Lost => "lost",
            IssuingCardReplacementReason::Stolen => "stolen",
            IssuingCardReplacementReason::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for IssuingCardReplacementReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingCardReplacementReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "damaged" => IssuingCardReplacementReason::Damaged,
            "expired" => IssuingCardReplacementReason::Expired,
            "fulfillment_error" => IssuingCardReplacementReason::FulfillmentError,
            "lost" => IssuingCardReplacementReason::Lost,
            "stolen" => IssuingCardReplacementReason::Stolen,
            other => IssuingCardReplacementReason::Unknown(other.to_string()),
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingCardShippingCustoms {
    #[doc = "A registration number used for customs in Europe."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eori_number: Option<String>,
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
pub enum IssuingCardholderRequirementsDisabledReason {
    Listed,
    RejectedListed,
    RequirementsPastDue,
    UnderReview,
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
impl IssuingCardholderRequirementsDisabledReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingCardholderRequirementsDisabledReason::Listed => "listed",
            IssuingCardholderRequirementsDisabledReason::RejectedListed => "rejected.listed",
            IssuingCardholderRequirementsDisabledReason::RequirementsPastDue => "requirements.past_due",
            IssuingCardholderRequirementsDisabledReason::UnderReview => "under_review",
            IssuingCardholderRequirementsDisabledReason::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for IssuingCardholderRequirementsDisabledReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingCardholderRequirementsDisabledReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "listed" => IssuingCardholderRequirementsDisabledReason::Listed,
            "rejected.listed" => IssuingCardholderRequirementsDisabledReason::RejectedListed,
            "requirements.past_due" => IssuingCardholderRequirementsDisabledReason::RequirementsPastDue,
            "under_review" => IssuingCardholderRequirementsDisabledReason::UnderReview,
            other => IssuingCardholderRequirementsDisabledReason::Unknown(other.to_string()),
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingCardholderUserTermsAcceptance {
    #[doc = "The Unix timestamp marking when the cardholder accepted the Authorized User Terms."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
    #[doc = "The IP address from which the cardholder accepted the Authorized User Terms."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The user agent of the browser from which the cardholder accepted the Authorized User Terms."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
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
pub enum IssuingDisputeEvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NoValidAuthorization,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
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
impl IssuingDisputeEvidenceReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingDisputeEvidenceReason::Canceled => "canceled",
            IssuingDisputeEvidenceReason::Duplicate => "duplicate",
            IssuingDisputeEvidenceReason::Fraudulent => "fraudulent",
            IssuingDisputeEvidenceReason::MerchandiseNotAsDescribed => "merchandise_not_as_described",
            IssuingDisputeEvidenceReason::NoValidAuthorization => "no_valid_authorization",
            IssuingDisputeEvidenceReason::NotReceived => "not_received",
            IssuingDisputeEvidenceReason::Other => "other",
            IssuingDisputeEvidenceReason::ServiceNotAsDescribed => "service_not_as_described",
            IssuingDisputeEvidenceReason::Unknown(value) => value,
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
impl Default for IssuingDisputeEvidenceReason {
    fn default() -> Self {
        IssuingDisputeEvidenceReason::Unknown(String::new())
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
impl<'json> json::Decode<'json> for IssuingDisputeEvidenceReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingDisputeEvidenceReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => IssuingDisputeEvidenceReason::Canceled,
            "duplicate" => IssuingDisputeEvidenceReason::Duplicate,
            "fraudulent" => IssuingDisputeEvidenceReason::Fraudulent,
            "merchandise_not_as_described" => IssuingDisputeEvidenceReason::MerchandiseNotAsDescribed,
            "no_valid_authorization" => IssuingDisputeEvidenceReason::NoValidAuthorization,
            "not_received" => IssuingDisputeEvidenceReason::NotReceived,
            "other" => IssuingDisputeEvidenceReason::Other,
            "service_not_as_described" => IssuingDisputeEvidenceReason::ServiceNotAsDescribed,
            other => IssuingDisputeEvidenceReason::Unknown(other.to_string()),
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
pub enum IssuingDisputeLossReason {
    CardholderAuthenticationIssuerLiability,
    Eci5TokenTransactionWithTavv,
    ExcessDisputesInTimeframe,
    HasNotMetTheMinimumDisputeAmountRequirements,
    InvalidDuplicateDispute,
    InvalidIncorrectAmountDispute,
    InvalidNoAuthorization,
    InvalidUseOfDisputes,
    MerchandiseDeliveredOrShipped,
    MerchandiseOrServiceAsDescribed,
    NotCancelled,
    Other,
    RefundIssued,
    SubmittedBeyondAllowableTimeLimit,
    Transaction3dsRequired,
    TransactionApprovedAfterPriorFraudDispute,
    TransactionAuthorized,
    TransactionElectronicallyRead,
    TransactionQualifiesForVisaEasyPaymentService,
    TransactionUnattended,
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
impl IssuingDisputeLossReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingDisputeLossReason::CardholderAuthenticationIssuerLiability => {
                "cardholder_authentication_issuer_liability"
            }
            IssuingDisputeLossReason::Eci5TokenTransactionWithTavv => "eci5_token_transaction_with_tavv",
            IssuingDisputeLossReason::ExcessDisputesInTimeframe => "excess_disputes_in_timeframe",
            IssuingDisputeLossReason::HasNotMetTheMinimumDisputeAmountRequirements => {
                "has_not_met_the_minimum_dispute_amount_requirements"
            }
            IssuingDisputeLossReason::InvalidDuplicateDispute => "invalid_duplicate_dispute",
            IssuingDisputeLossReason::InvalidIncorrectAmountDispute => "invalid_incorrect_amount_dispute",
            IssuingDisputeLossReason::InvalidNoAuthorization => "invalid_no_authorization",
            IssuingDisputeLossReason::InvalidUseOfDisputes => "invalid_use_of_disputes",
            IssuingDisputeLossReason::MerchandiseDeliveredOrShipped => "merchandise_delivered_or_shipped",
            IssuingDisputeLossReason::MerchandiseOrServiceAsDescribed => "merchandise_or_service_as_described",
            IssuingDisputeLossReason::NotCancelled => "not_cancelled",
            IssuingDisputeLossReason::Other => "other",
            IssuingDisputeLossReason::RefundIssued => "refund_issued",
            IssuingDisputeLossReason::SubmittedBeyondAllowableTimeLimit => "submitted_beyond_allowable_time_limit",
            IssuingDisputeLossReason::Transaction3dsRequired => "transaction_3ds_required",
            IssuingDisputeLossReason::TransactionApprovedAfterPriorFraudDispute => {
                "transaction_approved_after_prior_fraud_dispute"
            }
            IssuingDisputeLossReason::TransactionAuthorized => "transaction_authorized",
            IssuingDisputeLossReason::TransactionElectronicallyRead => "transaction_electronically_read",
            IssuingDisputeLossReason::TransactionQualifiesForVisaEasyPaymentService => {
                "transaction_qualifies_for_visa_easy_payment_service"
            }
            IssuingDisputeLossReason::TransactionUnattended => "transaction_unattended",
            IssuingDisputeLossReason::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for IssuingDisputeLossReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingDisputeLossReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cardholder_authentication_issuer_liability" => {
                IssuingDisputeLossReason::CardholderAuthenticationIssuerLiability
            }
            "eci5_token_transaction_with_tavv" => IssuingDisputeLossReason::Eci5TokenTransactionWithTavv,
            "excess_disputes_in_timeframe" => IssuingDisputeLossReason::ExcessDisputesInTimeframe,
            "has_not_met_the_minimum_dispute_amount_requirements" => {
                IssuingDisputeLossReason::HasNotMetTheMinimumDisputeAmountRequirements
            }
            "invalid_duplicate_dispute" => IssuingDisputeLossReason::InvalidDuplicateDispute,
            "invalid_incorrect_amount_dispute" => IssuingDisputeLossReason::InvalidIncorrectAmountDispute,
            "invalid_no_authorization" => IssuingDisputeLossReason::InvalidNoAuthorization,
            "invalid_use_of_disputes" => IssuingDisputeLossReason::InvalidUseOfDisputes,
            "merchandise_delivered_or_shipped" => IssuingDisputeLossReason::MerchandiseDeliveredOrShipped,
            "merchandise_or_service_as_described" => IssuingDisputeLossReason::MerchandiseOrServiceAsDescribed,
            "not_cancelled" => IssuingDisputeLossReason::NotCancelled,
            "other" => IssuingDisputeLossReason::Other,
            "refund_issued" => IssuingDisputeLossReason::RefundIssued,
            "submitted_beyond_allowable_time_limit" => IssuingDisputeLossReason::SubmittedBeyondAllowableTimeLimit,
            "transaction_3ds_required" => IssuingDisputeLossReason::Transaction3dsRequired,
            "transaction_approved_after_prior_fraud_dispute" => {
                IssuingDisputeLossReason::TransactionApprovedAfterPriorFraudDispute
            }
            "transaction_authorized" => IssuingDisputeLossReason::TransactionAuthorized,
            "transaction_electronically_read" => IssuingDisputeLossReason::TransactionElectronicallyRead,
            "transaction_qualifies_for_visa_easy_payment_service" => {
                IssuingDisputeLossReason::TransactionQualifiesForVisaEasyPaymentService
            }
            "transaction_unattended" => IssuingDisputeLossReason::TransactionUnattended,
            other => IssuingDisputeLossReason::Unknown(other.to_string()),
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
pub enum IssuingNetworkTokenDeviceType {
    Other,
    Phone,
    Watch,
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
impl IssuingNetworkTokenDeviceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingNetworkTokenDeviceType::Other => "other",
            IssuingNetworkTokenDeviceType::Phone => "phone",
            IssuingNetworkTokenDeviceType::Watch => "watch",
            IssuingNetworkTokenDeviceType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for IssuingNetworkTokenDeviceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingNetworkTokenDeviceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "other" => IssuingNetworkTokenDeviceType::Other,
            "phone" => IssuingNetworkTokenDeviceType::Phone,
            "watch" => IssuingNetworkTokenDeviceType::Watch,
            other => IssuingNetworkTokenDeviceType::Unknown(other.to_string()),
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
pub enum IssuingNetworkTokenWalletProviderCardNumberSource {
    App,
    Manual,
    OnFile,
    Other,
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
impl IssuingNetworkTokenWalletProviderCardNumberSource {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingNetworkTokenWalletProviderCardNumberSource::App => "app",
            IssuingNetworkTokenWalletProviderCardNumberSource::Manual => "manual",
            IssuingNetworkTokenWalletProviderCardNumberSource::OnFile => "on_file",
            IssuingNetworkTokenWalletProviderCardNumberSource::Other => "other",
            IssuingNetworkTokenWalletProviderCardNumberSource::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for IssuingNetworkTokenWalletProviderCardNumberSource {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingNetworkTokenWalletProviderCardNumberSource::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "app" => IssuingNetworkTokenWalletProviderCardNumberSource::App,
            "manual" => IssuingNetworkTokenWalletProviderCardNumberSource::Manual,
            "on_file" => IssuingNetworkTokenWalletProviderCardNumberSource::OnFile,
            "other" => IssuingNetworkTokenWalletProviderCardNumberSource::Other,
            other => IssuingNetworkTokenWalletProviderCardNumberSource::Unknown(other.to_string()),
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingPersonalizationDesignCarrierText {
    #[doc = "The footer body text of the carrier letter."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub footer_body: Option<String>,
    #[doc = "The footer title text of the carrier letter."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub footer_title: Option<String>,
    #[doc = "The header body text of the carrier letter."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub header_body: Option<String>,
    #[doc = "The header title text of the carrier letter."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub header_title: Option<String>,
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
pub enum IssuingPersonalizationDesignRejectionReasonsCardLogoItem {
    GeographicLocation,
    Inappropriate,
    NetworkName,
    NonBinaryImage,
    NonFiatCurrency,
    Other,
    OtherEntity,
    PromotionalMaterial,
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
impl IssuingPersonalizationDesignRejectionReasonsCardLogoItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingPersonalizationDesignRejectionReasonsCardLogoItem::GeographicLocation => "geographic_location",
            IssuingPersonalizationDesignRejectionReasonsCardLogoItem::Inappropriate => "inappropriate",
            IssuingPersonalizationDesignRejectionReasonsCardLogoItem::NetworkName => "network_name",
            IssuingPersonalizationDesignRejectionReasonsCardLogoItem::NonBinaryImage => "non_binary_image",
            IssuingPersonalizationDesignRejectionReasonsCardLogoItem::NonFiatCurrency => "non_fiat_currency",
            IssuingPersonalizationDesignRejectionReasonsCardLogoItem::Other => "other",
            IssuingPersonalizationDesignRejectionReasonsCardLogoItem::OtherEntity => "other_entity",
            IssuingPersonalizationDesignRejectionReasonsCardLogoItem::PromotionalMaterial => "promotional_material",
            IssuingPersonalizationDesignRejectionReasonsCardLogoItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for IssuingPersonalizationDesignRejectionReasonsCardLogoItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingPersonalizationDesignRejectionReasonsCardLogoItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "geographic_location" => IssuingPersonalizationDesignRejectionReasonsCardLogoItem::GeographicLocation,
            "inappropriate" => IssuingPersonalizationDesignRejectionReasonsCardLogoItem::Inappropriate,
            "network_name" => IssuingPersonalizationDesignRejectionReasonsCardLogoItem::NetworkName,
            "non_binary_image" => IssuingPersonalizationDesignRejectionReasonsCardLogoItem::NonBinaryImage,
            "non_fiat_currency" => IssuingPersonalizationDesignRejectionReasonsCardLogoItem::NonFiatCurrency,
            "other" => IssuingPersonalizationDesignRejectionReasonsCardLogoItem::Other,
            "other_entity" => IssuingPersonalizationDesignRejectionReasonsCardLogoItem::OtherEntity,
            "promotional_material" => IssuingPersonalizationDesignRejectionReasonsCardLogoItem::PromotionalMaterial,
            other => IssuingPersonalizationDesignRejectionReasonsCardLogoItem::Unknown(other.to_string()),
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
pub enum IssuingPhysicalBundleType {
    Custom,
    Standard,
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
impl IssuingPhysicalBundleType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingPhysicalBundleType::Custom => "custom",
            IssuingPhysicalBundleType::Standard => "standard",
            IssuingPhysicalBundleType::Unknown(value) => value,
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
impl Default for IssuingPhysicalBundleType {
    fn default() -> Self {
        IssuingPhysicalBundleType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for IssuingPhysicalBundleType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingPhysicalBundleType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "custom" => IssuingPhysicalBundleType::Custom,
            "standard" => IssuingPhysicalBundleType::Standard,
            other => IssuingPhysicalBundleType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListIssuingTokensParamsStatus {
    Active,
    Deleted,
    Requested,
    Suspended,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "issuing")]
impl ListIssuingTokensParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListIssuingTokensParamsStatus::Active => "active",
            ListIssuingTokensParamsStatus::Deleted => "deleted",
            ListIssuingTokensParamsStatus::Requested => "requested",
            ListIssuingTokensParamsStatus::Suspended => "suspended",
            ListIssuingTokensParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "issuing")]
impl<'json> json::Decode<'json> for ListIssuingTokensParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListIssuingTokensParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => ListIssuingTokensParamsStatus::Active,
            "deleted" => ListIssuingTokensParamsStatus::Deleted,
            "requested" => ListIssuingTokensParamsStatus::Requested,
            "suspended" => ListIssuingTokensParamsStatus::Suspended,
            other => ListIssuingTokensParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ListIssuingTokensParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
