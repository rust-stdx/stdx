#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct CreateRadarPaymentEvaluationParamsCustomerDetails {
    #[doc = "The ID of the customer associated with the payment evaluation."]
    pub customer: Option<String>,
    #[doc = "The ID of the Account representing the customer associated with the payment evaluation."]
    pub customer_account: Option<String>,
    #[doc = "The customer's email address."]
    pub email: Option<String>,
    #[doc = "The customer's full name or business name."]
    pub name: Option<String>,
    #[doc = "The customer's phone number."]
    pub phone: Option<String>,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for CreateRadarPaymentEvaluationParamsCustomerDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("email", self.email.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardCustomerPresence {
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardCustomerPresence {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardCustomerPresence::OffSession => {
                "off_session"
            }
            CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardCustomerPresence::OnSession => {
                "on_session"
            }
            CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardCustomerPresence::Unknown(
                value,
            ) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json>
    for CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardCustomerPresence
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardCustomerPresence::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => {
                CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardCustomerPresence::OffSession
            }
            "on_session" => {
                CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardCustomerPresence::OnSession
            }
            other => CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardCustomerPresence::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode
    for CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardCustomerPresence
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct CreateRadarPaymentEvaluationParamsPaymentDetailsPaymentMethodDetailsBillingDetailsAddress {
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
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub state: Option<String>,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode
    for CreateRadarPaymentEvaluationParamsPaymentDetailsPaymentMethodDetailsBillingDetailsAddress
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct CreateRadarValueListItemParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The value of the item (whose type must match the type of the parent value list)."]
    pub value: String,
    #[doc = "The identifier of the value list which the created item will be added to."]
    pub value_list: String,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for CreateRadarValueListItemParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("value", &self.value);
        form.field("value_list", &self.value_list);
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateRadarValueListParamsItemType {
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
impl CreateRadarValueListParamsItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateRadarValueListParamsItemType::Account => "account",
            CreateRadarValueListParamsItemType::CardBin => "card_bin",
            CreateRadarValueListParamsItemType::CardFingerprint => "card_fingerprint",
            CreateRadarValueListParamsItemType::CaseSensitiveString => "case_sensitive_string",
            CreateRadarValueListParamsItemType::Country => "country",
            CreateRadarValueListParamsItemType::CryptoFingerprint => "crypto_fingerprint",
            CreateRadarValueListParamsItemType::CustomerId => "customer_id",
            CreateRadarValueListParamsItemType::Email => "email",
            CreateRadarValueListParamsItemType::IpAddress => "ip_address",
            CreateRadarValueListParamsItemType::SepaDebitFingerprint => "sepa_debit_fingerprint",
            CreateRadarValueListParamsItemType::String => "string",
            CreateRadarValueListParamsItemType::UsBankAccountFingerprint => "us_bank_account_fingerprint",
            CreateRadarValueListParamsItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for CreateRadarValueListParamsItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateRadarValueListParamsItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => CreateRadarValueListParamsItemType::Account,
            "card_bin" => CreateRadarValueListParamsItemType::CardBin,
            "card_fingerprint" => CreateRadarValueListParamsItemType::CardFingerprint,
            "case_sensitive_string" => CreateRadarValueListParamsItemType::CaseSensitiveString,
            "country" => CreateRadarValueListParamsItemType::Country,
            "crypto_fingerprint" => CreateRadarValueListParamsItemType::CryptoFingerprint,
            "customer_id" => CreateRadarValueListParamsItemType::CustomerId,
            "email" => CreateRadarValueListParamsItemType::Email,
            "ip_address" => CreateRadarValueListParamsItemType::IpAddress,
            "sepa_debit_fingerprint" => CreateRadarValueListParamsItemType::SepaDebitFingerprint,
            "string" => CreateRadarValueListParamsItemType::String,
            "us_bank_account_fingerprint" => CreateRadarValueListParamsItemType::UsBankAccountFingerprint,
            other => CreateRadarValueListParamsItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for CreateRadarValueListParamsItemType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct GetRadarEarlyFraudWarningParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for GetRadarEarlyFraudWarningParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType {
    OneOff,
    Recurring,
    SetupOneOff,
    SetupRecurring,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::OneOff => "one_off",
            InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::Recurring => "recurring",
            InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::SetupOneOff => "setup_one_off",
            InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::SetupRecurring => "setup_recurring",
            InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "one_off" => InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::OneOff,
            "recurring" => InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::Recurring,
            "setup_one_off" => InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::SetupOneOff,
            "setup_recurring" => InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::SetupRecurring,
            other => InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationUserInterventionRaisedType {
    N3ds,
    Captcha,
    Custom,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationUserInterventionRaisedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationUserInterventionRaisedType::N3ds => "3ds",
            InsightsResourcesPaymentEvaluationUserInterventionRaisedType::Captcha => "captcha",
            InsightsResourcesPaymentEvaluationUserInterventionRaisedType::Custom => "custom",
            InsightsResourcesPaymentEvaluationUserInterventionRaisedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for InsightsResourcesPaymentEvaluationUserInterventionRaisedType {
    fn default() -> Self {
        InsightsResourcesPaymentEvaluationUserInterventionRaisedType::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationUserInterventionRaisedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationUserInterventionRaisedType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "3ds" => InsightsResourcesPaymentEvaluationUserInterventionRaisedType::N3ds,
            "captcha" => InsightsResourcesPaymentEvaluationUserInterventionRaisedType::Captcha,
            "custom" => InsightsResourcesPaymentEvaluationUserInterventionRaisedType::Custom,
            other => InsightsResourcesPaymentEvaluationUserInterventionRaisedType::Unknown(other.to_string()),
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
pub struct RadarReviewResourceSession {
    #[doc = "The browser used in this browser session (e.g., `Chrome`)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[doc = "Information about the device used for the browser session (e.g., `Samsung SM-G930T`)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[doc = "The platform for the browser session (e.g., `Macintosh`)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[doc = "The version for the browser session (e.g., `61.0.3163.100`)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
