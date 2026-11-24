#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum GelatoDocumentReportStatus {
    Unverified,
    Verified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl GelatoDocumentReportStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GelatoDocumentReportStatus::Unverified => "unverified",
            GelatoDocumentReportStatus::Verified => "verified",
            GelatoDocumentReportStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl Default for GelatoDocumentReportStatus {
    fn default() -> Self {
        GelatoDocumentReportStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for GelatoDocumentReportStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GelatoDocumentReportStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "unverified" => GelatoDocumentReportStatus::Unverified,
            "verified" => GelatoDocumentReportStatus::Verified,
            other => GelatoDocumentReportStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum GelatoPhoneReportErrorCode {
    PhoneUnverifiedOther,
    PhoneVerificationDeclined,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl GelatoPhoneReportErrorCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GelatoPhoneReportErrorCode::PhoneUnverifiedOther => "phone_unverified_other",
            GelatoPhoneReportErrorCode::PhoneVerificationDeclined => "phone_verification_declined",
            GelatoPhoneReportErrorCode::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for GelatoPhoneReportErrorCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GelatoPhoneReportErrorCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "phone_unverified_other" => GelatoPhoneReportErrorCode::PhoneUnverifiedOther,
            "phone_verification_declined" => GelatoPhoneReportErrorCode::PhoneVerificationDeclined,
            other => GelatoPhoneReportErrorCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum GelatoSelfieReportErrorCode {
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl GelatoSelfieReportErrorCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GelatoSelfieReportErrorCode::SelfieDocumentMissingPhoto => "selfie_document_missing_photo",
            GelatoSelfieReportErrorCode::SelfieFaceMismatch => "selfie_face_mismatch",
            GelatoSelfieReportErrorCode::SelfieManipulated => "selfie_manipulated",
            GelatoSelfieReportErrorCode::SelfieUnverifiedOther => "selfie_unverified_other",
            GelatoSelfieReportErrorCode::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for GelatoSelfieReportErrorCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GelatoSelfieReportErrorCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "selfie_document_missing_photo" => GelatoSelfieReportErrorCode::SelfieDocumentMissingPhoto,
            "selfie_face_mismatch" => GelatoSelfieReportErrorCode::SelfieFaceMismatch,
            "selfie_manipulated" => GelatoSelfieReportErrorCode::SelfieManipulated,
            "selfie_unverified_other" => GelatoSelfieReportErrorCode::SelfieUnverifiedOther,
            other => GelatoSelfieReportErrorCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum IdentityVerificationSessionStatus {
    Canceled,
    Processing,
    RequiresInput,
    Verified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl IdentityVerificationSessionStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IdentityVerificationSessionStatus::Canceled => "canceled",
            IdentityVerificationSessionStatus::Processing => "processing",
            IdentityVerificationSessionStatus::RequiresInput => "requires_input",
            IdentityVerificationSessionStatus::Verified => "verified",
            IdentityVerificationSessionStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl Default for IdentityVerificationSessionStatus {
    fn default() -> Self {
        IdentityVerificationSessionStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for IdentityVerificationSessionStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IdentityVerificationSessionStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => IdentityVerificationSessionStatus::Canceled,
            "processing" => IdentityVerificationSessionStatus::Processing,
            "requires_input" => IdentityVerificationSessionStatus::RequiresInput,
            "verified" => IdentityVerificationSessionStatus::Verified,
            other => IdentityVerificationSessionStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "identity")]
#[derive(Debug, Clone, Default)]
pub struct ListIdentityVerificationReportsParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for ListIdentityVerificationReportsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}

#[cfg(feature = "identity")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListIdentityVerificationSessionsParamsStatus {
    Canceled,
    Processing,
    RequiresInput,
    Verified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "identity")]
impl ListIdentityVerificationSessionsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListIdentityVerificationSessionsParamsStatus::Canceled => "canceled",
            ListIdentityVerificationSessionsParamsStatus::Processing => "processing",
            ListIdentityVerificationSessionsParamsStatus::RequiresInput => "requires_input",
            ListIdentityVerificationSessionsParamsStatus::Verified => "verified",
            ListIdentityVerificationSessionsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "identity")]
impl<'json> json::Decode<'json> for ListIdentityVerificationSessionsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListIdentityVerificationSessionsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => ListIdentityVerificationSessionsParamsStatus::Canceled,
            "processing" => ListIdentityVerificationSessionsParamsStatus::Processing,
            "requires_input" => ListIdentityVerificationSessionsParamsStatus::RequiresInput,
            "verified" => ListIdentityVerificationSessionsParamsStatus::Verified,
            other => ListIdentityVerificationSessionsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for ListIdentityVerificationSessionsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationSessionRedactionStatus {
    Processing,
    Redacted,
    Validated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl VerificationSessionRedactionStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            VerificationSessionRedactionStatus::Processing => "processing",
            VerificationSessionRedactionStatus::Redacted => "redacted",
            VerificationSessionRedactionStatus::Validated => "validated",
            VerificationSessionRedactionStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl Default for VerificationSessionRedactionStatus {
    fn default() -> Self {
        VerificationSessionRedactionStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for VerificationSessionRedactionStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(VerificationSessionRedactionStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "processing" => VerificationSessionRedactionStatus::Processing,
            "redacted" => VerificationSessionRedactionStatus::Redacted,
            "validated" => VerificationSessionRedactionStatus::Validated,
            other => VerificationSessionRedactionStatus::Unknown(other.to_string()),
        })
    }
}
