#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CollectInputsParamsInputsItemType {
    Email,
    Numeric,
    Phone,
    Selection,
    Signature,
    Text,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl CollectInputsParamsInputsItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CollectInputsParamsInputsItemType::Email => "email",
            CollectInputsParamsInputsItemType::Numeric => "numeric",
            CollectInputsParamsInputsItemType::Phone => "phone",
            CollectInputsParamsInputsItemType::Selection => "selection",
            CollectInputsParamsInputsItemType::Signature => "signature",
            CollectInputsParamsInputsItemType::Text => "text",
            CollectInputsParamsInputsItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for CollectInputsParamsInputsItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CollectInputsParamsInputsItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "email" => CollectInputsParamsInputsItemType::Email,
            "numeric" => CollectInputsParamsInputsItemType::Numeric,
            "phone" => CollectInputsParamsInputsItemType::Phone,
            "selection" => CollectInputsParamsInputsItemType::Selection,
            "signature" => CollectInputsParamsInputsItemType::Signature,
            "text" => CollectInputsParamsInputsItemType::Text,
            other => CollectInputsParamsInputsItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CollectInputsParamsInputsItemType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CollectPaymentMethodParamsCollectConfigAllowRedisplay {
    Always,
    Limited,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl CollectPaymentMethodParamsCollectConfigAllowRedisplay {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CollectPaymentMethodParamsCollectConfigAllowRedisplay::Always => "always",
            CollectPaymentMethodParamsCollectConfigAllowRedisplay::Limited => "limited",
            CollectPaymentMethodParamsCollectConfigAllowRedisplay::Unspecified => "unspecified",
            CollectPaymentMethodParamsCollectConfigAllowRedisplay::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for CollectPaymentMethodParamsCollectConfigAllowRedisplay {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CollectPaymentMethodParamsCollectConfigAllowRedisplay::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CollectPaymentMethodParamsCollectConfigAllowRedisplay::Always,
            "limited" => CollectPaymentMethodParamsCollectConfigAllowRedisplay::Limited,
            "unspecified" => CollectPaymentMethodParamsCollectConfigAllowRedisplay::Unspecified,
            other => CollectPaymentMethodParamsCollectConfigAllowRedisplay::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CollectPaymentMethodParamsCollectConfigAllowRedisplay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CreateTerminalConfigurationParamsCellular {
    #[doc = "Determines whether to allow the reader to connect to a cellular network."]
    pub enabled: bool,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalConfigurationParamsCellular {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CreateTerminalConfigurationParamsRebootWindow {
    #[doc = "Integer between 0 to 23 that represents the end hour of the reboot time window."]
    pub end_hour: i64,
    #[doc = "Integer between 0 to 23 that represents the start hour of the reboot time window."]
    pub start_hour: i64,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalConfigurationParamsRebootWindow {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("end_hour", &self.end_hour);
        form.field("start_hour", &self.start_hour);
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CreateTerminalConfigurationParamsWifiEnterpriseEapTls {
    #[doc = "A File ID representing a PEM file containing the server certificate"]
    pub ca_certificate_file: Option<String>,
    #[doc = "A File ID representing a PEM file containing the client certificate"]
    pub client_certificate_file: String,
    #[doc = "A File ID representing a PEM file containing the client RSA private key"]
    pub private_key_file: String,
    #[doc = "Password for the private key file"]
    pub private_key_file_password: Option<String>,
    #[doc = "Name of the WiFi network"]
    pub ssid: String,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalConfigurationParamsWifiEnterpriseEapTls {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ca_certificate_file", self.ca_certificate_file.as_ref());
        form.field("client_certificate_file", &self.client_certificate_file);
        form.field("private_key_file", &self.private_key_file);
        form.field_opt("private_key_file_password", self.private_key_file_password.as_ref());
        form.field("ssid", &self.ssid);
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTerminalConfigurationParamsWifiType {
    EnterpriseEapPeap,
    EnterpriseEapTls,
    PersonalPsk,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl CreateTerminalConfigurationParamsWifiType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTerminalConfigurationParamsWifiType::EnterpriseEapPeap => "enterprise_eap_peap",
            CreateTerminalConfigurationParamsWifiType::EnterpriseEapTls => "enterprise_eap_tls",
            CreateTerminalConfigurationParamsWifiType::PersonalPsk => "personal_psk",
            CreateTerminalConfigurationParamsWifiType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for CreateTerminalConfigurationParamsWifiType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTerminalConfigurationParamsWifiType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "enterprise_eap_peap" => CreateTerminalConfigurationParamsWifiType::EnterpriseEapPeap,
            "enterprise_eap_tls" => CreateTerminalConfigurationParamsWifiType::EnterpriseEapTls,
            "personal_psk" => CreateTerminalConfigurationParamsWifiType::PersonalPsk,
            other => CreateTerminalConfigurationParamsWifiType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalConfigurationParamsWifiType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct ListTerminalLocationsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for ListTerminalLocationsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
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
pub struct OnlineAcceptance {
    #[doc = "The customer accepts the mandate from this IP address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The customer accepts the mandate using the user agent of the browser."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalConfigurationConfigurationResourceCellularConfig {
    #[doc = "Whether a cellular-capable reader can connect to the internet over cellular."]
    #[json(default)]
    pub enabled: bool,
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalConfigurationConfigurationResourcePersonalPskWifi {
    #[doc = "Password for connecting to the WiFi network"]
    #[json(default)]
    pub password: String,
    #[doc = "Name of the WiFi network"]
    #[json(default)]
    pub ssid: String,
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalOnboardingLinkObject {
    TerminalOnboardingLink,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl TerminalOnboardingLinkObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalOnboardingLinkObject::TerminalOnboardingLink => "terminal.onboarding_link",
            TerminalOnboardingLinkObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl Default for TerminalOnboardingLinkObject {
    fn default() -> Self {
        TerminalOnboardingLinkObject::Unknown(String::new())
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for TerminalOnboardingLinkObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalOnboardingLinkObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "terminal.onboarding_link" => TerminalOnboardingLinkObject::TerminalOnboardingLink,
            other => TerminalOnboardingLinkObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceConfirmConfig {
    #[doc = "If the customer doesn't abandon authenticating the payment, they're redirected to this URL after completion."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceEmail {
    #[doc = "The collected email address"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceFileMetadata {
    #[doc = "Creation time of the object (in seconds since the Unix epoch)."]
    #[json(default)]
    pub created_at: i64,
    #[doc = "The original name of the uploaded file (e.g."]
    #[json(default)]
    pub filename: String,
    #[doc = "The size (in bytes) of the uploaded file."]
    #[json(default)]
    pub size: i64,
    #[doc = "The format of the uploaded file."]
    #[json(rename = "type", default)]
    pub r#type: String,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalReaderReaderResourceReaderActionType {
    CollectInputs,
    CollectPaymentMethod,
    ConfirmPaymentIntent,
    PrintContent,
    ProcessPaymentIntent,
    ProcessSetupIntent,
    RefundPayment,
    SetReaderDisplay,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl TerminalReaderReaderResourceReaderActionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalReaderReaderResourceReaderActionType::CollectInputs => "collect_inputs",
            TerminalReaderReaderResourceReaderActionType::CollectPaymentMethod => "collect_payment_method",
            TerminalReaderReaderResourceReaderActionType::ConfirmPaymentIntent => "confirm_payment_intent",
            TerminalReaderReaderResourceReaderActionType::PrintContent => "print_content",
            TerminalReaderReaderResourceReaderActionType::ProcessPaymentIntent => "process_payment_intent",
            TerminalReaderReaderResourceReaderActionType::ProcessSetupIntent => "process_setup_intent",
            TerminalReaderReaderResourceReaderActionType::RefundPayment => "refund_payment",
            TerminalReaderReaderResourceReaderActionType::SetReaderDisplay => "set_reader_display",
            TerminalReaderReaderResourceReaderActionType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl Default for TerminalReaderReaderResourceReaderActionType {
    fn default() -> Self {
        TerminalReaderReaderResourceReaderActionType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl<'json> json::Decode<'json> for TerminalReaderReaderResourceReaderActionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalReaderReaderResourceReaderActionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "collect_inputs" => TerminalReaderReaderResourceReaderActionType::CollectInputs,
            "collect_payment_method" => TerminalReaderReaderResourceReaderActionType::CollectPaymentMethod,
            "confirm_payment_intent" => TerminalReaderReaderResourceReaderActionType::ConfirmPaymentIntent,
            "print_content" => TerminalReaderReaderResourceReaderActionType::PrintContent,
            "process_payment_intent" => TerminalReaderReaderResourceReaderActionType::ProcessPaymentIntent,
            "process_setup_intent" => TerminalReaderReaderResourceReaderActionType::ProcessSetupIntent,
            "refund_payment" => TerminalReaderReaderResourceReaderActionType::RefundPayment,
            "set_reader_display" => TerminalReaderReaderResourceReaderActionType::SetReaderDisplay,
            other => TerminalReaderReaderResourceReaderActionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalReaderReaderResourceRefundPaymentActionReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl TerminalReaderReaderResourceRefundPaymentActionReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalReaderReaderResourceRefundPaymentActionReason::Duplicate => "duplicate",
            TerminalReaderReaderResourceRefundPaymentActionReason::Fraudulent => "fraudulent",
            TerminalReaderReaderResourceRefundPaymentActionReason::RequestedByCustomer => "requested_by_customer",
            TerminalReaderReaderResourceRefundPaymentActionReason::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl<'json> json::Decode<'json> for TerminalReaderReaderResourceRefundPaymentActionReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalReaderReaderResourceRefundPaymentActionReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "duplicate" => TerminalReaderReaderResourceRefundPaymentActionReason::Duplicate,
            "fraudulent" => TerminalReaderReaderResourceRefundPaymentActionReason::Fraudulent,
            "requested_by_customer" => TerminalReaderReaderResourceRefundPaymentActionReason::RequestedByCustomer,
            other => TerminalReaderReaderResourceRefundPaymentActionReason::Unknown(other.to_string()),
        })
    }
}
