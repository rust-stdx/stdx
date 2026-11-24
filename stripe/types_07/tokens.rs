#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsAccountIndividualVerification {
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    pub additional_document:
        Option<stripe_types_05::tokens::CreateTokenParamsAccountIndividualVerificationAdditionalDocument>,
    #[doc = "An identifying document, either a passport or local ID card."]
    pub document: Option<stripe_types_05::tokens::CreateTokenParamsAccountIndividualVerificationDocument>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountIndividualVerification {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("additional_document", self.additional_document.as_ref());
        form.field_opt("document", self.document.as_ref());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsPersonDocuments {
    #[doc = "One or more documents that demonstrate proof that this person is authorized to represent the company."]
    pub company_authorization: Option<stripe_types_00::tokens::CreateTokenParamsPersonDocumentsCompanyAuthorization>,
    #[doc = "One or more documents showing the person's passport page with photo and personal data."]
    pub passport: Option<stripe_types_05::tokens::CreateTokenParamsPersonDocumentsPassport>,
    #[doc = "One or more documents showing the person's visa required for living in the country where they are residing."]
    pub visa: Option<stripe_types_06::tokens::CreateTokenParamsPersonDocumentsVisa>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsPersonDocuments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("company_authorization", self.company_authorization.as_ref());
        form.field_opt("passport", self.passport.as_ref());
        form.field_opt("visa", self.visa.as_ref());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsPersonUsCfpbData {
    #[doc = "The persons ethnicity details"]
    pub ethnicity_details: Option<stripe_types_06::tokens::CreateTokenParamsPersonUsCfpbDataEthnicityDetails>,
    #[doc = "The persons race details"]
    pub race_details: Option<stripe_types_05::tokens::CreateTokenParamsPersonUsCfpbDataRaceDetails>,
    #[doc = "The persons self-identified gender"]
    pub self_identified_gender: Option<String>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsPersonUsCfpbData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ethnicity_details", self.ethnicity_details.as_ref());
        form.field_opt("race_details", self.race_details.as_ref());
        form.field_opt("self_identified_gender", self.self_identified_gender.as_ref());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsPersonVerification {
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    pub additional_document: Option<stripe_types_06::tokens::CreateTokenParamsPersonVerificationAdditionalDocument>,
    #[doc = "An identifying document, either a passport or local ID card."]
    pub document: Option<stripe_types_05::tokens::CreateTokenParamsPersonVerificationDocument>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsPersonVerification {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("additional_document", self.additional_document.as_ref());
        form.field_opt("document", self.document.as_ref());
    }
}
