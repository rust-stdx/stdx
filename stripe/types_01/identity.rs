#[cfg(feature = "identity")]
#[derive(Debug, Clone, Default)]
pub struct CreateIdentityVerificationSessionParamsProvidedDetails {
    #[doc = "Email of user being verified"]
    pub email: Option<String>,
    #[doc = "Phone number of user being verified"]
    pub phone: Option<String>,
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for CreateIdentityVerificationSessionParamsProvidedDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("email", self.email.as_ref());
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoDataDocumentReportIssuedDate {
    #[doc = "Numerical day between 1 and 31."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<i64>,
    #[doc = "Numerical month between 1 and 12."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    #[doc = "The four-digit year."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoDataIdNumberReportDate {
    #[doc = "Numerical day between 1 and 31."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<i64>,
    #[doc = "Numerical month between 1 and 12."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    #[doc = "The four-digit year."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum GelatoDocumentReportType {
    DrivingLicense,
    IdCard,
    Passport,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl GelatoDocumentReportType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GelatoDocumentReportType::DrivingLicense => "driving_license",
            GelatoDocumentReportType::IdCard => "id_card",
            GelatoDocumentReportType::Passport => "passport",
            GelatoDocumentReportType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for GelatoDocumentReportType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GelatoDocumentReportType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "driving_license" => GelatoDocumentReportType::DrivingLicense,
            "id_card" => GelatoDocumentReportType::IdCard,
            "passport" => GelatoDocumentReportType::Passport,
            other => GelatoDocumentReportType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum GelatoIdNumberReportErrorCode {
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl GelatoIdNumberReportErrorCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GelatoIdNumberReportErrorCode::IdNumberInsufficientDocumentData => "id_number_insufficient_document_data",
            GelatoIdNumberReportErrorCode::IdNumberMismatch => "id_number_mismatch",
            GelatoIdNumberReportErrorCode::IdNumberUnverifiedOther => "id_number_unverified_other",
            GelatoIdNumberReportErrorCode::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for GelatoIdNumberReportErrorCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GelatoIdNumberReportErrorCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "id_number_insufficient_document_data" => GelatoIdNumberReportErrorCode::IdNumberInsufficientDocumentData,
            "id_number_mismatch" => GelatoIdNumberReportErrorCode::IdNumberMismatch,
            "id_number_unverified_other" => GelatoIdNumberReportErrorCode::IdNumberUnverifiedOther,
            other => GelatoIdNumberReportErrorCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoSessionIdNumberOptions {}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum GelatoSessionMatchingOptionsDob {
    None,
    Similar,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl GelatoSessionMatchingOptionsDob {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GelatoSessionMatchingOptionsDob::None => "none",
            GelatoSessionMatchingOptionsDob::Similar => "similar",
            GelatoSessionMatchingOptionsDob::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for GelatoSessionMatchingOptionsDob {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GelatoSessionMatchingOptionsDob::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => GelatoSessionMatchingOptionsDob::None,
            "similar" => GelatoSessionMatchingOptionsDob::Similar,
            other => GelatoSessionMatchingOptionsDob::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoSessionPhoneOptions {
    #[doc = "Request one time password verification of `provided_details.phone`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub require_verification: Option<bool>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum IdentityVerificationReportObject {
    IdentityVerificationReport,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl IdentityVerificationReportObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IdentityVerificationReportObject::IdentityVerificationReport => "identity.verification_report",
            IdentityVerificationReportObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl Default for IdentityVerificationReportObject {
    fn default() -> Self {
        IdentityVerificationReportObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for IdentityVerificationReportObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IdentityVerificationReportObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "identity.verification_report" => IdentityVerificationReportObject::IdentityVerificationReport,
            other => IdentityVerificationReportObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum IdentityVerificationSessionObject {
    IdentityVerificationSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl IdentityVerificationSessionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IdentityVerificationSessionObject::IdentityVerificationSession => "identity.verification_session",
            IdentityVerificationSessionObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl Default for IdentityVerificationSessionObject {
    fn default() -> Self {
        IdentityVerificationSessionObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for IdentityVerificationSessionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IdentityVerificationSessionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "identity.verification_session" => IdentityVerificationSessionObject::IdentityVerificationSession,
            other => IdentityVerificationSessionObject::Unknown(other.to_string()),
        })
    }
}
