#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AddressApiResourceTerminal {
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

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsConfirmConfig {
    #[doc = "The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app..."]
    pub return_url: Option<String>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsConfirmConfig {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("return_url", self.return_url.as_ref());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CreateTerminalConnectionTokenParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The id of the location that this connection token is scoped to."]
    pub location: Option<String>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalConnectionTokenParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("location", self.location.as_ref());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CreateTerminalOnboardingLinkParamsLinkOptionsAppleTermsAndConditions {
    #[doc = "Whether the link should also support users relinking their Apple account."]
    pub allow_relinking: Option<bool>,
    #[doc = "The business name of the merchant accepting Apple's Terms and Conditions."]
    pub merchant_display_name: String,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalOnboardingLinkParamsLinkOptionsAppleTermsAndConditions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("allow_relinking", self.allow_relinking.as_ref());
        form.field("merchant_display_name", &self.merchant_display_name);
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTerminalReadersParamsStatus {
    Offline,
    Online,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl ListTerminalReadersParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTerminalReadersParamsStatus::Offline => "offline",
            ListTerminalReadersParamsStatus::Online => "online",
            ListTerminalReadersParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for ListTerminalReadersParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTerminalReadersParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "offline" => ListTerminalReadersParamsStatus::Offline,
            "online" => ListTerminalReadersParamsStatus::Online,
            other => ListTerminalReadersParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for ListTerminalReadersParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct ProcessSetupIntentParamsProcessConfig {
    #[doc = "Enables cancel button on transaction screens."]
    pub enable_customer_cancellation: Option<bool>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for ProcessSetupIntentParamsProcessConfig {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enable_customer_cancellation", self.enable_customer_cancellation.as_ref());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
    #[doc = "Fixed amounts displayed when collecting a tip"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,
    #[doc = "Percentages displayed when collecting a tip"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,
    #[doc = "Below this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalConnectionTokenObject {
    TerminalConnectionToken,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl TerminalConnectionTokenObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalConnectionTokenObject::TerminalConnectionToken => "terminal.connection_token",
            TerminalConnectionTokenObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl Default for TerminalConnectionTokenObject {
    fn default() -> Self {
        TerminalConnectionTokenObject::Unknown(String::new())
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for TerminalConnectionTokenObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalConnectionTokenObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "terminal.connection_token" => TerminalConnectionTokenObject::TerminalConnectionToken,
            other => TerminalConnectionTokenObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceCustomText {
    #[doc = "Customize the default description for this input"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Customize the default label for this input's skip button"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub skip_button: Option<String>,
    #[doc = "Customize the default label for this input's submit button"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub submit_button: Option<String>,
    #[doc = "Customize the default title for this input"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalReaderReaderResourceInputType {
    Email,
    Numeric,
    Phone,
    Selection,
    Signature,
    Text,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl TerminalReaderReaderResourceInputType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalReaderReaderResourceInputType::Email => "email",
            TerminalReaderReaderResourceInputType::Numeric => "numeric",
            TerminalReaderReaderResourceInputType::Phone => "phone",
            TerminalReaderReaderResourceInputType::Selection => "selection",
            TerminalReaderReaderResourceInputType::Signature => "signature",
            TerminalReaderReaderResourceInputType::Text => "text",
            TerminalReaderReaderResourceInputType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl Default for TerminalReaderReaderResourceInputType {
    fn default() -> Self {
        TerminalReaderReaderResourceInputType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl<'json> json::Decode<'json> for TerminalReaderReaderResourceInputType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalReaderReaderResourceInputType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "email" => TerminalReaderReaderResourceInputType::Email,
            "numeric" => TerminalReaderReaderResourceInputType::Numeric,
            "phone" => TerminalReaderReaderResourceInputType::Phone,
            "selection" => TerminalReaderReaderResourceInputType::Selection,
            "signature" => TerminalReaderReaderResourceInputType::Signature,
            "text" => TerminalReaderReaderResourceInputType::Text,
            other => TerminalReaderReaderResourceInputType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourcePhone {
    #[doc = "The collected phone number"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceSignature {
    #[doc = "The File ID of a collected signature image"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceTippingConfig {
    #[doc = "Amount used to calculate tip suggestions on tipping selection screen for this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_eligible: Option<i64>,
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTerminalReaderParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The new label of the reader."]
    pub label: Option<json::OwnedValue>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for UpdateTerminalReaderParams {
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
