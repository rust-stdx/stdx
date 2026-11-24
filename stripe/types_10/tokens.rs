#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsAccountIndividual {
    #[doc = "The individual's primary address."]
    pub address: Option<stripe_types_05::tokens::CreateTokenParamsAccountIndividualAddress>,
    #[doc = "The Kana variation of the individual's primary address (Japan only)."]
    pub address_kana: Option<stripe_types_05::tokens::CreateTokenParamsAccountIndividualAddressKana>,
    #[doc = "The Kanji variation of the individual's primary address (Japan only)."]
    pub address_kanji: Option<stripe_types_05::tokens::CreateTokenParamsAccountIndividualAddressKanji>,
    #[doc = "The individual's date of birth."]
    pub dob: Option<stripe_types_06::tokens::CreateTokenParamsAccountIndividualDob>,
    #[doc = "The individual's email address."]
    pub email: Option<String>,
    #[doc = "The individual's first name."]
    pub first_name: Option<String>,
    #[doc = "The Kana variation of the individual's first name (Japan only)."]
    pub first_name_kana: Option<String>,
    #[doc = "The Kanji variation of the individual's first name (Japan only)."]
    pub first_name_kanji: Option<String>,
    #[doc = "A list of alternate names or aliases that the individual is known by."]
    pub full_name_aliases: Option<json::OwnedValue>,
    #[doc = "The individual's gender"]
    pub gender: Option<String>,
    #[doc = "The government-issued ID number of the individual, as appropriate for the representative's country."]
    pub id_number: Option<String>,
    #[doc = "The government-issued secondary ID number of the individual, as appropriate for the representative's country, will be..."]
    pub id_number_secondary: Option<String>,
    #[doc = "The individual's last name."]
    pub last_name: Option<String>,
    #[doc = "The Kana variation of the individual's last name (Japan only)."]
    pub last_name_kana: Option<String>,
    #[doc = "The Kanji variation of the individual's last name (Japan only)."]
    pub last_name_kanji: Option<String>,
    #[doc = "The individual's maiden name."]
    pub maiden_name: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The individual's phone number."]
    pub phone: Option<String>,
    #[doc = "Indicates if the person or any of their representatives, family members, or other closely related persons, declares t..."]
    pub political_exposure: Option<stripe_types_04::tokens::CreateTokenParamsAccountIndividualPoliticalExposure>,
    #[doc = "The individual's registered address."]
    pub registered_address: Option<stripe_types_06::tokens::CreateTokenParamsAccountIndividualRegisteredAddress>,
    #[doc = "Describes the person’s relationship to the account."]
    pub relationship: Option<stripe_types_04::tokens::CreateTokenParamsAccountIndividualRelationship>,
    #[doc = "The last four digits of the individual's Social Security Number (U.S."]
    pub ssn_last_4: Option<String>,
    #[doc = "The individual's verification document information."]
    pub verification: Option<stripe_types_07::tokens::CreateTokenParamsAccountIndividualVerification>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountIndividual {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("address_kana", self.address_kana.as_ref());
        form.field_opt("address_kanji", self.address_kanji.as_ref());
        form.field_opt("dob", self.dob.as_ref());
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
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("political_exposure", self.political_exposure.as_ref());
        form.field_opt("registered_address", self.registered_address.as_ref());
        form.field_opt("relationship", self.relationship.as_ref());
        form.field_opt("ssn_last_4", self.ssn_last_4.as_ref());
        form.field_opt("verification", self.verification.as_ref());
    }
}
