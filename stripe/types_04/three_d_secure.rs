#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, Default)]
pub struct GetThreeDSecureAuthenticationParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for GetThreeDSecureAuthenticationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListThreeDSecureAuthenticationsParamsStatus {
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
impl ListThreeDSecureAuthenticationsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListThreeDSecureAuthenticationsParamsStatus::Canceled => "canceled",
            ListThreeDSecureAuthenticationsParamsStatus::Error => "error",
            ListThreeDSecureAuthenticationsParamsStatus::Failed => "failed",
            ListThreeDSecureAuthenticationsParamsStatus::RequiresChallenge => "requires_challenge",
            ListThreeDSecureAuthenticationsParamsStatus::RequiresSubmission => "requires_submission",
            ListThreeDSecureAuthenticationsParamsStatus::Succeeded => "succeeded",
            ListThreeDSecureAuthenticationsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ListThreeDSecureAuthenticationsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListThreeDSecureAuthenticationsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => ListThreeDSecureAuthenticationsParamsStatus::Canceled,
            "error" => ListThreeDSecureAuthenticationsParamsStatus::Error,
            "failed" => ListThreeDSecureAuthenticationsParamsStatus::Failed,
            "requires_challenge" => ListThreeDSecureAuthenticationsParamsStatus::RequiresChallenge,
            "requires_submission" => ListThreeDSecureAuthenticationsParamsStatus::RequiresSubmission,
            "succeeded" => ListThreeDSecureAuthenticationsParamsStatus::Succeeded,
            other => ListThreeDSecureAuthenticationsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for ListThreeDSecureAuthenticationsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureAuthenticationOutcome {
    Abandoned,
    AttemptAcknowledged,
    Authenticated,
    Canceled,
    Denied,
    Informational,
    InternalError,
    NotSupported,
    NotTriggered,
    ProcessingError,
    Rejected,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureAuthenticationOutcome {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureAuthenticationOutcome::Abandoned => "abandoned",
            ThreeDSecureAuthenticationOutcome::AttemptAcknowledged => "attempt_acknowledged",
            ThreeDSecureAuthenticationOutcome::Authenticated => "authenticated",
            ThreeDSecureAuthenticationOutcome::Canceled => "canceled",
            ThreeDSecureAuthenticationOutcome::Denied => "denied",
            ThreeDSecureAuthenticationOutcome::Informational => "informational",
            ThreeDSecureAuthenticationOutcome::InternalError => "internal_error",
            ThreeDSecureAuthenticationOutcome::NotSupported => "not_supported",
            ThreeDSecureAuthenticationOutcome::NotTriggered => "not_triggered",
            ThreeDSecureAuthenticationOutcome::ProcessingError => "processing_error",
            ThreeDSecureAuthenticationOutcome::Rejected => "rejected",
            ThreeDSecureAuthenticationOutcome::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureAuthenticationOutcome {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureAuthenticationOutcome::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "abandoned" => ThreeDSecureAuthenticationOutcome::Abandoned,
            "attempt_acknowledged" => ThreeDSecureAuthenticationOutcome::AttemptAcknowledged,
            "authenticated" => ThreeDSecureAuthenticationOutcome::Authenticated,
            "canceled" => ThreeDSecureAuthenticationOutcome::Canceled,
            "denied" => ThreeDSecureAuthenticationOutcome::Denied,
            "informational" => ThreeDSecureAuthenticationOutcome::Informational,
            "internal_error" => ThreeDSecureAuthenticationOutcome::InternalError,
            "not_supported" => ThreeDSecureAuthenticationOutcome::NotSupported,
            "not_triggered" => ThreeDSecureAuthenticationOutcome::NotTriggered,
            "processing_error" => ThreeDSecureAuthenticationOutcome::ProcessingError,
            "rejected" => ThreeDSecureAuthenticationOutcome::Rejected,
            other => ThreeDSecureAuthenticationOutcome::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureAuthenticationReason {
    CardholderAuthentication,
    IssuerRequested,
    LiabilityShift,
    ProcessingCosts,
    RegulatoryCompliance,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureAuthenticationReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureAuthenticationReason::CardholderAuthentication => "cardholder_authentication",
            ThreeDSecureAuthenticationReason::IssuerRequested => "issuer_requested",
            ThreeDSecureAuthenticationReason::LiabilityShift => "liability_shift",
            ThreeDSecureAuthenticationReason::ProcessingCosts => "processing_costs",
            ThreeDSecureAuthenticationReason::RegulatoryCompliance => "regulatory_compliance",
            ThreeDSecureAuthenticationReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureAuthenticationReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureAuthenticationReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cardholder_authentication" => ThreeDSecureAuthenticationReason::CardholderAuthentication,
            "issuer_requested" => ThreeDSecureAuthenticationReason::IssuerRequested,
            "liability_shift" => ThreeDSecureAuthenticationReason::LiabilityShift,
            "processing_costs" => ThreeDSecureAuthenticationReason::ProcessingCosts,
            "regulatory_compliance" => ThreeDSecureAuthenticationReason::RegulatoryCompliance,
            other => ThreeDSecureAuthenticationReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceAcquirerDetails {
    #[doc = "The Acquirer BIN (specific to the directory_server)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acquirer_bin: Option<String>,
    #[doc = "The two-letter country code of the acquirer ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acquirer_country: Option<String>,
    #[doc = "The Merchant ID (or Card Acceptor ID) that your acquirer assigned you (specific to the directory_server)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acquirer_merchant_id: Option<String>,
    #[doc = "The [merchant category code](https://en.wikipedia.org/wiki/Merchant_category_code) as defined by each payment system ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[doc = "The merchant name assigned by the acquirer or payment system."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    #[doc = "Requestor ID if you’re enrolled in the card network’s 3DS program."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub requestor_id: Option<String>,
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceCartesBancaires {
    #[doc = "The cryptogram calculation algorithm used by the card Issuer's ACS to calculate the Authentication cryptogram."]
    #[json(default)]
    pub avalgo: String,
    #[doc = "The exemption indicator returned from Cartes Bancaires in the ARes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cb_exemption: Option<String>,
    #[doc = "The risk score returned from Cartes Bancaires in the ARes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cb_score: Option<String>,
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus {
    A,
    C,
    D,
    I,
    N,
    R,
    S,
    U,
    Y,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::A => "A",
            ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::C => "C",
            ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::D => "D",
            ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::I => "I",
            ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::N => "N",
            ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::R => "R",
            ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::S => "S",
            ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::U => "U",
            ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::Y => "Y",
            ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "A" => ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::A,
            "C" => ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::C,
            "D" => ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::D,
            "I" => ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::I,
            "N" => ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::N,
            "R" => ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::R,
            "S" => ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::S,
            "U" => ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::U,
            "Y" => ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::Y,
            other => ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::Unknown(other.to_string()),
        })
    }
}
