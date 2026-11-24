#[cfg(feature = "identity")]
#[derive(Debug, Clone, Default)]
pub struct CreateIdentityVerificationSessionParamsOptions {
    #[doc = "Options that apply to the [document check](https://docs.stripe.com/identity/verification-checks?type=document)."]
    pub document: Option<stripe_types_05::identity::CreateIdentityVerificationSessionParamsOptionsDocument>,
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for CreateIdentityVerificationSessionParamsOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("document", self.document.as_ref());
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoEmailReport {
    #[doc = "Email to be verified."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Details on the verification error."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<stripe_types_05::identity::GelatoEmailReportError>,
    #[doc = "Status of this `email` check."]
    #[json(default)]
    pub status: stripe_types_02::identity::GelatoDocumentReportStatus,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoVerificationSessionOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<stripe_types_05::identity::GelatoSessionDocumentOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<stripe_types_04::identity::GelatoSessionEmailOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<stripe_types_01::identity::GelatoSessionIdNumberOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub matching: Option<stripe_types_06::identity::GelatoSessionMatchingOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<stripe_types_01::identity::GelatoSessionPhoneOptions>,
}
