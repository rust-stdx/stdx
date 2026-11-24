#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CollectInputsParamsInputsItemSelectionChoicesItemStyle {
    Primary,
    Secondary,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl CollectInputsParamsInputsItemSelectionChoicesItemStyle {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CollectInputsParamsInputsItemSelectionChoicesItemStyle::Primary => "primary",
            CollectInputsParamsInputsItemSelectionChoicesItemStyle::Secondary => "secondary",
            CollectInputsParamsInputsItemSelectionChoicesItemStyle::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for CollectInputsParamsInputsItemSelectionChoicesItemStyle {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CollectInputsParamsInputsItemSelectionChoicesItemStyle::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "primary" => CollectInputsParamsInputsItemSelectionChoicesItemStyle::Primary,
            "secondary" => CollectInputsParamsInputsItemSelectionChoicesItemStyle::Secondary,
            other => CollectInputsParamsInputsItemSelectionChoicesItemStyle::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CollectInputsParamsInputsItemSelectionChoicesItemStyle {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CreateTerminalConfigurationParamsWifiEnterpriseEapPeap {
    #[doc = "A File ID representing a PEM file containing the server certificate"]
    pub ca_certificate_file: Option<String>,
    #[doc = "Password for connecting to the WiFi network"]
    pub password: String,
    #[doc = "Name of the WiFi network"]
    pub ssid: String,
    #[doc = "Username for connecting to the WiFi network"]
    pub username: String,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalConfigurationParamsWifiEnterpriseEapPeap {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ca_certificate_file", self.ca_certificate_file.as_ref());
        form.field("password", &self.password);
        form.field("ssid", &self.ssid);
        form.field("username", &self.username);
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CreateTerminalConfigurationParamsWifiPersonalPsk {
    #[doc = "Password for connecting to the WiFi network"]
    pub password: String,
    #[doc = "Name of the WiFi network"]
    pub ssid: String,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalConfigurationParamsWifiPersonalPsk {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("password", &self.password);
        form.field("ssid", &self.ssid);
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CreateTerminalLocationParamsAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: String,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: Option<String>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub state: Option<String>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalLocationParamsAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field("country", &self.country);
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalConfigurationConfigurationResourceOfflineConfig {
    #[doc = "Determines whether to allow transactions to be collected while reader is offline."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalConfigurationObject {
    TerminalConfiguration,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl TerminalConfigurationObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalConfigurationObject::TerminalConfiguration => "terminal.configuration",
            TerminalConfigurationObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl Default for TerminalConfigurationObject {
    fn default() -> Self {
        TerminalConfigurationObject::Unknown(String::new())
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for TerminalConfigurationObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalConfigurationObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "terminal.configuration" => TerminalConfigurationObject::TerminalConfiguration,
            other => TerminalConfigurationObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalOnboardingLinkLinkType {
    AppleTermsAndConditions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl TerminalOnboardingLinkLinkType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalOnboardingLinkLinkType::AppleTermsAndConditions => "apple_terms_and_conditions",
            TerminalOnboardingLinkLinkType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl Default for TerminalOnboardingLinkLinkType {
    fn default() -> Self {
        TerminalOnboardingLinkLinkType::Unknown(String::new())
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for TerminalOnboardingLinkLinkType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalOnboardingLinkLinkType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "apple_terms_and_conditions" => TerminalOnboardingLinkLinkType::AppleTermsAndConditions,
            other => TerminalOnboardingLinkLinkType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalReaderReaderResourceChoiceStyle {
    Primary,
    Secondary,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl TerminalReaderReaderResourceChoiceStyle {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalReaderReaderResourceChoiceStyle::Primary => "primary",
            TerminalReaderReaderResourceChoiceStyle::Secondary => "secondary",
            TerminalReaderReaderResourceChoiceStyle::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl<'json> json::Decode<'json> for TerminalReaderReaderResourceChoiceStyle {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalReaderReaderResourceChoiceStyle::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "primary" => TerminalReaderReaderResourceChoiceStyle::Primary,
            "secondary" => TerminalReaderReaderResourceChoiceStyle::Secondary,
            other => TerminalReaderReaderResourceChoiceStyle::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceLineItem {
    #[doc = "The amount of the line item."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Description of the line item."]
    #[json(default)]
    pub description: String,
    #[doc = "The quantity of the line item."]
    #[json(default)]
    pub quantity: i64,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceProcessSetupConfig {
    #[doc = "Enable customer-initiated cancellation when processing this SetupIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub enable_customer_cancellation: Option<bool>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalReaderReaderResourceReaderActionStatus {
    Failed,
    InProgress,
    Succeeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl TerminalReaderReaderResourceReaderActionStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalReaderReaderResourceReaderActionStatus::Failed => "failed",
            TerminalReaderReaderResourceReaderActionStatus::InProgress => "in_progress",
            TerminalReaderReaderResourceReaderActionStatus::Succeeded => "succeeded",
            TerminalReaderReaderResourceReaderActionStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl Default for TerminalReaderReaderResourceReaderActionStatus {
    fn default() -> Self {
        TerminalReaderReaderResourceReaderActionStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl<'json> json::Decode<'json> for TerminalReaderReaderResourceReaderActionStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalReaderReaderResourceReaderActionStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "failed" => TerminalReaderReaderResourceReaderActionStatus::Failed,
            "in_progress" => TerminalReaderReaderResourceReaderActionStatus::InProgress,
            "succeeded" => TerminalReaderReaderResourceReaderActionStatus::Succeeded,
            other => TerminalReaderReaderResourceReaderActionStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalReaderReaderResourceSetReaderDisplayActionType {
    Cart,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl TerminalReaderReaderResourceSetReaderDisplayActionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalReaderReaderResourceSetReaderDisplayActionType::Cart => "cart",
            TerminalReaderReaderResourceSetReaderDisplayActionType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl Default for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn default() -> Self {
        TerminalReaderReaderResourceSetReaderDisplayActionType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl<'json> json::Decode<'json> for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalReaderReaderResourceSetReaderDisplayActionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cart" => TerminalReaderReaderResourceSetReaderDisplayActionType::Cart,
            other => TerminalReaderReaderResourceSetReaderDisplayActionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceText {
    #[doc = "The collected text value"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
