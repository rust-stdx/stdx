#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsIndividual {
    #[doc = "The individual's primary address."]
    pub address: Option<stripe_types_06::accounts::CreateAccountParamsIndividualAddress>,
    #[doc = "The Kana variation of the individual's primary address (Japan only)."]
    pub address_kana: Option<stripe_types_05::accounts::CreateAccountParamsIndividualAddressKana>,
    #[doc = "The Kanji variation of the individual's primary address (Japan only)."]
    pub address_kanji: Option<stripe_types_05::accounts::CreateAccountParamsIndividualAddressKanji>,
    #[doc = "The individual's date of birth."]
    pub dob: Option<stripe_types_06::accounts::CreateAccountParamsIndividualDob>,
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
    pub political_exposure: Option<stripe_types_03::accounts::CreateAccountParamsIndividualPoliticalExposure>,
    #[doc = "The individual's registered address."]
    pub registered_address: Option<stripe_types_06::accounts::CreateAccountParamsIndividualRegisteredAddress>,
    #[doc = "Describes the person’s relationship to the account."]
    pub relationship: Option<stripe_types_02::accounts::CreateAccountParamsIndividualRelationship>,
    #[doc = "The last four digits of the individual's Social Security Number (U.S."]
    pub ssn_last_4: Option<String>,
    #[doc = "The individual's verification document information."]
    pub verification: Option<stripe_types_07::accounts::CreateAccountParamsIndividualVerification>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsIndividual {
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

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountExternalAccountParams {
    #[doc = "The name of the person or business that owns the bank account."]
    pub account_holder_name: Option<String>,
    #[doc = "The type of entity that holds the account."]
    pub account_holder_type: Option<stripe_types_03::accounts::UpdateAccountExternalAccountParamsAccountHolderType>,
    #[doc = "The bank account type."]
    pub account_type: Option<stripe_types_00::accounts::UpdateAccountExternalAccountParamsAccountType>,
    #[doc = "City/District/Suburb/Town/Village."]
    pub address_city: Option<String>,
    #[doc = "Billing address country, if provided when creating card."]
    pub address_country: Option<String>,
    #[doc = "Address line 1 (Street address/PO Box/Company name)."]
    pub address_line1: Option<String>,
    #[doc = "Address line 2 (Apartment/Suite/Unit/Building)."]
    pub address_line2: Option<String>,
    #[doc = "State/County/Province/Region."]
    pub address_state: Option<String>,
    #[doc = "ZIP or postal code."]
    pub address_zip: Option<String>,
    #[doc = "When set to true, this becomes the default external account for its currency."]
    pub default_for_currency: Option<bool>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    pub documents: Option<stripe_types_07::accounts::UpdateAccountExternalAccountParamsDocuments>,
    #[doc = "Two digit number representing the card’s expiration month."]
    pub exp_month: Option<String>,
    #[doc = "Four digit number representing the card’s expiration year."]
    pub exp_year: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Cardholder name."]
    pub name: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountExternalAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_name", self.account_holder_name.as_ref());
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field_opt("account_type", self.account_type.as_ref());
        form.field_opt("address_city", self.address_city.as_ref());
        form.field_opt("address_country", self.address_country.as_ref());
        form.field_opt("address_line1", self.address_line1.as_ref());
        form.field_opt("address_line2", self.address_line2.as_ref());
        form.field_opt("address_state", self.address_state.as_ref());
        form.field_opt("address_zip", self.address_zip.as_ref());
        form.field_opt("default_for_currency", self.default_for_currency.as_ref());
        form.field_opt("documents", self.documents.as_ref());
        form.field_opt("exp_month", self.exp_month.as_ref());
        form.field_opt("exp_year", self.exp_year.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("name", self.name.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsCompany {
    #[doc = "The company's primary address."]
    pub address: Option<stripe_types_05::accounts::UpdateAccountParamsCompanyAddress>,
    #[doc = "The Kana variation of the company's primary address (Japan only)."]
    pub address_kana: Option<stripe_types_06::accounts::UpdateAccountParamsCompanyAddressKana>,
    #[doc = "The Kanji variation of the company's primary address (Japan only)."]
    pub address_kanji: Option<stripe_types_05::accounts::UpdateAccountParamsCompanyAddressKanji>,
    #[doc = "The location where the business is administered."]
    pub administrative_address: Option<stripe_types_06::accounts::UpdateAccountParamsCompanyAdministrativeAddress>,
    #[doc = "Whether the company's directors have been provided."]
    pub directors_provided: Option<bool>,
    #[doc = "This hash is used to attest that the directors information provided to Stripe is both current and correct."]
    pub directorship_declaration: Option<stripe_types_05::accounts::UpdateAccountParamsCompanyDirectorshipDeclaration>,
    #[doc = "Whether the company's executives have been provided."]
    pub executives_provided: Option<bool>,
    #[doc = "The export license ID number of the company, also referred as Import Export Code (India only)."]
    pub export_license_id: Option<String>,
    #[doc = "The purpose code to use for export transactions (India only)."]
    pub export_purpose_code: Option<String>,
    #[doc = "The company's legal name."]
    pub name: Option<String>,
    #[doc = "The Kana variation of the company's legal name (Japan only)."]
    pub name_kana: Option<String>,
    #[doc = "The Kanji variation of the company's legal name (Japan only)."]
    pub name_kanji: Option<String>,
    #[doc = "Whether the company's owners have been provided."]
    pub owners_provided: Option<bool>,
    #[doc = "This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct."]
    pub ownership_declaration: Option<stripe_types_05::accounts::UpdateAccountParamsCompanyOwnershipDeclaration>,
    #[doc = "This value is used to determine if a business is exempt from providing ultimate beneficial owners."]
    pub ownership_exemption_reason:
        Option<stripe_types_05::accounts::UpdateAccountParamsCompanyOwnershipExemptionReason>,
    #[doc = "The company's phone number (used for verification)."]
    pub phone: Option<String>,
    #[doc = "The primary location where the business conducts operations."]
    pub principal_place_of_business:
        Option<stripe_types_06::accounts::UpdateAccountParamsCompanyPrincipalPlaceOfBusiness>,
    pub registration_date: Option<stripe_types_06::accounts::UpdateAccountParamsCompanyRegistrationDate>,
    #[doc = "The identification number given to a company when it is registered or incorporated, if distinct from the identificati..."]
    pub registration_number: Option<String>,
    #[doc = "This hash is used to attest that the representative is authorized to act as the representative of their legal entity."]
    pub representative_declaration:
        Option<stripe_types_06::accounts::UpdateAccountParamsCompanyRepresentativeDeclaration>,
    #[doc = "The category identifying the legal structure of the company or legal entity."]
    pub structure: Option<stripe_types_06::accounts::UpdateAccountParamsCompanyStructure>,
    #[doc = "The business ID number of the company, as appropriate for the company’s country."]
    pub tax_id: Option<String>,
    #[doc = "The jurisdiction in which the `tax_id` is registered (Germany-based companies only)."]
    pub tax_id_registrar: Option<String>,
    #[doc = "The VAT number of the company."]
    pub vat_id: Option<String>,
    #[doc = "Information on the verification state of the company."]
    pub verification: Option<stripe_types_08::accounts::UpdateAccountParamsCompanyVerification>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsCompany {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("address_kana", self.address_kana.as_ref());
        form.field_opt("address_kanji", self.address_kanji.as_ref());
        form.field_opt("administrative_address", self.administrative_address.as_ref());
        form.field_opt("directors_provided", self.directors_provided.as_ref());
        form.field_opt("directorship_declaration", self.directorship_declaration.as_ref());
        form.field_opt("executives_provided", self.executives_provided.as_ref());
        form.field_opt("export_license_id", self.export_license_id.as_ref());
        form.field_opt("export_purpose_code", self.export_purpose_code.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("name_kana", self.name_kana.as_ref());
        form.field_opt("name_kanji", self.name_kanji.as_ref());
        form.field_opt("owners_provided", self.owners_provided.as_ref());
        form.field_opt("ownership_declaration", self.ownership_declaration.as_ref());
        form.field_opt("ownership_exemption_reason", self.ownership_exemption_reason.as_ref());
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("principal_place_of_business", self.principal_place_of_business.as_ref());
        form.field_opt("registration_date", self.registration_date.as_ref());
        form.field_opt("registration_number", self.registration_number.as_ref());
        form.field_opt("representative_declaration", self.representative_declaration.as_ref());
        form.field_opt("structure", self.structure.as_ref());
        form.field_opt("tax_id", self.tax_id.as_ref());
        form.field_opt("tax_id_registrar", self.tax_id_registrar.as_ref());
        form.field_opt("vat_id", self.vat_id.as_ref());
        form.field_opt("verification", self.verification.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsSettingsPaypayPayments {
    #[doc = "Additional files that are required to support the onboarding process of your business."]
    pub additional_files: Option<Vec<String>>,
    #[doc = "The type of goods your business sells."]
    pub goods_type: Option<stripe_types_06::accounts::UpdateAccountParamsSettingsPaypayPaymentsGoodsType>,
    #[doc = "Details regarding your business's website."]
    pub site: Option<stripe_types_08::accounts::UpdateAccountParamsSettingsPaypayPaymentsSite>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsSettingsPaypayPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.additional_files.as_ref() {
            form.field_array("additional_files", values.as_slice());
        }
        form.field_opt("goods_type", self.goods_type.as_ref());
        form.field_opt("site", self.site.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountPersonParamsUsCfpbData {
    #[doc = "The persons ethnicity details"]
    pub ethnicity_details: Option<stripe_types_07::accounts::UpdateAccountPersonParamsUsCfpbDataEthnicityDetails>,
    #[doc = "The persons race details"]
    pub race_details: Option<stripe_types_08::accounts::UpdateAccountPersonParamsUsCfpbDataRaceDetails>,
    #[doc = "The persons self-identified gender"]
    pub self_identified_gender: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountPersonParamsUsCfpbData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ethnicity_details", self.ethnicity_details.as_ref());
        form.field_opt("race_details", self.race_details.as_ref());
        form.field_opt("self_identified_gender", self.self_identified_gender.as_ref());
    }
}
