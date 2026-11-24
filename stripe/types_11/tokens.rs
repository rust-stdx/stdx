#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsPerson {
    #[doc = "Details on the legal guardian's or authorizer's acceptance of the required Stripe agreements."]
    pub additional_tos_acceptances: Option<stripe_types_05::tokens::CreateTokenParamsPersonAdditionalTosAcceptances>,
    #[doc = "The person's address."]
    pub address: Option<stripe_types_05::tokens::CreateTokenParamsPersonAddress>,
    #[doc = "The Kana variation of the person's address (Japan only)."]
    pub address_kana: Option<stripe_types_06::tokens::CreateTokenParamsPersonAddressKana>,
    #[doc = "The Kanji variation of the person's address (Japan only)."]
    pub address_kanji: Option<stripe_types_05::tokens::CreateTokenParamsPersonAddressKanji>,
    #[doc = "The person's date of birth."]
    pub dob: Option<stripe_types_05::tokens::CreateTokenParamsPersonDob>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    pub documents: Option<stripe_types_07::tokens::CreateTokenParamsPersonDocuments>,
    #[doc = "The person's email address."]
    pub email: Option<String>,
    #[doc = "The person's first name."]
    pub first_name: Option<String>,
    #[doc = "The Kana variation of the person's first name (Japan only)."]
    pub first_name_kana: Option<String>,
    #[doc = "The Kanji variation of the person's first name (Japan only)."]
    pub first_name_kanji: Option<String>,
    #[doc = "A list of alternate names or aliases that the person is known by."]
    pub full_name_aliases: Option<json::OwnedValue>,
    #[doc = "The person's gender (International regulations require either 'male' or 'female')."]
    pub gender: Option<String>,
    #[doc = "The person's ID number, as appropriate for their country."]
    pub id_number: Option<String>,
    #[doc = "The person's secondary ID number, as appropriate for their country, will be used for enhanced verification checks."]
    pub id_number_secondary: Option<String>,
    #[doc = "The person's last name."]
    pub last_name: Option<String>,
    #[doc = "The Kana variation of the person's last name (Japan only)."]
    pub last_name_kana: Option<String>,
    #[doc = "The Kanji variation of the person's last name (Japan only)."]
    pub last_name_kanji: Option<String>,
    #[doc = "The person's maiden name."]
    pub maiden_name: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The country where the person is a national."]
    pub nationality: Option<String>,
    #[doc = "The person's phone number."]
    pub phone: Option<String>,
    #[doc = "Indicates if the person or any of their representatives, family members, or other closely related persons, declares t..."]
    pub political_exposure: Option<stripe_types_05::tokens::CreateTokenParamsPersonPoliticalExposure>,
    #[doc = "The person's registered address."]
    pub registered_address: Option<stripe_types_05::tokens::CreateTokenParamsPersonRegisteredAddress>,
    #[doc = "The relationship that this person has with the account's legal entity."]
    pub relationship: Option<stripe_types_02::tokens::CreateTokenParamsPersonRelationship>,
    #[doc = "The last four digits of the person's Social Security number (U.S."]
    pub ssn_last_4: Option<String>,
    #[doc = "Demographic data related to the person."]
    pub us_cfpb_data: Option<stripe_types_07::tokens::CreateTokenParamsPersonUsCfpbData>,
    #[doc = "The person's verification status."]
    pub verification: Option<stripe_types_07::tokens::CreateTokenParamsPersonVerification>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsPerson {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("additional_tos_acceptances", self.additional_tos_acceptances.as_ref());
        form.field_opt("address", self.address.as_ref());
        form.field_opt("address_kana", self.address_kana.as_ref());
        form.field_opt("address_kanji", self.address_kanji.as_ref());
        form.field_opt("dob", self.dob.as_ref());
        form.field_opt("documents", self.documents.as_ref());
        form.field_opt("email", self.email.as_ref());
        form.field_opt("first_name", self.first_name.as_ref());
        form.field_opt("first_name_kana", self.first_name_kana.as_ref());
        form.field_opt("first_name_kanji", self.first_name_kanji.as_ref());
        form.field_opt("gender", self.gender.as_ref());
        form.field_opt("id_number", self.id_number.as_ref());
        form.field_opt("id_number_secondary", self.id_number_secondary.as_ref());
        form.field_opt("last_name", self.last_name.as_ref());
        form.field_opt("last_name_kana", self.last_name_kana.as_ref());
        form.field_opt("last_name_kanji", self.last_name_kanji.as_ref());
        form.field_opt("maiden_name", self.maiden_name.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("nationality", self.nationality.as_ref());
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("political_exposure", self.political_exposure.as_ref());
        form.field_opt("registered_address", self.registered_address.as_ref());
        form.field_opt("relationship", self.relationship.as_ref());
        form.field_opt("ssn_last_4", self.ssn_last_4.as_ref());
        form.field_opt("us_cfpb_data", self.us_cfpb_data.as_ref());
        form.field_opt("verification", self.verification.as_ref());
    }
}
