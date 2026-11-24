#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoDataVerifiedOutputsDate {
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
pub enum GelatoDocumentReportErrorCode {
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl GelatoDocumentReportErrorCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GelatoDocumentReportErrorCode::DocumentExpired => "document_expired",
            GelatoDocumentReportErrorCode::DocumentTypeNotSupported => "document_type_not_supported",
            GelatoDocumentReportErrorCode::DocumentUnverifiedOther => "document_unverified_other",
            GelatoDocumentReportErrorCode::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for GelatoDocumentReportErrorCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GelatoDocumentReportErrorCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "document_expired" => GelatoDocumentReportErrorCode::DocumentExpired,
            "document_type_not_supported" => GelatoDocumentReportErrorCode::DocumentTypeNotSupported,
            "document_unverified_other" => GelatoDocumentReportErrorCode::DocumentUnverifiedOther,
            other => GelatoDocumentReportErrorCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum GelatoIdNumberReportIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl GelatoIdNumberReportIdNumberType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GelatoIdNumberReportIdNumberType::BrCpf => "br_cpf",
            GelatoIdNumberReportIdNumberType::SgNric => "sg_nric",
            GelatoIdNumberReportIdNumberType::UsSsn => "us_ssn",
            GelatoIdNumberReportIdNumberType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for GelatoIdNumberReportIdNumberType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GelatoIdNumberReportIdNumberType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "br_cpf" => GelatoIdNumberReportIdNumberType::BrCpf,
            "sg_nric" => GelatoIdNumberReportIdNumberType::SgNric,
            "us_ssn" => GelatoIdNumberReportIdNumberType::UsSsn,
            other => GelatoIdNumberReportIdNumberType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoProvidedDetails {
    #[doc = "Email of user being verified"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Phone number of user being verified"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum GelatoSessionLastErrorCode {
    Abandoned,
    ConsentDeclined,
    CountryNotSupported,
    DeviceNotSupported,
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
    EmailUnverifiedOther,
    EmailVerificationDeclined,
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
    PhoneUnverifiedOther,
    PhoneVerificationDeclined,
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
    UnderSupportedAge,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl GelatoSessionLastErrorCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GelatoSessionLastErrorCode::Abandoned => "abandoned",
            GelatoSessionLastErrorCode::ConsentDeclined => "consent_declined",
            GelatoSessionLastErrorCode::CountryNotSupported => "country_not_supported",
            GelatoSessionLastErrorCode::DeviceNotSupported => "device_not_supported",
            GelatoSessionLastErrorCode::DocumentExpired => "document_expired",
            GelatoSessionLastErrorCode::DocumentTypeNotSupported => "document_type_not_supported",
            GelatoSessionLastErrorCode::DocumentUnverifiedOther => "document_unverified_other",
            GelatoSessionLastErrorCode::EmailUnverifiedOther => "email_unverified_other",
            GelatoSessionLastErrorCode::EmailVerificationDeclined => "email_verification_declined",
            GelatoSessionLastErrorCode::IdNumberInsufficientDocumentData => "id_number_insufficient_document_data",
            GelatoSessionLastErrorCode::IdNumberMismatch => "id_number_mismatch",
            GelatoSessionLastErrorCode::IdNumberUnverifiedOther => "id_number_unverified_other",
            GelatoSessionLastErrorCode::PhoneUnverifiedOther => "phone_unverified_other",
            GelatoSessionLastErrorCode::PhoneVerificationDeclined => "phone_verification_declined",
            GelatoSessionLastErrorCode::SelfieDocumentMissingPhoto => "selfie_document_missing_photo",
            GelatoSessionLastErrorCode::SelfieFaceMismatch => "selfie_face_mismatch",
            GelatoSessionLastErrorCode::SelfieManipulated => "selfie_manipulated",
            GelatoSessionLastErrorCode::SelfieUnverifiedOther => "selfie_unverified_other",
            GelatoSessionLastErrorCode::UnderSupportedAge => "under_supported_age",
            GelatoSessionLastErrorCode::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for GelatoSessionLastErrorCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GelatoSessionLastErrorCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "abandoned" => GelatoSessionLastErrorCode::Abandoned,
            "consent_declined" => GelatoSessionLastErrorCode::ConsentDeclined,
            "country_not_supported" => GelatoSessionLastErrorCode::CountryNotSupported,
            "device_not_supported" => GelatoSessionLastErrorCode::DeviceNotSupported,
            "document_expired" => GelatoSessionLastErrorCode::DocumentExpired,
            "document_type_not_supported" => GelatoSessionLastErrorCode::DocumentTypeNotSupported,
            "document_unverified_other" => GelatoSessionLastErrorCode::DocumentUnverifiedOther,
            "email_unverified_other" => GelatoSessionLastErrorCode::EmailUnverifiedOther,
            "email_verification_declined" => GelatoSessionLastErrorCode::EmailVerificationDeclined,
            "id_number_insufficient_document_data" => GelatoSessionLastErrorCode::IdNumberInsufficientDocumentData,
            "id_number_mismatch" => GelatoSessionLastErrorCode::IdNumberMismatch,
            "id_number_unverified_other" => GelatoSessionLastErrorCode::IdNumberUnverifiedOther,
            "phone_unverified_other" => GelatoSessionLastErrorCode::PhoneUnverifiedOther,
            "phone_verification_declined" => GelatoSessionLastErrorCode::PhoneVerificationDeclined,
            "selfie_document_missing_photo" => GelatoSessionLastErrorCode::SelfieDocumentMissingPhoto,
            "selfie_face_mismatch" => GelatoSessionLastErrorCode::SelfieFaceMismatch,
            "selfie_manipulated" => GelatoSessionLastErrorCode::SelfieManipulated,
            "selfie_unverified_other" => GelatoSessionLastErrorCode::SelfieUnverifiedOther,
            "under_supported_age" => GelatoSessionLastErrorCode::UnderSupportedAge,
            other => GelatoSessionLastErrorCode::Unknown(other.to_string()),
        })
    }
}
