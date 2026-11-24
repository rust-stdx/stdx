#[cfg(feature = "identity")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem {
    DrivingLicense,
    IdCard,
    Passport,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "identity")]
impl CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem::DrivingLicense => "driving_license",
            CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem::IdCard => "id_card",
            CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem::Passport => "passport",
            CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "identity")]
impl<'json> json::Decode<'json> for CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "driving_license" => CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem::DrivingLicense,
            "id_card" => CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem::IdCard,
            "passport" => CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem::Passport,
            other => CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "identity")]
#[derive(Debug, Clone, Default)]
pub struct CreateIdentityVerificationSessionParamsRelatedPerson {
    #[doc = "A token representing a connected account."]
    pub account: String,
    #[doc = "A token referencing a Person resource that this verification is being used to verify."]
    pub person: String,
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for CreateIdentityVerificationSessionParamsRelatedPerson {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("account", &self.account);
        form.field("person", &self.person);
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoDataDocumentReportDateOfBirth {
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
pub enum GelatoDocumentReportSex {
    Redacted,
    Female,
    Male,
    UnknownValue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl GelatoDocumentReportSex {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GelatoDocumentReportSex::Redacted => "[redacted]",
            GelatoDocumentReportSex::Female => "female",
            GelatoDocumentReportSex::Male => "male",
            GelatoDocumentReportSex::UnknownValue => "unknown",
            GelatoDocumentReportSex::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for GelatoDocumentReportSex {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GelatoDocumentReportSex::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "[redacted]" => GelatoDocumentReportSex::Redacted,
            "female" => GelatoDocumentReportSex::Female,
            "male" => GelatoDocumentReportSex::Male,
            "unknown" => GelatoDocumentReportSex::UnknownValue,
            other => GelatoDocumentReportSex::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum GelatoEmailReportErrorCode {
    EmailUnverifiedOther,
    EmailVerificationDeclined,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl GelatoEmailReportErrorCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GelatoEmailReportErrorCode::EmailUnverifiedOther => "email_unverified_other",
            GelatoEmailReportErrorCode::EmailVerificationDeclined => "email_verification_declined",
            GelatoEmailReportErrorCode::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for GelatoEmailReportErrorCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GelatoEmailReportErrorCode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "email_unverified_other" => GelatoEmailReportErrorCode::EmailUnverifiedOther,
            "email_verification_declined" => GelatoEmailReportErrorCode::EmailVerificationDeclined,
            other => GelatoEmailReportErrorCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoSessionEmailOptions {
    #[doc = "Request one time password verification of `provided_details.email`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub require_verification: Option<bool>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, PartialEq)]
pub enum IdentityVerificationReportType {
    Document,
    IdNumber,
    VerificationFlow,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "identity"))]
impl IdentityVerificationReportType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IdentityVerificationReportType::Document => "document",
            IdentityVerificationReportType::IdNumber => "id_number",
            IdentityVerificationReportType::VerificationFlow => "verification_flow",
            IdentityVerificationReportType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl Default for IdentityVerificationReportType {
    fn default() -> Self {
        IdentityVerificationReportType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
impl<'json> json::Decode<'json> for IdentityVerificationReportType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IdentityVerificationReportType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "document" => IdentityVerificationReportType::Document,
            "id_number" => IdentityVerificationReportType::IdNumber,
            "verification_flow" => IdentityVerificationReportType::VerificationFlow,
            other => IdentityVerificationReportType::Unknown(other.to_string()),
        })
    }
}
