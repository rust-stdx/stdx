#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoDocumentReport {
    #[doc = "Address as it appears in the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
    #[doc = "Date of birth as it appears in the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dob: Option<stripe_types_04::identity::GelatoDataDocumentReportDateOfBirth>,
    #[doc = "Details on the verification error."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<stripe_types_05::identity::GelatoDocumentReportError>,
    #[doc = "Expiration date of the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<stripe_types_03::identity::GelatoDataDocumentReportExpirationDate>,
    #[doc = "Array of [File](https://docs.stripe.com/api/files) ids containing images for this document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    #[doc = "First name as it appears in the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Issued date of the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issued_date: Option<stripe_types_01::identity::GelatoDataDocumentReportIssuedDate>,
    #[doc = "Issuing country of the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuing_country: Option<String>,
    #[doc = "Last name as it appears in the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Document ID number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[doc = "Sex of the person in the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sex: Option<stripe_types_04::identity::GelatoDocumentReportSex>,
    #[doc = "Status of this `document` check."]
    #[json(default)]
    pub status: stripe_types_02::identity::GelatoDocumentReportStatus,
    #[doc = "Type of the document."]
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<stripe_types_01::identity::GelatoDocumentReportType>,
    #[doc = "Place of birth as it appears in the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unparsed_place_of_birth: Option<String>,
    #[doc = "Sex as it appears in the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unparsed_sex: Option<String>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoIdNumberReport {
    #[doc = "Date of birth."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dob: Option<stripe_types_01::identity::GelatoDataIdNumberReportDate>,
    #[doc = "Details on the verification error."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<stripe_types_06::identity::GelatoIdNumberReportError>,
    #[doc = "First name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "ID number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    #[doc = "Type of ID number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id_number_type: Option<stripe_types_00::identity::GelatoIdNumberReportIdNumberType>,
    #[doc = "Last name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Status of this `id_number` check."]
    #[json(default)]
    pub status: stripe_types_02::identity::GelatoDocumentReportStatus,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoPhoneReport {
    #[doc = "Details on the verification error."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<stripe_types_06::identity::GelatoPhoneReportError>,
    #[doc = "Phone to be verified."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "Status of this `phone` check."]
    #[json(default)]
    pub status: stripe_types_02::identity::GelatoDocumentReportStatus,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoSelfieReport {
    #[doc = "ID of the [File](https://docs.stripe.com/api/files) holding the image of the identity document used in this check."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[doc = "Details on the verification error."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<stripe_types_06::identity::GelatoSelfieReportError>,
    #[doc = "ID of the [File](https://docs.stripe.com/api/files) holding the image of the selfie used in this check."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub selfie: Option<String>,
    #[doc = "Status of this `selfie` check."]
    #[json(default)]
    pub status: stripe_types_02::identity::GelatoDocumentReportStatus,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoVerificationReportOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<stripe_types_06::identity::GelatoReportDocumentOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<stripe_types_03::identity::GelatoReportIdNumberOptions>,
}

#[cfg(feature = "identity")]
#[derive(Debug, Clone, Default)]
pub struct ListIdentityVerificationSessionsParams {
    pub client_reference_id: Option<String>,
    pub created: Option<stripe_types_06::identity::ListIdentityVerificationSessionsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub related_customer: Option<String>,
    pub related_customer_account: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_02::identity::ListIdentityVerificationSessionsParamsStatus>,
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for ListIdentityVerificationSessionsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("client_reference_id", self.client_reference_id.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("related_customer", self.related_customer.as_ref());
        form.field_opt("related_customer_account", self.related_customer_account.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}
