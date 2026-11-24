#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CollectInputsParamsInputsItemTogglesItemDefaultValue {
    Disabled,
    Enabled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl CollectInputsParamsInputsItemTogglesItemDefaultValue {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CollectInputsParamsInputsItemTogglesItemDefaultValue::Disabled => "disabled",
            CollectInputsParamsInputsItemTogglesItemDefaultValue::Enabled => "enabled",
            CollectInputsParamsInputsItemTogglesItemDefaultValue::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for CollectInputsParamsInputsItemTogglesItemDefaultValue {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CollectInputsParamsInputsItemTogglesItemDefaultValue::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "disabled" => CollectInputsParamsInputsItemTogglesItemDefaultValue::Disabled,
            "enabled" => CollectInputsParamsInputsItemTogglesItemDefaultValue::Enabled,
            other => CollectInputsParamsInputsItemTogglesItemDefaultValue::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CollectInputsParamsInputsItemTogglesItemDefaultValue {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTerminalOnboardingLinkParamsLinkType {
    AppleTermsAndConditions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl CreateTerminalOnboardingLinkParamsLinkType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTerminalOnboardingLinkParamsLinkType::AppleTermsAndConditions => "apple_terms_and_conditions",
            CreateTerminalOnboardingLinkParamsLinkType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for CreateTerminalOnboardingLinkParamsLinkType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTerminalOnboardingLinkParamsLinkType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "apple_terms_and_conditions" => CreateTerminalOnboardingLinkParamsLinkType::AppleTermsAndConditions,
            other => CreateTerminalOnboardingLinkParamsLinkType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalOnboardingLinkParamsLinkType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTerminalReadersParamsDeviceType {
    BbposChipper2x,
    BbposWisepad3,
    BbposWiseposE,
    MobilePhoneReader,
    SimulatedStripeS700,
    SimulatedStripeS710,
    SimulatedVerifoneM425,
    SimulatedVerifoneP630,
    SimulatedVerifoneUx700,
    SimulatedVerifoneV660p,
    SimulatedWiseposE,
    StripeM2,
    StripeS700,
    StripeS710,
    VerifoneP400,
    VerifoneM425,
    VerifoneP630,
    VerifoneUx700,
    VerifoneV660p,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl ListTerminalReadersParamsDeviceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTerminalReadersParamsDeviceType::BbposChipper2x => "bbpos_chipper2x",
            ListTerminalReadersParamsDeviceType::BbposWisepad3 => "bbpos_wisepad3",
            ListTerminalReadersParamsDeviceType::BbposWiseposE => "bbpos_wisepos_e",
            ListTerminalReadersParamsDeviceType::MobilePhoneReader => "mobile_phone_reader",
            ListTerminalReadersParamsDeviceType::SimulatedStripeS700 => "simulated_stripe_s700",
            ListTerminalReadersParamsDeviceType::SimulatedStripeS710 => "simulated_stripe_s710",
            ListTerminalReadersParamsDeviceType::SimulatedVerifoneM425 => "simulated_verifone_m425",
            ListTerminalReadersParamsDeviceType::SimulatedVerifoneP630 => "simulated_verifone_p630",
            ListTerminalReadersParamsDeviceType::SimulatedVerifoneUx700 => "simulated_verifone_ux700",
            ListTerminalReadersParamsDeviceType::SimulatedVerifoneV660p => "simulated_verifone_v660p",
            ListTerminalReadersParamsDeviceType::SimulatedWiseposE => "simulated_wisepos_e",
            ListTerminalReadersParamsDeviceType::StripeM2 => "stripe_m2",
            ListTerminalReadersParamsDeviceType::StripeS700 => "stripe_s700",
            ListTerminalReadersParamsDeviceType::StripeS710 => "stripe_s710",
            ListTerminalReadersParamsDeviceType::VerifoneP400 => "verifone_P400",
            ListTerminalReadersParamsDeviceType::VerifoneM425 => "verifone_m425",
            ListTerminalReadersParamsDeviceType::VerifoneP630 => "verifone_p630",
            ListTerminalReadersParamsDeviceType::VerifoneUx700 => "verifone_ux700",
            ListTerminalReadersParamsDeviceType::VerifoneV660p => "verifone_v660p",
            ListTerminalReadersParamsDeviceType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for ListTerminalReadersParamsDeviceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTerminalReadersParamsDeviceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bbpos_chipper2x" => ListTerminalReadersParamsDeviceType::BbposChipper2x,
            "bbpos_wisepad3" => ListTerminalReadersParamsDeviceType::BbposWisepad3,
            "bbpos_wisepos_e" => ListTerminalReadersParamsDeviceType::BbposWiseposE,
            "mobile_phone_reader" => ListTerminalReadersParamsDeviceType::MobilePhoneReader,
            "simulated_stripe_s700" => ListTerminalReadersParamsDeviceType::SimulatedStripeS700,
            "simulated_stripe_s710" => ListTerminalReadersParamsDeviceType::SimulatedStripeS710,
            "simulated_verifone_m425" => ListTerminalReadersParamsDeviceType::SimulatedVerifoneM425,
            "simulated_verifone_p630" => ListTerminalReadersParamsDeviceType::SimulatedVerifoneP630,
            "simulated_verifone_ux700" => ListTerminalReadersParamsDeviceType::SimulatedVerifoneUx700,
            "simulated_verifone_v660p" => ListTerminalReadersParamsDeviceType::SimulatedVerifoneV660p,
            "simulated_wisepos_e" => ListTerminalReadersParamsDeviceType::SimulatedWiseposE,
            "stripe_m2" => ListTerminalReadersParamsDeviceType::StripeM2,
            "stripe_s700" => ListTerminalReadersParamsDeviceType::StripeS700,
            "stripe_s710" => ListTerminalReadersParamsDeviceType::StripeS710,
            "verifone_P400" => ListTerminalReadersParamsDeviceType::VerifoneP400,
            "verifone_m425" => ListTerminalReadersParamsDeviceType::VerifoneM425,
            "verifone_p630" => ListTerminalReadersParamsDeviceType::VerifoneP630,
            "verifone_ux700" => ListTerminalReadersParamsDeviceType::VerifoneUx700,
            "verifone_v660p" => ListTerminalReadersParamsDeviceType::VerifoneV660p,
            other => ListTerminalReadersParamsDeviceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for ListTerminalReadersParamsDeviceType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct SetReaderDisplayParamsCartLineItemsItem {
    #[doc = "The price of the item in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal)."]
    pub amount: i64,
    #[doc = "The description or name of the item."]
    pub description: String,
    #[doc = "The quantity of the line item being purchased."]
    pub quantity: i64,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for SetReaderDisplayParamsCartLineItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("description", &self.description);
        form.field("quantity", &self.quantity);
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum SetReaderDisplayParamsType {
    Cart,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl SetReaderDisplayParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetReaderDisplayParamsType::Cart => "cart",
            SetReaderDisplayParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for SetReaderDisplayParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetReaderDisplayParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cart" => SetReaderDisplayParamsType::Cart,
            other => SetReaderDisplayParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for SetReaderDisplayParamsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalConfigurationConfigurationResourceEnterprisePeapWifi {
    #[doc = "A File ID representing a PEM file containing the server certificate"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ca_certificate_file: Option<String>,
    #[doc = "Password for connecting to the WiFi network"]
    #[json(default)]
    pub password: String,
    #[doc = "Name of the WiFi network"]
    #[json(default)]
    pub ssid: String,
    #[doc = "Username for connecting to the WiFi network"]
    #[json(default)]
    pub username: String,
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalConfigurationConfigurationResourceRebootWindow {
    #[doc = "Integer between 0 to 23 that represents the end hour of the reboot time window."]
    #[json(default)]
    pub end_hour: i64,
    #[doc = "Integer between 0 to 23 that represents the start hour of the reboot time window."]
    #[json(default)]
    pub start_hour: i64,
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalConfigurationConfigurationResourceWifiConfigType {
    EnterpriseEapPeap,
    EnterpriseEapTls,
    PersonalPsk,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "terminal")]
impl TerminalConfigurationConfigurationResourceWifiConfigType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalConfigurationConfigurationResourceWifiConfigType::EnterpriseEapPeap => "enterprise_eap_peap",
            TerminalConfigurationConfigurationResourceWifiConfigType::EnterpriseEapTls => "enterprise_eap_tls",
            TerminalConfigurationConfigurationResourceWifiConfigType::PersonalPsk => "personal_psk",
            TerminalConfigurationConfigurationResourceWifiConfigType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "terminal")]
impl Default for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn default() -> Self {
        TerminalConfigurationConfigurationResourceWifiConfigType::Unknown(String::new())
    }
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalConfigurationConfigurationResourceWifiConfigType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "enterprise_eap_peap" => TerminalConfigurationConfigurationResourceWifiConfigType::EnterpriseEapPeap,
            "enterprise_eap_tls" => TerminalConfigurationConfigurationResourceWifiConfigType::EnterpriseEapTls,
            "personal_psk" => TerminalConfigurationConfigurationResourceWifiConfigType::PersonalPsk,
            other => TerminalConfigurationConfigurationResourceWifiConfigType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalReaderDeviceType {
    BbposChipper2x,
    BbposWisepad3,
    BbposWiseposE,
    MobilePhoneReader,
    SimulatedStripeS700,
    SimulatedStripeS710,
    SimulatedVerifoneM425,
    SimulatedVerifoneP630,
    SimulatedVerifoneUx700,
    SimulatedVerifoneV660p,
    SimulatedWiseposE,
    StripeM2,
    StripeS700,
    StripeS710,
    VerifoneP400,
    VerifoneM425,
    VerifoneP630,
    VerifoneUx700,
    VerifoneV660p,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl TerminalReaderDeviceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalReaderDeviceType::BbposChipper2x => "bbpos_chipper2x",
            TerminalReaderDeviceType::BbposWisepad3 => "bbpos_wisepad3",
            TerminalReaderDeviceType::BbposWiseposE => "bbpos_wisepos_e",
            TerminalReaderDeviceType::MobilePhoneReader => "mobile_phone_reader",
            TerminalReaderDeviceType::SimulatedStripeS700 => "simulated_stripe_s700",
            TerminalReaderDeviceType::SimulatedStripeS710 => "simulated_stripe_s710",
            TerminalReaderDeviceType::SimulatedVerifoneM425 => "simulated_verifone_m425",
            TerminalReaderDeviceType::SimulatedVerifoneP630 => "simulated_verifone_p630",
            TerminalReaderDeviceType::SimulatedVerifoneUx700 => "simulated_verifone_ux700",
            TerminalReaderDeviceType::SimulatedVerifoneV660p => "simulated_verifone_v660p",
            TerminalReaderDeviceType::SimulatedWiseposE => "simulated_wisepos_e",
            TerminalReaderDeviceType::StripeM2 => "stripe_m2",
            TerminalReaderDeviceType::StripeS700 => "stripe_s700",
            TerminalReaderDeviceType::StripeS710 => "stripe_s710",
            TerminalReaderDeviceType::VerifoneP400 => "verifone_P400",
            TerminalReaderDeviceType::VerifoneM425 => "verifone_m425",
            TerminalReaderDeviceType::VerifoneP630 => "verifone_p630",
            TerminalReaderDeviceType::VerifoneUx700 => "verifone_ux700",
            TerminalReaderDeviceType::VerifoneV660p => "verifone_v660p",
            TerminalReaderDeviceType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl Default for TerminalReaderDeviceType {
    fn default() -> Self {
        TerminalReaderDeviceType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl<'json> json::Decode<'json> for TerminalReaderDeviceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalReaderDeviceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bbpos_chipper2x" => TerminalReaderDeviceType::BbposChipper2x,
            "bbpos_wisepad3" => TerminalReaderDeviceType::BbposWisepad3,
            "bbpos_wisepos_e" => TerminalReaderDeviceType::BbposWiseposE,
            "mobile_phone_reader" => TerminalReaderDeviceType::MobilePhoneReader,
            "simulated_stripe_s700" => TerminalReaderDeviceType::SimulatedStripeS700,
            "simulated_stripe_s710" => TerminalReaderDeviceType::SimulatedStripeS710,
            "simulated_verifone_m425" => TerminalReaderDeviceType::SimulatedVerifoneM425,
            "simulated_verifone_p630" => TerminalReaderDeviceType::SimulatedVerifoneP630,
            "simulated_verifone_ux700" => TerminalReaderDeviceType::SimulatedVerifoneUx700,
            "simulated_verifone_v660p" => TerminalReaderDeviceType::SimulatedVerifoneV660p,
            "simulated_wisepos_e" => TerminalReaderDeviceType::SimulatedWiseposE,
            "stripe_m2" => TerminalReaderDeviceType::StripeM2,
            "stripe_s700" => TerminalReaderDeviceType::StripeS700,
            "stripe_s710" => TerminalReaderDeviceType::StripeS710,
            "verifone_P400" => TerminalReaderDeviceType::VerifoneP400,
            "verifone_m425" => TerminalReaderDeviceType::VerifoneM425,
            "verifone_p630" => TerminalReaderDeviceType::VerifoneP630,
            "verifone_ux700" => TerminalReaderDeviceType::VerifoneUx700,
            "verifone_v660p" => TerminalReaderDeviceType::VerifoneV660p,
            other => TerminalReaderDeviceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalReaderReaderResourcePrintContentType {
    Image,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl TerminalReaderReaderResourcePrintContentType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalReaderReaderResourcePrintContentType::Image => "image",
            TerminalReaderReaderResourcePrintContentType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl Default for TerminalReaderReaderResourcePrintContentType {
    fn default() -> Self {
        TerminalReaderReaderResourcePrintContentType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl<'json> json::Decode<'json> for TerminalReaderReaderResourcePrintContentType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalReaderReaderResourcePrintContentType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "image" => TerminalReaderReaderResourcePrintContentType::Image,
            other => TerminalReaderReaderResourcePrintContentType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalReaderReaderResourceToggleDefaultValue {
    Disabled,
    Enabled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl TerminalReaderReaderResourceToggleDefaultValue {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TerminalReaderReaderResourceToggleDefaultValue::Disabled => "disabled",
            TerminalReaderReaderResourceToggleDefaultValue::Enabled => "enabled",
            TerminalReaderReaderResourceToggleDefaultValue::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl<'json> json::Decode<'json> for TerminalReaderReaderResourceToggleDefaultValue {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TerminalReaderReaderResourceToggleDefaultValue::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "disabled" => TerminalReaderReaderResourceToggleDefaultValue::Disabled,
            "enabled" => TerminalReaderReaderResourceToggleDefaultValue::Enabled,
            other => TerminalReaderReaderResourceToggleDefaultValue::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTerminalLocationParamsAddress {
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

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for UpdateTerminalLocationParamsAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}
