#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IdentityVerificationSession {
    #[doc = "A string to reference this user."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,
    #[doc = "The short-lived client secret used by Stripe.js to show a verification modal..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If present, this property tells you the last error encountered when processing the verification."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last_error: Option<stripe_types_05::identity::GelatoSessionLastError>,
    #[doc = "ID of the most recent VerificationReport."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last_verification_report: Option<crate::Expandable<stripe_types_11::identity::IdentityVerificationReport>>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::identity::IdentityVerificationSessionObject,
    #[doc = "A set of options for the session’s verification checks."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<stripe_types_07::identity::GelatoVerificationSessionOptions>,
    #[doc = "Details provided about the user being verified."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub provided_details: Option<stripe_types_00::identity::GelatoProvidedDetails>,
    #[doc = "Redaction status of this VerificationSession."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub redaction: Option<stripe_types_06::identity::VerificationSessionRedaction>,
    #[doc = "Customer ID"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub related_customer: Option<String>,
    #[doc = "The ID of the Account representing a customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub related_customer_account: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub related_person: Option<stripe_types_03::identity::GelatoRelatedPerson>,
    #[doc = "Status of this VerificationSession."]
    #[json(default)]
    pub status: stripe_types_02::identity::IdentityVerificationSessionStatus,
    #[doc = "The type of [verification check](https://docs.stripe.com/identity/verification-checks) to be performed."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::identity::IdentityVerificationReportType,
    #[doc = "The short-lived URL that you use to redirect a user to Stripe to submit their identity information."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The configuration token of a verification flow from the dashboard."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification_flow: Option<String>,
    #[doc = "The user’s verified data."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_outputs: Option<stripe_types_05::identity::GelatoVerifiedOutputs>,
}

#[cfg(any(feature = "events", feature = "identity"))]
impl crate::PagedItem for IdentityVerificationSession {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "identity")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIdentityVerificationSessionParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "A set of options for the session’s verification checks."]
    pub options: Option<stripe_types_10::identity::UpdateIdentityVerificationSessionParamsOptions>,
    #[doc = "Details provided about the user being verified."]
    pub provided_details: Option<stripe_types_05::identity::UpdateIdentityVerificationSessionParamsProvidedDetails>,
    #[doc = "The type of [verification check](https://docs.stripe.com/identity/verification-checks) to be performed."]
    pub r#type: Option<stripe_types_06::identity::UpdateIdentityVerificationSessionParamsType>,
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for UpdateIdentityVerificationSessionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("options", self.options.as_ref());
        form.field_opt("provided_details", self.provided_details.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}
