#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoIdNumberReportError {
    #[doc = "A short machine-readable string giving the reason for the verification failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<stripe_types_01::identity::GelatoIdNumberReportErrorCode>,
    #[doc = "A human-readable message giving the reason for the failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoPhoneReportError {
    #[doc = "A short machine-readable string giving the reason for the verification failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<stripe_types_02::identity::GelatoPhoneReportErrorCode>,
    #[doc = "A human-readable message giving the reason for the failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoReportDocumentOptions {
    #[doc = "Array of strings of allowed identity document types."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<Vec<stripe_types_01::identity::GelatoDocumentReportType>>,
    #[doc = "Collect an ID number and perform an ID number check..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub require_id_number: Option<bool>,
    #[doc = "Disable image uploads, identity document images have to be captured using the device’s camera."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub require_live_capture: Option<bool>,
    #[doc = "Capture a face image and perform a [selfie check](https://docs.stripe.com/identity/verification-checks?type=selfie) c..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub require_matching_selfie: Option<bool>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoSelfieReportError {
    #[doc = "A short machine-readable string giving the reason for the verification failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<stripe_types_02::identity::GelatoSelfieReportErrorCode>,
    #[doc = "A human-readable message giving the reason for the failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoSessionMatchingOptions {
    #[doc = "Strictness of the DOB matching policy to apply."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dob: Option<stripe_types_01::identity::GelatoSessionMatchingOptionsDob>,
    #[doc = "Strictness of the name matching policy to apply."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<stripe_types_01::identity::GelatoSessionMatchingOptionsDob>,
}

#[cfg(feature = "identity")]
pub type ListIdentityVerificationSessionsParamsCreated =
    stripe_types_02::identity::ListIdentityVerificationReportsParamsCreated;

#[cfg(feature = "identity")]
pub type RedactIdentityVerificationSessionParams = stripe_types_03::identity::GetIdentityVerificationReportParams;

#[cfg(feature = "identity")]
pub type UpdateIdentityVerificationSessionParamsType =
    stripe_types_03::identity::ListIdentityVerificationReportsParamsType;

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct VerificationSessionRedaction {
    #[doc = "Indicates whether this object and its related objects have been redacted or not."]
    #[json(default)]
    pub status: stripe_types_02::identity::VerificationSessionRedactionStatus,
}
