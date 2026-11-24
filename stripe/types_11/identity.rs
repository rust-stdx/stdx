#[cfg(feature = "identity")]
#[derive(Debug, Clone, Default)]
pub struct CreateIdentityVerificationSessionParams {
    #[doc = "A string to reference this user."]
    pub client_reference_id: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "A set of options for the session’s verification checks."]
    pub options: Option<stripe_types_07::identity::CreateIdentityVerificationSessionParamsOptions>,
    #[doc = "Details provided about the user being verified."]
    pub provided_details: Option<stripe_types_01::identity::CreateIdentityVerificationSessionParamsProvidedDetails>,
    #[doc = "Customer ID"]
    pub related_customer: Option<String>,
    #[doc = "The ID of the Account representing a customer."]
    pub related_customer_account: Option<String>,
    #[doc = "Tokens referencing a Person resource and its associated account."]
    pub related_person: Option<stripe_types_04::identity::CreateIdentityVerificationSessionParamsRelatedPerson>,
    #[doc = "The URL that the user will be redirected to upon completing the verification flow."]
    pub return_url: Option<String>,
    #[doc = "The type of [verification check](https://docs.stripe.com/identity/verification-checks) to be performed."]
    pub r#type: Option<stripe_types_05::identity::CreateIdentityVerificationSessionParamsType>,
    #[doc = "The ID of a verification flow from the Dashboard."]
    pub verification_flow: Option<String>,
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for CreateIdentityVerificationSessionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("client_reference_id", self.client_reference_id.as_ref());
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
        form.field_opt("related_customer", self.related_customer.as_ref());
        form.field_opt("related_customer_account", self.related_customer_account.as_ref());
        form.field_opt("related_person", self.related_person.as_ref());
        form.field_opt("return_url", self.return_url.as_ref());
        form.field_opt("type", self.r#type.as_ref());
        form.field_opt("verification_flow", self.verification_flow.as_ref());
    }
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IdentityVerificationReport {
    #[doc = "A string to reference this user."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<stripe_types_09::identity::GelatoDocumentReport>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<stripe_types_07::identity::GelatoEmailReport>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<stripe_types_09::identity::GelatoIdNumberReport>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::identity::IdentityVerificationReportObject,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<stripe_types_09::identity::GelatoVerificationReportOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<stripe_types_09::identity::GelatoPhoneReport>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub selfie: Option<stripe_types_09::identity::GelatoSelfieReport>,
    #[doc = "Type of report."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::identity::IdentityVerificationReportType,
    #[doc = "The configuration token of a verification flow from the dashboard."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification_flow: Option<String>,
    #[doc = "ID of the VerificationSession that created this report."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification_session: Option<String>,
}

#[cfg(any(feature = "events", feature = "identity"))]
impl crate::PagedItem for IdentityVerificationReport {
    fn id(&self) -> &str {
        &self.id
    }
}
