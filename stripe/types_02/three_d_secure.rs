#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, Default)]
pub struct CreateThreeDSecureAuthenticationParamsChannelBrowser {
    #[doc = "The HTTP accept headers from the cardholder's browser."]
    pub accept_header: String,
    #[doc = "The color depth of the cardholder’s screen."]
    pub color_depth: Option<i64>,
    #[doc = "Unique and immutable identifier linked to a device that is consistent across 3DS transactions for the specific user d..."]
    pub device_id: Option<String>,
    #[doc = "The IP address of the browser."]
    pub ip_address: String,
    #[doc = "The cardholder browser’s ability to execute Java."]
    pub java_enabled: Option<bool>,
    #[doc = "The cardholder browser’s ability to execute JavaScript."]
    pub javascript_enabled: bool,
    #[doc = "An IETF BCP 47 language tag representing the browser language."]
    pub language: String,
    #[doc = "The total height of the cardholder’s screen in pixels."]
    pub screen_height: Option<i64>,
    #[doc = "The total width of the cardholder’s screen in pixels."]
    pub screen_width: Option<i64>,
    #[doc = "The time difference between UTC time and the local time of the cardholder’s browser, in minutes."]
    pub timezone_offset: Option<i64>,
    #[doc = "The browser user agent."]
    pub user_agent: String,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsChannelBrowser {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("accept_header", &self.accept_header);
        form.field_opt("color_depth", self.color_depth.as_ref());
        form.field_opt("device_id", self.device_id.as_ref());
        form.field("ip_address", &self.ip_address);
        form.field_opt("java_enabled", self.java_enabled.as_ref());
        form.field("javascript_enabled", &self.javascript_enabled);
        form.field("language", &self.language);
        form.field_opt("screen_height", self.screen_height.as_ref());
        form.field_opt("screen_width", self.screen_width.as_ref());
        form.field_opt("timezone_offset", self.timezone_offset.as_ref());
        form.field("user_agent", &self.user_agent);
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsChannelThreeRIType {
    DelayedShipment,
    OtherPayment,
    Recurring,
    SplitShipment,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsChannelThreeRIType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsChannelThreeRIType::DelayedShipment => "delayed_shipment",
            CreateThreeDSecureAuthenticationParamsChannelThreeRIType::OtherPayment => "other_payment",
            CreateThreeDSecureAuthenticationParamsChannelThreeRIType::Recurring => "recurring",
            CreateThreeDSecureAuthenticationParamsChannelThreeRIType::SplitShipment => "split_shipment",
            CreateThreeDSecureAuthenticationParamsChannelThreeRIType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsChannelThreeRIType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsChannelThreeRIType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "delayed_shipment" => CreateThreeDSecureAuthenticationParamsChannelThreeRIType::DelayedShipment,
            "other_payment" => CreateThreeDSecureAuthenticationParamsChannelThreeRIType::OtherPayment,
            "recurring" => CreateThreeDSecureAuthenticationParamsChannelThreeRIType::Recurring,
            "split_shipment" => CreateThreeDSecureAuthenticationParamsChannelThreeRIType::SplitShipment,
            other => CreateThreeDSecureAuthenticationParamsChannelThreeRIType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsChannelThreeRIType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsFlowPreferenceType {
    Challenge,
    DataShare,
    Frictionless,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsFlowPreferenceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsFlowPreferenceType::Challenge => "challenge",
            CreateThreeDSecureAuthenticationParamsFlowPreferenceType::DataShare => "data_share",
            CreateThreeDSecureAuthenticationParamsFlowPreferenceType::Frictionless => "frictionless",
            CreateThreeDSecureAuthenticationParamsFlowPreferenceType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsFlowPreferenceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsFlowPreferenceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "challenge" => CreateThreeDSecureAuthenticationParamsFlowPreferenceType::Challenge,
            "data_share" => CreateThreeDSecureAuthenticationParamsFlowPreferenceType::DataShare,
            "frictionless" => CreateThreeDSecureAuthenticationParamsFlowPreferenceType::Frictionless,
            other => CreateThreeDSecureAuthenticationParamsFlowPreferenceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFlowPreferenceType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsMessageCategory {
    NonPaymentAuthentication,
    PaymentAuthentication,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsMessageCategory {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsMessageCategory::NonPaymentAuthentication => {
                "non_payment_authentication"
            }
            CreateThreeDSecureAuthenticationParamsMessageCategory::PaymentAuthentication => "payment_authentication",
            CreateThreeDSecureAuthenticationParamsMessageCategory::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsMessageCategory {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsMessageCategory::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "non_payment_authentication" => {
                CreateThreeDSecureAuthenticationParamsMessageCategory::NonPaymentAuthentication
            }
            "payment_authentication" => CreateThreeDSecureAuthenticationParamsMessageCategory::PaymentAuthentication,
            other => CreateThreeDSecureAuthenticationParamsMessageCategory::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsMessageCategory {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsPaymentMethodDataType {
    Card,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsPaymentMethodDataType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsPaymentMethodDataType::Card => "card",
            CreateThreeDSecureAuthenticationParamsPaymentMethodDataType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsPaymentMethodDataType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsPaymentMethodDataType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "card" => CreateThreeDSecureAuthenticationParamsPaymentMethodDataType::Card,
            other => CreateThreeDSecureAuthenticationParamsPaymentMethodDataType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsPaymentMethodDataType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, Default)]
pub struct ListThreeDSecureAuthenticationsParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for ListThreeDSecureAuthenticationsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureAuthenticationDirectoryServer {
    AmericanExpress,
    CartesBancaires,
    Discover,
    Mastercard,
    Visa,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureAuthenticationDirectoryServer {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureAuthenticationDirectoryServer::AmericanExpress => "american_express",
            ThreeDSecureAuthenticationDirectoryServer::CartesBancaires => "cartes_bancaires",
            ThreeDSecureAuthenticationDirectoryServer::Discover => "discover",
            ThreeDSecureAuthenticationDirectoryServer::Mastercard => "mastercard",
            ThreeDSecureAuthenticationDirectoryServer::Visa => "visa",
            ThreeDSecureAuthenticationDirectoryServer::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl Default for ThreeDSecureAuthenticationDirectoryServer {
    fn default() -> Self {
        ThreeDSecureAuthenticationDirectoryServer::Unknown(String::new())
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureAuthenticationDirectoryServer {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureAuthenticationDirectoryServer::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "american_express" => ThreeDSecureAuthenticationDirectoryServer::AmericanExpress,
            "cartes_bancaires" => ThreeDSecureAuthenticationDirectoryServer::CartesBancaires,
            "discover" => ThreeDSecureAuthenticationDirectoryServer::Discover,
            "mastercard" => ThreeDSecureAuthenticationDirectoryServer::Mastercard,
            "visa" => ThreeDSecureAuthenticationDirectoryServer::Visa,
            other => ThreeDSecureAuthenticationDirectoryServer::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureStandaloneResourceChallengeDetailsType {
    Mandated,
    Preferred,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureStandaloneResourceChallengeDetailsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureStandaloneResourceChallengeDetailsType::Mandated => "mandated",
            ThreeDSecureStandaloneResourceChallengeDetailsType::Preferred => "preferred",
            ThreeDSecureStandaloneResourceChallengeDetailsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl Default for ThreeDSecureStandaloneResourceChallengeDetailsType {
    fn default() -> Self {
        ThreeDSecureStandaloneResourceChallengeDetailsType::Unknown(String::new())
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureStandaloneResourceChallengeDetailsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureStandaloneResourceChallengeDetailsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "mandated" => ThreeDSecureStandaloneResourceChallengeDetailsType::Mandated,
            "preferred" => ThreeDSecureStandaloneResourceChallengeDetailsType::Preferred,
            other => ThreeDSecureStandaloneResourceChallengeDetailsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureStandaloneResourceFutureUsageType {
    CardOnFile,
    Installment,
    Recurring,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureStandaloneResourceFutureUsageType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureStandaloneResourceFutureUsageType::CardOnFile => "card_on_file",
            ThreeDSecureStandaloneResourceFutureUsageType::Installment => "installment",
            ThreeDSecureStandaloneResourceFutureUsageType::Recurring => "recurring",
            ThreeDSecureStandaloneResourceFutureUsageType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl Default for ThreeDSecureStandaloneResourceFutureUsageType {
    fn default() -> Self {
        ThreeDSecureStandaloneResourceFutureUsageType::Unknown(String::new())
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureStandaloneResourceFutureUsageType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureStandaloneResourceFutureUsageType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "card_on_file" => ThreeDSecureStandaloneResourceFutureUsageType::CardOnFile,
            "installment" => ThreeDSecureStandaloneResourceFutureUsageType::Installment,
            "recurring" => ThreeDSecureStandaloneResourceFutureUsageType::Recurring,
            other => ThreeDSecureStandaloneResourceFutureUsageType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator {
    N01,
    N02,
    N03,
    N04,
    N05,
    N06,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::N01 => "01",
            ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::N02 => "02",
            ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::N03 => "03",
            ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::N04 => "04",
            ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::N05 => "05",
            ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::N06 => "06",
            ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "01" => ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::N01,
            "02" => ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::N02,
            "03" => ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::N03,
            "04" => ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::N04,
            "05" => ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::N05,
            "06" => ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::N06,
            other => {
                ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::Unknown(other.to_string())
            }
        })
    }
}
