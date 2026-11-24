#[cfg(feature = "identity")]
pub type CancelIdentityVerificationSessionParams = stripe_types_03::identity::GetIdentityVerificationReportParams;

#[cfg(feature = "identity")]
#[derive(Debug, Clone, Default)]
pub struct CreateIdentityVerificationSessionParamsOptionsDocument {
    #[doc = "Array of strings of allowed identity document types."]
    pub allowed_types:
        Option<Vec<stripe_types_04::identity::CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem>>,
    #[doc = "Collect an ID number and perform an ID number check..."]
    pub require_id_number: Option<bool>,
    #[doc = "Disable image uploads, identity document images have to be captured using the device’s camera."]
    pub require_live_capture: Option<bool>,
    #[doc = "Capture a face image and perform a [selfie check](https://docs.stripe.com/identity/verification-checks?type=selfie) c..."]
    pub require_matching_selfie: Option<bool>,
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for CreateIdentityVerificationSessionParamsOptionsDocument {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.allowed_types.as_ref() {
            form.field_array("allowed_types", values.as_slice());
        }
        form.field_opt("require_id_number", self.require_id_number.as_ref());
        form.field_opt("require_live_capture", self.require_live_capture.as_ref());
        form.field_opt("require_matching_selfie", self.require_matching_selfie.as_ref());
    }
}

#[cfg(feature = "identity")]
pub type CreateIdentityVerificationSessionParamsType =
    stripe_types_03::identity::ListIdentityVerificationReportsParamsType;

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoDocumentReportError {
    #[doc = "A short machine-readable string giving the reason for the verification failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<stripe_types_00::identity::GelatoDocumentReportErrorCode>,
    #[doc = "A human-readable message giving the reason for the failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoEmailReportError {
    #[doc = "A short machine-readable string giving the reason for the verification failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<stripe_types_04::identity::GelatoEmailReportErrorCode>,
    #[doc = "A human-readable message giving the reason for the failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoSessionDocumentOptions {
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
pub struct GelatoSessionLastError {
    #[doc = "A short machine-readable string giving the reason for the verification or user-session failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<stripe_types_00::identity::GelatoSessionLastErrorCode>,
    #[doc = "A message that explains the reason for verification or user-session failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoVerifiedOutputs {
    #[doc = "The user's verified address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
    #[doc = "The user’s verified date of birth."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dob: Option<stripe_types_00::identity::GelatoDataVerifiedOutputsDate>,
    #[doc = "The user's verified email address"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The user's verified first name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "The user's verified id number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    #[doc = "The user's verified id number type."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id_number_type: Option<stripe_types_00::identity::GelatoIdNumberReportIdNumberType>,
    #[doc = "The user's verified last name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "The user's verified phone number"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The user's verified sex."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sex: Option<stripe_types_04::identity::GelatoDocumentReportSex>,
    #[doc = "The user's verified place of birth as it appears in the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unparsed_place_of_birth: Option<String>,
    #[doc = "The user's verified sex as it appears in the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unparsed_sex: Option<String>,
}

#[cfg(feature = "identity")]
pub type GetIdentityVerificationSessionParams = stripe_types_03::identity::GetIdentityVerificationReportParams;

#[cfg(feature = "identity")]
#[derive(Debug, Clone, Default)]
pub struct ListIdentityVerificationReportsParams {
    pub client_reference_id: Option<String>,
    pub created: Option<stripe_types_02::identity::ListIdentityVerificationReportsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub r#type: Option<stripe_types_03::identity::ListIdentityVerificationReportsParamsType>,
    pub verification_session: Option<String>,
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for ListIdentityVerificationReportsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("client_reference_id", self.client_reference_id.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("type", self.r#type.as_ref());
        form.field_opt("verification_session", self.verification_session.as_ref());
    }
}

#[cfg(feature = "identity")]
pub type UpdateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem =
    stripe_types_04::identity::CreateIdentityVerificationSessionParamsOptionsDocumentAllowedTypesItem;

#[cfg(feature = "identity")]
pub type UpdateIdentityVerificationSessionParamsProvidedDetails =
    stripe_types_01::identity::CreateIdentityVerificationSessionParamsProvidedDetails;
